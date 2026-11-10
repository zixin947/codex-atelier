use super::*;
use cockpit_session_compat as core;
use tauri::Emitter;
fn host<T>(app: &tauri::AppHandle, work: impl FnOnce() -> Result<T>) -> Result<T> {
    let s = Store::native(app)?;
    core::with_host(&s.home, &s.backups.join("cockpit"), crate::desktop_runtime::binary().as_deref(), work)
}
#[tauri::command]
pub async fn codex_list_session_visibility_repair_instances(app: tauri::AppHandle) -> Result<core::CodexSessionVisibilityRepairInstanceList> {
    tauri::async_runtime::spawn_blocking(move || host(&app, core::list_session_visibility_repair_instances)).await.map_err(err)?
}
#[tauri::command]
pub async fn codex_list_session_visibility_repair_providers(app: tauri::AppHandle) -> Result<core::CodexSessionVisibilityRepairProviderList> {
    tauri::async_runtime::spawn_blocking(move || host(&app, core::list_session_visibility_repair_providers)).await.map_err(err)?
}
#[tauri::command]
pub async fn codex_repair_session_visibility_across_instances(app: tauri::AppHandle, mode: core::CodexSessionVisibilityRepairMode, run_id: Option<String>, dry_run: Option<bool>, target_provider: Option<String>, target_instance_id: Option<String>, repair_instance_ids: Option<Vec<String>>, session_ids: Option<Vec<String>>) -> Result<core::CodexSessionVisibilityRepairSummary> {
    tauri::async_runtime::spawn_blocking(move || {
        host(&app, || {
        let target_provider = if mode == core::CodexSessionVisibilityRepairMode::History { None } else {
            match target_provider.map(|p| p.trim().to_owned()).filter(|p| !p.is_empty()) {
                Some(p) => Some(p),
                None => match target_instance_id.as_deref().map(str::trim).filter(|p| !p.is_empty()) {
                    Some(id) => Some(core::resolve_session_visibility_target_provider_from_instance_id(id)?),
                    None => None,
                },
            }
        };
        core::repair_session_visibility_across_instances_with_target(mode, run_id, Some(&|progress| { let _ = app.emit("codex:session_visibility_repair_progress", progress); }), target_provider, session_ids, repair_instance_ids, dry_run.unwrap_or(false)) })
    }).await.map_err(err)?
}
