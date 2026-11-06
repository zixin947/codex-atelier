//! Desktop maintenance: inspection is read-only; mutations require a reviewed preview.
pub mod diagnostics;
pub mod image_config;
mod process;
mod store;
#[cfg(test)]
mod tests;
pub mod utilities;
use crate::gateway::types::now_ms;
use serde::Serialize;
use std::{collections::HashMap, path::PathBuf, sync::Mutex};
use tauri::{Manager, State};
type Result<T> = std::result::Result<T, String>;
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Check {
    pub label: String,
    pub status: String,
    pub detail: String,
}
impl Check {
    fn new(label: &str, status: &str, detail: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            status: status.into(),
            detail: detail.into(),
        }
    }
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub mode: String,
    pub platform: String,
    pub arch: String,
    pub version: String,
    pub account_count: usize,
    pub managed_root: String,
    pub desktop_path: Option<String>,
    pub checks: Vec<Check>,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    pub ticket: String,
    pub action: String,
    pub title: String,
    pub changes: Vec<String>,
    pub warnings: Vec<String>,
    pub count: usize,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionResult {
    pub action: String,
    pub completed_at: u64,
    pub summary: String,
    pub changed: usize,
    pub warnings: Vec<String>,
    pub checks: Vec<Check>,
    pub backup_path: Option<String>,
}
#[derive(Clone)]
struct Ticket {
    view: Plan,
    created_at: u64,
    root: PathBuf,
    fingerprint: String,
    desktop: Option<PathBuf>,
}
#[derive(Default)]
pub struct MaintenanceState {
    tickets: Mutex<HashMap<String, Ticket>>,
    gate: Mutex<()>,
}
fn root(app: &tauri::AppHandle) -> Result<PathBuf> {
    Ok(app
        .path()
        .app_data_dir()
        .map_err(|_| "无法定位 Atelier 数据目录")?
        .join("codex-accounts"))
}
fn result(
    action: &str,
    summary: String,
    changed: usize,
    warnings: Vec<String>,
    backup_path: Option<String>,
) -> ActionResult {
    ActionResult {
        action: action.into(),
        completed_at: now_ms(),
        summary,
        changed,
        warnings,
        checks: vec![],
        backup_path,
    }
}
#[tauri::command]
pub async fn maintenance_snapshot(app: tauri::AppHandle) -> Result<Snapshot> {
    let root = root(&app)?;
    tauri::async_runtime::spawn_blocking(move || store::snapshot(&root))
        .await
        .map_err(|e| e.to_string())?
}
#[tauri::command]
pub async fn maintenance_diagnose(app: tauri::AppHandle) -> Result<ActionResult> {
    let root = root(&app)?;
    tauri::async_runtime::spawn_blocking(move || {
        let s = store::snapshot(&root)?;
        let warnings = s
            .checks
            .iter()
            .filter(|c| c.status != "pass")
            .map(|c| format!("{}：{}", c.label, c.detail))
            .collect();
        Ok(ActionResult {
            action: "diagnose".into(),
            completed_at: now_ms(),
            summary: format!(
                "检查完成：{} / {} · {} 个账号。未联网验证凭据。",
                s.platform, s.arch, s.account_count
            ),
            changed: 0,
            warnings,
            checks: s.checks,
            backup_path: None,
        })
    })
    .await
    .map_err(|e| e.to_string())?
}
#[tauri::command]
pub async fn maintenance_preview(app: tauri::AppHandle, action: String) -> Result<Plan> {
    let root = root(&app)?;
    let app_clone = app.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let ticket = store::preview(&root, &action)?;
        let state = app_clone.state::<MaintenanceState>();
        let mut tickets = state.tickets.lock().map_err(|_| "维护状态不可用")?;
        tickets.retain(|_, t| now_ms().saturating_sub(t.created_at) < 300_000);
        if tickets.len() >= 16 {
            return Err("待确认操作过多，请稍后重试".into());
        }
        let view = ticket.view.clone();
        tickets.insert(view.ticket.clone(), ticket);
        Ok(view)
    })
    .await
    .map_err(|e| e.to_string())?
}
#[tauri::command]
pub async fn maintenance_apply(
    app: tauri::AppHandle,
    ticket: String,
    confirmed: bool,
) -> Result<ActionResult> {
    if !confirmed {
        return Err("需要明确确认维护操作".into());
    }
    tauri::async_runtime::spawn_blocking(move || {
        let state: State<'_, MaintenanceState> = app.state();
        let _gate = state.gate.try_lock().map_err(|_| "已有维护操作正在执行")?;
        let ticket = state
            .tickets
            .lock()
            .map_err(|_| "维护状态不可用")?
            .remove(&ticket)
            .ok_or("预览不存在或已经执行，请重新预览")?;
        if now_ms().saturating_sub(ticket.created_at) > 300_000 {
            return Err("预览已过期，请重新预览".into());
        }
        if ticket.root != root(&app)? {
            return Err("维护目标已变化".into());
        }
        if matches!(ticket.view.action.as_str(), "restart" | "forceKill") {
            let path = ticket.desktop.ok_or("没有可用的 Desktop 目标")?;
            if process::discover().as_ref() != Some(&path) {
                return Err("Desktop 路径已变化，请重新预览".into());
            }
            if ticket.view.action == "forceKill" {
                let count = process::force_kill(&path, &process::SystemForceRunner)?;
                return Ok(result(
                    "forceKill",
                    if count == 0 {
                        "未检测到目标 Desktop 进程，无需结束。".into()
                    } else {
                        format!("已结束 {count} 个目标 Desktop 进程；未重新启动。")
                    },
                    count,
                    vec![],
                    None,
                ));
            }
            process::restart(&path, &process::SystemRunner)?;
            return Ok(result(
                "restart",
                "Desktop 已重新启动（进程已确认；会话就绪需在客户端核对）。".into(),
                1,
                vec![],
                None,
            ));
        }
        store::apply(&ticket)
    })
    .await
    .map_err(|e| e.to_string())?
}
