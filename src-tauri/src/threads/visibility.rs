//! Provider visibility repair never rewrites durable rollout contents.
use super::*;
use std::{
    collections::BTreeSet,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub(super) struct IndexChange {
    database: PathBuf,
    table: String,
    id: String,
    host: Option<String>,
    before: String,
    after: String,
    #[serde(default)]
    restoring: bool,
}
#[derive(Clone, Serialize, Deserialize)]
pub(super) struct FileTime {
    path: PathBuf,
    size: u64,
    before: (u64, u32),
    after: (u64, u32),
    identity: Option<(u64, u64)>,
    #[serde(default)]
    restoring: bool,
}
#[derive(Clone, Default, Serialize, Deserialize)]
pub(super) struct Plan {
    indexes: Vec<IndexChange>,
    times: Vec<FileTime>,
    #[serde(default)]
    metadata: Vec<super::quick_repair::Change>,
    #[serde(default)]
    roots: Vec<String>,
}
fn columns(c: &Connection, schema: &str, table: &str) -> Result<BTreeSet<String>> {
    c.prepare(&format!("PRAGMA {schema}.table_info({table})"))
        .map_err(err)?
        .query_map([], |r| r.get(1))
        .map_err(err)?
        .collect::<std::result::Result<_, _>>()
        .map_err(err)
}
fn databases(s: &Store) -> Result<Vec<PathBuf>> {
    let mut paths = vec![s.db()?];
    let dir = s.home.join("sqlite");
    if dir.exists() {
        for entry in fs::read_dir(dir).map_err(err)? {
            let path = entry.map_err(err)?.path();
            if path.extension().is_some_and(|e| e == "db" || e == "sqlite") {
                paths.push(path);
            }
        }
    }
    paths.sort();
    paths.dedup();
    if paths.len() > 10 {
        return Err("数据库超过单次事务上限，请先核对 sqlite 目录".into());
    }
    for path in &paths {
        let m = fs::symlink_metadata(path).map_err(err)?;
        if !m.is_file() || m.file_type().is_symlink() {
            return Err("会话索引必须是普通数据库文件".into());
        }
    }
    Ok(paths)
}
fn scan(
    c: &Connection,
    database: &Path,
    schema: &str,
    ids: &[String],
    target: &str,
) -> Result<Vec<IndexChange>> {
    let mut out = vec![];
    for (table, key) in [("threads", "id"), ("local_thread_catalog", "thread_id")] {
        let cols = columns(c, schema, table)?;
        if !cols.contains(key) || !cols.contains("model_provider") {
            continue;
        }
        let host = if table == "local_thread_catalog" {
            if !cols.contains("host_id") {
                return Err("本地目录缺少 host_id，不能区分远端记录".into());
            }
            let hosts = columns(c, schema, "local_thread_catalog_hosts")?;
            if !hosts.contains("host_id") {
                return Err("本地目录缺少宿主身份表，未修改".into());
            }
            let condition = if hosts.contains("host_kind") {
                "LOWER(COALESCE(host_kind,''))='local'"
            } else {
                "host_id='local'"
            };
            let mut stmt = c
                .prepare(&format!(
                    "SELECT host_id FROM {schema}.local_thread_catalog_hosts WHERE {condition}"
                ))
                .map_err(err)?;
            let hosts = stmt
                .query_map([], |r| r.get::<_, String>(0))
                .map_err(err)?
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(err)?;
            if hosts.len() != 1 {
                return Err("本地目录宿主不唯一，未修改".into());
            }
            Some(hosts[0].clone())
        } else {
            None
        };
        for id in ids {
            let condition = if host.is_some() {
                " AND host_id=?2"
            } else {
                ""
            };
            let sql=format!("SELECT COALESCE(model_provider,'') FROM {schema}.{table} WHERE {key}=?1{condition}");
            let mut stmt = c.prepare(&sql).map_err(err)?;
            let params: Vec<&dyn rusqlite::ToSql> = if let Some(host) = &host {
                vec![id, host]
            } else {
                vec![id]
            };
            let rows = stmt
                .query_map(params.as_slice(), |r| r.get::<_, String>(0))
                .map_err(err)?
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(err)?;
            if rows.len() > 1 {
                return Err("线程索引含重复身份，未修改".into());
            }
            if let Some(before) = rows.into_iter().next() {
                if before != target {
                    out.push(IndexChange {
                        database: database.to_owned(),
                        table: table.into(),
                        id: id.clone(),
                        host: host.clone(),
                        before,
                        after: target.into(),
                        restoring: false,
                    });
                }
            }
        }
    }
    Ok(out)
}
fn identity(m: &fs::Metadata) -> Option<(u64, u64)> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        Some((m.dev(), m.ino()))
    }
    #[cfg(not(unix))]
    {
        let _ = m;
        None
    }
}
fn stamp(t: SystemTime) -> Result<(u64, u32)> {
    let d = t.duration_since(UNIX_EPOCH).map_err(err)?;
    Ok((d.as_secs(), d.subsec_nanos()))
}
fn time(t: (u64, u32)) -> SystemTime {
    UNIX_EPOCH + Duration::new(t.0, t.1)
}
pub(super) fn prepare(s: &Store, rows: &[Thread], target: &str) -> Result<Plan> {
    if rows.iter().any(|r| r.kind == "subtask") {
        return Err("线程对齐只处理主对话；请取消选择子任务后重新预览".into());
    }
    let ids = rows.iter().map(|r| r.id.clone()).collect::<Vec<_>>();
    let mut plan = Plan::default();
    plan.roots = rows
        .iter()
        .filter(|r| r.kind == "conversation")
        .map(|r| r.id.clone())
        .collect();
    plan.metadata = super::quick_repair::prepare(&s.connection(false)?, rows)?;
    for database in databases(s)? {
        let c = Connection::open_with_flags(&database, OpenFlags::SQLITE_OPEN_READ_ONLY)
            .map_err(err)?;
        c.busy_timeout(Duration::from_secs(2)).map_err(err)?;
        plan.indexes.extend(
            scan(&c, &database, "main", &ids, target)?
                .into_iter()
                .filter(|e| database != s.db().unwrap_or_default() || e.table != "threads"),
        );
    }
    for row in rows {
        if row.rollout_path.is_empty() || row.updated_at <= 0 {
            continue;
        }
        let recorded = PathBuf::from(&row.rollout_path);
        let path = if recorded.is_absolute() {
            recorded
        } else {
            s.home.join(recorded)
        };
        // Missing and compressed histories can still have their SQLite visibility repaired.
        let path = if path.exists() {
            path
        } else {
            let compressed = PathBuf::from(format!("{}.zst", path.display()));
            if compressed.exists() {
                compressed
            } else {
                continue;
            }
        };
        let meta = fs::symlink_metadata(&path).map_err(err)?;
        let path = path.canonicalize().map_err(err)?;
        if !meta.is_file()
            || meta.file_type().is_symlink()
            || !["sessions", "archived_sessions"].iter().any(|d| {
                s.home
                    .join(d)
                    .canonicalize()
                    .is_ok_and(|p| path.starts_with(p))
            })
        {
            return Err("会话文件不在允许的目录中".into());
        }
        let before = stamp(meta.modified().map_err(err)?)?;
        let after = (
            (row.updated_at / 1000) as u64,
            ((row.updated_at % 1000) * 1_000_000) as u32,
        );
        if before != after {
            plan.times.push(FileTime {
                path,
                size: meta.len(),
                before,
                after,
                identity: identity(&meta),
                restoring: false,
            });
        }
    }
    Ok(plan)
}
impl Plan {
    pub fn without_times(mut self) -> Self {
        self.times.clear();
        self
    }
    pub fn change_count(&self) -> usize {
        self.indexes.len() + self.times.len() + self.metadata.len()
    }
    pub fn metadata(&self) -> &[super::quick_repair::Change] {
        &self.metadata
    }
    pub fn index_details(&self) -> Value {
        json!(self.indexes.iter().map(|i| json!({"id":i.id,"database":i.database,"table":i.table,"before":i.before,"after":i.after})).collect::<Vec<_>>())
    }
    pub fn time_details(&self) -> Value {
        json!(self.times.iter().map(|t| json!({"path":t.path,"before":t.before.0 as f64*1000.0+t.before.1 as f64/1_000_000.0,"after":t.after.0 as f64*1000.0+t.after.1 as f64/1_000_000.0})).collect::<Vec<_>>())
    }
    pub fn thread_ids(&self) -> BTreeSet<String> {
        self.indexes
            .iter()
            .map(|e| e.id.clone())
            .chain(self.metadata.iter().map(|e| e.id.clone()))
            .collect()
    }
    pub fn count(&self) -> usize {
        self.indexes.len()
    }
    pub fn inverse(&self) -> Self {
        Self {
            metadata: self.metadata.iter().map(|m| m.inverse()).collect(),
            roots: vec![],
            indexes: self
                .indexes
                .iter()
                .cloned()
                .map(|mut c| {
                    std::mem::swap(&mut c.before, &mut c.after);
                    c.restoring = true;
                    c
                })
                .collect(),
            times: self
                .times
                .iter()
                .cloned()
                .map(|mut t| {
                    std::mem::swap(&mut t.before, &mut t.after);
                    t.restoring = true;
                    t
                })
                .collect(),
        }
    }
    pub fn attach(&self, s: &Store, c: &Connection) -> Result<Vec<(PathBuf, String)>> {
        let allowed = databases(s)?;
        let main = s.db()?;
        let mut dbs = vec![];
        for path in self
            .indexes
            .iter()
            .map(|i| &i.database)
            .collect::<BTreeSet<_>>()
        {
            if !allowed.contains(path) {
                return Err("预览数据库集合已改变".into());
            }
            let alias = if *path == main {
                "main".to_string()
            } else {
                let alias = format!("visibility{}", dbs.len());
                c.execute(
                    &format!("ATTACH DATABASE ?1 AS {alias}"),
                    [path.to_str().ok_or("数据库路径不是UTF8")?],
                )
                .map_err(err)?;
                alias
            };
            dbs.push((path.clone(), alias));
        }
        Ok(dbs)
    }
    pub fn apply_indexes(&self, c: &Connection, dbs: &[(PathBuf, String)]) -> Result<()> {
        if !self.roots.is_empty()
            && Store::query(c)?
                .iter()
                .any(|r| self.roots.contains(&r.id) && r.kind != "conversation")
        {
            return Err("预览后会话类型发生变化，事务已取消".into());
        }
        for change in &self.metadata {
            change.apply(c)?;
        }
        for edge in &self.indexes {
            let schema = &dbs
                .iter()
                .find(|(p, _)| *p == edge.database)
                .ok_or("未连接审批数据库")?
                .1;
            let key = match edge.table.as_str() {
                "threads" => "id",
                "local_thread_catalog" => "thread_id",
                _ => return Err("无效目录表".into()),
            };
            let condition = if edge.host.is_some() {
                " AND host_id=?4"
            } else {
                ""
            };
            let sql=format!("UPDATE {schema}.{} SET model_provider=?1 WHERE {key}=?2 AND COALESCE(model_provider,'')=?3{condition}",edge.table);
            let mut params: Vec<&dyn rusqlite::ToSql> = vec![&edge.after, &edge.id, &edge.before];
            if let Some(host) = &edge.host {
                params.push(host);
            }
            if c.execute(&sql, params.as_slice()).map_err(err)? != 1 {
                let query = format!(
                    "SELECT COALESCE(model_provider,'') FROM {schema}.{} WHERE {key}=?1{}",
                    edge.table,
                    if edge.host.is_some() {
                        " AND host_id=?2"
                    } else {
                        ""
                    }
                );
                let mut values: Vec<&dyn rusqlite::ToSql> = vec![&edge.id];
                if let Some(host) = &edge.host {
                    values.push(host);
                }
                if !edge.restoring
                    || c.query_row(&query, values.as_slice(), |r| r.get::<_, String>(0))
                        .map_err(err)?
                        != edge.after
                {
                    return Err("目录 Provider 在预览后变化，事务已取消".into());
                }
                continue;
            }
            if edge.table == "local_thread_catalog" {
                let cols = columns(c, schema, "local_thread_catalog_metadata")?;
                if cols.contains("catalog_revision") {
                    c.execute(&format!("UPDATE {schema}.local_thread_catalog_metadata SET catalog_revision=catalog_revision+1"),[]).map_err(err)?;
                }
            }
        }
        Ok(())
    }
    pub fn apply_times(&self) -> Result<Vec<(fs::File, FileTime)>> {
        let mut applied = vec![];
        for t in &self.times {
            let result = (|| {
                let m = fs::symlink_metadata(&t.path).map_err(err)?;
                if !m.is_file() || m.file_type().is_symlink() {
                    return Err("会话文件身份变化".into());
                }
                let f = fs::File::open(&t.path).map_err(err)?;
                f.try_lock().map_err(err)?;
                let m = f.metadata().map_err(err)?;
                if m.len() != t.size
                    || identity(&m) != t.identity
                    || identity(&fs::symlink_metadata(&t.path).map_err(err)?) != t.identity
                {
                    return Err("会话文件已被替换，未修改".into());
                }
                let current = stamp(m.modified().map_err(err)?)?;
                if t.restoring && current == t.after {
                    return Ok(None);
                }
                if current != t.before {
                    return Err("会话在预览后更新，未覆盖时间戳".into());
                }
                f.set_modified(time(t.after)).map_err(err)?;
                if f.metadata().map_err(err)?.len() != t.size {
                    return Err("更新时间戳时会话发生新写入".into());
                }
                Ok(Some(f))
            })();
            match result {
                Ok(Some(f)) => applied.push((f, t.clone())),
                Ok(None) => (),
                Err(e) => {
                    Self::undo_times(&applied)?;
                    return Err(e);
                }
            }
        }
        Ok(applied)
    }
    pub fn undo_times(applied: &[(fs::File, FileTime)]) -> Result<()> {
        for (f, t) in applied.iter().rev() {
            let m = f.metadata().map_err(err)?;
            if m.len() != t.size
                || identity(&m) != t.identity
                || stamp(m.modified().map_err(err)?)? != t.after
            {
                return Err("会话又有新写入，保留最新时间戳与恢复记录".into());
            }
            f.set_modified(time(t.before)).map_err(err)?;
        }
        Ok(())
    }
}
