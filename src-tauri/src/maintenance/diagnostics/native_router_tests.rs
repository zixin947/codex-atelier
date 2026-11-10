use super::*;
use crate::gateway::types::{GatewayModel, Profile, Settings};
use serde_json::json;
use std::{
    net::TcpListener,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
struct Fixture {
    root: PathBuf,
    ctx: Context,
    listener: TcpListener,
    source: native_catalog::Source,
}
impl Fixture {
    fn new() -> Self {
        let root =
            std::env::temp_dir().join(format!("atelier-native-router-{}", uuid::Uuid::new_v4()));
        let home = root.join("codex");
        let app = root.join("atelier");
        private_dir(&home).unwrap();
        private_dir(&app.join("gateway/credentials")).unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let ctx = Context {
            home,
            app_root: app.clone(),
            desktop_running: false,
            runtime: None,
            official_catalog: None,
            proxy_handle: Some(ProxyHandle {
                port,
                alive: Arc::new(AtomicBool::new(true)),
            }),
            working_directory: None,
            sqlite_home_env: None,
        };
        let source = native_catalog::Source::Gateway {
            catalog_path: app.join("gateway/catalog.json"),
            catalog_root: ctx.home.clone(),
            bundle: None,
            route_enabled: true,
        };
        let id = uuid::Uuid::new_v4().to_string();
        let profile = Profile {
            id: id.clone(),
            kind: ProfileKind::ApiKey,
            name: "synthetic".into(),
            email: None,
            enabled: true,
            priority: 0,
            base_url: "https://upstream.invalid/v1".into(),
            auth_status: "warning".into(),
            models: vec![GatewayModel {
                id: "raw-fixture".into(),
                name: "Fixture".into(),
            }],
            cooldown_until: None,
            last_error: None,
            requests: 0,
            input_tokens: 0,
            output_tokens: 0,
            created_at: 0,
        };
        let catalog = Catalog {
            revision: "persisted-not-CAS".into(),
            host_id: "fixture-host".into(),
            profiles: vec![profile],
            settings: Settings {
                port,
                preferred_profile_id: Some(id.clone()),
                model: "raw-fixture".into(),
                ..Default::default()
            },
        };
        atomic_private(source_path(&source), &serde_json::to_vec(&catalog).unwrap()).unwrap();
        atomic_private(
            &app.join("gateway/credentials").join(format!("{id}.json")),
            &serde_json::to_vec(&Credential {
                api_key: Some("fixture-key-not-real".into()),
                ..Default::default()
            })
            .unwrap(),
        )
        .unwrap();
        fs::write(ctx.config(),format!("# keep fixture comment\nmodel_provider='atelier'\nmodel='raw-fixture'\nmodel_catalog_json={:?}\n[model_providers.atelier]\nbase_url='http://127.0.0.1:{port}/v1'\nwire_api='responses'\nenv_key='ATELIER_GATEWAY_TOKEN'\nrequires_openai_auth=false\nsupports_websockets=false\n[unrelated]\nkeep='literal bytes'\n",ctx.home.join(catalog::CATALOG_FILE).to_str().unwrap())).unwrap();
        let f = Self {
            root,
            ctx,
            listener,
            source,
        };
        native_catalog::apply(native_catalog::prepare(&f.source).unwrap()).unwrap();
        f
    }
    fn catalog(&self) -> Catalog {
        serde_json::from_slice(&fs::read(source_path(&self.source)).unwrap()).unwrap()
    }
    fn save(&self, c: &Catalog) {
        atomic_private(source_path(&self.source), &serde_json::to_vec(c).unwrap()).unwrap();
    }
    fn native(&self, on: bool) -> native_catalog::Source {
        let path = native_catalog::native_state_path(&self.ctx.home);
        private_dir(path.parent().unwrap()).unwrap();
        let mut state = RelayStateDocument::native_default().as_value().clone();
        state["codexRouterEnabled"] = json!(on);
        state["proxy"] = json!({"running":true,"port":self.listener.local_addr().unwrap().port(),"baseUrl":"","codexBaseUrl":""});
        atomic_private(&path, &serde_json::to_vec(&state).unwrap()).unwrap();
        native_catalog::Source::Native {
            state_path: path,
            catalog_root: self.ctx.home.join("codexmate/relay"),
            bundle: None,
        }
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
#[test]
fn actual_gateway_raw_inventory_credentials_config_and_tcp_satisfy_contract() {
    let f = Fixture::new();
    let state = snapshot(&f.ctx, &f.source).unwrap();
    assert!(matches!(state, Snapshot::Gateway { .. }));
    let report = verify(&f.ctx, &state, Depth::Full).unwrap();
    assert!(report.issues.is_empty(), "{:?}", report.issues);
    assert_eq!(diagnose(&f.ctx, &f.source).status, "ok");
}
#[test]
fn empty_preferred_profile_and_missing_credentials_do_not_turn_empty_catalog_green() {
    let f = Fixture::new();
    let mut c = f.catalog();
    c.settings.preferred_profile_id = None;
    f.save(&c);
    let report = verify(&f.ctx, &snapshot(&f.ctx, &f.source).unwrap(), Depth::Full).unwrap();
    assert!(report.issues.iter().any(|i| i.dim == InvariantDim::Auth));
    assert_eq!(diagnose(&f.ctx, &f.source).status, "error");
}
#[test]
fn stale_profile_source_and_dead_runtime_are_actual_issues() {
    let mut f = Fixture::new();
    let state = snapshot(&f.ctx, &f.source).unwrap();
    let mut c = f.catalog();
    c.settings.model = "different".into();
    f.save(&c);
    f.ctx
        .proxy_handle
        .as_ref()
        .unwrap()
        .alive
        .store(false, Ordering::SeqCst);
    let report = verify(&f.ctx, &state, Depth::Full).unwrap();
    assert!(report
        .issues
        .iter()
        .any(|i| i.message.contains("snapshot后变化")));
    assert!(report.issues.iter().any(|i| i.dim == InvariantDim::Proxy));
    f.ctx.proxy_handle = None;
    assert!(
        verify(&f.ctx, &snapshot(&f.ctx, &f.source).unwrap(), Depth::Full)
            .unwrap()
            .issues
            .iter()
            .any(|i| i.dim == InvariantDim::Proxy)
    );
}
#[test]
fn gateway_credentials_are_checked_from_private_file_and_never_from_codex_auth() {
    let f = Fixture::new();
    let c = f.catalog();
    let id = &c.profiles[0].id;
    fs::remove_file(
        f.ctx
            .app_root
            .join("gateway/credentials")
            .join(format!("{id}.json")),
    )
    .unwrap();
    fs::write(
        f.ctx.home.join("auth.json"),
        b"{\"OPENAI_API_KEY\":\"not-gateway-credential\"}",
    )
    .unwrap();
    let report = verify(&f.ctx, &snapshot(&f.ctx, &f.source).unwrap(), Depth::Full).unwrap();
    assert!(report.issues.iter().any(|i| i.dim == InvariantDim::Auth));
}
#[test]
fn native_source_never_borrows_gateway_handle_or_gateway_json_defaults() {
    let f = Fixture::new();
    let source = f.native(true);
    fs::write(f.ctx.config(), "model_provider='aimai1'\n").unwrap();
    let report = verify(&f.ctx, &snapshot(&f.ctx, &source).unwrap(), Depth::Full).unwrap();
    assert!(report.issues.iter().any(|i| i.dim == InvariantDim::Proxy));
    let fake = native_catalog::Source::Native {
        state_path: source_path(&f.source).to_owned(),
        catalog_root: f.ctx.home.clone(),
        bundle: None,
    };
    assert!(snapshot(&f.ctx, &fake).is_err());
    let batch = super::super::native_coordinator::prepare_batch(
        &f.ctx,
        &["config_stale".into()],
        &f.root.join("native-owner-batch"),
    )
    .unwrap();
    assert!(prepare_bound_batch_resync_from_source(
        &f.ctx,
        &source,
        TargetState::RouterOn,
        &f.root.join("native-borrowed-owner"),
        f.ctx.proxy_handle.clone(),
        &batch,
    )
    .is_err());
    assert!(!f.root.join("native-borrowed-owner").exists());
    assert!(prepare_bound_batch_resync_from_source(
        &f.ctx,
        &source,
        TargetState::RouterOn,
        &f.root.join("native-missing-owner"),
        None,
        &batch,
    )
    .is_err());
    assert!(!f.root.join("native-missing-owner").exists());
}
#[test]
fn reviewed_gateway_reconcile_preserves_comments_backups_and_rechecks_real_target() {
    let f = Fixture::new();
    let before = fs::read(f.ctx.config()).unwrap();
    let text = String::from_utf8(before.clone())
        .unwrap()
        .replace("wire_api='responses'", "wire_api='chat'");
    fs::write(f.ctx.config(), text.as_bytes()).unwrap();
    let plan = prepare_reconcile(&f.ctx, &f.source, &f.root.join("reviewed-backup")).unwrap();
    assert!(preview_lines(&plan)
        .iter()
        .any(|l| l.contains("Atelier Responses")));
    let result = apply(&f.ctx, &plan).unwrap();
    assert_eq!(result.failure, None);
    assert!(
        result.verification.issues.is_empty(),
        "{:?}",
        result.verification.issues
    );
    assert_eq!(result.completed, vec!["catalog", "config"]);
    let after = fs::read_to_string(f.ctx.config()).unwrap();
    assert!(after.contains("# keep fixture comment"));
    assert!(after.contains("keep='literal bytes'"));
    let backup: serde_json::Value = serde_json::from_slice(
        &fs::read(f.root.join("reviewed-backup/router-config-original.json")).unwrap(),
    )
    .unwrap();
    use base64::Engine;
    assert_eq!(
        base64::engine::general_purpose::STANDARD
            .decode(backup["beforeBase64"].as_str().unwrap())
            .unwrap(),
        text.as_bytes()
    );
}
#[test]
fn source_cas_rejects_before_creating_review_backup_or_touching_config() {
    let f = Fixture::new();
    let root = f.root.join("stale-review");
    let plan = prepare_reconcile(&f.ctx, &f.source, &root).unwrap();
    let before = fs::read(f.ctx.config()).unwrap();
    let mut c = f.catalog();
    c.settings.port += 1;
    f.save(&c);
    assert!(apply(&f.ctx, &plan).is_err());
    assert_eq!(fs::read(f.ctx.config()).unwrap(), before);
    assert!(!root.exists());
}
#[test]
fn malformed_provider_table_and_foreign_provider_are_not_rewritten() {
    let f = Fixture::new();
    fs::write(
        f.ctx.config(),
        "model_provider='custom'\n[model_providers.custom]\nbase_url='https://custom.invalid'\n",
    )
    .unwrap();
    let before = fs::read(f.ctx.config()).unwrap();
    assert!(prepare_reconcile_for_target(
        &f.ctx,
        &f.source,
        TargetState::RouterOn,
        &f.root.join("foreign-backup")
    )
    .is_err());
    assert_eq!(fs::read(f.ctx.config()).unwrap(), before);
}
#[test]
fn gateway_running_does_not_override_observed_off_intent_or_explicit_target() {
    let f = Fixture::new();
    fs::write(f.ctx.config(), "model_provider='openai'\n").unwrap();
    let state = snapshot(&f.ctx, &f.source).unwrap();
    assert_eq!(state.target(), TargetState::NativeOff);
    assert!(
        verify_target(&f.ctx, &state, TargetState::NativeOff, Depth::Full)
            .unwrap()
            .issues
            .is_empty()
    );
    assert!(
        !verify_target(&f.ctx, &state, TargetState::RouterOn, Depth::Full)
            .unwrap()
            .issues
            .is_empty()
    );
    assert!(!super::super::router_intent::path(&f.ctx).exists());
}
#[test]
fn explicitly_reviewed_off_reconcile_checks_off_without_rewriting_snapshot() {
    let f = Fixture::new();
    let source_before = fs::read(source_path(&f.source)).unwrap();
    let plan = prepare_reconcile_for_target(
        &f.ctx,
        &f.source,
        TargetState::NativeOff,
        &f.root.join("off-review"),
    )
    .unwrap();
    let result = apply(&f.ctx, &plan).unwrap();
    assert_eq!(result.failure, None);
    assert!(
        result.verification.issues.is_empty(),
        "{:?}",
        result.verification.issues
    );
    assert_eq!(fs::read(source_path(&f.source)).unwrap(), source_before);
    assert_eq!(result.completed, vec!["config"]);
    assert_eq!(
        snapshot(&f.ctx, &f.source).unwrap().target(),
        TargetState::NativeOff
    );
}
#[test]
fn actual_gateway_thread_database_and_rollout_provider_are_checked() {
    let f = Fixture::new();
    let db = f.ctx.home.join("state_5.sqlite");
    let rollout = f.ctx.home.join("fixture.jsonl");
    fs::write(&rollout,format!("{}\n",json!({"type":"session_meta","payload":{"id":"fixture","model_provider":"atelier","model":"raw-fixture"}}))).unwrap();
    let c = rusqlite::Connection::open(&db).unwrap();
    c.execute_batch("CREATE TABLE threads(id TEXT,rollout_path TEXT,model_provider TEXT,model TEXT,source TEXT,archived INTEGER);").unwrap();
    c.execute(
        "INSERT INTO threads VALUES('fixture',?1,'atelier','raw-fixture','cli',0)",
        [rollout.to_str().unwrap()],
    )
    .unwrap();
    drop(c);
    let state = snapshot(&f.ctx, &f.source).unwrap();
    assert!(
        verify_target(&f.ctx, &state, TargetState::RouterOn, Depth::Full)
            .unwrap()
            .issues
            .is_empty()
    );
    fs::write(&rollout,format!("{}\n",json!({"type":"session_meta","payload":{"id":"fixture","model_provider":"openai","model":"raw-fixture"}}))).unwrap();
    let report = verify_target(&f.ctx, &state, TargetState::RouterOn, Depth::Full).unwrap();
    assert!(report.issues.iter().any(|i| i.dim == InvariantDim::Thread));
}
#[test]
fn malformed_provider_table_returns_error_instead_of_panicking() {
    let f = Fixture::new();
    fs::write(
        f.ctx.config(),
        "model_provider='atelier'\nmodel_providers=3\n",
    )
    .unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        prepare_reconcile_for_target(
            &f.ctx,
            &f.source,
            TargetState::RouterOn,
            &f.root.join("malformed"),
        )
    }));
    assert!(result.is_ok());
    assert!(result.unwrap().is_err());
}
#[test]
fn expired_oauth_with_refresh_material_is_warning_and_does_not_block_config_repair() {
    let f = Fixture::new();
    let mut c = f.catalog();
    c.profiles[0].kind = ProfileKind::Chatgpt;
    c.profiles[0].auth_status = "ready".into();
    f.save(&c);
    let id = &c.profiles[0].id;
    atomic_private(
        &f.ctx
            .app_root
            .join("gateway/credentials")
            .join(format!("{id}.json")),
        &serde_json::to_vec(&Credential {
            access_token: Some("expired-fixture".into()),
            expires_at: Some(1),
            refresh_token: Some("refresh-fixture".into()),
            client_id: Some("fixture-client".into()),
            ..Default::default()
        })
        .unwrap(),
    )
    .unwrap();
    let state = snapshot(&f.ctx, &f.source).unwrap();
    assert!(
        verify_target(&f.ctx, &state, TargetState::RouterOn, Depth::Full)
            .unwrap()
            .issues
            .is_empty()
    );
    assert!(!availability_warnings(&state).is_empty());
    let text = fs::read_to_string(f.ctx.config())
        .unwrap()
        .replace("wire_api='responses'", "wire_api='bad'");
    fs::write(f.ctx.config(), text).unwrap();
    let plan = prepare_reconcile_for_target(
        &f.ctx,
        &f.source,
        TargetState::RouterOn,
        &f.root.join("oauth-refresh-review"),
    )
    .unwrap();
    let outcome = apply(&f.ctx, &plan).unwrap();
    assert_eq!(outcome.failure, None);
    assert!(outcome.verification.issues.is_empty());
    assert!(!outcome.warnings.is_empty());
}
#[test]
fn native_off_review_restores_config_and_preserves_current_login_while_clearing_residue() {
    let f = Fixture::new();
    let source = f.native(false);
    let relay = f.ctx.home.join("codexmate/relay");
    fs::write(f.ctx.config(),"model_provider='aimai1'\n# user comment\n[model_providers.aimai1]\nbase_url='http://127.0.0.1:1/codex/router/v1'\n").unwrap();
    let auth = br#"{"OPENAI_API_KEY":"sk-synthetic-current-user-A"}"#;
    fs::write(f.ctx.home.join("auth.json"), auth).unwrap();
    atomic_private(
        &relay.join("router-unlock-auth-backup.json"),
        br#"{"OPENAI_API_KEY":"sk-synthetic-different-backup-B"}"#,
    )
    .unwrap();
    atomic_private(
        &relay.join("router-unlock-auth-marker.json"),
        br#"{"backedUpPreviousAuth":true}"#,
    )
    .unwrap();
    atomic_private(&relay.join("router-takeover-backup.json"),&serde_json::to_vec(&json!({"version":1,"createdAtMs":1,"topLevelLines":["model='original-fixture'"],"originalConfig":"model='original-fixture'\n"})).unwrap()).unwrap();
    let plan = prepare_reconcile_for_target(
        &f.ctx,
        &source,
        TargetState::NativeOff,
        &f.root.join("native-off-review"),
    )
    .unwrap();
    let outcome = apply(&f.ctx, &plan).unwrap();
    assert_eq!(outcome.failure, None);
    assert!(
        outcome.verification.issues.is_empty(),
        "{:?}",
        outcome.verification.issues
    );
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), auth);
    assert!(!relay.join("router-unlock-auth-backup.json").exists());
    assert!(!relay.join("router-takeover-backup.json").exists());
    assert!(fs::read_to_string(f.ctx.config())
        .unwrap()
        .contains("original-fixture"));
}
#[test]
fn readonly_source_getters_return_stamps_and_intent_provider_drift_is_reported() {
    let f = Fixture::new();
    let state = snapshot(&f.ctx, &f.source).unwrap();
    assert_eq!(state.source_revision().len(), 64);
    assert_eq!(state.source_dependency().0, source_path(&f.source));
    assert!(state.gateway_catalog().is_some());
    assert!(state.native_document().is_none());
    assert!(state.intent_dependency(&f.ctx).is_some());
    let intent = super::super::router_intent::IntentRecord {
        schema_version: 1,
        target: TargetState::RouterOn,
        provider: Some("atelier_expected".into()),
        operation_id: None,
        catalog_revision: None,
        updated_at_ms: 1,
    };
    atomic_private(
        &super::super::router_intent::path(&f.ctx),
        &serde_json::to_vec(&intent).unwrap(),
    )
    .unwrap();
    let state = snapshot(&f.ctx, &f.source).unwrap();
    assert!(
        verify_target(&f.ctx, &state, TargetState::RouterOn, Depth::Full)
            .unwrap()
            .issues
            .iter()
            .any(|i| i.message.contains("atelier_expected"))
    );
}
#[test]
fn real_gateway_host_transition_publishes_intent_without_creating_native_state() {
    use atelier_maintenance_core::{
        manager::{
            CompletedRepair, FixResult, HostState, ManagerError, Processes, QuietRepairRequest,
            RepairPaths, RepairTransitionPolicy, TransitionOutcome,
        },
        process::{CommandOutput, Operation, ProcessContext, ProcessPolicy, Runner},
        router_transition::run_host_repair,
        transition_journal::TransitionJournal,
    };
    use std::{sync::atomic::AtomicU64, time::Duration};
    struct Host<'a>(&'a Fixture);
    impl HostState for Host<'_> {
        type Error = io::Error;
        type Snapshot = Snapshot;
        fn snapshot(&self) -> Snapshot {
            snapshot(&self.0.ctx, &self.0.source).unwrap()
        }
        fn initialization_error(&self) -> Option<&str> {
            None
        }
        fn checked_recovery(
            &self,
            p: &RepairPaths,
        ) -> std::result::Result<(), ManagerError<io::Error>> {
            atelier_maintenance_core::transition_journal::JournalStore::new(
                &p.verification.relay_root,
            )
            .read_checked()
            .map(|_| ())
            .map_err(ManagerError::Io)
        }
        fn preflight(
            &self,
            p: &RepairPaths,
            _: &Snapshot,
        ) -> std::result::Result<(), ManagerError<io::Error>> {
            fs::create_dir_all(&p.verification.relay_root).map_err(ManagerError::Io)
        }
        fn read_journal(
            &self,
            p: &RepairPaths,
        ) -> std::result::Result<Option<TransitionJournal>, ManagerError<io::Error>> {
            atelier_maintenance_core::transition_journal::JournalStore::new(
                &p.verification.relay_root,
            )
            .read_checked()
            .map_err(ManagerError::Io)
        }
        fn verify_target(
            &self,
            _: &RepairPaths,
            target: TargetState,
            depth: Depth,
        ) -> VerificationReport {
            verify_target(&self.0.ctx, &self.snapshot(), target, depth).unwrap()
        }
    }
    struct Fake {
        clock: AtomicU64,
        owner: std::thread::ThreadId,
    }
    impl Runner for Fake {
        fn output(&self, o: &Operation) -> io::Result<CommandOutput> {
            Ok(CommandOutput {
                success: true,
                stdout: if *o == Operation::IsAppRunning {
                    b"false".to_vec()
                } else {
                    vec![]
                },
                stderr: vec![],
            })
        }
        fn signal_kill(&self, _: u32) -> io::Result<()> {
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
            Duration::from_millis(self.clock.load(Ordering::SeqCst))
        }
        fn sleep(&self, d: Duration) {
            if std::thread::current().id() == self.owner {
                self.clock.fetch_add(d.as_millis() as u64, Ordering::SeqCst);
            }
            std::thread::yield_now();
        }
    }
    let f = Fixture::new();
    let reviewed = prepare_reconcile_for_target(
        &f.ctx,
        &f.source,
        TargetState::RouterOn,
        &f.root.join("host-review"),
    )
    .unwrap();
    let intent = super::super::router_intent::prepare(&f.ctx).unwrap();
    let host = Host(&f);
    let original = host.snapshot();
    let mut verification_paths = paths(&f.ctx, &original).unwrap();
    verification_paths.relay_root = f.ctx.app_root.join("gateway");
    let paths = RepairPaths {
        state_path: super::super::router_intent::path(&f.ctx),
        verification: verification_paths,
    };
    let runner = Arc::new(Fake {
        clock: AtomicU64::new(0),
        owner: std::thread::current().id(),
    });
    let process = ProcessContext {
        codex_home: f.ctx.home.clone(),
        user_home: None,
        sqlite_home_env: None,
        cwd: None,
    };
    let request = QuietRepairRequest {
        paths: &paths,
        source: &original,
        original: &original,
        selected: "router_postcondition",
        prepared_catalog: None::<&()>,
        include_missing_rollouts: false,
        reason: "synthetic Gateway Host",
        policy: RepairTransitionPolicy::Strict,
        writer_policy: ProcessPolicy::ForceCloseAll,
        graceful_timeout: Duration::from_secs(5),
    };
    let result = run_host_repair(
        request,
        Processes {
            runner,
            context: &process,
        },
        &host,
        &mut |_| {
            let outcome =
                apply(&f.ctx, &reviewed).map_err(|e| ManagerError::Io(io::Error::other(e)))?;
            if let Some(e) = outcome.failure {
                return Err(ManagerError::Operation(e));
            }
            Ok(CompletedRepair::from_parts(
                FixResult {
                    fixed_ids: outcome.completed,
                    failed_ids: vec![],
                    details: vec![],
                },
                None::<()>,
                outcome.warnings,
            ))
        },
        &mut |_, id| {
            super::super::router_intent::commit_after_approved_config(
                &f.ctx,
                &intent,
                reviewed.config_after(),
                id,
                1,
            )
            .map(|p| p.publication)
            .map_err(|e| ManagerError::Io(io::Error::other(e)))
        },
    );
    assert!(matches!(result, TransitionOutcome::Verified { .. }));
    assert!(super::super::router_intent::read(&f.ctx)
        .unwrap()
        .unwrap()
        .operation_id
        .is_some());
    assert!(!f.ctx.home.join("codexmate/relay/state.json").exists());
    assert!(!f.ctx.home.join("accounts").exists());
}
#[test]
fn manager_guard_reuses_actual_dual_locks_and_validates_scope() {
    let f = Fixture::new();
    let plan = prepare_reconcile_for_target(
        &f.ctx,
        &f.source,
        TargetState::RouterOn,
        &f.root.join("guard-review"),
    )
    .unwrap();
    let guard = super::super::repair::acquire(&f.ctx).unwrap();
    assert!(apply(&f.ctx, &plan).is_err());
    let outcome = apply_under_manager(&f.ctx, &plan, &guard).unwrap();
    assert_eq!(outcome.failure, None);
    assert!(outcome.verification.issues.is_empty());
    assert_eq!(plan.config_effect().0, f.ctx.config());
    assert!(plan
        .approved_dependencies()
        .iter()
        .any(|(p, _)| p == source_path(&f.source)));
    let mut other = f.ctx.clone();
    other.home = f.root.join("other-home");
    assert!(apply_under_manager(&other, &plan, &guard).is_err());
}
#[test]
fn gateway_off_preserves_codex_auth_native_residue_and_both_catalog_files() {
    let f = Fixture::new();
    let auth = br#"{"OPENAI_API_KEY":"sk-synthetic-user-kept"}"#;
    fs::write(f.ctx.home.join("auth.json"), auth).unwrap();
    let relay = f.ctx.home.join("codexmate/relay");
    private_dir(&relay).unwrap();
    let marker = relay.join("router-unlock-auth-marker.json");
    atomic_private(&marker, br#"{"backedUpPreviousAuth":true}"#).unwrap();
    let source_before = fs::read(source_path(&f.source)).unwrap();
    let generated_before = fs::read(f.ctx.home.join(catalog::CATALOG_FILE)).unwrap();
    let secret_path = f
        .ctx
        .app_root
        .join("gateway/credentials")
        .join(format!("{}.json", f.catalog().profiles[0].id));
    let secret_before = fs::read(&secret_path).unwrap();
    let plan = prepare_reconcile_for_target(
        &f.ctx,
        &f.source,
        TargetState::NativeOff,
        &f.root.join("preserve-off"),
    )
    .unwrap();
    let outcome = apply(&f.ctx, &plan).unwrap();
    assert_eq!(outcome.failure, None);
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), auth);
    assert!(marker.exists());
    assert_eq!(fs::read(source_path(&f.source)).unwrap(), source_before);
    assert_eq!(
        fs::read(f.ctx.home.join(catalog::CATALOG_FILE)).unwrap(),
        generated_before
    );
    assert_eq!(fs::read(secret_path).unwrap(), secret_before);
}
#[test]
fn reviewed_gateway_rebase_preserves_original_intent_when_primary_removes_profile() {
    let f = Fixture::new();
    let text = fs::read_to_string(f.ctx.config()).unwrap().replace(
        "model_provider='atelier'",
        "model_provider='openai'\nprofile='route-profile'",
    );
    let text = format!(
        "{text}\n[profiles.route-profile]\nmodel_provider='atelier'\nmodel='raw-fixture'\n"
    );
    fs::write(f.ctx.config(), &text).unwrap();
    let reviewed = prepare_reconcile_for_target(
        &f.ctx,
        &f.source,
        TargetState::RouterOn,
        &f.root.join("initial-review"),
    )
    .unwrap();
    let primary = config::fix_config_profile_conflict_text(&text, &f.ctx.home).after;
    let expected = compose_config_after(
        &f.ctx,
        &reviewed.snapshot,
        TargetState::RouterOn,
        Some(primary.as_bytes()),
    )
    .unwrap();
    fs::write(f.ctx.config(), primary.as_bytes()).unwrap();
    assert_eq!(
        snapshot(&f.ctx, &f.source).unwrap().target(),
        TargetState::NativeOff
    );
    let rebased = rebase_after_approved_config(
        &f.ctx,
        &reviewed,
        Some(primary.as_bytes()),
        &f.root.join("followup-review"),
    )
    .unwrap();
    assert_eq!(rebased.config_after, expected);
    assert_eq!(rebased.snapshot.target(), TargetState::RouterOn);
    let guard = super::super::repair::acquire(&f.ctx).unwrap();
    let outcome = apply_under_manager(&f.ctx, &rebased, &guard).unwrap();
    assert_eq!(outcome.failure, None);
    assert!(
        outcome.verification.issues.is_empty(),
        "{:?}",
        outcome.verification.issues
    );
    assert!(fs::read_to_string(f.ctx.config())
        .unwrap()
        .contains("keep='literal bytes'"));
}
#[test]
fn gateway_rebase_rejects_other_changed_dependencies_and_unapproved_config() {
    let f = Fixture::new();
    let reviewed = prepare_reconcile(&f.ctx, &f.source, &f.root.join("initial-review")).unwrap();
    let primary = fs::read(f.ctx.config()).unwrap();
    fs::write(f.ctx.config(), b"model_provider='openai'\n").unwrap();
    assert!(rebase_after_approved_config(
        &f.ctx,
        &reviewed,
        Some(&primary),
        &f.root.join("followup")
    )
    .is_err());
    fs::write(f.ctx.config(), &primary).unwrap();
    let mut c = f.catalog();
    c.settings.model = "different".into();
    f.save(&c);
    assert!(rebase_after_approved_config(
        &f.ctx,
        &reviewed,
        Some(&primary),
        &f.root.join("followup")
    )
    .is_err());
}
#[test]
fn native_rebase_refuses_primary_config_effect_without_losing_original_plan() {
    let f = Fixture::new();
    let source = f.native(false);
    fs::write(f.ctx.config(), "model_provider='aimai1'\n").unwrap();
    let reviewed = prepare_reconcile_for_target(
        &f.ctx,
        &source,
        TargetState::NativeOff,
        &f.root.join("native-initial"),
    )
    .unwrap();
    let original = reviewed.config_before.clone();
    assert!(rebase_after_approved_config(
        &f.ctx,
        &reviewed,
        Some(b"model_provider='openai'\n"),
        &f.root.join("native-followup")
    )
    .is_err());
    assert_eq!(reviewed.config_before, original);
    assert_eq!(fs::read(f.ctx.config()).unwrap(), original.unwrap());
}
#[test]
fn diagnostic_postcondition_reads_own_checked_journal_before_marking_healthy() {
    let f = Fixture::new();
    let state = snapshot(&f.ctx, &f.source).unwrap();
    let root = journal_root(&f.ctx, &state);
    let journal = atelier_maintenance_core::transition_journal::JournalStore::new(&root);
    journal
        .begin_commit(TargetState::RouterOn, Some("synthetic pending"), 1)
        .unwrap();
    let item = diagnose(&f.ctx, &f.source);
    assert_eq!(item.status, "error");
    assert!(item.detail.contains("ROUTER_COMMIT_RECOVERY_REQUIRED"));
    fs::write(journal.path(), b"malformed").unwrap();
    let item = diagnose(&f.ctx, &f.source);
    assert_eq!(item.status, "error");
    assert!(item.detail.contains("恢复记录不可读"));
}
#[test]
fn partial_native_config_publication_auth_lock_failure_can_resume_exact_approved_effects() {
    let f = Fixture::new();
    let source = f.native(false);
    let relay = f.ctx.home.join("codexmate/relay");
    fs::write(
        f.ctx.config(),
        "model_provider='aimai1'\n# preserve original\n",
    )
    .unwrap();
    let original = fs::read(f.ctx.config()).unwrap();
    let auth = br#"{"OPENAI_API_KEY":"sk-synthetic-current-A"}"#;
    fs::write(f.ctx.home.join("auth.json"), auth).unwrap();
    atomic_private(
        &relay.join("router-unlock-auth-backup.json"),
        br#"{"OPENAI_API_KEY":"sk-synthetic-other-B"}"#,
    )
    .unwrap();
    atomic_private(
        &relay.join("router-unlock-auth-marker.json"),
        br#"{"backedUpPreviousAuth":true}"#,
    )
    .unwrap();
    let reviewed = prepare_reconcile_for_target(
        &f.ctx,
        &source,
        TargetState::NativeOff,
        &f.root.join("initial-partial"),
    )
    .unwrap();
    let guard = super::super::repair::acquire(&f.ctx).unwrap();
    let lock = f.ctx.home.join("accounts/.live-auth.lock");
    let partial = apply_under_manager_using(&f.ctx, &reviewed, &guard, &mut |path, bytes| {
        let p = publication::write_atomic_private_report(path, bytes)?;
        fs::create_dir_all(lock.parent().unwrap())?;
        fs::create_dir(&lock)?;
        Ok(p)
    })
    .unwrap();
    assert!(partial.failure.is_some());
    assert!(partial.completed.iter().any(|s| s == "config"));
    assert_eq!(
        fs::read(f.ctx.config()).unwrap(),
        reviewed.config_after.clone().unwrap()
    );
    assert!(relay.join("router-unlock-auth-marker.json").exists());
    let original_backup =
        fs::read(f.root.join("initial-partial/router-config-original.json")).unwrap();
    fs::remove_dir(&lock).unwrap();
    let resumed =
        resume_after_approved_effects(&f.ctx, &reviewed, &f.root.join("resumed-partial")).unwrap();
    assert_eq!(resumed.config_after, reviewed.config_after);
    let result = apply_under_manager(&f.ctx, &resumed, &guard).unwrap();
    assert_eq!(result.failure, None);
    assert!(
        result.verification.issues.is_empty(),
        "{:?}",
        result.verification.issues
    );
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), auth);
    assert!(!relay.join("router-unlock-auth-marker.json").exists());
    assert_eq!(
        fs::read(f.root.join("initial-partial/router-config-original.json")).unwrap(),
        original_backup
    );
    use base64::Engine;
    let manifest: serde_json::Value = serde_json::from_slice(&original_backup).unwrap();
    assert_eq!(
        base64::engine::general_purpose::STANDARD
            .decode(manifest["beforeBase64"].as_str().unwrap())
            .unwrap(),
        original
    );
}
#[test]
fn partial_resume_rejects_external_auth_or_source_change_without_expanding_scope() {
    let f = Fixture::new();
    let source = f.native(false);
    let relay = f.ctx.home.join("codexmate/relay");
    fs::write(f.ctx.config(), "model_provider='aimai1'\n").unwrap();
    let auth = br#"{"OPENAI_API_KEY":"sk-synthetic-current-A"}"#;
    fs::write(f.ctx.home.join("auth.json"), auth).unwrap();
    atomic_private(&relay.join("router-unlock-auth-marker.json"), br#"{}"#).unwrap();
    let reviewed = prepare_reconcile_for_target(
        &f.ctx,
        &source,
        TargetState::NativeOff,
        &f.root.join("initial"),
    )
    .unwrap();
    publication::write_atomic_private_report(&f.ctx.config(), reviewed.config_after().unwrap())
        .unwrap()
        .require_sync()
        .unwrap();
    fs::write(
        f.ctx.home.join("auth.json"),
        br#"{"OPENAI_API_KEY":"sk-unapproved-external-C"}"#,
    )
    .unwrap();
    let root = f.root.join("unapproved-resume");
    assert!(resume_after_approved_effects(&f.ctx, &reviewed, &root).is_err());
    assert!(!root.exists());
    assert!(relay.join("router-unlock-auth-marker.json").exists());
    fs::write(f.ctx.home.join("auth.json"), auth).unwrap();
    let native_path = source_path(&source);
    let mut state: serde_json::Value =
        serde_json::from_slice(&fs::read(native_path).unwrap()).unwrap();
    state["codexRouterNoAccountMode"] = json!(true);
    fs::write(native_path, serde_json::to_vec(&state).unwrap()).unwrap();
    assert!(resume_after_approved_effects(&f.ctx, &reviewed, &root).is_err());
}
#[test]
fn approved_after_bytes_in_wrong_stage_order_are_not_a_resumable_prefix() {
    let f = Fixture::new();
    let source = f.native(false);
    let relay = f.ctx.home.join("codexmate/relay");
    fs::write(f.ctx.config(), "model_provider='aimai1'\n").unwrap();
    fs::write(
        f.ctx.home.join("auth.json"),
        br#"{"OPENAI_API_KEY":"sk-synthetic-A"}"#,
    )
    .unwrap();
    let backup = relay.join("router-unlock-auth-backup.json");
    let marker = relay.join("router-unlock-auth-marker.json");
    let bytes = br#"{"OPENAI_API_KEY":"sk-synthetic-B"}"#;
    atomic_private(&backup, bytes).unwrap();
    atomic_private(&marker, b"{}").unwrap();
    let reviewed = prepare_reconcile_for_target(
        &f.ctx,
        &source,
        TargetState::NativeOff,
        &f.root.join("ordered-initial"),
    )
    .unwrap();
    fs::remove_file(&backup).unwrap();
    assert!(resume_after_approved_effects(&f.ctx, &reviewed, &f.root.join("wrong-order")).is_err());
    atomic_private(&backup, bytes).unwrap();
    publication::write_atomic_private_report(&f.ctx.config(), reviewed.config_after().unwrap())
        .unwrap()
        .require_sync()
        .unwrap();
    fs::remove_file(&marker).unwrap();
    assert!(
        resume_after_approved_effects(&f.ctx, &reviewed, &f.root.join("wrong-within-auth"))
            .is_err()
    );
}
#[test]
fn partial_quarantine_copy_is_pinned_to_approved_path_and_finishes_real_cleanup() {
    let f = Fixture::new();
    let source = f.native(false);
    let relay = f.ctx.home.join("codexmate/relay");
    fs::write(f.ctx.config(), "model_provider='aimai1'\n").unwrap();
    let damaged = relay.join("router-takeover-backup.json");
    let bytes = b"not-valid-json-fixture";
    atomic_private(&damaged, bytes).unwrap();
    let reviewed = prepare_reconcile_for_target(
        &f.ctx,
        &source,
        TargetState::NativeOff,
        &f.root.join("quarantine-initial"),
    )
    .unwrap();
    let copy = reviewed
        .auth
        .iter()
        .flat_map(native_auth::approved_effects)
        .find(|(p, after)| p.to_string_lossy().contains(".damaged-") && after.is_some())
        .unwrap()
        .0;
    publication::write_atomic_private_report(&f.ctx.config(), reviewed.config_after().unwrap())
        .unwrap()
        .require_sync()
        .unwrap();
    atomic_private(&copy, bytes).unwrap();
    let resumed =
        resume_after_approved_effects(&f.ctx, &reviewed, &f.root.join("quarantine-resume"))
            .unwrap();
    let result = apply(&f.ctx, &resumed).unwrap();
    assert_eq!(result.failure, None);
    assert!(!damaged.exists());
    assert_eq!(fs::read(&copy).unwrap(), bytes);
    assert!(result.verification.issues.is_empty());
    let count = fs::read_dir(&relay)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().contains(".damaged-"))
        .count();
    assert_eq!(count, 1);
}
#[test]
fn gateway_catalog_and_config_sync_partial_resume_keep_old_target_provider() {
    let f = Fixture::new();
    fs::remove_file(f.ctx.home.join(catalog::CATALOG_FILE)).unwrap();
    let text = fs::read_to_string(f.ctx.config())
        .unwrap()
        .replace("wire_api='responses'", "wire_api='bad'");
    fs::write(f.ctx.config(), text).unwrap();
    let reviewed = prepare_reconcile_for_target(
        &f.ctx,
        &f.source,
        TargetState::RouterOn,
        &f.root.join("gateway-initial-partial"),
    )
    .unwrap();
    let guard = super::super::repair::acquire(&f.ctx).unwrap();
    let partial = apply_under_manager_using(&f.ctx, &reviewed, &guard, &mut |_, _| {
        Err(io::Error::other("synthetic publication refused"))
    })
    .unwrap();
    assert_eq!(partial.completed, vec!["catalog"]);
    assert!(partial.failure.is_some());
    let next =
        resume_after_approved_effects(&f.ctx, &reviewed, &f.root.join("gateway-catalog-resume"))
            .unwrap();
    assert_eq!(next.config_after, reviewed.config_after);
    let sync_partial = apply_under_manager_using(&f.ctx, &next, &guard, &mut |path, bytes| {
        let mut p = publication::write_atomic_private_report(path, bytes)?;
        p.directory_sync_error = Some(io::Error::other("synthetic sync uncertainty"));
        Ok(p)
    })
    .unwrap();
    assert!(sync_partial.failure.is_some());
    assert!(sync_partial
        .completed
        .iter()
        .any(|s| s == "config_published_sync_unconfirmed"));
    let final_plan =
        resume_after_approved_effects(&f.ctx, &reviewed, &f.root.join("gateway-sync-resume"))
            .unwrap();
    let result = apply_under_manager(&f.ctx, &final_plan, &guard).unwrap();
    assert_eq!(result.failure, None);
    assert!(result.verification.issues.is_empty());
    assert_eq!(final_plan.snapshot.target(), TargetState::RouterOn);
}
fn native_primary_setup(
    f: &Fixture,
) -> (native_catalog::Source, ReviewedPlan, Vec<String>, Vec<u8>) {
    let source = f.native(false);
    let relay = f.ctx.home.join("codexmate/relay");
    let missing = f.root.join("missing-original.json");
    let other_missing = f.root.join("missing-restored.json");
    let text=format!("# original preserved\nmodel_provider='aimai1'\nprofile='conflict'\nmodel_catalog_json={:?}\n[model_providers.aimai1]\nbase_url='http://127.0.0.1:1/codex/router/v1'\n[unrelated]\nkeep='fixture'\n",missing.to_str().unwrap());
    fs::write(f.ctx.config(), &text).unwrap();
    fs::write(
        f.ctx.home.join("auth.json"),
        br#"{"OPENAI_API_KEY":"sk-synthetic-A"}"#,
    )
    .unwrap();
    atomic_private(
        &relay.join("router-unlock-auth-backup.json"),
        br#"{"OPENAI_API_KEY":"sk-synthetic-B"}"#,
    )
    .unwrap();
    atomic_private(
        &relay.join("router-unlock-auth-marker.json"),
        br#"{"backedUpPreviousAuth":true}"#,
    )
    .unwrap();
    let lines = vec![
        "model='restored-fixture'".to_owned(),
        format!("model_catalog_json={:?}", other_missing.to_str().unwrap()),
        "profile='restored-profile'".to_owned(),
    ];
    atomic_private(&relay.join("router-takeover-backup.json"),&serde_json::to_vec(&json!({"version":1,"createdAtMs":1,"topLevelLines":lines,"originalConfig":lines.join("\n")})).unwrap()).unwrap();
    let reviewed = prepare_reconcile_for_target(
        &f.ctx,
        &source,
        TargetState::NativeOff,
        &f.root.join("native-primary-original"),
    )
    .unwrap();
    let ids = vec![
        "config_global_validity".into(),
        "config_profile_conflict".into(),
    ];
    let primary = apply_primary_items(&f.ctx, &text, &ids, &PrimaryMetadata::default())
        .unwrap()
        .into_bytes();
    (source, reviewed, ids, primary)
}
#[test]
fn native_primary_global_profile_composition_does_not_restore_removed_keys() {
    let f = Fixture::new();
    let (source, reviewed, ids, primary) = native_primary_setup(&f);
    let before = fs::read(f.ctx.config()).unwrap();
    let token =
        prepare_native_primary_composition(&f.ctx, &reviewed, &ids, Some(&primary)).unwrap();
    assert_eq!(fs::read(f.ctx.config()).unwrap(), before);
    assert_eq!(token.item_ids(), ids);
    assert!(!token.metadata_dependencies().is_empty());
    let expected: toml::Value =
        toml::from_str(std::str::from_utf8(token.config_after().unwrap()).unwrap()).unwrap();
    assert!(expected.get("profile").is_none());
    assert!(expected.get("model_catalog_json").is_none());
    assert_eq!(expected["model"].as_str(), Some("restored-fixture"));
    assert_eq!(expected["unrelated"]["keep"].as_str(), Some("fixture"));
    publication::write_atomic_private_report(&f.ctx.config(), &primary)
        .unwrap()
        .require_sync()
        .unwrap();
    let next = rebase_native_primary(
        &f.ctx,
        &reviewed,
        &token,
        &f.root.join("native-primary-followup"),
    )
    .unwrap();
    assert_eq!(next.config_after(), token.config_after());
    let result = apply(&f.ctx, &next).unwrap();
    assert_eq!(result.failure, None);
    assert!(
        result.verification.issues.is_empty(),
        "{:?}",
        result.verification.issues
    );
    let actual: toml::Value = toml::from_str(&fs::read_to_string(f.ctx.config()).unwrap()).unwrap();
    assert!(actual.get("model_catalog_json").is_none());
    assert!(actual.get("profile").is_none());
    assert!(actual.get("model_provider").is_none());
    assert!(!source_root(&source)
        .join("router-takeover-backup.json")
        .exists());
    assert_eq!(
        fs::read(f.ctx.home.join("auth.json")).unwrap(),
        br#"{"OPENAI_API_KEY":"sk-synthetic-A"}"#
    );
}
#[test]
fn native_primary_rebase_rejects_metadata_and_auth_source_drift() {
    let f = Fixture::new();
    let (_, reviewed, ids, primary) = native_primary_setup(&f);
    let token =
        prepare_native_primary_composition(&f.ctx, &reviewed, &ids, Some(&primary)).unwrap();
    publication::write_atomic_private_report(&f.ctx.config(), &primary)
        .unwrap()
        .require_sync()
        .unwrap();
    let missing = f.root.join("missing-original.json");
    fs::write(&missing, b"{}").unwrap();
    let backup = f.root.join("native-unapproved-followup");
    assert!(rebase_native_primary(&f.ctx, &reviewed, &token, &backup).is_err());
    assert!(!backup.exists());
    assert_eq!(fs::read(f.ctx.config()).unwrap(), primary);
    fs::remove_file(missing).unwrap();
    fs::write(
        f.ctx.home.join("auth.json"),
        br#"{"OPENAI_API_KEY":"sk-external-C"}"#,
    )
    .unwrap();
    assert!(rebase_native_primary(&f.ctx, &reviewed, &token, &backup).is_err());
}
#[test]
fn native_primary_proof_rejects_unknown_ids_and_arbitrary_primary_bytes() {
    let f = Fixture::new();
    let (_, reviewed, ids, primary) = native_primary_setup(&f);
    let before = fs::read(f.ctx.config()).unwrap();
    assert!(prepare_native_primary_composition(
        &f.ctx,
        &reviewed,
        &["auth_integrity".into()],
        Some(&primary)
    )
    .is_err());
    let forged = [primary.as_slice(), b"# unapproved extra bytes\n"].concat();
    assert!(prepare_native_primary_composition(&f.ctx, &reviewed, &ids, Some(&forged)).is_err());
    assert_eq!(fs::read(f.ctx.config()).unwrap(), before);
}
#[test]
fn native_on_primary_uses_independent_owned_native_proxy_instead_of_gateway_handle() {
    let f = Fixture::new();
    let source = f.native(true);
    let native_listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let native_port = native_listener.local_addr().unwrap().port();
    let native_proxy = ProxyHandle {
        port: native_port,
        alive: Arc::new(AtomicBool::new(true)),
    };
    let text=format!("profile='legacy'\nmodel_catalog_json={:?}\n[model_providers.aimai1]\nbase_url='http://127.0.0.1:1/codex/router/v1'\nwire_api='chat'\nrequires_openai_auth=true\nsupports_websockets=true\n[profiles.legacy]\nmodel_provider='aimai1'\n",f.root.join("missing.json").to_str().unwrap());
    fs::write(f.ctx.config(), &text).unwrap();
    let reviewed = prepare_reconcile_for_target_with_proxy(
        &f.ctx,
        &source,
        TargetState::RouterOn,
        &f.root.join("native-on-initial"),
        Some(native_proxy),
    )
    .unwrap();
    let ids = vec![
        "config_global_validity".into(),
        "config_profile_conflict".into(),
    ];
    let primary = apply_primary_items(&f.ctx, &text, &ids, &PrimaryMetadata::default()).unwrap();
    let token =
        prepare_native_primary_composition(&f.ctx, &reviewed, &ids, Some(primary.as_bytes()))
            .unwrap();
    let final_doc: toml::Value =
        toml::from_str(std::str::from_utf8(token.config_after().unwrap()).unwrap()).unwrap();
    assert_eq!(
        final_doc["model_providers"]["aimai1"]["base_url"].as_str(),
        Some(format!("http://127.0.0.1:{native_port}/codex/router/v1").as_str())
    );
    assert!(!final_doc
        .to_string()
        .contains(&format!(":{}/v1", f.listener.local_addr().unwrap().port())));
    assert!(final_doc.get("model_catalog_json").is_none());
    publication::write_atomic_private_report(&f.ctx.config(), primary.as_bytes())
        .unwrap()
        .require_sync()
        .unwrap();
    let next = rebase_native_primary(
        &f.ctx,
        &reviewed,
        &token,
        &f.root.join("native-on-followup"),
    )
    .unwrap();
    let result = apply(&f.ctx, &next).unwrap();
    assert_eq!(result.failure, None);
    assert!(result
        .verification
        .issues
        .iter()
        .any(|i| i.dim == InvariantDim::Catalog)); // actual empty active inventory remains unavailable
}
#[test]
fn native_primary_partial_auth_failure_resumes_without_readding_global_profile_keys() {
    let f = Fixture::new();
    let (_, reviewed, ids, primary) = native_primary_setup(&f);
    let token =
        prepare_native_primary_composition(&f.ctx, &reviewed, &ids, Some(&primary)).unwrap();
    publication::write_atomic_private_report(&f.ctx.config(), &primary)
        .unwrap()
        .require_sync()
        .unwrap();
    let next = rebase_native_primary(
        &f.ctx,
        &reviewed,
        &token,
        &f.root.join("native-combined-partial"),
    )
    .unwrap();
    let guard = super::super::repair::acquire(&f.ctx).unwrap();
    let lock = f.ctx.home.join("accounts/.live-auth.lock");
    let partial = apply_under_manager_using(&f.ctx, &next, &guard, &mut |p, b| {
        let published = publication::write_atomic_private_report(p, b)?;
        fs::create_dir_all(lock.parent().unwrap())?;
        fs::create_dir(&lock)?;
        Ok(published)
    })
    .unwrap();
    assert!(partial.failure.is_some());
    assert!(partial.completed.iter().any(|s| s == "config"));
    fs::remove_dir(&lock).unwrap();
    let resumed =
        resume_after_approved_effects(&f.ctx, &next, &f.root.join("native-combined-resume"))
            .unwrap();
    assert_eq!(resumed.config_after(), token.config_after());
    let result = apply_under_manager(&f.ctx, &resumed, &guard).unwrap();
    assert_eq!(result.failure, None);
    assert!(result.verification.issues.is_empty());
    let final_doc: toml::Value =
        toml::from_str(&fs::read_to_string(f.ctx.config()).unwrap()).unwrap();
    assert!(final_doc.get("profile").is_none());
    assert!(final_doc.get("model_catalog_json").is_none());
}

