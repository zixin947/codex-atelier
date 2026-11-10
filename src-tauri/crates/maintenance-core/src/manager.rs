//! Native diagnostic-repair Manager control flow (AiMaMi arm64 0x100682818).
//!
//! All paths are explicit. `RepairOperations` supplies the application's real diagnostic,
//! dispatch and router-transition primitives; there are no default successful methods.
//! The driver owns gating, both locks, filesystem preflight, compensation selection and
//! process relaunch. Merely constructing a Manager never executes a client operation.
use crate::{
    catalog::{publication::write_atomic_private_report, state::RelayStateDocument},
    policy::{self, RepairGates},
    process::{self, ProcessContext, ProcessError, Runner},
    router_verification::{
        self, Depth, ProxyHandle, VerificationContext, VerificationPaths, VerificationReport,
    },
    transition_journal::{JournalStore, TransitionTarget},
};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt,
    fs::{self, File, OpenOptions},
    io::{self, Read},
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock},
    time::Duration,
};

pub const POSTCONDITION: &str = "router_postcondition";
pub const REPAIR_REASON: &str = "diagnostic repair";
pub const MUTATION_CONTEXT: &str = "repairing Codex diagnostics";
pub const UNVERIFIED_PREFIX: &str =
    "ROUTER_TARGET_RECOVERY_REQUIRED: 路由状态仍需确认，ChatGPT 未自动启动：";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticItem {
    pub id: String,
    pub status: String,
    pub label: String,
    pub detail: String,
    pub fixable: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticReport {
    pub items: Vec<DiagnosticItem>,
    pub has_issues: bool,
    pub transition_in_flight: bool,
}
impl DiagnosticReport {
    pub fn repair_gates(&self, selected: &str) -> Result<RepairGates, String> {
        policy::repair_gates(
            selected,
            &self
                .items
                .iter()
                .map(|i| policy::Item {
                    id: &i.id,
                    status: &i.status,
                    fixable: i.fixable,
                })
                .collect::<Vec<_>>(),
        )
    }
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FixResult {
    pub fixed_ids: Vec<String>,
    pub failed_ids: Vec<String>,
    pub details: Vec<String>,
}
impl FixResult {
    fn append(&mut self, other: Self) {
        self.fixed_ids.extend(other.fixed_ids);
        self.failed_ids.extend(other.failed_ids);
        self.details.extend(other.details);
    }
}

/// Contains credentials, so it deliberately implements neither Debug nor Serialize.
#[derive(Clone)]
pub struct Snapshot {
    document: RelayStateDocument,
    target: TransitionTarget,
}
impl Snapshot {
    pub fn new(document: RelayStateDocument) -> Result<Self, crate::catalog::state::StateError> {
        let enabled = document.catalog_inputs()?.router_enabled;
        Ok(Self {
            document,
            target: if enabled {
                TransitionTarget::RouterOn
            } else {
                TransitionTarget::NativeOff
            },
        })
    }
    pub fn document(&self) -> &RelayStateDocument {
        &self.document
    }
    pub fn target(&self) -> TransitionTarget {
        self.target
    }
}
#[derive(Clone)]
pub struct StateHandle(Arc<Mutex<RelayStateDocument>>);
impl StateHandle {
    pub fn new(document: RelayStateDocument) -> Self {
        Self(Arc::new(Mutex::new(document)))
    }
    pub fn from_shared(state: Arc<Mutex<RelayStateDocument>>) -> Self {
        Self(state)
    }
    pub fn shared(&self) -> &Arc<Mutex<RelayStateDocument>> {
        &self.0
    }
    /// Native 0x10068cdb4 unwraps this lock with exactly this panic message.
    pub fn snapshot(&self) -> Snapshot {
        Snapshot::new(self.0.lock().expect("relay state poisoned").clone())
            .expect("validated relay state")
    }
}
#[derive(Clone, Debug)]
pub struct RepairPaths {
    pub verification: VerificationPaths,
    pub state_path: PathBuf,
}
impl RepairPaths {
    pub fn from_codex_home(codex_home: &Path, databases: Vec<PathBuf>) -> Self {
        let relay_root = codex_home.join("codexmate/relay");
        Self {
            state_path: relay_root.join("state.json"),
            verification: VerificationPaths {
                codex_home: codex_home.to_owned(),
                relay_root,
                config_path: codex_home.join("config.toml"),
                databases,
            },
        }
    }
}

/// Failure +0x60/+0x61 are stage and attempt, not mutation-start booleans.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FailureStage {
    Preflight = 0,
    Stop = 1,
    Reconcile = 2,
    Settle = 3,
    Verify = 4,
    Commit = 5,
}
#[derive(Debug)]
pub struct TransitionFailure<E> {
    pub error: E,
    pub stage: FailureStage,
    pub attempt: u8,
}
impl<E> TransitionFailure<E> {
    pub fn needs_compensation(&self) -> bool {
        self.stage as u8 >= 2 || self.attempt >= 2
    }
}
/// Actual callback result; it contains no caller-supplied verified/success decision.
pub struct CompletedRepair<T> {
    fix: FixResult,
    reconciled: Option<T>,
    warnings: Vec<String>,
}
impl<T> CompletedRepair<T> {
    pub fn from_parts(fix: FixResult, reconciled: Option<T>, warnings: Vec<String>) -> Self {
        Self {
            fix,
            reconciled,
            warnings,
        }
    }
    pub fn fix_result(&self) -> &FixResult {
        &self.fix
    }
    pub fn reconciled(&self) -> Option<&T> {
        self.reconciled.as_ref()
    }
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }
    pub fn take_reconciled(&mut self) -> Option<T> {
        self.reconciled.take()
    }
    pub fn into_parts(self) -> (FixResult, Option<T>, Vec<String>) {
        (self.fix, self.reconciled, self.warnings)
    }
}
pub struct ReconcileResult<T, S = Snapshot> {
    pub value: T,
    pub details: Vec<String>,
    pub warnings: Vec<String>,
    pub state_after: S,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RepairAttempt {
    pub number: u8,
    pub initially_quiet: bool,
}
pub enum TransitionOutcome<T, E> {
    Verified {
        value: CompletedRepair<T>,
        warnings: Vec<String>,
        reconciled: Option<TransitionTarget>,
    },
    Unverified {
        value: CompletedRepair<T>,
        report: VerificationReport,
        recovery_reasons: Vec<String>,
        warnings: Vec<String>,
        reconciled: Option<TransitionTarget>,
    },
    Failed(TransitionFailure<E>),
}
impl<T, E> TransitionOutcome<T, E> {
    pub fn from_verification(
        value: CompletedRepair<T>,
        report: VerificationReport,
        warnings: Vec<String>,
        reconciled: Option<TransitionTarget>,
    ) -> Self {
        if report.issues.is_empty() {
            Self::Verified {
                value,
                warnings,
                reconciled,
            }
        } else {
            let recovery_reasons = report.issues.iter().map(|i| i.message.clone()).collect();
            Self::Unverified {
                value,
                report,
                recovery_reasons,
                warnings,
                reconciled,
            }
        }
    }
}

#[derive(Debug)]
pub enum ManagerError<E> {
    /// Native CoreError tag 9; this is the cached load error, not a journal busy flag.
    Initialization(String),
    Operation(String),
    Primitive(E),
    Io(io::Error),
    Process(ProcessError),
    Persistence(PersistenceError),
    Restoration {
        primary: Box<Self>,
        restoration: Box<Self>,
    },
    Relaunch {
        primary: Box<Self>,
        launch: ProcessError,
    },
}
impl<E: fmt::Display> fmt::Display for ManagerError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Initialization(s) | Self::Operation(s) => f.write_str(s),
            Self::Primitive(e) => e.fmt(f),
            Self::Io(e) => e.fmt(f),
            Self::Process(e) => e.fmt(f),
            Self::Persistence(e) => e.fmt(f),
            Self::Restoration {
                primary,
                restoration,
            } => write!(
                f,
                "{primary}; restoring the pre-repair router state also failed: {restoration}"
            ),
            Self::Relaunch { primary, launch } => write!(
                f,
                "diagnostic repair failed: {primary}; ChatGPT relaunch also failed: {launch}"
            ),
        }
    }
}
impl<E: Error + 'static> Error for ManagerError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Primitive(e) => Some(e),
            Self::Io(e) => Some(e),
            Self::Process(e) => Some(e),
            Self::Persistence(e) => Some(e),
            Self::Restoration { primary, .. } | Self::Relaunch { primary, .. } => Some(primary),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuditStep {
    Fix,
    Reconcile,
    Compensate,
    Relaunch,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AuditEvent {
    pub step: AuditStep,
    pub success: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum RelaunchEffect {
    #[default]
    NotRequested,
    UnconfirmedTarget,
    AlreadyRunning,
    Launched {
        pid: u32,
    },
    Failed,
}
pub struct RepairExecution<E> {
    pub result: Result<FixResult, ManagerError<E>>,
    pub audit: Vec<AuditEvent>,
    pub relaunch: RelaunchEffect,
    pub gates: Option<RepairGates>,
    pub transition_failure: Option<(FailureStage, u8)>,
}
impl<E> RepairExecution<E> {
    fn initial() -> Self {
        Self {
            result: Ok(FixResult::default()),
            audit: Vec::new(),
            relaunch: RelaunchEffect::NotRequested,
            gates: None,
            transition_failure: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiagnosticMode {
    Repair,
}
/// Fixed native policies for this entry point, derived by the driver, never caller booleans.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RepairTransitionPolicy {
    Strict,
}
pub struct QuietRepairRequest<'a, P, S = Snapshot> {
    pub paths: &'a RepairPaths,
    pub source: &'a S,
    pub original: &'a S,
    pub selected: &'a str,
    pub prepared_catalog: Option<&'a P>,
    pub include_missing_rollouts: bool,
    pub reason: &'static str,
    pub policy: RepairTransitionPolicy,
    pub writer_policy: process::ProcessPolicy,
    pub graceful_timeout: Duration,
}
pub struct Processes<'a, R> {
    pub runner: Arc<R>,
    pub context: &'a ProcessContext,
}
impl<R: Runner + Send + Sync + 'static> Processes<'_, R> {
    pub fn is_running(&self) -> bool {
        process::is_codex_app_running(self.runner.as_ref(), self.context)
    }
    pub fn launch(&self) -> Result<u32, ProcessError> {
        process::launch_codex_app(self.runner.as_ref(), self.context)
    }
}
/// Required native/app primitives. Their errors and values are retained, not mapped to
/// known_status/should_fix flags. No method has a default implementation.
///
/// `run_quiet_transition` owns native retries, stop/fence/settle/verify/commit and invokes
/// `repair` inside that transition. The Manager NEVER dispatches a second repair after it.
pub trait RepairOperations {
    type Error: Error + Send + Sync + 'static;
    type PreparedCatalog;
    type Reconciled;
    fn resolve_runtime_paths(
        &self,
        base: &RepairPaths,
        state: &Snapshot,
    ) -> Result<RepairPaths, Self::Error>;
    fn diagnose(
        &self,
        paths: &RepairPaths,
        state: &Snapshot,
        mode: DiagnosticMode,
    ) -> Result<DiagnosticReport, Self::Error>;
    fn prepare_catalog(
        &self,
        paths: &RepairPaths,
        state: &Snapshot,
    ) -> Result<Self::PreparedCatalog, Self::Error>;
    fn ensure_router_on_ready(&self, state: &Snapshot) -> Result<(), Self::Error>;
    fn fix(
        &self,
        paths: &RepairPaths,
        state: &Snapshot,
        selected: &str,
        prepared: Option<&Self::PreparedCatalog>,
        shared: &StateHandle,
    ) -> Result<FixResult, Self::Error>;
    fn fix_postcondition(
        &self,
        paths: &RepairPaths,
        shared: &StateHandle,
    ) -> Result<FixResult, Self::Error>;
    fn config_stale_reason(
        &self,
        paths: &RepairPaths,
        state: &Snapshot,
    ) -> Result<Option<String>, Self::Error>;
    fn reconcile_target<R: Runner + Send + Sync + 'static>(
        &self,
        paths: &RepairPaths,
        source: &Snapshot,
        attempt: RepairAttempt,
        processes: Processes<'_, R>,
        shared: &StateHandle,
    ) -> Result<ReconcileResult<Self::Reconciled>, Self::Error>;
    fn run_quiet_transition<R: Runner + Send + Sync + 'static>(
        &self,
        request: QuietRepairRequest<'_, Self::PreparedCatalog>,
        processes: Processes<'_, R>,
        shared: &StateHandle,
        repair: &mut dyn FnMut(
            RepairAttempt,
        ) -> Result<
            CompletedRepair<Self::Reconciled>,
            ManagerError<Self::Error>,
        >,
    ) -> TransitionOutcome<Self::Reconciled, ManagerError<Self::Error>>;
    fn restore_router_on(
        &self,
        paths: &RepairPaths,
        source: &Snapshot,
        reason: &str,
        shared: &StateHandle,
    ) -> Result<(), Self::Error>;
    fn rollback_native_off(
        &self,
        paths: &RepairPaths,
        source: &Snapshot,
        reason: &str,
        shared: &StateHandle,
    ) -> Result<Option<Vec<String>>, Self::Error>;
}

