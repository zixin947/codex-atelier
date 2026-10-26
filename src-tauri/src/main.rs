#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod plugins;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            plugins::behavior_plugins_status,
            plugins::behavior_plugin_install,
            plugins::behavior_plugin_set,
            plugins::behavior_plugin_rollback
        ])
        .run(tauri::generate_context!())
        .expect("failed to start Codex Atelier");
}
