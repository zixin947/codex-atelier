use atelier_maintenance_core::router_verification::*;
use atelier_maintenance_core::{catalog, threads, transition_journal};
use catalog::state::RelayStateDocument;
use serde_json::json;
use std::{
    cell::RefCell,
    collections::VecDeque,
    fs, io,
    net::{SocketAddr, TcpListener},
    sync::{atomic::AtomicBool, Arc, Mutex},
    time::Duration,
};
struct Fixture {
    _temp: tempfile::TempDir,
    paths: VerificationPaths,
    state: RelayStateDocument,
    proxy: Mutex<Option<ProxyHandle>>,
}
impl Fixture {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let paths = VerificationPaths::from_home(&temp.path().join("codex"), vec![]);
        fs::create_dir_all(&paths.relay_root).unwrap();
        Self {
            _temp: temp,
            paths,
            state: RelayStateDocument::native_default(),
            proxy: Mutex::new(None),
        }
    }
    fn context(&self) -> VerificationContext<'_> {
        VerificationContext {
            paths: &self.paths,
            state: &self.state,
            proxy: &self.proxy,
        }
    }
    fn config(&self, text: &str) {
        fs::write(&self.paths.config_path, text).unwrap();
    }
    fn catalog(&self, no_account: bool, slugs: &[&str]) {
        fs::write(self.paths.catalog_path(),serde_json::to_vec(&json!({"aimami_no_account":no_account,"models":slugs.iter().map(|s|json!({"slug":s})).collect::<Vec<_>>()})).unwrap()).unwrap();
    }
    fn on(&mut self, port: u16, no_account: bool) {
        let mut state = self.state.as_value().clone();
        state["codexRouterEnabled"] = json!(true);
        state["providers"] =
            json!([{"id":"aimami_relay_p","model":"fixture","wireApi":"openai-responses"}]);
        state["activeByIde"] = json!({"codex":["aimami_relay_p"]});
        state["codexRouterNoAccountMode"] = json!(no_account);
        self.state = RelayStateDocument::from_value(state).unwrap();
        let path = if no_account {
            format!(
                "model_catalog_json={:?}\n",
                self.paths.catalog_path().to_str().unwrap()
            )
        } else {
            String::new()
        };
        self.config(&format!("{path}model_provider='aimai1'\nmodel='gpt-5.4'\n[model_providers.aimai1]\nbase_url='http://127.0.0.1:{port}/codex/router/v1'\nwire_api='responses'\nrequires_openai_auth={}\nsupports_websockets=false\n",!no_account));
        self.catalog(no_account, &["aimami_relay_p::fixture"]);
        *self.proxy.lock().unwrap() = Some(ProxyHandle {
            port,
            alive: Arc::new(AtomicBool::new(true)),
        });
    }
    fn db(&mut self, rows: &[(&str, &str, &str)]) {
        let path = self.paths.codex_home.join("state_5.sqlite");
        let c = rusqlite::Connection::open(&path).unwrap();
        c.execute_batch("CREATE TABLE threads(id TEXT,rollout_path TEXT,model_provider TEXT,source TEXT,archived INTEGER,model TEXT);").unwrap();
        for (id, provider, model) in rows {
            let rollout = self.paths.codex_home.join(format!("{id}.jsonl"));
            fs::write(&rollout,format!("{}\n",json!({"type":"session_meta","payload":{"id":id,"model_provider":provider,"model":model}}))).unwrap();
            c.execute(
                "INSERT INTO threads VALUES(?1,?2,?3,'cli',0,?4)",
                rusqlite::params![id, rollout.to_str().unwrap(), provider, model],
            )
            .unwrap();
        }
        self.paths.databases = vec![path];
    }
}
#[test]
fn atelier_off_preserves_external_history_selected_out_by_native_planner() {
    let mut f = Fixture::new();
    f.config("model_provider='openai'\n");
    f.db(&[("foreign-anchor", "openai", "gpt-fixture")]);
    let rollout = f.paths.codex_home.join("foreign-anchor.jsonl");
    let original = format!(
        "{}\n",
        json!({"type":"session_meta","payload":{"id":"foreign-anchor","model_provider":"enterprise"}})
    );
    fs::write(&rollout, &original).unwrap();
    let plan = threads::repair::plan_provider_convergence(
        &f.paths.databases,
        threads::repair::ProviderTarget::Official,
        &f.paths.codex_home.join("backup"),
    )
    .unwrap();
    assert!(plan.db_changes.is_empty() && plan.files.is_empty());
    let models = Default::default();
    let contract = VerificationContract::AtelierResponses(AtelierContract {
        provider_id: "atelier_fixture",
        expected_base_url: "http://127.0.0.1:18910/v1",
        catalog_path: &f.paths.catalog_path(),
        expected_raw_models: &models,
        selected_model: "gpt-fixture",
    });
    let report = verify_with_contract(
        &ContractContext {
            paths: &f.paths,
            proxy: &f.proxy,
        },
        &contract,
        TargetState::NativeOff,
        Depth::Full,
    );
    assert!(
        report.is_satisfied(),
        "planner preserved external history but verifier rejected it: {:?}",
        report.issues
    );
    assert_eq!(fs::read_to_string(rollout).unwrap(), original);
}