/// A target is projected from the actual source snapshot; this is not a validity verdict.
pub trait RepairSnapshot: Clone {
    fn target(&self) -> TransitionTarget;
}
impl RepairSnapshot for Snapshot {
    fn target(&self) -> TransitionTarget {
        self.target
    }
}
/// The two real storage sources implement this boundary: native RelayStateDocument and
/// Atelier Gateway catalog + durable RouterIntent. Neither implementation may synthesize
/// the other's document or substitute a caller-provided valid/verified flag.
pub trait HostState {
    type Error: Error + Send + Sync + 'static;
    type Snapshot: RepairSnapshot;
    fn snapshot(&self) -> Self::Snapshot;
    fn initialization_error(&self) -> Option<&str>;
    fn check_write_capacity(
        &self,
        paths: &RepairPaths,
        include_missing_rollouts: bool,
    ) -> Result<(), ManagerError<Self::Error>> {
        crate::router_capacity::ensure_transition_rollout_write_capacity(
            &paths.verification,
            include_missing_rollouts,
        )
        .map_err(ManagerError::Io)
    }
    fn checked_recovery(&self, base: &RepairPaths) -> Result<(), ManagerError<Self::Error>>;
    fn preflight(
        &self,
        base: &RepairPaths,
        source: &Self::Snapshot,
    ) -> Result<(), ManagerError<Self::Error>>;
    fn read_journal(
        &self,
        paths: &RepairPaths,
    ) -> Result<Option<crate::transition_journal::TransitionJournal>, ManagerError<Self::Error>>;
    fn verify_target(
        &self,
        paths: &RepairPaths,
        target: TransitionTarget,
        depth: Depth,
    ) -> VerificationReport;
}
/// Same native repair primitives, with the source snapshot and state store kept typed.
/// No method has a default success implementation; publication belongs to the required
/// real quiet-transition primitive and the selected source's concrete state store.
pub trait HostRepairOperations<H: HostState> {
    type PreparedCatalog;
    type Reconciled;
    fn resolve_runtime_paths(
        &self,
        base: &RepairPaths,
        state: &H::Snapshot,
    ) -> Result<RepairPaths, H::Error>;
    fn diagnose(
        &self,
        paths: &RepairPaths,
        state: &H::Snapshot,
        mode: DiagnosticMode,
    ) -> Result<DiagnosticReport, H::Error>;
    fn prepare_catalog(
        &self,
        paths: &RepairPaths,
        state: &H::Snapshot,
    ) -> Result<Self::PreparedCatalog, H::Error>;
    fn ensure_router_on_ready(&self, state: &H::Snapshot) -> Result<(), H::Error>;
    fn fix(
        &self,
        paths: &RepairPaths,
        state: &H::Snapshot,
        selected: &str,
        prepared: Option<&Self::PreparedCatalog>,
        host: &H,
    ) -> Result<FixResult, H::Error>;
    fn fix_postcondition(&self, paths: &RepairPaths, host: &H) -> Result<FixResult, H::Error>;
    fn config_stale_reason(
        &self,
        paths: &RepairPaths,
        state: &H::Snapshot,
    ) -> Result<Option<String>, H::Error>;
    fn reconcile_target<R: Runner + Send + Sync + 'static>(
        &self,
        paths: &RepairPaths,
        source: &H::Snapshot,
        attempt: RepairAttempt,
        processes: Processes<'_, R>,
        host: &H,
    ) -> Result<ReconcileResult<Self::Reconciled, H::Snapshot>, H::Error>;
    fn run_quiet_transition<R: Runner + Send + Sync + 'static>(
        &self,
        request: QuietRepairRequest<'_, Self::PreparedCatalog, H::Snapshot>,
        processes: Processes<'_, R>,
        host: &H,
        repair: &mut dyn FnMut(
            RepairAttempt,
        )
            -> Result<CompletedRepair<Self::Reconciled>, ManagerError<H::Error>>,
    ) -> TransitionOutcome<Self::Reconciled, ManagerError<H::Error>>;
    fn restore_router_on(
        &self,
        paths: &RepairPaths,
        source: &H::Snapshot,
        reason: &str,
        host: &H,
    ) -> Result<(), H::Error>;
    fn rollback_native_off(
        &self,
        paths: &RepairPaths,
        source: &H::Snapshot,
        reason: &str,
        host: &H,
    ) -> Result<Option<Vec<String>>, H::Error>;
}

