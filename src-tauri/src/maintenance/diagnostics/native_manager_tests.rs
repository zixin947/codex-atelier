use super::*;
use atelier_maintenance_core::process::{BundleField, CommandOutput, Operation};
use std::{
    fs,
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
    time::Duration,
};

struct FakeRunner {
    root: PathBuf,
    running: AtomicBool,
    refuse_stop: AtomicBool,
    fail_process_list: AtomicBool,
    release_auth_lock_after_failure: AtomicBool,
    clock: AtomicU64,
    owner: std::thread::ThreadId,
    calls: Mutex<Vec<Operation>>,
}
impl FakeRunner {
    fn new(root: PathBuf) -> Self {
        Self {
            root,
            running: AtomicBool::new(false),
            refuse_stop: AtomicBool::new(false),
            fail_process_list: AtomicBool::new(false),
            release_auth_lock_after_failure: AtomicBool::new(false),
            clock: AtomicU64::new(0),
            owner: std::thread::current().id(),
            calls: Mutex::new(vec![]),
        }
    }
}
impl Runner for FakeRunner {
    fn output(&self, op: &Operation) -> io::Result<CommandOutput> {
        self.calls.lock().unwrap().push(op.clone());
        if *op == Operation::Processes
            && self.release_auth_lock_after_failure.load(Ordering::SeqCst)
        {
            let journal = self
                .root
                .join("codex/codexmate/relay")
                .join(atelier_maintenance_core::transition_journal::FILE_NAME);
            let failed = fs::read(&journal)
                .ok()
                .and_then(|b| serde_json::from_slice::<TransitionJournal>(&b).ok())
                .is_some_and(|j| j.phase == "failed");
            if failed {
                fs::remove_dir(self.root.join("codex/accounts/.live-auth.lock"))?;
                self.release_auth_lock_after_failure
                    .store(false, Ordering::SeqCst);
            }
        }
        if *op == Operation::Processes && self.fail_process_list.load(Ordering::SeqCst) {
            return Err(io::Error::other("synthetic process observation failure"));
        }
        let main = "/Applications/ChatGPT.app/Contents/MacOS/ChatGPT";
        let text = match op {
            Operation::Processes if self.running.load(Ordering::SeqCst) => format!("77 {main}\n"),
            Operation::CommandForPid(77) if self.running.load(Ordering::SeqCst) => main.into(),
            Operation::IsAppRunning => self.running.load(Ordering::SeqCst).to_string(),
            Operation::BundleValue {
                field: BundleField::Identifier,
                ..
            } => "com.openai.codex".into(),
            Operation::BundleValue {
                field: BundleField::Executable,
                ..
            } => "ChatGPT".into(),
            Operation::QuitApp => {
                if !self.refuse_stop.load(Ordering::SeqCst) {
                    self.running.store(false, Ordering::SeqCst);
                }
                String::new()
            }
            Operation::OpenBundle(_) => {
                self.running.store(true, Ordering::SeqCst);
                String::new()
            }
            _ => String::new(),
        };
        if let Operation::DbHolders(paths) = op {
            assert!(
                paths.iter().all(|p| p.starts_with(&self.root)),
                "process adapter escaped fixture"
            );
        }
        Ok(CommandOutput {
            success: true,
            stdout: text.into_bytes(),
            stderr: vec![],
        })
    }
    fn signal_kill(&self, pid: u32) -> io::Result<()> {
        assert_eq!(pid, 77);
        if !self.refuse_stop.load(Ordering::SeqCst) {
            self.running.store(false, Ordering::SeqCst);
        }
        Ok(())
    }
    fn exists(&self, path: &Path) -> bool {
        path == Path::new("/Applications/ChatGPT.app")
            || path.starts_with(&self.root) && path.exists()
    }
    fn is_file(&self, path: &Path) -> bool {
        path == Path::new("/Applications/ChatGPT.app/Contents/MacOS/ChatGPT")
            || path.starts_with(&self.root) && path.is_file()
    }
    fn read_text(&self, path: &Path) -> io::Result<String> {
        if !path.starts_with(&self.root) {
            return Err(io::ErrorKind::NotFound.into());
        }
        fs::read_to_string(path)
    }
    fn read_dir(&self, path: &Path) -> io::Result<Vec<PathBuf>> {
        assert!(path.starts_with(&self.root));
        fs::read_dir(path)?.map(|e| e.map(|e| e.path())).collect()
    }
    fn process_id(&self) -> u32 {
        99999
    }
    fn now(&self) -> Duration {
        Duration::from_millis(self.clock.load(Ordering::SeqCst))
    }
    fn sleep(&self, duration: Duration) {
        if std::thread::current().id() == self.owner {
            self.clock
                .fetch_add(duration.as_millis() as u64, Ordering::SeqCst);
        }
        std::thread::sleep(Duration::from_micros(100));
    }
}
struct Fixture {
    root: PathBuf,
    ctx: Context,
    process: ProcessContext,
    runner: Arc<FakeRunner>,
}
impl Fixture {
    fn new(config: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "atelier-production-manager-{}",
            uuid::Uuid::new_v4()
        ));
        let ctx = Context {
            home: root.join("codex"),
            app_root: root.join("app"),
            desktop_running: false,
            runtime: None,
            official_catalog: None,
            proxy_handle: None,
            working_directory: Some(root.clone()),
            sqlite_home_env: None,
        };
        fs::create_dir_all(&ctx.home).unwrap();
        fs::create_dir_all(&ctx.app_root).unwrap();
        fs::write(ctx.config(), config).unwrap();
        let process = ProcessContext {
            user_home: None,
            codex_home: ctx.home.clone(),
            sqlite_home_env: None,
            cwd: Some(root.clone()),
        };
        let runner = Arc::new(FakeRunner::new(root.clone()));
        Self {
            root,
            ctx,
            process,
            runner,
        }
    }
    fn apply(&self, reviewed: Reviewed) -> Result<DiagnosticFixResult> {
        apply_with_runner(
            &self.ctx,
            reviewed,
            self.runner.clone(),
            &self.process,
            &|| (self.ctx.runtime.clone(), self.ctx.proxy_handle.clone()),
        )
    }
    fn gateway(&mut self) -> std::net::TcpListener {
        use crate::gateway::{
            storage::atomic_private,
            types::{Catalog, Credential, GatewayModel, Profile, ProfileKind, Settings},
        };
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let profile_id = uuid::Uuid::new_v4().to_string();
        let catalog = Catalog {
            revision: "fixture-revision".into(),
            host_id: "fixture-host".into(),
            profiles: vec![Profile {
                id: profile_id.clone(),
                kind: ProfileKind::ApiKey,
                name: "fixture".into(),
                email: None,
                enabled: true,
                priority: 0,
                base_url: "https://upstream.invalid/v1".into(),
                auth_status: "warning".into(),
                models: vec![GatewayModel {
                    id: "raw-model".into(),
                    name: "Fixture Model".into(),
                }],
                cooldown_until: None,
                last_error: None,
                requests: 0,
                input_tokens: 0,
                output_tokens: 0,
                created_at: 0,
            }],
            settings: Settings {
                port,
                preferred_profile_id: Some(profile_id.clone()),
                model: "raw-model".into(),
                ..Default::default()
            },
        };
        crate::gateway::storage::private_dir(&self.ctx.app_root.join("gateway/credentials"))
            .unwrap();
        atomic_private(
            &self.ctx.app_root.join("gateway/catalog.json"),
            &serde_json::to_vec(&catalog).unwrap(),
        )
        .unwrap();
        atomic_private(
            &self
                .ctx
                .app_root
                .join("gateway/credentials")
                .join(format!("{profile_id}.json")),
            &serde_json::to_vec(&Credential {
                api_key: Some("fixture-not-a-live-key".into()),
                ..Default::default()
            })
            .unwrap(),
        )
        .unwrap();
        self.ctx.proxy_handle = Some(verification::ProxyHandle {
            port,
            alive: Arc::new(AtomicBool::new(true)),
        });
        self.ctx.runtime = Some(crate::gateway::types::RuntimeView {
            state: "running".into(),
            address: Some(format!("http://127.0.0.1:{port}")),
            ..Default::default()
        });
        fs::write(self.ctx.config(), "model_provider='atelier'\nmodel='raw-model'\n[model_providers.atelier]\nbase_url='http://127.0.0.1:1/v1'\nwire_api='chat'\n# fixture tail\n").unwrap();
        listener
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
const BAD: &str = "model='gpt-fixture'\n# >>> aimami-relay managed start (DO NOT EDIT MANUALLY)\n[[[broken\n# <<< aimami-relay managed end\n";

#[test]
fn confirmed_syntax_repair_runs_shared_manager_and_publishes_actual_intent_receipt() {
    let f = Fixture::new(BAD);
    let plan = preview(&f.ctx, "config_toml_syntax").unwrap();
    assert!(!plan.view.requires_desktop_closed);
    assert!(!router_intent::path(&f.ctx).exists());
    assert_eq!(fs::read(f.ctx.config()).unwrap(), BAD.as_bytes());
    let result = f.apply(plan).unwrap();
    assert!(result.fixed_ids.contains(&"config_toml_syntax".into()));
    let text = fs::read_to_string(f.ctx.config()).unwrap();
    assert!(
        toml::from_str::<toml::Value>(&text).is_ok(),
        "unexpected output: {text}"
    );
    let intent = router_intent::read(&f.ctx).unwrap().unwrap();
    assert_eq!(intent.target, TransitionTarget::NativeOff);
    assert!(intent.operation_id.is_some());
    assert!(!f.ctx.home.join("codexmate/relay/state.json").exists());
    assert!(!f.ctx.home.join("accounts").exists());
    assert!(JournalStore::new(&f.ctx.app_root.join("gateway"))
        .read_checked()
        .unwrap()
        .is_none());
    assert!(Path::new(result.backup_path.as_ref().unwrap())
        .join("manifest.json")
        .is_file());
}

#[test]
fn running_desktop_is_stopped_inside_confirmed_flow_and_relaunched_after_readback() {
    let f = Fixture::new(BAD);
    f.runner.running.store(true, Ordering::SeqCst);
    let plan = preview(&f.ctx, "config_toml_syntax").unwrap();
    assert!(
        f.runner.calls.lock().unwrap().is_empty(),
        "preview must not run process controls"
    );
    let result = f.apply(plan).unwrap();
    assert!(result.failed_ids.is_empty());
    let calls = f.runner.calls.lock().unwrap();
    let quit = calls.iter().position(|c| *c == Operation::QuitApp).unwrap();
    let open = calls
        .iter()
        .position(|c| matches!(c, Operation::OpenBundle(_)))
        .unwrap();
    assert!(quit < open);
    assert!(f.runner.running.load(Ordering::SeqCst));
    assert!(router_intent::read(&f.ctx)
        .unwrap()
        .unwrap()
        .operation_id
        .is_some());
}

#[test]
fn failed_quiet_window_keeps_config_and_does_not_relaunch() {
    let f = Fixture::new(BAD);
    f.runner.running.store(true, Ordering::SeqCst);
    f.runner.refuse_stop.store(true, Ordering::SeqCst);
    let plan = preview(&f.ctx, "config_toml_syntax").unwrap();
    assert!(f.apply(plan).is_err());
    assert_eq!(fs::read(f.ctx.config()).unwrap(), BAD.as_bytes());
    assert!(!router_intent::path(&f.ctx).exists());
    assert!(!f
        .runner
        .calls
        .lock()
        .unwrap()
        .iter()
        .any(|c| matches!(c, Operation::OpenBundle(_))));
}

#[test]
fn config_changed_after_preview_is_rejected_before_any_process_control() {
    let f = Fixture::new(BAD);
    let plan = preview(&f.ctx, "config_toml_syntax").unwrap();
    fs::write(f.ctx.config(), "model='external'\n").unwrap();
    assert!(f.apply(plan).unwrap_err().contains("外部更新"));
    assert!(f.runner.calls.lock().unwrap().is_empty());
    assert!(!router_intent::path(&f.ctx).exists());
    assert_eq!(
        fs::read_to_string(f.ctx.config()).unwrap(),
        "model='external'\n"
    );
}

#[test]
fn unknown_or_unreceipted_pending_commit_does_not_get_erased_or_dispatch_fix() {
    let f = Fixture::new(BAD);
    let plan = preview(&f.ctx, "config_toml_syntax").unwrap();
    let root = f.ctx.app_root.join("gateway");
    JournalStore::new(&root)
        .begin_commit(TransitionTarget::NativeOff, None, 1)
        .unwrap();
    let journal_path = JournalStore::new(&root).path().to_owned();
    let before = fs::read(&journal_path).unwrap();
    assert!(f.apply(plan).unwrap_err().contains("回执"));
    assert_eq!(fs::read(f.ctx.config()).unwrap(), BAD.as_bytes());
    assert_eq!(fs::read(&journal_path).unwrap(), before);
    assert!(JournalStore::new(&root)
        .read_checked()
        .unwrap()
        .unwrap()
        .is_commit_pending());
    assert!(f.runner.calls.lock().unwrap().is_empty());
}

#[test]
fn process_read_failure_never_gets_converted_into_a_quiet_context() {
    let f = Fixture::new(BAD);
    let plan = preview(&f.ctx, "config_toml_syntax").unwrap();
    f.runner.fail_process_list.store(true, Ordering::SeqCst);
    assert!(f.apply(plan).is_err());
    assert_eq!(fs::read(f.ctx.config()).unwrap(), BAD.as_bytes());
    assert!(!router_intent::path(&f.ctx).exists());
}

#[test]
fn a_new_native_source_cannot_replace_a_reviewed_absence_even_with_the_same_off_target() {
    let f = Fixture::new(BAD);
    let plan = preview(&f.ctx, "config_toml_syntax").unwrap();
    let path = f.ctx.home.join("codexmate/relay/state.json");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        &path,
        atelier_maintenance_core::catalog::state::RelayStateDocument::native_default()
            .to_pretty_json()
            .unwrap(),
    )
    .unwrap();
    assert!(f.apply(plan).unwrap_err().contains("来源"));
    assert_eq!(fs::read(f.ctx.config()).unwrap(), BAD.as_bytes());
    assert!(f.runner.calls.lock().unwrap().is_empty());
    assert!(!router_intent::path(&f.ctx).exists());
}

