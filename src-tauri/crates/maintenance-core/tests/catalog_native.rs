use atelier_maintenance_core::catalog;
#[path = "../src/catalog/credentials.rs"]
mod credentials;
#[path = "../src/catalog/publication.rs"]
mod publication;
#[path = "../src/catalog/state.rs"]
mod state;
use serde_json::json;
use state::RelayStateDocument;
use std::{fs, io, path::Path};
fn materialized_state() -> serde_json::Value {
    json!({
        "schemaVersion":16,"providers":[{"id":"provider","ide":"codex","name":"Synthetic provider","baseUrl":"https://example.invalid/v1","model":"deepseek-v4","wireApi":"openai-chat","brand":"openrouter","models":[{"model":"chosen","wireApi":"openai-responses","contextWindow":123456,"displayName":"Chosen","suffix":"test"}],"apiKeyStored":false,"credential":{"kind":"legacy","revision":2},"healthScore":90,"lastError":null,"network":"system"}],
        "activeByIde":{"codex":["provider"]},"proxy":{"port":12345},"codexRouterEnabled":true,"codexRouterNoAccountMode":true,"codexNoAccountSlots":[{"providerId":"provider","model":"chosen","catalogSlug":"gpt-5.5"}],"claudeWebSearchCompat":false,"unknownExtension":{"retain":[1,2]}
    })
}
#[test]
fn wire_projection_preserves_opaque_state_and_openrouter_semantics() {
    let value = materialized_state();
    let mut doc = RelayStateDocument::from_value(value.clone()).unwrap();
    assert_eq!(doc.schema_version().unwrap(), 16);
    let inputs = doc.catalog_inputs().unwrap();
    assert_eq!(
        inputs.providers[0].brand,
        catalog::builder::Brand::OpenRouter
    );

    let mut openrouter = inputs.providers[0].clone();
    openrouter.models.clear();
    let row =
        catalog::builder::make_relay_model_info(&openrouter, &openrouter.exposed_models()[0], None)
            .unwrap();
    assert_eq!(row["default_reasoning_level"], "medium");
    assert_eq!(row["context_window"], 272000);
    assert_eq!(inputs.providers[0].models[0].context_window, Some(123456));
    assert_eq!(inputs.aliases[0].target_model, "chosen");
    let projected = doc.diagnostic_state().unwrap();
    assert!(catalog::expected_slugs(&projected).contains("provider::chosen"));
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&doc.to_pretty_json().unwrap()).unwrap(),
        value
    );
    doc.set_router_flags(false, false);
    doc.set_active_codex_provider_ids(vec![]).unwrap();
    doc.set_router_commit_id(Some(
        "f6540691-006c-4ab2-8b36-67a904cb85f6"
            .to_owned()
            .try_into()
            .unwrap(),
    ));
    doc.set_router_commit_id(None);
    assert_eq!(
        doc.as_value()["unknownExtension"],
        value["unknownExtension"]
    );
    assert_eq!(
        doc.as_value()["providers"][0]["credential"],
        value["providers"][0]["credential"]
    );
    assert_eq!(doc.opaque_fields()["proxy"], value["proxy"]);
    assert!(!doc.catalog_inputs().unwrap().router_enabled);
}
#[test]
fn actual_wire_names_reject_guessed_enum_names() {
    let mut value = materialized_state();
    value["providers"][0]["brand"] = json!("mimo");
    assert!(RelayStateDocument::from_value(value)
        .unwrap()
        .catalog_inputs()
        .is_err());
    let mut value = materialized_state();
    value["providers"][0]["wireApi"] = json!("responses");
    assert!(RelayStateDocument::from_value(value)
        .unwrap()
        .catalog_inputs()
        .is_err());
    let mut value = materialized_state();
    value["providers"][0]["models"][0]["contextWindow"] = json!("123");
    assert!(RelayStateDocument::from_value(value)
        .unwrap()
        .catalog_inputs()
        .is_err());
    assert!(RelayStateDocument::from_json(b"[]").is_err());
    let mut future = materialized_state();
    future["schemaVersion"] = json!(17);
    assert!(RelayStateDocument::from_json(&serde_json::to_vec(&future).unwrap()).is_err());
    assert!(RelayStateDocument::from_value(future)
        .unwrap()
        .catalog_inputs()
        .is_err());
}
#[test]
fn bundle_priority_and_metadata_signature_use_only_synthetic_paths() {
    let dir = tempfile::tempdir().unwrap();
    let bundle = dir.path().join("Synthetic.app");
    let legacy = bundle.join("Contents/Resources/codex");
    fs::create_dir_all(legacy.parent().unwrap()).unwrap();
    fs::write(&legacy, b"legacy").unwrap();
    let first = publication::current_bundled_cli_signature(Some(&bundle));
    assert!(
        matches!(first,publication::CliSignature::Located {ref path,metadata:Some(ref m)} if *path==legacy && m.length==6)
    );
    let nested = bundle.join("Contents/Resources/codex-cli/CodexCLI.app/Contents/MacOS/codex");
    fs::create_dir_all(nested.parent().unwrap()).unwrap();
    fs::write(&nested, b"nested-cli").unwrap();
    assert_eq!(
        publication::codex_bundled_cli_path(&bundle),
        Some(nested.clone())
    );
    assert_ne!(
        first,
        publication::current_bundled_cli_signature(Some(&bundle))
    );
    assert!(matches!(
        publication::cli_signature_for_path(&dir.path().join("gone")),
        publication::CliSignature::Located { metadata: None, .. }
    ));
    assert_eq!(
        publication::current_bundled_cli_signature(None),
        publication::CliSignature::Unavailable
    );
}
#[test]
fn prepared_document_is_constructed_and_atomically_published() {
    let dir = tempfile::tempdir().unwrap();
    let doc = RelayStateDocument::from_value(materialized_state()).unwrap();
    let prepared = publication::prepare_catalog(&doc, None).unwrap();
    assert_eq!(prepared.document["models"][0]["slug"], "provider::chosen");
    assert_eq!(prepared.document["models"][1]["visibility"], "hide");
    assert!(!prepared.native_account);
    assert_eq!(
        prepared.cli_signature,
        publication::CliSignature::Unavailable
    );
    let root = dir.path().join("new/nested");
    let receipt = publication::write_prepared_catalog(&root, &prepared).unwrap();
    assert_eq!(receipt.destination, root.join(catalog::CATALOG_FILE));
    let readback: serde_json::Value =
        serde_json::from_slice(&fs::read(receipt.destination).unwrap()).unwrap();
    assert_eq!(readback, prepared.document);
    assert_eq!(fs::read_dir(root).unwrap().count(), 1);
    let state_path = dir.path().join("synthetic-relay-state.json");
    publication::write_atomic_report(&state_path, &doc.to_pretty_json().unwrap())
        .unwrap()
        .require_sync()
        .unwrap();
    assert_eq!(
        RelayStateDocument::read(&state_path).unwrap().as_value(),
        doc.as_value()
    );
}
#[cfg(unix)]
#[test]
fn atomic_mode_replacement_and_owned_cleanup() {
    use std::os::unix::fs::{symlink, PermissionsExt};
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("catalog.json");
    fs::write(&target, b"before").unwrap();
    fs::set_permissions(&target, fs::Permissions::from_mode(0o640)).unwrap();
    publication::write_atomic_report(&target, b"after")
        .unwrap()
        .require_sync()
        .unwrap();
    assert_eq!(fs::read(&target).unwrap(), b"after");
    assert_eq!(
        fs::metadata(&target).unwrap().permissions().mode() & 0o777,
        0o640
    );
    let staged = publication::stage_atomic(&target, b"unpublished").unwrap();
    let stage = staged.staging_path().to_owned();
    drop(staged);
    assert!(!stage.exists());
    let staged = publication::stage_atomic(&target, b"unpublished").unwrap();
    let stage = staged.staging_path().to_owned();
    fs::remove_file(&stage).unwrap();
    symlink(&target, &stage).unwrap();
    let error = staged.publish().unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    assert!(fs::symlink_metadata(&stage)
        .unwrap()
        .file_type()
        .is_symlink());
    assert_eq!(fs::read(&target).unwrap(), b"after");
    fs::remove_file(stage).unwrap();
}
#[cfg(unix)]
#[test]
fn hardlink_staging_is_rejected_and_rename_failure_cleans_only_owned_stage() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("target");
    let staged = publication::stage_atomic(&target, b"new").unwrap();
    let stage = staged.staging_path().to_owned();
    let link = dir.path().join("hardlink");
    fs::hard_link(&stage, &link).unwrap();
    assert_eq!(
        staged.publish().unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert!(stage.exists());
    assert!(!target.exists());
    fs::remove_file(stage).unwrap();
    fs::remove_file(link).unwrap();
    fs::create_dir(&target).unwrap();
    fs::write(target.join("keep"), b"unchanged").unwrap();
    let staged = publication::stage_atomic(&target, b"new").unwrap();
    let stage = staged.staging_path().to_owned();
    assert!(staged.publish().is_err());
    assert!(!stage.exists());
    assert_eq!(fs::read(target.join("keep")).unwrap(), b"unchanged");
}
#[test]
fn acknowledge_exposes_post_rename_sync_warning_without_reclassifying_publication() {
    let receipt = publication::Publication {
        destination: Path::new("synthetic").into(),
        directory_sync_error: Some(io::Error::from(io::ErrorKind::Unsupported)),
    }
    .acknowledge();
    assert_eq!(
        receipt.directory_sync_error,
        Some(io::ErrorKind::Unsupported)
    );
    assert!(publication::Publication {
        destination: Path::new("synthetic").into(),
        directory_sync_error: Some(io::Error::from(io::ErrorKind::PermissionDenied))
    }
    .require_sync()
    .is_err());
    assert_eq!(publication::temp_file_stem(Path::new("/")), "aimami");
    assert_eq!(
        publication::temp_file_stem(Path::new(&"中".repeat(60)))
            .chars()
            .count(),
        48
    );
}

