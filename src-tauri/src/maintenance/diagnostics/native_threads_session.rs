//! Per-item native batch execution. Only real producer certificates enter the ledger.
use super::super::{
    native_coordinator::ApprovedBatchPrefix,
    native_orphans::{ApprovedEffects, EffectStamp},
};
use super::*;

fn from_stamp(value: Option<EffectStamp>) -> Option<Fingerprint> {
    value.map(|(len, sha256, modified)| Fingerprint {
        len,
        sha256,
        modified,
    })
}
fn to_stamp(value: &Option<Fingerprint>) -> Option<EffectStamp> {
    value
        .as_ref()
        .map(|s| (s.len, s.sha256.clone(), s.modified))
}
#[derive(Clone, Default)]
pub(in crate::maintenance::diagnostics) struct ApprovedThreadEffects {
    edges: BTreeMap<PathBuf, (Option<Fingerprint>, Option<Fingerprint>)>,
    home: Option<PathBuf>,
    app_root: Option<PathBuf>,
}
impl ApprovedThreadEffects {
    pub fn validate_current(&self, ctx: &Context) -> Result<()> {
        if self.home.as_ref() != Some(&ctx.home) || self.app_root.as_ref() != Some(&ctx.app_root) {
            return Err("线程效果凭据目标目录不匹配".into());
        }
        for (path, (_, after)) in &self.edges {
            if fingerprint(path)? != *after {
                return Err("已核线程效果发生外部变化".into());
            }
        }
        Ok(())
    }
    pub fn paths(&self) -> Vec<PathBuf> {
        self.edges.keys().cloned().collect()
    }
    pub fn effect_stamp(&self, path: &Path) -> Option<(Option<EffectStamp>, Option<EffectStamp>)> {
        self.edges
            .get(path)
            .map(|(a, b)| (to_stamp(a), to_stamp(b)))
    }
    pub fn effect(&self, path: &Path) -> Option<(Option<EffectStamp>, Option<EffectStamp>)> {
        self.effect_stamp(path)
    }
}
pub(in crate::maintenance::diagnostics) struct Session {
    home: PathBuf,
    app_root: PathBuf,
    plans: BTreeMap<String, PreparedAction>,
    approved: Scope,
    ledger: BTreeMap<PathBuf, Option<Fingerprint>>,
    effects: ApprovedThreadEffects,
    poisoned: Option<String>,
    called: BTreeSet<String>,
}
impl Session {
    pub fn effects(&self) -> &ApprovedThreadEffects {
        &self.effects
    }
    /// Full final receipt, including unchanged original thread inputs. Values come
    /// from original plans and verified producer updates, never from current hashes.
    pub fn verified_effects(&self, ctx: &Context) -> Result<ApprovedThreadEffects> {
        validate_ledger(ctx, self)?;
        let mut effects = self.effects.clone();
        for (path, after) in &self.ledger {
            if let Some((_, certified)) = effects.edges.get(path) {
                if certified != after {
                    return Err("线程最终凭据与核验台账不一致".into());
                }
                continue;
            }
            let mut original = None;
            for plan in self.plans.values() {
                if let Some(before) = plan.dependencies.get(path) {
                    if original.as_ref().is_some_and(|prior| prior != before) {
                        return Err("线程最终凭据有冲突的原始依赖".into());
                    }
                    original = Some(before.clone());
                }
            }
            let before = original.ok_or("线程台账没有原审批或已核生产者凭据")?;
            effects.edges.insert(path.clone(), (before, after.clone()));
        }
        effects.validate_current(ctx)?;
        Ok(effects)
    }
}
pub(in crate::maintenance::diagnostics) fn absorb_global(
    ctx: &Context,
    session: &mut Session,
    proof: &super::super::native_coordinator::ApprovedGlobalEffects,
) -> Result<()> {
    proof.validate_current(ctx)?;
    absorb(ctx, session, proof.paths(), |path| proof.effect_stamp(path))
}
pub(in crate::maintenance::diagnostics) fn absorb_ordinary(
    ctx: &Context,
    session: &mut Session,
    proof: &super::super::repair::ApprovedOrdinaryEffects,
) -> Result<()> {
    proof.validate_current(ctx)?;
    absorb(ctx, session, proof.paths(), |path| proof.effect_stamp(path))
}
pub(in crate::maintenance::diagnostics) fn absorb_project(
    ctx: &Context,
    session: &mut Session,
    proof: &super::super::project_state::ApprovedProjectEffects,
) -> Result<()> {
    proof.validate_current(ctx)?;
    absorb(ctx, session, proof.paths(), |path| proof.effect_stamp(path))
}
fn absorb(
    ctx: &Context,
    session: &mut Session,
    paths: Vec<PathBuf>,
    effect: impl Fn(&Path) -> Option<(Option<EffectStamp>, Option<EffectStamp>)>,
) -> Result<()> {
    if session.home != ctx.home || session.app_root != ctx.app_root || session.poisoned.is_some() {
        return Err("线程批次不具备继续接收已批准普通阶段效果的条件".into());
    }
    let mut after = session.ledger.clone();
    for (path, expected) in &session.ledger {
        match effect(path) {
            Some((before, changed)) => {
                if from_stamp(before) != *expected
                    || fingerprint(path)? != from_stamp(changed.clone())
                {
                    return Err("已批准普通阶段效果没有接续已核线程依赖".into());
                }
                after.insert(path.clone(), from_stamp(changed));
            }
            None => {
                if fingerprint(path)? != *expected {
                    return Err("普通阶段出现无凭据的外部依赖变化".into());
                }
            }
        }
    }
    let mut effects = session.effects.clone();
    for path in paths {
        let (before, current) = effect(&path).ok_or("普通阶段缺少效果凭据")?;
        let before = from_stamp(before);
        let current = from_stamp(current);
        merge_edge(&mut effects, path.clone(), before, current.clone())?;
        after.insert(path, current);
    }
    session.effects = effects;
    session.ledger = after;
    Ok(())
}
fn merge_edge(
    edges: &mut ApprovedThreadEffects,
    path: PathBuf,
    before: Option<Fingerprint>,
    after: Option<Fingerprint>,
) -> Result<()> {
    if let Some((original, prior)) = edges.edges.get(&path).cloned() {
        if prior != before {
            return Err("已审核前缀的依赖链不连续，未批准线程后继写入".into());
        }
        edges.edges.insert(path, (original, after));
    } else {
        edges.edges.insert(path, (before, after));
    }
    Ok(())
}
fn begin(
    ctx: &Context,
    plans: &[PreparedAction],
    orphans: &ApprovedEffects,
    batch: Option<&ApprovedBatchPrefix>,
    resync: Option<&super::super::native_router::ApprovedResyncEffects>,
) -> Result<Session> {
    let mut effects = ApprovedThreadEffects {
        home: Some(ctx.home.clone()),
        app_root: Some(ctx.app_root.clone()),
        ..Default::default()
    };
    if let Some(batch) = batch {
        if resync.is_none() {
            batch.validate_current(ctx)?;
        }
        for path in batch.paths() {
            let (before, after) = batch
                .effect_stamp(&path)
                .ok_or("批次前缀缺少原生效果凭据")?;
            merge_edge(&mut effects, path, from_stamp(before), from_stamp(after))?;
        }
    }
    if let Some(resync) = resync {
        resync.validate_current(ctx)?;
        if batch.is_none_or(|batch| !batch.plan().same_plan(resync.plan())) {
            return Err("路由重同步证书不属于本批次审批".into());
        }
        // Resync is a cumulative original -> final certificate. It supersedes the
        // intermediate pure-preparation edge only when its original is identical.
        for path in resync.paths() {
            let (before, after) = resync.effect_stamp(&path).ok_or("路由重同步缺少效果凭据")?;
            let before = from_stamp(before);
            let after = from_stamp(after);
            if effects
                .edges
                .get(&path)
                .is_some_and(|(original, _)| original != &before)
            {
                return Err("路由重同步与配置审批的原依赖不一致".into());
            }
            effects.edges.insert(path, (before, after));
        }
    }
    for path in orphans.paths() {
        let (before, after) = orphans
            .effect_stamp(&path)
            .ok_or("孤儿前缀缺少原生效果凭据")?;
        merge_edge(&mut effects, path, from_stamp(before), from_stamp(after))?;
    }
    let mut ledger = BTreeMap::new();
    let mut approved = Scope::default();
    let mut selected = BTreeMap::new();
    for plan in plans {
        validate_context(ctx, plan)?;
        if selected.insert(plan.id.clone(), plan.clone()).is_some() {
            return Err("线程审批包含重复项目".into());
        }
        approved.extend(&plan.scope);
        for (path, original) in &plan.dependencies {
            let current = match effects.edges.get(path) {
                Some((before, after)) => {
                    if before != original {
                        return Err("线程前缀原值不属于当前审批".into());
                    }
                    after.clone()
                }
                None => original.clone(),
            };
            if ledger.get(path).is_some_and(|prior| *prior != current) {
                return Err("线程原依赖集合不一致".into());
            }
            ledger.insert(path.clone(), current);
        }
    }
    // Even an empty thread selection forwards the concrete preceding effects to Project.
    for (path, (_, after)) in &effects.edges {
        if ledger.get(path).is_some_and(|prior| prior != after) {
            return Err("线程前缀最终依赖不一致".into());
        }
        ledger.insert(path.clone(), after.clone());
    }
    let result = Session {
        home: ctx.home.clone(),
        app_root: ctx.app_root.clone(),
        plans: selected,
        approved,
        ledger,
        effects,
        poisoned: None,
        called: BTreeSet::new(),
    };
    validate_ledger(ctx, &result)?;
    Ok(result)
}
pub(in crate::maintenance::diagnostics) fn begin_reviewed(
    ctx: &Context,
    plans: &[PreparedAction],
    prefix: &ApprovedEffects,
) -> Result<Session> {
    begin(ctx, plans, prefix, None, None)
}
pub(in crate::maintenance::diagnostics) fn begin_reviewed_after_batch_prefix(
    ctx: &Context,
    plans: &[PreparedAction],
    prefix: &ApprovedEffects,
    batch: &ApprovedBatchPrefix,
    resync: Option<&super::super::native_router::ApprovedResyncEffects>,
) -> Result<Session> {
    begin(ctx, plans, prefix, Some(batch), resync)
}
fn validate_ledger(ctx: &Context, session: &Session) -> Result<()> {
    if let Some(reason) = &session.poisoned {
        return Err(format!("线程批次存在未核验效果，拒绝后继写入：{reason}"));
    }
    if ctx.home != session.home || ctx.app_root != session.app_root {
        return Err("线程批次目标目录不匹配".into());
    }
    for (path, expected) in &session.ledger {
        if fingerprint(path)? != *expected {
            return Err("线程批次依赖被外部更新，未将当前值重新签为批准效果".into());
        }
    }
    Ok(())
}
#[derive(Clone, PartialEq, Eq)]
struct Content {
    len: u64,
    sha256: String,
}
fn bytes(value: &[u8]) -> Content {
    Content {
        len: value.len() as u64,
        sha256: format!("{:x}", Sha256::digest(value)),
    }
}
fn content(value: &Option<Fingerprint>) -> Option<Content> {
    value.as_ref().map(|f| Content {
        len: f.len,
        sha256: f.sha256.clone(),
    })
}
struct FileOracle {
    before: Option<Content>,
    after: Option<Content>,
}
#[derive(Clone)]
struct Sql {
    sql: &'static str,
    args: Vec<String>,
}
struct Program {
    transaction: bool,
    steps: Vec<Sql>,
}
struct Oracle {
    files: BTreeMap<PathBuf, FileOracle>,
    databases: BTreeMap<PathBuf, BTreeSet<String>>,
}
fn add_file(
    session: &Session,
    files: &mut BTreeMap<PathBuf, FileOracle>,
    path: PathBuf,
    before: Option<&[u8]>,
    after: Option<&[u8]>,
) -> Result<()> {
    let before = before.map(bytes);
    let after = after.map(bytes);
    if session.ledger.get(&path).map(content) != Some(before.clone()) {
        return Err("实际线程文件计划与已批准依赖不同".into());
    }
    if let Some(prior) = files.get(&path) {
        if prior.before != before || prior.after != after {
            return Err("线程条目的文件效果冲突".into());
        }
    }
    files.insert(path, FileOracle { before, after });
    Ok(())
}
fn digest(connection: &rusqlite::Connection) -> Result<String> {
    super::super::native_orphans::logical_database_digest(connection)
}
fn oracle(ctx: &Context, session: &Session, fresh: &PreparedAction) -> Result<Oracle> {
    let mut files = BTreeMap::new();
    let mut programs: BTreeMap<PathBuf, Program> = BTreeMap::new();
    let mut file = |f: &native::RolloutFileChange| {
        add_file(
            session,
            &mut files,
            f.physical_path.clone(),
            Some(&f.original),
            Some(&f.replacement),
        )
    };
    match &fresh.body {
        Body::Missing(p) => {
            for change in &p.changes {
                let program = programs
                    .entry(change.db.clone())
                    .or_insert_with(|| Program {
                        transaction: false,
                        steps: vec![],
                    });
                match &change.action {
                native::MissingRolloutAction::Archive=>program.steps.push(Sql{sql:"UPDATE threads SET archived = 1 WHERE id = ?1 AND COALESCE(archived, 0) = 0",args:vec![change.id.clone()]}),
                native::MissingRolloutAction::Restore{source,destination}=>{let original=fs::read(source).map_err(err)?;if session.ledger.get(source).map(content)!=Some(Some(bytes(&original))){return Err("恢复来源不等于已核验历史备份".into());}add_file(session,&mut files,destination.clone(),None,Some(&original))?;}
            }
            }
        }
        Body::Write(p) => {
            for f in &p.files {
                file(f)?;
                for db in &f.databases {
                    programs.entry(db.clone()).or_insert_with(|| Program {
                        transaction: true,
                        steps: vec![],
                    });
                }
            }
            if let Some(j) = &p.model_restore_journal {
                add_file(
                    session,
                    &mut files,
                    j.path.clone(),
                    j.original.as_deref(),
                    Some(&j.replacement),
                )?;
            }
            for change in &p.db_changes {
                programs
                    .entry(change.db.clone())
                    .or_insert_with(|| Program {
                        transaction: true,
                        steps: vec![],
                    })
                    .steps
                    .push(Sql {
                        sql: if p.kind == native::WriteRepairKind::Provider {
                            "UPDATE threads SET model_provider = ?2 WHERE id = ?1"
                        } else {
                            "UPDATE threads SET model = ?2 WHERE id = ?1"
                        },
                        args: vec![change.id.clone(), change.replacement.clone()],
                    });
            }
        }
        Body::Legacy(p) => {
            for db in &p.databases {
                for f in &db.files {
                    file(f)?;
                }
                programs.insert(db.db.clone(),Program{transaction:false,steps:vec![Sql{sql:"UPDATE threads SET model_provider = ?1 WHERE model_provider = ?2 OR model_provider = ?3 OR (model_provider LIKE 'openai' AND model_provider <> ?1)",args:vec!["openai".into(),"aimai1".into(),"aimami_codex_router".into()]}]});
            }
        }
        Body::Response(p) => {
            for f in &p.files {
                file(&f.file)?;
            }
        }
        Body::Instructions(p) => {
            for f in &p.rollouts {
                file(&f.file)?;
            }
            for s in &p.snapshots {
                add_file(
                    session,
                    &mut files,
                    s.path.clone(),
                    Some(&s.original),
                    Some(&s.replacement),
                )?;
            }
        }
        Body::Noop => (),
        Body::DeferredProvider => return Err("线程依赖仍未具备，不能提前执行".into()),
    }
    let mut databases = BTreeMap::new();
    for (db, program) in programs {
        let allowed = super::super::snapshot::with_snapshot_path(&db, |_, connection| {
            let mut allowed = BTreeSet::from([digest(connection)?]);
            if program.transaction {
                connection.execute_batch("BEGIN IMMEDIATE").map_err(err)?;
                let mut failed = false;
                for step in &program.steps {
                    if connection
                        .execute(step.sql, rusqlite::params_from_iter(&step.args))
                        .is_err()
                    {
                        failed = true;
                        break;
                    }
                }
                if failed {
                    if !connection.is_autocommit() {
                        connection.execute_batch("ROLLBACK").map_err(err)?;
                    }
                } else if connection.execute_batch("COMMIT").is_err() && !connection.is_autocommit()
                {
                    connection.execute_batch("ROLLBACK").map_err(err)?;
                }
                allowed.insert(digest(connection)?);
            } else {
                for step in &program.steps {
                    let executed =
                        connection.execute(step.sql, rusqlite::params_from_iter(&step.args));
                    allowed.insert(digest(connection)?);
                    if executed.is_err() {
                        break;
                    }
                }
            }
            Ok(allowed)
        })?;
        databases.insert(db, allowed);
    }
    let _ = ctx;
    Ok(Oracle { files, databases })
}
fn verify_effects(
    session: &Session,
    oracle: &Oracle,
) -> Result<BTreeMap<PathBuf, Option<Fingerprint>>> {
    let mut after = session.ledger.clone();
    let mut database_files = BTreeSet::new();
    for (db, allowed) in &oracle.databases {
        let wal = PathBuf::from(format!("{}-wal", db.display()));
        let before = (fingerprint(db)?, fingerprint(&wal)?);
        let logical =
            super::super::snapshot::with_snapshot_path(db, |_, connection| digest(connection))?;
        let current = (fingerprint(db)?, fingerprint(&wal)?);
        if before != current || !allowed.contains(&logical) {
            return Err("线程数据库变化不等于原始或已核原生SQL效果".into());
        }
        after.insert(db.clone(), current.0);
        after.insert(wal.clone(), current.1);
        database_files.extend([db.clone(), wal]);
    }
    for (path, expected) in &session.ledger {
        if database_files.contains(path) {
            continue;
        }
        let current = fingerprint(path)?;
        if let Some(file) = oracle.files.get(path) {
            let actual = content(&current);
            if actual != file.before && actual != file.after {
                return Err("线程历史变化不等于原始或精确批准字节".into());
            }
            after.insert(path.clone(), current);
        } else if current != *expected {
            return Err("线程操作改变了未批准的依赖，拒绝续签当前值".into());
        }
    }
    Ok(after)
}
fn failed(ctx: &Context, id: &str, detail: &str) -> Outcome {
    let check = inspect_one(ctx, id).unwrap_or_else(|_| {
        diagnostic(
            id,
            "warning",
            "线程批次复检",
            "无法完整读取当前线程状态。".into(),
            false,
        )
    });
    Outcome {
        id: id.into(),
        success: false,
        changed: 0,
        detail: detail.into(),
        backups: vec![],
        diagnostic: check,
        warnings: vec![],
        partial_effects_possible: false,
    }
}
pub(in crate::maintenance::diagnostics) fn apply_item(
    ctx: &Context,
    session: &mut Session,
    id: &str,
) -> Result<Outcome> {
    apply_item_inner(ctx, session, id, || {})
}
#[cfg(test)]
pub(in crate::maintenance::diagnostics) fn apply_item_with_hook(
    ctx: &Context,
    session: &mut Session,
    id: &str,
    after_native: impl FnOnce(),
) -> Result<Outcome> {
    apply_item_inner(ctx, session, id, after_native)
}
fn apply_item_inner(
    ctx: &Context,
    session: &mut Session,
    id: &str,
    after_native: impl FnOnce(),
) -> Result<Outcome> {
    if let Err(error) = validate_ledger(ctx, session) {
        session.poisoned = Some(error.clone());
        return Err(error);
    }
    let reviewed = session
        .plans
        .get(id)
        .cloned()
        .ok_or("该线程条目不在原始审批中，未添加新修复目标")?;
    const ORDER: [&str; 6] = [
        "thread_missing_rollouts",
        "thread_provider_consistency",
        "thread_model_catalog_consistency",
        "thread_poisoned_instructions",
        "thread_response_item_ids",
        "legacy_migrated_threads",
    ];
    let next = ORDER.into_iter().find(|candidate| {
        session.plans.contains_key(*candidate) && !session.called.contains(*candidate)
    });
    if next != Some(id) {
        return Err("线程条目重复派发或不符合原生批次顺序，未执行".into());
    }
    session.called.insert(id.into());
    // validate_context retains original DB membership/route state; config/file bytes
    // are checked against the producer ledger, not the obsolete original fingerprints.
    validate_context(ctx, &reviewed)?;
    let fresh = match prepare_with_backup(ctx, id, &reviewed.backup_root) {
        Ok(p) => p,
        Err(_) => {
            validate_ledger(ctx, session)?;
            return Ok(failed(
                ctx,
                id,
                "原生依赖未满足，本项未执行；后续独立条目仍由批次驱动逐项处理。",
            ));
        }
    };
    if !session.approved.contains(&fresh.scope) {
        return Err("当前线程条目产生原审批之外的新目标，未执行".into());
    }
    for (path, current) in &fresh.dependencies {
        if session.ledger.get(path) != Some(current) {
            return Err("线程新计划含未批准或已外部变化的依赖".into());
        }
    }
    let expected = oracle(ctx, session, &fresh)?;
    validate_ledger(ctx, session)?;
    let mut outcome = execute(ctx, &fresh);
    after_native();
    match verify_effects(session, &expected) {
        Ok(after) => {
            for (path, current) in &after {
                if session.ledger.get(path) != Some(current) {
                    let original = session
                        .effects
                        .edges
                        .get(path)
                        .map(|(before, _)| before.clone())
                        .unwrap_or_else(|| session.ledger.get(path).cloned().flatten());
                    session
                        .effects
                        .edges
                        .insert(path.clone(), (original, current.clone()));
                }
            }
            session.ledger = after;
            if !outcome.success {
                outcome.warnings.push("本项失败后的实际数据库/文件状态均已核对为原值或批准的原生效果；后续独立条目可继续。".into());
            }
        }
        Err(error) => {
            session.poisoned = Some(error.clone());
            outcome.success = false;
            outcome.partial_effects_possible = true;
            outcome.detail = format!("{error}；已完成效果与备份保留，后续线程写入被拒绝。");
        }
    }
    Ok(outcome)
}
