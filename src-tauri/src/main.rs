#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod accounts;
mod desktop_bridge;
mod gateway;
mod plugins;

fn main() {
    if std::env::var_os("ATELIER_DESKTOP_BRIDGE").as_deref() == Some(std::ffi::OsStr::new("1")) {
        std::process::exit(desktop_bridge::cli());
    }
    tauri::Builder::default()
        .manage(gateway::GatewayState::default())
        .manage(accounts::AccountsState::default())
        .invoke_handler(tauri::generate_handler![
            plugins::behavior_plugins_status,
            plugins::behavior_plugin_install,
            plugins::behavior_plugin_set,
            plugins::behavior_plugin_rollback,
            desktop_bridge::desktop_bridge_launch,
            accounts::accounts_snapshot,
            accounts::accounts_login_start,
            accounts::accounts_login_poll,
            accounts::accounts_login_cancel,
            accounts::accounts_login_open,
            accounts::accounts_import,
            accounts::accounts_detect_current,
            accounts::accounts_refresh,
            accounts::accounts_reset_credit_preview,
            accounts::accounts_edit,
            accounts::accounts_group,
            accounts::accounts_export,
            accounts::accounts_remove,
            accounts::accounts_switch_preview,
            accounts::accounts_switch_apply,
            gateway::gateway_snapshot,
            gateway::gateway_provider_save,
            gateway::gateway_profile_update,
            gateway::gateway_profile_remove,
            gateway::gateway_settings_save,
            gateway::gateway_profile_select,
            gateway::gateway_start,
            gateway::gateway_stop,
            gateway::gateway_local_key,
            gateway::gateway_models_refresh,
            gateway::gateway_oauth_begin,
            gateway::gateway_oauth_poll,
            gateway::gateway_oauth_cancel,
            gateway::gateway_oauth_signout,
            gateway::gateway_config_preview,
            gateway::gateway_launch_desktop,
            gateway::gateway_open_login
        ])
        .run(tauri::generate_context!())
        .expect("failed to start Codex Atelier");
}
