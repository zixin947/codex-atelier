//! The image-generation declaration switch, with a preview of the precise config change.
use super::{utilities, Result};
use crate::accounts::storage::{hash, read_external};
use crate::gateway::{
    storage::{atomic_private, open_private, private_dir},
    types::now_ms,
};
use fs2::FileExt;
use serde::Serialize;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};
use tauri::Manager;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageCompatPlan {
    ticket: String,
    enabled: bool,
    changes: Vec<String>,
    warnings: Vec<String>,
}
struct Ticket {
    view: ImageCompatPlan,
    app_data: PathBuf,
    codex_home: PathBuf,
    before: Option<Vec<u8>>,
    after: Vec<u8>,
    created_at: u64,
}
#[derive(Default)]
pub struct ImageCompatState(Mutex<HashMap<String, Ticket>>);

fn codex_home() -> Result<PathBuf> {
    std::env::var_os("CODEX_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|p| PathBuf::from(p).join(".codex")))
        .ok_or_else(|| "无法定位 Codex 配置目录".into())
}
fn config(home: &Path) -> Result<Option<Vec<u8>>> {
    if !home.is_absolute() {
        return Err("Codex 目录必须为绝对路径".into());
    }
    let m = fs::symlink_metadata(home).map_err(|_| "Codex 目录不存在；不会自动创建".to_string())?;
    if !m.is_dir() || m.file_type().is_symlink() {
        return Err("Codex 目录必须为普通目录".into());
    }
    read_external(&home.join("config.toml"), 1024 * 1024)
}
fn compose(before: &[u8], enabled: bool) -> Result<Vec<u8>> {
    let source = std::str::from_utf8(before).map_err(|_| "配置不是 UTF-8，未修改")?;
    Ok(
        atelier_maintenance_core::image_compat::compose_image_compat_flag(source, enabled)
            .into_bytes(),
    )
}
fn prepare(app_data: PathBuf, home: PathBuf, enabled: bool) -> Result<Ticket> {
    let before = config(&home)?;
    let after = compose(before.as_deref().unwrap_or_default(), enabled)?;
    let ticket = uuid::Uuid::new_v4().to_string();
    let view = ImageCompatPlan {
        ticket: ticket.clone(), enabled,
        changes: vec![
            format!("{}：{}", home.join("config.toml").display(), if enabled { "设置 [features] image_generation = false" } else { "移除 [features] image_generation 限制" }),
            if enabled { "Atelier 网关从下一次请求开始过滤生图工具声明" } else { "Atelier 网关从下一次请求开始恢复生图工具透传" }.into(),
            format!("修改前备份到 {}", app_data.join("maintenance/image-compat").join(&ticket).display()),
        ],
        warnings: vec!["不改变图片输入，也不新增生图能力；Desktop 何时重新读取声明由客户端决定。未选中行保持文本；按原生流程统一为 LF 换行，空 features 段可能移除。".into()],
    };
    Ok(Ticket {
        view,
        app_data,
        codex_home: home,
        before,
        after,
        created_at: now_ms(),
    })
}
fn same(a: &Option<Vec<u8>>, b: &Option<Vec<u8>>) -> bool {
    a.as_ref().map(|v| hash(v)) == b.as_ref().map(|v| hash(v))
}
fn commit<T>(ticket: &Ticket, save: impl FnOnce(bool) -> Result<T>) -> Result<T> {
    if now_ms().saturating_sub(ticket.created_at) > 300_000 {
        return Err("预览已过期，请重新操作".into());
    }
    let maintenance = ticket.app_data.join("maintenance");
    private_dir(&maintenance)?;
    let lock = open_private(&maintenance.join("codex-write.lock"))?;
    lock.try_lock_exclusive()
        .map_err(|_| "另一维护操作正在修改配置，请稍后重试")?;
    if !same(&ticket.before, &config(&ticket.codex_home)?) {
        return Err("配置在预览后已变化，请重新预览".into());
    }
    let parent = maintenance.join("image-compat");
    private_dir(&parent)?;
    let backup = parent.join(&ticket.view.ticket);
    private_dir(&backup)?;
    if let Some(raw) = &ticket.before {
        atomic_private(&backup.join("config.toml"), raw)?;
    }
    atomic_private(&backup.join("manifest.json"), &serde_json::to_vec_pretty(&serde_json::json!({
        "configExisted": ticket.before.is_some(), "enabled":ticket.view.enabled, "createdAt":now_ms()
    })).map_err(|_| "备份清单编码失败")?)?;
    let path = ticket.codex_home.join("config.toml");
    let changed = ticket.before.as_deref() != Some(ticket.after.as_slice())
        && (ticket.before.is_some() || !ticket.after.is_empty());
    if changed {
        atomic_private(&path, &ticket.after)?;
        if config(&ticket.codex_home)?.as_deref() != Some(ticket.after.as_slice()) {
            return Err("配置写入后读回不一致；备份已保留，未覆盖可能的外部修改".into());
        }
    }
    match save(ticket.view.enabled) {
        Ok(value) => Ok(value),
        Err(error) => {
            if changed && config(&ticket.codex_home)?.as_deref() == Some(ticket.after.as_slice()) {
                if let Some(before) = &ticket.before {
                    atomic_private(&path, before)?;
                } else {
                    fs::remove_file(&path)
                        .map_err(|_| "偏好保存失败，配置回退失败；请使用已保存的备份")?;
                }
                Err(format!("{error}；配置已回退"))
            } else {
                Err(format!(
                    "{error}；未覆盖外部变化，备份位于 {}",
                    backup.display()
                ))
            }
        }
    }
}

#[tauri::command]
pub async fn maintenance_preview_image_compat(
    app: tauri::AppHandle,
    enabled: bool,
) -> Result<ImageCompatPlan> {
    tauri::async_runtime::spawn_blocking(move || {
        let app_data = app
            .path()
            .app_data_dir()
            .map_err(|_| "无法定位 Atelier 数据目录")?;
        let t = prepare(app_data, codex_home()?, enabled)?;
        let state = app.state::<ImageCompatState>();
        let mut tickets = state.0.lock().map_err(|_| "图片兼容预览状态不可用")?;
        tickets.retain(|_, t| now_ms().saturating_sub(t.created_at) < 300_000);
        if tickets.len() >= 16 {
            return Err("待确认预览过多，请稍后重试".into());
        }
        let view = t.view.clone();
        tickets.insert(view.ticket.clone(), t);
        Ok(view)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn maintenance_apply_image_compat(
    app: tauri::AppHandle,
    ticket: String,
    confirmed: bool,
) -> Result<utilities::UtilitySettings> {
    if !confirmed {
        return Err("请确认图片兼容变更".into());
    }
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<ImageCompatState>();
        let t = state
            .0
            .lock()
            .map_err(|_| "图片兼容预览状态不可用")?
            .remove(&ticket)
            .ok_or("预览不存在或已执行，请重新预览")?;
        if t.codex_home != codex_home()?
            || t.app_data
                != app
                    .path()
                    .app_data_dir()
                    .map_err(|_| "无法定位 Atelier 数据目录")?
        {
            return Err("维护目标发生变化，请重新预览".into());
        }
        commit(&t, |_| {
            // Native image state is the config getter, never the old utility JSON preference.
            let mut receipt = utilities::read_settings_at(&t.app_data).unwrap_or_default();
            receipt.image_compat = atelier_maintenance_core::image_compat::get_image_compat(
                &t.codex_home.join("config.toml"),
            )
            .enabled;
            Ok(receipt)
        })
    })
    .await
    .map_err(|e| e.to_string())?
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let p = std::env::temp_dir().join(format!("atelier-image-{}", uuid::Uuid::new_v4()));
            fs::create_dir_all(p.join("codex")).unwrap();
            Self(p)
        }
        fn ticket(&self, enabled: bool) -> Ticket {
            prepare(self.0.join("app"), self.0.join("codex"), enabled).unwrap()
        }
        fn path(&self) -> PathBuf {
            self.0.join("codex/config.toml")
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    #[test]
    fn preserves_comments_and_other_features_and_restores_passthrough() {
        let f = Fixture::new();
        fs::write(
            f.path(),
            "# keep\nmodel='test'\n[features]\n# also keep\nweb_search=true\n",
        )
        .unwrap();
        let on = f.ticket(true);
        commit(&on, |v| {
            assert!(v);
            Ok(())
        })
        .unwrap();
        let text = fs::read_to_string(f.path()).unwrap();
        assert!(text.contains("# also keep"));
        assert!(text.contains("web_search=true"));
        let parsed: toml::Value = toml::from_str(&text).unwrap();
        assert_eq!(
            parsed["features"]["image_generation"].as_bool(),
            Some(false)
        );
        commit(&f.ticket(false), |v| {
            assert!(!v);
            Ok(())
        })
        .unwrap();
        assert!(!fs::read_to_string(f.path())
            .unwrap()
            .contains("image_generation"));
        assert!(on
            .app_data
            .join("maintenance/image-compat")
            .join(on.view.ticket)
            .join("config.toml")
            .is_file());
    }
    #[test]
    fn rejects_external_change_and_rolls_back_failed_completion() {
        let f = Fixture::new();
        fs::write(f.path(), "model='original'\n").unwrap();
        let t = f.ticket(true);
        fs::write(f.path(), "model='external'\n").unwrap();
        assert!(commit(&t, |_| Ok(())).unwrap_err().contains("变化"));
        let t = f.ticket(true);
        assert!(commit::<()>(&t, |_| Err("fixture failure".into()))
            .unwrap_err()
            .contains("回退"));
        assert_eq!(fs::read_to_string(f.path()).unwrap(), "model='external'\n");
    }
    #[test]
    fn native_lexical_update_preserves_malformed_unrelated_bytes_and_disable_is_non_creating() {
        let f = Fixture::new();
        commit(&f.ticket(false), |_| Ok(())).unwrap();
        assert!(!f.path().exists());
        fs::write(f.path(), "features = [").unwrap();
        let t = f.ticket(true);
        commit(&t, |_| Ok(())).unwrap();
        assert_eq!(
            fs::read_to_string(f.path()).unwrap(),
            "features = [\n\n[features]\nimage_generation = false\n"
        );
        assert_eq!(
            fs::read(
                t.app_data
                    .join("maintenance/image-compat")
                    .join(&t.view.ticket)
                    .join("config.toml")
            )
            .unwrap(),
            b"features = ["
        );
    }
    #[test]
    fn rollback_removes_only_config_created_by_the_operation() {
        let f = Fixture::new();
        assert!(commit::<()>(&f.ticket(true), |_| Err("fixture failure".into())).is_err());
        assert!(!f.path().exists());
    }
}
