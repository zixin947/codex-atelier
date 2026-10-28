use super::{oauth::AuthManager, storage::Store, types::*};
use axum::{
    body::{Body, Bytes},
    extract::{DefaultBodyLimit, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use futures_util::StreamExt;
use serde_json::{json, Value};
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
    time::UNIX_EPOCH,
};
use tokio::sync::{oneshot, OwnedSemaphorePermit, Semaphore};
use uuid::Uuid;

#[derive(Default)]
struct Stats {
    requests: u64,
    input: u64,
    output: u64,
}
#[derive(Default)]
struct Live {
    view: RuntimeView,
    key: Option<String>,
    shutdown: Option<oneshot::Sender<()>>,
    events: VecDeque<RouteEvent>,
    stats: HashMap<String, Stats>,
    responses: HashMap<String, String>,
    sessions: HashMap<String, String>,
}
pub struct GatewayServer {
    store: Arc<Store>,
    auth: Arc<AuthManager>,
    client: reqwest::Client,
    live: Mutex<Live>,
    lifecycle: tokio::sync::Mutex<()>,
    slots: Arc<Semaphore>,
}
impl GatewayServer {
    pub fn new(store: Arc<Store>, auth: Arc<AuthManager>, client: reqwest::Client) -> Self {
        Self {
            store,
            auth,
            client,
            live: Mutex::new(Live::default()),
            lifecycle: tokio::sync::Mutex::new(()),
            slots: Arc::new(Semaphore::new(8)),
        }
    }
    pub fn view(&self) -> RuntimeView {
        self.live
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .view
            .clone()
    }
    pub fn events(&self) -> Vec<RouteEvent> {
        self.live
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .events
            .iter()
            .rev()
            .cloned()
            .collect()
    }
    pub fn active_requests(&self) -> usize {
        self.view().active_requests
    }
    pub fn overlay_profiles(&self, p: &mut [Profile]) {
        let live = self.live.lock().unwrap_or_else(|e| e.into_inner());
        for p in p {
            if let Some(s) = live.stats.get(&p.id) {
                p.requests = s.requests;
                p.input_tokens = s.input;
                p.output_tokens = s.output;
            }
        }
    }
    pub fn local_key(&self) -> Result<String, String> {
        self.live
            .lock()
            .map_err(|_| "网关状态锁异常。")?
            .key
            .clone()
            .ok_or("请先启动本地网关。".into())
    }
    pub async fn start(self: &Arc<Self>) -> Result<(), String> {
        let _lock = self.lifecycle.lock().await;
        if !["stopped", "error"].contains(&self.view().state.as_str()) {
            return Err("网关已运行或正在停止。".into());
        }
        let c = self.store.read()?;
        let preferred = c
            .settings
            .preferred_profile_id
            .as_ref()
            .and_then(|id| c.profiles.iter().find(|p| &p.id == id))
            .ok_or("请先选择当前账号。")?;
        if !eligible(preferred, &c.settings.model) {
            return Err("当前账号未就绪，或未配置所选模型。请检查账号与模型。".into());
        }
        let listener =
            tokio::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, c.settings.port))
                .await
                .map_err(|_| "本地端口不可用，请修改端口后重试。")?;
        let address = format!("http://127.0.0.1:{}/v1", c.settings.port);
        let (tx, rx) = oneshot::channel();
        {
            let mut live = self.live.lock().map_err(|_| "网关状态锁异常。")?;
            live.view.state = "running".into();
            live.view.address = Some(address);
            live.view.message = None;
            live.key = Some(format!(
                "atelier_{}{}",
                Uuid::new_v4().simple(),
                Uuid::new_v4().simple()
            ));
            live.shutdown = Some(tx);
            live.responses.clear();
            live.sessions.clear();
        }
        let app = Router::new()
            .route("/health", get(health))
            .route("/v1/models", get(models))
            .route("/v1/responses", post(responses).get(no_websocket))
            .route("/v1/responses/compact", post(compact))
            .layer(DefaultBodyLimit::max(16 * 1024 * 1024))
            .with_state(self.clone());
        let server = self.clone();
        tokio::spawn(async move {
            let result = axum::serve(listener, app)
                .with_graceful_shutdown(async {
                    let _ = rx.await;
                })
                .await;
            let mut live = server.live.lock().unwrap_or_else(|e| e.into_inner());
            live.view.state = if result.is_ok() { "stopped" } else { "error" }.into();
            live.view.address = None;
            live.key = None;
            live.shutdown = None;
            if result.is_err() {
                live.view.message = Some("网关监听已退出，请重新启动。".into());
            }
        });
        Ok(())
    }
    pub async fn stop(&self) -> Result<(), String> {
        let _lock = self.lifecycle.lock().await;
        let mut live = self.live.lock().map_err(|_| "网关状态锁异常。")?;
        if let Some(shutdown) = live.shutdown.take() {
            live.view.state = "stopping".into();
            live.view.message = Some("已停止接收新请求，等待在途请求结束。".into());
            let _ = shutdown.send(());
        }
        Ok(())
    }
    fn check(&self, h: &HeaderMap) -> Result<(), Response> {
        let live = self
            .live
            .lock()
            .map_err(|_| failure(503, "gateway_unavailable", "网关不可用。"))?;
        if live.view.state != "running" {
            return Err(failure(503, "gateway_stopping", "网关正在停止。"));
        }
        if h.contains_key("origin") {
            return Err(failure(
                403,
                "browser_origin_rejected",
                "网关不接受网页来源请求。",
            ));
        }
        let port = live
            .view
            .address
            .as_ref()
            .and_then(|s| url::Url::parse(s).ok())
            .and_then(|u| u.port())
            .unwrap_or(0);
        let host = h.get("host").and_then(|v| v.to_str().ok()).unwrap_or("");
        if host != format!("127.0.0.1:{port}") && host != format!("localhost:{port}") {
            return Err(failure(403, "invalid_host", "只允许本地网关地址。"));
        }
        let actual = h
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .unwrap_or("");
        if !constant_equal(
            actual.as_bytes(),
            live.key.as_deref().unwrap_or("").as_bytes(),
        ) {
            return Err(failure(
                401,
                "invalid_gateway_key",
                "需要 Atelier 本地网关密钥。",
            ));
        }
        Ok(())
    }
    fn remember(&self, response: &str, profile: &str, session: Option<&str>) {
        let mut live = self.live.lock().unwrap_or_else(|e| e.into_inner());
        if live.responses.len() >= 4096 {
            live.responses.clear();
        }
        if !response.is_empty() {
            let value = match live.responses.get(response) {
                Some(old) if old != profile => "ambiguous".to_owned(),
                _ => profile.into(),
            };
            live.responses.insert(response.into(), value);
        }
        if let Some(s) = session {
            if live.sessions.len() >= 4096 {
                live.sessions.clear();
            }
            let value = match live.sessions.get(s) {
                Some(old) if old != profile => "ambiguous".to_owned(),
                _ => profile.into(),
            };
            live.sessions.insert(s.into(), value);
        }
    }
    fn mark(&self, id: &str, status: u16, code: &str, retry_after: Option<&str>) {
        let _ = self.store.update(None, |c| {
            if let Some(p) = c.profiles.iter_mut().find(|p| p.id == id) {
                p.last_error = Some(format!("上游状态 {status} / {code}"));
                if status == 401 {
                    p.auth_status = "needsLogin".into();
                } else if code == "subscription_sharing_usage_limit_exceeded" {
                    p.enabled = false;
                    p.cooldown_until = None;
                } else if code == "user_not_eligible" {
                    p.enabled = false;
                    p.auth_status = "planDisabled".into();
                } else if status == 429 {
                    p.cooldown_until = Some(retry_time(retry_after));
                }
            }
            Ok(())
        });
    }
}
fn constant_equal(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() || a.is_empty() {
        return false;
    }
    a.iter().zip(b).fold(0u8, |v, (a, b)| v | (a ^ b)) == 0
}
fn eligible(p: &Profile, model: &str) -> bool {
    p.enabled
        && (p.auth_status == "ready"
            || (p.kind == ProfileKind::ApiKey && p.auth_status == "unknown"))
        && p.cooldown_until.is_none_or(|v| v <= now_ms())
        && !model.is_empty()
        && p.models.iter().any(|m| m.id == model)
}
fn failure(status: u16, code: &str, message: &str) -> Response {
    (
        StatusCode::from_u16(status).unwrap_or(StatusCode::BAD_GATEWAY),
        Json(json!({"error":{"type":"atelier_gateway_error","code":code,"message":message}})),
    )
        .into_response()
}
async fn health() -> Json<Value> {
    Json(json!({"status":"ok","service":"codex-atelier"}))
}
async fn no_websocket() -> Response {
    failure(
        426,
        "websocket_not_supported",
        "本版网关使用 HTTP/SSE，请设置 supports_websockets=false。",
    )
}
async fn models(State(server): State<Arc<GatewayServer>>, headers: HeaderMap) -> Response {
    if let Err(e) = server.check(&headers) {
        return e;
    }
    let c = match server.store.read() {
        Ok(c) => c,
        Err(_) => return failure(503, "storage_unavailable", "账号目录不可用。"),
    };
    let p = match c
        .profiles
        .iter()
        .find(|p| Some(&p.id) == c.settings.preferred_profile_id.as_ref())
    {
        Some(p) => p,
        None => return failure(503, "no_selected_account", "请在 Atelier 选择当前账号。"),
    };
    Json(json!({"object":"list","data":p.models.iter().map(|m|json!({"id":m.id,"object":"model","owned_by":"configured-upstream"})).collect::<Vec<_>>()})).into_response()
}
async fn responses(State(s): State<Arc<GatewayServer>>, h: HeaderMap, b: Bytes) -> Response {
    forward(s, h, b, false).await
}
async fn compact(State(s): State<Arc<GatewayServer>>, h: HeaderMap, b: Bytes) -> Response {
    forward(s, h, b, true).await
}
struct RequestGuard {
    server: Arc<GatewayServer>,
    profile: Option<String>,
    attempts: usize,
    finished: bool,
    _permit: OwnedSemaphorePermit,
}
impl RequestGuard {
    fn new(server: Arc<GatewayServer>, permit: OwnedSemaphorePermit) -> Self {
        server
            .live
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .view
            .active_requests += 1;
        Self {
            server,
            profile: None,
            attempts: 0,
            finished: false,
            _permit: permit,
        }
    }
    fn finish(&mut self, success: bool, code: &str, usage: Option<&Value>) {
        if self.finished {
            return;
        }
        self.finished = true;
        let mut l = self.server.live.lock().unwrap_or_else(|e| e.into_inner());
        if success {
            l.view.completed_requests += 1;
        } else {
            l.view.failed_requests += 1;
        }
        if let Some(id) = &self.profile {
            let stats = l.stats.entry(id.clone()).or_default();
            stats.requests += 1;
            if let Some(u) = usage {
                stats.input += u["input_tokens"].as_u64().unwrap_or(0);
                stats.output += u["output_tokens"].as_u64().unwrap_or(0);
            }
        }
        l.events.push_back(RouteEvent {
            at: now_ms(),
            profile_id: self.profile.clone(),
            status: if success { "completed" } else { "failed" }.into(),
            code: code.into(),
            attempts: self.attempts,
        });
        while l.events.len() > 100 {
            l.events.pop_front();
        }
    }
}
impl Drop for RequestGuard {
    fn drop(&mut self) {
        if !self.finished {
            self.finish(false, "stream_interrupted", None);
        }
        let mut l = self.server.live.lock().unwrap_or_else(|e| e.into_inner());
        l.view.active_requests = l.view.active_requests.saturating_sub(1);
    }
}
async fn forward(
    server: Arc<GatewayServer>,
    headers: HeaderMap,
    bytes: Bytes,
    is_compact: bool,
) -> Response {
    if let Err(e) = server.check(&headers) {
        return e;
    }
    let permit = match server.slots.clone().try_acquire_owned() {
        Ok(v) => v,
        Err(_) => return failure(503, "gateway_busy", "网关并发已满，请稍后重试。"),
    };
    let mut guard = RequestGuard::new(server.clone(), permit);
    let mut body: Value = match serde_json::from_slice::<Value>(&bytes) {
        Ok(v) if v.is_object() => v,
        _ => {
            guard.finish(false, "invalid_json", None);
            return failure(400, "invalid_json", "请求必须是 JSON 对象。");
        }
    };
    let c = match server.store.read() {
        Ok(v) => v,
        Err(_) => {
            guard.finish(false, "storage_unavailable", None);
            return failure(503, "storage_unavailable", "账号目录不可用。");
        }
    };
    let preferred = match c
        .profiles
        .iter()
        .find(|p| Some(&p.id) == c.settings.preferred_profile_id.as_ref())
    {
        Some(p) => p,
        None => {
            guard.finish(false, "no_selected_account", None);
            return failure(503, "no_selected_account", "请在 Atelier 选择当前账号。");
        }
    };
    let model = body["model"]
        .as_str()
        .unwrap_or(&c.settings.model)
        .to_owned();
    body["model"] = Value::String(model.clone());
    let session = headers
        .get("session_id")
        .or_else(|| headers.get("x-atelier-session-id"))
        .and_then(|v| v.to_str().ok())
        .filter(|s| !s.is_empty() && s.len() <= 128)
        .map(String::from);
    let mut pinned = None;
    if let Some(previous) = body["previous_response_id"].as_str() {
        pinned = server
            .live
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .responses
            .get(previous)
            .cloned();
        if pinned.is_none() || pinned.as_deref() == Some("ambiguous") {
            guard.finish(false, "unknown_response_owner", None);
            return failure(
                409,
                "unknown_response_owner",
                "无法确定前序响应所属账号，不能跨账号续传。请发送完整上下文。",
            );
        }
    }
    if has_opaque(&body["input"]) && pinned.is_none() {
        pinned = session
            .as_ref()
            .and_then(|s| server.live.lock().ok()?.sessions.get(s).cloned());
        if pinned.is_none() {
            guard.finish(false, "unknown_context_owner", None);
            return failure(
                409,
                "unknown_context_owner",
                "加密或引用上下文的账号归属未知，已停止转发。",
            );
        }
    }
    let mut candidates: Vec<_> = c
        .profiles
        .iter()
        .filter(|p| {
            p.kind == preferred.kind
                && eligible(p, &model)
                && pinned.as_ref().is_none_or(|id| id == &p.id)
                && (c.settings.auto_switch || p.id == preferred.id)
        })
        .cloned()
        .collect();
    candidates.sort_by_key(|p| (p.id != preferred.id, p.priority));
    if candidates.is_empty() {
        guard.finish(false, "no_eligible_account", None);
        return failure(
            503,
            "no_eligible_account",
            "没有匹配模型且可用的账号；请检查授权、冷却、模型与账号池。",
        );
    }
    let max = if pinned.is_some() {
        1
    } else {
        usize::from(c.settings.max_attempts)
    };
    for profile in candidates.into_iter().take(max) {
        guard.profile = Some(profile.id.clone());
        guard.attempts += 1;
        if profile.kind == ProfileKind::Chatgpt {
            if is_compact {
                guard.finish(false, "unsupported_plan_capability", None);
                return failure(
                    400,
                    "unsupported_plan_capability",
                    "ChatGPT 套餐通道不支持此 compact 接口。",
                );
            }
            if let Err(message) = validate_plan(&mut body) {
                guard.finish(false, "unsupported_plan_request", None);
                return failure(400, "unsupported_plan_request", message);
            }
        }
        let token = match server.auth.access_token(&profile.id).await {
            Ok(t) => t,
            Err(_) => {
                guard.finish(false, "credential_unavailable", None);
                return failure(
                    401,
                    "credential_unavailable",
                    "账号凭据暂不可用，请在 Atelier 检查授权。",
                );
            }
        };
        let suffix = if is_compact {
            "responses/compact"
        } else {
            "responses"
        };
        let target = format!("{}/{suffix}", profile.base_url.trim_end_matches('/'));
        let upstream = match server
            .client
            .post(target)
            .bearer_auth(token)
            .header("Accept", "text/event-stream, application/json")
            .json(&body)
            .send()
            .await
        {
            Ok(r) => r,
            Err(_) => {
                guard.finish(false, "upstream_execution_unknown", None);
                return failure(
                    502,
                    "upstream_execution_unknown",
                    "上游连接失败，执行状态未知；未自动重发。",
                );
            }
        };
        if !upstream.status().is_success() {
            let status = upstream.status().as_u16();
            let retry = upstream
                .headers()
                .get("retry-after")
                .and_then(|s| s.to_str().ok())
                .map(String::from);
            let err = super::oauth::json_response(upstream)
                .await
                .unwrap_or(Value::Null);
            let code = error_code(&err);
            server.mark(&profile.id, status, &code, retry.as_deref());
            if c.settings.auto_switch
                && pinned.is_none()
                && [401, 429].contains(&status)
                && guard.attempts < max
            {
                continue;
            }
            guard.finish(false, &code, None);
            return failure(
                status,
                &code,
                "上游拒绝请求，请在 Atelier 检查该账号的授权或限额。",
            );
        }
        server.remember("", &profile.id, session.as_deref());
        let stream = body["stream"].as_bool().unwrap_or(false) && !is_compact;
        if !stream {
            let mut upstream = upstream;
            let mut bytes = Vec::new();
            loop {
                match upstream.chunk().await {
                    Ok(Some(chunk)) if bytes.len() + chunk.len() <= 16 * 1024 * 1024 => {
                        bytes.extend_from_slice(&chunk)
                    }
                    Ok(None) => break,
                    _ => {
                        guard.finish(false, "upstream_response_incomplete", None);
                        return failure(
                            502,
                            "upstream_response_incomplete",
                            "上游响应未完整接收。",
                        );
                    }
                }
            }
            let value: Value = match serde_json::from_slice::<Value>(&bytes) {
                Ok(v) => v,
                Err(_) => {
                    guard.finish(false, "invalid_upstream_response", None);
                    return failure(502, "invalid_upstream_response", "上游返回了不兼容的响应。");
                }
            };
            let ok = if is_compact {
                value.get("output").is_some() && !value.is_null()
            } else {
                value["status"] == "completed"
            };
            if let Some(id) = value["id"].as_str() {
                server.remember(id, &profile.id, session.as_deref());
            }
            guard.finish(
                ok,
                if ok {
                    "response.completed"
                } else {
                    "response.incomplete"
                },
                value.get("usage"),
            );
            return Json(value).into_response();
        }
        if !upstream
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .is_some_and(|s| s.starts_with("text/event-stream"))
        {
            guard.finish(false, "expected_event_stream", None);
            return failure(
                502,
                "expected_event_stream",
                "上游没有返回 Responses 事件流。",
            );
        }
        let owner = profile.id.clone();
        let mut chunks = upstream.bytes_stream();
        let server = server.clone();
        let output = async_stream::stream! {
            let mut pending=Vec::new();let mut event_data=String::new();let mut terminal:Option<(bool,String,Option<Value>)>=None;
            while let Some(next)=chunks.next().await {
                let chunk=match next {Ok(c)=>c,Err(_)=>{break;}};
                pending.extend_from_slice(&chunk);
                if pending.len()>16*1024*1024 {break;}
                while let Some(pos)=pending.iter().position(|v|*v==b'\n'){
                    let line=pending.drain(..=pos).collect::<Vec<_>>();
                    let line=String::from_utf8_lossy(&line);let line=line.trim_end_matches(['\r','\n']);
                    if let Some(data)=line.strip_prefix("data:"){event_data.push_str(data.trim_start());event_data.push('\n');}
                    if event_data.len()>16*1024*1024{terminal=Some((false,"event_too_large".into(),None));break;}
                    if line.is_empty() && !event_data.is_empty(){
                        if let Ok(v)=serde_json::from_str::<Value>(&event_data){
                            if let Some(id)=v["response"]["id"].as_str(){server.remember(id,&owner,session.as_deref());}
                            match v["type"].as_str(){
                                Some("response.completed")=>terminal=Some((true,"response.completed".into(),v["response"].get("usage").cloned())),
                                Some("response.failed")|Some("error")=>{let err=v.get("response").unwrap_or(&v);let code=error_code(err);if code=="subscription_sharing_usage_limit_exceeded"{server.mark(&owner,429,&code,None);}terminal=Some((false,code,None));},
                                Some("response.incomplete")=>terminal=Some((false,"response.incomplete".into(),None)),_=>{}
                            }
                        }
                        event_data.clear();
                    }
                }
                yield Ok::<Bytes,std::io::Error>(chunk);
                if terminal.is_some(){break;}
            }
            if let Some((success,code,usage))=terminal {guard.finish(success,&code,usage.as_ref());}
            else{guard.finish(false,"stream_interrupted",None);let e=json!({"type":"error","error":{"code":"stream_interrupted","message":"上游流未完整结束；执行状态未知，未自动重发。"}});yield Ok(Bytes::from(format!("event: error\ndata: {e}\n\n")));}
        };
        return (
            [
                ("content-type", "text/event-stream"),
                ("cache-control", "no-cache"),
            ],
            Body::from_stream(output),
        )
            .into_response();
    }
    guard.finish(false, "account_pool_exhausted", None);
    failure(
        503,
        "account_pool_exhausted",
        "可用账号已尝试完毕，请稍后重试或选择账号。",
    )
}
fn error_code(v: &Value) -> String {
    let s = v["error"]["code"]
        .as_str()
        .or_else(|| v["code"].as_str())
        .unwrap_or("upstream_rejected");
    if s.len() <= 100
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
    {
        s.into()
    } else {
        "upstream_rejected".into()
    }
}
fn retry_time(v: Option<&str>) -> u64 {
    v.and_then(|s| {
        s.parse::<u64>()
            .ok()
            .map(|seconds| now_ms().saturating_add(seconds.saturating_mul(1000)))
            .or_else(|| {
                httpdate::parse_http_date(s)
                    .ok()
                    .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                    .map(|t| t.as_millis() as u64)
            })
    })
    .unwrap_or_else(|| now_ms() + 60_000)
    .max(now_ms() + 1000)
}
fn has_opaque(v: &Value) -> bool {
    match v {
        Value::Object(o) => {
            o.contains_key("encrypted_content")
                || o.get("type").is_some_and(|v| v == "item_reference")
                || o.values().any(has_opaque)
        }
        Value::Array(a) => a.iter().any(has_opaque),
        _ => false,
    }
}
fn validate_plan(v: &mut Value) -> Result<(), &'static str> {
    if v.get("previous_response_id").is_some_and(|v| !v.is_null()) {
        return Err("订阅 HTTP 通道要求完整 input，不支持 previous_response_id。");
    }
    if !v["input"].is_array() {
        return Err("订阅通道的 input 必须为完整上下文数组。");
    }
    for key in [
        "background",
        "conversation",
        "max_output_tokens",
        "max_tool_calls",
        "metadata",
        "moderation",
        "multi_agent",
        "prompt",
        "prompt_cache_retention",
        "safety_identifier",
        "temperature",
        "top_logprobs",
        "top_p",
        "truncation",
        "user",
    ] {
        if v.get(key).is_some_and(|v| !v.is_null()) {
            return Err("请求包含 ChatGPT 套餐通道不支持的字段，请使用兼容配置。");
        }
    }
    if v["input"]
        .as_array()
        .is_some_and(|a| a.iter().any(|v| v["role"] == "system"))
    {
        return Err("订阅通道请使用 instructions 或 developer 消息。");
    }
    if let Some(tools) = v["tools"].as_array() {
        for tool in tools {
            if tool["type"] == "namespace"
                && !tool["tools"].as_array().is_some_and(|items| {
                    items
                        .iter()
                        .all(|item| matches!(item["type"].as_str(), Some("function" | "custom")))
                })
            {
                return Err("命名空间内只允许 function/custom 工具。");
            }
            if !matches!(
                tool["type"].as_str(),
                Some("namespace" | "web_search" | "web_search_preview")
            ) {
                return Err("订阅通道仅支持命名空间内的 function/custom 工具及允许的 web_search。");
            }
        }
    }
    v["store"] = json!(false);
    v["stream"] = json!(true);
    Ok(())
}