static MUTATION_GATE: OnceLock<Arc<Mutex<()>>> = OnceLock::new();
pub fn shared_mutation_gate() -> Arc<Mutex<()>> {
    MUTATION_GATE
        .get_or_init(|| Arc::new(Mutex::new(())))
        .clone()
}
pub struct NativeManager<O, R> {
    operations: O,
    runner: Arc<R>,
    process_context: ProcessContext,
    base_paths: RepairPaths,
    state: StateHandle,
    load_error: Option<String>,
    transition_lock: Arc<Mutex<()>>,
    mutation_gate: Arc<Mutex<()>>,
    proxy: Arc<Mutex<Option<ProxyHandle>>>,
}
impl<O: RepairOperations, R: Runner + Send + Sync + 'static> NativeManager<O, R> {
    /// `loaded` must be the actual storage::load result. Failed load is retained permanently
    /// and blocks writes before selector validation, exactly as native RelayManager::new.
    pub fn from_loaded(
        operations: O,
        runner: R,
        process_context: ProcessContext,
        base_paths: RepairPaths,
        loaded: Result<RelayStateDocument, O::Error>,
        proxy: Arc<Mutex<Option<ProxyHandle>>>,
    ) -> Self {
        let (document,load_error)=match loaded { Ok(s)=>(s,None),Err(e)=>(RelayStateDocument::native_default(),Some(format!("中转状态文件无法安全读取，原文件已保留且所有中转写入已停止；请修复 state.json 后重启 Atelier。{e}"))) };
        Self {
            operations,
            runner: Arc::new(runner),
            process_context,
            base_paths,
            state: StateHandle::new(document),
            load_error,
            transition_lock: Arc::new(Mutex::new(())),
            mutation_gate: shared_mutation_gate(),
            proxy,
        }
    }
    pub fn with_coordination(
        mut self,
        state: StateHandle,
        transition_lock: Arc<Mutex<()>>,
        mutation_gate: Arc<Mutex<()>>,
    ) -> Self {
        self.state = state;
        self.transition_lock = transition_lock;
        self.mutation_gate = mutation_gate;
        self
    }
    pub fn state(&self) -> &StateHandle {
        &self.state
    }
    pub fn operations(&self) -> &O {
        &self.operations
    }
    pub fn runner(&self) -> &R {
        self.runner.as_ref()
    }
    pub fn native_host(&self) -> NativeHostState<'_, O::Error> {
        NativeHostState::new(&self.state, &self.proxy, self.load_error.as_deref())
    }
    pub fn repair(&self, selected: &str) -> RepairExecution<O::Error> {
        let host = self.native_host();
        let operations = NativeOperations {
            operations: &self.operations,
        };
        HostManager::new(
            &host,
            &operations,
            self.runner.clone(),
            &self.process_context,
            &self.base_paths,
        )
        .with_locks(self.transition_lock.clone(), self.mutation_gate.clone())
        .repair(selected)
    }
    pub fn has_confirmed_target_state(
        &self,
        paths: &RepairPaths,
        target: TransitionTarget,
    ) -> bool {
        confirmed_target_state(&self.native_host(), paths, target)
    }
}

