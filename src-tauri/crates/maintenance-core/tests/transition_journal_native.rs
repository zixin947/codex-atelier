use atelier_maintenance_core::{catalog::publication::Publication, transition_journal::*};
use std::{fs, io, path::Path};
use tempfile::TempDir;

#[test]
fn exact_wire_targets_and_active_phases() {
    assert_eq!(
        serde_json::to_string(&TransitionTarget::RouterOn).unwrap(),
        "\"routerOn\""
    );
    assert_eq!(
        serde_json::to_string(&TransitionTarget::NativeOff).unwrap(),
        "\"nativeOff\""
    );
    for phase in ["retrying", "committing", "reconciling", "stopping_codex"] {
        assert!(is_active_phase(phase));
    }
    for phase in [
        "starting",
        "committed",
        "failed",
        "retrying ",
        "",
        "stopping",
    ] {
        assert!(!is_active_phase(phase));
    }
}
#[test]
fn absent_reads_do_not_create_files() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("not-created");
    let store = JournalStore::new(&root);
    assert_eq!(store.read_checked().unwrap(), None);
    assert_eq!(store.in_flight(), None);
    assert!(!store.commit_pending().unwrap());
    assert!(!store.clear(None).unwrap());
    assert!(!root.exists());
}
#[test]
fn same_target_preserves_start_and_changed_target_resets_it() {
    let tmp = TempDir::new().unwrap();
    let store = JournalStore::new(tmp.path());
    store
        .write(TransitionTarget::RouterOn, "stopping_codex", None, 17)
        .unwrap()
        .acknowledge();
    store
        .write(
            TransitionTarget::RouterOn,
            "reconciling",
            Some("fixture"),
            29,
        )
        .unwrap()
        .acknowledge();
    let record = store.in_flight().unwrap();
    assert_eq!((record.started_at_ms, record.updated_at_ms), (17, 29));
    assert_eq!(record.reason.as_deref(), Some("fixture"));
    store
        .write(TransitionTarget::NativeOff, "retrying", None, 40)
        .unwrap()
        .acknowledge();
    assert_eq!(store.read_checked().unwrap().unwrap().started_at_ms, 40);
    assert!(store.clear(None).unwrap());
    assert!(!store.path().exists());
}
#[test]
fn commit_record_cannot_be_overwritten_or_generically_cleared() {
    let tmp = TempDir::new().unwrap();
    let store = JournalStore::new(tmp.path());
    store
        .begin_commit(TransitionTarget::RouterOn, None, 1)
        .unwrap();
    let before = fs::read(store.path()).unwrap();
    assert!(store.commit_pending().unwrap());
    for error in [
        store
            .write(TransitionTarget::RouterOn, "retrying", None, 2)
            .unwrap_err(),
        store.clear(None).unwrap_err(),
        store.clear(Some(TransitionTarget::NativeOff)).unwrap_err(),
    ] {
        assert!(error.to_string().starts_with(RECOVERY_REQUIRED));
    }
    assert_eq!(fs::read(store.path()).unwrap(), before);
    assert!(store.clear(Some(TransitionTarget::RouterOn)).unwrap());
}
#[test]
fn explicit_recovery_can_advance_pending_commit_and_preserve_start() {
    let tmp = TempDir::new().unwrap();
    let store = JournalStore::new(tmp.path());
    store
        .begin_commit(TransitionTarget::NativeOff, Some("interrupted"), 4)
        .unwrap();
    store
        .recover_write(TransitionTarget::NativeOff, "retrying", None, 11)
        .unwrap()
        .acknowledge();
    let record = store.read_checked().unwrap().unwrap();
    assert_eq!(record.started_at_ms, 4);
    assert_eq!(record.updated_at_ms, 11);
    assert!(!store.commit_pending().unwrap());
}
#[test]
fn schema_and_required_fields_are_checked_without_rewriting_bad_record() {
    let tmp = TempDir::new().unwrap();
    let store = JournalStore::new(tmp.path());
    for text in [
        "{",
        "{}",
        r#"{"schemaVersion":2}"#,
        r#"{"schemaVersion":256}"#,
        r#"{"schemaVersion":1,"target":"noAccount"}"#,
        r#"{"schemaVersion":1,"schemaVersion":1}"#,
    ] {
        fs::write(store.path(), text).unwrap();
        assert!(store.read_checked().is_err());
        assert!(store.commit_pending().is_err());
        assert!(store
            .write(TransitionTarget::RouterOn, "retrying", None, 1)
            .is_err());
        assert_eq!(fs::read(store.path()).unwrap(), text.as_bytes());
        assert!(store.in_flight().is_none());
    }
    fs::write(store.path(),r#"{"schemaVersion":1,"target":"nativeOff","phase":"stable","startedAtMs":-1,"updatedAtMs":0,"future":"retained-on-read"}"#).unwrap();
    let record = store.read_checked().unwrap().unwrap();
    assert_eq!(record.reason, None);
    assert_eq!(record.started_at_ms, -1);
    assert!(store.in_flight().is_none());
}
#[test]
fn oversize_record_is_refused_before_publication() {
    let tmp = TempDir::new().unwrap();
    let store = JournalStore::new(tmp.path());
    fs::write(store.path(), vec![b' '; SIZE_LIMIT as usize + 1]).unwrap();
    assert!(store
        .read_checked()
        .unwrap_err()
        .to_string()
        .contains("size limit"));
    fs::remove_file(store.path()).unwrap();
    assert!(store
        .write(
            TransitionTarget::RouterOn,
            "retrying",
            Some(&"x".repeat(SIZE_LIMIT as usize)),
            1
        )
        .is_err());
    assert!(!store.path().exists());
}
#[cfg(unix)]
#[test]
fn symlink_is_not_followed_and_new_record_is_private() {
    use std::os::unix::fs::{symlink, PermissionsExt};
    let tmp = TempDir::new().unwrap();
    let store = JournalStore::new(tmp.path());
    let target = tmp.path().join("other");
    fs::write(&target, b"unchanged").unwrap();
    symlink(&target, store.path()).unwrap();
    assert!(store.read_checked().is_err());
    assert!(store
        .write(TransitionTarget::RouterOn, "retrying", None, 1)
        .is_err());
    assert_eq!(fs::read(&target).unwrap(), b"unchanged");
    fs::remove_file(store.path()).unwrap();
    store
        .write(TransitionTarget::RouterOn, "retrying", None, 1)
        .unwrap()
        .acknowledge();
    assert_eq!(
        fs::metadata(store.path()).unwrap().permissions().mode() & 0o777,
        0o600
    );
}
struct SyncFailure;
impl JournalIo for SyncFailure {
    fn read_regular(&self, p: &Path) -> io::Result<Option<Vec<u8>>> {
        LocalJournalIo.read_regular(p)
    }
    fn publish(&self, p: &Path, b: &[u8]) -> io::Result<Publication> {
        let mut r = LocalJournalIo.publish(p, b)?;
        r.directory_sync_error = Some(io::Error::other("injected directory fsync"));
        Ok(r)
    }
    fn remove(&self, p: &Path) -> io::Result<Publication> {
        let mut r = LocalJournalIo.remove(p)?;
        r.directory_sync_error = Some(io::Error::other("injected directory fsync"));
        Ok(r)
    }
}
#[test]
fn commit_sync_failure_retains_published_recovery_record() {
    let tmp = TempDir::new().unwrap();
    let store = JournalStore::with_io(tmp.path(), SyncFailure);
    let error = store
        .begin_commit(TransitionTarget::RouterOn, None, 1)
        .unwrap_err();
    assert!(error
        .to_string()
        .starts_with("ATOMIC_WRITE_PUBLISHED_SYNC_UNCONFIRMED"));
    assert!(store.commit_pending().unwrap());
    assert!(store.clear(None).is_err());
}
#[test]
fn normal_write_acknowledges_sync_warning_but_clear_reports_removed_state() {
    let tmp = TempDir::new().unwrap();
    let store = JournalStore::with_io(tmp.path(), SyncFailure);
    let receipt = store
        .write(TransitionTarget::NativeOff, "retrying", None, 1)
        .unwrap()
        .acknowledge();
    assert_eq!(receipt.directory_sync_error, Some(io::ErrorKind::Other));
    let error = store.clear(None).unwrap_err();
    assert!(error
        .to_string()
        .starts_with("ROUTER_JOURNAL_REMOVED_SYNC_UNCONFIRMED"));
    assert!(!store.path().exists());
}
#[test]
fn journal_success_and_pending_rejection_append_only_classified_event_reasons() {
    let tmp = TempDir::new().unwrap();
    let store = JournalStore::new(tmp.path());
    store
        .begin_commit_with_source(
            TransitionTarget::RouterOn,
            Some("diagnostic repair"),
            Some("auth token secret fixture"),
            1,
        )
        .unwrap();
    let error = store
        .write_with_source(
            TransitionTarget::NativeOff,
            "retrying",
            Some("router toggle"),
            Some("database is locked; private fixture"),
            2,
        )
        .unwrap_err();
    assert!(error.to_string().starts_with(RECOVERY_REQUIRED));
    assert!(store.commit_pending().unwrap());
    let log = fs::read_to_string(tmp.path().join("router-events.jsonl")).unwrap();
    assert!(!log.contains("secret"));
    assert!(!log.contains("private fixture"));
    let lines = log
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0]["source"], "diagnostic");
    assert_eq!(lines[0]["reason"], "auth");
    assert_eq!(lines[1]["source"], "toggle");
    assert_eq!(lines[1]["reason"], "db_lock");
}
