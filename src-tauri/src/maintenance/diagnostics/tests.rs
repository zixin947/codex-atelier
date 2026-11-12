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
            official_catalog: None,
            proxy_handle: None,
            working_directory: None,
            sqlite_home_env: None,
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
            json!({"type":"session_meta","payload":{"id":"thread-1","model_provider":rollout_provider,"base_instructions":{"text":"Healthy synthetic fixture instructions."}}}),
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
        db.execute_batch("CREATE TABLE threads(id TEXT PRIMARY KEY,model_provider TEXT,model TEXT,rollout_path TEXT,archived INTEGER DEFAULT 0,source TEXT DEFAULT 'cli')").unwrap();
        db.execute(
            "INSERT INTO threads(id,model_provider,model,rollout_path) VALUES ('thread-1',?1,'gpt-fixture',?2)",
            rusqlite::params![db_provider, path.to_str().unwrap()],
        )
        .unwrap();
        path
    }
}
fn preview_all_with_official_fixture(f: &Fixture) -> repair::Ticket {
    struct FakeRpc;
    impl native_orphans::CatalogRpc for FakeRpc {
        fn request(
            &mut self,
            method: &str,
            _params: serde_json::Value,
            _timeout: std::time::Duration,
        ) -> Result<serde_json::Value> {
            match method {
                "config/read" => Ok(json!({"config":{}})),
                "model/list" => Ok(
                    json!({"data":[{"id":"fixture","model":"gpt-fixture","hidden":false,"isDefault":true,"supportedReasoningEfforts":[{"reasoningEffort":"medium"}],"defaultReasoningEffort":"medium"}]}),
                ),
                _ => Err("unexpected synthetic RPC".into()),
            }
        }
    }
    let catalog = native_orphans::prepare_catalog_with_rpc(&f.ctx, &mut FakeRpc).unwrap();
    repair::preview_with_official_catalog(&f.ctx, "all", &catalog).unwrap()
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.base);
    }
}
fn item<'a>(r: &'a DiagnosticReport, id: &str) -> &'a DiagnosticItem {
    r.items.iter().find(|i| i.id == id).unwrap()
}

fn relocation_fixture(f: &Fixture) -> (PathBuf, PathBuf) {
    let active = f.ctx.home.join("sqlite/state_5.sqlite");
    fs::create_dir_all(active.parent().unwrap()).unwrap();
    let db = rusqlite::Connection::open(&active).unwrap();
    db.execute_batch(
        "CREATE TABLE threads(id TEXT,model_provider TEXT,model TEXT,rollout_path TEXT)",
    )
    .unwrap();
    drop(db);
    f.config(&format!(
        "sqlite_home={}\n",
        serde_json::to_string(&active.parent().unwrap().to_string_lossy()).unwrap()
    ));
    let old = f.ctx.home.join("state_4.sqlite");
    (old, active)
}

#[test]
fn relocation_preview_and_apply_back_up_all_old_sidecars_and_keep_active_database() {
    let f = Fixture::new();
    let (old, active) = relocation_fixture(&f);
    let old_files = atelier_maintenance_core::state_db::related_files(&old);
    for (index, path) in old_files.iter().enumerate() {
        fs::write(path, format!("synthetic old file {index}")).unwrap();
    }
    let active_before = fs::read(&active).unwrap();
    let report = inspect::run(&f.ctx).unwrap();
    assert!(item(&report, "state_db_relocation").fixable);
    let ticket = repair::preview(&f.ctx, "state_db_relocation").unwrap();
    assert!(ticket.view.requires_desktop_closed);
    assert!(old_files.iter().all(|path| path.exists()));
    let result = repair::apply(&f.ctx, ticket).unwrap();
    assert!(result.fixed_ids.contains(&"state_db_relocation".into()));
    assert!(old_files.iter().all(|path| !path.exists()));
    assert_eq!(fs::read(&active).unwrap(), active_before);
    let backup = PathBuf::from(result.backup_path.unwrap());
    let manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(backup.join("manifest.json")).unwrap()).unwrap();
    for (index, path) in old_files.iter().enumerate() {
        let entry = manifest["files"]
            .as_array()
            .unwrap()
            .iter()
            .find(|row| row["path"].as_str() == path.to_str())
            .unwrap();
        assert_eq!(entry["operation"], "delete");
        assert_eq!(
            fs::read(backup.join(entry["backup"].as_str().unwrap())).unwrap(),
            format!("synthetic old file {index}").as_bytes()
        );
    }
    assert_eq!(item(&result.report, "state_db_relocation").status, "ok");
}

#[test]
fn relocation_rejects_a_new_sidecar_after_preview_before_deleting_any_source() {
    let f = Fixture::new();
    let (old, _) = relocation_fixture(&f);
    fs::write(&old, b"old database").unwrap();
    let ticket = repair::preview(&f.ctx, "state_db_relocation").unwrap();
    let wal = atelier_maintenance_core::state_db::related_files(&old)[1].clone();
    fs::write(&wal, b"new external WAL").unwrap();
    assert!(repair::apply(&f.ctx, ticket)
        .unwrap_err()
        .contains("清理范围已被外部更新"));
    assert_eq!(fs::read(&old).unwrap(), b"old database");
    assert_eq!(fs::read(wal).unwrap(), b"new external WAL");
}

#[test]
fn relocation_does_not_delete_root_dbs_for_an_arbitrary_configured_directory() {
    let f = Fixture::new();
    let (old, _) = relocation_fixture(&f);
    fs::write(&old, b"keep").unwrap();
    let elsewhere = f.base.join("external-database");
    fs::create_dir_all(&elsewhere).unwrap();
    f.config(&format!(
        "sqlite_home={}\n",
        serde_json::to_string(&elsewhere.to_string_lossy()).unwrap()
    ));
    let report = inspect::run(&f.ctx).unwrap();
    assert!(!item(&report, "state_db_relocation").fixable);
    assert!(repair::preview(&f.ctx, "state_db_relocation").is_err());
    assert_eq!(fs::read(old).unwrap(), b"keep");
}

