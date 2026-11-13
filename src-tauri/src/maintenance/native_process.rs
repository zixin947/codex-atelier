//! Installed command wrappers, with a reviewable Atelier process ticket.
//! Every OS effect is delegated to the recovered engine and explicit Runner.
use super::super::{result, ActionResult, Check, Result};
use atelier_maintenance_core::process::{self as native, ProcessContext, ProcessEntry, Runner};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{path::PathBuf, time::Duration};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Action {
    ForceKill,
    Restart,
}
impl Action {
    fn parse(value: &str) -> Result<Self> {
        match value {
            "forceKill" => Ok(Self::ForceKill),
            "restart" => Ok(Self::Restart),
            _ => Err("进程维护操作无效".into()),
        }
    }
    fn id(self) -> &'static str {
        match self {
            Self::ForceKill => "forceKill",
            Self::Restart => "restart",
        }
    }
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProcessTarget {
    pub pid: u32,
    /// Executable display name only; command arguments may contain session text.
    pub name: String,
}
impl From<&ProcessEntry> for ProcessTarget {
    fn from(p: &ProcessEntry) -> Self {
        Self {
            pid: p.pid,
            name: native::extract_process_name(&p.command),
        }
    }
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProcessPreview {
    pub action: String,
    pub scope: String,
    pub targets: Vec<ProcessTarget>,
    pub external_cli: Vec<ProcessTarget>,
    pub protected_db_holders: Vec<ProcessTarget>,
    pub codex_home: PathBuf,
    pub source_directories: Vec<PathBuf>,
    pub source_databases: Vec<PathBuf>,
    pub bundle: Option<PathBuf>,
}
/// Private full-command identities never cross IPC or Debug output.
#[derive(Clone)]
pub(crate) struct PreparedAction {
    action: Action,
    context: ProcessContext,
    preview: ProcessPreview,
    identities: Vec<ProcessEntry>,
    config_digest: Option<[u8; 32]>,
    main: Option<PathBuf>,
}
impl PreparedAction {
    pub(crate) fn preview(&self) -> &ProcessPreview {
        &self.preview
    }
    pub(crate) fn preview_lines(&self) -> Vec<String> {
        let mut lines = vec![self.preview.scope.clone()];
        lines.push(format!(
            "当前目标 {} 个，外部 CLI {} 个，受保护数据库持有者 {} 个。",
            self.preview.targets.len(),
            self.preview.external_cli.len(),
            self.preview.protected_db_holders.len()
        ));
        for p in &self.preview.targets {
            lines.push(format!("目标 PID {}：{}", p.pid, p.name));
        }
        for directory in &self.preview.source_directories {
            lines.push(format!("状态数据库来源目录：{}", directory.display()));
        }
        for database in &self.preview.source_databases {
            lines.push(format!("已发现状态数据库：{}", database.display()));
        }
        if let Some(bundle) = &self.preview.bundle {
            lines.push(format!("Desktop 应用：{}", bundle.display()));
        }
        lines
    }
    pub(crate) fn warnings(&self) -> Vec<String> {
        let mut warnings = vec!["此操作覆盖原生识别的全部 Codex 相关进程，包括 CLI；正在生成的任务会中断，未保存内容可能丢失。".into()];
        if !self.preview.protected_db_holders.is_empty() {
            warnings.push("发现受保护或归属未知的数据库持有者：不会结束这些进程；即使相关进程已退出，原生流程仍会报告失败。".into());
        }
        if self.action == Action::ForceKill {
            warnings.push("本操作只做优雅退出：发送退出请求后轮询等待，超时即中止并报告仍未退出的进程；不会发送任何结束信号，也不会重新启动客户端。".into());
        }
        if self.action == Action::Restart {
            warnings.push("重启会先请求正常退出，超时或仍有相关进程时中止，不强杀；确认退出后才启动。".into());
        }
        warnings
    }
}
fn map_error(error: native::ProcessError) -> String {
    use native::ProcessError;
    let safe = |rows: &[ProcessEntry]| {
        rows.iter()
            .map(|p| {
                format!(
                    "{} (pid {})",
                    native::extract_process_name(&p.command),
                    p.pid
                )
            })
            .collect::<Vec<_>>()
            .join(" | ")
    };
    match error {
        ProcessError::ProtectedDbHolders(rows) => {
            format!("codex_protected_db_holders: {}", safe(&rows))
        }
        ProcessError::ExternalWriters(rows) => format!("codex_external_writers: {}", safe(&rows)),
        other => other.to_string(),
    }
}
fn config(r: &impl Runner, ctx: &ProcessContext) -> Option<String> {
    r.read_text(&ctx.codex_home.join("config.toml")).ok()
}
fn digest(text: Option<&str>) -> Option<[u8; 32]> {
    text.map(|s| Sha256::digest(s.as_bytes()).into())
}
fn source_directories(ctx: &ProcessContext, text: Option<&str>) -> Vec<PathBuf> {
    // Same precedence and non-expansion contract as native runtime_state_db_candidates.
    let configured = text
        .and_then(|s| toml::from_str::<toml::Value>(s).ok())
        .and_then(|v| {
            v.get("sqlite_home")
                .and_then(|v| v.as_str())
                .map(str::to_owned)
        })
        .filter(|s| !s.trim().is_empty())
        .map(|s| PathBuf::from(s.trim()))
        .or_else(|| {
            ctx.sqlite_home_env
                .as_ref()
                .filter(|s| !s.is_empty())
                .map(PathBuf::from)
        });
    match configured {
        Some(p) => vec![if p.is_relative() {
            ctx.cwd.as_ref().map(|cwd| cwd.join(&p)).unwrap_or(p)
        } else {
            p
        }],
        None => vec![ctx.codex_home.join("sqlite"), ctx.codex_home.clone()],
    }
}
fn full_targets(
    r: &impl Runner,
    ctx: &ProcessContext,
    main: Option<&std::path::Path>,
) -> Result<Vec<ProcessEntry>> {
    let mut rows = native::list_codex_processes_matching(r, ctx).map_err(map_error)?;
    for row in native::list_codex_db_holder_processes(r, ctx) {
        if native::is_codex_related_command_with_main(&row.command, main)
            && !rows.iter().any(|p| p.pid == row.pid)
        {
            rows.push(row);
        }
    }
    rows.sort_by(|a, b| a.pid.cmp(&b.pid).then(a.command.cmp(&b.command)));
    Ok(rows)
}
pub(crate) fn prepare(
    ctx: &ProcessContext,
    r: &impl Runner,
    action: &str,
) -> Result<PreparedAction> {
    let action = Action::parse(action)?;
    let bundle = native::codex_app_bundle_path(r, ctx);
    let main = bundle
        .as_ref()
        .and_then(|p| native::bundle_main_executable(r, p));
    if action == Action::Restart && main.is_none() {
        return Err(
            "未找到通过原生 bundle ID 与 executable 校验的 Desktop 安装，无法预览重启".into(),
        );
    }
    let stop =
        native::preview_stop(r, ctx, native::ProcessPolicy::ForceCloseAll).map_err(map_error)?;
    let identities = full_targets(r, ctx, main.as_deref())?;
    let text = config(r, ctx);
    let preview = ProcessPreview {
        action: action.id().into(),
        scope: match action {
            Action::ForceKill => "请求 Codex 正常退出并轮询等待；超时未退出即中止并报告仍存在的相关进程（含 CLI），不会强制结束任何进程，也不会自动启动客户端。",
            Action::Restart => "按原版流程重启：正常退出等待5秒，随后执行 policy0 全相关进程强制退出，再启动并确认主进程，最后激活应用。",
        }.into(),
        targets: identities.iter().map(ProcessTarget::from).collect(),
        external_cli: stop.external_cli.iter().map(ProcessTarget::from).collect(),
        protected_db_holders: stop.protected_db_holders.iter().map(ProcessTarget::from).collect(),
        codex_home: ctx.codex_home.clone(),
        source_directories: source_directories(ctx, text.as_deref()),
        source_databases: native::runtime_state_db_candidates(r, ctx),
        bundle,
    };
    Ok(PreparedAction {
        action,
        context: ctx.clone(),
        preview,
        identities,
        config_digest: digest(text.as_deref()),
        main,
    })
}
pub(crate) fn validate(plan: &PreparedAction, r: &impl Runner) -> Result<()> {
    let ctx = &plan.context;
    let text = config(r, ctx);
    let bundle = native::codex_app_bundle_path(r, ctx);
    let main = bundle
        .as_ref()
        .and_then(|p| native::bundle_main_executable(r, p));
    if digest(text.as_deref()) != plan.config_digest
        || source_directories(ctx, text.as_deref()) != plan.preview.source_directories
        || native::runtime_state_db_candidates(r, ctx) != plan.preview.source_databases
        || bundle != plan.preview.bundle
        || main != plan.main
    {
        return Err("进程维护来源目录、状态数据库或安装已变化，请重新预览".into());
    }
    // Exited targets need no signal; a new/recycled identity requires a fresh preview.
    if full_targets(r, ctx, main.as_deref())?
        .iter()
        .any(|p| !plan.identities.contains(p))
    {
        return Err("预览后出现新的目标进程或PID身份已变化，请重新预览".into());
    }
    Ok(())
}
pub(crate) fn apply(plan: &PreparedAction, r: &impl Runner) -> Result<ActionResult> {
    validate(plan, r)?;
    let ctx = &plan.context;
    match plan.action {
        Action::ForceKill => {
            // 只请求优雅退出：轮询等待，超时即中止并如实报错。
            // 这里绝不发送结束信号，因此不会出现「强制关闭」式的 kill -9。
            let mut attempted = false;
            if native::is_codex_app_running(r, ctx) {
                r.output(&native::Operation::QuitApp)
                    .map_err(|e| format!("无法请求 Codex 退出：{e}；未强杀任何进程"))?;
                attempted = true;
                let start = r.now();
                while native::is_codex_app_running(r, ctx) {
                    if r.now().saturating_sub(start) >= Duration::from_secs(10) {
                        break;
                    }
                    r.sleep(Duration::from_millis(100));
                }
            }
            let remaining = full_targets(r, ctx, plan.main.as_deref())?;
            if !remaining.is_empty() {
                let names = remaining
                    .iter()
                    .take(8)
                    .map(|p| format!("{} (pid {})", native::extract_process_name(&p.command), p.pid))
                    .collect::<Vec<_>>()
                    .join(" | ");
                return Err(format!(
                    "仍有 {} 个 Codex 相关进程未退出：{}；已中止，未强杀任何进程，也未重新启动，请保存工作后手动退出再试。",
                    remaining.len(),
                    names
                ));
            }
            if !native::protected_codex_db_holders(r, ctx).is_empty() {
                return Err("仍有受保护的数据库持有者；已中止，未强杀任何进程".into());
            }
            if !attempted && plan.identities.is_empty() {
                let mut out = result("forceKill", "未检测到正在运行的 Codex 相关进程；未发送任何信号，也未重新启动。".into(), 0, vec![], None);
                out.checks.push(Check::new("原生进程复检", "pass", "相关目标本就不存在。"));
                return Ok(out);
            }
            let count = plan.identities.len();
            let mut out = result("forceKill", format!("Codex 已正常退出：请求退出后轮询确认 {count} 个相关进程清空，未强杀进程，也未重新启动。"), count, vec![], None);
            out.checks.push(Check::new(
                "原生进程复检",
                "pass",
                "相关目标已清空，受保护数据库持有者检查通过；未强杀，未重新启动。",
            ));
            Ok(out)
        }
        Action::Restart => {
            let mut stop = plan.clone();
            stop.action = Action::ForceKill;
            apply(&stop, r).map_err(|e| format!("{e}；退出阶段未通过，未启动客户端"))?;
            let pid = native::launch_codex_app(r, ctx)
                .map_err(|e| format!("{}；退出已完成，启动阶段未通过", map_error(e)))?;
            let mut out = result(
                "restart",
                format!("Desktop 已按原版流程重新启动，已确认主进程 PID {pid} 并激活应用。"),
                1,
                vec![],
                None,
            );
            out.checks.push(Check::new(
                "原生启动复检",
                "pass",
                format!("已检测到通过bundle校验的主进程 PID {pid}；会话就绪由客户端核对。"),
            ));
            Ok(out)
        }
    }
}
fn native_context() -> Result<ProcessContext> {
    let user_home = std::env::var_os("HOME").map(PathBuf::from);
    let codex_home = std::env::var_os("CODEX_HOME")
        .map(PathBuf::from)
        .or_else(|| user_home.as_ref().map(|p| p.join(".codex")))
        .ok_or("无法定位Codex目录")?;
    Ok(ProcessContext {
        user_home,
        codex_home,
        sqlite_home_env: std::env::var_os("CODEX_SQLITE_HOME"),
        cwd: std::env::current_dir().ok(),
    })
}
pub(crate) fn prepare_native(action: &str) -> Result<PreparedAction> {
    prepare(&native_context()?, &native::SystemRunner::default(), action)
}
pub(crate) fn apply_native(plan: &PreparedAction) -> Result<ActionResult> {
    let ctx = native_context()?;
    if ctx.user_home != plan.context.user_home
        || ctx.codex_home != plan.context.codex_home
        || ctx.sqlite_home_env != plan.context.sqlite_home_env
        || ctx.cwd != plan.context.cwd
    {
        return Err("进程维护上下文已变化，请重新预览".into());
    }
    apply(plan, &native::SystemRunner::default())
}
