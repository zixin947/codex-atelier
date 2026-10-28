use super::storage::{hints, normalize, read_external, Store};
use crate::gateway::types::now_ms;
use serde_json::Value;
use std::{fs, path::PathBuf};
use uuid::Uuid;

fn roots() -> Vec<(String, PathBuf)> {
    let mut roots = Vec::new();
    if let Some(root) = std::env::var_os("CODEX_HOME") {
        roots.push(("CODEX_HOME".into(), PathBuf::from(root)));
    }
    if let Some(home) = std::env::var_os("HOME") {
        let root = PathBuf::from(home).join(".codex");
        if !roots.iter().any(|(_, p)| p == &root) {
            roots.push(("默认 Codex 目录".into(), root));
        }
    }
    roots
}
fn read(root: &PathBuf) -> Result<Vec<u8>, String> {
    if !root.is_absolute() {
        return Err("目录不是绝对路径，未读取。".into());
    }
    let m = fs::symlink_metadata(root).map_err(|_| "目录不存在或无法读取。")?;
    if !m.is_dir() {
        return Err("目录不是普通目录，未跟随符号链接。".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if m.uid() != unsafe { libc::getuid() } {
            return Err("目录不属于当前用户。".into());
        }
    }
    let config = read_external(&root.join("config.toml"), 1024 * 1024)?.unwrap_or_default();
    let config_text = std::str::from_utf8(&config).map_err(|_| "配置不是 UTF-8，未读取凭据。")?;
    let config_value: toml::Value =
        toml::from_str(config_text).map_err(|_| "配置无法解析，未读取凭据。")?;
    if config_value
        .get("cli_auth_credentials_store")
        .and_then(|v| v.as_str())
        .unwrap_or("file")
        != "file"
    {
        return Err("使用钥匙串、auto 或临时认证；此入口只识别文件凭据，不读取钥匙串。".into());
    }
    let bytes = read_external(&root.join("auth.json"), 256 * 1024)?
        .ok_or("未发现 auth.json 文件登录资料。")?;
    Ok(bytes)
}
// Current means the observed file identity, never an inferred running-session identity.
fn signature(path: &std::path::Path) -> Result<String, String> {
    let m = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok("absent".into()),
        Err(_) => return Err("无法核对本机登录文件状态。".into()),
    };
    if !m.is_file() {
        return Err("本机登录来源不是普通文件。".into());
    }
    let modified = m
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if m.uid() != unsafe { libc::getuid() } {
            return Err("本机登录来源不属于当前用户。".into());
        }
        Ok(format!(
            "{}:{}:{}:{}:{}",
            m.dev(),
            m.ino(),
            m.len(),
            modified,
            m.ctime_nsec()
        ))
    }
    #[cfg(not(unix))]
    {
        Ok(format!("{}:{}", m.len(), modified))
    }
}
fn capture() -> Result<(super::types::CurrentObservation, Value), String> {
    let (_, root) = roots()
        .into_iter()
        .next()
        .ok_or("无法定位本机 Codex 目录。")?;
    let path = root.join("auth.json");
    let auth_signature = signature(&path)?;
    let config_signature = signature(&root.join("config.toml"))?;
    let bytes = read(&root)?;
    let raw: Value = serde_json::from_slice(&bytes).map_err(|_| "本机登录资料格式无效。")?;
    if raw["OPENAI_API_KEY"]
        .as_str()
        .is_some_and(|key| !key.trim().is_empty())
        && raw["auth_mode"].as_str() != Some("chatgpt")
    {
        return Err("当前为 API Key 登录，请在模型与连接管理。".into());
    }
    let auth = normalize(&raw)?;
    let (identity, _, _) = hints(&auth);
    if signature(&path)? != auth_signature
        || signature(&root.join("config.toml"))? != config_signature
    {
        return Err("核对期间本机登录资料发生变化，请重试。".into());
    }
    Ok((
        super::types::CurrentObservation {
            identity,
            path: path.to_string_lossy().into(),
            auth_signature,
            config_signature,
            checked_at: now_ms(),
        },
        auth,
    ))
}
pub fn observe() -> Result<super::types::CurrentObservation, String> {
    capture().map(|(observation, _)| observation)
}

// Only the effective home is current. Never fall back to another logged-in directory.
// Unchanged source signatures avoid re-reading/copying credentials on each UI check.
pub fn detect(store: &Store) -> Result<(), String> {
    let catalog = store.read()?;
    if current_view(&catalog).state == "matched" {
        return Ok(());
    }
    let (observed, auth) = capture()?;
    let (_, email, _) = hints(&auth);
    if let Some(existing) = catalog
        .accounts
        .iter()
        .find(|a| a.identity == observed.identity)
    {
        let saved = store
            .auth(&existing.id)
            .ok()
            .and_then(|bytes| serde_json::from_slice::<Value>(&bytes).ok());
        if saved.as_ref() != Some(&auth) {
            store.reauthorize(&existing.id, &auth, &catalog.revision)?;
        }
    } else {
        let id = Uuid::new_v4().to_string();
        if let Err(error) = store.add(&id, email.as_deref().unwrap_or("本机 Codex 账号"), &auth)
        {
            // Another instance may have discovered the same identity concurrently.
            if !store
                .read()?
                .accounts
                .iter()
                .any(|a| a.identity == observed.identity)
            {
                return Err(error);
            }
        }
    }
    let path = PathBuf::from(&observed.path);
    let root = path.parent().ok_or("来源目录无效。")?;
    if signature(&path)? != observed.auth_signature
        || signature(&root.join("config.toml"))? != observed.config_signature
    {
        return Err("检测期间本机登录发生变化，稍后会重新检测。".into());
    }
    store.update(None, |catalog| {
        catalog.current_observation = Some(observed);
        Ok(())
    })?;
    Ok(())
}
pub fn current_view(c: &super::types::Catalog) -> super::types::CurrentAccount {
    use super::types::CurrentAccount;
    let unknown = |state: &str, message: &str| CurrentAccount {
        account_id: None,
        state: state.into(),
        path: None,
        checked_at: None,
        message: message.into(),
    };
    let Some(observed) = &c.current_observation else {
        return unknown("unknown", "尚未自动检测当前登录账号。");
    };
    let path = PathBuf::from(&observed.path);
    if roots().first().map(|(_, r)| r.join("auth.json")) != Some(path.clone()) {
        return unknown("changed", "Codex 来源目录已变化，请重新核对。");
    }
    let parent = match path.parent() {
        Some(p) => p,
        None => return unknown("unavailable", "来源目录无效。"),
    };
    if signature(&path).ok().as_deref() != Some(&observed.auth_signature)
        || signature(&parent.join("config.toml")).ok().as_deref()
            != Some(&observed.config_signature)
    {
        return CurrentAccount {
            account_id: None,
            state: "changed".into(),
            path: Some(observed.path.clone()),
            checked_at: Some(observed.checked_at),
            message: "文件或配置已发生变化，旧的当前账号标记已失效；请重新核对。".into(),
        };
    }
    let id = c
        .accounts
        .iter()
        .find(|p| p.identity == observed.identity)
        .map(|p| p.id.clone());
    CurrentAccount {
        state: if id.is_some() { "matched" } else { "unmanaged" }.into(),
        account_id: id,
        path: Some(observed.path.clone()),
        checked_at: Some(observed.checked_at),
        message:
            "自动检测到当前文件登录；只读来源并同步到 Atelier，不代表运行中的 Desktop 或会话身份。"
                .into(),
    }
}