/// Borrowed native store view. All reads and writes refer to the actual shared document.
pub struct NativeHostState<'a, E> {
    state: &'a StateHandle,
    proxy: &'a Mutex<Option<ProxyHandle>>,
    load_error: Option<&'a str>,
    error: std::marker::PhantomData<fn() -> E>,
}
impl<'a, E> NativeHostState<'a, E> {
    pub fn new(
        state: &'a StateHandle,
        proxy: &'a Mutex<Option<ProxyHandle>>,
        load_error: Option<&'a str>,
    ) -> Self {
        Self {
            state,
            proxy,
            load_error,
            error: std::marker::PhantomData,
        }
    }
    pub fn state(&self) -> &StateHandle {
        self.state
    }
    pub fn proxy(&self) -> &Mutex<Option<ProxyHandle>> {
        self.proxy
    }
}
impl<E: Error + Send + Sync + 'static> HostState for NativeHostState<'_, E> {
    type Error = E;
    type Snapshot = Snapshot;
    fn snapshot(&self) -> Snapshot {
        self.state.snapshot()
    }
    fn initialization_error(&self) -> Option<&str> {
        self.load_error
    }
    fn checked_recovery(&self, base: &RepairPaths) -> Result<(), ManagerError<E>> {
        ensure_transition_paths_allowed(base, self.state).map_err(ManagerError::Io)
    }
    fn preflight(&self, base: &RepairPaths, source: &Snapshot) -> Result<(), ManagerError<E>> {
        preflight_state_write(base, source.document()).map_err(|e| {
            ManagerError::Operation(format!(
                "diagnostic repair: cannot start because relay state is not safely writable: {e}"
            ))
        })
    }
    fn read_journal(
        &self,
        paths: &RepairPaths,
    ) -> Result<Option<crate::transition_journal::TransitionJournal>, ManagerError<E>> {
        JournalStore::new(&paths.verification.relay_root)
            .read_checked()
            .map_err(ManagerError::Io)
    }
    fn verify_target(
        &self,
        paths: &RepairPaths,
        target: TransitionTarget,
        depth: Depth,
    ) -> VerificationReport {
        let snapshot = self.state.snapshot();
        router_verification::verify_target_state_at_with_depth(
            &VerificationContext {
                paths: &paths.verification,
                state: snapshot.document(),
                proxy: self.proxy,
            },
            target,
            depth,
        )
    }
}
struct NativeOperations<'a, O> {
    operations: &'a O,
}
impl<O: RepairOperations> HostRepairOperations<NativeHostState<'_, O::Error>>
    for NativeOperations<'_, O>
{
    type PreparedCatalog = O::PreparedCatalog;
    type Reconciled = O::Reconciled;
    fn resolve_runtime_paths(
        &self,
        base: &RepairPaths,
        state: &Snapshot,
    ) -> Result<RepairPaths, O::Error> {
        self.operations.resolve_runtime_paths(base, state)
    }
    fn diagnose(
        &self,
        paths: &RepairPaths,
        state: &Snapshot,
        mode: DiagnosticMode,
    ) -> Result<DiagnosticReport, O::Error> {
        self.operations.diagnose(paths, state, mode)
    }
    fn prepare_catalog(
        &self,
        paths: &RepairPaths,
        state: &Snapshot,
    ) -> Result<Self::PreparedCatalog, O::Error> {
        self.operations.prepare_catalog(paths, state)
    }
    fn ensure_router_on_ready(&self, state: &Snapshot) -> Result<(), O::Error> {
        self.operations.ensure_router_on_ready(state)
    }
    fn fix(
        &self,
        paths: &RepairPaths,
        state: &Snapshot,
        selected: &str,
        prepared: Option<&Self::PreparedCatalog>,
        host: &NativeHostState<'_, O::Error>,
    ) -> Result<FixResult, O::Error> {
        self.operations
            .fix(paths, state, selected, prepared, host.state)
    }
    fn fix_postcondition(
        &self,
        paths: &RepairPaths,
        host: &NativeHostState<'_, O::Error>,
    ) -> Result<FixResult, O::Error> {
        self.operations.fix_postcondition(paths, host.state)
    }
    fn config_stale_reason(
        &self,
        paths: &RepairPaths,
        state: &Snapshot,
    ) -> Result<Option<String>, O::Error> {
        self.operations.config_stale_reason(paths, state)
    }
    fn reconcile_target<R: Runner + Send + Sync + 'static>(
        &self,
        paths: &RepairPaths,
        source: &Snapshot,
        attempt: RepairAttempt,
        processes: Processes<'_, R>,
        host: &NativeHostState<'_, O::Error>,
    ) -> Result<ReconcileResult<Self::Reconciled>, O::Error> {
        self.operations
            .reconcile_target(paths, source, attempt, processes, host.state)
    }
    fn run_quiet_transition<R: Runner + Send + Sync + 'static>(
        &self,
        request: QuietRepairRequest<'_, Self::PreparedCatalog>,
        processes: Processes<'_, R>,
        host: &NativeHostState<'_, O::Error>,
        repair: &mut dyn FnMut(
            RepairAttempt,
        )
            -> Result<CompletedRepair<Self::Reconciled>, ManagerError<O::Error>>,
    ) -> TransitionOutcome<Self::Reconciled, ManagerError<O::Error>> {
        self.operations
            .run_quiet_transition(request, processes, host.state, repair)
    }
    fn restore_router_on(
        &self,
        paths: &RepairPaths,
        source: &Snapshot,
        reason: &str,
        host: &NativeHostState<'_, O::Error>,
    ) -> Result<(), O::Error> {
        self.operations
            .restore_router_on(paths, source, reason, host.state)
    }
    fn rollback_native_off(
        &self,
        paths: &RepairPaths,
        source: &Snapshot,
        reason: &str,
        host: &NativeHostState<'_, O::Error>,
    ) -> Result<Option<Vec<String>>, O::Error> {
        self.operations
            .rollback_native_off(paths, source, reason, host.state)
    }
}
/// One control-flow engine for the two actual stores. NativeManager above delegates here;
/// Gateway hosts retain their own catalog/intent type and real storage operations.
pub struct HostManager<'a, H, O, R> {
    host: &'a H,
    operations: &'a O,
    runner: Arc<R>,
    process_context: &'a ProcessContext,
    base_paths: &'a RepairPaths,
    transition_lock: Arc<Mutex<()>>,
    mutation_gate: Arc<Mutex<()>>,
}
impl<'a, H: HostState, O: HostRepairOperations<H>, R: Runner + Send + Sync + 'static>
    HostManager<'a, H, O, R>
{
    pub fn new(
        host: &'a H,
        operations: &'a O,
        runner: Arc<R>,
        process_context: &'a ProcessContext,
        base_paths: &'a RepairPaths,
    ) -> Self {
        Self {
            host,
            operations,
            runner,
            process_context,
            base_paths,
            transition_lock: Arc::new(Mutex::new(())),
            mutation_gate: shared_mutation_gate(),
        }
    }
    pub fn with_locks(
        mut self,
        transition_lock: Arc<Mutex<()>>,
        mutation_gate: Arc<Mutex<()>>,
    ) -> Self {
        self.transition_lock = transition_lock;
        self.mutation_gate = mutation_gate;
        self
    }
    pub fn host(&self) -> &H {
        self.host
    }
    pub fn operations(&self) -> &O {
        self.operations
    }
    fn processes(&self) -> Processes<'_, R> {
        Processes {
            runner: self.runner.clone(),
            context: self.process_context,
        }
    }
    pub fn repair(&self, selected: &str) -> RepairExecution<H::Error> {
        let mut execution = RepairExecution::initial();
        if let Some(error) = self.host.initialization_error() {
            execution.result = Err(ManagerError::Initialization(error.to_owned()));
            return execution;
        }
        if selected != "all" && policy::quiet_window(selected) == policy::QuietWindow::Unknown {
            execution.result = Err(ManagerError::Operation(format!(
                "unknown or non-fixable diagnostic item: {selected}"
            )));
            return execution;
        }
        // Declaration order guarantees mutation guard is released before transition guard,
        // including every early error and unwind. Poison is never cleared or ignored.
        let _transition = match self.transition_lock.lock() {
            Ok(g) => g,
            Err(_) => {
                execution.result = Err(ManagerError::Operation(
                    "Codex router transition lock is poisoned".into(),
                ));
                return execution;
            }
        };
        let _mutation = match self.mutation_gate.lock() {
            Ok(g) => g,
            Err(_) => {
                execution.result = Err(ManagerError::Operation(format!(
                    "Codex mutation gate is poisoned while {MUTATION_CONTEXT}"
                )));
                return execution;
            }
        };
        execution.result = self.repair_locked(selected, &mut execution);
        execution
    }
    fn repair_locked(
        &self,
        selected: &str,
        execution: &mut RepairExecution<H::Error>,
    ) -> Result<FixResult, ManagerError<H::Error>> {
        let original = self.host.snapshot();
        // Both recovery guards execute against the configured base before resolving Codex's
        // runtime home. The real filesystem checks are implemented below, not a busy bool.
        self.host.checked_recovery(self.base_paths)?;
        let paths = self
            .operations
            .resolve_runtime_paths(&self.base_paths, &original)
            .map_err(ManagerError::Primitive)?;
        let report = self
            .operations
            .diagnose(&paths, &original, DiagnosticMode::Repair)
            .map_err(ManagerError::Primitive)?;
        let gates = report
            .repair_gates(selected)
            .map_err(ManagerError::Operation)?;
        execution.gates = Some(gates);
        let mut preparation_warning = None;
        let prepared = if gates.prepare_catalog {
            match self.operations.prepare_catalog(&paths, &original) {
                Ok(p) => Some(p),
                Err(e) if selected == "all" => {
                    preparation_warning = Some(e.to_string());
                    None
                }
                Err(e) => return Err(ManagerError::Primitive(e)),
            }
        } else {
            None
        };
        if gates.preflight_state_write {
            self.host.preflight(self.base_paths, &original)?;
            if original.target() == TransitionTarget::RouterOn {
                self.operations
                    .ensure_router_on_ready(&original)
                    .map_err(ManagerError::Primitive)?;
            }
        }
        let running_before = self.processes().is_running();
        let (mut result, relaunch_eligible) = if gates.direct_postcondition {
            let fixed = self.operations.fix_postcondition(&paths, self.host);
            execution.audit.push(AuditEvent {
                step: AuditStep::Fix,
                success: fixed.is_ok(),
            });
            (fixed.map_err(ManagerError::Primitive), true)
        } else if gates.use_quiet_transition {
            let source = self.host.snapshot();
            let mut repair = |attempt| {
                let mut fixed = self
                    .operations
                    .fix(&paths, &original, selected, prepared.as_ref(), self.host)
                    .map_err(ManagerError::Primitive)?;
                if let Some(w) = &preparation_warning {
                    fixed.details.push(format!("孤儿任务目录准备未完成：{w}"));
                }
                let verification = self
                    .host
                    .verify_target(&paths, source.target(), Depth::Full);
                let needs_reconcile =
                    if gates.reconcile_after_fix || !verification.issues.is_empty() {
                        true
                    } else {
                        self.operations
                            .config_stale_reason(&paths, &self.host.snapshot())
                            .map_err(ManagerError::Primitive)?
                            .is_some()
                    };
                let mut stale_snapshot = None;
                let mut warnings = Vec::new();
                let reconciled = if needs_reconcile {
                    let reconciled = self.operations.reconcile_target(
                        &paths,
                        &source,
                        attempt,
                        self.processes(),
                        self.host,
                    );
                    execution.audit.push(AuditEvent {
                        step: AuditStep::Reconcile,
                        success: reconciled.is_ok(),
                    });
                    let reconciled = reconciled.map_err(ManagerError::Primitive)?;
                    if !fixed.fixed_ids.iter().any(|id| id == POSTCONDITION) {
                        fixed.fixed_ids.push(POSTCONDITION.into());
                    }
                    fixed.details.extend(reconciled.details);
                    warnings = reconciled.warnings;
                    stale_snapshot = Some(reconciled.state_after);
                    Some(reconciled.value)
                } else {
                    None
                };
                let stale_snapshot = stale_snapshot.unwrap_or_else(|| self.host.snapshot());
                if let Some(reason) = self
                    .operations
                    .config_stale_reason(&paths, &stale_snapshot)
                    .map_err(ManagerError::Primitive)?
                {
                    return Err(ManagerError::Operation(format!(
                        "diagnostic repair left stale Codex config: {reason}"
                    )));
                }
                Ok(CompletedRepair {
                    fix: fixed,
                    reconciled,
                    warnings,
                })
            };
            let outcome = self.operations.run_quiet_transition(
                QuietRepairRequest {
                    paths: &paths,
                    source: &source,
                    original: &original,
                    selected,
                    prepared_catalog: prepared.as_ref(),
                    include_missing_rollouts: gates.include_missing_rollouts,
                    reason: REPAIR_REASON,
                    policy: RepairTransitionPolicy::Strict,
                    writer_policy: process::ProcessPolicy::ForceCloseAll,
                    graceful_timeout: Duration::from_secs(5),
                },
                self.processes(),
                self.host,
                &mut repair,
            );
            let result = match outcome {
                TransitionOutcome::Verified {
                    mut value,
                    warnings,
                    ..
                } => {
                    value.fix.details.extend(warnings);
                    Ok(value.fix)
                }
                TransitionOutcome::Unverified {
                    recovery_reasons, ..
                } => Err(ManagerError::Operation(format!(
                    "{UNVERIFIED_PREFIX}{}",
                    recovery_reasons.join(" | ")
                ))),
                TransitionOutcome::Failed(failure) => {
                    execution.transition_failure = Some((failure.stage, failure.attempt));
                    let compensate = failure.needs_compensation();
                    let primary = failure.error;
                    if compensate {
                        let reason =
                            format!("diagnostic repair failed after mutation began: {primary}");
                        let restored = self.compensate(&paths, &source, &reason);
                        execution.audit.push(AuditEvent {
                            step: AuditStep::Compensate,
                            success: restored.is_ok(),
                        });
                        match restored {
                            Ok(()) => Err(primary),
                            Err(restoration) => Err(ManagerError::Restoration {
                                primary: Box::new(primary),
                                restoration: Box::new(restoration),
                            }),
                        }
                    } else {
                        Err(primary)
                    }
                }
            };
            (result, true)
        } else {
            match self
                .operations
                .fix(&paths, &original, selected, prepared.as_ref(), self.host)
            {
                Err(e) => (Err(ManagerError::Primitive(e)), false),
                Ok(mut fixed) => {
                    if let Some(w) = preparation_warning {
                        fixed.details.push(format!("孤儿任务目录准备未完成：{w}"));
                    }
                    if gates.reconcile_after_fix {
                        let reconciled = self.operations.fix_postcondition(&paths, self.host);
                        execution.audit.push(AuditEvent {
                            step: AuditStep::Fix,
                            success: reconciled.is_ok(),
                        });
                        match reconciled {
                            Ok(other) => {
                                fixed.append(other);
                                (Ok(fixed), true)
                            }
                            Err(e) => (Err(ManagerError::Primitive(e)), true),
                        }
                    } else {
                        (Ok(fixed), false)
                    }
                }
            }
        };
        if running_before && relaunch_eligible {
            if result.is_err() && !self.has_confirmed_target_state(&paths, original.target()) {
                execution.relaunch = RelaunchEffect::UnconfirmedTarget;
                return result;
            }
            if self.processes().is_running() {
                execution.relaunch = RelaunchEffect::AlreadyRunning;
                // Native leaves w26=1 here and appends the same detail despite no launch.
                if let Ok(fixed) = &mut result {
                    fixed
                        .details
                        .push("已自动重新打开 ChatGPT 使修复生效".into());
                }
            } else {
                let launched = self.processes().launch();
                execution.audit.push(AuditEvent {
                    step: AuditStep::Relaunch,
                    success: launched.is_ok(),
                });
                match launched {
                    Ok(pid) => {
                        execution.relaunch = RelaunchEffect::Launched { pid };
                        if let Ok(fixed) = &mut result {
                            fixed
                                .details
                                .push("已自动重新打开 ChatGPT 使修复生效".into());
                        }
                    }
                    Err(launch) => {
                        execution.relaunch = RelaunchEffect::Failed;
                        result = match result {
                            Ok(mut fixed) => {
                                fixed.details.push(format!("修复成功，但自动重新打开 ChatGPT 失败：{launch}，请手动打开 ChatGPT"));
                                Ok(fixed)
                            }
                            Err(primary) => Err(ManagerError::Relaunch {
                                primary: Box::new(primary),
                                launch,
                            }),
                        };
                    }
                }
            }
        }
        result
    }
    fn compensate(
        &self,
        paths: &RepairPaths,
        source: &H::Snapshot,
        reason: &str,
    ) -> Result<(), ManagerError<H::Error>> {
        match source.target() {
            TransitionTarget::RouterOn=>self.operations.restore_router_on(paths,source,reason,self.host).map_err(ManagerError::Primitive),
            TransitionTarget::NativeOff=>match self.operations.rollback_native_off(paths,source,reason,self.host).map_err(ManagerError::Primitive)? {
                None=>Ok(()),Some(warnings)=>Err(ManagerError::Operation(format!("source NativeOff core state was restored but diagnostics still require recovery: {}",warnings.join(" | ")))),
            }
        }
    }
    pub fn has_confirmed_target_state(
        &self,
        paths: &RepairPaths,
        target: TransitionTarget,
    ) -> bool {
        confirmed_target_state(self.host, paths, target)
    }
}

