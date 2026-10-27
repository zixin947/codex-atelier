#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod desktop_bridge;
mod plugins;

fn main() {
    if std::env::var_os("ATELIER_DESKTOP_BRIDGE").as_deref() == Some(std::ffi::OsStr::new("1")) {
        std::process::exit(desktop_bridge::cli());
    }
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            plugins::behavior_plugins_status,
            plugins::behavior_plugin_install,
            plugins::behavior_plugin_set,
            plugins::behavior_plugin_rollback,
            desktop_bridge::desktop_bridge_launch
        ])
        .run(tauri::generate_context!())
        .expect("failed to start Codex Atelier");
}