#[derive(Default)]
struct Probe {
    results: RefCell<VecDeque<io::Result<()>>>,
    calls: RefCell<Vec<(SocketAddr, Duration)>>,
    pauses: RefCell<Vec<Duration>>,
}
impl PortTransport for Probe {
    fn connect(&self, address: SocketAddr, timeout: Duration) -> io::Result<()> {
        self.calls.borrow_mut().push((address, timeout));
        self.results
            .borrow_mut()
            .pop_front()
            .unwrap_or(Err(io::ErrorKind::ConnectionRefused.into()))
    }
    fn pause(&self, duration: Duration) {
        self.pauses.borrow_mut().push(duration);
    }
}
#[test]
fn native_off_allows_missing_config_but_checks_actual_auth_and_takeover_sidecars_in_order() {
    let f = Fixture::new();
    assert!(
        verify_target_state_at_with_depth(&f.context(), TargetState::NativeOff, Depth::Full)
            .is_satisfied()
    );
    fs::write(
        f.paths.relay_root.join("router-takeover-backup.json"),
        b"fixture backup",
    )
    .unwrap();
    fs::write(
        f.paths.relay_root.join("virtual-auth-marker.json"),
        b"fixture marker",
    )
    .unwrap();
    fs::write(
        f.paths.codex_home.join("auth.json"),
        br#"{"auth_mode":"apikey","OPENAI_API_KEY":"sk-aimami-relay-virtual-fixture","tokens":{}}"#,
    )
    .unwrap();
    fs::write(
        f.paths.relay_root.join("router-unlock-auth-marker.json"),
        b"fixture marker",
    )
    .unwrap();
    let report = verify_invariants_with_depth(&f.context(), TargetState::NativeOff, Depth::Full);
    assert_eq!(
        report
            .issues
            .iter()
            .map(|i| i.message.as_str())
            .collect::<Vec<_>>(),
        [
            "NativeOff must not keep takeover backup",
            "NativeOff must not keep a legacy virtual auth marker",
            "NativeOff must not keep virtual unlock auth active",
            "NativeOff must not keep virtual unlock auth marker"
        ]
    );
    fs::remove_file(f.paths.relay_root.join("router-unlock-auth-marker.json")).unwrap();
    fs::write(
        f.paths.codex_home.join("auth.json"),
        br#"{"OPENAI_API_KEY":"sk-synthetic-user"}"#,
    )
    .unwrap();
    fs::write(
        f.paths.relay_root.join("router-unlock-auth-backup.json"),
        b"fixture backup",
    )
    .unwrap();
    assert!(
        verify_invariants_with_depth(&f.context(), TargetState::NativeOff, Depth::Full)
            .issues
            .iter()
            .any(|i| i.message == "NativeOff must not keep an orphaned virtual unlock auth backup")
    );
    fs::write(&f.paths.config_path, [0xff, 0xfe]).unwrap();
    assert!(
        verify_invariants_with_depth(&f.context(), TargetState::NativeOff, Depth::Full).issues[0]
            .message
            .starts_with("cannot read Codex config.toml:")
    );
}
#[test]
fn real_loopback_listener_and_owned_live_handle_are_required_in_addition_to_file_invariants() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let mut f = Fixture::new();
    f.on(port, false);
    let report =
        verify_target_state_at_with_depth(&f.context(), TargetState::RouterOn, Depth::Full);
    assert!(report.issues.is_empty(), "{:?}", report.issues);
    assert!(has_confirmed_target_state(
        &f.context(),
        TargetState::RouterOn
    ));
    f.proxy
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .alive
        .store(false, std::sync::atomic::Ordering::Release);
    let report =
        verify_target_state_at_with_depth(&f.context(), TargetState::RouterOn, Depth::Full);
    assert_eq!(report.issues.len(), 1);
    assert_eq!(report.issues[0].dim, InvariantDim::Proxy);
    assert!(report.issues[0]
        .message
        .contains("not accepting connections"));
    f.proxy
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .alive
        .store(true, std::sync::atomic::Ordering::Release);
    drop(listener);
    assert!(
        verify_target_state_at_with_depth(&f.context(), TargetState::RouterOn, Depth::Full)
            .issues
            .iter()
            .any(|i| i.dim == InvariantDim::Proxy)
    );
    let mut raw = f.state.as_value().clone();
    raw["proxy"]["running"] = json!(true);
    f.state = RelayStateDocument::from_value(raw).unwrap();
    *f.proxy.lock().unwrap() = None;
    assert_eq!(
        verify_target_state_at_with_depth(&f.context(), TargetState::RouterOn, Depth::Full)
            .issues
            .last()
            .unwrap()
            .message,
        "RouterOn requires a live local proxy handle"
    );
}
#[test]
fn native_probe_retries_once_with_exact_timeout_and_ignores_dead_handles() {
    let handle = ProxyHandle {
        port: 19001,
        alive: Arc::new(AtomicBool::new(true)),
    };
    let probe = Probe::default();
    probe
        .results
        .borrow_mut()
        .push_back(Err(io::ErrorKind::TimedOut.into()));
    probe.results.borrow_mut().push_back(Ok(()));
    assert!(proxy_handle_port_accepts(&handle, &probe));
    assert_eq!(probe.calls.borrow().len(), 2);
    assert!(probe
        .calls
        .borrow()
        .iter()
        .all(|(a, t)| a.ip().is_loopback()
            && a.port() == 19001
            && *t == Duration::from_millis(300)));
    assert_eq!(*probe.pauses.borrow(), [Duration::from_millis(50)]);
    handle
        .alive
        .store(false, std::sync::atomic::Ordering::Release);
    let dead = Probe::default();
    assert!(!proxy_handle_port_accepts(&handle, &dead));
    assert!(dead.calls.borrow().is_empty());
}
#[test]
fn proxy_poison_is_a_real_lock_failure_but_native_off_never_reads_that_lock() {
    let f = Fixture::new();
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _guard = f.proxy.lock().unwrap();
        panic!("synthetic poison");
    }));
    let report = verify_target_state_with_transport(
        &f.context(),
        TargetState::RouterOn,
        Depth::Full,
        &Probe::default(),
    );
    assert_eq!(
        report.issues.last().unwrap().message,
        "RouterOn cannot inspect the local proxy because its state lock is poisoned"
    );
    assert!(
        verify_target_state_at_with_depth(&f.context(), TargetState::NativeOff, Depth::Full)
            .issues
            .is_empty()
    );
}
#[test]
fn config_failures_skip_the_extra_http_contract_but_still_probe_the_proxy_port() {
    let mut f = Fixture::new();
    f.on(19002, false);
    let text = fs::read_to_string(&f.paths.config_path)
        .unwrap()
        .replace("model_provider='aimai1'", "model_provider='wrong'");
    f.config(&text.replace("wire_api='responses'", "wire_api='chat'"));
    let probe = Probe::default();
    probe.results.borrow_mut().push_back(Ok(()));
    let report = verify_target_state_with_transport(
        &f.context(),
        TargetState::RouterOn,
        Depth::Full,
        &probe,
    );
    assert!(report.issues.iter().any(|i| i.dim == InvariantDim::Config));
    assert!(!report
        .issues
        .iter()
        .any(|i| i.message.contains("HTTP contract mismatch")));
    assert_eq!(probe.calls.borrow().len(), 1);
    f.config(
        &text
            .replace("model_provider='wrong'", "model_provider='aimai1'")
            .replace("19002", "19003"),
    );
    let probe = Probe::default();
    probe.results.borrow_mut().push_back(Ok(()));
    assert!(verify_target_state_with_transport(
        &f.context(),
        TargetState::RouterOn,
        Depth::Full,
        &probe
    )
    .issues
    .iter()
    .any(|i| i.message.contains("HTTP contract mismatch")));
}
#[test]
fn http_contract_uses_native_suffix_typed_booleans_and_catalog_mode_fallback() {
    assert_eq!(
        router_http_contract_issues("", None, None).unwrap(),
        ["缺少 [model_providers.aimai1]"]
    );
    let text="[model_providers.aimai1]\nbase_url='opaque/codex/router/v1///'\nwire_api='responses'\nrequires_openai_auth=true\nsupports_websockets=false\n";
    assert!(router_http_contract_issues(text, None, None)
        .unwrap()
        .is_empty());
    assert_eq!(
        router_http_contract_issues(
            text,
            Some("http://127.0.0.1:1/codex/router/v1"),
            Some(false)
        )
        .unwrap()
        .len(),
        2
    );
    assert!(router_http_contract_issues("malformed = [", None, None).is_err());
    let wrong="[model_providers.aimai1]\nbase_url=1\nwire_api='chat'\nrequires_openai_auth='false'\nsupports_websockets=true\n";
    let issues = router_http_contract_issues(wrong, None, None).unwrap();
    assert_eq!(issues.len(), 4);
    assert_eq!(issues[2], "requires_openai_auth 必须为布尔值");
    let mut f = Fixture::new();
    f.on(19004, true);
    fs::remove_file(f.paths.catalog_path()).unwrap();
    let report = verify_router_on_http_contract(&f.paths, "http://127.0.0.1:19004/codex/router/v1");
    assert!(report.issues[0]
        .message
        .contains("requires_openai_auth 必须为 true"));
}
#[test]
fn no_account_mode_comes_from_actual_catalog_and_checks_relay_namespace_and_portable_id() {
    let mut f = Fixture::new();
    f.on(19005, true);
    assert!(verify_invariants_with_depth(
        &f.context(),
        TargetState::RouterOn,
        Depth::SkipProviderConvergence
    )
    .issues
    .is_empty());
    f.catalog(true, &["gpt-5.4", "aimami_relay_p::fixture"]);
    assert!(
        verify_invariants_with_depth(&f.context(), TargetState::RouterOn, Depth::Full)
            .issues
            .is_empty()
    );
    f.catalog(true, &["gpt-5.4", "ordinary_provider::fixture"]);
    assert!(
        verify_invariants_with_depth(&f.context(), TargetState::RouterOn, Depth::Full)
            .issues
            .iter()
            .any(|i| i.message
                == "RouterOn no-account catalog must contain relay models and legacy aliases only")
    );
    for slug in ["aimami_relay_p::ok", "aimami_relay_::ok::more"] {
        assert!(is_relay_model_slug(slug));
    }
    for slug in [
        "p::ok",
        "aimami_relay_p:: ",
        "aimami_relay_bad/id::ok",
        "aimami_relay_p",
    ] {
        assert!(!is_relay_model_slug(slug));
    }
    f.catalog(false, &["gpt-5.4"]);
    assert!(
        verify_invariants_with_depth(&f.context(), TargetState::RouterOn, Depth::Full)
            .issues
            .iter()
            .any(|i| i.message == "RouterOn requires_openai_auth must match the catalog mode")
    );
    fs::write(f.paths.catalog_path(), b"malformed JSON").unwrap();
    assert!(
        verify_invariants_with_depth(&f.context(), TargetState::RouterOn, Depth::Full)
            .issues
            .iter()
            .any(|i| i.dim == InvariantDim::Catalog
                && i.message.starts_with("invalid router catalog mode:"))
    );
}
#[test]
fn depth_one_skips_only_provider_convergence_and_still_reads_actual_model_databases() {
    let mut f = Fixture::new();
    f.on(19006, false);
    f.db(&[
        ("provider-mismatch", "openai", "gpt-5.4"),
        ("model-mismatch", "aimai1", "aimami_relay_missing::fixture"),
    ]);
    let full = verify_invariants_with_depth(&f.context(), TargetState::RouterOn, Depth::Full);
    assert_eq!(
        full.issues
            .iter()
            .filter(|i| i.dim == InvariantDim::Thread)
            .count(),
        2
    );
    let skip = verify_invariants_with_depth(
        &f.context(),
        TargetState::RouterOn,
        Depth::SkipProviderConvergence,
    );
    assert_eq!(
        skip.issues
            .iter()
            .filter(|i| i.dim == InvariantDim::Thread)
            .count(),
        1
    );
    assert!(skip
        .issues
        .iter()
        .any(|i| i.message.contains("thread models must be catalog-valid")));
    let off = Fixture::new();
    let mut off = off;
    off.db(&[("row", "aimai1", "gpt-5.4")]);
    assert!(
        verify_invariants_with_depth(&off.context(), TargetState::NativeOff, Depth::Full)
            .issues
            .iter()
            .any(|i| i.message.contains("convergence target=openai"))
    );
    assert!(verify_invariants_with_depth(
        &off.context(),
        TargetState::NativeOff,
        Depth::SkipProviderConvergence
    )
    .issues
    .is_empty());
}
#[test]
fn native_provider_wrapper_keeps_its_confirmed_corrupt_only_gate_difference() {
    let mut f = Fixture::new();
    f.db(&[("corrupt", "openai", "gpt-5.4")]);
    fs::write(f.paths.codex_home.join("corrupt.jsonl"), b"not json\n").unwrap();
    let inspection = threads::consistency::inspect_thread_provider_convergence_with_reader(
        &f.paths.databases,
        "openai",
        &mut threads::consistency::DiagnosticRolloutProviderCache::default(),
    );
    assert_eq!(inspection.corrupt_rollouts, 1);
    assert!(
        verify_invariants_with_depth(&f.context(), TargetState::NativeOff, Depth::Full)
            .issues
            .is_empty()
    );
}
#[test]
fn confirmation_reads_checked_commit_journal_and_diagnostic_postcondition_is_not_a_proxy_probe() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let mut f = Fixture::new();
    f.on(listener.local_addr().unwrap().port(), false);
    assert!(router_postcondition(&f.context()).unwrap().is_satisfied());
    assert!(has_confirmed_target_state(
        &f.context(),
        TargetState::RouterOn
    ));
    fs::write(f.paths.relay_root.join(transition_journal::FILE_NAME),br#"{"schemaVersion":1,"target":"routerOn","phase":"committing","startedAtMs":1,"updatedAtMs":1}"#).unwrap();
    let post = router_postcondition(&f.context()).unwrap();
    assert_eq!(post.journal, PostconditionJournal::Committing);
    assert!(!post.is_satisfied());
    assert!(post.verification.issues.is_empty());
    let probe = Probe::default();
    assert!(!has_confirmed_target_state_with_transport(
        &f.context(),
        TargetState::RouterOn,
        &probe
    ));
    assert!(probe.calls.borrow().is_empty());
    fs::write(
        f.paths.relay_root.join(transition_journal::FILE_NAME),
        b"malformed journal",
    )
    .unwrap();
    assert!(!has_confirmed_target_state(
        &f.context(),
        TargetState::RouterOn
    ));
    let post = router_postcondition(&f.context()).unwrap();
    assert!(matches!(post.journal, PostconditionJournal::Unreadable(_)));
    assert!(!post.is_satisfied());
    fs::remove_file(f.paths.relay_root.join(transition_journal::FILE_NAME)).unwrap();
    *f.proxy.lock().unwrap() = None;
    assert!(router_postcondition(&f.context()).unwrap().is_satisfied());
    assert!(!has_confirmed_target_state(
        &f.context(),
        TargetState::RouterOn
    ));
    assert_eq!(
        serde_json::to_string(&InvariantDim::Catalog).unwrap(),
        "\"catalog\""
    );
}