#[test]
fn absent_stores_do_not_hide_native_virtual_auth_and_unlock_markers() {
    let f = Fixture::new("model='fixture'\n");
    let root = f.ctx.home.join("codexmate/relay");
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("router-unlock-auth-marker.json"), b"{}").unwrap();
    fs::write(root.join("router-takeover-backup.json"), b"{}").unwrap();
    fs::write(f.ctx.home.join("auth.json"), br#"{"auth_mode":"apikey","OPENAI_API_KEY":"sk-aimami-router-virtual-fixture","aimami_router_unlock_auth":true}"#).unwrap();
    let report = report(&f.ctx).unwrap();
    let item = report
        .items
        .iter()
        .find(|i| i.id == "router_postcondition")
        .unwrap();
    assert_eq!(item.status, "error");
    assert!(item.detail.contains("takeover backup"));
    assert!(item.detail.contains("unlock auth"));
    assert!(!router_intent::path(&f.ctx).exists());
}

#[test]
fn gateway_direct_postcondition_uses_own_catalog_and_commits_a_real_router_on_receipt() {
    let mut f = Fixture::new("model='fixture'\n");
    let _listener = f.gateway();
    let source_before = fs::read(f.ctx.app_root.join("gateway/catalog.json")).unwrap();
    let plan = preview(&f.ctx, "router_postcondition").unwrap();
    assert!(plan
        .view
        .changes
        .iter()
        .any(|c| c.contains("Atelier Responses Gateway")));
    assert!(!router_intent::path(&f.ctx).exists());
    let result = f.apply(plan).unwrap();
    assert!(result.fixed_ids.contains(&"router_postcondition".into()));
    assert_eq!(
        result
            .report
            .items
            .iter()
            .find(|i| i.id == "router_postcondition")
            .unwrap()
            .status,
        "ok"
    );
    let receipt = router_intent::read(&f.ctx).unwrap().unwrap();
    assert_eq!(receipt.target, TransitionTarget::RouterOn);
    assert_eq!(receipt.provider.as_deref(), Some("atelier"));
    assert!(receipt.operation_id.is_some());
    assert_eq!(
        fs::read(f.ctx.app_root.join("gateway/catalog.json")).unwrap(),
        source_before
    );
    assert!(!f.ctx.home.join("codexmate/relay/state.json").exists());
    assert!(!f.ctx.home.join("auth.json").exists());
    assert!(!f.ctx.home.join("accounts").exists());
    let config = fs::read_to_string(f.ctx.config()).unwrap();
    assert!(config.contains("ATELIER_GATEWAY_TOKEN"));
    assert!(!config.contains("fixture-not-a-live-key"));
    assert!(JournalStore::new(&f.ctx.app_root.join("gateway"))
        .read_checked()
        .unwrap()
        .is_none());
}

