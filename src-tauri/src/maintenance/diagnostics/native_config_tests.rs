//! Actual diagnostic entry against explicit temporary source inventories/configs.
use super::*;
use crate::gateway::types::{Catalog, Profile, ProfileKind, Settings};
use serde_json::json;
use std::fs;
struct Fixture {
    root: PathBuf,
    ctx: Context,
}
impl Fixture {
    fn new(text: &str) -> Self {
        let root =
            std::env::temp_dir().join(format!("atelier-native-config-{}", uuid::Uuid::new_v4()));
        let ctx = Context {
            home: root.join("codex"),
            app_root: root.join("atelier"),
            desktop_running: false,
            runtime: None,
            official_catalog: None,
            proxy_handle: None,
            working_directory: Some(root.clone()),
            sqlite_home_env: None,
        };
        fs::create_dir_all(&ctx.home).unwrap();
        fs::create_dir_all(&ctx.app_root).unwrap();
        fs::write(ctx.config(), text).unwrap();
        Self { root, ctx }
    }
    fn native(&self, on: bool, ids: &[&str]) -> PathBuf {
        let path = self.ctx.home.join("codexmate/relay/state.json");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let mut state =
            atelier_maintenance_core::catalog::state::RelayStateDocument::native_default()
                .as_value()
                .clone();
        state["codexRouterEnabled"] = json!(on);
        state["providers"] = json!(ids
            .iter()
            .map(|id| json!({"id": id, "name": id}))
            .collect::<Vec<_>>());
        fs::write(&path, serde_json::to_vec(&state).unwrap()).unwrap();
        path
    }
    fn gateway(&self, ids: &[&str]) -> PathBuf {
        let path = self.ctx.app_root.join("gateway/catalog.json");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let profiles = ids
            .iter()
            .map(|id| Profile {
                id: id.to_string(),
                kind: ProfileKind::ApiKey,
                name: id.to_string(),
                email: None,
                enabled: false,
                priority: 0,
                base_url: "https://fixture.invalid/v1".into(),
                auth_status: "warning".into(),
                models: vec![],
                cooldown_until: None,
                last_error: None,
                requests: 0,
                input_tokens: 0,
                output_tokens: 0,
                created_at: 0,
            })
            .collect();
        let catalog = Catalog {
            revision: "fixture".into(),
            host_id: "fixture-host".into(),
            profiles,
            settings: Settings::default(),
        };
        fs::write(&path, serde_json::to_vec(&catalog).unwrap()).unwrap();
        path
    }
    fn report(&self) -> DiagnosticReport {
        super::super::inspect::run(&self.ctx).unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
fn item<'a>(report: &'a DiagnosticReport, id: &str) -> &'a DiagnosticItem {
    report.items.iter().find(|i| i.id == id).unwrap()
}

#[test]
fn native_redirect_detection_is_textual_and_redacts_url_credentials() {
    let f = Fixture::new("openai_base_url=\"https://user:password@api.openai.com/v1?secret=value#token\"\nmodel='keep'\n");
    let before = fs::read(f.ctx.config()).unwrap();
    let report = f.report();
    assert_eq!(report.items.len(), 34);
    let issue = item(&report, "config_third_party");
    assert_eq!(issue.status, "error");
    assert!(issue.fixable);
    assert!(issue.detail.contains("api.openai.com"));
    for secret in ["password", "secret", "value", "token"] {
        assert!(!issue.detail.contains(secret));
    }
    assert_eq!(fs::read(f.ctx.config()).unwrap(), before);
}

#[test]
fn native_fingerprint_is_structural_substring_even_for_an_owned_provider_label() {
    let f = Fixture::new("model_provider='personal'\n[model_providers.personal]\nname='cc-switch is only a label'\nbase_url='https://fixture.invalid/v1'\n");
    f.native(false, &["personal"]);
    assert_eq!(
        known_provider_ids(&f.ctx).unwrap(),
        HashSet::from(["personal".into()])
    );
    let report = f.report();
    let issue = item(&report, "config_third_party");
    assert_eq!(issue.status, "error");
    assert!(issue.detail.contains("cc-switch 指纹"));
    assert!(issue.fixable);
}