fn confirmed_target_state<H: HostState>(
    host: &H,
    paths: &RepairPaths,
    target: TransitionTarget,
) -> bool {
    match host.read_journal(paths) {
        Ok(None) => {}
        Ok(Some(journal)) if !journal.is_commit_pending() => {}
        _ => return false,
    }
    host.verify_target(paths, target, Depth::Full)
        .issues
        .is_empty()
}

/// Actual native save preflight: ordered directory creation, existing target writable
/// without truncation, pretty JSON -> private atomic temporary publication -> remove.
/// A directory-sync warning is acknowledged exactly as native, not promoted to failure.
pub fn preflight_state_write(paths: &RepairPaths, state: &RelayStateDocument) -> io::Result<()> {
    ensure_state_directories(paths)?;
    if paths.state_path.is_dir() {
        return Err(io::Error::other(format!(
            "preflight relay state write failed: {} is a directory",
            paths.state_path.display()
        )));
    }
    if fs::metadata(&paths.state_path).is_ok() {
        drop(
            OpenOptions::new()
                .write(true)
                .open(&paths.state_path)
                .map_err(|e| {
                    io::Error::new(
                        e.kind(),
                        format!("preflight relay state target access failed: {e}"),
                    )
                })?,
        );
    }
    let bytes = state
        .to_pretty_json()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let parent = paths
        .state_path
        .parent()
        .ok_or_else(|| io::Error::other("relay state path has no parent directory"))?;
    let temporary = parent.join(format!(
        ".relay-state-preflight-{}-{}",
        std::process::id(),
        random_v4_simple()?
    ));
    write_atomic_private_report(&temporary, &bytes)
        .map_err(|e| io::Error::new(e.kind(), format!("preflight relay state write failed: {e}")))?
        .acknowledge();
    fs::remove_file(&temporary).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!(
                "preflight relay state cleanup failed for {}: {e}",
                temporary.display()
            ),
        )
    })
}
fn random_v4_simple() -> io::Result<String> {
    let mut bytes = [0u8; 16];
    // The recovered implementation is a macOS binary. /dev/urandom is the actual Unix
    // entropy source here; no home, credential or application state is consulted.
    File::open("/dev/urandom")?.read_exact(&mut bytes)?;
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

/// Guard implementation follows the checked native recovery files; see evidence/manager.
/// The two concrete filesystem helpers are intentionally separate from diagnostic gates.
pub fn ensure_transition_paths_allowed(paths: &RepairPaths, state: &StateHandle) -> io::Result<()> {
    ensure_recovery_records_allowed(&paths.verification.relay_root)?;
    ensure_pending_commit_matches(paths, &state.snapshot())
}
pub fn ensure_pending_commit_matches(paths: &RepairPaths, state: &Snapshot) -> io::Result<()> {
    if JournalStore::new(&paths.verification.relay_root)
        .read_checked()?
        .is_some_and(|j| j.is_commit_pending())
    {
        verify_router_commit(paths, state.document())?;
    }
    Ok(())
}

pub const COMMIT_STATE_CONFLICT:&str="ROUTER_COMMIT_STATE_CONFLICT: 无法核对磁盘路由回执与当前模式，已停止继续写入；请检查状态文件并重启 Atelier";
/// Native Receipt only deserializes these four fields. Unrelated provider credentials in
/// the file are ignored; duplicate known fields, invalid types and trailing data fail.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CommitReceipt {
    schema_version: i32,
    router_commit_id: Option<crate::catalog::state::RouterOperationId>,
    codex_router_enabled: bool,
    codex_router_no_account_mode: bool,
}
pub fn verify_router_commit(paths: &RepairPaths, state: &RelayStateDocument) -> io::Result<()> {
    let expected = state
        .catalog_inputs()
        .map_err(|_| io::Error::other(COMMIT_STATE_CONFLICT))?;
    let commit = state
        .router_commit_id()
        .map_err(|_| io::Error::other(COMMIT_STATE_CONFLICT))?;
    let file = match File::open(&paths.state_path) {
        Ok(file) => file,
        Err(e)
            if e.kind() == io::ErrorKind::NotFound
                && commit.is_none()
                && !expected.router_enabled =>
        {
            return Ok(())
        }
        Err(_) => return Err(io::Error::other(COMMIT_STATE_CONFLICT)),
    };
    let receipt: CommitReceipt = serde_json::from_reader(io::BufReader::new(file))
        .map_err(|_| io::Error::other(COMMIT_STATE_CONFLICT))?;
    if state
        .schema_version()
        .map_err(|_| io::Error::other(COMMIT_STATE_CONFLICT))?
        != receipt.schema_version
        || commit != receipt.router_commit_id
        || expected.router_enabled != receipt.codex_router_enabled
        || (expected.router_enabled
            && expected.no_account_mode != receipt.codex_router_no_account_mode)
    {
        return Err(io::Error::other(COMMIT_STATE_CONFLICT));
    }
    Ok(())
}

