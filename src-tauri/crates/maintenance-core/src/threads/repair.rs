//! Native maintenance writes on explicitly supplied offline paths. No implicit user paths.
use crate::threads::{
    self,
    consistency::{self, ModelDisposition},
};
use rusqlite::{Connection, OpenFlags};
use rusqlite::OptionalExtension;use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairError(pub String);
impl std::fmt::Display for RepairError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for RepairError {}
impl From<std::io::Error> for RepairError {
    fn from(e: std::io::Error) -> Self {
        Self(format!("IO error: {e}"))
    }
}
impl From<rusqlite::Error> for RepairError {
    fn from(e: rusqlite::Error) -> Self {
        Self(format!("Operation failed: {e}"))
    }
}
type Result<T> = std::result::Result<T, RepairError>;
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RepairSummary {
    pub scanned: usize,
    pub db_updated: usize,
    pub catalog_updated: usize,
    pub rollout_updated: usize,
    pub restored: usize,
    pub archived: usize,
    pub backups: Vec<PathBuf>,
    pub warnings: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MissingRolloutAction {
    Restore {
        source: PathBuf,
        destination: PathBuf,
    },
    Archive,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingRolloutChange {
    pub db: PathBuf,
    pub id: String,
    pub provider: String,
    pub recorded_path: Option<String>,
    pub io_path: Option<PathBuf>,
    pub action: MissingRolloutAction,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingRolloutRepairPlan {
    pub changes: Vec<MissingRolloutChange>,
    pub backup_directory: PathBuf,
    pub scanned: usize,
    pub databases: Vec<PathBuf>,
    pub trusted_backup_root: PathBuf,
    pub rollout_base: Option<PathBuf>,
}
fn open_rw(db: &Path) -> Result<Connection> {
    let c = Connection::open_with_flags(
        db,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )?;
    c.busy_timeout(Duration::from_secs(5))?;
    Ok(c)
}
fn token() -> String {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    format!(
        "{}-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos(),
        NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    )
}
fn backup_file(path: &Path, root: &Path, tag: &str) -> Result<PathBuf> {
    fs::create_dir_all(root)?;
    let p = root.join(format!(
        "{}-{tag}-{}",
        token(),
        path.file_name().unwrap_or_default().to_string_lossy()
    ));
    fs::copy(path, &p)?;
    fs::File::open(&p)?.sync_all()?;
    Ok(p)
}
fn write_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    use std::io::Write;
    let parent = path
        .parent()
        .ok_or_else(|| RepairError("rollout has no parent".into()))?;
    fs::create_dir_all(parent)?;
    let tmp = parent.join(format!(
        ".{}.repair-{}",
        path.file_name().unwrap_or_default().to_string_lossy(),
        token()
    ));
    let result = (|| -> Result<()> {
        let mut f = fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&tmp)?;
        f.write_all(bytes)?;
        f.sync_all()?;
        fs::rename(&tmp, path)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(tmp);
    }
    result
}
fn row_unchanged(c: &Connection, id: &str, provider: &str, path: Option<&str>) -> Result<bool> {
    let r = c.query_row(
        "SELECT model_provider, rollout_path FROM threads WHERE id=?1 AND COALESCE(archived,0)=0",
        [id],
        |r| Ok((r.get::<_, String>(0)?, r.get::<_, Option<String>>(1)?)),
    );
    match r {
        Ok((p, r)) => Ok(p == provider && r.as_deref() == path),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(false),
        Err(e) => Err(e.into()),
    }
}
/// @0x1008cef90: restore exact-name, identity-proven backup or archive an active managed/native row.
pub fn plan_missing_rollout_repair(
    dbs: &[PathBuf],
    trusted_backup_root: &Path,
    backup_directory: &Path,
) -> Result<MissingRolloutRepairPlan> {
    let mut changes = vec![];
    let mut scanned = 0;
    for db in dbs {
        if fs::metadata(db).is_err() {
            continue;
        }
        let c = open_rw(db)?;
        let rows = threads::collect_provider_convergence_targets(&c).map_err(RepairError)?;
        scanned += rows.len();
        for row in rows {
            let action = match row.rollout_path.as_deref() {
                None => MissingRolloutAction::Archive,
                Some(p) => match threads::resolve_rollout(Path::new(p))? {
                    Some(_) => continue,
                    None => match threads::find_trusted_rollout_backup(
                        trusted_backup_root,
                        &row.id,
                        Path::new(p),
                    ) {
                        Some(source) => MissingRolloutAction::Restore {
                            source,
                            destination: p.into(),
                        },
                        None => MissingRolloutAction::Archive,
                    },
                },
            };
            changes.push(MissingRolloutChange {
                db: db.clone(),
                id: row.id,
                provider: row.provider,
                io_path: row.rollout_path.as_ref().map(PathBuf::from),
                recorded_path: row.rollout_path,
                action,
            });
        }
    }
    Ok(MissingRolloutRepairPlan {
        changes,
        backup_directory: backup_directory.into(),
        scanned,
        databases: dbs.to_vec(),
        trusted_backup_root: trusted_backup_root.into(),
        rollout_base: None,
    })
}
pub fn apply_missing_rollout_repair(plan: &MissingRolloutRepairPlan) -> Result<RepairSummary> {
    apply_missing_rollout_repair_with_verifier(plan, |plan| {
        Ok(inspect_missing_rollouts_with_base(
            &plan.databases,
            &plan.trusted_backup_root,
            plan.rollout_base.as_deref(),
        ))
    })
}
/// The application may supply its real private-snapshot inspection instead of opening source DBs for verification.
pub fn apply_missing_rollout_repair_with_verifier(
    plan: &MissingRolloutRepairPlan,
    verify: impl FnOnce(&MissingRolloutRepairPlan) -> Result<threads::MissingRolloutsInspection>,
) -> Result<RepairSummary> {
    let mut summary = RepairSummary {
        scanned: plan.scanned,
        ..Default::default()
    };
    let mut backed_up = BTreeSet::new();
    let mut audit = vec![];
    for change in &plan.changes {
        let c = open_rw(&change.db)?;
        if !row_unchanged(
            &c,
            &change.id,
            &change.provider,
            change.recorded_path.as_deref(),
        )? {
            return Err(RepairError(format!(
                "missing rollout candidate changed: {}",
                change.id
            )));
        }
        if let Some(p) = change.io_path.as_ref() {
            if threads::resolve_rollout(p)?.is_some() {
                continue;
            }
        }
        match &change.action {
            MissingRolloutAction::Restore {
                source,
                destination,
            } => {
                let physical = threads::resolve_rollout(source)?
                    .ok_or_else(|| RepairError("rollout backup is missing".into()))?;
                if physical != *source {
                    return Err(RepairError(
                        "rollout backup representation changed; history preserved".into(),
                    ));
                }
                if !threads::rollout_file_belongs_to_thread(source, &change.id) {
                    return Err(RepairError(format!(
                        "backup rollout does not belong to thread {}",
                        change.id
                    )));
                }
                let metadata = fs::metadata(source)?;
                let bytes = fs::read(source)?;
                write_atomic(destination, &bytes)?;
                if let Ok(modified) = metadata.modified() {
                    if let Ok(f) = fs::OpenOptions::new().write(true).open(destination) {
                        let _ = f.set_times(fs::FileTimes::new().set_modified(modified));
                    }
                }
                summary.restored += 1;
                summary.rollout_updated += 1;
                audit.push(json!({"thread_id":change.id,"source":source,"destination":destination,"status":"restored"}));
            }
            MissingRolloutAction::Archive => {
                if backed_up.insert(change.db.clone()) {
                    summary
                        .backups
                        .push(backup_database(&c, &change.db, &plan.backup_directory)?)
                }
                let n = c.execute(
                    "UPDATE threads SET archived = 1 WHERE id = ?1 AND COALESCE(archived, 0) = 0",
                    [&change.id],
                )?;
                summary.archived += n;
                summary.db_updated += n;
                let _ = c.execute_batch("PRAGMA wal_checkpoint(TRUNCATE);");
                audit.push(json!({"thread_id":change.id,"db":change.db,"status":"isolated"}));
            }
        }
    }
    if !audit.is_empty() {
        let result = (|| -> Result<PathBuf> {
            let root = plan.backup_directory.join("thread-rollout-repairs");
            fs::create_dir_all(&root)?;
            let report = root.join(format!("{}-missing-rollout-repair.json", token()));
            write_atomic(&report,&serde_json::to_vec_pretty(&json!({"records":audit,"restored":summary.restored,"isolated":summary.archived})).map_err(|e|RepairError(e.to_string()))?)?;
            Ok(report)
        })();
        match result {
            Ok(p) => summary.backups.push(p),
            Err(e) => summary
                .warnings
                .push(format!("write missing rollout repair report failed: {e}")),
        }
    }
    let verification = verify(plan)?;
    if verification.missing != 0 || !verification.errors.is_empty() {
        return Err(RepairError(format!(
            "missing rollout repair incomplete: missing={}, errors={:?}, examples={:?}",
            verification.missing, verification.errors, verification.examples
        )));
    }
    Ok(summary)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderTarget {
    Official,
    Router,
}
impl ProviderTarget {
    pub fn id(self) -> &'static str {
        match self {
            Self::Official => "openai",
            Self::Router => "aimai1",
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WriteRepairKind {
    Provider,
    Model,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadDbChange {
    pub db: PathBuf,
    pub id: String,
    pub expected_provider: String,
    pub expected_model: Option<String>,
    pub expected_rollout: Option<String>,
    pub replacement: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolloutFileChange {
    pub requested_path: PathBuf,
    pub physical_path: PathBuf,
    pub original: Vec<u8>,
    pub replacement: Vec<u8>,
    pub thread_ids: Vec<String>,
    pub databases: Vec<PathBuf>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadCatalogChange {
    pub db: PathBuf,
    pub host_id: String,
    pub target: String,
    pub has_missing_candidate: bool,
    pub entries: Vec<ThreadCatalogEntry>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadCatalogEntry {
    pub thread_id: String,
    pub expected_provider: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadWriteRepairPlan {
    pub kind: WriteRepairKind,
    pub db_changes: Vec<ThreadDbChange>,
    pub files: Vec<RolloutFileChange>,
    pub catalog_changes: Vec<ThreadCatalogChange>,
    pub warnings: Vec<String>,
    pub backup_directory: PathBuf,
    pub scanned: usize,
    pub no_account: bool,
    pub catalog_directory: Option<PathBuf>,
    pub model_restore_journal: Option<JournalFileChange>,
}
#[derive(Debug, Clone)]
struct Record {
    start: usize,
    content_end: usize,
    end: usize,
    value: Value,
}
fn records(bytes: &[u8]) -> Vec<Record> {
    let mut out = vec![];
    let mut start = 0;
    while start < bytes.len() {
        let end = bytes[start..]
            .iter()
            .position(|&b| b == b'\n')
            .map(|p| start + p + 1)
            .unwrap_or(bytes.len());
        let mut content_end = end;
        if bytes.get(content_end.wrapping_sub(1)) == Some(&b'\n') {
            content_end -= 1
        }
        if bytes.get(content_end.wrapping_sub(1)) == Some(&b'\r') {
            content_end -= 1
        }
        if let Ok(value) = serde_json::from_slice(&bytes[start..content_end]) {
            out.push(Record {
                start,
                content_end,
                end,
                value,
            });
        }
        start = end;
    }
    out
}
fn meta(v: &Value) -> bool {
    v.get("type").and_then(Value::as_str) == Some("session_meta")
}
fn native(v: &Value) -> bool {
    v.get("type").and_then(Value::as_str) == Some("event_msg")
        && v.pointer("/payload/type").and_then(Value::as_str) == Some("thread_settings_applied")
}
fn selected_records(rows: &[Record], id: &str) -> Result<(usize, usize, Option<usize>)> {
    let first = rows.iter().position(|r| meta(&r.value)).ok_or_else(|| {
        RepairError(format!(
            "Invalid data: session_meta missing for thread {id}"
        ))
    })?;
    if rows[first]
        .value
        .pointer("/payload/id")
        .and_then(Value::as_str)
        != Some(id)
    {
        return Err(RepairError(format!(
            "rollout identity does not match thread {id}; repair its reference before retrying"
        )));
    }
    let mut last = first;
    let mut settings = None;
    for (n, r) in rows.iter().enumerate().skip(first + 1) {
        if meta(&r.value) && r.value.pointer("/payload/id").and_then(Value::as_str) == Some(id) {
            last = n;
            settings = None
        } else if native(&r.value) {
            settings = Some(n)
        }
    }
    if let Some(n) = settings {
        match rows[n].value.pointer("/payload/thread_id") {
            None | Some(Value::Null) => (),
            Some(Value::String(s)) if s == id => (),
            _ => {
                return Err(RepairError(
                    "latest native settings belongs to another task; history preserved".into(),
                ))
            }
        }
    }
    Ok((first, last, settings))
}
/// @0x1008af6b4/0x1008af874. Paginated/ordinal records keep their exact UTF-8 byte length.
pub fn fit_record(original: &[u8], mut replacement: Vec<u8>) -> Result<Vec<u8>> {
    let v: Value =
        serde_json::from_slice(original).map_err(|e| RepairError(format!("JSON error: {e}")))?;
    let protected = v.get("ordinal").is_some()
        || v.pointer("/payload/history_mode").and_then(Value::as_str) == Some("paginated")
        || v.pointer("/payload/history_base")
            .is_some_and(|v| !v.is_null());
    if !protected {
        return Ok(replacement);
    }
    if replacement.len() > original.len() {
        while matches!(replacement.last(), Some(b' ' | b'\t')) {
            replacement.pop();
        }
    }
    if replacement.len() > original.len() {
        return Err(RepairError("ROUTER_HISTORY_LAYOUT_PROTECTED: native paginated history cannot grow an existing record; history preserved".into()));
    }
    replacement.resize(original.len(), b' ');
    Ok(replacement)
}
fn logical_file(path: &Path) -> Result<(PathBuf, Vec<u8>, Vec<u8>)> {
    let physical =
        threads::resolve_rollout(path)?.ok_or_else(|| RepairError("rollout is missing".into()))?;
    let original = fs::read(&physical)?;
    let logical = if physical.to_string_lossy().ends_with(".jsonl.zst") {
        zstd::stream::decode_all(original.as_slice())?
    } else {
        original.clone()
    };
    Ok((physical, original, logical))
}
fn rewrite_records(logical: &[u8], changes: &BTreeMap<usize, Vec<u8>>, rows: &[Record]) -> Vec<u8> {
    let mut out = vec![];
    let mut copied = 0;
    for (index, replacement) in changes {
        let r = &rows[*index];
        out.extend_from_slice(&logical[copied..r.start]);
        out.extend_from_slice(replacement);
        out.extend_from_slice(&logical[r.content_end..r.end]);
        copied = r.end;
    }
    out.extend_from_slice(&logical[copied..]);
    out
}
fn encode_logical(path: &Path, logical: Vec<u8>) -> Result<Vec<u8>> {
    if path.to_string_lossy().ends_with(".jsonl.zst") {
        Ok(zstd::stream::encode_all(logical.as_slice(), 3)?)
    } else {
        Ok(logical)
    }
}
fn payload_mut(v: &mut Value) -> Result<&mut serde_json::Map<String, Value>> {
    v.get_mut("payload")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| RepairError("Invalid data: session_meta payload not found".into()))
}
fn append_file(plan: &mut ThreadWriteRepairPlan, file: RolloutFileChange) -> Result<()> {
    if let Some(existing) = plan
        .files
        .iter_mut()
        .find(|f| f.physical_path == file.physical_path)
    {
        if existing.original != file.original || existing.replacement != file.replacement {
            return Err(RepairError(
                "shared rollout has incompatible repair targets; history preserved".into(),
            ));
        }
        existing.thread_ids.extend(file.thread_ids);
        existing.databases.extend(file.databases);
    } else {
        plan.files.push(file)
    }
    Ok(())
}
fn empty_plan(kind: WriteRepairKind, root: &Path) -> ThreadWriteRepairPlan {
    ThreadWriteRepairPlan {
        kind,
        db_changes: vec![],
        files: vec![],
        catalog_changes: vec![],
        warnings: vec![],
        backup_directory: root.into(),
        scanned: 0,
        no_account: false,
        catalog_directory: None,
        model_restore_journal: None,
    }
}
/// Provider convergence selects first/last session_meta and latest native settings only.
pub fn plan_provider_convergence(
    dbs: &[PathBuf],
    target: ProviderTarget,
    backup_directory: &Path,
) -> Result<ThreadWriteRepairPlan> {
    let mut plan = empty_plan(WriteRepairKind::Provider, backup_directory);
    for db in dbs {
        if fs::metadata(db).is_err() {
            continue;
        }
        let c = open_rw(db)?;
        let rows = threads::collect_provider_convergence_targets(&c).map_err(RepairError)?;
        plan.scanned += rows.len();
        let mut catalog_eligible = BTreeSet::new();
        for row in rows {
            let Some(path) = row.rollout_path.as_deref() else {
                return Err(RepairError(format!("Operation failed: ROUTER_HISTORY_REPAIR_REQUIRED: 任务 {} 的历史记录缺失，请先修复历史记录后重试",row.id)));
            };
            if threads::resolve_rollout(Path::new(path))?.is_none() {
                return Err(RepairError(format!("Operation failed: ROUTER_HISTORY_REPAIR_REQUIRED: 任务 {} 的历史记录缺失，请先修复历史记录后重试",row.id)));
            }
            let anchors = match consistency::read_rollout_provider_anchors(Path::new(path)) {
                Ok(Some(a)) => a,
                Ok(None) => {
                    return Err(RepairError(format!(
                        "Invalid data: session_meta missing in {path}"
                    )))
                }
                Err(consistency::RolloutReadFailure::Corrupt(e)) => return Err(RepairError(e)),
                Err(consistency::RolloutReadFailure::Other(e)) => return Err(RepairError(e)),
            };
            if [&anchors.first, &anchors.last, &anchors.native]
                .iter()
                .any(|p| {
                    p.as_deref()
                        .is_some_and(|p| !threads::is_router_or_openai_provider(p))
                })
            {
                plan.warnings
                    .push(format!("{}: externally owned history preserved", row.id));
                continue;
            }
            catalog_eligible.insert(row.id.clone());
            let (physical, original, logical) = logical_file(Path::new(path))?;
            let parsed = records(&logical);
            let (first, last, settings) = selected_records(&parsed, &row.id)?;
            let mut changes = BTreeMap::new();
            for index in [first, last] {
                let r = &parsed[index];
                if r.value
                    .pointer("/payload/model_provider")
                    .and_then(Value::as_str)
                    .is_some_and(|s| s != target.id())
                {
                    let mut v = r.value.clone();
                    payload_mut(&mut v)?.insert("model_provider".into(), json!(target.id()));
                    let fitted = fit_record(
                        &logical[r.start..r.content_end],
                        serde_json::to_vec(&v).map_err(|e| RepairError(e.to_string()))?,
                    )?;
                    changes.insert(index, fitted);
                }
            }
            if let Some(index) = settings {
                let r = &parsed[index];
                if r.value
                    .pointer("/payload/thread_settings/model_provider_id")
                    .and_then(Value::as_str)
                    != Some(target.id())
                {
                    let mut v = r.value.clone();
                    v["payload"]["thread_settings"]["model_provider_id"] = json!(target.id());
                    let fitted = fit_record(
                        &logical[r.start..r.content_end],
                        serde_json::to_vec(&v).map_err(|e| RepairError(e.to_string()))?,
                    )?;
                    changes.insert(index, fitted);
                }
            }
            if !changes.is_empty() {
                let replacement =
                    encode_logical(&physical, rewrite_records(&logical, &changes, &parsed))?;
                append_file(
                    &mut plan,
                    RolloutFileChange {
                        requested_path: path.into(),
                        physical_path: physical,
                        original,
                        replacement,
                        thread_ids: vec![row.id.clone()],
                        databases: vec![db.clone()],
                    },
                )?;
            }
            if row.provider != target.id() {
                plan.db_changes.push(ThreadDbChange {
                    db: db.clone(),
                    id: row.id,
                    expected_provider: row.provider,
                    expected_model: row.model,
                    expected_rollout: row.rollout_path,
                    replacement: target.id().into(),
                });
            }
        }
        // Codex desktop keeps its own thread catalog; leaving stale provider ids here
        // makes the desktop report a provider mismatch even after threads/rollout
        // converge (jlcodes99/cockpit-tools session-visibility repair, SQLite pass).
        if let Some(change) = catalog_provider_change(&c, db, target.id(), &catalog_eligible)? {
            plan.catalog_changes.push(change);
        }
    }
    Ok(plan)
}
/// Mirror the desktop's local_thread_catalog layout check. Returns None when the
/// catalog or its hosts table is absent so older databases stay untouched.
fn catalog_provider_change(
    c: &Connection,
    db: &Path,
    target: &str,
    ids: &BTreeSet<String>,
) -> Result<Option<ThreadCatalogChange>> {
    if ids.is_empty() {
        return Ok(None);
    }
    let exists = c
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('local_thread_catalog','local_thread_catalog_hosts')",
            [],
            |r| r.get::<_, i64>(0),
        )
        .map_err(|e| RepairError(e.to_string()))?;
    if exists != 2 {
        return Ok(None);
    }
    let columns: BTreeSet<String> = {
        let mut st = c
            .prepare("PRAGMA table_info(local_thread_catalog)")
            .map_err(|e| RepairError(e.to_string()))?;
        let rows = st
            .query_map([], |r| r.get::<_, String>(1))
            .map_err(|e| RepairError(e.to_string()))?;
        rows.collect::<std::result::Result<_, _>>().map_err(|e| RepairError(e.to_string()))?
    };
    if !columns.contains("host_id")
        || !columns.contains("thread_id")
        || !columns.contains("model_provider")
    {
        return Ok(None);
    }
    let host_id = local_catalog_host_id(c)?;
    let Some(host_id) = host_id else { return Ok(None) };
    let mut entries = vec![];
    for id in ids {
        let current: Option<Option<String>> = c
            .query_row(
                "SELECT model_provider FROM local_thread_catalog WHERE host_id=?1 AND thread_id=?2",
                rusqlite::params![host_id, id],
                |r| r.get(0),
            )
            .optional()
            .map_err(|e| RepairError(e.to_string()))?;
        if let Some(provider) = current.flatten() {
            if provider != target {
                entries.push(ThreadCatalogEntry {
                    thread_id: id.clone(),
                    expected_provider: provider,
                });
            }
        }
    }
    if entries.is_empty() {
        return Ok(None);
    }
    let has_missing_candidate = columns.contains("missing_candidate");
    Ok(Some(ThreadCatalogChange {
        db: db.to_path_buf(),
        host_id,
        target: target.to_owned(),
        has_missing_candidate,
        entries,
    }))
}
/// Same host selection as the desktop catalog itself: the first local host row,
/// falling back to the literal "local" host id used by fresh profiles.
fn local_catalog_host_id(c: &Connection) -> Result<Option<String>> {
    let query = "SELECT host_id FROM local_thread_catalog_hosts WHERE LOWER(COALESCE(host_kind, '')) = 'local' ORDER BY host_id LIMIT 1";
    match c.query_row(query, [], |r| r.get::<_, String>(0)) {
        Ok(host) => Ok(Some(host)),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            match c.query_row(
                "SELECT host_id FROM local_thread_catalog_hosts WHERE host_id = 'local' LIMIT 1",
                [],
                |r| r.get::<_, String>(0),
            ) {
                Ok(host) => Ok(Some(host)),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(RepairError(format!("Operation failed: read local catalog host: {e}"))),
            }
        }
        Err(e) => Err(RepairError(format!(
            "Operation failed: read local catalog hosts: {e}"
        ))),
    }
}
pub fn plan_model_catalog_convergence(
    dbs: &[PathBuf],
    catalog_directory: &Path,
    backup_directory: &Path,
) -> Result<ThreadWriteRepairPlan> {
    plan_model_catalog_convergence_with_journal_directory(
        dbs,
        catalog_directory,
        catalog_directory,
        backup_directory,
    )
}
/// Explicit journal ownership: catalog may be a private alias; journal stays in the relay root.
pub fn plan_model_catalog_convergence_with_journal_directory(
    dbs: &[PathBuf],
    catalog_directory: &Path,
    journal_directory: &Path,
    backup_directory: &Path,
) -> Result<ThreadWriteRepairPlan> {
    let index =
        consistency::load_router_catalog_model_index(catalog_directory).map_err(RepairError)?;
    let mut plan = empty_plan(WriteRepairKind::Model, backup_directory);
    plan.no_account = index.no_account;
    plan.catalog_directory = Some(catalog_directory.into());
    for db in dbs {
        if fs::metadata(db).is_err() {
            continue;
        }
        let c = open_rw(db)?;
        let rows = consistency::collect_router_model_targets(&c).map_err(RepairError)?;
        plan.scanned += rows.len();
        for row in rows {
            let replacement = match index.disposition(row.model.as_deref()) {
                ModelDisposition::Catalog | ModelDisposition::OfficialPassthrough => continue,
                ModelDisposition::Replace(m) => m,
                ModelDisposition::NoReplacement => {
                    plan.warnings
                        .push(format!("{}: no catalog fallback model", row.id));
                    continue;
                }
            };
            if let Some(path) = row.rollout_path.as_deref() {
                if threads::resolve_rollout(Path::new(path))?.is_some() {
                    let anchors = match consistency::read_rollout_provider_anchors(Path::new(path))
                    {
                        Ok(Some(a)) => a,
                        Ok(None) | Err(consistency::RolloutReadFailure::Corrupt(_)) => {
                            plan.warnings.push(format!(
                                "{}: unreadable or externally owned rollout",
                                row.id
                            ));
                            continue;
                        }
                        Err(consistency::RolloutReadFailure::Other(e)) => {
                            plan.warnings.push(format!("{}: {e}", row.id));
                            continue;
                        }
                    };
                    if [&anchors.first, &anchors.last, &anchors.native]
                        .iter()
                        .any(|p| {
                            p.as_deref()
                                .is_some_and(|p| !threads::is_router_or_openai_provider(p))
                        })
                    {
                        plan.warnings
                            .push(format!("{}: externally owned history preserved", row.id));
                        continue;
                    }
                    let (physical, original, logical) = logical_file(Path::new(path))?;
                    let parsed = records(&logical);
                    let (_, last, _) = match selected_records(&parsed, &row.id) {
                        Ok(s) => s,
                        Err(e) => {
                            plan.warnings.push(e.to_string());
                            continue;
                        }
                    };
                    let r = &parsed[last];
                    let mut v = r.value.clone();
                    payload_mut(&mut v)?.insert("model".into(), json!(replacement));
                    match fit_record(
                        &logical[r.start..r.content_end],
                        serde_json::to_vec(&v).map_err(|e| RepairError(e.to_string()))?,
                    ) {
                        Ok(record) => {
                            let mut changes = BTreeMap::new();
                            changes.insert(last, record);
                            let output = encode_logical(
                                &physical,
                                rewrite_records(&logical, &changes, &parsed),
                            )?;
                            append_file(
                                &mut plan,
                                RolloutFileChange {
                                    requested_path: path.into(),
                                    physical_path: physical,
                                    original,
                                    replacement: output,
                                    thread_ids: vec![row.id.clone()],
                                    databases: vec![db.clone()],
                                },
                            )?
                        }
                        Err(e) => plan
                            .warnings
                            .push(format!("{}: patch rollout failed: {e}", row.id)),
                    }
                }
            }
            plan.db_changes.push(ThreadDbChange {
                db: db.clone(),
                id: row.id,
                expected_provider: "aimai1".into(),
                expected_model: row.model,
                expected_rollout: row.rollout_path,
                replacement,
            });
        }
    }
    if plan.no_account && !plan.db_changes.is_empty() {
        plan.model_restore_journal =
            Some(plan_restore_journal(journal_directory, &plan.db_changes)?);
    }
    Ok(plan)
}
fn backup_database(c: &Connection, db: &Path, root: &Path) -> Result<PathBuf> {
    fs::create_dir_all(root)?;
    let dest = root.join(format!(
        "{}-threads-db-{}",
        token(),
        db.file_name().unwrap_or_default().to_string_lossy()
    ));
    // VACUUM INTO captures committed WAL contents, unlike copying an open SQLite main file.
    c.execute("VACUUM INTO ?1", [dest.to_string_lossy().as_ref()])?;
    Ok(dest)
}
fn validate_db_change(
    c: &Connection,
    change: &ThreadDbChange,
    kind: WriteRepairKind,
) -> Result<()> {
    if !row_unchanged(
        c,
        &change.id,
        &change.expected_provider,
        change.expected_rollout.as_deref(),
    )? {
        return Err(RepairError(format!(
            "thread changed since repair preview: {}",
            change.id
        )));
    }
    if kind == WriteRepairKind::Model {
        let actual = c
            .query_row("SELECT model FROM threads WHERE id=?1", [&change.id], |r| {
                r.get::<_, Option<String>>(0)
            })?
            .filter(|s| !s.trim().is_empty());
        if actual != change.expected_model {
            return Err(RepairError(format!(
                "thread model changed since repair preview: {}",
                change.id
            )));
        }
    }
    Ok(())
}
fn apply_rollout_file(
    file: &RolloutFileChange,
    root: &Path,
    summary: &mut RepairSummary,
) -> Result<bool> {
    let physical = threads::resolve_rollout(&file.requested_path)?.ok_or_else(|| {
        RepairError("rollout disappeared after repair preview; history preserved".into())
    })?;
    if physical != file.physical_path {
        return Err(RepairError(
            "rollout representation changed after repair preview; history preserved".into(),
        ));
    }
    let bytes = fs::read(&physical)?;
    if bytes == file.replacement {
        return Ok(false);
    }
    if bytes != file.original {
        return Err(RepairError(
            "rollout changed after repair preview; history preserved".into(),
        ));
    }
    let modified = fs::metadata(&physical)?.modified().ok();
    summary
        .backups
        .push(backup_file(&physical, root, "rollout")?);
    write_atomic(&physical, &file.replacement)?;
    if let Some(m) = modified {
        if let Ok(f) = fs::OpenOptions::new().write(true).open(&physical) {
            let _ = f.set_times(fs::FileTimes::new().set_modified(m));
        }
    }
    summary.rollout_updated += 1;
    Ok(true)
}
fn rollback_rollouts(files: &[&RolloutFileChange], cause: RepairError) -> RepairError {
    let mut failures = vec![];
    for file in files.iter().rev() {
        let result = (|| -> Result<()> {
            if fs::read(&file.physical_path)? != file.replacement {
                return Err(RepairError(
                    "rollout changed during rollback; history preserved".into(),
                ));
            }
            let modified = fs::metadata(&file.physical_path)?.modified().ok();
            write_atomic(&file.physical_path, &file.original)?;
            if let Some(m) = modified {
                if let Ok(f) = fs::OpenOptions::new().write(true).open(&file.physical_path) {
                    let _ = f.set_times(fs::FileTimes::new().set_modified(m));
                }
            }
            Ok(())
        })();
        if let Err(e) = result {
            failures.push(format!("{}: {e}", file.physical_path.display()));
        }
    }
    if failures.is_empty() {
        cause
    } else {
        RepairError(format!(
            "{cause}; rollout rollback failed: {}",
            failures.join("; ")
        ))
    }
}
/// Execute a reviewed plan against its original DB values and original physical bytes.
/// Provider failures roll back that database's history patches. Model rollout failures are
/// warnings and the corresponding DB model update still commits, matching @0x1008d2e0c.
pub fn apply_thread_write_repair(plan: &ThreadWriteRepairPlan) -> Result<RepairSummary> {
    let mut summary = RepairSummary {
        scanned: plan.scanned,
        warnings: plan.warnings.clone(),
        ..Default::default()
    };
    for change in &plan.db_changes {
        validate_db_change(&open_rw(&change.db)?, change, plan.kind)?;
    }
    if plan.kind == WriteRepairKind::Provider {
        for change in &plan.catalog_changes {
            let c = open_rw(&change.db)?;
            for entry in &change.entries {
                let current: Option<String> = c
                    .query_row(
                        "SELECT model_provider FROM local_thread_catalog WHERE host_id=?1 AND thread_id=?2",
                        rusqlite::params![change.host_id, entry.thread_id],
                        |r| r.get(0),
                    )
                    .optional()
                    .map_err(|e| RepairError(e.to_string()))?;
                if current.as_deref() != Some(entry.expected_provider.as_str()) {
                    return Err(RepairError(format!(
                        "desktop catalog changed since repair preview: {}",
                        entry.thread_id
                    )));
                }
            }
        }
    }
    if let Some(journal) = &plan.model_restore_journal {
        let current = read_optional_file(&journal.path)?;
        if current != journal.original {
            return Err(RepairError(
                "no-account model restore journal changed after repair preview".into(),
            ));
        }
        write_private_report(&journal.path, &journal.replacement)?;
    }
    let databases: BTreeSet<_> = plan
        .db_changes
        .iter()
        .map(|c| c.db.clone())
        .chain(plan.files.iter().flat_map(|f| f.databases.iter().cloned()))
        .chain(plan.catalog_changes.iter().map(|c| c.db.clone()))
        .collect();
    for db in databases {
        let c = open_rw(&db)?;
        let changes: Vec<_> = plan.db_changes.iter().filter(|r| r.db == db).collect();
        let files: Vec<_> = plan
            .files
            .iter()
            .filter(|r| r.databases.contains(&db))
            .collect();
        for change in &changes {
            validate_db_change(&c, change, plan.kind)?;
        }
        if !changes.is_empty() {
            summary
                .backups
                .push(backup_database(&c, &db, &plan.backup_directory)?)
        }
        let mut applied = vec![];
        let mut catalog_count = 0usize;
        if plan.kind == WriteRepairKind::Provider {
            for file in &files {
                match apply_rollout_file(file, &plan.backup_directory, &mut summary) {
                    Ok(true) => applied.push(*file),
                    Ok(false) => (),
                    Err(e) => return Err(rollback_rollouts(&applied, e)),
                }
            }
        }
        let transaction = (|| -> Result<usize> {
            c.execute_batch("BEGIN IMMEDIATE")?;
            let mut count = 0;
            for change in &changes {
                validate_db_change(&c, change, plan.kind)?;
                count += match plan.kind {
                    WriteRepairKind::Provider => c.execute(
                        "UPDATE threads SET model_provider = ?2 WHERE id = ?1",
                        rusqlite::params![change.id, change.replacement],
                    )?,
                    WriteRepairKind::Model => c.execute(
                        "UPDATE threads SET model = ?2 WHERE id = ?1",
                        rusqlite::params![change.id, change.replacement],
                    )?,
                };
            }
            if plan.kind == WriteRepairKind::Provider {
                for cat in plan.catalog_changes.iter().filter(|c| c.db == db) {
                    for entry in &cat.entries {
                        let sql = if cat.has_missing_candidate {
                            "UPDATE local_thread_catalog SET model_provider=?2, missing_candidate=0 WHERE thread_id=?1 AND host_id=?3 AND model_provider=?4"
                        } else {
                            "UPDATE local_thread_catalog SET model_provider=?2 WHERE thread_id=?1 AND host_id=?3 AND model_provider=?4"
                        };
                        let n = c
                            .execute(
                                sql,
                                rusqlite::params![
                                    entry.thread_id,
                                    cat.target,
                                    cat.host_id,
                                    entry.expected_provider
                                ],
                            )
                            .map_err(|e| RepairError(e.to_string()))?;
                        if n == 0 {
                            return Err(RepairError(format!(
                                "desktop catalog changed since repair preview: {}",
                                entry.thread_id
                            )));
                        }
                        catalog_count += n;
                        count += n;
                    }
                }
            }
            if plan.kind == WriteRepairKind::Model {
                for file in &files {
                    match apply_rollout_file(file, &plan.backup_directory, &mut summary) {
                        Ok(true) => applied.push(*file),
                        Ok(false) => (),
                        Err(e) => summary.warnings.push(format!(
                            "{}: patch rollout failed: {e}",
                            file.physical_path.display()
                        )),
                    }
                }
            }
            c.execute_batch("COMMIT")?;
            Ok(count)
        })();
        match transaction {
            Ok(n) => {
                summary.db_updated += n;
                summary.catalog_updated += catalog_count;
            }
            Err(e) => {
                if !c.is_autocommit() {
                    let _ = c.execute_batch("ROLLBACK");
                }
                return Err(rollback_rollouts(&applied, e));
            }
        }
        if let Err(e) = c.execute_batch("PRAGMA wal_checkpoint(TRUNCATE)") {
            summary
                .warnings
                .push(format!("checkpoint after committed thread repair: {e}"));
        }
    }
    Ok(summary)
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalFileChange {
    pub path: PathBuf,
    pub original: Option<Vec<u8>>,
    pub replacement: Vec<u8>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRestoreEntry {
    pub thread_id: String,
    pub original_model: Option<String>,
    pub borrowed_model: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRestoreJournal {
    pub version: u32,
    pub entries: Vec<ModelRestoreEntry>,
}
const JOURNAL_LIMIT: u64 = 8 * 1024 * 1024;
fn read_optional_file(path: &Path) -> Result<Option<Vec<u8>>> {
    match fs::symlink_metadata(path) {
        Ok(m) => {
            if !m.file_type().is_file() {
                return Err(RepairError(
                    "Operation failed: invalid no-account model restore journal path type".into(),
                ));
            }
            if m.len() > JOURNAL_LIMIT {
                return Err(RepairError(
                    "Operation failed: no-account model restore journal exceeds size limit".into(),
                ));
            }
            use std::io::Read;
            let mut bytes = vec![];
            fs::File::open(path)?
                .take(JOURNAL_LIMIT + 1)
                .read_to_end(&mut bytes)?;
            if bytes.len() as u64 > JOURNAL_LIMIT {
                return Err(RepairError(
                    "Operation failed: no-account model restore journal exceeds size limit".into(),
                ));
            }
            Ok(Some(bytes))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.into()),
    }
}
fn slot_slug(s: &str) -> bool {
    matches!(
        s,
        "gpt-5.6-sol" | "gpt-5.6-terra" | "gpt-5.6-luna" | "gpt-5.5" | "gpt-5.4"
    )
}
fn relay_slug(s: &str) -> bool {
    s.split_once("::").is_some_and(|(provider, model)| {
        provider.starts_with("aimami_relay_")
            && crate::catalog::builder::validate_relay_provider_id(provider).is_ok()
            && !model.trim().is_empty()
    })
}
/// Native journal v1 accepts canonical slots; v2 additionally accepts a managed qualified model.
pub fn load_model_restore_journal(directory: &Path) -> Result<ModelRestoreJournal> {
    let path = directory.join("router_no_account_model_restore.json");
    let Some(bytes) = read_optional_file(&path)? else {
        return Ok(ModelRestoreJournal {
            version: 2,
            entries: vec![],
        });
    };
    let journal: ModelRestoreJournal = serde_json::from_slice(&bytes).map_err(|e| {
        RepairError(format!(
            "Operation failed: decode no-account model restore journal {}: {e}",
            path.display()
        ))
    })?;
    if journal.version != 1 && journal.version != 2 {
        return Err(RepairError(format!(
            "Operation failed: unsupported no-account model restore journal version: {}",
            journal.version
        )));
    }
    let mut ids = BTreeSet::new();
    if journal.entries.iter().any(|e| {
        e.thread_id.trim().is_empty()
            || (!slot_slug(&e.borrowed_model)
                && !(journal.version == 2 && relay_slug(&e.borrowed_model)))
            || !ids.insert(e.thread_id.clone())
    }) {
        return Err(RepairError(
            "Operation failed: invalid no-account model restore journal contents".into(),
        ));
    }
    Ok(journal)
}
fn plan_restore_journal(directory: &Path, changes: &[ThreadDbChange]) -> Result<JournalFileChange> {
    let path = directory.join("router_no_account_model_restore.json");
    let original = read_optional_file(&path)?;
    let mut journal = load_model_restore_journal(directory)?;
    journal.version = 2;
    for change in changes {
        if let Some(entry) = journal
            .entries
            .iter_mut()
            .find(|e| e.thread_id == change.id)
        {
            if change.expected_model.as_deref() != Some(&entry.borrowed_model) {
                entry.original_model = change.expected_model.clone();
            }
            entry.borrowed_model = change.replacement.clone();
        } else {
            journal.entries.push(ModelRestoreEntry {
                thread_id: change.id.clone(),
                original_model: change.expected_model.clone(),
                borrowed_model: change.replacement.clone(),
            });
        }
    }
    journal
        .entries
        .sort_by(|a, b| a.thread_id.cmp(&b.thread_id));
    let replacement = serde_json::to_vec_pretty(&journal).map_err(|e| {
        RepairError(format!(
            "Operation failed: encode no-account model restore journal: {e}"
        ))
    })?;
    if replacement.len() as u64 > JOURNAL_LIMIT {
        return Err(RepairError(
            "Operation failed: no-account model restore journal exceeds write limit".into(),
        ));
    }
    Ok(JournalFileChange {
        path,
        original,
        replacement,
    })
}
fn write_private_report(path: &Path, bytes: &[u8]) -> Result<()> {
    use std::io::Write;
    let parent = path
        .parent()
        .ok_or_else(|| RepairError("report has no parent".into()))?;
    fs::create_dir_all(parent)?;
    let tmp = parent.join(format!(".journal-{}", token()));
    let result = (|| -> Result<()> {
        let mut opts = fs::OpenOptions::new();
        opts.create_new(true).write(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            opts.mode(0o600);
        }
        let mut f = opts.open(&tmp)?;
        f.write_all(bytes)?;
        f.sync_all()?;
        fs::rename(&tmp, path)?;
        fs::File::open(parent)?.sync_all()?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(tmp);
    }
    result
}
/// @0x1008db528. Only legacy tool-search/custom-tool IDs are candidates.
/// Existing canonical IDs, malformed IDs and ordinary function-call IDs are untouched.
pub fn canonical_response_item_id(item_type: &str, id: &str) -> Option<String> {
    if item_type == "tool_search_call"
        && id.len() == 15
        && id.starts_with("ts_")
        && id.as_bytes()[3..]
            .iter()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(b))
    {
        return Some(format!("tsc_{}", &id[3..]));
    }
    if item_type == "custom_tool_call"
        && (id.starts_with("ct_") || id.starts_with("fc_"))
        && id.len() > 3
        && id.as_bytes()[3..]
            .iter()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-'))
    {
        return Some(format!("ctc_{}", &id[3..]));
    }
    None
}
pub(super) fn candidate_response_record(record: &[u8]) -> bool {
    fn contains(s: &[u8], part: &[u8]) -> bool {
        s.windows(part.len()).any(|s| s == part)
    }
    (contains(record, b"tool_search_call") && contains(record, b"ts_"))
        || (contains(record, b"custom_tool_call")
            && (contains(record, b"ct_") || contains(record, b"fc_")))
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseIdReplacement {
    pub item_type: String,
    pub old_id: String,
    pub new_id: String,
    pub line: usize,
}
/// Preserve every byte except the unique JSON string literal naming payload.id.
/// @0x1008dc290 intentionally rejects duplicate occurrences instead of serializing the record.
pub fn rewrite_response_item_record(
    record: &[u8],
    path: &Path,
    line: usize,
) -> Result<Option<(Vec<u8>, ResponseIdReplacement)>> {
    let (body, ending) = if let Some(body) = record.strip_suffix(b"\r\n") {
        (body, &b"\r\n"[..])
    } else if let Some(body) = record.strip_suffix(b"\n") {
        (body, &b"\n"[..])
    } else {
        (record, &b""[..])
    };
    if !candidate_response_record(body) {
        return Ok(None);
    }
    let text = std::str::from_utf8(body).map_err(|e| {
        RepairError(format!(
            "Invalid data: candidate rollout record is not UTF-8 in {} line {line}: {e}",
            path.display()
        ))
    })?;
    let v: Value = serde_json::from_str(text).map_err(|e| {
        RepairError(format!(
            "Invalid data: parse candidate rollout record line {line} in {}: {e}",
            path.display()
        ))
    })?;
    if v.get("type").and_then(Value::as_str) != Some("response_item") {
        return Ok(None);
    }
    let (Some(item_type), Some(id)) = (
        v.pointer("/payload/type").and_then(Value::as_str),
        v.pointer("/payload/id").and_then(Value::as_str),
    ) else {
        return Ok(None);
    };
    let Some(new_id) = canonical_response_item_id(item_type, id) else {
        return Ok(None);
    };
    let old_literal =
        serde_json::to_string(id).map_err(|e| RepairError(format!("JSON error: {e}")))?;
    let mut matches = text.match_indices(&old_literal);
    let Some((offset, _)) = matches.next() else {
        return Err(RepairError(format!(
            "Invalid data: cannot locate validated response item ID in {} line {line}",
            path.display()
        )));
    };
    if matches.next().is_some() {
        return Err(RepairError(format!(
            "Invalid data: ambiguous response item ID occurrence in {} line {line}",
            path.display()
        )));
    }
    let mut replacement = text.to_owned();
    replacement.replace_range(
        offset..offset + old_literal.len(),
        &serde_json::to_string(&new_id).map_err(|e| RepairError(format!("JSON error: {e}")))?,
    );
    let mut output = fit_record(body, replacement.into_bytes())?;
    output.extend_from_slice(ending);
    Ok(Some((
        output,
        ResponseIdReplacement {
            item_type: item_type.into(),
            old_id: id.into(),
            new_id,
            line,
        },
    )))
}
/// The native diagnostic parses only raw candidate records; unrelated malformed JSON is preserved.
pub fn count_invalid_response_item_ids(path: &Path) -> Result<usize> {
    if let Some(observed) = super::observation::read(path) {
        let observed = observed.map_err(RepairError)?;
        observed.require_complete().map_err(RepairError)?;
        return observed.response_ids.clone().map_err(RepairError);
    }
    let (_, _, bytes) = logical_file(path)?;
    count_response_ids(&bytes, path)
}
fn count_response_ids(bytes: &[u8], path: &Path) -> Result<usize> {
    let mut count = 0;
    for (line, record) in bytes.split_inclusive(|b| *b == b'\n').enumerate() {
        if !candidate_response_record(record) {
            continue;
        }
        let v: Value = serde_json::from_slice(record).map_err(|e| {
            RepairError(format!(
                "Invalid data: parse candidate rollout record line {} in {}: {e}",
                line + 1,
                path.display()
            ))
        })?;
        if v.get("type").and_then(Value::as_str) == Some("response_item") {
            if let (Some(kind), Some(id)) = (
                v.pointer("/payload/type").and_then(Value::as_str),
                v.pointer("/payload/id").and_then(Value::as_str),
            ) {
                if canonical_response_item_id(kind, id).is_some() {
                    count += 1;
                }
            }
        }
    }
    Ok(count)
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseIdFileChange {
    pub file: RolloutFileChange,
    pub replacements: Vec<ResponseIdReplacement>,
    pub source_modified: Option<SystemTime>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseIdRepairPlan {
    pub files: Vec<ResponseIdFileChange>,
    pub backup_directory: PathBuf,
    pub scanned: usize,
}
fn record_has_positions(v: &Value) -> bool {
    v.get("ordinal").is_some()
        || v.pointer("/payload/history_mode").and_then(Value::as_str) == Some("paginated")
        || v.pointer("/payload/history_base")
            .is_some_and(|v| !v.is_null())
}
fn response_file_plan(
    path: &Path,
    thread_id: &str,
    db: &Path,
) -> Result<Option<ResponseIdFileChange>> {
    if let Some(observed) = super::observation::read(path) {
        let observed = observed.map_err(RepairError)?;
        observed.require_complete().map_err(RepairError)?;
        if observed.response_ids.clone().map_err(RepairError)? == 0 {
            return Ok(None);
        }
    }
    let (physical, original, logical) = logical_file(path)?;
    if count_response_ids(&logical, path)? == 0 {
        return Ok(None);
    }
    // @0x1008afc44: the global offset constraint comes from the FIRST session_meta only.
    let has_positions = records(&logical)
        .iter()
        .find(|r| meta(&r.value))
        .is_some_and(|r| record_has_positions(&r.value));
    let mut output = vec![];
    let mut replacements = vec![];
    for (line, record) in logical.split_inclusive(|b| *b == b'\n').enumerate() {
        if let Some((rewritten, replacement)) =
            rewrite_response_item_record(record, path, line + 1)?
        {
            if has_positions && rewritten.len() != record.len() {
                return Err(RepairError("Operation failed: ROUTER_HISTORY_LAYOUT_PROTECTED: call-item ID repair would change native history offsets; history preserved".into()));
            }
            output.extend(rewritten);
            replacements.push(replacement);
        } else {
            output.extend_from_slice(record)
        }
    }
    let replacement = encode_logical(&physical, output)?;
    let source_modified = fs::metadata(&physical)?.modified().ok();
    Ok(Some(ResponseIdFileChange {
        file: RolloutFileChange {
            requested_path: path.into(),
            physical_path: physical,
            original,
            replacement,
            thread_ids: vec![thread_id.into()],
            databases: vec![db.into()],
        },
        replacements,
        source_modified,
    }))
}
/// Active main threads of every provider participate, with exact recorded-path deduplication.
/// Errors in the preflight prevent publication, as in the native diagnostic wrapper.
pub fn plan_response_item_id_repair(
    dbs: &[PathBuf],
    backup_directory: &Path,
) -> Result<ResponseIdRepairPlan> {
    let rows = threads::collect_diagnostic_main_threads(dbs).map_err(RepairError)?;
    let mut seen = BTreeSet::new();
    let mut plan = ResponseIdRepairPlan {
        backup_directory: backup_directory.into(),
        ..Default::default()
    };
    for row in rows {
        let Some(path) = row.rollout_path else {
            return Err(RepairError(format!(
                "Operation failed: rollout path missing for {}",
                row.id
            )));
        };
        if !seen.insert(path.clone()) {
            continue;
        }
        plan.scanned += 1;
        if let Some(file) = response_file_plan(Path::new(&path), &row.id, &row.db)? {
            plan.files.push(file);
        }
    }
    Ok(plan)
}
fn validate_response_snapshot(change: &ResponseIdFileChange) -> Result<()> {
    let file = &change.file;
    let current = threads::resolve_rollout(&file.requested_path)?
        .ok_or_else(|| RepairError("rollout is missing".into()))?;
    if current != file.physical_path
        || fs::read(&current)? != file.original
        || fs::metadata(&current)?.modified().ok() != change.source_modified
    {
        return Err(RepairError(format!(
            "Operation failed: rollout changed before repair; retry required: {}",
            current.display()
        )));
    }
    Ok(())
}
/// Publish a private, native-shaped JSON backup BEFORE changing the rollout, then recount IDs.
/// This operation does not change SQLite rows. Individual failures do not prevent other files
/// from being attempted; the returned error explicitly reports partial publication.
pub fn apply_response_item_id_repair(plan: &ResponseIdRepairPlan) -> Result<RepairSummary> {
    use sha2::{Digest, Sha256};
    let mut summary = RepairSummary {
        scanned: plan.scanned,
        ..Default::default()
    };
    let mut errors = vec![];
    for change in &plan.files {
        let result = (|| -> Result<()> {
            validate_response_snapshot(change)?;
            let file = &change.file;
            let digest = format!("{:x}", Sha256::digest(&file.original));
            let replacements: Vec<_> = change
                .replacements
                .iter()
                .map(|r| json!({"line":r.line,"beforeId":r.old_id,"afterId":r.new_id}))
                .collect();
            let backup = plan.backup_directory.join(format!(
                "{}-{}.response-item-id-repair.json",
                file.physical_path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy(),
                token()
            ));
            let bytes=serde_json::to_vec_pretty(&json!({"schemaVersion":1,"sourcePath":file.physical_path.to_string_lossy(),"sourceLength":file.original.len(),"sourceSha256":digest,"replacements":replacements})).map_err(|e|RepairError(format!("JSON error: {e}")))?;
            write_private_report(&backup, &bytes)?;
            if let Err(e) = validate_response_snapshot(change) {
                let _ = fs::remove_file(&backup);
                return Err(e);
            }
            if let Err(e) = write_atomic(&file.physical_path, &file.replacement) {
                let _ = fs::remove_file(&backup);
                return Err(e);
            }
            if file.physical_path.to_string_lossy().ends_with(".jsonl.zst") {
                if let Some(m) = change.source_modified {
                    if let Ok(f) = fs::OpenOptions::new().write(true).open(&file.physical_path) {
                        let _ = f.set_times(fs::FileTimes::new().set_modified(m));
                    }
                }
            }
            summary.backups.push(backup);
            summary.rollout_updated += 1;
            let remaining = count_invalid_response_item_ids(&file.physical_path)?;
            if remaining != 0 {
                return Err(RepairError(format!("Operation failed: response item ID repair verification failed: {remaining} remaining in {}",file.physical_path.display())));
            }
            Ok(())
        })();
        if let Err(e) = result {
            errors.push(e.to_string());
        }
    }
    if !errors.is_empty() {
        return Err(RepairError(format!(
            "response item ID repair partially applied: updated={}, backups={:?}; {}",
            summary.rollout_updated,
            summary.backups,
            errors.join("; ")
        )));
    }
    Ok(summary)
}
/// @0x1008ba02c: modern aimami_relay_* IDs are intentionally outside this legacy predicate.
pub fn is_legacy_or_miscased_provider(provider: &str) -> bool {
    matches!(provider, "aimai1" | "aimami_codex_router")
        || (provider.eq_ignore_ascii_case("openai") && provider != "openai")
}
const LEGACY_COUNT_SQL:&str="SELECT COUNT(*) FROM threads WHERE model_provider = ?1 OR model_provider = ?2 OR (model_provider LIKE 'openai' AND model_provider <> ?3)";
const LEGACY_UPDATE_SQL:&str="UPDATE threads SET model_provider = ?1 WHERE model_provider = ?2 OR model_provider = ?3 OR (model_provider LIKE 'openai' AND model_provider <> ?1)";
const LEGACY_PATHS_SQL:&str="SELECT DISTINCT rollout_path FROM threads WHERE rollout_path IS NOT NULL AND rollout_path <> '' AND COALESCE(archived, 0) = 0";
fn legacy_rows(c: &Connection) -> Result<Vec<(String, String)>> {
    let mut stmt=c.prepare("SELECT id,model_provider FROM threads WHERE model_provider = ?1 OR model_provider = ?2 OR (model_provider LIKE 'openai' AND model_provider <> ?3) ORDER BY id")?;
    let rows = stmt
        .query_map(
            rusqlite::params!["aimai1", "aimami_codex_router", "openai"],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(rows)
}
fn legacy_paths(c: &Connection) -> Result<Vec<String>> {
    let mut stmt = c.prepare(LEGACY_PATHS_SQL)?;
    let rows = stmt
        .query_map([], |r| r.get::<_, String>(0))?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(rows)
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyDatabaseCleanup {
    pub db: PathBuf,
    pub expected_legacy_rows: Vec<(String, String)>,
    pub expected_rollout_paths: Vec<String>,
    pub files: Vec<RolloutFileChange>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyCleanupPlan {
    pub databases: Vec<LegacyDatabaseCleanup>,
    pub backup_directory: PathBuf,
    pub scanned: usize,
}
/// Native strict cleanup normalizes ALL matching DB rows, including archived/subagent rows,
/// while reading only active distinct rollout paths. History changes touch first/last metadata;
/// modern managed and external providers, native settings, model, and source remain intact.
pub fn plan_legacy_provider_cleanup(
    dbs: &[PathBuf],
    backup_directory: &Path,
) -> Result<LegacyCleanupPlan> {
    let mut plan = LegacyCleanupPlan {
        databases: vec![],
        backup_directory: backup_directory.into(),
        scanned: 0,
    };
    for db in dbs {
        if fs::metadata(db).is_err() {
            continue;
        }
        let c = open_rw(db)?;
        let expected_legacy_rows = legacy_rows(&c)?;
        let paths = legacy_paths(&c)?;
        let mut files = vec![];
        for path in &paths {
            plan.scanned += 1;
            if threads::resolve_rollout(Path::new(path))?.is_none() {
                continue;
            }
            let anchors = match consistency::read_rollout_provider_anchors(Path::new(path)) {
                Ok(Some(a)) => a,
                Ok(None) => continue,
                Err(e) => {
                    return Err(RepairError(match e {
                        consistency::RolloutReadFailure::Corrupt(s)
                        | consistency::RolloutReadFailure::Other(s) => s,
                    }))
                }
            };
            if ![&anchors.first, &anchors.last]
                .iter()
                .any(|p| p.as_deref().is_some_and(is_legacy_or_miscased_provider))
            {
                continue;
            }
            let (physical, original, logical) = logical_file(Path::new(path))?;
            let parsed = records(&logical);
            let Some(first) = parsed.iter().position(|r| meta(&r.value)) else {
                continue;
            };
            let id = parsed[first]
                .value
                .pointer("/payload/id")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    RepairError("Invalid data: first session_meta thread id missing".into())
                })?;
            let last = parsed
                .iter()
                .rposition(|r| {
                    meta(&r.value)
                        && r.value.pointer("/payload/id").and_then(Value::as_str) == Some(id)
                })
                .unwrap_or(first);
            let mut changes = BTreeMap::new();
            for index in [first, last] {
                let r = &parsed[index];
                if r.value
                    .pointer("/payload/model_provider")
                    .and_then(Value::as_str)
                    .is_some_and(is_legacy_or_miscased_provider)
                {
                    let mut v = r.value.clone();
                    payload_mut(&mut v)?.insert("model_provider".into(), json!("openai"));
                    changes.insert(
                        index,
                        fit_record(
                            &logical[r.start..r.content_end],
                            serde_json::to_vec(&v)
                                .map_err(|e| RepairError(format!("JSON error: {e}")))?,
                        )?,
                    );
                }
            }
            if !changes.is_empty() {
                let replacement =
                    encode_logical(&physical, rewrite_records(&logical, &changes, &parsed))?;
                files.push(RolloutFileChange {
                    requested_path: path.into(),
                    physical_path: physical,
                    original,
                    replacement,
                    thread_ids: vec![id.into()],
                    databases: vec![db.clone()],
                });
            }
        }
        plan.databases.push(LegacyDatabaseCleanup {
            db: db.clone(),
            expected_legacy_rows,
            expected_rollout_paths: paths,
            files,
        });
    }
    Ok(plan)
}
/// @0x1008c6b80: FULL checkpoint → rollout patches → one autocommit provider UPDATE →
/// TRUNCATE checkpoint → SQL count verification. UPDATE failure reverts history patches.
/// As in native, no broad transaction pretends to undo already committed earlier databases.
pub fn apply_legacy_provider_cleanup(plan: &LegacyCleanupPlan) -> Result<RepairSummary> {
    let mut summary = RepairSummary {
        scanned: plan.scanned,
        ..Default::default()
    };
    for database in &plan.databases {
        let c = open_rw(&database.db)?;
        if legacy_rows(&c)? != database.expected_legacy_rows
            || legacy_paths(&c)?.into_iter().collect::<BTreeSet<_>>()
                != database.expected_rollout_paths.iter().cloned().collect()
        {
            return Err(RepairError(
                "legacy provider candidates changed after repair preview".into(),
            ));
        }
        c.execute_batch("PRAGMA wal_checkpoint(FULL);")?;
        if !database.expected_legacy_rows.is_empty() {
            summary
                .backups
                .push(backup_database(&c, &database.db, &plan.backup_directory)?);
        }
        let mut applied = vec![];
        for file in &database.files {
            if apply_rollout_file(file, &plan.backup_directory, &mut summary)? {
                applied.push(file);
            }
        }
        let count = match c.execute(
            LEGACY_UPDATE_SQL,
            rusqlite::params!["openai", "aimai1", "aimami_codex_router"],
        ) {
            Ok(n) => n,
            Err(e) => {
                return Err(rollback_rollouts(
                    &applied,
                    RepairError(format!(
                        "Operation failed: sqlite provider cleanup failed: {e}"
                    )),
                ))
            }
        };
        summary.db_updated += count;
        let _ = c.execute_batch("PRAGMA wal_checkpoint(TRUNCATE);");
        let remaining: i64 = c.query_row(
            LEGACY_COUNT_SQL,
            rusqlite::params!["aimai1", "aimami_codex_router", "openai"],
            |r| r.get(0),
        )?;
        if remaining > 0 {
            return Err(rollback_rollouts(&applied,RepairError(format!("Operation failed: sqlite provider cleanup verification failed: {remaining} remaining"))));
        }
    }
    Ok(summary)
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LegacyThreadVisibility {
    pub legacy_rows: usize,
    pub active_official_main_threads: usize,
    pub errors: Vec<String>,
}
/// @0x1008bc6d4: legacy count deliberately includes archived/subagent rows; official count
/// uses active exact-openai rows and excludes a source containing the literal "subagent" token.
pub fn inspect_legacy_thread_visibility(dbs: &[PathBuf]) -> LegacyThreadVisibility {
    let mut result = LegacyThreadVisibility::default();
    for db in dbs {
        if fs::metadata(db).is_err() {
            continue;
        }
        let c = match Connection::open_with_flags(
            db,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        ) {
            Ok(c) => c,
            Err(e) => {
                result.errors.push(e.to_string());
                continue;
            }
        };
        let _ = c.busy_timeout(Duration::from_secs(3));
        match c.query_row(
            LEGACY_COUNT_SQL,
            rusqlite::params!["aimai1", "aimami_codex_router", "openai"],
            |r| r.get::<_, i64>(0),
        ) {
            Ok(n) => result.legacy_rows += n.max(0) as usize,
            Err(e) => result.errors.push(e.to_string()),
        }
        let scanned = (|| -> std::result::Result<(), rusqlite::Error> {
            let mut s=c.prepare("SELECT source FROM threads WHERE COALESCE(archived, 0) = 0 AND model_provider = ?1")?;
            let mut rows = s.query(["openai"])?;
            while let Some(row) = rows.next()? {
                match row.get::<_, Option<String>>(0) {
                    Ok(source) => {
                        if !source.unwrap_or_default().contains("\"subagent\"") {
                            result.active_official_main_threads += 1
                        }
                    }
                    Err(e) => {
                        result.active_official_main_threads += 1;
                        result.errors.push(e.to_string());
                    }
                }
            }
            Ok(())
        })();
        if let Err(e) = scanned {
            result.errors.push(e.to_string())
        }
    }
    result
}
pub const MINIMAL_SAFE_INSTRUCTIONS: &str =
    "Follow the user's instructions carefully. Respond helpfully and use tools when appropriate.";
pub const APPROVAL_JUDGE_PREFIX: &str = "You are judging one planned coding-agent action";
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionInstructionIssue {
    Missing,
    Poisoned,
    Healthy,
}
/// @0x100393c6c. Invalid JSON and non-session_meta values are outside this check.
pub fn session_meta_instruction_issue(record: &[u8]) -> SessionInstructionIssue {
    let Ok(v) = serde_json::from_slice::<Value>(record) else {
        return SessionInstructionIssue::Healthy;
    };
    session_meta_instruction_issue_value(&v)
}
pub(super) fn session_meta_instruction_issue_value(v: &Value) -> SessionInstructionIssue {
    if !meta(v) {
        return SessionInstructionIssue::Healthy;
    }
    let text = v
        .pointer("/payload/base_instructions/text")
        .and_then(Value::as_str)
        .unwrap_or("");
    if text.trim().is_empty() {
        SessionInstructionIssue::Missing
    } else if text.trim_start().starts_with(APPROVAL_JUDGE_PREFIX) {
        SessionInstructionIssue::Poisoned
    } else {
        SessionInstructionIssue::Healthy
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstructionReference {
    pub path: PathBuf,
    pub physical_path: PathBuf,
    pub line: usize,
    pub text: String,
}
fn reference_in_ordered_paths(paths: &[PathBuf]) -> Option<InstructionReference> {
    use std::io::{BufRead, BufReader};
    for path in paths.iter().take(50) {
        let Ok(Some(physical)) = threads::resolve_rollout(path) else {
            continue;
        };
        if let Some(observed) = super::observation::read(path) {
            if let Ok(observed) = observed {
                if let Some((line, text)) = &observed.reference {
                    return Some(InstructionReference {
                        path: path.clone(),
                        physical_path: physical,
                        line: *line,
                        text: text.clone(),
                    });
                }
            }
            continue;
        }
        let Ok(file) = fs::File::open(&physical) else {
            continue;
        };
        let mut reader: Box<dyn BufRead> = if physical.to_string_lossy().ends_with(".jsonl.zst") {
            let Ok(decoder) = zstd::stream::read::Decoder::new(file) else {
                continue;
            };
            Box::new(BufReader::new(decoder))
        } else {
            Box::new(BufReader::new(file))
        };
        let mut bytes = vec![];
        for line in 1..=5 {
            bytes.clear();
            match reader.read_until(b'\n', &mut bytes) {
                Ok(0) => break,
                Err(_) => continue,
                Ok(_) => (),
            };
            let Ok(v) = serde_json::from_slice::<Value>(&bytes) else {
                continue;
            };
            if !meta(&v) {
                continue;
            }
            let Some(text) = v
                .pointer("/payload/base_instructions/text")
                .and_then(Value::as_str)
            else {
                continue;
            };
            if text.len() > 1000 && !text.trim_start().starts_with(APPROVAL_JUDGE_PREFIX) {
                return Some(InstructionReference {
                    path: path.clone(),
                    physical_path: physical,
                    line,
                    text: text.into(),
                });
            }
        }
    }
    None
}
/// @0x1008b6900: read-only DBs, 5000 ms timeout, exact archived=0 SQL, first-seen
/// path order, exact string deduplication; individual DB/row failures are ignored.
pub fn collect_active_instruction_rollout_paths(dbs: &[PathBuf]) -> Vec<PathBuf> {
    let mut seen = BTreeSet::new();
    let mut paths = vec![];
    for db in dbs {
        let Ok(c) = Connection::open_with_flags(db, OpenFlags::SQLITE_OPEN_READ_ONLY) else {
            continue;
        };
        let _ = c.execute_batch("PRAGMA busy_timeout = 5000;");
        let Ok(mut stmt) =
            c.prepare("SELECT rollout_path FROM threads WHERE archived = 0 AND rollout_path != ''")
        else {
            continue;
        };
        let Ok(mut rows) = stmt.query([]) else {
            continue;
        };
        while let Ok(Some(row)) = rows.next() {
            if let Ok(Some(path)) = row.get::<_, Option<String>>(0) {
                if seen.insert(path.clone()) {
                    paths.push(path.into())
                }
            }
        }
    }
    paths
}
/// @0x1008b5cb8: at most 50 paths, five lines each, first session_meta text >1000
/// UTF-8 bytes which is not the approval judge prefix after trim_start; no model grouping.
pub fn find_reference_instructions(dbs: &[PathBuf]) -> Option<InstructionReference> {
    reference_in_ordered_paths(&collect_active_instruction_rollout_paths(dbs))
}
/// @0x10039da78: replace the entire base_instructions object, dropping stale provenance.
/// Record size protection belongs to the rollout publication step, not snapshot JSON strings.
pub fn rewrite_session_meta_instructions(record: &[u8], instructions: &str) -> Result<Vec<u8>> {
    let mut value: Value = serde_json::from_slice(record)
        .map_err(|e| RepairError(format!("Invalid data: parse session_meta line: {e}")))?;
    payload_mut(&mut value)?.insert("base_instructions".into(), json!({"text":instructions}));
    serde_json::to_vec(&value)
        .map_err(|e| RepairError(format!("Invalid data: serialize session_meta line: {e}")))
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionRolloutChange {
    pub file: RolloutFileChange,
    pub first_meta_backup: Vec<u8>,
    pub backup_path: PathBuf,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionSnapshotChange {
    pub path: PathBuf,
    pub original: Vec<u8>,
    pub replacement: Vec<u8>,
    pub values_updated: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoisonedInstructionRepairPlan {
    pub reference: Option<InstructionReference>,
    pub rollouts: Vec<InstructionRolloutChange>,
    pub snapshots: Vec<InstructionSnapshotChange>,
    pub warnings: Vec<String>,
    pub scanned: usize,
    pub relay_directory: PathBuf,
    pub databases: Vec<PathBuf>,
    pub unwritable_issues: usize,
    pub backup_directory: PathBuf,
    pub rollout_base: Option<PathBuf>,
}
fn poisoned_snapshot_paths(relay_directory: &Path, warnings: &mut Vec<String>) -> Vec<PathBuf> {
    let root = relay_directory.join("thread-router-migrations");
    let mut paths = vec![root.join("current.json")];
    match fs::read_dir(root.join("history")) {
        Ok(entries) => {
            for e in entries.flatten() {
                let p = e.path();
                if p.extension().is_some_and(|s| s == "json") {
                    paths.push(p)
                }
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
        Err(e) => warnings.push(format!("read migration snapshot history failed: {e}")),
    }
    paths
}
fn plan_instruction_snapshot(
    path: &Path,
    instructions: &str,
) -> Result<Option<InstructionSnapshotChange>> {
    if fs::metadata(path).is_err() {
        return Ok(None);
    }
    let text = fs::read_to_string(path)?;
    let mut value: Value =
        serde_json::from_str(&text).map_err(|e| RepairError(format!("JSON error: {e}")))?;
    let mut count = 0;
    if let Some(entries) = value.get_mut("entries").and_then(Value::as_array_mut) {
        for entry in entries {
            if let Some(fields) = entry.as_object_mut() {
                for field in fields.values_mut() {
                    if let Some(record) = field.as_str() {
                        if session_meta_instruction_issue(record.as_bytes())
                            != SessionInstructionIssue::Healthy
                        {
                            let Ok(new) =
                                rewrite_session_meta_instructions(record.as_bytes(), instructions)
                            else {
                                continue;
                            };
                            *field = Value::String(
                                String::from_utf8(new)
                                    .map_err(|e| RepairError(format!("Invalid data: {e}")))?,
                            );
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    if count == 0 {
        return Ok(None);
    }
    let replacement =
        serde_json::to_vec_pretty(&value).map_err(|e| RepairError(format!("JSON error: {e}")))?;
    Ok(Some(InstructionSnapshotChange {
        path: path.into(),
        original: text.into_bytes(),
        replacement,
        values_updated: count,
    }))
}
/// Recover the real reference and patch active main-thread FIRST metadata plus stored migration
/// metadata strings. A short healthy prompt is not a reference; absent full templates use the
/// native minimal instruction. No caller-supplied template or status flag participates.
pub fn plan_poisoned_instruction_repair(
    dbs: &[PathBuf],
    relay_directory: &Path,
) -> Result<PoisonedInstructionRepairPlan> {
    plan_poisoned_instruction_repair_with_base(dbs, relay_directory, None)
}
pub fn plan_poisoned_instruction_repair_with_base(
    dbs: &[PathBuf],
    relay_directory: &Path,
    rollout_base: Option<&Path>,
) -> Result<PoisonedInstructionRepairPlan> {
    let mut plan = PoisonedInstructionRepairPlan {
        reference: None,
        rollouts: vec![],
        snapshots: vec![],
        warnings: vec![],
        scanned: 0,
        relay_directory: relay_directory.into(),
        databases: dbs.to_vec(),
        unwritable_issues: 0,
        backup_directory: relay_directory.join("guardian-prompt-fix-backup"),
        rollout_base: rollout_base.map(Path::to_path_buf),
    };
    if dbs.is_empty() {
        return Ok(plan);
    }
    let rows = threads::collect_diagnostic_main_threads(dbs).map_err(RepairError)?;
    plan.reference = reference_in_ordered_paths(
        &collect_active_instruction_rollout_paths(dbs)
            .into_iter()
            .map(|p| project_rollout_path(&p, rollout_base))
            .collect::<Vec<_>>(),
    );
    let instructions = plan
        .reference
        .as_ref()
        .map(|r| r.text.as_str())
        .unwrap_or(MINIMAL_SAFE_INSTRUCTIONS)
        .to_owned();
    let backup = relay_directory.join("guardian-prompt-fix-backup");
    let mut seen = BTreeSet::new();
    for row in rows {
        let Some(path) = row.rollout_path else {
            plan.warnings
                .push(format!("{}: rollout path missing", row.id));
            continue;
        };
        let path = project_rollout_path(Path::new(&path), rollout_base)
            .to_string_lossy()
            .into_owned();
        if !seen.insert(path.clone()) {
            continue;
        }
        plan.scanned += 1;
        if let Some(observed) = super::observation::read(Path::new(&path)) {
            match observed {
                Ok(observed)
                    if observed.complete
                        && observed.first_meta.is_some()
                        && observed.instruction_issue == SessionInstructionIssue::Healthy =>
                {
                    continue
                }
                Err(error) => {
                    plan.warnings.push(format!("{}: {error}", row.id));
                    continue;
                }
                Ok(observed) if !observed.complete => {
                    plan.warnings
                        .push(format!("{}: history observation incomplete", row.id));
                    continue;
                }
                _ => (),
            }
        }
        let mut issue_observed = false;
        let attempt = (|| -> Result<Option<InstructionRolloutChange>> {
            let (physical, original, logical) = logical_file(Path::new(&path))?;
            let parsed = records(&logical);
            let Some((index, first)) = parsed.iter().enumerate().find(|(_, r)| meta(&r.value))
            else {
                return Err(RepairError("Invalid data: session_meta missing".into()));
            };
            let old = &logical[first.start..first.content_end];
            if session_meta_instruction_issue(old) == SessionInstructionIssue::Healthy {
                return Ok(None);
            }
            issue_observed = true;
            let new = fit_record(old, rewrite_session_meta_instructions(old, &instructions)?)?;
            let mut changes = BTreeMap::new();
            changes.insert(index, new);
            let replacement =
                encode_logical(&physical, rewrite_records(&logical, &changes, &parsed))?;
            Ok(Some(InstructionRolloutChange {
                file: RolloutFileChange {
                    requested_path: path.clone().into(),
                    physical_path: physical,
                    original,
                    replacement,
                    thread_ids: vec![row.id.clone()],
                    databases: vec![row.db.clone()],
                },
                first_meta_backup: old.to_vec(),
                backup_path: backup.join(format!("{}.jsonl.bak", row.id)),
            }))
        })();
        match attempt {
            Ok(Some(change)) => plan.rollouts.push(change),
            Ok(None) => (),
            Err(e) => {
                if issue_observed {
                    plan.unwritable_issues += 1;
                }
                plan.warnings.push(format!(
                    "{}: poisoned instruction candidate preserved: {e}",
                    row.id
                ));
            }
        }
    }
    for path in poisoned_snapshot_paths(relay_directory, &mut plan.warnings) {
        match plan_instruction_snapshot(&path, &instructions) {
            Ok(Some(change)) => plan.snapshots.push(change),
            Ok(None) => (),
            Err(e) => plan
                .warnings
                .push(format!("{}: snapshot preserved: {e}", path.display())),
        }
    }
    Ok(plan)
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstructionRepairSummary {
    pub summary: RepairSummary,
    pub snapshot_values_updated: usize,
    pub remaining_issues: usize,
    pub used_minimal_fallback: bool,
}
/// Native failures preserve the affected file and continue other tasks; first-meta backups
/// are written under guardian-prompt-fix-backup before rollout replacement.
pub fn apply_poisoned_instruction_repair(
    plan: &PoisonedInstructionRepairPlan,
) -> Result<InstructionRepairSummary> {
    apply_poisoned_instruction_repair_with_verifier(plan, |plan| {
        plan_poisoned_instruction_repair_with_base(
            &plan.databases,
            &plan.relay_directory,
            plan.rollout_base.as_deref(),
        )
    })
}
/// Verification is a typed, real producer; the app supplies its stable private SQLite projection.
pub fn apply_poisoned_instruction_repair_with_verifier(
    plan: &PoisonedInstructionRepairPlan,
    verify: impl FnOnce(&PoisonedInstructionRepairPlan) -> Result<PoisonedInstructionRepairPlan>,
) -> Result<InstructionRepairSummary> {
    let mut result = InstructionRepairSummary {
        summary: RepairSummary {
            scanned: plan.scanned,
            warnings: plan.warnings.clone(),
            ..Default::default()
        },
        used_minimal_fallback: false,
        ..Default::default()
    };
    for change in &plan.rollouts {
        let attempt = (|| -> Result<()> {
            let file = &change.file;
            if threads::resolve_rollout(&file.requested_path)?.as_ref() != Some(&file.physical_path)
                || fs::read(&file.physical_path)? != file.original
            {
                return Err(RepairError(
                    "first session_meta changed before instruction repair; history preserved"
                        .into(),
                ));
            }
            write_private_report(&change.backup_path, &change.first_meta_backup)?;
            result.summary.backups.push(change.backup_path.clone());
            let modified = fs::metadata(&file.physical_path)?.modified().ok();
            write_atomic(&file.physical_path, &file.replacement)?;
            if let Some(m) = modified {
                if let Ok(f) = fs::OpenOptions::new().write(true).open(&file.physical_path) {
                    let _ = f.set_times(fs::FileTimes::new().set_modified(m));
                }
            }
            result.summary.rollout_updated += 1;
            Ok(())
        })();
        if let Err(e) = attempt {
            result.summary.warnings.push(format!(
                "{}: instruction repair failed: {e}",
                change.file.physical_path.display()
            ))
        }
    }
    for change in &plan.snapshots {
        let attempt = (|| -> Result<()> {
            if fs::read(&change.path)? != change.original {
                return Err(RepairError(
                    "migration snapshot changed after repair preview".into(),
                ));
            }
            result.summary.backups.push(backup_file(
                &change.path,
                &plan.backup_directory,
                "migration-snapshot",
            )?);
            write_atomic(&change.path, &change.replacement)?;
            result.snapshot_values_updated += change.values_updated;
            Ok(())
        })();
        if let Err(e) = attempt {
            result.summary.warnings.push(format!(
                "{}: snapshot repair failed: {e}",
                change.path.display()
            ))
        }
    }
    let verification = verify(plan)?;
    result.used_minimal_fallback = plan.reference.is_none()
        && (result.summary.rollout_updated > 0 || result.snapshot_values_updated > 0);
    result.remaining_issues = verification.unwritable_issues
        + verification.rollouts.len()
        + verification
            .snapshots
            .iter()
            .map(|s| s.values_updated)
            .sum::<usize>();
    for warning in verification.warnings {
        if !result.summary.warnings.contains(&warning) {
            result.summary.warnings.push(warning)
        }
    }
    Ok(result)
}

fn project_rollout_path(path: &Path, base: Option<&Path>) -> PathBuf {
    if path.is_absolute() {
        path.to_owned()
    } else {
        base.map(|b| b.join(path))
            .unwrap_or_else(|| path.to_owned())
    }
}
/// Compatibility adapter for Atelier's stored paths relative to its explicit Codex home.
pub fn inspect_missing_rollouts_with_base(
    dbs: &[PathBuf],
    trusted_root: &Path,
    base: Option<&Path>,
) -> threads::MissingRolloutsInspection {
    if base.is_none() {
        return threads::inspect_missing_rollouts_for_convergence(dbs, trusted_root);
    }
    let mut result = threads::MissingRolloutsInspection::default();
    for db in dbs {
        if fs::metadata(db).is_err() {
            continue;
        }
        let rows = open_rw(db)
            .and_then(|c| threads::collect_provider_convergence_targets(&c).map_err(RepairError));
        match rows {
            Err(e) => result.errors.push(e.to_string()),
            Ok(rows) => {
                for row in rows {
                    result.total += 1;
                    let path = row
                        .rollout_path
                        .as_ref()
                        .map(|p| project_rollout_path(Path::new(p), base));
                    let existing = path
                        .as_ref()
                        .map(|p| threads::resolve_rollout(p))
                        .transpose();
                    match existing {
                        Err(e) => result.errors.push(e.to_string()),
                        Ok(Some(Some(_))) => (),
                        Ok(_) => {
                            result.missing += 1;
                            if path.as_ref().is_some_and(|p| {
                                threads::find_trusted_rollout_backup(trusted_root, &row.id, p)
                                    .is_some()
                            }) {
                                result.recoverable += 1
                            } else {
                                result.unrecoverable += 1
                            }
                            if result.examples.len() < 3 {
                                result.examples.push(row.id);
                            }
                        }
                    }
                }
            }
        }
    }
    result
}
#[derive(Debug, Clone, Default)]
pub struct LegacyMigrationInspection {
    pub ids: BTreeSet<String>,
    pub errors: Vec<String>,
}
/// Diagnostic wrapper @0x100391470: unlike strict cleanup, only active main-thread IDs
/// participate; aimai1 is valid while routing is enabled. Cleanup is not offered in that mode.
pub fn inspect_legacy_migrated_threads(
    dbs: &[PathBuf],
    router_enabled: bool,
) -> LegacyMigrationInspection {
    let mut result = LegacyMigrationInspection::default();
    for db in dbs {
        let attempt = (|| -> Result<Vec<String>> {
            let c = Connection::open_with_flags(
                db,
                OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
            )?;
            let sql = if router_enabled {
                "SELECT id FROM threads WHERE (model_provider IN ('aimami_codex_router') OR (model_provider LIKE 'openai' AND model_provider <> 'openai')) AND COALESCE(archived,0)=0 AND instr(COALESCE(source,''),'\"subagent\"')=0"
            } else {
                "SELECT id FROM threads WHERE (model_provider IN ('aimami_codex_router','aimai1') OR (model_provider LIKE 'openai' AND model_provider <> 'openai')) AND COALESCE(archived,0)=0 AND instr(COALESCE(source,''),'\"subagent\"')=0"
            };
            let mut stmt = c.prepare(sql)?;
            let ids = stmt
                .query_map([], |r| r.get::<_, String>(0))?
                .collect::<std::result::Result<Vec<_>, _>>()?;
            Ok(ids)
        })();
        match attempt {
            Ok(ids) => result.ids.extend(ids),
            Err(e) => result.errors.push(e.to_string()),
        }
    }
    result
}
