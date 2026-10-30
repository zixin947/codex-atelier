use crate::desktop_bridge::{self, BridgeStatus};
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
    hook_trust: HookTrust,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
enum HookTrustState {
    Trusted,
    ReviewRequired,
    NotLoaded,
    NotInstalled,
    Unavailable,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct HookTrust {
    state: HookTrustState,
    total: usize,
    trusted: usize,
    managed: usize,
    untrusted: usize,
    modified: usize,
    disabled: usize,
    reason: Option<String>,
    hooks: Vec<HookDetail>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct HookDetail {
    key: Option<String>,
    event_name: String,
    trust_status: String,
    enabled: bool,
    source_path: Option<String>,
    handler_type: String,
    command: Option<String>,
    mcp_target: Option<String>,
    matcher: Option<String>,
}
impl HookTrust {
    fn unknown(reason: &str) -> Self {
        Self {
            state: HookTrustState::Unavailable,
            total: 0,
            trusted: 0,
            managed: 0,
            untrusted: 0,
            modified: 0,
            disabled: 0,
            reason: Some(reason.into()),
            hooks: Vec::new(),
        }
    }
}

// The host owns definition hashing and persisted trust. Never infer trust from
// installation/enabled state, paths, command text, or our own cached records.
fn hook_trust(
    response: &Result<Value, String>,
    plugin_id: &str,
    installed: bool,
    enabled: bool,
    duplicates: bool,
) -> HookTrust {
    if !installed {
        return HookTrust {
            state: HookTrustState::NotInstalled,
            ..HookTrust::unknown("安装后可读取 Hook 信任状态。")
        };
    }
    if duplicates {
        return HookTrust::unknown("插件来源重复，请先保留一个来源，再刷新检测。");
    }
    let Ok(response) = response else {
        return HookTrust::unknown(
            "Hook 读取失败或 CLI 不支持 hooks/list；请重试或在 CLI /hooks 中核对。",
        );
    };
    let Some(entries) = response["data"].as_array().filter(|v| v.len() == 1) else {
        return HookTrust::unknown("Hook 接口返回格式不兼容，未判断信任。");
    };
    let entry = &entries[0];
    // Incomplete discovery must not become a green 'all trusted' assertion.
    if !entry["errors"].as_array().is_some_and(Vec::is_empty)
        || !entry["warnings"].as_array().is_some_and(Vec::is_empty)
    {
        return HookTrust::unknown(
            "Codex 报告 Hook 发现警告或错误，结果不完整；请在 CLI /hooks 中核对。",
        );
    }
    let Some(hooks) = entry["hooks"].as_array() else {
        return HookTrust::unknown("Hook 清单格式不兼容，未判断信任。");
    };
    let mut trust = HookTrust {
        state: HookTrustState::NotLoaded,
        reason: None,
        ..HookTrust::unknown("")
    };
    for hook in hooks.iter().filter(|h| h["pluginId"] == plugin_id) {
        let Some(enabled) = hook["enabled"].as_bool() else {
            return HookTrust::unknown("Hook 状态字段不兼容，未判断信任。");
        };
        match hook["trustStatus"].as_str() {
            Some("trusted") => trust.trusted += 1,
            Some("managed") => trust.managed += 1,
            Some("untrusted") => trust.untrusted += 1,
            Some("modified") => trust.modified += 1,
            _ => return HookTrust::unknown("CLI 返回未知的 Hook 信任状态，未判断信任。"),
        }
        trust.total += 1;
        trust.disabled += usize::from(!enabled);
        // Return only display fields belonging to this exact managed plugin.
        // Command strings are review material, never execution input.
        trust.hooks.push(HookDetail {
            key: hook["key"].as_str().map(String::from),
            event_name: hook["eventName"].as_str().unwrap_or("unknown").into(),
            trust_status: hook["trustStatus"].as_str().unwrap().into(),
            enabled,
            source_path: hook["sourcePath"].as_str().map(String::from),
            handler_type: hook["handlerType"].as_str().unwrap_or("unknown").into(),
            command: hook["command"].as_str().map(String::from),
            mcp_target: hook["server"]
                .as_str()
                .zip(hook["tool"].as_str())
                .map(|(server, tool)| format!("{server}/{tool}")),
            matcher: hook["matcher"].as_str().map(String::from),
        });
    }
    if trust.total == 0 {
        trust.reason = Some(
            if enabled {
                "未发现此来源的可加载 Hook，不能据此判断已信任或未信任。"
            } else {
                "插件已停用，CLI 未枚举其 Hook；保留原信任状态，不自动启用检测。"
            }
            .into(),
        );
    } else if trust.untrusted + trust.modified > 0 {
        trust.state = HookTrustState::ReviewRequired;
    } else {
        trust.state = HookTrustState::Trusted;
    }
    trust
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
    desktop_bridge: BridgeStatus,
    runtime_receipt: Option<String>,
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
        self.cli_with_timeout(args, Duration::from_secs(90))
    }
    fn cli_with_timeout(&self, args: &[&str], timeout: Duration) -> Result<Value, String> {
        let output = run(self.command(args), timeout)?;
        serde_json::from_slice(&output)
            .map_err(|_| "Codex 未返回预期 JSON，请更新 Desktop 后重试。".into())
    }
    fn installed_plugins(&self) -> Result<Vec<Value>, String> {
        // Unfiltered `plugin list` also queries unrelated remote marketplaces.
        // Loading switches must depend only on our three known source names.
        let mut entries = Vec::new();
        for market in [MARKETPLACE, "ponytail", "stop-that-shit"] {
            let list = self
                .cli_with_timeout(
                    &["plugin", "list", "--marketplace", market, "--json"],
                    Duration::from_secs(8),
                )
                .map_err(|_| {
                    format!("无法读取受管插件市场 {market}，最新状态未确认。请刷新后重试。")
                })?;
            let rows = list["installed"].as_array().ok_or("插件清单格式不兼容。")?;
            entries.extend(rows.iter().cloned());
        }
        Ok(entries)
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
        let entries = self.installed_plugins()?;
        // Discovery only: no thread/start, model turn, Hook execution, or trust write.
        // A missing/failed method must not disable installation and loading controls.
        let hooks = rpc.call("hooks/list", json!({"cwds":[self.home]}));
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
                    hook_trust: hook_trust(
                        &hooks,
                        &key,
                        entry.is_some(),
                        entry.is_some_and(|p| p["enabled"].as_bool() == Some(true)),
                        matching.len() > 1,
                    ),
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
            desktop_bridge: if self.isolated {
                BridgeStatus::disconnected()
            } else {
                desktop_bridge::status(&self.codex_home())
            },
            runtime_receipt: None,
        })
    }
    fn codex_home(&self) -> PathBuf {
        if self.isolated {
            self.home.clone()
        } else {
            std::env::var_os("CODEX_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| self.home.join(".codex"))
        }
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
        #[cfg(unix)]
        let mut owners = if self.isolated {
            Vec::new()
        } else {
            desktop_bridge::owners(&self.codex_home()).unwrap_or_default()
        };
        #[cfg(unix)]
        let connected_threads: usize = owners.iter().map(|h| h.threads).sum();
        #[cfg(unix)]
        let connected = !owners.is_empty();
        #[cfg(not(unix))]
        let connected = false;
        fs::create_dir_all(&self.data).map_err(|_| "无法创建插件回退记录目录。")?;
        let pending = self.data.join(format!("pending-{}.json", id.key()));
        private_write(&pending, &serde_json::to_vec(&json!({"id":id,"pluginId":plugin.plugin_id,"before":plugin.enabled,"beforeVersion":expected})).unwrap())?;
        #[cfg(unix)]
        let mut rpc = if connected {
            Rpc::from_owner(owners.remove(0))
        } else {
            self.rpc()?
        };
        #[cfg(not(unix))]
        let mut rpc = self.rpc()?;
        let result = if connected {
            rpc.call("config/batchWrite", json!({"edits":[{"keyPath":format!("plugins.\"{}\".enabled",plugin.plugin_id),"value":enabled,"mergeStrategy":"replace"}],"expectedVersion":expected,"reloadUserConfig":true}))?
        } else {
            write_enabled(&mut rpc, &plugin.plugin_id, enabled, expected)?
        };
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
        #[cfg(unix)]
        for owner in &mut owners {
            owner.call("config/batchWrite", json!({"edits":[],"expectedVersion":backup.after_version,"reloadUserConfig":true}))
                .map_err(|_| "配置已保存，但部分 Desktop 宿主未确认热重载。请刷新；不能视为所有当前会话已停用。")?;
        }
        let mut after = self.snapshot()?;
        let verified = after
            .plugins
            .iter()
            .find(|p| p.definition.id == id)
            .unwrap();
        if result["status"] != "ok" || verified.enabled != Some(enabled) {
            return Err("配置已写入，但有效状态未匹配（可能受项目或管理策略覆盖）。请刷新或回退；未确认会话生效。".into());
        }
        #[cfg(unix)]
        if connected {
            after.runtime_receipt = Some(format!("Desktop 宿主已确认热重载 · 覆盖 {} 个已加载会话 · 已运行的 Hook 和已注入内容不会撤回", connected_threads));
        }
        if after.runtime_receipt.is_none() {
            after.runtime_receipt = Some(
                "请新建 Desktop 对话加载最新开关，无需重启应用；已有对话不保证同步变化".into(),
            );
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
pub(crate) struct Rpc {
    process: Option<Process>,
    #[cfg(unix)]
    owner: Option<desktop_bridge::Owner>,
    lines: mpsc::Receiver<Value>,
    next: u64,
}
impl Rpc {
    #[cfg(unix)]
    fn from_owner(owner: desktop_bridge::Owner) -> Self {
        let (_, lines) = mpsc::channel();
        Self {
            process: None,
            owner: Some(owner),
            lines,
            next: 0,
        }
    }
    pub(crate) fn start(mut cmd: Command) -> Result<Self, String> {
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
            process: Some(process),
            #[cfg(unix)]
            owner: None,
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
        let input = self
            .process
            .as_mut()
            .and_then(|p| p.0.stdin.as_mut())
            .ok_or("配置接口已关闭。")?;
        writeln!(input, "{}", v)
            .and_then(|_| input.flush())
            .map_err(|_| "配置接口通信失败。".into())
    }
    pub(crate) fn call(&mut self, method: &str, params: Value) -> Result<Value, String> {
        #[cfg(unix)]
        if let Some(owner) = &mut self.owner {
            return owner.call(method, params);
        }
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
    crate::desktop_runtime::binary()
}

fn context(app: &tauri::AppHandle) -> Result<Context, String> {
    Ok(Context {
        binary: find_codex().ok_or(
            "未找到受支持的 Codex Desktop 内置引擎。请安装 Desktop；Atelier 不会安装或升级 CLI。",
        )?,
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
pub(crate) struct ModeInstallation {
    pub source: PathBuf,
    pub runtime_data: PathBuf,
    pub enabled: bool,
}
pub(crate) fn mode_installation(
    app: &tauri::AppHandle,
    id: PluginId,
) -> Result<ModeInstallation, String> {
    let ctx = context(app)?;
    let entries = ctx.installed_plugins()?;
    let own = format!("{}@{MARKETPLACE}", id.key());
    let upstream = format!("{}@{}", id.key(), id.key());
    let matches: Vec<_> = entries
        .iter()
        .filter(|p| p["pluginId"] == own || p["pluginId"] == upstream)
        .collect();
    if matches.len() != 1 {
        return Err("请先安装插件并保留唯一来源，再读取模式。".into());
    }
    let entry = matches[0];
    let d = definition(id);
    if entry["version"].as_str() != Some(&d.version) {
        return Err("当前插件版本尚未适配模式控制，请使用 Atelier 固定版本。".into());
    }
    let source = ctx.data.join("marketplace").join(id.key());
    verify_source(&source, &d)?;
    let market = entry["pluginId"]
        .as_str()
        .and_then(|s| s.split_once('@'))
        .map(|(_, m)| m)
        .ok_or("插件来源无效。")?;
    // Refuse a cache that differs from the verified source we execute to update
    // STS state; do not silently use a changed schema or third-party controller.
    let cache = ctx
        .codex_home()
        .join("plugins/cache")
        .join(market)
        .join(id.key())
        .join(&d.version);
    let sub = if id == PluginId::StopThatShit {
        "src"
    } else {
        "hooks"
    };
    for file in fs::read_dir(source.join(sub)).map_err(|_| "模式适配源码不完整。")? {
        let file = file.map_err(|_| "无法检查模式适配源码。")?;
        if file
            .path()
            .extension()
            .is_some_and(|s| s == "cjs" || s == "js")
        {
            let target = cache.join(sub).join(file.file_name());
            if fs::symlink_metadata(&target)
                .map_err(|_| "插件缓存不完整。")?
                .file_type()
                .is_symlink()
                || fs::read(file.path()).map_err(|_| "无法校验插件源码。")?
                    != fs::read(target).map_err(|_| "无法校验插件缓存。")?
            {
                return Err("插件缓存与固定源码不一致，未修改模式。".into());
            }
        }
    }
    Ok(ModeInstallation {
        source,
        runtime_data: ctx
            .codex_home()
            .join("plugins/data")
            .join(format!("{}-{market}", id.key())),
        enabled: entry["enabled"].as_bool() == Some(true),
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
                hook_trust: HookTrust::unknown("未连接 Desktop 内置引擎，无法读取 Hook 信任状态。"),
            })
            .collect(),
        desktop_bridge: BridgeStatus::disconnected(),
        runtime_receipt: None,
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
    #[ignore = "called by the isolated Desktop bridge acceptance harness only"]
    fn bridge_acceptance_control() {
        let root = PathBuf::from(
            std::env::var_os("ATELIER_BRIDGE_ACCEPTANCE_ROOT").expect("isolated root required"),
        );
        let root = fs::canonicalize(root).unwrap();
        assert!(root.starts_with(fs::canonicalize(std::env::temp_dir()).unwrap()));
        assert_eq!(
            fs::read_to_string(root.join("acceptance-marker")).unwrap(),
            "atelier-isolated-bridge-test"
        );
        assert_eq!(
            PathBuf::from(std::env::var_os("CODEX_HOME").unwrap()),
            root.join("codex")
        );
        assert_eq!(desktop_bridge::directory(), root.join("bridge"));
        let ctx = Context {
            binary: PathBuf::from(std::env::var_os("ATELIER_REAL_CODEX").unwrap()),
            home: PathBuf::from(std::env::var_os("HOME").unwrap()),
            data: root.join("control-data"),
            isolated: false,
        };
        let action = std::env::var("ATELIER_BRIDGE_ACCEPTANCE_ACTION").unwrap();
        let before = ctx.snapshot().unwrap();
        if action == "stale" {
            assert_eq!(before.desktop_bridge.state, "error");
            let after = ctx.set(PluginId::Ponytail, true, &before.revision).unwrap();
            assert_eq!(after.plugins[0].enabled, Some(true));
            assert!(after
                .runtime_receipt
                .as_ref()
                .unwrap()
                .contains("新建 Desktop 对话"));
            let restored = ctx.rollback(PluginId::Ponytail, &after.revision).unwrap();
            assert_eq!(restored.plugins[0].enabled, before.plugins[0].enabled);
            println!("ATELIER_RESULT {{\"staleControlRejected\":true,\"newConversationWriteAllowed\":true}}");
            return;
        }
        assert_eq!(before.desktop_bridge.state, "connected");
        assert_eq!(before.desktop_bridge.loaded_threads, 1);
        if action == "conflict" {
            let mut rpc = ctx.rpc().unwrap();
            rpc.call("config/value/write", json!({"keyPath":"model","value":"atelier-conflict-probe","mergeStrategy":"replace","expectedVersion":before.revision})).unwrap();
            assert!(ctx.set(PluginId::Ponytail, true, &before.revision).is_err());
            let changed = ctx.snapshot().unwrap();
            assert_eq!(
                changed
                    .plugins
                    .iter()
                    .map(|p| p.enabled)
                    .collect::<Vec<_>>(),
                before.plugins.iter().map(|p| p.enabled).collect::<Vec<_>>()
            );
            rpc.call("config/value/write", json!({"keyPath":"model","value":"atelier-local-probe","mergeStrategy":"replace","expectedVersion":changed.revision})).unwrap();
            println!("ATELIER_RESULT {{\"conflictRejected\":true}}");
            return;
        }
        assert!(matches!(action.as_str(), "enable" | "disable"));
        let mut after = before;
        for id in [PluginId::Ponytail, PluginId::StopThatShit] {
            after = ctx.set(id, action == "enable", &after.revision).unwrap();
            assert!(after
                .runtime_receipt
                .as_ref()
                .is_some_and(|r| r.contains("热重载")));
        }
        assert!(after
            .plugins
            .iter()
            .all(|p| p.enabled == Some(action == "enable")));
        println!(
            "ATELIER_RESULT {}",
            json!({"runtimeReceipt":after.runtime_receipt,"desktopBridge":after.desktop_bridge})
        );
    }
    fn hook_fixture(hooks: Value) -> Result<Value, String> {
        Ok(json!({"data":[{"cwd":"/fixture", "hooks":hooks,"warnings":[],"errors":[]}]}))
    }
    #[test]
    fn trust_uses_exact_plugin_identity_and_host_status() {
        let hooks = hook_fixture(json!([
            {"pluginId":"ponytail@atelier-companions","trustStatus":"trusted","enabled":true},
            {"pluginId":"ponytail@atelier-companions","trustStatus":"managed","enabled":false},
            {"pluginId":"ponytail@other-marketplace","trustStatus":"untrusted","enabled":true},
            {"pluginId":null,"trustStatus":"modified","enabled":true}
        ]));
        let trust = hook_trust(&hooks, "ponytail@atelier-companions", true, true, false);
        assert_eq!(trust.state, HookTrustState::Trusted);
        assert_eq!(
            (trust.total, trust.trusted, trust.managed, trust.disabled),
            (2, 1, 1, 1)
        );
        assert_eq!(trust.hooks.len(), 2);
    }
    #[test]
    fn review_details_only_include_exact_plugin_hooks() {
        let hooks = hook_fixture(json!([
            {"pluginId":"ponytail@atelier-companions","key":"one","eventName":"sessionStart","trustStatus":"trusted","enabled":true,"sourcePath":"/fixture/ponytail/hooks.json","handlerType":"command","command":"python3 session-start.py","matcher":"startup","currentHash":"not-needed-for-display"},
            {"pluginId":"ponytail@atelier-companions","key":"two","eventName":"userPromptSubmit","trustStatus":"modified","enabled":false,"handlerType":"mcpTool","server":"fixture","tool":"validate"},
            {"pluginId":"unrelated@private","key":"private-key","trustStatus":"trusted","enabled":true,"command":"private-command"}
        ]));
        let trust = hook_trust(&hooks, "ponytail@atelier-companions", true, true, false);
        assert_eq!(trust.hooks.len(), 2);
        assert_eq!(trust.hooks[0].event_name, "sessionStart");
        assert_eq!(
            trust.hooks[0].source_path.as_deref(),
            Some("/fixture/ponytail/hooks.json")
        );
        assert_eq!(
            trust.hooks[0].command.as_deref(),
            Some("python3 session-start.py")
        );
        assert_eq!(
            trust.hooks[1].mcp_target.as_deref(),
            Some("fixture/validate")
        );
        assert_eq!(trust.hooks[1].trust_status, "modified");
        assert!(!trust.hooks[1].enabled);
        let serialized = serde_json::to_string(&trust).unwrap();
        assert!(!serialized.contains("private-command"));
        assert!(!serialized.contains("currentHash"));
    }
    #[test]
    fn mixed_or_modified_hooks_require_review() {
        let hooks = hook_fixture(json!([
            {"pluginId":"ponytail@atelier-companions","trustStatus":"trusted","enabled":true},
            {"pluginId":"ponytail@atelier-companions","trustStatus":"untrusted","enabled":true},
            {"pluginId":"ponytail@atelier-companions","trustStatus":"modified","enabled":true}
        ]));
        let trust = hook_trust(&hooks, "ponytail@atelier-companions", true, true, false);
        assert_eq!(trust.state, HookTrustState::ReviewRequired);
        assert_eq!(
            (trust.total, trust.trusted, trust.untrusted, trust.modified),
            (3, 1, 1, 1)
        );
    }
    #[test]
    fn missing_or_disabled_discovery_is_never_untrusted_or_trusted() {
        let hooks = hook_fixture(json!([]));
        let off = hook_trust(&hooks, "ponytail@atelier-companions", true, false, false);
        assert_eq!(off.state, HookTrustState::NotLoaded);
        assert!(off.reason.unwrap().contains("已停用"));
        assert_eq!(
            hook_trust(&hooks, "ponytail@atelier-companions", true, true, false).state,
            HookTrustState::NotLoaded
        );
        assert_eq!(
            hook_trust(&hooks, "ponytail@atelier-companions", false, false, false).state,
            HookTrustState::NotInstalled
        );
    }
    #[test]
    fn failed_partial_or_unknown_protocol_fails_closed_without_raw_data() {
        let cases = [
            Err("private upstream error text".into()),
            Ok(json!({"data":[]})),
            Ok(json!({"data":[{"hooks":[],"warnings":["private warning"],"errors":[]}]})),
            Ok(json!({"data":[{"hooks":[],"warnings":[],"errors":[{"message":"private error"}]}]})),
            hook_fixture(
                json!([{"pluginId":"ponytail@atelier-companions","trustStatus":"futureStatus","enabled":true}]),
            ),
            hook_fixture(
                json!([{"pluginId":"ponytail@atelier-companions","trustStatus":"trusted"}]),
            ),
        ];
        for case in cases {
            let trust = hook_trust(&case, "ponytail@atelier-companions", true, true, false);
            assert_eq!(trust.state, HookTrustState::Unavailable);
            assert!(!trust.reason.unwrap().contains("private"));
        }
        assert_eq!(
            hook_trust(
                &hook_fixture(json!([])),
                "ponytail@atelier-companions",
                true,
                true,
                true
            )
            .state,
            HookTrustState::Unavailable
        );
    }
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
        let binary = find_codex().expect("Codex installed");
        #[cfg(unix)]
        let binary = {
            use std::os::unix::fs::PermissionsExt;
            let wrapper = root.join("codex-filtered-only");
            // Simulate an unrelated marketplace outage: any global enumeration
            // fails, while scoped reads still go through the real Codex CLI.
            let quoted = binary.to_string_lossy().replace('\'', "'\\''");
            fs::write(&wrapper, format!("#!/bin/sh\nif [ \"$1\" = plugin ] && [ \"$2\" = list ]; then\ncase \" $* \" in *\" --marketplace \"*) ;; *) exit 86 ;; esac\nfi\nexec '{quoted}' \"$@\"\n")).unwrap();
            fs::set_permissions(&wrapper, fs::Permissions::from_mode(0o700)).unwrap();
            wrapper
        };
        let ctx = Context {
            binary,
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
            fs::create_dir_all(p.join("hooks")).unwrap();
            fs::write(p.join("hooks/hooks.json"), json!({"hooks":{"SessionStart":[{"hooks":[{"type":"command","command":"echo atelier-hook-must-not-run"}]}]}}).to_string()).unwrap();
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
        assert!(s
            .plugins
            .iter()
            .all(|p| p.hook_trust.state == HookTrustState::ReviewRequired
                && p.hook_trust.untrusted == 1));
        // A repeated read may discover trust, but cannot grant it or edit config.
        let untouched = fs::read(ctx.home.join("config.toml")).unwrap();
        let again = ctx.snapshot().unwrap();
        assert!(again.plugins.iter().all(|p| p.hook_trust.untrusted == 1));
        assert_eq!(fs::read(ctx.home.join("config.toml")).unwrap(), untouched);
        let off = ctx.set(PluginId::Ponytail, false, &s.revision).unwrap();
        assert_eq!(off.plugins[0].enabled, Some(false));
        assert!(off
            .runtime_receipt
            .as_ref()
            .unwrap()
            .contains("新建 Desktop 对话"));
        assert!(!off.runtime_receipt.as_ref().unwrap().contains("热重载"));
        assert_eq!(off.plugins[0].hook_trust.state, HookTrustState::NotLoaded);
        assert!(ctx.set(PluginId::StopThatShit, false, &s.revision).is_err());
        let restored = ctx.rollback(PluginId::Ponytail, &off.revision).unwrap();
        assert_eq!(restored.plugins[0].enabled, Some(true));
        let off = ctx
            .set(PluginId::StopThatShit, false, &restored.revision)
            .unwrap();
        assert_eq!(off.plugins[1].enabled, Some(false));
        let both_off = ctx.set(PluginId::Ponytail, false, &off.revision).unwrap();
        assert!(both_off.plugins.iter().all(|p| p.enabled == Some(false)));
        // A fresh host process must preserve both disabled values.
        assert!(ctx
            .snapshot()
            .unwrap()
            .plugins
            .iter()
            .all(|p| p.enabled == Some(false)));
        let content = fs::read_to_string(ctx.home.join("config.toml")).unwrap();
        assert!(content.contains("# preserve me"));
        assert!(content.contains("example-model"));
        let mut altered = content;
        altered = altered.replace("example-model", "external-model");
        fs::write(ctx.home.join("config.toml"), altered).unwrap();
        assert!(ctx
            .rollback(PluginId::Ponytail, &both_off.revision)
            .is_err());
        let _ = fs::remove_dir_all(root);
    }
}
