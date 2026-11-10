use super::native_threads as adapter;
use super::*;
use rusqlite::{params, Connection};
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
};
struct Fixture {
    root: PathBuf,
    ctx: Context,
    db: PathBuf,
}
impl Fixture {
    fn new(router: bool) -> Self {
        let root = std::env::temp_dir().join(format!(
            "atelier-native-thread-tests-{}",
            uuid::Uuid::new_v4()
        ));
        let home = root.join("codex");
        let app = root.join("app");
        fs::create_dir_all(home.join("sessions")).unwrap();
        fs::create_dir_all(&app).unwrap();
        fs::write(
            home.join("config.toml"),
            format!(
                "model_provider = '{}'\n",
                if router { "aimai1" } else { "openai" }
            ),
        )
        .unwrap();
        let db = home.join("state_5.sqlite");
        Connection::open(&db).unwrap().execute_batch("CREATE TABLE threads(id TEXT PRIMARY KEY,model_provider TEXT,rollout_path TEXT,model TEXT,archived INTEGER,source TEXT)").unwrap();
        let ctx = Context {
            home,
            app_root: app,
            desktop_running: false,
            runtime: None,
            official_catalog: None,
            proxy_handle: None,
            working_directory: None,
            sqlite_home_env: None,
        };
        if router {
            let path = ctx.home.join("codexmate/relay/state.json");
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            let mut state =
                atelier_maintenance_core::catalog::state::RelayStateDocument::native_default()
                    .as_value()
                    .clone();
            state["codexRouterEnabled"] = json!(true);
            state["codexNoAccountMode"] = json!(false);
            fs::write(path, serde_json::to_vec(&state).unwrap()).unwrap();
        }
        Self { root, ctx, db }
    }
    fn insert(&self, id: &str, provider: &str, path: &str, model: &str) {
        Connection::open(&self.db)
            .unwrap()
            .execute(
                "INSERT INTO threads VALUES(?1,?2,?3,?4,0,'cli')",
                params![id, provider, path, model],
            )
            .unwrap();
    }
    fn backup(&self) -> PathBuf {
        self.root.join("approved-ticket-backup")
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
fn meta(id: &str, provider: &str, model: &str) -> Value {
    json!({"type":"session_meta","payload":{"id":id,"model_provider":provider,"model":model,"base_instructions":{"text":"healthy fixture"}}})
}
fn write_rows(path: &Path, rows: &[Value]) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let bytes = rows
        .iter()
        .map(|v| serde_json::to_string(v).unwrap() + "\n")
        .collect::<String>()
        .into_bytes();
    fs::write(
        path,
        if path.to_string_lossy().ends_with(".zst") {
            zstd::stream::encode_all(bytes.as_slice(), 3).unwrap()
        } else {
            bytes
        },
    )
    .unwrap();
}
fn rows(path: &Path) -> Vec<Value> {
    let raw = fs::read(path).unwrap();
    let raw = if path.to_string_lossy().ends_with(".zst") {
        zstd::stream::decode_all(raw.as_slice()).unwrap()
    } else {
        raw
    };
    raw.split(|b| *b == b'\n')
        .filter_map(|line| serde_json::from_slice(line).ok())
        .collect()
}
fn provider(f: &Fixture, id: &str) -> String {
    Connection::open(&f.db)
        .unwrap()
        .query_row(
            "SELECT model_provider FROM threads WHERE id=?1",
            [id],
            |r| r.get(0),
        )
        .unwrap()
}
#[test]
fn shared_projection_matches_independent_checks_and_reduces_snapshot_count() {
    let f = Fixture::new(false);
    let c = Connection::open(&f.db).unwrap();
    for n in 0..32 {
        let id = format!("bench-{n}");
        let path = f.ctx.home.join(format!("sessions/{id}.jsonl"));
        let mut values = vec![meta(&id, "openai", "gpt-fixture")];
        values.extend((0..32).map(|i| json!({"type":"event_msg","payload":{"message":format!("synthetic-{i}-{}", "x".repeat(512))}})));
        write_rows(&path, &values);
        c.execute(
            "INSERT INTO threads VALUES(?1,'openai',?2,'gpt-fixture',0,'cli')",
            params![id, path.to_str().unwrap()],
        )
        .unwrap();
    }
    drop(c);
    super::snapshot::take_snapshot_count();
    let start = std::time::Instant::now();
    let independent = adapter::inspect_independently(&f.ctx);
    let old_ms = start.elapsed().as_secs_f64() * 1000.0;
    let old_copies = super::snapshot::take_snapshot_count();
    let start = std::time::Instant::now();
    let (shared, opens, bytes, summary) = super::snapshot::with_observation(|| {
        let items = adapter::inspect(&f.ctx);
        let (opens, bytes) = atelier_maintenance_core::threads::observation::metrics();
        let summary = atelier_maintenance_core::threads::observation::retained_summary_bytes();
        Ok((items, opens, bytes, summary))
    })
    .unwrap();
    let new_ms = start.elapsed().as_secs_f64() * 1000.0;
    let new_copies = super::snapshot::take_snapshot_count();
    assert_eq!(
        serde_json::to_value(independent).unwrap(),
        serde_json::to_value(shared).unwrap()
    );
    assert_eq!(old_copies, 6);
    assert_eq!(new_copies, 1);
    assert_eq!(opens, 32);
    assert!(summary < 64 * 1024 * 1024);
    println!("THREAD_CHECK_BENCH threads=32 events_per_thread=32 independent_ms={old_ms:.2} shared_ms={new_ms:.2} snapshots={old_copies}->{new_copies} shared_opens={opens} shared_decoded_bytes={bytes} retained_summary_estimate_bytes={summary}");
}

#[test]
fn native_provider_preflight_rejects_zero_effect_and_preserves_external_history() {
    let f = Fixture::new(false);
    let path = f.ctx.home.join("sessions/a.jsonl");
    write_rows(&path, &[meta("a", "aimai1", "x")]);
    f.insert("a", "openai", path.to_str().unwrap(), "x");
    let before = fs::read(&path).unwrap();
    assert!(adapter::review_provider_target(&f.ctx, "openai", false)
        .unwrap_err()
        .contains("没有对应"));
    assert_eq!(fs::read(&path).unwrap(), before);
    let plan =
        adapter::prepare_with_backup(&f.ctx, "thread_provider_consistency", &f.backup()).unwrap();
    assert!(plan.changes_provider_target());
    adapter::review_provider_target(&f.ctx, "openai", plan.changes_provider_target()).unwrap();
    assert!(adapter::apply(&f.ctx, &plan).unwrap().success);
    adapter::review_provider_target(&f.ctx, "openai", false).unwrap();
    assert_eq!(rows(&path)[0]["payload"]["model_provider"], "openai");
    write_rows(&path, &[meta("a", "enterprise", "x")]);
    let external = fs::read(&path).unwrap();
    let preserved =
        adapter::prepare_with_backup(&f.ctx, "thread_provider_consistency", &f.backup()).unwrap();
    assert!(!preserved.changes_provider_target());
    adapter::review_provider_target(&f.ctx, "openai", false).unwrap();
    assert_eq!(fs::read(path).unwrap(), external);
}

#[test]
fn native_provider_preflight_blocks_unreadable_and_unplanned_atelier_rows() {
    let f = Fixture::new(false);
    let path = f.ctx.home.join("sessions/a.jsonl");
    write_rows(&path, &[meta("a", "atelier", "x")]);
    f.insert("a", "atelier", path.to_str().unwrap(), "x");
    let before = fs::read(&f.db).unwrap();
    assert!(adapter::review_provider_target(&f.ctx, "openai", true)
        .unwrap_err()
        .contains("Atelier受管线程"));
    assert_eq!(fs::read(&f.db).unwrap(), before);
    Connection::open(&f.db)
        .unwrap()
        .execute("UPDATE threads SET model_provider='openai'", [])
        .unwrap();
    write_rows(
        &path,
        &[json!({"type":"session_meta","payload":{"id":"a","model_provider":3}})],
    );
    assert!(adapter::review_provider_target(&f.ctx, "openai", true)
        .unwrap_err()
        .contains("无法完整核对"));
}

#[test]
fn private_preview_and_diagnostics_read_wal_without_creating_source_shm() {
    let f = Fixture::new(true);
    let history = f.ctx.home.join("sessions/a.jsonl");
    write_rows(&history, &[meta("a", "openai", "x")]);
    let c = Connection::open(&f.db).unwrap();
    c.execute_batch(
        "PRAGMA journal_mode=WAL;PRAGMA wal_autocheckpoint=0;PRAGMA wal_checkpoint(TRUNCATE);",
    )
    .unwrap();
    c.execute(
        "INSERT INTO threads VALUES('a','openai',?1,'x',0,'cli')",
        [history.to_string_lossy().as_ref()],
    )
    .unwrap();
    let db_bytes = fs::read(&f.db).unwrap();
    let wal = PathBuf::from(format!("{}-wal", f.db.display()));
    let wal_bytes = fs::read(&wal).unwrap();
    drop(c);
    fs::write(&f.db, &db_bytes).unwrap();
    fs::write(&wal, &wal_bytes).unwrap();
    let shm = PathBuf::from(format!("{}-shm", f.db.display()));
    assert!(!shm.exists());
    let plan =
        adapter::prepare_with_backup(&f.ctx, "thread_provider_consistency", &f.backup()).unwrap();
    assert!(!adapter::preview_lines(&plan).is_empty());
    let checks = adapter::inspect(&f.ctx);
    assert_eq!(checks.len(), 6);
    assert!(checks.iter().all(|item| item.group == "runtime"));
    assert_eq!(fs::read(&f.db).unwrap(), db_bytes);
    assert_eq!(fs::read(&wal).unwrap(), wal_bytes);
    assert!(!shm.exists());
    assert!(!f.backup().exists());
}
#[test]
fn relative_provider_projection_keeps_original_db_path_and_zstd_representation() {
    let f = Fixture::new(true);
    let history = f.ctx.home.join("sessions/a.jsonl.zst");
    write_rows(&history, &[meta("a", "openai", "x")]);
    f.insert("a", "openai", "sessions/a.jsonl", "x");
    let plan =
        adapter::prepare_with_backup(&f.ctx, "thread_provider_consistency", &f.backup()).unwrap();
    assert_eq!(provider(&f, "a"), "openai");
    let out = adapter::apply(&f.ctx, &plan).unwrap();
    assert!(out.success, "{}", out.detail);
    assert_eq!(provider(&f, "a"), "aimai1");
    assert_eq!(rows(&history)[0]["payload"]["model_provider"], "aimai1");
    assert!(!f.ctx.home.join("sessions/a.jsonl").exists());
    let raw: String = Connection::open(&f.db)
        .unwrap()
        .query_row("SELECT rollout_path FROM threads", [], |r| r.get(0))
        .unwrap();
    assert_eq!(raw, "sessions/a.jsonl");
    assert!(out.backups.iter().all(|p| p.starts_with(f.backup())));
}
#[test]
fn missing_uses_native_codexmate_backup_root_and_preserves_relative_reference() {
    let f = Fixture::new(false);
    let backup = f.ctx.home.join("codexmate/native-backup/deeper/a.jsonl");
    write_rows(&backup, &[meta("a", "openai", "x")]);
    f.insert("a", "openai", "sessions/a.jsonl", "x");
    let plan =
        adapter::prepare_with_backup(&f.ctx, "thread_missing_rollouts", &f.backup()).unwrap();
    let out = adapter::apply(&f.ctx, &plan).unwrap();
    assert!(out.success, "{}", out.detail);
    assert_eq!(
        fs::read(f.ctx.home.join("sessions/a.jsonl")).unwrap(),
        fs::read(backup).unwrap()
    );
    let raw: String = Connection::open(&f.db)
        .unwrap()
        .query_row("SELECT rollout_path FROM threads", [], |r| r.get(0))
        .unwrap();
    assert_eq!(raw, "sessions/a.jsonl");
}
#[test]
fn model_catalog_alias_does_not_move_the_restore_journal_out_of_relay() {
    let f = Fixture::new(true);
    let catalog = f.ctx.home.join("custom/models.json");
    fs::create_dir_all(catalog.parent().unwrap()).unwrap();
    fs::write(
        &catalog,
        serde_json::to_vec(
            &json!({"aimami_no_account":true,"models":[{"slug":"aimami_relay_fixture::current"}]}),
        )
        .unwrap(),
    )
    .unwrap();
    fs::write(
        f.ctx.config(),
        "model_provider='aimai1'\nmodel_catalog_json='custom/models.json'\n",
    )
    .unwrap();
    let history = f.ctx.home.join("sessions/a.jsonl");
    write_rows(&history, &[meta("a", "aimai1", "old::missing")]);
    f.insert("a", "aimai1", "sessions/a.jsonl", "old::missing");
    let plan =
        adapter::prepare_with_backup(&f.ctx, "thread_model_catalog_consistency", &f.backup())
            .unwrap();
    assert!(!f
        .ctx
        .home
        .join("codexmate/relay/router_no_account_model_restore.json")
        .exists());
    let out = adapter::apply(&f.ctx, &plan).unwrap();
    assert!(out.success, "{}", out.detail);
    assert!(f
        .ctx
        .home
        .join("codexmate/relay/router_no_account_model_restore.json")
        .exists());
    assert!(!catalog
        .parent()
        .unwrap()
        .join("router_no_account_model_restore.json")
        .exists());
    assert_eq!(
        rows(&history)[0]["payload"]["model"],
        "aimami_relay_fixture::current"
    );
}
#[test]
fn stale_db_and_history_plans_refuse_before_mutation() {
    let f = Fixture::new(true);
    let history = f.ctx.home.join("sessions/a.jsonl");
    write_rows(&history, &[meta("a", "openai", "x")]);
    f.insert("a", "openai", history.to_str().unwrap(), "x");
    let original = fs::read(&history).unwrap();
    let plan = adapter::prepare(&f.ctx, "thread_provider_consistency").unwrap();
    Connection::open(&f.db)
        .unwrap()
        .execute("UPDATE threads SET model='external'", [])
        .unwrap();
    assert!(adapter::apply(&f.ctx, &plan)
        .unwrap_err()
        .contains("外部更新"));
    assert_eq!(fs::read(&history).unwrap(), original);
    let plan = adapter::prepare(&f.ctx, "thread_provider_consistency").unwrap();
    fs::write(&history, b"external file revision\n").unwrap();
    assert!(adapter::apply(&f.ctx, &plan)
        .unwrap_err()
        .contains("外部更新"));
    assert_eq!(provider(&f, "a"), "openai");
}
#[test]
fn mixed_missing_then_provider_is_reprepared_inside_approved_scope() {
    let f = Fixture::new(true);
    let source = f.ctx.home.join("codexmate/backups/a.jsonl");
    write_rows(&source, &[meta("a", "openai", "x")]);
    f.insert("a", "openai", "sessions/a.jsonl", "x");
    let plans = adapter::prepare_sequence(
        &f.ctx,
        &[
            "thread_provider_consistency".into(),
            "thread_missing_rollouts".into(),
        ],
        &f.backup(),
    )
    .unwrap();
    let out = adapter::apply_sequence(&f.ctx, &plans).unwrap();
    assert_eq!(
        out.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(),
        vec!["thread_missing_rollouts", "thread_provider_consistency"]
    );
    assert!(out.iter().all(|r| r.success));
    assert_eq!(provider(&f, "a"), "aimai1");
    assert_eq!(
        rows(&f.ctx.home.join("sessions/a.jsonl"))[0]["payload"]["model_provider"],
        "aimai1"
    );
}
#[test]
fn mixed_later_sql_failure_reports_partial_effects_and_real_final_check() {
    let f = Fixture::new(true);
    let source = f.ctx.home.join("codexmate/backups/a.jsonl");
    write_rows(&source, &[meta("a", "openai", "x")]);
    f.insert("a", "openai", "sessions/a.jsonl", "x");
    Connection::open(&f.db).unwrap().execute_batch("CREATE TRIGGER reject_provider BEFORE UPDATE OF model_provider ON threads BEGIN SELECT RAISE(ABORT,'fixture denied');END").unwrap();
    let plans = adapter::prepare_sequence(
        &f.ctx,
        &[
            "thread_missing_rollouts".into(),
            "thread_provider_consistency".into(),
        ],
        &f.backup(),
    )
    .unwrap();
    let out = adapter::apply_sequence(&f.ctx, &plans).unwrap();
    assert_eq!(out.len(), 2);
    assert!(out[0].success);
    assert!(!out[1].success);
    assert!(f.ctx.home.join("sessions/a.jsonl").exists());
    assert_eq!(provider(&f, "a"), "openai");
    assert_eq!(
        rows(&f.ctx.home.join("sessions/a.jsonl"))[0]["payload"]["model_provider"],
        "openai"
    );
    assert_ne!(out[1].diagnostic.status, "ok");
}
#[test]
fn external_provider_history_is_preserved_by_real_native_postcheck() {
    let f = Fixture::new(true);
    let history = f.ctx.home.join("sessions/a.jsonl");
    write_rows(&history, &[meta("a", "enterprise", "x")]);
    f.insert("a", "openai", history.to_str().unwrap(), "x");
    let original = fs::read(&history).unwrap();
    let p = adapter::prepare(&f.ctx, "thread_provider_consistency").unwrap();
    let out = adapter::apply(&f.ctx, &p).unwrap();
    assert!(out.success);
    assert_eq!(out.changed, 0);
    assert_eq!(provider(&f, "a"), "openai");
    assert_eq!(fs::read(history).unwrap(), original);
}
#[test]
fn instruction_previews_redact_templates_and_ticket_owns_backup_location() {
    let f = Fixture::new(false);
    let source = f.ctx.home.join("sessions/source.jsonl");
    let target = f.ctx.home.join("sessions/target.jsonl");
    let reference = "PRIVATE_TEMPLATE_MUST_NOT_APPEAR".repeat(50);
    let mut healthy = meta("source", "openai", "x");
    healthy["payload"]["base_instructions"]["text"] = json!(reference);
    let mut poisoned = meta("target", "openai", "x");
    poisoned["payload"]["base_instructions"]["text"] =
        json!("You are judging one planned coding-agent action PRIVATE_SESSION_BODY");
    write_rows(&source, &[healthy]);
    write_rows(&target, &[poisoned]);
    f.insert("source", "openai", source.to_str().unwrap(), "x");
    f.insert("target", "openai", "sessions/target.jsonl", "x");
    let p =
        adapter::prepare_with_backup(&f.ctx, "thread_poisoned_instructions", &f.backup()).unwrap();
    let lines = adapter::preview_lines(&p).join("\n");
    assert!(!lines.contains("PRIVATE_"));
    let result = adapter::apply(&f.ctx, &p).unwrap();
    assert!(result.success, "{}", result.detail);
    assert!(!result.detail.contains("PRIVATE_"));
    assert!(result.backups.iter().all(|p| p.starts_with(f.backup())));
    assert_eq!(
        rows(&target)[0]["payload"]["base_instructions"]["text"],
        reference
    );
    let path: String = Connection::open(&f.db)
        .unwrap()
        .query_row(
            "SELECT rollout_path FROM threads WHERE id='target'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(path, "sessions/target.jsonl");
}
#[test]
fn stale_committed_wal_is_rejected_even_when_main_database_bytes_are_unchanged() {
    let f = Fixture::new(true);
    let history = f.ctx.home.join("sessions/a.jsonl");
    write_rows(&history, &[meta("a", "openai", "x")]);
    let c = Connection::open(&f.db).unwrap();
    c.execute_batch("PRAGMA journal_mode=WAL;PRAGMA wal_autocheckpoint=0;")
        .unwrap();
    c.execute(
        "INSERT INTO threads VALUES('a','openai',?1,'x',0,'cli')",
        [history.to_string_lossy().as_ref()],
    )
    .unwrap();
    let p = adapter::prepare(&f.ctx, "thread_provider_consistency").unwrap();
    let before = fs::read(&f.db).unwrap();
    c.execute("UPDATE threads SET model='external wal revision'", [])
        .unwrap();
    assert_eq!(fs::read(&f.db).unwrap(), before);
    assert!(adapter::apply(&f.ctx, &p).unwrap_err().contains("外部更新"));
    assert_eq!(rows(&history)[0]["payload"]["model_provider"], "openai");
}
#[test]
fn response_adapter_uses_actual_history_count_and_private_ticket_backup() {
    let f = Fixture::new(false);
    let history = f.ctx.home.join("sessions/a.jsonl");
    write_rows(
        &history,
        &[
            meta("a", "openai", "x"),
            json!({"type":"response_item","payload":{"type":"custom_tool_call","id":"fc_fixture"}}),
        ],
    );
    f.insert("a", "openai", "sessions/a.jsonl", "x");
    let p = adapter::prepare_with_backup(&f.ctx, "thread_response_item_ids", &f.backup()).unwrap();
    let result = adapter::apply(&f.ctx, &p).unwrap();
    assert!(result.success);
    assert_eq!(rows(&history)[1]["payload"]["id"], "ctc_fixture");
    assert!(result.backups.iter().all(|p| p.starts_with(f.backup())));
    assert_eq!(result.diagnostic.status, "ok");
}
#[test]
fn router_enabled_legacy_cleanup_is_not_approved_as_a_conflicting_action() {
    let f = Fixture::new(true);
    let history = f.ctx.home.join("sessions/a.jsonl");
    write_rows(&history, &[meta("a", "aimai1", "x")]);
    f.insert("a", "aimai1", "sessions/a.jsonl", "x");
    let checks = adapter::inspect(&f.ctx);
    let check = checks
        .iter()
        .find(|i| i.id == "legacy_migrated_threads")
        .unwrap();
    assert_eq!(check.status, "ok");
    assert!(!check.fixable);
    let error = adapter::prepare_sequence(
        &f.ctx,
        &[
            "legacy_migrated_threads".into(),
            "thread_provider_consistency".into(),
        ],
        &f.backup(),
    )
    .err()
    .unwrap();
    assert!(error.contains("路由启用"));
    assert_eq!(provider(&f, "a"), "aimai1");
    assert!(!f.backup().exists());
}

#[test]
fn a_new_migration_target_after_preview_requires_fresh_review() {
    let f = Fixture::new(false);
    let history = f.ctx.home.join("sessions/a.jsonl");
    write_rows(&history, &[meta("a", "openai", "x")]);
    f.insert("a", "openai", "sessions/a.jsonl", "x");
    let p =
        adapter::prepare_with_backup(&f.ctx, "thread_poisoned_instructions", &f.backup()).unwrap();
    let root = f
        .ctx
        .home
        .join("codexmate/relay/thread-router-migrations/history");
    fs::create_dir_all(&root).unwrap();
    let mut dirty = meta("stored", "openai", "x");
    dirty["payload"]["base_instructions"]["text"] = json!("");
    let file = root.join("new.json");
    let original =
        serde_json::to_vec(&json!({"entries":[{"meta":serde_json::to_string(&dirty).unwrap()}]}))
            .unwrap();
    fs::write(&file, &original).unwrap();
    assert!(adapter::apply(&f.ctx, &p)
        .unwrap_err()
        .contains("范围已变化"));
    assert_eq!(fs::read(file).unwrap(), original);
    assert!(!f.backup().exists());
}
#[test]
fn private_relative_projection_does_not_fire_application_path_triggers() {
    let f = Fixture::new(true);
    let history = f.ctx.home.join("sessions/a.jsonl");
    write_rows(&history, &[meta("a", "openai", "x")]);
    f.insert("a", "openai", "sessions/a.jsonl", "x");
    Connection::open(&f.db).unwrap().execute_batch("CREATE TRIGGER preserve_raw_paths BEFORE UPDATE OF rollout_path ON threads BEGIN SELECT RAISE(ABORT,'path must remain relative');END").unwrap();
    let p =
        adapter::prepare_with_backup(&f.ctx, "thread_provider_consistency", &f.backup()).unwrap();
    assert_eq!(p.id(), "thread_provider_consistency");
    adapter::validate_sequence(&f.ctx, &[p.clone()]).unwrap();
    let outcome = adapter::apply(&f.ctx, &p).unwrap();
    assert!(outcome.success);
    assert!(!outcome.partial_effects_possible);
    assert!(outcome.warnings.is_empty());
    let c = Connection::open(&f.db).unwrap();
    let count: i64 = c
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='trigger' AND name='preserve_raw_paths'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(count, 1);
    let path: String = c
        .query_row("SELECT rollout_path FROM threads", [], |r| r.get(0))
        .unwrap();
    assert_eq!(path, "sessions/a.jsonl");
}
#[test]
fn router_off_legacy_adapter_uses_real_cleanup_and_keeps_relative_db_path() {
    let f = Fixture::new(false);
    let history = f.ctx.home.join("sessions/a.jsonl");
    write_rows(&history, &[meta("a", "aimai1", "x")]);
    f.insert("a", "aimai1", "sessions/a.jsonl", "x");
    let check = adapter::inspect(&f.ctx)
        .into_iter()
        .find(|i| i.id == "legacy_migrated_threads")
        .unwrap();
    assert_eq!(check.status, "error");
    assert!(check.fixable);
    let plan =
        adapter::prepare_with_backup(&f.ctx, "legacy_migrated_threads", &f.backup()).unwrap();
    let outcome = adapter::apply(&f.ctx, &plan).unwrap();
    assert!(outcome.success);
    assert_eq!(provider(&f, "a"), "openai");
    assert_eq!(rows(&history)[0]["payload"]["model_provider"], "openai");
    assert_eq!(
        Connection::open(&f.db)
            .unwrap()
            .query_row("SELECT rollout_path FROM threads", [], |r| r
                .get::<_, String>(0))
            .unwrap(),
        "sessions/a.jsonl"
    );
    assert!(!outcome.backups.is_empty());
}

struct OfficialFixtureRpc {
    step: usize,
}
impl super::native_orphans::CatalogRpc for OfficialFixtureRpc {
    fn request(
        &mut self,
        method: &str,
        params: Value,
        timeout: std::time::Duration,
    ) -> Result<Value> {
        assert!(!timeout.is_zero());
        self.step += 1;
        match self.step {
            1 | 3 => {
                assert_eq!(method, "config/read");
                assert_eq!(params, json!({"includeLayers":true}));
                Ok(json!({"config":{"model_provider":"openai"}}))
            }
            2 => {
                assert_eq!(method, "model/list");
                assert_eq!(params, json!({"includeHidden":true,"limit":100}));
                Ok(json!({"data":[{"model":"gpt-known","isDefault":true,
                    "supportedReasoningEfforts":[{"reasoningEffort":"medium"}],
                    "defaultReasoningEffort":"medium"}],"nextCursor":null}))
            }
            _ => panic!("unexpected real-source fixture RPC"),
        }
    }
}
fn orphan_prefix_plan(f: &Fixture) -> super::native_orphans::Prepared {
    let mut rpc = OfficialFixtureRpc { step: 0 };
    let official = super::native_orphans::prepare_catalog_with_rpc(&f.ctx, &mut rpc).unwrap();
    assert_eq!(rpc.step, 3);
    super::native_orphans::prepare_with_catalog(&f.ctx, &official).unwrap()
}
fn orphan_history(f: &Fixture, compressed: bool) -> PathBuf {
    let requested = f.ctx.home.join("sessions/a.jsonl");
    let physical = if compressed {
        requested.with_extension("jsonl.zst")
    } else {
        requested
    };
    write_rows(
        &physical,
        &[
            meta("a", "aimami_relay_deleted", "gpt-known"),
            json!({"type":"response_item","payload":{"type":"custom_tool_call","id":"fc_fixture"}}),
        ],
    );
    f.insert("a", "aimami_relay_deleted", "sessions/a.jsonl", "gpt-known");
    physical
}

#[test]
fn verified_orphan_prefix_allows_real_followup_repairs_for_plain_and_zstd_history() {
    for compressed in [false, true] {
        let f = Fixture::new(false);
        let history = orphan_history(&f, compressed);
        let external = f.ctx.home.join("sessions/external.jsonl");
        write_rows(
            &external,
            &[meta("external", "enterprise", "private-model")],
        );
        f.insert(
            "external",
            "enterprise",
            "sessions/external.jsonl",
            "private-model",
        );
        let external_before = fs::read(&external).unwrap();
        let plans =
            adapter::prepare_sequence(&f.ctx, &["thread_response_item_ids".into()], &f.backup())
                .unwrap();
        adapter::validate_sequence(&f.ctx, &plans).unwrap();
        let orphan = orphan_prefix_plan(&f);
        let guard = super::repair::acquire(&f.ctx).unwrap();
        let prefix = super::native_orphans::apply_verified(
            &f.ctx,
            &orphan,
            &f.root.join("orphan-prefix"),
            &guard,
        )
        .unwrap();
        assert!(
            adapter::validate_sequence(&f.ctx, &plans).is_err(),
            "old before must actually have changed"
        );
        let outcomes =
            adapter::apply_sequence_after_approved_prefix(&f.ctx, &plans, &prefix.effects).unwrap();
        assert!(outcomes.iter().all(|o| o.success));
        assert_eq!(provider(&f, "a"), "openai");
        assert_eq!(rows(&history)[0]["payload"]["model_provider"], "openai");
        assert_eq!(rows(&history)[1]["payload"]["id"], "ctc_fixture");
        assert!(outcomes[0]
            .backups
            .iter()
            .all(|p| p.starts_with(f.backup())));
        assert_eq!(fs::read(external).unwrap(), external_before);
        assert_eq!(provider(&f, "external"), "enterprise");
    }
}

#[test]
fn orphan_prefix_from_a_different_original_history_cannot_authorize_old_thread_plan() {
    let f = Fixture::new(false);
    let history = orphan_history(&f, false);
    let plans =
        adapter::prepare_sequence(&f.ctx, &["thread_response_item_ids".into()], &f.backup())
            .unwrap();
    let mut changed = rows(&history);
    changed.push(json!({"type":"event_msg","payload":{"type":"user_message","message":"another approved preview"}}));
    write_rows(&history, &changed);
    let orphan = orphan_prefix_plan(&f);
    let guard = super::repair::acquire(&f.ctx).unwrap();
    let prefix = super::native_orphans::apply_verified(
        &f.ctx,
        &orphan,
        &f.root.join("orphan-prefix"),
        &guard,
    )
    .unwrap();
    let after = fs::read(&history).unwrap();
    let error =
        adapter::apply_sequence_after_approved_prefix(&f.ctx, &plans, &prefix.effects).unwrap_err();
    assert!(error.contains("原始依赖不属于"));
    assert_eq!(fs::read(history).unwrap(), after);
    assert!(!f.backup().exists());
}

#[test]
fn orphan_prefix_after_revision_is_not_renewed_from_external_current_bytes_or_mtime() {
    for timestamp_only in [false, true] {
        let f = Fixture::new(false);
        let history = orphan_history(&f, false);
        let plans =
            adapter::prepare_sequence(&f.ctx, &["thread_response_item_ids".into()], &f.backup())
                .unwrap();
        adapter::validate_sequence(&f.ctx, &plans).unwrap();
        let orphan = orphan_prefix_plan(&f);
        let guard = super::repair::acquire(&f.ctx).unwrap();
        let prefix = super::native_orphans::apply_verified(
            &f.ctx,
            &orphan,
            &f.root.join("orphan-prefix"),
            &guard,
        )
        .unwrap();
        if timestamp_only {
            let modified = fs::metadata(&history).unwrap().modified().unwrap()
                + std::time::Duration::from_secs(5);
            fs::File::options()
                .write(true)
                .open(&history)
                .unwrap()
                .set_times(fs::FileTimes::new().set_modified(modified))
                .unwrap();
        } else {
            let mut changed = rows(&history);
            changed.push(json!({"type":"event_msg","payload":{"type":"user_message","message":"external current bytes"}}));
            write_rows(&history, &changed);
        }
        let current = fs::read(&history).unwrap();
        assert!(
            adapter::apply_sequence_after_approved_prefix(&f.ctx, &plans, &prefix.effects)
                .unwrap_err()
                .contains("不等于核验效果")
        );
        assert_eq!(fs::read(history).unwrap(), current);
        assert!(!f.backup().exists());
    }
}

#[test]
fn unmodified_by_orphan_prefix_dependencies_still_require_the_original_revision() {
    let f = Fixture::new(false);
    orphan_history(&f, false);
    let plans =
        adapter::prepare_sequence(&f.ctx, &["thread_response_item_ids".into()], &f.backup())
            .unwrap();
    adapter::validate_sequence(&f.ctx, &plans).unwrap();
    let orphan = orphan_prefix_plan(&f);
    let guard = super::repair::acquire(&f.ctx).unwrap();
    let prefix = super::native_orphans::apply_verified(
        &f.ctx,
        &orphan,
        &f.root.join("orphan-prefix"),
        &guard,
    )
    .unwrap();
    fs::write(
        f.ctx.config(),
        "model_provider='openai'\n# external configuration revision\n",
    )
    .unwrap();
    let current = fs::read(f.ctx.config()).unwrap();
    assert!(
        adapter::apply_sequence_after_approved_prefix(&f.ctx, &plans, &prefix.effects).is_err()
    );
    assert_eq!(fs::read(f.ctx.config()).unwrap(), current);
    assert!(!f.backup().exists());
}

#[test]
fn approved_orphan_prefix_cannot_expand_followup_snapshot_targets() {
    let f = Fixture::new(false);
    let history = f.ctx.home.join("sessions/a.jsonl");
    let mut healthy = meta("a", "aimami_relay_deleted", "gpt-known");
    healthy["payload"]["base_instructions"]["text"] =
        json!("healthy explicit synthetic template ".repeat(50));
    write_rows(&history, &[healthy]);
    f.insert("a", "aimami_relay_deleted", "sessions/a.jsonl", "gpt-known");
    let plans = adapter::prepare_sequence(
        &f.ctx,
        &["thread_poisoned_instructions".into()],
        &f.backup(),
    )
    .unwrap();
    adapter::validate_sequence(&f.ctx, &plans).unwrap();
    let orphan = orphan_prefix_plan(&f);
    let guard = super::repair::acquire(&f.ctx).unwrap();
    let prefix = super::native_orphans::apply_verified(
        &f.ctx,
        &orphan,
        &f.root.join("orphan-prefix"),
        &guard,
    )
    .unwrap();
    let directory = f
        .ctx
        .home
        .join("codexmate/relay/thread-router-migrations/history");
    fs::create_dir_all(&directory).unwrap();
    let mut dirty = meta("stored", "openai", "gpt-known");
    dirty["payload"]["base_instructions"]["text"] = json!("");
    let path = directory.join("new.json");
    let bytes =
        serde_json::to_vec(&json!({"entries":[{"meta":serde_json::to_string(&dirty).unwrap()}]}))
            .unwrap();
    fs::write(&path, &bytes).unwrap();
    let error =
        adapter::apply_sequence_after_approved_prefix(&f.ctx, &plans, &prefix.effects).unwrap_err();
    assert!(error.contains("未批准的新目标"));
    assert_eq!(fs::read(path).unwrap(), bytes);
    assert!(!f.backup().exists());
}

#[test]
fn per_item_session_executes_only_the_requested_step_and_certifies_actual_sql_and_files() {
    let f = Fixture::new(true);
    let history = f.ctx.home.join("sessions/a.jsonl");
    write_rows(
        &history,
        &[
            meta("a", "openai", "x"),
            json!({"type":"response_item","payload":{"type":"custom_tool_call","id":"fc_fixture"}}),
        ],
    );
    f.insert("a", "openai", "sessions/a.jsonl", "x");
    let plans = adapter::prepare_sequence(
        &f.ctx,
        &[
            "thread_provider_consistency".into(),
            "thread_response_item_ids".into(),
        ],
        &f.backup(),
    )
    .unwrap();
    adapter::validate_sequence(&f.ctx, &plans).unwrap();
    let mut session = adapter::begin_reviewed(
        &f.ctx,
        &plans,
        &super::native_orphans::ApprovedEffects::default(),
    )
    .unwrap();
    assert!(
        adapter::apply_item(&f.ctx, &mut session, "thread_response_item_ids")
            .unwrap_err()
            .contains("原生批次顺序")
    );
    let first = adapter::apply_item(&f.ctx, &mut session, "thread_provider_consistency").unwrap();
    assert!(first.success, "{}", first.detail);
    assert_eq!(provider(&f, "a"), "aimai1");
    assert!(
        adapter::apply_item(&f.ctx, &mut session, "thread_provider_consistency")
            .unwrap_err()
            .contains("重复派发")
    );
    assert_eq!(
        rows(&history)[1]["payload"]["id"],
        "fc_fixture",
        "first step must not pre-run the next repair"
    );
    session.effects().validate_current(&f.ctx).unwrap();
    assert!(session.effects().effect_stamp(&f.db).is_some());
    let next = adapter::apply_item(&f.ctx, &mut session, "thread_response_item_ids").unwrap();
    assert!(next.success, "{}", next.detail);
    assert_eq!(rows(&history)[1]["payload"]["id"], "ctc_fixture");
    session.effects().validate_current(&f.ctx).unwrap();
}
#[test]
fn ordinary_failure_with_verified_rollback_does_not_skip_the_next_independent_thread_item() {
    let f = Fixture::new(false);
    let history = f.ctx.home.join("sessions/a.jsonl");
    write_rows(
        &history,
        &[
            meta("a", "aimai1", "x"),
            json!({"type":"response_item","payload":{"type":"custom_tool_call","id":"fc_fixture"}}),
        ],
    );
    f.insert("a", "aimai1", "sessions/a.jsonl", "x");
    Connection::open(&f.db).unwrap().execute_batch("CREATE TRIGGER reject_provider BEFORE UPDATE OF model_provider ON threads BEGIN SELECT RAISE(ABORT,'ordinary provider failure'); END").unwrap();
    let plans = adapter::prepare_sequence(
        &f.ctx,
        &[
            "thread_provider_consistency".into(),
            "thread_response_item_ids".into(),
        ],
        &f.backup(),
    )
    .unwrap();
    let mut session = adapter::begin_reviewed(
        &f.ctx,
        &plans,
        &super::native_orphans::ApprovedEffects::default(),
    )
    .unwrap();
    let first = adapter::apply_item(&f.ctx, &mut session, "thread_provider_consistency").unwrap();
    assert!(!first.success);
    assert_eq!(provider(&f, "a"), "aimai1");
    assert_eq!(rows(&history)[0]["payload"]["model_provider"], "aimai1");
    let second = adapter::apply_item(&f.ctx, &mut session, "thread_response_item_ids").unwrap();
    assert!(second.success, "{}", second.detail);
    assert_eq!(rows(&history)[1]["payload"]["id"], "ctc_fixture");
}
#[test]
fn missing_history_session_restores_then_replans_only_the_later_requested_provider_step() {
    let f = Fixture::new(true);
    let source = f.ctx.home.join("codexmate/backups/a.jsonl");
    write_rows(&source, &[meta("a", "openai", "x")]);
    f.insert("a", "openai", "sessions/a.jsonl", "x");
    let plans = adapter::prepare_sequence(
        &f.ctx,
        &[
            "thread_missing_rollouts".into(),
            "thread_provider_consistency".into(),
        ],
        &f.backup(),
    )
    .unwrap();
    let mut session = adapter::begin_reviewed(
        &f.ctx,
        &plans,
        &super::native_orphans::ApprovedEffects::default(),
    )
    .unwrap();
    let restored = adapter::apply_item(&f.ctx, &mut session, "thread_missing_rollouts").unwrap();
    assert!(restored.success, "{}", restored.detail);
    assert_eq!(provider(&f, "a"), "openai");
    assert_eq!(
        rows(&f.ctx.home.join("sessions/a.jsonl"))[0]["payload"]["model_provider"],
        "openai"
    );
    let fixed = adapter::apply_item(&f.ctx, &mut session, "thread_provider_consistency").unwrap();
    assert!(fixed.success, "{}", fixed.detail);
    assert_eq!(provider(&f, "a"), "aimai1");
}
#[test]
fn real_orphan_certificate_is_forwarded_and_followup_response_runs_only_on_dispatch() {
    let f = Fixture::new(false);
    let history = orphan_history(&f, true);
    let plans =
        adapter::prepare_sequence(&f.ctx, &["thread_response_item_ids".into()], &f.backup())
            .unwrap();
    adapter::validate_sequence(&f.ctx, &plans).unwrap();
    let orphan = orphan_prefix_plan(&f);
    let guard = super::repair::acquire(&f.ctx).unwrap();
    let prefix = super::native_orphans::apply_verified(
        &f.ctx,
        &orphan,
        &f.root.join("orphan-prefix"),
        &guard,
    )
    .unwrap();
    let mut session = adapter::begin_reviewed(&f.ctx, &plans, &prefix.effects).unwrap();
    assert_eq!(rows(&history)[1]["payload"]["id"], "fc_fixture");
    assert_eq!(
        session.effects().effect_stamp(&f.db),
        prefix.effects.effect_stamp(&f.db)
    );
    let result = adapter::apply_item(&f.ctx, &mut session, "thread_response_item_ids").unwrap();
    assert!(result.success, "{}", result.detail);
    assert_eq!(rows(&history)[1]["payload"]["id"], "ctc_fixture");
    session.effects().validate_current(&f.ctx).unwrap();
}
#[test]
fn external_changes_between_items_are_not_accepted_as_a_new_approved_baseline() {
    let f = Fixture::new(true);
    let history = f.ctx.home.join("sessions/a.jsonl");
    write_rows(
        &history,
        &[
            meta("a", "openai", "x"),
            json!({"type":"response_item","payload":{"type":"custom_tool_call","id":"fc_fixture"}}),
        ],
    );
    f.insert("a", "openai", "sessions/a.jsonl", "x");
    let plans = adapter::prepare_sequence(
        &f.ctx,
        &[
            "thread_provider_consistency".into(),
            "thread_response_item_ids".into(),
        ],
        &f.backup(),
    )
    .unwrap();
    let mut session = adapter::begin_reviewed(
        &f.ctx,
        &plans,
        &super::native_orphans::ApprovedEffects::default(),
    )
    .unwrap();
    assert!(
        adapter::apply_item(&f.ctx, &mut session, "thread_provider_consistency")
            .unwrap()
            .success
    );
    let mut changed = rows(&history);
    changed.push(json!({"type":"event_msg","payload":{"type":"user_message","message":"external replacement"}}));
    write_rows(&history, &changed);
    let current = fs::read(&history).unwrap();
    assert!(adapter::apply_item(&f.ctx, &mut session, "thread_response_item_ids").is_err());
    assert!(adapter::apply_item(&f.ctx, &mut session, "thread_response_item_ids").is_err());
    assert_eq!(fs::read(history).unwrap(), current);
}
#[test]
fn changed_unrelated_sqlite_column_after_native_write_poison_session_instead_of_signing_current_hash(
) {
    let f = Fixture::new(true);
    let history = f.ctx.home.join("sessions/a.jsonl");
    write_rows(
        &history,
        &[
            meta("a", "openai", "x"),
            json!({"type":"response_item","payload":{"type":"custom_tool_call","id":"fc_fixture"}}),
        ],
    );
    f.insert("a", "openai", "sessions/a.jsonl", "x");
    let foreign = f.ctx.home.join("sessions/foreign.jsonl");
    write_rows(
        &foreign,
        &[meta("foreign", "enterprise", "original-private-model")],
    );
    f.insert(
        "foreign",
        "enterprise",
        "sessions/foreign.jsonl",
        "original-private-model",
    );
    let plans = adapter::prepare_sequence(
        &f.ctx,
        &[
            "thread_provider_consistency".into(),
            "thread_response_item_ids".into(),
        ],
        &f.backup(),
    )
    .unwrap();
    let mut session = adapter::begin_reviewed(
        &f.ctx,
        &plans,
        &super::native_orphans::ApprovedEffects::default(),
    )
    .unwrap();
    let result =
        adapter::apply_item_with_hook(&f.ctx, &mut session, "thread_provider_consistency", || {
            Connection::open(&f.db)
                .unwrap()
                .execute(
                    "UPDATE threads SET model='external-unreviewed-model' WHERE id='foreign'",
                    [],
                )
                .unwrap();
        })
        .unwrap();
    assert!(!result.success);
    assert!(result.partial_effects_possible);
    assert!(result.detail.contains("原生SQL效果"));
    assert_eq!(provider(&f, "a"), "aimai1");
    assert!(adapter::apply_item(&f.ctx, &mut session, "thread_response_item_ids").is_err());
    assert_eq!(rows(&history)[1]["payload"]["id"], "fc_fixture");
}
#[test]
fn no_thread_items_still_forward_verified_orphan_prefix_and_reject_unreviewed_ids() {
    let f = Fixture::new(false);
    orphan_history(&f, false);
    let orphan = orphan_prefix_plan(&f);
    let guard = super::repair::acquire(&f.ctx).unwrap();
    let prefix = super::native_orphans::apply_verified(
        &f.ctx,
        &orphan,
        &f.root.join("orphan-prefix"),
        &guard,
    )
    .unwrap();
    let mut session = adapter::begin_reviewed(&f.ctx, &[], &prefix.effects).unwrap();
    assert_eq!(
        session.effects().effect_stamp(&f.db),
        prefix.effects.effect_stamp(&f.db)
    );
    session.effects().validate_current(&f.ctx).unwrap();
    assert!(adapter::apply_item(&f.ctx, &mut session, "thread_response_item_ids").is_err());
}

#[test]
fn final_session_receipt_includes_unchanged_original_database_and_rejects_late_changes() {
    let f = Fixture::new(false);
    let history = f.ctx.home.join("sessions/a.jsonl");
    write_rows(
        &history,
        &[
            meta("a", "openai", "x"),
            json!({"type":"response_item","payload":{"type":"custom_tool_call","id":"fc_fixture"}}),
        ],
    );
    f.insert("a", "openai", "sessions/a.jsonl", "x");
    let plans =
        adapter::prepare_sequence(&f.ctx, &["thread_response_item_ids".into()], &f.backup())
            .unwrap();
    let mut session = adapter::begin_reviewed(
        &f.ctx,
        &plans,
        &super::native_orphans::ApprovedEffects::default(),
    )
    .unwrap();
    assert!(
        adapter::apply_item(&f.ctx, &mut session, "thread_response_item_ids")
            .unwrap()
            .success
    );
    assert!(session.effects().effect_stamp(&f.db).is_none());
    let final_receipt = session.verified_effects(&f.ctx).unwrap();
    let (before, after) = final_receipt.effect_stamp(&f.db).unwrap();
    assert_eq!(before, after);
    assert!(before.is_some());
    Connection::open(&f.db)
        .unwrap()
        .execute("UPDATE threads SET model='late external change'", [])
        .unwrap();
    assert!(session.verified_effects(&f.ctx).is_err());
    assert!(final_receipt.validate_current(&f.ctx).is_err());
}