#[test]
fn relocation_keeps_sources_when_the_quiet_window_cannot_be_obtained() {
    let f = Fixture::new();
    let (old, _) = relocation_fixture(&f);
    fs::write(&old, b"keep").unwrap();
    let ticket = repair::preview(&f.ctx, "state_db_relocation").unwrap();
    let mut ctx = f.ctx.clone();
    ctx.desktop_running = true;
    // 无法关闭 Desktop（测试环境没有安装）时必须中止，源库保持不变。
    let err = repair::apply(&ctx, ticket).unwrap_err();
    assert!(err.contains("静默窗口") || err.contains("Desktop"), "{err}");
    assert_eq!(fs::read(old).unwrap(), b"keep");
}

#[test]
fn relocation_streams_large_database_instead_of_using_rollout_text_read_limit() {
    let f = Fixture::new();
    let (old, _) = relocation_fixture(&f);
    let length = 40 * 1024 * 1024;
    fs::File::create(&old).unwrap().set_len(length).unwrap();
    let ticket = repair::preview(&f.ctx, "state_db_relocation").unwrap();
    let result = repair::apply(&f.ctx, ticket).unwrap();
    assert!(result.fixed_ids.contains(&"state_db_relocation".into()));
    assert!(!old.exists());
    let backup = PathBuf::from(result.backup_path.unwrap());
    let manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(backup.join("manifest.json")).unwrap()).unwrap();
    let entry = manifest["files"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["path"].as_str() == old.to_str())
        .unwrap();
    let copy = backup.join(entry["backup"].as_str().unwrap());
    assert_eq!(fs::metadata(copy).unwrap().len(), length);
}

#[test]
fn relocation_backup_failure_is_a_failed_result_with_original_database_preserved() {
    let f = Fixture::new();
    let (old, _) = relocation_fixture(&f);
    fs::write(&old, b"original").unwrap();
    let ticket = repair::preview(&f.ctx, "state_db_relocation").unwrap();
    let backup = f
        .ctx
        .app_root
        .join("maintenance/diagnostic-backups")
        .join(&ticket.view.ticket);
    fs::create_dir_all(&backup).unwrap();
    fs::write(
        backup.join("state-db-backups"),
        b"occupied by synthetic file",
    )
    .unwrap();
    let result = repair::apply(&f.ctx, ticket).unwrap();
    assert!(result.fixed_ids.is_empty());
    assert_eq!(result.failed_ids, vec!["state_db_relocation"]);
    assert_eq!(item(&result.report, "state_db_relocation").status, "error");
    assert!(result
        .details
        .iter()
        .any(|detail| detail.contains("无法创建备份目录")));
    assert_eq!(fs::read(old).unwrap(), b"original");
}
#[test]
fn exact_34_unique_checks_without_codex_writes() {
    let f = Fixture::new();
    f.config("# preserve\nmodel = 'gpt-fixture'\n");
    let before = fs::read(f.ctx.config()).unwrap();
    let r = inspect::run(&f.ctx).unwrap();
    assert_eq!(r.items.len(), 34);
    assert_eq!(
        r.items
            .iter()
            .map(|i| &i.id)
            .collect::<std::collections::HashSet<_>>()
            .len(),
        34
    );
    assert_eq!(before, fs::read(f.ctx.config()).unwrap());
    assert!(!f.ctx.home.join(".aimami_probe").exists());
    assert_eq!(item(&r, "auth_integrity").status, "warning");
    assert_eq!(item(&r, "router_postcondition").status, "ok");
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
fn legacy_top_level_profile_is_reported_and_nested_profile_is_preserved() {
    let f = Fixture::new();
    f.config("profile = 'work'\n[profiles.work]\nmodel = 'gpt-fixture'\n");
    let r = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&r, "config_profile_conflict").status, "warning");
    assert!(item(&r, "config_profile_conflict").label.contains("legacy"));
    let ticket = repair::preview(&f.ctx, "config_profile_conflict").unwrap();
    // Preview is read-only; confirmation removes only the unsupported selector.
    assert!(fs::read_to_string(f.ctx.config())
        .unwrap()
        .contains("profile = 'work'"));
    repair::apply(&f.ctx, ticket).unwrap();
    let config = inspect::config(&f.ctx).unwrap();
    assert!(config.get("profile").is_none());
    assert_eq!(
        config["profiles"]["work"]["model"].as_str(),
        Some("gpt-fixture")
    );
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
fn running_desktop_is_closed_automatically_before_write() {
    // 对齐 AiMaMi：Desktop 运行时不再拒绝，而是自动关闭 → 修复 → 自动重启。
    let f = Fixture::new();
    f.config("profile = 'missing'\n");
    let t = repair::preview(&f.ctx, "all").unwrap();
    let mut ctx = f.ctx.clone();
    ctx.desktop_running = true;
    // 测试环境没有真实 Desktop 安装，进入静默窗口会明确失败而不是静默写入。
    let err = repair::apply(&ctx, t).unwrap_err();
    assert!(err.contains("静默窗口") || err.contains("Desktop"), "{err}");
    // 配置在未取得静默窗口时不得被修改。
    assert!(fs::read_to_string(f.ctx.config())
        .unwrap()
        .contains("profile"));
}
#[test]
fn provider_repair_updates_db_and_plain_rollout() {
    let f = Fixture::new();
    f.config("");
    let path = f.thread("aimami_relay_fixture", "aimami_relay_fixture", false);
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
    let path = f.thread("aimami_relay_fixture", "aimami_relay_fixture", true);
    let r = repair::apply(&f.ctx, preview_all_with_official_fixture(&f)).unwrap();
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
    let path = f.thread("aimami_relay_fixture", "aimami_relay_fixture", false);
    let t = preview_all_with_official_fixture(&f);
    fs::write(&path, b"external data").unwrap();
    assert!(repair::apply(&f.ctx, t).is_err());
    let db = inspect::open_db(&f.ctx.home.join("state_5.sqlite"), false).unwrap();
    assert_eq!(
        inspect::rows(&db).unwrap()[0].provider,
        "aimami_relay_fixture"
    );
    assert_eq!(fs::read(&path).unwrap(), b"external data");
}
#[test]
fn invalid_toml_is_error_and_rescue_fixable() {
    // AiMaMi 1.2.7 decomp-check_config_toml_syntax.c：语法错误项 fixable=true（抢救可修复）
    let f = Fixture::new();
    f.config("[bad\n");
    let r = inspect::run(&f.ctx).unwrap();
    let it = item(&r, "config_toml_syntax");
    assert_eq!(it.status, "error");
    assert_eq!(it.label, "config.toml TOML 语法错误");
    assert!(it.fixable);
    assert_eq!(item(&r, "config_global_validity").status, "warning");
}