#[test]
fn gateway_primary_config_repair_and_followup_share_the_reviewed_target_without_lock_reentry() {
    let mut f = Fixture::new("model='fixture'\n");
    let _listener = f.gateway();
    let plan = preview(&f.ctx, "router_http_contract").unwrap();
    let expected = plan.expected_final_config.clone();
    let result = f.apply(plan).unwrap();
    assert!(result.fixed_ids.contains(&"router_http_contract".into()));
    assert_eq!(optional(&f.ctx.config()).unwrap(), expected);
    assert!(router_intent::read(&f.ctx)
        .unwrap()
        .unwrap()
        .operation_id
        .is_some());
    assert_eq!(
        result
            .report
            .items
            .iter()
            .find(|i| i.id == "router_postcondition")
            .unwrap()
            .status,
        "ok"
    );
}

#[test]
fn stopped_owned_gateway_after_review_is_not_replaced_by_a_persisted_running_flag() {
    let mut f = Fixture::new("model='fixture'\n");
    let listener = f.gateway();
    let plan = preview(&f.ctx, "router_postcondition").unwrap();
    let config_before = fs::read(f.ctx.config()).unwrap();
    f.ctx
        .proxy_handle
        .as_ref()
        .unwrap()
        .alive
        .store(false, Ordering::SeqCst);
    drop(listener);
    assert!(f.apply(plan).unwrap_err().contains("proxy"));
    assert_eq!(fs::read(f.ctx.config()).unwrap(), config_before);
    assert!(!router_intent::path(&f.ctx).exists());
    assert!(f.runner.calls.lock().unwrap().is_empty());
}

#[test]
fn an_unrepaired_syntax_error_keeps_recovery_and_never_publishes_a_healthy_receipt() {
    let f = Fixture::new("model='fixture'\n[[[cannot-salvage\n");
    let plan = preview(&f.ctx, "config_toml_syntax").unwrap();
    assert!(f.apply(plan).is_err());
    assert!(toml::from_str::<toml::Value>(&fs::read_to_string(f.ctx.config()).unwrap()).is_err());
    assert!(!router_intent::path(&f.ctx).exists());
    assert!(JournalStore::new(&f.ctx.app_root.join("gateway"))
        .read_checked()
        .unwrap()
        .is_some());
    assert!(!f
        .runner
        .calls
        .lock()
        .unwrap()
        .iter()
        .any(|c| matches!(c, Operation::OpenBundle(_))));
}

#[test]
fn real_native_off_source_publishes_native_commit_without_creating_an_atelier_intent() {
    let f = Fixture::new("model_provider='aimai1'\nmodel='gpt-fixture'\n[model_providers.aimai1]\nbase_url='http://127.0.0.1:1234/codex/router/v1'\nwire_api='responses'\n");
    let state_path = f.ctx.home.join("codexmate/relay/state.json");
    fs::create_dir_all(state_path.parent().unwrap()).unwrap();
    let original = atelier_maintenance_core::catalog::state::RelayStateDocument::native_default();
    fs::write(&state_path, original.to_pretty_json().unwrap()).unwrap();
    let plan = preview(&f.ctx, "router_postcondition").unwrap();
    assert!(plan.original.intent.is_none());
    let result = f.apply(plan).unwrap();
    assert_eq!(
        result
            .report
            .items
            .iter()
            .find(|i| i.id == "router_postcondition")
            .unwrap()
            .status,
        "ok"
    );
    let actual = atelier_maintenance_core::catalog::state::RelayStateDocument::from_json(
        &fs::read(state_path).unwrap(),
    )
    .unwrap();
    assert!(!actual.catalog_inputs().unwrap().router_enabled);
    assert!(actual.router_commit_id().unwrap().is_some());
    assert!(!router_intent::path(&f.ctx).exists());
    assert!(JournalStore::new(&f.ctx.home.join("codexmate/relay"))
        .read_checked()
        .unwrap()
        .is_none());
}

#[test]
fn a_corrupted_app_journal_stays_visible_in_the_report_and_blocks_mutation() {
    let f = Fixture::new(BAD);
    let plan = preview(&f.ctx, "config_toml_syntax").unwrap();
    let store = JournalStore::new(&f.ctx.app_root.join("gateway"));
    fs::create_dir_all(store.path().parent().unwrap()).unwrap();
    fs::write(store.path(), b"{unreadable record").unwrap();
    let report = report(&f.ctx).unwrap();
    assert_eq!(
        report
            .items
            .iter()
            .find(|i| i.id == "router_recovery_record")
            .unwrap()
            .status,
        "error"
    );
    assert!(f.apply(plan).is_err());
    assert_eq!(fs::read(store.path()).unwrap(), b"{unreadable record");
    assert_eq!(fs::read(f.ctx.config()).unwrap(), BAD.as_bytes());
}

