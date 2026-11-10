use atelier_maintenance_core::cleanup::*;
use serde_json::{json, Value};
use std::{
    fs, io,
    path::{Path, PathBuf},
};
struct Fixture {
    temp: tempfile::TempDir,
    paths: CleanupPaths,
}
impl Fixture {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let paths = CleanupPaths::from_home(&temp.path().join("codex"));
        Self { temp, paths }
    }
    fn initialize(&self) {
        fs::create_dir_all(&self.paths.snapshots_directory).unwrap();
        fs::create_dir_all(&self.paths.auth_backups_directory).unwrap();
        fs::create_dir_all(&self.paths.registry_backups_directory).unwrap();
    }
    fn snapshot(&self, name: &str, body: Value) -> PathBuf {
        let p = self.paths.snapshots_directory.join(name);
        fs::write(&p, serde_json::to_vec(&body).unwrap()).unwrap();
        p
    }
    fn registry(&self, items: Vec<Value>, active: Option<&str>) {
        fs::create_dir_all(&self.paths.accounts_directory).unwrap();
        fs::write(
            &self.paths.registry_path,
            serde_json::to_vec(
                &json!({"schemaVersion":2,"updatedAt":1,"activeAccountKey":active,"items":items}),
            )
            .unwrap(),
        )
        .unwrap();
    }
    fn quota(&self, items: Vec<Value>) {
        fs::create_dir_all(self.paths.quota_store_path.parent().unwrap()).unwrap();
        fs::write(
            &self.paths.quota_store_path,
            serde_json::to_vec(&json!({"schemaVersion":1,"updatedAt":2,"items":items})).unwrap(),
        )
        .unwrap();
    }
    fn read_registry(&self) -> Value {
        serde_json::from_slice(&fs::read(&self.paths.registry_path).unwrap()).unwrap()
    }
}
fn oauth() -> Value {
    json!({"auth_mode":"chatgpt","tokens":{"access_token":"synthetic.header.payload"}})
}
fn item(key: &str, path: &Path) -> Value {
    json!({"accountKey":key,"snapshotPath":path,"email":"synthetic@example.invalid","alias":"fixture","plan":"free","authMode":"chatgpt","createdAt":1})
}
fn quota(key: &str) -> Value {
    json!({"accountKey":key,"capturedAt":10,"usageSource":"api"})
}
#[test]
fn real_cleanup_prunes_unusable_registry_and_quota_then_deletes_only_direct_backup_files() {
    let f = Fixture::new();
    f.initialize();
    let good = f.snapshot("good.json", oauth());
    let api = f.snapshot("api.json", json!({"OPENAI_API_KEY":"synthetic-only"}));
    let bad = f.snapshot("bad.json", json!(null));
    let outside = f.temp.path().join("outside.json");
    fs::write(&outside, serde_json::to_vec(&oauth()).unwrap()).unwrap();
    f.registry(
        vec![
            item("live", &good),
            item("api", &api),
            item("bad", &bad),
            item("outside", &outside),
            item("missing", &good.with_file_name("missing.json")),
        ],
        Some("missing"),
    );
    f.quota(vec![quota("live"), quota("missing")]);
    fs::write(
        f.paths.auth_backups_directory.join("any-name.txt"),
        b"auth backup",
    )
    .unwrap();
    fs::write(
        f.paths.registry_backups_directory.join("not-a-timestamp"),
        b"registry backup",
    )
    .unwrap();
    let child = f.paths.auth_backups_directory.join("nested");
    fs::create_dir(&child).unwrap();
    fs::write(child.join("keep"), b"nested unchanged").unwrap();
    let report = clean_under_existing_lock_with_fs(&StdCleanupFs, &f.paths, 1000).unwrap();
    assert_eq!(
        report.data,
        CleanPayload {
            auth_backups_removed: 1,
            registry_backups_removed: 1,
            stale_entries_removed: 4
        }
    );
    assert_eq!(report.quota_items_removed, 1);
    assert_eq!(report.warnings.len(), 1);
    assert_eq!(report.warnings[0].code, "AUTH_BACKUP_CLEANUP_INCOMPLETE");
    assert!(child.join("keep").exists());
    let registry = f.read_registry();
    assert_eq!(registry["schemaVersion"], 4);
    assert_eq!(registry["updatedAt"], 1000);
    assert_eq!(registry["items"].as_array().unwrap().len(), 1);
    assert!(registry["activeAccountKey"].is_null());
    assert_eq!(registry["autoSwitch"]["revision"], 0);
    assert_eq!(registry["api"], json!({"usageRefreshEnabled":true}));
    let quota: Value =
        serde_json::from_slice(&fs::read(&f.paths.quota_store_path).unwrap()).unwrap();
    assert_eq!(quota["schemaVersion"], 3);
    assert_eq!(quota["updatedAt"], 1000);
    assert_eq!(quota["items"].as_array().unwrap().len(), 1);
    assert_eq!(
        fs::read(&outside).unwrap(),
        serde_json::to_vec(&oauth()).unwrap()
    );
    assert!(good.exists());
    assert!(api.exists());
    for dir in &f.paths.auxiliary_directories {
        assert!(dir.is_dir());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            fs::metadata(&f.paths.registry_path)
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
    }
}
#[test]
fn active_live_key_retained_and_unlock_keys_rejected_even_with_real_oauth_bytes() {
    let f = Fixture::new();
    f.initialize();
    let good = f.snapshot("good.json", oauth());
    let unlock = "synthetic-unlock::00000000-0000-4000-8000-000000000001";
    f.registry(vec![item("live", &good), item(unlock, &good)], Some("live"));
    let report = clean_under_existing_lock_with_fs(&StdCleanupFs, &f.paths, 200).unwrap();
    assert_eq!(report.data.stale_entries_removed, 1);
    assert_eq!(f.read_registry()["activeAccountKey"], "live");
    assert!(good.exists());
}
#[test]
fn invalid_existing_registry_stops_before_any_backup_deletion() {
    let f = Fixture::new();
    f.initialize();
    fs::write(&f.paths.registry_path, b"{invalid").unwrap();
    let backup = f.paths.auth_backups_directory.join("sentinel");
    fs::write(&backup, b"keep").unwrap();
    assert!(clean(&f.paths).is_err());
    assert_eq!(fs::read(&backup).unwrap(), b"keep");
    assert_eq!(fs::read(&f.paths.registry_path).unwrap(), b"{invalid");
    assert!(!f.paths.quota_store_path.exists());
}
#[test]
fn quota_read_failure_is_disposable_and_does_not_rewrite_original_or_warn() {
    let f = Fixture::new();
    f.initialize();
    f.registry(vec![], None);
    fs::create_dir_all(f.paths.quota_store_path.parent().unwrap()).unwrap();
    fs::write(&f.paths.quota_store_path, b"invalid quota JSON").unwrap();
    let backup = f.paths.registry_backups_directory.join("old");
    fs::write(&backup, b"delete").unwrap();
    let result = clean_under_existing_lock_with_fs(&StdCleanupFs, &f.paths, 250).unwrap();
    assert_eq!(result.data.registry_backups_removed, 1);
    assert!(result.warnings.is_empty());
    assert_eq!(
        fs::read(&f.paths.quota_store_path).unwrap(),
        b"invalid quota JSON"
    );
    assert_eq!(f.read_registry()["updatedAt"], 250);
}
struct FaultFs {
    fail_write: Option<PathBuf>,
    entry_error: Option<PathBuf>,
}
impl CleanupFs for FaultFs {
    fn metadata_exists(&self, p: &Path) -> bool {
        StdCleanupFs.metadata_exists(p)
    }
    fn canonicalize(&self, p: &Path) -> io::Result<PathBuf> {
        StdCleanupFs.canonicalize(p)
    }
    fn regular_file_without_symlink(&self, p: &Path) -> bool {
        StdCleanupFs.regular_file_without_symlink(p)
    }
    fn read_directory(&self, p: &Path) -> io::Result<Vec<io::Result<PathBuf>>> {
        let mut entries = StdCleanupFs.read_directory(p)?;
        if self.entry_error.as_deref() == Some(p) {
            entries.push(Err(io::Error::other("synthetic entry error")));
        }
        Ok(entries)
    }
    fn remove_file(&self, p: &Path) -> io::Result<()> {
        StdCleanupFs.remove_file(p)
    }
    fn read(&self, p: &Path) -> io::Result<Vec<u8>> {
        StdCleanupFs.read(p)
    }
    fn write_atomic_private(&self, p: &Path, b: &[u8]) -> io::Result<()> {
        if self.fail_write.as_deref() == Some(p) {
            Err(io::Error::other("synthetic publication failure"))
        } else {
            StdCleanupFs.write_atomic_private(p, b)
        }
    }
    fn create_directory(&self, p: &Path) -> io::Result<()> {
        StdCleanupFs.create_directory(p)
    }
}
#[test]
fn quota_save_failure_preserves_registry_effects_and_warns_after_both_backup_warnings() {
    let f = Fixture::new();
    f.initialize();
    f.registry(vec![], None);
    f.quota(vec![quota("stale")]);
    let before = fs::read(&f.paths.quota_store_path).unwrap();
    for dir in [
        &f.paths.auth_backups_directory,
        &f.paths.registry_backups_directory,
    ] {
        fs::write(dir.join("file"), b"delete").unwrap();
        fs::create_dir(dir.join("directory")).unwrap();
    }
    let fs = FaultFs {
        fail_write: Some(f.paths.quota_store_path.clone()),
        entry_error: None,
    };
    let r = clean_under_existing_lock_with_fs(&fs, &f.paths, 300).unwrap();
    assert_eq!(r.data.auth_backups_removed, 1);
    assert_eq!(r.data.registry_backups_removed, 1);
    assert_eq!(
        r.warnings
            .iter()
            .map(|w| w.code.as_str())
            .collect::<Vec<_>>(),
        vec![
            "AUTH_BACKUP_CLEANUP_INCOMPLETE",
            "REGISTRY_BACKUP_CLEANUP_INCOMPLETE",
            "QUOTA_CACHE_CLEANUP_INCOMPLETE"
        ]
    );
    assert_eq!(f.read_registry()["updatedAt"], 300);
    assert_eq!(fs::read(&f.paths.quota_store_path).unwrap(), before);
}
#[test]
fn registry_publication_failure_prevents_quota_and_backup_mutation() {
    let f = Fixture::new();
    f.initialize();
    f.registry(vec![], None);
    f.quota(vec![quota("stale")]);
    let backup = f.paths.auth_backups_directory.join("keep");
    fs::write(&backup, b"keep").unwrap();
    let registry = fs::read(&f.paths.registry_path).unwrap();
    let quota = fs::read(&f.paths.quota_store_path).unwrap();
    let fs = FaultFs {
        fail_write: Some(f.paths.registry_path.clone()),
        entry_error: None,
    };
    assert!(clean_under_existing_lock_with_fs(&fs, &f.paths, 400).is_err());
    assert_eq!(fs::read(&f.paths.registry_path).unwrap(), registry);
    assert_eq!(fs::read(&f.paths.quota_store_path).unwrap(), quota);
    assert!(backup.exists());
}
#[test]
fn backup_scan_and_entry_failures_are_distinct_from_successful_deletion_count() {
    let f = Fixture::new();
    f.initialize();
    let file = f.paths.auth_backups_directory.join("file");
    fs::write(&file, b"delete").unwrap();
    fs::create_dir(f.paths.auth_backups_directory.join("child")).unwrap();
    let fault = FaultFs {
        fail_write: None,
        entry_error: Some(f.paths.auth_backups_directory.clone()),
    };
    let r = remove_account_backup_files_with_fs(
        &fault,
        &f.paths.auth_backups_directory,
        "CODE",
        "authentication",
    );
    assert_eq!((r.removed, r.failed), (1, 2));
    assert_eq!(
        r.warnings[0].message,
        "2 authentication backup(s) could not be removed."
    );
    let r = remove_account_backup_files(&file, "CODE", "authentication");
    assert_eq!(r.removed, 0);
    assert!(r.warnings.is_empty());
    fs::write(&file, b"not a directory").unwrap();
    let r = remove_account_backup_files(&file, "CODE", "authentication");
    assert_eq!(
        r.warnings[0].message,
        "Some authentication backups could not be inspected."
    );
    assert!(file.exists());
}
#[test]
fn quota_window_normalization_is_saved_only_when_an_item_is_pruned() {
    let f = Fixture::new();
    f.initialize();
    let good = f.snapshot("good", oauth());
    f.registry(vec![item("live", &good)], Some("live"));
    let w = |minutes, resets| json!({"usedPercent":25.0,"remainingPercent":75,"windowMinutes":minutes,"resetsAt":resets});
    let mut live = quota("live");
    live["primaryWindow"] = w(361, json!(2000));
    live["secondaryWindow"] = w(8640, json!(1000));
    f.quota(vec![live.clone()]);
    let before = fs::read(&f.paths.quota_store_path).unwrap();
    clean_under_existing_lock_with_fs(&StdCleanupFs, &f.paths, 1000).unwrap();
    assert_eq!(fs::read(&f.paths.quota_store_path).unwrap(), before);
    f.quota(vec![live, quota("stale")]);
    clean_under_existing_lock_with_fs(&StdCleanupFs, &f.paths, 1000).unwrap();
    let state: Value =
        serde_json::from_slice(&fs::read(&f.paths.quota_store_path).unwrap()).unwrap();
    assert!(state["items"][0]["primaryWindow"].is_null());
    assert!(state["items"][0]["secondaryWindow"].is_null());
}
#[test]
fn prepare_is_noncreating_and_apply_creates_native_paths_and_schema_without_registry_backup() {
    let f = Fixture::new();
    let plan = prepare(&f.paths).unwrap();
    assert!(!f.paths.accounts_directory.exists());
    assert_eq!(plan.preview().stale_entries, 0);
    assert_eq!(plan.paths().registry_path, f.paths.registry_path);
    let report = apply(&plan).unwrap();
    assert!(report.registry_published);
    assert_eq!(f.read_registry()["schemaVersion"], 4);
    assert_eq!(
        fs::read_dir(&f.paths.registry_backups_directory)
            .unwrap()
            .count(),
        0
    );
    assert_eq!(
        fs::read_dir(&f.paths.auth_backups_directory)
            .unwrap()
            .count(),
        0
    );
}
#[test]
fn stale_snapshot_bytes_and_new_backup_targets_require_another_preview() {
    let f = Fixture::new();
    f.initialize();
    let good = f.snapshot("good", oauth());
    f.registry(vec![item("live", &good)], Some("live"));
    let plan = prepare(&f.paths).unwrap();
    fs::write(&good, b"external credential update").unwrap();
    assert!(matches!(apply(&plan), Err(CleanupError::StalePlan)));
    assert_eq!(f.read_registry()["updatedAt"], 1);
    fs::write(&good, serde_json::to_vec(&oauth()).unwrap()).unwrap();
    let plan = prepare(&f.paths).unwrap();
    let extra = f.paths.auth_backups_directory.join("new");
    fs::write(&extra, b"new backup").unwrap();
    assert!(matches!(apply(&plan), Err(CleanupError::StalePlan)));
    assert!(extra.exists());
}
#[cfg(unix)]
#[test]
fn symlink_snapshot_rejected_and_backup_link_unlinked_without_deleting_external_target() {
    use std::os::unix::fs::symlink;
    let f = Fixture::new();
    f.initialize();
    let target = f.temp.path().join("external");
    fs::write(&target, serde_json::to_vec(&oauth()).unwrap()).unwrap();
    let link = f.paths.snapshots_directory.join("snapshot-link");
    symlink(&target, &link).unwrap();
    f.registry(vec![item("link", &link)], None);
    let backup = f.paths.auth_backups_directory.join("link");
    symlink(&target, &backup).unwrap();
    let r = clean(&f.paths).unwrap();
    assert_eq!(r.data.stale_entries_removed, 1);
    assert_eq!(r.data.auth_backups_removed, 1);
    assert!(!backup.exists());
    assert!(target.exists());
    assert!(fs::symlink_metadata(&link)
        .unwrap()
        .file_type()
        .is_symlink());
}
#[test]
fn account_domain_lock_is_exclusive_and_released_by_drop() {
    let f = Fixture::new();
    let lock = acquire_account_domain_lock(&f.paths).unwrap();
    let another = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(f.paths.accounts_directory.join(".account-domain.lock"))
        .unwrap();
    assert!(matches!(
        another.try_lock(),
        Err(fs::TryLockError::WouldBlock)
    ));
    drop(lock);
    another.try_lock().unwrap();
}

