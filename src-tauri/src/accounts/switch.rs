use super::{
    direct,
    storage::{hash, normalize, read_external},
    types::*,
};
use crate::gateway::types::now_ms;
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
};
use uuid::Uuid;

#[derive(Clone)]
pub struct Ticket {
    pub view: SwitchPreview,
    pub account: Option<String>,
    pub source_hash: String,
    pub config_hash: String,
    home: PathBuf,
    home_identity: PathBuf,
    directory_identities: Vec<(PathBuf, u64, u64)>,
    app_root: PathBuf,
    files: Vec<SwitchFileBackup>,
    dependencies: Vec<(PathBuf, Option<Vec<u8>>)>,
    previous_backup: Option<Vec<u8>>,
}
pub fn target() -> Result<(PathBuf, String), String> {
    let root = std::env::var_os("CODEX_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".codex")))
        .ok_or("无法定位Codex目录")?;
    validate_home(&root)?;
    let config = read_external(&root.join("config.toml"), 1024 * 1024)?;
    Ok((root.join("auth.json"), digest(&config)))
}
fn directory_identity(path: &Path) -> Result<(PathBuf, u64, u64), String> {
    let metadata = fs::symlink_metadata(path).map_err(|_| "无法核对切换目录身份")?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err("切换目录身份不安全".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        Ok((path.into(), metadata.dev(), metadata.ino()))
    }
    #[cfg(not(unix))]
    {
        Err("本平台尚未支持账号文件安全切换".into())
    }
}
fn validate_home(root: &Path) -> Result<(), String> {
    if !root.is_absolute() {
        return Err("Codex目录必须为绝对路径".into());
    }
    let meta = fs::symlink_metadata(root).map_err(|_| "目标Codex目录不存在；不会自动创建")?;
    if !meta.is_dir() || meta.file_type().is_symlink() {
        return Err("目标Codex目录不是普通目录".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if meta.uid() != unsafe { libc::getuid() } {
            return Err("目标目录不属于当前用户".into());
        }
    }
    Ok(())
}
pub fn digest(raw: &Option<Vec<u8>>) -> String {
    raw.as_ref()
        .map(|v| hash(v))
        .unwrap_or_else(|| "absent".into())
}
fn summary(raw: &Option<Vec<u8>>) -> String {
    let Some(raw) = raw else {
        return "尚无文件凭据".into();
    };
    let Ok(v) = serde_json::from_slice::<Value>(raw) else {
        return "现有凭据文件（内容未展示）".into();
    };
    if v["OPENAI_API_KEY"].as_str().is_some() {
        return "API Key登录（密钥隐藏）".into();
    }
    if v["tokens"].is_object() {
        return "ChatGPT登录（令牌隐藏）".into();
    }
    "其他认证资料（内容未展示）".into()
}
fn validate_environment() -> Result<(), String> {
    if ["OPENAI_BASE_URL", "CODEX_MODEL_PROVIDER", "CODEX_PROFILE"]
        .iter()
        .any(|name| std::env::var_os(name).is_some_and(|v| !v.is_empty()))
    {
        return Err("Atelier进程存在连接环境覆盖，无法保证官方直连；请先核对启动环境（未修改环境或系统代理）".into());
    }
    Ok(())
}
pub fn preview(store: &super::storage::Store, id: Option<String>) -> Result<Ticket, String> {
    validate_environment()?;
    let root = std::env::var_os("CODEX_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".codex")))
        .ok_or("无法定位Codex目录")?;
    preview_at(store, id, &root)
}
fn backup_files(
    backup: &Backup,
    home: &Path,
    app: &Path,
    current_config: &Option<Vec<u8>>,
) -> Result<Vec<SwitchFileBackup>, String> {
    if backup.target != home.join("auth.json").to_string_lossy() {
        return Err("回退目标目录不匹配".into());
    }
    if backup.version == 0 {
        // Legacy backups never authorized configuration or intent changes.
        let old_hash = hash(current_config.as_deref().unwrap_or_default());
        if backup.config_hash != old_hash {
            return Err("目标配置已变化，不能回退旧版凭据记录".into());
        }
        let raw = read_external(&home.join("auth.json"), 256 * 1024)?;
        if digest(&raw) != backup.after_hash {
            return Err("目标凭据已变化，不能覆盖外部修改".into());
        }
        return Ok(vec![SwitchFileBackup {
            path: backup.target.clone(),
            before: raw,
            after: backup.before.clone(),
        }]);
    }
    if backup.version == 2 && !backup.completed {
        return Err("上次账号切换未完整完成；请依据保留的原件备份处理，不能按成功切换回退".into());
    }
    if backup.version != 2 || backup.files.len() != 3 {
        return Err("不支持的账号回退格式".into());
    }
    let allowed = [
        home.join("auth.json"),
        home.join("config.toml"),
        app.join("gateway/router-intent.json"),
    ];
    let mut result = vec![];
    for path in allowed {
        let matches: Vec<_> = backup
            .files
            .iter()
            .filter(|f| Path::new(&f.path) == path)
            .collect();
        if matches.len() != 1 {
            return Err("回退记录包含缺失、重复或未批准目标".into());
        }
        let f = matches[0];
        if read_external(&path, 1024 * 1024)? != f.after {
            return Err("凭据、配置或路由意图已变化，不能覆盖外部修改".into());
        }
        result.push(SwitchFileBackup {
            path: f.path.clone(),
            before: f.after.clone(),
            after: f.before.clone(),
        });
    }
    Ok(result)
}
fn preview_at(
    store: &super::storage::Store,
    id: Option<String>,
    home: &Path,
) -> Result<Ticket, String> {
    validate_home(home)?;
    let app = direct::app_root(store)?;
    direct::validate_parents(home, &app)?;
    let auth_path = home.join("auth.json");
    let config_path = home.join("config.toml");
    let intent_path = app.join("gateway/router-intent.json");
    let raw = read_external(&auth_path, 256 * 1024)?;
    let config = read_external(&config_path, 1024 * 1024)?;
    let intent = read_external(&intent_path, 1024 * 1024)?;
    let previous_backup = read_external(&store.root.join("last-switch.json"), 24 * 1024 * 1024)?;
    if id.is_some() {
        if let Some(bytes) = &previous_backup {
            let old: Backup =
                serde_json::from_slice(bytes).map_err(|_| "现有回退记录损坏，未覆盖")?;
            if old.version > 2 || (old.version == 2 && !old.completed) {
                return Err(
                    "上次切换未完成或备份版本不受支持，先处理原件备份，不能覆盖恢复资料".into(),
                );
            }
        }
    }
    let (after, source_hash, files, changes, dependencies) = if let Some(id) = &id {
        let c = store.read()?;
        let p = c
            .accounts
            .iter()
            .find(|a| &a.id == id)
            .ok_or("账号不存在")?;
        let bytes = store.auth(id)?;
        normalize(&serde_json::from_slice::<Value>(&bytes).map_err(|_| "账号凭据损坏")?)?;
        let plan = direct::prepare(home, &app, config.as_deref(), intent.as_deref())?;
        let files = vec![
            SwitchFileBackup {
                path: auth_path.to_string_lossy().into(),
                before: raw.clone(),
                after: Some(bytes.clone()),
            },
            SwitchFileBackup {
                path: config_path.to_string_lossy().into(),
                before: config.clone(),
                after: plan.config,
            },
            SwitchFileBackup {
                path: intent_path.to_string_lossy().into(),
                before: intent,
                after: Some(plan.intent),
            },
        ];
        (
            format!("{} · {}", p.name, p.email.as_deref().unwrap_or("邮箱未知")),
            hash(&bytes),
            files,
            plan.changes,
            plan.dependencies,
        )
    } else {
        let bytes = previous_backup.as_ref().ok_or("没有可回退记录")?;
        let backup: Backup = serde_json::from_slice(bytes).map_err(|_| "回退记录损坏")?;
        let files = backup_files(&backup, home, &app, &config)?;
        (
            summary(&backup.before),
            hash(bytes),
            files,
            vec!["按已核对的切换记录恢复凭据及本次修改的连接配置；不覆盖外部变化".into()],
            direct::dependencies(home, &app)?,
        )
    };
    let rollback = id.is_none();
    Ok(Ticket {
        view: SwitchPreview { ticket: Uuid::new_v4().to_string(), target: auth_path.to_string_lossy().into(), before: summary(&raw), after,
            expires_at: now_ms()+300_000, rollback,
            warning: "请先保存工作并退出使用此目录的客户端。确认后同时切换文件凭据与所列连接配置；不热切换旧会话、不访问钥匙串，不修改系统代理/VPN。".into(),
            connection: if rollback { "恢复切换前连接（以备份为准）" } else { "官方直连 · 绕过Atelier路由网关" }.into(), changes },
        account: id, source_hash, config_hash: digest(&config),
        home: home.into(), home_identity: fs::canonicalize(home).map_err(|_| "无法核对目标目录")?, directory_identities: vec![directory_identity(home)?, directory_identity(&app)?, directory_identity(&store.root)?], app_root: app, files, dependencies, previous_backup,
    })
}
fn validate(
    store: &super::storage::Store,
    t: &Ticket,
    current: &[SwitchFileBackup],
) -> Result<(), String> {
    validate_home(&t.home)?;
    if direct::app_root(store)? != t.app_root
        || fs::canonicalize(&t.home).ok().as_ref() != Some(&t.home_identity)
    {
        return Err("切换目标目录已变化".into());
    }
    for expected in &t.directory_identities {
        if directory_identity(&expected.0)? != *expected {
            return Err("切换目录对象已被替换，请重新预览".into());
        }
    }
    direct::validate_parents(&t.home, &t.app_root)?;
    direct::validate_dependencies(&t.dependencies)?;
    for f in current {
        if read_external(Path::new(&f.path), 1024 * 1024)? != f.before {
            return Err("预览后目标已变化，未覆盖".into());
        }
    }
    Ok(())
}
fn publish(path: &Path, bytes: Option<&[u8]>) -> Result<(), String> {
    if let Some(bytes) = bytes {
        atelier_maintenance_core::catalog::publication::write_atomic_private_report(path, bytes)
            .map_err(|e| e.to_string())?
            .require_sync()
            .map_err(|e| e.to_string())?;
    } else {
        match fs::remove_file(path) {
            Ok(()) => {
                fs::File::open(path.parent().ok_or("目标缺少父目录")?)
                    .and_then(|f| f.sync_all())
                    .map_err(|_| "删除后目录同步失败")?;
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
            Err(_) => return Err("无法恢复目标不存在状态".into()),
        }
    }
    Ok(())
}
pub fn apply(store: &super::storage::Store, t: &Ticket, confirmed: bool) -> Result<(), String> {
    validate_environment()?;
    let (current, _) = target()?;
    if current != t.home.join("auth.json") {
        return Err("当前Codex目标已变化，请重新预览".into());
    }
    apply_at(store, t, confirmed)
}
fn apply_at(store: &super::storage::Store, t: &Ticket, confirmed: bool) -> Result<(), String> {
    apply_with(store, t, confirmed, publish)
}
fn apply_with(
    store: &super::storage::Store,
    t: &Ticket,
    confirmed: bool,
    mut write: impl FnMut(&Path, Option<&[u8]>) -> Result<(), String>,
) -> Result<(), String> {
    if !confirmed || t.view.expires_at <= now_ms() {
        return Err("确认缺失或预览已过期".into());
    }
    let _target_lock = store.lock("target")?;
    let guard = crate::maintenance::diagnostics::account_write_guard(&t.home, &t.app_root)?;
    guard.validate()?;
    validate(store, t, &t.files)?;
    if read_external(&store.root.join("last-switch.json"), 24 * 1024 * 1024)? != t.previous_backup {
        return Err("回退记录在预览后变化".into());
    }
    let _account_lock = t.account.as_ref().map(|id| store.lock(id)).transpose()?;
    let catalog = store.read()?;
    if let Some(id) = &t.account {
        let bytes = store.auth(id)?;
        if hash(&bytes) != t.source_hash || !catalog.accounts.iter().any(|p| &p.id == id) {
            return Err("账号凭据或列表已变化，请重新预览".into());
        }
    }
    let backup_path = store.root.join("last-switch.json");
    let evidence = store.root.join(format!("switch-{}.json", t.view.ticket));
    let mut backup = Backup {
        target: t.view.target.clone(),
        before: t.files[0].before.clone(),
        after_hash: digest(&t.files[0].after),
        config_hash: t.config_hash.clone(),
        selected_before: catalog.last_selected_id.clone(),
        version: 2,
        completed: false,
        files: t.files.clone(),
    };
    // Durable originals exist before any target changes; evidence survives partial failures.
    let pending_bytes = serde_json::to_vec(&backup).map_err(|_| "备份编码失败")?;
    publish(&evidence, Some(&pending_bytes))?;
    if t.account.is_some() {
        publish(&backup_path, Some(&pending_bytes))?;
    }
    let expected_backup = if t.account.is_some() {
        Some(pending_bytes)
    } else {
        t.previous_backup.clone()
    };
    let mut attempted = vec![];
    let operation = (|| {
        let mut current = t.files.clone();
        for (index, file) in t.files.iter().enumerate() {
            guard.validate()?;
            validate(store, t, &current)?;
            if read_external(&backup_path, 24 * 1024 * 1024)? != expected_backup {
                return Err("执行中回退记录已变化，停止后续写入".into());
            }
            if file.before != file.after {
                attempted.push(index);
                write(Path::new(&file.path), file.after.as_deref())?;
            }
            if read_external(Path::new(&file.path), 1024 * 1024)? != file.after {
                return Err("切换写入读回不一致".into());
            }
            current[index].before = file.after.clone();
        }
        validate(store, t, &current)?;
        let selected = if let Some(id) = &t.account {
            Some(id.clone())
        } else {
            let b: Backup =
                serde_json::from_slice(t.previous_backup.as_deref().ok_or("回退记录缺失")?)
                    .map_err(|_| "回退记录损坏")?;
            b.selected_before
        };
        store.update(Some(&catalog.revision), |c| {
            c.last_selected_id = selected;
            c.current_observation = None;
            Ok(())
        })?;
        Ok::<_, String>(())
    })();
    if let Err(error) = operation {
        let mut conflicts = vec![];
        for index in attempted.into_iter().rev() {
            let file = &t.files[index];
            if guard.validate().is_err()
                || direct::validate_parents(&t.home, &t.app_root).is_err()
                || t.directory_identities
                    .iter()
                    .any(|expected| directory_identity(&expected.0).ok().as_ref() != Some(expected))
            {
                conflicts.push(file.path.clone());
                continue;
            }
            let path = Path::new(&file.path);
            match read_external(path, 1024 * 1024) {
                Ok(actual) if actual == file.before => (),
                Ok(actual) if actual == file.after => {
                    if publish(path, file.before.as_deref()).is_err() {
                        conflicts.push(file.path.clone());
                    }
                }
                _ => conflicts.push(file.path.clone()),
            }
        }
        if conflicts.is_empty() {
            if t.account.is_some() {
                if read_external(&backup_path, 24 * 1024 * 1024)? != expected_backup {
                    return Err(format!(
                        "文件已条件回退，但回退记录被外部修改，未覆盖；原件备份：{}",
                        evidence.display()
                    ));
                }
                publish(&backup_path, t.previous_backup.as_deref())?;
            }
            return Err(format!(
                "切换失败，已条件回退本次文件变更：{error}；原件备份保留于{}",
                evidence.display()
            ));
        }
        return Err(format!(
            "切换部分完成，检测到外部变化或回退失败，未覆盖冲突目标；{error}；原件备份：{}",
            evidence.display()
        ));
    }
    if read_external(&backup_path, 24 * 1024 * 1024)? != expected_backup {
        return Err(format!(
            "文件已切换，但回退记录被外部修改，未覆盖；原件备份：{}",
            evidence.display()
        ));
    }
    if t.account.is_none() {
        publish(&backup_path, None).map_err(|e| {
            format!(
                "文件已回退，但清理回退记录失败：{e}；原件备份：{}",
                evidence.display()
            )
        })?;
    } else {
        backup.completed = true;
        publish(
            &backup_path,
            Some(&serde_json::to_vec(&backup).map_err(|_| "备份编码失败")?),
        )
        .map_err(|e| {
            format!(
                "文件已切换，但完成标记保存失败：{e}；原件备份：{}",
                evidence.display()
            )
        })?;
    }
    Ok(())
}

#[cfg(test)]
#[path = "switch_tests.rs"]
mod tests;
