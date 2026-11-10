//! Explicit temporary Contexts only; no client discovery or process operations.
use super::*;
use atelier_maintenance_core::transition_journal::{JournalStore, TransitionTarget};

const BEFORE: &[u8] = b"model = 'fixture'\n# >>> aimami-relay managed start (DO NOT EDIT MANUALLY)\nbroken = [\n# <<< aimami-relay managed end\n";
const AFTER: &[u8] = b"model = 'fixture'\n";
struct Fixture {
    root: PathBuf,
    ctx: Context,
}
impl Fixture {
    fn new() -> Self {
        let root =
            std::env::temp_dir().join(format!("atelier-mutation-guard-{}", uuid::Uuid::new_v4()));
        let ctx = Context {
            home: root.join("codex"),
            app_root: root.join("atelier"),
            desktop_running: false,
            runtime: None,
            official_catalog: None,
            proxy_handle: None,
            working_directory: None,
            sqlite_home_env: None,
        };
        std::fs::create_dir_all(&ctx.home).unwrap();
        std::fs::create_dir_all(&ctx.app_root).unwrap();
        std::fs::write(ctx.config(), BEFORE).unwrap();
        Self { root, ctx }
    }
    fn ticket(&self) -> Ticket {
        preview(&self.ctx, "config_toml_syntax").unwrap()
    }
    fn no_mutation(&self) {
        assert_eq!(std::fs::read(self.ctx.config()).unwrap(), BEFORE);
        assert!(!self
            .ctx
            .app_root
            .join("maintenance/diagnostic-backups")
            .exists());
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

#[test]
fn guard_is_exclusive_and_released_by_drop() {
    let f = Fixture::new();
    let guard = acquire(&f.ctx).unwrap();
    assert!(acquire(&f.ctx).err().unwrap().contains("另一维护"));
    // The very same lock paths are used by existing configuration and thread writers.
    for relative in [
        "maintenance/codex-write.lock",
        "thread-backups/mutation.lock",
    ] {
        let file = open_private(&f.ctx.app_root.join(relative)).unwrap();
        assert!(file.try_lock_exclusive().is_err());
    }
    drop(guard);
    drop(acquire(&f.ctx).unwrap());
    f.no_mutation();
}

#[test]
fn failure_to_acquire_thread_lock_releases_configuration_lock() {
    let f = Fixture::new();
    let thread_dir = f.ctx.app_root.join("thread-backups");
    private_dir(&thread_dir).unwrap();
    let thread = open_private(&thread_dir.join("mutation.lock")).unwrap();
    thread.try_lock_exclusive().unwrap();
    assert!(acquire(&f.ctx).err().unwrap().contains("线程管理"));
    let config = open_private(&f.ctx.app_root.join("maintenance/codex-write.lock")).unwrap();
    config.try_lock_exclusive().unwrap();
    drop(config);
    drop(thread);
    drop(acquire(&f.ctx).unwrap());
}

#[test]
fn guard_cannot_authorize_another_home_or_app_root() {
    let f = Fixture::new();
    let other = Fixture::new();
    let guard = acquire(&f.ctx).unwrap();
    assert!(apply_under_manager(&other.ctx, other.ticket(), &guard)
        .unwrap_err()
        .contains("维护锁的目标目录"));
    let mut changed = f.ctx.clone();
    changed.home = other.ctx.home.clone();
    assert!(apply_under_manager(&changed, f.ticket(), &guard)
        .unwrap_err()
        .contains("维护锁的目标目录"));
    f.no_mutation();
    other.no_mutation();
}

#[test]
fn manager_can_repair_under_its_active_journal_without_releasing_guard() {
    let f = Fixture::new();
    let ticket = f.ticket();
    let journal = JournalStore::new(&f.ctx.home.join("codexmate/relay"));
    journal
        .begin_commit(TransitionTarget::NativeOff, None, 1)
        .unwrap();
    let before = std::fs::read(journal.path()).unwrap();
    assert!(apply(&f.ctx, ticket.clone())
        .unwrap_err()
        .contains("路由转换进行中"));
    let guard = acquire(&f.ctx).unwrap();
    let result = apply_under_manager(&f.ctx, ticket, &guard).unwrap();
    assert_eq!(std::fs::read(f.ctx.config()).unwrap(), AFTER);
    assert_eq!(std::fs::read(journal.path()).unwrap(), before);
    assert!(result.fixed_ids.iter().any(|id| id == "config_toml_syntax"));
    assert!(result.report.transition_in_flight);
    let backup = PathBuf::from(result.backup_path.unwrap());
    assert_eq!(std::fs::read(backup.join("0000.bin")).unwrap(), BEFORE);
    assert!(acquire(&f.ctx).is_err());
}

#[test]
fn manager_retains_real_desktop_and_active_request_guards() {
    for desktop in [true, false] {
        let mut f = Fixture::new();
        let ticket = f.ticket();
        let guard = acquire(&f.ctx).unwrap();
        f.ctx.desktop_running = desktop;
        if !desktop {
            f.ctx.runtime = Some(crate::gateway::types::RuntimeView {
                active_requests: 1,
                ..Default::default()
            });
        }
        let error = apply_under_manager(&f.ctx, ticket, &guard).unwrap_err();
        assert!(error.contains(if desktop {
            "Desktop 仍在运行"
        } else {
            "在途请求"
        }));
        f.no_mutation();
    }
}

#[test]
fn manager_rejects_expired_ticket_and_malformed_journal() {
    let f = Fixture::new();
    let guard = acquire(&f.ctx).unwrap();
    let mut ticket = f.ticket();
    ticket.created_at = now_ms().saturating_sub(300_001);
    assert!(apply_under_manager(&f.ctx, ticket, &guard)
        .unwrap_err()
        .contains("已过期"));
    let ticket = f.ticket();
    let journal = JournalStore::new(&f.ctx.home.join("codexmate/relay"));
    std::fs::create_dir_all(journal.path().parent().unwrap()).unwrap();
    std::fs::write(journal.path(), b"{bad").unwrap();
    assert!(apply_under_manager(&f.ctx, ticket, &guard)
        .unwrap_err()
        .contains("恢复记录无法核对"));
    f.no_mutation();
}

#[test]
fn manager_rejects_external_configuration_and_selected_file_changes() {
    for config in [true, false] {
        let f = Fixture::new();
        let mut ticket = f.ticket();
        let path = f.ctx.home.join("selected.json");
        std::fs::write(&path, b"before").unwrap();
        ticket.files.push(FileChange::write(
            path.clone(),
            b"before".to_vec(),
            b"after".to_vec(),
        ));
        let guard = acquire(&f.ctx).unwrap();
        let changed = if config { f.ctx.config() } else { path.clone() };
        std::fs::write(&changed, b"external").unwrap();
        let error = apply_under_manager(&f.ctx, ticket, &guard).unwrap_err();
        assert!(error.contains("外部更新"));
        assert_eq!(std::fs::read(changed).unwrap(), b"external");
        assert_eq!(
            std::fs::read(path).unwrap(),
            if config {
                &b"before"[..]
            } else {
                &b"external"[..]
            }
        );
        assert!(!f
            .ctx
            .app_root
            .join("maintenance/diagnostic-backups")
            .exists());
    }
}

#[cfg(unix)]
#[test]
fn replacing_a_locked_file_invalidates_guard() {
    let f = Fixture::new();
    let ticket = f.ticket();
    let guard = acquire(&f.ctx).unwrap();
    let path = f.ctx.app_root.join("maintenance/codex-write.lock");
    std::fs::remove_file(&path).unwrap();
    let replacement = open_private(&path).unwrap();
    replacement.try_lock_exclusive().unwrap();
    assert!(apply_under_manager(&f.ctx, ticket, &guard)
        .unwrap_err()
        .contains("维护锁文件已变化"));
    f.no_mutation();
}

#[test]
fn reviewed_ticket_getters_preserve_absence_deletion_and_content() {
    let f = Fixture::new();
    let mut ticket = f.ticket();
    assert_eq!(ticket.home(), f.ctx.home);
    assert_eq!(ticket.app_root(), f.ctx.app_root);
    assert_eq!(ticket.original_config_hash(), config_hash(&f.ctx).unwrap());
    assert_eq!(ticket.selected_ids(), ["config_toml_syntax"]);
    assert_eq!(ticket.config_after(), Some(Some(AFTER.to_vec())));
    let change = &ticket.file_changes()[0];
    assert_eq!(change.path(), f.ctx.config());
    assert_eq!(change.before(), Some(BEFORE));
    assert_eq!(change.after(), Some(AFTER));
    ticket.files.clear();
    assert_eq!(ticket.config_after(), None);
    ticket.files.push(FileChange {
        path: f.ctx.config(),
        before: Some(BEFORE.to_vec()),
        after: None,
    });
    assert_eq!(ticket.config_after(), Some(None));
}

#[test]
fn reviewed_configuration_effect_includes_the_native_composed_plan() {
    let f = Fixture::new();
    std::fs::write(
        f.ctx.config(),
        b"model_provider='openai'\nprofile='legacy'\n",
    )
    .unwrap();
    let ticket = preview(&f.ctx, "config_profile_conflict").unwrap();
    assert!(ticket.coordinated.is_some());
    assert!(!ticket
        .file_changes()
        .iter()
        .any(|change| change.path() == f.ctx.config()));
    let after = ticket.config_after().unwrap().unwrap();
    let parsed: toml::Value = toml::from_str(std::str::from_utf8(&after).unwrap()).unwrap();
    assert!(parsed.get("profile").is_none());
    assert_eq!(parsed["model_provider"].as_str(), Some("openai"));
    assert!(std::fs::read_to_string(f.ctx.config())
        .unwrap()
        .contains("profile='legacy'"));
}