fn bound_batch_setup(
    f: &Fixture,
) -> (
    ReviewedPlan,
    super::super::native_coordinator::PreparedBatch,
) {
    let (source, _, _, _) = native_primary_setup(f);
    // Integrity has A, while the old unlock backup contains distinct B. An unlock
    // fresh plan after Integrity must preserve A rather than overwrite it with B.
    fs::remove_file(f.ctx.home.join("auth.json")).unwrap();
    private_dir(&f.ctx.home.join("accounts/backups")).unwrap();
    atomic_private(
        &f.ctx.home.join("accounts/backups/A.json"),
        br#"{"OPENAI_API_KEY":"sk-synthetic-account-A"}"#,
    )
    .unwrap();
    let reviewed = prepare_reconcile_for_target(
        &f.ctx,
        &source,
        TargetState::NativeOff,
        &f.root.join("bound-router-original"),
    )
    .unwrap();
    let ids = [
        "auth_integrity",
        "router_unlock_auth_residue",
        "takeover_backup_orphan",
        "config_profile_conflict",
        "config_stale",
    ]
    .map(str::to_owned);
    let batch = super::super::native_coordinator::prepare_batch(
        &f.ctx,
        &ids,
        &f.root.join("bound-batch-original"),
    )
    .unwrap();
    (reviewed, batch)
}
fn execute_bound_batch_auth(
    f: &Fixture,
    batch: &super::super::native_coordinator::PreparedBatch,
) -> super::super::native_coordinator::BatchSession {
    let mut session = super::super::native_coordinator::begin_batch(&f.ctx, batch).unwrap();
    for id in [
        "auth_integrity",
        "router_unlock_auth_residue",
        "takeover_backup_orphan",
    ] {
        let result =
            super::super::native_coordinator::batch_auth(&f.ctx, &mut session, id).unwrap();
        assert!(result.success, "{}: {}", id, result.detail);
    }
    session
}
#[test]
fn bound_batch_resync_consumes_real_integrity_unlock_takeover_prefix_and_finite_config_candidate() {
    use atelier_maintenance_core::batch_repair::ConfigPreparation;
    let f = Fixture::new();
    let (reviewed, batch) = bound_batch_setup(&f);
    let token = prepare_bound_batch_resync(&f.ctx, &reviewed, &batch).unwrap();
    assert!(token.config_candidates().iter().all(|(_, r)| r.is_ok()));
    assert!(token
        .approved_dependencies()
        .iter()
        .any(|(p, _)| p.ends_with("accounts/backups/A.json")));
    let mut session = execute_bound_batch_auth(&f, &batch);
    for kind in [ConfigPreparation::Profile, ConfigPreparation::Stale] {
        super::super::native_coordinator::batch_prepare_config(&f.ctx, &mut session, kind).unwrap();
    }
    // The old catalog->config->auth resumption order cannot explain this actual
    // full-batch prefix; the bound proof can, without approving current bytes anew.
    assert!(
        resume_after_approved_effects(&f.ctx, &reviewed, &f.root.join("old-resume-refused"))
            .is_err()
    );
    let next = rebase_bound_batch_resync(
        &f.ctx,
        &reviewed,
        &token,
        session.prefix(),
        &f.root.join("bound-resync"),
    )
    .unwrap();
    assert!(next
        .auth
        .iter()
        .flat_map(native_auth::approved_effects)
        .next()
        .is_none());
    let approved = token
        .config_candidates()
        .into_iter()
        .find(|(k, _)| *k == session.prefix().candidate_key())
        .unwrap()
        .1
        .unwrap();
    assert_eq!(next.config_after, approved);
    let result = apply(&f.ctx, &next).unwrap();
    assert_eq!(result.failure, None);
    assert!(
        result.verification.issues.is_empty(),
        "{:?}",
        result.verification.issues
    );
    assert_eq!(
        fs::read(f.ctx.home.join("auth.json")).unwrap(),
        br#"{"OPENAI_API_KEY":"sk-synthetic-account-A"}"#
    );
    let text = fs::read_to_string(f.ctx.config()).unwrap();
    let config: toml::Value = toml::from_str(&text).unwrap();
    assert!(config.get("profile").is_none());
    assert!(config.get("model_provider").is_none());
    assert_eq!(config["model"].as_str(), Some("restored-fixture"));
    assert_eq!(config["unrelated"]["keep"].as_str(), Some("fixture"));
}
#[test]
fn bound_batch_provenance_cannot_be_replaced_by_a_fresh_equal_preview() {
    let f = Fixture::new();
    let (reviewed, batch) = bound_batch_setup(&f);
    let token = prepare_bound_batch_resync(&f.ctx, &reviewed, &batch).unwrap();
    let other = super::super::native_coordinator::prepare_batch(
        &f.ctx,
        batch.item_ids(),
        &f.root.join("separately-reviewed-batch"),
    )
    .unwrap();
    let session = super::super::native_coordinator::begin_batch(&f.ctx, &other).unwrap();
    assert!(rebase_bound_batch_resync(
        &f.ctx,
        &reviewed,
        &token,
        session.prefix(),
        &f.root.join("forged-binding")
    )
    .is_err());
    assert!(!f.root.join("forged-binding").exists());
    assert!(!f.ctx.home.join("auth.json").exists());
}
#[test]
fn bound_batch_external_auth_config_source_and_new_candidate_are_never_signed_as_prefix() {
    for drift in ["auth", "config", "source", "candidate"] {
        let f = Fixture::new();
        let (reviewed, batch) = bound_batch_setup(&f);
        let token = prepare_bound_batch_resync(&f.ctx, &reviewed, &batch).unwrap();
        let session = execute_bound_batch_auth(&f, &batch);
        match drift {
            "auth" => atomic_private(
                &f.ctx.home.join("auth.json"),
                br#"{"OPENAI_API_KEY":"external"}"#,
            )
            .unwrap(),
            "config" => {
                let mut b = fs::read(f.ctx.config()).unwrap();
                b.extend_from_slice(b"\n# external\n");
                fs::write(f.ctx.config(), b).unwrap();
            }
            "source" => {
                let path = source_path(reviewed.snapshot.source());
                let mut b = fs::read(path).unwrap();
                b.push(b' ');
                fs::write(path, b).unwrap();
            }
            "candidate" => atomic_private(
                &f.ctx.home.join("accounts/backups/new.json"),
                br#"{"OPENAI_API_KEY":"sk-synthetic-account-A"}"#,
            )
            .unwrap(),
            _ => unreachable!(),
        }
        let before = fs::read(f.ctx.config()).unwrap();
        assert!(
            rebase_bound_batch_resync(
                &f.ctx,
                &reviewed,
                &token,
                session.prefix(),
                &f.root.join("drift-denied")
            )
            .is_err(),
            "{drift}"
        );
        assert_eq!(fs::read(f.ctx.config()).unwrap(), before);
        assert!(!f.root.join("drift-denied").exists());
    }
}
#[test]
fn bound_batch_token_is_native_only_and_original_union_must_be_unchanged() {
    let f = Fixture::new();
    let gateway = prepare_reconcile_for_target(
        &f.ctx,
        &f.source,
        TargetState::RouterOn,
        &f.root.join("gateway-bound-refuse"),
    )
    .unwrap();
    let (_, batch) = bound_batch_setup(&f);
    assert!(prepare_bound_batch_resync(&f.ctx, &gateway, &batch).is_err());
    let (reviewed, batch) = bound_batch_setup(&f);
    let mut bytes = fs::read(f.ctx.config()).unwrap();
    bytes.extend_from_slice(b"\n# changed after approval\n");
    fs::write(f.ctx.config(), bytes).unwrap();
    assert!(prepare_bound_batch_resync(&f.ctx, &reviewed, &batch).is_err());
}
#[test]
fn bound_resync_receipt_proves_original_to_final_effects_and_detects_later_touch() {
    let f = Fixture::new();
    let (reviewed, batch) = bound_batch_setup(&f);
    let token = prepare_bound_batch_resync(&f.ctx, &reviewed, &batch).unwrap();
    let session = execute_bound_batch_auth(&f, &batch);
    let guard = super::super::repair::acquire(&f.ctx).unwrap();
    let (outcome, receipt) = apply_bound_batch_resync(
        &f.ctx,
        &reviewed,
        &token,
        session.prefix(),
        &f.root.join("bound-receipt"),
        &guard,
    )
    .unwrap();
    assert!(outcome.failure.is_none());
    receipt.validate_current(&f.ctx).unwrap();
    assert!(receipt.plan().same_plan(&batch));
    let config = receipt.effect_stamp(&f.ctx.config()).unwrap();
    assert_eq!(
        config.0.as_ref().map(|s| s.0),
        reviewed.config_before().map(|b| b.len() as u64)
    );
    assert_eq!(
        config.1.as_ref().map(|s| s.0),
        Some(fs::metadata(f.ctx.config()).unwrap().len())
    );
    assert!(receipt.paths().contains(&f.ctx.home.join("auth.json")));
    assert!(receipt
        .effect_stamp(&f.ctx.home.join("auth.json"))
        .unwrap()
        .0
        .is_none());
    let mut after = fs::read(f.ctx.config()).unwrap();
    after.extend_from_slice(b"\n# external after resync\n");
    fs::write(f.ctx.config(), after).unwrap();
    assert!(receipt.validate_current(&f.ctx).is_err());
}
#[test]
fn bound_syntax_source_seed_never_publishes_invalid_original_and_only_proved_rescue_can_run() {
    use atelier_maintenance_core::batch_repair::ConfigPreparation;
    let f = Fixture::new();
    let source = f.native(false);
    let original=b"model='synthetic-original'\n# >>> aimami-relay managed start (DO NOT EDIT MANUALLY)\nbroken = [\n# <<< aimami-relay managed end\n";
    fs::write(f.ctx.config(), original).unwrap();
    fs::write(
        f.ctx.home.join("auth.json"),
        br#"{"OPENAI_API_KEY":"synthetic-user"}"#,
    )
    .unwrap();
    let batch = super::super::native_coordinator::prepare_batch(
        &f.ctx,
        &["config_toml_syntax".into()],
        &f.root.join("syntax-batch"),
    )
    .unwrap();
    assert!(prepare_reconcile_for_target(
        &f.ctx,
        &source,
        TargetState::NativeOff,
        &f.root.join("old-syntax-fails")
    )
    .is_err());
    let (seed, token) = prepare_bound_batch_resync_from_source(
        &f.ctx,
        &source,
        TargetState::NativeOff,
        &f.root.join("syntax-seed"),
        None,
        &batch,
    )
    .unwrap();
    assert!(token.config_candidates().iter().any(|(_, r)| r.is_err()));
    assert!(token.config_candidates().iter().any(|(_, r)| r.is_ok()));
    assert!(apply(&f.ctx, &seed).is_err());
    assert_eq!(fs::read(f.ctx.config()).unwrap(), original);
    let mut session = super::super::native_coordinator::begin_batch(&f.ctx, &batch).unwrap();
    super::super::native_coordinator::batch_prepare_config(
        &f.ctx,
        &mut session,
        ConfigPreparation::Syntax,
    )
    .unwrap();
    let guard = super::super::repair::acquire(&f.ctx).unwrap();
    let (outcome, proof) = apply_bound_batch_resync(
        &f.ctx,
        &seed,
        &token,
        session.prefix(),
        &f.root.join("syntax-real-resync"),
        &guard,
    )
    .unwrap();
    assert!(outcome.failure.is_none());
    proof.validate_current(&f.ctx).unwrap();
    let actual = fs::read_to_string(f.ctx.config()).unwrap();
    assert!(!actual.contains("broken = ["));
    assert_eq!(
        toml::from_str::<toml::Value>(&actual).unwrap()["model"].as_str(),
        Some("synthetic-original")
    );
}
#[test]
fn bound_resync_uncertain_publication_keeps_completed_result_and_poisoned_receipt() {
    let f = Fixture::new();
    let (reviewed, batch) = bound_batch_setup(&f);
    let token = prepare_bound_batch_resync(&f.ctx, &reviewed, &batch).unwrap();
    let session = execute_bound_batch_auth(&f, &batch);
    let guard = super::super::repair::acquire(&f.ctx).unwrap();
    let (outcome, receipt) = apply_bound_batch_resync_using(
        &f.ctx,
        &reviewed,
        &token,
        session.prefix(),
        &f.root.join("bound-sync-uncertain"),
        &guard,
        &mut |path, bytes| {
            let mut p = publication::write_atomic_private_report(path, bytes)?;
            p.directory_sync_error = Some(io::Error::other("synthetic directory sync uncertainty"));
            Ok(p)
        },
    )
    .unwrap();
    assert!(outcome.failure.is_some());
    assert!(outcome
        .completed
        .iter()
        .any(|s| s == "config_published_sync_unconfirmed"));
    assert_eq!(
        fs::read(f.ctx.config()).unwrap(),
        token
            .config_candidates()
            .into_iter()
            .find(|(key, _)| *key == session.prefix().candidate_key())
            .unwrap()
            .1
            .unwrap()
            .unwrap()
    );
    assert!(receipt.validate_current(&f.ctx).is_err());
    assert_eq!(
        fs::read(f.ctx.home.join("auth.json")).unwrap(),
        br#"{"OPENAI_API_KEY":"sk-synthetic-account-A"}"#
    );
}

