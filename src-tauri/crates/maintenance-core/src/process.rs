//! Desktop process lifecycle recovered from AiMaMi 1.2.7 arm64.
//! SystemRunner is an explicit opt-in. Tests use a synthetic runner exclusively.
use std::{
    ffi::OsString,
    io,
    path::{Path, PathBuf},
    time::Duration,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessEntry {
    pub pid: u32,
    pub command: String,
}
/// @1007c09ac: equality or exact executable prefix followed by Rust whitespace.
pub fn command_runs_executable(command: &str, executable: &Path) -> bool {
    let executable = executable.to_string_lossy();
    command == executable
        || command
            .strip_prefix(executable.as_ref())
            .is_some_and(|rest| rest.chars().next().is_some_and(char::is_whitespace))
}
/// @1007be050: first whitespace-delimited word, then last slash component.
pub fn extract_process_name(command: &str) -> String {
    command
        .split_whitespace()
        .next()
        .unwrap_or(command)
        .rsplit('/')
        .next()
        .unwrap_or("")
        .to_owned()
}
/// @1007c7910: separator normalization and ASCII lowercase, then substring match.
pub fn is_openai_ide_extension_writer_path(command: &str) -> bool {
    command
        .replace('\\', "/")
        .to_ascii_lowercase()
        .contains("/extensions/openai.chatgpt")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WriterClass {
    Desktop,
    ExternalCli,
    Other,
}
const EXTENSION_ROOTS: [&str; 5] = [
    "/.vscode/extensions/",
    "/.vscode-insiders/extensions/",
    "/.vscode-oss/extensions/",
    "/.cursor/extensions/",
    "/.windsurf/extensions/",
];
fn normalized(command: &str) -> String {
    command.replace('\\', "/").to_ascii_lowercase()
}
fn basename(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}
fn codex_name(s: &str) -> bool {
    s == "codex" || s.starts_with("codex-") || s.starts_with("codex_")
}
pub fn is_protected_process_command(command: &str) -> bool {
    let normalized = normalized(command);
    let executable = normalized.split_whitespace().next().unwrap_or("");
    (is_openai_ide_extension_writer_path(command) && codex_name(basename(executable)))
        || EXTENSION_ROOTS.iter().any(|s| normalized.contains(s))
        || [
            "/Cursor.app/",
            "/.codex/computer-use/",
            "/Codex Computer Use.app/",
            "/AiMaMi.app/",
            "/AiMaMi ",
            "crashpad_handler",
        ]
        .iter()
        .any(|s| command.contains(s))
}
pub fn classify_macos_codex_writer(command: &str) -> WriterClass {
    let normalized = normalized(command);
    if normalized.contains("/chatgpt.app/contents/") {
        return WriterClass::Desktop;
    }
    if is_openai_ide_extension_writer_path(command) {
        return WriterClass::ExternalCli;
    }
    if EXTENSION_ROOTS
        .iter()
        .chain(["/visual studio code.app/", "/cursor.app/", "/windsurf.app/"].iter())
        .any(|s| normalized.contains(s))
    {
        return WriterClass::Other;
    }
    let mut words = normalized.split_whitespace();
    let executable = words.next().unwrap_or("");
    let origin = |p: &str| {
        p.contains("/node_modules/@openai/codex/")
            || p.starts_with("/opt/homebrew/")
            || p.starts_with("/usr/local/")
            || ["/.local/bin/", "/.volta/", "/.nvm/", "/.local/share/pnpm/"]
                .iter()
                .any(|s| p.contains(s))
    };
    match basename(executable) {
        "codex"
            if executable == "codex"
                || origin(executable)
                || normalized.contains("/node_modules/@openai/codex/") =>
        {
            WriterClass::ExternalCli
        }
        "node" | "node.exe" => {
            let script = words.next().unwrap_or("");
            if matches!(basename(script), "codex" | "codex.js")
                && (script == "codex" || origin(script))
            {
                WriterClass::ExternalCli
            } else {
                WriterClass::Other
            }
        }
        _ => WriterClass::Other,
    }
}
pub fn is_codex_related_command_with_main(command: &str, main: Option<&Path>) -> bool {
    if is_protected_process_command(command) {
        return false;
    }
    if let Some(main) = main {
        if command_runs_executable(command, main) {
            return true;
        }
        for ancestor in main.ancestors() {
            if ancestor
                .extension()
                .is_some_and(|e| e.to_string_lossy().eq_ignore_ascii_case("app"))
            {
                if command
                    .strip_prefix(ancestor.to_string_lossy().as_ref())
                    .is_some_and(|s| s.starts_with('/'))
                {
                    return true;
                }
                break;
            }
        }
    }
    if command.contains("Codex.app/") || command.contains("Codex Helper") {
        return true;
    }
    let mut words = command.split_whitespace();
    let executable = words.next().unwrap_or("");
    let name = basename(executable).to_ascii_lowercase();
    if codex_name(&name) {
        return true;
    }
    matches!(name.as_str(), "node" | "node.exe")
        && words.next().is_some_and(|s| {
            let s = basename(s).to_ascii_lowercase();
            s == "codex" || s.starts_with("codex-") || s == "codex.js"
        })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BundleField {
    Identifier,
    Executable,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    Processes,
    CommandForPid(u32),
    FindBundles,
    BundleValue { bundle: PathBuf, field: BundleField },
    IsAppRunning,
    QuitApp,
    OpenBundle(PathBuf),
    DbHolders(Vec<PathBuf>),
}
impl Operation {
    pub fn argv(&self) -> (&'static str, Vec<OsString>) {
        let args = |a: &[&str]| a.iter().map(OsString::from).collect();
        match self{
   Self::Processes=>("ps",args(&["-ax","-o","pid=,command="])),
   Self::CommandForPid(pid)=>("ps",vec!["-p".into(),pid.to_string().into(),"-o".into(),"command=".into()]),
   Self::FindBundles=>("/usr/bin/mdfind",args(&["kMDItemCFBundleIdentifier == 'com.openai.codex'"])),
   Self::BundleValue{bundle,field}=>("/usr/bin/plutil",vec!["-extract".into(),match field{BundleField::Identifier=>"CFBundleIdentifier",BundleField::Executable=>"CFBundleExecutable"}.into(),"raw".into(),"-o".into(),"-".into(),bundle.join("Contents/Info.plist").into_os_string()]),
   Self::IsAppRunning=>("/usr/bin/osascript",args(&["-e","application id \"com.openai.codex\" is running"])),
   Self::QuitApp=>("/usr/bin/osascript",args(&["-e","if application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if"])),
   Self::OpenBundle(p)=>("/usr/bin/open",vec![p.as_os_str().to_owned()]),
   Self::DbHolders(paths)=>{let mut a=args(&["-t","--"]);a.extend(paths.iter().map(|p|p.as_os_str().to_owned()));("lsof",a)},
  }
    }
}
#[derive(Clone, Debug, Default)]
pub struct CommandOutput {
    pub success: bool,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}
pub trait Runner {
    fn output(&self, op: &Operation) -> io::Result<CommandOutput>;
    fn signal_kill(&self, pid: u32) -> io::Result<()>;
    fn exists(&self, path: &Path) -> bool;
    fn is_file(&self, path: &Path) -> bool;
    fn read_text(&self, path: &Path) -> io::Result<String>;
    fn read_dir(&self, path: &Path) -> io::Result<Vec<PathBuf>>;
    fn process_id(&self) -> u32;
    fn now(&self) -> Duration;
    fn sleep(&self, duration: Duration);
}
pub struct SystemRunner {
    started: std::time::Instant,
}
impl Default for SystemRunner {
    fn default() -> Self {
        Self {
            started: std::time::Instant::now(),
        }
    }
}
impl Runner for SystemRunner {
    fn output(&self, op: &Operation) -> io::Result<CommandOutput> {
        #[cfg(target_os = "macos")]
        {
            let (program, args) = op.argv();
            let out = std::process::Command::new(program)
                .args(args)
                .stdin(std::process::Stdio::null())
                .output()?;
            Ok(CommandOutput {
                success: out.status.success(),
                stdout: out.stdout,
                stderr: out.stderr,
            })
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = op;
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "installed arm64 lifecycle is macOS-only",
            ))
        }
    }
    fn signal_kill(&self, pid: u32) -> io::Result<()> {
        #[cfg(target_os = "macos")]
        {
            unsafe extern "C" {
                fn kill(pid: i32, sig: i32) -> i32;
            }
            if unsafe { kill(pid as i32, 9) } == 0 {
                return Ok(());
            }
            let e = io::Error::last_os_error();
            if e.raw_os_error() == Some(3) {
                Ok(())
            } else {
                Err(e)
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = pid;
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "installed arm64 lifecycle is macOS-only",
            ))
        }
    }
    fn exists(&self, path: &Path) -> bool {
        std::fs::metadata(path).is_ok()
    }
    fn is_file(&self, path: &Path) -> bool {
        path.is_file()
    }
    fn read_text(&self, path: &Path) -> io::Result<String> {
        std::fs::read_to_string(path)
    }
    fn read_dir(&self, path: &Path) -> io::Result<Vec<PathBuf>> {
        Ok(std::fs::read_dir(path)?
            .filter_map(Result::ok)
            .map(|e| e.path())
            .collect())
    }
    fn process_id(&self) -> u32 {
        std::process::id()
    }
    fn now(&self) -> Duration {
        self.started.elapsed()
    }
    fn sleep(&self, duration: Duration) {
        std::thread::sleep(duration)
    }
}
#[derive(Clone, Debug)]
pub struct ProcessContext {
    pub user_home: Option<PathBuf>,
    pub codex_home: PathBuf,
    pub sqlite_home_env: Option<OsString>,
    pub cwd: Option<PathBuf>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QuietTimeoutKind {
    UnconfirmedWindow,
    QuitBlocked,
    OrphanWriter,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProcessError {
    Command {
        operation: &'static str,
        detail: String,
    },
    NotFound(String),
    InvalidBundle(String),
    ExternalWriters(Vec<ProcessEntry>),
    ProtectedDbHolders(Vec<ProcessEntry>),
    QuietTimeout {
        kind: QuietTimeoutKind,
        writers: Vec<ProcessEntry>,
        last_signal_error: Option<String>,
    },
    ForceKillTimeout {
        writers: Vec<ProcessEntry>,
        last_signal_error: Option<String>,
    },
    DesktopForceKillTimeout {
        writers: Vec<ProcessEntry>,
        last_signal_error: Option<String>,
    },
    LaunchTimeout(PathBuf),
}
impl std::fmt::Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Command { operation, detail } => write!(f, "{operation} failed: {detail}"),
            Self::NotFound(s) | Self::InvalidBundle(s) => f.write_str(s),
            Self::ExternalWriters(p) => write!(f, "CODEX_EXTERNAL_WRITER_RUNNING: {}", describe(p)),
            Self::ProtectedDbHolders(p) => {
                write!(f, "CODEX_DB_HELD_BY_PROTECTED_PROCESS: {}", describe(p))
            }
            Self::QuietTimeout {
                kind,
                writers,
                last_signal_error,
            } => {
                let (code, message) = match kind {
                    QuietTimeoutKind::UnconfirmedWindow => ("CODEX_TRANSITION_QUIET_TIMEOUT", "未能在时限内确认 Codex 安全写入窗口"),
                    QuietTimeoutKind::QuitBlocked => ("CODEX_QUIT_BLOCKED", "ChatGPT 未释放 Codex 写入进程，可能存在进行中的任务或未处理的退出确认。请先完成任务并在 ChatGPT 中确认退出后重试"),
                    QuietTimeoutKind::OrphanWriter => ("CODEX_ORPHAN_WRITER_TIMEOUT", "ChatGPT 主进程已退出，但孤儿 Codex 写入进程未能正常终止"),
                };
                write!(f, "{code}: {message}")?;
                if !writers.is_empty() {
                    write!(f, "：{}", describe(writers))?;
                }
                if let Some(error) = last_signal_error {
                    write!(f, "；安全停止请求错误：{error}")?;
                }
                Ok(())
            }
            Self::ForceKillTimeout {
                writers,
                last_signal_error,
            } => write!(
                f,
                "CODEX_FORCE_KILL_FAILED: {}{}",
                describe(writers),
                last_signal_error.as_deref().unwrap_or("")
            ),
            Self::DesktopForceKillTimeout {
                writers,
                last_signal_error,
            } => write!(
                f,
                "CHATGPT_DESKTOP_FORCE_KILL_FAILED: {}{}",
                describe(writers),
                last_signal_error.as_deref().unwrap_or("")
            ),
            Self::LaunchTimeout(p) => write!(
                f,
                "Codex/ChatGPT main process did not start: {}",
                p.display()
            ),
        }
    }
}
impl std::error::Error for ProcessError {}
fn describe(p: &[ProcessEntry]) -> String {
    p.iter()
        .map(|p| format!("{} {}", p.pid, extract_process_name(&p.command)))
        .collect::<Vec<_>>()
        .join(" | ")
}
fn stdout(o: &CommandOutput) -> String {
    String::from_utf8_lossy(&o.stdout).into_owned()
}
fn checked_output(
    r: &impl Runner,
    op: &Operation,
    name: &'static str,
) -> Result<CommandOutput, ProcessError> {
    let o = r.output(op).map_err(|e| ProcessError::Command {
        operation: name,
        detail: e.to_string(),
    })?;
    if o.success {
        Ok(o)
    } else {
        Err(ProcessError::Command {
            operation: name,
            detail: String::from_utf8_lossy(&o.stderr).trim().into(),
        })
    }
}
pub fn parse_processes(text: &str, current_pid: u32) -> Vec<ProcessEntry> {
    text.lines()
        .filter_map(|line| {
            let line = line.trim_start();
            let (pid, command) = line.split_once(char::is_whitespace)?;
            let pid = pid.parse::<u32>().ok()?;
            (pid != current_pid).then(|| ProcessEntry {
                pid,
                command: command.trim().to_owned(),
            })
        })
        .collect()
}
fn bundle_value(r: &impl Runner, bundle: &Path, field: BundleField) -> Option<String> {
    r.output(&Operation::BundleValue {
        bundle: bundle.to_path_buf(),
        field,
    })
    .ok()
    .filter(|o| o.success)
    .map(|o| stdout(&o).trim().to_owned())
}
pub fn bundle_main_executable(r: &impl Runner, bundle: &Path) -> Option<PathBuf> {
    if !bundle_value(r, bundle, BundleField::Identifier)?.eq_ignore_ascii_case("com.openai.codex") {
        return None;
    }
    let name = bundle_value(r, bundle, BundleField::Executable)?;
    let path = bundle.join("Contents/MacOS").join(name);
    r.is_file(&path).then_some(path)
}
pub fn codex_app_bundle_path(r: &impl Runner, ctx: &ProcessContext) -> Option<PathBuf> {
    let mut roots = vec![PathBuf::from("/Applications")];
    if let Some(h) = &ctx.user_home {
        roots.push(h.join("Applications"))
    }
    for root in roots {
        let p = root.join("ChatGPT.app");
        if bundle_main_executable(r, &p).is_some() {
            return Some(p);
        }
    }
    let mut paths = r
        .output(&Operation::FindBundles)
        .ok()
        .filter(|o| o.success)
        .map(|o| {
            stdout(&o)
                .lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(PathBuf::from)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    paths.sort();
    paths.dedup();
    paths
        .into_iter()
        .find(|p| bundle_main_executable(r, p).is_some())
}
pub fn main_process_pid(r: &impl Runner, main: &Path) -> Option<u32> {
    let o = r.output(&Operation::Processes).ok().filter(|o| o.success)?;
    parse_processes(&stdout(&o), r.process_id())
        .into_iter()
        .find(|p| command_runs_executable(&p.command, main))
        .map(|p| p.pid)
}
pub fn is_codex_app_running(r: &impl Runner, ctx: &ProcessContext) -> bool {
    if let Some(main) = codex_app_bundle_path(r, ctx).and_then(|p| bundle_main_executable(r, &p)) {
        if main_process_pid(r, &main).is_some() {
            return true;
        }
    }
    r.output(&Operation::IsAppRunning)
        .ok()
        .is_some_and(|o| o.success && stdout(&o).trim() == "true")
}

impl ProcessContext {
    /// Inputs are explicit so fixtures and preview callers never resolve a real user home.
    pub fn new(codex_home: PathBuf, user_home: Option<PathBuf>) -> Self {
        Self {
            codex_home,
            user_home,
            sqlite_home_env: None,
            cwd: None,
        }
    }
}
/// 1.2.7 sorts numeric versions descending, then Path components ascending.
pub fn runtime_state_db_candidates(r: &impl Runner, ctx: &ProcessContext) -> Vec<PathBuf> {
    let config = r.read_text(&ctx.codex_home.join("config.toml")).ok();
    let configured = config
        .as_deref()
        .and_then(|s| toml::from_str::<toml::Value>(s).ok())
        .and_then(|v| {
            v.get("sqlite_home")
                .and_then(|v| v.as_str())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(PathBuf::from)
        })
        .or_else(|| {
            ctx.sqlite_home_env
                .as_ref()
                .filter(|s| !s.is_empty())
                .map(PathBuf::from)
        });
    let roots = if let Some(p) = configured {
        vec![if p.is_relative() {
            ctx.cwd.as_ref().map(|cwd| cwd.join(&p)).unwrap_or(p)
        } else {
            p
        }]
    } else {
        vec![ctx.codex_home.join("sqlite"), ctx.codex_home.clone()]
    };
    let mut paths = Vec::new();
    for root in roots {
        for path in r.read_dir(&root).unwrap_or_default() {
            if r.is_file(&path) && state_db_version(&path).is_some() && !paths.contains(&path) {
                paths.push(path)
            }
        }
    }
    paths.sort_by(|a, b| {
        state_db_version(b)
            .cmp(&state_db_version(a))
            .then_with(|| a.cmp(b))
    });
    paths
}
fn state_db_version(p: &Path) -> Option<u32> {
    p.file_name()?
        .to_str()?
        .strip_prefix("state_")?
        .strip_suffix(".sqlite")?
        .parse()
        .ok()
}
fn main_executable(r: &impl Runner, ctx: &ProcessContext) -> Option<PathBuf> {
    codex_app_bundle_path(r, ctx).and_then(|p| bundle_main_executable(r, &p))
}
pub fn list_codex_processes_matching(
    r: &impl Runner,
    ctx: &ProcessContext,
) -> Result<Vec<ProcessEntry>, ProcessError> {
    let main = main_executable(r, ctx);
    let o = checked_output(r, &Operation::Processes, "ps")?;
    Ok(parse_processes(&stdout(&o), r.process_id())
        .into_iter()
        .filter(|p| is_codex_related_command_with_main(&p.command, main.as_deref()))
        .collect())
}
pub fn collect_external_codex_writers(r: &impl Runner) -> Result<Vec<ProcessEntry>, ProcessError> {
    let o = checked_output(r, &Operation::Processes, "ps")?;
    Ok(parse_processes(&stdout(&o), r.process_id())
        .into_iter()
        .filter(|p| classify_macos_codex_writer(&p.command) == WriterClass::ExternalCli)
        .collect())
}
/// lsof and per-PID ps failures are best effort, as in @1007c6c44.
pub fn db_holders_for(r: &impl Runner, database: &Path) -> Vec<ProcessEntry> {
    let mut paths = Vec::new();
    for suffix in ["", "-wal", "-shm"] {
        let mut p = database.as_os_str().to_owned();
        p.push(suffix);
        let p = PathBuf::from(p);
        if r.exists(&p) {
            paths.push(p)
        }
    }
    if paths.is_empty() {
        return Vec::new();
    }
    let Ok(o) = r.output(&Operation::DbHolders(paths)) else {
        return Vec::new();
    };
    let mut seen = Vec::new();
    let mut result = Vec::new();
    for pid in stdout(&o)
        .lines()
        .filter_map(|s| s.trim().parse::<u32>().ok())
    {
        if pid == r.process_id() || seen.contains(&pid) {
            continue;
        }
        seen.push(pid);
        let Ok(o) = r.output(&Operation::CommandForPid(pid)) else {
            continue;
        };
        if !o.success {
            continue;
        }
        let command = stdout(&o).trim().to_owned();
        if !command.is_empty() {
            result.push(ProcessEntry { pid, command })
        }
    }
    result
}
pub fn list_codex_db_holder_processes(r: &impl Runner, ctx: &ProcessContext) -> Vec<ProcessEntry> {
    let mut holders = Vec::new();
    for db in runtime_state_db_candidates(r, ctx) {
        for p in db_holders_for(r, &db) {
            if !holders
                .iter()
                .any(|prior: &ProcessEntry| prior.pid == p.pid)
            {
                holders.push(p)
            }
        }
    }
    holders
}
pub fn collect_codex_kill_targets(
    r: &impl Runner,
    ctx: &ProcessContext,
) -> Result<Vec<ProcessEntry>, ProcessError> {
    let mut targets = list_codex_processes_matching(r, ctx)?;
    let main = main_executable(r, ctx);
    for p in list_codex_db_holder_processes(r, ctx) {
        if is_codex_related_command_with_main(&p.command, main.as_deref())
            && !targets.iter().any(|t| t.pid == p.pid)
        {
            targets.push(p)
        }
    }
    for p in &mut targets {
        p.command = extract_process_name(&p.command)
    }
    Ok(targets)
}
pub fn protected_codex_db_holders(r: &impl Runner, ctx: &ProcessContext) -> Vec<ProcessEntry> {
    let main = main_executable(r, ctx);
    list_codex_db_holder_processes(r, ctx)
        .into_iter()
        .filter(|p| !is_codex_related_command_with_main(&p.command, main.as_deref()))
        .collect()
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessPolicy {
    ForceCloseAll,
    DesktopTransition,
}
fn reject_external(r: &impl Runner) -> Result<(), ProcessError> {
    let writers = collect_external_codex_writers(r)?;
    if writers.is_empty() {
        Ok(())
    } else {
        Err(ProcessError::ExternalWriters(writers))
    }
}
fn desktop_targets(
    r: &impl Runner,
    ctx: &ProcessContext,
) -> Result<Vec<ProcessEntry>, ProcessError> {
    Ok(list_codex_processes_matching(r, ctx)?
        .into_iter()
        .filter(|p| {
            p.command
                .to_ascii_lowercase()
                .contains("/chatgpt.app/contents/")
        })
        .map(|mut p| {
            p.command = extract_process_name(&p.command);
            p
        })
        .collect())
}
/// Preview includes policy0 external CLI targets and policy1 rejection before any signal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StopPreview {
    pub policy: ProcessPolicy,
    pub targets: Vec<ProcessEntry>,
    pub protected_db_holders: Vec<ProcessEntry>,
    pub external_cli: Vec<ProcessEntry>,
}
pub fn preview_stop(
    r: &impl Runner,
    ctx: &ProcessContext,
    policy: ProcessPolicy,
) -> Result<StopPreview, ProcessError> {
    let external_cli = collect_external_codex_writers(r)?;
    let targets = match policy {
        ProcessPolicy::ForceCloseAll => collect_codex_kill_targets(r, ctx)?,
        ProcessPolicy::DesktopTransition => desktop_targets(r, ctx)?,
    };
    Ok(StopPreview {
        policy,
        targets,
        external_cli,
        protected_db_holders: protected_codex_db_holders(r, ctx),
    })
}
/// Each target is attempted; ESRCH handling belongs to the concrete signal runner.
pub fn force_kill_targets(r: &impl Runner, targets: &[ProcessEntry]) -> Result<(), ProcessError> {
    let errors = targets
        .iter()
        .filter_map(|p| {
            r.signal_kill(p.pid)
                .err()
                .map(|e| format!("{} (pid {}): {}", p.command, p.pid, e))
        })
        .collect::<Vec<_>>();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(ProcessError::Command {
            operation: "force kill",
            detail: errors.join(" | "),
        })
    }
}
fn check_protected(r: &impl Runner, ctx: &ProcessContext) -> Result<(), ProcessError> {
    let p = protected_codex_db_holders(r, ctx);
    if p.is_empty() {
        Ok(())
    } else {
        Err(ProcessError::ProtectedDbHolders(p))
    }
}
pub fn kill_codex_targets_until_clear(
    r: &impl Runner,
    ctx: &ProcessContext,
) -> Result<(), ProcessError> {
    let start = r.now();
    let mut last_signal_error = None;
    loop {
        let writers = collect_codex_kill_targets(r, ctx)?;
        if writers.is_empty() {
            return check_protected(r, ctx);
        }
        if r.now().saturating_sub(start) >= Duration::from_secs(3) {
            return Err(ProcessError::ForceKillTimeout {
                writers,
                last_signal_error,
            });
        }
        last_signal_error = force_kill_targets(r, &writers).err().map(|e| e.to_string());
        r.sleep(Duration::from_millis(300))
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForceCloseReport {
    pub processes: Vec<ProcessEntry>,
}
pub fn force_close_all(
    r: &impl Runner,
    ctx: &ProcessContext,
) -> Result<ForceCloseReport, ProcessError> {
    let processes = collect_codex_kill_targets(r, ctx)?;
    kill_codex_targets_until_clear(r, ctx)?;
    Ok(ForceCloseReport { processes })
}
pub fn prepare_transition(
    r: &impl Runner,
    ctx: &ProcessContext,
    graceful_timeout: Duration,
    policy: ProcessPolicy,
) -> Result<(), ProcessError> {
    if policy == ProcessPolicy::DesktopTransition {
        reject_external(r)?
    }
    if is_codex_app_running(r, ctx) {
        let _ = r.output(&Operation::QuitApp);
        let start = r.now();
        loop {
            if r.now().saturating_sub(start) >= graceful_timeout {
                let _ = collect_codex_kill_targets(r, ctx);
                break;
            }
            if collect_codex_kill_targets(r, ctx).is_ok_and(|p| p.is_empty()) {
                break;
            }
            r.sleep(Duration::from_millis(100));
        }
    }
    if policy == ProcessPolicy::ForceCloseAll {
        return kill_codex_targets_until_clear(r, ctx);
    }
    let start = r.now();
    let mut last_signal_error = None;
    loop {
        let writers = desktop_targets(r, ctx)?;
        if writers.is_empty() {
            return Ok(());
        }
        if r.now().saturating_sub(start) >= Duration::from_secs(3) {
            return Err(ProcessError::DesktopForceKillTimeout {
                writers,
                last_signal_error,
            });
        }
        last_signal_error = force_kill_targets(r, &writers).err().map(|e| e.to_string());
        r.sleep(Duration::from_millis(300))
    }
}
pub fn quiesce_transition_targets(
    r: &impl Runner,
    ctx: &ProcessContext,
    writers: &[ProcessEntry],
    policy: ProcessPolicy,
) -> Result<(), ProcessError> {
    if writers.is_empty() {
        return Ok(());
    }
    match policy {
        ProcessPolicy::ForceCloseAll => force_kill_targets(r, writers),
        ProcessPolicy::DesktopTransition => {
            reject_external(r)?;
            force_kill_targets(r, &desktop_targets(r, ctx)?)
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Generation {
    Empty,
    Initial,
    Retry,
    Persistent,
    Respawn,
}
#[derive(Clone, Debug, Default)]
pub struct WriterGenerationTracker {
    last_ids: Vec<u32>,
    seen_empty: bool,
    quiesced: bool,
    attempts: u8,
}
impl WriterGenerationTracker {
    pub fn new(initially_quiet: bool) -> Self {
        Self {
            seen_empty: initially_quiet,
            ..Self::default()
        }
    }
    pub fn observe(&mut self, writers: &[ProcessEntry]) -> Generation {
        let mut ids = writers.iter().map(|p| p.pid).collect::<Vec<_>>();
        ids.sort_unstable();
        ids.dedup();
        if ids.is_empty() {
            self.last_ids = ids;
            self.seen_empty = true;
            self.quiesced = false;
            self.attempts = 0;
            return Generation::Empty;
        }
        let changed = self.last_ids.is_empty() || ids.iter().any(|id| !self.last_ids.contains(id));
        let generation = if changed {
            self.quiesced = false;
            self.attempts = 0;
            if self.last_ids.is_empty() && !self.seen_empty {
                Generation::Initial
            } else {
                Generation::Respawn
            }
        } else if self.quiesced || self.attempts > 2 {
            Generation::Persistent
        } else {
            Generation::Retry
        };
        self.last_ids = ids;
        generation
    }
    fn attempted(&mut self, success: bool) {
        self.attempts = self.attempts.saturating_add(1);
        if success {
            self.quiesced = true;
            self.attempts = 0
        }
    }
}
fn quiet_timeout(
    r: &impl Runner,
    ctx: &ProcessContext,
    writers: Vec<ProcessEntry>,
    last_signal_error: Option<String>,
) -> ProcessError {
    let kind = if writers.is_empty() {
        QuietTimeoutKind::UnconfirmedWindow
    } else if is_codex_app_running(r, ctx) {
        QuietTimeoutKind::QuitBlocked
    } else {
        QuietTimeoutKind::OrphanWriter
    };
    ProcessError::QuietTimeout {
        kind,
        writers,
        last_signal_error,
    }
}
/// Require a continuous 700 ms empty interval, sampling at 80 ms. A successfully
/// quiesced generation is not signaled again; new PIDs reset that decision.
pub fn enforce_quiet_window(
    r: &impl Runner,
    ctx: &ProcessContext,
    timeout: Duration,
    initially_quiet: bool,
    policy: ProcessPolicy,
) -> Result<u32, ProcessError> {
    let start = r.now();
    let mut tracker = WriterGenerationTracker::new(initially_quiet);
    let mut empty_since = None;
    let mut respawns = 0u32;
    let mut last_signal_error = None;
    loop {
        let now = r.now();
        let writers = collect_codex_kill_targets(r, ctx)?;
        let generation = tracker.observe(&writers);
        if generation == Generation::Respawn {
            respawns = respawns.saturating_add(1)
        }
        if matches!(
            generation,
            Generation::Initial | Generation::Retry | Generation::Respawn
        ) {
            let result = quiesce_transition_targets(r, ctx, &writers, policy);
            tracker.attempted(result.is_ok());
            if let Err(e) = result {
                last_signal_error = Some(e.to_string());
                if tracker.attempts > 2 {
                    return Err(quiet_timeout(r, ctx, writers, last_signal_error));
                }
            } else {
                last_signal_error = None
            }
        }
        if writers.is_empty() {
            let since = empty_since.get_or_insert(now);
            if now.saturating_sub(*since) >= Duration::from_millis(700) {
                check_protected(r, ctx)?;
                return Ok(respawns);
            }
        } else {
            empty_since = None
        }
        if now.saturating_sub(start) >= timeout {
            return Err(quiet_timeout(r, ctx, writers, last_signal_error));
        }
        r.sleep(Duration::from_millis(80))
    }
}
/// Validates the bundle executable, opens it, observes a real main PID, then
/// activates the bundle again. Both open failures propagate in 1.2.7.
pub fn launch_codex_app(r: &impl Runner, ctx: &ProcessContext) -> Result<u32, ProcessError> {
    let bundle = codex_app_bundle_path(r, ctx)
        .ok_or_else(|| ProcessError::NotFound("Codex/ChatGPT app bundle not found".into()))?;
    let name = bundle_value(r, &bundle, BundleField::Executable).ok_or_else(|| {
        ProcessError::InvalidBundle(format!(
            "app bundle has no CFBundleExecutable: {}",
            bundle.display()
        ))
    })?;
    let main = bundle.join("Contents/MacOS").join(name);
    if !r.is_file(&main) {
        return Err(ProcessError::NotFound(format!(
            "app bundle executable not found: {}",
            main.display()
        )));
    }
    checked_output(r, &Operation::OpenBundle(bundle.clone()), "open")?;
    let start = r.now();
    let pid = loop {
        if let Some(pid) = main_process_pid(r, &main) {
            break Some(pid);
        }
        if r.now().saturating_sub(start) >= Duration::from_secs(10) {
            break main_process_pid(r, &main);
        }
        r.sleep(Duration::from_millis(100))
    }
    .ok_or_else(|| ProcessError::LaunchTimeout(main))?;
    checked_output(r, &Operation::OpenBundle(bundle), "open")?;
    Ok(pid)
}

/// One native background fence poll. Initial seen_empty=true counts the first
/// observed writer during a protected transition as a respawn.
#[derive(Debug)]
pub struct FencePoller {
    tracker: WriterGenerationTracker,
    respawns: u32,
    tainted: bool,
}
impl Default for FencePoller {
    fn default() -> Self {
        Self {
            tracker: WriterGenerationTracker::new(true),
            respawns: 0,
            tainted: false,
        }
    }
}
impl FencePoller {
    pub fn respawns(&self) -> u32 {
        self.respawns
    }
    pub fn tainted(&self) -> bool {
        self.tainted
    }
    pub fn poll(
        &mut self,
        r: &impl Runner,
        ctx: &ProcessContext,
        policy: ProcessPolicy,
    ) -> Result<Generation, ProcessError> {
        let writers = match collect_codex_kill_targets(r, ctx) {
            Ok(p) => p,
            Err(e) => {
                self.tainted = true;
                return Err(e);
            }
        };
        let generation = self.tracker.observe(&writers);
        if generation == Generation::Respawn {
            self.respawns = self.respawns.wrapping_add(1)
        }
        if matches!(
            generation,
            Generation::Initial | Generation::Retry | Generation::Respawn
        ) {
            let result = quiesce_transition_targets(r, ctx, &writers, policy);
            self.tracker.attempted(result.is_ok());
            if let Err(e) = result {
                self.tainted = true;
                return Err(e);
            }
        }
        Ok(generation)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FenceMode {
    Strict,
    BestEffort,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FenceReport {
    pub respawns: u32,
    pub tainted: bool,
}
use std::sync::{
    atomic::{AtomicBool, AtomicU32, Ordering},
    Arc,
};
/// An actual named worker remains active while the caller performs a transition.
/// `finish` stops/joins it and performs the final native sweep. Dropping without
/// finish stops/joins but does not claim a successful final quiet check.
pub struct CodexTransitionFence<R: Runner + Send + Sync + 'static> {
    runner: Arc<R>,
    ctx: ProcessContext,
    policy: ProcessPolicy,
    mode: FenceMode,
    initially_quiet: bool,
    stop: Arc<AtomicBool>,
    respawns: Arc<AtomicU32>,
    tainted: Arc<AtomicBool>,
    worker: Option<std::thread::JoinHandle<()>>,
}
impl<R: Runner + Send + Sync + 'static> CodexTransitionFence<R> {
    pub fn acquire(
        runner: Arc<R>,
        ctx: ProcessContext,
        graceful_timeout: Duration,
        mode: FenceMode,
        policy: ProcessPolicy,
    ) -> Result<Self, ProcessError> {
        let prepare = prepare_transition(runner.as_ref(), &ctx, graceful_timeout, policy);
        let mut tainted = prepare.is_err();
        if mode == FenceMode::Strict {
            prepare?
        }
        let quiet = enforce_quiet_window(
            runner.as_ref(),
            &ctx,
            Duration::from_secs(if mode == FenceMode::Strict { 12 } else { 2 }),
            false,
            policy,
        );
        let initially_quiet = quiet.is_ok();
        if mode == FenceMode::Strict {
            quiet?;
        } else if quiet.is_err() {
            tainted = true;
        }
        let stop = Arc::new(AtomicBool::new(false));
        let respawns = Arc::new(AtomicU32::new(0));
        let tainted = Arc::new(AtomicBool::new(tainted));
        let (r, c, s, n, t) = (
            runner.clone(),
            ctx.clone(),
            stop.clone(),
            respawns.clone(),
            tainted.clone(),
        );
        let worker = std::thread::Builder::new()
            .name("codex-transition-fence".into())
            .spawn(move || {
                let mut poller = FencePoller::default();
                while !s.load(Ordering::Acquire) {
                    let _ = poller.poll(r.as_ref(), &c, policy);
                    n.store(poller.respawns(), Ordering::Release);
                    if poller.tainted() {
                        t.store(true, Ordering::Release)
                    }
                    r.sleep(Duration::from_millis(80));
                }
            })
            .map_err(|e| ProcessError::Command {
                operation: "CODEX_TRANSITION_FENCE_START_FAILED",
                detail: e.to_string(),
            })?;
        Ok(Self {
            runner,
            ctx,
            policy,
            mode,
            initially_quiet,
            stop,
            respawns,
            tainted,
            worker: Some(worker),
        })
    }
    pub fn initially_quiet(&self) -> bool {
        self.initially_quiet
    }
    fn halt(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(worker) = self.worker.take() {
            if worker.join().is_err() {
                self.tainted.store(true, Ordering::Release)
            }
        }
    }
    pub fn finish(mut self) -> Result<FenceReport, ProcessError> {
        self.halt();
        let final_sweep = (|| {
            let writers = collect_codex_kill_targets(self.runner.as_ref(), &self.ctx)?;
            if writers.is_empty() {
                check_protected(self.runner.as_ref(), &self.ctx)?;
                Ok(0)
            } else {
                enforce_quiet_window(
                    self.runner.as_ref(),
                    &self.ctx,
                    Duration::from_secs(4),
                    true,
                    self.policy,
                )
            }
        })();
        let final_respawns = match final_sweep {
            Ok(n) => n,
            Err(e) => {
                if self.mode == FenceMode::Strict {
                    return Err(e);
                }
                self.tainted.store(true, Ordering::Release);
                0
            }
        };
        Ok(FenceReport {
            respawns: self
                .respawns
                .load(Ordering::Acquire)
                .saturating_add(final_respawns),
            tainted: self.tainted.load(Ordering::Acquire),
        })
    }
}
impl<R: Runner + Send + Sync + 'static> Drop for CodexTransitionFence<R> {
    fn drop(&mut self) {
        self.halt()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelaunchRequest {
    LeaveClosed,
    RestorePreviousRunningState,
}
/// Captures the initial running state from the runner, only when requested.
/// @100680d9c..100680e44 constructs these exact two policy fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CodexRelaunchPolicy {
    requested: bool,
    was_running: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RelaunchOutcome {
    Skipped,
    Started(u32),
    Warning(ProcessError),
}
impl CodexRelaunchPolicy {
    pub fn capture(r: &impl Runner, ctx: &ProcessContext, request: RelaunchRequest) -> Self {
        let requested = request == RelaunchRequest::RestorePreviousRunningState;
        Self {
            requested,
            was_running: requested && is_codex_app_running(r, ctx),
        }
    }
    /// The Manager supplies its actual verification Result. Failures never launch.
    /// Launch failure is a warning after a verified transition, as @10068ee30.
    pub fn after_verification_result<T, E>(
        &self,
        r: &impl Runner,
        ctx: &ProcessContext,
        verification: &Result<T, E>,
    ) -> RelaunchOutcome {
        if verification.is_err() || !self.requested {
            return RelaunchOutcome::Skipped;
        }
        let running = is_codex_app_running(r, ctx);
        if !self.was_running || running {
            return RelaunchOutcome::Skipped;
        }
        match launch_codex_app(r, ctx) {
            Ok(pid) => RelaunchOutcome::Started(pid),
            Err(e) => RelaunchOutcome::Warning(e),
        }
    }
}
