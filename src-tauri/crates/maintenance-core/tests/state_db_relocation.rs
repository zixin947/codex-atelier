use atelier_maintenance_core::state_db::*;
use std::{
    io,
    path::{Path, PathBuf},
};

fn context(root: &Path) -> RelocationContext {
    RelocationContext {
        codex_home: root.join("codex"),
        configured_sqlite_home: Some(root.join("codex/sqlite")),
        active_database: root.join("codex/sqlite/state_5.sqlite"),
        relay_root: root.join("relay"),
    }
}
fn file(path: &Path, data: &[u8]) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, data).unwrap();
}

#[test]
fn real_files_backup_and_remove_database_wal_and_shm_without_touching_active_layout() {
    let t = tempfile::tempdir().unwrap();
    let ctx = context(t.path());
    let db = ctx.codex_home.join("state_5.sqlite");
    let originals = [
        b"old database".as_slice(),
        b"old WAL".as_slice(),
        b"old SHM".as_slice(),
    ];
    for (p, bytes) in related_files(&db).into_iter().zip(originals) {
        file(&p, bytes);
    }
    file(&ctx.active_database, b"active must survive");
    assert_eq!(planned_old_files(&LocalFs, &ctx).len(), 3);
    let result = fix_state_db_relocation(&LocalFs, &ctx, 1234);
    assert!(result.fixed && !result.failed);
    assert_eq!(result.removed_databases, 1);
    assert!(result.remaining.is_empty());
    for (p, bytes) in related_files(&db).into_iter().zip(originals) {
        assert!(!p.exists());
        assert_eq!(
            std::fs::read(
                ctx.relay_root
                    .join("state-db-backups")
                    .join(format!("1234.{}", p.file_name().unwrap().to_str().unwrap()))
            )
            .unwrap(),
            bytes
        );
    }
    assert_eq!(
        std::fs::read(&ctx.active_database).unwrap(),
        b"active must survive"
    );
}

#[test]
fn native_gate_requires_exact_configured_sqlite_subdirectory_not_any_other_location() {
    let t = tempfile::tempdir().unwrap();
    let mut ctx = context(t.path());
    let db = ctx.codex_home.join("state_5.sqlite");
    file(&db, b"keep");
    ctx.configured_sqlite_home = None;
    assert_eq!(assess(&LocalFs, &ctx), Assessment::UnconfirmedConfiguration);
    assert!(fix_state_db_relocation(&LocalFs, &ctx, 1).failed);
    ctx.configured_sqlite_home = Some(ctx.codex_home.join("sqlite/child"));
    assert_eq!(assess(&LocalFs, &ctx), Assessment::UnconfirmedConfiguration);
    ctx.configured_sqlite_home = Some(ctx.codex_home.join("sqlite"));
    ctx.active_database = t.path().join("another/sqlite/state_5.sqlite");
    assert_eq!(assess(&LocalFs, &ctx), Assessment::ActiveOutsideSqlite);
    assert!(fix_state_db_relocation(&LocalFs, &ctx, 1).fixed);
    assert_eq!(std::fs::read(&db).unwrap(), b"keep");
    assert!(!ctx.relay_root.exists());
}

struct CopyFailure {
    fail: PathBuf,
}
impl RelocationFs for CopyFailure {
    fn read_dir(&self, p: &Path) -> io::Result<Vec<PathBuf>> {
        LocalFs.read_dir(p)
    }
    fn is_file(&self, p: &Path) -> bool {
        LocalFs.is_file(p)
    }
    fn metadata(&self, p: &Path) -> io::Result<()> {
        LocalFs.metadata(p)
    }
    fn create_dir_all(&self, p: &Path) -> io::Result<()> {
        LocalFs.create_dir_all(p)
    }
    fn copy(&self, p: &Path, to: &Path) -> io::Result<()> {
        if p == self.fail {
            Err(io::ErrorKind::PermissionDenied.into())
        } else {
            LocalFs.copy(p, to)
        }
    }
    fn remove_file(&self, p: &Path) -> io::Result<()> {
        LocalFs.remove_file(p)
    }
}

#[test]
fn copy_failure_retains_its_source_continues_other_files_and_reports_partial_failure() {
    let t = tempfile::tempdir().unwrap();
    let ctx = context(t.path());
    let db = ctx.codex_home.join("state_5.sqlite");
    for p in related_files(&db) {
        file(&p, b"original");
    }
    let result = fix_state_db_relocation(&CopyFailure { fail: db.clone() }, &ctx, 12);
    assert!(result.failed && !result.fixed);
    assert_eq!(result.removed_databases, 0);
    assert_eq!(result.remaining, vec![db.clone()]);
    assert!(db.exists());
    assert!(!related_files(&db)[1].exists());
    assert_eq!(result.backups.len(), 2);
}

#[test]
fn delete_failure_is_not_hidden_by_a_successful_database_cleanup() {
    let t = tempfile::tempdir().unwrap();
    let ctx = context(t.path());
    let db = ctx.codex_home.join("state_5.sqlite");
    file(&db, b"old");
    let wal = related_files(&db)[1].clone();
    std::fs::create_dir_all(&wal).unwrap();
    let result = fix_state_db_relocation(&LocalFs, &ctx, 9);
    assert!(result.failed && !result.fixed);
    assert_eq!(result.removed_databases, 1);
    assert_eq!(result.remaining, vec![wal]);
}

#[test]
fn filename_filter_is_native_prefix_suffix_without_numeric_or_recursive_restriction() {
    let t = tempfile::tempdir().unwrap();
    let ctx = context(t.path());
    for n in [
        "state_5.sqlite",
        "state_custom.sqlite",
        "state_.sqlite",
        "state_5.sqlite-wal",
        "state5.sqlite",
        "STATE_5.sqlite",
        "unrelated",
    ] {
        file(&ctx.codex_home.join(n), b"x");
    }
    file(&ctx.codex_home.join("nested/state_7.sqlite"), b"nested");
    let paths = list_state_dbs_in(&LocalFs, &ctx.codex_home);
    assert_eq!(paths.len(), 3);
    assert!(paths.contains(&ctx.codex_home.join("state_custom.sqlite")));
    assert!(paths.contains(&ctx.codex_home.join("state_.sqlite")));
}

#[test]
fn config_wins_environment_and_relative_paths_use_cwd_without_expanding_tilde() {
    let cwd = Path::new("/synthetic/cwd");
    assert_eq!(
        configured_sqlite_home(
            Some("sqlite_home='  relative  '"),
            Some("/environment".as_ref()),
            Some(cwd)
        ),
        Some(cwd.join("relative"))
    );
    for invalid in ["sqlite_home=42", "sqlite_home=''", "[bad", ""] {
        assert_eq!(
            configured_sqlite_home(Some(invalid), Some("env-relative".as_ref()), Some(cwd)),
            Some(cwd.join("env-relative"))
        );
    }
    assert_eq!(
        configured_sqlite_home(Some("sqlite_home='~/db'"), None, Some(cwd)),
        Some(cwd.join("~/db"))
    );
    assert_eq!(
        configured_sqlite_home(None, Some(" ".as_ref()), Some(cwd)),
        Some(cwd.join(" "))
    );
    assert_eq!(configured_sqlite_home(None, None, Some(cwd)), None);
}