#[test]
fn comments_multiline_fake_markers_and_native_router_url_are_preserved_as_clean() {
    let f = Fixture::new("# cc-switch\nmodel='keep'\nopenai_base_url='http://127.0.0.1:9/codex/router/v1'\ntext='''\nprovider_router\n[model_providers.deecodex]\n'''\n[model_providers.personal]\nbase_url='https://fixture.invalid/v1'\n");
    let before = fs::read(f.ctx.config()).unwrap();
    let report = f.report();
    assert_eq!(item(&report, "config_third_party").status, "ok");
    assert!(!item(&report, "config_third_party").fixable);
    assert_eq!(fs::read(f.ctx.config()).unwrap(), before);
}

#[test]
fn missing_config_and_readonly_adapter_do_not_claim_fixable_health() {
    let f = Fixture::new("model='keep'\n");
    fs::remove_file(f.ctx.config()).unwrap();
    let report = f.report();
    let missing = item(&report, "config_third_party");
    assert_eq!(missing.status, "warning");
    assert!(!missing.fixable);
    let readonly = third_party(
        &f.ctx,
        Some("openai_base_url='https://fixture.invalid'"),
        false,
    )
    .unwrap();
    assert_eq!(readonly.status, "error");
    assert!(!readonly.fixable);
    fs::write(f.ctx.config(), [0xff, 0xfe]).unwrap();
    let unreadable = f.report();
    assert_eq!(item(&unreadable, "config_third_party").status, "warning");
    assert!(!item(&unreadable, "config_third_party").fixable);
}

#[test]
fn known_owner_ids_come_from_saved_native_inventory_not_current_config_headers() {
    use sha2::{Digest, Sha256};
    let f = Fixture::new("[model_providers.config_only]\nname='legitimate'\n");
    let path = f.native(false, &["saved-owner", "inactive-owner"]);
    let bytes = fs::read(&path).unwrap();
    let (ids, deps) = known_provider_ids_with_dependencies(&f.ctx).unwrap();
    assert_eq!(
        ids,
        HashSet::from(["saved-owner".into(), "inactive-owner".into()])
    );
    assert_eq!(
        deps,
        vec![
            (
                path,
                Some((bytes.len(), format!("{:x}", Sha256::digest(&bytes))))
            ),
            (super::super::router_intent::path(&f.ctx), None)
        ]
    );
}

#[test]
fn absent_sources_and_gateway_inventory_have_actual_dependency_stamps() {
    let f = Fixture::new("model='keep'\n");
    let (ids, deps) = known_provider_ids_with_dependencies(&f.ctx).unwrap();
    assert!(ids.is_empty());
    assert_eq!(
        deps,
        vec![
            (f.ctx.home.join("codexmate/relay/state.json"), None),
            (super::super::router_intent::path(&f.ctx), None),
            (f.ctx.app_root.join("gateway/catalog.json"), None)
        ]
    );
    let path = f.gateway(&["gateway-owner", "disabled-owner"]);
    let (ids, changed) = known_provider_ids_with_dependencies(&f.ctx).unwrap();
    assert_eq!(
        ids,
        HashSet::from(["gateway-owner".into(), "disabled-owner".into()])
    );
    assert_eq!(changed[0], deps[0]);
    assert_eq!(changed[1], deps[1]);
    assert_eq!(changed[2].0, path);
    assert!(changed[2].1.is_some());
    assert_ne!(changed, deps);
}

#[test]
fn enabled_gateway_selection_binds_disabled_native_and_gateway_dependencies() {
    let f = Fixture::new("model_provider='atelier'\n");
    let native = f.native(false, &["inactive-native"]);
    let gateway = f.gateway(&["actual-gateway"]);
    let (ids, deps) = known_provider_ids_with_dependencies(&f.ctx).unwrap();
    assert_eq!(ids, HashSet::from(["actual-gateway".into()]));
    assert_eq!(
        deps.iter().map(|d| d.0.clone()).collect::<Vec<_>>(),
        [native, super::super::router_intent::path(&f.ctx), gateway]
    );
    assert!(deps[0].1.is_some());
    assert!(deps[1].1.is_none());
    assert!(deps[2].1.is_some());
}

