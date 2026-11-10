pub mod original_commands;
mod cockpit;
mod listing;
mod migration;
mod mutations;
mod quick_repair;
mod records;
mod usage_history;
mod visibility;
use crate::gateway::types::now_ms;
use rusqlite::{Connection, OpenFlags};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{fs, path::PathBuf, time::Duration};
use tauri::Manager;
type Result<T> = std::result::Result<T, String>;
fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thread {
    pub id: String,
    pub title: String,
    pub cwd: String,
    pub provider: String,
    pub model: String,
    pub updated_at: i64,
    pub created_at: i64,
    pub archived: bool,
    pub tokens: Option<i64>,
    pub rollout_path: String,
    pub source: String,
    pub kind: String,
    #[serde(default)]
    pub explicit_user: bool,
}
#[derive(Clone)]
struct Store {
    home: PathBuf,
    backups: PathBuf,
}
impl Store {
    fn native(app: &tauri::AppHandle) -> Result<Self> {
        let home = std::env::var_os("CODEX_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|p| PathBuf::from(p).join(".codex")))
            .ok_or("无法定位 Codex 目录")?;
        Ok(Self {
            home,
            backups: app
                .path()
                .app_data_dir()
                .map_err(err)?
                .join("thread-backups"),
        })
    }
    fn db(&self) -> Result<PathBuf> {
        #[cfg(not(test))]
        let environment = std::env::var_os("CODEX_SQLITE_HOME");
        #[cfg(test)]
        let environment: Option<std::ffi::OsString> = None;
        listing::database(
            self,
            environment.as_deref(),
            std::env::current_dir().ok().as_deref(),
        )
    }

    fn connection(&self, write: bool) -> Result<Connection> {
        let c = Connection::open_with_flags(
            self.db()?,
            if write {
                OpenFlags::SQLITE_OPEN_READ_WRITE
            } else {
                OpenFlags::SQLITE_OPEN_READ_ONLY
            },
        )
        .map_err(err)?;
        c.busy_timeout(Duration::from_secs(2)).map_err(err)?;
        Ok(c)
    }
    fn config(&self) -> Result<toml::Value> {
        let p = self.home.join("config.toml");
        if !p.exists() {
            return Ok(toml::Value::Table(Default::default()));
        }
        let file = fs::File::open(&p).map_err(err)?;
        if file.metadata().map_err(err)?.len() > 1024 * 1024 {
            return Err("配置文件过大".into());
        }
        let mut bytes = Vec::new();
        std::io::Read::read_to_end(&mut std::io::Read::take(file, 1024 * 1024 + 1), &mut bytes)
            .map_err(err)?;
        if bytes.len() > 1024 * 1024 {
            return Err("配置文件过大".into());
        }
        toml::from_str(std::str::from_utf8(&bytes).map_err(err)?).map_err(err)
    }
    fn provider(&self) -> Result<String> {
        let config = self.config()?;
        let selected = config
            .get("profile")
            .and_then(|v| v.as_str())
            .and_then(|name| config.get("profiles").and_then(|p| p.get(name)))
            .and_then(|profile| profile.get("model_provider"))
            .or_else(|| config.get("model_provider"));
        Ok(selected
            .map(|v| v.as_str().ok_or("model_provider 必须是字符串"))
            .transpose()?
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or("openai")
            .into())
    }
    fn provider_ids(&self, rows: &[Thread]) -> Result<std::collections::BTreeSet<String>> {
        let config = self.config()?;
        let mut ids = std::collections::BTreeSet::from(["openai".to_owned(), self.provider()?]);
        ids.extend(
            rows.iter()
                .map(|r| r.provider.clone())
                .filter(|p| !p.is_empty()),
        );
        if let Some(providers) = config
            .get("model_providers")
            .and_then(toml::Value::as_table)
        {
            ids.extend(providers.keys().cloned());
        }
        if let Some(profiles) = config.get("profiles").and_then(toml::Value::as_table) {
            ids.extend(
                profiles
                    .values()
                    .filter_map(|p| p.get("model_provider").and_then(toml::Value::as_str))
                    .map(str::to_owned),
            );
        }
        ids.retain(|id| {
            !id.trim().is_empty() && id.len() <= 256 && !id.chars().any(char::is_control)
        });
        Ok(ids)
    }
    fn rows(&self) -> Result<Vec<Thread>> {
        Self::query(&self.connection(false)?)
    }
    fn query(c: &Connection) -> Result<Vec<Thread>> {
        let non_root = listing::non_root_ids(c)?;
        let columns = c
            .prepare("PRAGMA table_info(threads)")
            .map_err(err)?
            .query_map([], |r| r.get::<_, String>(1))
            .map_err(err)?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(err)?;
        if !columns.iter().any(|x| x == "id") {
            return Err("不支持此版本的线程数据库：缺少 threads.id".into());
        }
        let col = |name: &str, default: &str| {
            let alternate = match name {
                "updated_at" => "updated_at_ms",
                "created_at" => "created_at_ms",
                "parent_thread_id" => "parent_session_id",
                _ => name,
            };
            let name =
                if !columns.iter().any(|x| x == name) && columns.iter().any(|x| x == alternate) {
                    alternate
                } else {
                    name
                };
            if columns.iter().any(|x| x == name) {
                format!("COALESCE({name},{default})")
            } else {
                default.into()
            }
        };
        let names = [
            ("id", "''"),
            ("title", "''"),
            ("cwd", "''"),
            ("model_provider", "''"),
            ("model", "''"),
            ("updated_at", "0"),
            ("created_at", "0"),
            ("archived", "0"),
            ("tokens_used", "NULL"),
            ("rollout_path", "''"),
            ("source", "''"),
            ("thread_source", "''"),
            ("parent_thread_id", "''"),
            ("has_user_event", "0"),
            ("first_user_message", "''"),
        ];
        let sql = format!(
            "SELECT {} FROM threads ORDER BY {} DESC LIMIT 10001",
            names
                .iter()
                .map(|(n, d)| col(n, d))
                .collect::<Vec<_>>()
                .join(","),
            col("updated_at", "NULL")
        );
        let mut stmt = c.prepare(&sql).map_err(err)?;
        let rows = stmt
            .query_map([], |r| {
                let ms = |t: i64| {
                    if t > 0 && t < 10_000_000_000 {
                        t * 1000
                    } else {
                        t
                    }
                };
                let source: String = r.get(10)?;
                let explicit_user = r.get::<_, String>(11)?.trim().eq_ignore_ascii_case("user");
                let id: String = r.get(0)?;
                let kind = if explicit_user {
                    "conversation"
                } else if non_root.contains(&id) {
                    "subtask"
                } else {
                    listing::kind(
                        &source,
                        &r.get::<_, String>(11)?,
                        &r.get::<_, String>(12)?,
                        r.get::<_, i64>(13)? != 0,
                        &r.get::<_, String>(14)?,
                        columns
                            .iter()
                            .any(|c| c == "has_user_event" || c == "first_user_message"),
                    )
                }
                .to_owned();
                Ok(Thread {
                    id,
                    title: r.get(1)?,
                    cwd: r.get(2)?,
                    provider: r.get(3)?,
                    model: r.get(4)?,
                    updated_at: ms(r.get(5)?),
                    created_at: ms(r.get(6)?),
                    archived: r.get::<_, i64>(7)? != 0,
                    tokens: r.get(8)?,
                    rollout_path: r.get(9)?,
                    source,
                    kind,
                    explicit_user,
                })
            })
            .map_err(err)?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(err)?;
        Ok(rows)
    }
    fn row(&self, id: &str) -> Result<Thread> {
        self.rows()?
            .into_iter()
            .find(|r| r.id == id)
            .ok_or("线程不存在或超出当前索引范围".into())
    }
    fn catalog(&self) -> Result<Value> {
        let mut rows = self.rows()?;
        let mut warnings = vec![];
        if rows.len() > 10000 {
            rows.truncate(10000);
            warnings.push("当前仅展示最近 10000 条索引，不代表全部历史".to_string())
        }
        let provider = match self.provider() {
            Ok(p) => p,
            Err(e) => {
                warnings.push(format!("配置读取失败：{e}"));
                String::new()
            }
        };
        let conversations = rows.iter().filter(|r| r.kind == "conversation").count();
        let subtasks = rows.iter().filter(|r| r.kind == "subtask").count();
        let unverified = rows.len() - conversations - subtasks;
        let provider_options = match self.provider_ids(&rows) {
            Ok(ids) => ids.into_iter().collect::<Vec<_>>(),
            Err(e) => {
                warnings.push(format!("Provider 候选读取失败：{e}"));
                vec![]
            }
        };
        Ok(
            json!({"source":"native","listingSource":"index","database":self.db()?.display().to_string(),"provider":provider,"providerOptions":provider_options,"checkedAt":now_ms(),"threads":rows,"warnings":warnings,"counts":{"indexed":rows.len(),"conversations":conversations,"subtasks":subtasks,"unverified":unverified}}),
        )
    }
}
#[tauri::command]
pub async fn threads_catalog(app: tauri::AppHandle) -> Result<Value> {
    // A provider-filtered app-server response cannot establish local index membership.
    tauri::async_runtime::spawn_blocking(move || Store::native(&app)?.catalog())
        .await
        .map_err(err)?
}
#[tauri::command]
pub async fn threads_detail(app: tauri::AppHandle, id: String) -> Result<Value> {
    tauri::async_runtime::spawn_blocking(move || {
        let s = Store::native(&app)?;
        records::detail(&s, &s.row(&id)?)
    })
    .await
    .map_err(err)?
}
#[tauri::command]
pub async fn threads_preview(
    app: tauri::AppHandle,
    ids: Vec<String>,
    action: String,
    value: String,
) -> Result<Value> {
    tauri::async_runtime::spawn_blocking(move || {
        let s = Store::native(&app)?;
        if matches!(action.as_str(), "provider" | "migration") {
            cockpit::preview(&s, ids, action == "migration", value)
        } else {
            s.preview(ids, action, value)
        }
    })
    .await
    .map_err(err)?
}
#[tauri::command]
pub async fn threads_apply(
    app: tauri::AppHandle,
    ticket: String,
    confirmed: bool,
) -> Result<Value> {
    if !confirmed {
        return Err("需要明确确认".into());
    }
    tauri::async_runtime::spawn_blocking(move || {
        let s = Store::native(&app)?;
        if cockpit::pending(&s, &ticket)? {
            cockpit::apply(&s, &ticket)
        } else {
            s.apply(&ticket)?;
            Ok(json!({"message":"索引操作已执行"}))
        }
    })
    .await
    .map_err(err)?
}
#[tauri::command]
pub async fn threads_backups(app: tauri::AppHandle) -> Result<Value> {
    tauri::async_runtime::spawn_blocking(move || {
        let s = Store::native(&app)?;
        let mut list = s.list_backups()?.as_array().ok_or("备份格式错误")?.clone();
        list.extend(cockpit::backups(&s)?);
        list.sort_by_key(|r| std::cmp::Reverse(r["createdAt"].as_u64().unwrap_or(0)));
        Ok(json!(list))
    })
    .await
    .map_err(err)?
}
#[tauri::command]
pub async fn threads_restore(app: tauri::AppHandle, id: String) -> Result<Value> {
    tauri::async_runtime::spawn_blocking(move || {
        let s = Store::native(&app)?;
        if cockpit::backed(&s, &id)? {
            cockpit::restore(&s, &id)
        } else {
            s.restore(&id)
        }
    })
    .await
    .map_err(err)?
}
#[tauri::command]
pub async fn threads_export(app: tauri::AppHandle, id: String, format: String) -> Result<String> {
    tauri::async_runtime::spawn_blocking(move || {
        let s = Store::native(&app)?;
        let row = s.row(&id)?;
        let d = records::detail(&s, &row)?;
        let text = match format.as_str() {
            "json" => {
                serde_json::to_string_pretty(&json!({"thread":row,"detail":d})).map_err(err)?
            }
            "md" => {
                let mut text = format!("# {}\n\n", row.title.replace(['\n', '\r'], " "));
                if let Some(warnings) = d["warnings"].as_array() {
                    for w in warnings {
                        text.push_str(&format!("> {}\n\n", w.as_str().unwrap_or("")))
                    }
                }
                if let Some(messages) = d["messages"].as_array() {
                    for m in messages {
                        text.push_str(&format!(
                            "## {}\n\n{}\n\n",
                            m["role"].as_str().unwrap_or(""),
                            m["text"].as_str().unwrap_or("")
                        ))
                    }
                }
                text
            }
            _ => return Err("不支持的导出格式".into()),
        };
        let root = s
            .backups
            .parent()
            .ok_or("导出目录不可用")?
            .join("thread-exports");
        crate::gateway::storage::private_dir(&root)?;
        let p = root.join(format!("thread-{}.{}", uuid::Uuid::new_v4(), format));
        crate::gateway::storage::atomic_private(&p, text.as_bytes())?;
        Ok(p.display().to_string())
    })
    .await
    .map_err(err)?
}