#[test]
fn a_durable_matching_receipt_requires_full_readback_before_pending_commit_is_cleared() {
    let f = Fixture::new("model='fixture'\n");
    let prepared = router_intent::prepare(&f.ctx).unwrap();
    let operation = RouterOperationId::try_from(uuid::Uuid::new_v4().to_string()).unwrap();
    router_intent::commit(&f.ctx, &prepared, &operation, 1)
        .unwrap()
        .publication
        .require_sync()
        .unwrap();
    let original = snapshot(&f.ctx).unwrap();
    let paths = paths(&f.ctx, &original);
    let runtime = || (None, None);
    let host = Host {
        context: &f.ctx,
        current: RefCell::new(original),
        paths: &paths,
        processes: &f.process,
        runner: f.runner.clone(),
        read_runtime: &runtime,
    };
    let journal = JournalStore::new(&paths.verification.relay_root);
    journal
        .begin_commit(TransitionTarget::NativeOff, None, 2)
        .unwrap();
    fs::write(f.ctx.config(), "model='fixture'\n[[[not-valid\n").unwrap();
    assert!(host.checked_recovery(&paths).is_err());
    assert!(journal.read_checked().unwrap().unwrap().is_commit_pending());
    fs::write(f.ctx.config(), "model='fixture'\n").unwrap();
    host.checked_recovery(&paths).unwrap();
    assert!(journal.read_checked().unwrap().is_none());
    assert_eq!(
        router_intent::read(&f.ctx).unwrap().unwrap().operation_id,
        Some(operation)
    );
    assert!(!f
        .runner
        .calls
        .lock()
        .unwrap()
        .iter()
        .any(|o| matches!(o, Operation::QuitApp | Operation::OpenBundle(_))));
}

#[test]
fn pending_target_mismatch_preserves_the_original_recovery_record() {
    let f = Fixture::new("model='fixture'\n");
    let operation = RouterOperationId::try_from(uuid::Uuid::new_v4().to_string()).unwrap();
    router_intent::commit(
        &f.ctx,
        &router_intent::prepare(&f.ctx).unwrap(),
        &operation,
        1,
    )
    .unwrap()
    .publication
    .require_sync()
    .unwrap();
    let original = snapshot(&f.ctx).unwrap();
    let paths = paths(&f.ctx, &original);
    let runtime = || (None, None);
    let host = Host {
        context: &f.ctx,
        current: RefCell::new(original),
        paths: &paths,
        processes: &f.process,
        runner: f.runner.clone(),
        read_runtime: &runtime,
    };
    let journal = JournalStore::new(&paths.verification.relay_root);
    journal
        .begin_commit(TransitionTarget::RouterOn, None, 2)
        .unwrap();
    let before = fs::read(journal.path()).unwrap();
    assert!(host
        .checked_recovery(&paths)
        .unwrap_err()
        .to_string()
        .contains("目标不一致"));
    assert_eq!(fs::read(journal.path()).unwrap(), before);
}

#[test]
fn retry_rejects_a_previously_fixed_item_that_has_become_unknown_and_nonfixable() {
    let f = Fixture::new("openai_base_url='https://foreign.invalid/v1'\n");
    let reviewed = preview(&f.ctx, "config_third_party").unwrap();
    let guard = repair::acquire(&f.ctx).unwrap();
    let paths = paths(&f.ctx, &reviewed.original);
    let runtime = || (None, None);
    let host = Host {
        context: &f.ctx,
        current: RefCell::new(reviewed.original.clone()),
        paths: &paths,
        processes: &f.process,
        runner: f.runner.clone(),
        read_runtime: &runtime,
    };
    let operations = Operations {
        reviewed: &reviewed,
        guard: &guard,
        primary_result: RefCell::new(None),
        batch_receipt: RefCell::new(None),
        backup: RefCell::new(None),
        reconciled: RefCell::new(false),
        active_reconcile: RefCell::new(None),
    };
    let result = operations.apply_primary(&host).unwrap();
    assert!(result.fixed_ids.contains(&"config_third_party".into()));
    fs::create_dir_all(f.ctx.app_root.join("gateway")).unwrap();
    fs::write(
        f.ctx.app_root.join("gateway/catalog.json"),
        b"broken fixture catalog",
    )
    .unwrap();
    let changed = report(&f.ctx).unwrap();
    let third_party = changed
        .items
        .iter()
        .find(|i| i.id == "config_third_party")
        .unwrap();
    assert_eq!(third_party.status, "warning");
    assert!(!third_party.fixable);
    assert!(operations
        .apply_primary(&host)
        .unwrap_err()
        .to_string()
        .contains("再次异常"));
}

#[test]
fn direct_postcondition_failure_really_resumes_approved_auth_effects_and_retains_primary_error() {
    let f = Fixture::new("model_provider='aimai1'\n# keep original comment\n");
    let relay = f.ctx.home.join("codexmate/relay");
    fs::create_dir_all(&relay).unwrap();
    fs::write(
        relay.join("state.json"),
        atelier_maintenance_core::catalog::state::RelayStateDocument::native_default()
            .to_pretty_json()
            .unwrap(),
    )
    .unwrap();
    let auth = br#"{"OPENAI_API_KEY":"sk-synthetic-current-A"}"#;
    fs::write(f.ctx.home.join("auth.json"), auth).unwrap();
    fs::write(
        relay.join("router-unlock-auth-backup.json"),
        br#"{"OPENAI_API_KEY":"sk-synthetic-other-B"}"#,
    )
    .unwrap();
    fs::write(
        relay.join("router-unlock-auth-marker.json"),
        br#"{"backedUpPreviousAuth":true}"#,
    )
    .unwrap();
    let plan = preview(&f.ctx, "router_postcondition").unwrap();
    let backup = f
        .ctx
        .app_root
        .join("maintenance/diagnostic-backups")
        .join(&plan.view.ticket)
        .join("router/router-config-original.json");
    let original = fs::read(f.ctx.config()).unwrap();
    let expected = plan.expected_final_config.clone();
    fs::create_dir_all(f.ctx.home.join("accounts/.live-auth.lock")).unwrap();
    // Release only after the first real reconciliation has failed and left its
    // native journal. No hook can hide the original auth-lock error.
    f.runner
        .release_auth_lock_after_failure
        .store(true, Ordering::SeqCst);
    let error = f.apply(plan).unwrap_err();
    assert!(error.contains("路由修复部分完成"), "{error}");
    assert!(!error.contains("源目标恢复失败"), "{error}");
    assert_eq!(optional(&f.ctx.config()).unwrap(), expected);
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), auth);
    assert!(!relay.join("router-unlock-auth-marker.json").exists());
    assert!(!relay.join("router-unlock-auth-backup.json").exists());
    let actual = atelier_maintenance_core::catalog::state::RelayStateDocument::from_json(
        &fs::read(relay.join("state.json")).unwrap(),
    )
    .unwrap();
    assert!(actual.router_commit_id().unwrap().is_some());
    assert!(!actual.catalog_inputs().unwrap().router_enabled);
    assert!(JournalStore::new(&relay).read_checked().unwrap().is_none());
    assert!(!router_intent::path(&f.ctx).exists());
    let saved: serde_json::Value = serde_json::from_slice(&fs::read(backup).unwrap()).unwrap();
    use base64::Engine;
    assert_eq!(
        base64::engine::general_purpose::STANDARD
            .decode(saved["beforeBase64"].as_str().unwrap())
            .unwrap(),
        original
    );
}

