mod migration;
mod mutations;
mod records;
mod usage_history;
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
        let mut files = fs::read_dir(&self.home)
            .map_err(|_| "无法读取 Codex 目录")?
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                let n = e.file_name().to_string_lossy().into_owned();
                let version = n
                    .strip_prefix("state_")?
                    .strip_suffix(".sqlite")?
                    .parse::<u32>()
                    .ok()?;
                Some((version, e.path()))
            })
            .collect::<Vec<_>>();
        files.sort_by_key(|x| x.0);
        let p = files.pop().ok_or("没有找到 state_*.sqlite 会话索引")?.1;
        if fs::symlink_metadata(&p)
            .map_err(err)?
            .file_type()
            .is_symlink()
        {
            return Err("不支持符号链接数据库".into());
        }
        Ok(p)
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
    fn provider(&self) -> Result<String> {
        let p = self.home.join("config.toml");
        if !p.exists() {
            return Ok("openai".into());
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
        let config: toml::Value =
            toml::from_str(std::str::from_utf8(&bytes).map_err(err)?).map_err(err)?;
        Ok(config
            .get("model_provider")
            .and_then(|v| v.as_str())
            .unwrap_or("openai")
            .into())
    }
    fn rows(&self) -> Result<Vec<Thread>> {
        Self::query(&self.connection(false)?)
    }
    fn query(c: &Connection) -> Result<Vec<Thread>> {
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
        ];
        let sql = format!(
            "SELECT {} FROM threads ORDER BY {} DESC LIMIT 10001",
            names
                .iter()
                .map(|(n, d)| col(n, d))
                .collect::<Vec<_>>()
                .join(","),
            col("updated_at", "0")
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
                Ok(Thread {
                    id: r.get(0)?,
                    title: r.get(1)?,
                    cwd: r.get(2)?,
                    provider: r.get(3)?,
                    model: r.get(4)?,
                    updated_at: ms(r.get(5)?),
                    created_at: ms(r.get(6)?),
                    archived: r.get::<_, i64>(7)? != 0,
                    tokens: r.get(8)?,
                    rollout_path: r.get(9)?,
                    source: r.get(10)?,
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
        Ok(
            json!({"source":"native","database":self.db()?.display().to_string(),"provider":provider,"checkedAt":now_ms(),"threads":rows,"warnings":warnings}),
        )
    }
}
#[tauri::command]
pub async fn threads_catalog(app: tauri::AppHandle) -> Result<Value> {
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
    tauri::async_runtime::spawn_blocking(move || Store::native(&app)?.preview(ids, action, value))
        .await
        .map_err(err)?
}
#[tauri::command]
pub async fn threads_apply(app: tauri::AppHandle, ticket: String, confirmed: bool) -> Result<()> {
    if !confirmed {
        return Err("需要明确确认".into());
    }
    tauri::async_runtime::spawn_blocking(move || Store::native(&app)?.apply(&ticket))
        .await
        .map_err(err)?
}
#[tauri::command]
pub async fn threads_backups(app: tauri::AppHandle) -> Result<Value> {
    tauri::async_runtime::spawn_blocking(move || Store::native(&app)?.list_backups())
        .await
        .map_err(err)?
}
#[tauri::command]
pub async fn threads_restore(app: tauri::AppHandle, id: String) -> Result<Value> {
    tauri::async_runtime::spawn_blocking(move || Store::native(&app)?.restore(&id))
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