fn observed_off_setup(f: &Fixture) -> super::super::native_coordinator::PreparedBatch {
    fs::remove_file(f.ctx.app_root.join("gateway/catalog.json")).unwrap();
    let relay = f.ctx.home.join("codexmate/relay");
    private_dir(&relay).unwrap();
    let text=format!("# source-free fixture\nmodel_provider='openai'\nprofile='aimai1'\nmodel='aimami_relay_fixture'\nmodel_catalog_json={:?}\n[model_providers.aimai1]\nbase_url='http://127.0.0.1:1/codex/router/v1'\n[profiles.aimai1]\nmodel='aimami_relay_fixture'\n[unrelated]\nkeep='source-free'\n",relay.join(catalog::CATALOG_FILE).to_str().unwrap());
    fs::write(f.ctx.config(), text).unwrap();
    fs::remove_file(f.ctx.home.join("auth.json")).ok();
    private_dir(&f.ctx.home.join("accounts/backups")).unwrap();
    atomic_private(
        &f.ctx.home.join("accounts/backups/A.json"),
        br#"{"OPENAI_API_KEY":"sk-source-free-A"}"#,
    )
    .unwrap();
    atomic_private(
        &relay.join("router-unlock-auth-backup.json"),
        br#"{"OPENAI_API_KEY":"sk-source-free-B"}"#,
    )
    .unwrap();
    atomic_private(
        &relay.join("router-unlock-auth-marker.json"),
        br#"{"backedUpPreviousAuth":true}"#,
    )
    .unwrap();
    atomic_private(&relay.join("router-takeover-backup.json"),&serde_json::to_vec(&json!({"version":1,"createdAtMs":1,"topLevelLines":["model='restored-source-free'","profile='aimai1'"],"originalConfig":"not blindly restored"})).unwrap()).unwrap();
    let ids = [
        "auth_integrity",
        "router_unlock_auth_residue",
        "takeover_backup_orphan",
        "config_toml_syntax",
    ]
    .map(str::to_owned);
    super::super::native_coordinator::prepare_batch(&f.ctx, &ids, &f.root.join("source-free-batch"))
        .unwrap()
}
#[test]
fn observed_off_real_source_none_auth_syntax_batch_resync_creates_no_routing_source() {
    use atelier_maintenance_core::batch_repair::ConfigPreparation;
    let f = Fixture::new();
    let batch = observed_off_setup(&f);
    let token = prepare_bound_observed_off(&f.ctx, &batch).unwrap();
    assert!(token.catalog_effect().is_none());
    assert!(token.original.is_none());
    let mut session = execute_bound_batch_auth(&f, &batch);
    super::super::native_coordinator::batch_prepare_config(
        &f.ctx,
        &mut session,
        ConfigPreparation::Syntax,
    )
    .unwrap();
    let guard = super::super::repair::acquire(&f.ctx).unwrap();
    let (outcome, receipt) = apply_bound_observed_off(
        &f.ctx,
        &token,
        session.prefix(),
        &f.root.join("source-free-resync"),
        &guard,
    )
    .unwrap();
    assert!(outcome.failure.is_none(), "{:?}", outcome.failure);
    assert!(
        outcome.verification.issues.is_empty(),
        "{:?}",
        outcome.verification.issues
    );
    receipt.validate_current(&f.ctx).unwrap();
    assert!(!native_catalog::native_state_path(&f.ctx.home).exists());
    assert!(!f.ctx.app_root.join("gateway/catalog.json").exists());
    assert!(!f
        .ctx
        .home
        .join("codexmate/relay")
        .join(catalog::CATALOG_FILE)
        .exists());
    assert_eq!(
        fs::read(f.ctx.home.join("auth.json")).unwrap(),
        br#"{"OPENAI_API_KEY":"sk-source-free-A"}"#
    );
    let config: toml::Value = toml::from_str(&fs::read_to_string(f.ctx.config()).unwrap()).unwrap();
    assert!(config.get("profile").is_none());
    assert!(config.get("model_catalog_json").is_none());
    assert!(config
        .get("model_providers")
        .and_then(|v| v.get("aimai1"))
        .is_none());
    // The takeover primitive preserves an already-present model key; the Off
    // cleanup then removes that stale managed relay value.
    assert!(config.get("model").is_none());
    assert_eq!(config["unrelated"]["keep"].as_str(), Some("source-free"));
}
#[test]
fn observed_off_rejects_source_appearance_external_bytes_and_on_effective_config() {
    for drift in ["native", "gateway", "config"] {
        let f = Fixture::new();
        let batch = observed_off_setup(&f);
        let token = prepare_bound_observed_off(&f.ctx, &batch).unwrap();
        let session = execute_bound_batch_auth(&f, &batch);
        match drift {
            "native" => {
                f.native(false);
            }
            "gateway" => {
                atomic_private(&f.ctx.app_root.join("gateway/catalog.json"), b"{}").unwrap()
            }
            "config" => {
                let mut b = fs::read(f.ctx.config()).unwrap();
                b.extend_from_slice(b"\n# external\n");
                fs::write(f.ctx.config(), b).unwrap();
            }
            _ => unreachable!(),
        }
        let guard = super::super::repair::acquire(&f.ctx).unwrap();
        assert!(apply_bound_observed_off(
            &f.ctx,
            &token,
            session.prefix(),
            &f.root.join("source-free-drift-denied"),
            &guard
        )
        .is_err());
        assert!(!f.root.join("source-free-drift-denied").exists());
    }
    let f = Fixture::new();
    fs::remove_file(f.ctx.app_root.join("gateway/catalog.json")).unwrap();
    let batch = super::super::native_coordinator::prepare_batch(
        &f.ctx,
        &["config_toml_syntax".into()],
        &f.root.join("on-no-source"),
    )
    .unwrap();
    assert!(batch.mode().enabled());
    assert!(prepare_bound_observed_off(&f.ctx, &batch).is_err());
}
#[test]
fn observed_off_persisted_intent_deactivates_atelier_config_without_catalog_source() {
    let f = Fixture::new();
    fs::remove_file(f.ctx.app_root.join("gateway/catalog.json")).unwrap();
    let generated = f.ctx.home.join(catalog::CATALOG_FILE);
    let catalog_bytes = fs::read(&generated).unwrap();
    let intent = super::super::router_intent::IntentRecord {
        schema_version: 1,
        target: atelier_maintenance_core::transition_journal::TransitionTarget::NativeOff,
        provider: None,
        operation_id: None,
        catalog_revision: None,
        updated_at_ms: 1,
    };
    let path = super::super::router_intent::path(&f.ctx);
    atomic_private(&path, &serde_json::to_vec(&intent).unwrap()).unwrap();
    let batch = super::super::native_coordinator::prepare_batch(
        &f.ctx,
        &["config_toml_syntax".into()],
        &f.root.join("intent-off-batch"),
    )
    .unwrap();
    assert_eq!(batch.mode(), RouterMode::Off);
    let token = prepare_bound_observed_off(&f.ctx, &batch).unwrap();
    let session = super::super::native_coordinator::begin_batch(&f.ctx, &batch).unwrap();
    let guard = super::super::repair::acquire(&f.ctx).unwrap();
    let (outcome, receipt) = apply_bound_observed_off(
        &f.ctx,
        &token,
        session.prefix(),
        &f.root.join("intent-off-resync"),
        &guard,
    )
    .unwrap();
    assert!(outcome.failure.is_none());
    assert!(outcome.verification.issues.is_empty());
    receipt.validate_current(&f.ctx).unwrap();
    let text = fs::read_to_string(f.ctx.config()).unwrap();
    assert_eq!(provider(&text).unwrap(), "openai");
    let config: toml::Value = toml::from_str(&text).unwrap();
    assert!(config.get("model_catalog_json").is_none());
    assert_eq!(fs::read(generated).unwrap(), catalog_bytes);
    assert!(!native_catalog::native_state_path(&f.ctx.home).exists());
    assert!(!f.ctx.home.join("auth.json").exists());
    assert_eq!(
        super::super::router_intent::read(&f.ctx).unwrap().unwrap(),
        intent
    );
}
#[test]
fn observed_off_bad_toml_explicit_atelier_owner_and_unreadable_source_are_not_absence() {
    let f = Fixture::new();
    fs::remove_file(f.ctx.app_root.join("gateway/catalog.json")).unwrap();
    fs::write(
        f.ctx.config(),
        "model_provider='atelier_custom'\nbroken=[\n",
    )
    .unwrap();
    let scope = ObservedOff {
        home: f.ctx.home.clone(),
        app_root: f.ctx.app_root.clone(),
    };
    assert!(scope.validate(&f.ctx).is_err());
    fs::write(f.ctx.config(), "model_provider='openai'\n").unwrap();
    private_dir(&native_catalog::native_state_path(&f.ctx.home)).unwrap();
    assert!(scope.validate(&f.ctx).is_err());
}
#[test]
fn observed_off_missing_three_sources_bad_syntax_uses_native_default_witness_and_exact_rescue() {
    use atelier_maintenance_core::batch_repair::ConfigPreparation;
    let f = Fixture::new();
    fs::remove_file(f.ctx.app_root.join("gateway/catalog.json")).unwrap();
    let original=b"model_provider='openai'\nmodel='user-model'\n# >>> aimami-relay managed start (DO NOT EDIT MANUALLY)\nbroken = [\n# <<< aimami-relay managed end\n";
    fs::write(f.ctx.config(), original).unwrap();
    let batch = super::super::native_coordinator::prepare_batch(
        &f.ctx,
        &["config_toml_syntax".into()],
        &f.root.join("missing-source-syntax-batch"),
    )
    .unwrap();
    let token = prepare_bound_observed_off(&f.ctx, &batch).unwrap();
    assert_eq!(token.mode, RouterMode::Off);
    // NativeOff itself strips owned managed blocks before parsing, so this
    // malformed managed fragment has a concrete approved final in both branches.
    assert!(token.config_candidates().iter().all(|(_, r)| r.is_ok()));
    let mut session = super::super::native_coordinator::begin_batch(&f.ctx, &batch).unwrap();
    super::super::native_coordinator::batch_prepare_config(
        &f.ctx,
        &mut session,
        ConfigPreparation::Syntax,
    )
    .unwrap();
    assert_eq!(
        fs::read(f.ctx.config()).unwrap(),
        b"model_provider='openai'\nmodel='user-model'\n"
    );
    assert!(toml::from_str::<toml::Value>(
        std::str::from_utf8(session.prefix().config_after().unwrap()).unwrap()
    )
    .is_ok());
    let guard = super::super::repair::acquire(&f.ctx).unwrap();
    let (outcome, receipt) = apply_bound_observed_off(
        &f.ctx,
        &token,
        session.prefix(),
        &f.root.join("missing-source-syntax-resync"),
        &guard,
    )
    .unwrap();
    assert!(outcome.failure.is_none());
    receipt.validate_current(&f.ctx).unwrap();
    let config: toml::Value = toml::from_str(&fs::read_to_string(f.ctx.config()).unwrap()).unwrap();
    assert_eq!(config["model"].as_str(), Some("user-model"));
    assert!(!native_catalog::native_state_path(&f.ctx.home).exists());
    assert!(!f.ctx.app_root.join("gateway/catalog.json").exists());
    assert!(!super::super::router_intent::path(&f.ctx).exists());
    assert!(!f.ctx.home.join("auth.json").exists());
}
#[test]
fn observed_off_unrecoverable_syntax_salvage_is_published_but_resync_error_remains_approved() {
    use atelier_maintenance_core::batch_repair::ConfigPreparation;
    let f = Fixture::new();
    fs::remove_file(f.ctx.app_root.join("gateway/catalog.json")).unwrap();
    // This malformed array is outside a managed block. Native rescue preserves
    // nonempty lines with a rescue header; publishing it does not make TOML valid.
    let original = b"model_provider='openai'\nmodel='user-model'\nbroken = [\n";
    fs::write(f.ctx.config(), original).unwrap();
    let expected = match super::super::config_syntax::plan_config_toml_syntax_rescue(original) {
        super::super::config_syntax::RescuePlan::Plan { after, .. } => after,
        super::super::config_syntax::RescuePlan::None => {
            panic!("malformed unmanaged array requires salvage")
        }
    };
    assert_ne!(expected, original);
    assert!(String::from_utf8_lossy(&expected).contains("broken = ["));
    assert!(toml::from_str::<toml::Value>(std::str::from_utf8(&expected).unwrap()).is_err());
    let batch = super::super::native_coordinator::prepare_batch(
        &f.ctx,
        &["config_toml_syntax".into()],
        &f.root.join("unrecoverable-syntax-batch"),
    )
    .unwrap();
    let token = prepare_bound_observed_off(&f.ctx, &batch).unwrap();
    let mut session = super::super::native_coordinator::begin_batch(&f.ctx, &batch).unwrap();
    super::super::native_coordinator::batch_prepare_config(
        &f.ctx,
        &mut session,
        ConfigPreparation::Syntax,
    )
    .unwrap();
    assert_eq!(fs::read(f.ctx.config()).unwrap(), expected);
    assert_eq!(session.prefix().config_after(), Some(expected.as_slice()));
    let reviewed_error = token
        .config_candidates()
        .into_iter()
        .find(|(key, _)| *key == session.prefix().candidate_key())
        .unwrap()
        .1
        .unwrap_err();
    let guard = super::super::repair::acquire(&f.ctx).unwrap();
    let backup = f.root.join("unrecoverable-resync-rejected");
    let actual_error = apply_bound_observed_off(&f.ctx, &token, session.prefix(), &backup, &guard)
        .err()
        .expect("resync must keep the reviewed error");
    assert_eq!(actual_error, reviewed_error);
    assert_eq!(fs::read(f.ctx.config()).unwrap(), expected);
    assert!(!backup.exists());
    assert!(!native_catalog::native_state_path(&f.ctx.home).exists());
    assert!(!f.ctx.app_root.join("gateway/catalog.json").exists());
    assert!(!super::super::router_intent::path(&f.ctx).exists());
    assert!(!f.ctx.home.join("auth.json").exists());
}
