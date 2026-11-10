#[path = "../src/process.rs"]
mod process;
use process::*;
use std::path::Path;
#[test]
fn executable_match_requires_boundary() {
    let p = Path::new("/Applications/Codex.app/Contents/MacOS/Codex");
    assert!(command_runs_executable(
        "/Applications/Codex.app/Contents/MacOS/Codex",
        p
    ));
    assert!(command_runs_executable(
        "/Applications/Codex.app/Contents/MacOS/Codex --type=renderer",
        p
    ));
    assert!(command_runs_executable(
        "/Applications/Codex.app/Contents/MacOS/Codex\u{2003}arg",
        p
    ));
    assert!(!command_runs_executable(
        "/Applications/Codex.app/Contents/MacOS/Codex-other",
        p
    ));
    assert!(!command_runs_executable(
        "sh -c /Applications/Codex.app/Contents/MacOS/Codex",
        p
    ));
}
#[test]
fn ide_extension_rule_keeps_native_substring_behavior() {
    assert!(is_openai_ide_extension_writer_path(
        "C:\\Users\\Synthetic\\.vscode\\extensions\\OpenAI.ChatGPT-1.0\\bin\\codex.exe"
    ));
    assert!(is_openai_ide_extension_writer_path(
        "/synthetic/extensions/openai.chatgpt-extra/cli"
    ));
    assert!(!is_openai_ide_extension_writer_path(
        "/synthetic/extensions/other.chatgpt/bin/codex"
    ));
    assert_eq!(extract_process_name("/synthetic/bin/codex --arg"), "codex");
}