const MODEL_JOURNAL_LIMIT: u64 = 8 * 1024 * 1024;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FallbackRestoreEntry {
    thread_id: String,
    original_model: String,
    fallback_model: String,
    #[serde(default)]
    attempts: u32,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FallbackRestoreJournal {
    version: u32,
    entries: Vec<FallbackRestoreEntry>,
}
fn fallback_error(reason: &str) -> io::Error {
    io::Error::other(format!("中转原模型恢复记录{reason}，本次操作未能完成。"))
}
/// Native fallback journal loader @1008ab7c4 and any-predicate @100914dd8.
pub fn validate_fallback_model_restore_journal(relay_root: &Path) -> io::Result<()> {
    let path = relay_root.join("router_model_restore.json");
    let metadata = match fs::symlink_metadata(&path) {
        Ok(m) => m,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(_) => return Err(fallback_error("不可读取")),
    };
    if !metadata.is_file() {
        return Err(fallback_error("路径类型异常"));
    }
    if metadata.len() > MODEL_JOURNAL_LIMIT {
        return Err(fallback_error("超过读取上限"));
    }
    let file = File::open(path).map_err(|_| fallback_error("不可读取"))?;
    let mut bytes = Vec::new();
    file.take(MODEL_JOURNAL_LIMIT + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| fallback_error("读取失败"))?;
    if bytes.len() as u64 > MODEL_JOURNAL_LIMIT {
        return Err(fallback_error("超过读取上限"));
    }
    let journal: FallbackRestoreJournal =
        serde_json::from_slice(&bytes).map_err(|_| fallback_error("格式异常或版本不兼容"))?;
    let mut ids = std::collections::HashSet::new();
    if journal.version != 1
        || journal.entries.iter().any(|e| {
            let _ = e.attempts;
            e.thread_id.trim().is_empty()
                || !e.original_model.starts_with("aimami_relay_")
                || e.fallback_model.trim().is_empty()
                || !ids.insert(e.thread_id.as_str())
        })
    {
        return Err(fallback_error("内容异常或版本不兼容"));
    }
    Ok(())
}
pub fn ensure_recovery_records_allowed(relay_root: &Path) -> io::Result<()> {
    JournalStore::new(relay_root).read_checked()?;
    validate_fallback_model_restore_journal(relay_root)?;
    validate_no_account_model_restore_journal(relay_root)?;
    crate::auth::takeover::validate_recovery_backups(relay_root).map_err(io::Error::other)
}

/// Native CatalogRuntime's two counters and cached preparation. The artifact remains the
/// application's actual typed preparation, rather than a serialized success decision.
pub struct CatalogRuntime<C> {
    pub generation: u64,
    pub preparation_generation: u64,
    pub cached: Option<C>,
}
impl<C> Default for CatalogRuntime<C> {
    fn default() -> Self {
        Self {
            generation: 0,
            preparation_generation: 0,
            cached: None,
        }
    }
}
pub struct PersistenceContext<'a, C> {
    pub paths: &'a RepairPaths,
    pub state: &'a StateHandle,
    pub proxy: &'a Mutex<Option<ProxyHandle>>,
    pub catalog: &'a Mutex<CatalogRuntime<C>>,
    pub initialization_error: Option<&'a str>,
}
impl<C> Copy for PersistenceContext<'_, C> {}
impl<C> Clone for PersistenceContext<'_, C> {
    fn clone(&self) -> Self {
        *self
    }
}
#[derive(Debug)]
pub enum PersistenceError {
    Initialization(String),
    Operation(String),
    Conflict(&'static str),
    State(crate::catalog::state::StateError),
    Catalog(crate::catalog::builder::CatalogBuildError),
    Io(io::Error),
    Json(serde_json::Error),
}
impl fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Initialization(s) | Self::Operation(s) => f.write_str(s),
            Self::Conflict(s) => f.write_str(s),
            Self::State(e) => e.fmt(f),
            Self::Catalog(e) => e.fmt(f),
            Self::Io(e) => e.fmt(f),
            Self::Json(e) => e.fmt(f),
        }
    }
}
impl Error for PersistenceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::State(e) => Some(e),
            Self::Catalog(e) => Some(e),
            Self::Io(e) => Some(e),
            Self::Json(e) => Some(e),
            _ => None,
        }
    }
}
#[derive(Debug)]
pub struct StatePublication {
    /// Rename happened. A sync warning must remain available to the transition engine.
    pub publication: crate::catalog::publication::Publication,
    /// Native only logs failed receipt readback after a sync warning, then updates memory.
    pub receipt_warning: Option<io::Error>,
    pub catalog_invalidated: bool,
}
pub trait StateIo {
    fn publish(
        &self,
        path: &Path,
        bytes: &[u8],
    ) -> io::Result<crate::catalog::publication::Publication>;
}
#[derive(Clone, Copy, Default)]
pub struct LocalStateIo;
impl StateIo for LocalStateIo {
    fn publish(
        &self,
        path: &Path,
        bytes: &[u8],
    ) -> io::Result<crate::catalog::publication::Publication> {
        crate::catalog::publication::write_atomic_report(path, bytes)
    }
}
/// Native derives status from handle presence/port. It does not probe TCP or the alive
/// flag here. Poisoned proxy lock yields default status; verification has stricter rules.
pub fn compose_proxy_status(
    proxy: &Mutex<Option<ProxyHandle>>,
) -> crate::catalog::state::RelayProxyStatus {
    let Ok(handle) = proxy.lock() else {
        return Default::default();
    };
    match handle.as_ref() {
        None => Default::default(),
        Some(handle) => crate::catalog::state::RelayProxyStatus {
            running: true,
            port: handle.port,
            base_url: format!("http://127.0.0.1:{}", handle.port),
            codex_base_url: format!("http://127.0.0.1:{}/codex/v1", handle.port),
            last_error: None,
        },
    }
}
fn ensure_state_directories(paths: &RepairPaths) -> io::Result<()> {
    let home = &paths.verification.codex_home;
    let managed = home.join("codexmate");
    for directory in [
        home.join("accounts"),
        home.join("accounts/snapshots"),
        home.join("accounts/backups"),
        home.join("accounts/registry-backups"),
        managed.clone(),
        managed.join("skill-backups"),
        managed.join("custom-instructions"),
        managed.join("custom-instructions/history"),
        paths.verification.relay_root.clone(),
    ] {
        fs::create_dir_all(directory)?;
    }
    Ok(())
}
/// Exact native storage save boundary, including post-rename Publication uncertainty.
pub fn save_state_with_publication(
    paths: &RepairPaths,
    state: &RelayStateDocument,
) -> Result<crate::catalog::publication::Publication, PersistenceError> {
    save_state_with_publication_using(paths, state, &LocalStateIo)
}
pub fn save_state_with_publication_using(
    paths: &RepairPaths,
    state: &RelayStateDocument,
    io: &impl StateIo,
) -> Result<crate::catalog::publication::Publication, PersistenceError> {
    crate::catalog::credentials::validate_credentials(state).map_err(PersistenceError::State)?;
    let version = state.schema_version().map_err(PersistenceError::State)?;
    let receipt = state.router_commit_id().map_err(PersistenceError::State)?;
    if version > 16 || (version <= 14 && receipt.is_some()) {
        return Err(PersistenceError::Initialization(
            "relay state schema cannot represent this router commit receipt".into(),
        ));
    }
    ensure_state_directories(paths).map_err(PersistenceError::Io)?;
    let bytes = state.to_pretty_json().map_err(PersistenceError::State)?;
    io.publish(&paths.state_path, &bytes)
        .map_err(|e| PersistenceError::Operation(format!("write relay state failed: {e}")))
}
/// Native `revision` @100678ae0 hashes only compact JSON of active exposed relay models.
/// Key sorting preserves the shipped serde_json BTreeMap wire representation even when
/// a downstream application's feature unification enables preserve_order.
pub fn native_catalog_revision(state: &RelayStateDocument) -> Result<String, PersistenceError> {
    use sha2::{Digest, Sha256};
    fn sorted(value: serde_json::Value) -> serde_json::Value {
        match value {
            serde_json::Value::Array(a) => {
                serde_json::Value::Array(a.into_iter().map(sorted).collect())
            }
            serde_json::Value::Object(o) => {
                let ordered = o.into_iter().collect::<std::collections::BTreeMap<_, _>>();
                serde_json::Value::Object(
                    ordered.into_iter().map(|(k, v)| (k, sorted(v))).collect(),
                )
            }
            v => v,
        }
    }
    let input = state.catalog_inputs().map_err(PersistenceError::State)?;
    let models =
        crate::catalog::builder::build_relay_models(&input.providers, &input.active_provider_ids)
            .map_err(PersistenceError::Catalog)?;
    let bytes = serde_json::to_vec(&sorted(serde_json::Value::Array(models)))
        .map_err(PersistenceError::Json)?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}
