use super::*;
use serde_json::{json, Value};
struct Fixture {
    root: PathBuf,
    ctx: Context,
    source: native_catalog::Source,
    original: Vec<u8>,
    missing: PathBuf,
}
impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "atelier-config-composition-{}",
            uuid::Uuid::new_v4()
        ));
        let home = root.join("codex");
        let app_root = root.join("atelier");
        private_dir(&home.join("codexmate/relay")).unwrap();
        private_dir(&app_root).unwrap();
        let ctx = Context {
            home: home.clone(),
            app_root,
            desktop_running: false,
            runtime: None,
            official_catalog: None,
            proxy_handle: None,
            working_directory: None,
            sqlite_home_env: None,
        };
        let original=b"# initial synthetic config\nmodel_provider='openai'\nmodel='current-model'\n\n[profiles.keep]\nmodel='keep-profile'\n# keep trailing comment\n".to_vec();
        atomic_private(&ctx.config(), &original).unwrap();
        let state = native_catalog::native_state_path(&home);
        atomic_private(&state,&serde_json::to_vec(&json!({"schemaVersion":16,"codexRouterEnabled":false,"providers":[{"id":"p","models":[{"model":"fixture-model","wireApi":"openai-responses"}]}],"activeByIde":{"codex":["p"]}})).unwrap()).unwrap();
        let missing = root.join("missing-external-catalog.json");
        let backup = json!({"version":1,"createdAtMs":0,"topLevelLines":["profile='legacy-restored'","model_reasoning_effort=''",format!("model_catalog_json={:?}",missing.to_str().unwrap()),"restored_extra='keep-restored'"],
            "sectionBlocks":[{"key":"profiles.restored","lines":["[profiles.restored]","model='restored-profile-model'"]}],"originalConfig":"never blindly restore whole originalConfig"});
        atomic_private(
            &home.join("codexmate/relay/router-takeover-backup.json"),
            &serde_json::to_vec(&backup).unwrap(),
        )
        .unwrap();
        let source = native_catalog::Source::Native {
            state_path: state,
            catalog_root: home,
            bundle: None,
        };
        Self {
            root,
            ctx,
            source,
            original,
            missing,
        }
    }
    fn ids(&self) -> Vec<String> {
        [
            "takeover_backup_orphan",
            "config_global_validity",
            "config_profile_conflict",
            "catalog_integrity",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    }
    fn backup(&self) -> PathBuf {
        self.ctx.app_root.join("reviewed-repairs")
    }
    fn takeover(&self) -> PathBuf {
        self.ctx
            .home
            .join("codexmate/relay/router-takeover-backup.json")
    }
    fn catalog(&self) -> PathBuf {
        self.ctx
            .home
            .join(atelier_maintenance_core::catalog::CATALOG_FILE)
    }
    fn plan(&self) -> Prepared {
        prepare(&self.ctx, &self.ids(), Some(&self.source), &self.backup()).unwrap()
    }
    fn lease(&self) -> fs::File {
        let dir = self.ctx.app_root.join("maintenance");
        private_dir(&dir).unwrap();
        let f = open_private(&dir.join("codex-write.lock")).unwrap();
        f.try_lock_exclusive().unwrap();
        f
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
#[test]
fn real_takeover_global_profile_catalog_compose_once_without_losing_restored_or_existing_fields() {
    let f = Fixture::new();
    let plan = f.plan();
    assert_eq!(plan.item_ids(), f.ids());
    assert!(plan.owns_config());
    assert_eq!(plan.config_path(), f.ctx.config());
    assert_eq!(plan.config_before(), Some(f.original.as_slice()));
    let after = std::str::from_utf8(plan.config_after().unwrap()).unwrap();
    let parsed: toml::Value = toml::from_str(after).unwrap();
    for removed in ["profile", "model_reasoning_effort", "model_catalog_json"] {
        assert!(parsed.get(removed).is_none(), "{removed}");
    }
    assert_eq!(parsed["restored_extra"].as_str(), Some("keep-restored"));
    assert_eq!(parsed["model"].as_str(), Some("current-model"));
    assert_eq!(
        parsed["profiles"]["keep"]["model"].as_str(),
        Some("keep-profile")
    );
    assert_eq!(
        parsed["profiles"]["restored"]["model"].as_str(),
        Some("restored-profile-model")
    );
    assert!(after.contains("# keep trailing comment"));
    assert_eq!(fs::read(f.ctx.config()).unwrap(), f.original);
    assert!(f.takeover().exists());
    assert!(!f.catalog().exists());
    assert!(!f.backup().exists());
    validate_initial(&f.ctx, &plan).unwrap();
    let result = apply(&f.ctx, &plan).unwrap();
    assert!(result.failure.is_none());
    assert_eq!(
        result
            .completed
            .iter()
            .map(|r| r.stage.as_str())
            .collect::<Vec<_>>(),
        ["catalog", "config", "takeover_backup_orphan"]
    );
    assert!(result.completed.iter().all(|r| r.success));
    assert!(!f.takeover().exists());
    assert_eq!(
        fs::read(f.ctx.config()).unwrap(),
        plan.config_after().unwrap()
    );
    assert_eq!(
        fs::read(result.backup_path.join("config.toml.before")).unwrap(),
        f.original
    );
    let catalog: Value = serde_json::from_slice(&fs::read(f.catalog()).unwrap()).unwrap();
    assert_eq!(catalog["models"][0]["slug"], "p::fixture-model");
    let journal: Value =
        serde_json::from_slice(&fs::read(result.backup_path.join("stage-results.json")).unwrap())
            .unwrap();
    assert_eq!(journal["completed"].as_array().unwrap().len(), 3);
}
#[test]
fn original_auth_or_global_metadata_change_blocks_all_stages_before_mutation() {
    for changed in ["auth", "metadata", "config"] {
        let f = Fixture::new();
        let plan = f.plan();
        match changed {
            "auth" => atomic_private(&f.takeover(), b"external auth backup").unwrap(),
            "metadata" => atomic_private(&f.missing, b"file appeared externally").unwrap(),
            "config" => atomic_private(&f.ctx.config(), b"model='external'\n").unwrap(),
            _ => unreachable!(),
        }
        assert!(validate_initial(&f.ctx, &plan).is_err());
        assert!(apply(&f.ctx, &plan).is_err());
        assert!(!f.catalog().exists());
        assert!(!f.backup().exists());
        assert!(f.takeover().exists());
        if changed != "config" {
            assert_eq!(fs::read(f.ctx.config()).unwrap(), f.original);
        }
    }
}
#[test]
fn stage_failure_retains_actual_completed_catalog_and_preserves_external_config() {
    let f = Fixture::new();
    let plan = f.plan();
    let _lease = f.lease();
    let mut session = begin_under_existing_gate(&f.ctx, &plan).unwrap();
    catalog_stage(&f.ctx, &mut session);
    assert!(session.outcome().completed[0].success);
    assert!(f.catalog().is_file());
    atomic_private(&f.ctx.config(), b"# external update\nmodel='external'\n").unwrap();
    config_stage(&f.ctx, &mut session);
    auth_stage(&f.ctx, &mut session);
    let result = session.into_outcome();
    assert!(result.failure.is_some());
    assert_eq!(result.completed.len(), 2);
    assert_eq!(result.completed[0].stage, "catalog");
    assert!(result.completed[0].success);
    assert_eq!(result.completed[1].stage, "config");
    assert!(!result.completed[1].success);
    assert_eq!(
        fs::read(f.ctx.config()).unwrap(),
        b"# external update\nmodel='external'\n"
    );
    assert!(f.takeover().is_file());
    assert!(f.catalog().is_file());
    assert_eq!(
        fs::read(result.backup_path.join("config.toml.before")).unwrap(),
        f.original
    );
}
#[test]
fn new_unapproved_backup_after_config_stops_cleanup_but_keeps_prior_stage_receipts() {
    let f = Fixture::new();
    let plan = f.plan();
    let _lease = f.lease();
    let mut session = begin_under_existing_gate(&f.ctx, &plan).unwrap();
    catalog_stage(&f.ctx, &mut session);
    config_stage(&f.ctx, &mut session);
    assert_eq!(
        fs::read(f.ctx.config()).unwrap(),
        plan.config_after().unwrap()
    );
    let unapproved = f
        .ctx
        .home
        .join("codexmate/relay/codex_router_top_level_backup.json");
    atomic_private(&unapproved, b"new unapproved input").unwrap();
    auth_stage(&f.ctx, &mut session);
    let result = session.into_outcome();
    assert!(result.failure.is_some());
    assert_eq!(result.completed.len(), 3);
    assert!(result.completed[..2].iter().all(|r| r.success));
    assert!(!result.completed[2].success);
    assert!(f.takeover().is_file());
    assert_eq!(fs::read(unapproved).unwrap(), b"new unapproved input");
    assert_eq!(
        fs::read(f.ctx.config()).unwrap(),
        plan.config_after().unwrap()
    );
}
#[test]
fn concrete_unlock_and_takeover_both_rebase_only_the_approved_combined_config() {
    let f = Fixture::new();
    let relay = f.ctx.home.join("codexmate/relay");
    let user = br#"{"OPENAI_API_KEY":"sk-synthetic-coordinator-user","preserve":true}"#;
    let virtual_auth=br#"{"auth_mode":"apikey","OPENAI_API_KEY":"sk-aimami-relay-virtual-synthetic","tokens":{}}"#;
    atomic_private(&f.ctx.home.join("auth.json"), virtual_auth).unwrap();
    atomic_private(
        &relay.join("router-unlock-auth-marker.json"),
        br#"{"backedUpPreviousAuth":true}"#,
    )
    .unwrap();
    atomic_private(&relay.join("router-unlock-auth-backup.json"), user).unwrap();
    let mut ids = f.ids();
    ids.insert(0, "router_unlock_auth_residue".into());
    let plan = prepare(&f.ctx, &ids, Some(&f.source), &f.backup()).unwrap();
    let result = apply(&f.ctx, &plan).unwrap();
    assert!(result.failure.is_none());
    assert_eq!(result.completed.len(), 4);
    assert!(result.completed.iter().all(|r| r.success));
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), user);
    assert_eq!(
        fs::read(f.ctx.config()).unwrap(),
        plan.config_after().unwrap()
    );
    assert!(!f.takeover().exists());
    let journal = fs::read_to_string(result.backup_path.join("stage-results.json")).unwrap();
    assert!(!journal.contains("sk-synthetic"));
    assert!(!journal.contains("sk-aimami"));
}
#[test]
fn coordinator_never_forces_off_mode_or_accepts_unrecovered_generic_effects() {
    let f = Fixture::new();
    assert!(prepare(
        &f.ctx,
        &["db_orphan_providers".into()],
        Some(&f.source),
        &f.backup()
    )
    .is_err());
    let path = native_catalog::native_state_path(&f.ctx.home);
    let mut state: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    state["codexRouterEnabled"] = json!(true);
    atomic_private(&path, &serde_json::to_vec(&state).unwrap()).unwrap();
    assert!(prepare(&f.ctx, &f.ids(), Some(&f.source), &f.backup()).is_err());
    assert!(!f.catalog().exists());
    assert!(f.takeover().exists());
}

