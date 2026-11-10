//! No SystemRunner is constructed. All filesystem fixtures live below TempDir.
use atelier_maintenance_core::{
    catalog::state::RelayStateDocument,
    manager::*,
    process::{BundleField, CommandOutput, Operation, ProcessContext, Runner},
    router_verification::{InvariantDim, VerificationIssue, VerificationReport},
    transition_journal::{JournalStore, TransitionTarget},
};
use std::{
    collections::VecDeque,
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Duration,
};
use tempfile::TempDir;
#[derive(Clone, Debug, PartialEq, Eq)]
struct PrimitiveError {
    code: u16,
    message: &'static str,
}
impl fmt::Display for PrimitiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.code, self.message)
    }
}
impl Error for PrimitiveError {}
fn err(message: &'static str) -> PrimitiveError {
    PrimitiveError { code: 73, message }
}
#[derive(Clone)]
enum Script {
    Verified(Vec<String>),
    Unverified,
    Failure(FailureStage, u8),
    CallbackFailure,
}
struct Ops {
    calls: Mutex<Vec<String>>,
    report: Vec<DiagnosticItem>,
    prepare_error: bool,
    ready_error: bool,
    fix_error: bool,
    post_error: bool,
    script: Script,
    restore_error: bool,
    off_warnings: Option<Vec<String>>,
    pending_during_transition: bool,
    change_source_to_on: bool,
    stale_reasons: Mutex<VecDeque<Option<String>>>,
    stale_calls: Mutex<usize>,
    reconcile_error: bool,
    reconcile_details: Vec<String>,
    reconcile_warnings: Vec<String>,
    existing_postcondition_id: bool,
}
impl Default for Ops {
    fn default() -> Self {
        Self {
            calls: Mutex::new(vec![]),
            report: vec![],
            prepare_error: false,
            ready_error: false,
            fix_error: false,
            post_error: false,
            script: Script::Verified(vec![]),
            restore_error: false,
            off_warnings: None,
            pending_during_transition: false,
            change_source_to_on: false,
            stale_reasons: Mutex::new(VecDeque::new()),
            stale_calls: Mutex::new(0),
            reconcile_error: false,
            reconcile_details: vec![],
            reconcile_warnings: vec![],
            existing_postcondition_id: false,
        }
    }
}
impl Ops {
    fn call(&self, s: impl Into<String>) {
        self.calls.lock().unwrap().push(s.into());
    }
    fn trace(&self) -> Vec<String> {
        self.calls.lock().unwrap().clone()
    }
}
fn result(id: &str) -> FixResult {
    FixResult {
        fixed_ids: vec![id.into()],
        failed_ids: vec![],
        details: vec![],
    }
}
impl RepairOperations for Ops {
    type Error = PrimitiveError;
    type PreparedCatalog = String;
    type Reconciled = ();
    fn resolve_runtime_paths(
        &self,
        p: &RepairPaths,
        _: &Snapshot,
    ) -> Result<RepairPaths, Self::Error> {
        self.call("paths");
        Ok(p.clone())
    }
    fn diagnose(
        &self,
        _: &RepairPaths,
        _: &Snapshot,
        m: DiagnosticMode,
    ) -> Result<DiagnosticReport, Self::Error> {
        assert_eq!(m, DiagnosticMode::Repair);
        self.call("diagnose");
        Ok(DiagnosticReport {
            items: self.report.clone(),
            has_issues: !self.report.is_empty(),
            transition_in_flight: false,
        })
    }
    fn prepare_catalog(&self, _: &RepairPaths, _: &Snapshot) -> Result<String, Self::Error> {
        self.call("prepare");
        if self.prepare_error {
            Err(err("prepare"))
        } else {
            Ok("reviewed catalog".into())
        }
    }
    fn ensure_router_on_ready(&self, _: &Snapshot) -> Result<(), Self::Error> {
        self.call("ready");
        if self.ready_error {
            Err(err("ready"))
        } else {
            Ok(())
        }
    }
    fn fix(
        &self,
        _: &RepairPaths,
        original: &Snapshot,
        id: &str,
        prepared: Option<&String>,
        shared: &StateHandle,
    ) -> Result<FixResult, Self::Error> {
        self.call(format!(
            "fix:{id}:prepared={}:original={:?}",
            prepared.is_some(),
            original.target()
        ));
        if self.change_source_to_on {
            assert_eq!(shared.snapshot().target(), TransitionTarget::RouterOn);
            assert_eq!(original.target(), TransitionTarget::NativeOff);
        }
        if self.fix_error {
            Err(err("fix"))
        } else {
            let mut r = result(id);
            if self.existing_postcondition_id {
                r.fixed_ids.push(POSTCONDITION.into());
            }
            Ok(r)
        }
    }
    fn fix_postcondition(
        &self,
        _: &RepairPaths,
        _: &StateHandle,
    ) -> Result<FixResult, Self::Error> {
        self.call("postcondition");
        if self.post_error {
            Err(err("post"))
        } else {
            Ok(result(POSTCONDITION))
        }
    }
    fn config_stale_reason(
        &self,
        _: &RepairPaths,
        _: &Snapshot,
    ) -> Result<Option<String>, Self::Error> {
        *self.stale_calls.lock().unwrap() += 1;
        Ok(self.stale_reasons.lock().unwrap().pop_front().flatten())
    }
    fn reconcile_target<R: Runner + Send + Sync + 'static>(
        &self,
        _: &RepairPaths,
        source: &Snapshot,
        _: RepairAttempt,
        _: Processes<'_, R>,
        _: &StateHandle,
    ) -> Result<ReconcileResult<()>, Self::Error> {
        self.call("reconcile");
        if self.reconcile_error {
            Err(err("reconcile"))
        } else {
            Ok(ReconcileResult {
                value: (),
                details: self.reconcile_details.clone(),
                warnings: self.reconcile_warnings.clone(),
                state_after: source.clone(),
            })
        }
    }
    fn run_quiet_transition<R: Runner + Send + Sync + 'static>(
        &self,
        r: QuietRepairRequest<'_, String>,
        _: Processes<'_, R>,
        shared: &StateHandle,
        repair: &mut dyn FnMut(
            RepairAttempt,
        ) -> Result<
            CompletedRepair<Self::Reconciled>,
            ManagerError<Self::Error>,
        >,
    ) -> TransitionOutcome<Self::Reconciled, ManagerError<Self::Error>> {
        self.call(format!(
            "quiet:{}:missing={}:source={:?}",
            r.selected,
            r.include_missing_rollouts,
            r.source.target()
        ));
        assert_eq!(r.reason, REPAIR_REASON);
        assert_eq!(r.policy, RepairTransitionPolicy::Strict);
        assert_eq!(
            r.writer_policy,
            atelier_maintenance_core::process::ProcessPolicy::ForceCloseAll
        );
        assert_eq!(r.graceful_timeout, Duration::from_secs(5));
        if self.pending_during_transition {
            JournalStore::new(&r.paths.verification.relay_root)
                .write_with_source(
                    TransitionTarget::NativeOff,
                    "committing",
                    Some("test"),
                    Some("test"),
                    1,
                )
                .unwrap();
        }
        if self.change_source_to_on {
            shared
                .shared()
                .lock()
                .unwrap()
                .set_router_flags(true, false);
        }
        match &self.script {
            Script::Failure(stage, attempt) => TransitionOutcome::Failed(TransitionFailure {
                error: ManagerError::Primitive(err("transition")),
                stage: *stage,
                attempt: *attempt,
            }),
            Script::CallbackFailure => match repair(RepairAttempt {
                number: 1,
                initially_quiet: true,
            }) {
                Err(e) => TransitionOutcome::Failed(TransitionFailure {
                    error: e,
                    stage: FailureStage::Reconcile,
                    attempt: 1,
                }),
                Ok(_) => panic!("expected failing dispatch"),
            },
            Script::Verified(w) => match repair(RepairAttempt {
                number: 1,
                initially_quiet: true,
            }) {
                Ok(value) => {
                    let mut warnings = value.warnings().to_vec();
                    warnings.extend(w.clone());
                    TransitionOutcome::from_verification(
                        value,
                        VerificationReport::default(),
                        warnings,
                        Some(r.source.target()),
                    )
                }
                Err(error) => TransitionOutcome::Failed(TransitionFailure {
                    error,
                    stage: FailureStage::Reconcile,
                    attempt: 1,
                }),
            },
            Script::Unverified => TransitionOutcome::from_verification(
                repair(RepairAttempt {
                    number: 1,
                    initially_quiet: true,
                })
                .unwrap(),
                VerificationReport {
                    issues: vec![
                        VerificationIssue {
                            dim: InvariantDim::Config,
                            message: "first".into(),
                        },
                        VerificationIssue {
                            dim: InvariantDim::Auth,
                            message: "second".into(),
                        },
                    ],
                },
                vec!["warning".into()],
                None,
            ),
        }
    }
    fn restore_router_on(
        &self,
        _: &RepairPaths,
        source: &Snapshot,
        reason: &str,
        _: &StateHandle,
    ) -> Result<(), Self::Error> {
        assert_eq!(source.target(), TransitionTarget::RouterOn);
        assert!(reason.starts_with("diagnostic repair failed after mutation began: 73:"));
        self.call("restore-on");
        if self.restore_error {
            Err(err("restore"))
        } else {
            Ok(())
        }
    }
    fn rollback_native_off(
        &self,
        _: &RepairPaths,
        source: &Snapshot,
        reason: &str,
        _: &StateHandle,
    ) -> Result<Option<Vec<String>>, Self::Error> {
        assert_eq!(source.target(), TransitionTarget::NativeOff);
        assert!(reason.contains("after mutation began"));
        self.call("rollback-off");
        if self.restore_error {
            Err(err("restore"))
        } else {
            Ok(self.off_warnings.clone())
        }
    }
}
#[derive(Default)]
struct FakeRunner {
    running: Mutex<VecDeque<bool>>,
    last: Mutex<bool>,
    opens: Mutex<usize>,
    fail_open: bool,
    now: Mutex<Duration>,
}
impl FakeRunner {
    fn sequence(v: impl Into<VecDeque<bool>>) -> Self {
        Self {
            running: Mutex::new(v.into()),
            ..Self::default()
        }
    }
}
impl Runner for FakeRunner {
    fn output(&self, o: &Operation) -> io::Result<CommandOutput> {
        let text = match o {
            Operation::BundleValue {
                field: BundleField::Identifier,
                ..
            } => "com.openai.codex".into(),
            Operation::BundleValue {
                field: BundleField::Executable,
                ..
            } => "ChatGPT".into(),
            Operation::Processes => {
                if let Some(b) = self.running.lock().unwrap().pop_front() {
                    *self.last.lock().unwrap() = b;
                }
                if *self.last.lock().unwrap() {
                    "42 /Applications/ChatGPT.app/Contents/MacOS/ChatGPT".into()
                } else {
                    String::new()
                }
            }
            Operation::IsAppRunning => "false".into(),
            Operation::OpenBundle(_) => {
                *self.opens.lock().unwrap() += 1;
                if self.fail_open {
                    return Err(io::Error::other("fixture open failure"));
                }
                String::new()
            }
            other => panic!("unexpected real operation {other:?}"),
        };
        Ok(CommandOutput {
            success: true,
            stdout: text.into_bytes(),
            stderr: vec![],
        })
    }
    fn signal_kill(&self, _: u32) -> io::Result<()> {
        panic!("Manager driver never signals outside required transition operation")
    }
    fn exists(&self, _: &Path) -> bool {
        false
    }
    fn is_file(&self, p: &Path) -> bool {
        p == Path::new("/Applications/ChatGPT.app/Contents/MacOS/ChatGPT")
    }
    fn read_text(&self, _: &Path) -> io::Result<String> {
        panic!("no real fixture reads through runner")
    }
    fn read_dir(&self, _: &Path) -> io::Result<Vec<PathBuf>> {
        panic!("no real directory scan")
    }
    fn process_id(&self) -> u32 {
        999
    }
    fn now(&self) -> Duration {
        *self.now.lock().unwrap()
    }
    fn sleep(&self, d: Duration) {
        *self.now.lock().unwrap() += d;
    }
}
fn issue(id: &str) -> DiagnosticItem {
    DiagnosticItem {
        id: id.into(),
        status: "error".into(),
        label: String::new(),
        detail: String::new(),
        fixable: true,
    }
}
fn state(on: bool) -> RelayStateDocument {
    let mut s = RelayStateDocument::native_default();
    s.set_router_flags(on, false);
    s
}
fn manager(t: &TempDir, ops: Ops, runner: FakeRunner, on: bool) -> NativeManager<Ops, FakeRunner> {
    let p = RepairPaths::from_codex_home(t.path(), vec![]);
    NativeManager::from_loaded(
        ops,
        runner,
        ProcessContext::new(t.path().into(), None),
        p,
        Ok(state(on)),
        Arc::new(Mutex::new(None)),
    )
    .with_coordination(
        StateHandle::new(state(on)),
        Arc::new(Mutex::new(())),
        Arc::new(Mutex::new(())),
    )
}
fn outcome_error(e: RepairExecution<PrimitiveError>) -> ManagerError<PrimitiveError> {
    e.result.unwrap_err()
}
#[test]
fn initialization_load_failure_precedes_unknown_selector_and_locks() {
    let t = TempDir::new().unwrap();
    let m = NativeManager::from_loaded(
        Ops::default(),
        FakeRunner::default(),
        ProcessContext::new(t.path().into(), None),
        RepairPaths::from_codex_home(t.path(), vec![]),
        Err(err("load")),
        Arc::new(Mutex::new(None)),
    );
    assert!(
        matches!(m.repair("unknown").result,Err(ManagerError::Initialization(s)) if s.ends_with("73:load"))
    );
    assert!(m.operations().trace().is_empty());
}
#[test]
fn unknown_selector_is_rejected_without_io() {
    let t = TempDir::new().unwrap();
    let m = manager(&t, Ops::default(), FakeRunner::default(), false);
    assert!(matches!(
        m.repair("thread_history_integrity").result,
        Err(ManagerError::Operation(_))
    ));
    assert!(m.operations().trace().is_empty());
    assert!(!t.path().join("accounts").exists());
}
#[test]
fn ordinary_residual_does_not_prepare_preflight_transition_reconcile_or_relaunch() {
    let t = TempDir::new().unwrap();
    let m = manager(&t, Ops::default(), FakeRunner::sequence([true]), false);
    let e = m.repair("residual_cache");
    assert_eq!(e.result.unwrap().fixed_ids, ["residual_cache"]);
    assert_eq!(
        m.operations().trace(),
        [
            "paths",
            "diagnose",
            "fix:residual_cache:prepared=false:original=NativeOff"
        ]
    );
    assert_eq!(e.relaunch, RelaunchEffect::NotRequested);
    assert!(!t.path().join("accounts").exists());
}
#[test]
fn single_prepare_failure_returns_before_preflight_and_process_probe() {
    let t = TempDir::new().unwrap();
    let ops = Ops {
        prepare_error: true,
        report: vec![issue("db_orphan_providers")],
        ..Ops::default()
    };
    let m = manager(&t, ops, FakeRunner::sequence([true]), false);
    assert!(
        matches!(m.repair("db_orphan_providers").result,Err(ManagerError::Primitive(e)) if e.message=="prepare")
    );
    assert_eq!(m.operations().trace(), ["paths", "diagnose", "prepare"]);
    assert!(!t.path().join("accounts").exists());
    assert_eq!(m.runner().running.lock().unwrap().len(), 1);
}
#[test]
fn all_prepare_failure_is_warning_and_real_callback_runs_inside_transition_once() {
    let t = TempDir::new().unwrap();
    let ops = Ops {
        prepare_error: true,
        report: vec![issue("db_orphan_providers")],
        ..Ops::default()
    };
    let m = manager(&t, ops, FakeRunner::default(), false);
    let e = m.repair("all");
    let f = e.result.unwrap();
    assert_eq!(f.fixed_ids, ["all"]);
    assert_eq!(f.details, ["孤儿任务目录准备未完成：73:prepare"]);
    assert_eq!(
        m.operations().trace(),
        [
            "paths",
            "diagnose",
            "prepare",
            "quiet:all:missing=true:source=NativeOff",
            "fix:all:prepared=false:original=NativeOff"
        ]
    );
    assert!(t.path().join("accounts/registry-backups").is_dir());
    assert!(!t.path().join("codexmate/relay/state.json").exists());
}
#[test]
fn healthy_or_nonfixable_all_rows_do_not_trigger_quiet_window() {
    for mut row in [issue("config_stale"), issue("db_orphan_providers")] {
        row.status = "ok".into();
        let t = TempDir::new().unwrap();
        let m = manager(
            &t,
            Ops {
                report: vec![row],
                ..Ops::default()
            },
            FakeRunner::default(),
            false,
        );
        let e = m.repair("all");
        assert!(e.result.is_ok());
        assert!(!e.gates.unwrap().use_quiet_transition);
        assert!(m
            .operations()
            .trace()
            .iter()
            .all(|s| !s.starts_with("quiet:") && s != "prepare"));
    }
}
#[test]
fn direct_postcondition_calls_audit_fix_once_and_bypasses_preflight() {
    let t = TempDir::new().unwrap();
    let m = manager(&t, Ops::default(), FakeRunner::default(), true);
    let e = m.repair(POSTCONDITION);
    assert_eq!(e.result.unwrap().fixed_ids, [POSTCONDITION]);
    assert_eq!(
        m.operations().trace(),
        ["paths", "diagnose", "postcondition"]
    );
    assert_eq!(
        e.audit,
        [AuditEvent {
            step: AuditStep::Fix,
            success: true
        }]
    );
    assert!(!t.path().join("accounts").exists());
}
#[test]
fn quiet_on_checks_readiness_after_real_preflight_before_process_probe() {
    let t = TempDir::new().unwrap();
    let m = manager(
        &t,
        Ops {
            ready_error: true,
            ..Ops::default()
        },
        FakeRunner::sequence([true]),
        true,
    );
    assert!(
        matches!(m.repair("config_stale").result,Err(ManagerError::Primitive(e)) if e.message=="ready")
    );
    assert_eq!(m.operations().trace(), ["paths", "diagnose", "ready"]);
    assert!(t.path().join("accounts/backups").is_dir());
    assert_eq!(m.runner().running.lock().unwrap().len(), 1);
}
#[test]
fn preflight_directory_target_error_is_not_ignored() {
    let t = TempDir::new().unwrap();
    fs::create_dir_all(t.path().join("codexmate/relay/state.json")).unwrap();
    let m = manager(&t, Ops::default(), FakeRunner::default(), false);
    let e = m.repair("config_stale");
    assert!(
        matches!(e.result,Err(ManagerError::Operation(s)) if s.starts_with("diagnostic repair: cannot start because relay state is not safely writable:"))
    );
    assert_eq!(m.operations().trace(), ["paths", "diagnose"]);
}
#[test]
fn failure_stage_and_attempt_control_compensation_not_success() {
    for (stage, attempt, comp) in [
        (FailureStage::Preflight, 0, false),
        (FailureStage::Stop, 1, false),
        (FailureStage::Stop, 2, true),
        (FailureStage::Preflight, 3, true),
        (FailureStage::Reconcile, 1, true),
        (FailureStage::Settle, 1, true),
        (FailureStage::Verify, 3, true),
        (FailureStage::Commit, 1, true),
    ] {
        let t = TempDir::new().unwrap();
        let m = manager(
            &t,
            Ops {
                script: Script::Failure(stage, attempt),
                ..Ops::default()
            },
            FakeRunner::default(),
            false,
        );
        let e = m.repair("config_stale");
        assert_eq!(e.transition_failure, Some((stage, attempt)));
        assert!(matches!(
            e.result,
            Err(ManagerError::Primitive(PrimitiveError {
                code: 73,
                message: "transition"
            }))
        ));
        assert_eq!(
            m.operations().trace().iter().any(|s| s == "rollback-off"),
            comp,
            "{stage:?}/{attempt}"
        );
    }
}
#[test]
fn compensation_restores_source_mode_and_retains_primary_error_payload() {
    let t = TempDir::new().unwrap();
    let m = manager(
        &t,
        Ops {
            script: Script::Failure(FailureStage::Commit, 1),
            restore_error: true,
            ..Ops::default()
        },
        FakeRunner::default(),
        true,
    );
    let e = outcome_error(m.repair("config_stale"));
    assert!(e
        .to_string()
        .contains("73:transition; restoring the pre-repair router state also failed: 73:restore"));
    match e {
        ManagerError::Restoration {
            primary,
            restoration,
        } => {
            assert!(matches!(
                *primary,
                ManagerError::Primitive(PrimitiveError {
                    code: 73,
                    message: "transition"
                })
            ));
            assert!(matches!(
                *restoration,
                ManagerError::Primitive(PrimitiveError {
                    code: 73,
                    message: "restore"
                })
            ));
        }
        other => panic!("{other:?}"),
    };
    assert!(m.operations().trace().contains(&"restore-on".into()));
}
#[test]
fn native_off_some_empty_restoration_warnings_still_mean_failure() {
    let t = TempDir::new().unwrap();
    let m = manager(
        &t,
        Ops {
            script: Script::Failure(FailureStage::Reconcile, 1),
            off_warnings: Some(vec![]),
            ..Ops::default()
        },
        FakeRunner::default(),
        false,
    );
    let e = outcome_error(m.repair("config_stale"));
    assert!(matches!(e, ManagerError::Restoration { .. }));
    assert!(e.to_string().ends_with(
        "source NativeOff core state was restored but diagnostics still require recovery: "
    ));
}
#[test]
fn unverified_outcome_is_error_with_original_issue_join_not_fake_success() {
    let t = TempDir::new().unwrap();
    let m = manager(
        &t,
        Ops {
            script: Script::Unverified,
            ..Ops::default()
        },
        FakeRunner::default(),
        false,
    );
    assert_eq!(
        outcome_error(m.repair("config_stale")).to_string(),
        format!("{UNVERIFIED_PREFIX}first | second")
    );
    assert!(m.operations().trace().iter().all(|s| s != "rollback-off"));
}
#[test]
fn callback_failure_is_preserved_and_not_dispatched_outside_transition() {
    let t = TempDir::new().unwrap();
    let m = manager(
        &t,
        Ops {
            fix_error: true,
            script: Script::CallbackFailure,
            ..Ops::default()
        },
        FakeRunner::default(),
        false,
    );
    assert!(
        matches!(m.repair("config_stale").result,Err(ManagerError::Primitive(e)) if e.message=="fix")
    );
    assert_eq!(
        m.operations()
            .trace()
            .iter()
            .filter(|s| s.starts_with("fix:"))
            .count(),
        1
    );
}
#[test]
fn verified_transition_warnings_append_without_turning_success_into_failure() {
    let t = TempDir::new().unwrap();
    let m = manager(
        &t,
        Ops {
            script: Script::Verified(vec!["durability warning".into()]),
            ..Ops::default()
        },
        FakeRunner::default(),
        false,
    );
    assert_eq!(
        m.repair("config_stale").result.unwrap().details,
        ["durability warning"]
    );
}
#[test]
fn quiet_success_relaunches_only_previously_running_stopped_client() {
    let t = TempDir::new().unwrap();
    let m = manager(
        &t,
        Ops::default(),
        FakeRunner::sequence([true, false, true]),
        false,
    );
    let e = m.repair("config_stale");
    assert_eq!(e.relaunch, RelaunchEffect::Launched { pid: 42 });
    assert_eq!(*m.runner().opens.lock().unwrap(), 2);
    assert!(e
        .result
        .unwrap()
        .details
        .contains(&"已自动重新打开 ChatGPT 使修复生效".into()));
}
#[test]
fn already_running_retains_native_detail_quirk_without_claiming_actual_launch_effect() {
    let t = TempDir::new().unwrap();
    let m = manager(
        &t,
        Ops::default(),
        FakeRunner::sequence([true, true]),
        false,
    );
    let e = m.repair("config_stale");
    assert_eq!(e.relaunch, RelaunchEffect::AlreadyRunning);
    assert_eq!(*m.runner().opens.lock().unwrap(), 0);
    assert_eq!(
        e.result.unwrap().details,
        ["已自动重新打开 ChatGPT 使修复生效"]
    );
    assert!(e.audit.iter().all(|a| a.step != AuditStep::Relaunch));
}
#[test]
fn success_launch_failure_is_warning_not_repair_failure() {
    let t = TempDir::new().unwrap();
    let m = manager(
        &t,
        Ops::default(),
        FakeRunner {
            fail_open: true,
            ..FakeRunner::sequence([true, false])
        },
        false,
    );
    let e = m.repair("config_stale");
    assert_eq!(e.relaunch, RelaunchEffect::Failed);
    assert!(e.result.unwrap().details[0].starts_with("修复成功，但自动重新打开 ChatGPT 失败："));
}
#[test]
fn failing_quiet_transition_requires_actual_confirmed_source_before_relaunch() {
    let t = TempDir::new().unwrap();
    let m = manager(
        &t,
        Ops {
            script: Script::Failure(FailureStage::Stop, 1),
            ..Ops::default()
        },
        FakeRunner::sequence([true, false, true]),
        false,
    );
    let e = m.repair("config_stale");
    assert_eq!(e.relaunch, RelaunchEffect::Launched { pid: 42 });
    assert!(e.result.is_err());
    assert_eq!(*m.runner().opens.lock().unwrap(), 2);
}
#[test]
fn pending_journal_created_during_failure_blocks_relaunch_even_when_off_files_clean() {
    let t = TempDir::new().unwrap();
    let m = manager(
        &t,
        Ops {
            pending_during_transition: true,
            script: Script::Failure(FailureStage::Stop, 1),
            ..Ops::default()
        },
        FakeRunner::sequence([true, false, true]),
        false,
    );
    let e = m.repair("config_stale");
    assert_eq!(e.relaunch, RelaunchEffect::UnconfirmedTarget);
    assert_eq!(*m.runner().opens.lock().unwrap(), 0);
    assert!(e.result.is_err());
    assert_eq!(m.runner().running.lock().unwrap().len(), 2);
}
#[test]
fn failed_repair_and_failed_relaunch_retain_both_errors() {
    let t = TempDir::new().unwrap();
    let m = manager(
        &t,
        Ops {
            script: Script::Failure(FailureStage::Stop, 1),
            ..Ops::default()
        },
        FakeRunner {
            fail_open: true,
            ..FakeRunner::sequence([true, false])
        },
        false,
    );
    let e = outcome_error(m.repair("config_stale"));
    assert!(matches!(e, ManagerError::Relaunch { .. }));
    assert!(e
        .to_string()
        .contains("diagnostic repair failed: 73:transition; ChatGPT relaunch also failed:"));
}
#[test]
fn ordinary_dispatch_failure_does_not_probe_or_relaunch_after_error() {
    let t = TempDir::new().unwrap();
    let m = manager(
        &t,
        Ops {
            fix_error: true,
            ..Ops::default()
        },
        FakeRunner::sequence([true, false, true]),
        false,
    );
    let e = m.repair("residual_manifest");
    assert_eq!(e.relaunch, RelaunchEffect::NotRequested);
    assert!(e.result.is_err());
    assert_eq!(m.runner().running.lock().unwrap().len(), 2);
}
#[test]
fn locks_are_released_after_every_normal_early_return() {
    let t = TempDir::new().unwrap();
    let outer = Arc::new(Mutex::new(()));
    let inner = Arc::new(Mutex::new(()));
    let m = manager(
        &t,
        Ops {
            ready_error: true,
            ..Ops::default()
        },
        FakeRunner::default(),
        true,
    )
    .with_coordination(StateHandle::new(state(true)), outer.clone(), inner.clone());
    assert!(m.repair("config_stale").result.is_err());
    assert!(outer.try_lock().is_ok());
    assert!(inner.try_lock().is_ok());
}
#[test]
fn poisoned_outer_and_inner_locks_have_distinct_native_errors() {
    for outer_poison in [true, false] {
        let t = TempDir::new().unwrap();
        let outer = Arc::new(Mutex::new(()));
        let inner = Arc::new(Mutex::new(()));
        let poison = if outer_poison {
            outer.clone()
        } else {
            inner.clone()
        };
        let _ = std::thread::spawn(move || {
            let _guard = poison.lock().unwrap();
            panic!("fixture poison");
        })
        .join();
        let m = manager(&t, Ops::default(), FakeRunner::default(), false).with_coordination(
            StateHandle::new(state(false)),
            outer.clone(),
            inner.clone(),
        );
        let e = outcome_error(m.repair("residual_cache"));
        assert_eq!(
            e.to_string(),
            if outer_poison {
                "Codex router transition lock is poisoned"
            } else {
                "Codex mutation gate is poisoned while repairing Codex diagnostics"
            }
        );
        assert!(m.operations().trace().is_empty());
        if !outer_poison {
            assert!(outer.try_lock().is_ok());
        }
    }
}
#[test]
fn state_preflight_preserves_existing_bytes_and_removes_private_temporary() {
    let t = TempDir::new().unwrap();
    let paths = RepairPaths::from_codex_home(t.path(), vec![]);
    fs::create_dir_all(&paths.verification.relay_root).unwrap();
    fs::write(&paths.state_path, b"untouched original").unwrap();
    preflight_state_write(&paths, &state(false)).unwrap();
    assert_eq!(fs::read(&paths.state_path).unwrap(), b"untouched original");
    assert!(fs::read_dir(&paths.verification.relay_root)
        .unwrap()
        .all(|e| !e
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".relay-state-preflight-")));
    for p in [
        "accounts",
        "accounts/snapshots",
        "accounts/backups",
        "accounts/registry-backups",
        "codexmate/skill-backups",
        "codexmate/custom-instructions/history",
        "codexmate/relay",
    ] {
        assert!(t.path().join(p).is_dir(), "{p}");
    }
}
#[test]
fn receipt_not_found_only_passes_for_off_without_commit_id() {
    let t = TempDir::new().unwrap();
    let paths = RepairPaths::from_codex_home(t.path(), vec![]);
    assert!(verify_router_commit(&paths, &state(false)).is_ok());
    assert_eq!(
        verify_router_commit(&paths, &state(true))
            .unwrap_err()
            .to_string(),
        COMMIT_STATE_CONFLICT
    );
    let mut s = state(false);
    s.set_router_commit_id(Some(
        serde_json::from_str("\"3f4e7a87-399a-4bba-aed7-d06b1ff96023\"").unwrap(),
    ));
    assert!(verify_router_commit(&paths, &s).is_err());
}
#[test]
fn receipt_matches_only_native_four_fields_and_ignores_off_noaccount() {
    let t = TempDir::new().unwrap();
    let p = RepairPaths::from_codex_home(t.path(), vec![]);
    fs::create_dir_all(&p.verification.relay_root).unwrap();
    for content in [
        r#"{"schemaVersion":16,"codexRouterEnabled":false,"codexRouterNoAccountMode":true,"providers":"ignored"}"#,
        r#"[16,null,false,true]"#,
    ] {
        fs::write(&p.state_path, content).unwrap();
        assert!(verify_router_commit(&p, &state(false)).is_ok(), "{content}");
    }
    for content in [
        r#"{"schemaVersion":17,"codexRouterEnabled":false,"codexRouterNoAccountMode":false}"#,
        r#"{"schemaVersion":16,"codexRouterEnabled":true,"codexRouterNoAccountMode":false}"#,
        r#"{"schemaVersion":16,"codexRouterEnabled":false}"#,
        r#"{"schemaVersion":16,"schemaVersion":16,"codexRouterEnabled":false,"codexRouterNoAccountMode":false}"#,
        r#"{"schemaVersion":16,"routerCommitId":"bad","codexRouterEnabled":false,"codexRouterNoAccountMode":false}"#,
        "not json",
    ] {
        fs::write(&p.state_path, content).unwrap();
        assert_eq!(
            verify_router_commit(&p, &state(false))
                .unwrap_err()
                .to_string(),
            COMMIT_STATE_CONFLICT,
            "{content}"
        );
    }
}
#[test]
fn direct_postcondition_error_can_relaunch_only_after_actual_off_confirmation() {
    let t = TempDir::new().unwrap();
    let m = manager(
        &t,
        Ops {
            post_error: true,
            ..Ops::default()
        },
        FakeRunner::sequence([true, false, true]),
        false,
    );
    let e = m.repair(POSTCONDITION);
    assert_eq!(e.relaunch, RelaunchEffect::Launched { pid: 42 });
    assert!(matches!(e.result,Err(ManagerError::Primitive(e)) if e.message=="post"));
}
fn fallback_entry(id: &str, original: &str, fallback: &str) -> serde_json::Value {
    serde_json::json!({"thread_id":id,"original_model":original,"fallback_model":fallback,"attempts":0})
}
fn write_fallback(t: &TempDir, entries: Vec<serde_json::Value>) -> PathBuf {
    let relay = t.path().join("codexmate/relay");
    fs::create_dir_all(&relay).unwrap();
    fs::write(
        relay.join("router_model_restore.json"),
        serde_json::to_vec(&serde_json::json!({"version":1,"entries":entries})).unwrap(),
    )
    .unwrap();
    relay
}
#[test]
fn fallback_guard_uses_raw_thread_identity_and_prefix_only_original_model() {
    let t = TempDir::new().unwrap();
    let relay = write_fallback(
        &t,
        vec![
            fallback_entry("t", "aimami_relay_", " model "),
            fallback_entry(" t ", "aimami_relay_::anything", "bare"),
        ],
    );
    assert!(validate_fallback_model_restore_journal(&relay).is_ok());
}
#[test]
fn fallback_guard_rejects_actual_native_predicate_failures() {
    for entries in [
        vec![fallback_entry(" ", "aimami_relay_x", "bare")],
        vec![fallback_entry("t", " aimami_relay_x", "bare")],
        vec![fallback_entry("t", "aimami_relay_x", " ")],
        vec![
            fallback_entry("t", "aimami_relay_x", "bare"),
            fallback_entry("t", "aimami_relay_y", "bare"),
        ],
    ] {
        let t = TempDir::new().unwrap();
        let relay = write_fallback(&t, entries);
        assert_eq!(
            validate_fallback_model_restore_journal(&relay)
                .unwrap_err()
                .to_string(),
            "中转原模型恢复记录内容异常或版本不兼容，本次操作未能完成。"
        );
    }
}
#[test]
fn fallback_guard_schema_and_path_failures_use_redacted_native_categories() {
    let t = TempDir::new().unwrap();
    let relay = write_fallback(&t, vec![]);
    let path = relay.join("router_model_restore.json");
    fs::write(&path, b"{private broken value").unwrap();
    let e = validate_fallback_model_restore_journal(&relay)
        .unwrap_err()
        .to_string();
    assert_eq!(
        e,
        "中转原模型恢复记录格式异常或版本不兼容，本次操作未能完成。"
    );
    assert!(!e.contains("private"));
    fs::remove_file(&path).unwrap();
    fs::create_dir(&path).unwrap();
    assert!(validate_fallback_model_restore_journal(&relay)
        .unwrap_err()
        .to_string()
        .contains("路径类型异常"));
}
#[test]
fn fallback_guard_checks_metadata_size_before_parsing() {
    let t = TempDir::new().unwrap();
    let relay = write_fallback(&t, vec![]);
    fs::File::create(relay.join("router_model_restore.json"))
        .unwrap()
        .set_len(8 * 1024 * 1024 + 1)
        .unwrap();
    assert!(validate_fallback_model_restore_journal(&relay)
        .unwrap_err()
        .to_string()
        .contains("超过读取上限"));
}
#[test]
fn recovery_guard_checked_journal_precedes_model_and_takeover_errors() {
    let t = TempDir::new().unwrap();
    let relay = write_fallback(&t, vec![fallback_entry(" ", "wrong", " ")]);
    fs::write(
        relay.join("router-transition-journal.json"),
        b"{\"schemaVersion\":99}",
    )
    .unwrap();
    let e = ensure_recovery_records_allowed(&relay)
        .unwrap_err()
        .to_string();
    assert_eq!(e, "unsupported transition record version");
    fs::remove_file(relay.join("router-transition-journal.json")).unwrap();
    assert!(ensure_recovery_records_allowed(&relay)
        .unwrap_err()
        .to_string()
        .contains("内容异常或版本不兼容"));
}
#[test]
fn recovery_guard_validates_both_model_restore_files_before_backups() {
    let t = TempDir::new().unwrap();
    let relay = write_fallback(&t, vec![]);
    fs::write(
        relay.join("router_no_account_model_restore.json"),
        b"not json",
    )
    .unwrap();
    fs::write(relay.join("router-takeover-backup.json"), b"{}").unwrap();
    let e = ensure_recovery_records_allowed(&relay)
        .unwrap_err()
        .to_string();
    assert!(e.contains("decode no-account model restore journal"));
    assert!(!e.contains("unsupported recovery schema"));
}
#[test]
fn takeover_guard_allows_damaged_json_for_quarantine_but_rejects_future_valid_schema() {
    let t = TempDir::new().unwrap();
    let relay = write_fallback(&t, vec![]);
    let path = relay.join("router-takeover-backup.json");
    fs::write(&path, b"broken JSON").unwrap();
    assert!(ensure_recovery_records_allowed(&relay).is_ok());
    assert_eq!(fs::read(&path).unwrap(), b"broken JSON");
    fs::write(&path, b"{}").unwrap();
    assert!(ensure_recovery_records_allowed(&relay)
        .unwrap_err()
        .to_string()
        .contains("unsupported recovery schema"));
}
#[test]
fn pending_receipt_guard_is_not_blanket_pending_rejection() {
    let t = TempDir::new().unwrap();
    let p = RepairPaths::from_codex_home(t.path(), vec![]);
    fs::create_dir_all(&p.verification.relay_root).unwrap();
    JournalStore::new(&p.verification.relay_root)
        .write(TransitionTarget::NativeOff, "committing", None, 1)
        .unwrap()
        .acknowledge();
    let m = manager(&t, Ops::default(), FakeRunner::default(), false);
    assert!(m.repair("residual_cache").result.is_ok());
    assert!(JournalStore::new(&p.verification.relay_root)
        .commit_pending()
        .unwrap());
}
#[test]
fn pending_receipt_guard_conflict_returns_before_diagnostics() {
    let t = TempDir::new().unwrap();
    let p = RepairPaths::from_codex_home(t.path(), vec![]);
    fs::create_dir_all(&p.verification.relay_root).unwrap();
    JournalStore::new(&p.verification.relay_root)
        .write(TransitionTarget::RouterOn, "committing", None, 1)
        .unwrap()
        .acknowledge();
    let m = manager(&t, Ops::default(), FakeRunner::default(), true);
    assert_eq!(
        outcome_error(m.repair("residual_cache")).to_string(),
        COMMIT_STATE_CONFLICT
    );
    assert!(m.operations().trace().is_empty());
}
#[test]
fn noncommitting_journal_does_not_compare_unrelated_disk_receipt() {
    let t = TempDir::new().unwrap();
    let p = RepairPaths::from_codex_home(t.path(), vec![]);
    fs::create_dir_all(&p.verification.relay_root).unwrap();
    JournalStore::new(&p.verification.relay_root)
        .write(TransitionTarget::NativeOff, "degraded", None, 1)
        .unwrap()
        .acknowledge();
    fs::write(&p.state_path, b"not a receipt").unwrap();
    let m = manager(&t, Ops::default(), FakeRunner::default(), true);
    assert!(m.repair("residual_manifest").result.is_ok());
}
struct PersistFixture {
    _temp: TempDir,
    paths: RepairPaths,
    shared: StateHandle,
    proxy: Mutex<Option<atelier_maintenance_core::router_verification::ProxyHandle>>,
    cache: Mutex<CatalogRuntime<&'static str>>,
}
impl PersistFixture {
    fn new(document: RelayStateDocument) -> Self {
        let temp = TempDir::new().unwrap();
        let paths = RepairPaths::from_codex_home(temp.path(), vec![]);
        Self {
            _temp: temp,
            paths,
            shared: StateHandle::new(document),
            proxy: Mutex::new(None),
            cache: Mutex::new(CatalogRuntime {
                generation: 7,
                preparation_generation: 9,
                cached: Some("prepared artifact"),
            }),
        }
    }
    fn context(&self) -> PersistenceContext<'_, &'static str> {
        PersistenceContext {
            paths: &self.paths,
            state: &self.shared,
            proxy: &self.proxy,
            catalog: &self.cache,
            initialization_error: None,
        }
    }
}
fn operation_id(n: u8) -> atelier_maintenance_core::catalog::state::RouterOperationId {
    format!("3f4e7a87-399a-4bba-aed7-d06b1ff960{n:02}")
        .try_into()
        .unwrap()
}
fn edit_document(
    s: &RelayStateDocument,
    edit: impl FnOnce(&mut serde_json::Value),
) -> RelayStateDocument {
    let mut v = s.as_value().clone();
    edit(&mut v);
    RelayStateDocument::from_value(v).unwrap()
}
#[test]
fn native_status_is_handle_presence_and_port_even_with_dead_flag() {
    use std::sync::atomic::AtomicBool;
    let proxy = Mutex::new(Some(
        atelier_maintenance_core::router_verification::ProxyHandle {
            port: 43210,
            alive: Arc::new(AtomicBool::new(false)),
        },
    ));
    let s = compose_proxy_status(&proxy);
    assert!(s.running);
    assert_eq!(s.port, 43210);
    assert_eq!(s.base_url, "http://127.0.0.1:43210");
    assert_eq!(s.codex_base_url, "http://127.0.0.1:43210/codex/v1");
    assert_eq!(s.last_error, None);
}
#[test]
fn poisoned_proxy_status_defaults_without_clearing_poison() {
    let proxy = Arc::new(Mutex::new(None));
    let p = proxy.clone();
    let _ = std::thread::spawn(move || {
        let _g = p.lock().unwrap();
        panic!("fixture proxy poison");
    })
    .join();
    assert_eq!(compose_proxy_status(&proxy), Default::default());
    assert!(proxy.is_poisoned());
}
#[test]
fn native_empty_models_revision_is_sha256_of_compact_array() {
    assert_eq!(
        native_catalog_revision(&state(false)).unwrap(),
        "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945"
    );
}
#[test]
fn ordinary_state_save_preserves_current_commit_and_cached_preparation() {
    let mut document = state(false);
    document.set_router_commit_id(Some(operation_id(1)));
    let f = PersistFixture::new(document);
    let stale = edit_document(f.shared.snapshot().document(), |v| {
        v.as_object_mut().unwrap().remove("routerCommitId");
        v["claudeWebSearchCompat"] = false.into();
    });
    let publication = persist_with_commit(f.context(), stale, None).unwrap();
    assert!(!publication.catalog_invalidated);
    publication.publication.require_sync().unwrap();
    assert_eq!(
        f.shared.snapshot().document().router_commit_id().unwrap(),
        Some(operation_id(1))
    );
    assert!(f.cache.lock().unwrap().cached.is_some());
    assert_eq!(f.cache.lock().unwrap().generation, 7);
    verify_router_commit(&f.paths, f.shared.snapshot().document()).unwrap();
}
#[test]
fn ordinary_save_cannot_change_active_route_without_commit() {
    let f = PersistFixture::new(state(false));
    let mut candidate = state(true);
    candidate.set_router_flags(true, true);
    let e = persist_with_commit(f.context(), candidate, None).unwrap_err();
    assert!(matches!(
        e,
        PersistenceError::Conflict(ROUTE_CHANGE_REQUIRES_COMMIT)
    ));
    assert!(!f.paths.state_path.exists());
    assert_eq!(f.shared.snapshot().target(), TransitionTarget::NativeOff);
}
#[test]
fn transition_save_refuses_stale_proposal_receipt_before_writing() {
    let mut initial = state(false);
    initial.set_router_commit_id(Some(operation_id(1)));
    let f = PersistFixture::new(initial);
    let stale = state(true);
    let e = persist_with_commit(f.context(), stale, Some(operation_id(2))).unwrap_err();
    assert!(matches!(
        e,
        PersistenceError::Conflict(ROUTE_SNAPSHOT_CHANGED)
    ));
    assert_eq!(
        f.shared.snapshot().document().router_commit_id().unwrap(),
        Some(operation_id(1))
    );
    assert!(!f.paths.state_path.exists());
}
#[test]
fn off_noaccount_change_is_allowed_only_with_current_receipt_snapshot() {
    let mut initial = state(false);
    initial.set_router_commit_id(Some(operation_id(1)));
    let f = PersistFixture::new(initial);
    let mut changed = f.shared.snapshot().document().clone();
    changed.set_router_flags(false, true);
    persist_with_commit(f.context(), changed, None)
        .unwrap()
        .publication
        .require_sync()
        .unwrap();
    assert_eq!(f.cache.lock().unwrap().generation, 8);
    assert!(f.cache.lock().unwrap().cached.is_none());
    let mut stale = state(false);
    stale.set_router_flags(false, false);
    assert!(matches!(
        persist_with_commit(f.context(), stale, None),
        Err(PersistenceError::Conflict(ROUTE_SNAPSHOT_CHANGED))
    ));
}
struct FailStateIo;
impl StateIo for FailStateIo {
    fn publish(
        &self,
        _: &Path,
        _: &[u8],
    ) -> io::Result<atelier_maintenance_core::catalog::publication::Publication> {
        Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "fixture denied",
        ))
    }
}
#[test]
fn failed_publication_does_not_update_memory_or_invalidate_cache() {
    let f = PersistFixture::new(state(false));
    let e = persist_with_commit_using(
        f.context(),
        state(true),
        Some(operation_id(1)),
        &FailStateIo,
    )
    .unwrap_err();
    assert!(matches!(e, PersistenceError::Operation(_)));
    assert!(e.to_string().contains("write relay state failed:"));
    assert_eq!(f.shared.snapshot().target(), TransitionTarget::NativeOff);
    assert_eq!(f.cache.lock().unwrap().generation, 7);
    assert_eq!(f.cache.lock().unwrap().cached, Some("prepared artifact"));
}
struct SyncWarningIo {
    corrupt: bool,
}
impl StateIo for SyncWarningIo {
    fn publish(
        &self,
        p: &Path,
        b: &[u8],
    ) -> io::Result<atelier_maintenance_core::catalog::publication::Publication> {
        let mut receipt =
            atelier_maintenance_core::catalog::publication::write_atomic_report(p, b)?;
        if self.corrupt {
            fs::write(p, b"fixture concurrent corruption")?;
        }
        receipt.directory_sync_error = Some(io::Error::other("fixture directory sync failure"));
        Ok(receipt)
    }
}
#[test]
fn published_sync_warning_updates_memory_and_retains_uncertain_publication() {
    let f = PersistFixture::new(state(false));
    let p = persist_with_commit_using(
        f.context(),
        state(true),
        Some(operation_id(1)),
        &SyncWarningIo { corrupt: false },
    )
    .unwrap();
    assert!(p.receipt_warning.is_none());
    assert!(p.publication.directory_sync_error.is_some());
    assert_eq!(f.shared.snapshot().target(), TransitionTarget::RouterOn);
    assert_eq!(
        f.shared.snapshot().document().router_commit_id().unwrap(),
        Some(operation_id(1))
    );
    assert!(p.publication.require_sync().is_err());
    assert_eq!(f.cache.lock().unwrap().generation, 8);
    verify_router_commit(&f.paths, f.shared.snapshot().document()).unwrap();
}
#[test]
fn postpublication_receipt_conflict_is_warning_and_memory_still_advances() {
    let f = PersistFixture::new(state(false));
    let p = persist_with_commit_using(
        f.context(),
        state(true),
        Some(operation_id(1)),
        &SyncWarningIo { corrupt: true },
    )
    .unwrap();
    assert_eq!(
        p.receipt_warning.unwrap().to_string(),
        COMMIT_STATE_CONFLICT
    );
    assert_eq!(f.shared.snapshot().target(), TransitionTarget::RouterOn);
    assert_eq!(
        fs::read(&f.paths.state_path).unwrap(),
        b"fixture concurrent corruption"
    );
    assert!(p.publication.directory_sync_error.is_some());
}
fn provider_state(active: bool) -> RelayStateDocument {
    edit_document(&state(false), |v| {
        v["providers"] = serde_json::json!([{"id":"p","name":"Provider","baseUrl":"https://example.test/v1","model":"gpt-5.6","wireApi":"openai-responses"}]);
        v["activeByIde"]["codex"] = if active {
            serde_json::json!(["p"])
        } else {
            serde_json::json!([])
        };
    })
}
#[test]
fn only_active_exposed_model_revision_invalidates_actual_cache() {
    for active in [true, false] {
        let initial = provider_state(active);
        let f = PersistFixture::new(initial);
        let next = edit_document(f.shared.snapshot().document(), |v| {
            v["providers"][0]["model"] = "gpt-5.5".into()
        });
        let p = persist_with_commit(f.context(), next, None).unwrap();
        assert_eq!(p.catalog_invalidated, active);
        assert_eq!(
            f.cache.lock().unwrap().generation,
            if active { 8 } else { 7 }
        );
        assert_eq!(f.cache.lock().unwrap().cached.is_none(), active);
    }
}
#[test]
fn commit_target_derives_on_mode_from_actual_reconciliation_metadata() {
    let f = PersistFixture::new(state(false));
    let source = f.shared.snapshot();
    let p = commit_reconciled_target(
        f.context(),
        &ReconciledTarget::RouterOn {
            no_account_mode: true,
        },
        &source,
        &operation_id(1),
        |proposal| {
            assert_eq!(proposal.target(), TransitionTarget::RouterOn);
            assert!(
                proposal
                    .document()
                    .catalog_inputs()
                    .unwrap()
                    .no_account_mode
            );
            Ok::<_, PrimitiveError>(None)
        },
    )
    .unwrap();
    p.publication.require_sync().unwrap();
    assert!(
        f.shared
            .snapshot()
            .document()
            .catalog_inputs()
            .unwrap()
            .no_account_mode
    );
    assert_eq!(
        f.shared.snapshot().document().router_commit_id().unwrap(),
        Some(operation_id(1))
    );
}
#[test]
fn unavailable_native_off_window_does_not_call_stale_check_or_publish() {
    let f = PersistFixture::new(state(true));
    let result = commit_reconciled_target(
        f.context(),
        &ReconciledTarget::NativeOff {
            write_window: NativeOffWriteWindow::Unavailable,
        },
        &f.shared.snapshot(),
        &operation_id(1),
        |_| -> Result<Option<String>, PrimitiveError> {
            panic!("unavailable cannot reach stale check")
        },
    );
    assert!(
        matches!(result,Err(CommitError::Operation(s)) if s=="NativeOff was not applied because no verified ChatGPT write window was available")
    );
    assert!(!f.paths.state_path.exists());
}
#[test]
fn commit_stale_reason_preserves_native_error_prefix_and_original_state() {
    for on in [true, false] {
        let f = PersistFixture::new(state(!on));
        let target = if on {
            ReconciledTarget::RouterOn {
                no_account_mode: false,
            }
        } else {
            ReconciledTarget::NativeOff {
                write_window: NativeOffWriteWindow::Applied,
            }
        };
        let result = commit_reconciled_target(
            f.context(),
            &target,
            &f.shared.snapshot(),
            &operation_id(1),
            |_| Ok::<_, PrimitiveError>(Some("actual fixture config drift".into())),
        );
        let e = result.unwrap_err();
        assert_eq!(
            e.to_string(),
            format!(
                "{}actual fixture config drift",
                if on {
                    "RouterOn config does not match the reconciled auth mode: "
                } else {
                    "NativeOff config still contains router residue: "
                }
            )
        );
        assert!(!f.paths.state_path.exists());
        assert_eq!(
            f.shared.snapshot().target(),
            if on {
                TransitionTarget::NativeOff
            } else {
                TransitionTarget::RouterOn
            }
        );
    }
}
#[test]
fn storage_schema_commit_guard_and_invalid_credentials_precede_any_publication() {
    let t = TempDir::new().unwrap();
    let paths = RepairPaths::from_codex_home(t.path(), vec![]);
    for version in [14, 17] {
        let document = edit_document(&state(false), |v| {
            v["schemaVersion"] = version.into();
            v["routerCommitId"] = operation_id(1).as_str().into();
        });
        let e = save_state_with_publication(&paths, &document).unwrap_err();
        assert_eq!(
            e.to_string(),
            "relay state schema cannot represent this router commit receipt"
        );
    }
    assert!(!paths.state_path.exists());
    assert!(!t.path().join("accounts").exists());
}
#[test]
fn quiet_all_postcondition_report_forces_native_reconcile_even_when_files_satisfy_off() {
    let t = TempDir::new().unwrap();
    let m = manager(
        &t,
        Ops {
            report: vec![issue(POSTCONDITION)],
            ..Ops::default()
        },
        FakeRunner::default(),
        false,
    );
    let e = m.repair("all");
    assert_eq!(e.result.unwrap().fixed_ids, ["all", POSTCONDITION]);
    assert_eq!(
        m.operations()
            .trace()
            .iter()
            .filter(|s| *s == "reconcile")
            .count(),
        1
    );
    assert_eq!(*m.operations().stale_calls.lock().unwrap(), 1);
}
#[test]
fn actual_config_invariant_can_trigger_reconcile_without_report_postcondition() {
    let t = TempDir::new().unwrap();
    fs::write(t.path().join("config.toml"), "model_provider='aimai1'\n").unwrap();
    let m = manager(&t, Ops::default(), FakeRunner::default(), false);
    let f = m.repair("config_stale").result.unwrap();
    assert_eq!(f.fixed_ids, ["config_stale", POSTCONDITION]);
    assert!(m.operations().trace().contains(&"reconcile".into()));
    assert_eq!(*m.operations().stale_calls.lock().unwrap(), 1);
}
#[test]
fn first_stale_reason_triggers_reconcile_then_second_check_can_clear() {
    let t = TempDir::new().unwrap();
    let ops = Ops {
        stale_reasons: Mutex::new([Some("actual initial drift".into()), None].into()),
        ..Ops::default()
    };
    let m = manager(&t, ops, FakeRunner::default(), false);
    assert!(m.repair("config_stale").result.is_ok());
    assert!(m.operations().trace().contains(&"reconcile".into()));
    assert_eq!(*m.operations().stale_calls.lock().unwrap(), 2);
}
#[test]
fn second_stale_check_fails_instead_of_silently_reconciling_again() {
    let t = TempDir::new().unwrap();
    let ops = Ops {
        stale_reasons: Mutex::new([None, Some("late real drift".into())].into()),
        ..Ops::default()
    };
    let m = manager(&t, ops, FakeRunner::default(), false);
    let e = m.repair("config_stale");
    assert_eq!(e.transition_failure, Some((FailureStage::Reconcile, 1)));
    assert_eq!(
        e.result.unwrap_err().to_string(),
        "diagnostic repair left stale Codex config: late real drift"
    );
    assert!(!m.operations().trace().contains(&"reconcile".into()));
    assert!(m.operations().trace().contains(&"rollback-off".into()));
}
#[test]
fn reconciliation_details_warnings_and_existing_fixed_id_stay_separate() {
    let t = TempDir::new().unwrap();
    let ops = Ops {
        report: vec![issue(POSTCONDITION)],
        existing_postcondition_id: true,
        reconcile_details: vec!["concrete reconciliation detail".into()],
        reconcile_warnings: vec!["reconciliation warning".into()],
        script: Script::Verified(vec!["transition warning".into()]),
        ..Ops::default()
    };
    let m = manager(&t, ops, FakeRunner::default(), false);
    let f = m.repair("all").result.unwrap();
    assert_eq!(f.fixed_ids, ["all", POSTCONDITION]);
    assert_eq!(
        f.details,
        [
            "concrete reconciliation detail",
            "reconciliation warning",
            "transition warning"
        ]
    );
}
#[test]
fn reconcile_primitive_error_keeps_payload_and_compensates_source() {
    let t = TempDir::new().unwrap();
    let m = manager(
        &t,
        Ops {
            report: vec![issue(POSTCONDITION)],
            reconcile_error: true,
            ..Ops::default()
        },
        FakeRunner::default(),
        false,
    );
    let e = m.repair("all");
    assert!(matches!(
        e.result,
        Err(ManagerError::Primitive(PrimitiveError {
            code: 73,
            message: "reconcile"
        }))
    ));
    assert!(e.audit.contains(&AuditEvent {
        step: AuditStep::Reconcile,
        success: false
    }));
    assert!(m.operations().trace().contains(&"rollback-off".into()));
}