#[test]
fn missing_config_uses_skip_label_and_blank_uses_empty_label() {
    // decomp-check_config_toml_syntax.c：不存在语境与空文件 label 区分
    let f = Fixture::new();
    if f.ctx.config().exists() {
        fs::remove_file(f.ctx.config()).unwrap();
    }
    let r = inspect::run(&f.ctx).unwrap();
    assert_eq!(
        item(&r, "config_toml_syntax").label,
        "TOML 语法（文件不存在，跳过）"
    );

    let f2 = Fixture::new();
    f2.config("   \n\t ");
    let r2 = inspect::run(&f2.ctx).unwrap();
    assert_eq!(
        item(&r2, "config_toml_syntax").label,
        "TOML 语法正常（空文件）"
    );
    assert!(!item(&r2, "config_toml_syntax").fixable);
}

#[test]
fn valid_config_reports_ok_label() {
    let f = Fixture::new();
    f.config("model = \"gpt\"\n");
    let r = inspect::run(&f.ctx).unwrap();
    let it = item(&r, "config_toml_syntax");
    assert_eq!(it.status, "ok");
    assert_eq!(it.label, "TOML 语法正常");
    assert!(!it.fixable);
}
#[test]
fn missing_records_not_fabricated() {
    let f = Fixture::new();
    f.config("");
    let path = f.thread("openai", "openai", false);
    fs::remove_file(&path).unwrap();
    let r = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&r, "thread_missing_rollouts").status, "error");
    assert!(item(&r, "thread_missing_rollouts").fixable);
    let result = repair::apply(
        &f.ctx,
        repair::preview(&f.ctx, "thread_missing_rollouts").unwrap(),
    )
    .unwrap();
    assert!(result.fixed_ids.contains(&"thread_missing_rollouts".into()));
    assert!(!path.exists());
    let db = rusqlite::Connection::open(f.ctx.home.join("state_5.sqlite")).unwrap();
    let (raw, archived): (String, i64) = db
        .query_row("SELECT rollout_path,archived FROM threads", [], |r| {
            Ok((r.get(0)?, r.get(1)?))
        })
        .unwrap();
    assert_eq!(raw, path.to_string_lossy());
    assert_eq!(archived, 1);
    assert!(PathBuf::from(result.backup_path.unwrap())
        .join("threads")
        .is_dir());
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
fn gateway_catalog_bytes(port: u16) -> Vec<u8> {
    serde_json::to_vec(&crate::gateway::types::Catalog {
        revision: "synthetic-revision".into(),
        host_id: "synthetic-host".into(),
        profiles: vec![],
        settings: crate::gateway::types::Settings {
            port,
            ..Default::default()
        },
    })
    .unwrap()
}
#[test]
fn gateway_contract_repairs_only_owned_provider_and_rechecks() {
    let f = Fixture::new();
    f.config("# keep\nmodel_provider='atelier_gateway'\n[model_providers.atelier_gateway]\nbase_url='http://127.0.0.1:1/wrong'\nwire_api='chat'\n[model_providers.personal]\nbase_url='https://example.test/v1'\n");
    fs::create_dir_all(f.ctx.app_root.join("gateway")).unwrap();
    fs::write(
        f.ctx.app_root.join("gateway/catalog.json"),
        gateway_catalog_bytes(18910),
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
    fs::write(&catalog, gateway_catalog_bytes(18910)).unwrap();
    let t = repair::preview(&f.ctx, "all").unwrap();
    fs::write(&catalog, gateway_catalog_bytes(18911)).unwrap();
    assert!(repair::apply(&f.ctx, t)
        .unwrap_err()
        .contains("网关配置已变化"));
}
#[test]
fn restores_missing_history_from_identity_matched_native_backup() {
    let f = Fixture::new();
    f.config("");
    let old = f.thread("openai", "openai", false);
    let moved = f.ctx.home.join("codexmate/backups/native/one.jsonl");
    fs::create_dir_all(moved.parent().unwrap()).unwrap();
    fs::rename(&old, &moved).unwrap();
    let original = fs::read(&moved).unwrap();
    let report = inspect::run(&f.ctx).unwrap();
    assert!(item(&report, "thread_missing_rollouts").fixable);
    let r = repair::apply(
        &f.ctx,
        repair::preview(&f.ctx, "thread_missing_rollouts").unwrap(),
    )
    .unwrap();
    assert!(r.fixed_ids.contains(&"thread_missing_rollouts".into()));
    assert_eq!(fs::read(&old).unwrap(), original);
    assert_eq!(fs::read(&moved).unwrap(), original);
    let db = inspect::open_db(&f.ctx.home.join("state_5.sqlite"), false).unwrap();
    assert_eq!(inspect::rows(&db).unwrap()[0].path, old.to_string_lossy());
    assert!(PathBuf::from(r.backup_path.unwrap())
        .join("threads/thread-rollout-repairs")
        .is_dir());
}
#[test]
fn sqlite_update_failure_restores_written_rollouts() {
    let f = Fixture::new();
    f.config("");
    let p = f.thread("aimami_relay_fixture", "aimami_relay_fixture", false);
    let before = fs::read(&p).unwrap();
    let db = rusqlite::Connection::open(f.ctx.home.join("state_5.sqlite")).unwrap();
    db.execute_batch("CREATE TRIGGER reject_changes BEFORE UPDATE ON threads BEGIN SELECT RAISE(ABORT,'fixture refusal'); END;").unwrap();
    drop(db);
    let t = preview_all_with_official_fixture(&f);
    let result = repair::apply(&f.ctx, t).unwrap();
    assert!(result
        .failed_ids
        .contains(&"thread_provider_consistency".into()));
    assert!(PathBuf::from(result.backup_path.unwrap())
        .join("threads")
        .is_dir());
    assert_eq!(fs::read(&p).unwrap(), before);
    let db = inspect::open_db(&f.ctx.home.join("state_5.sqlite"), false).unwrap();
    assert_eq!(
        inspect::rows(&db).unwrap()[0].provider,
        "aimami_relay_fixture"
    );
}
#[test]
fn wal_snapshot_is_marked_incomplete_without_shm_creation() {
    let f = Fixture::new();
    f.config("");
    f.thread("openai", "openai", false);
    let wal = f.ctx.home.join("state_5.sqlite-wal");
    fs::write(&wal, vec![0u8; 64]).unwrap();
    let r = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&r, "thread_history_integrity").status, "warning");
    assert!(item(&r, "thread_history_integrity")
        .detail
        .contains("覆盖度未知"));
    assert!(!item(&r, "thread_provider_consistency").fixable);
    assert!(!f.ctx.home.join("state_5.sqlite-shm").exists());
}
#[test]
fn untrusted_session_copies_do_not_relink_a_missing_index() {
    let f = Fixture::new();
    f.config("");
    let old = f.thread("openai", "openai", false);
    let a = f.ctx.home.join("sessions/a-thread-1.jsonl");
    let b = f.ctx.home.join("sessions/b-thread-1.jsonl");
    fs::copy(&old, &a).unwrap();
    fs::rename(&old, &b).unwrap();
    let bytes = fs::read(&a).unwrap();
    let report = inspect::run(&f.ctx).unwrap();
    assert!(item(&report, "thread_missing_rollouts").fixable);
    let result = repair::apply(
        &f.ctx,
        repair::preview(&f.ctx, "thread_missing_rollouts").unwrap(),
    )
    .unwrap();
    assert!(result.fixed_ids.contains(&"thread_missing_rollouts".into()));
    let c = rusqlite::Connection::open(f.ctx.home.join("state_5.sqlite")).unwrap();
    let (raw, archived): (String, i64) = c
        .query_row("SELECT rollout_path,archived FROM threads", [], |r| {
            Ok((r.get(0)?, r.get(1)?))
        })
        .unwrap();
    assert_eq!(raw, old.to_string_lossy());
    assert_eq!(archived, 1);
    assert_eq!(fs::read(&a).unwrap(), bytes);
    assert_eq!(fs::read(&b).unwrap(), bytes);
    assert!(!old.exists());
}

