use super::*;
use crate::gateway::storage::{atomic_private, open_private, private_dir, read_private};
use fs2::FileExt;
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Change {
    id: String,
    title: String,
    before: String,
    after: String,
    updated_at: i64,
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Plan {
    ticket: String,
    action: String,
    field: String,
    changes: Vec<Change>,
    warnings: Vec<String>,
    created_at: u64,
    database: String,
    provider: String,
    #[serde(default)]
    target_provider: Option<String>,
    #[serde(default)]
    selected_count: usize,
    #[serde(default)]
    rollouts: Vec<super::migration::RolloutChange>,
    #[serde(default)]
    visibility: Option<super::visibility::Plan>,
}
impl Store {
    fn ticket_path(&self, id: &str, suffix: &str) -> Result<PathBuf> {
        uuid::Uuid::parse_str(id).map_err(|_| "无效的操作标识")?;
        Ok(self.backups.join(format!("{id}.{suffix}.json")))
    }
    fn save_plan(&self, p: &Plan) -> Result<Value> {
        let bytes = serde_json::to_vec(p).map_err(err)?;
        if bytes.len() > 32 * 1024 * 1024 {
            return Err("预览备份超过 32 MiB，请缩小线程范围后重试".into());
        }
        private_dir(&self.backups)?;
        atomic_private(&self.ticket_path(&p.ticket, "pending")?, &bytes)?;
        let mut view = serde_json::to_value(p).map_err(err)?;
        view["visibilityChanges"] = json!(p.visibility.as_ref().map_or(0, |v| v.change_count()));
        view["metadataChanges"] = json!(p.visibility.as_ref().map(|v| v.metadata()).unwrap_or(&[]));
        if let Some(v) = &p.visibility {
            view["indexChanges"] = v.index_details();
            view["fileTimeChanges"] = v.time_details();
        }
        Ok(view)
    }
    fn read_plan(&self, id: &str, suffix: &str) -> Result<Plan> {
        serde_json::from_slice(&read_private(
            &self.ticket_path(id, suffix)?,
            32 * 1024 * 1024,
        )?)
        .map_err(err)
    }
    pub(super) fn preview(&self, ids: Vec<String>, action: String, value: String) -> Result<Value> {
        let limit = if matches!(action.as_str(), "provider" | "migration") {
            10000
        } else {
            100
        };
        if ids.is_empty() || ids.len() > limit {
            return Err(format!("每次请选择 1–{limit} 条线程"));
        }
        let selected_count = ids.len();
        let ids = ids.into_iter().collect::<std::collections::BTreeSet<_>>();
        if ids.len() != selected_count {
            return Err("线程 ID 重复，请刷新后重试".into());
        }
        let field = match action.as_str() {
            "provider" | "migration" => "model_provider",
            "archive" => "archived",
            _ => return Err("不支持的操作".into()),
        };
        if action == "archive" && !matches!(value.as_str(), "true" | "false") {
            return Err("归档状态无效".into());
        }
        let provider = self.provider()?;
        let c = self.connection(false)?;
        c.prepare(&format!("SELECT {field},updated_at FROM threads LIMIT 0"))
            .map_err(|_| "当前数据库不支持此项修改")?;
        let rows = self.rows()?;
        if matches!(action.as_str(), "provider" | "migration")
            && !self.provider_ids(&rows)?.contains(&value)
        {
            return Err("目标 Provider 不在当前配置或线程索引中，请刷新后重新选择".into());
        }
        if ids.iter().any(|id| !rows.iter().any(|r| &r.id == id)) {
            return Err("部分线程已不在索引中，请刷新".into());
        }
        let visibility = if matches!(action.as_str(), "provider" | "migration") {
            let v = super::visibility::prepare(
                self,
                &rows
                    .iter()
                    .filter(|r| ids.contains(&r.id))
                    .cloned()
                    .collect::<Vec<_>>(),
                &value,
            )?;
            Some(if action == "migration" {
                v.without_times()
            } else {
                v
            })
        } else {
            None
        };
        let rollouts = if action == "migration" {
            super::migration::prepare(
                self,
                &rows
                    .iter()
                    .filter(|r| ids.contains(&r.id))
                    .cloned()
                    .collect::<Vec<_>>(),
                &value,
            )?
        } else {
            vec![]
        };
        let changes = rows
            .into_iter()
            .filter(|r| ids.contains(&r.id))
            .filter_map(|r| {
                let before = if matches!(action.as_str(), "provider" | "migration") {
                    r.provider
                } else {
                    r.archived.to_string()
                };
                (before != value).then(|| Change {
                    id: r.id,
                    title: r.title,
                    before,
                    after: value.clone(),
                    updated_at: r.updated_at,
                })
            })
            .collect();
        let mut warnings = vec![
            "仅修改所选线程的官方 SQLite 索引；不修改 config.toml、会话正文或认证资料。".into(),
            "请先结束所选对话的生成。客户端可能持有旧状态或重新写回索引；操作后请在客户端核对。"
                .into(),
        ];
        if action == "provider" {
            warnings = vec![
                "对齐所选主对话的 Provider、本地派生目录及缺失的用户事件/来源/摘要标记，并对齐关联文件时间戳；聊天文件内容保持原样。".into(),
                format!("另有 {} 条派生索引待对齐；远端宿主记录不修改。",visibility.as_ref().map_or(0,|p|p.count())),
                "支持客户端运行时修复；预览后发生写入将取消操作。加密历史不转换，续聊结果由你在 Desktop 验收。".into(),
            ];
            warnings.insert(0, format!("将 {selected_count} 条主对话统一到目标 Provider：{value}；当前配置仍为 {provider}，本操作不切换连接配置。"));
        }
        if action == "migration" {
            warnings=vec![format!("完整迁移 {selected_count} 条主对话到 {value}：同时对齐主索引、本地派生目录和会话文件 session_meta。执行前必须退出 Desktop 及其引擎；Atelier 不会自动关闭客户端。"),"只重写会话元数据；其他记录按原字节保存。加密内容不转换，跨账号续聊仍可能被上游拒绝。".into(),format!("将备份并迁移 {} 个会话文件。单文件上限 32 MiB、全批 128 MiB；压缩或缺失文件会阻止整批迁移，不会跳过后宣称成功。",rollouts.len())];
        }
        let target_provider =
            matches!(action.as_str(), "provider" | "migration").then(|| value.clone());
        self.save_plan(&Plan {
            ticket: uuid::Uuid::new_v4().to_string(),
            action,
            field: field.into(),
            changes,
            warnings,
            created_at: now_ms(),
            database: self.db()?.display().to_string(),
            provider,
            target_provider,
            selected_count,
            rollouts,
            visibility,
        })
    }
    pub(super) fn apply(&self, id: &str) -> Result<()> {
        private_dir(&self.backups)?;
        let lock = open_private(&self.backups.join("mutation.lock"))?;
        lock.try_lock_exclusive()
            .map_err(|_| "另一线程管理操作正在执行")?;
        let p = self.read_plan(id, "pending")?;
        #[cfg(not(test))]
        if matches!(p.action.as_str(), "provider" | "migration") {
            return Err("旧版线程对齐预览已停用，请重新打开线程修复并预览".into());
        }
        if now_ms().saturating_sub(p.created_at) > 10 * 60 * 1000 {
            return Err("预览已过期，请重新检查".into());
        }
        if p.database != self.db()?.display().to_string() || p.provider != self.provider()? {
            return Err("数据库或配置已发生变化，请重新预览".into());
        }
        if matches!(p.action.as_str(), "provider" | "migration") {
            if let Some(target) = &p.target_provider {
                if !self.provider_ids(&self.rows()?)?.contains(target) {
                    return Err("目标 Provider 已不可用，请重新预览".into());
                }
            }
        }
        if !matches!(p.field.as_str(), "model_provider" | "archived") {
            return Err("无效的修改字段".into());
        }
        if p.action == "migration" || !p.rollouts.is_empty() {
            super::migration::require_stopped()?;
            for r in &p.rollouts {
                super::migration::verify(self, r)?;
            }
        }
        let mut c = self.connection(true)?;
        let attached = p
            .visibility
            .as_ref()
            .map(|v| v.attach(self, &c))
            .transpose()?
            .unwrap_or_default();
        let tx = c
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(err)?;
        for change in &p.changes {
            let (raw, updated): (Value, i64) = {
                let sql = format!("SELECT {},updated_at FROM threads WHERE id=?1", p.field);
                tx.query_row(&sql, [&change.id], |r| {
                    let value = if p.field == "archived" {
                        Value::Bool(r.get::<_, i64>(0)? != 0)
                    } else {
                        Value::String(r.get(0)?)
                    };
                    Ok((value, r.get(1)?))
                })
                .map_err(err)?
            };
            let actual = raw
                .as_str()
                .map(str::to_string)
                .unwrap_or_else(|| raw.to_string());
            let ms = if updated > 0 && updated < 10_000_000_000 {
                updated * 1000
            } else {
                updated
            };
            if actual != change.before || ms != change.updated_at {
                return Err(format!(
                    "线程「{}」已被外部更新，未执行任何修改",
                    change.title
                ));
            }
        }
        // Journal is durable before any DB write. If process exits after commit, this still permits recovery.
        atomic_private(
            &self.ticket_path(id, "backup")?,
            &serde_json::to_vec(&p).map_err(err)?,
        )?;
        for change in &p.changes {
            let sql = format!("UPDATE threads SET {}=?1 WHERE id=?2", p.field);
            let value = if p.field == "archived" {
                rusqlite::types::Value::Integer(i64::from(change.after == "true"))
            } else {
                rusqlite::types::Value::Text(change.after.clone())
            };
            if tx
                .execute(&sql, rusqlite::params![value, change.id])
                .map_err(err)?
                != 1
            {
                return Err("线程记录数量发生变化，已回滚".into());
            }
        }
        if let Some(visibility) = &p.visibility {
            visibility.apply_indexes(&tx, &attached)?;
        }
        let mut written = Vec::new();
        for rollout in &p.rollouts {
            if let Err(error) = super::migration::verify(self, rollout)
                .and_then(|_| super::migration::write(self, rollout, false))
            {
                let mut recovery = Vec::new();
                for previous in written.iter().rev() {
                    if let Err(e) = super::migration::write(self, previous, true) {
                        recovery.push(e)
                    }
                }
                return Err(format!(
                    "{error}; 数据库未提交；文件回退错误：{recovery:?}；备份 {}",
                    p.ticket
                ));
            }
            written.push(rollout.clone());
        }
        let file_times = p
            .visibility
            .as_ref()
            .map(|v| v.apply_times())
            .transpose()?
            .unwrap_or_default();
        if let Err(error) = tx.commit() {
            let times_error = super::visibility::Plan::undo_times(&file_times).err();
            let mut recovery = Vec::new();
            for previous in written.iter().rev() {
                if let Err(e) = super::migration::write(self, previous, true) {
                    recovery.push(e)
                }
            }
            return Err(format!(
                "数据库提交失败：{error}；时间戳回退：{times_error:?}；文件回退错误：{recovery:?}；备份 {}",
                p.ticket
            ));
        }
        // A failed cleanup does not turn a successful database commit into a reported failure.
        let _ = fs::remove_file(self.ticket_path(id, "pending")?);
        Ok(())
    }
    pub(super) fn list_backups(&self) -> Result<Value> {
        if !self.backups.exists() {
            return Ok(json!([]));
        }
        let mut list = vec![];
        for entry in fs::read_dir(&self.backups).map_err(err)? {
            let entry = entry.map_err(err)?;
            let n = entry.file_name().to_string_lossy().into_owned();
            if n.ends_with(".cockpit.backup.json") {
                continue;
            }
            if let Some(id) = n.strip_suffix(".backup.json") {
                let p = self.read_plan(id, "backup")?;
                let mut ids = p
                    .visibility
                    .as_ref()
                    .map(|v| v.thread_ids())
                    .unwrap_or_default();
                ids.extend(p.changes.iter().map(|c| c.id.clone()));
                list.push(json!({"id":p.ticket,"createdAt":p.created_at,"action":p.action,"count":ids.len()}));
            }
        }
        list.sort_by_key(|v| std::cmp::Reverse(v["createdAt"].as_i64().unwrap_or(0)));
        Ok(json!(list))
    }
    pub(super) fn restore(&self, id: &str) -> Result<Value> {
        let old = self.read_plan(id, "backup")?;
        if old.database != self.db()?.display().to_string() {
            return Err("备份不属于当前数据库".into());
        }
        let visibility = old.visibility.as_ref().map(|v| v.inverse());
        let rows = self.rows()?;
        let rollouts = super::migration::inverse(self, &old.rollouts)?;
        let mut changes = vec![];
        for change in old.changes {
            let row = rows
                .iter()
                .find(|r| r.id == change.id)
                .ok_or("原线程已删除，不能恢复")?;
            let current = if old.field == "archived" {
                row.archived.to_string()
            } else {
                row.provider.clone()
            };
            if current == change.before {
                continue;
            }
            if current != change.after {
                return Err("备份之后记录已被修改，拒绝覆盖".into());
            }
            changes.push(Change {
                id: change.id,
                title: row.title.clone(),
                before: current,
                after: change.before,
                updated_at: row.updated_at,
            })
        }
        self.save_plan(&Plan {
            ticket: uuid::Uuid::new_v4().to_string(),
            action: "restore".into(),
            field: old.field,
            changes,
            warnings: vec!["恢复所选备份修改前的索引字段；外部变化会阻止覆盖。".into()],
            created_at: now_ms(),
            database: old.database,
            provider: self.provider()?,
            target_provider: None,
            selected_count: old.selected_count,
            rollouts,
            visibility,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compressed_full_migration_is_explicitly_rejected_without_writes() {
        let s = fixture();
        let path = s.home.join("sessions/a.jsonl.zst");
        let bytes = zstd::stream::encode_all(b"example".as_slice(), 1).unwrap();
        fs::write(&path, &bytes).unwrap();
        s.connection(true)
            .unwrap()
            .execute("UPDATE threads SET rollout_path='sessions/a.jsonl.zst'", [])
            .unwrap();
        let error = s
            .preview(vec!["a".into()], "migration".into(), "atelier".into())
            .unwrap_err();
        assert!(error.contains("压缩"));
        assert_eq!(s.row("a").unwrap().provider, "old");
        assert_eq!(fs::read(path).unwrap(), bytes);
        fs::remove_dir_all(s.home).unwrap();
    }
    #[test]
    fn complete_migration_repairs_rollout_and_catalog_even_when_main_provider_matches() {
        let s = fixture();
        let d = derived(&s);
        let c = s.connection(true).unwrap();
        c.execute_batch("ALTER TABLE threads ADD COLUMN source TEXT DEFAULT 'cli'; UPDATE threads SET model_provider='openai_https';").unwrap();
        fs::write(s.home.join("config.toml"), "model_provider='openai_https'").unwrap();
        let bytes=b"{\"type\":\"session_meta\",\"payload\":{\"id\":\"a\",\"model_provider\":\"openai\"}}\n{\"type\":\"response_item\",\"payload\":{\"encrypted_content\":\"untouched\"}}\n";
        fs::write(s.home.join("sessions/a.jsonl"), bytes).unwrap();
        let p = s
            .preview(vec!["a".into()], "migration".into(), "openai_https".into())
            .unwrap();
        assert!(p["changes"].as_array().unwrap().is_empty());
        assert_eq!(p["rollouts"].as_array().unwrap().len(), 1);
        s.apply(p["ticket"].as_str().unwrap()).unwrap();
        let after = fs::read(s.home.join("sessions/a.jsonl")).unwrap();
        assert_eq!(
            serde_json::from_slice::<Value>(after.split(|b| *b == b'\n').next().unwrap()).unwrap()
                ["payload"]["model_provider"],
            "openai_https"
        );
        assert_eq!(
            after.split(|b| *b == b'\n').nth(1),
            bytes.split(|b| *b == b'\n').nth(1)
        );
        assert_eq!(
            d.query_row("SELECT model_provider FROM threads WHERE id='a'", [], |r| r
                .get::<_, String>(0))
                .unwrap(),
            "openai_https"
        );
        let list = s.catalog().unwrap();
        assert_eq!(list["listingSource"], "index");
        assert_eq!(list["counts"]["conversations"], 1);
        let restore = s.restore(p["ticket"].as_str().unwrap()).unwrap();
        s.apply(restore["ticket"].as_str().unwrap()).unwrap();
        assert_eq!(fs::read(s.home.join("sessions/a.jsonl")).unwrap(), bytes);
        assert_eq!(
            d.query_row("SELECT model_provider FROM threads WHERE id='a'", [], |r| r
                .get::<_, String>(0))
                .unwrap(),
            "old"
        );
        drop(d);
        drop(c);
        fs::remove_dir_all(s.home).unwrap();
    }
    #[test]
    fn catalog_preserves_other_provider_rows_after_alignment() {
        let s = fixture();
        let c = s.connection(true).unwrap();
        c.execute_batch("ALTER TABLE threads ADD COLUMN source TEXT DEFAULT 'cli'; INSERT INTO threads(id,title,model_provider,updated_at,created_at,archived) VALUES('b','Other','openai_https',1700000000,1700000000,0);").unwrap();
        let before = s.catalog().unwrap();
        assert_eq!(before["counts"]["conversations"], 2);
        let p = s
            .preview(vec!["a".into()], "provider".into(), "openai_https".into())
            .unwrap();
        s.apply(p["ticket"].as_str().unwrap()).unwrap();
        let after = s.catalog().unwrap();
        assert_eq!(after["threads"].as_array().unwrap().len(), 2);
        assert_eq!(after["counts"]["conversations"], 2);
        drop(c);
        fs::remove_dir_all(s.home).unwrap();
    }
    #[test]
    fn chosen_provider_applies_to_more_than_one_page_and_rolls_back_without_switching_config() {
        let s = fixture();
        let c = s.connection(true).unwrap();
        c.execute_batch("ALTER TABLE threads ADD COLUMN source TEXT DEFAULT 'cli'")
            .unwrap();
        let config = "model_provider='atelier'\n[model_providers.openai_https]\nname='Relay'\n";
        fs::write(s.home.join("config.toml"), config).unwrap();
        for n in 0..100 {
            c.execute("INSERT INTO threads (id,title,cwd,model_provider,updated_at,created_at,archived,tokens_used,rollout_path) VALUES (?1,'extra','/project','old',1700000000,1700000000,0,0,'')", [format!("extra-{n}")]).unwrap();
        }
        let ids = s
            .rows()
            .unwrap()
            .iter()
            .map(|r| r.id.clone())
            .collect::<Vec<_>>();
        assert!(s.catalog().unwrap()["providerOptions"]
            .as_array()
            .unwrap()
            .contains(&json!("openai_https")));
        let p = s
            .preview(ids, "provider".into(), "openai_https".into())
            .unwrap();
        assert_eq!(p["selectedCount"], 101);
        assert_eq!(p["targetProvider"], "openai_https");
        assert_eq!(p["changes"].as_array().unwrap().len(), 101);
        s.apply(p["ticket"].as_str().unwrap()).unwrap();
        assert!(s
            .rows()
            .unwrap()
            .iter()
            .all(|r| r.provider == "openai_https"));
        assert_eq!(
            fs::read_to_string(s.home.join("config.toml")).unwrap(),
            config
        );
        let restore = s.restore(p["ticket"].as_str().unwrap()).unwrap();
        s.apply(restore["ticket"].as_str().unwrap()).unwrap();
        assert!(s.rows().unwrap().iter().all(|r| r.provider == "old"));
        fs::remove_dir_all(s.home).unwrap();
    }
    #[test]
    fn index_provider_is_a_valid_target_but_unknown_or_removed_target_is_rejected() {
        let s = fixture();
        assert!(s
            .preview(vec!["a".into()], "provider".into(), "old".into())
            .is_ok());
        assert!(s
            .preview(vec!["a".into()], "provider".into(), "unseen".into())
            .is_err());
        fs::write(
            s.home.join("config.toml"),
            "model_provider='atelier'\n[model_providers.other]\nname='Other'\n",
        )
        .unwrap();
        let p = s
            .preview(vec!["a".into()], "provider".into(), "other".into())
            .unwrap();
        fs::write(s.home.join("config.toml"), "model_provider='atelier'").unwrap();
        assert!(s.apply(p["ticket"].as_str().unwrap()).is_err());
        assert_eq!(s.row("a").unwrap().provider, "old");
        fs::remove_dir_all(s.home).unwrap();
    }
    #[test]
    fn quick_alignment_repairs_markers_and_preserves_nulls_on_rollback() {
        let s = fixture();
        let c = s.connection(true).unwrap();
        c.execute_batch("ALTER TABLE threads ADD COLUMN source TEXT DEFAULT 'cli'; ALTER TABLE threads ADD COLUMN first_user_message TEXT; ALTER TABLE threads ADD COLUMN has_user_event INTEGER; ALTER TABLE threads ADD COLUMN thread_source TEXT; ALTER TABLE threads ADD COLUMN preview TEXT; UPDATE threads SET model_provider='atelier',first_user_message='hello',has_user_event=0,thread_source=NULL,preview='';").unwrap();
        let p = s
            .preview(vec!["a".into()], "provider".into(), "atelier".into())
            .unwrap();
        assert_eq!(p["metadataChanges"].as_array().unwrap().len(), 3);
        assert!(p["changes"].as_array().unwrap().is_empty());
        assert_eq!(
            c.query_row("SELECT has_user_event FROM threads", [], |r| r
                .get::<_, i64>(0))
                .unwrap(),
            0
        );
        s.apply(p["ticket"].as_str().unwrap()).unwrap();
        assert_eq!(s.list_backups().unwrap()[0]["count"], 1);
        assert_eq!(
            c.query_row(
                "SELECT has_user_event,thread_source,preview FROM threads",
                [],
                |r| Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?
                ))
            )
            .unwrap(),
            (1, "user".into(), "hello".into())
        );
        let restore = s.restore(p["ticket"].as_str().unwrap()).unwrap();
        s.apply(restore["ticket"].as_str().unwrap()).unwrap();
        assert_eq!(
            c.query_row(
                "SELECT has_user_event,thread_source,preview FROM threads",
                [],
                |r| Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, Option<String>>(1)?,
                    r.get::<_, String>(2)?
                ))
            )
            .unwrap(),
            (0, None, "".into())
        );
        fs::remove_dir_all(s.home).unwrap();
    }
    #[test]
    fn quick_alignment_cancels_on_new_message_or_changed_thread_role() {
        for change_role in [false, true] {
            let s = fixture();
            let c = s.connection(true).unwrap();
            let d = derived(&s);
            c.execute_batch("ALTER TABLE threads ADD COLUMN source TEXT DEFAULT 'cli'; ALTER TABLE threads ADD COLUMN first_user_message TEXT; ALTER TABLE threads ADD COLUMN has_user_event INTEGER DEFAULT 0; UPDATE threads SET first_user_message='first'; CREATE TABLE thread_spawn_edges(child_thread_id TEXT);").unwrap();
            let p = s
                .preview(vec!["a".into()], "provider".into(), "atelier".into())
                .unwrap();
            if change_role {
                c.execute("INSERT INTO thread_spawn_edges VALUES('a')", [])
                    .unwrap();
            } else {
                c.execute("UPDATE threads SET first_user_message='external'", [])
                    .unwrap();
            }
            assert!(s.apply(p["ticket"].as_str().unwrap()).is_err());
            assert_eq!(s.row("a").unwrap().provider, "old");
            assert_eq!(
                d.query_row("SELECT model_provider FROM threads WHERE id='a'", [], |r| r
                    .get::<_, String>(0))
                    .unwrap(),
                "old"
            );
            if change_role {
                assert!(s
                    .preview(vec!["a".into()], "provider".into(), "atelier".into())
                    .is_err());
            }
            drop(d);
            drop(c);
            fs::remove_dir_all(s.home).unwrap();
        }
    }
    fn fixture() -> Store {
        let root =
            std::env::temp_dir().join(format!("atelier-thread-test-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(root.join("sessions")).unwrap();
        let c = Connection::open(root.join("state_5.sqlite")).unwrap();
        c.execute_batch("CREATE TABLE threads(id TEXT PRIMARY KEY,title TEXT,cwd TEXT,model_provider TEXT,updated_at INTEGER,created_at INTEGER,archived INTEGER,tokens_used INTEGER,rollout_path TEXT); INSERT INTO threads VALUES('a','Test','/project','old',1700000000,1700000000,0,100,'sessions/a.jsonl');").unwrap();
        fs::write(root.join("config.toml"), "model_provider='atelier'").unwrap();
        Store {
            backups: root.join("backups"),
            home: root,
        }
    }
    fn derived(s: &Store) -> Connection {
        fs::create_dir_all(s.home.join("sqlite")).unwrap();
        let c = Connection::open(s.home.join("sqlite/catalog.db")).unwrap();
        c.execute_batch("CREATE TABLE threads(id TEXT PRIMARY KEY,model_provider TEXT); INSERT INTO threads VALUES('a','old'); CREATE TABLE local_thread_catalog_hosts(host_id TEXT,host_kind TEXT); INSERT INTO local_thread_catalog_hosts VALUES('mine','local'),('other','remote'); CREATE TABLE local_thread_catalog(thread_id TEXT,host_id TEXT,model_provider TEXT); INSERT INTO local_thread_catalog VALUES('a','mine','old'),('a','other','remote-provider'); CREATE TABLE local_thread_catalog_metadata(catalog_revision INTEGER); INSERT INTO local_thread_catalog_metadata VALUES(4);").unwrap();
        c
    }
    #[test]
    fn visibility_unifies_derived_indexes_and_restores_without_rewriting_history() {
        for compressed in [false, true] {
            let s = fixture();
            let d = derived(&s);
            let original=b"{\"type\":\"session_meta\",\"payload\":{\"id\":\"a\",\"model_provider\":\"old\"}}\n{\"type\":\"response_item\",\"payload\":{\"encrypted_content\":\"keep\"}}\n";
            let bytes = if compressed {
                zstd::stream::encode_all(original.as_slice(), 1).unwrap()
            } else {
                original.to_vec()
            };
            let path = s.home.join(if compressed {
                "sessions/a.jsonl.zst"
            } else {
                "sessions/a.jsonl"
            });
            fs::write(&path, &bytes).unwrap();
            let before = fs::metadata(&path).unwrap().modified().unwrap();
            let p = s
                .preview(vec!["a".into()], "provider".into(), "atelier".into())
                .unwrap();
            assert_eq!(fs::metadata(&path).unwrap().modified().unwrap(), before);
            s.apply(p["ticket"].as_str().unwrap()).unwrap();
            assert_eq!(s.row("a").unwrap().provider, "atelier");
            let get = |host: &str| {
                d.query_row("SELECT model_provider FROM local_thread_catalog WHERE thread_id='a' AND host_id=?1",[host],|r|r.get::<_,String>(0)).unwrap()
            };
            assert_eq!(get("mine"), "atelier");
            assert_eq!(get("other"), "remote-provider");
            assert_eq!(
                d.query_row("SELECT model_provider FROM threads WHERE id='a'", [], |r| r
                    .get::<_, String>(0))
                    .unwrap(),
                "atelier"
            );
            assert_eq!(fs::read(&path).unwrap(), bytes);
            assert_eq!(
                fs::metadata(&path)
                    .unwrap()
                    .modified()
                    .unwrap()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
                1700000000000
            );
            let restore = s.restore(p["ticket"].as_str().unwrap()).unwrap();
            s.apply(restore["ticket"].as_str().unwrap()).unwrap();
            assert_eq!(get("mine"), "old");
            assert_eq!(s.row("a").unwrap().provider, "old");
            assert_eq!(fs::read(&path).unwrap(), bytes);
            assert_eq!(fs::metadata(&path).unwrap().modified().unwrap(), before);
            drop(d);
            fs::remove_dir_all(s.home).unwrap();
        }
    }
    #[test]
    fn visibility_external_catalog_or_rollout_change_aborts_all_databases() {
        for catalog in [false, true] {
            let s = fixture();
            let d = derived(&s);
            let path = s.home.join("sessions/a.jsonl");
            fs::write(&path, b"original bytes").unwrap();
            let p = s
                .preview(vec!["a".into()], "provider".into(), "atelier".into())
                .unwrap();
            if catalog {
                d.execute("UPDATE local_thread_catalog SET model_provider='external' WHERE host_id='mine'",[]).unwrap();
            } else {
                fs::write(&path, b"new append from client").unwrap();
            }
            assert!(s.apply(p["ticket"].as_str().unwrap()).is_err());
            assert_eq!(s.row("a").unwrap().provider, "old");
            assert_eq!(
                d.query_row("SELECT model_provider FROM threads WHERE id='a'", [], |r| r
                    .get::<_, String>(0))
                    .unwrap(),
                "old"
            );
            if !catalog {
                assert_eq!(fs::read(&path).unwrap(), b"new append from client");
            }
            drop(d);
            fs::remove_dir_all(s.home).unwrap();
        }
    }
    #[test]
    fn visibility_repairs_catalog_when_main_provider_already_matches_and_restores_partial_state() {
        let s = fixture();
        let d = derived(&s);
        s.connection(true)
            .unwrap()
            .execute("UPDATE threads SET model_provider='atelier'", [])
            .unwrap();
        let p = s
            .preview(vec!["a".into()], "provider".into(), "atelier".into())
            .unwrap();
        assert!(p["changes"].as_array().unwrap().is_empty());
        assert!(p["visibilityChanges"].as_u64().unwrap() > 0);
        s.apply(p["ticket"].as_str().unwrap()).unwrap();
        // A partially restored secondary index does not prevent recovering the remaining local catalog.
        d.execute("UPDATE threads SET model_provider='old'", [])
            .unwrap();
        let restored = s.restore(p["ticket"].as_str().unwrap()).unwrap();
        s.apply(restored["ticket"].as_str().unwrap()).unwrap();
        assert_eq!(
            d.query_row(
                "SELECT model_provider FROM local_thread_catalog WHERE host_id='mine'",
                [],
                |r| r.get::<_, String>(0)
            )
            .unwrap(),
            "old"
        );
        assert_eq!(s.row("a").unwrap().provider, "atelier");
        drop(d);
        fs::remove_dir_all(s.home).unwrap();
    }
    #[test]
    fn provider_uses_effective_profile_id_instead_of_display_name() {
        let s = fixture();
        fs::write(s.home.join("config.toml"),"model_provider='openai'\nprofile='work'\n[profiles.work]\nmodel_provider='codex_local_access'\n[model_providers.codex_local_access]\nname='OpenAI'\n").unwrap();
        assert_eq!(s.provider().unwrap(), "codex_local_access");
        assert!(s
            .preview(vec!["a".into()], "provider".into(), "OpenAI".into())
            .is_err());
        let p = s
            .preview(
                vec!["a".into()],
                "provider".into(),
                "codex_local_access".into(),
            )
            .unwrap();
        s.apply(p["ticket"].as_str().unwrap()).unwrap();
        assert_eq!(s.row("a").unwrap().provider, "codex_local_access");
        fs::remove_dir_all(s.home).unwrap();
    }
    #[test]
    fn migration_roundtrip_keeps_original_bytes() {
        let s = fixture();
        let original=b"{ \"type\":\"session_meta\",\"payload\":{\"id\":\"a\",\"model_provider\":\"old\"}}\n{\"type\":\"response_item\",\"payload\":{\"encrypted_content\":\"preserve\"}}\n";
        fs::write(s.home.join("sessions/a.jsonl"), original).unwrap();
        let p = s
            .preview(vec!["a".into()], "migration".into(), "atelier".into())
            .unwrap();
        let ticket = p["ticket"].as_str().unwrap();
        s.apply(ticket).unwrap();
        assert_eq!(s.row("a").unwrap().provider, "atelier");
        let bytes = fs::read(s.home.join("sessions/a.jsonl")).unwrap();
        assert!(String::from_utf8_lossy(&bytes).contains("atelier"));
        let restore = s.restore(ticket).unwrap();
        s.apply(restore["ticket"].as_str().unwrap()).unwrap();
        assert_eq!(fs::read(s.home.join("sessions/a.jsonl")).unwrap(), original);
        fs::remove_dir_all(s.home).unwrap();
    }
    #[test]
    fn migration_rejects_file_change_after_preview() {
        let s = fixture();
        fs::write(
            s.home.join("sessions/a.jsonl"),
            b"{\"type\":\"session_meta\",\"payload\":{\"id\":\"a\",\"model_provider\":\"old\"}}\n",
        )
        .unwrap();
        let p = s
            .preview(vec!["a".into()], "migration".into(), "atelier".into())
            .unwrap();
        fs::write(s.home.join("sessions/a.jsonl"), b"changed").unwrap();
        assert!(s.apply(p["ticket"].as_str().unwrap()).is_err());
        assert_eq!(s.row("a").unwrap().provider, "old");
        fs::remove_dir_all(s.home).unwrap();
    }
    #[test]
    fn preview_apply_restore_preserves_other_fields() {
        let s = fixture();
        let p = s
            .preview(vec!["a".into()], "provider".into(), "atelier".into())
            .unwrap();
        assert_eq!(s.row("a").unwrap().provider, "old");
        s.apply(p["ticket"].as_str().unwrap()).unwrap();
        assert_eq!(s.row("a").unwrap().provider, "atelier");
        assert_eq!(s.row("a").unwrap().tokens, Some(100));
        let restore = s.restore(p["ticket"].as_str().unwrap()).unwrap();
        s.apply(restore["ticket"].as_str().unwrap()).unwrap();
        assert_eq!(s.row("a").unwrap().provider, "old");
        fs::remove_dir_all(s.home).unwrap();
    }
    #[test]
    fn rejects_external_change_and_changed_config() {
        let s = fixture();
        let p = s
            .preview(vec!["a".into()], "archive".into(), "true".into())
            .unwrap();
        s.connection(true)
            .unwrap()
            .execute("UPDATE threads SET updated_at=updated_at+1", [])
            .unwrap();
        assert!(s.apply(p["ticket"].as_str().unwrap()).is_err());
        assert!(!s.row("a").unwrap().archived);
        fs::write(s.home.join("config.toml"), "model_provider='other'").unwrap();
        assert!(s.apply(p["ticket"].as_str().unwrap()).is_err());
        fs::remove_dir_all(s.home).unwrap();
    }
    #[test]
    fn compressed_fallback_and_explicit_path_match_plain_records() {
        let s = fixture();
        let bytes=b"{\"type\":\"turn_context\",\"payload\":{\"model\":\"test-model\"}}\n{\"timestamp\":\"2026-11-05T00:00:00Z\",\"type\":\"event_msg\",\"payload\":{\"type\":\"token_count\",\"info\":{\"total_token_usage\":{\"total_tokens\":150}}}}\n";
        let path = s.home.join("sessions/a.jsonl");
        fs::write(&path, bytes).unwrap();
        let plain = records::detail(&s, &s.row("a").unwrap()).unwrap();
        let encoded = zstd::stream::encode_all(bytes.as_slice(), 1).unwrap();
        fs::write(s.home.join("sessions/a.jsonl.zst"), &encoded).unwrap();
        fs::remove_file(&path).unwrap();
        let compressed = records::detail(&s, &s.row("a").unwrap()).unwrap();
        for key in ["usage", "timeline", "modelUsage", "tools", "messages"] {
            assert_eq!(plain[key], compressed[key]);
        }
        assert_eq!(compressed["compressed"], true);
        assert!(!path.exists());
        assert_eq!(
            fs::read(s.home.join("sessions/a.jsonl.zst")).unwrap(),
            encoded
        );
        s.connection(true)
            .unwrap()
            .execute("UPDATE threads SET rollout_path='sessions/a.jsonl.zst'", [])
            .unwrap();
        assert_eq!(
            records::detail(&s, &s.row("a").unwrap()).unwrap()["usage"]["total"],
            150
        );
        fs::remove_dir_all(s.home).unwrap();
    }
    #[test]
    fn corrupt_compressed_record_is_not_reported_as_missing() {
        let s = fixture();
        fs::write(s.home.join("sessions/a.jsonl.zst"), b"invalid zstd").unwrap();
        let e = records::detail(&s, &s.row("a").unwrap()).unwrap_err();
        assert!(!e.contains("未找到"));
        assert!(e.contains("失败") || e.contains("Zstandard"));
        fs::remove_dir_all(s.home).unwrap();
    }
    #[test]
    fn decoded_limit_applies_to_compressed_records() {
        let s = fixture();
        let line = b"{\"type\":\"event_msg\",\"payload\":{}}\n";
        let mut bytes = Vec::new();
        while bytes.len() <= 32 * 1024 * 1024 {
            bytes.extend_from_slice(line)
        }
        let encoded = zstd::stream::encode_all(bytes.as_slice(), 1).unwrap();
        fs::write(s.home.join("sessions/a.jsonl.zst"), encoded).unwrap();
        let d = records::detail(&s, &s.row("a").unwrap()).unwrap();
        assert_eq!(d["complete"], false);
        assert!(d["warnings"].to_string().contains("32 MiB"));
        fs::remove_dir_all(s.home).unwrap();
    }
    #[test]
    fn usage_uses_latest_cumulative_snapshot() {
        let s = fixture();
        fs::write(s.home.join("sessions/a.jsonl"),concat!("{\"type\":\"event_msg\",\"payload\":{\"type\":\"token_count\",\"info\":{\"total_token_usage\":{\"total_tokens\":100}}}}\n","{\"type\":\"event_msg\",\"payload\":{\"type\":\"token_count\",\"info\":{\"total_token_usage\":{\"total_tokens\":150}}}}\n")).unwrap();
        let d = records::detail(&s, &s.row("a").unwrap()).unwrap();
        assert_eq!(d["usage"]["total"], 150);
        fs::remove_dir_all(s.home).unwrap();
    }
    #[test]
    fn rejects_rollout_outside_session_root() {
        let s = fixture();
        s.connection(true)
            .unwrap()
            .execute("UPDATE threads SET rollout_path='config.toml'", [])
            .unwrap();
        assert!(records::detail(&s, &s.row("a").unwrap()).is_err());
        fs::remove_dir_all(s.home).unwrap();
    }
}
