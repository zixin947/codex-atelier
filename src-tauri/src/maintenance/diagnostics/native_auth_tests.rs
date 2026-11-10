//! Temporary synthetic credential/config data; no native Context, Keychain or clients.
use super::native_auth as adapter;
use super::*;
use atelier_maintenance_core::auth::RouterMode;
use std::fs;
const USER: &[u8] = br#"{"OPENAI_API_KEY":"sk-synthetic-user-private","fixture":true}"#;
const VIRTUAL: &[u8] =
    br#"{"auth_mode":"apikey","OPENAI_API_KEY":"sk-aimami-relay-virtual-synthetic","tokens":{}}"#;
struct Fixture {
    root: PathBuf,
    ctx: Context,
}
impl Fixture {
    fn new() -> Self {
        let root =
            std::env::temp_dir().join(format!("atelier-native-auth-{}", uuid::Uuid::new_v4()));
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
    fn backups(&self) -> PathBuf {
        self.ctx.app_root.join("approved-backups")
    }
    fn unlock(&self) {
        fs::write(self.ctx.home.join("auth.json"), VIRTUAL).unwrap();
        fs::write(
            self.relay("router-unlock-auth-marker.json"),
            br#"{"backedUpPreviousAuth":true}"#,
        )
        .unwrap();
        fs::write(self.relay("router-unlock-auth-backup.json"), USER).unwrap();
    }
    fn takeover(&self) {
        fs::write(self.relay("router-takeover-backup.json"),serde_json::to_vec(&serde_json::json!({"version":1,"createdAtMs":0,"topLevelLines":["model='synthetic-original'"],"sectionBlocks":[],"originalConfig":"never blindly restore"})).unwrap()).unwrap();
    }
    fn prepare(&self, id: &str) -> adapter::PreparedAction {
        adapter::prepare(&self.ctx, id, &self.backups(), RouterMode::Off).unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
#[test]
fn preview_is_pure_counts_only_and_unlock_specialized_restore_is_private() {
    let f = Fixture::new();
    f.unlock();
    let plan = f.prepare("router_unlock_auth_residue");
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), VIRTUAL);
    assert!(!f.backups().exists());
    assert!(!f.ctx.home.join("accounts/.live-auth.lock").exists());
    let preview = adapter::preview_lines(&plan).join("\n");
    assert!(!preview.contains("sk-synthetic"));
    assert!(!preview.contains("sk-aimami"));
    let result = adapter::apply(&f.ctx, &plan, RouterMode::Off).unwrap();
    assert!(result.success);
    assert_eq!(result.changed, 3);
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), USER);
    assert!(!f.relay("router-unlock-auth-marker.json").exists());
    assert!(f.ctx.home.join("accounts/.live-auth.lock").exists());
    assert!(result
        .backups
        .iter()
        .any(|p| fs::read(p).unwrap() == VIRTUAL));
    assert!(!format!("{result:?}").contains("sk-synthetic"));
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        for p in result.backups {
            assert_eq!(fs::metadata(p).unwrap().permissions().mode() & 0o777, 0o600)
        }
    }
}
#[test]
fn existing_user_login_is_preserved_while_sidecars_are_removed() {
    let f = Fixture::new();
    f.unlock();
    fs::write(f.ctx.home.join("auth.json"), USER).unwrap();
    fs::write(
        f.relay("router-unlock-auth-backup.json"),
        b"malformed unused backup",
    )
    .unwrap();
    let result = adapter::apply(
        &f.ctx,
        &f.prepare("router_unlock_auth_residue"),
        RouterMode::Off,
    )
    .unwrap();
    assert!(result.success);
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), USER);
    assert_eq!(result.changed, 2);
}
#[test]
fn no_account_mode_refuses_unlock_and_any_enabled_mode_refuses_takeover() {
    let f = Fixture::new();
    f.unlock();
    f.takeover();
    assert!(adapter::prepare(
        &f.ctx,
        "router_unlock_auth_residue",
        &f.backups(),
        RouterMode::NoAccount
    )
    .is_err());
    for mode in [RouterMode::Native, RouterMode::NoAccount] {
        assert!(adapter::prepare(&f.ctx, "takeover_backup_orphan", &f.backups(), mode).is_err())
    }
    let checks = adapter::inspect(&f.ctx, RouterMode::NoAccount);
    assert!(!checks.iter().any(|d| d.fixable));
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), VIRTUAL);
    assert!(!f.backups().exists());
}
#[test]
fn native_account_route_still_allows_unlock_cleanup() {
    let f = Fixture::new();
    f.unlock();
    let plan = adapter::prepare(
        &f.ctx,
        "router_unlock_auth_residue",
        &f.backups(),
        RouterMode::Native,
    )
    .unwrap();
    let result = adapter::apply(&f.ctx, &plan, RouterMode::Native).unwrap();
    assert!(result.success);
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), USER);
}
#[test]
fn changed_auth_marker_backup_or_config_refuses_before_backup_and_mutation() {
    for target in [
        "auth.json",
        "codexmate/relay/router-unlock-auth-marker.json",
        "codexmate/relay/router-unlock-auth-backup.json",
        "config.toml",
    ] {
        let f = Fixture::new();
        f.unlock();
        let plan = f.prepare("router_unlock_auth_residue");
        fs::write(f.ctx.home.join(target), b"synthetic external edit").unwrap();
        assert!(adapter::apply(&f.ctx, &plan, RouterMode::Off).is_err());
        assert!(!f.backups().exists());
        assert!(f.relay("router-unlock-auth-marker.json").exists());
    }
}
#[test]
fn changed_mode_or_native_state_dependency_refuses_reviewed_plan() {
    let f = Fixture::new();
    f.unlock();
    let p = f.prepare("router_unlock_auth_residue");
    assert!(adapter::apply(&f.ctx, &p, RouterMode::NoAccount).is_err());
    fs::write(f.relay("state.json"), b"{}").unwrap();
    assert!(adapter::apply(&f.ctx, &p, RouterMode::Off).is_err());
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), VIRTUAL);
    assert!(!f.backups().exists());
}
#[test]
fn unreadable_live_auth_does_not_report_false_success() {
    let f = Fixture::new();
    f.unlock();
    fs::write(f.ctx.home.join("auth.json"), b"malformed fixture").unwrap();
    let p = f.prepare("router_unlock_auth_residue");
    let result = adapter::apply(&f.ctx, &p, RouterMode::Off).unwrap();
    assert!(!result.success);
    assert_eq!(result.changed, 0);
    assert_eq!(result.diagnostic.status, "error");
    assert!(f.relay("router-unlock-auth-marker.json").exists());
}
#[test]
fn unsafe_user_backup_is_preserved_without_partial_cleanup() {
    let f = Fixture::new();
    f.unlock();
    fs::write(
        f.relay("router-unlock-auth-backup.json"),
        b"malformed fixture",
    )
    .unwrap();
    assert!(adapter::prepare(
        &f.ctx,
        "router_unlock_auth_residue",
        &f.backups(),
        RouterMode::Off
    )
    .is_err());
    assert!(f.relay("router-unlock-auth-marker.json").exists());
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), VIRTUAL);
    assert!(
        !adapter::inspect(&f.ctx, RouterMode::Off)
            .into_iter()
            .find(|i| i.id == "router_unlock_auth_residue")
            .unwrap()
            .fixable
    );
}
#[test]
fn legacy_unlock_uses_native_backup_and_removes_only_sidecars() {
    let f = Fixture::new();
    fs::write(
        f.ctx.home.join("auth.json"),
        br#"{"OPENAI_API_KEY":"synthetic-legacy"}"#,
    )
    .unwrap();
    fs::write(
        f.relay("virtual-auth-marker.json"),
        br#"{"apiKey":"synthetic-legacy","backedUpRealAuth":true}"#,
    )
    .unwrap();
    fs::write(f.relay("api-login-auth-backup.json"), USER).unwrap();
    let result = adapter::apply(
        &f.ctx,
        &f.prepare("router_unlock_auth_residue"),
        RouterMode::Off,
    )
    .unwrap();
    assert!(result.success);
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), USER);
    assert!(!f.relay("virtual-auth-marker.json").exists());
}
#[test]
fn takeover_restores_missing_value_preserving_unrelated_config_and_removes_backup() {
    let f = Fixture::new();
    f.takeover();
    fs::write(f.ctx.config(), "model_provider='openai'\nuser_flag=true\n").unwrap();
    let p = f.prepare("takeover_backup_orphan");
    let result = adapter::apply(&f.ctx, &p, RouterMode::Off).unwrap();
    assert!(result.success);
    let config = fs::read_to_string(f.ctx.config()).unwrap();
    assert!(config.contains("synthetic-original"));
    assert!(config.contains("user_flag=true"));
    assert!(!config.contains("never blindly restore"));
    assert!(!f.relay("router-takeover-backup.json").exists());
    assert_eq!(result.changed, 2);
}
#[test]
fn takeover_user_conflict_is_preserved_and_still_consumes_usable_backup() {
    let f = Fixture::new();
    f.takeover();
    fs::write(f.ctx.config(), "model='synthetic-user-choice'\n").unwrap();
    let result = adapter::apply(
        &f.ctx,
        &f.prepare("takeover_backup_orphan"),
        RouterMode::Off,
    )
    .unwrap();
    assert!(result.success);
    assert_eq!(
        fs::read_to_string(f.ctx.config()).unwrap(),
        "model='synthetic-user-choice'\n"
    );
    assert!(!f.relay("router-takeover-backup.json").exists());
}
#[test]
fn damaged_takeover_quarantine_is_pinned_to_reviewed_destination() {
    let f = Fixture::new();
    let source = f.relay("router-takeover-backup.json");
    fs::write(&source, b"malformed native backup fixture").unwrap();
    let p = f.prepare("takeover_backup_orphan");
    let preview = adapter::preview_lines(&p).join("\n");
    assert!(preview.contains("隔离损坏备份 1"));
    let result = adapter::apply(&f.ctx, &p, RouterMode::Off).unwrap();
    assert!(result.success);
    assert!(!source.exists());
    let quarantine = fs::read_dir(source.parent().unwrap())
        .unwrap()
        .filter_map(std::result::Result::ok)
        .map(|e| e.path())
        .find(|p| {
            p.file_name()
                .unwrap()
                .to_string_lossy()
                .contains("damaged-")
        })
        .unwrap();
    assert_eq!(
        fs::read(quarantine).unwrap(),
        b"malformed native backup fixture"
    );
}
#[test]
fn unsupported_takeover_schema_is_not_deleted_or_quarantined() {
    let f = Fixture::new();
    let p = f.relay("router-takeover-backup.json");
    fs::write(&p, br#"{"version":2,"future":true}"#).unwrap();
    assert!(adapter::prepare(
        &f.ctx,
        "takeover_backup_orphan",
        &f.backups(),
        RouterMode::Off
    )
    .is_err());
    assert!(p.exists());
    assert!(!f.backups().exists());
}
#[test]
fn empty_legacy_takeover_remains_warning_after_native_noop() {
    let f = Fixture::new();
    let p = f.relay("codex_router_top_level_backup.json");
    fs::write(&p, br#"{"lines":[]}"#).unwrap();
    let result = adapter::apply(
        &f.ctx,
        &f.prepare("takeover_backup_orphan"),
        RouterMode::Off,
    )
    .unwrap();
    assert!(!result.success);
    assert_eq!(result.changed, 0);
    assert_eq!(result.diagnostic.status, "warning");
    assert!(p.exists());
}
#[test]
fn approved_sequence_replans_without_credential_or_config_overwrite() {
    for ids in [
        ["router_unlock_auth_residue", "takeover_backup_orphan"],
        ["takeover_backup_orphan", "router_unlock_auth_residue"],
    ] {
        let f = Fixture::new();
        f.unlock();
        f.takeover();
        fs::write(f.ctx.config(), "model_provider='openai'\n").unwrap();
        let ids = ids.map(str::to_owned);
        let plans = adapter::prepare_sequence(&f.ctx, &ids, &f.backups(), RouterMode::Off).unwrap();
        let outcomes = adapter::apply_sequence(&f.ctx, &plans, RouterMode::Off).unwrap();
        assert_eq!(outcomes.len(), 2);
        assert!(outcomes.iter().all(|o| o.success));
        assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), USER);
        assert!(fs::read_to_string(f.ctx.config())
            .unwrap()
            .contains("synthetic-original"));
    }
}
#[test]
fn sequence_validates_all_initial_inputs_before_any_mutation() {
    let f = Fixture::new();
    f.unlock();
    f.takeover();
    let ids = [
        "router_unlock_auth_residue".into(),
        "takeover_backup_orphan".into(),
    ];
    let plans = adapter::prepare_sequence(&f.ctx, &ids, &f.backups(), RouterMode::Off).unwrap();
    fs::write(
        f.relay("router-takeover-backup.json"),
        b"new external native backup",
    )
    .unwrap();
    assert!(adapter::apply_sequence(&f.ctx, &plans, RouterMode::Off).is_err());
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), VIRTUAL);
    assert!(f.relay("router-unlock-auth-marker.json").exists());
    assert!(!f.backups().exists());
}
#[test]
fn resolver_reads_native_flags_not_alias_or_directory_guesses() {
    let mut f = Fixture::new();
    assert_eq!(adapter::resolve_mode(&f.ctx).unwrap(), RouterMode::Off);
    fs::write(f.ctx.config(), "model_provider='aimai1'\n").unwrap();
    assert_eq!(adapter::resolve_mode(&f.ctx).unwrap(), RouterMode::Off);
    fs::write(f.relay("state.json"),serde_json::to_vec(&serde_json::json!({"schemaVersion":16,"providers":[],"codexRouterEnabled":true,"codexRouterNoAccountMode":true})).unwrap()).unwrap();
    assert_eq!(
        adapter::resolve_mode(&f.ctx).unwrap(),
        RouterMode::NoAccount
    );
    fs::write(f.relay("state.json"), b"malformed fixture").unwrap();
    assert!(adapter::resolve_mode(&f.ctx).is_err());
    fs::remove_file(f.relay("state.json")).unwrap();
    fs::write(f.ctx.config(), "model_provider='openai'\n").unwrap();
    f.ctx.runtime = Some(crate::gateway::types::RuntimeView {
        state: "running".into(),
        ..Default::default()
    });
    assert_eq!(adapter::resolve_mode(&f.ctx).unwrap(), RouterMode::Off);
    fs::create_dir_all(f.ctx.app_root.join("gateway")).unwrap();
    fs::write(super::router_intent::path(&f.ctx), br#"{"schemaVersion":1,"target":"routerOn","provider":"atelier","operationId":null,"catalogRevision":null,"updatedAtMs":1}"#).unwrap();
    assert_eq!(adapter::resolve_mode(&f.ctx).unwrap(), RouterMode::Native);
}
#[cfg(unix)]
#[test]
fn adapter_rejects_symlink_dependencies_without_mutation() {
    use std::os::unix::fs::symlink;
    let f = Fixture::new();
    f.unlock();
    let outside = f.root.join("outside");
    fs::write(&outside, USER).unwrap();
    fs::remove_file(f.relay("router-unlock-auth-backup.json")).unwrap();
    symlink(&outside, f.relay("router-unlock-auth-backup.json")).unwrap();
    assert!(adapter::prepare(
        &f.ctx,
        "router_unlock_auth_residue",
        &f.backups(),
        RouterMode::Off
    )
    .is_err());
    assert_eq!(fs::read(outside).unwrap(), USER);
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), VIRTUAL);
}
#[test]
fn takeover_preserves_existing_owned_values_as_native_conflicts_too() {
    let f = Fixture::new();
    f.takeover();
    let current = "model='aimami_relay_fixture'\n";
    fs::write(f.ctx.config(), current).unwrap();
    let result = adapter::apply(
        &f.ctx,
        &f.prepare("takeover_backup_orphan"),
        RouterMode::Off,
    )
    .unwrap();
    assert!(result.success);
    assert_eq!(result.changed, 1);
    assert_eq!(fs::read_to_string(f.ctx.config()).unwrap(), current);
}
#[test]
fn newly_created_residue_after_noop_preview_is_not_silently_added_to_scope() {
    let f = Fixture::new();
    let plan = f.prepare("router_unlock_auth_residue");
    f.unlock();
    assert!(adapter::apply(&f.ctx, &plan, RouterMode::Off).is_err());
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), VIRTUAL);
    assert!(f.relay("router-unlock-auth-marker.json").exists());
    assert!(!f.backups().exists());
}
#[test]
fn stale_input_of_later_action_prevents_earlier_credential_side_effects() {
    let f = Fixture::new();
    f.unlock();
    f.takeover();
    let plans = adapter::prepare_sequence(
        &f.ctx,
        &[
            "router_unlock_auth_residue".into(),
            "takeover_backup_orphan".into(),
        ],
        &f.backups(),
        RouterMode::Off,
    )
    .unwrap();
    fs::write(f.ctx.config(), "external_choice=true\n").unwrap();
    assert!(adapter::apply_sequence(&f.ctx, &plans, RouterMode::Off).is_err());
    assert!(!f.ctx.home.join("accounts/.live-auth.lock").exists());
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), VIRTUAL);
    assert!(!f.backups().exists());
}
#[test]
fn coordinated_config_is_not_rewritten_by_fresh_takeover_restore() {
    let f = Fixture::new();
    f.takeover();
    let p = f.prepare("takeover_backup_orphan");
    let (config, before, after) = adapter::planned_config(&p).unwrap();
    assert_eq!(config, f.ctx.config());
    assert!(after
        .unwrap()
        .windows(b"synthetic-original".len())
        .any(|w| w == b"synthetic-original"));
    let combined = b"model_provider='openai'\ncoordinator_final=true\n";
    fs::write(&config, combined).unwrap();
    let result = adapter::apply_after_approved_config(
        &f.ctx,
        &p,
        RouterMode::Off,
        before.as_deref(),
        Some(combined),
    )
    .unwrap();
    assert!(result.success);
    assert_eq!(fs::read(config).unwrap(), combined);
    assert!(!f.relay("router-takeover-backup.json").exists());
    assert_eq!(result.changed, 1);
}
#[test]
fn coordinated_auth_still_rejects_nonconfig_or_unreviewed_config_changes() {
    for bad_config in [false, true] {
        let f = Fixture::new();
        f.unlock();
        let p = f.prepare("router_unlock_auth_residue");
        assert!(!adapter::approved_dependencies(&p).is_empty());
        assert!(adapter::approved_paths(&p).contains(&f.ctx.home.join("auth.json")));
        let before = fs::read(f.ctx.config()).unwrap();
        let after = b"coordinator_final=true\n";
        fs::write(f.ctx.config(), after).unwrap();
        if !bad_config {
            fs::write(
                f.relay("router-unlock-auth-marker.json"),
                b"external update",
            )
            .unwrap();
        }
        let expected = if bad_config {
            b"unapproved config bytes".as_slice()
        } else {
            after
        };
        assert!(adapter::apply_after_approved_config(
            &f.ctx,
            &p,
            RouterMode::Off,
            Some(&before),
            Some(expected)
        )
        .is_err());
        assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), VIRTUAL);
        assert!(!f.backups().exists());
    }
}
fn integrity_fixture(f: &Fixture) {
    f.unlock();
    fs::remove_file(f.ctx.home.join("auth.json")).unwrap();
    fs::create_dir_all(f.ctx.home.join("accounts/backups")).unwrap();
    fs::write(f.ctx.home.join("accounts/backups/A.json"), USER).unwrap();
    fs::write(
        f.relay("router-unlock-auth-backup.json"),
        br#"{"OPENAI_API_KEY":"sk-synthetic-distinct-user-B"}"#,
    )
    .unwrap();
}
#[test]
fn integrity_precedes_unlock_and_fresh_unlock_preserves_the_restored_login() {
    let f = Fixture::new();
    integrity_fixture(&f);
    let config = fs::read(f.ctx.config()).unwrap();
    let plans = adapter::prepare_sequence(
        &f.ctx,
        &["router_unlock_auth_residue".into(), "auth_integrity".into()],
        &f.backups(),
        RouterMode::Off,
    )
    .unwrap();
    assert_eq!(
        plans.iter().map(adapter::item_id).collect::<Vec<_>>(),
        ["auth_integrity", "router_unlock_auth_residue"]
    );
    let (first, effects) = adapter::apply_after_approved_effects(
        &f.ctx,
        &plans[0],
        RouterMode::Off,
        Some(&config),
        Some(&config),
        &[],
    )
    .unwrap();
    assert!(first.success);
    let auth = f.ctx.home.join("auth.json");
    let after = effects.iter().find(|(p, _)| p == &auth).unwrap().1.clone();
    let (second, fresh_effects) = adapter::apply_after_approved_effects(
        &f.ctx,
        &plans[1],
        RouterMode::Off,
        Some(&config),
        Some(&config),
        &[(auth.clone(), None, after)],
    )
    .unwrap();
    assert!(second.success);
    assert!(!fresh_effects.iter().any(|(p, _)| p == &auth));
    assert_eq!(fs::read(auth).unwrap(), USER);
    assert!(!f.relay("router-unlock-auth-marker.json").exists());
    assert!(!f.relay("router-unlock-auth-backup.json").exists());
}
#[test]
fn standalone_sequence_uses_native_integrity_unlock_takeover_order() {
    let f = Fixture::new();
    integrity_fixture(&f);
    f.takeover();
    let plans = adapter::prepare_sequence(
        &f.ctx,
        &[
            "takeover_backup_orphan".into(),
            "router_unlock_auth_residue".into(),
            "auth_integrity".into(),
        ],
        &f.backups(),
        RouterMode::Off,
    )
    .unwrap();
    assert_eq!(
        plans.iter().map(adapter::item_id).collect::<Vec<_>>(),
        [
            "auth_integrity",
            "router_unlock_auth_residue",
            "takeover_backup_orphan"
        ]
    );
    let out = adapter::apply_sequence(&f.ctx, &plans, RouterMode::Off).unwrap();
    assert_eq!(out.len(), 3);
    assert!(out.iter().all(|o| o.success));
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), USER);
}
#[test]
fn integrity_excludes_keyring_ephemeral_and_enabled_route_before_auth_io() {
    for store in ["keyring", "ephemeral"] {
        let f = Fixture::new();
        fs::write(
            f.ctx.config(),
            format!("cli_auth_credentials_store='{store}'\n"),
        )
        .unwrap();
        fs::create_dir_all(f.ctx.home.join("auth.json")).unwrap();
        let check = adapter::inspect(&f.ctx, RouterMode::Off)
            .into_iter()
            .find(|i| i.id == "auth_integrity")
            .unwrap();
        assert_eq!(check.status, "ok");
        assert!(!check.fixable);
        assert!(
            adapter::prepare(&f.ctx, "auth_integrity", &f.backups(), RouterMode::Off)
                .err()
                .unwrap()
                .contains("文件模式")
        );
    }
    let f = Fixture::new();
    integrity_fixture(&f);
    assert!(adapter::prepare(&f.ctx, "auth_integrity", &f.backups(), RouterMode::Native).is_err());
    fs::write(f.ctx.config(), "model_provider='aimai1'\n").unwrap();
    let mut state = atelier_maintenance_core::catalog::state::RelayStateDocument::native_default();
    state.set_router_flags(true, false);
    fs::write(f.relay("state.json"), state.to_pretty_json().unwrap()).unwrap();
    assert!(adapter::prepare(&f.ctx, "auth_integrity", &f.backups(), RouterMode::Off).is_err());
}
#[test]
fn integrity_refuses_new_candidate_path_even_if_credential_bytes_match() {
    let f = Fixture::new();
    integrity_fixture(&f);
    let p = f.prepare("auth_integrity");
    fs::write(f.ctx.home.join("accounts/backups/new-candidate.json"), USER).unwrap();
    assert!(adapter::apply(&f.ctx, &p, RouterMode::Off).is_err());
    assert!(!f.ctx.home.join("auth.json").exists());
    let cfg = fs::read(f.ctx.config()).unwrap();
    assert!(adapter::apply_after_approved_effects(
        &f.ctx,
        &p,
        RouterMode::Off,
        Some(&cfg),
        Some(&cfg),
        &[]
    )
    .is_err());
    assert!(!f.backups().exists());
}
#[test]
fn approved_prefix_must_match_original_and_current_auth_fingerprints() {
    let f = Fixture::new();
    integrity_fixture(&f);
    let p = f.prepare("router_unlock_auth_residue");
    let cfg = fs::read(f.ctx.config()).unwrap();
    fs::write(f.ctx.home.join("auth.json"), USER).unwrap();
    let wrong = Some((USER.len(), "not-the-approved-sha".to_string()));
    assert!(adapter::apply_after_approved_effects(
        &f.ctx,
        &p,
        RouterMode::Off,
        Some(&cfg),
        Some(&cfg),
        &[(f.ctx.home.join("auth.json"), None, wrong)]
    )
    .is_err());
    assert!(f.relay("router-unlock-auth-marker.json").exists());
    assert!(!f.backups().exists());
}