pub const ROUTE_CHANGE_REQUIRES_COMMIT: &str =
    "ROUTER_STATE_CONFLICT: active route changes require a transition commit";
pub const ROUTE_SNAPSHOT_CHANGED: &str =
    "ROUTER_STATE_CONFLICT: route changed after this state snapshot; retry from current state";
pub fn persist_with_commit<C>(
    context: PersistenceContext<'_, C>,
    candidate: RelayStateDocument,
    commit: Option<crate::catalog::state::RouterOperationId>,
) -> Result<StatePublication, PersistenceError> {
    persist_with_commit_using(context, candidate, commit, &LocalStateIo)
}
pub fn persist_with_commit_using<C>(
    context: PersistenceContext<'_, C>,
    mut candidate: RelayStateDocument,
    commit: Option<crate::catalog::state::RouterOperationId>,
    io: &impl StateIo,
) -> Result<StatePublication, PersistenceError> {
    if let Some(error) = context.initialization_error {
        return Err(PersistenceError::Initialization(error.into()));
    }
    let proposed = candidate
        .catalog_inputs()
        .map_err(PersistenceError::State)?;
    crate::catalog::builder::validate_legacy_no_account_aliases(&proposed.aliases)
        .map_err(PersistenceError::Catalog)?;
    candidate
        .set_proxy_status(compose_proxy_status(context.proxy))
        .map_err(PersistenceError::State)?;
    let mut current = context.state.shared().lock().expect("relay state poisoned");
    ensure_pending_commit_matches(
        context.paths,
        &Snapshot::new(current.clone()).map_err(PersistenceError::State)?,
    )
    .map_err(PersistenceError::Io)?;
    let existing = current.catalog_inputs().map_err(PersistenceError::State)?;
    let current_commit = current
        .router_commit_id()
        .map_err(PersistenceError::State)?;
    if commit.is_some() {
        ensure_recovery_records_allowed(&context.paths.verification.relay_root)
            .map_err(PersistenceError::Io)?;
    } else if proposed.router_enabled != existing.router_enabled
        || (proposed.router_enabled && proposed.no_account_mode != existing.no_account_mode)
    {
        return Err(PersistenceError::Conflict(ROUTE_CHANGE_REQUIRES_COMMIT));
    }
    if candidate
        .router_commit_id()
        .map_err(PersistenceError::State)?
        != current_commit
        && (commit.is_some()
            || proposed.router_enabled != existing.router_enabled
            || proposed.no_account_mode != existing.no_account_mode)
    {
        return Err(PersistenceError::Conflict(ROUTE_SNAPSHOT_CHANGED));
    }
    candidate.set_router_commit_id(commit.or(current_commit));
    // A failed publication leaves memory and the actual cached preparation unchanged.
    let publication = save_state_with_publication_using(context.paths, &candidate, io)?;
    let receipt_warning = if publication.directory_sync_error.is_some() {
        verify_router_commit(context.paths, &candidate).err()
    } else {
        None
    };
    let catalog_invalidated = proposed.router_enabled != existing.router_enabled
        || proposed.no_account_mode != existing.no_account_mode
        || native_catalog_revision(&candidate)? != native_catalog_revision(&current)?;
    if catalog_invalidated {
        let mut catalog = context.catalog.lock().expect("relay state poisoned");
        catalog.generation = catalog.generation.wrapping_add(1);
        catalog.preparation_generation = catalog.preparation_generation.wrapping_add(1);
        catalog.cached = None;
    }
    *current = candidate;
    Ok(StatePublication {
        publication,
        receipt_warning,
        catalog_invalidated,
    })
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativeOffWriteWindow {
    Applied,
    Unavailable,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReconciledTarget {
    RouterOn { no_account_mode: bool },
    NativeOff { write_window: NativeOffWriteWindow },
}
#[derive(Debug)]
pub enum CommitError<E> {
    Primitive(E),
    Persistence(PersistenceError),
    Operation(String),
}
impl<E: fmt::Display> fmt::Display for CommitError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Primitive(e) => e.fmt(f),
            Self::Persistence(e) => e.fmt(f),
            Self::Operation(s) => f.write_str(s),
        }
    }
}
impl<E: Error + 'static> Error for CommitError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Primitive(e) => Some(e),
            Self::Persistence(e) => Some(e),
            _ => None,
        }
    }
}
/// Actual commit proposal is derived from the live Manager snapshot and reconciliation
/// metadata, then checked against the real config before receipt publication.
pub fn commit_reconciled_target<C, E>(
    context: PersistenceContext<'_, C>,
    target: &ReconciledTarget,
    source: &Snapshot,
    id: &crate::catalog::state::RouterOperationId,
    stale_reason: impl FnOnce(&Snapshot) -> Result<Option<String>, E>,
) -> Result<StatePublication, CommitError<E>> {
    if *target
        == (ReconciledTarget::NativeOff {
            write_window: NativeOffWriteWindow::Unavailable,
        })
    {
        return Err(CommitError::Operation(
            "NativeOff was not applied because no verified ChatGPT write window was available"
                .into(),
        ));
    }
    let mut candidate = context.state.snapshot().document;
    match target {
        ReconciledTarget::RouterOn { no_account_mode } => {
            candidate.set_router_flags(true, *no_account_mode)
        }
        ReconciledTarget::NativeOff { .. } => candidate.set_router_flags(
            false,
            source
                .document
                .catalog_inputs()
                .map_err(|e| CommitError::Persistence(PersistenceError::State(e)))?
                .no_account_mode,
        ),
    }
    let snapshot = Snapshot::new(candidate.clone())
        .map_err(|e| CommitError::Persistence(PersistenceError::State(e)))?;
    if let Some(reason) = stale_reason(&snapshot).map_err(CommitError::Primitive)? {
        let prefix = match target {
            ReconciledTarget::RouterOn { .. } => {
                "RouterOn config does not match the reconciled auth mode: "
            }
            ReconciledTarget::NativeOff { .. } => {
                "NativeOff config still contains router residue: "
            }
        };
        return Err(CommitError::Operation(format!("{prefix}{reason}")));
    }
    persist_with_commit(context, candidate, Some(id.clone())).map_err(CommitError::Persistence)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NoAccountRestoreJournal {
    version: u32,
    entries: Vec<NoAccountRestoreEntry>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NoAccountRestoreEntry {
    thread_id: String,
    #[serde(deserialize_with = "required_optional_string")]
    original_model: Option<String>,
    borrowed_model: String,
}
fn required_optional_string<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    Option::<String>::deserialize(deserializer)
}
/// Strict native no-account schema. The thread repair reader's content rules are reused
/// here with one read, while preserving required nullable fields and deny_unknown_fields.
pub fn validate_no_account_model_restore_journal(relay_root: &Path) -> io::Result<()> {
    fn operation(message: impl Into<String>) -> io::Error {
        io::Error::other(format!("Operation failed: {}", message.into()))
    }
    let path = relay_root.join("router_no_account_model_restore.json");
    let metadata = match fs::symlink_metadata(&path) {
        Ok(metadata) => metadata,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e),
    };
    if !metadata.is_file() {
        return Err(operation(
            "invalid no-account model restore journal path type",
        ));
    }
    if metadata.len() > MODEL_JOURNAL_LIMIT {
        return Err(operation(
            "no-account model restore journal exceeds size limit",
        ));
    }
    let mut bytes = Vec::new();
    File::open(&path)?
        .take(MODEL_JOURNAL_LIMIT + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > MODEL_JOURNAL_LIMIT {
        return Err(operation(
            "no-account model restore journal exceeds size limit",
        ));
    }
    let journal: NoAccountRestoreJournal = serde_json::from_slice(&bytes).map_err(|e| {
        operation(format!(
            "decode no-account model restore journal {}: {e}",
            path.display()
        ))
    })?;
    if journal.version != 1 && journal.version != 2 {
        return Err(operation(format!(
            "unsupported no-account model restore journal version: {}",
            journal.version
        )));
    }
    let mut ids = std::collections::HashSet::new();
    if journal.entries.iter().any(|e| {
        let _ = &e.original_model;
        e.thread_id.trim().is_empty()
            || (crate::catalog::canonical_no_account_slot_slug(&e.borrowed_model).is_none()
                && !(journal.version == 2
                    && crate::router_verification::is_relay_model_slug(&e.borrowed_model)))
            || !ids.insert(e.thread_id.as_str())
    }) {
        return Err(operation(
            "invalid no-account model restore journal contents",
        ));
    }
    Ok(())
}
