#[path = "../../upstream/atomic_write.rs"]
pub mod atomic_write;
#[path = "../../upstream/codex_config_format.rs"]
pub mod codex_config_format;
#[path = "../../upstream/codex_history_projection_repair.rs"]
pub mod codex_history_projection_repair;
#[path = "../../upstream/codex_session_file_time.rs"]
pub mod codex_session_file_time;
#[path = "../../upstream/codex_session_import_paths.rs"]
pub mod codex_session_import_paths;
pub mod codex_session_visibility;
#[path = "../../upstream/codex_official_app_server.rs"]
pub mod upstream_official_app_server;
pub mod logger {
    pub fn log_info(_: &str) {}
    pub fn log_warn(s: &str) {
        eprintln!("Thread repair: {s}");
    }
}
pub mod windows_operation {
    pub fn format_permission_io_error(
        _: &str,
        _: &str,
        _: &str,
        _: &std::io::Error,
    ) -> Option<String> {
        None
    }
}
pub mod config {
    pub struct Config {
        pub language: String,
        pub codex_app_path: String,
    }
    pub fn get_user_config() -> Config {
        Config {
            language: "zh-CN".into(),
            codex_app_path: crate::host()
                .ok()
                .and_then(|h| h.binary)
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_default(),
        }
    }
}
pub mod i18n {
    pub fn translate(_: &str, key: &str, args: &[(&str, &str)]) -> String {
        if key.ends_with(".stopInstanceHint") {
            return "目标实例的 Desktop 主进程仍在运行，请完全退出该实例后重试。".into();
        }
        if key.ends_with(".projectsDeferred") {
            return "项目恢复已延后，尚未全部完成".into();
        }
        format!("{key} {args:?}")
    }
}
pub mod codex_instance {
    pub struct Instance {
        pub id: String,
        pub name: String,
        pub user_data_dir: String,
        pub last_pid: Option<u32>,
    }
    pub struct Settings {
        pub last_pid: Option<u32>,
    }
    pub struct Store {
        pub instances: Vec<Instance>,
        pub default_settings: Settings,
    }
    pub fn get_default_codex_home() -> Result<std::path::PathBuf, String> {
        Ok(crate::host()?.home)
    }
    pub fn load_instance_store() -> Result<Store, String> {
        Ok(Store { instances: vec![], default_settings: Settings { last_pid: None } })
    }
}
pub mod codex_account {
    pub fn try_acquire_profile_mutation_lease(
        home: &std::path::Path,
        _: &str,
    ) -> Result<std::fs::File, String> {
        let f = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(home.join(".atelier-cockpit-profile.lock"))
            .map_err(|e| e.to_string())?;
        f.try_lock().map_err(|e| e.to_string())?;
        Ok(f)
    }
    pub fn desktop_renderer_project_owner(
        _: &std::path::Path,
    ) -> Result<Option<Option<String>>, String> {
        Err("此线程修复适配器不读取认证资料".into())
    }
}
pub mod db {
    pub fn is_unusable_sqlite_database_error(e: &rusqlite::Error) -> bool {
        matches!(e,rusqlite::Error::SqliteFailure(c,_) if matches!(c.code,rusqlite::ErrorCode::DatabaseCorrupt|rusqlite::ErrorCode::NotADatabase))
    }
}
pub mod backup_storage {
    use sha2::{Digest, Sha256};
    use std::path::{Path, PathBuf};
    pub fn scope_for_path(p: &Path) -> String {
        format!("{:x}", Sha256::digest(p.to_string_lossy().as_bytes()))
    }
    pub fn behavior_backup_dir(source: &str, scope: &str, op: &str) -> Result<PathBuf, String> {
        for s in [source, scope, op] {
            if s.is_empty() || s.contains('/') || s.contains('\\') || s == ".." {
                return Err("无效备份路径段".into());
            }
        }
        #[cfg(test)]
        let root = crate::host().map(|h| h.backups).unwrap_or_else(|_| {
            std::env::temp_dir().join(format!("atelier-cockpit-tests-{}", std::process::id()))
        });
        #[cfg(not(test))]
        let root = crate::host()?.backups;
        let p = root.join(source).join(scope).join(op);
        std::fs::create_dir_all(&p).map_err(|e| e.to_string())?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o700))
                .map_err(|e| e.to_string())?;
        }
        Ok(p)
    }
    pub fn prune_behavior_backups(_: &str, _: &str) -> Result<(), String> {
        Ok(())
    } // Atelier retains recovery backups until explicit user cleanup.
}
pub mod process;

pub mod codex_official_app_server {
    pub fn rebuild_thread_metadata(home: &std::path::Path) -> Result<(), String> {
        if crate::host()?.binary.is_none() {
            return Err("宿主未授权引擎启动；未调用真实客户端".into());
        }
        super::upstream_official_app_server::rebuild_thread_metadata(home)
    }
}