#[test]
fn malformed_provider_definition_is_not_treated_as_mutable_table() {
    let f = Fixture::new();
    f.config("model_provider='atelier_gateway'\n[model_providers]\natelier_gateway=4\n");
    fs::create_dir_all(f.ctx.app_root.join("gateway")).unwrap();
    fs::write(
        f.ctx.app_root.join("gateway/catalog.json"),
        gateway_catalog_bytes(18910),
    )
    .unwrap();
    let r = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&r, "config_global_validity").status, "ok");
    assert_eq!(item(&r, "router_http_contract").status, "error");
    assert!(!item(&r, "router_http_contract").fixable);
    let before = fs::read(f.ctx.config()).unwrap();
    assert!(repair::preview(&f.ctx, "config_global_validity").is_err());
    assert_eq!(fs::read(f.ctx.config()).unwrap(), before);
}
#[test]
fn all_repairs_combine_provider_and_legacy_tool_id_once() {
    let f = Fixture::new();
    f.config("");
    let path = f.thread("aimami_relay_fixture", "aimami_relay_fixture", false);
    use std::io::Write;
    let mut file = fs::OpenOptions::new().append(true).open(&path).unwrap();
    writeln!(file,"{}",json!({"type":"response_item","payload":{"type":"custom_tool_call","id":"fc_example","call_id":"call-untouched","input":"keep body"}})).unwrap();
    drop(file);
    assert!(item(&inspect::run(&f.ctx).unwrap(), "thread_response_item_ids").fixable);
    let r = repair::apply(&f.ctx, preview_all_with_official_fixture(&f)).unwrap();
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
fn native_history_positions_allow_provider_shrink_but_block_id_growth() {
    let f = Fixture::new();
    f.config("");
    let path = f.thread("aimami_relay_fixture", "aimami_relay_fixture", false);
    let header = json!({"type":"session_meta","ordinal":null,"payload":{"id":"thread-1","model_provider":"aimami_relay_fixture","base_instructions":{"text":"Healthy fixture"}}});
    let response =
        json!({"type":"response_item","payload":{"type":"custom_tool_call","id":"ct_example"}});
    let bytes = format!("{header}\n{response}\n").into_bytes();
    fs::write(&path, &bytes).unwrap();
    let report = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&report, "thread_response_item_ids").status, "error");
    assert!(!item(&report, "thread_response_item_ids").fixable);
    assert!(item(&report, "thread_provider_consistency").fixable);
    let result = repair::apply(
        &f.ctx,
        repair::preview(&f.ctx, "thread_provider_consistency").unwrap(),
    )
    .unwrap();
    assert!(result
        .fixed_ids
        .contains(&"thread_provider_consistency".into()));
    let after = fs::read(&path).unwrap();
    assert_eq!(after.len(), bytes.len());
    assert_eq!(
        after.split(|b| *b == b'\n').nth(1),
        bytes.split(|b| *b == b'\n').nth(1)
    );
    assert_eq!(inspect::meta(&after).unwrap()["model_provider"], "openai");
}

