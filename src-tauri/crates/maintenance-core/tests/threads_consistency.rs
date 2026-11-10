use atelier_maintenance_core::threads::consistency::*;
use serde_json::json;
#[test]
fn model_index_preserves_native_default_alias_and_passthrough_policy() {
    let index=RouterCatalogModelIndex::from_json(&json!({"aimami_no_account":false,"models":[{"slug":"aimami_relay_a::deepseek"},{"slug":"aimami_relay_b::deepseek"},{"slug":"gpt-5.6-sol"}]})).unwrap();
    assert_eq!(
        index.default_model.as_deref(),
        Some("aimami_relay_a::deepseek")
    );
    assert_eq!(
        index.aliases.get("deepseek").map(String::as_str),
        Some("aimami_relay_a::deepseek")
    );
    assert_eq!(
        index.disposition(Some(" aimami_relay_a::deepseek ")),
        ModelDisposition::Catalog
    );
    assert_eq!(
        index.disposition(Some("unlisted-bare-model")),
        ModelDisposition::OfficialPassthrough
    );
    assert_eq!(
        index.disposition(Some("unknown::model")),
        ModelDisposition::Replace("aimami_relay_a::deepseek".into())
    );
    let slots =
        RouterCatalogModelIndex::from_json(&json!({"models":[{"slug":"gpt-5.6-sol"}]})).unwrap();
    assert!(slots.no_account);
    assert_eq!(
        slots.disposition(Some("unlisted")),
        ModelDisposition::Replace("gpt-5.6-sol".into())
    );
    assert!(catalog_no_account_mode(&json!({"aimami_no_account":"true","models":[]})).is_err());
    assert!(!catalog_no_account_mode(&json!({"models":[]})).unwrap());
    assert!(catalog_no_account_mode(&json!({"aimami_no_account":true})).unwrap());
}
#[test]
fn provider_observation_counts_only_owned_healthy_histories() {
    let row = atelier_maintenance_core::threads::ProviderConvergenceTarget {
        id: "id".into(),
        provider: "openai".into(),
        rollout_path: None,
        model: None,
    };
    let mut i = ThreadProviderConvergenceInspection {
        target_provider: "aimai1".into(),
        ..Default::default()
    };
    apply_provider_convergence_observation(
        &mut i,
        &row,
        ProviderObservation::FileMissing("missing".into()),
    );
    assert_eq!((i.total, i.missing_rollouts, i.db_mismatch), (1, 1, 0));
    apply_provider_convergence_observation(
        &mut i,
        &row,
        ProviderObservation::Anchors(RolloutProviderAnchors {
            first: Some("enterprise".into()),
            last: Some("openai".into()),
            native: None,
        }),
    );
    assert_eq!(i.db_mismatch, 0);
    apply_provider_convergence_observation(
        &mut i,
        &row,
        ProviderObservation::Anchors(RolloutProviderAnchors {
            first: Some("OPENAI".into()),
            last: Some("aimai1".into()),
            native: None,
        }),
    );
    assert_eq!((i.db_mismatch, i.rollout_mismatch), (1, 1));
    assert_eq!(i.total, 3);
}
fn meta(id: &str, p: &str) -> String {
    json!({"type":"session_meta","payload":{"id":id,"model_provider":p}}).to_string() + "\n"
}
fn settings(id: &str, p: &str) -> String {
    json!({"type":"event_msg","payload":{"type":"thread_settings_applied","thread_id":id,"thread_settings":{"model_provider_id":p}}}).to_string()+"\n"
}
#[test]
fn anchors_plain_reverse_and_compressed_forward_reset_settings() {
    let t = tempfile::tempdir().unwrap();
    let p = t.path().join("r.jsonl");
    let text = meta("id", "openai")
        + &settings("id", "aimai1")
        + &meta("id", "aimai1")
        + &settings("id", "OPENAI");
    std::fs::write(&p, &text).unwrap();
    let anchors = read_rollout_provider_anchors(&p).unwrap().unwrap();
    assert_eq!(
        anchors,
        RolloutProviderAnchors {
            first: Some("openai".into()),
            last: Some("aimai1".into()),
            native: Some("OPENAI".into())
        }
    );
    let compressed = t.path().join("z.jsonl.zst");
    std::fs::write(
        &compressed,
        zstd::stream::encode_all(text.as_bytes(), 1).unwrap(),
    )
    .unwrap();
    assert_eq!(
        read_rollout_provider_anchors(&compressed).unwrap(),
        Some(anchors)
    );
    std::fs::write(
        &p,
        meta("id", "openai") + &settings("id", "aimai1") + &meta("id", "openai"),
    )
    .unwrap();
    assert_eq!(
        read_rollout_provider_anchors(&p).unwrap().unwrap().native,
        None
    );
}
#[test]
fn cache_threshold_and_snapshot_reuse_are_real_file_reads() {
    let t = tempfile::tempdir().unwrap();
    let paths: Vec<_> = (0..8)
        .map(|n| {
            let p = t.path().join(format!("{n}.jsonl"));
            std::fs::write(&p, meta("id", "openai")).unwrap();
            p
        })
        .collect();
    let mut cache = DiagnosticRolloutProviderCache::default();
    cache.prefetch(&paths[..7]);
    assert!(cache.is_empty());
    cache.prefetch(&paths);
    assert_eq!(cache.len(), 8);
    std::fs::write(&paths[0], meta("id", "aimai1")).unwrap();
    assert_eq!(
        cache.read(&paths[0]).unwrap().unwrap().first.as_deref(),
        Some("openai")
    );
    assert_eq!(
        read_rollout_provider_anchors(&paths[0])
            .unwrap()
            .unwrap()
            .first
            .as_deref(),
        Some("aimai1")
    );
}
#[test]
fn provider_end_to_end_counts_cache_native_and_foreign_ownership() {
    let t = tempfile::tempdir().unwrap();
    let db = t.path().join("db");
    let c = rusqlite::Connection::open(&db).unwrap();
    c.execute_batch("CREATE TABLE threads(id TEXT,rollout_path TEXT,model_provider TEXT,source TEXT,archived INT)").unwrap();
    for (id, text) in [
        (
            "drift",
            meta("drift", "openai") + &settings("drift", "aimai1"),
        ),
        ("foreign", meta("foreign", "enterprise")),
        (
            "broken",
            "{\"type\":\"session_meta\",\"payload\":{}}\n".into(),
        ),
        ("empty", "{}\n".into()),
    ] {
        let p = t.path().join(format!("{id}.jsonl"));
        std::fs::write(&p, text).unwrap();
        c.execute(
            "INSERT INTO threads VALUES (?1,?2,'openai','cli',0)",
            rusqlite::params![id, p.to_str()],
        )
        .unwrap();
    }
    let mut cache = DiagnosticRolloutProviderCache::default();
    let i = inspect_thread_provider_convergence_with_reader(&[db.clone()], "aimai1", &mut cache);
    assert_eq!(
        (
            i.total,
            i.db_mismatch,
            i.rollout_mismatch,
            i.corrupt_rollouts
        ),
        (4, 1, 1, 2)
    );
    assert!(i.errors.is_empty());
    assert_eq!(cache.len(), 4);
    assert_eq!(
        check_thread_provider_consistency_with_cache(&[db], true, &mut cache).status,
        "error"
    );
    assert!(
        !inspect_thread_provider_convergence_with_reader(&[], "OPENAI", &mut cache)
            .errors
            .is_empty()
    );
}
#[test]
fn model_catalog_end_to_end_preserves_unwritable_and_native_identity_boundaries() {
    let t = tempfile::tempdir().unwrap();
    std::fs::write(
        t.path().join("codex_router_catalog.json"),
        json!({"aimami_no_account":false,"models":[{"slug":"aimami_relay_a::model"}]}).to_string(),
    )
    .unwrap();
    let db = t.path().join("db");
    let c = rusqlite::Connection::open(&db).unwrap();
    c.execute_batch("CREATE TABLE threads(id TEXT,rollout_path TEXT,model_provider TEXT,source TEXT,archived INT,model TEXT)").unwrap();
    for (id, text) in [
        ("drift", Some(meta("drift", "openai"))),
        ("foreign", Some(meta("foreign", "enterprise"))),
        (
            "broken",
            Some("{\"type\":\"session_meta\",\"payload\":{}}\n".into()),
        ),
        ("wrongid", Some(meta("another", "openai"))),
        ("missing", None),
        (
            "wrongnative",
            Some(meta("wrongnative", "openai") + &settings("another", "openai")),
        ),
    ] {
        let p = t.path().join(format!("{id}.jsonl"));
        if let Some(text) = text {
            std::fs::write(&p, text).unwrap();
        }
        c.execute(
            "INSERT INTO threads VALUES (?1,?2,'aimai1','cli',0,'bad::model')",
            rusqlite::params![id, p.to_str()],
        )
        .unwrap();
    }
    c.execute(
        "INSERT INTO threads VALUES ('bare',NULL,'aimai1','cli',0,'unlisted-bare')",
        [],
    )
    .unwrap();
    let i = inspect_router_thread_model_catalog(&[db], t.path());
    assert_eq!((i.total, i.inconsistent, i.accepted), (7, 2, 1));
    assert_eq!(i.errors.len(), 2);
    assert!(i
        .errors
        .iter()
        .any(|e| e.contains("latest native settings belongs to another task")));
    assert_eq!(
        check_thread_model_catalog_consistency(true, &i).status,
        "error"
    );
}
#[test]
fn reverse_blocks_join_long_metadata_and_only_corruption_warns() {
    let t = tempfile::tempdir().unwrap();
    let p = t.path().join("long.jsonl");
    let mut second =
        json!({"type":"session_meta","payload":{"id":"id","model_provider":"aimai1","padding":""}});
    second["payload"]["padding"] = json!("x".repeat(1024 * 1024 + 37));
    std::fs::write(
        &p,
        meta("id", "openai") + &second.to_string() + "\n" + &settings("id", "openai"),
    )
    .unwrap();
    let a = read_rollout_provider_anchors(&p).unwrap().unwrap();
    assert_eq!(a.last.as_deref(), Some("aimai1"));
    assert_eq!(a.native.as_deref(), Some("openai"));
    let db = t.path().join("db");
    let c = rusqlite::Connection::open(&db).unwrap();
    c.execute_batch("CREATE TABLE threads(id TEXT,rollout_path TEXT,model_provider TEXT,source TEXT,archived INT)").unwrap();
    let bad = t.path().join("bad");
    std::fs::write(&bad, "{\"type\":\"session_meta\",\"payload\":{}}\n").unwrap();
    c.execute(
        "INSERT INTO threads VALUES ('bad',?1,'openai','cli',0)",
        [bad.to_str()],
    )
    .unwrap();
    let item = check_thread_provider_consistency_with_cache(&[db], true, &mut Default::default());
    assert_eq!(item.status, "warning");
    assert!(!item.fixable);
    assert_eq!(
        check_thread_model_catalog_consistency_from_sources(
            &[],
            std::path::Path::new("/not/read"),
            false
        )
        .status,
        "ok"
    );
}
