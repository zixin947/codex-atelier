//! AiMaMi `fix_all @1003a69d8`: one initial report, conditional resync report,
//! fixed dependency order, ordinary errors accumulated without losing progress.
//! The enclosing Manager owns transition stages, compensation, locks and relaunch.
use crate::{
    manager::{DiagnosticReport, FixResult},
    policy::{self, QuietWindow},
    transition_journal::TransitionTarget,
};
use std::{collections::BTreeSet, error::Error, fmt};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigPreparation {
    Syntax,
    ThirdParty,
    Profile,
    Stale,
}
impl ConfigPreparation {
    pub fn id(self) -> &'static str {
        match self {
            Self::Syntax => "config_toml_syntax",
            Self::ThirdParty => "config_third_party",
            Self::Profile => "config_profile_conflict",
            Self::Stale => "config_stale",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BatchItem {
    AuthIntegrity,
    UnlockAuthResidue,
    TakeoverBackupOrphan,
    /// A single native adoption call fixes the DB/rollout orphan union.
    OrphanProviders,
    ThreadMissingRollouts,
    ThreadProviderConsistency,
    ThreadModelCatalogConsistency,
    ThreadPoisonedInstructions,
    ThreadResponseItemIds,
    LegacyMigratedThreads,
    ResidualManifest,
    ResidualCache,
    ConfigGlobalValidity,
    ProjectStateIntegrity,
    StateDbRelocation,
}
impl BatchItem {
    pub fn id(self) -> &'static str {
        match self {
            Self::AuthIntegrity => "auth_integrity",
            Self::UnlockAuthResidue => "router_unlock_auth_residue",
            Self::TakeoverBackupOrphan => "takeover_backup_orphan",
            Self::OrphanProviders => "db_orphan_providers",
            Self::ThreadMissingRollouts => "thread_missing_rollouts",
            Self::ThreadProviderConsistency => "thread_provider_consistency",
            Self::ThreadModelCatalogConsistency => "thread_model_catalog_consistency",
            Self::ThreadPoisonedInstructions => "thread_poisoned_instructions",
            Self::ThreadResponseItemIds => "thread_response_item_ids",
            Self::LegacyMigratedThreads => "legacy_migrated_threads",
            Self::ResidualManifest => "residual_manifest",
            Self::ResidualCache => "residual_cache",
            Self::ConfigGlobalValidity => "config_global_validity",
            Self::ProjectStateIntegrity => "project_state_integrity",
            Self::StateDbRelocation => "state_db_relocation",
        }
    }
}
/// Actual rodata @10181b9d0: eleven (pointer,length) pairs. Not report/UI order.
pub const ORDINARY_ORDER: [BatchItem; 11] = [
    BatchItem::ThreadMissingRollouts,
    BatchItem::ThreadProviderConsistency,
    BatchItem::ThreadModelCatalogConsistency,
    BatchItem::ThreadPoisonedInstructions,
    BatchItem::ThreadResponseItemIds,
    BatchItem::LegacyMigratedThreads,
    BatchItem::ResidualManifest,
    BatchItem::ResidualCache,
    BatchItem::ConfigGlobalValidity,
    BatchItem::ProjectStateIntegrity,
    BatchItem::StateDbRelocation,
];
#[derive(Debug)]
pub struct ItemFailure<E> {
    pub error: E,
    pub partial: FixResult,
}
impl<E> ItemFailure<E> {
    pub fn new(error: E) -> Self {
        Self {
            error,
            partial: FixResult::default(),
        }
    }
    pub fn from_partial(error: E, partial: FixResult) -> Self {
        Self { error, partial }
    }
}
/// No success/status booleans are supplied. Reports and receipts must come from
/// the application's concrete readers/writers and reviewed source/effect ledger.
pub trait BatchHost {
    type Error: Error + Send + Sync + 'static;
    type OfficialCatalog;
    fn diagnose(&mut self) -> Result<DiagnosticReport, Self::Error>;
    /// Project the actual source snapshot (Native RelayState or Gateway intent).
    fn router_target(&self) -> TransitionTarget;
    /// Validate the real MutationGuard and inspect actual writers. Do not stop or
    /// launch here: the Manager's actual fence remains alive around this driver.
    fn ensure_quiet_window(&mut self) -> Result<(), Self::Error>;
    fn prepare_config(&mut self, item: ConfigPreparation) -> Result<String, Self::Error>;
    fn resync_config(&mut self) -> Result<(), Self::Error>;
    /// The optional pool was prepared by the Manager before this batch. Its source
    /// bytes must still be checked at adoption after earlier auth/config effects.
    fn dispatch(
        &mut self,
        item: BatchItem,
        prepared: Option<&Self::OfficialCatalog>,
    ) -> Result<FixResult, ItemFailure<Self::Error>>;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    InitialDiagnosis,
    QuietWindow,
    Item(BatchItem),
    PrepareConfig(ConfigPreparation),
    ResyncConfig,
    ResyncDiagnosis,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StageResult {
    Completed,
    Failed(String),
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StageRecord {
    pub stage: Stage,
    pub result: StageResult,
}
#[derive(Clone, Debug, Default)]
pub struct BatchOutcome {
    pub fix: FixResult,
    pub trace: Vec<StageRecord>,
    pub initial_needs_fix: BTreeSet<String>,
    /// Successful preparation/deferred HTTP/catalog entries rechecked by resync.
    pub deferred: Vec<String>,
    pub initial_report: Option<DiagnosticReport>,
    pub resync_report: Option<DiagnosticReport>,
}
#[derive(Debug)]
pub struct BatchFailure<E> {
    pub stage: Stage,
    pub error: E,
    pub progress: BatchOutcome,
}
impl<E: fmt::Display> fmt::Display for BatchFailure<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "batch {:?}: {}", self.stage, self.error)
    }
}
impl<E: Error + 'static> Error for BatchFailure<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}
fn merge(to: &mut FixResult, from: FixResult) {
    to.fixed_ids.extend(from.fixed_ids);
    to.failed_ids.extend(from.failed_ids);
    to.details.extend(from.details);
}
fn record(out: &mut BatchOutcome, stage: Stage, result: StageResult) {
    out.trace.push(StageRecord { stage, result });
}
fn fail_id(out: &mut BatchOutcome, id: &str) {
    if !out.fix.failed_ids.iter().any(|s| s == id) {
        out.fix.failed_ids.push(id.into());
    }
}
fn deferred(out: &mut BatchOutcome, id: &str) {
    out.fix.fixed_ids.push(id.into());
    out.deferred.push(id.into());
}
fn dispatch<H: BatchHost>(
    host: &mut H,
    out: &mut BatchOutcome,
    item: BatchItem,
    prepared: Option<&H::OfficialCatalog>,
) {
    let stage = Stage::Item(item);
    match host.dispatch(item, prepared) {
        Ok(result) => {
            merge(&mut out.fix, result);
            record(out, stage, StageResult::Completed);
        }
        Err(failure) => {
            let message = failure.error.to_string();
            merge(&mut out.fix, failure.partial);
            if item == BatchItem::OrphanProviders {
                for id in ["db_orphan_providers", "rollout_orphan_providers"] {
                    if out.initial_needs_fix.contains(id) {
                        fail_id(out, id);
                    }
                }
                out.fix
                    .details
                    .push(format!("orphan thread adoption: {message}"));
            } else {
                fail_id(out, item.id());
                out.fix.details.push(format!("{}: {message}", item.id()));
            }
            record(out, stage, StageResult::Failed(message));
        }
    }
}
fn fatal<E>(mut out: BatchOutcome, stage: Stage, error: E) -> BatchFailure<E>
where
    E: fmt::Display,
{
    record(&mut out, stage, StageResult::Failed(error.to_string()));
    BatchFailure {
        stage,
        error,
        progress: out,
    }
}
/// Execute the native batch exactly once. There is no speculative retry/multi-pass
/// loop, no implicit state commit and no blanket conversion of failures to healthy.
pub fn run<H: BatchHost>(
    host: &mut H,
    prepared: Option<&H::OfficialCatalog>,
) -> Result<BatchOutcome, BatchFailure<H::Error>> {
    let mut out = BatchOutcome::default();
    let report = match host.diagnose() {
        Ok(report) => report,
        Err(e) => return Err(fatal(out, Stage::InitialDiagnosis, e)),
    };
    out.initial_needs_fix = report
        .items
        .iter()
        .filter(|i| i.fixable && i.status != "ok")
        .map(|i| i.id.clone())
        .collect();
    out.initial_report = Some(report);
    record(&mut out, Stage::InitialDiagnosis, StageResult::Completed);
    if out.initial_needs_fix.is_empty() {
        out.fix.details.push("所有项目正常，无需修复".into());
        return Ok(out);
    }
    if out
        .initial_needs_fix
        .iter()
        .any(|id| policy::quiet_window(id) == QuietWindow::Required)
    {
        if let Err(e) = host.ensure_quiet_window() {
            return Err(fatal(out, Stage::QuietWindow, e));
        }
        record(&mut out, Stage::QuietWindow, StageResult::Completed);
    }
    let target = host.router_target();
    for item in [
        BatchItem::AuthIntegrity,
        BatchItem::UnlockAuthResidue,
        BatchItem::TakeoverBackupOrphan,
    ] {
        if out.initial_needs_fix.contains(item.id()) {
            dispatch(host, &mut out, item, prepared);
        }
    }
    let mut resync = false;
    for prep in [
        ConfigPreparation::Syntax,
        ConfigPreparation::ThirdParty,
        ConfigPreparation::Profile,
        ConfigPreparation::Stale,
    ] {
        if !out.initial_needs_fix.contains(prep.id()) {
            continue;
        }
        match host.prepare_config(prep) {
            Ok(detail) => {
                deferred(&mut out, prep.id());
                out.fix.details.push(detail);
                // Native stale-text preparation requires resync only while On.
                resync |= prep != ConfigPreparation::Stale || target == TransitionTarget::RouterOn;
                record(&mut out, Stage::PrepareConfig(prep), StageResult::Completed);
            }
            Err(e) => {
                let message = e.to_string();
                fail_id(&mut out, prep.id());
                out.fix.details.push(format!("{}: {message}", prep.id()));
                record(
                    &mut out,
                    Stage::PrepareConfig(prep),
                    StageResult::Failed(message),
                );
            }
        }
    }
    if out.initial_needs_fix.contains("router_http_contract") {
        deferred(&mut out, "router_http_contract");
        out.fix
            .details
            .push("将重新写入 aimai1 HTTP-only 路由契约".into());
        resync = true;
    }
    for id in ["catalog_integrity", "catalog_path_validity"] {
        if out.initial_needs_fix.contains(id) {
            deferred(&mut out, id);
            resync = true;
        }
    }
    if resync {
        if out
            .deferred
            .iter()
            .any(|id| matches!(id.as_str(), "catalog_integrity" | "catalog_path_validity"))
        {
            out.fix
                .details
                .push("将随 config 一起重新生成 Catalog".into());
        }
        match host.resync_config() {
            Err(e) => {
                let message = e.to_string();
                out.fix
                    .details
                    .push(format!("resync config 失败: {message}"));
                for id in out.deferred.clone() {
                    fail_id(&mut out, &id);
                }
                let deferred = &out.deferred;
                out.fix.fixed_ids.retain(|id| !deferred.contains(id));
                record(&mut out, Stage::ResyncConfig, StageResult::Failed(message));
            }
            Ok(()) => {
                record(&mut out, Stage::ResyncConfig, StageResult::Completed);
                let report = match host.diagnose() {
                    Ok(report) => report,
                    Err(e) => return Err(fatal(out, Stage::ResyncDiagnosis, e)),
                };
                for id in out.deferred.clone() {
                    // Native full reports always contain all checks. It searches for
                    // a matching non-ok item, rather than inventing a success status.
                    if report.items.iter().any(|i| i.id == id && i.status != "ok") {
                        fail_id(&mut out, &id);
                        out.fix.fixed_ids.retain(|s| s != &id);
                        out.fix.details.push(format!("{id}: resync 后复查仍异常"));
                    }
                }
                out.resync_report = Some(report);
                record(&mut out, Stage::ResyncDiagnosis, StageResult::Completed);
            }
        }
    }
    if out.initial_needs_fix.contains("db_orphan_providers")
        || out.initial_needs_fix.contains("rollout_orphan_providers")
    {
        dispatch(host, &mut out, BatchItem::OrphanProviders, prepared);
    }
    for item in ORDINARY_ORDER {
        if out.initial_needs_fix.contains(item.id()) {
            dispatch(host, &mut out, item, prepared);
        }
    }
    Ok(out)
}
