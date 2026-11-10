//! Native project-index inspection/repair with explicit Context roots and private SQLite copies.
use super::*;
use atelier_maintenance_core::{
    catalog::publication,
    state_db,
    threads::{self as core, project},
};
use rusqlite::{Connection, OpenFlags};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    time::{SystemTime, UNIX_EPOCH},
};
#[cfg(test)]
#[path = "project_state_tests.rs"]
mod tests;

#[derive(Clone, PartialEq, Eq)]
struct Stamp {
    length: u64,
    sha256: String,
    modified: Option<SystemTime>,
}
fn stamp(path: &Path) -> Result<Option<Stamp>> {
    let before = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(err(e)),
    };
    if !before.is_file() || before.file_type().is_symlink() {
        return Err("项目维护依赖不是普通文件".into());
    }
    let mut file = fs::File::open(path).map_err(err)?;
    let mut hash = Sha256::new();
    let mut buffer = [0u8; 65536];
    let mut length = 0;
    loop {
        let n = file.read(&mut buffer).map_err(err)?;
        if n == 0 {
            break;
        }
        hash.update(&buffer[..n]);
        length += n as u64;
    }
    let after = fs::symlink_metadata(path).map_err(err)?;
    if before.len() != length
        || before.len() != after.len()
        || before.modified().ok() != after.modified().ok()
    {
        return Err("项目维护依赖在读取期间变化".into());
    }
    Ok(Some(Stamp {
        length,
        sha256: format!("{:x}", hash.finalize()),
        modified: before.modified().ok(),
    }))
}
fn optional(path: &Path, limit: u64) -> Result<Option<Vec<u8>>> {
    match fs::symlink_metadata(path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(err(e)),
        Ok(_) => read_bounded(path, limit).map(Some),
    }
}
struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Result<Self> {
        let path =
            std::env::temp_dir().join(format!("atelier-native-project-{}", uuid::Uuid::new_v4()));
        crate::gateway::storage::private_dir(&path)?;
        Ok(Self(path))
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn wal(db: &Path) -> PathBuf {
    PathBuf::from(format!("{}-wal", db.display()))
}
/// No SQLite connection ever opens the source for planning or diagnosis.
fn with_private<T>(db: &Path, mut read: impl FnMut(&Path) -> Result<T>) -> Result<T> {
    if atelier_maintenance_core::threads::observation::active() {
        return super::snapshot::with_observed_path(db, |_, connection| {
            let scratch = Scratch::new()?;
            let copy = scratch.0.join("project.sqlite");
            connection
                .execute("VACUUM INTO ?1", [copy.to_string_lossy().as_ref()])
                .map_err(err)?;
            read(&copy)
        });
    }
    for _ in 0..2 {
        let before = (stamp(db)?, stamp(&wal(db))?);
        if before.0.is_none() {
            return Err("项目线程库不存在".into());
        }
        let scratch = Scratch::new()?;
        let copy = scratch.0.join("state.sqlite");
        crate::gateway::storage::atomic_private(&copy, &read_bounded(db, 128 * 1024 * 1024)?)?;
        if before.1.as_ref().is_some_and(|s| s.length > 0) {
            let bytes = read_bounded(&wal(db), 64 * 1024 * 1024)?;
            // 同 snapshot：WAL 不可信时只读主库，不判为故障。
            if super::wal::validate_relaxed(&bytes)? == super::wal::WalStatus::Usable {
                crate::gateway::storage::atomic_private(&wal(&copy), &bytes)?;
            }
        }
        if before != (stamp(db)?, stamp(&wal(db))?) {
            continue;
        }
        let result = read(&copy);
        if before == (stamp(db)?, stamp(&wal(db))?) {
            return result;
        }
    }
    Err("项目线程库持续变化，无法建立稳定快照".into())
}
fn candidate_databases(ctx: &Context) -> Result<Vec<(PathBuf, bool)>> {
    let raw = optional(&ctx.config(), 1024 * 1024)?
        .map(|b| String::from_utf8(b).map_err(err))
        .transpose()?;
    let configured = state_db::configured_sqlite_home(
        raw.as_deref(),
        ctx.sqlite_home_env.as_deref(),
        ctx.working_directory.as_deref(),
    );
    let directories = configured
        .map(|p| vec![(p, true)])
        .unwrap_or_else(|| vec![(ctx.home.join("sqlite"), true), (ctx.home.clone(), false)]);
    Ok(directories
        .into_iter()
        .filter_map(|(dir, preferred)| state_db::highest_state_db(&dir).map(|db| (db, preferred)))
        .collect())
}
fn score(db: &Path, preferred: bool) -> Result<project::DatabaseScore> {
    let modified_ns = [db.to_path_buf(), wal(db)]
        .iter()
        .filter_map(|p| {
            fs::metadata(p)
                .ok()?
                .modified()
                .ok()?
                .duration_since(UNIX_EPOCH)
                .ok()
        })
        .map(|d| d.as_nanos())
        .max()
        .unwrap_or(0);
    let version = db
        .file_name()
        .and_then(|s| s.to_str())
        .and_then(|s| s.strip_prefix("state_"))
        .and_then(|s| s.strip_suffix(".sqlite"))
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let probe = with_private(db, |copy| {
        let mut last = None;
        for attempt in 0..3 {
            let probe = Connection::open_with_flags(
                copy,
                OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
            )
            .and_then(|c| project::probe_database(&c));
            match probe {
                Ok(v) => return Ok(Ok(v)),
                Err(e) => {
                    let retry = project::database_probe_retryable(&e);
                    last = Some(project::database_probe_error_kind(&e));
                    if !retry || attempt == 2 {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(25));
                }
            }
        }
        Ok(Err(last.unwrap_or(1)))
    })?;
    Ok(project::DatabaseScore {
        modified_ns,
        version,
        preferred_directory: preferred,
        probe,
    })
}
/// Native active_db_path resolves ONE candidate. It does not merge historical databases.
pub(super) fn active_database(ctx: &Context) -> Result<Option<PathBuf>> {
    let candidates = candidate_databases(ctx)?;
    let Some((first, preferred)) = candidates.first() else {
        return Ok(None);
    };
    if candidates.len() == 1 {
        return Ok(fs::metadata(first).is_ok().then(|| first.clone()));
    }
    let mut selected = first.clone();
    let mut prior = score(first, *preferred)?;
    for (candidate, preferred) in candidates.iter().skip(1) {
        let current = score(candidate, *preferred)?;
        if prior.compare(&current) != std::cmp::Ordering::Greater {
            selected = candidate.clone();
            prior = current;
        }
    }
    Ok(fs::metadata(&selected).is_ok().then_some(selected))
}
#[derive(Clone, Serialize, PartialEq, Eq)]
pub(super) struct CwdUpdate {
    pub id: String,
    pub before: String,
    pub after: String,
    pub path: String,
}
#[derive(Clone)]
pub(super) struct Prepared {
    home: PathBuf,
    app_root: PathBuf,
    working_directory: Option<PathBuf>,
    sqlite_home_env: Option<std::ffi::OsString>,
    database: Option<PathBuf>,
    state: PathBuf,
    before: Option<Vec<u8>>,
    after: Option<Vec<u8>>,
    updates: Vec<CwdUpdate>,
    dependencies: BTreeMap<PathBuf, Option<Stamp>>,
    diagnostic: DiagnosticItem,
    roots: usize,
    hints: usize,
    potential_cwds: BTreeMap<String, String>,
    reviewed_state: Option<Vec<u8>>,
}
impl Prepared {
    pub(super) fn diagnostic(&self) -> &DiagnosticItem {
        &self.diagnostic
    }
    pub(super) fn state_path(&self) -> &Path {
        &self.state
    }
    pub(super) fn before(&self) -> Option<&[u8]> {
        self.before.as_deref()
    }
    pub(super) fn after(&self) -> Option<&[u8]> {
        self.after.as_deref()
    }
    pub(super) fn database(&self) -> Option<&Path> {
        self.database.as_deref()
    }
    pub(super) fn cwd_updates(&self) -> &[CwdUpdate] {
        &self.updates
    }
}
fn diagnostic(i: &core::ProjectStateInspection) -> DiagnosticItem {
    let native = core::check_project_state_integrity(i);
    DiagnosticItem {
        id: native.id.into(),
        group: "runtime".into(),
        status: native.status.into(),
        label: native.title,
        detail: native.detail.unwrap_or_default(),
        fixable: native.fixable,
    }
}
fn inspect_value(
    before: Option<&[u8]>,
    db: core::ProjectDbSnapshot,
) -> Result<core::ProjectStateInspection> {
    let scratch = Scratch::new()?;
    let path = scratch.0.join("project.json");
    if let Some(bytes) = before {
        crate::gateway::storage::atomic_private(&path, bytes)?;
    }
    Ok(core::inspect_project_state_with_db(&path, db))
}
fn representations(path: &Path) -> Vec<PathBuf> {
    let logical = if path.to_string_lossy().ends_with(".jsonl.zst") {
        path.with_extension("")
    } else {
        path.to_owned()
    };
    vec![logical.clone(), logical.with_extension("jsonl.zst")]
}
fn prepare_database(
    ctx: &Context,
    db: &Path,
    before: Option<&[u8]>,
    force_compose: bool,
) -> Result<(
    core::ProjectStateInspection,
    Vec<CwdUpdate>,
    project::ComposedProjectState,
    BTreeSet<PathBuf>,
    BTreeMap<String, String>,
)> {
    let projectless = project::projectless_ids(before);
    with_private(db, |copy| {
        let connection = Connection::open_with_flags(
            copy,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .map_err(err)?;
        let columns = project::columns(&connection).map_err(err)?;
        // Projection is private: drop copied application triggers before resolving relative IO paths.
        let triggers = connection
            .prepare("SELECT name FROM sqlite_master WHERE type='trigger'")
            .map_err(err)?
            .query_map([], |r| r.get::<_, String>(0))
            .map_err(err)?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(err)?;
        for trigger in triggers {
            connection
                .execute_batch(&format!(
                    "DROP TRIGGER \"{}\"",
                    trigger.replace('"', "\"\"")
                ))
                .map_err(err)?;
        }
        let mut raw = BTreeMap::new();
        if columns.contains("id") && columns.contains("rollout_path") {
            let paths = connection
                .prepare("SELECT id,rollout_path FROM threads")
                .map_err(err)?
                .query_map([], |r| {
                    Ok((r.get::<_, String>(0)?, r.get::<_, Option<String>>(1)?))
                })
                .map_err(err)?
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(err)?;
            for (id, path) in paths {
                if let Some(path) = path {
                    if !path.is_empty() && !Path::new(&path).is_absolute() {
                        connection
                            .execute(
                                "UPDATE threads SET rollout_path=?1 WHERE id=?2",
                                rusqlite::params![ctx.home.join(&path).to_string_lossy(), id],
                            )
                            .map_err(err)?;
                    }
                    raw.insert(id, path);
                }
            }
        }
        let mut potential = BTreeMap::new();
        let mut observed = BTreeSet::new();
        if ["id", "cwd", "rollout_path"]
            .iter()
            .all(|c| columns.contains(*c))
        {
            let active = if columns.contains("archived") {
                "COALESCE(archived, 0) = 0"
            } else {
                "1 = 1"
            };
            let source = if columns.contains("source") {
                "COALESCE(source, '')"
            } else {
                "''"
            };
            let mut statement=connection.prepare(&format!("SELECT id,{source} FROM threads WHERE {active} AND COALESCE(cwd, '') = '' AND COALESCE(rollout_path, '') != ''")).map_err(err)?;
            let candidates = statement
                .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))
                .map_err(err)?;
            for row in candidates {
                let (id, source) = row.map_err(err)?;
                if projectless.contains(&id) || source.contains("\"subagent\"") {
                    continue;
                }
                if let Some(path) = raw.get(&id) {
                    potential.insert(id, path.clone());
                    let path = Path::new(path);
                    let path = if path.is_absolute() {
                        path.to_owned()
                    } else {
                        ctx.home.join(path)
                    };
                    observed.extend(representations(&path));
                }
            }
        }
        let records = project::collect_records(&connection, &projectless).map_err(err)?;
        let inspected = inspect_value(before, records.summary())?;
        if !inspected.needs_repair && !force_compose {
            return Ok((
                inspected,
                vec![],
                project::compose_project_state(before, &records).map_err(err)?,
                observed,
                potential,
            ));
        }
        let candidates = core::collect_cwd_backfill_candidates(&connection, &columns, &projectless)
            .map_err(err)?;
        // Only producer-selected cwd candidates contribute repair source dependencies.
        for (id, _) in &candidates {
            if let Some(raw) = raw.get(id) {
                let path = Path::new(raw);
                let path = if path.is_absolute() {
                    path.to_owned()
                } else {
                    ctx.home.join(path)
                };
                observed.extend(representations(&path));
            }
        }
        let updates = candidates
            .iter()
            .map(|(id, after)| {
                Ok(CwdUpdate {
                    id: id.clone(),
                    before: String::new(),
                    after: after.clone(),
                    path: raw.get(id).cloned().ok_or("cwd回填缺少原始rollout路径")?,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        project::write_cwd_backfill(&connection, &candidates).map_err(err)?;
        let after = project::collect_records(&connection, &projectless).map_err(err)?;
        Ok((
            inspected,
            updates,
            project::compose_project_state(before, &after).map_err(err)?,
            observed,
            potential,
        ))
    })
}
fn prepare_for_database(ctx: &Context, database: Option<PathBuf>) -> Result<Prepared> {
    let state = ctx.home.join(".codex-global-state.json");
    let before = optional(&state, 8 * 1024 * 1024)?;
    let mut dependencies = BTreeMap::new();
    for p in [ctx.config(), state.clone()] {
        dependencies.insert(p.clone(), stamp(&p)?);
    }
    for (db, _) in candidate_databases(ctx)? {
        for p in [db.clone(), wal(&db)] {
            dependencies.insert(p.clone(), stamp(&p)?);
        }
    }
    if let Some(db) = &database {
        for p in [db.clone(), wal(db)] {
            dependencies.insert(p.clone(), stamp(&p)?);
        }
    }
    let (inspection, updates, composed, observed, potential_cwds) = match database.as_deref() {
        Some(db) => prepare_database(ctx, db, before.as_deref(), false)
            .map_err(|error| format!("{}: {error}", db.display()))?,
        None => {
            let i = inspect_value(before.as_deref(), core::ProjectDbSnapshot::default())?;
            let c = project::compose_project_state(
                before.as_deref(),
                &project::ProjectRecords::default(),
            )
            .map_err(err)?;
            (i, vec![], c, BTreeSet::new(), BTreeMap::new())
        }
    };
    for p in observed {
        dependencies.insert(p.clone(), stamp(&p)?);
    }
    let reviewed_state = composed.after.clone();
    let after = if inspection.needs_repair {
        composed.after
    } else {
        None
    };
    for (p, s) in &dependencies {
        if stamp(p)? != *s {
            return Err("项目修复来源在预览期间变化".into());
        }
    }
    Ok(Prepared {
        home: ctx.home.clone(),
        app_root: ctx.app_root.clone(),
        working_directory: ctx.working_directory.clone(),
        sqlite_home_env: ctx.sqlite_home_env.clone(),
        database,
        state,
        before,
        after,
        updates,
        dependencies,
        diagnostic: diagnostic(&inspection),
        roots: composed.roots,
        hints: composed.hints,
        potential_cwds,
        reviewed_state,
    })
}
pub(super) fn prepare(ctx: &Context) -> Result<Prepared> {
    prepare_for_database(ctx, active_database(ctx)?)
}
pub(super) fn inspect(ctx: &Context) -> DiagnosticItem {
    // Diagnosis needs the observation, not sealed repair dependencies or source hashes.
    let observed = (|| -> Result<DiagnosticItem> {
        let before = optional(&ctx.home.join(".codex-global-state.json"), 8 * 1024 * 1024)?;
        let inspection = match active_database(ctx)? {
            Some(db) => prepare_database(ctx, &db, before.as_deref(), false)?.0,
            None => inspect_value(before.as_deref(), core::ProjectDbSnapshot::default())?,
        };
        Ok(diagnostic(&inspection))
    })();
    match observed {
        Ok(item) => item,
        Err(e) => {
            let state = optional(&ctx.home.join(".codex-global-state.json"), 8 * 1024 * 1024);
            let i = if state.is_err() {
                core::ProjectStateInspection {
                    read_error: Some(e),
                    ..Default::default()
                }
            } else {
                core::ProjectStateInspection {
                    db_errors: vec![e],
                    ..Default::default()
                }
            };
            diagnostic(&i)
        }
    }
}
pub(super) fn preview_lines(plan: &Prepared) -> Vec<String> {
    vec![format!("按原生活跃线程库回填 {} 条空cwd，合并项目入口 {} 个、线程映射 {} 项；保留已有字段与projectless标记。",plan.updates.len(),plan.roots,plan.hints),if plan.before.is_none()&&plan.after.is_some(){"缺失项目索引将从实际线程cwd重建；不会创建空索引。".into()}else{"先备份原项目索引，再发布并执行原生复检；已提交cwd不因后续JSON失败回滚。".into()}]
}
pub(super) fn validate(ctx: &Context, p: &Prepared) -> Result<()> {
    if ctx.home != p.home
        || ctx.app_root != p.app_root
        || ctx.working_directory != p.working_directory
        || ctx.sqlite_home_env != p.sqlite_home_env
        || active_database(ctx)? != p.database
    {
        return Err("项目修复上下文或活跃数据库已变化".into());
    }
    for (path, before) in &p.dependencies {
        if stamp(path)? != *before {
            return Err("项目修复依赖已被外部更新，请重新预览".into());
        }
    }
    let fresh = prepare(ctx)?;
    if fresh.after != p.after || fresh.updates != p.updates || fresh.before != p.before {
        return Err("项目重建或cwd回填范围已变化，请重新预览".into());
    }
    Ok(())
}
fn validate_prefix(
    ctx: &Context,
    p: &Prepared,
    prefix: &super::native_threads::ApprovedThreadEffects,
) -> Result<()> {
    prefix.validate_current(ctx)?;
    if ctx.home != p.home
        || ctx.app_root != p.app_root
        || ctx.working_directory != p.working_directory
        || ctx.sqlite_home_env != p.sqlite_home_env
        || active_database(ctx)? != p.database
    {
        return Err("项目批次作用域或活跃数据库已变化".into());
    }
    let convert = |stamp: Option<super::native_orphans::EffectStamp>| {
        stamp.map(|(length, sha256, modified)| Stamp {
            length,
            sha256,
            modified,
        })
    };
    for (path, original) in &p.dependencies {
        let expected = match prefix.effect_stamp(path) {
            Some((before, after)) => {
                if convert(before) != *original {
                    return Err("项目前序凭据起点不是原始审批".into());
                }
                convert(after)
            }
            None => original.clone(),
        };
        if stamp(path)? != expected {
            return Err("项目依赖不等于原值或已核前序效果".into());
        }
    }
    Ok(())
}
fn restrict_rebased_project(
    ctx: &Context,
    original: &Prepared,
    fresh: &Prepared,
    prefix: &super::native_threads::ApprovedThreadEffects,
) -> Result<()> {
    if fresh.database != original.database
        || fresh
            .dependencies
            .keys()
            .any(|p| !original.dependencies.contains_key(p))
    {
        return Err("前序修复产生未批准的新项目数据来源".into());
    }
    let mut restored = BTreeMap::new();
    for change in &fresh.updates {
        if original.potential_cwds.get(&change.id) != Some(&change.path) {
            return Err("前序修复产生未批准的cwd行或路径".into());
        }
        if original.updates.iter().any(|u| u == change) {
            continue;
        }
        let requested = Path::new(&change.path);
        let requested = if requested.is_absolute() {
            requested.to_owned()
        } else {
            ctx.home.join(requested)
        };
        let restored_source = representations(&requested).iter().any(|p| {
            matches!(prefix.effect_stamp(p), Some((None, Some(_))))
                && original.dependencies.get(p) == Some(&None)
        });
        if !restored_source {
            return Err("新的项目cwd没有来自批准历史恢复的精确字节凭据".into());
        }
        restored.insert(change.id.clone(), change.after.clone());
    }
    let Some(after) = fresh.after.as_deref() else {
        return Ok(());
    };
    fn value(bytes: Option<&[u8]>) -> serde_json::Value {
        bytes
            .and_then(|b| serde_json::from_slice(b).ok())
            .filter(serde_json::Value::is_object)
            .unwrap_or_else(|| serde_json::json!({}))
    }
    fn strings(value: &serde_json::Value, key: &str) -> BTreeSet<String> {
        value
            .get(key)
            .and_then(serde_json::Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(serde_json::Value::as_str)
            .map(core::normalize_codex_path)
            .filter(|s| !s.is_empty())
            .collect()
    }
    let before = value(fresh.before.as_deref());
    let reviewed = value(original.reviewed_state.as_deref());
    let after = value(Some(after));
    let mut roots = strings(&before, "electron-saved-workspace-roots");
    roots.extend(strings(&reviewed, "electron-saved-workspace-roots"));
    roots.extend(restored.values().cloned());
    let mut order = strings(&before, "project-order");
    order.extend(strings(&reviewed, "project-order"));
    order.extend(roots.iter().cloned());
    if !strings(&after, "electron-saved-workspace-roots").is_subset(&roots)
        || !strings(&after, "active-workspace-roots").is_subset(&roots)
        || !strings(&after, "project-order").is_subset(&order)
    {
        return Err("前序修复扩大了未批准的项目入口集合".into());
    }
    let mut hints: BTreeMap<String, Vec<serde_json::Value>> = BTreeMap::new();
    for state in [&before, &reviewed] {
        if let Some(map) = state
            .get("thread-workspace-root-hints")
            .and_then(serde_json::Value::as_object)
        {
            for (id, cwd) in map {
                hints.entry(id.clone()).or_default().push(cwd.clone());
            }
        }
    }
    for (id, cwd) in restored {
        hints
            .entry(id)
            .or_default()
            .push(serde_json::Value::String(cwd));
    }
    if after
        .get("thread-workspace-root-hints")
        .and_then(serde_json::Value::as_object)
        .is_some_and(|map| {
            map.iter()
                .any(|(id, cwd)| !hints.get(id).is_some_and(|allowed| allowed.contains(cwd)))
        })
    {
        return Err("前序修复扩大了未批准的线程项目映射".into());
    }
    let extra = |mut value: serde_json::Value| {
        if let Some(map) = value.as_object_mut() {
            for key in [
                "electron-saved-workspace-roots",
                "project-order",
                "active-workspace-roots",
                "thread-workspace-root-hints",
            ] {
                map.remove(key);
            }
        }
        value
    };
    if extra(after) != extra(before) {
        return Err("项目修复改变了未批准的用户字段".into());
    }
    Ok(())
}
/// Consume the aggregate opaque certificate from auth/config/resync/orphans/threads/
/// ordinary stages. New cwd targets are allowed only for an original empty-cwd row
/// whose previously absent history was restored to exact approved bytes.
pub(super) fn apply_after_approved_prefix(
    ctx: &Context,
    p: &Prepared,
    guard: &super::repair::MutationGuard,
    backup_root: &Path,
    prefix: &super::native_threads::ApprovedThreadEffects,
) -> Result<Outcome> {
    guard.validate(ctx)?;
    validate_prefix(ctx, p, prefix)?;
    let fresh = prepare(ctx)?;
    restrict_rebased_project(ctx, p, &fresh, prefix)?;
    validate_prefix(ctx, p, prefix)?;
    apply_under_manager(ctx, &fresh, guard, backup_root)
}
#[derive(Clone)]
pub(super) struct ApprovedProjectEffects {
    home: PathBuf,
    app_root: PathBuf,
    edges: BTreeMap<
        PathBuf,
        (
            Option<super::native_orphans::EffectStamp>,
            Option<super::native_orphans::EffectStamp>,
        ),
    >,
}
impl std::fmt::Debug for ApprovedProjectEffects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApprovedProjectEffects")
            .field("paths", &self.edges.len())
            .finish()
    }
}
impl ApprovedProjectEffects {
    pub fn paths(&self) -> Vec<PathBuf> {
        self.edges.keys().cloned().collect()
    }
    pub fn effect_stamp(
        &self,
        path: &Path,
    ) -> Option<(
        Option<super::native_orphans::EffectStamp>,
        Option<super::native_orphans::EffectStamp>,
    )> {
        self.edges.get(path).cloned()
    }
    pub fn validate_current(&self, ctx: &Context) -> Result<()> {
        if self.home != ctx.home || self.app_root != ctx.app_root {
            return Err("项目效果凭据目录不匹配".into());
        }
        for (path, (_, after)) in &self.edges {
            if effect_stamp(path)? != *after {
                return Err("已核项目效果发生外部变化".into());
            }
        }
        Ok(())
    }
}
fn effect_stamp(path: &Path) -> Result<Option<super::native_orphans::EffectStamp>> {
    Ok(stamp(path)?.map(|s| (s.length, s.sha256, s.modified)))
}
fn project_sql_oracle(p: &Prepared) -> Result<Option<BTreeSet<String>>> {
    if p.updates.is_empty() {
        return Ok(None);
    }
    let db = p.database.as_deref().ok_or("项目SQL缺少原始数据库")?;
    with_private(db, |copy| {
        let c = Connection::open_with_flags(
            copy,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .map_err(err)?;
        let mut allowed = BTreeSet::from([super::native_orphans::logical_database_digest(&c)?]);
        c.execute_batch("BEGIN").map_err(err)?;
        let candidates = p
            .updates
            .iter()
            .map(|u| (u.id.clone(), u.after.clone()))
            .collect::<Vec<_>>();
        if project::write_cwd_backfill(&c, &candidates).is_ok() {
            if c.execute_batch("COMMIT").is_err() && !c.is_autocommit() {
                c.execute_batch("ROLLBACK").map_err(err)?;
            }
        } else if !c.is_autocommit() {
            c.execute_batch("ROLLBACK").map_err(err)?;
        }
        allowed.insert(super::native_orphans::logical_database_digest(&c)?);
        Ok(Some(allowed))
    })
}
fn certify_project(
    ctx: &Context,
    p: &Prepared,
    allowed: Option<&BTreeSet<String>>,
    published: bool,
) -> Result<ApprovedProjectEffects> {
    let mut current = p.dependencies.clone();
    let mut database_files = BTreeSet::new();
    if let Some(allowed) = allowed {
        let db = p.database.as_deref().ok_or("项目后验缺少原始数据库")?;
        let w = wal(db);
        let before = (stamp(db)?, stamp(&w)?);
        let logical = with_private(db, |copy| {
            let c = Connection::open_with_flags(
                copy,
                OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
            )
            .map_err(err)?;
            super::native_orphans::logical_database_digest(&c)
        })?;
        let after = (stamp(db)?, stamp(&w)?);
        if before != after || !allowed.contains(&logical) {
            return Err("项目数据库变化不等于原值或已核cwd SQL效果".into());
        }
        current.insert(db.to_owned(), after.0);
        current.insert(w.clone(), after.1);
        database_files.extend([db.to_owned(), w]);
    }
    for (path, before) in &p.dependencies {
        if database_files.contains(path) {
            continue;
        }
        let after = stamp(path)?;
        if *path == p.state && published {
            let bytes = p.after.as_deref().ok_or("项目JSON发布缺少批准字节")?;
            if !after.as_ref().is_some_and(|s| {
                s.length == bytes.len() as u64 && s.sha256 == format!("{:x}", Sha256::digest(bytes))
            }) {
                return Err("项目JSON不等于批准发布字节".into());
            }
            current.insert(path.clone(), after);
        } else if after != *before {
            return Err("项目阶段存在未批准的外部依赖变化".into());
        }
    }
    let tuple = |s: Option<Stamp>| s.map(|s| (s.length, s.sha256, s.modified));
    let edges = current
        .into_iter()
        .filter_map(|(path, after)| {
            let before = p.dependencies.get(&path).cloned().unwrap_or_default();
            (before != after).then(|| (path, (tuple(before), tuple(after))))
        })
        .collect();
    Ok(ApprovedProjectEffects {
        home: ctx.home.clone(),
        app_root: ctx.app_root.clone(),
        edges,
    })
}
#[derive(Clone, Debug)]
pub(super) struct Outcome {
    pub success: bool,
    pub cwd_updated: usize,
    pub state_written: bool,
    pub backups: Vec<PathBuf>,
    pub detail: String,
    pub diagnostic: DiagnosticItem,
    pub partial_effects_possible: bool,
    pub effects: Option<ApprovedProjectEffects>,
}
pub(super) fn apply_under_manager(
    ctx: &Context,
    p: &Prepared,
    guard: &super::repair::MutationGuard,
    backup_root: &Path,
) -> Result<Outcome> {
    apply_with_hook(ctx, p, guard, backup_root, || {})
}
fn apply_with_hook(
    ctx: &Context,
    p: &Prepared,
    guard: &super::repair::MutationGuard,
    backup_root: &Path,
    mut after_cwd_commit: impl FnMut(),
) -> Result<Outcome> {
    guard.validate(ctx)?;
    validate(ctx, p)?;
    if ctx.desktop_running || ctx.runtime.as_ref().is_some_and(|r| r.active_requests > 0) {
        return Err("项目修复尚未取得实际写入静默窗口".into());
    }
    let mut outcome = Outcome {
        success: false,
        cwd_updated: 0,
        state_written: false,
        backups: vec![],
        detail: String::new(),
        diagnostic: p.diagnostic.clone(),
        partial_effects_possible: false,
        effects: None,
    };
    let sql_oracle = project_sql_oracle(p)?;
    for (path, expected) in &p.dependencies {
        if stamp(path)? != *expected {
            return Err("项目SQL私有重放期间来源发生变化".into());
        }
    }
    let work = (|| -> Result<()> {
        if !p.updates.is_empty() {
            let db = p.database.as_deref().ok_or("cwd回填缺少实际活跃数据库")?;
            crate::gateway::storage::private_dir(backup_root)?;
            for (path, name) in [
                (db.to_owned(), "project-source.sqlite"),
                (wal(db), "project-source.sqlite-wal"),
            ] {
                if let Some(bytes) = optional(&path, 128 * 1024 * 1024)? {
                    let destination = backup_root.join(name);
                    publication::write_atomic_private_report(&destination, &bytes)
                        .map_err(err)?
                        .acknowledge();
                    outcome.backups.push(destination);
                }
            }
            let mut connection = Connection::open_with_flags(
                db,
                OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
            )
            .map_err(err)?;
            connection
                .busy_timeout(std::time::Duration::from_millis(500))
                .map_err(err)?;
            let tx = connection.transaction().map_err(err)?;
            for u in &p.updates {
                let current: (String, String) = tx
                    .query_row(
                        "SELECT COALESCE(cwd,''),rollout_path FROM threads WHERE id=?1",
                        [&u.id],
                        |r| Ok((r.get(0)?, r.get(1)?)),
                    )
                    .map_err(err)?;
                if current != (u.before.clone(), u.path.clone()) {
                    return Err("cwd回填目标已被外部修改".into());
                }
            }
            let candidates = p
                .updates
                .iter()
                .map(|u| (u.id.clone(), u.after.clone()))
                .collect::<Vec<_>>();
            let count = project::write_cwd_backfill(&tx, &candidates).map_err(err)?;
            tx.commit().map_err(err)?;
            outcome.cwd_updated = count;
            if count > 0 {
                let _ = connection.execute_batch("PRAGMA wal_checkpoint(FULL);");
            }
        }
        after_cwd_commit();
        // Native repair recollects real DB payloads after committing the backfill.
        // Retain the reviewed composition boundary instead of approving unexpected
        // trigger/writer effects as newly requested project targets.
        if let Some(db) = &p.database {
            let (_, pending, actual, _, _) = prepare_database(ctx, db, p.before.as_deref(), true)?;
            if !pending.is_empty() || (p.after.is_some() && actual.after != p.after) {
                return Err("cwd提交后的实际项目来源不等于已审核效果，保留已完成回填".into());
            }
        }
        if let Some(after) = &p.after {
            for (path, before) in &p.dependencies {
                if p.database
                    .as_ref()
                    .is_some_and(|db| path == db || *path == wal(db))
                {
                    continue;
                }
                if stamp(path)? != *before {
                    return Err("cwd提交后项目来源被外部修改，保留已完成回填".into());
                }
            }
            if let Some(before) = &p.before {
                crate::gateway::storage::private_dir(backup_root)?;
                let name = project::backup_filename(
                    now_ms() as i64,
                    &uuid::Uuid::new_v4().simple().to_string(),
                )
                .ok_or("备份时间无效")?;
                let path = backup_root.join(name);
                publication::write_atomic_private_report(&path, before)
                    .map_err(err)?
                    .acknowledge();
                outcome.backups.push(path);
            }
            publication::write_atomic_private_report(&p.state, after)
                .map_err(err)?
                .acknowledge();
            outcome.state_written = true;
            let actual = read_bounded(&p.state, 8 * 1024 * 1024)?;
            let _: serde_json::Value = serde_json::from_slice(&actual).map_err(err)?;
        }
        Ok(())
    })();
    outcome.diagnostic = inspect(ctx);
    outcome.success = work.is_ok() && outcome.diagnostic.status == "ok";
    outcome.partial_effects_possible =
        work.is_err() && (outcome.cwd_updated > 0 || outcome.state_written);
    outcome.detail = match work {
        Err(e) => format!(
            "项目修复未完成：{e}；已提交cwd {} 条、项目JSON发布 {}；已完成效果和备份保留。",
            outcome.cwd_updated, outcome.state_written
        ),
        Ok(()) if outcome.success => format!(
            "原生项目状态修复及真实复检通过：cwd {} 条，roots {}，hints {}。",
            outcome.cwd_updated, p.roots, p.hints
        ),
        Ok(()) => "没有可重建的项目cwd，或修复后项目状态仍不稳定；未标为成功。".into(),
    };
    match certify_project(ctx, p, sql_oracle.as_ref(), outcome.state_written) {
        Ok(proof) => outcome.effects = Some(proof),
        Err(error) => {
            outcome.success = false;
            outcome.partial_effects_possible |= outcome.cwd_updated > 0 || outcome.state_written;
            outcome.detail.push_str(&format!(
                " 后验无法认证：{error}；后继写入不能续签这些变化。"
            ));
        }
    }
    Ok(outcome)
}
// Compatibility read APIs for the older ticket plumbing. New callers own Prepared directly.
pub(super) struct ProjectPlan {
    pub issues: usize,
    pub before: Vec<u8>,
    pub after: Option<Vec<u8>>,
    pub detail: String,
}
pub(super) fn inspect_existing(ctx: &Context, db: &Path) -> Result<ProjectPlan> {
    let p = prepare_for_database(ctx, Some(db.to_owned()))?;
    Ok(ProjectPlan {
        issues: usize::from(p.diagnostic.fixable) + p.updates.len(),
        before: p.before.unwrap_or_default(),
        after: p.after,
        detail: p.diagnostic.detail,
    })
}
pub(super) fn cwd_repairs(ctx: &Context, db: &Path) -> Result<Vec<CwdUpdate>> {
    Ok(prepare_for_database(ctx, Some(db.to_owned()))?.updates)
}