#[test]
fn syntax_repair_rechecks_the_newly_readable_sqlite_home_before_publishing_a_receipt() {
    let f = Fixture::new(BAD);
    let database_home = f.root.join("custom-sqlite");
    fs::create_dir_all(&database_home).unwrap();
    let config = format!("sqlite_home={:?}\n{BAD}", database_home.to_str().unwrap());
    fs::write(f.ctx.config(), config).unwrap();
    let path = f.ctx.home.join("session.jsonl");
    fs::write(&path, b"{\"type\":\"session_meta\",\"payload\":{\"id\":\"fixture-thread\",\"model_provider\":\"atelier_old\"}}\n").unwrap();
    let database = rusqlite::Connection::open(database_home.join("state_9.sqlite")).unwrap();
    database.execute_batch("CREATE TABLE threads(id TEXT PRIMARY KEY, model_provider TEXT, rollout_path TEXT, archived INTEGER, source TEXT, model TEXT)").unwrap();
    database
        .execute(
            "INSERT INTO threads VALUES('fixture-thread','atelier_old',?1,0,'cli','fixture')",
            [&path.to_string_lossy()],
        )
        .unwrap();
    drop(database);
    let plan = preview(&f.ctx, "config_toml_syntax").unwrap();
    assert!(f.apply(plan).is_err());
    assert!(toml::from_str::<toml::Value>(&fs::read_to_string(f.ctx.config()).unwrap()).is_ok());
    assert!(
        !router_intent::path(&f.ctx).exists(),
        "a stale database path must not produce a healthy receipt"
    );
    let database = rusqlite::Connection::open(database_home.join("state_9.sqlite")).unwrap();
    assert_eq!(
        database
            .query_row("SELECT model_provider FROM threads", [], |r| r
                .get::<_, String>(0))
            .unwrap(),
        "atelier_old"
    );
}

#[test]
fn native_global_cleanup_stays_applied_when_later_takeover_restores_old_config_fields() {
    let f =
        Fixture::new("model_provider='aimai1'\nmodel_reasoning_effort=''\n# keep native comment\n");
    let relay = f.ctx.home.join("codexmate/relay");
    fs::create_dir_all(&relay).unwrap();
    fs::write(
        relay.join("state.json"),
        atelier_maintenance_core::catalog::state::RelayStateDocument::native_default()
            .to_pretty_json()
            .unwrap(),
    )
    .unwrap();
    fs::write(relay.join("router-takeover-backup.json"), serde_json::to_vec(&serde_json::json!({"version":1,"createdAtMs":0,"topLevelLines":["model='saved-native-model'","model_reasoning_effort=''"],"sectionBlocks":[],"originalConfig":"never restore whole original blindly"})).unwrap()).unwrap();
    let plan = preview(&f.ctx, "config_global_validity").unwrap();
    assert!(plan.native_primary.is_some());
    let expected = plan.expected_final_config.clone();
    let result = f.apply(plan).unwrap();
    assert!(result.fixed_ids.contains(&"config_global_validity".into()));
    assert_eq!(optional(&f.ctx.config()).unwrap(), expected);
    let final_config: toml::Value =
        toml::from_str(&fs::read_to_string(f.ctx.config()).unwrap()).unwrap();
    assert!(final_config.get("model_reasoning_effort").is_none());
    assert_ne!(
        final_config
            .get("model_provider")
            .and_then(toml::Value::as_str),
        Some("aimai1")
    );
    assert_eq!(final_config["model"].as_str(), Some("saved-native-model"));
    assert!(!relay.join("router-takeover-backup.json").exists());
    assert_eq!(
        result
            .report
            .items
            .iter()
            .find(|i| i.id == "router_postcondition")
            .unwrap()
            .status,
        "ok"
    );
    let state = atelier_maintenance_core::catalog::state::RelayStateDocument::from_json(
        &fs::read(relay.join("state.json")).unwrap(),
    )
    .unwrap();
    assert!(state.router_commit_id().unwrap().is_some());
    assert!(!router_intent::path(&f.ctx).exists());
}

#[tokio::test]
async fn production_report_reads_the_observed_official_runtime_and_rejects_a_changed_source() {
    use super::super::official_catalog::{OfficialCatalogRuntime, Scope};
    let mut f = Fixture::new("model='fixture'\n");
    let _owned_listener = f.gateway();
    let catalog_path = f.ctx.app_root.join("gateway/catalog.json");
    let mut catalog: crate::gateway::types::Catalog =
        serde_json::from_slice(&fs::read(&catalog_path).unwrap()).unwrap();
    catalog.profiles[0].kind = crate::gateway::types::ProfileKind::Chatgpt;
    fs::write(&catalog_path, serde_json::to_vec(&catalog).unwrap()).unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let upstream = tokio::spawn(async move {
        let app = axum::Router::new().route(
            "/models",
            axum::routing::get(|| async {
                axum::Json(serde_json::json!({"models":[{"slug":"actually-observed"}]}))
            }),
        );
        axum::serve(listener, app).await.unwrap();
    });
    let runtime = Arc::new(OfficialCatalogRuntime::with_test_endpoint(format!(
        "http://{address}/models"
    )));
    let scope = Scope::gateway(
        &catalog,
        catalog.settings.preferred_profile_id.as_deref().unwrap(),
    )
    .unwrap();
    runtime.configure(&scope).unwrap();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("authorization", "Bearer synthetic-fixture".parse().unwrap());
    runtime
        .observe_query(&scope, &headers, "client_version=1.2.3")
        .await
        .unwrap();
    f.ctx.official_catalog = Some(runtime);
    let observed = report(&f.ctx).unwrap();
    let item = observed
        .items
        .iter()
        .find(|i| i.id == "official_catalog_freshness")
        .unwrap();
    assert_eq!(item.status, "ok");
    assert!(item.detail.contains("官方模型 1 个"));
    catalog.settings.model = "different-source-model".into();
    fs::write(&catalog_path, serde_json::to_vec(&catalog).unwrap()).unwrap();
    let changed = report(&f.ctx).unwrap();
    assert_eq!(
        changed
            .items
            .iter()
            .find(|i| i.id == "official_catalog_freshness")
            .unwrap()
            .status,
        "warning"
    );
    upstream.abort();
}

#[test]
fn proxy_report_requires_the_actual_owned_listener_instead_of_a_running_string() {
    let mut f = Fixture::new("model='fixture'\n");
    let listener = f.gateway();
    let live = report(&f.ctx).unwrap();
    assert_eq!(
        live.items
            .iter()
            .find(|i| i.id == "proxy_running")
            .unwrap()
            .status,
        "ok"
    );
    f.ctx
        .proxy_handle
        .as_ref()
        .unwrap()
        .alive
        .store(false, Ordering::Release);
    drop(listener);
    let stopped = report(&f.ctx).unwrap();
    assert_eq!(
        stopped
            .items
            .iter()
            .find(|i| i.id == "proxy_running")
            .unwrap()
            .status,
        "warning"
    );
    assert_eq!(
        f.ctx.runtime.as_ref().unwrap().state,
        "running",
        "the stale text must not overrule listener ownership"
    );
}

#[test]
fn all_entry_uses_native_batch_instead_of_combined_single_ticket_and_commits_receipt() {
    let f = Fixture::new(BAD);
    let reviewed = preview(&f.ctx, "all").unwrap();
    assert!(reviewed.batch.is_some());
    assert!(reviewed.primary.is_none());
    assert!(f.runner.calls.lock().unwrap().is_empty());
    assert_eq!(fs::read(f.ctx.config()).unwrap(), BAD.as_bytes());
    let result = f.apply(reviewed).unwrap();
    assert!(
        result.fixed_ids.contains(&"config_toml_syntax".into()),
        "{:?}",
        result.failed_ids
    );
    assert!(result.failed_ids.is_empty());
    assert!(toml::from_str::<toml::Value>(&fs::read_to_string(f.ctx.config()).unwrap()).is_ok());
    assert!(router_intent::read(&f.ctx)
        .unwrap()
        .unwrap()
        .operation_id
        .is_some());
    assert!(JournalStore::new(&f.ctx.app_root.join("gateway"))
        .read_checked()
        .unwrap()
        .is_none());
    assert!(!f.ctx.home.join("codexmate/relay/state.json").exists());
    assert!(!f.ctx.home.join("accounts").exists());
    assert!(result
        .backup_path
        .as_ref()
        .is_some_and(|p| Path::new(p).is_dir()));
}