#[test]
fn native_default_is_disabled_empty_and_prepares_a_real_empty_catalog() {
    let doc = RelayStateDocument::native_default();
    let input = doc.catalog_inputs().unwrap();
    assert_eq!(doc.schema_version().unwrap(), 16);
    assert!(!input.router_enabled);
    assert!(!input.no_account_mode);
    assert!(input.providers.is_empty());
    assert!(input.active_provider_ids.is_empty());
    assert!(input.aliases.is_empty());
    assert_eq!(
        doc.as_value()["proxy"],
        json!({"running":false,"port":0,"baseUrl":"","codexBaseUrl":"","lastError":null})
    );
    assert_eq!(doc.as_value()["claudeWebSearchCompat"], true);
    assert!(doc.as_value().get("routerCommitId").is_none());
    let prepared = publication::prepare_catalog(&doc, None).unwrap();
    assert_eq!(
        prepared.document,
        json!({"models":[],"aimami_no_account":false})
    );
}

#[test]
fn native_map_defaults_apply_only_to_missing_fields_and_preserve_the_document() {
    for value in [json!({}), json!({"activeByIde": {"extension": 1}})] {
        let doc = RelayStateDocument::from_json(&serde_json::to_vec(&value).unwrap()).unwrap();
        assert_eq!(doc.schema_version().unwrap(), 16);
        let input = doc.catalog_inputs().unwrap();
        assert!(input.providers.is_empty());
        assert!(input.active_provider_ids.is_empty());
        assert!(input.aliases.is_empty());
        assert!(!input.router_enabled);
        assert!(!input.no_account_mode);
        assert_eq!(doc.as_value(), &value);
        assert_eq!(
            publication::prepare_catalog(&doc, None).unwrap().document,
            json!({"models":[],"aimami_no_account":false})
        );
    }
    for value in [
        json!({"providers":null}),
        json!({"activeByIde":null}),
        json!({"activeByIde":{"codex":null}}),
        json!({"codexRouterEnabled":null}),
        json!({"codexRouterNoAccountMode":"false"}),
        json!({"codexNoAccountSlots":null}),
    ] {
        assert!(RelayStateDocument::from_value(value)
            .unwrap()
            .catalog_inputs()
            .is_err());
    }
    let doc = RelayStateDocument::from_json(br#"{"schemaVersion":-1}"#).unwrap();
    assert_eq!(doc.schema_version().unwrap(), -1);
    assert!(doc.catalog_inputs().is_ok());
    for value in [
        json!(null),
        json!(2147483648i64),
        json!(-2147483649i64),
        json!(1.5),
    ] {
        assert!(RelayStateDocument::from_json(
            &serde_json::to_vec(&json!({"schemaVersion":value})).unwrap()
        )
        .is_err());
    }
}

#[test]
fn commit_ids_follow_native_canonical_hyphenated_v4_validation() {
    let canonical = "f6540691-006c-4ab2-8b36-67a904cb85f6";
    let id: state::RouterOperationId = canonical.to_owned().try_into().unwrap();
    assert_eq!(id.as_str(), canonical);
    let mut doc = RelayStateDocument::native_default();
    doc.set_router_commit_id(Some(id.clone()));
    assert_eq!(doc.router_commit_id().unwrap(), Some(id));
    assert_eq!(doc.as_value()["routerCommitId"], canonical);
    assert_eq!(
        RelayStateDocument::from_json(&doc.to_pretty_json().unwrap())
            .unwrap()
            .router_commit_id()
            .unwrap()
            .unwrap()
            .as_str(),
        canonical
    );
    // Native checks version only; do not add a variant nibble constraint.
    assert!(state::RouterOperationId::try_from(canonical.replacen("8b36", "0b36", 1)).is_ok());
    for value in [
        json!(canonical.to_uppercase()),
        json!(canonical.replace('-', "")),
        json!(canonical.replacen("4ab2", "3ab2", 1)),
        json!(format!("{{{canonical}}}")),
        json!({"revision":3}),
        json!(1),
    ] {
        assert!(RelayStateDocument::from_json(
            &serde_json::to_vec(&json!({"routerCommitId":value})).unwrap()
        )
        .is_err());
    }
    assert!(RelayStateDocument::from_json(br#"{"routerCommitId":null}"#)
        .unwrap()
        .router_commit_id()
        .unwrap()
        .is_none());
    doc.set_router_commit_id(None);
    assert!(doc.as_value().get("routerCommitId").is_none());
}

#[test]
fn provider_catalog_field_defaults_match_the_native_map_end_branch() {
    let provider: state::NativeCatalogProvider = serde_json::from_value(json!({})).unwrap();
    assert!(provider.id.is_empty());
    assert!(provider.name.is_empty());
    assert!(provider.base_url.is_empty());
    assert!(provider.model.is_empty());
    assert!(provider.models.is_empty());
    assert_eq!(provider.wire_api, state::NativeWireApi::Chat);
    assert_eq!(provider.brand, state::NativeBrand::Generic);
    let value = json!({"providers":[{"id":"synthetic","model":"deepseek-v4"}], "activeByIde":{"codex":["synthetic"]}});
    let doc = RelayStateDocument::from_value(value.clone()).unwrap();
    let row = publication::prepare_catalog(&doc, None).unwrap().document;
    assert_eq!(row["models"][0]["slug"], "synthetic::deepseek-v4");
    assert_eq!(doc.as_value(), &value);
    for field in [
        "id", "name", "baseUrl", "model", "wireApi", "brand", "models",
    ] {
        assert!(
            serde_json::from_value::<state::NativeCatalogProvider>(json!({field:null})).is_err()
        );
    }
}

#[test]
fn model_fields_keep_required_model_and_wire_but_default_optional_metadata() {
    let model: state::NativeModelEntry =
        serde_json::from_value(json!({"model":"synthetic","wireApi":"openai-chat"})).unwrap();
    assert!(model.suffix.is_empty());
    assert!(model.context_window.is_none());
    assert!(model.display_name.is_none());
    for value in [
        json!({}),
        json!({"model":"synthetic"}),
        json!({"wireApi":"openai-chat"}),
        json!({"model":"synthetic","wireApi":"openai-chat","suffix":null}),
    ] {
        assert!(serde_json::from_value::<state::NativeModelEntry>(value).is_err());
    }
}

#[test]
fn proxy_typed_updates_preserve_extensions_and_catalog_repair_preserves_proxy() {
    let dir = tempfile::tempdir().unwrap();
    let mut doc =
        RelayStateDocument::from_value(json!({"proxy":{"port":12345,"extension":{"keep":[1,2]}}}))
            .unwrap();
    assert_eq!(
        doc.proxy_status().unwrap(),
        state::RelayProxyStatus {
            port: 12345,
            ..Default::default()
        }
    );
    doc.set_proxy_status(state::RelayProxyStatus {
        running: true,
        port: 23456,
        base_url: "http://127.0.0.1:23456".into(),
        codex_base_url: "http://127.0.0.1:23456/codex".into(),
        last_error: Some("synthetic".into()),
    })
    .unwrap();
    let before = doc.as_value()["proxy"].clone();
    doc.set_router_flags(true, false);
    publication::write_prepared_catalog(
        dir.path(),
        &publication::prepare_catalog(&doc, None).unwrap(),
    )
    .unwrap();
    publication::write_atomic_report(
        &dir.path().join("state.json"),
        &doc.to_pretty_json().unwrap(),
    )
    .unwrap()
    .require_sync()
    .unwrap();
    let restored = RelayStateDocument::read(&dir.path().join("state.json")).unwrap();
    assert_eq!(restored.as_value()["proxy"], before);
    assert_eq!(
        restored.as_value()["proxy"]["extension"],
        json!({"keep":[1,2]})
    );
    for proxy in [
        json!(null),
        json!({"port":65536}),
        json!({"port":-1}),
        json!({"running":null}),
        json!({"baseUrl":null}),
        json!({"lastError":1}),
    ] {
        assert!(RelayStateDocument::from_value(json!({"proxy":proxy}))
            .unwrap()
            .proxy_status()
            .is_err());
    }
}

#[test]
fn storage_shape_migration_keeps_legacy_codex_selection_and_commit_version_guard() {
    for (selection, expected) in [
        (json!("synthetic"), json!(["synthetic"])),
        (json!(null), json!([])),
    ] {
        let doc = RelayStateDocument::from_json(&serde_json::to_vec(&json!({"schemaVersion":2,"activeByIde":{"codex":selection,"extension":true},"extension":"retain"})).unwrap()).unwrap();
        assert_eq!(doc.as_value()["activeByIde"]["codex"], expected);
        assert_eq!(doc.as_value()["activeByIde"]["extension"], true);
        assert_eq!(doc.as_value()["extension"], "retain");
        assert_eq!(doc.schema_version().unwrap(), 2);
        assert!(doc.catalog_inputs().is_ok());
    }
    let commit = "f6540691-006c-4ab2-8b36-67a904cb85f6";
    for version in [0, 2, 14] {
        assert!(RelayStateDocument::from_json(
            &serde_json::to_vec(&json!({"schemaVersion":version,"routerCommitId":commit})).unwrap()
        )
        .is_err());
    }
    for version in [15, 16] {
        assert!(RelayStateDocument::from_json(
            &serde_json::to_vec(&json!({"schemaVersion":version,"routerCommitId":commit})).unwrap()
        )
        .is_ok());
    }
    assert!(RelayStateDocument::from_json(br#"{"claudeWebSearchCompat":null}"#).is_err());
    assert!(RelayStateDocument::from_json(b"{}")
        .unwrap()
        .claude_web_search_compat()
        .unwrap());
}

#[test]
fn official_protocol_migrations_use_url_model_and_version_branches() {
    let provider = |url: &str, model: &str| json!({"id":"synthetic","baseUrl":url,"model":model,"opaque":{"keep":true}});
    for (version, url, model, expected) in [
        (
            9,
            " https://API.MINIMAX.IO/v1/// ",
            " vendor/MINIMAX-M3 ",
            true,
        ),
        (10, "https://api.minimax.io/v1", "minimax-m3", false),
        (12, "https://api.deepseek.com", "deepseek-v4-flash", true),
        (
            13,
            "https://api.deepseek.com/v1",
            "deepseek-v4-flash",
            false,
        ),
        (9, "https://api.minimaxi.com:444/", "minimax-m3", true),
    ] {
        let mut doc = RelayStateDocument::from_value(
            json!({"schemaVersion":version,"providers":[provider(url,model)]}),
        )
        .unwrap();
        let count = doc.migrate_legacy_official_protocols().unwrap();
        assert_eq!(
            count.minimax_m3 + count.deepseek_v4_flash,
            usize::from(expected)
        );
        assert_eq!(
            doc.as_value()["providers"][0].get("wireApi"),
            expected.then(|| json!("openai-responses")).as_ref()
        );
        assert_eq!(
            doc.as_value()["providers"][0]["opaque"],
            json!({"keep":true})
        );
        assert_eq!(doc.schema_version().unwrap(), version);
        assert_eq!(
            doc.migrate_legacy_official_protocols().unwrap(),
            state::ProtocolMigrationCounts::default()
        );
    }
    for url in [
        "http://api.minimax.io/v1",
        "https://user@api.minimax.io/v1",
        "https://api.minimax.io/v1?",
        "https://api.minimax.io/v1#",
        "https://api.minimax.io/v1/completions",
        "https://api.minimax.io.example.invalid/v1",
    ] {
        let value = json!({"schemaVersion":9,"providers":[provider(url,"minimax-m3")]});
        let mut doc = RelayStateDocument::from_value(value.clone()).unwrap();
        assert_eq!(
            doc.migrate_legacy_official_protocols().unwrap(),
            state::ProtocolMigrationCounts::default()
        );
        assert_eq!(doc.as_value(), &value);
    }
    let mut p = provider("https://api.deepseek.com/v1", "deepseek-v4-flash");
    p["models"] = json!([{"model":"vendor/DEEPSEEK-V4-FLASH","wireApi":"openai-chat"},{"model":"deepseek-v4-flash/","wireApi":"openai-chat"},{"model":"deepseek-v4-flash","wireApi":"anthropic"}]);
    let mut doc =
        RelayStateDocument::from_value(json!({"schemaVersion":12,"providers":[p]})).unwrap();
    assert_eq!(
        doc.migrate_legacy_official_protocols()
            .unwrap()
            .deepseek_v4_flash,
        1
    );
    assert_eq!(
        doc.as_value()["providers"][0]["wireApi"],
        "openai-responses"
    );
    assert_eq!(
        doc.as_value()["providers"][0]["models"][0]["wireApi"],
        "openai-responses"
    );
    assert_eq!(
        doc.as_value()["providers"][0]["models"][1]["wireApi"],
        "openai-chat"
    );
    assert_eq!(
        doc.as_value()["providers"][0]["models"][2]["wireApi"],
        "anthropic"
    );
}

#[test]
fn legacy_slot_migration_filters_membership_deduplicates_and_reserves_aliases() {
    let value = json!({"schemaVersion":13,"providers":[{"id":"p","model":"fallback","models":[{"model":"one","wireApi":"openai-chat"},{"model":"two","wireApi":"openai-chat"},{"model":"three","wireApi":"openai-chat"}]},{"id":"inactive","model":"one"}],"activeByIde":{"codex":["p"]},"codexNoAccountSlots":[
        {"providerId":"inactive","model":"one"}, {"providerId":"p","model":"absent"},
        {"providerId":"p","model":"one","catalogSlug":"gpt-5.6-terra"},
        {"providerId":"p","model":"one","catalogSlug":"gpt-5.4"},
        {"providerId":"p","model":"two","catalogSlug":"gpt-5.6-terra"},
        {"providerId":"p","model":"three","catalogSlug":"gpt-5.4"}],"proxy":{"extension":"keep"}});
    let mut doc = RelayStateDocument::from_value(value).unwrap();
    assert!(doc.migrate_legacy_no_account_slots().unwrap());
    assert_eq!(
        doc.as_value()["codexNoAccountSlots"],
        json!([
        {"providerId":"p","model":"one","catalogSlug":"gpt-5.6-terra"},
        {"providerId":"p","model":"two","catalogSlug":"gpt-5.6-sol"},
        {"providerId":"p","model":"three","catalogSlug":"gpt-5.4"}])
    );
    assert_eq!(doc.as_value()["proxy"], json!({"extension":"keep"}));
    assert!(!doc.migrate_legacy_no_account_slots().unwrap());
    let mut no_account = doc.as_value().clone();
    no_account["codexRouterNoAccountMode"] = json!(true);
    no_account["activeByIde"]["codex"] = json!(["inactive", "p", "inactive"]);
    let mut doc = RelayStateDocument::from_value(no_account).unwrap();
    // Native returns retained valid slots before reaching the no-account fallback branch.
    assert!(!doc.migrate_legacy_no_account_slots().unwrap());
    assert_eq!(doc.as_value()["codexNoAccountSlots"][0]["providerId"], "p");
    let mut no_existing = doc.as_value().clone();
    no_existing["codexNoAccountSlots"] = json!([]);
    let mut doc = RelayStateDocument::from_value(no_existing).unwrap();
    assert!(doc.migrate_legacy_no_account_slots().unwrap());
    assert_eq!(
        doc.as_value()["codexNoAccountSlots"],
        json!([
        {"providerId":"inactive","model":"one","catalogSlug":"gpt-5.6-sol"},
        {"providerId":"p","model":"one","catalogSlug":"gpt-5.6-terra"},
        {"providerId":"p","model":"two","catalogSlug":"gpt-5.6-luna"},
        {"providerId":"p","model":"three","catalogSlug":"gpt-5.5"}])
    );
    assert!(!doc.migrate_legacy_no_account_slots().unwrap());
    let mut current = doc.as_value().clone();
    current["schemaVersion"] = json!(14);
    current["codexNoAccountSlots"] =
        json!([{"providerId":"absent","model":"invalid","catalogSlug":"invalid"}]);
    let mut doc = RelayStateDocument::from_value(current.clone()).unwrap();
    assert!(!doc.migrate_legacy_no_account_slots().unwrap());
    assert_eq!(doc.as_value(), &current);
}

#[test]
fn provider_id_migration_plans_stable_hashes_collisions_and_updates_references() {
    let value = json!({"schemaVersion":16,"providers":[{"id":"","apiKeyStored":false,"extension":"keep"},{"id":"aimami_relay_legacy_e3b0c44298fc1c14"},{"id":"aimami_relay_legacy_e3b0c44298fc1c14_1"}],"activeByIde":{"codex":["","","aimami_relay_legacy_e3b0c44298fc1c14"]},"codexNoAccountSlots":[{"providerId":"","model":"test","catalogSlug":"gpt-5.4"}],"proxy":{"extension":"keep"}});
    let mut doc = RelayStateDocument::from_value(value.clone()).unwrap();
    let plan = doc.plan_provider_id_migrations().unwrap();
    assert_eq!(
        plan,
        vec![state::ProviderIdMigration {
            old_id: "".into(),
            new_id: "aimami_relay_legacy_e3b0c44298fc1c14_2".into(),
            provider_index: 0
        }]
    );
    assert_eq!(doc.as_value(), &value);
    let dir = tempfile::tempdir().unwrap();
    let mut store = credentials::CredentialStore::new(
        dir.path().to_path_buf(),
        synthetic_credential_namespace(),
        credentials::MemoryKeychain::default(),
    );
    let receipt = credentials::migrate_and_publish_state(
        &mut doc,
        &serde_json::to_vec(&value).unwrap(),
        &dir.path().join("state.json"),
        &mut store,
    )
    .unwrap();
    assert_eq!(receipt.provider_id_migrations, plan);
    assert_eq!(doc.as_value()["providers"][0]["id"], plan[0].new_id);
    assert_eq!(doc.as_value()["providers"][0]["extension"], "keep");
    assert_eq!(
        doc.as_value()["activeByIde"]["codex"],
        json!([plan[0].new_id, "aimami_relay_legacy_e3b0c44298fc1c14"])
    );
    assert_eq!(
        doc.as_value()["codexNoAccountSlots"][0]["providerId"],
        plan[0].new_id
    );
    assert_eq!(doc.as_value()["proxy"], value["proxy"]);
    assert!(doc.plan_provider_id_migrations().unwrap().is_empty());
    assert!(
        RelayStateDocument::from_value(json!({"providers":[{"id":"same"},{"id":"same"}]}))
            .unwrap()
            .plan_provider_id_migrations()
            .is_err()
    );
}

#[cfg(unix)]
#[test]
fn private_publication_and_provider_migration_backup_use_real_0600_files() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    let state_path = dir.path().join("synthetic-state.json");
    let original = br#"{ "schemaVersion": 16, "providers": [{"id":""}], "opaque": "retain" }"#;
    fs::write(&state_path, original).unwrap();
    fs::set_permissions(&state_path, fs::Permissions::from_mode(0o644)).unwrap();
    let backup =
        publication::preserve_state_before_provider_id_migration(dir.path(), original).unwrap();
    assert_eq!(fs::read(&backup).unwrap(), original);
    assert_eq!(
        fs::metadata(&backup).unwrap().permissions().mode() & 0o777,
        0o600
    );
    let mut doc = RelayStateDocument::from_json(original).unwrap();
    let mut store = credentials::CredentialStore::new(
        dir.path().to_path_buf(),
        synthetic_credential_namespace(),
        credentials::MemoryKeychain::default(),
    );
    credentials::migrate_and_publish_state(&mut doc, original, &state_path, &mut store).unwrap();
    publication::write_atomic_private_report(&state_path, &doc.to_pretty_json().unwrap())
        .unwrap()
        .require_sync()
        .unwrap();
    assert_eq!(
        fs::metadata(&state_path).unwrap().permissions().mode() & 0o777,
        0o600
    );
    assert_eq!(fs::read(&backup).unwrap(), original);
    assert_eq!(
        RelayStateDocument::read(&state_path).unwrap().as_value()["providers"][0]["id"],
        "aimami_relay_legacy_e3b0c44298fc1c14"
    );
}

#[test]
fn native_credential_schema_and_presence_validation_cover_all_three_kinds() {
    use credentials::{CredentialRevision, RelayCredential};
    let revision = "0123456789abcdef0123456789abcdef";
    assert_eq!(
        serde_json::from_value::<RelayCredential>(json!({"kind":"revision","id":revision}))
            .unwrap(),
        RelayCredential::Revision {
            id: CredentialRevision::try_from(revision.to_owned()).unwrap()
        }
    );
    assert_eq!(
        CredentialRevision::try_from(revision.to_owned())
            .unwrap()
            .as_str(),
        revision
    );
    for v in [
        json!({"kind":"revision","id":revision.to_uppercase()}),
        json!({"kind":"revision","id":"123"}),
        json!({"kind":"revision","revision":revision}),
        json!({"kind":"unknown"}),
        json!(null),
    ] {
        assert!(serde_json::from_value::<RelayCredential>(v).is_err());
    }
    for (schema, stored, credential, valid) in [
        (15, false, json!({"kind":"none"}), false),
        (15, true, json!({"kind":"revision","id":revision}), false),
        (16, false, json!({"kind":"none"}), true),
        (16, true, json!({"kind":"none"}), false),
        (16, true, json!({"kind":"revision","id":revision}), true),
        (16, false, json!({"kind":"revision","id":revision}), false),
        (1, true, json!({"kind":"legacy"}), true),
        (16, false, json!({"kind":"legacy"}), true),
    ] {
        let doc=RelayStateDocument::from_value(json!({"schemaVersion":schema,"providers":[{"id":"synthetic","apiKeyStored":stored,"credential":credential}]})).unwrap();
        assert_eq!(credentials::validate_credentials(&doc).is_ok(), valid);
    }
    let legacy = RelayStateDocument::from_value(
        json!({"providers":[{"id":"invalid/id","apiKeyStored":true}]}),
    )
    .unwrap();
    assert!(credentials::validate_credentials(&legacy).is_ok());
    let invalid=RelayStateDocument::from_value(json!({"providers":[{"id":"invalid/id","apiKeyStored":true,"credential":{"kind":"revision","id":revision}}]})).unwrap();
    assert!(credentials::validate_credentials(&invalid).is_err());
    let mut keychain = credentials::MemoryKeychain::default();
    keychain.insert(
        "synthetic.service",
        "synthetic.account",
        b"synthetic-secret",
    );
    use credentials::LegacyKeychain;
    assert_eq!(
        keychain
            .get("synthetic.service", "synthetic.account")
            .unwrap(),
        Some(b"synthetic-secret".to_vec())
    );
    keychain
        .delete("synthetic.service", "synthetic.account")
        .unwrap();
    assert!(keychain
        .get("synthetic.service", "synthetic.account")
        .unwrap()
        .is_none());
    #[cfg(target_os = "macos")]
    {
        let _compiled_only = credentials::MacKeychain;
    }
}

fn synthetic_credential_namespace() -> credentials::CredentialNamespace {
    credentials::CredentialNamespace {
        legacy_service: "test.atelier.relay".into(),
        obfuscation_domain: "test-atelier-relay".into(),
        hostname: "synthetic-host".into(),
        username: "synthetic-user".into(),
    }
}
fn legacy_encoded(secret: &str) -> String {
    use base64::Engine;
    use sha2::{Digest, Sha256};
    let mask = Sha256::digest(b"test-atelier-relay::synthetic-host::synthetic-user");
    let bytes: Vec<u8> = secret
        .bytes()
        .enumerate()
        .map(|(i, b)| b ^ mask[i % 32])
        .collect();
    base64::engine::general_purpose::STANDARD.encode(bytes)
}
#[test]
fn key_storage_uses_file_then_encoded_legacy_then_protected_keychain() {
    let dir = tempfile::tempdir().unwrap();
    let mut backend = credentials::MemoryKeychain::default();
    backend.insert("test.atelier.relay", "p", b"keychain-secret");
    let mut store = credentials::CredentialStore::new(
        dir.path().into(),
        synthetic_credential_namespace(),
        backend,
    );
    fs::write(
        dir.path().join("secrets.json"),
        serde_json::to_vec(&json!({"p":legacy_encoded("legacy-secret")})).unwrap(),
    )
    .unwrap();
    assert_eq!(
        store.get_api_key("p").unwrap(),
        Some("legacy-secret".into())
    );
    assert!(store.keychain.operations.is_empty());
    assert_eq!(
        fs::read_to_string(dir.path().join("keys/p")).unwrap(),
        "legacy-secret"
    );
    store.set_api_key("p", "   ").unwrap();
    assert!(store.get_api_key("p").unwrap().is_none()); // present empty file prevents fallback
    assert!(store.keychain.operations.is_empty());
    fs::remove_file(dir.path().join("keys/p")).unwrap();
    fs::write(dir.path().join("secrets.json"), b"{}").unwrap();
    assert_eq!(
        store.get_api_key("p").unwrap(),
        Some("keychain-secret".into())
    );
    assert_eq!(store.keychain.operations.len(), 1);
    assert!(store.get_api_key("../outside").is_err());
    fs::write(dir.path().join("secrets.json"), b"{bad").unwrap();
    fs::remove_file(dir.path().join("keys/p")).unwrap();
    assert!(store.get_api_key("p").is_err());
    assert_eq!(store.keychain.operations.len(), 1); // corrupt legacy file does not fall back
    for id in [
        "",
        ".",
        "..",
        "../outside",
        "slash\\id",
        "nul\0id",
        "c1\u{85}",
    ] {
        assert!(!credentials::legacy_id_is_safe_identifier(id));
    }
    assert!(credentials::legacy_id_is_safe_identifier("legacy:id. "));
}
#[test]
fn stored_key_provider_migration_copies_before_state_and_cleans_after_publication() {
    let dir = tempfile::tempdir().unwrap();
    let mut backend = credentials::MemoryKeychain::default();
    backend.insert(
        "test.atelier.relay",
        "legacy:id",
        b"stored-synthetic-secret",
    );
    let mut store = credentials::CredentialStore::new(
        dir.path().into(),
        synthetic_credential_namespace(),
        backend,
    );
    let original = json!({"schemaVersion":16,"providers":[{"id":"legacy:id","apiKeyStored":true}],"activeByIde":{"codex":["legacy:id"]},"proxy":{"port":12345,"extension":"keep"}});
    let bytes = serde_json::to_vec(&original).unwrap();
    let mut doc = RelayStateDocument::from_value(original).unwrap();
    let plan = doc.plan_provider_id_migrations().unwrap();
    let state_path = dir.path().join("state.json");
    fs::write(&state_path, &bytes).unwrap();
    let receipt =
        credentials::migrate_and_publish_state(&mut doc, &bytes, &state_path, &mut store).unwrap();
    assert_eq!(receipt.destination, state_path);
    assert_eq!(receipt.provider_id_migrations, plan);
    assert_eq!(
        fs::read(receipt.provider_id_backup.unwrap()).unwrap(),
        bytes
    );
    assert_eq!(receipt.imported_plaintext_keys, 0);
    assert!(receipt.cleanup_errors.is_empty());
    assert!(receipt.directory_sync_error.is_none());
    assert_eq!(
        store.get_api_key(&plan[0].new_id).unwrap(),
        Some("stored-synthetic-secret".into())
    );
    assert_eq!(fs::read(store.root().join("keys/legacy:id")).unwrap(), b"");
    assert_eq!(
        store
            .keychain
            .operations
            .iter()
            .map(|o| o.0.as_str())
            .collect::<Vec<_>>(),
        vec!["get", "delete"]
    );
    assert_eq!(
        RelayStateDocument::read(&state_path).unwrap().as_value(),
        doc.as_value()
    );
    assert_eq!(doc.as_value()["providers"][0]["apiKeyStored"], true);
    assert_eq!(doc.as_value()["proxy"]["extension"], "keep");
}
#[test]
fn failed_copy_degrades_provider_and_failed_state_commit_preserves_old_stores() {
    let dir = tempfile::tempdir().unwrap();
    let mut backend = credentials::MemoryKeychain::default();
    backend.fail_next_get = Some(io::ErrorKind::PermissionDenied);
    let mut store = credentials::CredentialStore::new(
        dir.path().into(),
        synthetic_credential_namespace(),
        backend,
    );
    let original = json!({"schemaVersion":16,"providers":[{"id":"legacy:id","apiKeyStored":true}]});
    let bytes = serde_json::to_vec(&original).unwrap();
    let mut doc = RelayStateDocument::from_value(original.clone()).unwrap();
    let path = dir.path().join("state.json");
    credentials::migrate_and_publish_state(&mut doc, &bytes, &path, &mut store).unwrap();
    assert_eq!(doc.as_value()["providers"][0]["apiKeyStored"], false);
    assert!(doc.as_value()["providers"][0]["lastError"]
        .as_str()
        .unwrap()
        .contains("迁移失败"));
    assert_eq!(
        store
            .keychain
            .operations
            .iter()
            .map(|o| o.0.as_str())
            .collect::<Vec<_>>(),
        vec!["get"]
    );
    store
        .keychain
        .insert("test.atelier.relay", "legacy:id", b"retained-secret");
    let mut doc = RelayStateDocument::from_value(original.clone()).unwrap();
    let plan = doc.plan_provider_id_migrations().unwrap();
    let invalid_destination = dir.path().join("nonempty-directory");
    fs::create_dir(&invalid_destination).unwrap();
    fs::write(invalid_destination.join("keep"), b"retained").unwrap();
    assert!(credentials::migrate_and_publish_state(
        &mut doc,
        &bytes,
        &invalid_destination,
        &mut store
    )
    .is_err());
    assert_eq!(doc.as_value(), &original);
    assert_eq!(
        fs::read_to_string(dir.path().join("keys").join(&plan[0].new_id)).unwrap(),
        "retained-secret"
    );
    assert!(!dir.path().join("keys/legacy:id").exists());
    assert!(!store.keychain.operations.iter().any(|o| o.0 == "delete"));
}
#[test]
fn schema_one_plaintext_key_is_stored_before_plaintext_is_removed_from_state() {
    let dir = tempfile::tempdir().unwrap();
    let mut store = credentials::CredentialStore::new(
        dir.path().into(),
        synthetic_credential_namespace(),
        credentials::MemoryKeychain::default(),
    );
    let original = json!({"schemaVersion":1,"providers":[{"id":"synthetic","apiKey":"synthetic-plaintext"}],"activeByIde":{"codex":"synthetic"}});
    let bytes = serde_json::to_vec(&original).unwrap();
    let mut doc = RelayStateDocument::from_value(original.clone()).unwrap();
    let path = dir.path().join("state.json");
    let receipt =
        credentials::migrate_and_publish_state(&mut doc, &bytes, &path, &mut store).unwrap();
    assert_eq!(receipt.imported_plaintext_keys, 1);
    assert_eq!(doc.schema_version().unwrap(), 16);
    assert!(doc.as_value()["providers"][0].get("apiKey").is_none());
    assert_eq!(doc.as_value()["providers"][0]["apiKeyStored"], true);
    assert_eq!(
        store.get_api_key("synthetic").unwrap(),
        Some("synthetic-plaintext".into())
    );
    assert!(!String::from_utf8(fs::read(path).unwrap())
        .unwrap()
        .contains("synthetic-plaintext"));
    let blocked = tempfile::tempdir().unwrap();
    fs::write(blocked.path().join("keys"), b"blocking-file").unwrap();
    let mut store = credentials::CredentialStore::new(
        blocked.path().into(),
        synthetic_credential_namespace(),
        credentials::MemoryKeychain::default(),
    );
    let mut doc = RelayStateDocument::from_value(original.clone()).unwrap();
    let path = blocked.path().join("state.json");
    fs::write(&path, &bytes).unwrap();
    assert!(credentials::migrate_and_publish_state(&mut doc, &bytes, &path, &mut store).is_err());
    assert_eq!(doc.as_value(), &original);
    assert_eq!(fs::read(path).unwrap(), bytes);
}

#[cfg(unix)]
#[test]
fn credential_revisions_prepare_commit_rollback_and_validate_storage_identity() {
    use credentials::{CredentialRevision, RelayCredential};
    use std::os::unix::fs::{symlink, PermissionsExt};
    let dir = tempfile::tempdir().unwrap();
    let original = json!({"schemaVersion":16,"providers":[{"id":"p","apiKeyStored":false,"credential":{"kind":"none"}}]});
    let mut doc = RelayStateDocument::from_value(original.clone()).unwrap();
    assert!(credentials::prepare_credential_revision(dir.path(), "p", " \n").is_err());
    assert!(
        credentials::prepare_credential_revision(dir.path(), "p", &"x".repeat(0x100001)).is_err()
    );
    let prepared =
        credentials::prepare_credential_revision(dir.path(), "p", " revision-synthetic-secret ")
            .unwrap();
    assert_eq!(prepared.provider_id(), "p");
    let path = prepared.path().to_owned();
    assert_eq!(
        fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o600
    );
    assert_eq!(
        fs::metadata(dir.path().join("key-revisions"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o700
    );
    let reference = prepared.reference();
    let RelayCredential::Revision { id: revision } = reference.clone() else {
        panic!("revision reference")
    };
    assert_eq!(
        path,
        credentials::revision_key_path(dir.path(), "p", &revision).unwrap()
    );
    assert_eq!(
        credentials::read_credential_revision(dir.path(), "p", &revision).unwrap(),
        " revision-synthetic-secret "
    );
    let mut candidate = original.clone();
    candidate["providers"][0]["credential"] = serde_json::to_value(reference).unwrap();
    candidate["providers"][0]["apiKeyStored"] = json!(true);
    let state_path = dir.path().join("state.json");
    credentials::publish_credential_revisions(
        &mut doc,
        RelayStateDocument::from_value(candidate.clone()).unwrap(),
        vec![prepared],
        &state_path,
    )
    .unwrap();
    assert!(path.exists());
    assert_eq!(
        RelayStateDocument::read(&state_path).unwrap().as_value(),
        &candidate
    );
    let second =
        credentials::prepare_credential_revision(dir.path(), "p", "rollback-secret").unwrap();
    let second_path = second.path().to_owned();
    let mut next = candidate.clone();
    next["providers"][0]["credential"] = serde_json::to_value(second.reference()).unwrap();
    let blocked = dir.path().join("blocked-directory");
    fs::create_dir(&blocked).unwrap();
    fs::write(blocked.join("keep"), b"keep").unwrap();
    assert!(credentials::publish_credential_revisions(
        &mut doc,
        RelayStateDocument::from_value(next).unwrap(),
        vec![second],
        &blocked
    )
    .is_err());
    assert!(!second_path.exists());
    assert!(path.exists());
    assert_eq!(doc.as_value(), &candidate);
    let guard = credentials::prepare_credential_revision(dir.path(), "p", "discard").unwrap();
    let replaced = guard.path().to_owned();
    fs::remove_file(&replaced).unwrap();
    symlink(&path, &replaced).unwrap();
    drop(guard);
    assert!(fs::symlink_metadata(&replaced)
        .unwrap()
        .file_type()
        .is_symlink());
    assert!(credentials::read_credential_revision(
        dir.path(),
        "p",
        &CredentialRevision::try_from(
            replaced
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .split_once('.')
                .unwrap()
                .1
                .to_owned()
        )
        .unwrap()
    )
    .is_err());
    fs::remove_file(replaced).unwrap();
    let mut stored: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    stored["providerId"] = json!("other");
    fs::write(&path, serde_json::to_vec(&stored).unwrap()).unwrap();
    assert!(
        credentials::read_credential_revision(dir.path(), "p", &revision)
            .unwrap_err()
            .0
            .contains("ownership mismatch")
    );
    stored["providerId"] = json!("p");
    stored["unknownField"] = json!(true);
    fs::write(&path, serde_json::to_vec(&stored).unwrap()).unwrap();
    assert!(credentials::read_credential_revision(dir.path(), "p", &revision).is_err());
    fs::write(&path, vec![b'x'; 0x600401]).unwrap();
    assert!(
        credentials::read_credential_revision(dir.path(), "p", &revision)
            .unwrap_err()
            .0
            .contains("storage limit")
    );
}

#[test]
fn missing_copy_and_post_commit_cleanup_failure_keep_native_status_distinctions() {
    let dir = tempfile::tempdir().unwrap();
    let mut backend = credentials::MemoryKeychain::default();
    backend.insert("test.atelier.relay", "unsafe/id", b"must-not-read-path");
    let mut store = credentials::CredentialStore::new(
        dir.path().into(),
        synthetic_credential_namespace(),
        backend,
    );
    let value = json!({"schemaVersion":16,"providers":[{"id":"unsafe/id","apiKeyStored":true}]});
    let bytes = serde_json::to_vec(&value).unwrap();
    let mut doc = RelayStateDocument::from_value(value).unwrap();
    credentials::migrate_and_publish_state(
        &mut doc,
        &bytes,
        &dir.path().join("state.json"),
        &mut store,
    )
    .unwrap();
    assert_eq!(doc.as_value()["providers"][0]["apiKeyStored"], false);
    assert!(doc.as_value()["providers"][0]["lastError"]
        .as_str()
        .unwrap()
        .contains("无法自动定位"));
    assert!(store.keychain.operations.is_empty());
    let value = json!({"schemaVersion":16,"providers":[{"id":"legacy:id","apiKeyStored":true}]});
    let bytes = serde_json::to_vec(&value).unwrap();
    let mut doc = RelayStateDocument::from_value(value).unwrap();
    store.keychain.insert(
        "test.atelier.relay",
        "legacy:id",
        b"copy-before-cleanup-fails",
    );
    store.keychain.fail_next_delete = Some(io::ErrorKind::PermissionDenied);
    let receipt = credentials::migrate_and_publish_state(
        &mut doc,
        &bytes,
        &dir.path().join("state.json"),
        &mut store,
    )
    .unwrap();
    assert_eq!(
        receipt.cleanup_errors,
        vec![io::ErrorKind::PermissionDenied]
    );
    assert_eq!(doc.as_value()["providers"][0]["apiKeyStored"], true);
}
#[test]
fn deletion_cleanup_requires_the_actual_empty_file_barrier() {
    let dir = tempfile::tempdir().unwrap();
    let mut backend = credentials::MemoryKeychain::default();
    backend.insert("test.atelier.relay", "p", b"legacy-key");
    let mut store = credentials::CredentialStore::new(
        dir.path().into(),
        synthetic_credential_namespace(),
        backend,
    );
    assert!(store.cleanup_deleted_api_key("p").is_err());
    assert!(store.keychain.operations.is_empty());
    store.delete_api_key("p").unwrap();
    assert!(store.get_api_key("p").unwrap().is_none());
    store.set_api_key("p", "new-key").unwrap();
    assert!(store.cleanup_deleted_api_key("p").is_err());
    assert!(store.keychain.operations.is_empty());
    store.delete_api_key("p").unwrap();
    assert!(store.cleanup_deleted_api_key("p").unwrap().is_empty());
    assert_eq!(
        store
            .keychain
            .operations
            .iter()
            .map(|o| o.0.as_str())
            .collect::<Vec<_>>(),
        vec!["delete"]
    );
    assert!(store.get_api_key("p").unwrap().is_none());
}

#[test]
fn revision_sweep_keeps_referenced_and_unknown_files_but_removes_only_owned_revisions() {
    let dir = tempfile::tempdir().unwrap();
    let state_path = dir.path().join("state.json");
    let keep =
        credentials::prepare_credential_revision(dir.path(), "keep", "fixture keep").unwrap();
    let remove =
        credentials::prepare_credential_revision(dir.path(), "remove", "fixture remove").unwrap();
    let keep_path = keep.path().to_path_buf();
    let remove_path = remove.path().to_path_buf();
    let keep_ref = serde_json::to_value(keep.reference()).unwrap();
    let mut state = RelayStateDocument::native_default();
    let candidate = RelayStateDocument::from_value(json!({"schemaVersion":16,"providers":[
        {"id":"keep","apiKeyStored":true,"credential":keep.reference()},
        {"id":"remove","apiKeyStored":true,"credential":remove.reference()}
    ]}))
    .unwrap();
    credentials::publish_credential_revisions(
        &mut state,
        candidate,
        vec![keep, remove],
        &state_path,
    )
    .unwrap();
    state = RelayStateDocument::from_value(json!({"schemaVersion":16,"providers":[
        {"id":"keep","apiKeyStored":true,"credential":keep_ref}
    ], "extension":"preserve"}))
    .unwrap();
    fs::write(&state_path, state.to_pretty_json().unwrap()).unwrap();
    let unknown = dir.path().join("key-revisions/unknown");
    fs::write(&unknown, b"unrecognized fixture").unwrap();
    // Valid StoredKey at the wrong path is retained, not claimed as an owned revision.
    let wrong_name = dir.path().join("key-revisions/wrong-name");
    fs::copy(&remove_path, &wrong_name).unwrap();
    let error =
        credentials::cleanup_unreferenced_revisions(dir.path(), &state_path, &state).unwrap_err();
    assert_eq!(error.0, "unrecognized relay credential files retained");
    assert!(keep_path.is_file());
    assert!(!remove_path.exists());
    assert!(unknown.is_file());
    assert!(wrong_name.is_file());
    assert_eq!(
        RelayStateDocument::read(&state_path).unwrap().as_value(),
        state.as_value()
    );
    fs::remove_file(unknown).unwrap();
    fs::remove_file(wrong_name).unwrap();
    credentials::cleanup_unreferenced_revisions(dir.path(), &state_path, &state).unwrap();
}

#[test]
fn revision_sweep_compares_schema_order_ids_and_credentials_before_deleting() {
    let dir = tempfile::tempdir().unwrap();
    let state_path = dir.path().join("state.json");
    // Missing directory is native's early success, before reading any state file.
    credentials::cleanup_unreferenced_revisions(
        dir.path(),
        &state_path,
        &RelayStateDocument::native_default(),
    )
    .unwrap();
    let key = credentials::prepare_credential_revision(dir.path(), "old", "fixture key").unwrap();
    let path = key.path().to_path_buf();
    let mut state = RelayStateDocument::native_default();
    let candidate = RelayStateDocument::from_value(json!({"schemaVersion":16,"providers":[
        {"id":"old","apiKeyStored":true,"credential":key.reference()}
    ]}))
    .unwrap();
    credentials::publish_credential_revisions(&mut state, candidate, vec![key], &state_path)
        .unwrap();
    for changed in [
        json!({"schemaVersion":15,"providers":[]}),
        json!({"schemaVersion":16,"providers":[]}),
        json!({"schemaVersion":16,"providers":[{"id":"new"}]}),
    ] {
        let changed = RelayStateDocument::from_value(changed).unwrap();
        assert_eq!(
            credentials::cleanup_unreferenced_revisions(dir.path(), &state_path, &changed)
                .unwrap_err()
                .0,
            "relay state changed before credential cleanup"
        );
        assert!(path.is_file());
    }
    // Native comparison does not include apiKeyStored or unrelated Provider fields.
    let mut expected = state.as_value().clone();
    expected["providers"][0]["apiKeyStored"] = json!(false);
    expected["providers"][0]["name"] = json!("different display only");
    credentials::cleanup_unreferenced_revisions(
        dir.path(),
        &state_path,
        &RelayStateDocument::from_value(expected).unwrap(),
    )
    .unwrap();
    assert!(path.is_file());
}

#[test]
fn revision_sweep_enforces_entry_budget_and_retains_unknown_files() {
    let dir = tempfile::tempdir().unwrap();
    let state = RelayStateDocument::native_default();
    let state_path = dir.path().join("state.json");
    fs::write(&state_path, state.to_pretty_json().unwrap()).unwrap();
    let revisions = dir.path().join("key-revisions");
    fs::create_dir(&revisions).unwrap();
    for i in 0..4097 {
        fs::write(revisions.join(format!("unknown-{i}")), b"fixture").unwrap();
    }
    assert_eq!(
        credentials::cleanup_unreferenced_revisions(dir.path(), &state_path, &state)
            .unwrap_err()
            .0,
        "relay credential cleanup budget exceeded; remaining files retained"
    );
    assert_eq!(fs::read_dir(revisions).unwrap().count(), 4097);
}