use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    io,
    path::PathBuf,
    time::Duration,
};
#[derive(Default)]
struct Fake {
    time: Cell<Duration>,
    frames: RefCell<Vec<(Duration, String)>>,
    killed: RefCell<Vec<u32>>,
    calls: RefCell<Vec<Operation>>,
    files: BTreeMap<PathBuf, String>,
    dirs: BTreeMap<PathBuf, Vec<PathBuf>>,
    holders: BTreeMap<u32, String>,
    signal_fails: Cell<bool>,
    signal_removes: Cell<bool>,
    ps_fails: Cell<bool>,
    quit_fails: Cell<bool>,
    open_fails_at: Cell<usize>,
    open_count: Cell<usize>,
    bundle: Option<PathBuf>,
    discovered: Vec<PathBuf>,
    launch_after: Option<Duration>,
}
impl Fake {
    fn with_processes(text: &str) -> Self {
        Self {
            frames: RefCell::new(vec![(Duration::ZERO, text.into())]),
            signal_removes: Cell::new(true),
            ..Self::default()
        }
    }
    fn processes(&self) -> String {
        self.frames
            .borrow()
            .iter()
            .filter(|(t, _)| *t <= self.time.get())
            .last()
            .map(|(_, s)| s.clone())
            .unwrap_or_default()
    }
    fn install_bundle(&mut self) {
        let b = PathBuf::from("/Applications/ChatGPT.app");
        self.files
            .insert(b.join("Contents/MacOS/ChatGPT"), String::new());
        self.bundle = Some(b)
    }
    fn output_text(success: bool, text: String) -> io::Result<CommandOutput> {
        Ok(CommandOutput {
            success,
            stdout: text.into_bytes(),
            stderr: if success {
                vec![]
            } else {
                b"synthetic failure".to_vec()
            },
        })
    }
}
impl Runner for Fake {
    fn output(&self, op: &Operation) -> io::Result<CommandOutput> {
        self.calls.borrow_mut().push(op.clone());
        match op {
            Operation::Processes => {
                let mut p = self.processes();
                if self
                    .launch_after
                    .is_some_and(|after| self.time.get() >= after)
                {
                    p.push_str("\n501 /Applications/ChatGPT.app/Contents/MacOS/ChatGPT")
                }
                Self::output_text(!self.ps_fails.get(), p)
            }
            Operation::CommandForPid(pid) => Self::output_text(
                self.holders.contains_key(pid),
                self.holders.get(pid).cloned().unwrap_or_default(),
            ),
            Operation::FindBundles => Self::output_text(
                true,
                self.discovered
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
            Operation::BundleValue { bundle, field } => Self::output_text(
                self.bundle.as_ref() == Some(bundle),
                match field {
                    BundleField::Identifier => "com.openai.codex",
                    BundleField::Executable => "ChatGPT",
                }
                .into(),
            ),
            Operation::IsAppRunning => Self::output_text(true, "false".into()),
            Operation::QuitApp => Self::output_text(!self.quit_fails.get(), String::new()),
            Operation::OpenBundle(_) => {
                self.open_count.set(self.open_count.get() + 1);
                Self::output_text(
                    self.open_fails_at.get() != self.open_count.get(),
                    String::new(),
                )
            }
            Operation::DbHolders(_) => Self::output_text(
                false,
                self.holders
                    .keys()
                    .map(u32::to_string)
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
        }
    }
    fn signal_kill(&self, pid: u32) -> io::Result<()> {
        self.killed.borrow_mut().push(pid);
        if self.signal_fails.get() {
            return Err(io::Error::from_raw_os_error(1));
        }
        if self.signal_removes.get() {
            let text = self
                .processes()
                .lines()
                .filter(|l| {
                    l.trim()
                        .split_whitespace()
                        .next()
                        .and_then(|s| s.parse::<u32>().ok())
                        != Some(pid)
                })
                .collect::<Vec<_>>()
                .join("\n");
            self.frames.borrow_mut().push((self.time.get(), text));
        }
        Ok(())
    }
    fn exists(&self, p: &Path) -> bool {
        self.files.contains_key(p)
    }
    fn is_file(&self, p: &Path) -> bool {
        self.files.contains_key(p)
    }
    fn read_text(&self, p: &Path) -> io::Result<String> {
        self.files
            .get(p)
            .cloned()
            .ok_or_else(|| io::ErrorKind::NotFound.into())
    }
    fn read_dir(&self, p: &Path) -> io::Result<Vec<PathBuf>> {
        self.dirs
            .get(p)
            .cloned()
            .ok_or_else(|| io::ErrorKind::NotFound.into())
    }
    fn process_id(&self) -> u32 {
        999
    }
    fn now(&self) -> Duration {
        self.time.get()
    }
    fn sleep(&self, d: Duration) {
        self.time.set(self.time.get() + d)
    }
}
fn ctx() -> ProcessContext {
    ProcessContext::new(
        PathBuf::from("/synthetic/codex"),
        Some(PathBuf::from("/synthetic/user")),
    )
}
fn p(pid: u32) -> ProcessEntry {
    ProcessEntry {
        pid,
        command: "codex".into(),
    }
}
#[test]
fn classifier_preserves_cli_origins_and_desktop_precedence() {
    for command in [
        "codex --version",
        "/opt/homebrew/bin/codex",
        "/usr/local/bin/codex",
        "/synthetic/.local/bin/codex",
        "node /usr/local/lib/node_modules/@openai/codex/bin/codex.js",
        "/synthetic/extensions/openai.chatgpt-1/bin/codex",
    ] {
        assert_eq!(
            classify_macos_codex_writer(command),
            WriterClass::ExternalCli,
            "{command}"
        )
    }
    for command in [
        "/synthetic/codex",
        "/synthetic/codex-nightly",
        "node /synthetic/codex.js",
        "/synthetic/.vscode/extensions/vendor/bin/codex",
    ] {
        assert_eq!(
            classify_macos_codex_writer(command),
            WriterClass::Other,
            "{command}"
        )
    }
    assert_eq!(
        classify_macos_codex_writer("/Applications/ChatGPT.app/Contents/Resources/bin/codex"),
        WriterClass::Desktop
    );
    assert_eq!(
        classify_macos_codex_writer("/Applications/Codex.app/Contents/MacOS/Codex"),
        WriterClass::Other
    );
}
#[test]
fn protected_rules_and_bundle_ownership_do_not_expand_names() {
    let main = Path::new("/synthetic/ChatGPT.app/Contents/MacOS/ChatGPT");
    for command in [
        "/synthetic/.vscode/extensions/vendor/bin/codex",
        "/synthetic/.codex/computer-use/bin/codex",
        "/Applications/Codex Computer Use.app/bin/codex",
        "/Applications/AiMaMi.app/codex",
        "/synthetic/extensions/openai.chatgpt-1/bin/codex",
        "/Applications/ChatGPT.app/Contents/Frameworks/crashpad_handler",
    ] {
        assert!(is_protected_process_command(command), "{command}");
        assert!(!is_codex_related_command_with_main(command, Some(main)))
    }
    assert!(is_codex_related_command_with_main(
        "/synthetic/ChatGPT.app/Contents/Frameworks/helper",
        Some(main)
    ));
    assert!(!is_codex_related_command_with_main(
        "/synthetic/ChatGPT.app-other/Contents/Frameworks/helper",
        Some(main)
    ));
    assert!(is_codex_related_command_with_main(
        "node /synthetic/codex-nightly",
        None
    ));
    assert!(!is_codex_related_command_with_main(
        "node /synthetic/codex_extra",
        None
    ));
    assert!(is_codex_related_command_with_main("codex_extra", None));
    assert!(!is_codex_related_command_with_main("sh -c codex", None));
}
#[test]
fn fixed_commands_have_no_shell_interpolation() {
    assert_eq!(
        Operation::Processes.argv(),
        (
            "ps",
            vec!["-ax".into(), "-o".into(), "pid=,command=".into()]
        )
    );
    let op = Operation::OpenBundle(PathBuf::from("/synthetic/$(touch evil); ChatGPT.app"));
    assert_eq!(op.argv().0, "/usr/bin/open");
    assert_eq!(op.argv().1.len(), 1);
    assert_eq!(Operation::QuitApp.argv().0, "/usr/bin/osascript");
    assert!(Operation::QuitApp.argv().1[1]
        .to_string_lossy()
        .contains("tell application id \"com.openai.codex\" to quit"));
}
#[test]
fn database_candidates_validate_u32_sort_descending_then_path() {
    let mut f = Fake::default();
    let c = ctx();
    let paths = [
        "state_2.sqlite",
        "state_10.sqlite",
        "state_+10.sqlite",
        "state_4294967296.sqlite",
        "state_no.sqlite",
        "state_3.sqlite-wal",
    ]
    .map(|s| c.codex_home.join(s));
    for p in &paths {
        f.files.insert(p.clone(), String::new());
    }
    f.dirs.insert(c.codex_home.clone(), paths.into());
    assert_eq!(
        runtime_state_db_candidates(&f, &c)
            .iter()
            .map(|p| p.file_name().unwrap().to_str().unwrap())
            .collect::<Vec<_>>(),
        vec!["state_+10.sqlite", "state_10.sqlite", "state_2.sqlite"]
    );
}
#[test]
fn configured_sqlite_home_is_exclusive_and_cwd_relative() {
    let mut f = Fake::default();
    let mut c = ctx();
    c.cwd = Some("/synthetic/work".into());
    c.sqlite_home_env = Some("/synthetic/env".into());
    f.files.insert(
        c.codex_home.join("config.toml"),
        "sqlite_home=' db '\n".into(),
    );
    let p = PathBuf::from("/synthetic/work/db/state_4.sqlite");
    f.files.insert(p.clone(), String::new());
    f.dirs.insert("/synthetic/work/db".into(), vec![p.clone()]);
    assert_eq!(runtime_state_db_candidates(&f, &c), vec![p]);
}
#[test]
fn holder_probe_keeps_nonzero_lsof_stdout_and_excludes_self() {
    let mut f = Fake::default();
    let db = PathBuf::from("/synthetic/state_1.sqlite");
    f.files.insert(db.clone(), String::new());
    f.files
        .insert("/synthetic/state_1.sqlite-wal".into(), String::new());
    f.holders.insert(55, "/synthetic/editor".into());
    f.holders.insert(999, "self".into());
    assert_eq!(
        db_holders_for(&f, &db),
        vec![ProcessEntry {
            pid: 55,
            command: "/synthetic/editor".into()
        }]
    );
    let op = f
        .calls
        .borrow()
        .iter()
        .find(|op| matches!(op, Operation::DbHolders(_)))
        .unwrap()
        .clone();
    assert_eq!(
        op.argv().1,
        vec![
            "-t".into(),
            "--".into(),
            db.into_os_string(),
            "/synthetic/state_1.sqlite-wal".into()
        ]
    );
}
#[test]
fn force_policy_kills_external_cli_but_protects_ide() {
    let f = Fake::with_processes(
        "111 codex --arg\n222 /synthetic/.vscode/extensions/openai.chatgpt/bin/codex\n999 codex",
    );
    let preview = preview_stop(&f, &ctx(), ProcessPolicy::ForceCloseAll).unwrap();
    assert_eq!(preview.targets, vec![p(111)]);
    assert_eq!(preview.external_cli.len(), 2);
    let report = force_close_all(&f, &ctx()).unwrap();
    assert_eq!(report.processes, vec![p(111)]);
    assert_eq!(*f.killed.borrow(), vec![111]);
}
#[test]
fn transition_rejects_external_before_quit_or_signal() {
    let f = Fake::with_processes("111 codex");
    assert!(matches!(
        prepare_transition(&f, &ctx(), Duration::ZERO, ProcessPolicy::DesktopTransition),
        Err(ProcessError::ExternalWriters(_))
    ));
    assert!(f.killed.borrow().is_empty());
    assert!(!f.calls.borrow().contains(&Operation::QuitApp));
}
#[test]
fn transition_filter_is_chatgpt_path_only() {
    let mut f=Fake::with_processes("111 /Applications/ChatGPT.app/Contents/MacOS/ChatGPT\n112 /Applications/Codex.app/Contents/MacOS/Codex");
    f.install_bundle();
    prepare_transition(&f, &ctx(), Duration::ZERO, ProcessPolicy::DesktopTransition).unwrap();
    assert_eq!(*f.killed.borrow(), vec![111]);
}
#[test]
fn signal_errors_attempt_all_targets_and_timeout_retries() {
    let f = Fake::with_processes("111 codex\n112 codex-nightly");
    f.signal_fails.set(true);
    let e = force_kill_targets(&f, &[p(111), p(112)]).unwrap_err();
    assert!(e.to_string().contains("111"));
    assert!(e.to_string().contains("112"));
    assert_eq!(*f.killed.borrow(), vec![111, 112]);
    f.killed.borrow_mut().clear();
    assert!(matches!(
        kill_codex_targets_until_clear(&f, &ctx()),
        Err(ProcessError::ForceKillTimeout {
            last_signal_error: Some(_),
            ..
        })
    ));
    assert_eq!(f.time.get(), Duration::from_secs(3));
    assert_eq!(f.killed.borrow().len(), 20);
}
#[test]
fn ps_failure_propagates_without_signal() {
    let f = Fake::with_processes("111 codex");
    f.ps_fails.set(true);
    assert!(matches!(
        force_close_all(&f, &ctx()),
        Err(ProcessError::Command {
            operation: "ps",
            ..
        })
    ));
    assert!(f.killed.borrow().is_empty());
}
#[test]
fn protected_unknown_db_holder_blocks_empty_success() {
    let mut f = Fake::default();
    let c = ctx();
    let db = c.codex_home.join("state_1.sqlite");
    f.files.insert(db.clone(), String::new());
    f.dirs.insert(c.codex_home.clone(), vec![db]);
    f.holders.insert(88, "/synthetic/editor".into());
    assert!(
        matches!(kill_codex_targets_until_clear(&f,&c),Err(ProcessError::ProtectedDbHolders(p))if p[0].pid==88)
    );
    assert!(f.killed.borrow().is_empty());
}
#[test]
fn tracker_counts_new_pids_but_not_reordering_or_subsets() {
    let mut t = WriterGenerationTracker::new(false);
    assert_eq!(t.observe(&[p(2), p(1), p(1)]), Generation::Initial);
    assert_eq!(t.observe(&[p(1)]), Generation::Retry);
    assert_eq!(t.observe(&[p(3)]), Generation::Respawn);
    assert_eq!(t.observe(&[]), Generation::Empty);
    assert_eq!(t.observe(&[p(3)]), Generation::Respawn);
}
#[test]
fn quiet_requires_700ms_continuously_empty() {
    let f = Fake::default();
    assert_eq!(
        enforce_quiet_window(
            &f,
            &ctx(),
            Duration::from_secs(2),
            false,
            ProcessPolicy::ForceCloseAll
        )
        .unwrap(),
        0
    );
    assert_eq!(f.time.get(), Duration::from_millis(720));
}
#[test]
fn quiet_success_does_not_repeat_signal_to_persistent_generation() {
    let f = Fake::with_processes("111 codex");
    f.signal_removes.set(false);
    assert!(matches!(
        enforce_quiet_window(
            &f,
            &ctx(),
            Duration::from_secs(1),
            false,
            ProcessPolicy::ForceCloseAll
        ),
        Err(ProcessError::QuietTimeout { .. })
    ));
    assert_eq!(*f.killed.borrow(), vec![111]);
}
#[test]
fn quiet_retries_three_failed_signals() {
    let f = Fake::with_processes("111 codex");
    f.signal_fails.set(true);
    assert!(matches!(
        enforce_quiet_window(
            &f,
            &ctx(),
            Duration::from_secs(9),
            false,
            ProcessPolicy::ForceCloseAll
        ),
        Err(ProcessError::QuietTimeout {
            last_signal_error: Some(_),
            ..
        })
    ));
    assert_eq!(*f.killed.borrow(), vec![111, 111, 111]);
    assert_eq!(f.time.get(), Duration::from_millis(160));
}
#[test]
fn quiet_new_generation_after_empty_counts_respawn() {
    let f = Fake::default();
    f.signal_removes.set(true);
    f.frames
        .borrow_mut()
        .push((Duration::from_millis(320), "112 codex".into()));
    assert_eq!(
        enforce_quiet_window(
            &f,
            &ctx(),
            Duration::from_secs(3),
            false,
            ProcessPolicy::ForceCloseAll
        )
        .unwrap(),
        1
    );
    assert_eq!(*f.killed.borrow(), vec![112]);
    assert_eq!(f.time.get(), Duration::from_millis(1120));
}
#[test]
fn launch_validates_main_and_activates_twice() {
    let mut f = Fake::default();
    f.install_bundle();
    f.launch_after = Some(Duration::from_millis(200));
    assert_eq!(launch_codex_app(&f, &ctx()).unwrap(), 501);
    assert_eq!(f.open_count.get(), 2);
    assert_eq!(f.time.get(), Duration::from_millis(200));
}
#[test]
fn launch_missing_bundle_never_opens() {
    let f = Fake::default();
    assert!(matches!(
        launch_codex_app(&f, &ctx()),
        Err(ProcessError::NotFound(_))
    ));
    assert_eq!(f.open_count.get(), 0);
}
#[test]
fn launch_timeout_and_second_activation_error_propagate() {
    let mut f = Fake::default();
    f.install_bundle();
    assert!(matches!(
        launch_codex_app(&f, &ctx()),
        Err(ProcessError::LaunchTimeout(_))
    ));
    assert_eq!(f.time.get(), Duration::from_secs(10));
    assert_eq!(f.open_count.get(), 1);
    f.launch_after = Some(Duration::ZERO);
    f.open_count.set(0);
    f.open_fails_at.set(2);
    assert!(matches!(
        launch_codex_app(&f, &ctx()),
        Err(ProcessError::Command {
            operation: "open",
            ..
        })
    ));
}
#[test]
fn not_running_skips_grace_period_and_quit() {
    let f = Fake::with_processes("111 codex-nightly");
    prepare_transition(
        &f,
        &ctx(),
        Duration::from_secs(30),
        ProcessPolicy::ForceCloseAll,
    )
    .unwrap();
    assert_eq!(f.time.get(), Duration::from_millis(300));
    assert!(!f.calls.borrow().contains(&Operation::QuitApp));
}
#[test]
fn graceful_quit_failure_is_ignored_before_policy_force_fallback() {
    let mut f = Fake::with_processes("111 /Applications/ChatGPT.app/Contents/MacOS/ChatGPT");
    f.install_bundle();
    f.quit_fails.set(true);
    prepare_transition(
        &f,
        &ctx(),
        Duration::from_millis(200),
        ProcessPolicy::DesktopTransition,
    )
    .unwrap();
    assert!(f.calls.borrow().contains(&Operation::QuitApp));
    assert_eq!(*f.killed.borrow(), vec![111]);
    assert_eq!(f.time.get(), Duration::from_millis(500));
}
#[test]
fn background_fence_counts_respawn_and_never_rekills_successful_generation() {
    let f = Fake::with_processes("111 codex");
    f.signal_removes.set(false);
    let mut poller = FencePoller::default();
    assert_eq!(
        poller
            .poll(&f, &ctx(), ProcessPolicy::ForceCloseAll)
            .unwrap(),
        Generation::Respawn
    );
    assert_eq!(poller.respawns(), 1);
    assert_eq!(
        poller
            .poll(&f, &ctx(), ProcessPolicy::ForceCloseAll)
            .unwrap(),
        Generation::Persistent
    );
    assert_eq!(*f.killed.borrow(), vec![111]);
    assert!(!poller.tainted());
}
#[test]
fn background_fence_marks_any_failed_poll_and_stops_retrying_generation_after_three_failures() {
    let f = Fake::with_processes("111 codex");
    f.signal_fails.set(true);
    let mut poller = FencePoller::default();
    for _ in 0..3 {
        assert!(poller
            .poll(&f, &ctx(), ProcessPolicy::ForceCloseAll)
            .is_err())
    }
    assert!(poller.tainted());
    assert_eq!(
        poller
            .poll(&f, &ctx(), ProcessPolicy::ForceCloseAll)
            .unwrap(),
        Generation::Persistent
    );
    assert_eq!(f.killed.borrow().len(), 3);
    f.ps_fails.set(true);
    assert!(poller
        .poll(&f, &ctx(), ProcessPolicy::ForceCloseAll)
        .is_err());
    assert_eq!(poller.respawns(), 1);
}
struct LockedFake(std::sync::Mutex<Fake>);
impl Runner for LockedFake {
    fn output(&self, op: &Operation) -> io::Result<CommandOutput> {
        self.0.lock().unwrap().output(op)
    }
    fn signal_kill(&self, pid: u32) -> io::Result<()> {
        self.0.lock().unwrap().signal_kill(pid)
    }
    fn exists(&self, p: &Path) -> bool {
        self.0.lock().unwrap().exists(p)
    }
    fn is_file(&self, p: &Path) -> bool {
        self.0.lock().unwrap().is_file(p)
    }
    fn read_text(&self, p: &Path) -> io::Result<String> {
        self.0.lock().unwrap().read_text(p)
    }
    fn read_dir(&self, p: &Path) -> io::Result<Vec<PathBuf>> {
        self.0.lock().unwrap().read_dir(p)
    }
    fn process_id(&self) -> u32 {
        999
    }
    fn now(&self) -> Duration {
        self.0.lock().unwrap().now()
    }
    fn sleep(&self, d: Duration) {
        self.0.lock().unwrap().sleep(d);
        std::thread::yield_now();
    }
}
#[test]
fn actual_named_fence_thread_runs_only_fake_operations_then_joins() {
    let f = std::sync::Arc::new(LockedFake(std::sync::Mutex::new(Fake::default())));
    let fence = CodexTransitionFence::acquire(
        f.clone(),
        ctx(),
        Duration::ZERO,
        FenceMode::Strict,
        ProcessPolicy::DesktopTransition,
    )
    .unwrap();
    assert!(fence.initially_quiet());
    let report = fence.finish().unwrap();
    assert_eq!(
        report,
        FenceReport {
            respawns: 0,
            tainted: false
        }
    );
    assert!(f.0.lock().unwrap().killed.borrow().is_empty());
}
#[test]
fn strict_fence_rejects_and_best_effort_preserves_taint() {
    let f = std::sync::Arc::new(LockedFake(std::sync::Mutex::new(Fake::with_processes(
        "111 codex",
    ))));
    assert!(matches!(
        CodexTransitionFence::acquire(
            f.clone(),
            ctx(),
            Duration::ZERO,
            FenceMode::Strict,
            ProcessPolicy::DesktopTransition
        ),
        Err(ProcessError::ExternalWriters(_))
    ));
    let fence = CodexTransitionFence::acquire(
        f.clone(),
        ctx(),
        Duration::ZERO,
        FenceMode::BestEffort,
        ProcessPolicy::DesktopTransition,
    )
    .unwrap();
    assert!(!fence.initially_quiet());
    assert!(fence.finish().unwrap().tainted);
    assert!(f.0.lock().unwrap().killed.borrow().is_empty());
}
#[test]
fn relaunch_policy_captures_actual_initial_state_and_requires_successful_verification() {
    let mut f = Fake::with_processes("111 /Applications/ChatGPT.app/Contents/MacOS/ChatGPT");
    f.install_bundle();
    let policy =
        CodexRelaunchPolicy::capture(&f, &ctx(), RelaunchRequest::RestorePreviousRunningState);
    f.frames.borrow_mut().clear();
    assert_eq!(
        policy.after_verification_result(&f, &ctx(), &Err::<(), _>("verify failed")),
        RelaunchOutcome::Skipped
    );
    assert_eq!(f.open_count.get(), 0);
    assert!(matches!(
        policy.after_verification_result(&f, &ctx(), &Ok::<_, ()>(())),
        RelaunchOutcome::Warning(ProcessError::LaunchTimeout(_))
    ));
    assert_eq!(f.open_count.get(), 1);
}
#[test]
fn relaunch_policy_skips_unrequested_initially_closed_or_currently_running() {
    let mut f = Fake::default();
    f.install_bundle();
    for request in [
        RelaunchRequest::LeaveClosed,
        RelaunchRequest::RestorePreviousRunningState,
    ] {
        let policy = CodexRelaunchPolicy::capture(&f, &ctx(), request);
        assert_eq!(
            policy.after_verification_result(&f, &ctx(), &Ok::<_, ()>(())),
            RelaunchOutcome::Skipped
        )
    }
    f.frames.borrow_mut().push((
        Duration::ZERO,
        "111 /Applications/ChatGPT.app/Contents/MacOS/ChatGPT".into(),
    ));
    let policy =
        CodexRelaunchPolicy::capture(&f, &ctx(), RelaunchRequest::RestorePreviousRunningState);
    assert_eq!(
        policy.after_verification_result(&f, &ctx(), &Ok::<_, ()>(())),
        RelaunchOutcome::Skipped
    );
    assert_eq!(f.open_count.get(), 0);
}
#[test]
fn relaunch_policy_launches_only_after_runner_observes_exit() {
    let mut f = Fake::with_processes("111 /Applications/ChatGPT.app/Contents/MacOS/ChatGPT");
    f.install_bundle();
    let policy =
        CodexRelaunchPolicy::capture(&f, &ctx(), RelaunchRequest::RestorePreviousRunningState);
    f.frames.borrow_mut().clear();
    f.launch_after = Some(Duration::from_millis(200));
    assert_eq!(
        policy.after_verification_result(&f, &ctx(), &Ok::<_, ()>(())),
        RelaunchOutcome::Started(501)
    );
    assert_eq!(f.open_count.get(), 2);
}
#[test]
fn quiet_timeout_distinguishes_empty_quit_blocked_and_orphan() {
    let f = Fake::default();
    assert!(matches!(
        enforce_quiet_window(
            &f,
            &ctx(),
            Duration::ZERO,
            false,
            ProcessPolicy::ForceCloseAll
        ),
        Err(ProcessError::QuietTimeout {
            kind: QuietTimeoutKind::UnconfirmedWindow,
            ..
        })
    ));
    let f = Fake::with_processes("111 codex");
    f.signal_removes.set(false);
    assert!(matches!(
        enforce_quiet_window(
            &f,
            &ctx(),
            Duration::ZERO,
            false,
            ProcessPolicy::ForceCloseAll
        ),
        Err(ProcessError::QuietTimeout {
            kind: QuietTimeoutKind::OrphanWriter,
            ..
        })
    ));
    let mut f = Fake::with_processes("111 /Applications/ChatGPT.app/Contents/MacOS/ChatGPT");
    f.install_bundle();
    f.signal_removes.set(false);
    assert!(matches!(
        enforce_quiet_window(
            &f,
            &ctx(),
            Duration::ZERO,
            false,
            ProcessPolicy::ForceCloseAll
        ),
        Err(ProcessError::QuietTimeout {
            kind: QuietTimeoutKind::QuitBlocked,
            ..
        })
    ));
}
#[test]
fn discovery_checks_user_applications_before_sorted_metadata_fallback() {
    let mut f = Fake::default();
    let c = ctx();
    let user_bundle = c
        .user_home
        .as_ref()
        .unwrap()
        .join("Applications/ChatGPT.app");
    f.bundle = Some(user_bundle.clone());
    f.files
        .insert(user_bundle.join("Contents/MacOS/ChatGPT"), String::new());
    assert_eq!(codex_app_bundle_path(&f, &c), Some(user_bundle));
    assert!(!f.calls.borrow().contains(&Operation::FindBundles));
    let mut f = Fake::default();
    let bundle = PathBuf::from("/synthetic/metadata/ChatGPT.app");
    f.bundle = Some(bundle.clone());
    f.files
        .insert(bundle.join("Contents/MacOS/ChatGPT"), String::new());
    f.discovered = vec![
        "/synthetic/z-invalid.app".into(),
        bundle.clone(),
        bundle.clone(),
    ];
    assert_eq!(codex_app_bundle_path(&f, &c), Some(bundle.clone()));
    assert_eq!(f.calls.borrow().iter().filter(|op|matches!(op,Operation::BundleValue{bundle:b,field:BundleField::Identifier}if b==&bundle)).count(),1);
}
#[test]
fn invalid_bundle_main_and_first_open_failure_never_claim_started() {
    let mut f = Fake::default();
    f.bundle = Some("/Applications/ChatGPT.app".into());
    assert!(matches!(
        launch_codex_app(&f, &ctx()),
        Err(ProcessError::NotFound(_))
    ));
    assert_eq!(f.open_count.get(), 0);
    f.install_bundle();
    f.open_fails_at.set(1);
    assert!(matches!(
        launch_codex_app(&f, &ctx()),
        Err(ProcessError::Command {
            operation: "open",
            ..
        })
    ));
    assert!(!f.calls.borrow().contains(&Operation::Processes));
    assert_eq!(f.open_count.get(), 1);
}
