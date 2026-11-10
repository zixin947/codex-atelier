//! Durable router-transition journal recovered from the shipped arm64 application.
//! Paths and timestamps are supplied explicitly; this module never discovers a home
//! directory or launches a process. Event-log categorization is a separate concern.
use crate::catalog::publication::{write_atomic_private_report, Publication};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

pub const FILE_NAME: &str = "router-transition-journal.json";
pub const SIZE_LIMIT: u64 = 1024 * 1024;
pub const RECOVERY_REQUIRED: &str = "ROUTER_COMMIT_RECOVERY_REQUIRED";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransitionTarget {
    RouterOn,
    NativeOff,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransitionJournal {
    pub schema_version: u8,
    pub target: TransitionTarget,
    pub phase: String,
    pub started_at_ms: i64,
    pub updated_at_ms: i64,
    pub reason: Option<String>,
}
impl TransitionJournal {
    pub fn is_active(&self) -> bool {
        is_active_phase(&self.phase)
    }
    pub fn is_commit_pending(&self) -> bool {
        self.phase == "committing"
    }
}
pub fn is_active_phase(phase: &str) -> bool {
    matches!(
        phase,
        "retrying" | "committing" | "reconciling" | "stopping_codex"
    )
}
fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}
fn recovery_required() -> io::Error {
    io::Error::other(format!("{RECOVERY_REQUIRED}: pending transition must be recovered before starting or clearing another transition"))
}

/// Production filesystem operations, injectable only to reproduce publication/removal
/// durability failures in fixtures. A returned Publication means rename already happened.
pub trait JournalIo {
    fn read_regular(&self, path: &Path) -> io::Result<Option<Vec<u8>>>;
    fn publish(&self, path: &Path, content: &[u8]) -> io::Result<Publication>;
    fn remove(&self, path: &Path) -> io::Result<Publication>;
}
#[derive(Clone, Copy, Default)]
pub struct LocalJournalIo;
impl JournalIo for LocalJournalIo {
    fn read_regular(&self, path: &Path) -> io::Result<Option<Vec<u8>>> {
        let meta = match fs::symlink_metadata(path) {
            Ok(meta) => meta,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(e),
        };
        if !meta.is_file() {
            return Err(invalid("phase record is not a regular file"));
        }
        let mut bytes = Vec::new();
        fs::File::open(path)?
            .take(SIZE_LIMIT + 1)
            .read_to_end(&mut bytes)?;
        if bytes.len() as u64 > SIZE_LIMIT {
            return Err(invalid("phase record exceeds the size limit"));
        }
        Ok(Some(bytes))
    }
    fn publish(&self, path: &Path, content: &[u8]) -> io::Result<Publication> {
        write_atomic_private_report(path, content)
    }
    fn remove(&self, path: &Path) -> io::Result<Publication> {
        match fs::remove_file(path) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
        let directory_sync_error = path
            .parent()
            .and_then(|p| fs::File::open(p).and_then(|f| f.sync_all()).err());
        Ok(Publication {
            destination: path.to_owned(),
            directory_sync_error,
        })
    }
}