#[test]
fn atelier_stale_http_and_catalog_share_typed_gateway_dependencies_and_one_config_effect() {
    let f = Fixture::new();
    let gateway_dir = f.ctx.app_root.join("gateway");
    private_dir(&gateway_dir).unwrap();
    let gateway_path = gateway_dir.join("catalog.json");
    let gateway = crate::gateway::types::Catalog {
        revision: "fixture persisted revision".into(),
        host_id: "fixture host".into(),
        profiles: vec![],
        settings: crate::gateway::types::Settings {
            port: 19731,
            ..crate::gateway::types::Settings::default()
        },
    };
    atomic_private(&gateway_path, &serde_json::to_vec(&gateway).unwrap()).unwrap();
    let before=b"# preserve\nmodel_provider='atelier_fixture'\nprofile='legacy'\nmodel_reasoning_effort=''\n\n[model_providers.atelier_fixture]\nname='preserve provider name'\nbase_url='http://127.0.0.1:1/wrong'\nwire_api='chat'\ncustom_field='keep provider field'\n\n[profiles.keep]\nmodel='keep-profile'\n";
    atomic_private(&f.ctx.config(), before).unwrap();
    let source = native_catalog::Source::Gateway {
        catalog_path: gateway_path.clone(),
        catalog_root: f.ctx.home.clone(),
        bundle: None,
        route_enabled: true,
    };
    let ids = [
        "config_stale",
        "router_http_contract",
        "config_profile_conflict",
        "config_global_validity",
        "catalog_path_validity",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<Vec<_>>();
    let plan = prepare(&f.ctx, &ids, Some(&source), &f.backup()).unwrap();
    let preview = preview_lines(&plan).join("\n");
    assert!(!preview.contains("custom_field"));
    assert!(preview.contains("一次config"));
    let parsed: toml::Value =
        toml::from_str(std::str::from_utf8(plan.config_after().unwrap()).unwrap()).unwrap();
    let provider = &parsed["model_providers"]["atelier_fixture"];
    assert_eq!(
        provider["base_url"].as_str(),
        Some("http://127.0.0.1:19731/v1")
    );
    assert_eq!(provider["wire_api"].as_str(), Some("responses"));
    assert_eq!(provider["env_key"].as_str(), Some("ATELIER_GATEWAY_TOKEN"));
    assert_eq!(provider["requires_openai_auth"].as_bool(), Some(false));
    assert_eq!(provider["supports_websockets"].as_bool(), Some(false));
    assert_eq!(
        provider["custom_field"].as_str(),
        Some("keep provider field")
    );
    assert!(parsed.get("profile").is_none());
    assert!(parsed.get("model_reasoning_effort").is_none());
    assert_eq!(
        parsed["model_catalog_json"].as_str(),
        Some(f.catalog().to_str().unwrap())
    );
    let result = apply(&f.ctx, &plan).unwrap();
    assert!(result.failure.is_none());
    assert!(result.requires_manager_reconcile);
    assert_eq!(result.diagnostics.len(), 5);
    assert!(result.diagnostics.iter().all(|d| d.status == "ok"));
    assert_eq!(
        result
            .completed
            .iter()
            .filter(|r| r.stage == "config")
            .count(),
        1
    );
    assert_eq!(
        fs::read(f.ctx.config()).unwrap(),
        plan.config_after().unwrap()
    );
    assert_eq!(
        fs::read(result.backup_path.join("config.toml.before")).unwrap(),
        before
    );
    // The unused takeover backup is outside the selected scope and remains.
    assert!(f.takeover().is_file());
}

#[test]
fn typed_gateway_revision_change_and_unmanaged_provider_reject_stale_config_plan() {
    let f = Fixture::new();
    let path = f.ctx.app_root.join("gateway/catalog.json");
    private_dir(path.parent().unwrap()).unwrap();
    let gateway = crate::gateway::types::Catalog {
        revision: "one".into(),
        host_id: "host".into(),
        profiles: vec![],
        settings: crate::gateway::types::Settings::default(),
    };
    atomic_private(&path, &serde_json::to_vec(&gateway).unwrap()).unwrap();
    let ids = vec!["config_stale".into()];
    assert!(prepare(&f.ctx, &ids, None, &f.backup()).is_err());
    atomic_private(&f.ctx.config(),b"model_provider='atelier_fixture'\n[model_providers.atelier_fixture]\nbase_url='http://127.0.0.1:1/v1'\n").unwrap();
    let plan = prepare(&f.ctx, &ids, None, &f.backup()).unwrap();
    let before = fs::read(f.ctx.config()).unwrap();
    let mut bytes = fs::read(&path).unwrap();
    bytes.push(b' ');
    atomic_private(&path, &bytes).unwrap();
    assert!(apply(&f.ctx, &plan).is_err());
    assert_eq!(fs::read(f.ctx.config()).unwrap(), before);
    assert!(!f.backup().exists());
}

impl Fixture {
    fn integrity_unlock_inputs(&self) -> (PathBuf, Vec<u8>, Vec<u8>) {
        let accounts = self.ctx.home.join("accounts/backups");
        private_dir(&accounts).unwrap();
        let a = br#"{"OPENAI_API_KEY":"sk-synthetic-account-backup-A","fixture":"restored A"}"#
            .to_vec();
        let b = br#"{"OPENAI_API_KEY":"sk-synthetic-older-unlock-B","fixture":"older B"}"#.to_vec();
        let account = accounts.join("account-A.json");
        atomic_private(&account, &a).unwrap();
        let auth = self.ctx.home.join("auth.json");
        if auth.exists() {
            fs::remove_file(auth).unwrap();
        }
        let relay = self.ctx.home.join("codexmate/relay");
        atomic_private(
            &relay.join("router-unlock-auth-marker.json"),
            br#"{"backedUpPreviousAuth":true}"#,
        )
        .unwrap();
        atomic_private(&relay.join("router-unlock-auth-backup.json"), &b).unwrap();
        (account, a, b)
    }
    fn integrity_ids(&self) -> Vec<String> {
        let mut ids = self.ids();
        ids.insert(0, "router_unlock_auth_residue".into());
        ids.push("auth_integrity".into());
        ids
    }
}
#[test]
fn native_integrity_prefix_restores_account_a_before_unlock_and_never_overwrites_it_with_b() {
    let f = Fixture::new();
    let (account, a, b) = f.integrity_unlock_inputs();
    let plan = prepare(&f.ctx, &f.integrity_ids(), Some(&f.source), &f.backup()).unwrap();
    // Input selection is deliberately reversed; the native dispatch sequence wins.
    assert_eq!(native_auth::item_id(&plan.auth[0]), "auth_integrity");
    assert_eq!(
        native_auth::item_id(&plan.auth[1]),
        "router_unlock_auth_residue"
    );
    let auth_path = f.ctx.home.join("auth.json");
    let before_integrity = native_auth::approved_effects(&plan.auth[0])
        .into_iter()
        .find(|(p, _)| p == &auth_path)
        .unwrap()
        .1;
    let before_unlock = native_auth::approved_effects(&plan.auth[1])
        .into_iter()
        .find(|(p, _)| p == &auth_path)
        .unwrap()
        .1;
    assert_ne!(
        before_integrity, before_unlock,
        "fixture must contain two conflicting old after images"
    );
    assert!(!auth_path.exists());
    let result = apply(&f.ctx, &plan).unwrap();
    assert!(result.failure.is_none());
    assert_eq!(
        result
            .completed
            .iter()
            .map(|r| r.stage.as_str())
            .collect::<Vec<_>>(),
        [
            "auth_integrity",
            "catalog",
            "config",
            "router_unlock_auth_residue",
            "takeover_backup_orphan"
        ]
    );
    assert!(result.completed.iter().all(|r| r.success));
    assert!(result.diagnostics.iter().all(|r| r.status == "ok"));
    assert_eq!(fs::read(&auth_path).unwrap(), a);
    assert_ne!(fs::read(&auth_path).unwrap(), b);
    assert_eq!(fs::read(account).unwrap(), a);
    assert!(!f
        .ctx
        .home
        .join("codexmate/relay/router-unlock-auth-marker.json")
        .exists());
    assert!(!f
        .ctx
        .home
        .join("codexmate/relay/router-unlock-auth-backup.json")
        .exists());
    assert!(!f.takeover().exists());
    assert_eq!(
        fs::read(f.ctx.config()).unwrap(),
        plan.config_after().unwrap()
    );
    let journal = fs::read_to_string(result.backup_path.join("stage-results.json")).unwrap();
    assert!(!journal.contains("sk-synthetic"));
}
#[test]
fn unapproved_auth_change_after_integrity_preserves_user_update_and_completed_prefix() {
    let f = Fixture::new();
    let (_, a, _) = f.integrity_unlock_inputs();
    let plan = prepare(&f.ctx, &f.integrity_ids(), Some(&f.source), &f.backup()).unwrap();
    let _lease = f.lease();
    let mut session = begin_under_existing_gate(&f.ctx, &plan).unwrap();
    auth_integrity_stage(&f.ctx, &mut session);
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), a);
    assert!(session.outcome().completed[0].success);
    let external = br#"{"OPENAI_API_KEY":"sk-synthetic-external-C"}"#;
    atomic_private(&f.ctx.home.join("auth.json"), external).unwrap();
    catalog_stage(&f.ctx, &mut session);
    config_stage(&f.ctx, &mut session);
    auth_stage(&f.ctx, &mut session);
    postcheck(&f.ctx, &mut session);
    let result = session.into_outcome();
    assert!(result.failure.is_some());
    assert_eq!(result.completed[0].stage, "auth_integrity");
    assert!(result.completed[0].success);
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), external);
    assert!(f
        .ctx
        .home
        .join("codexmate/relay/router-unlock-auth-backup.json")
        .exists());
    assert_eq!(fs::read(f.ctx.config()).unwrap(), f.original);
    assert!(!f.catalog().exists());
}
#[test]
fn integrity_candidate_inventory_does_not_accept_a_new_same_bytes_source() {
    let f = Fixture::new();
    let (account, a, _) = f.integrity_unlock_inputs();
    let ids = vec!["auth_integrity".into(), "router_unlock_auth_residue".into()];
    let plan = prepare(&f.ctx, &ids, None, &f.backup()).unwrap();
    fs::File::open(&account)
        .unwrap()
        .set_times(
            fs::FileTimes::new()
                .set_modified(std::time::UNIX_EPOCH + std::time::Duration::from_secs(1)),
        )
        .unwrap();
    let newcomer = account
        .parent()
        .unwrap()
        .join("unapproved-same-content.json");
    atomic_private(&newcomer, &a).unwrap();
    // Neither the old nor the new source authorizes a file write after this review changed.
    assert!(validate_initial(&f.ctx, &plan).is_err());
    assert!(apply(&f.ctx, &plan).is_err());
    assert!(!f.backup().exists());
    assert!(!f.ctx.home.join("auth.json").exists());
    assert!(newcomer.exists());
    assert!(f
        .ctx
        .home
        .join("codexmate/relay/router-unlock-auth-backup.json")
        .exists());
}
#[test]
fn auth_integrity_preserves_non_file_store_exclusion_and_real_route_gate() {
    for store in ["keyring", "ephemeral"] {
        let f = Fixture::new();
        f.integrity_unlock_inputs();
        let config = format!(
            "cli_auth_credentials_store='{store}'\n{}",
            std::str::from_utf8(&f.original).unwrap()
        );
        atomic_private(&f.ctx.config(), config.as_bytes()).unwrap();
        assert!(prepare(&f.ctx, &["auth_integrity".into()], None, &f.backup()).is_err());
        assert!(!f.ctx.home.join("auth.json").exists());
        assert!(!f.backup().exists());
    }
    let f = Fixture::new();
    f.integrity_unlock_inputs();
    let state = native_catalog::native_state_path(&f.ctx.home);
    let mut data: Value = serde_json::from_slice(&fs::read(&state).unwrap()).unwrap();
    data["codexRouterEnabled"] = json!(true);
    atomic_private(&state, &serde_json::to_vec(&data).unwrap()).unwrap();
    assert!(prepare(&f.ctx, &["auth_integrity".into()], None, &f.backup()).is_err());
    assert!(!f.ctx.home.join("auth.json").exists());
}
