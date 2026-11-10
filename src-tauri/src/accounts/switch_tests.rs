use super::*;
use serde_json::json;
struct Fixture {
    root: PathBuf,
    home: PathBuf,
    store: super::super::storage::Store,
    id: String,
}
impl Fixture {
    fn new(config: Option<&str>) -> Self {
        let root = std::env::temp_dir().join(format!("atelier-official-switch-{}", Uuid::new_v4()));
        fs::create_dir_all(root.join("home")).unwrap();
        let root = root.canonicalize().unwrap();
        let home = root.join("home");
        if let Some(c) = config {
            fs::write(home.join("config.toml"), c).unwrap();
        }
        let store = super::super::storage::Store::new(root.join("app/codex-accounts")).unwrap();
        let id = Uuid::new_v4().to_string();
        store.add(&id, "synthetic", &json!({"tokens":{"access_token":"fixture-access","refresh_token":"fixture-refresh","id_token":"fixture-id","account_id":"fixture-account"}})).unwrap();
        Self {
            root,
            home,
            store,
            id,
        }
    }
    fn preview(&self) -> Ticket {
        preview_at(&self.store, Some(self.id.clone()), &self.home).unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn official_switch_is_pure_until_confirmed_and_restores_all_three_targets() {
    let f = Fixture::new(Some("# retain comment\nmodel_provider='atelier_gateway'\nmodel='private-model'\nmodel_reasoning_effort='high'\nopenai_base_url='http://127.0.0.1:19191/v1'\n[model_providers.atelier_gateway]\nbase_url='http://127.0.0.1:19191/v1'\n[projects.fixture]\ntrust_level='trusted'\n"));
    let config_before = fs::read(f.home.join("config.toml")).unwrap();
    let intent_path = f.root.join("app/gateway/router-intent.json");
    fs::create_dir_all(intent_path.parent().unwrap()).unwrap();
    let intent_before = br#"{"schemaVersion":1,"target":"routerOn","provider":"atelier_gateway","operationId":null,"catalogRevision":null,"updatedAtMs":1}"#;
    fs::write(&intent_path, intent_before).unwrap();
    fs::write(f.home.join("auth.json"), b"original-auth").unwrap();
    let ticket = f.preview();
    assert_eq!(fs::read(f.home.join("config.toml")).unwrap(), config_before);
    assert_eq!(fs::read(&intent_path).unwrap(), intent_before);
    assert!(!f.store.root.join("last-switch.json").exists());
    assert!(apply_at(&f.store, &ticket, false).is_err());
    apply_at(&f.store, &ticket, true).unwrap();
    let config = fs::read_to_string(f.home.join("config.toml")).unwrap();
    let value: toml::Value = toml::from_str(&config).unwrap();
    assert!(value.get("model_provider").is_none());
    assert!(value.get("openai_base_url").is_none());
    assert!(value.get("model").is_none());
    assert!(value.get("model_reasoning_effort").is_none());
    assert_eq!(
        value["model_providers"]["atelier_gateway"]["base_url"].as_str(),
        Some("http://127.0.0.1:19191/v1")
    );
    assert!(config.contains("# retain comment"));
    assert_eq!(
        value["projects"]["fixture"]["trust_level"].as_str(),
        Some("trusted")
    );
    assert_eq!(
        crate::maintenance::diagnostics::gateway_route_target(
            &intent_path,
            Some(&f.home.join("config.toml"))
        )
        .unwrap(),
        atelier_maintenance_core::transition_journal::TransitionTarget::NativeOff
    );
    assert!(apply_at(&f.store, &ticket, true).is_err());
    let rollback = preview_at(&f.store, None, &f.home).unwrap();
    apply_at(&f.store, &rollback, true).unwrap();
    assert_eq!(fs::read(f.home.join("config.toml")).unwrap(), config_before);
    assert_eq!(fs::read(&intent_path).unwrap(), intent_before);
    assert_eq!(
        fs::read(f.home.join("auth.json")).unwrap(),
        b"original-auth"
    );
    assert!(!f.store.root.join("last-switch.json").exists());
}

#[test]
fn already_direct_preserves_config_bytes_and_missing_config_stays_absent() {
    for config in [None, Some("# native\nmodel='gpt-fixture'\nmodel_provider='openai'\n[projects.fixture]\ntrust_level='trusted'\n")] {
        let f = Fixture::new(config);
        let before = read_external(&f.home.join("config.toml"), 1024*1024).unwrap();
        let ticket = f.preview();
        apply_at(&f.store, &ticket, true).unwrap();
        assert_eq!(read_external(&f.home.join("config.toml"),1024*1024).unwrap(), before);
        let rollback = preview_at(&f.store, None, &f.home).unwrap();
        apply_at(&f.store, &rollback, true).unwrap();
        assert!(!f.home.join("auth.json").exists());
        assert!(!f.root.join("app/gateway/router-intent.json").exists());
    }
}

#[test]
fn rejects_unreviewed_effective_overrides_and_external_recovery() {
    for config in [
        "profile='custom'",
        "cli_auth_credentials_store='keyring'",
        "forced_login_method='api'",
        "[model_providers.openai]\nbase_url='http://127.0.0.1:7/v1'",
        "model_catalog_json='/external/catalog.json'",
        "model = ",
    ] {
        let f = Fixture::new(Some(config));
        assert!(
            preview_at(&f.store, Some(f.id.clone()), &f.home).is_err(),
            "{config}"
        );
        assert!(!f.home.join("auth.json").exists());
    }
    let f = Fixture::new(None);
    let relay = f.home.join("codexmate/relay");
    fs::create_dir_all(&relay).unwrap();
    fs::write(
        relay.join("state.json"),
        br#"{"schemaVersion":16,"codexRouterEnabled":true}"#,
    )
    .unwrap();
    assert!(preview_at(&f.store, Some(f.id.clone()), &f.home).is_err());
    fs::remove_file(relay.join("state.json")).unwrap();
    fs::write(relay.join("router-unlock-auth-marker.json"), b"{}").unwrap();
    assert!(preview_at(&f.store, Some(f.id.clone()), &f.home).is_err());
}

#[test]
fn model_restore_records_allow_switch_and_rollback_without_modification() {
    let f = Fixture::new(Some("model_provider='custom'\n"));
    let before = fs::read(f.home.join("config.toml")).unwrap();
    let mut records = Vec::new();
    for parent in [f.home.join("codexmate/relay"), f.root.join("app/gateway")] {
        fs::create_dir_all(&parent).unwrap();
        for name in [
            "router_model_restore.json",
            "router_no_account_model_restore.json",
        ] {
            let path = parent.join(name);
            // Opaque owner-managed records are observed, never interpreted or rewritten.
            let bytes = format!("owner-managed fixture: {name}").into_bytes();
            fs::write(&path, &bytes).unwrap();
            records.push((path, bytes));
        }
    }
    let ticket = f.preview();
    assert!(ticket
        .view
        .changes
        .iter()
        .any(|change| change.contains("保留已有模型恢复记录")));
    apply_at(&f.store, &ticket, true).unwrap();
    for (path, bytes) in &records {
        assert_eq!(fs::read(path).unwrap(), *bytes);
    }
    let rollback = preview_at(&f.store, None, &f.home).unwrap();
    apply_at(&f.store, &rollback, true).unwrap();
    assert_eq!(fs::read(f.home.join("config.toml")).unwrap(), before);
    assert!(!f.home.join("auth.json").exists());
    for (path, bytes) in &records {
        assert_eq!(fs::read(path).unwrap(), *bytes);
    }
}

#[test]
fn changed_model_restore_records_invalidate_preview_before_writes() {
    for parent in ["native", "atelier"] {
        for name in [
            "router_model_restore.json",
            "router_no_account_model_restore.json",
        ] {
            for mutation in ["create", "modify", "remove"] {
                let f = Fixture::new(Some("model_provider='custom'\n"));
                let directory = if parent == "native" {
                    f.home.join("codexmate/relay")
                } else {
                    f.root.join("app/gateway")
                };
                fs::create_dir_all(&directory).unwrap();
                let path = directory.join(name);
                if mutation != "create" {
                    fs::write(&path, b"original fixture").unwrap();
                }
                let before = fs::read(f.home.join("config.toml")).unwrap();
                let ticket = f.preview();
                if mutation == "remove" {
                    fs::remove_file(&path).unwrap();
                } else {
                    fs::write(&path, b"external fixture").unwrap();
                }
                let record = read_external(&path, 1024 * 1024).unwrap();
                assert!(
                    apply_at(&f.store, &ticket, true).is_err(),
                    "{parent}/{name}/{mutation}"
                );
                assert_eq!(fs::read(f.home.join("config.toml")).unwrap(), before);
                assert!(!f.home.join("auth.json").exists());
                assert!(!f.root.join("app/gateway/router-intent.json").exists());
                assert!(!f.store.root.join("last-switch.json").exists());
                assert_eq!(read_external(&path, 1024 * 1024).unwrap(), record);
            }
        }
    }
}

#[test]
fn unfinished_transition_records_still_reject_switch() {
    for parent in ["native", "atelier"] {
        let f = Fixture::new(None);
        let directory = if parent == "native" {
            f.home.join("codexmate/relay")
        } else {
            f.root.join("app/gateway")
        };
        fs::create_dir_all(&directory).unwrap();
        for bytes in [
            b"{}".as_slice(),
            br#"{"schemaVersion":1,"target":"nativeOff","phase":"committing","startedAtMs":1,"updatedAtMs":1,"reason":null}"#.as_slice(),
        ] {
            fs::write(directory.join("router-transition-journal.json"), bytes).unwrap();
            assert!(preview_at(&f.store, Some(f.id.clone()), &f.home).is_err());
            assert!(!f.home.join("auth.json").exists());
        }
    }
}

#[test]
fn config_auth_intent_and_source_changes_reject_before_target_mutation() {
    for changed in ["config", "auth", "intent", "source", "backup", "native"] {
        let f = Fixture::new(Some("model_provider='custom'\n"));
        let ticket = f.preview();
        match changed {
            "config" => fs::write(f.home.join("config.toml"), b"# external").unwrap(),
            "auth" => fs::write(f.home.join("auth.json"), b"external-auth").unwrap(),
            "intent" => {
                fs::create_dir_all(f.root.join("app/gateway")).unwrap();
                fs::write(
                    f.root.join("app/gateway/router-intent.json"),
                    b"external-intent",
                )
                .unwrap();
            }
            "source" => fs::write(
                f.store.home(&f.id).unwrap().join("auth.json"),
                b"external-source",
            )
            .unwrap(),
            "backup" => {
                fs::write(f.store.root.join("last-switch.json"), b"external-backup").unwrap()
            }
            "native" => {
                fs::create_dir_all(f.home.join("codexmate/relay")).unwrap();
                fs::write(f.home.join("codexmate/relay/state.json"), b"{}").unwrap();
            }
            _ => unreachable!(),
        }
        let config_before = fs::read(f.home.join("config.toml")).unwrap();
        let auth_before = read_external(&f.home.join("auth.json"), 256 * 1024).unwrap();
        assert!(apply_at(&f.store, &ticket, true).is_err(), "{changed}");
        assert_eq!(fs::read(f.home.join("config.toml")).unwrap(), config_before);
        assert_eq!(
            read_external(&f.home.join("auth.json"), 256 * 1024).unwrap(),
            auth_before
        );
    }
}

#[test]
fn partial_publish_failure_rolls_back_owned_effects_and_retains_originals() {
    for failure in 0..3 {
        for after_rename in [false, true] {
            let f = Fixture::new(Some("model_provider='custom'\n"));
            let ticket = f.preview();
            let mut calls = 0;
            let error = apply_with(&f.store, &ticket, true, |path, bytes| {
                let index = calls;
                calls += 1;
                if index == failure && !after_rename {
                    return Err("injected publication failure".into());
                }
                publish(path, bytes)?;
                if index == failure {
                    return Err("injected post-rename sync failure".into());
                }
                Ok(())
            })
            .unwrap_err();
            assert!(error.contains("已条件回退"), "{error}");
            assert!(!f.home.join("auth.json").exists());
            assert!(!f.root.join("app/gateway/router-intent.json").exists());
            assert!(!f.store.root.join("last-switch.json").exists());
            assert_eq!(
                fs::read_to_string(f.home.join("config.toml")).unwrap(),
                "model_provider='custom'\n"
            );
            assert!(f
                .store
                .root
                .join(format!("switch-{}.json", ticket.view.ticket))
                .is_file());
        }
    }
}

#[test]
fn compensation_does_not_overwrite_external_bytes_even_if_later_steps_fail() {
    let f = Fixture::new(Some("model_provider='custom'\n"));
    let ticket = f.preview();
    let mut calls = 0;
    let error = apply_with(&f.store, &ticket, true, |path, bytes| {
        calls += 1;
        publish(path, bytes)?;
        if calls == 2 {
            fs::write(f.home.join("auth.json"), b"external-race").unwrap();
            return Err("injected later failure".into());
        }
        Ok(())
    })
    .unwrap_err();
    assert!(error.contains("部分完成"));
    assert_eq!(
        fs::read(f.home.join("auth.json")).unwrap(),
        b"external-race"
    );
    assert_eq!(
        fs::read_to_string(f.home.join("config.toml")).unwrap(),
        "model_provider='custom'\n"
    );
    assert!(f.store.root.join("last-switch.json").exists());
}

#[test]
fn rollback_refuses_external_change_and_expired_ticket() {
    let f = Fixture::new(Some("model_provider='custom'\n"));
    let mut expired = f.preview();
    expired.view.expires_at = 0;
    assert!(apply_at(&f.store, &expired, true).is_err());
    let ticket = f.preview();
    apply_at(&f.store, &ticket, true).unwrap();
    fs::write(f.home.join("config.toml"), b"# external").unwrap();
    assert!(preview_at(&f.store, None, &f.home).is_err());
    assert!(f.store.root.join("last-switch.json").exists());
}

#[test]
fn managed_catalog_is_detached_without_deleting_it_or_provider_tables() {
    let f = Fixture::new(None);
    let path = f.root.join("app/gateway/codex_router_catalog.json");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, b"synthetic-preserve").unwrap();
    let input = format!("model_provider='custom'\nmodel_catalog_json={}\n[model_providers.custom]\nbase_url='https://example.invalid/v1'\n",serde_json::to_string(&path.to_string_lossy()).unwrap());
    fs::write(f.home.join("config.toml"), input).unwrap();
    apply_at(&f.store, &f.preview(), true).unwrap();
    assert_eq!(fs::read(&path).unwrap(), b"synthetic-preserve");
    assert!(!fs::read_to_string(f.home.join("config.toml"))
        .unwrap()
        .contains("model_catalog_json"));
}

#[test]
fn pending_switch_and_replaced_home_are_rejected() {
    let f = Fixture::new(Some("model_provider='custom'\n"));
    let ticket = f.preview();
    let old_home = f.root.join("original-home");
    fs::rename(&f.home, &old_home).unwrap();
    fs::create_dir(&f.home).unwrap();
    fs::copy(old_home.join("config.toml"), f.home.join("config.toml")).unwrap();
    assert!(apply_at(&f.store, &ticket, true)
        .unwrap_err()
        .contains("目录对象"));
    assert!(!f.home.join("auth.json").exists());
    let pending = json!({"version":2,"completed":false,"target":f.home.join("auth.json"),"before":null,"after_hash":"absent","config_hash":"fixture","selected_before":null,"files":[]});
    fs::write(
        f.store.root.join("last-switch.json"),
        serde_json::to_vec(&pending).unwrap(),
    )
    .unwrap();
    assert!(preview_at(&f.store, Some(f.id.clone()), &f.home).is_err());
    assert!(preview_at(&f.store, None, &f.home).is_err());
}

#[test]
fn shared_maintenance_lock_blocks_switch_without_target_writes() {
    let f = Fixture::new(Some("model_provider='custom'\n"));
    let ticket = f.preview();
    let _guard =
        crate::maintenance::diagnostics::account_write_guard(&f.home, &f.root.join("app")).unwrap();
    assert!(apply_at(&f.store, &ticket, true).is_err());
    assert!(!f.home.join("auth.json").exists());
    assert!(!f.store.root.join("last-switch.json").exists());
}

#[cfg(unix)]
#[test]
fn symlink_auth_and_config_are_never_followed() {
    use std::os::unix::fs::symlink;
    for filename in ["auth.json", "config.toml"] {
        let f = Fixture::new(None);
        let external = f.root.join("external");
        fs::write(&external, b"external-preserve").unwrap();
        symlink(&external, f.home.join(filename)).unwrap();
        assert!(preview_at(&f.store, Some(f.id.clone()), &f.home).is_err());
        assert_eq!(fs::read(&external).unwrap(), b"external-preserve");
    }
}

#[test]
fn legacy_backup_is_auth_only_and_rejects_configuration_change() {
    let f = Fixture::new(Some("# legacy\n"));
    fs::write(f.home.join("auth.json"), b"after").unwrap();
    let raw = json!({"target":f.home.join("auth.json"),"before":[98,101,102,111,114,101],"after_hash":hash(b"after"),"config_hash":hash(b"# legacy\n"),"selected_before":null});
    crate::gateway::storage::atomic_private(
        &f.store.root.join("last-switch.json"),
        &serde_json::to_vec(&raw).unwrap(),
    )
    .unwrap();
    let ticket = preview_at(&f.store, None, &f.home).unwrap();
    apply_at(&f.store, &ticket, true).unwrap();
    assert_eq!(fs::read(f.home.join("auth.json")).unwrap(), b"before");
    assert_eq!(fs::read(f.home.join("config.toml")).unwrap(), b"# legacy\n");
}
