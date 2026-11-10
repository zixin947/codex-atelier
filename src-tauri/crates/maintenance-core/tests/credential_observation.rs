use atelier_maintenance_core::catalog::credentials::{
    CredentialNamespace, CredentialStore, MemoryKeychain,
};
use std::fs;

fn namespace() -> CredentialNamespace {
    CredentialNamespace {
        legacy_service: "synthetic-observation".into(),
        obfuscation_domain: "synthetic".into(),
        hostname: "fixture-host".into(),
        username: "fixture-user".into(),
    }
}

#[test]
fn keychain_observation_does_not_migrate_or_delete() {
    let root = tempfile::tempdir().unwrap();
    let mut keychain = MemoryKeychain::default();
    keychain.insert("synthetic-observation", "provider", b"fixture-secret");
    let mut store = CredentialStore::new(root.path().into(), namespace(), keychain);
    assert_eq!(
        store.observe_api_key("provider").unwrap().as_deref(),
        Some("fixture-secret")
    );
    assert!(!root.path().join("keys").exists());
    assert!(!root.path().join("secrets.json").exists());
    assert_eq!(
        store.keychain.operations,
        vec![(
            "get".into(),
            "synthetic-observation".into(),
            "provider".into()
        )]
    );
    assert_eq!(
        store.observe_api_key("provider").unwrap().as_deref(),
        Some("fixture-secret")
    );
    assert!(!root.path().join("keys").exists());
}

#[test]
fn private_file_and_tombstone_prevent_legacy_fallback() {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir(root.path().join("keys")).unwrap();
    let key = root.path().join("keys/provider");
    fs::write(&key, " file-value \n").unwrap();
    let mut keychain = MemoryKeychain::default();
    keychain.insert("synthetic-observation", "provider", b"legacy-value");
    let mut store = CredentialStore::new(root.path().into(), namespace(), keychain);
    assert_eq!(
        store.observe_api_key("provider").unwrap().as_deref(),
        Some("file-value")
    );
    assert_eq!(fs::read(&key).unwrap(), b" file-value \n");
    fs::write(&key, "").unwrap();
    assert_eq!(store.observe_api_key("provider").unwrap(), None);
    assert!(store.keychain.operations.is_empty());
}

#[test]
fn observation_propagates_errors_and_rejects_unsafe_ids_without_io() {
    let root = tempfile::tempdir().unwrap();
    let mut keychain = MemoryKeychain::default();
    keychain.fail_next_get = Some(std::io::ErrorKind::PermissionDenied);
    let mut store = CredentialStore::new(root.path().into(), namespace(), keychain);
    assert!(store.observe_api_key("../escape").is_err());
    assert!(store.keychain.operations.is_empty());
    assert_eq!(
        store.observe_api_key("provider").unwrap_err().kind(),
        std::io::ErrorKind::PermissionDenied
    );
    assert!(!root.path().join("keys").exists());
}
