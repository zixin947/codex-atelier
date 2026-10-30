use crate::plugins::{mode_installation, PluginId, Rpc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{mpsc, Mutex},
    time::{Duration, Instant},
};
use tauri::Manager;

static OPERATIONS: Mutex<()> = Mutex::new(());
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PonyMode {
    Lite,
    Full,
    Ultra,
    Off,
}
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskMode {
    Unconfirmed,
    Answer,
    Review,
    Monitor,
    Change,
    Open,
}
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ControlLevel {
    Watch,
    Guard,
    Lock,
    Off,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PonyState {
    mode: PonyMode,
    config_mode: Option<PonyMode>,
    environment_mode: Option<PonyMode>,
    revision: String,
    path: String,
    can_rollback: bool,
    receipt: Option<String>,
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StsState {
    mode: TaskMode,
    level: ControlLevel,
    allowed_paths: Option<Vec<String>>,
    revision: String,
    can_rollback: bool,
    source: String,
    #[serde(default)]
    receipt: Option<String>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StsSession {
    id: String,
    title: String,
    cwd: String,
    updated_at: Option<i64>,
    #[serde(flatten)]
    state: StsState,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StsSessions {
    sessions: Vec<StsSession>,
    skipped: usize,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StsChange {
    session_id: String,
    expected_revision: String,
    mode: TaskMode,
    level: ControlLevel,
    allowed_paths: Option<Vec<String>>,
}

struct Context {
    home: PathBuf,
    codex_home: PathBuf,
    data: PathBuf,
    pony_path: PathBuf,
    environment_mode: Option<PonyMode>,
    isolated: bool,
}
fn normalize_mode(value: &str) -> Option<PonyMode> {
    serde_json::from_value(Value::String(value.trim().to_ascii_lowercase())).ok()
}
fn context(app: &tauri::AppHandle) -> Result<Context, String> {
    let home = PathBuf::from(std::env::var_os("HOME").ok_or("无法取得用户目录。")?);
    let codex_home = std::env::var_os("CODEX_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home.join(".codex"));
    let config = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home.join(".config"));
    if !config.is_absolute() {
        return Err("XDG_CONFIG_HOME 必须是绝对路径，未修改模式。".into());
    }
    Ok(Context {
        home,
        codex_home,
        data: app
            .path()
            .app_data_dir()
            .map_err(|_| "无法取得应用数据目录。")?
            .join("plugin-modes"),
        pony_path: config.join("ponytail/config.json"),
        environment_mode: std::env::var("PONYTAIL_DEFAULT_MODE")
            .ok()
            .and_then(|s| normalize_mode(&s)),
        isolated: false,
    })
}
fn bytes(path: &Path) -> Result<Option<Vec<u8>>, String> {
    let meta = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err("无法读取模式文件。".into()),
    };
    if !meta.is_file() || meta.file_type().is_symlink() || meta.len() > 512 * 1024 {
        return Err("模式文件类型或大小不安全，未修改。".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if meta.uid() != unsafe { libc::getuid() } {
            return Err("模式文件不属于当前用户。".into());
        }
    }
    fs::read(path)
        .map(Some)
        .map_err(|_| "无法读取模式文件。".into())
}
fn revision(raw: &Option<Vec<u8>>) -> String {
    raw.as_ref()
        .map(|v| format!("{:x}", Sha256::digest(v)))
        .unwrap_or_else(|| "missing".into())
}
fn object(raw: &Option<Vec<u8>>) -> Result<Value, String> {
    let Some(raw) = raw else { return Ok(json!({})) };
    let text = std::str::from_utf8(raw)
        .map_err(|_| "Ponytail 配置不是 UTF-8，未覆盖。")?
        .trim_start_matches('\u{feff}');
    let value: Value = serde_json::from_str(text).map_err(|_| "Ponytail 配置损坏，未覆盖。")?;
    if !value.is_object() {
        return Err("Ponytail 配置格式不兼容，未覆盖。".into());
    }
    Ok(value)
}
fn atomic(path: &Path, raw: &[u8]) -> Result<(), String> {
    use std::fs::OpenOptions;
    let parent = path.parent().ok_or("模式路径无效。")?;
    fs::create_dir_all(parent).map_err(|_| "无法创建模式目录。")?;
    if fs::symlink_metadata(parent)
        .map_err(|_| "无法检查模式目录。")?
        .file_type()
        .is_symlink()
    {
        return Err("模式目录不能是符号链接。".into());
    }
    let temporary = parent.join(format!(".atelier-mode-{}.tmp", uuid::Uuid::new_v4()));
    let result = (|| {
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options
            .open(&temporary)
            .map_err(|_| "无法创建模式临时文件。")?;
        file.write_all(raw)
            .and_then(|_| file.sync_all())
            .map_err(|_| "无法保存模式。")?;
        fs::rename(&temporary, path).map_err(|_| "无法提交模式文件。")
    })();
    let _ = fs::remove_file(temporary);
    result.map_err(String::from)
}
impl Context {
    fn backup(&self) -> PathBuf {
        self.data.join("ponytail-backup.json")
    }
    fn pony(&self) -> Result<PonyState, String> {
        let raw = bytes(&self.pony_path)?;
        let config = object(&raw)?;
        let config_mode = config["defaultMode"].as_str().and_then(normalize_mode);
        if config.get("defaultMode").is_some() && config_mode.is_none() {
            return Err("Ponytail defaultMode 不受支持，请先核对原配置。".into());
        }
        let revision = revision(&raw);
        let can_rollback = bytes(&self.backup())?
            .and_then(|b| serde_json::from_slice::<Value>(&b).ok())
            .is_some_and(|b| b["afterRevision"] == revision);
        Ok(PonyState {
            mode: self
                .environment_mode
                .or(config_mode)
                .unwrap_or(PonyMode::Full),
            config_mode,
            environment_mode: self.environment_mode,
            revision,
            path: self.pony_path.to_string_lossy().into(),
            can_rollback,
            receipt: None,
        })
    }
    fn pony_save(&self, mode: Option<PonyMode>, expected: &str) -> Result<PonyState, String> {
        if self.environment_mode.is_some() {
            return Err("PONYTAIL_DEFAULT_MODE 环境变量优先于配置，当前不能把文件保存当作生效。请先自行核对环境覆盖。".into());
        }
        let raw = bytes(&self.pony_path)?;
        if revision(&raw) != expected {
            return Err("Ponytail 配置已被外部修改，请刷新后重试。".into());
        }
        let mut config = object(&raw)?;
        let before = config.get("defaultMode").cloned();
        match mode {
            Some(value) => {
                config["defaultMode"] = serde_json::to_value(value).unwrap();
            }
            None => {
                config.as_object_mut().unwrap().remove("defaultMode");
            }
        }
        let next = serde_json::to_vec_pretty(&config).unwrap();
        let backup = json!({"hadDefault":before.is_some(),"defaultMode":before,"afterRevision":format!("{:x}",Sha256::digest(&next))});
        atomic(&self.backup(), &serde_json::to_vec(&backup).unwrap())?;
        if revision(&bytes(&self.pony_path)?) != expected {
            return Err("保存前配置再次变化，未覆盖。请刷新。".into());
        }
        atomic(&self.pony_path, &next)?;
        let mut result = self.pony()?;
        if result.config_mode != mode {
            return Err("Ponytail 模式读回不一致，请刷新核对。".into());
        }
        result.receipt = Some("默认档位已保存并读回；新对话的 SessionStart 读取此项，无需重启 Desktop。旧对话不保证改变。".into());
        Ok(result)
    }
    fn pony_rollback(&self, expected: &str) -> Result<PonyState, String> {
        let backup: Value =
            serde_json::from_slice(&bytes(&self.backup())?.ok_or("没有模式回退记录。")?)
                .map_err(|_| "模式回退记录损坏。")?;
        if backup["afterRevision"] != expected {
            return Err("模式或回退记录已变化，请刷新。".into());
        }
        let mode = if backup["hadDefault"] == true {
            Some(
                backup["defaultMode"]
                    .as_str()
                    .and_then(normalize_mode)
                    .ok_or("原档位不兼容，未回退。")?,
            )
        } else {
            None
        };
        self.pony_save(mode, expected)
    }
    fn rpc(&self) -> Result<Rpc, String> {
        let mut command =
            Command::new(crate::desktop_runtime::binary().ok_or("未找到 Desktop 内置引擎。")?);
        if self.isolated {
            command
                .env_clear()
                .env("PATH", "/usr/bin:/bin:/opt/homebrew/bin")
                .env("XDG_CONFIG_HOME", self.home.join("config"));
        }
        command
            .args(["app-server", "--stdio"])
            .current_dir(&self.home)
            .env("HOME", &self.home)
            .env("CODEX_HOME", &self.codex_home);
        Rpc::start(command)
    }
}
fn session_id(id: &str) -> Result<(), String> {
    let parsed = uuid::Uuid::parse_str(id).map_err(|_| "对话 ID 无效。")?;
    if parsed.to_string() != id.to_ascii_lowercase() {
        return Err("对话 ID 格式无效。".into());
    }
    Ok(())
}
fn state_file(data: &Path, id: &str) -> PathBuf {
    data.join("sessions").join(format!(
        "{}.json",
        &format!("{:x}", Sha256::digest(id.as_bytes()))[..24]
    ))
}
fn node() -> Result<PathBuf, String> {
    let candidates = std::env::var_os("PATH")
        .map(|s| {
            std::env::split_paths(&s)
                .map(|p| p.join("node"))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    candidates
        .into_iter()
        .chain([
            PathBuf::from("/opt/homebrew/bin/node"),
            PathBuf::from("/usr/local/bin/node"),
        ])
        .find(|p| p.is_file())
        .ok_or_else(|| "STS 模式控制需要已安装的 Node.js；Atelier 不会自动安装或升级。".into())
}
fn adapter(request: Value) -> Result<StsState, String> {
    let payload = serde_json::to_vec(&request).map_err(|_| "模式请求无效。")?;
    if payload.len() > 60 * 1024 {
        return Err("模式请求过大。".into());
    }
    let mut child = Command::new(node()?)
        .args(["-e", include_str!("../resources/sts-modes.cjs")])
        .env_remove("NODE_OPTIONS")
        .env_remove("NODE_PATH")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|_| "无法启动 STS 模式适配器。")?;
    child
        .stdin
        .take()
        .ok_or("模式适配器输入不可用。")?
        .write_all(&payload)
        .map_err(|_| "模式适配器输入失败。")?;
    let stdout = child.stdout.take().ok_or("模式适配器输出不可用。")?;
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let mut data = Vec::new();
        let result = stdout
            .take(512 * 1024 + 1)
            .read_to_end(&mut data)
            .map(|_| data);
        let _ = tx.send(result);
    });
    let deadline = Instant::now() + Duration::from_secs(8);
    let status = loop {
        match child.try_wait() {
            Ok(Some(s)) => break s,
            Ok(None) if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(20)),
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                return Err("STS 模式操作超时，请刷新；不会自动重试写入。".into());
            }
        }
    };
    let raw = rx
        .recv_timeout(Duration::from_secs(1))
        .map_err(|_| "模式输出超时。")?
        .map_err(|_| "模式输出读取失败。")?;
    if !status.success() || raw.len() > 512 * 1024 {
        return Err("STS 模式适配器异常退出。".into());
    }
    let result: Value = serde_json::from_slice(&raw).map_err(|_| "STS 模式返回格式不兼容。")?;
    if result["ok"] != true {
        return Err(match result["code"].as_str() {
            Some("STALE_REVISION") => "该对话状态已被外部修改，请刷新后重试。",
            Some("ENOENT") => "该对话尚无 STS 状态，请先在启用 STS 的新对话发送一条普通消息。",
            Some("MODE_REQUIRED") => {
                "guard / lock 需要明确任务模式；不能直接把未确认的任务设为强制控制。"
            }
            Some("INVALID_PATHS") => "lock 需要填写有效的文件边界，每行一个路径或目录/**。",
            Some("STS_LOCK_TIMEOUT") => "Desktop 正在更新该对话，暂未取得 STS 会话锁。请稍后刷新。",
            Some("DAMAGED_STATE" | "STS_LOCK_DAMAGED") => {
                "STS 会话状态或锁损坏，未尝试修复或覆盖。"
            }
            Some("READBACK_MISMATCH") => "STS 已写入但读回发生变化，请刷新核对。",
            _ => "STS 模式文件或协议不兼容，未确认保存。",
        }
        .into());
    }
    serde_json::from_value(result["data"].clone()).map_err(|_| "STS 模式状态不完整。".into())
}
fn sts_request(
    ctx: &Context,
    installation: &crate::plugins::ModeInstallation,
    id: &str,
    action: &str,
) -> Value {
    json!({"action":action,"sessionId":id,"sourceRoot":installation.source,"dataDir":installation.runtime_data,"backupDir":ctx.data})
}
fn thread_session(thread: &Value, state: StsState) -> Result<StsSession, String> {
    let id = thread["id"].as_str().ok_or("对话元信息无效。")?;
    session_id(id)?;
    if thread["source"] != "appServer" {
        return Err("只支持 app-server 对话，未修改其他宿主的模式。".into());
    }
    let title = thread["name"]
        .as_str()
        .filter(|s| !s.trim().is_empty())
        .or_else(|| thread["preview"].as_str())
        .unwrap_or("未命名对话")
        .chars()
        .take(90)
        .collect();
    Ok(StsSession {
        id: id.into(),
        title,
        cwd: thread["cwd"].as_str().unwrap_or_default().into(),
        updated_at: thread["updatedAt"].as_i64(),
        state,
    })
}
fn sts_read(
    app: &tauri::AppHandle,
    id: &str,
    action: &str,
    change: Option<StsChange>,
    expected: Option<&str>,
) -> Result<StsSession, String> {
    session_id(id)?;
    let ctx = context(app)?;
    let installation = mode_installation(app, PluginId::StopThatShit)?;
    if action != "status" && !installation.enabled {
        return Err("STS 插件已停用；请先启用并在目标新对话触发一次 Hook，再修改契约。".into());
    }
    let thread = ctx
        .rpc()?
        .call("thread/read", json!({"threadId":id,"includeTurns":false}))?["thread"]
        .clone();
    // Verify host metadata before touching the session-specific state file.
    if thread["source"] != "appServer" || thread["id"] != id {
        return Err("无法确认该 app-server 对话，未修改模式。".into());
    }
    let mut request = sts_request(&ctx, &installation, id, action);
    if let Some(change) = change {
        request["expectedRevision"] = json!(change.expected_revision);
        request["mode"] = serde_json::to_value(change.mode).unwrap();
        request["level"] = serde_json::to_value(change.level).unwrap();
        request["allowedPaths"] = json!(change.allowed_paths);
    } else if let Some(expected) = expected {
        request["expectedRevision"] = json!(expected);
    }
    thread_session(&thread, adapter(request)?)
}

#[tauri::command]
pub async fn plugin_pony_mode(app: tauri::AppHandle) -> Result<PonyState, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let _guard = OPERATIONS.lock().map_err(|_| "模式操作锁不可用。")?;
        mode_installation(&app, PluginId::Ponytail)?;
        context(&app)?.pony()
    })
    .await
    .map_err(|_| "模式读取任务异常。")?
}
#[tauri::command]
pub async fn plugin_pony_mode_set(
    app: tauri::AppHandle,
    mode: PonyMode,
    expected_revision: String,
) -> Result<PonyState, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let _guard = OPERATIONS.lock().map_err(|_| "模式操作锁不可用。")?;
        mode_installation(&app, PluginId::Ponytail)?;
        context(&app)?.pony_save(Some(mode), &expected_revision)
    })
    .await
    .map_err(|_| "模式保存任务异常。")?
}
#[tauri::command]
pub async fn plugin_pony_mode_rollback(
    app: tauri::AppHandle,
    expected_revision: String,
) -> Result<PonyState, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let _guard = OPERATIONS.lock().map_err(|_| "模式操作锁不可用。")?;
        mode_installation(&app, PluginId::Ponytail)?;
        context(&app)?.pony_rollback(&expected_revision)
    })
    .await
    .map_err(|_| "模式回退任务异常。")?
}
#[tauri::command]
pub async fn plugin_sts_sessions(app: tauri::AppHandle) -> Result<StsSessions, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let _guard=OPERATIONS.lock().map_err(|_| "模式操作锁不可用。")?;
        let ctx=context(&app)?; let installation=mode_installation(&app,PluginId::StopThatShit)?;
        let list=ctx.rpc()?.call("thread/list",json!({"limit":100,"archived":false,"sourceKinds":["appServer"],"sortKey":"updated_at","useStateDbOnly":true}))?;
        let mut sessions=Vec::new(); let mut skipped=0;
        for thread in list["data"].as_array().ok_or("对话清单格式不兼容。")? {
            let Some(id)=thread["id"].as_str() else {continue};
            if session_id(id).is_err() || !state_file(&installation.runtime_data,id).is_file() {continue}
            if sessions.len()>=32 {skipped+=1;continue}
            match adapter(sts_request(&ctx,&installation,id,"status")).and_then(|s|thread_session(thread,s)) {Ok(s)=>sessions.push(s),Err(_)=>skipped+=1}
        }
        Ok(StsSessions {sessions,skipped})
    }).await.map_err(|_| "对话清单任务异常。")?
}
#[tauri::command]
pub async fn plugin_sts_mode(
    app: tauri::AppHandle,
    session_id: String,
) -> Result<StsSession, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let _guard = OPERATIONS.lock().map_err(|_| "模式操作锁不可用。")?;
        sts_read(&app, &session_id, "status", None, None)
    })
    .await
    .map_err(|_| "对话模式读取异常。")?
}
#[tauri::command]
pub async fn plugin_sts_mode_set(
    app: tauri::AppHandle,
    change: StsChange,
) -> Result<StsSession, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let _guard = OPERATIONS.lock().map_err(|_| "模式操作锁不可用。")?;
        let id = change.session_id.clone();
        sts_read(&app, &id, "apply", Some(change), None)
    })
    .await
    .map_err(|_| "对话模式保存异常。")?
}
#[tauri::command]
pub async fn plugin_sts_mode_rollback(
    app: tauri::AppHandle,
    session_id: String,
    expected_revision: String,
) -> Result<StsSession, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let _guard = OPERATIONS.lock().map_err(|_| "模式操作锁不可用。")?;
        sts_read(
            &app,
            &session_id,
            "rollback",
            None,
            Some(&expected_revision),
        )
    })
    .await
    .map_err(|_| "对话模式回退异常。")?
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Context {
        let home = std::env::temp_dir().join(format!("atelier-modes-{}", uuid::Uuid::new_v4()));
        Context {
            codex_home: home.join("codex"),
            data: home.join("app"),
            pony_path: home.join("config/ponytail/config.json"),
            environment_mode: None,
            isolated: true,
            home,
        }
    }
    #[test]
    fn pony_preserves_other_fields_rejects_conflicts_and_rolls_back() {
        let ctx = fixture();
        fs::create_dir_all(ctx.pony_path.parent().unwrap()).unwrap();
        fs::write(
            &ctx.pony_path,
            "{\"quietStartup\":true,\"extra\":{\"keep\":42}}",
        )
        .unwrap();
        let first = ctx.pony().unwrap();
        assert_eq!(first.mode, PonyMode::Full);
        let saved = ctx
            .pony_save(Some(PonyMode::Ultra), &first.revision)
            .unwrap();
        assert_eq!(saved.mode, PonyMode::Ultra);
        assert!(saved.can_rollback);
        let content = object(&bytes(&ctx.pony_path).unwrap()).unwrap();
        assert_eq!(content["extra"]["keep"], 42);
        assert_eq!(content["quietStartup"], true);
        assert!(ctx
            .pony_save(Some(PonyMode::Lite), &first.revision)
            .is_err());
        let restored = ctx.pony_rollback(&saved.revision).unwrap();
        assert_eq!(restored.mode, PonyMode::Full);
        assert_eq!(restored.config_mode, None);
        // Upstream accepts mixed casing and surrounding whitespace. Such an
        // existing default must remain rollback-compatible after normalization.
        fs::write(
            &ctx.pony_path,
            r#"{"defaultMode":" FULL ","quietStartup":true}"#,
        )
        .unwrap();
        let mixed = ctx.pony().unwrap();
        let changed = ctx
            .pony_save(Some(PonyMode::Lite), &mixed.revision)
            .unwrap();
        let restored = ctx.pony_rollback(&changed.revision).unwrap();
        assert_eq!(restored.config_mode, Some(PonyMode::Full));
        assert_eq!(
            object(&bytes(&ctx.pony_path).unwrap()).unwrap()["quietStartup"],
            true
        );
        fs::write(&ctx.pony_path, "{broken").unwrap();
        assert!(ctx.pony().is_err());
        assert!(ctx.pony_save(Some(PonyMode::Lite), "missing").is_err());
        let _ = fs::remove_dir_all(ctx.home);
    }
    #[test]
    fn pony_blocks_environment_override_and_invalid_values() {
        let mut ctx = fixture();
        ctx.environment_mode = Some(PonyMode::Off);
        assert_eq!(ctx.pony().unwrap().mode, PonyMode::Off);
        assert!(ctx.pony_save(Some(PonyMode::Full), "missing").is_err());
        assert!(!ctx.pony_path.exists());
        assert!(serde_json::from_str::<PonyMode>("\"review\"").is_err());
        assert!(session_id("../../state").is_err());
    }
    #[cfg(unix)]
    #[test]
    fn pony_refuses_symlinks_without_touching_the_target() {
        let ctx = fixture();
        fs::create_dir_all(ctx.pony_path.parent().unwrap()).unwrap();
        let other = ctx.home.join("other.json");
        fs::write(&other, "{}").unwrap();
        std::os::unix::fs::symlink(&other, &ctx.pony_path).unwrap();
        assert!(ctx.pony().is_err());
        assert!(ctx.pony_save(Some(PonyMode::Ultra), "missing").is_err());
        assert_eq!(fs::read_to_string(other).unwrap(), "{}");
        let _ = fs::remove_dir_all(ctx.home);
    }
    #[test]
    #[ignore = "requires explicit pinned source path; runs original hooks only with disposable HOME and PLUGIN_DATA"]
    fn isolated_upstream_mode_effects() {
        let sources = PathBuf::from(
            std::env::var_os("ATELIER_MODE_TEST_SOURCE")
                .expect("explicit pinned source directory required"),
        );
        let ctx = fixture();
        fs::create_dir_all(ctx.home.join("work")).unwrap();
        fs::create_dir_all(&ctx.codex_home).unwrap();
        let pony_root = sources.join("ponytail");
        let sts_root = sources.join("stop-that-shit");
        let sts_data = ctx.home.join("sts-data");
        let pony_data = ctx.home.join("pony-data");
        let sid = "00000000-0000-4000-8000-000000000001";
        let other = "00000000-0000-4000-8000-000000000002";
        fn hook(ctx: &Context, script: &Path, data: &Path, input: Value) -> String {
            let mut child = Command::new(node().unwrap())
                .arg(script)
                .env_clear()
                .env("PATH", "/usr/bin:/bin:/opt/homebrew/bin")
                .env("HOME", &ctx.home)
                .env("XDG_CONFIG_HOME", ctx.home.join("config"))
                .env("PLUGIN_DATA", data)
                .env("CLAUDE_CONFIG_DIR", ctx.home.join("claude"))
                .env("PONYTAIL_MAP", "0")
                .current_dir(ctx.home.join("work"))
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();
            child
                .stdin
                .take()
                .unwrap()
                .write_all(&serde_json::to_vec(&input).unwrap())
                .unwrap();
            let output = child.wait_with_output().unwrap();
            assert!(output.status.success(), "isolated hook exited with error");
            assert!(output.stderr.is_empty(), "isolated hook reported an error");
            String::from_utf8(output.stdout).unwrap()
        }
        for mode in [
            PonyMode::Lite,
            PonyMode::Full,
            PonyMode::Ultra,
            PonyMode::Off,
        ] {
            let current = ctx.pony().unwrap();
            let saved = ctx.pony_save(Some(mode), &current.revision).unwrap();
            assert_eq!(saved.mode, mode);
            let output = hook(
                &ctx,
                &pony_root.join("hooks/ponytail-activate.js"),
                &pony_data,
                json!({"hook_event_name":"SessionStart","session_id":sid}),
            );
            if mode == PonyMode::Off {
                assert!(output.is_empty());
                assert!(!pony_data.join(".ponytail-active").exists());
            } else {
                let value = serde_json::to_value(mode).unwrap();
                assert!(output.contains(&format!(
                    "PONYTAIL MODE ACTIVE — level: {}",
                    value.as_str().unwrap()
                )));
                assert_eq!(
                    fs::read_to_string(pony_data.join(".ponytail-active")).unwrap(),
                    value.as_str().unwrap()
                );
            }
        }
        let initial = json!({"schemaVersion":4,"contract":{"mode":"unconfirmed","level":"watch","agentBudget":3,"hashPolicy":"deny","allowedPaths":null,"dependencyPolicy":"ask","source":"default"},"delegation":{"reservations":{},"agentIdsSeen":[],"stoppedAgentIds":[],"unresolved":{},"acceptedActions":{"preserved-action":1}},"directiveWarning":null,"directiveError":null,"lastPromptContext":"old context"});
        fs::create_dir_all(sts_data.join("sessions")).unwrap();
        for id in [sid, other] {
            fs::write(
                state_file(&sts_data, id),
                serde_json::to_vec(&initial).unwrap(),
            )
            .unwrap();
        }
        let untouched = fs::read(state_file(&sts_data, other)).unwrap();
        let installation = crate::plugins::ModeInstallation {
            source: sts_root.clone(),
            runtime_data: sts_data.clone(),
            enabled: true,
        };
        let status = || adapter(sts_request(&ctx, &installation, sid, "status")).unwrap();
        let apply_mode = |mode: &str, level: &str, allowed: Option<Vec<&str>>| {
            let mut request = sts_request(&ctx, &installation, sid, "apply");
            request["expectedRevision"] = json!(status().revision);
            request["mode"] = json!(mode);
            request["level"] = json!(level);
            request["allowedPaths"] = json!(allowed);
            adapter(request).unwrap()
        };
        let saved = apply_mode("review", "guard", None);
        let mut rollback = sts_request(&ctx, &installation, sid, "rollback");
        rollback["expectedRevision"] = json!(saved.revision);
        let restored = adapter(rollback).unwrap();
        assert_eq!(serde_json::to_value(restored.mode).unwrap(), "unconfirmed");
        let script = sts_root.join("hooks/stop-that-shit.cjs");
        let check_write = |file: &str| {
            let output = hook(
                &ctx,
                &script,
                &sts_data,
                json!({"hook_event_name":"PreToolUse","session_id":sid,"tool_name":"apply_patch","tool_use_id":"fixture-write","tool_input":{"command":format!("*** Begin Patch\n*** Add File: {file}\n+fixture\n*** End Patch")},"cwd":ctx.home.join("work")}),
            );
            if output.trim().is_empty() {
                Value::Null
            } else {
                serde_json::from_str::<Value>(&output).unwrap()
            }
        };
        for mode in ["answer", "review", "monitor", "change", "open"] {
            let saved = apply_mode(mode, "guard", None);
            let raw: Value =
                serde_json::from_slice(&fs::read(state_file(&sts_data, sid)).unwrap()).unwrap();
            assert!(raw["lastPromptContext"].is_null());
            assert_eq!(raw["delegation"]["acceptedActions"]["preserved-action"], 1);
            let output = hook(
                &ctx,
                &script,
                &sts_data,
                json!({"hook_event_name":"UserPromptSubmit","session_id":sid,"prompt":"Continue this task."}),
            );
            assert!(
                output.contains(mode),
                "next real prompt did not receive the selected mode"
            );
            let response = check_write("result.txt");
            let denied = response["hookSpecificOutput"]["permissionDecision"] == "deny";
            assert_eq!(denied, ["answer", "review", "monitor"].contains(&mode));
            let mut stale = sts_request(&ctx, &installation, sid, "apply");
            stale["expectedRevision"] = json!(saved.revision);
            stale["mode"] = json!("change");
            stale["level"] = json!("guard");
            assert!(
                adapter(stale).is_err(),
                "a real hook updated the version; stale UI must not overwrite it"
            );
        }
        apply_mode("change", "lock", Some(vec!["allowed.txt"]));
        assert_eq!(
            check_write("outside.txt")["hookSpecificOutput"]["permissionDecision"],
            "deny"
        );
        assert!(check_write("allowed.txt").is_null());
        apply_mode("review", "watch", None);
        assert_ne!(
            check_write("outside.txt")["hookSpecificOutput"]["permissionDecision"],
            "deny"
        );
        apply_mode("review", "off", None);
        assert!(check_write("outside.txt").is_null());
        let before_failed_backup = fs::read(state_file(&sts_data, sid)).unwrap();
        let broken_backup = ctx.home.join("not-a-directory");
        fs::write(&broken_backup, "fixture").unwrap();
        let mut request = sts_request(&ctx, &installation, sid, "apply");
        request["backupDir"] = json!(broken_backup);
        request["expectedRevision"] = json!(status().revision);
        request["mode"] = json!("change");
        request["level"] = json!("guard");
        assert!(adapter(request).is_err());
        assert_eq!(
            fs::read(state_file(&sts_data, sid)).unwrap(),
            before_failed_backup
        );
        assert_eq!(fs::read(state_file(&sts_data, other)).unwrap(), untouched);
        assert!(!ctx.home.join("work/outside.txt").exists());
        let list=ctx.rpc().unwrap().call("thread/list",json!({"limit":100,"archived":false,"sourceKinds":["appServer"],"sortKey":"updated_at","useStateDbOnly":true})).unwrap();
        assert!(list["data"].as_array().unwrap().is_empty());
        println!("Ponytail: lite/full/ultra/off actual SessionStart passed; STS: five task modes, watch/guard/lock/off, next-prompt context, rollback, stale revision, preserved ledger and second-session isolation passed. Real Desktop was not used.");
        let _ = fs::remove_dir_all(ctx.home);
    }
}
