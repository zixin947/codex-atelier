use atelier_maintenance_core::{router_capacity::*, router_verification::VerificationPaths};
use rusqlite::{params, Connection};
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use tempfile::TempDir;

struct Space(io::Result<u64>);
impl FreeSpace for Space {
    fn available_bytes(&self, _: &Path) -> io::Result<u64> {
        self.0
            .as_ref()
            .copied()
            .map_err(|e| io::Error::from(e.kind()))
    }
}
struct Fixture {
    _temp: TempDir,
    paths: VerificationPaths,
}
impl Fixture {
    fn new() -> Self {
        let temp = TempDir::new().unwrap();
        let home = temp.path().join("codex");
        fs::create_dir_all(home.join("codexmate/backups")).unwrap();
        fs::create_dir_all(home.join("sessions")).unwrap();
        let paths = VerificationPaths::from_home(&home, vec![]);
        Self { _temp: temp, paths }
    }
    fn database(&mut self, name: &str) -> Connection {
        let path = self.paths.codex_home.join(name);
        let c = Connection::open(&path).unwrap();
        c.execute_batch("CREATE TABLE threads(id TEXT PRIMARY KEY, rollout_path TEXT, model_provider TEXT, source TEXT,model TEXT,archived INTEGER)").unwrap();
        self.paths.databases.push(path);
        c
    }
    fn history(&self, name: &str, id: &str, compressed: bool) -> (PathBuf, usize) {
        let bytes=format!("{{\"type\":\"session_meta\",\"payload\":{{\"id\":\"{id}\",\"model_provider\":\"openai\"}}}}\n{}", " ".repeat(8192));
        let path = self.paths.codex_home.join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let raw = if compressed {
            zstd::stream::encode_all(bytes.as_bytes(), 3).unwrap()
        } else {
            bytes.as_bytes().to_vec()
        };
        fs::write(&path, raw).unwrap();
        (path, bytes.len())
    }
}
fn row(c: &Connection, id: &str, path: Option<&Path>, provider: &str, source: &str, archived: i32) {
    c.execute(
        "INSERT INTO threads VALUES (?1,?2,?3,?4,'fixture-model',?5)",
        params![
            id,
            path.map(|p| p.to_str().unwrap()),
            provider,
            source,
            archived
        ],
    )
    .unwrap();
}
#[test]
fn required_capacity_uses_four_largest_not_sum_or_input_order_and_saturates() {
    assert_eq!(
        required_bytes(7, &[1, 100, 2, 90, 3, 80, 4, 70]),
        RESERVE_BYTES + 7 + 100 + 90 + 80 + 70
    );
    assert_eq!(required_bytes(0, &[]), RESERVE_BYTES);
    assert_eq!(required_bytes(u64::MAX, &[1]), u64::MAX);
    assert_eq!(required_bytes(0, &[u64::MAX, 1]), u64::MAX);
}
#[test]
fn no_database_needs_reserve_and_equal_available_bytes_passes() {
    let f = Fixture::new();
    let report = inspect_transition_rollout_write_capacity_with_space(
        &f.paths,
        false,
        &Space(Ok(RESERVE_BYTES)),
    )
    .unwrap();
    assert_eq!(report.required_bytes, RESERVE_BYTES);
    assert_eq!(report.available_bytes, Some(RESERVE_BYTES));
    assert!(report.logical_lengths.is_empty());
    assert!(inspect_transition_rollout_write_capacity_with_space(
        &f.paths,
        false,
        &Space(Ok(RESERVE_BYTES - 1))
    )
    .unwrap_err()
    .to_string()
    .contains("空间不足"));
}
#[test]
fn logical_length_decodes_zstd_and_prefers_present_plain_representation() {
    let f = Fixture::new();
    let (compressed, length) = f.history("sessions/one.jsonl.zst", "one", true);
    assert!(fs::metadata(&compressed).unwrap().len() < length as u64);
    assert_eq!(rollout_logical_len(&compressed).unwrap(), length as u64);
    let plain = compressed.with_extension("");
    fs::write(&plain, b"plain").unwrap();
    assert_eq!(rollout_logical_len(&compressed).unwrap(), 5);
    fs::remove_file(plain).unwrap();
    assert_eq!(
        rollout_logical_len(&compressed.with_extension("")).unwrap(),
        length as u64
    );
}
#[test]
fn active_logical_sizes_include_external_providers_but_ignore_archived_records() {
    let mut f = Fixture::new();
    let c = f.database("state_5.sqlite");
    let (active, length) = f.history("sessions/active.jsonl.zst", "active", true);
    let (archived, _) = f.history("sessions/archived.jsonl", "archived", false);
    row(&c, "active", Some(&active), "enterprise", "cli", 0);
    row(&c, "archived", Some(&archived), "openai", "cli", 1);
    drop(c);
    let report =
        inspect_transition_rollout_write_capacity_with_space(&f.paths, false, &Space(Ok(u64::MAX)))
            .unwrap();
    assert_eq!(report.logical_lengths, [length as u64]);
    assert_eq!(report.write_bytes, 0);
    assert_eq!(report.required_bytes, RESERVE_BYTES + length as u64);
}
#[test]
fn strict_missing_fails_and_repair_missing_budgets_actual_database_backup_once() {
    let mut f = Fixture::new();
    let c = f.database("state_5.sqlite");
    row(&c, "one", None, "openai", "cli", 0);
    row(&c, "two", None, "aimai1", "cli", 0);
    drop(c);
    assert!(inspect_transition_rollout_write_capacity_with_space(
        &f.paths,
        false,
        &Space(Ok(u64::MAX))
    )
    .unwrap_err()
    .to_string()
    .contains("rollout is missing"));
    let length = fs::metadata(&f.paths.databases[0]).unwrap().len();
    let report =
        inspect_transition_rollout_write_capacity_with_space(&f.paths, true, &Space(Ok(u64::MAX)))
            .unwrap();
    assert_eq!(report.backup_databases, 1);
    assert_eq!(report.write_bytes, length);
    assert_eq!(report.required_bytes, RESERVE_BYTES + length);
}
#[test]
fn trusted_copy_uses_physical_bytes_and_deduplicates_same_missing_target_across_databases() {
    let mut f = Fixture::new();
    let target = f.paths.codex_home.join("sessions/missing.jsonl");
    let (backup, _) = f.history("codexmate/backups/missing.jsonl", "same", false);
    let length = fs::metadata(&backup).unwrap().len();
    for name in ["state_4.sqlite", "state_5.sqlite"] {
        let c = f.database(name);
        row(&c, "same", Some(&target), "openai", "cli", 0);
    }
    let report =
        inspect_transition_rollout_write_capacity_with_space(&f.paths, true, &Space(Ok(u64::MAX)))
            .unwrap();
    assert_eq!(report.restore_files, 1);
    assert_eq!(report.backup_databases, 0);
    assert_eq!(report.logical_lengths, [length]);
    assert_eq!(report.write_bytes, length);
    assert_eq!(report.required_bytes, RESERVE_BYTES + 2 * length);
    assert!(!target.exists());
    assert!(backup.exists());
}
#[test]
fn wrong_identity_backup_is_not_budgeted_as_restorable_and_target_is_unchanged() {
    let mut f = Fixture::new();
    let target = f.paths.codex_home.join("sessions/missing.jsonl");
    f.history("codexmate/backups/missing.jsonl", "other", false);
    let c = f.database("state_5.sqlite");
    row(&c, "wanted", Some(&target), "openai", "cli", 0);
    drop(c);
    let report =
        inspect_transition_rollout_write_capacity_with_space(&f.paths, true, &Space(Ok(u64::MAX)))
            .unwrap();
    assert_eq!(report.restore_files, 0);
    assert_eq!(report.backup_databases, 1);
    assert!(!target.exists());
}
#[test]
fn failed_free_space_probe_is_native_best_effort_not_a_synthetic_positive_value() {
    let f = Fixture::new();
    let report = inspect_transition_rollout_write_capacity_with_space(
        &f.paths,
        false,
        &Space(Err(io::ErrorKind::PermissionDenied.into())),
    )
    .unwrap();
    assert_eq!(report.available_bytes, None);
    assert_eq!(report.required_bytes, RESERVE_BYTES);
    assert!(report.warnings[0].contains("probe unavailable"));
}
#[test]
fn malformed_database_is_strict_error_but_missing_repair_mode_records_warning() {
    let mut f = Fixture::new();
    let path = f.paths.codex_home.join("state_5.sqlite");
    fs::write(&path, b"not a database").unwrap();
    f.paths.databases.push(path.clone());
    assert!(inspect_transition_rollout_write_capacity_with_space(
        &f.paths,
        false,
        &Space(Ok(u64::MAX))
    )
    .is_err());
    let report =
        inspect_transition_rollout_write_capacity_with_space(&f.paths, true, &Space(Ok(u64::MAX)))
            .unwrap();
    assert!(!report.warnings.is_empty());
    assert_eq!(fs::read(path).unwrap(), b"not a database");
}
#[test]
fn subagents_and_unowned_provider_rows_do_not_trigger_strict_missing_failure() {
    let mut f = Fixture::new();
    let c = f.database("state_5.sqlite");
    row(&c, "external", None, "enterprise", "cli", 0);
    row(&c, "sub", None, "openai", r#"{"subagent":"parent"}"#, 0);
    drop(c);
    let report =
        inspect_transition_rollout_write_capacity_with_space(&f.paths, false, &Space(Ok(u64::MAX)))
            .unwrap();
    assert_eq!(report.write_bytes, 0);
}
#[test]
fn corrupt_compressed_history_warns_without_treating_compressed_length_as_logical_length() {
    let mut f = Fixture::new();
    let c = f.database("state_5.sqlite");
    let path = f.paths.codex_home.join("sessions/corrupt.jsonl.zst");
    fs::write(&path, b"not-zstd").unwrap();
    row(&c, "corrupt", Some(&path), "openai", "cli", 0);
    drop(c);
    let report =
        inspect_transition_rollout_write_capacity_with_space(&f.paths, false, &Space(Ok(u64::MAX)))
            .unwrap();
    assert!(report.logical_lengths.is_empty());
    assert_eq!(report.warnings.len(), 1);
    assert_eq!(report.required_bytes, RESERVE_BYTES);
}
#[test]
fn real_local_disk_probe_reads_only_the_explicit_temporary_volume() {
    let f = Fixture::new();
    let report = inspect_transition_rollout_write_capacity(&f.paths, true).unwrap();
    assert!(report.available_bytes.is_some());
    assert_eq!(report.required_bytes, RESERVE_BYTES);
}