#[test]
fn all_residual_only_batch_does_not_stop_the_fake_desktop_or_create_router_state() {
    let f = Fixture::new("model_provider='openai'\n");
    let relay = f.ctx.home.join("codexmate/relay");
    fs::create_dir_all(relay.join("thread-router-migrations")).unwrap();
    fs::write(relay.join("thread-router-migrations/current.json"), b"{}").unwrap();
    fs::write(relay.join("relay-state-cache.json"), b"synthetic cache").unwrap();
    f.runner.running.store(true, Ordering::SeqCst);
    let reviewed = preview(&f.ctx, "all").unwrap();
    assert!(reviewed.batch.is_some());
    let result = f.apply(reviewed).unwrap();
    assert!(result.fixed_ids.contains(&"residual_manifest".into()));
    assert!(result.fixed_ids.contains(&"residual_cache".into()));
    assert!(result.failed_ids.is_empty());
    assert!(!relay.join("thread-router-migrations/current.json").exists());
    assert!(!relay.join("relay-state-cache.json").exists());
    assert!(!router_intent::path(&f.ctx).exists());
    assert!(!relay.join("state.json").exists());
    assert!(!f
        .runner
        .calls
        .lock()
        .unwrap()
        .iter()
        .any(|op| matches!(op, Operation::QuitApp | Operation::OpenBundle(_))));
}

#[test]
fn all_batch_final_full_failure_uses_sealed_global_successor_and_commits() {
    // Global's exact native cleanup preserves the stale managed model. With no
    // config preparation selected there is no mid-batch resync: only the final
    // Full failure is allowed to consume the sealed end successor.
    let f = Fixture::new("model_provider='openai'\nmodel='aimami_relay_deleted'\nmodel_reasoning_effort=''\n# retain fixture comment\n");
    let relay = f.ctx.home.join("codexmate/relay");
    fs::create_dir_all(&relay).unwrap();
    let reviewed = preview(&f.ctx, "all").unwrap();
    assert!(reviewed.batch.is_some());
    let reviewed_ids = reviewed.view.item_ids.clone();
    let reviewed_backup = reviewed.batch.as_ref().unwrap().backup_root().to_owned();
    let result = f.apply(reviewed).unwrap();
    assert!(result.failed_ids.is_empty(), "{:?}", result);
    assert!(result.fixed_ids.contains(&"config_global_validity".into()));
    let after = fs::read_to_string(f.ctx.config()).unwrap();
    assert!(!after.contains("aimami_relay_deleted"), "{after}");
    assert!(!after.contains("model_reasoning_effort"), "{after}");
    assert!(after.contains("retain fixture comment"));
    let record = router_intent::read(&f.ctx).unwrap().unwrap();
    assert!(record.operation_id.is_some());
    router_intent::verify_receipt(&f.ctx, &record).unwrap();
    assert!(!relay.join("state.json").exists());
    assert!(!f.ctx.app_root.join("gateway/catalog.json").exists());
    assert!(JournalStore::new(&f.ctx.app_root.join("gateway"))
        .read_checked()
        .unwrap()
        .is_none());
    assert!(
        reviewed_backup
            .join("final-router/successor-config-original.json")
            .exists(),
        "Full failure must consume reviewed successor; ids={reviewed_ids:?}; result={result:?}"
    );
}

#[test]
fn all_wal_missing_history_repair_commits_without_verification_sidecar_changes() {
    let f = Fixture::new("model_provider='openai'\n");
    let path = f.ctx.home.join("state_5.sqlite");
    let db = rusqlite::Connection::open(&path).unwrap();
    db.execute_batch("PRAGMA journal_mode=WAL; CREATE TABLE threads(id TEXT PRIMARY KEY,model_provider TEXT,rollout_path TEXT,model TEXT,archived INTEGER DEFAULT 0,source TEXT DEFAULT 'cli');").unwrap();
    db.execute("INSERT INTO threads(id,model_provider,rollout_path,model) VALUES('missing','openai',?1,'gpt-fixture')", [f.ctx.home.join("sessions/missing.jsonl").to_str().unwrap()]).unwrap();
    drop(db);
    let reviewed = preview(&f.ctx, "all").unwrap();
    assert!(reviewed
        .view
        .item_ids
        .contains(&"thread_missing_rollouts".into()));
    let result = f.apply(reviewed).unwrap();
    assert!(
        result.fixed_ids.contains(&"thread_missing_rollouts".into()),
        "{result:?}"
    );
    assert!(result.failed_ids.is_empty(), "{result:?}");
    let record = router_intent::read(&f.ctx).unwrap().unwrap();
    assert!(record.operation_id.is_some());
    router_intent::verify_receipt(&f.ctx, &record).unwrap();
    super::super::snapshot::with_snapshot_path(&path, |_, db| {
        assert_eq!(
            db.query_row("SELECT archived FROM threads WHERE id='missing'", [], |r| r
                .get::<_, i64>(0))
                .unwrap(),
            1
        );
        Ok(())
    })
    .unwrap();
}

#[test]
fn verification_and_capacity_observe_committed_wal_without_touching_source_sidecars() {
    for native in [false, true] {
        for live_writer in [false, true] {
            let f = Fixture::new("model_provider='openai'\n");
            if native {
                let relay = f.ctx.home.join("codexmate/relay");
                fs::create_dir_all(&relay).unwrap();
                fs::write(
                    relay.join("state.json"),
                    atelier_maintenance_core::catalog::state::RelayStateDocument::native_default()
                        .to_pretty_json()
                        .unwrap(),
                )
                .unwrap();
            }
            let db_path = f.ctx.home.join("state_5.sqlite");
            let db = rusqlite::Connection::open(&db_path).unwrap();
            db.execute_batch("PRAGMA journal_mode=WAL; PRAGMA wal_autocheckpoint=0; CREATE TABLE threads(id TEXT PRIMARY KEY,model_provider TEXT,rollout_path TEXT,model TEXT,archived INTEGER DEFAULT 0,source TEXT DEFAULT 'cli'); PRAGMA wal_checkpoint(TRUNCATE);").unwrap();
            db.execute("INSERT INTO threads(id,model_provider,rollout_path,model) VALUES('missing','openai',?1,'gpt-fixture')", [f.ctx.home.join("sessions/missing.jsonl").to_str().unwrap()]).unwrap();
            let _writer = if live_writer {
                Some(db)
            } else {
                drop(db);
                None
            };
            let files = [
                db_path.clone(),
                PathBuf::from(format!("{}-wal", db_path.display())),
                PathBuf::from(format!("{}-shm", db_path.display())),
            ];
            let observed = files
                .iter()
                .map(|p| read_external_test(p))
                .collect::<Vec<_>>();
            let original = snapshot(&f.ctx).unwrap();
            let paths = paths(&f.ctx, &original);
            let report = verify(
                &f.ctx,
                &original,
                &paths,
                TransitionTarget::NativeOff,
                Depth::Full,
            );
            assert!(
                report
                    .issues
                    .iter()
                    .any(|issue| issue.dim == InvariantDim::Thread),
                "committed missing history must be observed: {report:?}"
            );
            assert!(report
                .issues
                .iter()
                .all(|issue| !issue.message.contains("atelier-diagnostic-")));
            let runtime = || (None, None);
            let host = Host {
                context: &f.ctx,
                current: RefCell::new(original),
                paths: &paths,
                processes: &f.process,
                runner: f.runner.clone(),
                read_runtime: &runtime,
            };
            host.check_write_capacity(&paths, true).unwrap();
            for (p, before) in files.iter().zip(observed) {
                assert_eq!(
                    read_external_test(p),
                    before,
                    "source changed: native={native} live={live_writer} {p:?}"
                );
            }
        }
    }
}
fn read_external_test(path: &Path) -> Option<Vec<u8>> {
    match fs::read(path) {
        Ok(bytes) => Some(bytes),
        Err(e) if e.kind() == io::ErrorKind::NotFound => None,
        Err(e) => panic!("fixture read: {e}"),
    }
}

