use super::*;
use serde_json::json;
use std::fs;
struct Fixture {
    root: PathBuf,
    ctx: Context,
}
impl Fixture {
    fn new(text: &str) -> Self {
        let root =
            std::env::temp_dir().join(format!("atelier-route-mode-{}", uuid::Uuid::new_v4()));
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
        fs::create_dir_all(ctx.app_root.join("gateway")).unwrap();
        fs::write(ctx.config(), text).unwrap();
        Self { root, ctx }
    }
    fn native(&self, on: bool, no_account: bool) {
        let mut document = RelayStateDocument::native_default().as_value().clone();
        document["codexRouterEnabled"] = json!(on);
        document["codexRouterNoAccountMode"] = json!(no_account);
        let path = self.ctx.home.join("codexmate/relay/state.json");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, serde_json::to_vec(&document).unwrap()).unwrap();
    }
    fn intent(&self, target: TransitionTarget) {
        let value = super::super::router_intent::IntentRecord {
            schema_version: 1,
            target,
            provider: (target == TransitionTarget::RouterOn).then(|| "atelier".into()),
            operation_id: None,
            catalog_revision: None,
            updated_at_ms: 1,
        };
        fs::write(
            super::super::router_intent::path(&self.ctx),
            serde_json::to_vec(&value).unwrap(),
        )
        .unwrap();
    }
    fn config(&self) -> toml::Value {
        super::super::inspect::config(&self.ctx).unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn running_gateway_is_not_enabled_desktop_routing_and_does_not_block_off_auth_recovery() {
    let mut f = Fixture::new("model_provider='openai'\n");
    f.ctx.runtime = Some(crate::gateway::types::RuntimeView {
        state: "running".into(),
        ..Default::default()
    });
    assert_eq!(resolve(&f.ctx, &f.config()).unwrap(), RouterMode::Off);
    assert_eq!(
        super::super::native_auth::resolve_mode(&f.ctx).unwrap(),
        RouterMode::Off
    );
    assert!(!super::super::residual_cleanup::route_enabled(&f.ctx, &f.config()).unwrap());
}
#[test]
fn persisted_intent_is_authoritative_over_config_drift_in_both_directions() {
    let f = Fixture::new("model_provider='atelier'\n");
    f.intent(TransitionTarget::NativeOff);
    assert_eq!(resolve(&f.ctx, &f.config()).unwrap(), RouterMode::Off);
    fs::write(f.ctx.config(), "model_provider='openai'\n").unwrap();
    f.intent(TransitionTarget::RouterOn);
    assert_eq!(resolve(&f.ctx, &f.config()).unwrap(), RouterMode::Native);
}
#[test]
fn native_no_account_precedes_unrelated_atelier_intent_and_disabled_native_stays_disabled() {
    let f = Fixture::new("model_provider='aimai1'\n");
    f.native(true, true);
    f.intent(TransitionTarget::NativeOff);
    assert_eq!(resolve(&f.ctx, &f.config()).unwrap(), RouterMode::NoAccount);
    f.native(false, true);
    assert_eq!(resolve(&f.ctx, &f.config()).unwrap(), RouterMode::Off);
    fs::remove_file(super::super::router_intent::path(&f.ctx)).unwrap();
    assert_eq!(
        resolve(&f.ctx, &f.config()).unwrap(),
        RouterMode::Off,
        "a stale provider name must not re-enable a persisted off native router"
    );
}
#[test]
fn corrupt_state_or_intent_is_an_error_and_original_bytes_are_retained() {
    let f = Fixture::new("model_provider='openai'\n");
    let path = super::super::router_intent::path(&f.ctx);
    fs::write(&path, b"broken intent").unwrap();
    assert!(resolve(&f.ctx, &f.config()).is_err());
    assert_eq!(fs::read(&path).unwrap(), b"broken intent");
    fs::remove_file(path).unwrap();
    f.native(false, false);
    let native = f.ctx.home.join("codexmate/relay/state.json");
    fs::write(&native, b"broken native").unwrap();
    assert!(resolve(&f.ctx, &f.config()).is_err());
    assert_eq!(fs::read(native).unwrap(), b"broken native");
}
#[test]
fn native_and_atelier_residual_files_are_checked_against_their_actual_owners() {
    let f = Fixture::new("model_provider='openai'\n");
    f.native(true, false);
    f.intent(TransitionTarget::NativeOff);
    let kind = super::super::residual_cleanup::Kind::Cache;
    let [native, atelier] = kind.paths(&f.ctx);
    fs::write(&native, b"native live").unwrap();
    fs::write(&atelier, b"atelier off").unwrap();
    let plan = super::super::repair::preview(&f.ctx, kind.id()).unwrap();
    assert_eq!(
        plan.file_changes()
            .iter()
            .map(|c| c.path())
            .collect::<Vec<_>>(),
        vec![atelier.as_path()]
    );
    super::super::repair::apply(&f.ctx, plan).unwrap();
    assert_eq!(fs::read(native).unwrap(), b"native live");
    assert!(!atelier.exists());
}
#[test]
fn auth_preview_pins_intent_absence_and_rejects_new_router_ownership_before_restoring() {
    let f = Fixture::new("model_provider='openai'\n");
    let backups = f.ctx.home.join("accounts/backups");
    fs::create_dir_all(&backups).unwrap();
    let backup = backups.join("fixture.json");
    fs::write(&backup, br#"{"OPENAI_API_KEY":"sk-synthetic-preserved"}"#).unwrap();
    let plan = super::super::native_auth::prepare(
        &f.ctx,
        "auth_integrity",
        &f.ctx.app_root.join("reviewed-auth"),
        RouterMode::Off,
    )
    .unwrap();
    f.intent(TransitionTarget::RouterOn);
    assert!(super::super::native_auth::apply(&f.ctx, &plan, RouterMode::Off).is_err());
    assert!(!f.ctx.home.join("auth.json").exists());
    assert!(backup.exists());
}