#[test]
fn native_metadata_provenance_and_token_status_survive_real_cleanup_publication() {
    let f = Fixture::new();
    f.initialize();
    let good = f.snapshot("good", oauth());
    let mut entry = item("live", &good);
    entry["planStateSource"] = json!("usage");
    entry["subscriptionStateSource"] = json!("livePlan");
    entry["subscriptionLastCheckedAt"] = json!(77);
    f.registry(vec![entry], Some("live"));
    let mut registry = f.read_registry();
    registry["schemaVersion"] = json!(9);
    registry["autoSwitch"] =
        json!({"revision":31,"enabled":true,"threshold5hPercent":17,"thresholdWeeklyPercent":21});
    registry["api"] = json!({"usageRefreshEnabled":false});
    fs::write(
        &f.paths.registry_path,
        serde_json::to_vec(&registry).unwrap(),
    )
    .unwrap();
    let mut live = quota("live");
    live["usageSource"] = json!("local");
    live["availableResetCredits"] = json!(7);
    live["tokenStatus"] = json!({"code":"refreshed","credentialGeneration":"synthetic-generation","accessTokenExpiresAt":12345});
    live["tokenStatusUpdatedAt"] = json!(88);
    f.quota(vec![live.clone(), quota("stale")]);
    let r = clean_under_existing_lock_with_fs(&StdCleanupFs, &f.paths, 1000).unwrap();
    assert_eq!(r.quota_items_removed, 1);
    let registry = f.read_registry();
    assert_eq!(registry["schemaVersion"], 9);
    assert_eq!(registry["autoSwitch"]["revision"], 31);
    assert_eq!(registry["autoSwitch"]["threshold5hPercent"], 17);
    assert_eq!(registry["api"]["usageRefreshEnabled"], false);
    assert_eq!(registry["items"][0]["planStateSource"], "usage");
    assert_eq!(registry["items"][0]["subscriptionStateSource"], "livePlan");
    let stored: Value =
        serde_json::from_slice(&fs::read(&f.paths.quota_store_path).unwrap()).unwrap();
    for key in [
        "usageSource",
        "availableResetCredits",
        "tokenStatus",
        "tokenStatusUpdatedAt",
    ] {
        assert_eq!(stored["items"][0][key], live[key]);
    }
    assert!(!registry["items"][0]
        .get("planStateUpdatedAt")
        .unwrap()
        .is_number());
}
#[test]
fn missing_optional_native_provenance_and_token_status_are_omitted_not_written_as_null() {
    let f = Fixture::new();
    f.initialize();
    let good = f.snapshot("good", oauth());
    f.registry(vec![item("live", &good)], Some("live"));
    f.quota(vec![quota("live"), quota("stale")]);
    clean_under_existing_lock_with_fs(&StdCleanupFs, &f.paths, 1000).unwrap();
    let registry = f.read_registry();
    assert!(registry["items"][0].get("planStateSource").is_none());
    assert!(registry["items"][0]
        .get("subscriptionStateSource")
        .is_none());
    let stored: Value =
        serde_json::from_slice(&fs::read(&f.paths.quota_store_path).unwrap()).unwrap();
    assert!(stored["items"][0].get("tokenStatus").is_none());
    assert!(stored["items"][0].get("tokenStatusUpdatedAt").is_none());
    assert!(stored["items"][0]["availableResetCredits"].is_null());
}

