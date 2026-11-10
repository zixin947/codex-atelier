//! Synthetic credentials and temporary roots only. No native context/keychain/network.
use super::*;
use std::fs;
const CREDENTIAL: &[u8] =
    br#"{"tokens":{"access_token":"e30.e30.synthetic-signature"},"fixture":true}"#;
struct Fixture {
    root: PathBuf,
    ctx: Context,
    source: PathBuf,
}
impl Fixture {
    fn new(existing: bool) -> Self {
        let root =
            std::env::temp_dir().join(format!("atelier-auth-workflow-{}", uuid::Uuid::new_v4()));
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
        let backup_root = ctx.home.join("accounts/backups");
        fs::create_dir_all(&backup_root).unwrap();
        let source = backup_root.join("a-backup.json");
        fs::write(&source, CREDENTIAL).unwrap();
        if existing {
            fs::write(ctx.home.join("auth.json"), b"broken-original").unwrap();
        }
        Self { root, ctx, source }
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn auth_manifest(root: &Path) -> (PathBuf, serde_json::Value) {
    let mut pending = vec![root.to_owned()];
    while let Some(dir) = pending.pop() {
        for entry in fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                pending.push(path);
                continue;
            }
            if path.file_name().is_some_and(|n| n == "manifest.json") {
                let value: serde_json::Value =
                    serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
                if value["itemId"] == "auth_integrity" {
                    return (dir, value);
                }
            }
        }
    }
    panic!("actual native auth manifest must exist");
}
#[test]
fn preview_is_pure_and_confirm_restores_exact_bytes_with_original_backup() {
    let f = Fixture::new(true);
    let ticket = repair::preview(&f.ctx, "auth_integrity").unwrap();
    assert_eq!(
        fs::read(f.ctx.home.join("auth.json")).unwrap(),
        b"broken-original"
    );
    assert!(!f.ctx.app_root.join("maintenance").exists());
    let view = serde_json::to_string(&ticket.view).unwrap();
    assert!(!view.contains("e30.e30.synthetic-signature"));
    let result = repair::apply(&f.ctx, ticket).unwrap();
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), CREDENTIAL);
    assert_eq!(fs::read(&f.source).unwrap(), CREDENTIAL);
    assert_eq!(result.fixed_ids, ["auth_integrity"]);
    let (root, manifest) = auth_manifest(&PathBuf::from(result.backup_path.unwrap()));
    let original = manifest["files"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["path"].as_str() == f.ctx.home.join("auth.json").to_str())
        .unwrap();
    assert_eq!(
        fs::read(root.join(original["backup"].as_str().unwrap())).unwrap(),
        b"broken-original"
    );
}
#[test]
fn all_can_restore_a_missing_auth_file_without_fabricating_a_backup() {
    let f = Fixture::new(false);
    let result = repair::apply(&f.ctx, repair::preview(&f.ctx, "all").unwrap()).unwrap();
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), CREDENTIAL);
    assert!(result.fixed_ids.iter().any(|id| id == "auth_integrity"));
    let backup = PathBuf::from(result.backup_path.unwrap());
    assert!(!backup.join("0000.bin").exists());
    let (_, manifest) = auth_manifest(&backup);
    assert_eq!(manifest["files"][0]["operation"], "create");
    assert!(manifest["files"][0]["backup"].is_null());
}
#[test]
fn modified_source_after_preview_is_not_copied() {
    let f = Fixture::new(true);
    let ticket = repair::preview(&f.ctx, "auth_integrity").unwrap();
    fs::write(&f.source, br#"{"OPENAI_API_KEY":"external-synthetic"}"#).unwrap();
    assert!(repair::apply(&f.ctx, ticket).unwrap_err().contains("变化"));
    assert_eq!(
        fs::read(f.ctx.home.join("auth.json")).unwrap(),
        b"broken-original"
    );
    assert!(!f
        .ctx
        .app_root
        .join("maintenance/diagnostic-backups")
        .exists());
}
#[test]
fn newly_enabled_router_intent_blocks_auth_restore() {
    let f = Fixture::new(true);
    let ticket = repair::preview(&f.ctx, "auth_integrity").unwrap();
    fs::create_dir_all(f.ctx.app_root.join("gateway")).unwrap();
    fs::write(super::router_intent::path(&f.ctx), br#"{"schemaVersion":1,"target":"routerOn","provider":"atelier","operationId":null,"catalogRevision":null,"updatedAtMs":1}"#).unwrap();
    assert!(repair::apply(&f.ctx, ticket).unwrap_err().contains("路由"));
    assert_eq!(
        fs::read(f.ctx.home.join("auth.json")).unwrap(),
        b"broken-original"
    );
}
#[test]
fn a_new_login_after_preview_is_never_overwritten() {
    for existing in [false, true] {
        let f = Fixture::new(existing);
        let ticket = repair::preview(&f.ctx, "auth_integrity").unwrap();
        let external = br#"{"tokens":{"access_token":"synthetic-new-login"}}"#;
        fs::write(f.ctx.home.join("auth.json"), external).unwrap();
        assert!(repair::apply(&f.ctx, ticket).is_err());
        assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), external);
        assert_eq!(fs::read(&f.source).unwrap(), CREDENTIAL);
        assert!(!f
            .ctx
            .app_root
            .join("maintenance/diagnostic-backups")
            .exists());
    }
}
#[test]
fn keyring_mode_has_no_file_auth_restore_action() {
    let f = Fixture::new(true);
    fs::write(f.ctx.config(), "cli_auth_credentials_store = 'keyring'\n").unwrap();
    assert!(repair::preview(&f.ctx, "auth_integrity").is_err());
    assert_eq!(
        fs::read(f.ctx.home.join("auth.json")).unwrap(),
        b"broken-original"
    );
}