fn project_fixture(f: &Fixture) -> PathBuf {
    let path = f.thread("openai", "openai", false);
    let db = rusqlite::Connection::open(f.ctx.home.join("state_5.sqlite")).unwrap();
    db.execute_batch("ALTER TABLE threads ADD COLUMN cwd TEXT DEFAULT ''; ALTER TABLE threads ADD COLUMN updated_at_ms INTEGER DEFAULT 0;")
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
    // A selected custom profile does not make native managed history adopt that external owner.
    let native = Fixture::new();
    native.config("profile='work'\n[profiles.work]\nmodel_provider='custom'\n[model_providers.custom]\nbase_url='https://example.test/v1'\n");
    let path = native.thread("aimami_relay_fixture", "aimami_relay_fixture", false);
    let fixed = repair::apply(
        &native.ctx,
        repair::preview(&native.ctx, "thread_provider_consistency").unwrap(),
    )
    .unwrap();
    assert!(fixed
        .fixed_ids
        .contains(&"thread_provider_consistency".into()));
    assert_eq!(
        inspect::meta(&fs::read(path).unwrap()).unwrap()["model_provider"],
        "openai"
    );
    assert_eq!(
        inspect::rows(&inspect::open_db(&native.ctx.home.join("state_5.sqlite"), false).unwrap())
            .unwrap()[0]
            .provider,
        "openai"
    );
}
#[test]
fn native_global_check_does_not_delete_provider_choices_by_whitelist() {
    let f = Fixture::new();
    f.config("profile='work'\nmodel_provider='openai'\n[profiles.work]\nmodel_provider='missing'\nmodel='keep-model'\n");
    let before = fs::read(f.ctx.config()).unwrap();
    let report = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&report, "config_global_validity").status, "ok");
    assert!(repair::preview(&f.ctx, "config_global_validity").is_err());
    assert_eq!(fs::read(f.ctx.config()).unwrap(), before);
}
#[test]
fn native_global_fix_removes_only_confirmed_bad_references_and_rechecks() {
    let f = Fixture::new();
    let outside = f.ctx.app_root.join("missing-catalog.json");
    let input = format!("# keep\nmodel_provider='unrecognized'\nmodel='keep-model'\nmodel_reasoning_effort=''\nmodel_catalog_json={}\n", serde_json::to_string(&outside.to_string_lossy()).unwrap());
    f.config(&input);
    let report = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&report, "config_global_validity").status, "error");
    let ticket = repair::preview(&f.ctx, "config_global_validity").unwrap();
    assert_eq!(fs::read_to_string(f.ctx.config()).unwrap(), input);
    let result = repair::apply(&f.ctx, ticket).unwrap();
    assert!(result.fixed_ids.contains(&"config_global_validity".into()));
    let config = inspect::config(&f.ctx).unwrap();
    assert_eq!(config["model_provider"].as_str(), Some("unrecognized"));
    assert_eq!(config["model"].as_str(), Some("keep-model"));
    assert!(config.get("model_reasoning_effort").is_none());
    assert!(config.get("model_catalog_json").is_none());
    assert!(fs::read_to_string(f.ctx.config())
        .unwrap()
        .contains("# keep"));
    assert_eq!(item(&result.report, "config_global_validity").status, "ok");
}
#[test]
fn native_third_party_fingerprint_in_custom_provider_label_is_reported() {
    let f = Fixture::new();
    f.config("model_provider='personal'\n[model_providers.personal]\nname='cc-switch is only a label'\nbase_url='https://example.test/cc_switch/v1'\nwire_api='responses'\n");
    let r = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&r, "config_third_party").status, "error");
    assert!(item(&r, "config_third_party").fixable);
    assert!(item(&r, "config_third_party")
        .detail
        .contains("cc-switch 指纹"));
}
#[test]
fn explicit_structural_manager_marker_is_reported_without_deleting_provider() {
    let f = Fixture::new();
    f.config("model_provider='personal'\n[model_providers.personal]\nbase_url='https://example.test/v1'\n[provider_router]\nmode='managed'\n[deecodex]\nmanaged=true\n");
    let before = fs::read(f.ctx.config()).unwrap();
    let r = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&r, "config_third_party").status, "error");
    assert!(item(&r, "config_third_party")
        .detail
        .contains("cc-switch 指纹"));
    assert!(item(&r, "config_third_party").fixable);
    assert_eq!(fs::read(f.ctx.config()).unwrap(), before);
}

#[test]
fn non_file_auth_does_not_diagnose_stale_auth_file_as_active_login() {
    for mode in ["keyring", "ephemeral"] {
        let f = Fixture::new();
        f.config(&format!("cli_auth_credentials_store='{mode}'\n"));
        fs::write(f.ctx.home.join("auth.json"), b"stale-invalid-auth").unwrap();
        let report = inspect::run(&f.ctx).unwrap();
        assert_eq!(item(&report, "auth_integrity").status, "ok");
        // The credential observer explicitly reports an unauthorized source as unknown,
        // rather than inferring token health from the stale file.
        assert_eq!(item(&report, "auth_token_expiry").status, "warning");
        assert!(item(&report, "auth_token_expiry").detail.contains("未授权"));
        assert_eq!(
            fs::read(f.ctx.home.join("auth.json")).unwrap(),
            b"stale-invalid-auth"
        );
    }
}

#[test]
fn instruction_check_matches_installed_approval_judge_prefix() {
    let f = Fixture::new();
    f.config("model_provider='openai'\n");
    let path = f.thread("openai", "openai", false);
    for (text, expected) in [
        ("You are a coding assistant.", "ok"),
        (
            "  You are judging one planned coding-agent action. Return a verdict.",
            "error",
        ),
        ("   ", "error"),
    ] {
        let meta = json!({"type":"session_meta","payload":{"id":"thread-1","model_provider":"openai","base_instructions":{"text":text}}});
        fs::write(&path, format!("{meta}\n")).unwrap();
        let report = inspect::run(&f.ctx).unwrap();
        assert_eq!(
            item(&report, "thread_poisoned_instructions").status,
            expected
        );
    }
}

