//! Installed-app diagnostic contract, implemented against Atelier's storage.
//! Scanning never invokes upstream repair/recovery code or writes Codex files.
mod inspect;
mod project_state;
mod record_compat;
mod repair;
mod snapshot;
#[cfg(test)]
mod tests;
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
    tickets: Mutex<HashMap<String, repair::Ticket>>,
    gate: Mutex<()>,
}
#[derive(Clone)]
struct Context {
    home: PathBuf,
    app_root: PathBuf,
    desktop_running: bool,
    runtime: Option<crate::gateway::types::RuntimeView>,
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
#[tauri::command]
pub async fn maintenance_run_diagnostics(app: tauri::AppHandle) -> Result<DiagnosticReport> {
    tauri::async_runtime::spawn_blocking(move || {
        let context = Context::native(&app)?;
        let report = inspect::run(&context)?;
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
        let ticket = repair::preview(&context, &item_id)?;
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
        let result = repair::apply(&context, t)?;
        let _ = super::utilities::record_diagnostic_summary(&context.app_root, &result.report);
        Ok(result)
    })
    .await
    .map_err(err)?
}
