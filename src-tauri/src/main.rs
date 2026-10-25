#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main() {
    // No shell, filesystem, credential, network, or configuration commands exposed.
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("failed to start Codex Atelier");
}