#[test]
fn external_wal_change_still_invalidates_batch_but_does_not_block_source_route_observation() {
    let f = Fixture::new("model_provider='openai'\n");
    let db_path = f.ctx.home.join("state_5.sqlite");
    let db = rusqlite::Connection::open(&db_path).unwrap();
    db.execute_batch("PRAGMA journal_mode=WAL; CREATE TABLE threads(id TEXT PRIMARY KEY,model_provider TEXT,rollout_path TEXT,model TEXT,archived INTEGER DEFAULT 0,source TEXT DEFAULT 'cli');").unwrap();
    db.execute("INSERT INTO threads(id,model_provider,rollout_path,model) VALUES('missing','openai',?1,'gpt-fixture')", [f.ctx.home.join("sessions/missing.jsonl").to_str().unwrap()]).unwrap();
    drop(db);
    let reviewed = preview(&f.ctx, "all").unwrap();
    let guard = repair::acquire(&f.ctx).unwrap();
    let paths = paths(&f.ctx, &reviewed.original);
    let runtime = || (None, None);
    let host = Host {
        context: &f.ctx,
        current: RefCell::new(reviewed.original.clone()),
        paths: &paths,
        processes: &f.process,
        runner: f.runner.clone(),
        read_runtime: &runtime,
    };
    let operations = Operations {
        reviewed: &reviewed,
        guard: &guard,
        primary_result: RefCell::new(None),
        batch_receipt: RefCell::new(None),
        backup: RefCell::new(None),
        reconciled: RefCell::new(false),
        active_reconcile: RefCell::new(None),
    };
    operations.apply_primary(&host).unwrap();
    let db = rusqlite::Connection::open(&db_path).unwrap();
    db.execute_batch("CREATE TABLE external_data(value TEXT); INSERT INTO external_data VALUES('preserve this external write');").unwrap();
    drop(db);
    let before = fs::read(&db_path).unwrap();
    assert!(operations
        .batch_config(&f.ctx)
        .unwrap_err()
        .to_string()
        .contains("最终已批准效果发生变化"));
    assert!(operations
        .restore(&reviewed.original, "synthetic external write", &host)
        .unwrap()
        .is_empty());
    assert_eq!(fs::read(&db_path).unwrap(), before);
    assert!(
        !router_intent::path(&f.ctx).exists(),
        "route observation is not a batch success receipt"
    );
    fs::write(f.ctx.config(), "model_provider='external-unapproved'\n").unwrap();
    let config_before = fs::read(f.ctx.config()).unwrap();
    assert!(operations
        .restore(&reviewed.original, "unapproved config", &host)
        .unwrap_err()
        .to_string()
        .contains("不属于原审批"));
    assert_eq!(fs::read(f.ctx.config()).unwrap(), config_before);
}

#[test]
fn preview_reuses_ordinary_report_without_repeating_initial_scans() {
    let f = Fixture::new(
        "model_provider='openai'\nmodel='aimami_relay_deleted'\nmodel_reasoning_effort=''\n",
    );
    inspect::take_scan_count();
    let start = std::time::Instant::now();
    let old_original = snapshot(&f.ctx).unwrap();
    let old_report = report(&f.ctx).unwrap();
    let old_gates = manager_report(old_report.clone())
        .repair_gates("all")
        .unwrap();
    let _ordinary = inspect::run(&f.ctx).unwrap();
    let old_batch = super::super::native_batch::prepare(&f.ctx).unwrap();
    let old_ms = start.elapsed().as_secs_f64() * 1000.0;
    assert_eq!(old_original.target(), old_batch.target());
    assert!(old_gates.use_quiet_transition);
    let old_scans = inspect::take_scan_count();
    let start = std::time::Instant::now();
    let reviewed = preview(&f.ctx, "all").unwrap();
    let new_ms = start.elapsed().as_secs_f64() * 1000.0;
    let new_scans = inspect::take_scan_count();
    assert_eq!(
        old_batch.item_ids(),
        reviewed.batch.as_ref().unwrap().item_ids()
    );
    assert_eq!(old_scans, 3);
    assert_eq!(new_scans, 1);
    let start = std::time::Instant::now();
    let result = f.apply(reviewed).unwrap();
    let apply_ms = start.elapsed().as_secs_f64() * 1000.0;
    assert!(result.failed_ids.is_empty());
    println!("PREVIEW_BENCH old_ms={old_ms:.2} shared_ms={new_ms:.2} ordinary_scans={old_scans}->{new_scans} fixed_apply_ms={apply_ms:.2}");
}

#[test]
fn full_report_shares_rollouts_snapshot_and_starts_fresh_after_changes() {
    use atelier_maintenance_core::threads::observation;
    let f = Fixture::new("model_provider='openai'\n");
    let db = f.ctx.home.join("state_5.sqlite");
    let conn = rusqlite::Connection::open(&db).unwrap();
    conn.execute_batch("CREATE TABLE threads(id TEXT PRIMARY KEY,model_provider TEXT,rollout_path TEXT,model TEXT,reasoning_effort TEXT,archived INTEGER,source TEXT,cwd TEXT);").unwrap();
    let mut bytes = 0;
    for n in 0..12 {
        let path = f.ctx.home.join(format!(
            "history-{n}.jsonl{}",
            if n % 2 == 0 { ".zst" } else { "" }
        ));
        let meta = serde_json::json!({"type":"session_meta","payload":{"id":format!("t-{n}"),"cwd":"/synthetic/project","model_provider":"openai","base_instructions":{"text":"healthy fixture"}}});
        let mut data = serde_json::to_vec(&meta).unwrap();
        data.push(b'\n');
        data.extend_from_slice(b"{\"ordinal\":1}\n");
        bytes += data.len() as u64;
        fs::write(
            &path,
            if n % 2 == 0 {
                zstd::stream::encode_all(data.as_slice(), 1).unwrap()
            } else {
                data
            },
        )
        .unwrap();
        conn.execute("INSERT INTO threads VALUES(?1,'openai',?2,'gpt-fixture',NULL,0,'cli','/synthetic/project')", rusqlite::params![format!("t-{n}"),path.to_string_lossy()]).unwrap();
    }
    drop(conn);
    let source = fs::read(&db).unwrap();
    super::super::snapshot::take_snapshot_count();
    let first = super::super::snapshot::with_observation(|| {
        let result = report(&f.ctx)?;
        assert_eq!(observation::metrics(), (12, bytes));
        Ok(result)
    })
    .unwrap();
    assert_eq!(super::super::snapshot::take_snapshot_count(), 1);
    assert_eq!(fs::read(&db).unwrap(), source);
    assert!(!PathBuf::from(format!("{}-wal", db.display())).exists());
    assert_eq!(
        first
            .items
            .iter()
            .find(|i| i.id == "thread_response_item_ids")
            .unwrap()
            .status,
        "ok"
    );
    let changed = f.ctx.home.join("history-1.jsonl");
    use std::io::Write;
    fs::OpenOptions::new()
        .append(true)
        .open(&changed)
        .unwrap()
        .write_all(b"{bad}\n")
        .unwrap();
    let next = report(&f.ctx).unwrap();
    assert_eq!(
        next.items
            .iter()
            .find(|i| i.id == "thread_history_integrity")
            .unwrap()
            .status,
        "error"
    );
}

