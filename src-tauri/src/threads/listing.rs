use super::*;
use std::collections::BTreeSet;
use std::path::Path;

pub(super) fn non_root_ids(c: &Connection) -> Result<BTreeSet<String>> {
    let mut ids = BTreeSet::new();
    for (table, column) in [
        ("thread_spawn_edges", "child_thread_id"),
        ("agent_job_items", "assigned_thread_id"),
    ] {
        let columns = c
            .prepare(&format!("PRAGMA table_info({table})"))
            .map_err(err)?
            .query_map([], |r| r.get::<_, String>(1))
            .map_err(err)?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(err)?;
        if !columns.iter().any(|name| name == column) {
            continue;
        }
        let mut stmt = c
            .prepare(&format!(
                "SELECT DISTINCT {column} FROM {table} WHERE COALESCE({column},'')<>''"
            ))
            .map_err(err)?;
        for id in stmt.query_map([], |r| r.get::<_, String>(0)).map_err(err)? {
            ids.insert(id.map_err(err)?);
        }
    }
    Ok(ids)
}
pub(super) fn database(
    s: &Store,
    environment: Option<&std::ffi::OsStr>,
    cwd: Option<&Path>,
) -> Result<PathBuf> {
    use atelier_maintenance_core::state_db;
    let config = s.home.join("config.toml");
    let text = match fs::File::open(&config) {
        Ok(file) => {
            use std::io::Read;
            let mut t = String::new();
            file.take(1024 * 1024 + 1)
                .read_to_string(&mut t)
                .map_err(err)?;
            if t.len() > 1024 * 1024 {
                return Err("配置文件过大".into());
            }
            toml::from_str::<toml::Value>(&t).map_err(err)?;
            Some(t)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => return Err(err(e)),
    };
    let configured = state_db::configured_sqlite_home(text.as_deref(), environment, cwd);
    let dirs = if let Some(dir) = configured {
        vec![dir]
    } else {
        vec![s.home.join("sqlite"), s.home.clone()]
    };
    for dir in dirs {
        match fs::symlink_metadata(&dir) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => return Err(err(e)),
            Ok(m) if !m.is_dir() || m.file_type().is_symlink() => {
                return Err("索引目录不是普通目录".into())
            }
            _ => (),
        }
        let preferred = dir.join("codex-dev.db");
        let mut candidates = match fs::symlink_metadata(&preferred) {
            Ok(_) => vec![preferred],
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => vec![],
            Err(e) => return Err(err(e)),
        };
        if let Some(state) = state_db::highest_state_db(&dir) {
            candidates.push(state);
        }
        for path in candidates {
            let m = fs::symlink_metadata(&path).map_err(err)?;
            if !m.is_file() || m.file_type().is_symlink() {
                return Err("不支持非普通索引数据库".into());
            }
            let c = Connection::open_with_flags(&path, OpenFlags::SQLITE_OPEN_READ_ONLY)
                .map_err(err)?;
            // Other Codex databases can share this directory without containing session indexes.
            let has_threads: bool = c.query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='threads')",
                [], |row| row.get(0),
            ).map_err(|e|format!("检查索引结构失败（{}）：{e}",path.display()))?;
            if !has_threads {
                continue;
            }
            c.prepare("SELECT id FROM threads LIMIT 0")
                .map_err(|e| format!("当前线程索引结构不兼容（{}）：{e}", path.display()))?;
            return Ok(path);
        }
    }
    Err("当前配置目录没有可读取的会话索引".into())
}
pub(super) fn kind(
    source: &str,
    thread_source: &str,
    parent: &str,
    user_event: bool,
    first_message: &str,
    has_user_columns: bool,
) -> &'static str {
    if thread_source.eq_ignore_ascii_case("user") {
        return "conversation";
    }
    let lower = source.to_ascii_lowercase();
    let is_subagent = serde_json::from_str::<Value>(source)
        .ok()
        .is_some_and(|v| v.get("subagent").is_some() || v.get("sub_agent").is_some());
    if !parent.trim().is_empty()
        || is_subagent
        || lower.contains("subagent")
        || lower.contains("memory_consolidation")
        || matches!(
            thread_source.to_ascii_lowercase().as_str(),
            "subagent" | "memory_consolidation"
        )
    {
        return "subtask";
    }
    if lower.contains("internal") || thread_source.eq_ignore_ascii_case("ambient_suggestions") {
        return "unverified";
    }
    if user_event
        || !first_message.trim().is_empty()
        || (!has_user_columns
            && matches!(
                lower.as_str(),
                "cli" | "vscode" | "desktop" | "app-server" | "app_server"
            ))
    {
        "conversation"
    } else {
        "unverified"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn relationship_tables_separate_children_without_reclassifying_explicit_users() {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch("CREATE TABLE threads(id TEXT,title TEXT,model_provider TEXT,source TEXT,thread_source TEXT); INSERT INTO threads VALUES('user','u','openai','cli','user'),('spawn','s','openai','cli',NULL),('job','j','openai_https','cli',NULL); CREATE TABLE thread_spawn_edges(child_thread_id TEXT); INSERT INTO thread_spawn_edges VALUES('spawn'),('user'); CREATE TABLE agent_job_items(assigned_thread_id TEXT); INSERT INTO agent_job_items VALUES('job');").unwrap();
        let rows = Store::query(&c).unwrap();
        assert_eq!(
            rows.iter().find(|r| r.id == "user").unwrap().kind,
            "conversation"
        );
        assert_eq!(rows.iter().filter(|r| r.kind == "subtask").count(), 2);
        assert_eq!(rows.len(), 3);
    }
    fn fixture() -> Store {
        let home =
            std::env::temp_dir().join(format!("atelier-thread-source-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&home).unwrap();
        Store {
            backups: home.join("backups"),
            home,
        }
    }
    fn db(path: &Path) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        Connection::open(path)
            .unwrap()
            .execute_batch("CREATE TABLE threads(id TEXT PRIMARY KEY)")
            .unwrap();
    }
    #[test]
    fn current_sqlite_directory_precedes_legacy_root_and_explicit_config_is_respected() {
        let s = fixture();
        db(&s.home.join("state_99.sqlite"));
        db(&s.home.join("sqlite/codex-dev.db"));
        assert_eq!(
            database(&s, None, None).unwrap(),
            s.home.join("sqlite/codex-dev.db")
        );
        let custom = s.home.join("custom");
        db(&custom.join("state_7.sqlite"));
        fs::write(
            s.home.join("config.toml"),
            format!("sqlite_home={}\n", serde_json::to_string(&custom).unwrap()),
        )
        .unwrap();
        assert_eq!(
            database(&s, None, None).unwrap(),
            custom.join("state_7.sqlite")
        );
        fs::remove_file(custom.join("state_7.sqlite")).unwrap();
        assert!(database(&s, None, None).is_err());
        fs::remove_file(s.home.join("config.toml")).unwrap();
        assert_eq!(
            database(&s, Some(custom.as_os_str()), None).unwrap_err(),
            "当前配置目录没有可读取的会话索引"
        );
        fs::remove_dir_all(s.home).unwrap();
    }
    #[test]
    fn unrelated_preferred_database_does_not_hide_valid_thread_indexes() {
        let s = fixture();
        let auxiliary = s.home.join("sqlite/codex-dev.db");
        fs::create_dir_all(auxiliary.parent().unwrap()).unwrap();
        Connection::open(&auxiliary)
            .unwrap()
            .execute_batch("CREATE TABLE unrelated_settings(value TEXT)")
            .unwrap();
        db(&s.home.join("state_5.sqlite"));
        assert_eq!(
            database(&s, None, None).unwrap(),
            s.home.join("state_5.sqlite")
        );
        db(&s.home.join("sqlite/state_6.sqlite"));
        assert_eq!(
            database(&s, None, None).unwrap(),
            s.home.join("sqlite/state_6.sqlite")
        );
        fs::remove_file(s.home.join("sqlite/state_6.sqlite")).unwrap();
        fs::write(
            s.home.join("config.toml"),
            format!(
                "sqlite_home={}\n",
                serde_json::to_string(&s.home.join("sqlite")).unwrap()
            ),
        )
        .unwrap();
        assert!(database(&s, None, None).is_err());
        let c = Connection::open_with_flags(&auxiliary, OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
        assert_eq!(
            c.query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE name='threads'",
                [],
                |r| r.get::<_, i64>(0)
            )
            .unwrap(),
            0
        );
        drop(c);
        fs::remove_dir_all(s.home).unwrap();
    }
    #[test]
    fn classification_does_not_equate_missing_rollout_with_invalid_conversation() {
        assert_eq!(kind("", "user", "", false, "", true), "conversation");
        assert_eq!(kind("cli", "", "", true, "", true), "conversation");
        assert_eq!(kind("cli", "", "", false, "message", true), "conversation");
        assert_eq!(kind("cli", "", "", false, "", false), "conversation");
        assert_eq!(
            kind(
                r#"{"subagent":{"spawn":{"parent_thread_id":"p"}}}"#,
                "",
                "",
                true,
                "task",
                true
            ),
            "subtask"
        );
        assert_eq!(kind("cli", "", "p", true, "task", true), "subtask");
        assert_eq!(kind("cli", "", "", false, "", true), "unverified");
        assert_eq!(kind("", "", "", false, "", false), "unverified");
    }
    #[test]
    fn listing_keeps_index_only_user_chats_and_classifies_child_and_unknown_rows() {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch("CREATE TABLE threads(id TEXT,source TEXT,thread_source TEXT,parent_session_id TEXT,has_user_event INTEGER,first_user_message TEXT,updated_at_ms INTEGER); INSERT INTO threads VALUES('chat','','user','',0,'',1700000000123),('child','cli','','chat',1,'task',1700000000111),('old','','','',0,'',1700000000000);").unwrap();
        let rows = Store::query(&c).unwrap();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].kind, "conversation");
        assert_eq!(rows[0].updated_at, 1700000000123);
        assert!(rows[0].rollout_path.is_empty());
        assert_eq!(rows[1].kind, "subtask");
        assert_eq!(rows[2].kind, "unverified");
    }
    #[test]
    fn active_corrupt_database_does_not_fall_back_to_legacy_history() {
        let s = fixture();
        db(&s.home.join("state_5.sqlite"));
        fs::create_dir_all(s.home.join("sqlite")).unwrap();
        fs::write(s.home.join("sqlite/codex-dev.db"), b"invalid").unwrap();
        assert!(database(&s, None, None).is_err());
        fs::remove_dir_all(s.home).unwrap();
    }
}
