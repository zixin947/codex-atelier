use super::*;
use crate::gateway::{
    storage::{atomic_private, private_dir},
    types::{GatewayModel, Profile, ProfileKind, Settings},
};
use atelier_maintenance_core::catalog::credentials::{CredentialNamespace, MemoryKeychain};
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let p =
            std::env::temp_dir().join(format!("atelier-native-catalog-{}", uuid::Uuid::new_v4()));
        private_dir(&p).unwrap();
        Self(p)
    }
    fn output(&self) -> PathBuf {
        self.0.join(core::CATALOG_FILE)
    }
    fn native(&self, value: Value) -> Source {
        let state_path = native_state_path(&self.0);
        private_dir(state_path.parent().unwrap()).unwrap();
        atomic_private(&state_path, &serde_json::to_vec(&value).unwrap()).unwrap();
        Source::Native {
            state_path,
            catalog_root: self.0.clone(),
            bundle: None,
        }
    }
    fn gateway(&self) -> Source {
        let gateway = self.0.join("gateway");
        private_dir(&gateway).unwrap();
        let path = gateway.join("catalog.json");
        let catalog = GatewayCatalog {
            revision: "persisted revision is not CAS hash".into(),
            host_id: "synthetic-host".into(),
            profiles: vec![
                profile("other", vec!["other-model"]),
                profile("preferred", vec!["gpt-5.4", "gpt-5.4", "fixture-model"]),
            ],
            settings: Settings {
                preferred_profile_id: Some("preferred".into()),
                ..Settings::default()
            },
        };
        atomic_private(&path, &serde_json::to_vec(&catalog).unwrap()).unwrap();
        Source::Gateway {
            catalog_path: path,
            catalog_root: self.0.clone(),
            bundle: None,
            route_enabled: true,
        }
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn profile(id: &str, models: Vec<&str>) -> Profile {
    Profile {
        id: id.into(),
        kind: ProfileKind::ApiKey,
        name: format!("Fixture {id}"),
        email: None,
        enabled: true,
        priority: 0,
        base_url: "https://example.invalid/v1".into(),
        auth_status: "warning".into(),
        models: models
            .into_iter()
            .map(|id| GatewayModel {
                id: id.into(),
                name: format!("Model {id}"),
            })
            .collect(),
        cooldown_until: None,
        last_error: None,
        requests: 0,
        input_tokens: 0,
        output_tokens: 0,
        created_at: 0,
    }
}
fn native_value() -> Value {
    json!({"schemaVersion":16,"codexRouterEnabled":true,
    "providers":[{"id":"native","models":[{"model":"fixture","wireApi":"openai-responses"}]}],
    "activeByIde":{"codex":["native"]},"proxy":{"running":false,"port":0,"baseUrl":"","codexBaseUrl":"","opaque":"keep"}})
}

#[test]
fn actual_gateway_types_generate_only_selected_inventory_with_routable_raw_slugs() {
    let f = Fixture::new();
    let source = f.gateway();
    let report = inspect(&source, Some(""), true).unwrap();
    assert_eq!(report[0].status, "warning");
    assert_eq!(report[1].status, "error");
    assert!(report[1].fixable);
    let source_before = fs::read(source.input_path()).unwrap();
    let plan = prepare(&source).unwrap();
    assert_eq!(
        plan.revision,
        format!("{:x}", Sha256::digest(&source_before))
    );
    let rows = plan.document()["models"].as_array().unwrap();
    assert_eq!(
        rows.iter()
            .map(|r| r["slug"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["gpt-5.4", "fixture-model"]
    );
    assert!(rows.iter().all(
        |r| r.get("context_window").is_some() && r.get("supported_reasoning_levels").is_some()
    ));
    let publication = apply(plan).unwrap();
    assert_eq!(publication.destination, f.output());
    assert!(publication.directory_sync_error.is_none());
    assert_eq!(fs::read(source.input_path()).unwrap(), source_before);
    assert_eq!(inspect(&source, Some(""), true).unwrap()[1].status, "ok");
    let written: Value = serde_json::from_slice(&fs::read(f.output()).unwrap()).unwrap();
    assert_eq!(written["models"][1]["slug"], "fixture-model");
}

#[test]
fn native_state_uses_native_slug_and_no_account_path_rules_without_changing_state() {
    let f = Fixture::new();
    let mut value = native_value();
    value["codexRouterNoAccountMode"] = json!(true);
    let source = f.native(value);
    let original = fs::read(source.input_path()).unwrap();
    let report = inspect(&source, Some(""), true).unwrap();
    assert_eq!(report[0].status, "warning");
    let plan = prepare(&source).unwrap();
    assert_eq!(plan.document()["models"][0]["slug"], "native::fixture");
    apply(plan).unwrap();
    assert_eq!(fs::read(source.input_path()).unwrap(), original);
    let config = format!("model_catalog_json = {:?}\n", f.output().to_str().unwrap());
    let report = inspect(&source, Some(&config), true).unwrap();
    assert!(report.iter().all(|i| i.status == "ok"));
    let mut broken: Value = serde_json::from_slice(&fs::read(f.output()).unwrap()).unwrap();
    broken["models"] = json!([]);
    atomic_private(&f.output(), &serde_json::to_vec(&broken).unwrap()).unwrap();
    assert_eq!(
        inspect(&source, Some(&config), false).unwrap()[1].status,
        "warning"
    );
    assert!(!inspect(&source, Some(&config), false).unwrap()[1].fixable);
}

#[test]
fn revision_cas_and_existing_gateway_file_lock_prevent_stale_catalog_publication() {
    use fs2::FileExt;
    for changed in ["source", "output", "cli", "lock"] {
        let f = Fixture::new();
        let mut source = f.gateway();
        if changed == "cli" {
            let bundle = f.0.join("Fixture.app");
            let cli = bundle.join("Contents/Resources/codex");
            private_dir(cli.parent().unwrap()).unwrap();
            fs::write(&cli, b"fixture cli").unwrap();
            if let Source::Gateway { bundle: out, .. } = &mut source {
                *out = Some(bundle)
            }
        }
        let plan = prepare(&source).unwrap();
        let mut lease = None;
        match changed {
            "source" => {
                let mut bytes = fs::read(source.input_path()).unwrap();
                bytes.push(b' ');
                atomic_private(source.input_path(), &bytes).unwrap();
            }
            "output" => atomic_private(&f.output(), b"external replacement").unwrap(),
            "cli" => {
                let path = publication::codex_bundled_cli_path(source.bundle().unwrap()).unwrap();
                fs::write(path, b"changed fixture CLI length").unwrap();
            }
            "lock" => {
                let file = crate::gateway::storage::open_private(
                    &source.input_path().parent().unwrap().join("catalog.lock"),
                )
                .unwrap();
                file.try_lock_exclusive().unwrap();
                lease = Some(file);
            }
            _ => unreachable!(),
        }
        assert!(apply(plan).is_err(), "{changed}");
        assert_eq!(f.output().exists(), changed == "output");
        drop(lease);
    }
}

#[test]
fn different_schema_and_missing_native_state_are_never_swallowed_by_native_defaults() {
    let f = Fixture::new();
    let gateway = f.gateway();
    let as_native = Source::Native {
        state_path: gateway.input_path().to_owned(),
        catalog_root: f.0.clone(),
        bundle: None,
    };
    assert!(prepare(&as_native).is_err());
    assert!(inspect(&as_native, None, true).is_err());
    let native = f.native(native_value());
    let as_gateway = Source::Gateway {
        catalog_path: native.input_path().to_owned(),
        catalog_root: f.0.clone(),
        bundle: None,
        route_enabled: true,
    };
    assert!(prepare(&as_gateway).is_err());
    fs::remove_file(native.input_path()).unwrap();
    assert!(prepare(&native).is_err());
    assert!(!f.output().exists());
}

#[test]
fn explicit_native_migration_uses_synthetic_store_and_does_not_reinterpret_gateway_keys() {
    let f = Fixture::new();
    let source=f.native(json!({"schemaVersion":1,"providers":[{"id":"invalid:id","apiKeyStored":false,"apiKey":"fixture secret"}]}));
    let ns = CredentialNamespace {
        legacy_service: "test.atelier.catalog".into(),
        obfuscation_domain: "fixture-only".into(),
        hostname: "fixture-host".into(),
        username: "fixture-user".into(),
    };
    let mut store = CredentialStore::new(f.0.clone(), ns, MemoryKeychain::default());
    let receipt = migrate_native_source(&source, &mut store).unwrap();
    assert_eq!(receipt.imported_plaintext_keys, 1);
    let state = RelayStateDocument::read(source.input_path()).unwrap();
    let id = state.as_value()["providers"][0]["id"].as_str().unwrap();
    assert_eq!(
        store.get_api_key(id).unwrap().as_deref(),
        Some("fixture secret")
    );
    assert!(state.as_value()["providers"][0].get("apiKey").is_none());
    assert!(store.keychain.operations.is_empty());
    let gateway = f.gateway();
    assert!(migrate_native_source(&gateway, &mut store).is_err());
}

#[test]
fn item_plan_repairs_each_actual_path_mode_and_keeps_original_bytes_in_private_backup() {
    for mode in ["native", "no-account", "gateway"] {
        let f = Fixture::new();
        let source = if mode == "gateway" {
            f.gateway()
        } else {
            let mut value = native_value();
            value["codexRouterNoAccountMode"] = json!(mode == "no-account");
            f.native(value)
        };
        let config = f.0.join("config.toml");
        let before=b"# original fixture\nmodel_catalog_json = \"/foreign/catalog.json\" # obsolete\nmodel = 'keep model'\n\n[profiles.fixture]\nmodel_catalog_json = 'keep profile path'\n# keep trailing comment\n";
        atomic_private(&config, before).unwrap();
        let old_catalog = b" { \"models\": [], \"old\": \"preserve original bytes\" } ";
        atomic_private(&f.output(), old_catalog).unwrap();
        let source_before = fs::read(source.input_path()).unwrap();
        let plan = prepare_item(&source, &config, "catalog_path_validity").unwrap();
        assert_eq!(plan.item_id(), "catalog_path_validity");
        assert_eq!(plan.config_path(), Some(config.as_path()));
        assert_eq!(fs::read(&config).unwrap(), before);
        let preview = std::str::from_utf8(plan.config_after().unwrap()).unwrap();
        assert!(preview.contains("model = 'keep model'"));
        assert!(preview.contains("model_catalog_json = 'keep profile path'"));
        assert!(preview.contains("# keep trailing comment"));
        let parsed: toml::Value = toml::from_str(preview).unwrap();
        if mode == "native" {
            assert!(parsed.get("model_catalog_json").is_none());
        } else {
            assert_eq!(
                parsed["model_catalog_json"].as_str().unwrap(),
                f.output().to_str().unwrap()
            );
        }
        validate(&plan).unwrap();
        let result = apply(plan).unwrap();
        assert!(result.config_publication.is_some());
        assert_eq!(
            fs::read(result.backup_path.join("config.toml.before")).unwrap(),
            before
        );
        assert_eq!(
            fs::read(result.backup_path.join("catalog.json.before")).unwrap(),
            old_catalog
        );
        assert_eq!(
            fs::read(result.backup_path.join("source.json")).unwrap(),
            source_before
        );
        assert_eq!(fs::read(source.input_path()).unwrap(), source_before);
        let final_text = fs::read_to_string(&config).unwrap();
        assert!(inspect(&source, Some(&final_text), true)
            .unwrap()
            .iter()
            .all(|i| i.status == "ok"));
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                fs::metadata(result.backup_path.join("config.toml.before"))
                    .unwrap()
                    .permissions()
                    .mode()
                    & 0o777,
                0o600
            );
        }
    }
}

#[test]
fn item_plan_validates_toml_and_config_cas_before_any_output_mutation() {
    let f = Fixture::new();
    let source = f.gateway();
    let config = f.0.join("config.toml");
    atomic_private(&config, b"broken = [").unwrap();
    assert!(prepare_item(&source, &config, "catalog_path_validity").is_err());
    assert!(!f.output().exists());
    assert!(!f.0.join("maintenance-backups").exists());
    atomic_private(&config, b"# original\nmodel='keep'\n").unwrap();
    assert!(prepare_item(&source, &config, "db_orphan_providers").is_err());
    assert!(prepare_item(&source, source.input_path(), "catalog_integrity").is_err());
    let plan = prepare_item(&source, &config, "catalog_integrity").unwrap();
    atomic_private(&config, b"# external\nmodel='external'\n").unwrap();
    assert!(validate(&plan).is_err());
    assert!(apply(plan).is_err());
    assert_eq!(
        fs::read(&config).unwrap(),
        b"# external\nmodel='external'\n"
    );
    assert!(!f.output().exists());
    assert!(!f.0.join("maintenance-backups").exists());
}

#[test]
fn missing_config_creation_records_absence_and_writes_consumable_gateway_path() {
    let f = Fixture::new();
    let source = f.gateway();
    let config = f.0.join("config.toml");
    let plan = prepare_item(&source, &config, "catalog_integrity").unwrap();
    let result = apply(plan).unwrap();
    let parsed: toml::Value = toml::from_str(&fs::read_to_string(&config).unwrap()).unwrap();
    assert_eq!(
        parsed["model_catalog_json"].as_str().unwrap(),
        f.output().to_str().unwrap()
    );
    assert!(!result.backup_path.join("config.toml.before").exists());
    let manifest: Value =
        serde_json::from_slice(&fs::read(result.backup_path.join("manifest.json")).unwrap())
            .unwrap();
    let config_record = manifest
        .as_array()
        .unwrap()
        .iter()
        .find(|m| m["path"] == json!(config))
        .unwrap();
    assert_eq!(config_record["present"], false);
    assert!(config_record["backup"].is_null());
}

#[cfg(unix)]
#[test]
fn config_publication_failure_compensates_catalog_and_retains_exact_backups() {
    use std::os::unix::fs::PermissionsExt;
    let f = Fixture::new();
    let source = f.gateway();
    let dir = f.0.join("config-parent");
    private_dir(&dir).unwrap();
    let config = dir.join("config.toml");
    let before = b"# fixture\nmodel='keep'\n";
    atomic_private(&config, before).unwrap();
    let original_catalog = b"original fixture catalog bytes";
    atomic_private(&f.output(), original_catalog).unwrap();
    let plan = prepare_item(&source, &config, "catalog_path_validity").unwrap();
    fs::set_permissions(&dir, fs::Permissions::from_mode(0o500)).unwrap();
    let result = apply(plan);
    fs::set_permissions(&dir, fs::Permissions::from_mode(0o700)).unwrap();
    let error = result.unwrap_err();
    assert!(error.contains("已恢复原模型目录"), "{error}");
    assert_eq!(fs::read(&config).unwrap(), before);
    assert_eq!(fs::read(f.output()).unwrap(), original_catalog);
    let backup = fs::read_dir(f.0.join("maintenance-backups"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    assert_eq!(fs::read(backup.join("config.toml.before")).unwrap(), before);
}

#[test]
fn context_resolver_uses_only_supplied_roots_and_never_defaults_a_corrupt_native_state() {
    let f = Fixture::new();
    let gateway = f.gateway();
    let home = f.0.join("home");
    private_dir(&home).unwrap();
    let mut ctx = super::super::Context {
        home: home.clone(),
        app_root: f.0.clone(),
        desktop_running: false,
        runtime: None,
        official_catalog: None,
        proxy_handle: None,
        working_directory: None,
        sqlite_home_env: None,
    };
    atomic_private(
        &home.join("config.toml"),
        b"model_provider='atelier_fixture'\n",
    )
    .unwrap();
    let resolved = context_source(&ctx).unwrap();
    assert_eq!(resolved.input_path(), gateway.input_path());
    let native_path = native_state_path(&home);
    private_dir(native_path.parent().unwrap()).unwrap();
    let mut native = native_value();
    native["codexRouterEnabled"] = json!(true);
    atomic_private(&native_path, &serde_json::to_vec(&native).unwrap()).unwrap();
    assert!(matches!(
        context_source(&ctx).unwrap(),
        Source::Native { .. }
    ));
    native["codexRouterEnabled"] = json!(false);
    atomic_private(&native_path, &serde_json::to_vec(&native).unwrap()).unwrap();
    assert!(matches!(
        context_source(&ctx).unwrap(),
        Source::Gateway { .. }
    ));
    ctx.runtime = None;
    atomic_private(&home.join("config.toml"), b"model_provider='openai'\n").unwrap();
    assert!(matches!(
        context_source(&ctx).unwrap(),
        Source::Native { .. }
    ));
    atomic_private(&native_path, b"{ corrupt native").unwrap();
    assert!(context_source(&ctx).is_err());
}

#[test]
fn empty_gateway_inventory_reports_only_catalog_consistency_not_generation_acceptance() {
    let f = Fixture::new();
    let source = f.gateway();
    let mut data: Value = serde_json::from_slice(&fs::read(source.input_path()).unwrap()).unwrap();
    data["settings"]["preferredProfileId"] = Value::Null;
    atomic_private(source.input_path(), &serde_json::to_vec(&data).unwrap()).unwrap();
    let config = f.0.join("config.toml");
    let plan = prepare_item(&source, &config, "catalog_integrity").unwrap();
    assert_eq!(plan.document()["models"], json!([]));
    apply(plan).unwrap();
    let text = fs::read_to_string(config).unwrap();
    let result = inspect(&source, Some(&text), true).unwrap();
    assert_eq!(result[1].status, "ok");
    assert!(result[1].detail.contains("不代表整个路由已可生成"));
    assert!(result[1].detail.contains("preferred profile"));
}