pub struct JournalStore<I = LocalJournalIo> {
    path: PathBuf,
    io: I,
}
impl JournalStore<LocalJournalIo> {
    pub fn new(relay_root: &Path) -> Self {
        Self::with_io(relay_root, LocalJournalIo)
    }
}
impl<I: JournalIo> JournalStore<I> {
    pub fn with_io(relay_root: &Path, io: I) -> Self {
        Self {
            path: relay_root.join(FILE_NAME),
            io,
        }
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
    /// Version is checked before parsing fields of a possibly newer schema.
    pub fn read_checked(&self) -> io::Result<Option<TransitionJournal>> {
        let Some(bytes) = self.io.read_regular(&self.path)? else {
            return Ok(None);
        };
        if bytes.len() as u64 > SIZE_LIMIT {
            return Err(invalid("phase record exceeds the size limit"));
        }
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Envelope {
            schema_version: u8,
        }
        let envelope: Envelope =
            serde_json::from_slice(&bytes).map_err(|e| invalid(e.to_string()))?;
        if envelope.schema_version != 1 {
            return Err(invalid("unsupported transition record version"));
        }
        serde_json::from_slice(&bytes)
            .map(Some)
            .map_err(|e| invalid(e.to_string()))
    }
    /// Native diagnostic convenience path logs malformed records and yields None;
    /// mutating operations below use read_checked and propagate those errors.
    pub fn read(&self) -> Option<TransitionJournal> {
        match self.read_checked() {
            Ok(record) => record,
            Err(e) => {
                eprintln!(
                    "router transition journal could not be read: {:?}",
                    e.kind()
                );
                None
            }
        }
    }
    pub fn in_flight(&self) -> Option<TransitionJournal> {
        self.read().filter(TransitionJournal::is_active)
    }
    pub fn commit_pending(&self) -> io::Result<bool> {
        Ok(self.read_checked()?.is_some_and(|r| r.is_commit_pending()))
    }
    fn append_event(
        &self,
        target: TransitionTarget,
        phase: &str,
        source: Option<&str>,
        reason: Option<&str>,
        now_ms: i64,
    ) {
        if let Some(root) = self.path.parent() {
            let _ = crate::transition_events::append_event_at(
                root, target, phase, source, reason, now_ms,
            );
        }
    }
    fn write_inner(
        &self,
        target: TransitionTarget,
        phase: &str,
        source: Option<&str>,
        reason: Option<&str>,
        now_ms: i64,
        allow_pending: bool,
    ) -> io::Result<Publication> {
        let prior = self.read_checked()?;
        if !allow_pending
            && prior
                .as_ref()
                .is_some_and(TransitionJournal::is_commit_pending)
        {
            self.append_event(target, phase, source, reason, now_ms);
            return Err(recovery_required());
        }
        let started_at_ms = prior
            .filter(|r| r.target == target)
            .map_or(now_ms, |r| r.started_at_ms);
        let record = TransitionJournal {
            schema_version: 1,
            target,
            phase: phase.into(),
            started_at_ms,
            updated_at_ms: now_ms,
            reason: reason.map(str::to_owned),
        };
        let bytes = serde_json::to_vec_pretty(&record).map_err(|e| invalid(e.to_string()))?;
        if bytes.len() as u64 > SIZE_LIMIT {
            return Err(invalid("phase record exceeds the size limit"));
        }
        let publication = self.io.publish(&self.path, &bytes)?;
        self.append_event(target, phase, source, reason, now_ms);
        Ok(publication)
    }
    pub fn write(
        &self,
        target: TransitionTarget,
        phase: &str,
        reason: Option<&str>,
        now_ms: i64,
    ) -> io::Result<Publication> {
        self.write_with_source(target, phase, None, reason, now_ms)
    }
    pub fn write_with_source(
        &self,
        target: TransitionTarget,
        phase: &str,
        source: Option<&str>,
        reason: Option<&str>,
        now_ms: i64,
    ) -> io::Result<Publication> {
        self.write_inner(target, phase, source, reason, now_ms, false)
    }
    /// A committing record must survive an unconfirmed directory sync. Caller must
    /// not discard it or compensate as if publication never happened.
    pub fn begin_commit(
        &self,
        target: TransitionTarget,
        reason: Option<&str>,
        now_ms: i64,
    ) -> io::Result<PathBuf> {
        self.begin_commit_with_source(target, None, reason, now_ms)
    }
    pub fn begin_commit_with_source(
        &self,
        target: TransitionTarget,
        source: Option<&str>,
        reason: Option<&str>,
        now_ms: i64,
    ) -> io::Result<PathBuf> {
        self.write_inner(target, "committing", source, reason, now_ms, false)?
            .require_sync()
    }
    pub fn recover_write(
        &self,
        target: TransitionTarget,
        phase: &str,
        reason: Option<&str>,
        now_ms: i64,
    ) -> io::Result<Publication> {
        self.write_inner(target, phase, None, reason, now_ms, true)
    }
    /// None is native ordinary clear: it refuses any pending commit. A matching
    /// explicit target is the recovery/verified-commit clear path.
    pub fn clear(&self, target: Option<TransitionTarget>) -> io::Result<bool> {
        self.clear_at(target, now_millis())
    }
    pub fn clear_at(&self, target: Option<TransitionTarget>, now_ms: i64) -> io::Result<bool> {
        let Some(record) = self.read_checked()? else {
            return Ok(false);
        };
        if (target.is_none() && record.is_commit_pending())
            || target.is_some_and(|t| t != record.target)
        {
            return Err(recovery_required());
        }
        let removal = self.io.remove(&self.path)?;
        if let Some(e) = removal.directory_sync_error {
            self.append_event(
                record.target,
                "cleanup_sync_unconfirmed",
                None,
                None,
                now_ms,
            );
            return Err(io::Error::new(e.kind(), "ROUTER_JOURNAL_REMOVED_SYNC_UNCONFIRMED: journal removed; directory sync unconfirmed"));
        }
        self.append_event(
            record.target,
            "cleared",
            None,
            Some("transition completed or already stable"),
            now_ms,
        );
        Ok(true)
    }
}
fn now_millis() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_millis() as i64,
        Err(e) => -(e.duration().as_millis() as i64),
    }
}
