//! Repair visibility markers in the selected index, without changing rollout bytes.
use super::*;
use rusqlite::types::Value as SqlValue;
use std::collections::BTreeSet;

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct Change {
    pub id: String,
    pub field: String,
    pub before: Value,
    pub after: Value,
    first_message: String,
    #[serde(default)]
    restoring: bool,
}
fn sql_value(value: &Value) -> Result<SqlValue> {
    match value {
        Value::Null => Ok(SqlValue::Null),
        Value::String(s) => Ok(SqlValue::Text(s.clone())),
        Value::Number(n) if n.as_i64().is_some() => Ok(SqlValue::Integer(n.as_i64().unwrap())),
        _ => Err("不支持的索引字段值".into()),
    }
}
pub(super) fn prepare(c: &Connection, rows: &[Thread]) -> Result<Vec<Change>> {
    let cols = c
        .prepare("PRAGMA table_info(threads)")
        .map_err(err)?
        .query_map([], |r| r.get::<_, String>(1))
        .map_err(err)?
        .collect::<std::result::Result<BTreeSet<_>, _>>()
        .map_err(err)?;
    if !cols.contains("first_user_message") {
        return Ok(vec![]);
    }
    let mut changes = vec![];
    for row in rows.iter().filter(|r| r.kind == "conversation") {
        let first: String = c
            .query_row(
                "SELECT COALESCE(first_user_message,'') FROM threads WHERE id=?1",
                [&row.id],
                |r| r.get(0),
            )
            .map_err(err)?;
        if first.is_empty() {
            continue;
        }
        for (field, after) in [
            ("has_user_event", json!(1)),
            ("thread_source", json!("user")),
            ("preview", json!(first)),
        ] {
            if !cols.contains(field) {
                continue;
            }
            let before: Value = c
                .query_row(
                    &format!("SELECT {field} FROM threads WHERE id=?1"),
                    [&row.id],
                    |r| {
                        Ok(match r.get::<_, SqlValue>(0)? {
                            SqlValue::Null => Value::Null,
                            SqlValue::Integer(n) => json!(n),
                            SqlValue::Text(s) => json!(s),
                            _ => return Err(rusqlite::Error::InvalidQuery),
                        })
                    },
                )
                .map_err(err)?;
            let missing = before.is_null() || before.as_str() == Some("");
            if before != after && (field == "has_user_event" || missing) {
                changes.push(Change {
                    id: row.id.clone(),
                    field: field.into(),
                    before,
                    after,
                    first_message: first.clone(),
                    restoring: false,
                });
            }
        }
    }
    Ok(changes)
}
impl Change {
    pub fn inverse(&self) -> Self {
        Self {
            before: self.after.clone(),
            after: self.before.clone(),
            restoring: true,
            ..self.clone()
        }
    }
    pub fn apply(&self, c: &Connection) -> Result<()> {
        if !matches!(
            self.field.as_str(),
            "has_user_event" | "thread_source" | "preview"
        ) {
            return Err("不支持的可见性字段".into());
        }
        let before = sql_value(&self.before)?;
        let after = sql_value(&self.after)?;
        let sql = format!("UPDATE threads SET {}=?1 WHERE id=?2 AND {} IS ?3 AND COALESCE(first_user_message,'')=?4", self.field, self.field);
        if c.execute(
            &sql,
            rusqlite::params![after, self.id, before, self.first_message],
        )
        .map_err(err)?
            == 1
        {
            return Ok(());
        }
        if self.restoring {
            let same: bool = c.query_row(&format!("SELECT {} IS ?1 AND COALESCE(first_user_message,'')=?3 FROM threads WHERE id=?2", self.field), rusqlite::params![after, self.id, self.first_message], |r| r.get(0)).map_err(err)?;
            if same {
                return Ok(());
            }
        }
        Err("会话可见性标记或首条用户消息在预览后变化，事务已取消".into())
    }
}
