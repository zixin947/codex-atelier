//! Read DB + committed WAL from a stable, private copy. SQLite never opens the source.
use super::{inspect, read_bounded, Result};
use crate::gateway::storage::{atomic_private, private_dir};
use std::{
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};
#[derive(Clone, PartialEq, Eq)]
struct Stamp {
    len: u64,
    modified: Option<SystemTime>,
    identity: (u64, u64, i64, i64),
}
fn stamp(path: &Path) -> Result<Option<Stamp>> {
    let m = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err("无法读取会话索引元数据".into()),
    };
    if !m.is_file() || m.file_type().is_symlink() {
        return Err("会话索引或 WAL 不是普通文件".into());
    }
    #[cfg(unix)]
    let identity = {
        use std::os::unix::fs::MetadataExt;
        (m.dev(), m.ino(), m.ctime(), m.ctime_nsec())
    };
    #[cfg(not(unix))]
    let identity = (0, 0, 0, 0);
    Ok(Some(Stamp {
        len: m.len(),
        modified: m.modified().ok(),
        identity,
    }))
}
struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Result<Self> {
        let p = std::env::temp_dir().join(format!("atelier-diagnostic-{}", uuid::Uuid::new_v4()));
        private_dir(&p)?;
        Ok(Self(p))
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
struct ObservedCopy {
    _scratch: Scratch,
    path: PathBuf,
    source_stamp: (Option<Stamp>, Option<Stamp>),
}
thread_local! {
    static OBSERVED: std::cell::RefCell<Option<std::collections::BTreeMap<PathBuf, ObservedCopy>>> = const { std::cell::RefCell::new(None) };
}
/// Full diagnostic reads share streaming summaries, bounded to 4 GiB per operation.
pub(super) fn with_full_observation<T>(ctx: &super::Context, read: impl FnOnce() -> Result<T>) -> Result<T> {
    use atelier_maintenance_core::threads::observation;
    if observation::active() { return with_observation(read); }
    observation::with_full_scope(&[ctx.home.join("sessions"), ctx.home.join("archived_sessions")], 4 * 1024 * 1024 * 1024, || with_observation(read))?
}
/// Read-only phase only. Mutable planning projections still use independent copies.
pub(super) fn with_observation<T>(read: impl FnOnce() -> Result<T>) -> Result<T> {
    let owner = OBSERVED.with(|s| {
        let mut s = s.borrow_mut();
        if s.is_some() {
            false
        } else {
            *s = Some(Default::default());
            true
        }
    });
    struct Guard(bool);
    impl Drop for Guard {
        fn drop(&mut self) {
            if self.0 {
                OBSERVED.with(|s| *s.borrow_mut() = None);
            }
        }
    }
    let _guard = Guard(owner);
    atelier_maintenance_core::threads::observation::with_scope(|| {
        let result = read()?;
        atelier_maintenance_core::threads::observation::validate()?;
        OBSERVED.with(|s| -> Result<()> {
            if let Some(copies) = s.borrow().as_ref() {
                for (source, copy) in copies {
                    if copy.source_stamp
                        != (
                            stamp(source)?,
                            stamp(&PathBuf::from(format!("{}-wal", source.display())))?,
                        )
                    {
                        return Err("会话索引在本轮观察期间变化；请重新检测".into());
                    }
                }
            }
            Ok(())
        })?;
        Ok(result)
    })
}
pub(super) fn with_observed_path<T>(
    path: &Path,
    mut read: impl FnMut(&Path, &rusqlite::Connection) -> Result<T>,
) -> Result<T> {
    if !OBSERVED.with(|s| s.borrow().is_some()) {
        return with_snapshot_path(path, read);
    }
    let before = (
        stamp(path)?,
        stamp(&PathBuf::from(format!("{}-wal", path.display())))?,
    );
    let existing = OBSERVED.with(|s| {
        s.borrow()
            .as_ref()
            .unwrap()
            .get(path)
            .map(|c| (c.path.clone(), c.source_stamp.clone()))
    });
    let copy = match existing {
        Some((copy, expected)) => {
            if before != expected {
                return Err("会话索引在本轮观察期间变化；请重新检测".into());
            }
            copy
        }
        None => {
            let (scratch, copy) = with_snapshot_path(path, |_, connection| {
                // Each retry owns a fresh destination. VACUUM includes committed WAL pages.
                let scratch = Scratch::new()?;
                let copy = scratch.0.join("observed.sqlite");
                connection
                    .execute("VACUUM INTO ?1", [copy.to_string_lossy().as_ref()])
                    .map_err(|_| "无法建立本轮会话观察快照")?;
                Ok((scratch, copy))
            })?;
            OBSERVED.with(|s| {
                s.borrow_mut().as_mut().unwrap().insert(
                    path.into(),
                    ObservedCopy {
                        _scratch: scratch,
                        path: copy.clone(),
                        source_stamp: before.clone(),
                    },
                )
            });
            copy
        }
    };
    let connection = inspect::open_db(&copy, false)?;
    let result = read(&copy, &connection);
    if before
        != (
            stamp(path)?,
            stamp(&PathBuf::from(format!("{}-wal", path.display())))?,
        )
    {
        return Err("会话索引在本轮观察期间变化；请重新检测".into());
    }
    result
}
pub(super) fn read_rows(path: &Path) -> Result<Vec<inspect::Row>> {
    with_connection(path, inspect::rows)
}
pub(super) fn with_connection<T>(
    path: &Path,
    read: impl FnMut(&rusqlite::Connection) -> Result<T>,
) -> Result<T> {
    let mut read = read;
    with_snapshot_hook(path, |_, connection| read(connection), || {})
}
#[cfg(test)]
fn read_rows_with_hook(path: &Path, after_copy: impl FnMut()) -> Result<Vec<inspect::Row>> {
    read_with_hook(path, inspect::rows, after_copy)
}
/// The private path remains valid only during this callback. Source SQLite is never opened.
pub(super) fn with_snapshot_path<T>(
    path: &Path,
    read: impl FnMut(&Path, &rusqlite::Connection) -> Result<T>,
) -> Result<T> {
    with_snapshot_hook(path, read, || {})
}
/// Keep all copies alive for one verification, including committed WAL data.
/// Core verification may open SQLite connections; none of them receive source paths.
pub(super) fn with_verification_paths<T>(
    paths: &atelier_maintenance_core::router_verification::VerificationPaths,
    mut read: impl FnMut(&atelier_maintenance_core::router_verification::VerificationPaths) -> Result<T>,
) -> Result<T> {
    with_observation(|| with_verification_paths_observed(paths, &mut read))
}
fn with_verification_paths_observed<T>(
    paths: &atelier_maintenance_core::router_verification::VerificationPaths,
    read: &mut impl FnMut(
        &atelier_maintenance_core::router_verification::VerificationPaths,
    ) -> Result<T>,
) -> Result<T> {
    fn next<T>(
        paths: &atelier_maintenance_core::router_verification::VerificationPaths,
        copies: &mut Vec<PathBuf>,
        read: &mut impl FnMut(
            &atelier_maintenance_core::router_verification::VerificationPaths,
        ) -> Result<T>,
    ) -> Result<T> {
        let n = copies.len();
        if n == paths.databases.len() {
            let mut projected = paths.clone();
            projected.databases = copies.clone();
            return read(&projected);
        }
        with_observed_path(&paths.databases[n], |copy, _| {
            copies.push(copy.to_path_buf());
            let result = next(paths, copies, read);
            copies.pop();
            result
        })
    }
    next(paths, &mut Vec::new(), &mut |projected| {
        let result = read(projected);
        // Replace private paths in errors at the wrapper boundary, not in data.
        result.map_err(|mut error| {
            for (copy, source) in projected.databases.iter().zip(&paths.databases) {
                error = error.replace(
                    copy.to_string_lossy().as_ref(),
                    source.to_string_lossy().as_ref(),
                );
            }
            error
        })
    })
}

#[cfg(test)]
fn read_with_hook<T>(
    path: &Path,
    mut read: impl FnMut(&rusqlite::Connection) -> Result<T>,
    after_copy: impl FnMut(),
) -> Result<T> {
    with_snapshot_hook(path, |_, connection| read(connection), after_copy)
}
#[cfg(test)]
thread_local! {
    static SNAPSHOT_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}
#[cfg(test)]
pub(super) fn take_snapshot_count() -> usize {
    SNAPSHOT_COUNT.with(|count| count.replace(0))
}
// Hook is internal and no-op in production; tests exercise repeated external writes deterministically.
fn with_snapshot_hook<T>(
    path: &Path,
    mut read: impl FnMut(&Path, &rusqlite::Connection) -> Result<T>,
    mut after_copy: impl FnMut(),
) -> Result<T> {
    let wal = PathBuf::from(format!("{}-wal", path.display()));
    for _ in 0..2 {
        let before = (stamp(path)?, stamp(&wal)?);
        if before.0.is_none() {
            return Err("会话索引不存在".into());
        }
        let scratch = Scratch::new()?;
        #[cfg(test)]
        SNAPSHOT_COUNT.with(|count| count.set(count.get() + 1));
        let db = scratch.0.join("snapshot.sqlite");
        let copy = (|| -> Result<()> {
            atomic_private(&db, &read_bounded(path, 128 * 1024 * 1024)?)?;
            if before.1.as_ref().is_some_and(|s| s.len > 0) {
                let bytes = read_bounded(&wal, 64 * 1024 * 1024)?;
                // 运行中的 Codex 会让 WAL 代次重置或尾部未提交。此时丢弃 WAL 副本、
                // 仅读主库即可，不把这种正常竞争报成故障。
                if super::wal::validate_relaxed(&bytes)? == super::wal::WalStatus::Usable {
                    atomic_private(&scratch.0.join("snapshot.sqlite-wal"), &bytes)?;
                }
            }
            Ok(())
        })();
        after_copy();
        if before != (stamp(path)?, stamp(&wal)?) {
            continue;
        }
        copy?;
        let result = (|| -> Result<T> {
            let connection = inspect::open_db(&db, true)?;
            let health: String = connection
                .query_row("PRAGMA quick_check(1)", [], |r| r.get(0))
                .map_err(|_| "会话索引快照校验失败")?;
            if health != "ok" {
                return Err("会话索引快照结构无效；未返回不完整统计".into());
            }
            read(&db, &connection)
        })();
        if before != (stamp(path)?, stamp(&wal)?) {
            continue;
        }
        return result;
    }
    Err("会话索引持续被外部更新，两次快照均不稳定；请稍后重新检测".into())
}
#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    #[test]
    fn reads_committed_wal_without_changing_source_files() {
        let temp = Scratch::new().unwrap();
        let p = temp.0.join("source.sqlite");
        let conn = Connection::open(&p).unwrap();
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA wal_autocheckpoint=0; CREATE TABLE threads(id TEXT,model_provider TEXT,model TEXT,rollout_path TEXT); PRAGMA wal_checkpoint(TRUNCATE); INSERT INTO threads VALUES('wal-thread','openai','fixture','sessions/wal.jsonl');").unwrap();
        let before_db = fs::read(&p).unwrap();
        let wp = temp.0.join("source.sqlite-wal");
        let sp = temp.0.join("source.sqlite-shm");
        let before_wal = fs::read(&wp).unwrap();
        let before_shm = fs::read(&sp).unwrap();
        let rows = read_rows(&p).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].id, "wal-thread");
        assert_eq!(fs::read(&p).unwrap(), before_db);
        assert_eq!(fs::read(wp).unwrap(), before_wal);
        assert_eq!(fs::read(sp).unwrap(), before_shm);
    }
    #[test]
    fn concurrent_source_changes_retry_then_report_unknown() {
        let temp = Scratch::new().unwrap();
        let p = temp.0.join("source.sqlite");
        let conn = Connection::open(&p).unwrap();
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA wal_autocheckpoint=0; CREATE TABLE threads(id TEXT,model_provider TEXT,model TEXT,rollout_path TEXT);").unwrap();
        let mut calls = 0;
        let result = read_rows_with_hook(&p, || {
            calls += 1;
            conn.execute(
                "INSERT INTO threads VALUES(?1,'openai','','')",
                [format!("thread-{calls}")],
            )
            .unwrap();
        });
        assert_eq!(calls, 2);
        assert!(result.unwrap_err().contains("持续被外部更新"));
    }
    #[test]
    fn observation_rejects_late_wal_changes_and_new_round_reads_commits() {
        let temp = Scratch::new().unwrap();
        let path = temp.0.join("source.sqlite");
        let writer = Connection::open(&path).unwrap();
        writer.execute_batch("PRAGMA journal_mode=WAL; PRAGMA wal_autocheckpoint=0; CREATE TABLE sample(value INTEGER); INSERT INTO sample VALUES(1);").unwrap();
        let count = |_: &Path, connection: &Connection| -> Result<i64> {
            connection
                .query_row("SELECT COUNT(*) FROM sample", [], |r| r.get(0))
                .map_err(|e| e.to_string())
        };
        take_snapshot_count();
        let result = with_observation(|| {
            assert_eq!(with_observed_path(&path, count)?, 1);
            assert_eq!(with_observed_path(&path, count)?, 1);
            writer.execute("INSERT INTO sample VALUES(2)", []).unwrap();
            Ok(())
        });
        assert!(result.unwrap_err().contains("观察期间变化"));
        assert_eq!(take_snapshot_count(), 1);
        assert_eq!(
            with_observation(|| with_observed_path(&path, count)).unwrap(),
            2
        );
        assert_eq!(take_snapshot_count(), 1);
    }
    #[test]
    fn observation_rejects_db_and_wal_changes_at_each_boundary() {
        for mode in ["DELETE", "WAL"] {
            for boundary in ["cached", "callback", "scope"] {
                let temp = Scratch::new().unwrap();
                // Stability checks must not depend on a .sqlite extension.
                let path = temp.0.join("source.db");
                let writer = Connection::open(&path).unwrap();
                writer.execute_batch(&format!(
                    "PRAGMA journal_mode={mode}; PRAGMA wal_autocheckpoint=0; CREATE TABLE sample(value INTEGER); INSERT INTO sample VALUES(1);"
                )).unwrap();
                let mutate = || {
                    writer.execute("INSERT INTO sample VALUES(2)", []).unwrap();
                };
                let result = with_observation(|| {
                    with_observed_path(&path, |_, connection| {
                        let count: i64 = connection
                            .query_row("SELECT COUNT(*) FROM sample", [], |r| r.get(0))
                            .unwrap();
                        assert_eq!(count, 1);
                        if boundary == "callback" {
                            mutate();
                        }
                        Ok(())
                    })?;
                    if boundary == "callback" {
                        panic!("callback mutation was accepted");
                    }
                    mutate();
                    if boundary == "cached" {
                        with_observed_path(&path, |_, _| {
                            panic!("stale cached observation was reused")
                        })?;
                    }
                    Ok(())
                });
                assert!(
                    result.unwrap_err().contains("观察期间变化"),
                    "{mode}/{boundary}"
                );
                // A rejected scope must not leave a stale private copy in the next round.
                let count = with_observation(|| {
                    with_observed_path(&path, |_, connection| {
                        connection
                            .query_row("SELECT COUNT(*) FROM sample", [], |r| r.get::<_, i64>(0))
                            .map_err(|e| e.to_string())
                    })
                })
                .unwrap();
                assert_eq!(count, 2);
            }
        }
    }
    #[test]
    fn transient_source_change_retries_with_a_fresh_snapshot() {
        let temp = Scratch::new().unwrap();
        let path = temp.0.join("source.db");
        let writer = Connection::open(&path).unwrap();
        writer
            .execute_batch("CREATE TABLE sample(value INTEGER); INSERT INTO sample VALUES(1);")
            .unwrap();
        let mut attempts = 0;
        take_snapshot_count();
        let count = read_with_hook(
            &path,
            |connection| {
                connection
                    .query_row("SELECT COUNT(*) FROM sample", [], |r| r.get::<_, i64>(0))
                    .map_err(|e| e.to_string())
            },
            || {
                attempts += 1;
                if attempts == 1 {
                    writer.execute("INSERT INTO sample VALUES(2)", []).unwrap();
                }
            },
        )
        .unwrap();
        assert_eq!(count, 2);
        assert_eq!(attempts, 2);
        assert_eq!(take_snapshot_count(), 2);
    }
    #[test]
    fn invalid_wal_is_not_silently_ignored() {
        let temp = Scratch::new().unwrap();
        let p = temp.0.join("source.sqlite");
        let conn = Connection::open(&p).unwrap();
        conn.execute_batch("CREATE TABLE threads(id TEXT,rollout_path TEXT)")
            .unwrap();
        drop(conn);
        fs::write(temp.0.join("source.sqlite-wal"), vec![0; 64]).unwrap();
        assert!(read_rows(&p).unwrap_err().contains("WAL"));
    }
}
