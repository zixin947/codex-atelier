use atelier_maintenance_core::threads::*;
use rusqlite::Connection;
#[test]
fn sqlite_main_threads_optional_columns_filters_and_error_boundary() {
    let t = tempfile::tempdir().unwrap();
    let p = t.path().join("synthetic.db");
    let c = Connection::open(&p).unwrap();
    c.execute_batch("CREATE TABLE threads(id TEXT,rollout_path TEXT,model_provider TEXT,source TEXT,archived INTEGER); INSERT INTO threads VALUES ('z','r','  native  ',NULL,NULL),('a',NULL,'  ','cli',0),('b',NULL,'x','{\"subagent\":{}}',0),('c',NULL,'x','cli',1);").unwrap();
    let rows = collect_diagnostic_main_threads(&[p.clone()]).unwrap();
    assert_eq!(
        rows.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(),
        ["a", "z"]
    );
    assert_eq!(rows[0].provider, None);
    assert_eq!(rows[1].provider.as_deref(), Some("  native  "));
    assert!(!rows[0].has_model_column);
    assert!(!rows[0].has_effort_column);
    c.execute_batch("ALTER TABLE threads ADD COLUMN model TEXT; ALTER TABLE threads ADD COLUMN reasoning_effort TEXT; UPDATE threads SET model='gpt-x',reasoning_effort='high' WHERE id='z'").unwrap();
    let rows = collect_diagnostic_main_threads(&[p.clone()]).unwrap();
    assert_eq!(rows[1].model.as_deref(), Some("gpt-x"));
    assert!(rows[0].has_effort_column);
    let err = collect_diagnostic_main_threads(&[p, t.path().join("absent.db")]).unwrap_err();
    assert!(err.starts_with("open thread database "));
    assert!(!t.path().join("absent.db").exists());
}
#[test]
fn missing_wrapper_error_precedence() {
    assert_eq!(
        check_thread_missing_rollouts(&MissingRolloutsInspection::default()).status,
        "ok"
    );
    let r = check_thread_missing_rollouts(&MissingRolloutsInspection {
        errors: vec!["bad db".into()],
        ..Default::default()
    });
    assert_eq!(r.status, "error");
    assert!(r.fixable);
    assert_eq!(r.title, "0 条活跃线程缺少 rollout 文件");
    assert!(r.detail.unwrap().contains("读取状态库失败 1 项"));
}
#[test]
fn project_wrapper_priority() {
    let mut i = ProjectStateInspection {
        read_error: Some("broken json".into()),
        db_errors: vec!["bad db".into()],
        needs_repair: true,
        ..Default::default()
    };
    assert_eq!(
        check_project_state_integrity(&i).title,
        "Codex 项目索引无法读取"
    );
    i.read_error = None;
    assert!(!check_project_state_integrity(&i).fixable);
    i.db_errors.clear();
    assert!(check_project_state_integrity(&i).fixable);
    i.needs_repair = false;
    assert_eq!(check_project_state_integrity(&i).status, "ok");
}
#[test]
fn orphan_closed_ownership_and_unique_ids_across_dbs() {
    let t = tempfile::tempdir().unwrap();
    let mut dbs = vec![];
    for n in 0..2 {
        let p = t.path().join(format!("{n}.db"));
        let c = Connection::open(&p).unwrap();
        c.execute_batch("CREATE TABLE threads(id TEXT,rollout_path TEXT,model_provider TEXT,source TEXT,archived INT); INSERT INTO threads VALUES ('a',NULL,'aimami_relay_old','cli',0),('third',NULL,'external','cli',0),('valid',NULL,'aimai1','cli',0),('sub',NULL,'aimami_codex_router','\"subagent\"',0);").unwrap();
        dbs.push(p);
    }
    let valid = ["aimai1".to_string()].into_iter().collect();
    let r = check_db_orphan_providers(&dbs, &valid);
    assert_eq!(r.title, "1 条主线程引用了失效的旧版工具 provider");
    assert!(r.detail.unwrap().ends_with("示例：a"));
    assert!(!is_aimami_provider("aimami_custom"));
    assert!(!is_aimami_provider(" aimami_relay"));
    assert!(is_aimami_provider("aimami_relay_"));
    assert_eq!(check_db_orphan_providers(&[], &valid).status, "ok");
}
#[test]
fn rollout_representation_order_and_non_file_error() {
    let t = tempfile::tempdir().unwrap();
    let plain = t.path().join("r.jsonl");
    let compressed = t.path().join("r.jsonl.zst");
    assert!(resolve_rollout(&plain).unwrap().is_none());
    std::fs::write(&compressed, b"not even valid zstd").unwrap();
    assert_eq!(resolve_rollout(&plain).unwrap(), Some(compressed.clone()));
    std::fs::write(&plain, b"").unwrap();
    assert_eq!(resolve_rollout(&compressed).unwrap(), Some(plain.clone()));
    std::fs::remove_file(&plain).unwrap();
    std::fs::create_dir(&plain).unwrap();
    assert_eq!(
        resolve_rollout(&compressed).unwrap_err().kind(),
        std::io::ErrorKind::InvalidInput
    );
}
#[test]
fn convergence_filters_owned_and_openai_but_not_external() {
    let c = Connection::open_in_memory().unwrap();
    c.execute_batch("CREATE TABLE threads(id TEXT,rollout_path TEXT,model_provider TEXT,source TEXT,archived INT,model TEXT); INSERT INTO threads VALUES ('a',NULL,'OPENAI','cli',0,'  '),('b',NULL,'aimami_relay_old','cli',0,' x '),('c',NULL,'other','cli',0,NULL),('d',NULL,'aimai1','\"subagent\"',0,NULL),('e',NULL,'openai','cli',1,NULL)").unwrap();
    let rows = collect_provider_convergence_targets(&c).unwrap();
    assert_eq!(
        rows.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(),
        ["a", "b"]
    );
    assert_eq!(rows[0].model, None);
    assert_eq!(rows[1].model.as_deref(), Some(" x "));
    c.execute_batch("INSERT INTO threads VALUES ('z',NULL,NULL,'cli',0,NULL)")
        .unwrap();
    assert!(collect_provider_convergence_targets(&c)
        .unwrap_err()
        .starts_with("read thread convergence row: "));
}
#[test]
fn backup_requires_exact_filename_and_first_session_identity() {
    let t = tempfile::tempdir().unwrap();
    let root = t.path().join("backups");
    std::fs::create_dir_all(root.join("nested")).unwrap();
    let p = root.join("nested/r.jsonl");
    std::fs::write(
        &p,
        b"invalid\n\xff\n{\"type\":\"session_meta\",\"payload\":{\"id\":\"right\"}}\n",
    )
    .unwrap();
    assert!(rollout_file_belongs_to_thread(&p, "right"));
    assert!(!rollout_file_belongs_to_thread(&p, "wrong"));
    assert_eq!(
        find_trusted_rollout_backup(&root, "right", std::path::Path::new("missing/r.jsonl")),
        Some(p.clone())
    );
    assert!(find_trusted_rollout_backup(
        &root,
        "right",
        std::path::Path::new("missing/other.jsonl")
    )
    .is_none());
    std::fs::write(&p,b"{\"type\":\"session_meta\",\"payload\":{\"id\":\"wrong\"}}\n{\"type\":\"session_meta\",\"payload\":{\"id\":\"right\"}}\n").unwrap();
    assert!(!rollout_file_belongs_to_thread(&p, "right"));
}
#[test]
fn missing_scan_counts_recoverable_unrecoverable_and_io_separately() {
    let t = tempfile::tempdir().unwrap();
    let root = t.path().join("backups");
    std::fs::create_dir(&root).unwrap();
    std::fs::write(
        root.join("a.jsonl"),
        b"{\"type\":\"session_meta\",\"payload\":{\"id\":\"a\"}}\n",
    )
    .unwrap();
    let db = t.path().join("state.db");
    let c = Connection::open(&db).unwrap();
    c.execute_batch("CREATE TABLE threads(id TEXT,rollout_path TEXT,model_provider TEXT,source TEXT,archived INT)").unwrap();
    for (id, path) in [
        ("a", Some(t.path().join("a.jsonl"))),
        ("b", None),
        ("c", Some(t.path().join("c.jsonl"))),
        ("d", Some(root.clone())),
    ] {
        c.execute(
            "INSERT INTO threads VALUES (?1,?2,'openai','cli',0)",
            rusqlite::params![id, path.map(|p| p.to_str().unwrap().to_string())],
        )
        .unwrap();
    }
    let i = inspect_missing_rollouts_for_convergence(&[db, t.path().join("absent.db")], &root);
    assert_eq!(
        (i.total, i.missing, i.recoverable, i.unrecoverable),
        (4, 3, 1, 2)
    );
    assert_eq!(i.errors.len(), 1);
    assert_eq!(i.examples.len(), 3);
    assert!(i.errors[0].contains("rollout is not a file"));
}
#[test]
fn backup_zstd_identity_and_symlink_exclusion() {
    let t = tempfile::tempdir().unwrap();
    let root = t.path().join("backups");
    std::fs::create_dir(&root).unwrap();
    let p = root.join("r.jsonl.zst");
    let bytes = zstd::stream::encode_all(
        &b"{\"type\":\"session_meta\",\"payload\":{\"id\":\"id\"}}\n"[..],
        1,
    )
    .unwrap();
    std::fs::write(&p, bytes).unwrap();
    assert!(rollout_file_belongs_to_thread(&p, "id"));
    assert!(find_trusted_rollout_backup(&root, "id", std::path::Path::new("r.jsonl")).is_none());
    assert_eq!(
        find_trusted_rollout_backup(&root, "id", std::path::Path::new("r.jsonl.zst")),
        Some(p.clone())
    );
    #[cfg(unix)]
    {
        let linkroot = t.path().join("links");
        std::fs::create_dir(&linkroot).unwrap();
        std::os::unix::fs::symlink(&p, linkroot.join("r.jsonl.zst")).unwrap();
        assert!(
            find_trusted_rollout_backup(&linkroot, "id", std::path::Path::new("r.jsonl.zst"))
                .is_none()
        );
    }
}
#[test]
fn project_json_producer_parse_repair_and_normalization() {
    let t = tempfile::tempdir().unwrap();
    let p = t.path().join("global-state.json");
    let db = ProjectDbSnapshot {
        roots: 1,
        hints: 1,
        active_threads: 1,
        ..Default::default()
    };
    let absent = inspect_project_state_with_db(&p, db.clone());
    assert!(absent.needs_repair);
    assert_eq!(
        absent.issue.as_deref(),
        Some("Codex 项目入口为空，但线程库存在项目 cwd")
    );
    std::fs::write(&p, b"broken json").unwrap();
    let bad = inspect_project_state_with_db(&p, db.clone());
    assert!(bad.read_error.is_none());
    assert!(bad.needs_repair);
    assert_eq!(check_project_state_integrity(&bad).status, "error");
    std::fs::write(&p,br#"{"electron-saved-workspace-roots":[" /project/ ","/",2],"active-workspace-roots":["/project/"],"thread-workspace-root-hints":{"id":"/project"}}"#).unwrap();
    let healthy = inspect_project_state_with_db(&p, db);
    assert_eq!(
        (healthy.roots, healthy.active_roots, healthy.hints),
        (1, 1, 1)
    );
    assert!(!healthy.needs_repair);
    assert_eq!(normalize_codex_path(" \\\\?\\C:\\repo\\ "), "C:\\repo");
}
#[test]
fn valid_id_composition_honors_router_flag_and_ignores_source_errors() {
    let base = ["configured".to_string()].into_iter().collect();
    let quarantine = ["held".to_string()].into_iter().collect();
    let config = ["section".to_string()].into_iter().collect();
    let on = diagnostic_valid_provider_ids(&base, true, Ok(&quarantine), Ok(&config));
    assert!(on.contains("aimai1"));
    assert!(on.contains("held"));
    assert!(on.contains("section"));
    assert!(on.contains("openai"));
    assert!(on.contains("OpenAI"));
    let off = diagnostic_valid_provider_ids(&base, false, Ok(&quarantine), Err("unreadable"));
    assert!(!off.contains("held"));
    assert!(!off.contains("aimai1"));
    assert!(off.contains("configured"));
}
#[test]
fn valid_ids_reads_structural_config_even_when_toml_incomplete() {
    let t = tempfile::tempdir().unwrap();
    let p = t.path().join("synthetic.toml");
    std::fs::write(&p,"[model_providers.\"aimami_relay_x\"]\nbroken=\n[model_providers.foreign] # trailing comment\n").unwrap();
    let ids = diagnostic_valid_provider_ids_from_sources(
        &Default::default(),
        false,
        Err("no quarantine"),
        &p,
    );
    assert!(ids.contains("aimami_relay_x"));
    assert!(!ids.contains("foreign"));
}
#[test]
fn project_end_to_end_db_roots_hints_backfill_and_projectless_exclusion() {
    let t = tempfile::tempdir().unwrap();
    let state = t.path().join("state.json");
    std::fs::write(&state, br#"{"projectless-thread-ids":["held"]}"#).unwrap();
    let db = t.path().join("db");
    let c = Connection::open(&db).unwrap();
    c.execute_batch("CREATE TABLE threads(id TEXT,cwd,rollout_path TEXT,source TEXT,archived INT,updated_at INT);INSERT INTO threads VALUES ('existing',' /real/project/ ',NULL,'cli',0,4),('generated','/Users/me/Documents/Codex/2026-11-08',NULL,'cli',0,3),('numeric',123,NULL,'cli',0,2),('old','/old',NULL,'cli',1,1)").unwrap();
    for (id, source) in [("missing", "cli"), ("held", "cli"), ("sub", "\"subagent\"")] {
        let rollout = t.path().join(format!("{id}.jsonl"));
        std::fs::write(&rollout,format!("{{\"type\":\"session_meta\",\"payload\":{{\"id\":\"{id}\",\"cwd\":\"/restored/project\"}}}}\n")).unwrap();
        c.execute(
            "INSERT INTO threads VALUES (?1,'',?2,?3,0,0)",
            rusqlite::params![id, rollout.to_str(), source],
        )
        .unwrap();
    }
    let i = inspect_project_state(&state, Some(&db));
    assert!(i.db_errors.is_empty());
    assert_eq!(
        (i.db_roots, i.db_hints, i.active_threads, i.cwd_backfill),
        (1, 2, 6, 1)
    );
    assert!(i.needs_repair);
    let snapshot = collect_project_state_from_db(Some(&db), &Default::default());
    assert_eq!(snapshot.cwd_backfill, 2);
}
#[test]
fn project_missing_columns_empty_snapshot_and_invalid_order_error_are_native() {
    let t = tempfile::tempdir().unwrap();
    let db = t.path().join("db");
    let c = Connection::open(&db).unwrap();
    c.execute_batch("CREATE TABLE threads(id TEXT)").unwrap();
    assert_eq!(
        collect_project_state_from_db(Some(&db), &Default::default()),
        ProjectDbSnapshot::default()
    );
    c.execute_batch("ALTER TABLE threads ADD COLUMN cwd TEXT")
        .unwrap();
    assert!(
        !collect_project_state_from_db(Some(&db), &Default::default())
            .errors
            .is_empty()
    );
}