#[test]
fn syntax_rescue_preview_apply_roundtrip_with_isolation() {
    // 集成：坏 TOML → preview（无副作用）→ apply（备份+原子写）→ 复检 ok
    let f = Fixture::new();
    let original = "model = \"gpt\"\n]]] bad [[[\nkeep_me = true\n";
    f.config(original);

    // 预览前快照：原文件内容 + 备份目录（应不存在）
    let before_bytes = fs::read(f.ctx.config()).unwrap();
    let backup_dir = f.ctx.app_root.join("maintenance/diagnostic-backups");
    assert!(
        fs::read_dir(&backup_dir).map(|d| d.count()).unwrap_or(0) == 0,
        "预览不得创建备份内容"
    );

    // preview：纯计划
    let t = repair::preview(&f.ctx, "config_toml_syntax").unwrap();
    assert_eq!(t.view.item_ids, vec!["config_toml_syntax"]);
    assert_eq!(t.files.len(), 1);
    assert_eq!(
        t.files[0].before.as_deref(),
        Some(before_bytes.as_slice()),
        "before 必须对应原始字节"
    );
    let after_text = String::from_utf8(t.files[0].after.clone().unwrap()).unwrap();
    assert!(after_text.starts_with(super::config_syntax::RESCUE_HEADER));
    assert!(after_text.contains("keep_me = true"));

    // 预览不得改原文件
    assert_eq!(fs::read(f.ctx.config()).unwrap(), before_bytes);
    assert!(!backup_dir.exists(), "预览后仍不得有备份目录");

    // apply：锁 + 外部校验 + 逐字备份 + 原子写。
    // 注：抢救保留全部非空行（含坏行）→ 复检仍 error → failed_ids 如实反映（反编译语义）。
    let result = repair::apply(&f.ctx, t).unwrap();
    assert!(result.fixed_ids.is_empty());
    assert_eq!(result.failed_ids, vec!["config_toml_syntax"]);

    // 原文件已被替换为抢救文本
    let now = fs::read_to_string(f.ctx.config()).unwrap();
    assert!(now.starts_with(super::config_syntax::RESCUE_HEADER));
    assert!(now.contains("model = \"gpt\""));

    // 逐字备份存在
    assert!(backup_dir.exists(), "apply 应创建备份目录");
    let returned_backup = PathBuf::from(result.backup_path.as_deref().expect("应返回备份路径"));
    assert!(returned_backup.starts_with(&backup_dir));
    let backup_bytes = fs::read(returned_backup.join("0000.bin")).unwrap();
    assert_eq!(backup_bytes, before_bytes, "备份必须与原件逐字相同");

    // 复检：抢救文本仍含坏行 → error（不虚报）
    let r2 = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&r2, "config_toml_syntax").status, "error");
}

#[test]
fn syntax_rescue_strip_path_reports_ok_after_recheck() {
    // strip 路径（管理块损坏，外部内容合法）→ 复检 ok
    let f = Fixture::new();
    f.config("model = \"gpt\"\n# >>> aimami-relay managed start (DO NOT EDIT MANUALLY)\nbroken = [\n# <<< aimami-relay managed end\n");
    let t = repair::preview(&f.ctx, "config_toml_syntax").unwrap();
    let result = repair::apply(&f.ctx, t).unwrap();
    assert_eq!(result.fixed_ids, vec!["config_toml_syntax"]);
    let r = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&r, "config_toml_syntax").status, "ok");
}

#[test]
fn syntax_rescue_rejects_external_changes_between_preview_and_apply() {
    let f = Fixture::new();
    f.config("]]] bad [[[\n");
    let t = repair::preview(&f.ctx, "config_toml_syntax").unwrap();
    // 预览后外部改动
    fs::write(f.ctx.config(), b"externally changed").unwrap();
    let err = repair::apply(&f.ctx, t).unwrap_err();
    assert!(err.contains("外部更新"), "got: {err}");
    assert_eq!(fs::read(f.ctx.config()).unwrap(), b"externally changed");
}

#[test]
fn syntax_rescue_via_all_entry_uses_salvage_and_reports_honestly() {
    // "all" 入口：坏 TOML 无管理块 → salvage 路径保留坏行 → 复检仍 error（如实）
    let f = Fixture::new();
    f.config("]]] bad [[[\n");
    let t = repair::preview(&f.ctx, "all").unwrap();
    assert!(t.view.item_ids.contains(&"config_toml_syntax".to_string()));
    repair::apply(&f.ctx, t).unwrap();
    let now = fs::read_to_string(f.ctx.config()).unwrap();
    assert!(now.starts_with(super::config_syntax::RESCUE_HEADER));
    assert!(now.contains("]]] bad [[["), "抢救保留全部非空行");
    let r = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&r, "config_toml_syntax").status, "error");
}

#[test]
fn syntax_rescue_via_all_entry_strip_path_rechecks_ok() {
    // "all" 入口：管理块损坏 → strip 路径 → 复检 ok
    let f = Fixture::new();
    f.config("model = \"gpt\"\n# >>> aimami-relay managed start (DO NOT EDIT MANUALLY)\nbroken = [\n# <<< aimami-relay managed end\n");
    let t = repair::preview(&f.ctx, "all").unwrap();
    assert!(t.view.item_ids.contains(&"config_toml_syntax".to_string()));
    repair::apply(&f.ctx, t).unwrap();
    let r = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&r, "config_toml_syntax").status, "ok");
}

#[test]
fn syntax_rescue_still_abnormal_is_reported_honestly() {
    // 抢救保留的行本身仍非法（未闭合字符串）→ 复检 error，不虚报修复成功
    let f = Fixture::new();
    f.config("broken = \"never-closed\n");
    let t = repair::preview(&f.ctx, "config_toml_syntax").unwrap();
    repair::apply(&f.ctx, t).unwrap();
    let r = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&r, "config_toml_syntax").status, "error");
    assert_eq!(
        item(&r, "config_toml_syntax").label,
        "config.toml TOML 语法错误"
    );
}

