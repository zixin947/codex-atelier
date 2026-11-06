//! Read DB + committed WAL from a stable, private copy. SQLite never opens the source.
use super::{inspect, read_bounded, Result};
use crate::gateway::storage::{atomic_private, private_dir};
use std::{
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};
#[derive(PartialEq, Eq)]
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
pub(super) fn read_rows(path: &Path) -> Result<Vec<inspect::Row>> {
    with_connection(path, inspect::rows)
}
pub(super) fn with_connection<T>(
    path: &Path,
    read: impl FnMut(&rusqlite::Connection) -> Result<T>,
) -> Result<T> {
    read_with_hook(path, read, || {})
}
#[cfg(test)]
fn read_rows_with_hook(path: &Path, after_copy: impl FnMut()) -> Result<Vec<inspect::Row>> {
    read_with_hook(path, inspect::rows, after_copy)
}
// Hook is internal and no-op in production; tests exercise repeated external writes deterministically.
fn read_with_hook<T>(
    path: &Path,
    mut read: impl FnMut(&rusqlite::Connection) -> Result<T>,
    mut after_copy: impl FnMut(),
) -> Result<T> {
    let wal = PathBuf::from(format!("{}-wal", path.display()));
    for _ in 0..2 {
        let before = (stamp(path)?, stamp(&wal)?);
        if before.0.is_none() {
            return Err("会话索引不存在".into());
        }
        let scratch = Scratch::new()?;
        let db = scratch.0.join("snapshot.sqlite");
        let copy = (|| -> Result<()> {
            atomic_private(&db, &read_bounded(path, 128 * 1024 * 1024)?)?;
            if before.1.as_ref().is_some_and(|s| s.len > 0) {
                let bytes = read_bounded(&wal, 64 * 1024 * 1024)?;
                super::wal::validate(&bytes)?;
                atomic_private(&scratch.0.join("snapshot.sqlite-wal"), &bytes)?;
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
            read(&connection)
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
