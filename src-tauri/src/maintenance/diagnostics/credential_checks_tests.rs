use super::*;
use credentials::{CredentialNamespace, CredentialStore, LegacyKeychain};
use serde_json::json;
use std::{collections::BTreeMap, fs, io, path::PathBuf};
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let root =
            std::env::temp_dir().join(format!("atelier-credential-check-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&root).unwrap();
        Self(root)
    }
    fn ctx(&self) -> Context {
        Context {
            home: self.0.join("codex"),
            app_root: self.0.join("app"),
            desktop_running: false,
            runtime: None,
            proxy_handle: None,
            official_catalog: None,
            working_directory: Some(self.0.clone()),
            sqlite_home_env: None,
        }
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn token(exp: Value, refresh: bool) -> Value {
    let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .encode(serde_json::to_vec(&json!({"exp":exp})).unwrap());
    json!({"tokens":{"access_token":format!("e30.{payload}.synthetic-secret"),"refresh_token":if refresh {"synthetic-refresh"} else {""}}})
}
#[test]
fn token_thresholds_refresh_and_output_redaction_are_native_equivalent() {
    for refresh in [false, true] {
        for (remaining, expected) in [
            (-1, "error"),
            (0, "error"),
            (1, "warning"),
            (3599, "warning"),
            (3600, "ok"),
            (86400, "ok"),
        ] {
            let auth = token(json!(10000 + remaining), refresh);
            let check = token_expiry(Some(&auth), false, false, 10000);
            assert_eq!(
                check.status, expected,
                "remaining={remaining}, refresh={refresh}"
            );
            assert!(!check.fixable);
            assert!(check.detail.contains(if refresh {
                "存在 refresh_token"
            } else {
                "缺少 refresh_token"
            }));
            let output = serde_json::to_string(&check).unwrap();
            assert!(!output.contains("synthetic-secret") && !output.contains("synthetic-refresh"));
        }
    }
}
#[test]
fn missing_malformed_and_non_file_observations_do_not_claim_session_health() {
    for auth in [json!({}), json!({"tokens":{"access_token":""}})] {
        assert_eq!(token_expiry(Some(&auth), false, false, 1).status, "error");
    }
    for auth in [
        json!({"tokens":{"access_token":"invalid"}}),
        token(json!("not-a-timestamp"), false),
        token(json!(null), false),
    ] {
        assert_eq!(token_expiry(Some(&auth), false, false, 1).status, "warning");
    }
    assert_eq!(token_expiry(None, false, false, 1).status, "ok");
    assert_eq!(
        token_expiry(Some(&json!({"auth_mode":"apikey"})), false, false, 1).status,
        "ok"
    );
    assert_eq!(token_expiry(Some(&json!({})), false, true, 1).status, "ok");
    assert_eq!(token_expiry(None, true, true, 1).status, "warning");
}
#[derive(Default)]
struct FakeKeychain {
    values: BTreeMap<String, Vec<u8>>,
    reads: usize,
    deletes: usize,
    fail: bool,
}
impl LegacyKeychain for FakeKeychain {
    fn get(&mut self, service: &str, id: &str) -> io::Result<Option<Vec<u8>>> {
        assert_eq!(service, "synthetic-only");
        self.reads += 1;
        if self.fail {
            Err(io::Error::other("synthetic failure"))
        } else {
            Ok(self.values.get(id).cloned())
        }
    }
    fn delete(&mut self, _: &str, _: &str) -> io::Result<()> {
        self.deletes += 1;
        panic!("observation must not delete")
    }
}
fn store(root: &Path, keychain: FakeKeychain) -> CredentialStore<FakeKeychain> {
    CredentialStore::new(
        root.to_owned(),
        CredentialNamespace {
            legacy_service: "synthetic-only".into(),
            obfuscation_domain: "synthetic-domain".into(),
            hostname: "synthetic-host".into(),
            username: "synthetic-user".into(),
        },
        keychain,
    )
}
fn state(providers: Value) -> RelayStateDocument {
    let mut value = RelayStateDocument::native_default().as_value().clone();
    value["providers"] = providers;
    RelayStateDocument::from_json(&serde_json::to_vec(&value).unwrap()).unwrap()
}
#[test]
fn authorized_fake_keychain_observation_never_migrates_and_empty_file_is_a_barrier() {
    let f = Fixture::new();
    let mut fake = FakeKeychain::default();
    fake.values
        .insert("relay-a".into(), b"synthetic-keychain-key".to_vec());
    let mut store = store(&f.0, fake);
    assert_eq!(
        observe_authorized_legacy(&mut store, "relay-a"),
        KeyObservation::Present
    );
    assert!(!f.0.join("keys").exists());
    assert_eq!(store.keychain.reads, 1);
    fs::create_dir(f.0.join("keys")).unwrap();
    fs::write(f.0.join("keys/relay-a"), b"").unwrap();
    assert_eq!(
        observe_authorized_legacy(&mut store, "relay-a"),
        KeyObservation::Missing
    );
    assert_eq!(store.keychain.reads, 1);
    assert_eq!(store.keychain.deletes, 0);
    store.keychain.fail = true;
    assert_eq!(
        observe_authorized_legacy(&mut store, "relay-b"),
        KeyObservation::Failed
    );
    assert!(!f.0.join("keys/relay-b").exists());
}
#[test]
fn authorized_legacy_obfuscation_is_read_through_core_without_writing() {
    use sha2::{Digest, Sha256};
    let f = Fixture::new();
    let key = b"synthetic-legacy-obfuscated-key";
    let mask = Sha256::digest(b"synthetic-domain::synthetic-host::synthetic-user");
    let encoded = base64::engine::general_purpose::STANDARD.encode(
        key.iter()
            .enumerate()
            .map(|(i, b)| b ^ mask[i % 32])
            .collect::<Vec<_>>(),
    );
    let bytes = serde_json::to_vec(&json!({"relay-a":encoded})).unwrap();
    fs::write(f.0.join("secrets.json"), &bytes).unwrap();
    let mut store = store(&f.0, FakeKeychain::default());
    assert_eq!(
        observe_authorized_legacy(&mut store, "relay-a"),
        KeyObservation::Present
    );
    assert_eq!(fs::read(f.0.join("secrets.json")).unwrap(), bytes);
    assert!(!f.0.join("keys").exists());
    assert_eq!(store.keychain.reads, 0);
}
#[test]
fn native_revision_reads_real_ownership_and_flags_without_legacy_fallback() {
    let f = Fixture::new();
    let prepared =
        credentials::prepare_credential_revision(&f.0, "relay-a", "synthetic-revision-key")
            .unwrap();
    let reference = serde_json::to_value(prepared.reference()).unwrap();
    let state = state(json!([
        {"id":"relay-a","apiKeyStored":true,"credential":reference},
        {"id":"relay-unused","apiKeyStored":false},
    ]));
    let before = fs::read(prepared.path()).unwrap();
    let check = native_keys(&state, &f.0, |_| panic!("revision must not read legacy"));
    assert_eq!(check.status, "ok");
    assert_eq!(fs::read(prepared.path()).unwrap(), before);
    fs::remove_file(prepared.path()).unwrap();
    assert_eq!(
        native_keys(&state, &f.0, |_| panic!("no fallback")).status,
        "error"
    );
}
#[test]
fn native_legacy_is_unknown_without_authorization_and_bad_references_do_not_pass() {
    let f = Fixture::new();
    let s = state(json!([{"id":"relay-a","apiKeyStored":true}]));
    assert_eq!(
        native_keys(&s, &f.0, |_| KeyObservation::Unknown).status,
        "warning"
    );
    assert_eq!(
        native_keys(&s, &f.0, |_| KeyObservation::Missing).status,
        "error"
    );
    assert_eq!(
        native_keys(&s, &f.0, |_| KeyObservation::Failed).status,
        "error"
    );
    assert_eq!(
        native_keys(&s, &f.0, |_| KeyObservation::Present).status,
        "ok"
    );
    let mut invalid = RelayStateDocument::native_default().as_value().clone();
    invalid["providers"] =
        json!([{"id":"relay-a","apiKeyStored":true,"credential":{"kind":"none"}}]);
    assert!(RelayStateDocument::from_json(&serde_json::to_vec(&invalid).unwrap()).is_err());
}
#[test]
fn gateway_and_native_sources_use_distinct_schemas_and_inspection_is_read_only() {
    use crate::gateway::types::{Catalog, Credential, Profile, ProfileKind, Settings};
    let f = Fixture::new();
    let ctx = f.ctx();
    fs::create_dir_all(&ctx.home).unwrap();
    fs::write(ctx.config(), "").unwrap();
    fs::create_dir_all(ctx.app_root.join("gateway/credentials")).unwrap();
    let id = uuid::Uuid::new_v4().to_string();
    let p = Profile {
        id: id.clone(),
        kind: ProfileKind::ApiKey,
        name: "synthetic".into(),
        email: None,
        enabled: true,
        priority: 1,
        base_url: "http://synthetic.invalid".into(),
        auth_status: "ready".into(),
        models: vec![],
        cooldown_until: None,
        last_error: None,
        requests: 0,
        input_tokens: 0,
        output_tokens: 0,
        created_at: 0,
    };
    let catalog = Catalog {
        revision: "synthetic".into(),
        host_id: "synthetic".into(),
        profiles: vec![p],
        settings: Settings::default(),
    };
    let path = ctx.app_root.join("gateway/catalog.json");
    fs::write(&path, serde_json::to_vec(&catalog).unwrap()).unwrap();
    let secret = ctx
        .app_root
        .join("gateway/credentials")
        .join(format!("{id}.json"));
    let bytes = serde_json::to_vec(&Credential {
        api_key: Some("synthetic-gateway-key".into()),
        ..Default::default()
    })
    .unwrap();
    fs::write(&secret, &bytes).unwrap();
    let checks = inspect(&ctx, &toml::Value::Table(Default::default()), None, 10000);
    assert_eq!(checks[0].status, "ok");
    assert!(checks[0].detail.contains("Atelier Gateway"));
    assert_eq!(fs::read(&secret).unwrap(), bytes);
    fs::create_dir_all(ctx.home.join("codexmate/relay")).unwrap();
    let native = state(json!([{"id":"relay-a","apiKeyStored":true}]));
    fs::write(
        native_catalog::native_state_path(&ctx.home),
        serde_json::to_vec(native.as_value()).unwrap(),
    )
    .unwrap();
    assert_eq!(
        inspect(&ctx, &toml::Value::Table(Default::default()), None, 10000)[0].status,
        "warning"
    );
    assert!(!ctx.home.join("codexmate/relay/keys").exists());
    fs::write(native_catalog::native_state_path(&ctx.home), b"invalid").unwrap();
    assert_eq!(
        inspect(&ctx, &toml::Value::Table(Default::default()), None, 10000)[0].status,
        "warning"
    );
    assert!(!serde_json::to_string(&checks)
        .unwrap()
        .contains("synthetic-gateway-key"));
}
