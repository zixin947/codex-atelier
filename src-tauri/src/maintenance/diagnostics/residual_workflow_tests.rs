//! Only synthetic roots are used; never invoke Context::native or process control.
use super::*;
use std::fs;

struct Fixture {
    root: PathBuf,
    ctx: Context,
}
impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!("atelier-residual-{}", uuid::Uuid::new_v4()));
        let ctx = Context {
            home: root.join("codex"),
            app_root: root.join("atelier"),
            desktop_running: false,
            runtime: None,
            official_catalog: None,
            proxy_handle: None,
            working_directory: None,
            sqlite_home_env: None,
        };
        fs::create_dir_all(&ctx.home).unwrap();
        fs::create_dir_all(&ctx.app_root).unwrap();
        fs::write(ctx.config(), "model = 'fixture'\n").unwrap();
        for kind in residual_cleanup::Kind::ALL {
            for path in kind.paths(&ctx) {
                fs::create_dir_all(path.parent().unwrap()).unwrap();
                fs::write(path, kind.id()).unwrap();
            }
        }
        fs::write(
            ctx.app_root.join("gateway/legacy-thread-migration.json"),
            b"legacy-must-remain",
        )
        .unwrap();
        Self { root, ctx }
    }
    fn assert_legacy_preserved(&self) {
        assert_eq!(
            fs::read(
                self.ctx
                    .app_root
                    .join("gateway/legacy-thread-migration.json")
            )
            .unwrap(),
            b"legacy-must-remain"
        );
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn assert_single_item(kind: residual_cleanup::Kind, other: residual_cleanup::Kind) {
    let f = Fixture::new();
    let ticket = repair::preview(&f.ctx, kind.id()).unwrap();
    assert_eq!(ticket.view.item_ids, [kind.id()]);
    assert!(
        !f.ctx.app_root.join("maintenance").exists(),
        "preview cannot create backup/lock files"
    );
    for path in kind.paths(&f.ctx) {
        assert_eq!(fs::read(path).unwrap(), kind.id().as_bytes());
    }
    let result = repair::apply(&f.ctx, ticket).unwrap();
    for path in kind.paths(&f.ctx) {
        assert!(!path.exists());
    }
    for path in other.paths(&f.ctx) {
        assert_eq!(fs::read(path).unwrap(), other.id().as_bytes());
    }
    f.assert_legacy_preserved();
    assert_eq!(result.fixed_ids, [kind.id()]);
    let backup = PathBuf::from(result.backup_path.unwrap());
    for i in 0..2 {
        assert_eq!(
            fs::read(backup.join(format!("{i:04}.bin"))).unwrap(),
            kind.id().as_bytes()
        );
    }
    assert_eq!(
        result
            .report
            .items
            .iter()
            .find(|i| i.id == kind.id())
            .unwrap()
            .status,
        "ok"
    );
}

#[test]
fn selecting_cache_only_deletes_cache_and_keeps_manifest_and_legacy() {
    assert_single_item(
        residual_cleanup::Kind::Cache,
        residual_cleanup::Kind::Manifest,
    );
}
#[test]
fn selecting_manifest_only_deletes_manifest_and_keeps_cache_and_legacy() {
    assert_single_item(
        residual_cleanup::Kind::Manifest,
        residual_cleanup::Kind::Cache,
    );
}
#[test]
fn all_cleans_both_residual_kinds_but_never_deletes_legacy_migration_records() {
    let f = Fixture::new();
    let ticket = repair::preview(&f.ctx, "all").unwrap();
    let result = repair::apply(&f.ctx, ticket).unwrap();
    for kind in residual_cleanup::Kind::ALL {
        for path in kind.paths(&f.ctx) {
            assert!(!path.exists());
        }
        assert!(result.fixed_ids.iter().any(|id| id == kind.id()));
    }
    f.assert_legacy_preserved();
}
#[test]
fn enabled_router_keeps_manifest_and_cache_and_has_no_cleanup_action() {
    for provider in ["aimai1", "atelier"] {
        let f = Fixture::new();
        fs::write(
            f.ctx.config(),
            format!(
                "model_provider = '{provider}'\n[model_providers.{provider}]\nname = 'fixture'\n"
            ),
        )
        .unwrap();
        let mut native =
            atelier_maintenance_core::catalog::state::RelayStateDocument::native_default()
                .as_value()
                .clone();
        native["codexRouterEnabled"] = serde_json::json!(true);
        fs::write(
            f.ctx.home.join("codexmate/relay/state.json"),
            serde_json::to_vec(&native).unwrap(),
        )
        .unwrap();
        let intent = super::router_intent::IntentRecord {
            schema_version: 1,
            target: atelier_maintenance_core::transition_journal::TransitionTarget::RouterOn,
            provider: Some("atelier".into()),
            operation_id: None,
            catalog_revision: None,
            updated_at_ms: 1,
        };
        fs::write(
            super::router_intent::path(&f.ctx),
            serde_json::to_vec(&intent).unwrap(),
        )
        .unwrap();
        let report = inspect::run(&f.ctx).unwrap();
        for kind in residual_cleanup::Kind::ALL {
            let item = report.items.iter().find(|i| i.id == kind.id()).unwrap();
            assert_eq!(item.status, "ok");
            assert!(!item.fixable);
            assert!(repair::preview(&f.ctx, kind.id()).is_err());
            for path in kind.paths(&f.ctx) {
                assert_eq!(fs::read(path).unwrap(), kind.id().as_bytes());
            }
        }
    }
}
#[test]
fn router_started_after_preview_blocks_deletion_without_creating_backups() {
    let mut f = Fixture::new();
    let ticket = repair::preview(&f.ctx, "residual_cache").unwrap();
    let intent = super::router_intent::IntentRecord {
        schema_version: 1,
        target: atelier_maintenance_core::transition_journal::TransitionTarget::RouterOn,
        provider: Some("atelier".into()),
        operation_id: None,
        catalog_revision: None,
        updated_at_ms: 1,
    };
    fs::write(
        super::router_intent::path(&f.ctx),
        serde_json::to_vec(&intent).unwrap(),
    )
    .unwrap();
    assert!(repair::apply(&f.ctx, ticket).unwrap_err().contains("路由"));
    for path in residual_cleanup::Kind::Cache.paths(&f.ctx) {
        assert_eq!(fs::read(path).unwrap(), b"residual_cache");
    }
    assert!(!f
        .ctx
        .app_root
        .join("maintenance/diagnostic-backups")
        .exists());
}
#[test]
fn externally_updated_residual_is_preserved_and_no_other_target_is_deleted() {
    let f = Fixture::new();
    let ticket = repair::preview(&f.ctx, "residual_cache").unwrap();
    let [changed, other] = residual_cleanup::Kind::Cache.paths(&f.ctx);
    fs::write(&changed, b"external").unwrap();
    assert!(repair::apply(&f.ctx, ticket).is_err());
    assert_eq!(fs::read(changed).unwrap(), b"external");
    assert_eq!(fs::read(other).unwrap(), b"residual_cache");
    f.assert_legacy_preserved();
}

#[test]
fn residual_only_repair_runs_with_a_simulated_open_desktop() {
    for id in ["residual_cache", "residual_manifest", "all"] {
        let mut f = Fixture::new();
        f.ctx.desktop_running = true; // Synthetic context; no native process discovery/control.
        let ticket = repair::preview(&f.ctx, id).unwrap();
        assert!(!ticket.view.requires_desktop_closed);
        let result = repair::apply(&f.ctx, ticket).unwrap();
        assert!(result.failed_ids.is_empty());
        for kind in residual_cleanup::Kind::ALL {
            for path in kind.paths(&f.ctx) {
                if id == "all" || id == kind.id() {
                    assert!(!path.exists());
                    assert!(result.fixed_ids.iter().any(|fixed| fixed == kind.id()));
                } else {
                    assert_eq!(fs::read(path).unwrap(), kind.id().as_bytes());
                }
            }
        }
        f.assert_legacy_preserved();
    }
}

#[test]
fn mixed_auth_and_residual_repair_still_requires_a_quiet_window() {
    let mut f = Fixture::new();
    f.ctx.desktop_running = true;
    fs::write(f.ctx.home.join("auth.json"), b"synthetic-broken-auth").unwrap();
    fs::create_dir_all(f.ctx.home.join("accounts/backups")).unwrap();
    fs::write(
        f.ctx.home.join("accounts/backups/a-backup.json"),
        br#"{"tokens":{"access_token":"synthetic-auth"}}"#,
    )
    .unwrap();
    let ticket = repair::preview(&f.ctx, "all").unwrap();
    assert!(ticket.view.item_ids.iter().any(|id| id == "auth_integrity"));
    assert!(ticket.view.requires_desktop_closed);
    assert!(repair::apply(&f.ctx, ticket)
        .unwrap_err()
        .contains("Desktop"));
    assert_eq!(
        fs::read(f.ctx.home.join("auth.json")).unwrap(),
        b"synthetic-broken-auth"
    );
    for kind in residual_cleanup::Kind::ALL {
        for path in kind.paths(&f.ctx) {
            assert_eq!(fs::read(path).unwrap(), kind.id().as_bytes());
        }
    }
    assert!(!f.ctx.app_root.join("maintenance").exists());
}

#[test]
fn no_quiet_exception_does_not_bypass_the_router_gate() {
    let mut f = Fixture::new();
    f.ctx.desktop_running = true;
    let ticket = repair::preview(&f.ctx, "residual_cache").unwrap();
    assert!(!ticket.view.requires_desktop_closed);
    let intent = super::router_intent::IntentRecord {
        schema_version: 1,
        target: atelier_maintenance_core::transition_journal::TransitionTarget::RouterOn,
        provider: Some("atelier".into()),
        operation_id: None,
        catalog_revision: None,
        updated_at_ms: 1,
    };
    fs::write(
        super::router_intent::path(&f.ctx),
        serde_json::to_vec(&intent).unwrap(),
    )
    .unwrap();
    assert!(repair::apply(&f.ctx, ticket).unwrap_err().contains("路由"));
    for path in residual_cleanup::Kind::Cache.paths(&f.ctx) {
        assert_eq!(fs::read(path).unwrap(), b"residual_cache");
    }
}

#[test]
fn a_running_listener_with_routing_off_allows_reviewed_residual_cleanup() {
    let mut f = Fixture::new();
    f.ctx.desktop_running = true;
    f.ctx.runtime = Some(crate::gateway::types::RuntimeView {
        state: "running".into(),
        ..Default::default()
    });
    let ticket = repair::preview(&f.ctx, "residual_cache").unwrap();
    assert!(!ticket.view.requires_desktop_closed);
    let result = repair::apply(&f.ctx, ticket).unwrap();
    assert!(result.fixed_ids.iter().any(|id| id == "residual_cache"));
    assert!(residual_cleanup::Kind::Cache
        .paths(&f.ctx)
        .iter()
        .all(|p| !p.exists()));
    assert_eq!(f.ctx.runtime.as_ref().unwrap().state, "running");
    for p in residual_cleanup::Kind::Manifest.paths(&f.ctx) {
        assert!(p.exists());
    }
}

#[test]
fn no_quiet_exception_cannot_write_during_a_transition() {
    let mut f = Fixture::new();
    f.ctx.desktop_running = true;
    let ticket = repair::preview(&f.ctx, "residual_cache").unwrap();
    assert!(!ticket.view.requires_desktop_closed);
    fs::write(f.ctx.app_root.join("gateway/transition.json"), b"{}").unwrap();
    assert!(repair::apply(&f.ctx, ticket).unwrap_err().contains("转换"));
    for kind in residual_cleanup::Kind::ALL {
        for path in kind.paths(&f.ctx) {
            assert_eq!(fs::read(path).unwrap(), kind.id().as_bytes());
        }
    }
    assert!(!f.ctx.app_root.join("maintenance").exists());
}
