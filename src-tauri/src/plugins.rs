use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::{mpsc, Mutex},
    time::{Duration, Instant},
};
use tauri::Manager;

static OPERATIONS: Mutex<()> = Mutex::new(());
const MARKETPLACE: &str = "atelier-companions";

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum PluginId {
    Ponytail,
    StopThatShit,
}
impl PluginId {
    fn key(self) -> &'static str {
        match self {
            Self::Ponytail => "ponytail",
            Self::StopThatShit => "stop-that-shit",
        }
    }
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition {
    id: PluginId,
    name: String,
    repository: String,
    commit: String,
    tag: String,
    version: String,
    summary: String,
    description: String,
    license: String,
}
fn catalog() -> Vec<Definition> {
    serde_json::from_str(include_str!("../../integrations/behavior-plugins.json"))
        .expect("bundled plugin catalog")
}
fn definition(id: PluginId) -> Definition {
    catalog().into_iter().find(|d| d.id == id).unwrap()
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PluginStatus {
    #[serde(flatten)]
    definition: Definition,
    installed: Option<bool>,
    enabled: Option<bool>,
    installed_version: Option<String>,
    plugin_id: String,
    duplicates: bool,
    can_rollback: bool,
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    available: bool,
    reason: Option<String>,
    host_version: Option<String>,
    revision: String,
    config_target: String,
    plugins: Vec<PluginStatus>,
}
#[derive(Serialize, Deserialize)]
struct Rollback {
    id: PluginId,
    plugin_id: String,
    before: bool,
    after_version: String,
}

struct Process(Child);
impl Drop for Process {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
struct Context {
    binary: PathBuf,
    data: PathBuf,
    home: PathBuf,
    isolated: bool,
}
impl Context {
    fn command(&self, args: &[&str]) -> Command {
        let mut cmd = Command::new(&self.binary);
        cmd.args(args).current_dir(&self.home);
        if self.isolated {
            cmd.env("CODEX_HOME", &self.home);
        }
        // Finder does not inherit nvm's shell setup. Use the runtime next to Codex.
        let mut paths = vec![self.binary.parent().unwrap().to_path_buf()];
        paths.extend(std::env::split_paths(
            &std::env::var_os("PATH").unwrap_or_default(),
        ));
        paths.extend([
            PathBuf::from("/usr/bin"),
            PathBuf::from("/bin"),
            PathBuf::from("/opt/homebrew/bin"),
        ]);
        if let Ok(path) = std::env::join_paths(paths) {
            cmd.env("PATH", path);
        }
        cmd
    }
    fn cli(&self, args: &[&str]) -> Result<Value, String> {
        let output = run(self.command(args), Duration::from_secs(90))?;
        serde_json::from_slice(&output)
            .map_err(|_| "Codex 未返回预期 JSON，请更新 CLI 后重试。".into())
    }
    fn rpc(&self) -> Result<Rpc, String> {
        Rpc::start(self.command(&["app-server", "--stdio"]))
    }
    fn rollback_path(&self, id: PluginId) -> PathBuf {
        self.data.join(format!("rollback-{}.json", id.key()))
    }
    fn snapshot(&self) -> Result<Snapshot, String> {
        let mut rpc = self.rpc()?;
        let config = rpc.call("config/read", json!({"includeLayers":true}))?;
        let revision = user_revision(&config)?;
        let installed = self.cli(&["plugin", "list", "--json"])?;
        let entries = installed
            .get("installed")
            .and_then(Value::as_array)
            .ok_or("插件清单格式不兼容。")?;
        let plugins = catalog()
            .into_iter()
            .map(|d| {
                let own = format!("{}@{}", d.id.key(), MARKETPLACE);
                let upstream = format!("{}@{}", d.id.key(), d.id.key());
                let matching: Vec<_> = entries
                    .iter()
                    .filter(|p| p["pluginId"] == own || p["pluginId"] == upstream)
                    .collect();
                let entry = matching
                    .iter()
                    .find(|p| p["pluginId"] == own)
                    .or(matching.first());
                let key = entry
                    .and_then(|p| p["pluginId"].as_str())
                    .unwrap_or(&own)
                    .to_string();
                let can_rollback = fs::read(self.rollback_path(d.id))
                    .ok()
                    .and_then(|bytes| serde_json::from_slice::<Rollback>(&bytes).ok())
                    .is_some_and(|b| {
                        b.id == d.id && b.after_version == revision && b.plugin_id == key
                    });
                PluginStatus {
                    installed: Some(entry.is_some()),
                    enabled: Some(entry.is_some_and(|p| p["enabled"].as_bool() == Some(true))),
                    installed_version: entry.and_then(|p| p["version"].as_str()).map(String::from),
                    plugin_id: key,
                    duplicates: matching.len() > 1,
                    can_rollback,
                    definition: d,
                }
            })
            .collect();
        Ok(Snapshot {
            available: true,
            reason: None,
            host_version: Some(version(self)?),
            revision,
            config_target: "Codex 用户级插件配置".into(),
            plugins,
        })
    }
    fn set(&self, id: PluginId, enabled: bool, expected: &str) -> Result<Snapshot, String> {
        let before = self.snapshot()?;
        if before.revision != expected {
            return Err("Codex 配置已被外部修改。请刷新状态后重新操作。".into());
        }
        let plugin = before
            .plugins
            .iter()
            .find(|p| p.definition.id == id)
            .unwrap();
        if plugin.installed != Some(true) {
            return Err("请先安装插件。".into());
        }
        if plugin.duplicates {
            return Err("检测到重复安装。请先在 Codex 插件管理中保留一个来源，再刷新。".into());
        }
        if plugin.enabled == Some(enabled) {
            return Ok(before);
        }
        fs::create_dir_all(&self.data).map_err(|_| "无法创建插件回退记录目录。")?;
        let pending = self.data.join(format!("pending-{}.json", id.key()));
        private_write(&pending, &serde_json::to_vec(&json!({"id":id,"pluginId":plugin.plugin_id,"before":plugin.enabled,"beforeVersion":expected})).unwrap())?;
        let mut rpc = self.rpc()?;
        let result = write_enabled(&mut rpc, &plugin.plugin_id, enabled, expected)?;
        let after_version = result["version"]
            .as_str()
            .ok_or("Codex 写入未返回版本号。")?
            .to_string();
        let backup = Rollback {
            id,
            plugin_id: plugin.plugin_id.clone(),
            before: plugin.enabled.unwrap_or(false),
            after_version,
        };
        // Backup only the previous boolean and official version token, never the full config.
        fs::create_dir_all(&self.data).map_err(|_| "无法创建插件回退记录目录。")?;
        private_write(
            &self.rollback_path(id),
            &serde_json::to_vec(&backup).unwrap(),
        )?;
        let _ = fs::remove_file(pending);
        let after = self.snapshot()?;
        let verified = after
            .plugins
            .iter()
            .find(|p| p.definition.id == id)
            .unwrap();
        if result["status"] != "ok" || verified.enabled != Some(enabled) {
            return Err("配置已写入，但有效状态未匹配（可能受项目或管理策略覆盖）。请刷新或回退；未确认会话生效。".into());
        }
        Ok(after)
    }
    fn rollback(&self, id: PluginId, expected: &str) -> Result<Snapshot, String> {
        let backup: Rollback = serde_json::from_slice(
            &fs::read(self.rollback_path(id)).map_err(|_| "没有可用回退记录。")?,
        )
        .map_err(|_| "回退记录损坏。")?;
        let current = self.snapshot()?;
        if backup.id != id || current.revision != expected || expected != backup.after_version {
            return Err("配置在上次修改后已发生变化，不能覆盖回退。请刷新后手动切换。".into());
        }
        let current_plugin = current
            .plugins
            .iter()
            .find(|p| p.definition.id == id)
            .unwrap();
        if current_plugin.plugin_id != backup.plugin_id || current_plugin.duplicates {
            return Err("插件来源已变化，停止回退。".into());
        }
        self.set(id, backup.before, expected)
    }
    fn install(&self, id: PluginId, expected: &str) -> Result<Snapshot, String> {
        let before = self.snapshot()?;
        if before.revision != expected {
            return Err("配置发生变化，请刷新后再安装。".into());
        }
        if before
            .plugins
            .iter()
            .find(|p| p.definition.id == id)
            .unwrap()
            .installed
            == Some(true)
        {
            return Err("该插件已安装。请刷新状态，使用加载开关。".into());
        }
        let d = definition(id);
        let root = self.data.join("marketplace");
        fs::create_dir_all(root.join(".agents/plugins")).map_err(|_| "无法创建本地插件市场。")?;
        let source = root.join(id.key());
        if !source.exists() {
            let temp = root.join(format!("{}.download", id.key()));
            if temp.exists() {
                fs::remove_dir_all(&temp)
                    .map_err(|_| "上次下载未完成，请清理 Atelier 插件下载目录。")?;
            }
            let mut git = Command::new("/usr/bin/git");
            git.args([
                "-c",
                "core.hooksPath=/dev/null",
                "clone",
                "--depth",
                "1",
                "--branch",
                &d.tag,
                "--single-branch",
                "--",
                &format!("https://github.com/{}.git", d.repository),
            ])
            .arg(&temp)
            .env("GIT_TERMINAL_PROMPT", "0");
            run(git, Duration::from_secs(180))?;
            verify_source(&temp, &d)?;
            fs::rename(temp, &source).map_err(|_| "无法保存插件下载。")?;
        }
        verify_source(&source, &d)?;
        let entries:Vec<_>=catalog().into_iter().filter(|p|root.join(p.id.key()).exists()).map(|p|json!({"name":p.id.key(),"source":{"source":"local","path":format!("./{}",p.id.key())},"policy":{"installation":"AVAILABLE","authentication":"ON_INSTALL"},"category":"Productivity"})).collect();
        private_write(&root.join(".agents/plugins/marketplace.json"), &serde_json::to_vec_pretty(&json!({"name":MARKETPLACE,"interface":{"displayName":"Codex Atelier · 行为插件"},"plugins":entries})).unwrap())?;
        let latest = self.snapshot()?;
        if latest.revision != expected {
            return Err("下载期间 Codex 配置已变化，请刷新后重试安装。".into());
        }
        self.cli(&[
            "plugin",
            "marketplace",
            "add",
            root.to_str().ok_or("插件路径编码不支持。")?,
            "--json",
        ])?;
        self.cli(&[
            "plugin",
            "add",
            &format!("{}@{}", id.key(), MARKETPLACE),
            "--json",
        ])?;
        // Installation does not approve Hook trust. Leave new plugins disabled until explicitly toggled.
        let installed = self.snapshot()?;
        self.set(id, false, &installed.revision)
    }
}
fn verify_source(path: &Path, d: &Definition) -> Result<(), String> {
    let mut rev = Command::new("/usr/bin/git");
    rev.arg("-C").arg(path).args(["rev-parse", "HEAD"]);
    let actual =
        String::from_utf8(run(rev, Duration::from_secs(10))?).map_err(|_| "无法核对下载版本。")?;
    if actual.trim() != d.commit {
        return Err("上游提交与固定版本不匹配，未安装。".into());
    }
    let mut status = Command::new("/usr/bin/git");
    status
        .arg("-C")
        .arg(path)
        .args(["status", "--porcelain", "--untracked-files=normal"]);
    if !run(status, Duration::from_secs(10))?.is_empty() {
        return Err("插件下载目录被外部修改，停止安装。请移走该目录后重试。".into());
    }
    let manifest: Value = serde_json::from_slice(
        &fs::read(path.join(".codex-plugin/plugin.json"))
            .map_err(|_| "上游缺少 Codex 插件声明。")?,
    )
    .map_err(|_| "上游插件声明格式错误。")?;
    if manifest["name"] != d.id.key() || manifest["version"] != d.version {
        return Err("插件声明与固定版本不一致，未安装。".into());
    }
    Ok(())
}

fn user_revision(config: &Value) -> Result<String, String> {
    config["layers"]
        .as_array()
        .and_then(|layers| {
            layers
                .iter()
                .find(|l| l["name"]["type"] == "user" && l["name"]["profile"].is_null())
        })
        .and_then(|l| l["version"].as_str())
        .map(String::from)
        .ok_or("Codex 未返回用户配置版本，无法安全修改。".into())
}
fn write_enabled(
    rpc: &mut Rpc,
    plugin: &str,
    enabled: bool,
    version: &str,
) -> Result<Value, String> {
    let allowed = catalog().iter().any(|d| {
        plugin == format!("{}@{}", d.id.key(), MARKETPLACE)
            || plugin == format!("{}@{}", d.id.key(), d.id.key())
    });
    if !allowed {
        return Err("不允许修改此插件。".into());
    }
    rpc.call("config/value/write",json!({"keyPath":format!("plugins.\"{}\".enabled",plugin),"value":enabled,"mergeStrategy":"replace","expectedVersion":version}))
}
fn private_write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    use std::fs::OpenOptions;
    let temp = path.with_extension("tmp");
    let mut opts = OpenOptions::new();
    opts.create(true).truncate(true).write(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opts.mode(0o600);
    }
    let mut f = opts.open(&temp).map_err(|_| "无法写入插件回退记录。")?;
    f.write_all(bytes)
        .and_then(|_| f.sync_all())
        .map_err(|_| "无法保存插件回退记录。")?;
    fs::rename(temp, path).map_err(|_| "无法完成插件记录写入。".into())
}
fn run(mut cmd: Command, timeout: Duration) -> Result<Vec<u8>, String> {
    let mut process = Process(
        cmd.stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| "无法启动 Codex 或 Git，请检查本机安装。")?,
    );
    let stdout = process.0.stdout.take().unwrap();
    let reader = std::thread::spawn(move || {
        let mut bytes = Vec::new();
        let _ = stdout.take(8 * 1024 * 1024).read_to_end(&mut bytes);
        bytes
    });
    let start = Instant::now();
    loop {
        match process.0.try_wait() {
            Ok(Some(status)) => {
                let bytes = reader.join().map_err(|_| "读取子进程失败。")?;
                return if status.success() {
                    Ok(bytes)
                } else {
                    Err(format!(
                        "Codex/Git 操作失败（退出码 {:?}）。未确认成功，请刷新；安装请检查网络。",
                        status.code()
                    ))
                };
            }
            Ok(None) => {}
            Err(_) => return Err("无法读取子进程状态。".into()),
        }
        if start.elapsed() > timeout {
            return Err("操作超时，已停止子进程。请刷新状态后重试。".into());
        }
        std::thread::sleep(Duration::from_millis(30));
    }
}
struct Rpc {
    process: Process,
    lines: mpsc::Receiver<Value>,
    next: u64,
}
impl Rpc {
    fn start(mut cmd: Command) -> Result<Self, String> {
        let mut process = Process(
            cmd.stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
                .map_err(|_| "无法启动 Codex 配置接口。")?,
        );
        let stdout = process.0.stdout.take().unwrap();
        let (tx, lines) = mpsc::channel();
        std::thread::spawn(move || {
            for line in BufReader::new(stdout.take(16 * 1024 * 1024)).lines() {
                let Ok(line) = line else { break };
                if let Ok(v) = serde_json::from_str(&line) {
                    if tx.send(v).is_err() {
                        break;
                    }
                }
            }
        });
        let mut rpc = Self {
            process,
            lines,
            next: 0,
        };
        rpc.call(
            "initialize",
            json!({"clientInfo":{"name":"codex_atelier","version":"0.1.0"}}),
        )?;
        rpc.send(json!({"method":"initialized"}))?;
        Ok(rpc)
    }
    fn send(&mut self, v: Value) -> Result<(), String> {
        let input = self.process.0.stdin.as_mut().ok_or("配置接口已关闭。")?;
        writeln!(input, "{}", v)
            .and_then(|_| input.flush())
            .map_err(|_| "配置接口通信失败。".into())
    }
    fn call(&mut self, method: &str, params: Value) -> Result<Value, String> {
        self.next += 1;
        let id = self.next;
        self.send(json!({"id":id,"method":method,"params":params}))?;
        let deadline = Instant::now() + Duration::from_secs(20);
        loop {
            let v = self
                .lines
                .recv_timeout(deadline.saturating_duration_since(Instant::now()))
                .map_err(|_| "Codex 配置接口超时或退出。")?;
            if v["id"] == id {
                if v.get("error").is_some() {
                    return Err("Codex 拒绝配置操作：版本冲突或接口不兼容，请刷新状态。".into());
                }
                return v
                    .get("result")
                    .cloned()
                    .ok_or("配置接口响应不完整。".into());
            }
        }
    }
}
fn find_codex() -> Option<PathBuf> {
    let mut bins: Vec<PathBuf> =
        std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
            .map(|p| p.join("codex"))
            .collect();
    bins.extend([
        PathBuf::from("/opt/homebrew/bin/codex"),
        PathBuf::from("/usr/local/bin/codex"),
    ]);
    if let Some(home) = std::env::var_os("HOME") {
        if let Ok(entries) = fs::read_dir(PathBuf::from(home).join(".nvm/versions/node")) {
            let mut entries: Vec<_> = entries
                .flatten()
                .map(|e| e.path().join("bin/codex"))
                .collect();
            entries.sort();
            entries.reverse();
            bins.extend(entries)
        }
    }
    bins.into_iter().find(|p| p.is_file())
}
fn context(app: &tauri::AppHandle) -> Result<Context, String> {
    Ok(Context {
        binary: find_codex().ok_or("未找到 Codex CLI。请先安装 CLI；Atelier 不会自动升级环境。")?,
        data: app
            .path()
            .app_data_dir()
            .map_err(|_| "无法取得应用数据目录。")?
            .join("behavior-plugins"),
        home: std::env::var_os("HOME")
            .map(PathBuf::from)
            .ok_or("无法取得用户目录。")?,
        isolated: false,
    })
}
fn version(ctx: &Context) -> Result<String, String> {
    String::from_utf8(run(ctx.command(&["--version"]), Duration::from_secs(10))?)
        .map(|s| s.trim().into())
        .map_err(|_| "无法读取 CLI 版本。".into())
}
fn unavailable(reason: String) -> Snapshot {
    Snapshot {
        available: false,
        reason: Some(reason),
        host_version: None,
        revision: String::new(),
        config_target: "Codex 用户级插件配置".into(),
        plugins: catalog()
            .into_iter()
            .map(|d| PluginStatus {
                plugin_id: format!("{}@{}", d.id.key(), MARKETPLACE),
                definition: d,
                installed: None,
                enabled: None,
                installed_version: None,
                duplicates: false,
                can_rollback: false,
            })
            .collect(),
    }
}
#[tauri::command]
pub async fn behavior_plugins_status(app: tauri::AppHandle) -> Snapshot {
    tauri::async_runtime::spawn_blocking(move || {
        let _lock = OPERATIONS.lock().unwrap();
        match context(&app).and_then(|c| c.snapshot()) {
            Ok(s) => s,
            Err(e) => unavailable(e),
        }
    })
    .await
    .unwrap_or_else(|_| unavailable("插件管理任务异常退出。".into()))
}
#[tauri::command]
pub async fn behavior_plugin_install(
    app: tauri::AppHandle,
    id: PluginId,
    expected_revision: String,
) -> Result<Snapshot, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let _lock = OPERATIONS.lock().map_err(|_| "插件操作锁不可用。")?;
        context(&app)?.install(id, &expected_revision)
    })
    .await
    .map_err(|_| "插件安装任务异常。")?
}
#[tauri::command]
pub async fn behavior_plugin_set(
    app: tauri::AppHandle,
    id: PluginId,
    enabled: bool,
    expected_revision: String,
) -> Result<Snapshot, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let _lock = OPERATIONS.lock().map_err(|_| "插件操作锁不可用。")?;
        context(&app)?.set(id, enabled, &expected_revision)
    })
    .await
    .map_err(|_| "插件切换任务异常。")?
}
#[tauri::command]
pub async fn behavior_plugin_rollback(
    app: tauri::AppHandle,
    id: PluginId,
    expected_revision: String,
) -> Result<Snapshot, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let _lock = OPERATIONS.lock().map_err(|_| "插件操作锁不可用。")?;
        context(&app)?.rollback(id, &expected_revision)
    })
    .await
    .map_err(|_| "插件回退任务异常。")?
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_arbitrary_plugin_ids() {
        assert!(serde_json::from_str::<PluginId>("\"../../config\"").is_err());
        assert!(serde_json::from_str::<PluginId>("\"unknown\"").is_err());
    }
    #[test]
    fn catalog_is_fixed_and_pinned() {
        let defs = catalog();
        assert_eq!(defs.len(), 2);
        assert_eq!(definition(PluginId::Ponytail).tag, "v5.3.2");
        assert_eq!(
            definition(PluginId::StopThatShit).repository,
            "lennney/stop-that-shit"
        );
    }
    #[test]
    fn missing_config_version_is_not_writable() {
        assert!(user_revision(&json!({"layers":[]})).is_err());
    }
    #[test]
    #[ignore = "uses real Codex CLI with isolated CODEX_HOME; no models or real user config"]
    fn isolated_codex_roundtrip() {
        let root = std::env::temp_dir().join(format!("atelier-plugin-test-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let ctx = Context {
            binary: find_codex().expect("Codex installed"),
            data: root.join("data"),
            home: root.join("codex"),
            isolated: true,
        };
        fs::create_dir_all(&ctx.home).unwrap();
        fs::write(
            ctx.home.join("config.toml"),
            "# preserve me\nmodel = \"example-model\"\n",
        )
        .unwrap();
        let market = root.join("fixtures");
        fs::create_dir_all(market.join(".agents/plugins")).unwrap();
        let mut entries = vec![];
        for id in [PluginId::Ponytail, PluginId::StopThatShit] {
            let p = market.join(id.key());
            fs::create_dir_all(p.join(".codex-plugin")).unwrap();
            fs::write(
                p.join(".codex-plugin/plugin.json"),
                json!({"name":id.key(),"version":"0.0.1","description":"isolated test fixture"})
                    .to_string(),
            )
            .unwrap();
            entries.push(json!({"name":id.key(),"source":{"source":"local","path":format!("./{}",id.key())}}));
        }
        fs::write(
            market.join(".agents/plugins/marketplace.json"),
            json!({"name":MARKETPLACE,"plugins":entries}).to_string(),
        )
        .unwrap();
        ctx.cli(&[
            "plugin",
            "marketplace",
            "add",
            market.to_str().unwrap(),
            "--json",
        ])
        .unwrap();
        for id in [PluginId::Ponytail, PluginId::StopThatShit] {
            ctx.cli(&[
                "plugin",
                "add",
                &format!("{}@{}", id.key(), MARKETPLACE),
                "--json",
            ])
            .unwrap();
        }
        let s = ctx.snapshot().unwrap();
        assert!(s.plugins.iter().all(|p| p.installed == Some(true)));
        let off = ctx.set(PluginId::Ponytail, false, &s.revision).unwrap();
        assert_eq!(off.plugins[0].enabled, Some(false));
        assert!(ctx.set(PluginId::StopThatShit, false, &s.revision).is_err());
        let restored = ctx.rollback(PluginId::Ponytail, &off.revision).unwrap();
        assert_eq!(restored.plugins[0].enabled, Some(true));
        let off = ctx
            .set(PluginId::StopThatShit, false, &restored.revision)
            .unwrap();
        assert_eq!(off.plugins[1].enabled, Some(false));
        let content = fs::read_to_string(ctx.home.join("config.toml")).unwrap();
        assert!(content.contains("# preserve me"));
        assert!(content.contains("example-model"));
        let mut altered = content;
        altered = altered.replace("example-model", "external-model");
        fs::write(ctx.home.join("config.toml"), altered).unwrap();
        assert!(ctx.rollback(PluginId::StopThatShit, &off.revision).is_err());
        let _ = fs::remove_dir_all(root);
    }
}