// Deliberately a different source type: there is no RelayStateDocument in this host.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct GatewayCatalogFixture {
    enabled: bool,
    provider: String,
    base_url: String,
    model: String,
}
#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct RouterIntentFixture {
    target: TransitionTarget,
    operation_id: String,
}
#[derive(Clone)]
struct GatewaySnapshotFixture {
    catalog: GatewayCatalogFixture,
    intent: Option<RouterIntentFixture>,
    observed_owned_config: bool,
}
impl RepairSnapshot for GatewaySnapshotFixture {
    fn target(&self) -> TransitionTarget {
        self.intent
            .as_ref()
            .map(|i| i.target)
            .unwrap_or(if self.observed_owned_config {
                TransitionTarget::RouterOn
            } else {
                TransitionTarget::NativeOff
            })
    }
}
struct GatewayHostFixture {
    state: Mutex<GatewaySnapshotFixture>,
    catalog_path: PathBuf,
    intent_path: PathBuf,
    preflight_count: Mutex<usize>,
    verification_count: Mutex<usize>,
}
impl GatewayHostFixture {
    fn new(t: &TempDir, on: bool) -> Self {
        let catalog = GatewayCatalogFixture {
            enabled: true,
            provider: "fixture-gateway".into(),
            base_url: "http://127.0.0.1:45678/v1".into(),
            model: "fixture-model".into(),
        };
        let catalog_path = t.path().join("app-data/gateway/catalog.json");
        let intent_path = t.path().join("app-data/gateway/router-intent.json");
        fs::create_dir_all(catalog_path.parent().unwrap()).unwrap();
        fs::write(&catalog_path, serde_json::to_vec(&catalog).unwrap()).unwrap();
        let intent = on.then(|| RouterIntentFixture {
            target: TransitionTarget::RouterOn,
            operation_id: "fixture-original".into(),
        });
        if let Some(intent) = &intent {
            fs::write(&intent_path, serde_json::to_vec(intent).unwrap()).unwrap();
        }
        Self {
            state: Mutex::new(GatewaySnapshotFixture {
                catalog,
                intent,
                observed_owned_config: false,
            }),
            catalog_path,
            intent_path,
            preflight_count: Mutex::new(0),
            verification_count: Mutex::new(0),
        }
    }
    fn restore(&self, target: TransitionTarget) -> Result<(), PrimitiveError> {
        let intent = RouterIntentFixture {
            target,
            operation_id: "fixture-restored".into(),
        };
        fs::write(&self.intent_path, serde_json::to_vec(&intent).unwrap())
            .map_err(|_| err("intent write"))?;
        self.state.lock().unwrap().intent = Some(intent);
        Ok(())
    }
}
impl HostState for GatewayHostFixture {
    type Error = PrimitiveError;
    type Snapshot = GatewaySnapshotFixture;
    fn snapshot(&self) -> Self::Snapshot {
        self.state.lock().unwrap().clone()
    }
    fn initialization_error(&self) -> Option<&str> {
        None
    }
    fn checked_recovery(&self, base: &RepairPaths) -> Result<(), ManagerError<Self::Error>> {
        self.read_journal(base)?;
        let document: GatewayCatalogFixture =
            serde_json::from_slice(&fs::read(&self.catalog_path).map_err(ManagerError::Io)?)
                .map_err(|_| ManagerError::Primitive(err("catalog decode")))?;
        if document.provider != self.snapshot().catalog.provider {
            return Err(ManagerError::Primitive(err("catalog changed")));
        }
        Ok(())
    }
    fn preflight(
        &self,
        _: &RepairPaths,
        source: &Self::Snapshot,
    ) -> Result<(), ManagerError<Self::Error>> {
        *self.preflight_count.lock().unwrap() += 1;
        let probe = self.intent_path.with_extension("preflight");
        let bytes = serde_json::to_vec(&source.catalog)
            .map_err(|_| ManagerError::Primitive(err("catalog encode")))?;
        atelier_maintenance_core::catalog::publication::write_atomic_private_report(&probe, &bytes)
            .map_err(ManagerError::Io)?
            .acknowledge();
        fs::remove_file(probe).map_err(ManagerError::Io)
    }
    fn read_journal(
        &self,
        p: &RepairPaths,
    ) -> Result<
        Option<atelier_maintenance_core::transition_journal::TransitionJournal>,
        ManagerError<Self::Error>,
    > {
        JournalStore::new(&p.verification.relay_root)
            .read_checked()
            .map_err(ManagerError::Io)
    }
    fn verify_target(
        &self,
        p: &RepairPaths,
        target: TransitionTarget,
        _: atelier_maintenance_core::router_verification::Depth,
    ) -> VerificationReport {
        *self.verification_count.lock().unwrap() += 1;
        let text = fs::read_to_string(&p.verification.config_path).unwrap_or_default();
        let snapshot = self.snapshot();
        let owned = text.contains(&format!("model_provider='{}'", snapshot.catalog.provider));
        let url = text.contains(&snapshot.catalog.base_url);
        let satisfied = match target {
            TransitionTarget::RouterOn => owned && url,
            TransitionTarget::NativeOff => !owned,
        };
        VerificationReport {
            issues: if satisfied {
                vec![]
            } else {
                vec![VerificationIssue {
                    dim: InvariantDim::Config,
                    message: "fixture actual gateway config contract mismatch".into(),
                }]
            },
        }
    }
}
#[derive(Default)]
struct GatewayOpsFixture {
    calls: Mutex<Vec<String>>,
}
impl HostRepairOperations<GatewayHostFixture> for GatewayOpsFixture {
    type PreparedCatalog = GatewayCatalogFixture;
    type Reconciled = TransitionTarget;
    fn resolve_runtime_paths(
        &self,
        p: &RepairPaths,
        _: &GatewaySnapshotFixture,
    ) -> Result<RepairPaths, PrimitiveError> {
        Ok(p.clone())
    }
    fn diagnose(
        &self,
        _: &RepairPaths,
        _: &GatewaySnapshotFixture,
        _: DiagnosticMode,
    ) -> Result<DiagnosticReport, PrimitiveError> {
        Ok(DiagnosticReport {
            items: vec![issue("config_stale")],
            has_issues: true,
            transition_in_flight: false,
        })
    }
    fn prepare_catalog(
        &self,
        _: &RepairPaths,
        s: &GatewaySnapshotFixture,
    ) -> Result<Self::PreparedCatalog, PrimitiveError> {
        Ok(s.catalog.clone())
    }
    fn ensure_router_on_ready(&self, s: &GatewaySnapshotFixture) -> Result<(), PrimitiveError> {
        self.calls.lock().unwrap().push("ready".into());
        url::Url::parse(&s.catalog.base_url).map_err(|_| err("gateway URL"))?;
        Ok(())
    }
    fn fix(
        &self,
        p: &RepairPaths,
        s: &GatewaySnapshotFixture,
        id: &str,
        _: Option<&Self::PreparedCatalog>,
        _: &GatewayHostFixture,
    ) -> Result<FixResult, PrimitiveError> {
        self.calls
            .lock()
            .unwrap()
            .push(format!("fix:{id}:{:?}", s.target()));
        let text = match s.target() {
            TransitionTarget::RouterOn => format!(
                "model_provider='{}'\n[model_providers.{}]\nbase_url='{}'\nwire_api='responses'\n",
                s.catalog.provider, s.catalog.provider, s.catalog.base_url
            ),
            TransitionTarget::NativeOff => String::new(),
        };
        fs::write(&p.verification.config_path, text).map_err(|_| err("fixture config write"))?;
        Ok(result(id))
    }
    fn fix_postcondition(
        &self,
        p: &RepairPaths,
        h: &GatewayHostFixture,
    ) -> Result<FixResult, PrimitiveError> {
        self.fix(p, &h.snapshot(), POSTCONDITION, None, h)
    }
    fn config_stale_reason(
        &self,
        p: &RepairPaths,
        s: &GatewaySnapshotFixture,
    ) -> Result<Option<String>, PrimitiveError> {
        let text = fs::read_to_string(&p.verification.config_path).unwrap_or_default();
        Ok(
            (s.target() == TransitionTarget::RouterOn && !text.contains(&s.catalog.base_url))
                .then(|| "actual fixture URL is missing".into()),
        )
    }
    fn reconcile_target<R: Runner + Send + Sync + 'static>(
        &self,
        p: &RepairPaths,
        s: &GatewaySnapshotFixture,
        _: RepairAttempt,
        _: Processes<'_, R>,
        h: &GatewayHostFixture,
    ) -> Result<ReconcileResult<Self::Reconciled, GatewaySnapshotFixture>, PrimitiveError> {
        self.fix(p, s, POSTCONDITION, None, h)?;
        Ok(ReconcileResult {
            value: s.target(),
            details: vec![],
            warnings: vec![],
            state_after: s.clone(),
        })
    }
    fn run_quiet_transition<R: Runner + Send + Sync + 'static>(
        &self,
        r: QuietRepairRequest<'_, Self::PreparedCatalog, GatewaySnapshotFixture>,
        _: Processes<'_, R>,
        h: &GatewayHostFixture,
        repair: &mut dyn FnMut(
            RepairAttempt,
        ) -> Result<
            CompletedRepair<Self::Reconciled>,
            ManagerError<PrimitiveError>,
        >,
    ) -> TransitionOutcome<Self::Reconciled, ManagerError<PrimitiveError>> {
        self.calls
            .lock()
            .unwrap()
            .push(format!("quiet:{:?}", r.source.target()));
        let value = repair(RepairAttempt {
            number: 1,
            initially_quiet: true,
        })
        .unwrap();
        let report = h.verify_target(
            r.paths,
            r.source.target(),
            atelier_maintenance_core::router_verification::Depth::Full,
        );
        TransitionOutcome::from_verification(value, report, vec![], Some(r.source.target()))
    }
    fn restore_router_on(
        &self,
        _: &RepairPaths,
        _: &GatewaySnapshotFixture,
        _: &str,
        h: &GatewayHostFixture,
    ) -> Result<(), PrimitiveError> {
        h.restore(TransitionTarget::RouterOn)
    }
    fn rollback_native_off(
        &self,
        _: &RepairPaths,
        _: &GatewaySnapshotFixture,
        _: &str,
        h: &GatewayHostFixture,
    ) -> Result<Option<Vec<String>>, PrimitiveError> {
        h.restore(TransitionTarget::NativeOff)?;
        Ok(None)
    }
}
#[test]
fn gateway_host_running_catalog_does_not_imply_desktop_takeover_or_create_native_state() {
    let t = TempDir::new().unwrap();
    let host = GatewayHostFixture::new(&t, false);
    let ops = GatewayOpsFixture::default();
    let paths = RepairPaths::from_codex_home(t.path(), vec![]);
    let context = ProcessContext::new(t.path().into(), None);
    let runner = Arc::new(FakeRunner::default());
    let e = HostManager::new(&host, &ops, runner, &context, &paths)
        .with_locks(Arc::new(Mutex::new(())), Arc::new(Mutex::new(())))
        .repair("config_stale");
    assert_eq!(host.snapshot().target(), TransitionTarget::NativeOff);
    assert!(e.result.is_ok());
    assert_eq!(*host.preflight_count.lock().unwrap(), 1);
    assert!(ops
        .calls
        .lock()
        .unwrap()
        .contains(&"quiet:NativeOff".into()));
    assert!(!ops.calls.lock().unwrap().contains(&"ready".into()));
    assert!(!paths.state_path.exists());
    assert!(!t.path().join("accounts").exists());
    assert!(!host.intent_path.exists());
}
#[test]
fn gateway_host_uses_actual_intent_and_owned_contract_with_same_native_control_flow() {
    let t = TempDir::new().unwrap();
    let host = GatewayHostFixture::new(&t, true);
    let ops = GatewayOpsFixture::default();
    let paths = RepairPaths::from_codex_home(t.path(), vec![]);
    let context = ProcessContext::new(t.path().into(), None);
    let e = HostManager::new(
        &host,
        &ops,
        Arc::new(FakeRunner::default()),
        &context,
        &paths,
    )
    .with_locks(Arc::new(Mutex::new(())), Arc::new(Mutex::new(())))
    .repair("config_stale");
    assert!(e.result.is_ok());
    assert_eq!(host.snapshot().target(), TransitionTarget::RouterOn);
    assert!(ops.calls.lock().unwrap().contains(&"ready".into()));
    let text = fs::read_to_string(&paths.verification.config_path).unwrap();
    assert!(text.contains("fixture-gateway"));
    assert!(text.contains("http://127.0.0.1:45678/v1"));
    assert!(*host.verification_count.lock().unwrap() >= 2);
    assert!(!paths.state_path.exists());
    assert!(!t.path().join("accounts").exists());
    let intent: RouterIntentFixture =
        serde_json::from_slice(&fs::read(host.intent_path).unwrap()).unwrap();
    assert_eq!(intent.operation_id, "fixture-original");
}
#[test]
fn fallback_entry_attempts_defaults_to_zero_in_map_and_sequence() {
    let t = TempDir::new().unwrap();
    let relay = write_fallback(&t, vec![]);
    let path = relay.join("router_model_restore.json");
    for text in [
        r#"{"version":1,"entries":[{"thread_id":"t","original_model":"aimami_relay_","fallback_model":"bare"}]}"#,
        r#"[1,[["t","aimami_relay_","bare"]]]"#,
        r#"{"version":1,"entries":[{"thread_id":"t","original_model":"aimami_relay_","fallback_model":"bare","attempts":4294967295}]}"#,
    ] {
        fs::write(&path, text).unwrap();
        assert!(
            validate_fallback_model_restore_journal(&relay).is_ok(),
            "{text}"
        );
    }
}
#[test]
fn fallback_schema_u32_unknown_duplicate_and_required_fields_match_native_derive() {
    let t = TempDir::new().unwrap();
    let relay = write_fallback(&t, vec![]);
    let path = relay.join("router_model_restore.json");
    for text in [
        r#"{"version":-1,"entries":[]}"#,
        r#"{"version":1}"#,
        r#"{"version":1,"entries":[],"future":true}"#,
        r#"{"version":1,"entries":[{"thread_id":"t","original_model":"aimami_relay_","fallback_model":"bare","attempts":-1}]}"#,
        r#"{"version":1,"entries":[{"thread_id":"t","original_model":"aimami_relay_","fallback_model":"bare","attempts":4294967296}]}"#,
        r#"{"version":1,"entries":[{"thread_id":"t","original_model":"aimami_relay_","fallback_model":"bare","future":true}]}"#,
        r#"{"version":1,"entries":[{"thread_id":"t","original_model":"aimami_relay_","fallback_model":"bare","attempts":0,"attempts":0}]}"#,
    ] {
        fs::write(&path, text).unwrap();
        assert_eq!(
            validate_fallback_model_restore_journal(&relay)
                .unwrap_err()
                .to_string(),
            "中转原模型恢复记录格式异常或版本不兼容，本次操作未能完成。",
            "{text}"
        );
    }
}
#[test]
fn no_account_guard_preserves_explicit_nullable_original_and_rejects_unknown_schema() {
    let t = TempDir::new().unwrap();
    let relay = write_fallback(&t, vec![]);
    let path = relay.join("router_no_account_model_restore.json");
    for original in [serde_json::Value::Null, serde_json::json!("original-model")] {
        let text = serde_json::json!({"version":2,"entries":[{"thread_id":"t","original_model":original,"borrowed_model":"gpt-5.6-sol"}]});
        let bytes = serde_json::to_vec(&text).unwrap();
        fs::write(&path, &bytes).unwrap();
        validate_no_account_model_restore_journal(&relay).unwrap();
        assert_eq!(fs::read(&path).unwrap(), bytes);
    }
    for text in [
        r#"{"version":2,"entries":[],"future":true}"#,
        r#"{"version":2,"entries":[{"thread_id":"t","original_model":null,"borrowed_model":"gpt-5.6-sol","future":true}]}"#,
        r#"{"version":2,"entries":[{"thread_id":"t","borrowed_model":"gpt-5.6-sol"}]}"#,
    ] {
        fs::write(&path, text).unwrap();
        assert!(validate_no_account_model_restore_journal(&relay)
            .unwrap_err()
            .to_string()
            .contains("decode no-account model restore journal"));
        assert_eq!(fs::read_to_string(&path).unwrap(), text);
    }
}
#[test]
fn no_account_guard_validates_version_borrowed_models_and_raw_identity() {
    let t = TempDir::new().unwrap();
    let relay = write_fallback(&t, vec![]);
    let path = relay.join("router_no_account_model_restore.json");
    let entry = |id: &str, borrowed: &str| serde_json::json!({"thread_id":id,"original_model":null,"borrowed_model":borrowed});
    for (version, entries, valid) in [
        (1, vec![entry("t", "gpt-5.6-sol")], true),
        (1, vec![entry("t", "aimami_relay_p::model")], false),
        (2, vec![entry("t", "aimami_relay_p::model")], true),
        (2, vec![entry("t", "aimami_relay_p:: ")], false),
        (
            2,
            vec![entry("t", "gpt-5.6-sol"), entry(" t ", "gpt-5.6-sol")],
            true,
        ),
        (
            2,
            vec![entry("t", "gpt-5.6-sol"), entry("t", "gpt-5.6-sol")],
            false,
        ),
        (3, vec![], false),
    ] {
        fs::write(
            &path,
            serde_json::to_vec(&serde_json::json!({"version":version,"entries":entries})).unwrap(),
        )
        .unwrap();
        assert_eq!(
            validate_no_account_model_restore_journal(&relay).is_ok(),
            valid,
            "version {version}"
        );
    }
}
