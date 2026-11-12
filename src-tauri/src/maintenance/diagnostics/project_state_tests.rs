//! All paths, SQLite files and rollouts belong to a temporary fixture.
use super::*;
use serde_json::{json, Value};
struct Fixture {
    root: PathBuf,
    ctx: Context,
    db: PathBuf,
}
impl Fixture {
    fn new() -> Self {
        let root =
            std::env::temp_dir().join(format!("atelier-project-state-{}", uuid::Uuid::new_v4()));
        let ctx = Context {
            home: root.join("codex"),
            app_root: root.join("atelier"),
            desktop_running: false,
            runtime: None,
            official_catalog: None,
            proxy_handle: None,
            working_directory: Some(root.clone()),
            sqlite_home_env: None,
        };
        fs::create_dir_all(ctx.home.join("sessions")).unwrap();
        fs::create_dir_all(&ctx.app_root).unwrap();
        fs::write(ctx.config(), "model_provider='openai'\n").unwrap();
        let db = ctx.home.join("state_5.sqlite");
        Self::schema(&db);
        Self { root, ctx, db }
    }
    fn schema(path: &Path) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        Connection::open(path).unwrap().execute_batch("CREATE TABLE threads(id TEXT PRIMARY KEY,cwd TEXT,rollout_path TEXT,archived INTEGER,source TEXT,updated_at_ms INTEGER)").unwrap();
    }
    fn state(&self, value: Value) {
        fs::write(
            self.ctx.home.join(".codex-global-state.json"),
            serde_json::to_vec(&value).unwrap(),
        )
        .unwrap();
    }
    fn row_at(
        &self,
        db: &Path,
        id: &str,
        cwd: &str,
        source: Option<&str>,
        updated: i64,
        compressed: bool,
    ) -> PathBuf {
        let requested = format!("sessions/{id}.jsonl");
        let path = self.ctx.home.join(if compressed {
            format!("{requested}.zst")
        } else {
            requested.clone()
        });
        let bytes = format!(
            "{}\n",
            json!({"type":"session_meta","payload":{"id":id,"cwd":format!(" /restored/{id}/ ")}})
        )
        .into_bytes();
        fs::write(
            &path,
            if compressed {
                zstd::stream::encode_all(bytes.as_slice(), 3).unwrap()
            } else {
                bytes
            },
        )
        .unwrap();
        Connection::open(db)
            .unwrap()
            .execute(
                "INSERT INTO threads VALUES(?1,?2,?3,0,?4,?5)",
                rusqlite::params![id, cwd, requested, source, updated],
            )
            .unwrap();
        path
    }
    fn row(
        &self,
        id: &str,
        cwd: &str,
        source: Option<&str>,
        updated: i64,
        compressed: bool,
    ) -> PathBuf {
        self.row_at(&self.db, id, cwd, source, updated, compressed)
    }
    fn apply(&self, plan: &Prepared) -> Outcome {
        let guard = super::super::repair::acquire(&self.ctx).unwrap();
        apply_under_manager(&self.ctx, plan, &guard, &self.root.join("ticket-backup")).unwrap()
    }
    fn value(&self) -> Value {
        serde_json::from_slice(&fs::read(self.ctx.home.join(".codex-global-state.json")).unwrap())
            .unwrap()
    }
    fn cwd(&self, id: &str) -> String {
        Connection::open(&self.db)
            .unwrap()
            .query_row(
                "SELECT COALESCE(cwd,'') FROM threads WHERE id=?1",
                [id],
                |r| r.get(0),
            )
            .unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
#[test]
fn missing_file_is_rebuilt_from_native_records_and_active_database_only() {
    let f = Fixture::new();
    f.row("main", " /workspace/current/ ", Some("cli"), 3, false);
    let old = f.ctx.home.join("state_4.sqlite");
    Fixture::schema(&old);
    f.row_at(&old, "old", "/workspace/old", Some("cli"), 999, false);
    let p = prepare(&f.ctx).unwrap();
    assert_eq!(p.database(), Some(f.db.as_path()));
    assert!(p.before().is_none());
    assert!(p.after().is_some());
    assert_eq!(p.diagnostic.status, "error");
    assert!(!p.state.exists());
    let out = f.apply(&p);
    assert!(out.success, "{}", out.detail);
    assert!(out.state_written);
    assert_eq!(out.cwd_updated, 0);
    assert_eq!(
        f.value()["electron-saved-workspace-roots"],
        json!(["/workspace/current"])
    );
    assert!(f.value()["thread-workspace-root-hints"]
        .get("old")
        .is_none());
}
#[test]
fn native_repair_merges_existing_navigation_and_overrides_db_hint_values() {
    let f = Fixture::new();
    f.state(json!({"theme":"keep","electron-saved-workspace-roots":[" /pinned/ ","/pinned"],"project-order":["/orphan-order"],"active-workspace-roots":["/gone","/pinned","/pinned"],"thread-workspace-root-hints":{},"projectless-thread-ids":["excluded"]}));
    f.row("a", "/actual", Some("cli"), 3, false);
    f.row("excluded", "/existing-projectless-cwd", None, 4, false);
    // Empty cwd is the independent native repair trigger even with existing navigation.
    f.row("b", "", Some("cli"), 2, true);
    let p = prepare(&f.ctx).unwrap();
    let original = fs::read(p.state_path()).unwrap();
    let out = f.apply(&p);
    assert!(out.success, "{}", out.detail);
    assert_eq!(out.cwd_updated, 1);
    let after = f.value();
    assert_eq!(after["theme"], "keep");
    assert_eq!(
        after["project-order"],
        json!([
            "/orphan-order",
            "/pinned",
            "/actual",
            "/existing-projectless-cwd",
            "/restored/b"
        ])
    );
    assert_eq!(
        after["active-workspace-roots"],
        json!(["/pinned", "/pinned"])
    );
    assert_eq!(
        after["thread-workspace-root-hints"]["excluded"],
        "/existing-projectless-cwd"
    );
    assert_eq!(after["projectless-thread-ids"], json!(["excluded"]));
    assert_eq!(f.cwd("b"), "/restored/b");
    assert!(out
        .backups
        .iter()
        .any(|p| fs::read(p).is_ok_and(|b| b == original)));
}
#[test]
fn native_backfill_source_gate_is_quoted_subagent_only_and_preserves_raw_relative_paths() {
    let f = Fixture::new();
    f.state(json!({"projectless-thread-ids":["excluded"]}));
    let source = [
        ("normal", Some("cli")),
        ("unknown", Some("cli")),
        ("plain", Some("subagent")),
        ("nested", Some("{\"subagent\":{}}")),
        ("excluded", Some("cli")),
    ];
    let mut originals = vec![];
    for (id, source) in source {
        let p = f.row(id, "", source, 1, id == "unknown");
        originals.push((p.clone(), fs::read(p).unwrap()));
    }
    let plan = prepare(&f.ctx).unwrap();
    assert_eq!(
        plan.updates
            .iter()
            .map(|u| u.id.as_str())
            .collect::<BTreeSet<_>>(),
        ["normal", "unknown", "plain"].into()
    );
    let out = f.apply(&plan);
    assert!(out.success, "{}", out.detail);
    assert_eq!(out.cwd_updated, 3);
    assert_eq!(f.cwd("nested"), "");
    assert_eq!(f.cwd("excluded"), "");
    let conn = Connection::open(&f.db).unwrap();
    let raw: String = conn
        .query_row(
            "SELECT rollout_path FROM threads WHERE id='unknown'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(raw, "sessions/unknown.jsonl");
    for (p, b) in originals {
        assert_eq!(fs::read(p).unwrap(), b);
    }
}
#[test]
fn generated_projectless_cwd_stays_a_hint_but_not_a_new_saved_root() {
    let f = Fixture::new();
    f.row(
        "scratch",
        "/Users/demo/Documents/Codex/2026-11-08",
        Some("cli"),
        1,
        false,
    );
    f.row("project", "relative/project/", None, 2, false);
    let plan = prepare(&f.ctx).unwrap();
    let out = f.apply(&plan);
    assert!(out.success, "{}", out.detail);
    assert_eq!(
        f.value()["electron-saved-workspace-roots"],
        json!(["relative/project"])
    );
    assert_eq!(
        f.value()["thread-workspace-root-hints"]["scratch"],
        "/Users/demo/Documents/Codex/2026-11-08"
    );
}
#[test]
fn invalid_json_is_backed_up_and_rebuilt_but_no_roots_cannot_create_an_empty_repair() {
    let f = Fixture::new();
    fs::write(f.ctx.home.join(".codex-global-state.json"), b"broken JSON").unwrap();
    f.row("project", "/actual", None, 2, false);
    let plan = prepare(&f.ctx).unwrap();
    assert_eq!(plan.diagnostic.status, "error");
    let out = f.apply(&plan);
    assert!(out.success, "{}", out.detail);
    assert!(out
        .backups
        .iter()
        .any(|p| fs::read(p).is_ok_and(|b| b == b"broken JSON")));
    let empty = Fixture::new();
    fs::write(empty.ctx.home.join(".codex-global-state.json"), b"[]").unwrap();
    let p = prepare(&empty.ctx).unwrap();
    assert!(p.after.is_none());
    let out = empty.apply(&p);
    assert!(!out.success);
    assert!(!out.state_written);
    assert_eq!(fs::read(p.state_path()).unwrap(), b"[]");
}
#[test]
fn missing_timestamp_columns_retain_the_native_order_by_zero_error() {
    let f = Fixture::new();
    Connection::open(&f.db)
        .unwrap()
        .execute_batch("DROP TABLE threads; CREATE TABLE threads(id TEXT,cwd TEXT)")
        .unwrap();
    let checked = inspect(&f.ctx);
    assert_eq!(checked.status, "warning");
    assert!(!checked.fixable);
    assert!(checked.detail.contains("线程库"));
    assert!(!f.ctx.home.join(".codex-global-state.json").exists());
}
#[test]
fn source_database_and_project_preview_never_create_source_shm_or_backups() {
    let f = Fixture::new();
    f.row("project", "/actual", Some("cli"), 1, false);
    let conn = Connection::open(&f.db).unwrap();
    conn.execute_batch("PRAGMA journal_mode=WAL;PRAGMA wal_autocheckpoint=0;PRAGMA wal_checkpoint(TRUNCATE);UPDATE threads SET updated_at_ms=8").unwrap();
    let db = fs::read(&f.db).unwrap();
    let w = fs::read(wal(&f.db)).unwrap();
    let shm = f.db.with_extension("sqlite-shm");
    fs::remove_file(&shm).unwrap();
    let plan = prepare(&f.ctx).unwrap();
    assert_eq!(plan.diagnostic.status, "error");
    assert!(!shm.exists());
    assert_eq!(fs::read(&f.db).unwrap(), db);
    assert_eq!(fs::read(wal(&f.db)).unwrap(), w);
    assert!(!f.root.join("ticket-backup").exists());
}
#[test]
fn changed_rollout_database_and_state_reject_stale_plans_before_any_write() {
    for target in 0..3 {
        let f = Fixture::new();
        f.state(json!({"keep":true}));
        let path = f.row("a", "", Some("cli"), 1, false);
        let p = prepare(&f.ctx).unwrap();
        match target {
            0 => fs::write(&path, b"external rollout").unwrap(),
            1 => {
                Connection::open(&f.db)
                    .unwrap()
                    .execute("UPDATE threads SET cwd='/external'", [])
                    .unwrap();
            }
            _ => fs::write(p.state_path(), b"{\"external\":true}").unwrap(),
        }
        let before = fs::read(p.state_path()).unwrap();
        let guard = super::super::repair::acquire(&f.ctx).unwrap();
        assert!(apply_under_manager(&f.ctx, &p, &guard, &f.root.join("ticket-backup")).is_err());
        assert_eq!(fs::read(p.state_path()).unwrap(), before);
        assert!(!f.root.join("ticket-backup").exists());
    }
}
#[test]
fn database_trigger_failure_rolls_back_cwd_before_global_json_publication() {
    let f = Fixture::new();
    f.state(json!({"keep":true}));
    f.row("a", "", Some("cli"), 1, false);
    Connection::open(&f.db).unwrap().execute_batch("CREATE TRIGGER reject_cwd BEFORE UPDATE OF cwd ON threads BEGIN SELECT RAISE(ABORT,'fixture failure'); END").unwrap();
    let plan = prepare(&f.ctx).unwrap();
    let before = fs::read(plan.state_path()).unwrap();
    let out = f.apply(&plan);
    assert!(!out.success);
    assert_eq!(out.cwd_updated, 0);
    assert!(!out.state_written);
    assert!(!out.partial_effects_possible);
    assert_eq!(f.cwd("a"), "");
    assert_eq!(fs::read(plan.state_path()).unwrap(), before);
}

#[test]
fn external_json_change_after_cwd_commit_is_preserved_and_reports_real_partial_effects() {
    let f = Fixture::new();
    f.state(json!({"keep":true}));
    f.row("a", "", Some("cli"), 1, false);
    let plan = prepare(&f.ctx).unwrap();
    let guard = super::super::repair::acquire(&f.ctx).unwrap();
    let external = b"{\"external-after-commit\":true}";
    let out = apply_with_hook(&f.ctx, &plan, &guard, &f.root.join("ticket-backup"), || {
        fs::write(plan.state_path(), external).unwrap();
    })
    .unwrap();
    assert!(!out.success);
    assert!(out.partial_effects_possible);
    assert_eq!(out.cwd_updated, 1);
    assert!(!out.state_written);
    assert_eq!(f.cwd("a"), "/restored/a");
    assert_eq!(fs::read(plan.state_path()).unwrap(), external);
    assert!(!out.backups.is_empty());
}

#[test]
fn unexpected_trigger_project_effects_are_not_signed_as_reviewed_roots() {
    let f = Fixture::new();
    f.state(json!({"keep":true}));
    f.row("a", "", Some("cli"), 1, false);
    f.row("b", "/initial", Some("cli"), 2, false);
    Connection::open(&f.db).unwrap().execute_batch("CREATE TRIGGER extra_cwd AFTER UPDATE OF cwd ON threads WHEN NEW.id='a' BEGIN UPDATE threads SET cwd='/unreviewed' WHERE id='b'; END").unwrap();
    let plan = prepare(&f.ctx).unwrap();
    let before = fs::read(plan.state_path()).unwrap();
    let out = f.apply(&plan);
    assert!(!out.success);
    assert!(out.partial_effects_possible);
    assert_eq!(out.cwd_updated, 1);
    assert!(!out.state_written);
    assert_eq!(f.cwd("b"), "/unreviewed");
    assert_eq!(fs::read(plan.state_path()).unwrap(), before);
}
#[test]
fn existing_root_hints_values_are_preserved_except_actual_database_overwrites() {
    let records = project::ProjectRecords {
        roots: vec!["/actual".into()],
        hints: [("same".into(), "/actual".into())].into(),
        ..Default::default()
    };
    let before=br#"{"electron-saved-workspace-roots":["/old"],"thread-workspace-root-hints":{"same":"/stale","warning":42}}"#;
    let composed = project::compose_project_state(Some(before), &records).unwrap();
    let after: Value = serde_json::from_slice(composed.after.as_ref().unwrap()).unwrap();
    assert_eq!(after["thread-workspace-root-hints"]["same"], "/actual");
    assert_eq!(after["thread-workspace-root-hints"]["warning"], 42);
}
#[test]
fn single_active_database_selection_obeys_sqlite_home_and_native_score_priority() {
    let mut f = Fixture::new();
    f.row("root", "/root", Some("cli"), 900, false);
    let sqlite = f.ctx.home.join("sqlite/state_5.sqlite");
    Fixture::schema(&sqlite);
    f.row_at(&sqlite, "sqlite", "/sqlite", Some("cli"), 2, false);
    assert_eq!(active_database(&f.ctx).unwrap(), Some(f.db.clone()));
    Connection::open(&sqlite)
        .unwrap()
        .execute("UPDATE threads SET updated_at_ms=1000", [])
        .unwrap();
    assert_eq!(active_database(&f.ctx).unwrap(), Some(sqlite.clone()));
    let v6 = f.ctx.home.join("state_6.sqlite");
    Fixture::schema(&v6);
    f.row_at(&v6, "new", "/version6", Some("cli"), 0, false);
    assert_eq!(active_database(&f.ctx).unwrap(), Some(v6));
    fs::write(
        f.ctx.config(),
        format!("sqlite_home={:?}\n", sqlite.parent().unwrap()),
    )
    .unwrap();
    assert_eq!(active_database(&f.ctx).unwrap(), Some(sqlite));
    f.ctx.sqlite_home_env = Some(f.ctx.home.clone().into_os_string());
    assert_eq!(
        active_database(&f.ctx).unwrap().unwrap().parent(),
        Some(f.ctx.home.join("sqlite").as_path())
    );
}

#[test]
fn semantic_probe_failure_does_not_outrank_a_real_sqlite_candidate() {
    let f = Fixture::new();
    let sqlite = f.ctx.home.join("sqlite/state_5.sqlite");
    Fixture::schema(&sqlite);
    f.row_at(&sqlite, "valid", "/real", Some("cli"), 1, false);
    let corrupt = f.ctx.home.join("state_9.sqlite");
    fs::write(&corrupt, b"not a SQLite database").unwrap();
    assert_eq!(active_database(&f.ctx).unwrap(), Some(sqlite.clone()));
    let plan = prepare(&f.ctx).unwrap();
    assert_eq!(plan.database(), Some(sqlite.as_path()));
    let outcome = f.apply(&plan);
    assert!(outcome.success, "{}", outcome.detail);
    assert_eq!(fs::read(corrupt).unwrap(), b"not a SQLite database");
    assert_eq!(
        f.value()["electron-saved-workspace-roots"],
        json!(["/real"])
    );
}

#[test]
fn manager_apply_keeps_actual_desktop_and_gateway_request_guards() {
    for desktop in [true, false] {
        let mut f = Fixture::new();
        f.row("a", "/actual", Some("cli"), 1, false);
        let plan = prepare(&f.ctx).unwrap();
        let guard = super::super::repair::acquire(&f.ctx).unwrap();
        f.ctx.desktop_running = desktop;
        if !desktop {
            f.ctx.runtime = Some(crate::gateway::types::RuntimeView {
                active_requests: 1,
                ..Default::default()
            });
        }
        assert!(
            apply_under_manager(&f.ctx, &plan, &guard, &f.root.join("ticket-backup"))
                .unwrap_err()
                .contains("静默窗口")
        );
        assert!(!plan.state_path().exists());
        assert!(!f.root.join("ticket-backup").exists());
    }
}

fn add_native_thread_columns(f: &Fixture, provider: &str) {
    let connection = Connection::open(&f.db).unwrap();
    connection.execute_batch("ALTER TABLE threads ADD COLUMN model_provider TEXT DEFAULT 'openai';ALTER TABLE threads ADD COLUMN model TEXT DEFAULT 'fixture-model'").unwrap();
    connection
        .execute("UPDATE threads SET model_provider=?1", [provider])
        .unwrap();
}
#[test]
fn approved_provider_step_then_project_uses_original_origins_and_forwards_real_project_effects() {
    let f = Fixture::new();
    f.state(json!({"keep":true}));
    let history = f.row("a", "", Some("cli"), 1, false);
    add_native_thread_columns(&f, "aimai1");
    let mut meta: Value = serde_json::from_slice(
        fs::read(&history)
            .unwrap()
            .split(|b| *b == b'\n')
            .next()
            .unwrap(),
    )
    .unwrap();
    meta["payload"]["model_provider"] = json!("aimai1");
    fs::write(&history, format!("{meta}\n")).unwrap();
    let project = prepare(&f.ctx).unwrap();
    let plans = super::super::native_threads::prepare_sequence(
        &f.ctx,
        &["thread_provider_consistency".into()],
        &f.root.join("thread-backup"),
    )
    .unwrap();
    validate(&f.ctx, &project).unwrap();
    super::super::native_threads::validate_sequence(&f.ctx, &plans).unwrap();
    let mut session = super::super::native_threads::begin_reviewed(
        &f.ctx,
        &plans,
        &super::super::native_orphans::ApprovedEffects::default(),
    )
    .unwrap();
    let thread = super::super::native_threads::apply_item(
        &f.ctx,
        &mut session,
        "thread_provider_consistency",
    )
    .unwrap();
    assert!(thread.success, "{}", thread.detail);
    assert!(
        validate(&f.ctx, &project).is_err(),
        "original DB bytes must really differ"
    );
    let guard = super::super::repair::acquire(&f.ctx).unwrap();
    let outcome = apply_after_approved_prefix(
        &f.ctx,
        &project,
        &guard,
        &f.root.join("project-backup"),
        session.effects(),
    )
    .unwrap();
    assert!(outcome.success, "{}", outcome.detail);
    assert_eq!(outcome.cwd_updated, 1);
    assert_eq!(f.cwd("a"), "/restored/a");
    assert!(session.effects().validate_current(&f.ctx).is_err());
    super::super::native_threads::absorb_project(
        &f.ctx,
        &mut session,
        outcome.effects.as_ref().unwrap(),
    )
    .unwrap();
    session.effects().validate_current(&f.ctx).unwrap();
    assert_eq!(f.value()["keep"], true);
    assert_eq!(f.value()["thread-workspace-root-hints"]["a"], "/restored/a");
}
#[test]
fn exact_missing_history_restoration_can_supply_a_previously_unavailable_project_cwd() {
    let f = Fixture::new();
    f.state(json!([]));
    let history = f.row("a", "", Some("cli"), 1, true);
    let source_bytes = fs::read(&history).unwrap();
    fs::remove_file(&history).unwrap();
    // Native trusted recovery matches the recorded filename exactly. A compressed
    // backup restores a recorded .jsonl.zst target, retaining that representation.
    Connection::open(&f.db)
        .unwrap()
        .execute(
            "UPDATE threads SET rollout_path='sessions/a.jsonl.zst' WHERE id='a'",
            [],
        )
        .unwrap();
    let source = f.ctx.home.join("codexmate/backups/a.jsonl.zst");
    fs::create_dir_all(source.parent().unwrap()).unwrap();
    fs::write(&source, &source_bytes).unwrap();
    add_native_thread_columns(&f, "openai");
    let project = prepare(&f.ctx).unwrap();
    assert!(project.after().is_none());
    assert!(project.diagnostic().fixable);
    let plans = super::super::native_threads::prepare_sequence(
        &f.ctx,
        &["thread_missing_rollouts".into()],
        &f.root.join("thread-backup"),
    )
    .unwrap();
    validate(&f.ctx, &project).unwrap();
    let mut session = super::super::native_threads::begin_reviewed(
        &f.ctx,
        &plans,
        &super::super::native_orphans::ApprovedEffects::default(),
    )
    .unwrap();
    let restored =
        super::super::native_threads::apply_item(&f.ctx, &mut session, "thread_missing_rollouts")
            .unwrap();
    assert!(restored.success, "{}", restored.detail);
    let guard = super::super::repair::acquire(&f.ctx).unwrap();
    let outcome = apply_after_approved_prefix(
        &f.ctx,
        &project,
        &guard,
        &f.root.join("project-backup"),
        session.effects(),
    )
    .unwrap();
    assert!(outcome.success, "{}", outcome.detail);
    assert_eq!(f.cwd("a"), "/restored/a");
    assert_eq!(
        f.value()["electron-saved-workspace-roots"],
        json!(["/restored/a"])
    );
    assert_eq!(fs::read(history).unwrap(), source_bytes);
    super::super::native_threads::absorb_project(
        &f.ctx,
        &mut session,
        outcome.effects.as_ref().unwrap(),
    )
    .unwrap();
    session.effects().validate_current(&f.ctx).unwrap();
}
#[test]
fn project_prefix_rejects_unapproved_new_rows_and_external_after_revisions() {
    for new_row in [false, true] {
        let f = Fixture::new();
        f.state(json!({"keep":true}));
        let history = f.row("a", "", Some("cli"), 1, false);
        add_native_thread_columns(&f, "openai");
        let project = prepare(&f.ctx).unwrap();
        let session = super::super::native_threads::begin_reviewed(
            &f.ctx,
            &[],
            &super::super::native_orphans::ApprovedEffects::default(),
        )
        .unwrap();
        if new_row {
            Connection::open(&f.db).unwrap().execute("INSERT INTO threads(id,cwd,rollout_path,archived,source,updated_at_ms,model_provider,model) VALUES('new','/unapproved','sessions/new.jsonl',0,'cli',2,'openai','fixture-model')",[]).unwrap();
        } else {
            fs::write(
                &history,
                format!(
                    "{}\n",
                    json!({"type":"session_meta","payload":{"id":"a","cwd":"/unapproved"}})
                ),
            )
            .unwrap();
        }
        let before = fs::read(project.state_path()).unwrap();
        let guard = super::super::repair::acquire(&f.ctx).unwrap();
        assert!(apply_after_approved_prefix(
            &f.ctx,
            &project,
            &guard,
            &f.root.join("project-backup"),
            session.effects()
        )
        .is_err());
        assert_eq!(fs::read(project.state_path()).unwrap(), before);
        assert_eq!(f.cwd("a"), "");
        assert!(!f.root.join("project-backup").exists());
    }
}

#[test]
fn account_scoped_projects_are_not_rebuilt_as_legacy_global_roots() {
    let f = Fixture::new();
    f.row("one", "/project", None, 1, false);
    f.state(json!({
        "electron-owner-value:electron-saved-workspace-roots:account-a": ["/project"],
        "electron-owner-value:thread-workspace-root-hints:account-a": {"one": "/project"}
    }));
    let path = f.ctx.home.join(".codex-global-state.json");
    let before = fs::read(&path).unwrap();
    let item = inspect(&f.ctx);
    assert_eq!(item.status, "warning");
    assert!(!item.fixable);
    assert!(prepare(&f.ctx).err().unwrap().contains("账号隔离"));
    assert_eq!(fs::read(path).unwrap(), before);
}

fn account_fixture(f: &Fixture) -> (Value, String) {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
    let claims = json!({"https://api.openai.com/auth": {"user_id":"principal", "chatgpt_user_id":"different", "chatgpt_account_id":"account"}});
    let token = format!("header.{}.signature", URL_SAFE_NO_PAD.encode(serde_json::to_vec(&claims).unwrap()));
    fs::write(f.ctx.home.join("auth.json"), serde_json::to_vec(&json!({"tokens":{"access_token":token}})).unwrap()).unwrap();
    let owner = format!("{:x}", Sha256::digest(br#"["principal","account"]"#));
    let state = json!({
        "electron-renderer-content-owner":owner,
        format!("electron-owner-value:electron-saved-workspace-roots:{owner}"): ["/workspace"],
        format!("electron-owner-value:thread-workspace-root-hints:{owner}"): {"thread":"/workspace"},
        "electron-owner-value:electron-saved-workspace-roots:other-account": false
    });
    (state, owner)
}
#[test]
fn account_owner_uses_authenticated_principal_and_does_not_merge_other_accounts() {
    let f = Fixture::new();
    let (state, _) = account_fixture(&f);
    f.state(state);
    assert_eq!(inspect(&f.ctx).status, "ok");
    assert!(prepare(&f.ctx).is_err());
}
#[test]
fn account_mismatch_and_corrupt_scoped_fields_are_not_healthy() {
    for mode in 0..4 {
        let f = Fixture::new();
        let (mut state, owner) = account_fixture(&f);
        match mode {
            0 => state["electron-renderer-content-owner"] = json!("another"),
            1 => state[format!("electron-owner-value:electron-saved-workspace-roots:{owner}")] = json!([42]),
            2 => state[format!("electron-owner-value:thread-workspace-root-hints:{owner}")] = json!([]),
            _ => { fs::remove_file(f.ctx.home.join("auth.json")).unwrap(); }
        }
        f.state(state);
        let item = inspect(&f.ctx);
        assert_eq!(item.status, "warning");
        assert!(!item.fixable);
    }
}
#[test]
fn migrated_projects_allow_pending_assignments_but_reject_dangling_database_roots() {
    for corrupt in [false, true] {
        let f = Fixture::new();
        let (mut state, owner) = account_fixture(&f);
        state[format!("electron-owner-value:app-server-projects-migration-by-host:{owner}")] = json!({format!("local:{}",f.ctx.home.display()):{"version":1,"projectsMigrated":true,"threadAssignmentsMigrated":false,"pendingThreadAssignmentIds":["pending"]}});
        f.state(state);
        let c = Connection::open(&f.db).unwrap();
        c.execute_batch("ALTER TABLE threads ADD COLUMN project_id TEXT; CREATE TABLE projects(id TEXT PRIMARY KEY,name TEXT,metadata TEXT); CREATE TABLE project_roots(project_id TEXT,path TEXT); INSERT INTO projects VALUES('p','Project','{}'); INSERT INTO project_roots VALUES('p','/workspace');").unwrap();
        if corrupt { c.execute("UPDATE project_roots SET project_id='missing'", []).unwrap(); }
        assert_eq!(inspect(&f.ctx).status, if corrupt {"warning"} else {"ok"});
    }
}

#[test]
fn account_unknown_migration_schema_and_missing_project_tables_stay_warning() {
    for version in [1, 2] {
        let f = Fixture::new();
        let (mut state, owner) = account_fixture(&f);
        state[format!("electron-owner-value:app-server-projects-migration-by-host:{owner}")] = json!({format!("local:{}", f.ctx.home.display()):{"version":version,"projectsMigrated":true}});
        f.state(state);
        let before = fs::read(f.ctx.home.join(".codex-global-state.json")).unwrap();
        assert_eq!(inspect(&f.ctx).status, "warning");
        assert!(prepare(&f.ctx).is_err());
        assert_eq!(fs::read(f.ctx.home.join(".codex-global-state.json")).unwrap(), before);
    }
}
