//! Installed-app diagnostic contract, implemented against Atelier's storage.
//! Scanning never invokes upstream repair/recovery code or writes Codex files.
mod access_checks;
#[cfg(test)]
mod auth_workflow_tests;
mod config_syntax;
mod desktop_window;
mod credential_checks;
mod directory_probe;
mod inspect;
mod history_recovery;
mod history_recovery_workflow;
mod native_auth;
#[cfg(test)]
mod native_auth_tests;
mod native_batch;
mod native_catalog;
mod native_config;
mod native_coordinator;
mod native_manager;
mod native_orphans;
mod native_router;
mod native_threads;
#[cfg(test)]
mod native_threads_tests;
#[cfg(test)]
mod native_workflow_tests;
pub(crate) mod official_catalog;
mod poisoned_instructions;
#[cfg(test)]
mod poisoned_workflow_tests;
mod project_state;
mod quiet_window;
mod record_compat;
mod repair;
mod residual_cleanup;
#[cfg(test)]
mod residual_workflow_tests;
mod route_mode;
mod router_intent;
#[cfg(test)]
mod router_intent_tests;
mod snapshot;
mod state_db_relocation;
#[cfg(test)]
mod syntax_workflow_tests;
#[cfg(test)]
mod tests;
mod thread_observation;
mod toml_lexical;
mod wal;
use crate::gateway::types::now_ms;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Mutex,
};
use tauri::Manager;
type Result<T> = std::result::Result<T, String>;
fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticItem {
    pub id: String,
    pub group: String,
    pub status: String,
    pub label: String,
    pub detail: String,
    pub fixable: bool,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticReport {
    pub mode: String,
    pub items: Vec<DiagnosticItem>,
    pub has_issues: bool,
    pub transition_in_flight: bool,
    pub checked_at: u64,
    pub root: String,
}
#[derive(Clone, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticRepairPlan {
    pub ticket: String,
    pub item_ids: Vec<String>,
    pub title: String,
    pub changes: Vec<String>,
    pub warnings: Vec<String>,
    pub requires_desktop_closed: bool,
}
#[derive(Clone, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticFixResult {
    pub fixed_ids: Vec<String>,
    pub failed_ids: Vec<String>,
    pub details: Vec<String>,
    pub backup_path: Option<String>,
    pub report: DiagnosticReport,
}
#[derive(Default)]
pub struct DiagnosticState {
    tickets: Mutex<HashMap<String, native_manager::Reviewed>>,
    probe_tickets: Mutex<HashMap<String, directory_probe::ReviewedProbe>>,
    gate: Mutex<()>,
}
#[derive(Clone)]
struct Context {
    home: PathBuf,
    app_root: PathBuf,
    desktop_running: bool,
    runtime: Option<crate::gateway::types::RuntimeView>,
    proxy_handle: Option<atelier_maintenance_core::router_verification::ProxyHandle>,
    official_catalog: Option<std::sync::Arc<official_catalog::OfficialCatalogRuntime>>,
    working_directory: Option<PathBuf>,
    sqlite_home_env: Option<std::ffi::OsString>,
}
impl Context {
    fn native(app: &tauri::AppHandle) -> Result<Self> {
        use super::process::Runner;
        let home = std::env::var_os("CODEX_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|v| PathBuf::from(v).join(".codex")))
            .ok_or("无法定位 Codex 目录")?;
        let desktop_running = if let Some(p) = super::process::discover() {
            super::process::SystemRunner.running(&p)?
        } else {
            false
        };
        #[cfg(target_os = "macos")]
        let desktop_running = desktop_running
            || std::process::Command::new("/bin/ps")
                .args(["-axo", "comm="])
                .output()
                .map_err(|_| "无法核对 Codex 进程")?
                .stdout
                .split(|b| *b == b'\n')
                .filter_map(|l| std::str::from_utf8(l).ok())
                .any(|line| {
                    Path::new(line.trim())
                        .file_name()
                        .is_some_and(|n| n == "codex")
                });
        Ok(Self {
            home,
            app_root: app.path().app_data_dir().map_err(err)?,
            desktop_running,
            runtime: crate::gateway::diagnostic_runtime(app),
            proxy_handle: crate::gateway::diagnostic_proxy_handle(app),
            official_catalog: crate::gateway::diagnostic_official_catalog(app),
            working_directory: std::env::current_dir().ok(),
            sqlite_home_env: std::env::var_os("CODEX_SQLITE_HOME"),
        })
    }
    fn config(&self) -> PathBuf {
        self.home.join("config.toml")
    }
}
fn read_bounded(path: &Path, limit: u64) -> Result<Vec<u8>> {
    use std::io::Read;
    let meta = std::fs::symlink_metadata(path).map_err(|_| "文件不存在或无法读取")?;
    if !meta.is_file() || meta.file_type().is_symlink() {
        return Err("不支持符号链接或非普通文件".into());
    }
    if meta.len() > limit {
        return Err("文件超过诊断读取上限".into());
    }
    let mut bytes = vec![];
    std::fs::File::open(path)
        .map_err(|_| "无法读取文件")?
        .take(limit + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "读取文件失败")?;
    if bytes.len() as u64 > limit {
        return Err("读取期间文件超过上限".into());
    }
    Ok(bytes)
}
/// Account switching shares maintenance's writer locks without process control.
pub(crate) struct AccountWriteGuard {
    guard: repair::MutationGuard,
    context: Context,
}
impl AccountWriteGuard {
    pub(crate) fn validate(&self) -> Result<()> {
        self.guard.validate(&self.context)
    }
}
pub(crate) fn account_write_guard(home: &Path, app_root: &Path) -> Result<AccountWriteGuard> {
    let context = Context {
        home: home.into(),
        app_root: app_root.into(),
        desktop_running: false,
        runtime: None,
        proxy_handle: None,
        official_catalog: None,
        working_directory: None,
        sqlite_home_env: None,
    };
    let guard = repair::acquire(&context)?;
    Ok(AccountWriteGuard { guard, context })
}
/// A routing preference, not a Full-verification receipt. No host is constructed.
pub(crate) fn official_account_intent(
    before: Option<&[u8]>,
    catalog: Option<&[u8]>,
) -> Result<Vec<u8>> {
    use atelier_maintenance_core::transition_journal::TransitionTarget;
    if let Some(bytes) = before {
        router_intent::decode(bytes)?;
    }
    use sha2::{Digest, Sha256};
    let record = router_intent::IntentRecord {
        schema_version: 1,
        target: TransitionTarget::NativeOff,
        provider: None,
        operation_id: None,
        catalog_revision: catalog.map(|b| format!("{:x}", Sha256::digest(b))),
        updated_at_ms: now_ms() as i64,
    };
    serde_json::to_vec_pretty(&record).map_err(err)
}
/// Explicit store paths only; never discovers a home or constructs a gateway.
pub(crate) fn gateway_route_target(
    intent: &Path,
    config: Option<&Path>,
) -> Result<atelier_maintenance_core::transition_journal::TransitionTarget> {
    use atelier_maintenance_core::transition_journal::TransitionTarget;
    match std::fs::symlink_metadata(intent) {
        Ok(_) => return Ok(router_intent::decode(&read_bounded(intent, 1024 * 1024)?)?.target),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
        Err(e) => return Err(err(e)),
    }
    let Some(path) = config else {
        return Ok(TransitionTarget::NativeOff);
    };
    let bytes = match std::fs::symlink_metadata(path) {
        Ok(_) => read_bounded(path, 1024 * 1024)?,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Ok(TransitionTarget::NativeOff)
        }
        Err(e) => return Err(err(e)),
    };
    let config: toml::Value =
        toml::from_str(std::str::from_utf8(&bytes).map_err(|_| "路由配置不是UTF8")?)
            .map_err(|_| "路由配置不是有效TOML")?;
    Ok(
        if router_intent::owns_provider(&inspect::provider(&config)) {
            TransitionTarget::RouterOn
        } else {
            TransitionTarget::NativeOff
        },
    )
}
#[tauri::command]
pub async fn maintenance_run_diagnostics(app: tauri::AppHandle) -> Result<DiagnosticReport> {
    tauri::async_runtime::spawn_blocking(move || {
        let context = Context::native(&app)?;
        let report = native_manager::report(&context)?;
        let _ = super::utilities::record_diagnostic_summary(&context.app_root, &report);
        Ok(report)
    })
    .await
    .map_err(err)?
}
#[tauri::command]
pub async fn maintenance_preview_repair(
    app: tauri::AppHandle,
    item_id: String,
) -> Result<DiagnosticRepairPlan> {
    tauri::async_runtime::spawn_blocking(move || {
        let context = Context::native(&app)?;
        let ticket = native_manager::preview(&context, &item_id)?;
        let state = app.state::<DiagnosticState>();
        let mut tickets = state.tickets.lock().map_err(|_| "诊断状态不可用")?;
        tickets.retain(|_, t| now_ms().saturating_sub(t.created_at) < 300_000);
        if tickets.len() >= 16 {
            return Err("待确认修复过多，请关闭旧预览后重试".into());
        }
        let view = ticket.view.clone();
        tickets.insert(view.ticket.clone(), ticket);
        Ok(view)
    })
    .await
    .map_err(err)?
}
#[tauri::command]
pub async fn maintenance_preview_directory_probe(
    app: tauri::AppHandle,
) -> Result<DiagnosticRepairPlan> {
    tauri::async_runtime::spawn_blocking(move || {
        let context = Context::native(&app)?;
        let reviewed = directory_probe::preview(&context)?;
        let state = app.state::<DiagnosticState>();
        let mut tickets = state.probe_tickets.lock().map_err(|_| "诊断状态不可用")?;
        tickets.retain(|_, t| now_ms().saturating_sub(t.created_at) < 300_000);
        if tickets.len() >= 16 {
            return Err("待确认检查过多，请关闭旧预览后重试".into());
        }
        let view = reviewed.view.clone();
        tickets.insert(view.ticket.clone(), reviewed);
        Ok(view)
    })
    .await
    .map_err(err)?
}
#[tauri::command]
pub async fn maintenance_apply_directory_probe(
    app: tauri::AppHandle,
    ticket: String,
    confirmed: bool,
) -> Result<DiagnosticFixResult> {
    if !confirmed {
        return Err("需要明确确认目录写入检查范围".into());
    }
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<DiagnosticState>();
        let _guard = state
            .gate
            .try_lock()
            .map_err(|_| "另一项维护操作正在执行")?;
        let reviewed = state
            .probe_tickets
            .lock()
            .map_err(|_| "诊断状态不可用")?
            .remove(&ticket)
            .ok_or("目录检查预览不存在或已执行")?;
        directory_probe::apply(&Context::native(&app)?, reviewed)
    })
    .await
    .map_err(err)?
}
#[tauri::command]
pub async fn maintenance_apply_repair(
    app: tauri::AppHandle,
    ticket: String,
    confirmed: bool,
) -> Result<DiagnosticFixResult> {
    if !confirmed {
        return Err("需要确认修复范围".into());
    }
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<DiagnosticState>();
        let _guard = state.gate.try_lock().map_err(|_| "另一项修复正在执行")?;
        let t = state
            .tickets
            .lock()
            .map_err(|_| "诊断状态不可用")?
            .remove(&ticket)
            .ok_or("修复预览不存在或已执行")?;
        let context = Context::native(&app)?;
        let result = native_manager::apply_native(&app, &context, t)?;
        let _ = super::utilities::record_diagnostic_summary(&context.app_root, &result.report);
        Ok(result)
    })
    .await
    .map_err(err)?
}
