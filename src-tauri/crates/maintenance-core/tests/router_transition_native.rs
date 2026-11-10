use atelier_maintenance_core::{
    catalog::{
        publication::{write_atomic_private_report, Publication},
        state::RelayStateDocument,
    },
    manager::{
        CompletedRepair, FailureStage, FixResult, ManagerError, Processes, QuietRepairRequest,
        RepairPaths, RepairTransitionPolicy, Snapshot, StateHandle, TransitionOutcome,
    },
    process::{CommandOutput, Operation, ProcessContext, ProcessPolicy, Runner},
    router_transition::{run_repair, run_with_environment, TransitionPolicy},
    transition_journal::JournalStore,
};
use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};
#[derive(Default)]
struct FakeRunner {
    time: AtomicU64,
    clock_thread: Mutex<Option<std::thread::ThreadId>>,
    writer: AtomicBool,
    sticky: AtomicBool,
    fail_ps: AtomicBool,
    calls: Mutex<Vec<Operation>>,
}
impl Runner for FakeRunner {
    fn output(&self, op: &Operation) -> io::Result<CommandOutput> {
        self.calls.lock().unwrap().push(op.clone());
        if *op == Operation::Processes && self.fail_ps.load(Ordering::SeqCst) {
            return Err(io::Error::other("synthetic ps failure"));
        }
        let stdout = match op {
            Operation::Processes if self.writer.load(Ordering::SeqCst) => {
                b"77 /synthetic/bin/codex --fixture\n".to_vec()
            }
            Operation::IsAppRunning => b"false".to_vec(),
            _ => vec![],
        };
        Ok(CommandOutput {
            success: true,
            stdout,
            stderr: vec![],
        })
    }
    fn signal_kill(&self, _: u32) -> io::Result<()> {
        if !self.sticky.load(Ordering::SeqCst) {
            self.writer.store(false, Ordering::SeqCst);
        }
        Ok(())
    }
    fn exists(&self, _: &Path) -> bool {
        false
    }
    fn is_file(&self, _: &Path) -> bool {
        false
    }
    fn read_text(&self, _: &Path) -> io::Result<String> {
        Err(io::ErrorKind::NotFound.into())
    }
    fn read_dir(&self, _: &Path) -> io::Result<Vec<PathBuf>> {
        Ok(vec![])
    }
    fn process_id(&self) -> u32 {
        9999
    }
    fn now(&self) -> Duration {
        Duration::from_millis(self.time.load(Ordering::SeqCst))
    }
    fn sleep(&self, d: Duration) {
        if self.clock_thread.lock().unwrap().as_ref() == Some(&std::thread::current().id()) {
            self.time.fetch_add(d.as_millis() as u64, Ordering::SeqCst);
        }
        std::thread::sleep(Duration::from_micros(100));
    }
}
struct Fixture {
    _tmp: tempfile::TempDir,
    paths: RepairPaths,
    shared: StateHandle,
    source: Snapshot,
    ctx: ProcessContext,
    runner: Arc<FakeRunner>,
}
impl Fixture {
    fn new() -> Self {
        let tmp = tempfile::tempdir().unwrap();
        let home = tmp.path().join("codex");
        let paths = RepairPaths::from_codex_home(&home, vec![]);
        fs::create_dir_all(&paths.verification.relay_root).unwrap();
        let document = RelayStateDocument::native_default();
        let source = Snapshot::new(document.clone()).unwrap();
        let shared = StateHandle::new(document);
        let ctx = ProcessContext {
            user_home: None,
            codex_home: home,
            sqlite_home_env: None,
            cwd: None,
        };
        Self {
            _tmp: tmp,
            paths,
            shared,
            source,
            ctx,
            runner: Arc::new(FakeRunner {
                clock_thread: Mutex::new(Some(std::thread::current().id())),
                ..Default::default()
            }),
        }
    }
    fn request(&self) -> QuietRepairRequest<'_, ()> {
        QuietRepairRequest {
            paths: &self.paths,
            source: &self.source,
            original: &self.source,
            selected: "all",
            prepared_catalog: None,
            include_missing_rollouts: false,
            reason: "synthetic repair",
            policy: RepairTransitionPolicy::Strict,
            writer_policy: ProcessPolicy::ForceCloseAll,
            graceful_timeout: Duration::from_secs(5),
        }
    }
    fn processes(&self) -> Processes<'_, FakeRunner> {
        Processes {
            runner: self.runner.clone(),
            context: &self.ctx,
        }
    }
    fn publish(
        &self,
        id: &atelier_maintenance_core::catalog::state::RouterOperationId,
    ) -> io::Result<Publication> {
        assert!(JournalStore::new(&self.paths.verification.relay_root).commit_pending()?);
        let mut doc = self.shared.snapshot().document().clone();
        doc.set_router_commit_id(Some(id.clone()));
        write_atomic_private_report(&self.paths.state_path, &doc.to_pretty_json().unwrap())
    }
}
fn completed() -> CompletedRepair<()> {
    CompletedRepair::from_parts(
        FixResult {
            fixed_ids: vec!["fixture".into()],
            failed_ids: vec![],
            details: vec!["actual callback result".into()],
        },
        None,
        vec![],
    )
}
#[test]
fn actual_fence_verify_and_commit_are_ordered_and_journal_is_cleared() {
    let f = Fixture::new();
    let proxy = Mutex::new(None);
    let mut calls = vec![];
    let result = run_repair(
        f.request(),
        f.processes(),
        &f.shared,
        &proxy,
        &mut |a| {
            calls.push(a);
            assert_eq!(
                JournalStore::new(&f.paths.verification.relay_root)
                    .read_checked()
                    .unwrap()
                    .unwrap()
                    .phase,
                "reconciling"
            );
            Ok::<_, ManagerError<String>>(completed())
        },
        &mut |_, id| f.publish(id).map_err(ManagerError::Io),
    );
    assert!(matches!(result, TransitionOutcome::Verified { .. }));
    assert_eq!(calls.len(), 1);
    assert!(calls[0].initially_quiet);
    assert_eq!(calls[0].number, 1);
    assert!(JournalStore::new(&f.paths.verification.relay_root)
        .read_checked()
        .unwrap()
        .is_none());
    let disk = RelayStateDocument::from_json(&fs::read(&f.paths.state_path).unwrap()).unwrap();
    assert!(disk.router_commit_id().unwrap().is_some());
    assert!(!f
        .runner
        .calls
        .lock()
        .unwrap()
        .iter()
        .any(|o| matches!(o, Operation::OpenBundle(_))));
}
#[test]
fn quiet_callback_error_is_not_retried_and_carries_reconcile_stage() {
    let f = Fixture::new();
    let mut calls = 0;
    let mut commits = 0;
    let result: TransitionOutcome<(), ManagerError<String>> = run_repair(
        f.request(),
        f.processes(),
        &f.shared,
        &Mutex::new(None),
        &mut |_| {
            calls += 1;
            Err(ManagerError::Primitive("repair failure".into()))
        },
        &mut |_, _| {
            commits += 1;
            Err(ManagerError::Io(io::Error::other("must not commit")))
        },
    );
    assert_eq!((calls, commits), (1, 0));
    match result {
        TransitionOutcome::Failed(e) => {
            assert_eq!(e.stage, FailureStage::Reconcile);
            assert_eq!(e.attempt, 1);
            assert!(e.needs_compensation())
        }
        _ => panic!("expected failure"),
    }
}
#[test]
fn respawn_allows_retry_but_never_more_than_three_attempts() {
    let f = Fixture::new();
    let mut attempts = vec![];
    let result: TransitionOutcome<(), ManagerError<String>> = run_repair(
        f.request(),
        f.processes(),
        &f.shared,
        &Mutex::new(None),
        &mut |a| {
            attempts.push(a.number);
            f.runner.writer.store(true, Ordering::SeqCst);
            Err(ManagerError::Primitive(
                "synthetic reconcile failure".into(),
            ))
        },
        &mut |_, _| Err(ManagerError::Io(io::Error::other("must not commit"))),
    );
    assert_eq!(attempts, vec![1, 2, 3]);
    match result {
        TransitionOutcome::Failed(e) => {
            assert_eq!(e.stage, FailureStage::Reconcile);
            assert_eq!(e.attempt, 3)
        }
        _ => panic!("expected failure"),
    }
}
#[test]
fn actual_config_violation_prevents_commit_without_a_respawn() {
    let f = Fixture::new();
    fs::write(
        &f.paths.verification.config_path,
        "model_provider='aimai1'\n",
    )
    .unwrap();
    let mut commits = 0;
    let result = run_repair(
        f.request(),
        f.processes(),
        &f.shared,
        &Mutex::new(None),
        &mut |_| Ok::<_, ManagerError<String>>(completed()),
        &mut |_, _| {
            commits += 1;
            Err(ManagerError::Io(io::Error::other("must not commit")))
        },
    );
    assert_eq!(commits, 0);
    match result {
        TransitionOutcome::Failed(e) => assert_eq!(e.stage, FailureStage::Verify),
        _ => panic!("expected verify failure"),
    }
}
#[test]
fn published_state_sync_failure_keeps_committing_and_is_unverified() {
    let f = Fixture::new();
    let result = run_repair(
        f.request(),
        f.processes(),
        &f.shared,
        &Mutex::new(None),
        &mut |_| Ok::<_, ManagerError<String>>(completed()),
        &mut |_, id| {
            let mut p = f.publish(id).map_err(ManagerError::Io)?;
            p.directory_sync_error = Some(io::Error::other("synthetic directory fsync failure"));
            Ok(p)
        },
    );
    match result {
        TransitionOutcome::Unverified {
            recovery_reasons,
            warnings,
            ..
        } => {
            assert!(warnings.is_empty());
            assert!(recovery_reasons
                .iter()
                .any(|s| s.contains("sync unconfirmed")))
        }
        _ => panic!("sync failure must remain unverified"),
    }
    assert!(f.paths.state_path.is_file());
    assert!(JournalStore::new(&f.paths.verification.relay_root)
        .commit_pending()
        .unwrap());
}
#[test]
fn malformed_recovery_record_fails_before_process_calls() {
    let f = Fixture::new();
    fs::write(
        JournalStore::new(&f.paths.verification.relay_root).path(),
        b"invalid",
    )
    .unwrap();
    let mut called = false;
    let result = run_repair(
        f.request(),
        f.processes(),
        &f.shared,
        &Mutex::new(None),
        &mut |_| {
            called = true;
            Ok::<_, ManagerError<String>>(completed())
        },
        &mut |_, id| f.publish(id).map_err(ManagerError::Io),
    );
    assert!(!called);
    assert!(f.runner.calls.lock().unwrap().is_empty());
    match result {
        TransitionOutcome::Failed(e) => {
            assert_eq!(e.stage, FailureStage::Preflight);
            assert_eq!(e.attempt, 0)
        }
        _ => panic!("expected preflight failure"),
    }
}
#[test]
fn best_effort_native_off_commits_actual_unverified_state_as_degraded() {
    let f = Fixture::new();
    fs::write(
        &f.paths.verification.config_path,
        "model_provider='aimai1'\n",
    )
    .unwrap();
    let result = run_with_environment(
        f.request(),
        f.processes(),
        &f.shared,
        &Mutex::new(None),
        TransitionPolicy::NativeOffBestEffort,
        &mut |_| Ok::<_, ManagerError<String>>(completed()),
        &mut |_, id| f.publish(id).map_err(ManagerError::Io),
    );
    assert!(matches!(result, TransitionOutcome::Unverified { .. }));
    assert_eq!(
        JournalStore::new(&f.paths.verification.relay_root)
            .read_checked()
            .unwrap()
            .unwrap()
            .phase,
        "degraded"
    );
}
#[test]
fn stop_failure_does_not_invoke_callback_or_commit() {
    let f = Fixture::new();
    f.runner.fail_ps.store(true, Ordering::SeqCst);
    let mut calls = 0;
    let result = run_repair(
        f.request(),
        f.processes(),
        &f.shared,
        &Mutex::new(None),
        &mut |_| {
            calls += 1;
            Ok::<_, ManagerError<String>>(completed())
        },
        &mut |_, id| f.publish(id).map_err(ManagerError::Io),
    );
    assert_eq!(calls, 0);
    match result {
        TransitionOutcome::Failed(e) => {
            assert_eq!(e.stage, FailureStage::Stop);
            assert_eq!(e.attempt, 1);
            assert!(!e.needs_compensation())
        }
        _ => panic!("expected stop failure"),
    }
}
#[test]
fn settle_failure_preserves_its_stage_after_successful_callback() {
    let f = Fixture::new();
    let mut commits = 0;
    let result = run_repair(
        f.request(),
        f.processes(),
        &f.shared,
        &Mutex::new(None),
        &mut |_| {
            f.runner.sticky.store(true, Ordering::SeqCst);
            f.runner.writer.store(true, Ordering::SeqCst);
            Ok::<_, ManagerError<String>>(completed())
        },
        &mut |_, _| {
            commits += 1;
            Err(ManagerError::Io(io::Error::other("must not commit")))
        },
    );
    assert_eq!(commits, 0);
    match result {
        TransitionOutcome::Failed(e) => {
            assert_eq!(e.stage, FailureStage::Settle);
            assert_eq!(e.attempt, 1)
        }
        _ => panic!("expected settle failure"),
    }
}
#[test]
fn persistent_state_write_failure_keeps_recovery_journal() {
    let f = Fixture::new();
    let result = run_repair(
        f.request(),
        f.processes(),
        &f.shared,
        &Mutex::new(None),
        &mut |_| Ok::<_, ManagerError<String>>(completed()),
        &mut |_, _| {
            Err(ManagerError::Io(io::Error::other(
                "synthetic state publication failure",
            )))
        },
    );
    match result {
        TransitionOutcome::Failed(e) => {
            assert_eq!(e.stage, FailureStage::Commit);
            assert_eq!(e.attempt, 1)
        }
        _ => panic!("expected commit failure"),
    }
    assert!(JournalStore::new(&f.paths.verification.relay_root)
        .commit_pending()
        .unwrap());
    assert!(!f.paths.state_path.exists());
}
#[test]
fn respawn_retry_deadline_is_fixed_at_first_retry_not_reset_every_attempt() {
    let f = Fixture::new();
    let mut attempts = vec![];
    let result: TransitionOutcome<(), ManagerError<String>> = run_repair(
        f.request(),
        f.processes(),
        &f.shared,
        &Mutex::new(None),
        &mut |a| {
            attempts.push(a.number);
            if a.number == 2 {
                f.runner.time.fetch_add(13_000, Ordering::SeqCst);
            }
            f.runner.writer.store(true, Ordering::SeqCst);
            Err(ManagerError::Primitive(
                "synthetic long repair failure".into(),
            ))
        },
        &mut |_, _| Err(ManagerError::Io(io::Error::other("must not commit"))),
    );
    assert_eq!(attempts, vec![1, 2]);
    match result {
        TransitionOutcome::Failed(e) => assert_eq!(e.attempt, 2),
        _ => panic!("expected expired retry window"),
    }
}
#[test]
fn native_entry_persists_real_receipt_and_updates_shared_state() {
    use atelier_maintenance_core::{
        manager::{CatalogRuntime, PersistenceContext},
        router_transition::run_native_repair,
    };
    let f = Fixture::new();
    let proxy = Mutex::new(None);
    let cache = Mutex::new(CatalogRuntime::<()>::default());
    let context = PersistenceContext {
        paths: &f.paths,
        state: &f.shared,
        proxy: &proxy,
        catalog: &cache,
        initialization_error: None,
    };
    let result = run_native_repair(
        f.request(),
        f.processes(),
        context,
        &mut |_| {
            Ok::<_, ManagerError<String>>(CompletedRepair::from_parts(
                FixResult::default(),
                None,
                vec![],
            ))
        },
        &mut |snapshot| {
            let text = fs::read_to_string(&f.paths.verification.config_path).unwrap_or_default();
            Ok::<_, String>(
                atelier_maintenance_core::config::RouterConfigSurface::parse(
                    &text,
                    &f.paths.verification.relay_root,
                )
                .violations(
                    snapshot.target()
                        == atelier_maintenance_core::transition_journal::TransitionTarget::RouterOn,
                    &f.paths.verification.relay_root,
                )
                .into_iter()
                .next(),
            )
        },
    );
    assert!(matches!(result, TransitionOutcome::Verified { .. }));
    let state = f.shared.snapshot();
    assert!(state.document().router_commit_id().unwrap().is_some());
    atelier_maintenance_core::manager::verify_router_commit(&f.paths, state.document()).unwrap();
    assert!(!JournalStore::new(&f.paths.verification.relay_root)
        .commit_pending()
        .unwrap());
}
#[test]
fn native_entry_rejects_unapplied_write_window_and_preserves_commit_recovery() {
    use atelier_maintenance_core::{
        manager::{CatalogRuntime, NativeOffWriteWindow, PersistenceContext, ReconciledTarget},
        router_transition::run_native_repair,
    };
    let f = Fixture::new();
    let proxy = Mutex::new(None);
    let cache = Mutex::new(CatalogRuntime::<()>::default());
    let context = PersistenceContext {
        paths: &f.paths,
        state: &f.shared,
        proxy: &proxy,
        catalog: &cache,
        initialization_error: None,
    };
    let mut stale_called = false;
    let result = run_native_repair(
        f.request(),
        f.processes(),
        context,
        &mut |_| {
            Ok::<_, ManagerError<String>>(CompletedRepair::from_parts(
                FixResult::default(),
                Some(ReconciledTarget::NativeOff {
                    write_window: NativeOffWriteWindow::Unavailable,
                }),
                vec![],
            ))
        },
        &mut |_| {
            stale_called = true;
            Ok::<_, String>(None)
        },
    );
    assert!(!stale_called);
    match result {
        TransitionOutcome::Failed(e) => {
            assert_eq!(e.stage, FailureStage::Commit);
            assert!(e.error.to_string().contains("not applied"))
        }
        _ => panic!("unapplied write window must not succeed"),
    }
    assert!(!f.paths.state_path.exists());
    assert!(JournalStore::new(&f.paths.verification.relay_root)
        .commit_pending()
        .unwrap());
}