#[test]
fn ticket_apply_preserves_new_or_changed_backup_files_after_registry_publication() {
    use std::cell::Cell;
    struct RaceFs<'a> {
        paths: &'a CleanupPaths,
        changed: bool,
        triggered: Cell<bool>,
    }
    impl CleanupFs for RaceFs<'_> {
        fn metadata_exists(&self, p: &Path) -> bool {
            StdCleanupFs.metadata_exists(p)
        }
        fn canonicalize(&self, p: &Path) -> io::Result<PathBuf> {
            StdCleanupFs.canonicalize(p)
        }
        fn regular_file_without_symlink(&self, p: &Path) -> bool {
            StdCleanupFs.regular_file_without_symlink(p)
        }
        fn read_directory(&self, p: &Path) -> io::Result<Vec<io::Result<PathBuf>>> {
            StdCleanupFs.read_directory(p)
        }
        fn remove_file(&self, p: &Path) -> io::Result<()> {
            StdCleanupFs.remove_file(p)
        }
        fn read(&self, p: &Path) -> io::Result<Vec<u8>> {
            StdCleanupFs.read(p)
        }
        fn create_directory(&self, p: &Path) -> io::Result<()> {
            StdCleanupFs.create_directory(p)
        }
        fn write_atomic_private(&self, p: &Path, b: &[u8]) -> io::Result<()> {
            StdCleanupFs.write_atomic_private(p, b)?;
            if p == self.paths.registry_path && !self.triggered.replace(true) {
                fs::write(
                    self.paths.auth_backups_directory.join(if self.changed {
                        "approved"
                    } else {
                        "new"
                    }),
                    b"external new bytes",
                )?;
            }
            Ok(())
        }
    }
    for changed in [false, true] {
        let f = Fixture::new();
        f.initialize();
        f.registry(vec![], None);
        let original = f.paths.auth_backups_directory.join("approved");
        fs::write(&original, b"approved bytes").unwrap();
        let plan = prepare(&f.paths).unwrap();
        let race = RaceFs {
            paths: &f.paths,
            changed,
            triggered: Cell::new(false),
        };
        let result = apply_under_existing_lock_with_fs(&race, &plan, 1000).unwrap();
        assert!(result.scope_changed);
        assert_eq!(result.data.auth_backups_removed, 0);
        assert!(original.exists());
        assert_eq!(f.read_registry()["updatedAt"], 1000);
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == "CLEANUP_APPROVED_SCOPE_CHANGED"));
        if !changed {
            assert_eq!(
                fs::read(f.paths.auth_backups_directory.join("new")).unwrap(),
                b"external new bytes"
            );
        }
    }
}

