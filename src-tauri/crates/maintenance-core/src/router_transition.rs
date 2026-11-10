//! Native router transition engine, run_with_environment @0x100326b7c.
//! All mutation inputs are explicit. The callback performs the real repair/reconcile;
//! this engine owns process fencing, retry, on-disk verification and durable commit.
use crate::manager::{HostState, RepairSnapshot};
use crate::{
    catalog::{publication::Publication, state::RouterOperationId},
    manager::{
        self, CompletedRepair, FailureStage, ManagerError, Processes, QuietRepairRequest,
        RepairAttempt, StateHandle, TransitionFailure, TransitionOutcome,
    },
    process::{CodexTransitionFence, FenceMode, Runner},
    router_verification::{self, Depth, ProxyHandle, VerificationContext, VerificationReport},
    transition_journal::{JournalStore, TransitionTarget, RECOVERY_REQUIRED},
};
use std::{
    fs::File,
    io::{self, Read},
    sync::Mutex,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransitionPolicy {
    Strict,
    NativeOffBestEffort,
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or_else(|e| -(e.duration().as_millis() as i64))
}
fn operation_id() -> io::Result<RouterOperationId> {
    let mut b = [0u8; 16];
    File::open("/dev/urandom")?.read_exact(&mut b)?;
    b[6] = (b[6] & 15) | 64;
    b[8] = (b[8] & 63) | 128;
    let s = b
        .iter()
        .enumerate()
        .map(|(i, v)| {
            format!(
                "{}{v:02x}",
                if matches!(i, 4 | 6 | 8 | 10) { "-" } else { "" }
            )
        })
        .collect::<String>();
    RouterOperationId::try_from(s).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
fn failed<T, E>(
    error: ManagerError<E>,
    stage: FailureStage,
    attempt: u8,
) -> TransitionOutcome<T, ManagerError<E>> {
    TransitionOutcome::Failed(TransitionFailure {
        error,
        stage,
        attempt,
    })
}
fn describe(report: &VerificationReport) -> String {
    report
        .issues
        .iter()
        .map(|i| format!("{:?}: {}", i.dim, i.message))
        .collect::<Vec<_>>()
        .join("; ")
}
/// Native repair entry uses Strict policy; best-effort is only the separate NativeOff
/// transition policy. No caller-provided valid/verified flag participates in this path.
/// `commit` must return the actual state publication (including its sync result).
/// The Manager persistence helpers supply the concrete implementation.
pub fn run_repair<R, T, E, P>(
    request: QuietRepairRequest<'_, P>,
    processes: Processes<'_, R>,
    shared: &StateHandle,
    proxy: &Mutex<Option<ProxyHandle>>,
    repair: &mut dyn FnMut(RepairAttempt) -> Result<CompletedRepair<T>, ManagerError<E>>,
    commit: &mut dyn FnMut(
        &CompletedRepair<T>,
        &RouterOperationId,
    ) -> Result<Publication, ManagerError<E>>,
) -> TransitionOutcome<T, ManagerError<E>>
where
    R: Runner + Send + Sync + 'static,
    E: std::fmt::Display,
{
    run_with_environment(
        request,
        processes,
        shared,
        proxy,
        TransitionPolicy::Strict,
        repair,
        commit,
    )
}

/// At most three attempts. Retry is permitted only after a fence observed respawn or
/// taint, and only before the twelve-second deadline first established by that event.
pub fn run_with_environment<R, T, E, P>(
    request: QuietRepairRequest<'_, P>,
    processes: Processes<'_, R>,
    shared: &StateHandle,
    proxy: &Mutex<Option<ProxyHandle>>,
    policy: TransitionPolicy,
    repair: &mut dyn FnMut(RepairAttempt) -> Result<CompletedRepair<T>, ManagerError<E>>,
    commit: &mut dyn FnMut(
        &CompletedRepair<T>,
        &RouterOperationId,
    ) -> Result<Publication, ManagerError<E>>,
) -> TransitionOutcome<T, ManagerError<E>>
where
    R: Runner + Send + Sync + 'static,
    E: std::fmt::Display,
{
    let paths = request.paths;
    run_engine(
        request,
        processes,
        policy,
        &|| {
            manager::ensure_recovery_records_allowed(&paths.verification.relay_root)
                .map_err(ManagerError::Io)
        },
        &|include_missing| {
            crate::router_capacity::ensure_transition_rollout_write_capacity(
                &paths.verification,
                include_missing,
            )
            .map_err(ManagerError::Io)
        },
        &|target, depth| {
            let snapshot = shared.snapshot();
            router_verification::verify_target_state_at_with_depth(
                &VerificationContext {
                    paths: &paths.verification,
                    state: snapshot.document(),
                    proxy,
                },
                target,
                depth,
            )
        },
        repair,
        commit,
    )
}
/// Same concrete state machine with a real storage host's checked recovery and
/// verification strategy. Gateway hosts retain their Catalog/RouterIntent snapshot;
/// this entry never constructs a native RelayState for a different storage source.
pub fn run_host_repair<R, T, P, H: HostState>(
    request: QuietRepairRequest<'_, P, H::Snapshot>,
    processes: Processes<'_, R>,
    host: &H,
    repair: &mut dyn FnMut(RepairAttempt) -> Result<CompletedRepair<T>, ManagerError<H::Error>>,
    commit: &mut dyn FnMut(
        &CompletedRepair<T>,
        &RouterOperationId,
    ) -> Result<Publication, ManagerError<H::Error>>,
) -> TransitionOutcome<T, ManagerError<H::Error>>
where
    R: Runner + Send + Sync + 'static,
{
    let paths = request.paths;
    run_engine(
        request,
        processes,
        TransitionPolicy::Strict,
        &|| host.checked_recovery(paths),
        &|include_missing| host.check_write_capacity(paths, include_missing),
        &|target, depth| host.verify_target(paths, target, depth),
        repair,
        commit,
    )
}

fn run_engine<R, T, E, P, S: RepairSnapshot>(
    request: QuietRepairRequest<'_, P, S>,
    processes: Processes<'_, R>,
    policy: TransitionPolicy,
    checked_recovery: &dyn Fn() -> Result<(), ManagerError<E>>,
    check_capacity: &dyn Fn(bool) -> Result<(), ManagerError<E>>,
    verify: &dyn Fn(TransitionTarget, Depth) -> VerificationReport,
    repair: &mut dyn FnMut(RepairAttempt) -> Result<CompletedRepair<T>, ManagerError<E>>,
    commit: &mut dyn FnMut(
        &CompletedRepair<T>,
        &RouterOperationId,
    ) -> Result<Publication, ManagerError<E>>,
) -> TransitionOutcome<T, ManagerError<E>>
where
    R: Runner + Send + Sync + 'static,
    E: std::fmt::Display,
{
    let root = &request.paths.verification.relay_root;
    let journal = JournalStore::new(root);
    if let Err(e) = checked_recovery() {
        return failed(e, FailureStage::Preflight, 0);
    }
    if policy == TransitionPolicy::NativeOffBestEffort
        && request.source.target() != TransitionTarget::NativeOff
    {
        return failed(
            ManagerError::Operation("best-effort policy requires NativeOff target".into()),
            FailureStage::Preflight,
            0,
        );
    }
    if policy == TransitionPolicy::Strict {
        if let Err(e) = check_capacity(request.include_missing_rollouts) {
            return failed(e, FailureStage::Preflight, 0);
        }
    }
    let id = match operation_id() {
        Ok(v) => v,
        Err(e) => return failed(ManagerError::Io(e), FailureStage::Preflight, 0),
    };
    let target = request.source.target();
    let record = |phase: &str, reason: Option<&str>| {
        // Native write_journal is best effort; begin_commit below is mandatory.
        let _ = journal.write_with_source(target, phase, Some(request.reason), reason, now_ms());
    };
    let mut retry_deadline = None;
    for number in 1..=3 {
        record("stopping_codex", Some(request.reason));
        let fence = match CodexTransitionFence::acquire(
            processes.runner.clone(),
            processes.context.clone(),
            request.graceful_timeout,
            if policy == TransitionPolicy::Strict {
                FenceMode::Strict
            } else {
                FenceMode::BestEffort
            },
            request.writer_policy,
        ) {
            Ok(f) => f,
            Err(e) => {
                record("failed", Some(&e.to_string()));
                return failed(ManagerError::Process(e), FailureStage::Stop, number);
            }
        };
        let initially_quiet = fence.initially_quiet();
        record("reconciling", Some(request.reason));
        let result = repair(RepairAttempt {
            number,
            initially_quiet,
        });
        // finish is mandatory even after callback failure; it joins the worker and
        // performs the concrete final sweep before any verification or persistence.
        let settled = fence.finish();
        let settled = match settled {
            Ok(s) => s,
            Err(e) => {
                record("failed", Some(&e.to_string()));
                let (error, stage) = match result {
                    Ok(_) => (ManagerError::Process(e), FailureStage::Settle),
                    Err(primary) => (
                        ManagerError::Restoration {
                            primary: Box::new(primary),
                            restoration: Box::new(ManagerError::Process(e)),
                        },
                        FailureStage::Reconcile,
                    ),
                };
                return failed(error, stage, number);
            }
        };
        let unstable = settled.respawns != 0 || settled.tainted;
        let can_retry = |deadline: &mut Option<Duration>| {
            if !unstable || number >= 3 {
                return false;
            }
            let now = processes.runner.now();
            let limit =
                *deadline.get_or_insert_with(|| now.saturating_add(Duration::from_secs(12)));
            now < limit
        };
        let value = match result {
            Ok(v) => v,
            Err(e) => {
                if policy == TransitionPolicy::Strict && can_retry(&mut retry_deadline) {
                    record("retrying", Some("writer respawn during reconciliation"));
                    continue;
                }
                record("failed", Some(&e.to_string()));
                return failed(e, FailureStage::Reconcile, number);
            }
        };
        let report = verify(
            target,
            if unstable {
                Depth::Full
            } else {
                Depth::SkipProviderConvergence
            },
        );
        if !report.issues.is_empty() && policy == TransitionPolicy::Strict {
            if can_retry(&mut retry_deadline) {
                record("retrying", Some(&describe(&report)));
                continue;
            }
            let error = format!("postcondition still failed: {}", describe(&report));
            record("failed", Some(&error));
            return failed(ManagerError::Operation(error), FailureStage::Verify, number);
        }
        let reconciled = value.reconciled().map(|_| target);
        let mut warnings = value.warnings().to_vec();
        if !report.issues.is_empty() {
            warnings.push(format!(
                "NativeOff target remains unverified: {}",
                describe(&report)
            ));
        }
        if let Err(e) = journal.begin_commit_with_source(
            target,
            Some(request.reason),
            Some(request.reason),
            now_ms(),
        ) {
            // A begin_commit sync failure may already have published `committing`.
            // Never overwrite it with `failed` or clear it on this path.
            record("commit_failed", Some(&e.to_string()));
            return failed(ManagerError::Io(e), FailureStage::Commit, number);
        }
        let publication = match commit(&value, &id) {
            Ok(p) => p,
            Err(e) => {
                // write_journal refuses to replace a pending commit, but records the event.
                record("commit_failed", Some(&e.to_string()));
                return failed(e, FailureStage::Commit, number);
            }
        };
        if let Err(e) = publication.require_sync() {
            let reason =
                format!("{RECOVERY_REQUIRED}: state published; directory sync unconfirmed: {e}");
            let _ = crate::transition_events::append_event_at(
                root,
                target,
                "commit_sync_unconfirmed",
                Some(request.reason),
                Some(&reason),
                now_ms(),
            );
            return TransitionOutcome::Unverified {
                value,
                report,
                warnings,
                reconciled,
                recovery_reasons: vec![reason],
            };
        }
        if warnings.is_empty() {
            if let Err(e) = journal.clear(Some(target)) {
                // State is already durable: native preserves success with this cleanup warning.
                warnings.push(format!(
                    "committed, but transition journal cleanup failed: {e}"
                ));
            }
        } else {
            // Native warn_after_commit transitions out of committing but leaves the
            // degraded record. Its write failure is logged, not invented success.
            let detail = warnings.join(" | ");
            let warning_update = (|| -> io::Result<()> {
                if journal.read_checked()?.is_some_and(|j| j.target != target) {
                    return Err(io::Error::other(RECOVERY_REQUIRED));
                }
                journal
                    .recover_write(target, "degraded", Some(&detail), now_ms())?
                    .acknowledge();
                Ok(())
            })();
            if let Err(e) = warning_update {
                warnings.push(format!("committed, but warning journal update failed: {e}"));
            }
        }
        return TransitionOutcome::from_verification(value, report, warnings, reconciled);
    }
    unreachable!("three attempts always return or exhaust via final failure")
}

/// Concrete native persistence adapter. The callback's actual reconciliation target
/// selects commit_reconciled_target; an unchanged target persists the live snapshot.
/// Both paths publish a real receipt and update the shared state/cache through Manager.
/// `stale_reason` is the application's actual config inspection for the proposed state.
pub fn run_native_repair<R, E, P, C>(
    request: QuietRepairRequest<'_, P>,
    processes: Processes<'_, R>,
    persistence: manager::PersistenceContext<'_, C>,
    repair: &mut dyn FnMut(
        RepairAttempt,
    )
        -> Result<CompletedRepair<manager::ReconciledTarget>, ManagerError<E>>,
    stale_reason: &mut dyn FnMut(&manager::Snapshot) -> Result<Option<String>, E>,
) -> TransitionOutcome<manager::ReconciledTarget, ManagerError<E>>
where
    R: Runner + Send + Sync + 'static,
    E: std::fmt::Display,
{
    let source = request.source;
    run_repair(
        request,
        processes,
        persistence.state,
        persistence.proxy,
        repair,
        &mut |value, id| {
            let published = if let Some(target) = value.reconciled() {
                manager::commit_reconciled_target(persistence, target, source, id, |s| {
                    stale_reason(s)
                })
                .map_err(|e| match e {
                    manager::CommitError::Primitive(e) => ManagerError::Primitive(e),
                    manager::CommitError::Operation(s) => ManagerError::Operation(s),
                    manager::CommitError::Persistence(e) => ManagerError::Persistence(e),
                })?
            } else {
                manager::persist_with_commit(
                    persistence,
                    persistence.state.snapshot().document().clone(),
                    Some(id.clone()),
                )
                .map_err(ManagerError::Persistence)?
            };
            if let Some(e) = published.receipt_warning {
                eprintln!("router commit receipt readback after unconfirmed sync failed: {e}");
            }
            Ok(published.publication)
        },
    )
}