#[test]
fn poisoned_instructions_repair_restores_from_healthy_sibling() {
    // 集成：健康线程提供模板，污染线程被恢复（preview→apply→复检）
    let f = Fixture::new();
    f.config("model_provider='openai'\n");
    // 两条线程：不同 rollout 文件 + 同一 DB 表两行
    let healthy_path = f.ctx.home.join("sessions/healthy.jsonl");
    let healthy_meta = json!({"type":"session_meta","payload":{"id":"t-h","model_provider":"openai","base_instructions":{"text":"You are a helpful coding assistant. ".repeat(40)}}});
    fs::write(&healthy_path, format!("{healthy_meta}\n")).unwrap();

    let bad_path = f.ctx.home.join("sessions/poisoned.jsonl");
    let bad_meta = r#"{"type":"session_meta","payload":{"id":"t-b","model_provider":"openai","base_instructions":{"text":"You are judging one planned coding-agent action. Verdict."}}}"#;
    fs::write(&bad_path, format!("{bad_meta}\n")).unwrap();

    {
        let db = rusqlite::Connection::open(f.ctx.home.join("state_5.sqlite")).unwrap();
        db.execute_batch(
            "CREATE TABLE threads(id TEXT PRIMARY KEY,model_provider TEXT,model TEXT,rollout_path TEXT,archived INTEGER DEFAULT 0,source TEXT DEFAULT 'cli');
             INSERT INTO threads(id,model_provider,rollout_path) VALUES('t-h','openai','sessions/healthy.jsonl');
             INSERT INTO threads(id,model_provider,rollout_path) VALUES('t-b','openai','sessions/poisoned.jsonl');",
        )
        .unwrap();
    }

    let t = repair::preview(&f.ctx, "thread_poisoned_instructions").unwrap();
    assert!(t
        .view
        .item_ids
        .contains(&"thread_poisoned_instructions".to_string()));
    let result = repair::apply(&f.ctx, t).unwrap();
    assert_eq!(result.fixed_ids, vec!["thread_poisoned_instructions"]);

    // 污染线程的 session_meta 已恢复为健康模板
    let after = fs::read_to_string(&bad_path).unwrap();
    assert!(after.contains("You are a helpful coding assistant."));
    assert!(!after.contains("You are judging"));
    // 健康线程未被改动
    let healthy_after = fs::read_to_string(&healthy_path).unwrap();
    assert!(healthy_after.contains("You are a helpful coding assistant."));

    // 复检
    let r2 = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&r2, "thread_poisoned_instructions").status, "ok");
}

#[test]
fn poisoned_instructions_missing_falls_back_to_minimal_prompt() {
    let f = Fixture::new();
    f.config("model_provider='openai'\n");
    // 唯一线程：instructions 缺失（无健康模板可用）
    let path = f.thread("openai", "openai", false);
    let meta = r#"{"type":"session_meta","payload":{"id":"thread-1","model_provider":"openai"}}"#;
    fs::write(&path, format!("{meta}\n")).unwrap();

    let t = repair::preview(&f.ctx, "thread_poisoned_instructions").unwrap();
    repair::apply(&f.ctx, t).unwrap();

    let after = fs::read_to_string(&path).unwrap();
    assert!(after.contains(
        "Follow the user's instructions carefully. Respond helpfully and use tools when appropriate."
    ));
    let r2 = inspect::run(&f.ctx).unwrap();
    assert_eq!(item(&r2, "thread_poisoned_instructions").status, "ok");
}

#[test]
fn transition_started_after_preview_rejects_repair_before_any_write() {
    let f = Fixture::new();
    let original = "profile='missing'\nmodel='fixture'\n";
    f.config(original);
    let ticket = repair::preview(&f.ctx, "config_profile_conflict").unwrap();
    fs::create_dir_all(f.ctx.app_root.join("gateway")).unwrap();
    fs::write(f.ctx.app_root.join("gateway/transition.json"), b"{}").unwrap();
    assert!(inspect::run(&f.ctx).unwrap().transition_in_flight);
    assert!(repair::apply(&f.ctx, ticket).unwrap_err().contains("转换"));
    assert_eq!(fs::read_to_string(f.ctx.config()).unwrap(), original);
    assert!(!f.ctx.app_root.join("maintenance").exists());
}

#[test]
fn runtime_starting_or_stopping_blocks_old_repair_preview() {
    for state in ["starting", "stopping"] {
        let mut f = Fixture::new();
        let original = "profile='missing'\nmodel='fixture'\n";
        f.config(original);
        let ticket = repair::preview(&f.ctx, "config_profile_conflict").unwrap();
        f.ctx.runtime = Some(crate::gateway::types::RuntimeView {
            state: state.into(),
            ..Default::default()
        });
        assert!(inspect::run(&f.ctx).unwrap().transition_in_flight);
        assert!(repair::apply(&f.ctx, ticket).unwrap_err().contains("转换"));
        assert_eq!(fs::read_to_string(f.ctx.config()).unwrap(), original);
        assert!(!f.ctx.app_root.join("maintenance").exists());
    }
}

#[cfg(unix)]
#[test]
fn relocation_follows_native_source_symlink_copy_but_only_unlinks_old_entry() {
    use std::os::unix::fs::symlink;
    let f = Fixture::new();
    let (old, _) = relocation_fixture(&f);
    let target = f.base.join("outside-old-entry.fixture");
    fs::write(&target, b"synthetic symlink target").unwrap();
    symlink(&target, &old).unwrap();
    let ticket = repair::preview(&f.ctx, "state_db_relocation").unwrap();
    let result = repair::apply(&f.ctx, ticket).unwrap();
    assert!(result
        .fixed_ids
        .iter()
        .any(|id| id == "state_db_relocation"));
    assert!(fs::symlink_metadata(&old).is_err());
    assert_eq!(fs::read(&target).unwrap(), b"synthetic symlink target");
    let root = PathBuf::from(result.backup_path.unwrap());
    let manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(root.join("manifest.json")).unwrap()).unwrap();
    let row = manifest["files"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["path"].as_str() == old.to_str())
        .unwrap();
    assert_eq!(
        fs::read(root.join(row["backup"].as_str().unwrap())).unwrap(),
        b"synthetic symlink target"
    );
}
#[test]
fn configured_database_directory_uses_native_config_env_and_cwd_precedence() {
    let mut f = Fixture::new();
    let cwd = f.base.join("synthetic-working-directory");
    f.ctx.working_directory = Some(cwd.clone());
    f.ctx.sqlite_home_env = Some("environment-db".into());
    let config: toml::Value = toml::from_str("sqlite_home='  configured-db  '").unwrap();
    assert_eq!(inspect::db_dir(&f.ctx, &config), cwd.join("configured-db"));
    let blank: toml::Value = toml::from_str("sqlite_home='  '").unwrap();
    assert_eq!(inspect::db_dir(&f.ctx, &blank), cwd.join("environment-db"));
    let literal: toml::Value = toml::from_str("sqlite_home='~/literal-db'").unwrap();
    assert_eq!(inspect::db_dir(&f.ctx, &literal), cwd.join("~/literal-db"));
    f.ctx.sqlite_home_env = None;
    assert_eq!(inspect::db_dir(&f.ctx, &blank), f.ctx.home);
}