#[test]
fn native_all_repairs_owned_drift_and_preserves_external_history_through_full_commit() {
    let f = Fixture::new("model_provider='openai'\nmodel_reasoning_effort=''\n[model_providers.aimai1]\nbase_url='http://127.0.0.1:18910/codex/router/v1'\nwire_api='responses'\n");
    let db = f.ctx.home.join("state_5.sqlite");
    let connection = rusqlite::Connection::open(&db).unwrap();
    connection.execute_batch("CREATE TABLE threads(id TEXT PRIMARY KEY,model_provider TEXT,rollout_path TEXT,model TEXT,archived INTEGER DEFAULT 0,source TEXT DEFAULT 'cli');").unwrap();
    let foreign = f.ctx.home.join("foreign.jsonl");
    let owned = f.ctx.home.join("owned.jsonl.zst");
    for (id, path, anchor) in [
        ("foreign", &foreign, "enterprise"),
        ("owned", &owned, "aimai1"),
    ] {
        let bytes = format!("{}\n", serde_json::json!({"type":"session_meta","payload":{"id":id,"model_provider":anchor,"base_instructions":{"text":"Healthy fixture instructions"}}})).into_bytes();
        fs::write(
            path,
            if id == "owned" {
                zstd::stream::encode_all(bytes.as_slice(), 1).unwrap()
            } else {
                bytes
            },
        )
        .unwrap();
        connection
            .execute(
                "INSERT INTO threads VALUES(?1,'openai',?2,'gpt-fixture',0,'cli')",
                rusqlite::params![id, path.to_string_lossy()],
            )
            .unwrap();
    }
    drop(connection);
    let before = fs::read(&foreign).unwrap();
    let reviewed = preview(&f.ctx, "all").unwrap();
    assert!(reviewed
        .view
        .item_ids
        .contains(&"thread_provider_consistency".into()));
    assert!(reviewed
        .view
        .changes
        .iter()
        .any(|line| line.contains("文件 1")));
    let result = f.apply(reviewed).unwrap();
    assert!(result.failed_ids.is_empty(), "{result:?}");
    assert!(result
        .fixed_ids
        .contains(&"thread_provider_consistency".into()));
    assert_eq!(fs::read(&foreign).unwrap(), before);
    let anchors =
        atelier_maintenance_core::threads::consistency::read_rollout_provider_anchors(&owned)
            .unwrap()
            .unwrap();
    assert_eq!(anchors.first.as_deref(), Some("openai"));
    let final_report = report(&f.ctx).unwrap();
    assert_eq!(
        final_report
            .items
            .iter()
            .find(|i| i.id == "router_postcondition")
            .unwrap()
            .status,
        "ok"
    );
    router_intent::verify_receipt(&f.ctx, &router_intent::read(&f.ctx).unwrap().unwrap()).unwrap();
}

#[test]
fn direct_native_off_preview_rejects_unplanned_atelier_threads_before_writes() {
    let f = Fixture::new("model_provider='openai'\nmodel_reasoning_effort=''\n[model_providers.atelier]\nbase_url='http://127.0.0.1:18910/v1'\nwire_api='responses'\n");
    let path = f.ctx.home.join("owned.jsonl");
    fs::write(&path, format!("{}\n", serde_json::json!({"type":"session_meta","payload":{"id":"owned","model_provider":"atelier","base_instructions":{"text":"healthy fixture"}}}))).unwrap();
    let db = f.ctx.home.join("state_5.sqlite");
    let connection = rusqlite::Connection::open(&db).unwrap();
    connection.execute_batch("CREATE TABLE threads(id TEXT PRIMARY KEY,model_provider TEXT,rollout_path TEXT,model TEXT,archived INTEGER,source TEXT);").unwrap();
    connection
        .execute(
            "INSERT INTO threads VALUES('owned','atelier',?1,'gpt-fixture',0,'cli')",
            [path.to_string_lossy().as_ref()],
        )
        .unwrap();
    drop(connection);
    let before = [
        fs::read(f.ctx.config()).unwrap(),
        fs::read(&db).unwrap(),
        fs::read(&path).unwrap(),
    ];
    let error = preview(&f.ctx, "all")
        .err()
        .expect("unplanned target must block preview");
    assert!(error.contains("Atelier受管线程"), "{error}");
    assert_eq!(fs::read(f.ctx.config()).unwrap(), before[0]);
    assert_eq!(fs::read(db).unwrap(), before[1]);
    assert_eq!(fs::read(path).unwrap(), before[2]);
    assert!(!router_intent::path(&f.ctx).exists());
    assert!(!f
        .ctx
        .app_root
        .join("maintenance/diagnostic-backups")
        .exists());
    assert!(f.runner.calls.lock().unwrap().is_empty());
}

#[test]
fn confirmation_rechecks_unplanned_provider_rows_before_process_or_file_changes() {
    let f = Fixture::new("model_provider='openai'\nmodel_reasoning_effort=''\n[model_providers.atelier]\nbase_url='http://127.0.0.1:18910/v1'\nwire_api='responses'\n");
    let reviewed = preview(&f.ctx, "all").unwrap();
    let path = f.ctx.home.join("new-owned.jsonl");
    fs::write(&path, format!("{}\n", serde_json::json!({"type":"session_meta","payload":{"id":"new-owned","model_provider":"atelier"}}))).unwrap();
    let db = f.ctx.home.join("state_5.sqlite");
    let connection = rusqlite::Connection::open(&db).unwrap();
    connection.execute_batch("CREATE TABLE threads(id TEXT PRIMARY KEY,model_provider TEXT,rollout_path TEXT,model TEXT,archived INTEGER,source TEXT);").unwrap();
    connection
        .execute(
            "INSERT INTO threads VALUES('new-owned','atelier',?1,'gpt-fixture',0,'cli')",
            [path.to_string_lossy().as_ref()],
        )
        .unwrap();
    drop(connection);
    let config = fs::read(f.ctx.config()).unwrap();
    let original_db = fs::read(&db).unwrap();
    assert!(f.apply(reviewed).is_err());
    assert_eq!(fs::read(f.ctx.config()).unwrap(), config);
    assert_eq!(fs::read(db).unwrap(), original_db);
    assert!(!router_intent::path(&f.ctx).exists());
    assert!(!f
        .ctx
        .app_root
        .join("maintenance/diagnostic-backups")
        .exists());
    assert!(f.runner.calls.lock().unwrap().is_empty());
}

#[test]
fn all_batch_rejects_new_auth_candidate_before_any_native_write() {
    let f = Fixture::new("model_provider='openai'\n");
    let backups = f.ctx.home.join("accounts/backups");
    fs::create_dir_all(&backups).unwrap();
    fs::write(
        backups.join("approved.json"),
        br#"{"OPENAI_API_KEY":"sk-synthetic-before"}"#,
    )
    .unwrap();
    fs::write(f.ctx.home.join("auth.json"), b"{").unwrap();
    let reviewed = preview(&f.ctx, "all").unwrap();
    assert!(reviewed.batch.is_some());
    fs::write(
        backups.join("new.json"),
        br#"{"OPENAI_API_KEY":"sk-synthetic-after"}"#,
    )
    .unwrap();
    assert!(f.apply(reviewed).is_err());
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), b"{");
    assert!(!router_intent::path(&f.ctx).exists());
    assert!(!f
        .runner
        .calls
        .lock()
        .unwrap()
        .iter()
        .any(|op| matches!(op, Operation::QuitApp | Operation::OpenBundle(_))));
}