#[cfg(unix)]
#[test]
fn preview_does_not_read_rejected_snapshot_or_external_backup_symlink_contents() {
    use std::{collections::BTreeSet, os::unix::fs::symlink};
    struct GuardFs {
        forbidden: BTreeSet<PathBuf>,
    }
    impl CleanupFs for GuardFs {
        fn metadata_exists(&self, p: &Path) -> bool {
            StdCleanupFs.metadata_exists(p)
        }
        fn canonicalize(&self, p: &Path) -> io::Result<PathBuf> {
            StdCleanupFs.canonicalize(p)
        }
        fn regular_file_without_symlink(&self, p: &Path) -> bool {
            StdCleanupFs.regular_file_without_symlink(p)
        }
        fn read_directory(&self, p: &Path) -> io::Result<Vec<io::Result<PathBuf>>> {
            StdCleanupFs.read_directory(p)
        }
        fn remove_file(&self, p: &Path) -> io::Result<()> {
            StdCleanupFs.remove_file(p)
        }
        fn read(&self, p: &Path) -> io::Result<Vec<u8>> {
            assert!(
                !self.forbidden.contains(p),
                "outside/symlink credential contents must not be read"
            );
            StdCleanupFs.read(p)
        }
        fn write_atomic_private(&self, p: &Path, b: &[u8]) -> io::Result<()> {
            StdCleanupFs.write_atomic_private(p, b)
        }
        fn create_directory(&self, p: &Path) -> io::Result<()> {
            StdCleanupFs.create_directory(p)
        }
    }
    let f = Fixture::new();
    f.initialize();
    let outside = f.temp.path().join("outside-auth");
    fs::write(&outside, serde_json::to_vec(&oauth()).unwrap()).unwrap();
    let snapshot = f.paths.snapshots_directory.join("linked");
    let backup = f.paths.auth_backups_directory.join("linked");
    symlink(&outside, &snapshot).unwrap();
    symlink(&outside, &backup).unwrap();
    f.registry(
        vec![item("outside", &outside), item("link", &snapshot)],
        None,
    );
    let guard = GuardFs {
        forbidden: BTreeSet::from([outside.clone(), snapshot.clone(), backup.clone()]),
    };
    let plan = prepare_with_fs(&guard, &f.paths, 1000).unwrap();
    assert_eq!(plan.preview().stale_entries, 2);
    let result = apply_under_existing_lock_with_fs(&guard, &plan, 1000).unwrap();
    assert_eq!(result.data.stale_entries_removed, 2);
    assert_eq!(result.data.auth_backups_removed, 1);
    assert!(outside.exists());
    assert!(!backup.exists());
}
