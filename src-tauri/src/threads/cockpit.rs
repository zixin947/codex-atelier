//! Atelier approval/journal adapter. The repair algorithm lives in unmodified upstream files.
use super::*;
use crate::gateway::storage::{atomic_private, open_private, private_dir, read_private};
use fs2::FileExt;
use sha2::{Digest, Sha256};
use std::io::Read;
#[derive(Clone, Serialize, Deserialize)]
struct Ticket {
    ticket: String,
    home: PathBuf,
    database: PathBuf,
    provider: String,
    target: String,
    ids: Vec<String>,
    deep: bool,
    report: Value,
    fingerprint: String,
    created_at: u64,
    #[serde(default)]
    rollback: bool,
}
fn path(s: &Store, id: &str, kind: &str) -> Result<PathBuf> {
    uuid::Uuid::parse_str(id).map_err(err)?;
    Ok(s.backups.join(format!("{id}.cockpit.{kind}.json")))
}
fn read(s: &Store, id: &str, kind: &str) -> Result<Ticket> {
    serde_json::from_slice(&read_private(&path(s, id, kind)?, 32 * 1024 * 1024)?).map_err(err)
}
fn save(s: &Store, t: &Ticket, kind: &str) -> Result<()> {
    private_dir(&s.backups)?;
    atomic_private(
        &path(s, &t.ticket, kind)?,
        &serde_json::to_vec(t).map_err(err)?,
    )
}
pub(super) fn pending(s: &Store, id: &str) -> Result<bool> {
    Ok(path(s, id, "pending")?.is_file())
}
pub(super) fn backed(s: &Store, id: &str) -> Result<bool> {
    Ok(path(s, id, "backup")?.is_file())
}
fn fingerprint(s: &Store, ids: &[String]) -> Result<String> {
    let mut paths = vec![
        s.db()?,
        s.home.join("config.toml"),
        s.home.join(".codex-global-state.json"),
        s.home.join("session_index.jsonl"),
    ];
    for dir in [&s.home, &s.home.join("sqlite")] {
        if dir.is_dir() {
            for e in fs::read_dir(dir).map_err(err)? {
                let p = e.map_err(err)?.path();
                let n = p.file_name().unwrap_or_default().to_string_lossy();
                if n.ends_with(".sqlite") || n.ends_with(".db") || n.ends_with("-wal") {
                    paths.push(p);
                }
            }
        }
    }
    paths.sort();
    paths.dedup();
    let mut h = Sha256::new();
    for p in paths {
        h.update(p.to_string_lossy().as_bytes());
        match fs::symlink_metadata(&p) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                h.update(b"missing");
                continue;
            }
            Err(e) => return Err(err(e)),
            Ok(m) if !m.is_file() || m.file_type().is_symlink() => {
                return Err("修复来源不是普通文件".into())
            }
            _ => (),
        }
        let mut f = fs::File::open(p).map_err(err)?;
        let mut buf = [0u8; 65536];
        loop {
            let n = f.read(&mut buf).map_err(err)?;
            if n == 0 {
                break;
            }
            h.update(&buf[..n]);
        }
    }
    for r in s.rows()?.iter().filter(|r| ids.contains(&r.id)) {
        h.update(r.id.as_bytes());
        let p = PathBuf::from(&r.rollout_path);
        let p = if p.is_absolute() { p } else { s.home.join(p) };
        if let Ok(m) = fs::metadata(p) {
            h.update(m.len().to_le_bytes());
            h.update(format!("{:?}", m.modified()).as_bytes());
        }
    }
    Ok(format!("{:x}", h.finalize()))
}
fn invoke(s: &Store, t: &Ticket, dry: bool) -> Result<Value> {
    #[cfg(test)]
    let binary: Option<PathBuf> = None;
    #[cfg(not(test))]
    let binary = crate::desktop_runtime::binary();
    cockpit_session_compat::with_host(
        &s.home,
        &s.backups.join("cockpit"),
        binary.as_deref(),
        || {
            cockpit_session_compat::run(&s.home, &t.target, t.ids.clone(), t.deep, dry)
                .and_then(|r| serde_json::to_value(r).map_err(err))
        },
    )
}
fn changes(report: &Value) -> u64 {
    [
        "changedRolloutFileCount",
        "updatedSqliteRowCount",
        "updatedSqliteTimestampRowCount",
        "addedSessionIndexEntryCount",
        "updatedSessionIndexEntryCount",
        "insertedCatalogRowCount",
        "removedCatalogRowCount",
        "updatedGlobalStateEntryCount",
    ]
    .iter()
    .map(|k| report[k].as_u64().unwrap_or(0))
    .sum::<u64>()
    .max(report["mutatedInstanceCount"].as_u64().unwrap_or(0))
}
fn view(t: &Ticket) -> Value {
    json!({"ticket":t.ticket,"action":if t.rollback{"restore"}else if t.deep{"migration"}else{"provider"},"targetProvider":t.target,"selectedCount":t.ids.len(),"changes":[],"visibilityChanges":if t.rollback{1}else{changes(&t.report)},"upstreamReport":t.report,"warnings":["使用线程修复引擎；保留会话文件字节，不重写 session_meta。",if t.deep{"深度修复执行前必须退出 Desktop；会更新派生目录、全局状态，并尝试官方元数据重建（上游可能规范化 config.toml）。"}else{"快速修复按上游侧边栏条件选择记录，可能跳过归档或子任务；不能把选中数量当作已修复数量。"},"执行时由线程修复引擎重新扫描。恢复数据库快照仍会检查修复后是否有新数据。"]})
}
pub(super) fn preview(s: &Store, ids: Vec<String>, deep: bool, target: String) -> Result<Value> {
    if ids.is_empty() || ids.len() > 10000 {
        return Err("请选择1–10000条线程".into());
    }
    let rows = s.rows()?;
    let unique = ids.iter().collect::<std::collections::BTreeSet<_>>();
    if unique.len() != ids.len() || ids.iter().any(|id| !rows.iter().any(|r| &r.id == id)) {
        return Err("线程范围无效，请刷新后重试".into());
    }
    if !s.provider_ids(&rows)?.contains(&target) {
        return Err("目标 Provider 未在配置或索引中发现".into());
    }
    let mut t = Ticket {
        ticket: uuid::Uuid::new_v4().to_string(),
        home: s.home.clone(),
        database: s.db()?,
        provider: s.provider()?,
        target,
        ids,
        deep,
        report: Value::Null,
        fingerprint: String::new(),
        created_at: now_ms(),
        rollback: false,
    };
    t.report = invoke(s, &t, true)?;
    save(s, &t, "pending")?;
    Ok(view(&t))
}
pub(super) fn apply(s: &Store, id: &str) -> Result<Value> {
    private_dir(&s.backups)?;
    let lock = open_private(&s.backups.join("mutation.lock"))?;
    lock.try_lock_exclusive()
        .map_err(|_| "另一线程修复正在执行")?;
    let mut t = read(s, id, "pending")?;
    if t.home != s.home {
        return Err("操作票据不属于当前 Codex 目录".into());
    }
    if t.rollback && fingerprint(s, &t.ids)? != t.fingerprint {
        return Err("修复后已有新数据，拒绝用旧快照覆盖".into());
    }
    save(s, &t, "intent")?;
    if t.rollback {
        super::migration::require_stopped()?;
        let root = s.backups.join("cockpit").canonicalize().map_err(err)?;
        for d in t.report["backupDirs"]
            .as_array()
            .ok_or("缺少上游备份目录")?
            .iter()
            .rev()
        {
            let p = PathBuf::from(d.as_str().ok_or("无效备份目录")?)
                .canonicalize()
                .map_err(err)?;
            if !p.starts_with(&root) {
                return Err("备份不属于 Atelier".into());
            }
            cockpit_session_compat::with_host(&s.home, &s.backups.join("cockpit"), None, || {
                cockpit_session_compat::restore(&s.home, &p)
            })?;
        }
        let _ = fs::remove_file(path(s, id, "pending")?);
        return Ok(
            json!({"message":"已按上游备份恢复；请核对客户端状态","backupDirs":t.report["backupDirs"]}),
        );
    }
    t.report = invoke(s, &t, false)?;
    match invoke(s, &t, true) {
        Ok(check) => { t.report["remainingChanges"] = json!(changes(&check)); }
        Err(e) => { t.report["verificationError"] = json!(e); }
    }
    t.fingerprint = match fingerprint(s, &t.ids) {
        Ok(value) => value,
        Err(_) => {
            t.report["message"] = json!(format!(
                "{}；后台数据继续更新，Atelier 快照恢复校验不可用；上游备份已保留，请勿重复修复",
                t.report["message"].as_str().unwrap_or("原版修复已执行")
            ));
            String::new()
        }
    };
    save(s, &t, "backup").map_err(|e| {
        format!(
            "上游修复已执行，但 Atelier 回执保存失败：{e}；请勿重复执行；上游备份：{}",
            t.report["backupDirs"]
        )
    })?;
    let _ = fs::remove_file(path(s, id, "pending")?);
    Ok(t.report)
}
pub(super) fn backups(s: &Store) -> Result<Vec<Value>> {
    let mut out = vec![];
    if !s.backups.exists() {
        return Ok(out);
    }
    for e in fs::read_dir(&s.backups).map_err(err)? {
        let n = e.map_err(err)?.file_name().to_string_lossy().into_owned();
        if let Some(id) = n.strip_suffix(".cockpit.backup.json") {
            let t = read(s, id, "backup")?;
            out.push(json!({"id":id,"createdAt":fs::metadata(path(s,id,"backup")?).map_err(err)?.modified().map_err(err)?.duration_since(std::time::UNIX_EPOCH).map_err(err)?.as_millis(),"action":"线程修复","count":t.ids.len()}));
        }
    }
    Ok(out)
}
pub(super) fn restore(s: &Store, id: &str) -> Result<Value> {
    let mut t = read(s, id, "backup")?;
    if fingerprint(s, &t.ids)? != t.fingerprint {
        return Err("修复之后已有新数据，拒绝用数据库快照覆盖".into());
    }
    t.ticket = uuid::Uuid::new_v4().to_string();
    t.rollback = true;
    t.created_at = now_ms();
    save(s, &t, "pending")?;
    Ok(view(&t))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Store {
        let home =
            std::env::temp_dir().join(format!("atelier-cockpit-adapter-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(home.join("sessions")).unwrap();
        fs::write(
            home.join("config.toml"),
            "model_provider='openai'\n[model_providers.openai_https]\nname='Other'\n",
        )
        .unwrap();
        fs::write(home.join("sessions/rollout-a.jsonl"),b"{\"type\":\"session_meta\",\"payload\":{\"id\":\"a\",\"model_provider\":\"openai\"}}\n").unwrap();
        let c = Connection::open(home.join("state_5.sqlite")).unwrap();
        c.execute_batch("CREATE TABLE threads(id TEXT PRIMARY KEY,title TEXT,cwd TEXT,model_provider TEXT,updated_at INTEGER,created_at INTEGER,archived INTEGER,rollout_path TEXT,source TEXT,thread_source TEXT,has_user_event INTEGER,first_user_message TEXT,preview TEXT); INSERT INTO threads VALUES('a','Fixture','/fixture','openai',1700000000,1700000000,0,'sessions/rollout-a.jsonl','cli','user',1,'hello','hello');").unwrap();
        Store {
            backups: home.join("atelier-backups"),
            home,
        }
    }
    #[test]
    fn original_all_scope_uses_null_without_frontend_thread_filter() {
        let s = fixture();
        let c = s.connection(true).unwrap();
        c.execute_batch("INSERT INTO threads SELECT 'b',title,cwd,model_provider,updated_at,created_at,archived,rollout_path,source,thread_source,has_user_event,first_user_message,preview FROM threads WHERE id='a';").unwrap();
        drop(c);
        let report = cockpit_session_compat::with_host(&s.home, &s.backups, None, || {
            cockpit_session_compat::repair_session_visibility_across_instances_with_target(
                cockpit_session_compat::CodexSessionVisibilityRepairMode::Quick,
                None, None, Some("openai_https".into()), None, None, false)
        }).unwrap();
        assert!(report.updated_sqlite_row_count >= 2);
        assert_eq!(s.row("a").unwrap().provider, "openai_https");
        assert_eq!(s.row("b").unwrap().provider, "openai_https");
        fs::remove_dir_all(s.home).unwrap();
    }
    #[test]
    fn actual_adapter_preview_apply_and_restore_keep_durable_history_bytes() {
        let s = fixture();
        let path = s.home.join("sessions/rollout-a.jsonl");
        let bytes = fs::read(&path).unwrap();
        let p = preview(&s, vec!["a".into()], false, "openai_https".into()).unwrap();
        let id = p["ticket"].as_str().unwrap();
        assert_eq!(s.row("a").unwrap().provider, "openai");
        assert_eq!(p["upstreamReport"]["updatedSqliteRowCount"], 1);
        let r = apply(&s, id).unwrap();
        assert_eq!(s.row("a").unwrap().provider, "openai_https");
        assert_eq!(fs::read(&path).unwrap(), bytes);
        let second=preview(&s,vec!["a".into()],false,"openai_https".into()).unwrap();
        assert_eq!(second["visibilityChanges"],0,"second preview: {}",second);
        assert_eq!(r["remainingChanges"],0);
        let repeated=apply(&s,second["ticket"].as_str().unwrap()).unwrap();
        assert_eq!(repeated["remainingChanges"],0);
        assert_eq!(repeated["updatedSqliteRowCount"],0);
        assert!(!r["backupDirs"].as_array().unwrap().is_empty());
        assert_eq!(backups(&s).unwrap().len(), 2);
        assert!(s.list_backups().unwrap().as_array().unwrap().is_empty());
        let restore = restore(&s, id).unwrap();
        apply(&s, restore["ticket"].as_str().unwrap()).unwrap();
        assert_eq!(s.row("a").unwrap().provider, "openai");
        assert_eq!(fs::read(path).unwrap(), bytes);
        fs::remove_dir_all(s.home).unwrap();
    }
    #[test]
    fn normal_background_updates_do_not_block_repair_but_restore_refuses_new_data() {
        let s = fixture();
        let live = s.connection(true).unwrap();
        live.execute_batch("PRAGMA journal_mode=WAL; CREATE TABLE background_state(value INTEGER); INSERT INTO background_state VALUES(0);").unwrap();
        let p = preview(&s, vec!["a".into()], false, "openai_https".into()).unwrap();
        live.execute("UPDATE background_state SET value=1", [])
            .unwrap();
        live.execute_batch("PRAGMA wal_checkpoint(TRUNCATE)")
            .unwrap();
        {
            use std::io::Write;
            let mut file = fs::OpenOptions::new()
                .append(true)
                .open(s.home.join("sessions/rollout-a.jsonl"))
                .unwrap();
            writeln!(
                file,
                "{{\"type\":\"event_msg\",\"payload\":{{\"type\":\"token_count\"}}}}"
            )
            .unwrap();
        }
        s.connection(true)
            .unwrap()
            .execute("UPDATE threads SET title='external'", [])
            .unwrap();
        apply(&s, p["ticket"].as_str().unwrap()).unwrap();
        assert_eq!(s.row("a").unwrap().provider, "openai_https");
        assert_eq!(s.row("a").unwrap().title, "external");
        s.connection(true)
            .unwrap()
            .execute("UPDATE threads SET model_provider='openai'", [])
            .unwrap();
        let p = preview(&s, vec!["a".into()], false, "openai_https".into()).unwrap();
        let id = p["ticket"].as_str().unwrap();
        apply(&s, id).unwrap();
        s.connection(true)
            .unwrap()
            .execute("UPDATE threads SET title='new progress'", [])
            .unwrap();
        assert!(restore(&s, id).is_err());
        drop(live);
        fs::remove_dir_all(s.home).unwrap();
    }
}
