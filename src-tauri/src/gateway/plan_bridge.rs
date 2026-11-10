//! Refreshes Atelier's existing SIWC credentials for the native routing core.
use super::{oauth::AuthManager, storage::Store, types::ProfileKind};
use axum::{
    body::Body,
    extract::{DefaultBodyLimit, Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};
use std::sync::Arc;

#[derive(Clone)]
struct Bridge {
    accounts: Option<Arc<crate::accounts::Accounts>>,
    auth: Arc<AuthManager>,
    store: Arc<Store>,
    client: reqwest::Client,
    key: String,
}
pub struct PlanBridge {
    pub address: String,
    pub key: String,
    task: tokio::task::JoinHandle<()>,
}
impl Drop for PlanBridge {
    fn drop(&mut self) {
        self.task.abort();
    }
}
pub async fn start(
    auth: Arc<AuthManager>,
    store: Arc<Store>,
    client: reqwest::Client,
    accounts: Option<Arc<crate::accounts::Accounts>>,
) -> Result<PlanBridge, String> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .map_err(|_| "无法创建订阅凭据桥接。")?;
    let address = format!(
        "http://{}",
        listener
            .local_addr()
            .map_err(|_| "无法读取订阅桥接地址。")?
    );
    let key = format!(
        "{}{}",
        uuid::Uuid::new_v4().simple(),
        uuid::Uuid::new_v4().simple()
    );
    let handler = Router::new()
        .route("/{id}/responses", post(forward))
        .route("/prepare", post(prepare))
        .route("/codex/{id}/credentials", get(credentials))
        .layer(DefaultBodyLimit::max(16 * 1024 * 1024))
        .with_state(Bridge {
            accounts,
            auth,
            store,
            client,
            key: key.clone(),
        });
    let task = tokio::spawn(async move {
        let _ = axum::serve(listener, handler).await;
    });
    Ok(PlanBridge { address, key, task })
}
async fn forward(
    State(s): State<Bridge>,
    Path(id): Path<String>,
    headers: HeaderMap,
    Json(mut body): Json<Value>,
) -> Response {
    if headers.get("authorization").and_then(|v| v.to_str().ok())
        != Some(format!("Bearer {}", s.key).as_str())
    {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let c = match s.store.read() {
        Ok(c) => c,
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
    };
    let Some(p) = c
        .profiles
        .iter()
        .find(|p| p.id == id && p.kind == ProfileKind::Chatgpt && p.enabled)
    else {
        return StatusCode::FORBIDDEN.into_response();
    };
    if let Err(message) = super::server::validate_plan(&mut body) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error":{"code":"unsupported_plan_request","message":message}})),
        )
            .into_response();
    }
    let token = match s.auth.access_token(&id).await {
        Ok(t) => t,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };
    let upstream = match s
        .client
        .post(format!("{}/responses", p.base_url.trim_end_matches('/')))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await
    {
        Ok(r) => r,
        Err(_) => return StatusCode::BAD_GATEWAY.into_response(),
    };
    let mut response = Response::builder().status(upstream.status());
    for name in ["content-type", "retry-after", "cache-control"] {
        if let Some(v) = upstream.headers().get(name) {
            response = response.header(name, v);
        }
    }
    response
        .body(Body::from_stream(upstream.bytes_stream()))
        .unwrap()
}

async fn prepare(
    State(s): State<Bridge>,
    headers: HeaderMap,
    Json(mut body): Json<Value>,
) -> Response {
    if headers.get("authorization").and_then(|v| v.to_str().ok())
        != Some(format!("Bearer {}", s.key).as_str())
    {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let enabled = match s.store.image_compat_enabled() {
        Ok(v) => v,
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
    };
    crate::maintenance::utilities::filter_image_declarations(&mut body, enabled);
    Json(body).into_response()
}

async fn credentials(
    State(s): State<Bridge>,
    Path(id): Path<String>,
    Query(q): Query<std::collections::HashMap<String, String>>,
    headers: HeaderMap,
) -> Response {
    if headers.get("authorization").and_then(|v| v.to_str().ok())
        != Some(format!("Bearer {}", s.key).as_str())
    {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let Some(accounts) = s.accounts else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let allowed = s.store.read().is_ok_and(|c| {
        c.profiles
            .iter()
            .any(|p| p.id == id && p.kind == ProfileKind::Codex && p.enabled)
    });
    if !allowed {
        return StatusCode::FORBIDDEN.into_response();
    }
    match accounts
        .gateway_credentials(&id, q.get("force").is_some_and(|v| v == "true"))
        .await
    {
        Ok(v) => Json(v).into_response(),
        Err(_) => StatusCode::UNAUTHORIZED.into_response(),
    }
}