#[test]
fn source_parse_errors_replace_old_catalog_heuristic_ok_and_do_not_change_freshness() {
    for native in [true, false] {
        let f = Fixture::new("model='keep'\n");
        let catalog = f.ctx.home.join("custom-models.json");
        fs::write(&catalog, br#"{"models":[]}"#).unwrap();
        fs::write(
            f.ctx.config(),
            format!("model='keep'\nmodel_catalog_json={:?}\n", catalog),
        )
        .unwrap();
        let source = if native {
            f.native(false, &[])
        } else {
            f.gateway(&[])
        };
        fs::write(&source, b"{broken source").unwrap();
        assert!(known_provider_ids_with_dependencies(&f.ctx).is_err());
        let before = fs::read(&source).unwrap();
        let report = f.report();
        for id in ["catalog_path_validity", "catalog_integrity"] {
            let failed = item(&report, id);
            assert_eq!(failed.status, "warning");
            assert!(!failed.fixable);
            assert!(failed.detail.contains("来源选择失败"));
        }
        assert_eq!(item(&report, "config_third_party").status, "warning");
        assert_eq!(
            item(&report, "official_catalog_freshness").status,
            "warning"
        );
        assert!(item(&report, "official_catalog_freshness")
            .detail
            .contains("未联网"));
        assert_eq!(fs::read(source).unwrap(), before);
    }
}

#[cfg(unix)]
#[test]
fn source_symlink_cannot_become_an_empty_known_set_or_healthy_catalog() {
    let f = Fixture::new("model='keep'\n");
    let external = f.root.join("external.json");
    fs::write(&external, b"unreadable as a source").unwrap();
    let source = f.ctx.home.join("codexmate/relay/state.json");
    fs::create_dir_all(source.parent().unwrap()).unwrap();
    std::os::unix::fs::symlink(&external, &source).unwrap();
    assert!(known_provider_ids(&f.ctx).is_err());
    let report = f.report();
    assert_eq!(item(&report, "catalog_integrity").status, "warning");
    assert_eq!(item(&report, "config_third_party").status, "warning");
    assert_eq!(fs::read(external).unwrap(), b"unreadable as a source");
}

#[test]
fn actual_third_party_repair_preserves_unrelated_bytes_and_backs_up_original() {
    let original = "# keep fixture comment\nopenai_base_url=\"https://third.invalid/v1\"\nmodel='keep-model'\n[model_providers.personal]\nbase_url='https://personal.invalid/v1'\nwire_api='responses'\n";
    let f = Fixture::new(original);
    let ticket = super::super::repair::preview(&f.ctx, "config_third_party").unwrap();
    assert_eq!(fs::read(f.ctx.config()).unwrap(), original.as_bytes());
    let result = super::super::repair::apply(&f.ctx, ticket).unwrap();
    assert!(result.fixed_ids.iter().any(|id| id == "config_third_party"));
    assert!(!result
        .failed_ids
        .iter()
        .any(|id| id == "config_third_party"));
    let after = fs::read_to_string(f.ctx.config()).unwrap();
    assert!(after.contains("# keep fixture comment\n"));
    assert!(after.contains("model='keep-model'\n"));
    assert!(after.contains("[model_providers.personal]\nbase_url='https://personal.invalid/v1'\nwire_api='responses'\n"));
    assert!(!after.contains("openai_base_url"));
    assert_eq!(item(&result.report, "config_third_party").status, "ok");
    let backup = PathBuf::from(result.backup_path.unwrap());
    let outcome: serde_json::Value =
        serde_json::from_slice(&fs::read(backup.join("coordinated-result.json")).unwrap()).unwrap();
    let original_root = PathBuf::from(outcome["backup_path"].as_str().unwrap());
    assert!(original_root.starts_with(&backup));
    assert_eq!(
        fs::read(original_root.join("config.toml.before")).unwrap(),
        original.as_bytes()
    );
}

#[test]
fn third_party_repair_rejects_new_native_owner_inventory_after_empty_source_preview() {
    let original = "openai_base_url='https://third.invalid/v1'\nmodel='keep'\n";
    let f = Fixture::new(original);
    let ticket = super::super::repair::preview(&f.ctx, "config_third_party").unwrap();
    let source = f.native(false, &["new-owner"]);
    let bytes = fs::read(&source).unwrap();
    assert!(super::super::repair::apply(&f.ctx, ticket).is_err());
    assert_eq!(fs::read(f.ctx.config()).unwrap(), original.as_bytes());
    assert_eq!(fs::read(source).unwrap(), bytes);
    assert!(!f
        .ctx
        .app_root
        .join("maintenance/diagnostic-backups")
        .exists());
}