#[test]
fn explicit_atelier_expected_contract_validates_real_fields_without_faking_native_namespaces() {
    let text="model_provider='atelier_fixture'\n[model_providers.atelier_fixture]\nbase_url='http://127.0.0.1:18910/v1'\nwire_api='responses'\nrequires_openai_auth=false\nsupports_websockets=false\nenv_key='ATELIER_GATEWAY_TOKEN'\n";
    let expected =
        ExpectedConfig::atelier_responses("atelier_fixture", "http://127.0.0.1:18910/v1");
    assert_eq!(expected.provider_id(), "atelier_fixture");
    assert_eq!(expected.flavor, VerificationFlavor::AtelierResponses);
    assert!(config_contract_issues(text, &expected).unwrap().is_empty());
    assert_eq!(
        router_http_contract_issues(text, None, None).unwrap(),
        ["缺少 [model_providers.aimai1]"]
    );
    let issues = config_contract_issues(
        &text
            .replace("18910/v1", "18911/v1")
            .replace("ATELIER_GATEWAY_TOKEN", "OTHER_TOKEN"),
        &expected,
    )
    .unwrap();
    assert_eq!(issues.len(), 2);
    assert!(issues[0].contains("端口漂移"));
    assert_eq!(issues[1], "env_key 必须为 ATELIER_GATEWAY_TOKEN");
    let f = Fixture::new();
    assert!(read_catalog_slugs(&f.paths.catalog_path()).is_err());
    f.catalog(false, &["raw-gateway-model"]);
    assert_eq!(
        read_catalog_slugs(&f.paths.catalog_path())
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        ["raw-gateway-model"]
    );
    assert!(!read_catalog_no_account_mode(&f.paths.catalog_path()).unwrap());
}
