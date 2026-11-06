use super::*;
use serde_json::json;
use std::fs;
struct Fixture {
    base: PathBuf,
    ctx: Context,
}
impl Fixture {
    fn new() -> Self {
        let base = std::env::temp_dir().join(format!("atelier-diag-test-{}", uuid::Uuid::new_v4()));
        let ctx = Context {
            home: base.join("codex"),
            app_root: base.join("atelier"),
            desktop_running: false,
            runtime: None,
        };
        fs::create_dir_all(ctx.home.join("sessions")).unwrap();
        fs::create_dir_all(&ctx.app_root).unwrap();
        Self { base, ctx }
    }
    fn config(&self, s: &str) {
        fs::write(self.ctx.config(), s).unwrap()
    }
    fn thread(&self, db_provider: &str, rollout_provider: &str, compressed: bool) -> PathBuf {
        let path = self.ctx.home.join(if compressed {
            "sessions/one.jsonl.zst"
        } else {
            "sessions/one.jsonl"
        });
        let text = format!(
            "{}\n{}\n",
            json!({"type":"session_meta","payload":{"id":"thread-1","model_provider":rollout_provider}}),
            json!({"type":"event_msg","payload":{"type":"user_message","message":"keep exact body"}})
        );
        fs::write(
            &path,
            if compressed {
                zstd::stream::encode_all(text.as_bytes(), 3).unwrap()
            } else {
                text.as_bytes().to_vec()
            },
        )
        .unwrap();
        let db = rusqlite::Connection::open(self.ctx.home.join("state_5.sqlite")).unwrap();
        db.execute_batch("CREATE TABLE threads(id TEXT PRIMARY KEY,model_provider TEXT,model TEXT,rollout_path TEXT)").unwrap();
        db.execute(
            "INSERT INTO threads VALUES ('thread-1',?1,'gpt-fixture',?2)",
            rusqlite::params![db_provider, path.to_str().unwrap()],
        )
        .unwrap();
        path
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.base);
    }
}
fn item<'a>(r: &'a DiagnosticReport, id: &str) -> &'a DiagnosticItem {
    r.items.iter().find(|i| i.id == id).unwrap()
}
#[test]
fn exact_35_unique_checks_without_codex_writes() {
    let f = Fixture::new();
    f.config("# preserve\nmodel = 'gpt-fixture'\n");
    let before = fs::read(f.ctx.config()).unwrap();
    let r = inspect::run(&f.ctx).unwrap();
    assert_eq!(r.items.len(), 35);
    assert_eq!(
        r.items
            .iter()
            .map(|i| &i.id)
            .collect::<std::collections::HashSet<_>>()
            .len(),
        35
    );
    assert_eq!(before, fs::read(f.ctx.config()).unwrap());
    assert!(!f.ctx.home.join(".aimami_probe").exists());
    assert_eq!(item(&r, "auth_integrity").status, "unknown");
    assert_eq!(item(&r, "router_postcondition").status, "skipped");
}
#[test]
fn invalid_profile_preview_and_apply_preserve_comments() {
    let f = Fixture::new();
    f.config("# keep\nprofile = 'missing'\nmodel = 'fixture' # model note\n");
    let t = repair::preview(&f.ctx, "config_profile_conflict").unwrap();
    assert!(fs::read_to_string(f.ctx.config())
        .unwrap()
        .contains("profile"));
    let result = repair::apply(&f.ctx, t).unwrap();
    assert_eq!(result.fixed_ids, vec!["config_profile_conflict"]);
    let text = fs::read_to_string(f.ctx.config()).unwrap();
    assert!(text.contains("# keep") && text.contains("# model note"));
    assert!(!text.contains("profile"));
    assert!(PathBuf::from(result.backup_path.unwrap())
        .join("manifest.json")
        .is_file());
}
#[test]
fn valid_custom_profile_is_not_removed() {
    let f = Fixture::new();
    f.config("profile = 'work'\n[profiles.work]\nmodel = 'gpt-fixture'\n");
    let r = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&r, "config_profile_conflict").status, "ok");
    assert!(repair::preview(&f.ctx, "config_profile_conflict").is_err());
}
#[test]
fn stale_preview_rejects_config_change() {
    let f = Fixture::new();
    f.config("profile = 'missing'\n");
    let t = repair::preview(&f.ctx, "all").unwrap();
    f.config("profile='missing'\nmodel='external'\n");
    let before = fs::read(f.ctx.config()).unwrap();
    assert!(repair::apply(&f.ctx, t).unwrap_err().contains("外部更新"));
    assert_eq!(before, fs::read(f.ctx.config()).unwrap());
}
#[test]
fn running_desktop_prevents_write() {
    let mut f = Fixture::new();
    f.config("profile = 'missing'\n");
    let t = repair::preview(&f.ctx, "all").unwrap();
    f.ctx.desktop_running = true;
    assert!(repair::apply(&f.ctx, t).unwrap_err().contains("仍在运行"));
    assert!(fs::read_to_string(f.ctx.config())
        .unwrap()
        .contains("profile"));
}
#[test]
fn provider_repair_updates_db_and_plain_rollout() {
    let f = Fixture::new();
    f.config("");
    let path = f.thread("atelier_old", "atelier_old", false);
    let before = fs::read(&path).unwrap();
    let t = repair::preview(&f.ctx, "thread_provider_consistency").unwrap();
    let r = repair::apply(&f.ctx, t).unwrap();
    assert!(r.fixed_ids.contains(&"thread_provider_consistency".into()));
    let after = fs::read(&path).unwrap();
    assert_eq!(
        before.split(|b| *b == b'\n').nth(1),
        after.split(|b| *b == b'\n').nth(1)
    );
    assert_eq!(inspect::meta(&after).unwrap()["model_provider"], "openai");
    let db = inspect::open_db(&f.ctx.home.join("state_5.sqlite"), false).unwrap();
    assert_eq!(inspect::rows(&db).unwrap()[0].provider, "openai");
}
#[test]
fn compressed_rollout_provider_repair_retains_compression() {
    let f = Fixture::new();
    f.config("");
    let path = f.thread("atelier_old", "atelier_old", true);
    let r = repair::apply(&f.ctx, repair::preview(&f.ctx, "all").unwrap()).unwrap();
    assert!(r.failed_ids.is_empty());
    assert_eq!(
        inspect::meta(&inspect::rollout(&path).unwrap()).unwrap()["model_provider"],
        "openai"
    );
    assert_eq!(&fs::read(&path).unwrap()[..4], &[0x28, 0xb5, 0x2f, 0xfd]);
}
#[test]
fn external_rollout_change_prevents_any_mutation() {
    let f = Fixture::new();
    f.config("");
    let path = f.thread("atelier_old", "atelier_old", false);
    let t = repair::preview(&f.ctx, "all").unwrap();
    fs::write(&path, b"external data").unwrap();
    assert!(repair::apply(&f.ctx, t).is_err());
    let db = inspect::open_db(&f.ctx.home.join("state_5.sqlite"), false).unwrap();
    assert_eq!(inspect::rows(&db).unwrap()[0].provider, "atelier_old");
    assert_eq!(fs::read(&path).unwrap(), b"external data");
}
#[test]
fn invalid_toml_is_error_not_auto_repair() {
    let f = Fixture::new();
    f.config("[bad\n");
    let r = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&r, "config_toml_syntax").status, "error");
    assert!(!item(&r, "config_toml_syntax").fixable);
    assert_eq!(item(&r, "config_global_validity").status, "unknown");
}
#[test]
fn missing_records_not_fabricated() {
    let f = Fixture::new();
    f.config("");
    let path = f.thread("openai", "openai", false);
    fs::remove_file(&path).unwrap();
    let r = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&r, "thread_missing_rollouts").status, "warning");
    assert!(!item(&r, "thread_missing_rollouts").fixable);
    assert!(!path.exists());
}
#[test]
fn report_never_returns_credentials_or_conversation_text() {
    let f = Fixture::new();
    f.config("");
    f.thread("openai", "openai", false);
    fs::write(
        f.ctx.home.join("auth.json"),
        r#"{"OPENAI_API_KEY":"secret_test_never_render"}"#,
    )
    .unwrap();
    let r = serde_json::to_string(&inspect::run(&f.ctx).unwrap()).unwrap();
    assert!(!r.contains("secret_test_never_render"));
    assert!(!r.contains("keep exact body"));
}
#[test]
fn gateway_contract_repairs_only_owned_provider_and_rechecks() {
    let f = Fixture::new();
    f.config("# keep\nmodel_provider='atelier_gateway'\n[model_providers.atelier_gateway]\nbase_url='http://127.0.0.1:1/wrong'\nwire_api='chat'\n[model_providers.personal]\nbase_url='https://example.test/v1'\n");
    fs::create_dir_all(f.ctx.app_root.join("gateway")).unwrap();
    fs::write(
        f.ctx.app_root.join("gateway/catalog.json"),
        r#"{"settings":{"port":18910}}"#,
    )
    .unwrap();
    let r = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&r, "router_http_contract").status, "error");
    let fixed = repair::apply(
        &f.ctx,
        repair::preview(&f.ctx, "router_http_contract").unwrap(),
    )
    .unwrap();
    assert_eq!(item(&fixed.report, "router_http_contract").status, "ok");
    let c = inspect::config(&f.ctx).unwrap();
    assert_eq!(
        c["model_providers"]["atelier_gateway"]["wire_api"].as_str(),
        Some("responses")
    );
    assert_eq!(
        c["model_providers"]["personal"]["base_url"].as_str(),
        Some("https://example.test/v1")
    );
    assert!(fs::read_to_string(f.ctx.config())
        .unwrap()
        .contains("# keep"));
}
#[test]
fn changed_gateway_settings_reject_old_port_repair() {
    let f = Fixture::new();
    f.config("model_provider='atelier_gateway'\n[model_providers.atelier_gateway]\nbase_url='http://127.0.0.1:1'\n");
    fs::create_dir_all(f.ctx.app_root.join("gateway")).unwrap();
    let catalog = f.ctx.app_root.join("gateway/catalog.json");
    fs::write(&catalog, r#"{"settings":{"port":18910}}"#).unwrap();
    let t = repair::preview(&f.ctx, "all").unwrap();
    fs::write(&catalog, r#"{"settings":{"port":18911}}"#).unwrap();
    assert!(repair::apply(&f.ctx, t)
        .unwrap_err()
        .contains("网关配置已变化"));
}
#[test]
fn repairs_missing_index_path_only_to_identity_matched_record() {
    let f = Fixture::new();
    f.config("");
    let old = f.thread("openai", "openai", false);
    let moved = f.ctx.home.join("sessions/rollout-thread-1.jsonl");
    fs::rename(old, &moved).unwrap();
    let report = inspect::run(&f.ctx).unwrap();
    assert!(item(&report, "thread_missing_rollouts").fixable);
    let r = repair::apply(
        &f.ctx,
        repair::preview(&f.ctx, "thread_missing_rollouts").unwrap(),
    )
    .unwrap();
    assert!(r.fixed_ids.contains(&"thread_missing_rollouts".into()));
    let db = inspect::open_db(&f.ctx.home.join("state_5.sqlite"), false).unwrap();
    assert_eq!(
        inspect::rows(&db).unwrap()[0].path,
        moved.canonicalize().unwrap().display().to_string()
    );
}
#[test]
fn sqlite_update_failure_restores_written_rollouts() {
    let f = Fixture::new();
    f.config("");
    let p = f.thread("atelier_old", "atelier_old", false);
    let before = fs::read(&p).unwrap();
    let db = rusqlite::Connection::open(f.ctx.home.join("state_5.sqlite")).unwrap();
    db.execute_batch("CREATE TRIGGER reject_changes BEFORE UPDATE ON threads BEGIN SELECT RAISE(ABORT,'fixture refusal'); END;").unwrap();
    drop(db);
    let t = repair::preview(&f.ctx, "all").unwrap();
    assert!(repair::apply(&f.ctx, t).unwrap_err().contains("已恢复"));
    assert_eq!(fs::read(&p).unwrap(), before);
    let db = inspect::open_db(&f.ctx.home.join("state_5.sqlite"), false).unwrap();
    assert_eq!(inspect::rows(&db).unwrap()[0].provider, "atelier_old");
}
#[test]
fn wal_snapshot_is_marked_incomplete_without_shm_creation() {
    let f = Fixture::new();
    f.config("");
    f.thread("openai", "openai", false);
    let wal = f.ctx.home.join("state_5.sqlite-wal");
    fs::write(&wal, vec![0u8; 64]).unwrap();
    let r = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&r, "thread_history_integrity").status, "unknown");
    assert!(!item(&r, "thread_provider_consistency").fixable);
    assert!(!f.ctx.home.join("state_5.sqlite-shm").exists());
}
#[test]
fn ambiguous_existing_records_never_autoselect() {
    let f = Fixture::new();
    f.config("");
    let old = f.thread("openai", "openai", false);
    fs::copy(&old, f.ctx.home.join("sessions/a-thread-1.jsonl")).unwrap();
    fs::rename(&old, f.ctx.home.join("sessions/b-thread-1.jsonl")).unwrap();
    let r = inspect::run(&f.ctx).unwrap();
    assert!(!item(&r, "thread_missing_rollouts").fixable);
}
#[test]
fn malformed_provider_definition_is_not_treated_as_mutable_table() {
    let f = Fixture::new();
    f.config("model_provider='atelier_gateway'\n[model_providers]\natelier_gateway=4\n");
    fs::create_dir_all(f.ctx.app_root.join("gateway")).unwrap();
    fs::write(
        f.ctx.app_root.join("gateway/catalog.json"),
        r#"{"settings":{"port":18910}}"#,
    )
    .unwrap();
    let r = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&r, "config_global_validity").status, "error");
    assert!(!item(&r, "router_http_contract").fixable);
    let result = repair::apply(&f.ctx, repair::preview(&f.ctx, "all").unwrap()).unwrap();
    assert!(result.fixed_ids.contains(&"config_global_validity".into()));
}
#[test]
fn all_repairs_combine_provider_and_legacy_tool_id_once() {
    let f = Fixture::new();
    f.config("");
    let path = f.thread("atelier_old", "atelier_old", false);
    use std::io::Write;
    let mut file = fs::OpenOptions::new().append(true).open(&path).unwrap();
    writeln!(file,"{}",json!({"type":"response_item","payload":{"type":"custom_tool_call","id":"fc_example","call_id":"call-untouched","input":"keep body"}})).unwrap();
    drop(file);
    assert!(item(&inspect::run(&f.ctx).unwrap(), "thread_response_item_ids").fixable);
    let r = repair::apply(&f.ctx, repair::preview(&f.ctx, "all").unwrap()).unwrap();
    assert!(r.fixed_ids.contains(&"thread_response_item_ids".into()));
    assert!(r.fixed_ids.contains(&"thread_provider_consistency".into()));
    let content = fs::read_to_string(path).unwrap();
    assert!(
        content.contains("ctc_example")
            && content.contains("call-untouched")
            && content.contains("keep body")
    );
}
#[test]
fn native_history_positions_block_provider_and_id_byte_rewrites() {
    let f = Fixture::new();
    f.config("");
    let path = f.thread("atelier_old", "atelier_old", false);
    fs::write(&path,format!("{}\n{}\n",json!({"type":"session_meta","ordinal":null,"payload":{"id":"thread-1","model_provider":"atelier_old"}}),json!({"type":"response_item","payload":{"type":"custom_tool_call","id":"ct_example"}}))).unwrap();
    let r = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&r, "thread_response_item_ids").status, "warning");
    assert!(!item(&r, "thread_response_item_ids").fixable);
    assert!(!item(&r, "thread_provider_consistency").fixable);
    assert!(repair::preview(&f.ctx, "all").is_err());
}
fn project_fixture(f: &Fixture) -> PathBuf {
    let path = f.thread("openai", "openai", false);
    let db = rusqlite::Connection::open(f.ctx.home.join("state_5.sqlite")).unwrap();
    db.execute_batch("ALTER TABLE threads ADD COLUMN cwd TEXT DEFAULT ''; ")
        .unwrap();
    drop(db);
    fs::write(&path,format!("{}\n",json!({"type":"session_meta","payload":{"id":"thread-1","model_provider":"openai","cwd":"/fixture/real-workspace"}}))).unwrap();
    let state = f.ctx.home.join(".codex-global-state.json");
    fs::write(&state, b"{\"unrelated\":42}").unwrap();
    state
}
#[test]
fn project_repair_recovers_cwd_and_navigation_from_existing_record() {
    let f = Fixture::new();
    f.config("");
    let state = project_fixture(&f);
    let r = inspect::run(&f.ctx).unwrap();
    assert!(item(&r, "project_state_integrity").fixable);
    let result = repair::apply(
        &f.ctx,
        repair::preview(&f.ctx, "project_state_integrity").unwrap(),
    )
    .unwrap();
    assert!(result.fixed_ids.contains(&"project_state_integrity".into()));
    let value: serde_json::Value = serde_json::from_slice(&fs::read(state).unwrap()).unwrap();
    assert_eq!(value["unrelated"], 42);
    assert_eq!(
        value["thread-workspace-root-hints"]["thread-1"],
        "/fixture/real-workspace"
    );
    let db = inspect::open_db(&f.ctx.home.join("state_5.sqlite"), false).unwrap();
    let cwd: String = db
        .query_row("SELECT cwd FROM threads", [], |r| r.get(0))
        .unwrap();
    assert_eq!(cwd, "/fixture/real-workspace");
}
#[test]
fn project_file_rolls_back_when_cwd_transaction_fails() {
    let f = Fixture::new();
    f.config("");
    let state = project_fixture(&f);
    let before = fs::read(&state).unwrap();
    let db = rusqlite::Connection::open(f.ctx.home.join("state_5.sqlite")).unwrap();
    db.execute_batch("CREATE TRIGGER reject_cwd BEFORE UPDATE OF cwd ON threads BEGIN SELECT RAISE(ABORT,'fixture refusal'); END;").unwrap();
    drop(db);
    assert!(repair::apply(
        &f.ctx,
        repair::preview(&f.ctx, "project_state_integrity").unwrap()
    )
    .is_err());
    assert_eq!(fs::read(state).unwrap(), before);
}
#[test]
fn provider_checks_respect_active_profile_override() {
    let f = Fixture::new();
    f.config("profile='work'\n[profiles.work]\nmodel_provider='custom'\n[model_providers.custom]\nbase_url='https://example.test/v1'\n");
    f.thread("custom", "custom", false);
    let r = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&r, "thread_provider_consistency").status, "ok");
}
#[test]
fn invalid_profile_provider_fix_keeps_top_level_provider() {
    let f = Fixture::new();
    f.config("profile='work'\nmodel_provider='openai'\n[profiles.work]\nmodel_provider='missing'\nmodel='keep-model'\n");
    let r = repair::apply(
        &f.ctx,
        repair::preview(&f.ctx, "config_global_validity").unwrap(),
    )
    .unwrap();
    assert!(r.fixed_ids.contains(&"config_global_validity".into()));
    let c = inspect::config(&f.ctx).unwrap();
    assert_eq!(c["model_provider"].as_str(), Some("openai"));
    assert_eq!(c["profiles"]["work"]["model"].as_str(), Some("keep-model"));
    assert!(c["profiles"]["work"].get("model_provider").is_none());
}
#[test]
fn legitimate_custom_provider_is_not_third_party_residue() {
    let f = Fixture::new();
    f.config("model_provider='personal'\n[model_providers.personal]\nname='cc-switch is only a label'\nbase_url='https://example.test/cc_switch/v1'\nwire_api='responses'\n");
    let r = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&r, "config_third_party").status, "ok");
    assert!(!item(&r, "config_third_party").fixable);
}
#[test]
fn explicit_structural_manager_marker_is_reported_without_deleting_provider() {
    let f = Fixture::new();
    f.config("model_provider='personal'\n[model_providers.personal]\nbase_url='https://example.test/v1'\n[provider_router]\nmode='managed'\n[deecodex]\nmanaged=true\n");
    let before = fs::read(f.ctx.config()).unwrap();
    let r = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&r, "config_third_party").status, "warning");
    assert!(item(&r, "config_third_party")
        .detail
        .contains("provider_router"));
    assert!(item(&r, "config_third_party").detail.contains("deecodex"));
    assert!(!item(&r, "config_third_party").fixable);
    assert_eq!(fs::read(f.ctx.config()).unwrap(), before);
}
