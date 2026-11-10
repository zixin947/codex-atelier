use atelier_maintenance_core::{
    catalog,
    threads::{self, repair},
};
use repair::*;
use rusqlite::{params, Connection};
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
};
fn database(root: &Path) -> PathBuf {
    let p = root.join("state.db");
    let c = Connection::open(&p).unwrap();
    c.execute_batch("CREATE TABLE threads(id TEXT PRIMARY KEY,model_provider TEXT NOT NULL,rollout_path TEXT,model TEXT,archived INTEGER,source TEXT)").unwrap();
    p
}
fn insert(db: &Path, id: &str, provider: &str, path: Option<&Path>, model: Option<&str>) {
    Connection::open(db)
        .unwrap()
        .execute(
            "INSERT INTO threads VALUES(?1,?2,?3,?4,0,'cli')",
            params![id, provider, path.map(|p| p.to_str().unwrap()), model],
        )
        .unwrap();
}
fn meta(id: &str, p: &str, m: &str) -> Value {
    json!({"type":"session_meta","payload":{"id":id,"model_provider":p,"model":m,"keep":{"opaque":1}}})
}
fn write_rows(path: &Path, rows: &[Value]) {
    let mut bytes = vec![];
    for row in rows {
        bytes.extend(serde_json::to_vec(row).unwrap());
        bytes.extend(b"\r\n")
    }
    if path.to_string_lossy().ends_with(".zst") {
        fs::write(path, zstd::stream::encode_all(bytes.as_slice(), 3).unwrap()).unwrap()
    } else {
        fs::write(path, bytes).unwrap()
    }
}
fn read_rows(path: &Path) -> Vec<Value> {
    let b = fs::read(path).unwrap();
    let b = if path.to_string_lossy().ends_with(".zst") {
        zstd::stream::decode_all(b.as_slice()).unwrap()
    } else {
        b
    };
    b.split(|b| *b == b'\n')
        .filter_map(|s| serde_json::from_slice(s).ok())
        .collect()
}
#[test]
fn provider_changes_boundary_anchors_and_native_only() {
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    let p = t.path().join("a.jsonl");
    let native = json!({"type":"event_msg","payload":{"type":"thread_settings_applied","thread_id":"a","thread_settings":{"model_provider_id":"aimami_relay_p","opaque":true}}});
    write_rows(
        &p,
        &[
            meta("a", "aimami_relay_p", "x"),
            meta("a", "aimami_relay_middle", "x"),
            meta("a", "aimami_relay_p", "y"),
            native,
        ],
    );
    insert(&db, "a", "aimami_relay_p", Some(&p), Some("y"));
    let before = fs::read(&p).unwrap();
    let plan = plan_provider_convergence(
        &[db.clone()],
        ProviderTarget::Official,
        &t.path().join("backup"),
    )
    .unwrap();
    assert_eq!(fs::read(&p).unwrap(), before);
    let result = apply_thread_write_repair(&plan).unwrap();
    assert_eq!((result.db_updated, result.rollout_updated), (1, 1));
    let r = read_rows(&p);
    assert_eq!(r[0]["payload"]["model_provider"], "openai");
    assert_eq!(r[1]["payload"]["model_provider"], "aimami_relay_middle");
    assert_eq!(r[2]["payload"]["model_provider"], "openai");
    assert_eq!(
        r[3]["payload"]["thread_settings"]["model_provider_id"],
        "openai"
    );
    assert_eq!(r[2]["payload"]["keep"]["opaque"], 1);
    assert_eq!(
        Connection::open(db)
            .unwrap()
            .query_row("SELECT model_provider FROM threads", [], |r| r
                .get::<_, String>(0))
            .unwrap(),
        "openai"
    );
    assert!(result.backups.iter().all(|p| p.is_file()));
}
#[test]
fn compressed_file_keeps_representation_and_unrelated_bytes() {
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    let requested = t.path().join("a.jsonl");
    let physical = t.path().join("a.jsonl.zst");
    write_rows(&physical, &[meta("a", "openai", "x")]);
    insert(&db, "a", "openai", Some(&requested), Some("x"));
    let plan =
        plan_provider_convergence(&[db], ProviderTarget::Router, &t.path().join("backup")).unwrap();
    apply_thread_write_repair(&plan).unwrap();
    assert!(!requested.exists());
    assert_eq!(
        read_rows(&physical)[0]["payload"]["model_provider"],
        "aimai1"
    );
}
#[test]
fn foreign_native_and_mismatched_identity_preserve_history() {
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    let p = t.path().join("a.jsonl");
    write_rows(
        &p,
        &[
            meta("a", "openai", "x"),
            json!({"type":"event_msg","payload":{"type":"thread_settings_applied","thread_settings":{"model_provider_id":"enterprise"}}}),
        ],
    );
    insert(&db, "a", "openai", Some(&p), Some("x"));
    let before = fs::read(&p).unwrap();
    let plan = plan_provider_convergence(&[db.clone()], ProviderTarget::Router, t.path()).unwrap();
    assert!(plan.db_changes.is_empty());
    assert!(plan.files.is_empty());
    assert!(plan.warnings[0].contains("externally owned"));
    write_rows(&p, &[meta("different", "openai", "x")]);
    assert!(
        plan_provider_convergence(&[db], ProviderTarget::Router, t.path())
            .unwrap_err()
            .0
            .contains("identity")
    );
    assert_ne!(fs::read(&p).unwrap(), before);
}
#[test]
fn stale_file_is_rejected_before_db_write() {
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    let p = t.path().join("a.jsonl");
    write_rows(&p, &[meta("a", "openai", "x")]);
    insert(&db, "a", "openai", Some(&p), Some("x"));
    let plan = plan_provider_convergence(
        &[db.clone()],
        ProviderTarget::Router,
        &t.path().join("backups"),
    )
    .unwrap();
    fs::write(&p, b"external edit\n").unwrap();
    assert!(apply_thread_write_repair(&plan)
        .unwrap_err()
        .0
        .contains("changed after"));
    assert_eq!(
        Connection::open(db)
            .unwrap()
            .query_row("SELECT model_provider FROM threads", [], |r| r
                .get::<_, String>(0))
            .unwrap(),
        "openai"
    );
    assert_eq!(fs::read(&p).unwrap(), b"external edit\n");
}
#[test]
fn provider_sql_failure_rolls_back_rewritten_file_and_database() {
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    let p = t.path().join("a.jsonl");
    write_rows(&p, &[meta("a", "openai", "x")]);
    insert(&db, "a", "openai", Some(&p), Some("x"));
    let original = fs::read(&p).unwrap();
    Connection::open(&db).unwrap().execute_batch("CREATE TRIGGER reject_provider BEFORE UPDATE OF model_provider ON threads BEGIN SELECT RAISE(ABORT,'fixture denied'); END").unwrap();
    let plan = plan_provider_convergence(
        &[db.clone()],
        ProviderTarget::Router,
        &t.path().join("backups"),
    )
    .unwrap();
    assert!(apply_thread_write_repair(&plan)
        .unwrap_err()
        .0
        .contains("fixture denied"));
    assert_eq!(fs::read(&p).unwrap(), original);
    assert_eq!(
        Connection::open(db)
            .unwrap()
            .query_row("SELECT model_provider FROM threads", [], |r| r
                .get::<_, String>(0))
            .unwrap(),
        "openai"
    );
}
fn catalog(root: &Path, no_account: bool) {
    fs::write(root.join("codex_router_catalog.json"),serde_json::to_vec(&json!({"aimami_no_account":no_account,"models":[{"slug":"aimami_relay_p::new-model"}]})).unwrap()).unwrap();
}
#[test]
fn model_updates_last_meta_and_retains_provider_and_first_meta() {
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    catalog(t.path(), false);
    let p = t.path().join("a.jsonl");
    write_rows(
        &p,
        &[
            meta("a", "aimai1", "old::missing"),
            meta("a", "aimai1", "old::missing"),
        ],
    );
    insert(&db, "a", "aimai1", Some(&p), Some("old::missing"));
    insert(&db, "official", "aimai1", None, Some("official-bare"));
    let plan =
        plan_model_catalog_convergence(&[db.clone()], t.path(), &t.path().join("backups")).unwrap();
    assert_eq!(plan.db_changes.len(), 1);
    let result = apply_thread_write_repair(&plan).unwrap();
    assert_eq!(result.db_updated, 1);
    let rows = read_rows(&p);
    assert_eq!(rows[0]["payload"]["model"], "old::missing");
    assert_eq!(rows[1]["payload"]["model"], "aimami_relay_p::new-model");
    assert_eq!(rows[1]["payload"]["model_provider"], "aimai1");
}
#[test]
fn model_layout_failure_warns_but_commits_db_model() {
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    catalog(t.path(), false);
    let p = t.path().join("a.jsonl");
    let mut r = meta("a", "aimai1", "x::x");
    r["ordinal"] = json!(0);
    write_rows(&p, &[r]);
    insert(&db, "a", "aimai1", Some(&p), Some("x::x"));
    let before = fs::read(&p).unwrap();
    let plan =
        plan_model_catalog_convergence(&[db.clone()], t.path(), &t.path().join("backups")).unwrap();
    assert!(plan.files.is_empty());
    assert!(plan.warnings[0].contains("ROUTER_HISTORY_LAYOUT_PROTECTED"));
    let result = apply_thread_write_repair(&plan).unwrap();
    assert_eq!(result.db_updated, 1);
    assert_eq!(fs::read(&p).unwrap(), before);
    assert_eq!(
        Connection::open(db)
            .unwrap()
            .query_row("SELECT model FROM threads", [], |r| r.get::<_, String>(0))
            .unwrap(),
        "aimami_relay_p::new-model"
    );
}
#[test]
fn protected_records_pad_exact_utf8_bytes() {
    let original = br#"{"ordinal":null,"payload":{"model":"long-long"}}   "#;
    let patched = fit_record(
        original,
        br#"{"ordinal":null,"payload":{"model":"x"}}"#.to_vec(),
    )
    .unwrap();
    assert_eq!(patched.len(), original.len());
    assert_eq!(
        serde_json::from_slice::<Value>(&patched).unwrap()["payload"]["model"],
        "x"
    );
    assert!(fit_record(
        br#"{"ordinal":0}"#,
        br#"{"ordinal":0,"new":"data"}"#.to_vec()
    )
    .is_err());
}
#[test]
fn missing_repair_restores_identity_proven_backup_archives_only_unrecoverable() {
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    let trusted = t.path().join("trusted");
    fs::create_dir(&trusted).unwrap();
    let dest = t.path().join("sessions").join("a.jsonl");
    let source = trusted.join("a.jsonl");
    write_rows(&source, &[meta("a", "aimai1", "x")]);
    insert(&db, "a", "aimai1", Some(&dest), Some("x"));
    let missing = t.path().join("absent.jsonl");
    insert(&db, "b", "openai", Some(&missing), Some("x"));
    insert(&db, "foreign", "enterprise", Some(&missing), Some("x"));
    let plan =
        plan_missing_rollout_repair(&[db.clone()], &trusted, &t.path().join("backups")).unwrap();
    assert_eq!(plan.changes.len(), 2);
    assert!(!dest.exists());
    let result = apply_missing_rollout_repair(&plan).unwrap();
    assert_eq!((result.restored, result.archived), (1, 1));
    assert_eq!(fs::read(dest).unwrap(), fs::read(source).unwrap());
    let c = Connection::open(db).unwrap();
    let rows = c
        .prepare("SELECT id,archived FROM threads ORDER BY id")
        .unwrap()
        .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?)))
        .unwrap()
        .collect::<std::result::Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        rows,
        vec![("a".into(), 0), ("b".into(), 1), ("foreign".into(), 0)]
    );
}
#[test]
fn no_account_journal_is_private_sorted_and_preserves_original_across_borrow_chain() {
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    catalog(t.path(), true);
    insert(&db, "z", "aimai1", None, Some("old::missing"));
    insert(&db, "a", "aimai1", None, None);
    let path = t.path().join("router_no_account_model_restore.json");
    fs::write(&path,serde_json::to_vec(&json!({"version":1,"entries":[{"thread_id":"z","original_model":"user-model","fallback_model":"old::missing"}]})).unwrap()).unwrap();
    assert!(load_model_restore_journal(t.path()).is_err());
    fs::write(&path,serde_json::to_vec(&json!({"version":2,"entries":[{"thread_id":"z","original_model":"user-model","borrowed_model":"aimami_relay_old::missing"}]})).unwrap()).unwrap();
    Connection::open(&db)
        .unwrap()
        .execute(
            "UPDATE threads SET model='aimami_relay_old::missing' WHERE id='z'",
            [],
        )
        .unwrap();
    let plan = plan_model_catalog_convergence(&[db], t.path(), &t.path().join("backups")).unwrap();
    assert!(plan.model_restore_journal.is_some());
    apply_thread_write_repair(&plan).unwrap();
    let journal = load_model_restore_journal(t.path()).unwrap();
    assert_eq!(journal.version, 2);
    assert_eq!(
        journal
            .entries
            .iter()
            .map(|e| e.thread_id.as_str())
            .collect::<Vec<_>>(),
        vec!["a", "z"]
    );
    assert_eq!(
        journal.entries[1].original_model.as_deref(),
        Some("user-model")
    );
    assert_eq!(
        journal.entries[1].borrowed_model,
        "aimami_relay_p::new-model"
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            fs::metadata(path).unwrap().permissions().mode() & 0o777,
            0o600
        );
    }
}
#[test]
fn invalid_journal_rejected_before_model_writes() {
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    catalog(t.path(), true);
    insert(&db, "a", "aimai1", None, Some("old::missing"));
    fs::write(t.path().join("router_no_account_model_restore.json"),br#"{"version":2,"entries":[{"thread_id":"a","borrowed_model":"gpt-5.4"},{"thread_id":"a","borrowed_model":"gpt-5.5"}]}"#).unwrap();
    assert!(
        plan_model_catalog_convergence(&[db.clone()], t.path(), t.path())
            .unwrap_err()
            .0
            .contains("invalid no-account")
    );
    assert_eq!(
        Connection::open(db)
            .unwrap()
            .query_row("SELECT model FROM threads", [], |r| r.get::<_, String>(0))
            .unwrap(),
        "old::missing"
    );
}
#[test]
fn model_file_change_warns_while_db_commits() {
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    catalog(t.path(), false);
    let p = t.path().join("a.jsonl");
    write_rows(&p, &[meta("a", "aimai1", "old::missing")]);
    insert(&db, "a", "aimai1", Some(&p), Some("old::missing"));
    let plan =
        plan_model_catalog_convergence(&[db.clone()], t.path(), &t.path().join("backups")).unwrap();
    fs::write(&p, b"external edit\n").unwrap();
    let summary = apply_thread_write_repair(&plan).unwrap();
    assert_eq!(summary.db_updated, 1);
    assert_eq!(summary.rollout_updated, 0);
    assert!(summary.warnings[0].contains("patch rollout failed"));
    assert_eq!(
        Connection::open(db)
            .unwrap()
            .query_row("SELECT model FROM threads", [], |r| r.get::<_, String>(0))
            .unwrap(),
        "aimami_relay_p::new-model"
    );
    assert_eq!(fs::read(p).unwrap(), b"external edit\n");
}
#[test]
fn journal_size_version_and_path_type_are_checked() {
    let t = tempfile::tempdir().unwrap();
    let p = t.path().join("router_no_account_model_restore.json");
    fs::write(&p, br#"{"version":3,"entries":[]}"#).unwrap();
    assert!(load_model_restore_journal(t.path())
        .unwrap_err()
        .0
        .contains("unsupported"));
    let f = fs::File::create(&p).unwrap();
    f.set_len(8 * 1024 * 1024 + 1).unwrap();
    assert!(load_model_restore_journal(t.path())
        .unwrap_err()
        .0
        .contains("size limit"));
    fs::remove_file(&p).unwrap();
    fs::create_dir(&p).unwrap();
    assert!(load_model_restore_journal(t.path())
        .unwrap_err()
        .0
        .contains("path type"));
}
#[test]
fn provider_missing_history_requires_missing_rollout_repair_first() {
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    insert(&db, "a", "openai", None, None);
    assert!(
        plan_provider_convergence(&[db], ProviderTarget::Router, t.path())
            .unwrap_err()
            .0
            .contains("ROUTER_HISTORY_REPAIR_REQUIRED")
    );
}
#[test]
fn restore_revalidates_backup_identity_at_apply() {
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    let trusted = t.path().join("trusted");
    fs::create_dir(&trusted).unwrap();
    let dest = t.path().join("a.jsonl");
    let source = trusted.join("a.jsonl");
    write_rows(&source, &[meta("a", "openai", "x")]);
    insert(&db, "a", "openai", Some(&dest), Some("x"));
    let plan = plan_missing_rollout_repair(&[db], &trusted, &t.path().join("backups")).unwrap();
    write_rows(&source, &[meta("other", "openai", "x")]);
    assert!(apply_missing_rollout_repair(&plan)
        .unwrap_err()
        .0
        .contains("does not belong"));
    assert!(!dest.exists());
}
#[test]
fn response_id_mapping_exact_native_boundaries() {
    for (kind, old, new) in [
        ("tool_search_call", "ts_012345abcdef", "tsc_012345abcdef"),
        ("custom_tool_call", "ct_Ab-12_", "ctc_Ab-12_"),
        ("custom_tool_call", "fc_abc", "ctc_abc"),
    ] {
        assert_eq!(canonical_response_item_id(kind, old).as_deref(), Some(new));
    }
    for (kind, id) in [
        ("tool_search_call", "ts_012345abcdeF"),
        ("tool_search_call", "ts_0123"),
        ("tool_search_call", "tsc_012345abcdef"),
        ("custom_tool_call", "ctc_abc"),
        ("custom_tool_call", "ct_"),
        ("custom_tool_call", "fc_中文"),
        ("function_call", "fc_abc"),
    ] {
        assert_eq!(canonical_response_item_id(kind, id), None);
    }
}
#[test]
fn response_record_rewrite_preserves_raw_format_and_rejects_ambiguous_literals() {
    let row=b"{ \"type\": \"response_item\", \"payload\": {\"type\":\"custom_tool_call\", \"id\": \"fc_abc\"}, \"keep\" : [1,  2] }\r\n";
    let (out, replace) = rewrite_response_item_record(row, Path::new("fixture.jsonl"), 7)
        .unwrap()
        .unwrap();
    assert_eq!(replace.line, 7);
    assert_eq!(
        out,
        String::from_utf8(row.to_vec())
            .unwrap()
            .replace("fc_abc", "ctc_abc")
            .as_bytes()
    );
    let ambiguous=br#"{"type":"response_item","payload":{"type":"custom_tool_call","id":"ct_abc"},"other":"ct_abc"}"#;
    assert!(
        rewrite_response_item_record(ambiguous, Path::new("fixture.jsonl"), 1)
            .unwrap_err()
            .0
            .contains("ambiguous")
    );
    assert!(rewrite_response_item_record(
        b"invalid JSON custom_tool_call ct_abc",
        Path::new("fixture.jsonl"),
        1
    )
    .is_err());
    assert!(rewrite_response_item_record(
        b"unrelated malformed JSON",
        Path::new("fixture.jsonl"),
        1
    )
    .unwrap()
    .is_none());
}
#[test]
fn response_repair_writes_native_shaped_sha256_backup_and_does_not_change_db() {
    use sha2::{Digest, Sha256};
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    let p = t.path().join("a.jsonl");
    let input=b"malformed preserved\n{\"type\":\"session_meta\",\"payload\":{\"id\":\"a\"}}\n{ \"type\":\"response_item\",\"payload\":{\"type\":\"tool_search_call\",\"id\":\"ts_012345abcdef\"}}\r\n";
    fs::write(&p, input).unwrap();
    insert(&db, "a", "enterprise", Some(&p), Some("x"));
    insert(&db, "duplicate", "enterprise", Some(&p), Some("x"));
    assert_eq!(count_invalid_response_item_ids(&p).unwrap(), 1);
    let plan = plan_response_item_id_repair(&[db.clone()], &t.path().join("backups")).unwrap();
    assert_eq!(plan.scanned, 1);
    let summary = apply_response_item_id_repair(&plan).unwrap();
    assert_eq!((summary.rollout_updated, summary.db_updated), (1, 0));
    assert_eq!(count_invalid_response_item_ids(&p).unwrap(), 0);
    assert_eq!(
        fs::read(&p).unwrap(),
        String::from_utf8(input.to_vec())
            .unwrap()
            .replace("ts_012345abcdef", "tsc_012345abcdef")
            .as_bytes()
    );
    let backup: Value = serde_json::from_slice(&fs::read(&summary.backups[0]).unwrap()).unwrap();
    assert_eq!(backup["schemaVersion"], 1);
    assert_eq!(backup["sourceLength"], input.len());
    assert_eq!(
        backup["sourceSha256"],
        format!("{:x}", Sha256::digest(input))
    );
    assert_eq!(
        backup["replacements"][0],
        json!({"line":3,"beforeId":"ts_012345abcdef","afterId":"tsc_012345abcdef"})
    );
    assert_eq!(
        Connection::open(db)
            .unwrap()
            .query_row(
                "SELECT COUNT(*) FROM threads WHERE model_provider='enterprise'",
                [],
                |r| r.get::<_, i64>(0)
            )
            .unwrap(),
        2
    );
}
#[test]
fn response_compressed_and_first_meta_positions_are_real_constraints() {
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    let requested = t.path().join("a.jsonl");
    let physical = t.path().join("a.jsonl.zst");
    let mut m = meta("a", "openai", "x");
    m["payload"]["history_mode"] = json!("paginated");
    let item = json!({"type":"response_item","payload":{"type":"custom_tool_call","id":"ct_abc"}});
    write_rows(&physical, &[m, item.clone()]);
    insert(&db, "a", "openai", Some(&requested), None);
    let err = plan_response_item_id_repair(&[db.clone()], t.path()).unwrap_err();
    assert!(err.0.contains("LAYOUT_PROTECTED"));
    write_rows(&physical, &[meta("a", "openai", "x"), item]);
    let plan = plan_response_item_id_repair(&[db], &t.path().join("backups")).unwrap();
    apply_response_item_id_repair(&plan).unwrap();
    assert!(!requested.exists());
    assert_eq!(read_rows(&physical)[1]["payload"]["id"], "ctc_abc");
}
#[test]
fn response_ordinal_record_reuses_padding_and_preserves_escaped_non_candidates() {
    let t = tempfile::tempdir().unwrap();
    let p = t.path().join("a.jsonl");
    let record=b"{\"type\":\"response_item\",\"ordinal\":2,\"payload\":{\"type\":\"custom_tool_call\",\"id\":\"ct_abc\"}}    \n";
    let (out, _) = rewrite_response_item_record(record, &p, 1)
        .unwrap()
        .unwrap();
    assert_eq!(out.len(), record.len());
    assert!(out.ends_with(b"   \n"));
    let escaped =
        br#"{"type":"response_item","payload":{"type":"custom_tool_call","id":"\u0063t_abc"}}"#;
    assert!(rewrite_response_item_record(escaped, &p, 1)
        .unwrap()
        .is_none());
}
#[test]
fn legacy_cleanup_uses_all_db_rows_but_only_active_rollout_boundaries() {
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    let p = t.path().join("active.jsonl");
    let native = json!({"type":"event_msg","payload":{"type":"thread_settings_applied","thread_settings":{"model_provider_id":"aimai1"}}});
    write_rows(
        &p,
        &[
            meta("active", "aimai1", "x"),
            meta("active", "aimami_relay_middle", "x"),
            meta("active", "OPENAI", "x"),
            native,
        ],
    );
    insert(&db, "active", "aimai1", Some(&p), None);
    let archived = t.path().join("archived.jsonl");
    write_rows(&archived, &[meta("archived", "aimami_codex_router", "x")]);
    insert(
        &db,
        "archived",
        "aimami_codex_router",
        Some(&archived),
        None,
    );
    insert(&db, "child", "OpenAI", None, None);
    insert(&db, "modern", "aimami_relay_p", None, None);
    insert(&db, "foreign", "enterprise", None, None);
    let c = Connection::open(&db).unwrap();
    c.execute("UPDATE threads SET archived=1 WHERE id='archived'", [])
        .unwrap();
    c.execute(
        "UPDATE threads SET source='\"subagent\"' WHERE id='child'",
        [],
    )
    .unwrap();
    let before = inspect_legacy_thread_visibility(&[db.clone()]);
    assert_eq!(before.legacy_rows, 3);
    let original_archived = fs::read(&archived).unwrap();
    let plan = plan_legacy_provider_cleanup(&[db.clone()], &t.path().join("backups")).unwrap();
    let summary = apply_legacy_provider_cleanup(&plan).unwrap();
    assert_eq!((summary.db_updated, summary.rollout_updated), (3, 1));
    let r = read_rows(&p);
    assert_eq!(r[0]["payload"]["model_provider"], "openai");
    assert_eq!(r[1]["payload"]["model_provider"], "aimami_relay_middle");
    assert_eq!(r[2]["payload"]["model_provider"], "openai");
    assert_eq!(
        r[3]["payload"]["thread_settings"]["model_provider_id"],
        "aimai1"
    );
    assert_eq!(fs::read(archived).unwrap(), original_archived);
    let after = inspect_legacy_thread_visibility(&[db]);
    assert_eq!(
        (after.legacy_rows, after.active_official_main_threads),
        (0, 1)
    );
    assert!(after.errors.is_empty());
}
#[test]
fn legacy_cleanup_mixed_foreign_boundary_does_not_blanket_take_over_history() {
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    let p = t.path().join("a.jsonl");
    write_rows(
        &p,
        &[
            meta("a", "enterprise", "x"),
            meta("a", "aimami_codex_router", "x"),
        ],
    );
    insert(&db, "a", "enterprise", Some(&p), None);
    let plan = plan_legacy_provider_cleanup(&[db], &t.path().join("backups")).unwrap();
    let summary = apply_legacy_provider_cleanup(&plan).unwrap();
    assert_eq!(summary.db_updated, 0);
    let rows = read_rows(&p);
    assert_eq!(rows[0]["payload"]["model_provider"], "enterprise");
    assert_eq!(rows[1]["payload"]["model_provider"], "openai");
}
#[test]
fn legacy_sql_failure_reverts_history_and_missing_active_file_is_skipped() {
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    let p = t.path().join("a.jsonl");
    write_rows(&p, &[meta("a", "aimai1", "x")]);
    insert(&db, "a", "aimai1", Some(&p), None);
    insert(
        &db,
        "missing",
        "aimami_codex_router",
        Some(&t.path().join("absent.jsonl")),
        None,
    );
    let original = fs::read(&p).unwrap();
    let c = Connection::open(&db).unwrap();
    c.execute_batch("CREATE TRIGGER reject_cleanup BEFORE UPDATE OF model_provider ON threads BEGIN SELECT RAISE(ABORT,'legacy fixture denied');END").unwrap();
    let plan = plan_legacy_provider_cleanup(&[db], &t.path().join("backups")).unwrap();
    assert_eq!(plan.databases[0].files.len(), 1);
    assert!(apply_legacy_provider_cleanup(&plan)
        .unwrap_err()
        .0
        .contains("legacy fixture denied"));
    assert_eq!(fs::read(p).unwrap(), original);
}
#[test]
fn legacy_predicate_preserves_modern_managed_and_exact_official_ids() {
    for p in ["aimai1", "aimami_codex_router", "OPENAI", "OpenAI"] {
        assert!(is_legacy_or_miscased_provider(p));
    }
    for p in [
        "openai",
        "aimami_relay",
        "aimami_relay_p",
        "AIMAI1",
        "enterprise",
    ] {
        assert!(!is_legacy_or_miscased_provider(p));
    }
}
#[test]
fn response_stale_snapshot_refuses_publication_and_creates_no_backup() {
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    let p = t.path().join("a.jsonl");
    write_rows(
        &p,
        &[
            meta("a", "openai", "x"),
            json!({"type":"response_item","payload":{"type":"custom_tool_call","id":"ct_abc"}}),
        ],
    );
    insert(&db, "a", "openai", Some(&p), None);
    let backup = t.path().join("backups");
    let plan = plan_response_item_id_repair(&[db], &backup).unwrap();
    fs::write(&p, b"external new history\n").unwrap();
    assert!(apply_response_item_id_repair(&plan)
        .unwrap_err()
        .0
        .contains("changed before repair"));
    assert!(!backup.exists());
    assert_eq!(fs::read(p).unwrap(), b"external new history\n");
}
fn instruction_meta(id: &str, text: &str) -> Value {
    let mut m = meta(id, "openai", "x");
    m["payload"]["base_instructions"] = json!({"text":text,"provenance":{"old":true}});
    m
}
#[test]
fn instruction_issue_uses_type_and_prefix_not_arbitrary_text() {
    assert_eq!(
        session_meta_instruction_issue(b"not json"),
        SessionInstructionIssue::Healthy
    );
    assert_eq!(
        session_meta_instruction_issue(br#"{"type":"event_msg"}"#),
        SessionInstructionIssue::Healthy
    );
    assert_eq!(
        session_meta_instruction_issue(
            &serde_json::to_vec(&instruction_meta("a", " \n ")).unwrap()
        ),
        SessionInstructionIssue::Missing
    );
    assert_eq!(
        session_meta_instruction_issue(
            &serde_json::to_vec(&instruction_meta(
                "a",
                &format!(" \t{APPROVAL_JUDGE_PREFIX}.")
            ))
            .unwrap()
        ),
        SessionInstructionIssue::Poisoned
    );
    assert_eq!(
        session_meta_instruction_issue(
            &serde_json::to_vec(&instruction_meta("a", "healthy short")).unwrap()
        ),
        SessionInstructionIssue::Healthy
    );
}
#[test]
fn reference_selection_keeps_sql_order_and_exact_archive_and_dedup_rules() {
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    let short = t.path().join("short.jsonl");
    let a = t.path().join("z-first.jsonl");
    let b = t.path().join("a-second.jsonl");
    write_rows(&short, &[instruction_meta("short", "short healthy")]);
    let first = " 首个长模板".repeat(180);
    let second = "second".repeat(200);
    write_rows(&a, &[instruction_meta("a", &first)]);
    write_rows(&b, &[instruction_meta("b", &second)]);
    insert(&db, "short", "openai", Some(&short), None);
    insert(&db, "z-first", "enterprise", Some(&a), None);
    insert(&db, "a-second", "openai", Some(&b), None);
    insert(&db, "duplicate", "openai", Some(&a), None);
    let ignored = t.path().join("ignored.jsonl");
    write_rows(
        &ignored,
        &[instruction_meta("ignored", &"archived".repeat(200))],
    );
    insert(&db, "ignored", "openai", Some(&ignored), None);
    Connection::open(&db)
        .unwrap()
        .execute("UPDATE threads SET archived=NULL WHERE id='ignored'", [])
        .unwrap();
    let paths =
        collect_active_instruction_rollout_paths(&[db.clone(), t.path().join("missing.db")]);
    assert_eq!(paths, vec![short, a.clone(), b]);
    let reference = find_reference_instructions(&[db]).unwrap();
    assert_eq!(reference.path, a);
    assert_eq!(reference.text, first);
    assert_eq!(reference.line, 1);
}
#[test]
fn reference_selection_enforces_five_lines_fifty_paths_and_byte_threshold() {
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    let mut sources = vec![];
    for n in 0..51 {
        let p = t.path().join(format!("{n}.jsonl"));
        let good = instruction_meta(&n.to_string(), &"字".repeat(334));
        if n == 0 {
            let mut bytes = b"x\nx\nx\nx\nx\n".to_vec();
            bytes.extend(serde_json::to_vec(&good).unwrap());
            fs::write(&p, bytes).unwrap()
        } else if n == 1 {
            write_rows(
                &p,
                &[instruction_meta(
                    "1",
                    &format!(" {APPROVAL_JUDGE_PREFIX}{}", "x".repeat(1100)),
                )],
            );
        } else if n == 50 {
            write_rows(&p, &[good]);
        } else {
            write_rows(&p, &[instruction_meta(&n.to_string(), &"x".repeat(1000))]);
        }
        insert(&db, &n.to_string(), "openai", Some(&p), None);
        sources.push(p);
    }
    assert!(find_reference_instructions(&[db.clone()]).is_none());
    Connection::open(&db)
        .unwrap()
        .execute("UPDATE threads SET archived=1 WHERE id='2'", [])
        .unwrap();
    let reference = find_reference_instructions(&[db]).unwrap();
    assert_eq!(reference.path, sources[50]);
    assert_eq!(reference.text.as_bytes().len(), 1002);
}
#[test]
fn poisoned_repair_restores_first_meta_and_migration_snapshot_strings_only() {
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    let source = t.path().join("source.jsonl");
    let target = t.path().join("target.jsonl.zst");
    let reference = "full original template ".repeat(70);
    write_rows(&source, &[instruction_meta("source", &reference)]);
    let polluted = instruction_meta(
        "target",
        &format!("{APPROVAL_JUDGE_PREFIX} dangerous cached prompt"),
    );
    let later = instruction_meta("target", "later instruction unchanged");
    write_rows(&target, &[polluted.clone(), later.clone()]);
    insert(&db, "source", "enterprise", Some(&source), None);
    insert(&db, "target", "openai", Some(&target), None);
    let relay = t.path().join("relay");
    let migration = relay.join("thread-router-migrations");
    fs::create_dir_all(migration.join("history")).unwrap();
    let stored = serde_json::to_string(&polluted).unwrap();
    fs::write(
        migration.join("current.json"),
        serde_json::to_vec(
            &json!({"entries":[{"meta":stored,"ordinary":"untouched"}],"other":"keep"}),
        )
        .unwrap(),
    )
    .unwrap();
    fs::write(migration.join("history").join("one.json"),serde_json::to_vec(&json!({"entries":[{"old_meta":serde_json::to_string(&instruction_meta("saved","")).unwrap()}]})).unwrap()).unwrap();
    let plan = plan_poisoned_instruction_repair(&[db.clone()], &relay).unwrap();
    assert_eq!(plan.reference.as_ref().unwrap().path, source);
    assert_eq!((plan.rollouts.len(), plan.snapshots.len()), (1, 2));
    let summary = apply_poisoned_instruction_repair(&plan).unwrap();
    assert_eq!(summary.summary.rollout_updated, 1);
    assert_eq!(summary.snapshot_values_updated, 2);
    assert_eq!(summary.remaining_issues, 0);
    assert!(!summary.used_minimal_fallback);
    assert!(summary.summary.warnings.is_empty());
    let rows = read_rows(&target);
    assert_eq!(
        rows[0]["payload"]["base_instructions"],
        json!({"text":reference})
    );
    assert_eq!(rows[1], later);
    let snapshot: Value =
        serde_json::from_slice(&fs::read(migration.join("current.json")).unwrap()).unwrap();
    let saved: Value =
        serde_json::from_str(snapshot["entries"][0]["meta"].as_str().unwrap()).unwrap();
    assert_eq!(
        saved["payload"]["base_instructions"],
        json!({"text":reference})
    );
    assert_eq!(snapshot["entries"][0]["ordinary"], "untouched");
    let backup = relay
        .join("guardian-prompt-fix-backup")
        .join("target.jsonl.bak");
    assert_eq!(
        serde_json::from_slice::<Value>(&fs::read(backup).unwrap()).unwrap(),
        polluted
    );
    assert_eq!(
        Connection::open(db)
            .unwrap()
            .query_row(
                "SELECT model_provider FROM threads WHERE id='target'",
                [],
                |r| r.get::<_, String>(0)
            )
            .unwrap(),
        "openai"
    );
}
#[test]
fn poisoned_fallback_and_paginated_failure_are_reported_without_false_success() {
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    let a = t.path().join("a.jsonl");
    let b = t.path().join("b.jsonl");
    write_rows(&a, &[instruction_meta("a", "")]);
    let mut protected = instruction_meta("b", "");
    protected["ordinal"] = json!(0);
    write_rows(&b, &[protected]);
    insert(&db, "a", "openai", Some(&a), None);
    insert(&db, "b", "openai", Some(&b), None);
    let original = fs::read(&b).unwrap();
    let plan = plan_poisoned_instruction_repair(&[db], &t.path().join("relay")).unwrap();
    assert!(plan.reference.is_none());
    assert!(plan.warnings.iter().any(|s| s.contains("LAYOUT_PROTECTED")));
    let summary = apply_poisoned_instruction_repair(&plan).unwrap();
    assert!(summary.used_minimal_fallback);
    assert_eq!(summary.remaining_issues, 1);
    assert_eq!(summary.summary.rollout_updated, 1);
    assert_eq!(
        read_rows(&a)[0]["payload"]["base_instructions"],
        json!({"text":MINIMAL_SAFE_INSTRUCTIONS})
    );
    assert_eq!(fs::read(b).unwrap(), original);
    assert!(!summary.summary.warnings.is_empty());
}
#[test]
fn legacy_diagnostic_wrapper_excludes_live_router_and_archived_subagent_rows() {
    let t = tempfile::tempdir().unwrap();
    let db = database(t.path());
    insert(&db, "router", "aimai1", None, None);
    insert(&db, "old", "aimami_codex_router", None, None);
    insert(&db, "archived", "aimai1", None, None);
    insert(&db, "child", "OpenAI", None, None);
    let c = Connection::open(&db).unwrap();
    c.execute("UPDATE threads SET archived=1 WHERE id='archived'", [])
        .unwrap();
    c.execute(
        "UPDATE threads SET source='\"subagent\"' WHERE id='child'",
        [],
    )
    .unwrap();
    assert_eq!(
        inspect_legacy_migrated_threads(&[db.clone()], true)
            .ids
            .into_iter()
            .collect::<Vec<_>>(),
        vec!["old"]
    );
    assert_eq!(
        inspect_legacy_migrated_threads(&[db], false)
            .ids
            .into_iter()
            .collect::<Vec<_>>(),
        vec!["old", "router"]
    );
}
