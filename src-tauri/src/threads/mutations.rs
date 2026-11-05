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
    rollouts: Vec<super::migration::RolloutChange>,
}
impl Store {
    fn ticket_path(&self, id: &str, suffix: &str) -> Result<PathBuf> {
        uuid::Uuid::parse_str(id).map_err(|_| "无效的操作标识")?;
        Ok(self.backups.join(format!("{id}.{suffix}.json")))
    }
    fn save_plan(&self, p: &Plan) -> Result<Value> {
        private_dir(&self.backups)?;
        atomic_private(
            &self.ticket_path(&p.ticket, "pending")?,
            &serde_json::to_vec(p).map_err(err)?,
        )?;
        serde_json::to_value(p).map_err(err)
    }
    fn read_plan(&self, id: &str, suffix: &str) -> Result<Plan> {
        serde_json::from_slice(&read_private(
            &self.ticket_path(id, suffix)?,
            2 * 1024 * 1024,
        )?)
        .map_err(err)
    }
    pub(super) fn preview(&self, ids: Vec<String>, action: String, value: String) -> Result<Value> {
        if ids.is_empty() || ids.len() > 100 {
            return Err("每次请选择 1–100 条线程".into());
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
        if matches!(action.as_str(), "provider" | "migration")
            && (value != provider || value.is_empty())
        {
            return Err("目标必须与当前配置的 model_provider 一致".into());
        }
        let c = self.connection(false)?;
        c.prepare(&format!("SELECT {field},updated_at FROM threads LIMIT 0"))
            .map_err(|_| "当前数据库不支持此项修改")?;
        let rows = self.rows()?;
        if ids.iter().any(|id| !rows.iter().any(|r| &r.id == id)) {
            return Err("部分线程已不在索引中，请刷新".into());
        }
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
            warnings.push("索引对齐不迁移 rollout 的 Provider，也不转换 encrypted_content；可能仍无法续聊。需要同时修改会话文件时请选择完整迁移。".into())
        }
        if action == "migration" {
            warnings=vec!["完整迁移：对齐所选索引与会话文件 session_meta 中的 Provider。必须先退出 Codex / ChatGPT 与 codex CLI。".into(),"只重写元数据；其他记录按原字节保存。加密内容不转换，跨账号续聊仍可能被上游拒绝。".into(),format!("将备份并迁移 {} 个会话文件；不重建其他客户端版本的派生目录。",rollouts.len())];
        }
        self.save_plan(&Plan {
            ticket: uuid::Uuid::new_v4().to_string(),
            action,
            field: field.into(),
            changes,
            warnings,
            created_at: now_ms(),
            database: self.db()?.display().to_string(),
            provider,
            rollouts,
        })
    }
    pub(super) fn apply(&self, id: &str) -> Result<()> {
        private_dir(&self.backups)?;
        let lock = open_private(&self.backups.join("mutation.lock"))?;
        lock.try_lock_exclusive()
            .map_err(|_| "另一线程管理操作正在执行")?;
        let p = self.read_plan(id, "pending")?;
        if now_ms().saturating_sub(p.created_at) > 10 * 60 * 1000 {
            return Err("预览已过期，请重新检查".into());
        }
        if p.database != self.db()?.display().to_string() || p.provider != self.provider()? {
            return Err("数据库或配置已发生变化，请重新预览".into());
        }
        if !matches!(p.field.as_str(), "model_provider" | "archived") {
            return Err("无效的修改字段".into());
        }
        if !p.rollouts.is_empty() {
            super::migration::require_stopped()?;
            for r in &p.rollouts {
                super::migration::verify(self, r)?;
            }
        }
        let mut c = self.connection(true)?;
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
        if let Err(error) = tx.commit() {
            let mut recovery = Vec::new();
            for previous in written.iter().rev() {
                if let Err(e) = super::migration::write(self, previous, true) {
                    recovery.push(e)
                }
            }
            return Err(format!(
                "数据库提交失败：{error}；文件回退错误：{recovery:?}；备份 {}",
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
            if let Some(id) = n.strip_suffix(".backup.json") {
                let p = self.read_plan(id, "backup")?;
                list.push(json!({"id":p.ticket,"createdAt":p.created_at,"action":p.action,"count":p.changes.len()}));
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
            rollouts,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
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
