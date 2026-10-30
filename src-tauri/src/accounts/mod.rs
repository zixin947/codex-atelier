mod background;
mod local;
mod metadata;
mod rpc;
mod storage;
mod switch;
mod types;
use crate::gateway::{storage::read_private, types::now_ms};
use rpc::Rpc;
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};
use storage::Store;
use tauri::{Emitter, Manager, State};
use tokio::sync::Notify;
use types::*;
use uuid::Uuid;

#[derive(Default)]
pub struct AccountsState(Mutex<Option<Arc<Accounts>>>);
struct Attempt {
    view: Mutex<Login>,
    cancelled: AtomicBool,
    stop: Notify,
}
struct Accounts {
    store: Store,
    background: Arc<background::Controller>,
    attempts: Mutex<HashMap<String, Arc<Attempt>>>,
    tickets: Mutex<HashMap<String, switch::Ticket>>,
    login_gate: tokio::sync::Mutex<()>,
}
impl AccountsState {
    fn get(&self, app: &tauri::AppHandle) -> Result<Arc<Accounts>, String> {
        let mut guard = self.0.lock().map_err(|_| "账号管理状态锁异常。")?;
        if let Some(value) = &*guard {
            return Ok(value.clone());
        }
        let store = Store::new(
            app.path()
                .app_data_dir()
                .map_err(|_| "无法定位应用数据目录。")?
                .join("codex-accounts"),
        )?;
        let background = background::Controller::open(store.root.join("auto-refresh.json"));
        let value = Arc::new(Accounts {
            store,
            background,
            attempts: Mutex::new(HashMap::new()),
            tickets: Mutex::new(HashMap::new()),
            login_gate: tokio::sync::Mutex::new(()),
        });
        *guard = Some(value.clone());
        Ok(value)
    }
}
pub fn start_background(app: &tauri::AppHandle) -> Result<(), String> {
    let accounts = app.state::<AccountsState>().get(app)?;
    if accounts.background.started.swap(true, Ordering::SeqCst) {
        return Ok(());
    }
    let controller = accounts.background.clone();
    let handle = app.clone();
    tauri::async_runtime::spawn(controller.run(
        move || {
            let accounts = accounts.clone();
            async move { accounts.refresh_background().await }
        },
        move |status| {
            let _ = handle.emit("accounts-auto-refresh", status);
        },
    ));
    Ok(())
}
#[tauri::command]
pub fn accounts_auto_refresh_status(
    app: tauri::AppHandle,
    state: State<'_, AccountsState>,
) -> Result<background::Status, String> {
    start_background(&app)?;
    state.get(&app)?.background.status()
}
#[tauri::command]
pub fn accounts_auto_refresh_set(
    app: tauri::AppHandle,
    state: State<'_, AccountsState>,
    interval_ms: u64,
) -> Result<background::Status, String> {
    start_background(&app)?;
    let status = state.get(&app)?.background.configure(interval_ms)?;
    let _ = app.emit("accounts-auto-refresh", &status);
    Ok(status)
}
impl Accounts {
    async fn refresh_background(&self) -> background::RunResult {
        let mut result = background::RunResult::default();
        let ids = match self.store.read() {
            Ok(catalog) => catalog
                .accounts
                .into_iter()
                .map(|a| a.id)
                .collect::<Vec<_>>(),
            Err(error) => {
                return background::RunResult {
                    finished_at: now_ms(),
                    error: Some(error),
                    ..result
                }
            }
        };
        for id in ids {
            // Reuse the real quota path on Atelier-owned profiles. No local
            // login detection, subscription scraping or model requests here.
            match self.refresh(&id, false, false).await {
                Ok(()) => result.updated += 1,
                Err(error) => {
                    result.failed += 1;
                    if result.error.is_none() {
                        result.error = Some(error);
                    }
                }
            }
        }
        result.finished_at = now_ms();
        result
    }
    fn snapshot(&self) -> Result<Snapshot, String> {
        let mut c = self.store.read()?;
        // Backfill old managed records in the returned view, without reading the real Codex home.
        for p in &mut c.accounts {
            if let Ok(bytes) = self.store.auth(&p.id) {
                if let Ok(auth) = serde_json::from_slice::<Value>(&bytes) {
                    p.metadata = metadata::merge(&p.metadata, &metadata::from_auth(&auth));
                    if p.plan.is_none() {
                        p.plan = p.metadata.plan_snapshot.clone();
                    }
                }
            }
        }
        let current_account = local::current_view(&c);
        Ok(Snapshot {
            available: true,
            reason: None,
            revision: c.revision,
            accounts: c.accounts,
            last_selected_id: c.last_selected_id,
            rollback_available: self.store.root.join("last-switch.json").is_file(),
            current_account,
            pending_login: self
                .attempts
                .lock()
                .map_err(|_| "登录状态锁异常。")?
                .values()
                .find_map(|p| {
                    p.view
                        .lock()
                        .ok()
                        .filter(|v| v.state == "pending")
                        .map(|v| v.clone())
                }),
        })
    }
    async fn refresh(
        &self,
        id: &str,
        refresh_metadata: bool,
        include_reset_details: bool,
    ) -> Result<(), String> {
        let _lock = self.store.lock(id)?;
        let expected_identity = self
            .store
            .read()?
            .accounts
            .into_iter()
            .find(|a| a.id == id)
            .ok_or("账号不存在。")?
            .identity;
        let home = self.store.prepare(id)?;
        let mut rpc = match Rpc::start(&home).await {
            Ok(rpc) => rpc,
            Err(e) => {
                self.store.update(None, |c| {
                    if let Some(p) = c.accounts.iter_mut().find(|a| a.id == id) {
                        p.status = "error".into();
                        p.error = Some("无法启动额度读取；旧值仅供参考，请检查 CLI。".into());
                    }
                    Ok(())
                })?;
                return Err(e);
            }
        };
        let account_result = rpc
            .call("account/read", json!({"refreshToken":false}))
            .await;
        let account = match account_result {
            Ok(account) if account["account"]["type"] == "chatgpt" => account,
            _ => {
                rpc.close().await;
                self.store.update(None, |c| {
                    if let Some(p) = c.accounts.iter_mut().find(|p| p.id == id) {
                        p.status = "error".into();
                        p.error = Some("账号登录不可用，请重新授权；旧额度已保留。".into());
                    }
                    Ok(())
                })?;
                return Err("账号登录不可用，请重新授权。".into());
            }
        };
        let limits = rpc
            .call(
                "account/rateLimits/read",
                json!({"excludeResetCreditDetails":!include_reset_details}),
            )
            .await
            .and_then(|v| quota(&v));
        let user_agent = rpc.user_agent.clone();
        rpc.close().await;
        let auth = self
            .store
            .auth(id)
            .ok()
            .and_then(|b| serde_json::from_slice::<Value>(&b).ok());
        if auth
            .as_ref()
            .is_some_and(|value| storage::hints(value).0 != expected_identity)
        {
            self.store.update(None, |c| {
                if let Some(p) = c.accounts.iter_mut().find(|p| p.id == id) {
                    p.status = "error".into();
                    p.error =
                        Some("独立档案的凭据身份发生变化，未采用查询结果；请重新授权。".into());
                }
                Ok(())
            })?;
            return Err("凭据身份与账号档案不一致，未采用结果。".into());
        }
        let subscription = if refresh_metadata {
            Some(if let Some(auth) = &auth {
                metadata::subscription(auth, &user_agent).await
            } else {
                Err("本应用的账号凭据暂不可读，未更新订阅信息。".into())
            })
        } else {
            None
        };
        let quota_error = limits.as_ref().err().cloned();
        let metadata_error = subscription
            .as_ref()
            .and_then(|result| result.as_ref().err())
            .cloned();
        self.store.update(None, |c| {
            let p = c
                .accounts
                .iter_mut()
                .find(|p| p.id == id)
                .ok_or("账号已移除。")?;
            if let Some(email) = account["account"]["email"].as_str() {
                p.email = Some(email.chars().take(200).collect());
            }
            if let Some(plan) = account["account"]["planType"].as_str() {
                p.plan = Some(plan.chars().take(60).collect());
            }
            if let Some(auth) = &auth {
                p.metadata = metadata::merge(&p.metadata, &metadata::from_auth(auth));
            }
            match subscription {
                Some(Ok(details)) => {
                    if details.plan_snapshot.is_some() {
                        p.plan = details.plan_snapshot.clone();
                    }
                    p.metadata = metadata::merge(&p.metadata, &details);
                }
                Some(Err(e)) => p.metadata.error = Some(e),
                None => {}
            }
            match limits {
                Ok(mut quota) => {
                    if let Some(next) = &mut quota.reset_credits {
                        if next.credits.is_none() {
                            if let Some(previous) =
                                p.quota.as_ref().and_then(|q| q.reset_credits.as_ref())
                            {
                                if next.available_count == previous.available_count {
                                    next.credits = previous.credits.clone();
                                    next.details_checked_at = previous.details_checked_at;
                                }
                            }
                        }
                    }
                    p.quota = Some(quota);
                    p.status = "ready".into();
                    p.error = None;
                }
                Err(e) => {
                    p.status = "error".into();
                    p.error = Some(e);
                }
            }
            Ok(())
        })?;
        if let Some(error) = quota_error {
            return Err(error);
        }
        if let Some(error) = metadata_error {
            return Err(format!("额度已更新；{error}"));
        }
        Ok(())
    }
    async fn begin(
        self: &Arc<Self>,
        name: String,
        account_id: Option<String>,
    ) -> Result<Login, String> {
        let _gate = self.login_gate.lock().await;
        let name = name.trim().to_string();
        if name.is_empty() || name.chars().count() > 80 {
            return Err("名称需为 1–80 个字符。".into());
        }
        {
            let mut attempts = self.attempts.lock().map_err(|_| "登录锁异常。")?;
            attempts.retain(|_, v| v.view.lock().is_ok_and(|s| s.expires_at > now_ms()));
            if attempts
                .values()
                .any(|v| v.view.lock().is_ok_and(|s| s.state == "pending"))
            {
                return Err("已有登录正在进行，请先完成或取消。".into());
            }
        }
        let original = self.store.read()?;
        if let Some(id) = &account_id {
            if !original.accounts.iter().any(|p| &p.id == id) {
                return Err("重新授权的账号不存在。".into());
            }
        }
        if account_id.is_none() && original.accounts.len() >= 64 {
            return Err("账号数量已达上限。".into());
        }
        let id = Uuid::new_v4().to_string();
        let home = self.store.prepare(&id)?;
        let profile_lock = self.store.lock(&id)?;
        let mut rpc = Rpc::start(&home).await?;
        let response = rpc
            .call("account/login/start", json!({"type":"chatgpt"}))
            .await?;
        let url = response["authUrl"]
            .as_str()
            .ok_or("CLI 未提供授权链接。")?
            .to_owned();
        official_url(&url)?;
        let login_id = response["loginId"]
            .as_str()
            .ok_or("CLI 未提供登录标识。")?
            .to_owned();
        let view = Login {
            id: id.clone(),
            state: "pending".into(),
            url: Some(url),
            message: None,
            expires_at: now_ms() + 600_000,
        };
        let attempt = Arc::new(Attempt {
            view: Mutex::new(view.clone()),
            cancelled: AtomicBool::new(false),
            stop: Notify::new(),
        });
        self.attempts
            .lock()
            .map_err(|_| "登录锁异常。")?
            .insert(id.clone(), attempt.clone());
        let manager = self.clone();
        tokio::spawn(async move {
            let _lock = profile_lock;
            let deadline = tokio::time::Instant::now() + Duration::from_secs(600);
            let result = loop {
                tokio::select! {
                    _=attempt.stop.notified()=>{break Err("登录已取消。".to_string());},
                    _=tokio::time::sleep_until(deadline)=>{break Err("登录已超时，请重试。".to_string());},
                    event=rpc.event()=>{
                        match event{
                            Ok(v) if v["method"]=="account/login/completed"&&v["params"]["loginId"]==login_id=>{
                                if v["params"]["success"]==true{break Ok(());}else{break Err("官方登录未完成。".into());}
                            },
                            Ok(_)=>{},
                            Err(e)=>break Err(e),
                        }
                    }
                }
            };
            rpc.close().await;
            let saved = if result.is_ok() && !attempt.cancelled.load(Ordering::SeqCst) {
                let result = (|| {
                    let raw = read_private(&home.join("auth.json"), 256 * 1024)?;
                    let value: Value =
                        serde_json::from_slice(&raw).map_err(|_| "登录凭据格式不兼容。")?;
                    // Hold view through the commit: cancellation cannot report success then save late.
                    let mut view = attempt.view.lock().map_err(|_| "登录状态锁异常。")?;
                    if attempt.cancelled.load(Ordering::SeqCst) {
                        return Err("登录已取消，未保存。".to_string());
                    }
                    if let Some(target) = &account_id {
                        manager
                            .store
                            .reauthorize(target, &value, &original.revision)?;
                        let _ = std::fs::remove_dir_all(&home);
                    } else {
                        manager.store.add(&id, &name, &value)?;
                    }
                    view.state = "completed".into();
                    view.url = None;
                    view.message = Some("账号已添加，请刷新额度。".into());
                    Ok(())
                })();
                result
            } else {
                result.and(Err("登录已取消。".into()))
            };
            if let Err(message) = saved {
                if let Ok(mut view) = attempt.view.lock() {
                    view.state = if attempt.cancelled.load(Ordering::SeqCst) {
                        "cancelled"
                    } else {
                        "error"
                    }
                    .into();
                    view.url = None;
                    view.message = Some(message);
                }
                // This is an Atelier-created temporary profile, never the user's Codex home.
                let _ = std::fs::remove_dir_all(&home);
            }
        });
        Ok(view)
    }
}
fn quota(v: &Value) -> Result<Quota, String> {
    let bucket = if v.get("rateLimitsByLimitId").is_some_and(|v| !v.is_null()) {
        v["rateLimitsByLimitId"]
            .get("codex")
            .ok_or("接口未返回 Codex 配额桶。")?
    } else {
        v.get("rateLimits").ok_or("接口未返回配额。")?
    };
    let window = |key: &str| -> Option<Window> {
        let w = bucket.get(key)?;
        if w.is_null() {
            return None;
        }
        Some(Window {
            used_percent: w["usedPercent"]
                .as_f64()
                .filter(|n| n.is_finite() && *n >= 0.0 && *n <= 100.0),
            duration_mins: w["windowDurationMins"].as_u64().filter(|n| *n > 0),
            resets_at: w["resetsAt"].as_u64().and_then(|n| n.checked_mul(1000)),
        })
    };
    Ok(Quota {
        primary: window("primary"),
        secondary: window("secondary"),
        checked_at: now_ms(),
        reset_credits: parse_reset_credits(v),
    })
}
fn parse_reset_credits(v: &Value) -> Option<ResetCredits> {
    let summary = v.get("rateLimitResetCredits").filter(|v| v.is_object())?;
    let count = summary["availableCount"].as_u64();
    let text = |v: &Value, max: usize| v.as_str().map(|s| s.chars().take(max).collect::<String>());
    let credits = summary["credits"].as_array().map(|rows| {
        rows.iter()
            .take(256)
            .filter_map(|row| {
                let id = row["id"]
                    .as_str()
                    .filter(|id| !id.is_empty() && id.len() <= 512)?
                    .to_owned();
                Some(ResetCredit {
                    id,
                    reset_type: text(&row["resetType"], 100).unwrap_or("unknown".into()),
                    status: text(&row["status"], 100).unwrap_or("unknown".into()),
                    granted_at: metadata::timestamp(&row["grantedAt"]),
                    expires_at: metadata::timestamp(&row["expiresAt"]),
                    no_expiration: row.get("expiresAt").is_some_and(Value::is_null),
                    title: text(&row["title"], 200),
                    description: text(&row["description"], 1500),
                })
            })
            .collect()
    });
    let details_checked_at = credits.as_ref().map(|_: &Vec<ResetCredit>| now_ms());
    Some(ResetCredits {
        available_count: count,
        credits,
        details_checked_at,
    })
}
fn official_url(s: &str) -> Result<(), String> {
    let u = url::Url::parse(s).map_err(|_| "授权地址无效。")?;
    if u.scheme() != "https"
        || !matches!(u.host_str(), Some("auth.openai.com" | "chatgpt.com"))
        || !u.username().is_empty()
        || u.password().is_some()
    {
        return Err("授权地址不是支持的官方域名。".into());
    }
    Ok(())
}
#[tauri::command]
pub fn accounts_snapshot(
    app: tauri::AppHandle,
    state: State<'_, AccountsState>,
) -> Result<Snapshot, String> {
    state.get(&app)?.snapshot()
}
#[tauri::command]
pub async fn accounts_login_start(
    app: tauri::AppHandle,
    state: State<'_, AccountsState>,
    name: String,
    account_id: Option<String>,
) -> Result<Login, String> {
    state.get(&app)?.begin(name, account_id).await
}
#[tauri::command]
pub fn accounts_login_poll(
    app: tauri::AppHandle,
    state: State<'_, AccountsState>,
    id: String,
) -> Result<Login, String> {
    let a = state.get(&app)?;
    let p = a
        .attempts
        .lock()
        .map_err(|_| "登录状态锁异常。")?
        .get(&id)
        .cloned()
        .ok_or("登录记录不存在。")?;
    let view = p.view.lock().map_err(|_| "登录状态锁异常。")?.clone();
    Ok(view)
}
#[tauri::command]
pub fn accounts_login_cancel(
    app: tauri::AppHandle,
    state: State<'_, AccountsState>,
    id: String,
) -> Result<(), String> {
    let a = state.get(&app)?;
    let p = a
        .attempts
        .lock()
        .map_err(|_| "登录锁异常。")?
        .get(&id)
        .cloned()
        .ok_or("登录记录不存在。")?;
    let mut view = p.view.lock().map_err(|_| "登录状态锁异常。")?;
    if view.state == "pending" {
        p.cancelled.store(true, Ordering::SeqCst);
        view.state = "cancelled".into();
        view.url = None;
        p.stop.notify_one();
    }
    Ok(())
}
#[tauri::command]
pub fn accounts_login_open(
    app: tauri::AppHandle,
    state: State<'_, AccountsState>,
    id: String,
) -> Result<(), String> {
    let view = accounts_login_poll(app, state, id)?;
    if view.state != "pending" {
        return Err("授权已结束。".into());
    }
    let url = view.url.ok_or("授权链接已失效。")?;
    official_url(&url)?;
    #[cfg(target_os = "macos")]
    {
        let status = std::process::Command::new("/usr/bin/open")
            .arg(url)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map_err(|_| "打开浏览器失败。")?;
        if status.success() {
            Ok(())
        } else {
            Err("打开浏览器失败，请复制链接。".into())
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        Err("请复制链接到浏览器。".into())
    }
}
#[tauri::command]
pub fn accounts_import(
    app: tauri::AppHandle,
    state: State<'_, AccountsState>,
    content: String,
) -> Result<BatchResult, String> {
    if content.len() > 4 * 1024 * 1024 {
        return Err("导入 JSON 超过 4 MiB。".into());
    }
    let value: Value = serde_json::from_str(&content).map_err(|_| "请提供有效 JSON。")?;
    let rows: Vec<Value> = if let Some(rows) = value.as_array() {
        rows.clone()
    } else if let Some(rows) = value["accounts"].as_array() {
        rows.clone()
    } else {
        vec![value]
    };
    if rows.is_empty() || rows.len() > 64 {
        return Err("每次导入 1–64 个账号。".into());
    }
    let a = state.get(&app)?;
    let mut results = Vec::new();
    for (i, row) in rows.iter().enumerate() {
        let id = Uuid::new_v4().to_string();
        let name = row["name"]
            .as_str()
            .filter(|s| !s.trim().is_empty())
            .map(str::to_string)
            .unwrap_or_else(|| format!("导入账号 {}", i + 1));
        let result = a.store.add(&id, &name, row);
        results.push(ItemResult {
            id: (i + 1).to_string(),
            ok: result.is_ok(),
            message: match result {
                Ok(_) => "已导入；身份与额度待刷新核实。".into(),
                Err(e) => e,
            },
        });
    }
    Ok(BatchResult {
        snapshot: a.snapshot()?,
        results,
    })
}
#[tauri::command]
pub async fn accounts_refresh(
    app: tauri::AppHandle,
    state: State<'_, AccountsState>,
    ids: Vec<String>,
    refresh_metadata: Option<bool>,
    include_reset_details: Option<bool>,
) -> Result<BatchResult, String> {
    if ids.is_empty() || ids.len() > 64 {
        return Err("请选择 1–64 个账号。".into());
    }
    let a = state.get(&app)?;
    let mut results = Vec::new();
    for id in ids {
        let result = a
            .refresh(
                &id,
                refresh_metadata.unwrap_or(true),
                include_reset_details.unwrap_or(true),
            )
            .await;
        results.push(ItemResult {
            id,
            ok: result.is_ok(),
            message: result.err().unwrap_or("额度已刷新。".into()),
        });
    }
    Ok(BatchResult {
        snapshot: a.snapshot()?,
        results,
    })
}
#[tauri::command]
pub fn accounts_edit(
    app: tauri::AppHandle,
    state: State<'_, AccountsState>,
    input: Edit,
    revision: String,
) -> Result<Snapshot, String> {
    if input.name.trim().is_empty()
        || input.name.chars().count() > 80
        || input.group.chars().count() > 40
        || input.note.chars().count() > 500
    {
        return Err("名称、分组或备注超过长度限制。".into());
    }
    let a = state.get(&app)?;
    a.store.update(Some(&revision), |c| {
        let p = c
            .accounts
            .iter_mut()
            .find(|p| p.id == input.id)
            .ok_or("账号不存在。")?;
        p.name = input.name.trim().into();
        p.group = input.group.trim().into();
        p.note = input.note;
        Ok(())
    })?;
    a.snapshot()
}
#[tauri::command]
pub fn accounts_remove(
    app: tauri::AppHandle,
    state: State<'_, AccountsState>,
    ids: Vec<String>,
    revision: String,
) -> Result<BatchResult, String> {
    if ids.is_empty() || ids.len() > 64 {
        return Err("请选择 1–64 个账号。".into());
    }
    let a = state.get(&app)?;
    let mut current = revision;
    let mut results = Vec::new();
    for id in ids {
        match a.store.remove(&id, &current) {
            Ok(()) => {
                results.push(ItemResult {
                    id,
                    ok: true,
                    message: "本地档案已移除。".into(),
                });
                current = a.store.read()?.revision;
            }
            Err(e) => {
                results.push(ItemResult {
                    id,
                    ok: false,
                    message: e,
                });
                break;
            }
        }
    }
    Ok(BatchResult {
        snapshot: a.snapshot()?,
        results,
    })
}
#[tauri::command]
pub fn accounts_switch_preview(
    app: tauri::AppHandle,
    state: State<'_, AccountsState>,
    id: Option<String>,
) -> Result<SwitchPreview, String> {
    let a = state.get(&app)?;
    let ticket = switch::preview(&a.store, id)?;
    let view = ticket.view.clone();
    let mut tickets = a.tickets.lock().map_err(|_| "预览锁异常。")?;
    tickets.retain(|_, t| t.view.expires_at > now_ms());
    if tickets.len() >= 16 {
        tickets.clear();
    }
    tickets.insert(view.ticket.clone(), ticket);
    Ok(view)
}
#[tauri::command]
pub fn accounts_switch_apply(
    app: tauri::AppHandle,
    state: State<'_, AccountsState>,
    ticket: String,
    confirmed: bool,
) -> Result<Snapshot, String> {
    let a = state.get(&app)?;
    let t = a
        .tickets
        .lock()
        .map_err(|_| "预览锁异常。")?
        .remove(&ticket)
        .ok_or("预览已失效，请重新生成。")?;
    switch::apply(&a.store, &t, confirmed)?;
    let observation = local::observe().ok();
    a.store.update(None, |c| {
        c.current_observation = observation;
        Ok(())
    })?;
    a.snapshot()
}

#[tauri::command]
pub fn accounts_group(
    app: tauri::AppHandle,
    state: State<'_, AccountsState>,
    ids: Vec<String>,
    group: String,
    revision: String,
) -> Result<Snapshot, String> {
    if ids.is_empty() || ids.len() > 64 || group.chars().count() > 40 {
        return Err("请选择账号，分组名称最多 40 字符。".into());
    }
    let a = state.get(&app)?;
    a.store.update(Some(&revision), |c| {
        if ids.iter().any(|id| !c.accounts.iter().any(|p| &p.id == id)) {
            return Err("部分账号已移除，请刷新。".into());
        }
        for p in &mut c.accounts {
            if ids.contains(&p.id) {
                p.group = group.trim().into();
            }
        }
        Ok(())
    })?;
    a.snapshot()
}
#[tauri::command]
pub async fn accounts_export(
    app: tauri::AppHandle,
    state: State<'_, AccountsState>,
    ids: Vec<String>,
    confirmed: bool,
) -> Result<String, String> {
    if !confirmed || ids.is_empty() || ids.len() > 64 {
        return Err("需要明确确认导出所选账号凭据。".into());
    }
    let a = state.get(&app)?;
    let catalog = a.store.read()?;
    let mut rows = Vec::new();
    for id in ids {
        let _lock = a.store.lock(&id)?;
        let p = catalog
            .accounts
            .iter()
            .find(|p| p.id == id)
            .ok_or("账号不存在。")?;
        let auth: Value =
            serde_json::from_slice(&a.store.auth(&id)?).map_err(|_| "凭据格式错误。")?;
        rows.push(json!({"name":p.name,"group":p.group,"note":p.note,"auth":auth}));
    }
    let bytes =
        serde_json::to_vec_pretty(&json!({"format":"atelier-codex-accounts-v1","accounts":rows}))
            .map_err(|_| "备份编码失败。")?;
    #[cfg(target_os = "macos")]
    {
        tauri::async_runtime::spawn_blocking(move || {
            use std::io::Write;
            use std::os::unix::fs::OpenOptionsExt;
            let output=std::process::Command::new("/usr/bin/osascript")
                .args(["-e","POSIX path of (choose file name with prompt \"保存敏感账号备份，请勿分享\" default name \"atelier-codex-accounts.json\")"])
                .stdin(std::process::Stdio::null()).stderr(std::process::Stdio::null()).output().map_err(|_|"无法打开保存对话框。")?;
            if !output.status.success(){return Err("已取消导出。".to_string());}
            let text=String::from_utf8(output.stdout).map_err(|_|"保存路径不是 UTF-8。")?;
            let path=std::path::PathBuf::from(text.trim_end_matches(['\r','\n']));
            if !path.is_absolute(){return Err("保存路径无效。".into());}
            let mut f=std::fs::OpenOptions::new().create_new(true).write(true).mode(0o600).custom_flags(libc::O_NOFOLLOW)
                .open(&path).map_err(|_|"无法创建备份；请使用尚不存在的文件名。")?;
            if f.write_all(&bytes).and_then(|_|f.sync_all()).is_err(){let _=std::fs::remove_file(&path);return Err("备份写入失败。".into());}
            Ok(path.to_string_lossy().into_owned())
        }).await.map_err(|_|"保存任务未完成。")?
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = bytes;
        Err("此版本的系统保存对话框仅实现 macOS。".into())
    }
}

#[tauri::command]
pub async fn accounts_detect_current(
    app: tauri::AppHandle,
    state: State<'_, AccountsState>,
) -> Result<Snapshot, String> {
    let a = state.get(&app)?;
    tauri::async_runtime::spawn_blocking(move || {
        let detected = local::detect(&a.store);
        if detected.is_err() && a.store.read()?.current_observation.is_some() {
            a.store.update(None, |catalog| {
                catalog.current_observation = None;
                Ok(())
            })?;
        }
        let mut snapshot = a.snapshot()?;
        if let Err(message) = detected {
            snapshot.current_account = CurrentAccount {
                account_id: None,
                state: "unavailable".into(),
                path: None,
                checked_at: Some(now_ms()),
                message,
            };
        }
        Ok(snapshot)
    })
    .await
    .map_err(|_| "自动检测任务未完成。".to_string())?
}

#[tauri::command]
pub async fn accounts_reset_credit_preview(
    app: tauri::AppHandle,
    state: State<'_, AccountsState>,
    id: String,
) -> Result<Snapshot, String> {
    let a = state.get(&app)?;
    a.refresh(&id, false, true).await?;
    a.snapshot()
}
