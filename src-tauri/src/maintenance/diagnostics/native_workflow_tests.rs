//! Exercises the actual preview/apply entry, using only synthetic temporary roots.
use super::*;
use atelier_maintenance_core::transition_journal::{JournalStore, TransitionTarget};
use std::fs;
struct Fixture {
    root: PathBuf,
    ctx: Context,
}
impl Fixture {
    fn new() -> Self {
        let root =
            std::env::temp_dir().join(format!("atelier-native-workflow-{}", uuid::Uuid::new_v4()));
        let ctx = Context {
            home: root.join("codex"),
            app_root: root.join("app"),
            desktop_running: false,
            runtime: None,
            official_catalog: None,
            proxy_handle: None,
            working_directory: None,
            sqlite_home_env: None,
        };
        fs::create_dir_all(ctx.home.join("codexmate/relay")).unwrap();
        fs::create_dir_all(&ctx.app_root).unwrap();
        fs::write(ctx.config(), "model_provider='openai'\n").unwrap();
        Self { root, ctx }
    }
    fn relay(&self, name: &str) -> PathBuf {
        self.ctx.home.join("codexmate/relay").join(name)
    }
    fn takeover(&self) {
        fs::write(self.relay("router-takeover-backup.json"),serde_json::to_vec(&serde_json::json!({
            "version":1,"createdAtMs":0,"topLevelLines":["model='fixture-model'"],"sectionBlocks":[],"originalConfig":"fixture original"
        })).unwrap()).unwrap();
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
#[test]
fn native_unlock_is_reachable_through_the_actual_diagnostic_repair_entry() {
    let f = Fixture::new();
    let user = br#"{"OPENAI_API_KEY":"sk-synthetic-private-user"}"#;
    let virtual_auth=br#"{"auth_mode":"apikey","OPENAI_API_KEY":"sk-aimami-relay-virtual-synthetic","tokens":{}}"#;
    fs::write(f.ctx.home.join("auth.json"), virtual_auth).unwrap();
    fs::write(
        f.relay("router-unlock-auth-marker.json"),
        br#"{"backedUpPreviousAuth":true}"#,
    )
    .unwrap();
    fs::write(f.relay("router-unlock-auth-backup.json"), user).unwrap();
    let report = inspect::run(&f.ctx).unwrap();
    assert!(report
        .items
        .iter()
        .any(|i| i.id == "router_unlock_auth_residue" && i.fixable));
    let ticket = repair::preview(&f.ctx, "router_unlock_auth_residue").unwrap();
    assert!(!serde_json::to_string(&ticket.view).unwrap().contains("sk-"));
    assert_eq!(
        fs::read(f.ctx.home.join("auth.json")).unwrap(),
        virtual_auth
    );
    let result = repair::apply(&f.ctx, ticket).unwrap();
    assert_eq!(result.fixed_ids, ["router_unlock_auth_residue"]);
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), user);
    assert!(!f.relay("router-unlock-auth-marker.json").exists());
    assert!(!serde_json::to_string(&result).unwrap().contains("sk-"));
    let backup = PathBuf::from(result.backup_path.unwrap());
    assert!(backup.join("coordinated-result.json").exists());
}
#[test]
fn takeover_actual_entry_restores_missing_keys_and_preserves_existing_values() {
    let f = Fixture::new();
    f.takeover();
    let ticket = repair::preview(&f.ctx, "takeover_backup_orphan").unwrap();
    let result = repair::apply(&f.ctx, ticket).unwrap();
    assert_eq!(result.fixed_ids, ["takeover_backup_orphan"]);
    let config: toml::Value = toml::from_str(&fs::read_to_string(f.ctx.config()).unwrap()).unwrap();
    assert_eq!(config["model"].as_str(), Some("fixture-model"));
    assert_eq!(config["model_provider"].as_str(), Some("openai"));
    assert!(!f.relay("router-takeover-backup.json").exists());
}
#[test]
fn pending_commit_appearing_after_preview_blocks_the_actual_repair_before_mutation() {
    let f = Fixture::new();
    f.takeover();
    let ticket = repair::preview(&f.ctx, "takeover_backup_orphan").unwrap();
    let config = fs::read(f.ctx.config()).unwrap();
    JournalStore::new(&f.ctx.home.join("codexmate/relay"))
        .begin_commit(TransitionTarget::NativeOff, None, 1)
        .unwrap();
    assert!(inspect::run(&f.ctx).unwrap().transition_in_flight);
    let error = repair::apply(&f.ctx, ticket).unwrap_err();
    assert!(error.contains("路由转换进行中"));
    assert_eq!(fs::read(f.ctx.config()).unwrap(), config);
    assert!(f.relay("router-takeover-backup.json").exists());
}
#[test]
fn completed_journal_does_not_block_but_malformed_journal_is_not_overwritten() {
    let f = Fixture::new();
    f.takeover();
    let store = JournalStore::new(&f.ctx.home.join("codexmate/relay"));
    store
        .write(TransitionTarget::NativeOff, "stable", None, 1)
        .unwrap()
        .acknowledge();
    assert!(!inspect::run(&f.ctx).unwrap().transition_in_flight);
    let ticket = repair::preview(&f.ctx, "takeover_backup_orphan").unwrap();
    fs::write(store.path(), b"{broken").unwrap();
    let error = repair::apply(&f.ctx, ticket).unwrap_err();
    assert!(error.contains("恢复记录无法核对"));
    assert_eq!(fs::read(store.path()).unwrap(), b"{broken");
    assert!(f.relay("router-takeover-backup.json").exists());
}
#[test]
fn catalog_rebuild_actual_entry_uses_gateway_inventory_and_routable_model_ids() {
    use crate::gateway::types::{Catalog, GatewayModel, Profile, ProfileKind, Settings};
    let f = Fixture::new();
    fs::write(
        f.ctx.config(),
        "model_provider='atelier'\n# keep this comment\n",
    )
    .unwrap();
    fs::create_dir_all(f.ctx.app_root.join("gateway")).unwrap();
    let source = f.ctx.app_root.join("gateway/catalog.json");
    let catalog = Catalog {
        revision: "fixture-revision".into(),
        host_id: "fixture-host".into(),
        profiles: vec![Profile {
            id: "fixture-profile".into(),
            kind: ProfileKind::ApiKey,
            name: "Fixture".into(),
            email: None,
            enabled: true,
            priority: 0,
            base_url: "https://example.invalid/v1".into(),
            auth_status: "warning".into(),
            models: vec![GatewayModel {
                id: "fixture-model".into(),
                name: "Fixture model".into(),
            }],
            cooldown_until: None,
            last_error: None,
            requests: 0,
            input_tokens: 0,
            output_tokens: 0,
            created_at: 0,
        }],
        settings: Settings {
            preferred_profile_id: Some("fixture-profile".into()),
            ..Settings::default()
        },
    };
    let original = serde_json::to_vec(&catalog).unwrap();
    fs::write(&source, &original).unwrap();
    let report = inspect::run(&f.ctx).unwrap();
    assert!(report
        .items
        .iter()
        .any(|i| i.id == "catalog_integrity" && i.fixable));
    let ticket = repair::preview(&f.ctx, "catalog_integrity").unwrap();
    let result = repair::apply(&f.ctx, ticket).unwrap();
    assert_eq!(result.fixed_ids, ["catalog_integrity"]);
    let text = fs::read_to_string(f.ctx.config()).unwrap();
    assert!(text.contains("# keep this comment"));
    let config: toml::Value = toml::from_str(&text).unwrap();
    let path = PathBuf::from(config["model_catalog_json"].as_str().unwrap());
    let actual: serde_json::Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
    assert_eq!(actual["models"][0]["slug"], "fixture-model");
    assert_eq!(fs::read(source).unwrap(), original);
    assert!(PathBuf::from(result.backup_path.unwrap())
        .join("coordinated-result.json")
        .exists());
}
#[test]
fn actual_all_entry_composes_takeover_and_config_repairs_without_stale_overwrite() {
    let f = Fixture::new();
    f.takeover();
    fs::write(f.ctx.config(),"# preserve fixture comment\nmodel_provider='openai'\nprofile='legacy'\nmodel_reasoning_effort=''\n[profiles.keep]\nmodel='keep-model'\n").unwrap();
    let ticket = repair::preview(&f.ctx, "all").unwrap();
    for id in [
        "takeover_backup_orphan",
        "config_global_validity",
        "config_profile_conflict",
    ] {
        assert!(ticket.view.item_ids.iter().any(|selected| selected == id));
    }
    let result = repair::apply(&f.ctx, ticket).unwrap();
    for id in [
        "takeover_backup_orphan",
        "config_global_validity",
        "config_profile_conflict",
    ] {
        assert!(
            result.fixed_ids.iter().any(|fixed| fixed == id),
            "{id}: {:?}",
            result.details
        );
    }
    let text = fs::read_to_string(f.ctx.config()).unwrap();
    let config: toml::Value = toml::from_str(&text).unwrap();
    assert!(text.contains("# preserve fixture comment"));
    assert!(config.get("profile").is_none());
    assert!(config.get("model_reasoning_effort").is_none());
    assert_eq!(config["model"].as_str(), Some("fixture-model"));
    assert_eq!(
        config["profiles"]["keep"]["model"].as_str(),
        Some("keep-model")
    );
    assert!(!f.relay("router-takeover-backup.json").exists());
    let backup = PathBuf::from(result.backup_path.unwrap());
    let receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(backup.join("coordinated-result.json")).unwrap()).unwrap();
    let original =
        PathBuf::from(receipt["backup_path"].as_str().unwrap()).join("config.toml.before");
    assert!(fs::read_to_string(original)
        .unwrap()
        .contains("profile='legacy'"));
}
#[test]
fn global_reference_exclusion_is_relay_directory_rather_than_entire_codex_home() {
    for managed in [false, true] {
        let f = Fixture::new();
        let reference = if managed {
            f.relay("missing-catalog.json")
        } else {
            f.ctx.home.join("missing-catalog.json")
        };
        let before = format!(
            "model_provider='openai'\nmodel_catalog_json={}\n",
            serde_json::to_string(&reference.to_string_lossy()).unwrap()
        );
        fs::write(f.ctx.config(), before.as_bytes()).unwrap();
        let report = inspect::run(&f.ctx).unwrap();
        let item = report
            .items
            .iter()
            .find(|i| i.id == "config_global_validity")
            .unwrap();
        if managed {
            assert_eq!(item.status, "ok");
            assert!(!item.fixable);
            assert_eq!(fs::read_to_string(f.ctx.config()).unwrap(), before);
        } else {
            assert!(item.fixable);
            let ticket = repair::preview(&f.ctx, "config_global_validity").unwrap();
            let result = repair::apply(&f.ctx, ticket).unwrap();
            assert_eq!(result.fixed_ids, ["config_global_validity"]);
            let after = fs::read_to_string(f.ctx.config()).unwrap();
            assert!(!after.contains("model_catalog_json"));
            assert!(after.contains("model_provider='openai'"));
        }
    }
}
