//! Native thread maintenance adapter. Planning/diagnosis opens ONLY private SQLite copies.
use super::*;
use crate::gateway::storage::{atomic_private, private_dir};
use atelier_maintenance_core::{
    state_db,
    threads::{self as core, consistency, repair as native},
};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    time::SystemTime,
};
#[path = "native_threads_session.rs"]
mod session;
#[cfg(test)]
pub(super) use session::apply_item_with_hook;
pub(super) use session::{
    absorb_global, absorb_ordinary, absorb_project, apply_item, begin_reviewed,
    begin_reviewed_after_batch_prefix, ApprovedThreadEffects, Session,
};
const IDS: [&str; 6] = [
    "thread_missing_rollouts",
    "legacy_migrated_threads",
    "thread_provider_consistency",
    "thread_model_catalog_consistency",
    "thread_response_item_ids",
    "thread_poisoned_instructions",
];
pub(super) fn handles(id: &str) -> bool {
    IDS.contains(&id)
}
#[derive(Clone)]
enum Body {
    Missing(native::MissingRolloutRepairPlan),
    Write(native::ThreadWriteRepairPlan),
    Legacy(native::LegacyCleanupPlan),
    Response(native::ResponseIdRepairPlan),
    Instructions(native::PoisonedInstructionRepairPlan),
    DeferredProvider,
    Noop,
}
#[derive(Clone, PartialEq, Eq)]
struct Fingerprint {
    len: u64,
    sha256: String,
    modified: Option<SystemTime>,
}
fn fingerprint(path: &Path) -> Result<Option<Fingerprint>> {
    let m = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err("无法核对线程维护依赖文件".into()),
    };
    if !m.is_file() || m.file_type().is_symlink() {
        return Err("线程维护依赖不是普通文件，已保留".into());
    }
    let mut f = fs::File::open(path).map_err(|_| "线程维护依赖无法读取")?;
    let mut hash = Sha256::new();
    let mut buffer = [0u8; 65536];
    let mut len = 0;
    loop {
        let n = f.read(&mut buffer).map_err(|_| "线程维护依赖读取失败")?;
        if n == 0 {
            break;
        }
        hash.update(&buffer[..n]);
        len += n as u64;
    }
    let after = fs::symlink_metadata(path).map_err(|_| "线程维护依赖读取期间变化")?;
    if len != m.len() || m.len() != after.len() || m.modified().ok() != after.modified().ok() {
        return Err("线程维护依赖读取期间变化，请重新预览".into());
    }
    Ok(Some(Fingerprint {
        len,
        sha256: format!("{:x}", hash.finalize()),
        modified: m.modified().ok(),
    }))
}
#[derive(Clone, Default)]
struct Scope {
    databases: BTreeSet<PathBuf>,
    rows: BTreeSet<(PathBuf, String)>,
    files: BTreeSet<PathBuf>,
}
impl Scope {
    fn extend(&mut self, other: &Self) {
        self.databases.extend(other.databases.iter().cloned());
        self.rows.extend(other.rows.iter().cloned());
        self.files.extend(other.files.iter().cloned());
    }
    fn contains(&self, other: &Self) -> bool {
        other.databases.is_subset(&self.databases)
            && other.rows.is_subset(&self.rows)
            && other.files.is_subset(&self.files)
    }
}
#[derive(Clone)]
pub(super) struct PreparedAction {
    id: String,
    home: PathBuf,
    app_root: PathBuf,
    backup_root: PathBuf,
    route_enabled: bool,
    databases: Vec<PathBuf>,
    dependencies: BTreeMap<PathBuf, Option<Fingerprint>>,
    scope: Scope,
    body: Body,
}
#[derive(Clone, Debug)]
pub(super) struct Outcome {
    pub id: String,
    pub success: bool,
    pub changed: usize,
    pub detail: String,
    pub backups: Vec<PathBuf>,
    pub diagnostic: DiagnosticItem,
    pub warnings: Vec<String>,
    pub partial_effects_possible: bool,
}
impl PreparedAction {
    pub(super) fn id(&self) -> &str {
        &self.id
    }
    pub(super) fn changes_provider_target(&self) -> bool {
        match &self.body {
            Body::Missing(p) => !p.changes.is_empty(),
            Body::Write(p) => {
                p.kind == native::WriteRepairKind::Provider
                    && (!p.db_changes.is_empty() || !p.files.is_empty())
            }
            Body::Legacy(p) => p
                .databases
                .iter()
                .any(|d| !d.expected_legacy_rows.is_empty() || !d.files.is_empty()),
            Body::DeferredProvider => true,
            _ => false,
        }
    }
}
struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Result<Self> {
        let path =
            std::env::temp_dir().join(format!("atelier-native-threads-{}", uuid::Uuid::new_v4()));
        private_dir(&path)?;
        Ok(Self(path))
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
#[derive(Clone)]
struct Input {
    databases: Vec<PathBuf>,
    route_enabled: bool,
    catalog: PathBuf,
    relay: PathBuf,
    trusted: PathBuf,
}
fn input(ctx: &Context) -> Result<Input> {
    let c = inspect::config(ctx)?;
    let raw = if ctx.config().exists() {
        Some(
            String::from_utf8(read_bounded(&ctx.config(), 1024 * 1024)?)
                .map_err(|_| "配置不是UTF-8")?,
        )
    } else {
        None
    };
    let configured = state_db::configured_sqlite_home(
        raw.as_deref(),
        ctx.sqlite_home_env.as_deref(),
        ctx.working_directory.as_deref(),
    );
    let directories = if let Some(d) = configured {
        vec![d]
    } else {
        vec![ctx.home.join("sqlite"), ctx.home.clone()]
    };
    let mut databases = vec![];
    for d in directories {
        if let Some(p) = state_db::highest_state_db(&d) {
            if !databases.contains(&p) {
                databases.push(p)
            }
        }
    }
    let relay = ctx.home.join("codexmate/relay");
    let catalog = c
        .get("model_catalog_json")
        .and_then(|v| v.as_str())
        .map(|p| {
            let p = PathBuf::from(p);
            if p.is_absolute() {
                p
            } else {
                ctx.home.join(p)
            }
        })
        .unwrap_or_else(|| relay.join("codex_router_catalog.json"));
    Ok(Input {
        databases,
        route_enabled: residual_cleanup::route_enabled(ctx, &c)?,
        catalog,
        relay,
        trusted: ctx.home.join("codexmate"),
    })
}
fn io_path(home: &Path, p: &str) -> PathBuf {
    let p = PathBuf::from(p);
    if p.is_absolute() {
        p
    } else {
        home.join(p)
    }
}
fn representations(path: &Path) -> Vec<PathBuf> {
    let plain = if path.to_string_lossy().ends_with(".jsonl.zst") {
        path.with_extension("")
    } else {
        path.into()
    };
    let mut paths = vec![plain.clone()];
    if plain.extension().is_some_and(|e| e == "jsonl") {
        paths.push(plain.with_extension("jsonl.zst"));
    }
    paths
}
struct Projection {
    source: PathBuf,
    copy: PathBuf,
    raw: BTreeMap<String, Option<String>>,
    legacy_paths: Vec<String>,
}
fn with_projections<T>(
    ctx: &Context,
    dbs: &[PathBuf],
    read: &mut impl FnMut(&[Projection]) -> Result<T>,
) -> Result<T> {
    fn next<T>(
        ctx: &Context,
        dbs: &[PathBuf],
        frames: &mut Vec<Projection>,
        read: &mut impl FnMut(&[Projection]) -> Result<T>,
    ) -> Result<T> {
        let n = frames.len();
        if n == dbs.len() {
            return read(frames);
        }
        snapshot::with_observed_path(&dbs[n], |_, baseline| {
            // Path normalization is private to this projection, never to the shared baseline.
            // Include committed WAL pages even outside a shared observation scope.
            let scratch = Scratch::new()?;
            let projected = scratch.0.join("projection.sqlite");
            baseline
                .execute("VACUUM INTO ?1", [projected.to_string_lossy().as_ref()])
                .map_err(err)?;
            let copy = projected.as_path();
            let connection = inspect::open_db(copy, true)?;
            let mut stmt = connection
                .prepare("SELECT id,rollout_path FROM threads WHERE COALESCE(archived,0)=0")
                .map_err(|_| "会话索引缺少线程路径字段")?;
            let raw = stmt
                .query_map([], |r| {
                    Ok((r.get::<_, String>(0)?, r.get::<_, Option<String>>(1)?))
                })
                .map_err(err)?
                .collect::<std::result::Result<BTreeMap<_, _>, _>>()
                .map_err(err)?;
            drop(stmt);
            // The copied schema must not execute application triggers while projecting paths.
            let triggers = {
                let mut st = connection
                    .prepare("SELECT name FROM sqlite_master WHERE type='trigger'")
                    .map_err(err)?;
                let names = st
                    .query_map([], |r| r.get::<_, String>(0))
                    .map_err(err)?
                    .collect::<std::result::Result<Vec<_>, _>>()
                    .map_err(err)?;
                names
            };
            for name in triggers {
                connection
                    .execute_batch(&format!("DROP TRIGGER \"{}\"", name.replace('\"', "\"\"")))
                    .map_err(err)?;
            }

            let legacy_paths = {
                let mut st=connection.prepare("SELECT DISTINCT rollout_path FROM threads WHERE rollout_path IS NOT NULL AND rollout_path <> '' AND COALESCE(archived,0)=0").map_err(err)?;
                let v = st
                    .query_map([], |r| r.get::<_, String>(0))
                    .map_err(err)?
                    .collect::<std::result::Result<Vec<_>, _>>()
                    .map_err(err)?;
                v
            };
            for (id, path) in &raw {
                if let Some(p) = path {
                    if !p.is_empty() && !Path::new(p).is_absolute() {
                        connection
                            .execute(
                                "UPDATE threads SET rollout_path=?2 WHERE id=?1",
                                rusqlite::params![id, io_path(&ctx.home, p).to_string_lossy()],
                            )
                            .map_err(err)?;
                    }
                }
            }
            frames.push(Projection {
                source: dbs[n].clone(),
                copy: copy.into(),
                raw,
                legacy_paths,
            });
            let result = next(ctx, dbs, frames, read);
            frames.pop();
            result
        })
    }
    next(ctx, dbs, &mut vec![], read)
}
fn source_db(projections: &[Projection], p: &Path) -> Result<PathBuf> {
    projections
        .iter()
        .find(|d| d.copy == p)
        .map(|d| d.source.clone())
        .ok_or_else(|| "私有数据库路径没有原库映射".into())
}
fn raw_path(projections: &[Projection], db: &Path, id: &str) -> Result<Option<String>> {
    projections
        .iter()
        .find(|d| d.copy == db || d.source == db)
        .and_then(|d| d.raw.get(id))
        .cloned()
        .ok_or_else(|| "线程路径映射已缺失".into())
}
fn map_file(projections: &[Projection], file: &mut native::RolloutFileChange) -> Result<()> {
    for db in &mut file.databases {
        *db = source_db(projections, db)?;
    }
    Ok(())
}
fn remap(body: &mut Body, projections: &[Projection], ctx: &Context, input: &Input) -> Result<()> {
    match body {
        Body::Missing(p) => {
            for c in &mut p.changes {
                c.recorded_path = raw_path(projections, &c.db, &c.id)?;
                c.db = source_db(projections, &c.db)?;
            }
            p.databases = input.databases.clone();
            p.rollout_base = Some(ctx.home.clone());
        }
        Body::Write(p) => {
            for c in &mut p.db_changes {
                c.expected_rollout = raw_path(projections, &c.db, &c.id)?;
                c.db = source_db(projections, &c.db)?;
            }
            for f in &mut p.files {
                map_file(projections, f)?;
            }
            if p.catalog_directory.is_some() {
                p.catalog_directory = input.catalog.parent().map(Path::to_path_buf);
            }
        }
        Body::Legacy(p) => {
            for d in &mut p.databases {
                let frame = projections
                    .iter()
                    .find(|p| p.copy == d.db)
                    .ok_or("旧线程数据库映射缺失")?;
                d.expected_rollout_paths = frame.legacy_paths.clone();
                d.db = frame.source.clone();
                for f in &mut d.files {
                    map_file(projections, f)?;
                }
            }
        }
        Body::Response(p) => {
            for f in &mut p.files {
                map_file(projections, &mut f.file)?;
            }
        }
        Body::Instructions(p) => {
            for f in &mut p.rollouts {
                map_file(projections, &mut f.file)?;
            }
            p.databases = input.databases.clone();
            p.rollout_base = Some(ctx.home.clone());
        }
        Body::Noop | Body::DeferredProvider => (),
    };
    Ok(())
}
fn with_catalog<T>(input: &Input, read: impl FnOnce(&Path) -> Result<T>) -> Result<T> {
    let scratch = Scratch::new()?;
    let bytes =
        read_bounded(&input.catalog, 32 * 1024 * 1024).map_err(|_| "无法读取已配置的模型目录")?;
    atomic_private(&scratch.0.join("codex_router_catalog.json"), &bytes)?;
    read(&scratch.0)
}
fn core_body(
    ctx: &Context,
    id: &str,
    input: &Input,
    backup: &Path,
    projections: &[Projection],
    defer: bool,
) -> Result<Body> {
    let copies = projections
        .iter()
        .map(|p| p.copy.clone())
        .collect::<Vec<_>>();
    let result = match id {
        "thread_missing_rollouts" => {
            native::plan_missing_rollout_repair(&copies, &input.trusted, backup)
                .map(Body::Missing)
                .map_err(err)
        }
        "thread_provider_consistency" => match native::plan_provider_convergence(
            &copies,
            if input.route_enabled {
                native::ProviderTarget::Router
            } else {
                native::ProviderTarget::Official
            },
            backup,
        ) {
            Ok(p) => Ok(Body::Write(p)),
            Err(e) if defer && e.0.contains("ROUTER_HISTORY_REPAIR_REQUIRED") => {
                Ok(Body::DeferredProvider)
            }
            Err(e) => Err(e.to_string()),
        },
        "thread_model_catalog_consistency" => {
            if input.route_enabled {
                with_catalog(input, |dir| {
                    native::plan_model_catalog_convergence_with_journal_directory(
                        &copies,
                        dir,
                        &input.relay,
                        backup,
                    )
                    .map(Body::Write)
                    .map_err(err)
                })
            } else {
                Ok(Body::Noop)
            }
        }
        "legacy_migrated_threads" => {
            if input.route_enabled {
                Err(
                    "路由启用时旧版残留仅检测；原生规则不支持自动清理，请先关闭路由后重新预览"
                        .into(),
                )
            } else {
                native::plan_legacy_provider_cleanup(&copies, backup)
                    .map(Body::Legacy)
                    .map_err(err)
            }
        }
        "thread_response_item_ids" => native::plan_response_item_id_repair(&copies, backup)
            .map(Body::Response)
            .map_err(err),
        "thread_poisoned_instructions" => {
            native::plan_poisoned_instruction_repair(&copies, &input.relay)
                .map(Body::Instructions)
                .map_err(err)
        }
        _ => Err("不是已恢复的线程维护项目".into()),
    };
    let mut body = result?;
    remap(&mut body, projections, ctx, input)?;
    if let Body::Instructions(p) = &mut body {
        p.backup_directory = backup.join("guardian-prompt-fix-backup");
        for change in &mut p.rollouts {
            change.backup_path = p
                .backup_directory
                .join(change.backup_path.file_name().ok_or("指令备份文件名无效")?);
        }
    }
    Ok(body)
}
fn signature(body: &Body) -> Result<Vec<u8>> {
    match body {
        Body::Missing(p) => serde_json::to_vec(p),
        Body::Write(p) => serde_json::to_vec(p),
        Body::Legacy(p) => serde_json::to_vec(p),
        Body::Response(p) => serde_json::to_vec(p),
        Body::Instructions(p) => serde_json::to_vec(p),
        Body::DeferredProvider => return Ok(b"deferred-provider".to_vec()),
        Body::Noop => return Ok(b"noop".to_vec()),
    }
    .map_err(err)
}
fn prepare_internal(ctx: &Context, id: &str, backup: &Path, defer: bool) -> Result<PreparedAction> {
    if !handles(id) {
        return Err("不支持此线程维护项目".into());
    }
    let input = input(ctx)?;
    let mut dependencies = BTreeMap::new();
    for db in &input.databases {
        for p in [db.clone(), PathBuf::from(format!("{}-wal", db.display()))] {
            dependencies.insert(p.clone(), fingerprint(&p)?);
        }
    }
    for path in [
        ctx.config(),
        input.catalog.clone(),
        input.relay.join("router_no_account_model_restore.json"),
    ] {
        if path != ctx.config()
            && (id != "thread_model_catalog_consistency" || !input.route_enabled)
        {
            continue;
        }
        dependencies.insert(path.clone(), fingerprint(&path)?);
    }
    let mut scope = Scope {
        databases: input.databases.iter().cloned().collect(),
        ..Default::default()
    };
    let body = with_projections(ctx, &input.databases, &mut |projections| {
        for projection in projections {
            for (id, p) in &projection.raw {
                scope.rows.insert((projection.source.clone(), id.clone()));
                let _ = p;
            }
        }
        let copies = projections
            .iter()
            .map(|p| p.copy.clone())
            .collect::<Vec<_>>();
        let mut observed = BTreeSet::new();
        match id {
            "thread_missing_rollouts" | "thread_provider_consistency" => {
                for db in &copies {
                    let c = rusqlite::Connection::open_with_flags(
                        db,
                        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
                    )
                    .map_err(err)?;
                    for row in core::collect_provider_convergence_targets(&c).map_err(err)? {
                        if let Some(p) = row.rollout_path {
                            observed.insert(PathBuf::from(p));
                        }
                    }
                }
            }
            "thread_model_catalog_consistency" if input.route_enabled => {
                for db in &copies {
                    let c = rusqlite::Connection::open_with_flags(
                        db,
                        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
                    )
                    .map_err(err)?;
                    for row in consistency::collect_router_model_targets(&c).map_err(err)? {
                        if let Some(p) = row.rollout_path {
                            observed.insert(PathBuf::from(p));
                        }
                    }
                }
            }
            "legacy_migrated_threads" => {
                for frame in projections {
                    for p in &frame.legacy_paths {
                        observed.insert(io_path(&ctx.home, p));
                    }
                }
            }
            "thread_response_item_ids" | "thread_poisoned_instructions" => {
                for row in core::collect_diagnostic_main_threads(&copies).map_err(err)? {
                    if let Some(p) = row.rollout_path {
                        observed.insert(PathBuf::from(p));
                    }
                }
                if id == "thread_poisoned_instructions" {
                    observed.extend(
                        native::collect_active_instruction_rollout_paths(&copies)
                            .into_iter()
                            .take(50),
                    );
                }
            }
            _ => (),
        }
        for path in observed {
            for p in representations(&path) {
                scope.files.insert(p.clone());
                dependencies.insert(p.clone(), fingerprint(&p)?);
            }
        }
        core_body(ctx, id, &input, backup, projections, defer)
    })?;
    let mut add = |p: &Path| -> Result<()> {
        scope.files.insert(p.into());
        dependencies.insert(p.into(), fingerprint(p)?);
        Ok(())
    };
    if let Body::Legacy(p) = &body {
        for db in &p.databases {
            for (id, _) in &db.expected_legacy_rows {
                scope.rows.insert((db.db.clone(), id.clone()));
            }
        }
    }
    match &body {
        Body::Missing(p) => {
            for c in &p.changes {
                if let native::MissingRolloutAction::Restore {
                    source,
                    destination,
                } = &c.action
                {
                    add(source)?;
                    for p in representations(destination) {
                        add(&p)?;
                    }
                }
            }
        }
        Body::Write(p) => {
            if let Some(j) = &p.model_restore_journal {
                add(&j.path)?;
            }
        }
        Body::Instructions(p) => {
            if let Some(r) = &p.reference {
                add(&r.physical_path)?;
            }
            for s in &p.snapshots {
                add(&s.path)?;
            }
        }
        _ => (),
    };
    for (p, f) in &dependencies {
        if fingerprint(p)? != *f {
            return Err("线程维护依赖在预览期间变化，请重试".into());
        }
    }
    Ok(PreparedAction {
        id: id.into(),
        home: ctx.home.clone(),
        app_root: ctx.app_root.clone(),
        backup_root: backup.into(),
        route_enabled: input.route_enabled,
        databases: input.databases,
        dependencies,
        scope,
        body,
    })
}
pub(super) fn prepare(ctx: &Context, id: &str) -> Result<PreparedAction> {
    prepare_with_backup(ctx, id, &ctx.app_root.join("maintenance/thread-repairs"))
}
pub(super) fn prepare_with_backup(
    ctx: &Context,
    id: &str,
    backup_root: &Path,
) -> Result<PreparedAction> {
    prepare_internal(ctx, id, backup_root, false)
}
pub(super) fn preview_lines(plan: &PreparedAction) -> Vec<String> {
    let (rows, files, extra) = match &plan.body {
        Body::Missing(p) => (
            p.changes
                .iter()
                .filter(|c| matches!(c.action, native::MissingRolloutAction::Archive))
                .count(),
            p.changes
                .iter()
                .filter(|c| matches!(c.action, native::MissingRolloutAction::Restore { .. }))
                .count(),
            "恢复有身份凭据的历史备份，隔离无法恢复的活跃线程",
        ),
        Body::Write(p) => (
            p.db_changes.len(),
            p.files.len(),
            if p.kind == native::WriteRepairKind::Model {
                "按实际模型目录收敛数据库模型及历史元数据"
            } else {
                "按路由状态收敛首尾元数据、原生设置和数据库Provider"
            },
        ),
        Body::Legacy(p) => (
            p.databases
                .iter()
                .map(|d| d.expected_legacy_rows.len())
                .sum(),
            p.databases.iter().map(|d| d.files.len()).sum(),
            "归一旧版Provider与大小写残留，保留现代中转及外部Provider",
        ),
        Body::Response(p) => (
            0,
            p.files.len(),
            "只替换已验证的历史工具项ID字面量，保存SHA256恢复备份",
        ),
        Body::Instructions(p) => (
            0,
            p.rollouts.len() + p.snapshots.len(),
            "修复首条系统指令元数据及迁移快照，不展示模板或会话正文",
        ),
        Body::DeferredProvider => (
            0,
            0,
            "先恢复缺失历史，再在已批准的线程范围内计算Provider收敛",
        ),
        Body::Noop => (0, 0, "当前状态无需执行此项"),
    };
    vec![
        format!("{extra}；涉及数据库行 {rows}、文件 {files}。"),
        "执行前核对源索引/WAL及文件依赖，执行后重新读取真实数据库和历史校验结果。".into(),
    ]
}
fn validate_context(ctx: &Context, p: &PreparedAction) -> Result<()> {
    let current = input(ctx)?;
    if ctx.home != p.home
        || ctx.app_root != p.app_root
        || current.databases != p.databases
        || current.route_enabled != p.route_enabled
    {
        return Err("维护上下文或数据库集合已变化，请重新预览".into());
    }
    Ok(())
}
fn validate_initial(ctx: &Context, p: &PreparedAction) -> Result<()> {
    validate_context(ctx, p)?;
    for (path, prior) in &p.dependencies {
        if fingerprint(path)? != *prior {
            return Err("线程数据库、WAL或历史依赖已被外部更新，请重新预览".into());
        }
    }
    Ok(())
}
fn diagnostic(
    id: &str,
    status: &str,
    label: &str,
    detail: String,
    fixable: bool,
) -> DiagnosticItem {
    DiagnosticItem {
        id: id.into(),
        group: "runtime".into(),
        status: status.into(),
        label: label.into(),
        detail,
        fixable,
    }
}
fn adapt(item: core::DiagnosticItem) -> DiagnosticItem {
    diagnostic(
        &item.id,
        &item.status,
        &item.title,
        item.detail.unwrap_or_default(),
        item.fixable,
    )
}
fn inspect_one(ctx: &Context, id: &str) -> Result<DiagnosticItem> {
    let input = input(ctx)?;
    with_projections(ctx, &input.databases, &mut |projections| {
        inspect_projected(ctx, id, &input, projections)
    })
}
fn inspect_projected(
    ctx: &Context,
    id: &str,
    input: &Input,
    projections: &[Projection],
) -> Result<DiagnosticItem> {
    let copies = projections
        .iter()
        .map(|p| p.copy.clone())
        .collect::<Vec<_>>();
    match id {
        "thread_missing_rollouts" => Ok(adapt(core::check_thread_missing_rollouts(
            &core::inspect_missing_rollouts_for_convergence(&copies, &input.trusted),
        ))),
        "thread_provider_consistency" => Ok(adapt(
            consistency::check_thread_provider_consistency_with_cache(
                &copies,
                input.route_enabled,
                &mut consistency::DiagnosticRolloutProviderCache::default(),
            ),
        )),
        "thread_model_catalog_consistency" => {
            if input.route_enabled {
                with_catalog(&input, |dir| {
                    Ok(adapt(
                        consistency::check_thread_model_catalog_consistency_from_sources(
                            &copies, dir, true,
                        ),
                    ))
                })
            } else {
                Ok(adapt(
                    consistency::check_thread_model_catalog_consistency_from_sources(
                        &copies,
                        Path::new(""),
                        false,
                    ),
                ))
            }
        }
        "legacy_migrated_threads" => {
            let v = native::inspect_legacy_migrated_threads(&copies, input.route_enabled);
            Ok(diagnostic(
                id,
                if !v.errors.is_empty() {
                    "warning"
                } else if !v.ids.is_empty() {
                    "error"
                } else {
                    "ok"
                },
                "无旧版线程迁移残留",
                format!(
                    "旧版残留活跃主线程 {} 条；读取错误 {} 项。{}",
                    v.ids.len(),
                    v.errors.len(),
                    if input.route_enabled {
                        "路由启用时保留合法aimai1，旧版残留仅检测。"
                    } else {
                        "路由关闭时可清理旧版内部Provider。"
                    }
                ),
                !v.ids.is_empty() && v.errors.is_empty() && !input.route_enabled,
            ))
        }
        "thread_response_item_ids" => {
            let rows = core::collect_diagnostic_main_threads(&copies).map_err(err)?;
            let mut paths = BTreeSet::new();
            let mut count = 0;
            let mut errors = 0;
            for row in rows {
                let Some(path) = row.rollout_path else {
                    errors += 1;
                    continue;
                };
                if !paths.insert(path.clone()) {
                    continue;
                }
                match native::count_invalid_response_item_ids(Path::new(&path)) {
                    Ok(n) => count += n,
                    Err(_) => errors += 1,
                }
            }
            let writable = count > 0
                && errors == 0
                && native::plan_response_item_id_repair(
                    &copies,
                    &ctx.app_root.join("maintenance/thread-repairs"),
                )
                .is_ok();
            let (status, label) = if errors > 0 {
                ("warning", "无法完整检查线程调用项 ID")
            } else if count > 0 {
                ("error", "线程调用项 ID 存在旧版前缀")
            } else {
                ("ok", "线程调用项 ID 健康（数据库不存在，跳过）")
            };
            Ok(diagnostic(
                id,
                status,
                label,
                format!("可识别的旧版工具项ID {count} 个；无法完整检查的历史 {errors} 项。"),
                count > 0 && writable,
            ))
        }
        "thread_poisoned_instructions" => {
            let p = native::plan_poisoned_instruction_repair(&copies, &input.relay).map_err(err)?;
            let count = p.rollouts.len()
                + p.unwritable_issues
                + p.snapshots.iter().map(|s| s.values_updated).sum::<usize>();
            // 原版只有「健康」一个 ok 常量；异常与「读不全」必须换 label，
            // 否则会出现 label 说健康、状态却是 warning/error 的自相矛盾。
            let (status, label) = if count > 0 {
                ("error", "线程系统提示存在需要修复的记录")
            } else if !p.warnings.is_empty() {
                ("warning", "无法完整检查线程系统提示")
            } else {
                ("ok", "线程系统提示健康（数据库不存在，跳过）")
            };
            Ok(diagnostic(
                id,
                status,
                label,
                format!(
                    "待修复系统指令或迁移快照 {count} 项；受保护布局 {} 项；未完整读取 {} 项。",
                    p.unwritable_issues,
                    p.warnings.len()
                ),
                !p.rollouts.is_empty() || !p.snapshots.is_empty(),
            ))
        }
        _ => Err("不是已恢复的线程诊断项目".into()),
    }
}
#[cfg(test)]
pub(super) fn inspect_independently(ctx: &Context) -> Vec<DiagnosticItem> {
    IDS.iter()
        .map(|id| inspect_one(ctx, id).unwrap_or_else(|_| unavailable(id)))
        .collect()
}
/// 原版每个 check 在自己的 ok/error 分支里写 label；读取失败时不能借用别的条目的标签。
/// 这里按 id 给出与原版一致的「无法完整读取」表述。
fn unavailable(id: &str) -> DiagnosticItem {
    let label = match id {
        "thread_missing_rollouts" => "无法确认线程 rollout 是否存在",
        "thread_provider_consistency" => "无法完整读取线程数据库",
        "thread_poisoned_instructions" => "无法完整读取会话基础指令",
        "thread_response_item_ids" => "无法完整检查线程调用项 ID",
        "thread_model_catalog_consistency" => "无法完整读取线程模型目录",
        "thread_history_integrity" => "无法完整检查线程历史结构",
        "legacy_migrated_threads" => "旧版线程迁移（数据库不存在，跳过）",
        _ => "无法完整读取线程数据",
    };
    diagnostic(
        id,
        "ok",
        label,
        "无法建立稳定的私有索引快照，结果未标为健康。".into(),
        false,
    )
}
/// Six checks share one stable projection, never cached across mutation stages.
pub(super) fn inspect(ctx: &Context) -> Vec<DiagnosticItem> {
    let result = input(ctx).and_then(|input| {
        with_projections(ctx, &input.databases, &mut |projections| {
            Ok(IDS
                .iter()
                .map(|id| {
                    inspect_projected(ctx, id, &input, projections)
                        .unwrap_or_else(|_| unavailable(id))
                })
                .collect())
        })
    });
    result.unwrap_or_else(|_| IDS.iter().map(|id| unavailable(id)).collect())
}
fn execute(ctx: &Context, p: &PreparedAction) -> Outcome {
    let applied: std::result::Result<native::RepairSummary, String> = match &p.body {
        Body::Missing(plan) => native::apply_missing_rollout_repair_with_verifier(plan, |plan| {
            with_projections(ctx, &plan.databases, &mut |frames| {
                let copies = frames.iter().map(|f| f.copy.clone()).collect::<Vec<_>>();
                Ok(core::inspect_missing_rollouts_for_convergence(
                    &copies,
                    &plan.trusted_backup_root,
                ))
            })
            .map_err(native::RepairError)
        })
        .map_err(err),
        Body::Write(plan) => native::apply_thread_write_repair(plan).map_err(err),
        Body::Legacy(plan) => native::apply_legacy_provider_cleanup(plan).map_err(err),
        Body::Response(plan) => native::apply_response_item_id_repair(plan).map_err(err),
        Body::Instructions(plan) => {
            native::apply_poisoned_instruction_repair_with_verifier(plan, |plan| {
                with_projections(ctx, &plan.databases, &mut |frames| {
                    let copies = frames.iter().map(|f| f.copy.clone()).collect::<Vec<_>>();
                    native::plan_poisoned_instruction_repair(&copies, &plan.relay_directory)
                        .map_err(err)
                })
                .map_err(native::RepairError)
            })
            .map(|r| r.summary)
            .map_err(err)
        }
        Body::Noop => Ok(native::RepairSummary::default()),
        Body::DeferredProvider => Err("依赖修复尚未完成".into()),
    };
    let check = inspect_one(ctx, &p.id).unwrap_or_else(|_| {
        diagnostic(
            &p.id,
            "warning",
            "线程维护复检",
            "修复后无法建立稳定快照，未标为成功。".into(),
            false,
        )
    });
    let success = applied.is_ok() && check.status == "ok";
    let (summary, failed) = match applied {
        Ok(summary) => (summary, false),
        Err(_) => (native::RepairSummary::default(), true),
    };
    Outcome {
        id: p.id.clone(),
        success,
        changed: summary.db_updated + summary.rollout_updated,
        detail: if failed {
            "原生修复返回错误；已执行实际复检，请保留备份并重新检测。".into()
        } else if success {
            format!("原生修复及数据库/历史复检通过；更新索引 {} 行、历史 {} 项、恢复 {} 项、隔离 {} 项。",summary.db_updated,summary.rollout_updated,summary.restored,summary.archived)
        } else {
            "修复后仍有未解决或未完整读取的项目，未标为成功。".into()
        },
        backups: summary.backups,
        diagnostic: check,
        warnings: if summary.warnings.is_empty() {
            vec![]
        } else {
            vec![format!(
                "原生修复报告 {} 项历史保护或读写提示；结果已经真实复检。",
                summary.warnings.len()
            )]
        },
        partial_effects_possible: failed,
    }
}
/// Caller owns global/thread mutation gates. A stale reviewed plan never mutates its source.
pub(super) fn apply(ctx: &Context, plan: &PreparedAction) -> Result<Outcome> {
    validate_initial(ctx, plan)?;
    let fresh = prepare_with_backup(ctx, &plan.id, &plan.backup_root)?;
    if signature(&fresh.body)? != signature(&plan.body)? {
        return Err("原生线程修复范围已变化，请重新预览".into());
    }
    Ok(execute(ctx, &fresh))
}
/// Reuse the native producer before publication. A selected zero-effect plan is
/// not evidence that a known final Provider failure can be repaired.
pub(super) fn review_provider_target(
    ctx: &Context,
    target: &str,
    changes_provider: bool,
) -> Result<()> {
    super::snapshot::with_observation(|| {
        review_provider_target_observed(ctx, target, changes_provider)
    })
}
fn review_provider_target_observed(
    ctx: &Context,
    target: &str,
    changes_provider: bool,
) -> Result<()> {
    let input = input(ctx)?;
    with_projections(ctx, &input.databases, &mut |projections| {
        let copies = projections
            .iter()
            .map(|p| p.copy.clone())
            .collect::<Vec<_>>();
        let inspected = consistency::inspect_thread_provider_convergence_with_reader(
            &copies,
            target,
            &mut consistency::DiagnosticRolloutProviderCache::default(),
        );
        if !inspected.errors.is_empty() {
            return Err(format!(
                "写入前无法完整核对Provider目标；未开始修复：{}",
                inspected.errors.join("；")
            ));
        }
        if (inspected.db_mismatch > 0
            || inspected.rollout_mismatch > 0
            || inspected.missing_rollouts > 0)
            && !changes_provider
        {
            return Err("现有Provider目标未满足，但已批准计划没有对应的数据库或历史改动；未开始修复，请重新检测".into());
        }
        // Native collectors deliberately exclude Atelier IDs. Do not silently
        // promise a NativeOff migration for rows this adapter cannot rewrite.
        for row in core::collect_diagnostic_main_threads(&copies).map_err(err)? {
            if row
                .provider
                .as_deref()
                .is_some_and(super::router_intent::owns_provider)
            {
                return Err("仍有Atelier受管线程，原生Provider计划不包含这些行；未开始修复，需先提供对应Gateway线程迁移计划".into());
            }
        }
        Ok(())
    })
}
/// Missing histories must precede provider convergence. Remaining order stays user-selected.
pub(super) fn prepare_sequence(
    ctx: &Context,
    ids: &[String],
    backup_root: &Path,
) -> Result<Vec<PreparedAction>> {
    let mut ordered = vec![];
    if ids.iter().any(|i| i == "thread_missing_rollouts") {
        ordered.push("thread_missing_rollouts".to_owned())
    }
    for id in ids {
        if handles(id) && !ordered.contains(id) {
            ordered.push(id.clone())
        }
    }
    let defer = ordered.iter().any(|i| i == "thread_missing_rollouts");
    ordered
        .iter()
        .map(|id| prepare_internal(ctx, id, backup_root, defer))
        .collect()
}
/// Coordinator calls this before any other file/config publication.
pub(super) fn validate_sequence(ctx: &Context, plans: &[PreparedAction]) -> Result<()> {
    for p in plans {
        validate_initial(ctx, p)?;
        let fresh = prepare_internal(
            ctx,
            &p.id,
            &p.backup_root,
            matches!(p.body, Body::DeferredProvider),
        )?;
        if signature(&fresh.body)? != signature(&p.body)? {
            return Err("线程原生修复范围已变化，请重新预览".into());
        }
    }
    Ok(())
}
/// Verify ALL initial inputs once, then reprepare each item against prior approved effects.
/// New DBs, rows or target files outside the approved union are refused, never silently added.
pub(super) fn apply_sequence(ctx: &Context, plans: &[PreparedAction]) -> Result<Vec<Outcome>> {
    validate_sequence(ctx, plans)?;
    execute_sequence(ctx, plans)
}
/// The outer repair has validated every original ticket before the first orphan write.
/// This entry accepts only the opaque orphan producer's verified before -> after edges;
/// it never signs current bytes or repeats an obsolete original-before validation.
pub(super) fn apply_sequence_after_approved_prefix(
    ctx: &Context,
    plans: &[PreparedAction],
    prefix: &super::native_orphans::ApprovedEffects,
) -> Result<Vec<Outcome>> {
    fn stamp(value: Option<super::native_orphans::EffectStamp>) -> Option<Fingerprint> {
        value.map(|(len, sha256, modified)| Fingerprint {
            len,
            sha256,
            modified,
        })
    }
    // Validate the full original dependency union before any thread mutation. An edge
    // derived from a different preview cannot authorize this plan, even if current
    // bytes happen to equal its after value.
    for plan in plans {
        validate_context(ctx, plan)?;
        for (path, original) in &plan.dependencies {
            let expected = match prefix.effect(path) {
                Some((before, after)) => {
                    if stamp(before) != *original {
                        return Err("前序孤儿修复的原始依赖不属于本次线程预览，请重新预览".into());
                    }
                    stamp(after)
                }
                None => original.clone(),
            };
            if fingerprint(path)? != expected {
                return Err("前序修复后线程依赖已被外部更新或不等于核验效果，请重新预览".into());
            }
        }
    }
    execute_sequence(ctx, plans)
}
fn execute_sequence(ctx: &Context, plans: &[PreparedAction]) -> Result<Vec<Outcome>> {
    let mut approved = Scope::default();
    for p in plans {
        approved.extend(&p.scope)
    }
    let mut outcomes = vec![];
    for p in plans {
        let fresh = match prepare_with_backup(ctx, &p.id, &p.backup_root) {
            Ok(p) => p,
            Err(_) => {
                let check = inspect_one(ctx, &p.id).unwrap_or_else(|_| {
                    diagnostic(
                        &p.id,
                        "warning",
                        "线程维护复检",
                        "依赖未通过，未执行本项。".into(),
                        false,
                    )
                });
                outcomes.push(Outcome {
                    id: p.id.clone(),
                    success: false,
                    changed: 0,
                    detail: "依赖修复未满足原生条件，本项未执行。".into(),
                    backups: vec![],
                    diagnostic: check,
                    warnings: vec![],
                    partial_effects_possible: false,
                });
                break;
            }
        };
        if !approved.contains(&fresh.scope) {
            return Err("依赖修复产生未批准的新目标，请重新预览；此前结果及备份已保留".into());
        }
        let outcome = execute(ctx, &fresh);
        let stop = !outcome.success;
        outcomes.push(outcome);
        if stop {
            break;
        }
    } // Later requested actions can change earlier invariants; final flags use current native checks.
    for outcome in &mut outcomes {
        match inspect_one(ctx, &outcome.id) {
            Ok(check) => {
                outcome.success &= check.status == "ok";
                outcome.diagnostic = check;
                if !outcome.success {
                    outcome.detail = "组合修复后的最终原生复检仍有未解决项目，未标为成功。".into();
                }
            }
            Err(_) => {
                outcome.success = false;
                outcome.detail = "组合修复后的最终快照复检失败，未标为成功。".into();
            }
        }
    }
    Ok(outcomes)
}
