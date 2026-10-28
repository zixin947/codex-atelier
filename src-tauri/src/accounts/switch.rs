use super::{
    storage::{hash, normalize, read_external},
    types::*,
};
use crate::gateway::{
    storage::{atomic_private, read_private},
    types::now_ms,
};
use serde_json::Value;
use std::{fs, path::PathBuf};
use uuid::Uuid;
#[derive(Clone)]
pub struct Ticket {
    pub view: SwitchPreview,
    pub account: Option<String>,
    pub auth_hash: String,
    pub source_hash: String,
    pub config_hash: String,
}
pub fn target() -> Result<(PathBuf, String), String> {
    let root = std::env::var_os("CODEX_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".codex")))
        .ok_or("无法定位 Codex 目录。")?;
    if !root.is_absolute() {
        return Err("Codex 目录必须为绝对路径。".into());
    }
    let meta =
        fs::symlink_metadata(&root).map_err(|_| "目标 Codex 目录不存在；不会自动创建或接管。")?;
    if !meta.is_dir() {
        return Err("目标 Codex 目录不是普通目录。".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if meta.uid() != unsafe { libc::getuid() } {
            return Err("目标目录不属于当前用户。".into());
        }
    }
    let config = read_external(&root.join("config.toml"), 1024 * 1024)?.unwrap_or_default();
    let text = std::str::from_utf8(&config).map_err(|_| "Codex 配置不是 UTF-8。")?;
    let value: toml::Value = toml::from_str(text).map_err(|_| "Codex 配置无法解析；不会修改。")?;
    if value
        .get("cli_auth_credentials_store")
        .and_then(|v| v.as_str())
        .unwrap_or("file")
        != "file"
    {
        return Err(
            "目标使用钥匙串、自动或临时凭据。本版仅切换 file 模式，不覆盖其他认证存储。".into(),
        );
    }
    if value
        .get("profile")
        .is_some_and(|v| v.as_str().is_some_and(|s| !s.is_empty()))
    {
        return Err(
            "目标启用了命名配置档案；本版不猜测其认证覆盖，请先由你确认并切回默认配置。".into(),
        );
    }
    if value
        .get("forced_login_method")
        .and_then(|v| v.as_str())
        .is_some_and(|s| s != "chatgpt")
    {
        return Err("目标强制了其他认证方式，不能切换订阅账号。".into());
    }
    if value
        .get("model_provider")
        .and_then(|v| v.as_str())
        .is_some_and(|s| s != "openai")
    {
        return Err(
            "目标当前使用自定义 Provider；切换订阅凭据不会改变路由。请先由你切回官方 Provider。"
                .into(),
        );
    }
    Ok((root.join("auth.json"), hash(&config)))
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
        return "API Key 登录（密钥隐藏）".into();
    }
    if v["tokens"].is_object() {
        return "ChatGPT 登录（令牌隐藏）".into();
    }
    "其他认证资料（内容未展示）".into()
}
pub fn preview(store: &super::storage::Store, id: Option<String>) -> Result<Ticket, String> {
    let (path, config_hash) = target()?;
    let raw = read_external(&path, 256 * 1024)?;
    let (after, source_hash) = if let Some(id) = &id {
        let c = store.read()?;
        let p = c
            .accounts
            .iter()
            .find(|a| &a.id == id)
            .ok_or("账号不存在。")?;
        let bytes = store.auth(id)?;
        let v: Value = serde_json::from_slice(&bytes).map_err(|_| "账号凭据损坏。")?;
        normalize(&v)?;
        (
            format!("{} · {}", p.name, p.email.as_deref().unwrap_or("邮箱未知")),
            hash(&bytes),
        )
    } else {
        let bytes = read_private(&store.root.join("last-switch.json"), 512 * 1024)?;
        let backup: Backup = serde_json::from_slice(&bytes).map_err(|_| "回退记录损坏。")?;
        if backup.target != path.to_string_lossy()
            || backup.after_hash != digest(&raw)
            || backup.config_hash != config_hash
        {
            return Err("目标文件或配置已变化，不能覆盖外部修改。".into());
        }
        (summary(&backup.before), hash(&bytes))
    };
    Ok(Ticket{view:SwitchPreview{ticket:Uuid::new_v4().to_string(),target:path.to_string_lossy().into(),before:summary(&raw),after,expires_at:now_ms()+300_000,rollback:id.is_none(),warning:"仅切换所示 Codex 目录的文件凭据。请先保存工作并退出相关客户端；现有会话、钥匙串及独立 Desktop 账号不会被热切换。".into()},account:id,auth_hash:digest(&raw),source_hash,config_hash})
}
pub fn apply(store: &super::storage::Store, t: &Ticket, confirmed: bool) -> Result<(), String> {
    if !confirmed || t.view.expires_at < now_ms() {
        return Err("确认缺失或预览已过期，请重新预览。".into());
    }
    let _target_lock = store.lock("target")?;
    let (path, config_hash) = target()?;
    let old = read_external(&path, 256 * 1024)?;
    if path.to_string_lossy() != t.view.target
        || digest(&old) != t.auth_hash
        || config_hash != t.config_hash
    {
        return Err("预览后目标已变化，未写入。".into());
    }
    if let Some(id) = &t.account {
        let _lock = store.lock(id)?;
        let bytes = store.auth(id)?;
        if hash(&bytes) != t.source_hash {
            return Err("账号凭据已更新，请重新预览。".into());
        }
        let c = store.read()?;
        if !c.accounts.iter().any(|p| &p.id == id) {
            return Err("账号已移除。".into());
        }
        let backup = Backup {
            target: t.view.target.clone(),
            before: old,
            after_hash: hash(&bytes),
            config_hash,
            selected_before: c.last_selected_id,
        };
        atomic_private(
            &store.root.join("last-switch.json"),
            &serde_json::to_vec(&backup).map_err(|_| "无法编码备份。")?,
        )?;
        // Re-read immediately before replacement to reject external changes observed since preview.
        if digest(&read_external(&path, 256 * 1024)?) != t.auth_hash {
            return Err("备份期间目标发生变化，未写入。".into());
        }
        atomic_private(&path, &bytes)?;
        if digest(&read_external(&path, 256 * 1024)?) != hash(&bytes) {
            return Err("写入读回不一致，请检查客户端是否仍在写入。".into());
        }
        store.update(None, |c| {
            c.last_selected_id = Some(id.clone());
            Ok(())
        })?;
    } else {
        let bytes = read_private(&store.root.join("last-switch.json"), 512 * 1024)?;
        if hash(&bytes) != t.source_hash {
            return Err("回退记录已变化，请重新预览。".into());
        }
        let backup: Backup = serde_json::from_slice(&bytes).map_err(|_| "回退记录不可读。")?;
        if let Some(before) = &backup.before {
            atomic_private(&path, before)?;
        } else {
            fs::remove_file(&path).map_err(|_| "无法恢复无凭据状态。")?;
        }
        if digest(&read_external(&path, 256 * 1024)?) != digest(&backup.before) {
            return Err("回退读回不一致。".into());
        }
        fs::remove_file(store.root.join("last-switch.json"))
            .map_err(|_| "已回退，但备份记录清理失败。")?;
        store.update(None, |c| {
            c.last_selected_id = backup.selected_before;
            Ok(())
        })?;
    }
    Ok(())
}
