use super::*;
use crate::gateway::storage::{atomic_private, open_private, private_dir};
use fs2::FileExt;
use rusqlite::params;
use serde_json::json;
#[cfg(test)]
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
#[cfg(test)]
#[path = "mutation_guard_tests.rs"]
mod mutation_guard_tests;
#[derive(Clone)]
pub(super) struct FileChange {
    path: PathBuf,
    // None is absence; Some(empty) is a real zero-byte file.
    pub(super) before: Option<Vec<u8>>,
    pub(super) after: Option<Vec<u8>>,
}
impl FileChange {
    pub(super) fn path(&self) -> &Path {
        &self.path
    }
    pub(super) fn before(&self) -> Option<&[u8]> {
        self.before.as_deref()
    }
    pub(super) fn after(&self) -> Option<&[u8]> {
        self.after.as_deref()
    }
    fn write(path: PathBuf, before: Vec<u8>, after: Vec<u8>) -> Self {
        Self {
            path,
            before: Some(before),
            after: Some(after),
        }
    }
}
fn file_state(path: &Path) -> Result<Option<Vec<u8>>> {
    match std::fs::symlink_metadata(path) {
        Ok(_) => read_bounded(path, inspect::LIMIT).map(Some),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(_) => Err("无法检查待修复文件".into()),
    }
}
fn set_file_state(path: &Path, state: &Option<Vec<u8>>) -> Result<()> {
    match state {
        Some(bytes) => atomic_private(path, bytes),
        None => std::fs::remove_file(path).map_err(|_| "删除待修复文件失败".into()),
    }
}
#[derive(Clone)]
struct RowChange {
    id: String,
    before: String,
    after: String,
    path: String,
    after_path: String,
}
#[derive(Clone)]
pub(super) struct Ticket {
    pub view: DiagnosticRepairPlan,
    pub created_at: u64,
    home: PathBuf,
    app_root: PathBuf,
    config_hash: String,
    gateway_hash: Option<String>,
    pub(super) files: Vec<FileChange>,
    db: Option<PathBuf>,
    rows: Vec<RowChange>,
    cwds: Vec<super::project_state::CwdUpdate>,
    relocation: Vec<super::state_db_relocation::Deletion>,
    native_threads: Vec<super::native_threads::PreparedAction>,
    coordinated: Option<super::native_coordinator::Prepared>,
    orphans: Option<super::native_orphans::Prepared>,
    project: Option<super::project_state::Prepared>,
}
impl Ticket {
    pub(super) fn home(&self) -> &Path {
        &self.home
    }
    pub(super) fn app_root(&self) -> &Path {
        &self.app_root
    }
    pub(super) fn original_config_hash(&self) -> &str {
        &self.config_hash
    }
    pub(super) fn project_plan(&self) -> Option<&super::project_state::Prepared> {
        self.project.as_ref()
    }
    pub(super) fn orphan_catalog(&self) -> Option<&super::native_orphans::OfficialCatalog> {
        self.orphans.as_ref().map(|p| p.official_catalog())
    }
    pub(super) fn selected_ids(&self) -> &[String] {
        &self.view.item_ids
    }
    pub(super) fn changes_provider_target(&self) -> bool {
        self.orphans.is_some()
            || self
                .native_threads
                .iter()
                .any(super::native_threads::PreparedAction::changes_provider_target)
    }
    /// Generic file effects. Coordinated native effects remain owned by their typed plan.
    pub(super) fn file_changes(&self) -> &[FileChange] {
        &self.files
    }
    /// Some(None) is an approved deletion; None means configuration is not modified.
    pub(super) fn config_after(&self) -> Option<Option<Vec<u8>>> {
        if let Some(plan) = self.coordinated.as_ref().filter(|plan| plan.owns_config()) {
            return Some(plan.config_after().map(<[u8]>::to_vec));
        }
        self.files
            .iter()
            .find(|f| f.path == self.home.join("config.toml"))
            .map(|f| f.after.clone())
    }
}
fn digest(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn config_hash(ctx: &Context) -> Result<String> {
    if ctx.config().exists() {
        Ok(digest(&read_bounded(&ctx.config(), 1024 * 1024)?))
    } else {
        Ok("absent".into())
    }
}
pub(super) fn preview(ctx: &Context, item_id: &str) -> Result<Ticket> {
    preview_internal(ctx, item_id, None)
}
pub(super) fn preview_with_official_catalog(
    ctx: &Context,
    item_id: &str,
    official: &super::native_orphans::OfficialCatalog,
) -> Result<Ticket> {
    preview_internal(ctx, item_id, Some(official))
}
fn preview_internal(
    ctx: &Context,
    item_id: &str,
    official: Option<&super::native_orphans::OfficialCatalog>,
) -> Result<Ticket> {
    let report = inspect::run(ctx)?;
    let selected = report
        .items
        .iter()
        .filter(|i| i.fixable && (item_id == "all" || item_id == i.id))
        .collect::<Vec<_>>();
    if selected.is_empty() {
        return Err("当前没有可自动修复的项目；请重新检测并展开问题详情".into());
    }
    let item_ids = selected.iter().map(|i| i.id.clone()).collect::<Vec<_>>();
    let mut changes = vec![];
    let mut warnings = vec!["会先保存备份，校验外部变更；确认后才修改上述文件。".into()];
    let mut files = vec![];
    let syntax_only = item_ids.len() == 1 && item_ids[0] == "config_toml_syntax";
    let c = match inspect::config(ctx) {
        Ok(c) => c,
        Err(e) if syntax_only => {
            warnings.push(format!("原始配置解析失败，将进入抢救重建：{e}"));
            toml::Value::Table(Default::default())
        }
        Err(e) => return Err(e),
    };
    let original = if ctx.config().exists() {
        read_bounded(&ctx.config(), 1024 * 1024)?
    } else {
        vec![]
    };
    let _doc = match std::str::from_utf8(&original)
        .map_err(|_| "配置不是 UTF-8")?
        .parse::<toml_edit::DocumentMut>()
    {
        Ok(d) => d,
        Err(_) if syntax_only => {
            let plan = super::config_syntax::plan_config_toml_syntax_rescue(&original);
            let (after, message) = match plan {
                super::config_syntax::RescuePlan::Plan { after, message } => (after, message),
                super::config_syntax::RescuePlan::None => {
                    return Err("配置无需抢救（当前可解析）".into())
                }
            };
            changes.push(message.to_string());
            warnings.push("确认后将原子写入抢救重建的 config.toml；原件先逐字备份".into());
            return Ok(Ticket {
                view: DiagnosticRepairPlan {
                    ticket: format!("syntax-rescue-{}", now_ms()),
                    item_ids: item_ids.clone(),
                    title: "config.toml 语法抢救重建".into(),
                    changes,
                    warnings,
                    requires_desktop_closed: true,
                },
                created_at: now_ms(),
                home: ctx.home.clone(),
                app_root: ctx.app_root.clone(),
                config_hash: config_hash(ctx)?,
                gateway_hash: None,
                files: vec![FileChange {
                    path: ctx.config().clone(),
                    before: Some(original.clone()),
                    after: Some(after),
                }],
                db: None,
                rows: vec![],
                cwds: vec![],
                relocation: vec![],
                native_threads: vec![],
                coordinated: None,
                orphans: None,
                project: None,
            });
        }
        Err(e) => return Err(format!("配置解析失败：{e}")),
    };
    let relocation = if item_ids.iter().any(|id| id == "state_db_relocation") {
        let plan = super::state_db_relocation::plan(ctx, &c)?;
        changes.push(format!(
            "流式备份并清理 {} 个根目录旧库相关文件（含 WAL/SHM）；保留 sqlite/ 活库",
            plan.len()
        ));
        changes.extend(
            plan.iter()
                .map(|file| format!("备份并删除 {}（{} 字节）", file.path.display(), file.length)),
        );
        plan
    } else {
        vec![]
    };
    let db = None;
    let rows = vec![];
    let cwds = vec![];
    for kind in super::residual_cleanup::Kind::ALL {
        if !item_ids.iter().any(|id| id == kind.id()) {
            continue;
        }
        let repairable = kind.repairable(ctx, &c)?;
        if repairable.is_empty() {
            return Err("路由启用中，不能删除正常使用的迁移清单或缓存".into());
        }
        for (path, before) in repairable {
            files.push(FileChange {
                path,
                before: Some(before),
                after: None,
            });
        }
        changes.push(format!("路由已关闭，备份并删除{}", kind.label()));
    }
    let orphans = if item_ids.iter().any(|id| super::native_orphans::handles(id)) {
        // A reviewed official pool is opaque: it comes from actual guarded model/list.
        let plan = if let Some(official) = official {
            super::native_orphans::prepare_with_catalog(ctx, official)?
        } else {
            super::native_orphans::prepare(ctx)?
        };
        changes.extend(super::native_orphans::preview_lines(&plan));
        Some(plan)
    } else {
        None
    };
    let project = if item_ids.iter().any(|id| id == "project_state_integrity") {
        let plan = super::project_state::prepare(ctx)?;
        changes.extend(super::project_state::preview_lines(&plan));
        Some(plan)
    } else {
        None
    };
    let ticket_id = uuid::Uuid::new_v4().to_string();
    let native_backup = ctx
        .app_root
        .join("maintenance/diagnostic-backups")
        .join(&ticket_id)
        .join("threads");
    let native_threads = super::native_threads::prepare_sequence(ctx, &item_ids, &native_backup)?;
    for plan in &native_threads {
        changes.extend(super::native_threads::preview_lines(plan));
    }
    let coordinated_ids = item_ids
        .iter()
        .filter(|id| super::native_coordinator::handles(id))
        .cloned()
        .collect::<Vec<_>>();
    let coordinated = if !coordinated_ids.is_empty() {
        let source = if coordinated_ids
            .iter()
            .any(|id| super::native_catalog::handles(id))
        {
            Some(super::native_catalog::context_source(ctx)?)
        } else {
            None
        };
        let plan = super::native_coordinator::prepare(
            ctx,
            &coordinated_ids,
            source.as_ref(),
            &ctx.app_root
                .join("maintenance/diagnostic-backups")
                .join(&ticket_id)
                .join("coordinated"),
        )?;
        changes.extend(super::native_coordinator::preview_lines(&plan));
        Some(plan)
    } else {
        None
    };

    if files.is_empty()
        && rows.is_empty()
        && cwds.is_empty()
        && relocation.is_empty()
        && native_threads.is_empty()
        && coordinated.is_none()
        && orphans.is_none()
        && project.is_none()
    {
        return Err("没有实际文件变更，请重新检测".into());
    }
    if files
        .iter()
        .map(|f| f.before.as_ref().map_or(0, Vec::len) + f.after.as_ref().map_or(0, Vec::len))
        .sum::<usize>()
        > 256 * 1024 * 1024
    {
        return Err("本次修复文件超过256MiB，请在会话管理中分批处理".into());
    }
    changes.extend(files.iter().take(8).map(|f| {
        format!(
            "{} {}",
            match (&f.before, &f.after) {
                (None, Some(_)) => "创建",
                (_, None) => "备份并删除",
                _ => "备份并更新",
            },
            f.path.display()
        )
    }));
    if files.len() > 8 {
        changes.push(format!(
            "另有 {} 个会话文件，全部保存在备份清单",
            files.len() - 8
        ))
    }
    let requires_desktop_closed = !super::quiet_window::can_run_with_desktop(&item_ids);
    warnings.push(if requires_desktop_closed {
        "请先退出 Codex / ChatGPT Desktop 并结束 codex CLI。Atelier 不会替你强制结束客户端。".into()
    } else {
        "本次仅清理已关闭路由的残留，不需要退出 Desktop；不会发送停止或重启命令。".into()
    });
    Ok(Ticket {
        view: DiagnosticRepairPlan {
            ticket: ticket_id,
            item_ids: item_ids.clone(),
            title: if item_id == "all" {
                "预览全部可修复项目".into()
            } else {
                "预览修复".into()
            },
            changes,
            warnings,
            requires_desktop_closed,
        },
        created_at: now_ms(),
        home: ctx.home.clone(),
        app_root: ctx.app_root.clone(),
        config_hash: config_hash(ctx)?,
        gateway_hash: if item_ids
            .iter()
            .any(|id| matches!(id.as_str(), "config_stale" | "router_http_contract"))
        {
            Some(digest(&read_bounded(
                &ctx.app_root.join("gateway/catalog.json"),
                8 * 1024 * 1024,
            )?))
        } else {
            None
        },
        files,
        db,
        rows,
        cwds,
        relocation,
        native_threads,
        coordinated,
        orphans,
        project,
    })
}
fn restore(files: &[&FileChange]) -> Vec<String> {
    files
        .iter()
        .rev()
        .filter_map(|f| match file_state(&f.path) {
            Ok(current) if current == f.before => None,
            Ok(current) if current == f.after => set_file_state(&f.path, &f.before)
                .err()
                .map(|_| format!("恢复 {} 失败；请使用备份", f.path.display())),
            _ => Some(format!(
                "{} 已被外部更新，未覆盖；请对照备份手动恢复",
                f.path.display()
            )),
        })
        .collect()
}
/// Owns both maintenance mutation locks. Private descriptors cannot be detached from
/// their explicit Context scope; Manager keeps this value alive through repair.
pub(super) struct MutationGuard {
    home: PathBuf,
    app_root: PathBuf,
    canonical_home: Option<PathBuf>,
    canonical_app_root: PathBuf,
    config_lock: std::fs::File,
    thread_lock: std::fs::File,
}
impl MutationGuard {
    pub(super) fn validate(&self, ctx: &Context) -> Result<()> {
        if self.home != ctx.home
            || self.app_root != ctx.app_root
            || self.canonical_home != std::fs::canonicalize(&ctx.home).ok()
            || std::fs::canonicalize(&ctx.app_root).ok().as_ref() != Some(&self.canonical_app_root)
        {
            return Err("维护锁的目标目录与当前修复范围不一致".into());
        }
        for (file, path) in [
            (
                &self.config_lock,
                ctx.app_root.join("maintenance/codex-write.lock"),
            ),
            (
                &self.thread_lock,
                ctx.app_root.join("thread-backups/mutation.lock"),
            ),
        ] {
            let held = file.metadata().map_err(|_| "无法核对维护锁")?;
            let current = std::fs::symlink_metadata(path).map_err(|_| "维护锁文件已变化")?;
            if !current.is_file() || current.file_type().is_symlink() {
                return Err("维护锁文件已变化".into());
            }
            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                if held.dev() != current.dev() || held.ino() != current.ino() {
                    return Err("维护锁文件已变化".into());
                }
            }
            #[cfg(not(unix))]
            let _ = held;
        }
        Ok(())
    }
}
pub(super) fn acquire(ctx: &Context) -> Result<MutationGuard> {
    let lock_dir = ctx.app_root.join("maintenance");
    private_dir(&lock_dir)?;
    let config_lock = open_private(&lock_dir.join("codex-write.lock"))?;
    config_lock
        .try_lock_exclusive()
        .map_err(|_| "另一维护操作正在修改 Codex")?;
    let thread_dir = ctx.app_root.join("thread-backups");
    private_dir(&thread_dir)?;
    let thread_lock = open_private(&thread_dir.join("mutation.lock"))?;
    thread_lock
        .try_lock_exclusive()
        .map_err(|_| "线程管理正在执行修改")?;
    let guard = MutationGuard {
        home: ctx.home.clone(),
        app_root: ctx.app_root.clone(),
        canonical_home: std::fs::canonicalize(&ctx.home).ok(),
        canonical_app_root: std::fs::canonicalize(&ctx.app_root).map_err(err)?,
        config_lock,
        thread_lock,
    };
    guard.validate(ctx)?;
    Ok(guard)
}
fn validate_apply_context(ctx: &Context, t: &Ticket, reject_transition: bool) -> Result<()> {
    if now_ms().saturating_sub(t.created_at) > 300_000 {
        return Err("修复预览已过期，请重新检测".into());
    }
    if ctx.home != t.home || ctx.app_root != t.app_root {
        return Err("修复目标目录已变化".into());
    }
    // Atelier preview gate: reject an in-flight transition before mutation.
    // Native Manager initialization failure is a separate storage-load error, not this journal flag.
    // Recheck the current context; an earlier preview is not an authorization to
    // write across a RouterOn/NativeOff transition that started afterwards.
    atelier_maintenance_core::transition_journal::JournalStore::new(
        &ctx.home.join("codexmate/relay"),
    )
    .read_checked()
    .map_err(|e| format!("路由恢复记录无法核对，未修改文件：{e}"))?;
    if reject_transition && inspect::transition_in_flight(ctx) {
        return Err("路由转换进行中，请等待完成后重新预览；未修改任何文件。".into());
    }
    if ctx.desktop_running && !super::quiet_window::can_run_with_desktop(&t.view.item_ids) {
        return Err("Desktop 仍在运行；请退出客户端后重新预览。未修改任何文件。".into());
    }
    if ctx.runtime.as_ref().is_some_and(|r| r.active_requests > 0) {
        return Err("本地网关仍有在途请求，请等待完成后重新预览".into());
    }
    Ok(())
}
pub(super) fn apply(ctx: &Context, t: Ticket) -> Result<DiagnosticFixResult> {
    validate_apply_context(ctx, &t, true)?;
    let guard = acquire(ctx)?;
    // A transition can appear while waiting for the filesystem locks.
    validate_apply_context(ctx, &t, true)?;
    apply_body(ctx, t, &guard)
}
/// Manager owns the transition journal and has refreshed Context from real process
/// inspection. Only its own active-transition barrier is omitted; quiet/CAS checks remain.
pub(super) fn apply_under_manager(
    ctx: &Context,
    t: Ticket,
    guard: &MutationGuard,
) -> Result<DiagnosticFixResult> {
    apply_body(ctx, t, guard)
}
/// Read-only validation of every source in the original ticket. The batch calls
/// this for all reviewed ordinary items before its first mutation.
pub(super) fn validate_reviewed_sources(ctx: &Context, t: &Ticket) -> Result<()> {
    if ctx.home != t.home || ctx.app_root != t.app_root {
        return Err("维护票据目录已变化".into());
    }
    if config_hash(ctx)? != t.config_hash {
        return Err("配置已被外部更新，请重新预览；未修改文件".into());
    }
    if let Some(expected) = &t.gateway_hash {
        if digest(&read_bounded(
            &ctx.app_root.join("gateway/catalog.json"),
            8 * 1024 * 1024,
        )?) != *expected
        {
            return Err("网关配置已变化，请重新预览".into());
        }
    }
    for f in &t.files {
        if file_state(&f.path)? != f.before {
            return Err("待修复文件已被外部更新，请重新预览；未修改文件".into());
        }
    }
    let syntax_only = t.view.item_ids.len() == 1 && t.view.item_ids[0] == "config_toml_syntax";
    let c = match inspect::config(ctx) {
        Ok(c) => c,
        Err(_e) if syntax_only => toml::Value::Table(Default::default()),
        Err(e) => return Err(e),
    };
    for kind in super::residual_cleanup::Kind::ALL {
        if !t.view.item_ids.iter().any(|id| id == kind.id()) {
            continue;
        }
        for file in &t.files {
            if kind.paths(ctx).contains(&file.path) && !kind.can_delete(ctx, &c, &file.path)? {
                return Err("路由在预览后已启用，请重新检测；未删除文件".into());
            }
        }
    }
    if !t.relocation.is_empty() {
        super::state_db_relocation::validate(ctx, &c, &t.relocation)?;
    }
    if let Some(db) = &t.db {
        if inspect::databases(&inspect::db_dir(ctx, &c)).pop().as_ref() != Some(db) {
            return Err("会话索引已变化".into());
        }
    }
    if let Some(plan) = &t.project {
        super::project_state::validate(ctx, plan)?;
    }
    if let Some(plan) = &t.orphans {
        super::native_orphans::validate_plan(ctx, plan)?;
    }
    super::native_threads::validate_sequence(ctx, &t.native_threads)?;
    if let Some(plan) = &t.coordinated {
        super::native_coordinator::validate_initial(ctx, plan)?;
    }
    Ok(())
}
fn apply_body(ctx: &Context, t: Ticket, guard: &MutationGuard) -> Result<DiagnosticFixResult> {
    guard.validate(ctx)?;
    validate_apply_context(ctx, &t, false)?;
    let lock_dir = ctx.app_root.join("maintenance");
    validate_reviewed_sources(ctx, &t)?;
    let c = inspect::config(ctx).or_else(|e| {
        if t.view.item_ids == ["config_toml_syntax"] {
            Ok(toml::Value::Table(Default::default()))
        } else {
            Err(e)
        }
    })?;
    let backup = lock_dir.join("diagnostic-backups").join(&t.view.ticket);
    private_dir(&backup)?;
    let relocation_seconds = now_ms() / 1000;
    let manifest = json!({
        "version":2,"createdAt":now_ms(),"itemIds":t.view.item_ids,
        "database":t.db,"cwdUpdates":t.cwds,
        "files":t.files.iter().enumerate().map(|(i,f)|json!({
            "path":f.path,
            "operation":match (&f.before, &f.after) {
                (None, Some(_)) => "create", (_, None) => "delete", _ => "write"
            },
            "beforeExists":f.before.is_some(),"afterExists":f.after.is_some(),
            "backup":f.before.as_ref().map(|_|format!("{i:04}.bin")),
            "sha256":f.before.as_deref().map(digest),
            "afterSha256":f.after.as_deref().map(digest)
        })).chain(t.relocation.iter().map(|f|json!({
            "path":f.path,"operation":"delete","beforeExists":true,"afterExists":false,
            "backup":format!("state-db-backups/{}.{}",relocation_seconds,f.path.file_name().unwrap_or_default().to_string_lossy()),
            "sha256":f.sha256,"length":f.length,"afterSha256":null
        }))).collect::<Vec<_>>(),
        "rows":t.rows.iter().map(|r|json!({"id":r.id,"before":r.before,"after":r.after,"rolloutPath":r.path,"afterRolloutPath":r.after_path})).collect::<Vec<_>>()
    });
    for (i, f) in t.files.iter().enumerate() {
        if let Some(before) = &f.before {
            atomic_private(&backup.join(format!("{i:04}.bin")), before)?
        }
    }
    atomic_private(
        &backup.join("manifest.json"),
        &serde_json::to_vec_pretty(&manifest).map_err(err)?,
    )?;
    // Native actions validate their complete original input union before the first
    // mutation, then replan between related repairs. Keep their real partial outcomes.
    let orphan_result = if let Some(plan) = &t.orphans {
        Some(super::native_orphans::apply_verified(
            ctx,
            plan,
            &backup.join("orphan-adoptions"),
            guard,
        )?)
    } else {
        None
    };
    let thread_results = if let Some(outcome) = &orphan_result {
        super::native_threads::apply_sequence_after_approved_prefix(
            ctx,
            &t.native_threads,
            &outcome.effects,
        )?
    } else {
        super::native_threads::apply_sequence(ctx, &t.native_threads)?
    };
    atomic_private(
        &backup.join("thread-results.json"),
        &serde_json::to_vec_pretty(
            &thread_results
                .iter()
                .map(|r| {
                    json!({"id":r.id,"success":r.success,"changed":r.changed,
            "detail":r.detail,"backups":r.backups,"warnings":r.warnings,
            "partialEffectsPossible":r.partial_effects_possible})
                })
                .collect::<Vec<_>>(),
        )
        .map_err(err)?,
    )?;
    let coordinated_result = if let Some(plan) = &t.coordinated {
        let mut session = super::native_coordinator::begin_under_existing_gate(ctx, plan)?;
        super::native_coordinator::catalog_stage(ctx, &mut session);
        super::native_coordinator::config_stage(ctx, &mut session);
        super::native_coordinator::auth_stage(ctx, &mut session);
        super::native_coordinator::postcheck(ctx, &mut session);
        let outcome = session.into_outcome();
        atomic_private(
            &backup.join("coordinated-result.json"),
            &serde_json::to_vec_pretty(&outcome).map_err(err)?,
        )?;
        if outcome.failure.is_some() {
            return coordinated_failure(ctx, &t, &backup, &outcome, &thread_results);
        }
        Some(outcome)
    } else {
        None
    };
    let mut connection =
        t.db.as_ref()
            .map(|db| inspect::open_db(db, true))
            .transpose()?;
    let mut tx = connection
        .as_mut()
        .map(|db| {
            db.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
                .map_err(err)
        })
        .transpose()?;
    if let Some(tx) = &tx {
        for row in &t.rows {
            let existing: (String, String) = tx
                .query_row(
                    "SELECT COALESCE(model_provider,''),rollout_path FROM threads WHERE id=?1",
                    [&row.id],
                    |r| Ok((r.get(0)?, r.get(1)?)),
                )
                .map_err(|_| "会话索引已变化")?;
            if row.after_path != row.path {
                let resolved = inspect::resolve(ctx, &row.after_path)?;
                if inspect::meta(&inspect::rollout(&resolved)?)?["id"].as_str()
                    != Some(row.id.as_str())
                {
                    return Err("恢复目标的会话身份已变化".into());
                }
            }
            if existing != (row.before.clone(), row.path.clone()) {
                return Err("会话已被外部修改，请重新预览；未修改文件".into());
            }
        }
    }
    if let Some(tx) = &tx {
        for change in &t.cwds {
            let existing: (String, String) = tx
                .query_row(
                    "SELECT COALESCE(cwd,''),rollout_path FROM threads WHERE id=?1",
                    [&change.id],
                    |r| Ok((r.get(0)?, r.get(1)?)),
                )
                .map_err(|_| "项目索引已变化")?;
            if existing != (change.before.clone(), change.path.clone()) {
                return Err("线程工作目录已被外部修改，未执行修复".into());
            }
            let path = inspect::resolve(ctx, &change.path)?;
            let meta = inspect::meta(&inspect::rollout(&path)?)?;
            if meta["id"].as_str() != Some(change.id.as_str())
                || meta["cwd"]
                    .as_str()
                    .map(|s| s.trim().trim_end_matches(['/', '\\']))
                    != Some(change.after.as_str())
            {
                return Err("会话工作目录已被外部修改，未执行修复".into());
            }
        }
    }
    let mut written = vec![];
    let outcome = (|| -> Result<()> {
        for f in &t.files {
            if file_state(&f.path)? != f.before {
                return Err("应用期间文件发生变化".into());
            }
            set_file_state(&f.path, &f.after)?;
            written.push(f);
            if file_state(&f.path)? != f.after {
                return Err("写入读回校验失败".into());
            }
        }
        if let Some(tx) = &tx {
            for row in &t.rows {
                let n=tx.execute("UPDATE threads SET model_provider=?1,rollout_path=?5 WHERE id=?2 AND COALESCE(model_provider,'')=?3 AND rollout_path=?4",params![row.after,row.id,row.before,row.path,row.after_path]).map_err(err)?;
                if n != 1 {
                    return Err("会话更新数量不一致".into());
                }
            }
        }
        if let Some(tx) = &tx {
            for change in &t.cwds {
                let n = tx
                    .execute(
                        "UPDATE threads SET cwd=?1 WHERE id=?2 AND COALESCE(cwd,'')=?3",
                        params![change.after, change.id, change.before],
                    )
                    .map_err(err)?;
                if n != 1 {
                    return Err("工作目录更新数量不一致".into());
                }
            }
        }
        Ok(())
    })();
    if let Err(e) = outcome {
        drop(tx);
        let errors = restore(&written);
        return Err(format!(
            "修复失败：{e}。此前已完成的原生线程、认证修复及备份保留；其余文件事务：{} 备份：{}",
            if errors.is_empty() {
                "已恢复写入的文件，索引事务已回滚".into()
            } else {
                errors.join("；")
            },
            backup.display()
        ));
    }
    if let Some(tx) = tx.take() {
        if let Err(e) = tx.commit() {
            let errors = restore(&written);
            return Err(format!(
                "索引提交失败：{e}；此前原生线程、认证修复保留，文件事务恢复{}。备份：{}",
                if errors.is_empty() {
                    "成功"
                } else {
                    "未全部成功"
                },
                backup.display()
            ));
        }
    }
    // Close the write connection before re-inspection; SQLite may checkpoint its own WAL here.
    drop(tx);
    drop(connection);
    let relocation_result = if !t.relocation.is_empty() {
        Some(
            match super::state_db_relocation::apply(
                ctx,
                &c,
                &t.relocation,
                &backup,
                relocation_seconds,
            ) {
                Ok(result) => result,
                Err(error) => atelier_maintenance_core::state_db::RelocationResult {
                    fixed: false,
                    failed: true,
                    details: vec![format!("旧库清理未完成：{error}；其他已完成项目保留")],
                    removed_databases: 0,
                    backups: vec![],
                    remaining: t.relocation.iter().map(|p| p.path.clone()).collect(),
                },
            },
        )
    } else {
        None
    };
    let project_result = if let Some(plan) = &t.project {
        let result =
            super::project_state::apply_under_manager(ctx, plan, guard, &backup.join("project"))?;
        if !result.success {
            return Err(format!(
                "{}；项目备份目录：{}；部分效果可能存在：{}",
                result.detail,
                backup.join("project").display(),
                result.partial_effects_possible
            ));
        }
        Some(result)
    } else {
        None
    };
    let mut report = inspect::run(ctx)?;
    if let Some(result) = relocation_result.as_ref().filter(|r| r.failed) {
        if let Some(item) = report
            .items
            .iter_mut()
            .find(|i| i.id == "state_db_relocation")
        {
            item.status = "error".into();
            item.detail = result.details.join("；");
        }
        report.has_issues = true;
    }
    for outcome in thread_results.iter().filter(|r| !r.success) {
        if let Some(item) = report.items.iter_mut().find(|i| i.id == outcome.id) {
            if item.status == "ok" {
                item.status = "warning".into();
            }
            item.detail = outcome.detail.clone();
        }
        report.has_issues = true;
    }
    if let Some(outcome) = orphan_result.as_ref().filter(|o| o.failure.is_some()) {
        for item in report
            .items
            .iter_mut()
            .filter(|i| super::native_orphans::handles(&i.id))
        {
            item.status = "warning".into();
            item.detail = outcome.details.join("；");
        }
        report.has_issues = true;
    }
    if let Some(outcome) = project_result.as_ref().filter(|o| !o.success) {
        if let Some(item) = report
            .items
            .iter_mut()
            .find(|i| i.id == "project_state_integrity")
        {
            if item.status == "ok" {
                item.status = "warning".into();
            }
            item.detail = outcome.detail.clone();
        }
        report.has_issues = true;
    }
    let requested = t.view.item_ids.iter().cloned().collect::<BTreeSet<_>>();
    let mut fixed_ids = vec![];
    let mut failed_ids = vec![];
    for id in requested {
        if report.items.iter().any(|i| i.id == id && i.status == "ok") {
            fixed_ids.push(id)
        } else {
            failed_ids.push(id)
        }
    }
    let mut details = vec![format!(
        "已更新 {} 个文件、{} 条索引；原始内容已备份",
        t.files.len(),
        t.rows.len()
    )];
    if let Some(outcome) = &project_result {
        details.push(outcome.detail.clone());
    }
    if let Some(outcome) = &orphan_result {
        details.extend(outcome.details.clone());
    }
    if let Some(result) = &relocation_result {
        details.extend(result.details.clone());
    }
    details.extend(
        thread_results
            .iter()
            .map(|r| format!("{}：{}", r.id, r.detail)),
    );
    details.extend(
        thread_results
            .iter()
            .flat_map(|r| r.warnings.iter().cloned()),
    );
    if let Some(outcome) = &coordinated_result {
        details.extend(
            outcome
                .completed
                .iter()
                .map(|stage| format!("{}：{}", stage.stage, stage.detail)),
        );
        if outcome.requires_manager_reconcile {
            details.push(
                "本地文件修复已复检；完整 Manager 路由重收敛尚未接入，本结果不代表在线路由已验证。"
                    .into(),
            );
        }
    }

    if !failed_ids.is_empty() {
        details.push("部分项目复检未通过或覆盖不完整；请查看新报告，不标记为已修复".into())
    }
    atomic_private(
        &backup.join("result.json"),
        &serde_json::to_vec(&json!({"fixedIds":fixed_ids,"failedIds":failed_ids,"at":now_ms(),"databaseCleanup":relocation_result.as_ref().map(|r|json!({"backups":r.backups,"remaining":r.remaining,"removedDatabases":r.removed_databases}))}))
            .map_err(err)?,
    )?;
    Ok(DiagnosticFixResult {
        fixed_ids,
        failed_ids,
        details,
        backup_path: Some(backup.display().to_string()),
        report,
    })
}
fn coordinated_failure(
    ctx: &Context,
    ticket: &Ticket,
    backup: &Path,
    outcome: &super::native_coordinator::Outcome,
    threads: &[super::native_threads::Outcome],
) -> Result<DiagnosticFixResult> {
    let mut report = inspect::run(ctx)?;
    let subset = ticket
        .coordinated
        .as_ref()
        .map(|p| p.item_ids())
        .unwrap_or(&[]);
    for item in report.items.iter_mut().filter(|i| subset.contains(&i.id)) {
        if item.status == "ok" {
            item.status = "warning".into();
        }
        item.detail = "组合修复未全部完成；已完成阶段和备份保留，请查看修复结果。".into();
    }
    report.has_issues = true;
    let (fixed_ids, failed_ids): (Vec<_>, Vec<_>) =
        ticket.view.item_ids.iter().cloned().partition(|id| {
            !subset.contains(id) && report.items.iter().any(|i| &i.id == id && i.status == "ok")
        });
    let mut details = threads
        .iter()
        .map(|r| format!("{}：{}", r.id, r.detail))
        .collect::<Vec<_>>();
    details.extend(
        outcome
            .completed
            .iter()
            .map(|s| format!("{}：{}", s.stage, s.detail)),
    );
    details.push(format!(
        "组合修复失败：{}；后续文件事务未执行，已完成阶段保留。备份：{}",
        outcome.failure.as_deref().unwrap_or("未完成"),
        outcome.backup_path.display()
    ));
    let result = DiagnosticFixResult {
        fixed_ids,
        failed_ids,
        details,
        backup_path: Some(backup.display().to_string()),
        report,
    };
    atomic_private(
        &backup.join("result.json"),
        &serde_json::to_vec(&result).map_err(err)?,
    )?;
    Ok(result)
}
#[cfg(test)]
mod rollback_tests {
    use super::*;
    struct Fixture {
        root: PathBuf,
        ctx: Context,
    }
    impl Fixture {
        fn new() -> Self {
            let root =
                std::env::temp_dir().join(format!("atelier-file-change-{}", uuid::Uuid::new_v4()));
            let ctx = Context {
                home: root.join("codex"),
                app_root: root.join("atelier"),
                desktop_running: false,
                runtime: None,
                official_catalog: None,
                proxy_handle: None,
                working_directory: None,
                sqlite_home_env: None,
            };
            private_dir(&ctx.home).unwrap();
            private_dir(&ctx.app_root).unwrap();
            atomic_private(&ctx.config(), b"model = 'fixture'\n").unwrap();
            Self { root, ctx }
        }
        fn ticket(&self, files: Vec<FileChange>) -> Ticket {
            Ticket {
                view: DiagnosticRepairPlan {
                    ticket: uuid::Uuid::new_v4().to_string(),
                    item_ids: vec!["synthetic-file-change".into()],
                    title: "隔离文件变更测试".into(),
                    changes: vec![],
                    warnings: vec![],
                    requires_desktop_closed: true,
                },
                created_at: now_ms(),
                home: self.ctx.home.clone(),
                app_root: self.ctx.app_root.clone(),
                config_hash: config_hash(&self.ctx).unwrap(),
                gateway_hash: None,
                files,
                db: None,
                rows: vec![],
                cwds: vec![],
                relocation: vec![],
                native_threads: vec![],
                coordinated: None,
                orphans: None,
                project: None,
            }
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    #[test]
    fn confirmed_delete_unlinks_and_preserves_original_in_backup() {
        let f = Fixture::new();
        let path = f.ctx.home.join("selected.json");
        let other = f.ctx.home.join("unselected.json");
        atomic_private(&path, b"selected-original").unwrap();
        atomic_private(&other, b"unselected-original").unwrap();
        let ticket = f.ticket(vec![FileChange {
            path: path.clone(),
            before: Some(b"selected-original".to_vec()),
            after: None,
        }]);
        assert_eq!(std::fs::read(&path).unwrap(), b"selected-original");
        let result = apply(&f.ctx, ticket).unwrap();
        assert!(
            !path.exists(),
            "delete must unlink, not write an empty file"
        );
        assert_eq!(std::fs::read(other).unwrap(), b"unselected-original");
        let backup = PathBuf::from(result.backup_path.unwrap());
        assert_eq!(
            std::fs::read(backup.join("0000.bin")).unwrap(),
            b"selected-original"
        );
        let manifest: Value =
            serde_json::from_slice(&std::fs::read(backup.join("manifest.json")).unwrap()).unwrap();
        assert_eq!(manifest["files"][0]["operation"], "delete");
        assert_eq!(manifest["files"][0]["afterExists"], false);
        assert!(manifest["files"][0]["afterSha256"].is_null());
    }

    #[test]
    fn empty_write_keeps_a_real_file() {
        let f = Fixture::new();
        let path = f.ctx.home.join("empty");
        atomic_private(&path, b"before").unwrap();
        apply(
            &f.ctx,
            f.ticket(vec![FileChange::write(
                path.clone(),
                b"before".to_vec(),
                vec![],
            )]),
        )
        .unwrap();
        assert_eq!(file_state(&path).unwrap(), Some(vec![]));
    }

    #[test]
    fn create_records_absence_instead_of_a_fabricated_empty_backup() {
        let f = Fixture::new();
        let path = f.ctx.home.join("new-file");
        let result = apply(
            &f.ctx,
            f.ticket(vec![FileChange {
                path: path.clone(),
                before: None,
                after: Some(b"created".to_vec()),
            }]),
        )
        .unwrap();
        assert_eq!(std::fs::read(path).unwrap(), b"created");
        let backup = PathBuf::from(result.backup_path.unwrap());
        assert!(!backup.join("0000.bin").exists());
        let manifest: Value =
            serde_json::from_slice(&std::fs::read(backup.join("manifest.json")).unwrap()).unwrap();
        assert_eq!(manifest["files"][0]["operation"], "create");
        assert_eq!(manifest["files"][0]["beforeExists"], false);
        assert!(manifest["files"][0]["backup"].is_null());
    }

    #[test]
    fn external_creation_between_preview_and_apply_is_preserved() {
        let f = Fixture::new();
        let path = f.ctx.home.join("new-file");
        let ticket = f.ticket(vec![FileChange {
            path: path.clone(),
            before: None,
            after: Some(b"ours".to_vec()),
        }]);
        atomic_private(&path, b"external").unwrap();
        assert!(apply(&f.ctx, ticket).is_err());
        assert_eq!(std::fs::read(path).unwrap(), b"external");
        assert!(!f
            .ctx
            .app_root
            .join("maintenance/diagnostic-backups")
            .exists());
    }

    #[test]
    fn later_write_failure_rolls_back_both_create_and_delete() {
        let f = Fixture::new();
        let created = f.ctx.home.join("created");
        let deleted = f.ctx.home.join("deleted");
        atomic_private(&deleted, b"original").unwrap();
        let ticket = f.ticket(vec![
            FileChange {
                path: created.clone(),
                before: None,
                after: Some(b"new".to_vec()),
            },
            FileChange {
                path: deleted.clone(),
                before: Some(b"original".to_vec()),
                after: None,
            },
            FileChange {
                path: f.ctx.home.join("missing-parent/failure"),
                before: None,
                after: Some(vec![1]),
            },
        ]);
        assert!(apply(&f.ctx, ticket).unwrap_err().contains("已恢复"));
        assert!(!created.exists());
        assert_eq!(std::fs::read(deleted).unwrap(), b"original");
    }

    #[test]
    fn rollback_never_overwrites_a_file_recreated_after_deletion() {
        let f = Fixture::new();
        let path = f.ctx.home.join("deleted");
        let change = FileChange {
            path: path.clone(),
            before: Some(b"original".to_vec()),
            after: None,
        };
        atomic_private(&path, b"external").unwrap();
        assert_eq!(restore(&[&change]).len(), 1);
        assert_eq!(std::fs::read(path).unwrap(), b"external");
    }

    #[cfg(unix)]
    #[test]
    fn dangling_symlink_is_not_an_absent_creation_target() {
        let f = Fixture::new();
        let path = f.ctx.home.join("link");
        std::os::unix::fs::symlink(f.ctx.home.join("absent"), &path).unwrap();
        assert!(file_state(&path).is_err());
        let ticket = f.ticket(vec![FileChange {
            path: path.clone(),
            before: None,
            after: Some(b"ours".to_vec()),
        }]);
        assert!(apply(&f.ctx, ticket).is_err());
        assert!(std::fs::symlink_metadata(path)
            .unwrap()
            .file_type()
            .is_symlink());
    }

    #[test]
    fn rollback_does_not_overwrite_external_change() {
        let dir =
            std::env::temp_dir().join(format!("atelier-rollback-test-{}", uuid::Uuid::new_v4()));
        private_dir(&dir).unwrap();
        let path = dir.join("file");
        atomic_private(&path, b"external").unwrap();
        let change = FileChange {
            path: path.clone(),
            before: Some(b"original".to_vec()),
            after: Some(b"ours".to_vec()),
        };
        let errors = restore(&[&change]);
        assert_eq!(errors.len(), 1);
        assert_eq!(std::fs::read(path).unwrap(), b"external");
        std::fs::remove_dir_all(dir).unwrap();
    }
}

/// Reuse only an originally reviewed ordinary ticket. Configuration changes are
/// accepted exclusively through the actual batch/threads aggregate certificate;
/// no new target, source bytes, or current-state preview is introduced here.
pub(super) fn apply_ordinary_after_approved_prefix(
    ctx: &Context,
    t: &Ticket,
    guard: &MutationGuard,
    prefix: &super::native_threads::ApprovedThreadEffects,
) -> Result<DiagnosticFixResult> {
    if t.view.item_ids.len() != 1
        || t.orphans.is_some()
        || !t.native_threads.is_empty()
        || t.coordinated.is_some()
        || t.project.is_some()
    {
        return Err("普通批次入口仅接受单一原始残留/旧库票据".into());
    }
    if !matches!(
        t.view.item_ids[0].as_str(),
        "residual_manifest" | "residual_cache" | "state_db_relocation"
    ) {
        return Err("该项目不属于普通残留/旧库阶段".into());
    }
    let mut rebased = t.clone();
    if let Some((before, after)) = prefix.effect_stamp(&ctx.config()) {
        let original = before
            .as_ref()
            .map(|(_, h, _)| h.as_str())
            .unwrap_or("absent");
        if original != t.config_hash {
            return Err("普通项目的前序配置起点不是原始审批".into());
        }
        let current = file_state(&ctx.config())?;
        let actual = current.as_deref().map(|b| {
            (
                b.len() as u64,
                digest(b),
                std::fs::metadata(ctx.config())
                    .ok()
                    .and_then(|m| m.modified().ok()),
            )
        });
        if actual != after {
            return Err("普通项目前序配置效果已发生外部变化".into());
        }
        rebased.config_hash = after.map(|(_, h, _)| h).unwrap_or_else(|| "absent".into());
    }
    apply_under_manager(ctx, rebased, guard)
}

pub(super) struct PreparedOrdinary {
    ticket: Ticket,
    sources: std::collections::BTreeMap<PathBuf, Option<super::native_orphans::EffectStamp>>,
}
impl PreparedOrdinary {
    pub fn approved_sources(&self) -> Vec<(PathBuf, Option<super::native_orphans::EffectStamp>)> {
        self.sources
            .iter()
            .map(|(p, s)| (p.clone(), s.clone()))
            .collect()
    }
    pub fn ticket(&self) -> &Ticket {
        &self.ticket
    }
    pub fn validate_initial(&self, ctx: &Context) -> Result<()> {
        validate_reviewed_sources(ctx, &self.ticket)?;
        for (path, before) in &self.sources {
            if ordinary_stamp(path)? != *before {
                return Err("普通阶段原始目标的mtime/内容已变化".into());
            }
        }
        Ok(())
    }
}
pub(super) fn prepare_ordinary(ctx: &Context, id: &str) -> Result<PreparedOrdinary> {
    if !matches!(
        id,
        "residual_manifest" | "residual_cache" | "project_state_integrity" | "state_db_relocation"
    ) {
        return Err("不支持的普通尾项".into());
    }
    let ticket = preview(ctx, id)?;
    let mut sources = std::collections::BTreeMap::new();
    for file in &ticket.files {
        sources.insert(file.path.clone(), ordinary_stamp(&file.path)?);
    }
    for deletion in &ticket.relocation {
        sources.insert(deletion.path.clone(), ordinary_stamp(&deletion.path)?);
    }
    let prepared = PreparedOrdinary { ticket, sources };
    prepared.validate_initial(ctx)?;
    Ok(prepared)
}
pub(super) fn ordinary_stamp(path: &Path) -> Result<Option<super::native_orphans::EffectStamp>> {
    use std::io::Read;
    let metadata = match std::fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(err(e)),
    };
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err("普通维护目标不是普通文件".into());
    }
    let mut file = std::fs::File::open(path).map_err(err)?;
    let mut hash = Sha256::new();
    let mut length = 0;
    let mut buffer = [0u8; 65536];
    loop {
        let n = file.read(&mut buffer).map_err(err)?;
        if n == 0 {
            break;
        }
        length += n as u64;
        hash.update(&buffer[..n]);
    }
    let after = std::fs::metadata(path).map_err(err)?;
    if length != metadata.len()
        || after.len() != metadata.len()
        || after.modified().ok() != metadata.modified().ok()
    {
        return Err("普通维护目标读取期间变化".into());
    }
    Ok(Some((
        length,
        format!("{:x}", hash.finalize()),
        metadata.modified().ok(),
    )))
}
#[derive(Clone)]
pub(super) struct ApprovedOrdinaryEffects {
    home: PathBuf,
    app_root: PathBuf,
    effects: std::collections::BTreeMap<
        PathBuf,
        (
            Option<super::native_orphans::EffectStamp>,
            Option<super::native_orphans::EffectStamp>,
        ),
    >,
}
impl ApprovedOrdinaryEffects {
    pub fn paths(&self) -> Vec<PathBuf> {
        self.effects.keys().cloned().collect()
    }
    pub fn effect_stamp(
        &self,
        path: &Path,
    ) -> Option<(
        Option<super::native_orphans::EffectStamp>,
        Option<super::native_orphans::EffectStamp>,
    )> {
        self.effects.get(path).cloned()
    }
    pub fn validate_current(&self, ctx: &Context) -> Result<()> {
        if self.home != ctx.home || self.app_root != ctx.app_root {
            return Err("普通阶段凭据目录不匹配".into());
        }
        for (path, (_, after)) in &self.effects {
            if ordinary_stamp(path)? != *after {
                return Err("普通阶段已核效果遭外部变化".into());
            }
        }
        Ok(())
    }
}
pub(super) struct OrdinaryOutcome {
    pub result: Option<DiagnosticFixResult>,
    pub failure: Option<String>,
    pub effects: ApprovedOrdinaryEffects,
}
pub(super) fn apply_ordinary_certified(
    ctx: &Context,
    prepared: &PreparedOrdinary,
    guard: &MutationGuard,
    prefix: &super::native_threads::ApprovedThreadEffects,
) -> Result<OrdinaryOutcome> {
    guard.validate(ctx)?;
    prefix.validate_current(ctx)?;
    let mut ticket = prepared.ticket.clone();
    let mut before = std::collections::BTreeMap::new();
    for (path, original) in &prepared.sources {
        let expected = if let Some((initial, after)) = prefix.effect_stamp(path) {
            if &initial != original {
                return Err("普通阶段源不属于已审批前序起点".into());
            }
            after
        } else {
            original.clone()
        };
        if ordinary_stamp(path)? != expected {
            return Err("普通阶段源存在未审批变化".into());
        }
        before.insert(path.clone(), expected);
    }
    ticket
        .files
        .retain(|f| !(file_state(&f.path).ok() == Some(f.after.clone()) && f.before != f.after));
    ticket
        .relocation
        .retain(|d| before.get(&d.path) != Some(&None));
    let outcome = apply_ordinary_after_approved_prefix(ctx, &ticket, guard, prefix);
    let mut effects = std::collections::BTreeMap::new();
    for (path, current_before) in before {
        let actual = ordinary_stamp(&path)?;
        let same_content =
            |a: &Option<super::native_orphans::EffectStamp>,
             b: &Option<super::native_orphans::EffectStamp>| {
                a.as_ref().map(|(n, h, _)| (*n, h)) == b.as_ref().map(|(n, h, _)| (*n, h))
            };
        let after = prepared
            .ticket
            .files
            .iter()
            .find(|f| f.path == path)
            .map(|f| {
                f.after
                    .as_deref()
                    .map(|b| (b.len() as u64, digest(b), None))
            })
            .or_else(|| {
                prepared
                    .ticket
                    .relocation
                    .iter()
                    .any(|d| d.path == path)
                    .then_some(None)
            })
            .ok_or("普通效果缺少原始目标")?;
        if !same_content(&actual, &current_before) && !same_content(&actual, &after) {
            return Err("普通阶段部分写入超出原始before/after，禁止后继写入".into());
        }
        effects.insert(path, (current_before, actual));
    }
    let effects = ApprovedOrdinaryEffects {
        home: ctx.home.clone(),
        app_root: ctx.app_root.clone(),
        effects,
    };
    effects.validate_current(ctx)?;
    Ok(match outcome {
        Ok(result) => OrdinaryOutcome {
            result: Some(result),
            failure: None,
            effects,
        },
        Err(error) => OrdinaryOutcome {
            result: None,
            failure: Some(error),
            effects,
        },
    })
}
