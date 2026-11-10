use super::{router_intent as intent, *};
use atelier_maintenance_core::{
    catalog::state::RouterOperationId, transition_journal::TransitionTarget,
};
use std::fs;
struct Fixture {
    root: PathBuf,
    ctx: Context,
}
impl Fixture {
    fn new(config: &str) -> Self {
        let root = std::env::temp_dir().join(format!("atelier-intent-{}", uuid::Uuid::new_v4()));
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
        fs::create_dir_all(&ctx.home).unwrap();
        fs::create_dir_all(&ctx.app_root).unwrap();
        fs::write(ctx.config(), config).unwrap();
        Self { root, ctx }
    }
    fn operation() -> RouterOperationId {
        RouterOperationId::try_from("00000000-0000-4000-8000-000000000001".to_owned()).unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
#[test]
fn observing_config_never_creates_intent_or_turns_running_proxy_into_desktop_ownership() {
    let mut f = Fixture::new("model_provider='openai'\n");
    f.ctx.runtime = Some(crate::gateway::types::RuntimeView {
        state: "running".into(),
        ..Default::default()
    });
    let plan = intent::prepare(&f.ctx).unwrap();
    assert_eq!(
        plan.observation().record.target,
        TransitionTarget::NativeOff
    );
    assert_eq!(
        plan.observation().origin,
        intent::Origin::ObservedConfiguration
    );
    assert!(plan.observation().catalog.is_none());
    assert!(!intent::path(&f.ctx).exists());
    assert!(intent::read(&f.ctx).unwrap().is_none());
}
#[test]
fn effective_profile_provider_is_observed_without_conflating_gateway_and_native_sources() {
    let f=Fixture::new("model_provider='openai'\nprofile='work'\n[profiles.work]\nmodel_provider='atelier_gateway'\n");
    let plan = intent::prepare(&f.ctx).unwrap();
    assert_eq!(plan.observation().record.target, TransitionTarget::RouterOn);
    assert_eq!(
        plan.observation().record.provider.as_deref(),
        Some("atelier_gateway")
    );
    assert!(!f.ctx.home.join("codexmate/relay/state.json").exists());
    assert!(plan.destination().starts_with(&f.ctx.app_root));
}
#[test]
fn intent_commit_publishes_private_actual_receipt_and_preserves_original_config() {
    let f = Fixture::new("model_provider='atelier_gateway'\n");
    let original = fs::read(f.ctx.config()).unwrap();
    let plan = intent::prepare(&f.ctx).unwrap();
    let result = intent::commit(&f.ctx, &plan, &Fixture::operation(), 17).unwrap();
    assert_eq!(
        result.record.operation_id.as_ref().unwrap().as_str(),
        Fixture::operation().as_str()
    );
    result.publication.require_sync().unwrap();
    intent::verify_receipt(&f.ctx, &result.record).unwrap();
    assert_eq!(fs::read(f.ctx.config()).unwrap(), original);
    assert_eq!(
        intent::prepare(&f.ctx).unwrap().observation().origin,
        intent::Origin::Persisted
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            fs::metadata(intent::path(&f.ctx))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
    }
}
#[test]
fn corrupt_future_or_foreign_intent_is_not_replaced_with_inferred_defaults() {
    let f = Fixture::new("model_provider='openai'\n");
    fs::create_dir_all(f.ctx.app_root.join("gateway")).unwrap();
    for raw in [b"broken".as_slice(),br#"{"schemaVersion":2,"target":"nativeOff","provider":null,"operationId":null,"catalogRevision":null,"updatedAtMs":1}"#.as_slice(),
        br#"{"schemaVersion":1,"target":"routerOn","provider":"aimai1","operationId":null,"catalogRevision":null,"updatedAtMs":1}"#.as_slice()] {
        fs::write(intent::path(&f.ctx),raw).unwrap();assert!(intent::prepare(&f.ctx).is_err());assert_eq!(fs::read(intent::path(&f.ctx)).unwrap(),raw);
    }
}
#[test]
fn config_or_existing_intent_changes_after_review_prevent_any_publication() {
    let f = Fixture::new("model_provider='openai'\n");
    let plan = intent::prepare(&f.ctx).unwrap();
    fs::write(f.ctx.config(), b"model='external'\n").unwrap();
    assert!(intent::commit(&f.ctx, &plan, &Fixture::operation(), 1).is_err());
    assert!(!intent::path(&f.ctx).exists());
    let plan = intent::prepare(&f.ctx).unwrap();
    fs::create_dir_all(f.ctx.app_root.join("gateway")).unwrap();
    fs::write(intent::path(&f.ctx), b"external sentinel").unwrap();
    assert!(intent::commit(&f.ctx, &plan, &Fixture::operation(), 1).is_err());
    assert_eq!(
        fs::read(intent::path(&f.ctx)).unwrap(),
        b"external sentinel"
    );
}
#[test]
fn approved_config_rebase_requires_exact_bytes_and_the_reviewed_provider_target() {
    let f = Fixture::new("model_provider='atelier_gateway'\n");
    let plan = intent::prepare(&f.ctx).unwrap();
    let after = b"# approved repair\nmodel_provider='atelier_gateway'\n";
    fs::write(f.ctx.config(), after).unwrap();
    assert!(intent::commit_after_approved_config(
        &f.ctx,
        &plan,
        Some(b"unapproved"),
        &Fixture::operation(),
        1
    )
    .is_err());
    let result =
        intent::commit_after_approved_config(&f.ctx, &plan, Some(after), &Fixture::operation(), 2)
            .unwrap();
    result.publication.require_sync().unwrap();
    let next = intent::prepare(&f.ctx).unwrap();
    let foreign = b"model_provider='enterprise'\n";
    fs::write(f.ctx.config(), foreign).unwrap();
    assert!(intent::commit_after_approved_config(
        &f.ctx,
        &next,
        Some(foreign),
        &Fixture::operation(),
        3
    )
    .is_err());
    assert_eq!(intent::read(&f.ctx).unwrap().unwrap().updated_at_ms, 2);
}
#[test]
fn malformed_gateway_json_is_a_real_source_error_rather_than_native_default_state() {
    let f = Fixture::new("model_provider='atelier_gateway'\n");
    fs::create_dir_all(f.ctx.app_root.join("gateway")).unwrap();
    fs::write(
        f.ctx.app_root.join("gateway/catalog.json"),
        br#"{"providers":[],"schemaVersion":16}"#,
    )
    .unwrap();
    assert!(intent::prepare(&f.ctx).is_err());
    assert!(!intent::path(&f.ctx).exists());
}
