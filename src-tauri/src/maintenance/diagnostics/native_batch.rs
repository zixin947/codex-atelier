//! Production adapter for the recovered single-pass native fix_all driver.
//! Review fixes the original source union and every finite config alternative.
use super::{
    inspect, native_auth, native_catalog, native_coordinator as coordinator, native_orphans,
    native_router, native_threads, project_state, repair, Context, DiagnosticRepairPlan, Result,
};
use atelier_maintenance_core::{
    batch_repair::{self, BatchHost, BatchItem, ConfigPreparation, ItemFailure},
    manager::{self, FixResult},
    process::{self, ProcessContext, ProcessPolicy, Runner},
    transition_journal::TransitionTarget,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    io,
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Clone)]
enum ResyncPlan {
    Source(
        native_router::ReviewedPlan,
        native_router::BoundBatchResyncToken,
    ),
    ObservedOff(native_router::BoundBatchResyncToken),
}
impl ResyncPlan {
    fn token(&self) -> &native_router::BoundBatchResyncToken {
        match self {
            Self::Source(_, token) | Self::ObservedOff(token) => token,
        }
    }
}
pub(super) struct ReviewedBatch {
    identity: Arc<()>,
    home: PathBuf,
    app_root: PathBuf,
    backup: PathBuf,
    ids: Vec<String>,
    target: TransitionTarget,
    config: coordinator::PreparedBatch,
    router: Option<ResyncPlan>,
    global: Option<coordinator::PreparedGlobal>,
    successor: native_router::BoundBatchSuccessor,
    orphan: Option<native_orphans::Prepared>,
    threads: Vec<native_threads::PreparedAction>,
    ordinary: BTreeMap<String, repair::PreparedOrdinary>,
    view: DiagnosticRepairPlan,
}
impl ReviewedBatch {
    pub fn view(&self) -> &DiagnosticRepairPlan {
        &self.view
    }
    pub fn item_ids(&self) -> &[String] {
        &self.ids
    }
    pub fn home(&self) -> &Path {
        &self.home
    }
    pub fn app_root(&self) -> &Path {
        &self.app_root
    }
    pub fn target(&self) -> TransitionTarget {
        self.target
    }
    pub fn backup_root(&self) -> &Path {
        &self.backup
    }
    pub fn config_outputs(&self) -> Vec<Option<Vec<u8>>> {
        let mut outputs = self
            .config
            .config_candidates()
            .into_iter()
            .map(|(_, b)| b)
            .collect::<Vec<_>>();
        if let Some(router) = &self.router {
            outputs.extend(
                router
                    .token()
                    .config_candidates()
                    .into_iter()
                    .filter_map(|(_, b)| b.ok()),
            );
        }
        if let Some(global) = &self.global {
            outputs.extend(global.approved_outputs());
        }
        outputs.extend(self.successor.outputs());
        outputs
    }
    pub fn approves_config(&self, bytes: Option<&[u8]>) -> bool {
        self.config_outputs()
            .iter()
            .any(|candidate| candidate.as_deref() == bytes)
    }
    pub fn official_catalog(&self) -> Option<&native_orphans::OfficialCatalog> {
        self.orphan.as_ref().map(|p| p.official_catalog())
    }
    pub fn validate_initial(&self, ctx: &Context) -> Result<()> {
        if self.home != ctx.home || self.app_root != ctx.app_root {
            return Err("批次预览目录不匹配".into());
        }
        self.config.validate_initial(ctx)?;
        if let Some(router) = &self.router {
            router.token().validate_initial(ctx)?;
        }
        self.successor.validate_initial(ctx)?;
        if let Some(plan) = &self.global {
            plan.validate_initial(ctx)?;
        }
        if let Some(plan) = &self.orphan {
            native_orphans::validate_plan(ctx, plan)?;
        }
        native_threads::validate_sequence(ctx, &self.threads)?;
        for ticket in self.ordinary.values() {
            ticket.validate_initial(ctx)?;
        }
        if self.target == TransitionTarget::NativeOff && inspect::config(ctx).is_ok() {
            native_threads::review_provider_target(
                ctx,
                "openai",
                self.orphan.is_some()
                    || self
                        .threads
                        .iter()
                        .any(native_threads::PreparedAction::changes_provider_target),
            )?;
        }
        Ok(())
    }
}
fn selected(report: &super::DiagnosticReport) -> BTreeSet<String> {
    report
        .items
        .iter()
        .filter(|i| i.fixable && i.status != "ok")
        .map(|i| i.id.clone())
        .collect()
}
fn needs_resync(ids: &[String], target: TransitionTarget) -> bool {
    ids.iter().any(|id| {
        matches!(
            id.as_str(),
            "config_toml_syntax"
                | "config_third_party"
                | "config_profile_conflict"
                | "router_http_contract"
                | "catalog_integrity"
                | "catalog_path_validity"
        ) || id == "config_stale" && target == TransitionTarget::RouterOn
    })
}
fn actual_source(ctx: &Context) -> Result<Option<native_catalog::Source>> {
    let missing = |path: &Path| -> Result<bool> {
        match std::fs::symlink_metadata(path) {
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(true),
            Err(e) => Err(e.to_string()),
            Ok(_) => Ok(false),
        }
    };
    if missing(&ctx.home.join("codexmate/relay/state.json"))?
        && missing(&ctx.app_root.join("gateway/catalog.json"))?
    {
        Ok(None)
    } else {
        native_catalog::context_source(ctx).map(Some)
    }
}
pub(super) fn prepare(ctx: &Context) -> Result<ReviewedBatch> {
    prepare_internal(ctx, actual_source(ctx)?.as_ref(), None)
}
pub(super) fn prepare_with_catalog(
    ctx: &Context,
    official: &native_orphans::OfficialCatalog,
) -> Result<ReviewedBatch> {
    prepare_internal(ctx, actual_source(ctx)?.as_ref(), Some(official))
}
pub(super) fn prepare_from_source(
    ctx: &Context,
    source: &native_catalog::Source,
    official: Option<&native_orphans::OfficialCatalog>,
) -> Result<ReviewedBatch> {
    prepare_internal(ctx, Some(source), official)
}
fn prepare_internal(
    ctx: &Context,
    source: Option<&native_catalog::Source>,
    official: Option<&native_orphans::OfficialCatalog>,
) -> Result<ReviewedBatch> {
    let report = inspect::run(ctx)?;
    prepare_reviewed(ctx, source, official, &report)
}
pub(super) fn prepare_from_report(
    ctx: &Context,
    source: Option<&native_catalog::Source>,
    report: &super::DiagnosticReport,
) -> Result<ReviewedBatch> {
    prepare_reviewed(ctx, source, None, report)
}
fn prepare_reviewed(
    ctx: &Context,
    source: Option<&native_catalog::Source>,
    official: Option<&native_orphans::OfficialCatalog>,
    report: &super::DiagnosticReport,
) -> Result<ReviewedBatch> {
    let ids = selected(report).into_iter().collect::<Vec<_>>();
    if ids.is_empty() {
        return Err("当前没有可自动修复项目".into());
    }
    let snapshot = source
        .map(|source| native_router::snapshot(ctx, source))
        .transpose()?;
    let ticket = uuid::Uuid::new_v4().to_string();
    let backup = ctx
        .app_root
        .join("maintenance/diagnostic-backups")
        .join(&ticket);
    let config = coordinator::prepare_batch(ctx, &ids, &backup.join("auth-config"))?;
    let target = if let Some(snapshot) = snapshot {
        snapshot.target()
    } else {
        if config.mode() != atelier_maintenance_core::auth::RouterMode::Off {
            return Err("缺少真实路由源且观察到接管模式，不能作为默认Off维护".into());
        }
        TransitionTarget::NativeOff
    };
    // Seal the source/owner even when the original driver does not request its
    // single mid-batch resync (for example a Global-only repair).
    let route_seed = if let Some(source) = source {
        let actual_proxy = match source {
            native_catalog::Source::Gateway { .. } => ctx.proxy_handle.clone(),
            native_catalog::Source::Native { .. } => None,
        };
        let (plan, token) = native_router::prepare_bound_batch_resync_from_source(
            ctx,
            source,
            target,
            &backup.join("resync"),
            actual_proxy,
            &config,
        )?;
        ResyncPlan::Source(plan, token)
    } else {
        ResyncPlan::ObservedOff(native_router::prepare_bound_observed_off(ctx, &config)?)
    };
    let router = if needs_resync(&ids, target) {
        Some(route_seed.clone())
    } else {
        None
    };
    let global = if ids.iter().any(|id| id == "config_global_validity") {
        Some(coordinator::prepare_batch_global(
            ctx,
            &config,
            router.as_ref().map(ResyncPlan::token),
        )?)
    } else {
        None
    };
    let successor =
        native_router::prepare_batch_successor(ctx, route_seed.token(), global.as_ref())?;
    let orphan = if ids.iter().any(|id| native_orphans::handles(id)) {
        Some(if let Some(catalog) = official {
            native_orphans::prepare_with_catalog(ctx, catalog)?
        } else {
            native_orphans::prepare(ctx)?
        })
    } else {
        None
    };
    let threads = native_threads::prepare_sequence(ctx, &ids, &backup.join("threads"))?;
    let mut ordinary = BTreeMap::new();
    for id in &ids {
        if matches!(
            id.as_str(),
            "residual_manifest"
                | "residual_cache"
                | "project_state_integrity"
                | "state_db_relocation"
        ) {
            ordinary.insert(id.clone(), repair::prepare_ordinary(ctx, id)?);
        }
    }
    let mut changes=vec!["按原生顺序执行：认证完整性 → 解锁残留 → takeover备份 → syntax/third-party/profile/stale文本 → 必要时一次路由重同步 → 孤儿联合收养 → 固定11普通阶段".into()];
    changes.extend(config.config_candidates().iter().map(|(key, bytes)| {
        format!(
            "预审配置候选#{key}：{}字节",
            bytes.as_ref().map_or(0, Vec::len)
        )
    }));
    if let Some(router) = &router {
        changes.extend(
            router
                .token()
                .config_candidates()
                .iter()
                .map(|(key, value)| {
                    format!(
                        "重同步候选#{key}：{}",
                        match value {
                            Ok(bytes) => format!("{}字节", bytes.as_ref().map_or(0, Vec::len)),
                            Err(error) => format!("此分支将明确失败：{error}"),
                        }
                    )
                }),
        );
    }
    if let Some(plan) = &orphan {
        changes.extend(native_orphans::preview_lines(plan));
    }
    for plan in &threads {
        changes.extend(native_threads::preview_lines(plan));
    }
    if let Some(plan) = &global {
        changes.extend(plan.preview_lines());
    }
    for ticket in ordinary.values() {
        changes.extend(ticket.ticket().view.changes.clone());
    }
    let view=DiagnosticRepairPlan {ticket,title:"预览原生全部维护批次".into(),item_ids:ids.clone(),changes,
        warnings:vec!["所有原始源在首写前整体校验；普通失败保留已确认进展并继续后续项目，无法证明的部分效果阻止后继写入。".into()],
        requires_desktop_closed:!super::quiet_window::can_run_with_desktop(&ids)};
    let plan = ReviewedBatch {
        identity: Arc::new(()),
        home: ctx.home.clone(),
        app_root: ctx.app_root.clone(),
        backup,
        ids,
        target,
        config,
        router,
        global,
        successor,
        orphan,
        threads,
        ordinary,
        view,
    };
    plan.validate_initial(ctx)?;
    Ok(plan)
}
fn as_report(report: super::DiagnosticReport) -> manager::DiagnosticReport {
    manager::DiagnosticReport {
        items: report
            .items
            .into_iter()
            .map(|i| manager::DiagnosticItem {
                id: i.id,
                label: i.label,
                status: i.status,
                detail: i.detail,
                fixable: i.fixable,
            })
            .collect(),
        has_issues: report.has_issues,
        transition_in_flight: report.transition_in_flight,
    }
}
fn io_error(error: impl std::fmt::Display) -> io::Error {
    io::Error::other(error.to_string())
}
struct Host<'a, R> {
    ctx: &'a Context,
    plan: &'a ReviewedBatch,
    guard: &'a repair::MutationGuard,
    runner: &'a R,
    process: &'a ProcessContext,
    initial: bool,
    blocked: Option<String>,
    config: Option<coordinator::BatchSession>,
    resync: Option<native_router::ApprovedResyncEffects>,
    global_branch: Option<usize>,
    orphan_effects: native_orphans::ApprovedEffects,
    threads: Option<native_threads::Session>,
}
impl<R: Runner> Host<'_, R> {
    fn ensure_config(&mut self) -> Result<()> {
        if self.config.is_none() {
            self.config = Some(coordinator::begin_batch(self.ctx, &self.plan.config)?);
        }
        Ok(())
    }
    fn ensure_threads(&mut self) -> Result<()> {
        if self.threads.is_none() {
            self.ensure_config()?;
            self.threads = Some(native_threads::begin_reviewed_after_batch_prefix(
                self.ctx,
                &self.plan.threads,
                &self.orphan_effects,
                self.config.as_ref().unwrap().prefix(),
                self.resync.as_ref(),
            )?);
        }
        Ok(())
    }
}
impl<R: Runner> BatchHost for Host<'_, R> {
    type Error = io::Error;
    type OfficialCatalog = native_orphans::OfficialCatalog;
    fn diagnose(&mut self) -> std::result::Result<manager::DiagnosticReport, io::Error> {
        if !self.initial {
            self.guard.validate(self.ctx).map_err(io_error)?;
            self.plan.validate_initial(self.ctx).map_err(io_error)?;
        }
        let report = inspect::run(self.ctx).map_err(io_error)?;
        if !self.initial {
            if selected(&report)
                .iter()
                .any(|id| !self.plan.ids.contains(id))
            {
                return Err(io_error("首写前出现未审批的新修复项目"));
            }
            self.initial = true;
        }
        Ok(as_report(report))
    }
    fn router_target(&self) -> TransitionTarget {
        self.plan.target
    }
    fn ensure_quiet_window(&mut self) -> std::result::Result<(), io::Error> {
        self.guard.validate(self.ctx).map_err(io_error)?;
        if self
            .ctx
            .runtime
            .as_ref()
            .is_some_and(|r| r.active_requests > 0)
        {
            return Err(io_error("网关仍有在途写入"));
        }
        let observed =
            process::preview_stop(self.runner, self.process, ProcessPolicy::DesktopTransition)
                .map_err(io_error)?;
        if !observed.targets.is_empty()
            || !observed.external_cli.is_empty()
            || !observed.protected_db_holders.is_empty()
        {
            return Err(io_error("实际进程观察尚未确认维护写者静默"));
        }
        Ok(())
    }
    fn prepare_config(
        &mut self,
        item: ConfigPreparation,
    ) -> std::result::Result<String, io::Error> {
        self.ensure_config().map_err(io_error)?;
        coordinator::batch_prepare_config(self.ctx, self.config.as_mut().unwrap(), item)
            .map_err(io_error)
    }
    fn resync_config(&mut self) -> std::result::Result<(), io::Error> {
        self.ensure_config().map_err(io_error)?;
        let reviewed = self
            .plan
            .router
            .as_ref()
            .ok_or_else(|| io_error("此批次没有预审路由重同步候选"))?;
        let (outcome, proof) = match reviewed {
            ResyncPlan::Source(plan, token) => native_router::apply_bound_batch_resync(
                self.ctx,
                plan,
                token,
                self.config.as_ref().unwrap().prefix(),
                &self.plan.backup.join("resync"),
                self.guard,
            ),
            ResyncPlan::ObservedOff(token) => native_router::apply_bound_observed_off(
                self.ctx,
                token,
                self.config.as_ref().unwrap().prefix(),
                &self.plan.backup.join("resync"),
                self.guard,
            ),
        }
        .map_err(io_error)?;
        self.resync = Some(proof);
        if let Some(error) = outcome.failure {
            return Err(io_error(error));
        }
        Ok(())
    }
    fn dispatch(
        &mut self,
        item: BatchItem,
        _official: Option<&Self::OfficialCatalog>,
    ) -> std::result::Result<FixResult, ItemFailure<io::Error>> {
        self.dispatch_actual(item)
            .map_err(|e| ItemFailure::new(io_error(e)))
    }
}
impl<R: Runner> Host<'_, R> {
    fn dispatch_actual(&mut self, item: BatchItem) -> Result<FixResult> {
        let id = item.id();
        if let Some(reason) = &self.blocked {
            return Err(format!("前序部分效果未确认，停止后继写入：{reason}"));
        }
        if native_auth::handles(id) {
            self.ensure_config()?;
            let outcome = coordinator::batch_auth(self.ctx, self.config.as_mut().unwrap(), id)?;
            return Ok(FixResult {
                fixed_ids: if outcome.success {
                    vec![id.into()]
                } else {
                    vec![]
                },
                failed_ids: if outcome.success {
                    vec![]
                } else {
                    vec![id.into()]
                },
                details: vec![outcome.detail],
            });
        }
        if item == BatchItem::OrphanProviders {
            let plan = self.plan.orphan.as_ref().ok_or("孤儿项目没有原始审批")?;
            let outcome = native_orphans::apply_verified(
                self.ctx,
                plan,
                &self.plan.backup.join("orphans"),
                self.guard,
            )?;
            self.orphan_effects = outcome.effects;
            let ids = self
                .plan
                .ids
                .iter()
                .filter(|id| native_orphans::handles(id))
                .cloned()
                .collect::<Vec<_>>();
            return Ok(FixResult {
                fixed_ids: if outcome.failure.is_none() {
                    ids.clone()
                } else {
                    vec![]
                },
                failed_ids: if outcome.failure.is_some() {
                    ids
                } else {
                    vec![]
                },
                details: outcome.details,
            });
        }
        self.ensure_threads()?;
        if native_threads::handles(id) {
            let outcome = native_threads::apply_item(self.ctx, self.threads.as_mut().unwrap(), id)?;
            return Ok(FixResult {
                fixed_ids: if outcome.success {
                    vec![id.into()]
                } else {
                    vec![]
                },
                failed_ids: if outcome.success {
                    vec![]
                } else {
                    vec![id.into()]
                },
                details: std::iter::once(outcome.detail)
                    .chain(outcome.warnings)
                    .collect(),
            });
        }
        if item == BatchItem::ConfigGlobalValidity {
            let plan = self.plan.global.as_ref().ok_or("全局配置没有原始审批")?;
            let (detail, proof) = coordinator::apply_batch_global(
                self.ctx,
                plan,
                self.threads.as_ref().unwrap().effects(),
                self.guard,
            )?;
            native_threads::absorb_global(self.ctx, self.threads.as_mut().unwrap(), &proof)?;
            self.global_branch = Some(proof.branch());
            return Ok(FixResult {
                fixed_ids: vec![id.into()],
                failed_ids: vec![],
                details: vec![detail],
            });
        }
        let ticket = self.plan.ordinary.get(id).ok_or("普通项目没有原始审批")?;
        if item == BatchItem::ProjectStateIntegrity {
            let prepared = ticket
                .ticket()
                .project_plan()
                .ok_or("项目阶段缺少原始typed计划")?;
            let outcome = project_state::apply_after_approved_prefix(
                self.ctx,
                prepared,
                self.guard,
                &self.plan.backup.join("project"),
                self.threads.as_ref().unwrap().effects(),
            )?;
            if let Some(proof) = &outcome.effects {
                native_threads::absorb_project(self.ctx, self.threads.as_mut().unwrap(), proof)?;
            } else if outcome.partial_effects_possible {
                self.blocked = Some(outcome.detail.clone());
            }
            return Ok(FixResult {
                fixed_ids: if outcome.success {
                    vec![id.into()]
                } else {
                    vec![]
                },
                failed_ids: if outcome.success {
                    vec![]
                } else {
                    vec![id.into()]
                },
                details: vec![outcome.detail],
            });
        }
        let outcome = repair::apply_ordinary_certified(
            self.ctx,
            ticket,
            self.guard,
            self.threads.as_ref().unwrap().effects(),
        )?;
        native_threads::absorb_ordinary(
            self.ctx,
            self.threads.as_mut().unwrap(),
            &outcome.effects,
        )?;
        if let Some(result) = outcome.result {
            Ok(FixResult {
                fixed_ids: result.fixed_ids,
                failed_ids: result.failed_ids,
                details: result.details,
            })
        } else {
            Ok(FixResult {
                fixed_ids: vec![],
                failed_ids: vec![id.into()],
                details: vec![outcome.failure.unwrap_or_else(|| "普通阶段未完成".into())],
            })
        }
    }
}
/// Only a verified original-plan effect chain can produce this final receipt.
/// Configuration is selected from sealed preview alternatives, never rendered
/// or adopted from arbitrary current bytes at commit time.
#[derive(Clone)]
pub(super) struct BatchReceipt {
    identity: Arc<()>,
    home: PathBuf,
    app_root: PathBuf,
    config_after: Option<Vec<u8>>,
    global_branch: Option<usize>,
    successor_applied: bool,
    expected: BTreeMap<PathBuf, Option<native_orphans::EffectStamp>>,
}
impl std::fmt::Debug for BatchReceipt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BatchReceipt")
            .field("paths", &self.expected.len())
            .finish_non_exhaustive()
    }
}
impl BatchReceipt {
    pub fn config_after(&self) -> Option<&[u8]> {
        self.config_after.as_deref()
    }
    pub fn effect(&self, path: &Path) -> Option<Option<native_orphans::EffectStamp>> {
        self.expected.get(path).cloned()
    }
    pub fn global_branch(&self) -> Option<usize> {
        self.global_branch
    }
    pub fn validate_current(&self, ctx: &Context, plan: &ReviewedBatch) -> Result<()> {
        if !Arc::ptr_eq(&self.identity, &plan.identity)
            || ctx.home != self.home
            || ctx.app_root != self.app_root
        {
            return Err("批次最终凭据不属于此预览或目录".into());
        }
        for (path, after) in &self.expected {
            if repair::ordinary_stamp(path)? != *after {
                return Err(format!("最终已批准效果发生变化：{}", path.display()));
            }
        }
        Ok(())
    }
}
/// Consumes the final cumulative receipt once. On any partial/uncertain failure
/// the Manager must discard it; no current bytes are signed as a new receipt.
pub(super) fn reconcile_after_batch(
    ctx: &Context,
    plan: &ReviewedBatch,
    receipt: &BatchReceipt,
    guard: &repair::MutationGuard,
) -> Result<(native_router::Outcome, BatchReceipt)> {
    receipt.validate_current(ctx, plan)?;
    if receipt.successor_applied {
        return Err("批次后继只允许执行一次；请重新预览".into());
    }
    let (outcome, effects, config_after) = native_router::apply_batch_successor(
        ctx,
        &plan.successor,
        receipt,
        plan.global.as_ref(),
        &plan.backup.join("final-router"),
        guard,
    )?;
    if let Some(error) = &outcome.failure {
        return Err(format!("批次后继部分完成：{error}；备份保留"));
    }
    if !outcome.verification.issues.is_empty() {
        return Err(format!(
            "批次后继Full验证失败：{}；备份保留",
            outcome
                .verification
                .issues
                .iter()
                .map(|i| i.message.as_str())
                .collect::<Vec<_>>()
                .join("；")
        ));
    }
    let mut next = receipt.clone();
    next.successor_applied = true;
    next.config_after = config_after;
    for (path, after) in effects {
        next.expected.insert(path, after);
    }
    next.validate_current(ctx, plan)?;
    Ok((outcome, next))
}
#[derive(Debug)]
pub(super) struct BatchExecution {
    pub outcome: batch_repair::BatchOutcome,
    pub receipt: Option<BatchReceipt>,
    pub receipt_failure: Option<String>,
}
impl<R: Runner> Host<'_, R> {
    fn final_receipt(&mut self) -> Result<BatchReceipt> {
        if let Some(reason) = &self.blocked {
            return Err(reason.clone());
        }
        self.guard.validate(self.ctx)?;
        self.ensure_threads()?;
        let effects = self.threads.as_ref().unwrap().verified_effects(self.ctx)?;
        let mut expected = BTreeMap::new();
        for (path, _) in self.plan.config.original_dependencies() {
            expected.insert(
                path.clone(),
                self.plan
                    .config
                    .original_effect_stamp(&path)
                    .ok_or("原始批次依赖缺少stamp")?,
            );
        }
        for ordinary in self.plan.ordinary.values() {
            for (path, before) in ordinary.approved_sources() {
                if expected.get(&path).is_some_and(|prior| prior != &before) {
                    return Err("最终票据原始依赖不一致".into());
                }
                expected.insert(path, before);
            }
        }
        for path in effects.paths() {
            let (before, after) = effects.effect_stamp(&path).ok_or("最终线程凭据缺少效果")?;
            if expected.get(&path).is_some_and(|prior| prior != &before) {
                return Err("最终效果起点与原始批次审批不一致".into());
            }
            expected.insert(path, after);
        }
        let mut candidates = self
            .plan
            .config
            .config_candidates()
            .into_iter()
            .map(|(_, bytes)| bytes)
            .collect::<Vec<_>>();
        if let Some(router) = &self.plan.router {
            candidates.extend(
                router
                    .token()
                    .config_candidates()
                    .into_iter()
                    .filter_map(|(_, bytes)| bytes.ok()),
            );
        }
        if let Some(global) = &self.plan.global {
            candidates.extend(global.approved_outputs());
        }
        use sha2::{Digest, Sha256};
        let after = expected
            .get(&self.ctx.config())
            .ok_or("最终配置缺少批准stamp")?;
        let content = after.as_ref().map(|(n, h, _)| (*n, h.clone()));
        let config_after = candidates
            .into_iter()
            .find(|bytes| {
                bytes
                    .as_deref()
                    .map(|b| (b.len() as u64, format!("{:x}", Sha256::digest(b))))
                    == content
            })
            .ok_or("最终配置不属于任何原始预审候选")?;
        // Preserve the route seed's read-only source/intent/owner dependencies,
        // including when no mid-batch resync was dispatched.
        self.plan.successor.extend_expected(&mut expected)?;
        let receipt = BatchReceipt {
            global_branch: self.global_branch,
            successor_applied: false,
            identity: self.plan.identity.clone(),
            home: self.ctx.home.clone(),
            app_root: self.ctx.app_root.clone(),
            config_after,
            expected,
        };
        receipt.validate_current(self.ctx, self.plan)?;
        Ok(receipt)
    }
}
/// Called only inside the Manager's live fence. This observes real writers using
/// the supplied concrete Runner; it never quits, signals, launches, or shells.
pub(super) fn apply_under_manager<R: Runner>(
    ctx: &Context,
    plan: &ReviewedBatch,
    guard: &repair::MutationGuard,
    runner: &R,
    process: &ProcessContext,
) -> std::result::Result<BatchExecution, batch_repair::BatchFailure<io::Error>> {
    let mut host = Host {
        ctx,
        plan,
        guard,
        runner,
        process,
        initial: false,
        blocked: None,
        config: None,
        resync: None,
        global_branch: None,
        orphan_effects: Default::default(),
        threads: None,
    };
    let outcome = batch_repair::run(&mut host, plan.official_catalog())?;
    let (receipt, receipt_failure) = match host.final_receipt() {
        Ok(receipt) => (Some(receipt), None),
        Err(error) => (None, Some(error)),
    };
    Ok(BatchExecution {
        outcome,
        receipt,
        receipt_failure,
    })
}
#[cfg(test)]
#[path = "native_batch_tests.rs"]
mod tests;