/// 分组必须等于原版前端 qe 常量表：未具名的 6 项回退 runtime。
/// 分布应为 config 12 / auth 3 / runtime 18 / env 1，而不是界面上看似整齐的四等分。
#[test]
fn groups_match_the_original_frontend_table() {
    use std::collections::HashMap;
    let f = Fixture::new();
    f.config("model = 'gpt-fixture'\n");
    let r = inspect::run(&f.ctx).unwrap();
    let mut want: HashMap<&str, &str> = HashMap::new();
    for id in [
        "config_toml_syntax", "config_third_party", "config_profile_conflict", "config_stale",
        "router_http_contract", "takeover_backup_orphan", "catalog_path_validity",
        "catalog_integrity", "official_catalog_freshness", "config_global_validity",
        "config_access", "config_replace_readiness",
    ] { want.insert(id, "config"); }
    for id in [
        "auth_integrity", "auth_token_expiry", "router_unlock_auth_residue",
    ] { want.insert(id, "auth"); }
    for id in [
        "proxy_running", "thread_missing_rollouts", "thread_history_integrity",
        "thread_provider_consistency", "thread_poisoned_instructions", "thread_response_item_ids",
        "db_orphan_providers", "rollout_orphan_providers", "residual_manifest", "residual_cache",
        "codex_process_state", "state_db_relocation",
        // 未在原版前端分组表中具名的 6 项，原版回退 runtime。
        "router_postcondition", "thread_model_catalog_consistency", "legacy_migrated_threads",
        "project_state_integrity", "router_recovery_record",
        "native_off_invalid_openai_thread_models",
    ] { want.insert(id, "runtime"); }
    want.insert("codex_home_writable", "env");
    assert_eq!(want.len(), 34);
    for item in &r.items {
        assert_eq!(
            item.group,
            *want.get(item.id.as_str()).unwrap_or_else(|| panic!("未知条目 {}", item.id)),
            "{} 的分组与原版不同",
            item.id
        );
    }
    let count = |g: &str| r.items.iter().filter(|i| i.group == g).count();
    assert_eq!(
        (count("config"), count("auth"), count("runtime"), count("env")),
        (12, 3, 18, 1)
    );
}

/// 34 项诊断标签保持既有含义，第三方品牌使用中性展示名称。
#[test]
fn every_label_matches_the_public_display_text() {
    use std::collections::HashMap;
    let f = Fixture::new();
    f.config("# preserve\nmodel = 'gpt-fixture'\n");
    let r = inspect::run(&f.ctx).unwrap();
    let expected: HashMap<&str, &str> = HashMap::from([
        ("config_access", "config.toml 可正常读取"),
        ("config_replace_readiness", "config.toml 权限未显示只读"),
        ("config_toml_syntax", "TOML 语法正常"),
        ("config_third_party", "无第三方注入残留"),
        ("config_profile_conflict", "无 legacy 顶层 profile"),
        ("config_stale", "Codex 配置一致性正常"),
        ("config_global_validity", "本地配置引用检查通过"),
        ("router_recovery_record", "路由恢复记录可正常读取"),
        ("router_http_contract", "aimai1 HTTP-only 路由契约正常"),
        (
            "router_postcondition",
            "目标态非配置维度稳定（配置偏差由 config_stale 报告）",
        ),
        ("takeover_backup_orphan", "无 takeover 备份残留"),
        ("catalog_path_validity", "Catalog 路径（路由未启用，跳过）"),
        ("catalog_integrity", "Catalog 完整性（路由未启用，跳过）"),
        (
            "official_catalog_freshness",
            "尚未观测到目录请求，不能据此确认官方目录可用",
        ),
        ("auth_integrity", "auth.json 不存在"),
        ("auth_token_expiry", "Token 过期检测（无 auth.json，跳过）"),
        ("router_unlock_auth_residue", "无虚拟账号残留"),
        ("proxy_running", "本地代理（智能路由未启用，跳过检测）"),
        ("codex_process_state", "Codex 未运行"),
        ("db_orphan_providers", "未发现失效的旧版工具线程 provider"),
        (
            "rollout_orphan_providers",
            "未发现失效的旧版工具 Rollout provider（已检查 0 个）",
        ),
        ("thread_missing_rollouts", "线程 rollout 文件完整"),
        ("thread_history_integrity", "线程历史结构自洽"),
        (
            "thread_provider_consistency",
            "线程已收敛到官方 provider",
        ),
        (
            "thread_model_catalog_consistency",
            "线程模型与 Catalog 一致性（路由未启用，跳过）",
        ),
        (
            "thread_poisoned_instructions",
            "线程系统提示健康（数据库不存在，跳过）",
        ),
        (
            "thread_response_item_ids",
            "线程调用项 ID 健康（数据库不存在，跳过）",
        ),
        ("legacy_migrated_threads", "无旧版线程迁移残留"),
        ("residual_manifest", "无残留迁移 manifest"),
        ("residual_cache", "无残留 relay-state-cache"),
        ("codex_home_writable", "文件系统正常（空间检测不可用）"),
        ("project_state_integrity", "Codex 项目索引正常"),
        ("state_db_relocation", "线程数据库位置正常"),
        (
            "native_off_invalid_openai_thread_models",
            "NativeOff 官方线程模型正常",
        ),
    ]);
    assert_eq!(expected.len(), 34);
    for item in &r.items {
        match expected.get(item.id.as_str()) {
            Some(want) => assert_eq!(&item.label, want, "{} 的 label 与原版不一致", item.id),
            None => panic!("出现了矩阵里没有的条目：{}", item.id),
        }
    }
    assert_eq!(r.items.len(), expected.len());
}
