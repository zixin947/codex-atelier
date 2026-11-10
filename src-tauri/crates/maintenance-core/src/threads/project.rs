//! Project-state repair payloads recovered from codex_project_state::repair
//! (0x1003b357c). Connections/paths are explicit; this module discovers no home.
use super::{
    collect_cwd_backfill_candidates, is_codex_generated_projectless_dir, normalize_codex_path,
    ProjectDbSnapshot,
};
use rusqlite::{types::ValueRef, Connection};
use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet};

/// state_db_score@1008ebccc / comparator@1008f349c. SQL scoring is performed
/// against a private copy; physical DB/WAL mtime and directory preference are inputs.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DatabaseProbe {
    pub updated_ms: i64,
    pub time_column_rank: u8,
    pub active_threads: i64,
}
pub fn probe_database(connection: &Connection) -> rusqlite::Result<DatabaseProbe> {
    connection.busy_timeout(std::time::Duration::from_millis(75))?;
    let names = columns(connection)?;
    let time_column_rank = if names.contains("recency_at_ms") {
        2
    } else if names.contains("updated_at_ms") {
        1
    } else {
        0
    };
    let updated_ms = if names.contains("updated_at_ms") {
        connection.query_row(
            "SELECT COALESCE(MAX(updated_at_ms), 0) FROM threads",
            [],
            |r| r.get(0),
        )?
    } else if names.contains("updated_at") {
        connection
            .query_row(
                "SELECT COALESCE(MAX(updated_at), 0) FROM threads",
                [],
                |r| r.get::<_, i64>(0),
            )?
            .saturating_mul(1000)
    } else {
        0
    };
    let sql = if names.contains("archived") {
        "SELECT COUNT(*) FROM threads WHERE COALESCE(archived, 0) = 0"
    } else {
        "SELECT COUNT(*) FROM threads"
    };
    let active_threads = connection.query_row(sql, [], |r| r.get(0))?;
    Ok(DatabaseProbe {
        updated_ms,
        time_column_rank,
        active_threads,
    })
}
pub fn database_probe_error_kind(error: &rusqlite::Error) -> u8 {
    use rusqlite::ffi::ErrorCode;
    if matches!(
        error.sqlite_error_code(),
        Some(ErrorCode::DatabaseCorrupt | ErrorCode::NotADatabase)
    ) {
        0
    } else {
        1
    }
}
pub fn database_probe_retryable(error: &rusqlite::Error) -> bool {
    use rusqlite::ffi::ErrorCode;
    matches!(
        error.sqlite_error_code(),
        Some(
            ErrorCode::DatabaseBusy
                | ErrorCode::DatabaseLocked
                | ErrorCode::SystemIoFailure
                | ErrorCode::CannotOpen
                | ErrorCode::FileLockingProtocolFailed
                | ErrorCode::SchemaChanged
        )
    )
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DatabaseScore {
    pub modified_ns: u128,
    pub version: u32,
    pub preferred_directory: bool,
    pub probe: Result<DatabaseProbe, u8>,
}
impl DatabaseScore {
    pub fn compare(&self, other: &Self) -> std::cmp::Ordering {
        let kind = |p: &Result<DatabaseProbe, u8>| p.as_ref().map_or_else(|kind| *kind, |_| 2);
        let a = kind(&self.probe);
        let b = kind(&other.probe);
        let result = if a == 0 && b != 0 {
            std::cmp::Ordering::Less
        } else if a != 0 && b == 0 {
            std::cmp::Ordering::Greater
        } else if let (Ok(a), Ok(b)) = (&self.probe, &other.probe) {
            self.version
                .cmp(&other.version)
                .then(a.updated_ms.cmp(&b.updated_ms))
                .then(a.time_column_rank.cmp(&b.time_column_rank))
                .then(a.active_threads.cmp(&b.active_threads))
                .then(self.modified_ns.cmp(&other.modified_ns))
        } else {
            self.modified_ns
                .cmp(&other.modified_ns)
                .then(a.cmp(&b))
                .then(self.version.cmp(&other.version))
        };
        result.then(self.preferred_directory.cmp(&other.preferred_directory))
    }
}
pub fn backup_filename(timestamp_ms: i64, unique: &str) -> Option<String> {
    let timestamp = chrono::DateTime::from_timestamp_millis(timestamp_ms)?;
    Some(format!(
        "{}.{unique}.codex-global-state.json.bak",
        timestamp.format("%Y%m%d-%H%M%S")
    ))
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProjectRecords {
    pub roots: Vec<String>,
    pub hints: BTreeMap<String, String>,
    pub active_threads: usize,
    pub backfill: Vec<(String, String)>,
}
impl ProjectRecords {
    pub fn summary(&self) -> ProjectDbSnapshot {
        ProjectDbSnapshot {
            roots: self.roots.len(),
            hints: self.hints.len(),
            active_threads: self.active_threads,
            cwd_backfill: self.backfill.len(),
            errors: vec![],
        }
    }
}
pub fn columns(connection: &Connection) -> rusqlite::Result<BTreeSet<String>> {
    connection
        .prepare("PRAGMA table_info(threads)")?
        .query_map([], |r| r.get(1))?
        .collect()
}
/// Payload counterpart of collect_project_state_from_active_db@1003b25a4.
/// Existing cwd rows deliberately have no source/projectless exclusion. The separate
/// backfill producer skips only explicit projectless IDs and `"subagent"` substrings.
pub fn collect_records(
    connection: &Connection,
    projectless: &BTreeSet<String>,
) -> rusqlite::Result<ProjectRecords> {
    let columns = columns(connection)?;
    if !columns.contains("id") || !columns.contains("cwd") {
        return Ok(ProjectRecords::default());
    }
    let active = if columns.contains("archived") {
        "COALESCE(archived, 0) = 0"
    } else {
        "1 = 1"
    };
    let order = if columns.contains("updated_at_ms") {
        "COALESCE(updated_at_ms, 0)"
    } else if columns.contains("updated_at") {
        "COALESCE(updated_at, 0)"
    } else {
        "0"
    };
    let active_threads = connection
        .query_row(
            &format!("SELECT COUNT(*) FROM threads WHERE {active}"),
            [],
            |r| r.get::<_, i64>(0),
        )
        .unwrap_or(0)
        .max(0) as usize;
    let mut statement = connection.prepare(&format!("SELECT id, cwd FROM threads WHERE {active} AND COALESCE(cwd, '') != '' ORDER BY {order} DESC"))?;
    let mut rows = statement.query([])?;
    let mut roots = BTreeSet::new();
    let mut hints = BTreeMap::new();
    while let Some(row) = rows.next()? {
        let id = row.get::<_, String>(0)?;
        let cwd = match row.get_ref(1)? {
            ValueRef::Text(b) | ValueRef::Blob(b) => match std::str::from_utf8(b) {
                Ok(s) => normalize_codex_path(s),
                Err(_) => continue,
            },
            _ => continue,
        };
        if cwd.is_empty() {
            continue;
        }
        hints.insert(id, cwd.clone());
        if !is_codex_generated_projectless_dir(&cwd) {
            roots.insert(cwd);
        }
    }
    // Native diagnostic collection ignores a backfill-only SQL error, but repair's
    // write-stage collector propagates it. Keep that distinction in the concrete API.
    let backfill =
        collect_cwd_backfill_candidates(connection, &columns, projectless).unwrap_or_default();
    Ok(ProjectRecords {
        roots: roots.into_iter().collect(),
        hints,
        active_threads,
        backfill,
    })
}
pub fn projectless_ids(before: Option<&[u8]>) -> BTreeSet<String> {
    before
        .and_then(|b| serde_json::from_slice::<Value>(b).ok())
        .and_then(|v| {
            v.get("projectless-thread-ids")
                .and_then(Value::as_array)
                .cloned()
        })
        .unwrap_or_default()
        .into_iter()
        .filter_map(|v| v.as_str().map(str::to_owned))
        .collect()
}
fn strings(map: &Map<String, Value>, key: &str) -> Vec<String> {
    map.get(key)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(normalize_codex_path)
        .filter(|s| !s.is_empty())
        .collect()
}
/// merge_unique@1003aff10 preserves the first normalized occurrence, in input order.
pub fn merge_unique(
    first: impl IntoIterator<Item = String>,
    second: impl IntoIterator<Item = String>,
) -> Vec<String> {
    let mut seen = BTreeSet::new();
    first
        .into_iter()
        .chain(second)
        .map(|s| normalize_codex_path(&s))
        .filter(|s| !s.is_empty() && seen.insert(s.clone()))
        .collect()
}
#[derive(Clone, Debug, PartialEq)]
pub struct ComposedProjectState {
    /// None is native's no-reconstructable-roots branch: no JSON publication occurs.
    pub after: Option<Vec<u8>>,
    pub roots: usize,
    pub hints: usize,
}
/// repair@1003b357c: unreadable/invalid/non-object JSON falls back to an empty map
/// in the force-repair leaf; repair_if_needed separately refuses unstable reads.
pub fn compose_project_state(
    before: Option<&[u8]>,
    records: &ProjectRecords,
) -> Result<ComposedProjectState, serde_json::Error> {
    let mut map = before
        .and_then(|b| serde_json::from_slice::<Value>(b).ok())
        .and_then(|v| v.as_object().cloned())
        .unwrap_or_default();
    let roots = merge_unique(
        strings(&map, "electron-saved-workspace-roots"),
        records.roots.clone(),
    );
    if roots.is_empty() {
        return Ok(ComposedProjectState {
            after: None,
            roots: 0,
            hints: records.hints.len(),
        });
    }
    let order = merge_unique(strings(&map, "project-order"), roots.clone());
    let root_set: BTreeSet<_> = roots.iter().map(String::as_str).collect();
    let mut active = strings(&map, "active-workspace-roots")
        .into_iter()
        .filter(|s| root_set.contains(s.as_str()))
        .collect::<Vec<_>>();
    if active.is_empty() {
        active.push(roots[0].clone());
    }
    let mut hints = map
        .get("thread-workspace-root-hints")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    for (id, cwd) in &records.hints {
        hints.insert(id.clone(), Value::String(cwd.clone()));
    }
    let count = hints.len();
    map.insert(
        "electron-saved-workspace-roots".into(),
        serde_json::to_value(&roots)?,
    );
    map.insert("project-order".into(), serde_json::to_value(order)?);
    map.insert(
        "active-workspace-roots".into(),
        serde_json::to_value(active)?,
    );
    map.insert("thread-workspace-root-hints".into(), Value::Object(hints));
    Ok(ComposedProjectState {
        after: Some(serde_json::to_vec_pretty(&Value::Object(map))?),
        roots: roots.len(),
        hints: count,
    })
}
/// Native SQL@1013112ce. This is the concrete mutation used on an owned write
/// transaction and on a private projection; it never rewrites rollout_path.
pub fn write_cwd_backfill(
    connection: &Connection,
    candidates: &[(String, String)],
) -> rusqlite::Result<usize> {
    let mut count = 0;
    for (id, cwd) in candidates {
        count += connection.execute(
            "UPDATE threads SET cwd = ?1 WHERE id = ?2 AND COALESCE(cwd, '') = ''",
            rusqlite::params![cwd, id],
        )?;
    }
    Ok(count)
}