#[test]
fn malformed_but_eligible_backup_is_copied_then_reported_as_not_fixed() {
    let f = Fixture::new(true);
    let backup = br#"{"tokens":{"access_token":"synthetic-malformed-token"}}"#;
    fs::write(&f.source, backup).unwrap();
    let ticket = repair::preview(&f.ctx, "auth_integrity").unwrap();
    let result = repair::apply(&f.ctx, ticket).unwrap();
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), backup);
    assert!(result.fixed_ids.is_empty());
    assert_eq!(result.failed_ids, vec!["auth_integrity"]);
    assert_eq!(
        result
            .report
            .items
            .iter()
            .find(|i| i.id == "auth_integrity")
            .unwrap()
            .status,
        "warning"
    );
    assert!(!serde_json::to_string(&result)
        .unwrap()
        .contains("synthetic-malformed-token"));
}

#[test]
fn unrelated_root_json_is_not_a_native_auth_recovery_candidate() {
    let f = Fixture::new(true);
    fs::remove_file(&f.source).unwrap();
    fs::write(f.ctx.home.join("a-backup.json"), CREDENTIAL).unwrap();
    assert!(repair::preview(&f.ctx, "auth_integrity").is_err());
    assert_eq!(
        fs::read(f.ctx.home.join("auth.json")).unwrap(),
        b"broken-original"
    );
}
#[test]
fn actual_all_restores_account_backup_before_unlock_and_preserves_that_login() {
    let f = Fixture::new(false);
    let relay = f.ctx.home.join("codexmate/relay");
    fs::create_dir_all(&relay).unwrap();
    fs::write(
        relay.join("router-unlock-auth-marker.json"),
        br#"{"backedUpPreviousAuth":true}"#,
    )
    .unwrap();
    let old = br#"{"OPENAI_API_KEY":"sk-synthetic-old-backup-B"}"#;
    fs::write(relay.join("router-unlock-auth-backup.json"), old).unwrap();
    let ticket = repair::preview(&f.ctx, "all").unwrap();
    for id in ["auth_integrity", "router_unlock_auth_residue"] {
        assert!(ticket.view.item_ids.iter().any(|selected| selected == id));
    }
    let result = repair::apply(&f.ctx, ticket).unwrap();
    for id in ["auth_integrity", "router_unlock_auth_residue"] {
        assert!(
            result.fixed_ids.iter().any(|fixed| fixed == id),
            "{id}: {:?}",
            result.details
        );
    }
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), CREDENTIAL);
    assert_eq!(fs::read(&f.source).unwrap(), CREDENTIAL);
    assert!(!relay.join("router-unlock-auth-marker.json").exists());
    assert!(!relay.join("router-unlock-auth-backup.json").exists());
    let result_json = serde_json::to_string(&result).unwrap();
    assert!(!result_json.contains("synthetic-old-backup-B"));
    assert!(!result_json.contains("synthetic-signature"));
}
