//! Observed ChatGPT official catalog requests, not a Gateway model inventory.
//! Native evidence: official_models_handler_poll @1006ff084 and native_catalog helpers.
use super::{DiagnosticItem, Result};
use crate::gateway::types::{Catalog, ProfileKind};
use atelier_maintenance_core::catalog::state::RelayStateDocument;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, ETAG};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{collections::HashSet, sync::Mutex, time::Duration};

const OFFICIAL_ENDPOINT: &str = "https://chatgpt.com/backend-api/codex/models";
const FETCH_TIMEOUT: Duration = Duration::from_secs(4);
const MAX_BYTES: usize = 8 * 1024 * 1024;
const OFFICIAL_HEADERS: [&str; 8] = [
    "authorization",
    "chatgpt-account-id",
    "user-agent",
    "originator",
    "version",
    "openai-organization",
    "openai-project",
    "x-openai-internal-codex-residency",
];
pub(crate) fn client_version(query: &str) -> std::result::Result<String, ObserveError> {
    let values = url::form_urlencoded::parse(query.as_bytes())
        .filter(|(key, _)| key == "client_version")
        .map(|(_, value)| value.into_owned())
        .collect::<Vec<_>>();
    if values.len() != 1 || values[0].len() > 80 || semver::Version::parse(&values[0]).is_err() {
        return Err(ObserveError::InvalidClientVersion);
    }
    Ok(values.into_iter().next().expect("one validated version"))
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceMode {
    NativeOff,
    NativeNoAccount,
    Native,
    GatewayChatgpt,
    GatewayApiKey,
}
/// Constructed from actual typed state; no caller-provided availability flag.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Scope {
    mode: SourceMode,
    revision: String,
}
impl Scope {
    pub(crate) fn off() -> Self {
        Self {
            mode: SourceMode::NativeOff,
            revision: "no-active-routing-source".into(),
        }
    }
    pub(crate) fn native(state: &RelayStateDocument) -> Result<Self> {
        let inputs = state.catalog_inputs().map_err(super::err)?;
        Ok(Self {
            mode: if !inputs.router_enabled {
                SourceMode::NativeOff
            } else if inputs.no_account_mode {
                SourceMode::NativeNoAccount
            } else {
                SourceMode::Native
            },
            revision: atelier_maintenance_core::manager::native_catalog_revision(state)
                .map_err(super::err)?,
        })
    }
    pub(crate) fn gateway(catalog: &Catalog, profile_id: &str) -> Result<Self> {
        let p = catalog
            .profiles
            .iter()
            .find(|p| p.id == profile_id)
            .ok_or("官方目录来源 profile 不存在")?;
        // The native providers/active revision has no direct Gateway representation.
        // Hash the actual typed source and selected identity; never interpret it as RelayState.
        let profiles: Vec<_> = catalog
            .profiles
            .iter()
            .map(|p| (&p.id, p.kind, p.enabled, p.priority, &p.base_url, &p.models))
            .collect();
        let bytes =
            serde_json::to_vec(&(&catalog.host_id, &catalog.settings, profiles, profile_id))
                .map_err(super::err)?;
        Ok(Self {
            mode: if p.kind == ProfileKind::Chatgpt {
                SourceMode::GatewayChatgpt
            } else {
                SourceMode::GatewayApiKey
            },
            revision: format!("{:x}", Sha256::digest(bytes)),
        })
    }
    pub(crate) fn gateway_at_target(
        catalog: &Catalog,
        profile_id: &str,
        target: atelier_maintenance_core::transition_journal::TransitionTarget,
    ) -> Result<Self> {
        let mut scope = Self::gateway(catalog, profile_id)?;
        if target == atelier_maintenance_core::transition_journal::TransitionTarget::NativeOff {
            scope.mode = SourceMode::NativeOff;
        }
        Ok(scope)
    }
    pub(crate) fn mode(&self) -> SourceMode {
        self.mode
    }
    fn applicable(&self) -> bool {
        matches!(self.mode, SourceMode::Native | SourceMode::GatewayChatgpt)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub(crate) enum CatalogFailure {
    MissingAuth,
    Http { status: u16 },
    Network,
    Timeout,
    InvalidCatalog,
    TooLarge,
}
impl CatalogFailure {
    /// Original CatalogReply mapping; HTTP failures outside 400..599 become 502.
    pub(crate) fn response_status(&self) -> u16 {
        match self {
            Self::MissingAuth => 401,
            Self::Http { status } if (400..600).contains(status) => *status,
            Self::Timeout => 504,
            _ => 502,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OfficialCatalogStatus {
    available: bool,
    checked_at: i64,
    model_count: u64,
    dropped_entries: u64,
    failure: Option<CatalogFailure>,
}
impl OfficialCatalogStatus {
    pub(crate) fn available(&self) -> bool {
        self.available
    }
    pub(crate) fn failure(&self) -> Option<&CatalogFailure> {
        self.failure.as_ref()
    }
    pub(crate) fn model_count(&self) -> u64 {
        self.model_count
    }
}
#[derive(Default)]
struct Inner {
    scope: Option<Scope>,
    generation: u64,
    sequence: u64,
    status: Option<OfficialCatalogStatus>,
    // Atelier's in-memory readback, never a substitute response after a failed refresh.
    catalog: Option<Value>,
}
#[derive(Default)]
pub(crate) struct OfficialCatalogRuntime {
    inner: Mutex<Inner>,
    #[cfg(test)]
    test_endpoint: Option<String>,
}
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ObserveError {
    InvalidClientVersion,
    NotApplicable,
    ConfigurationChanged,
    Poisoned,
}
impl ObserveError {
    pub(crate) fn response_status(&self) -> u16 {
        match self {
            Self::InvalidClientVersion => 400,
            Self::NotApplicable | Self::ConfigurationChanged => 409,
            Self::Poisoned => 500,
        }
    }
}
/// No credential/header bytes are retained or serialized in this result.
#[derive(Clone, Debug)]
pub(crate) struct Observation {
    pub(crate) status: OfficialCatalogStatus,
    pub(crate) catalog: Option<Value>,
    pub(crate) upstream_etag: Option<HeaderValue>,
    /// An older request can finish successfully without replacing a newer observation.
    pub(crate) published: bool,
}
impl Observation {
    pub(crate) fn response_status(&self) -> u16 {
        self.status
            .failure
            .as_ref()
            .map_or(200, CatalogFailure::response_status)
    }
    pub(crate) fn failure_body(&self) -> Option<Value> {
        self.status.failure.as_ref().map(|_| serde_json::json!({"error": {
            "code": "official_catalog_unavailable",
            "message": "Official model catalog refresh failed. No replacement catalog was returned."
        }}))
    }
}
impl OfficialCatalogRuntime {
    #[cfg(test)]
    pub(crate) fn with_test_endpoint(endpoint: String) -> Self {
        let parsed = url::Url::parse(&endpoint).expect("fixture endpoint URL");
        assert!(parsed.scheme() == "http" && parsed.host_str() == Some("127.0.0.1"));
        Self {
            inner: Mutex::default(),
            test_endpoint: Some(endpoint),
        }
    }
    pub(crate) fn status(&self, scope: &Scope) -> Result<Option<OfficialCatalogStatus>> {
        let inner = self.inner.lock().map_err(|_| "官方目录运行态锁已损坏")?;
        Ok(if inner.scope.as_ref() == Some(scope) {
            inner.status.clone()
        } else {
            None
        })
    }
    pub(crate) fn cached_catalog(&self, scope: &Scope) -> Result<Option<Value>> {
        let inner = self.inner.lock().map_err(|_| "官方目录运行态锁已损坏")?;
        Ok(
            if inner.scope.as_ref() == Some(scope)
                && inner.status.as_ref().is_some_and(|s| s.available)
            {
                inner.catalog.clone()
            } else {
                None
            },
        )
    }
    /// Called on actual configuration publication to fence in-flight observations.
    pub(crate) fn configure(&self, scope: &Scope) -> Result<()> {
        let mut inner = self.inner.lock().map_err(|_| "官方目录运行态锁已损坏")?;
        if inner.scope.as_ref() != Some(scope) {
            inner.generation = inner.generation.wrapping_add(1);
            inner.scope = Some(scope.clone());
            inner.status = None;
            inner.catalog = None;
        }
        Ok(())
    }
    /// Caller supplies the actual observed client_version, never a fabricated fallback.
    pub(crate) async fn observe_request(
        &self,
        scope: &Scope,
        headers: &HeaderMap,
        client_version: &str,
    ) -> std::result::Result<Observation, ObserveError> {
        #[cfg(test)]
        let endpoint = self.test_endpoint.as_deref().unwrap_or(OFFICIAL_ENDPOINT);
        #[cfg(not(test))]
        let endpoint = OFFICIAL_ENDPOINT;
        self.observe_at(scope, headers, client_version, endpoint, FETCH_TIMEOUT)
            .await
    }
    /// Native HTTP entry: exactly one decoded client_version query parameter.
    pub(crate) async fn observe_query(
        &self,
        scope: &Scope,
        headers: &HeaderMap,
        query: &str,
    ) -> std::result::Result<Observation, ObserveError> {
        let version = client_version(query)?;
        self.observe_request(scope, headers, &version).await
    }
    async fn observe_at(
        &self,
        scope: &Scope,
        headers: &HeaderMap,
        client_version: &str,
        endpoint: &str,
        timeout: Duration,
    ) -> std::result::Result<Observation, ObserveError> {
        if client_version.len() > 80 || semver::Version::parse(client_version).is_err() {
            return Err(ObserveError::InvalidClientVersion);
        }
        if !scope.applicable() {
            return Err(ObserveError::NotApplicable);
        }
        let (generation, sequence) = {
            let mut inner = self.inner.lock().map_err(|_| ObserveError::Poisoned)?;
            if inner.scope.is_none() {
                inner.scope = Some(scope.clone());
            }
            if inner.scope.as_ref() != Some(scope) {
                return Err(ObserveError::ConfigurationChanged);
            }
            inner.sequence = inner.sequence.wrapping_add(1);
            (inner.generation, inner.sequence)
        };
        let result =
            tokio::time::timeout(timeout, fetch(endpoint, headers, client_version, timeout))
                .await
                .unwrap_or(Err(CatalogFailure::Timeout));
        let (catalog, dropped, etag, failure) = match result {
            Ok((v, etag)) => match validate_official_models(v) {
                Ok((v, dropped)) if v["models"].as_array().is_some_and(|a| !a.is_empty()) => {
                    (Some(v), dropped, etag, None)
                }
                Ok((_, dropped)) => (None, dropped, None, Some(CatalogFailure::InvalidCatalog)),
                Err(f) => (None, 0, None, Some(f)),
            },
            Err(f) => (None, 0, None, Some(f)),
        };
        let status = OfficialCatalogStatus {
            available: failure.is_none(),
            checked_at: chrono_now_ms(),
            model_count: catalog
                .as_ref()
                .and_then(|v| v["models"].as_array())
                .map_or(0, |a| a.len() as u64),
            dropped_entries: dropped,
            failure,
        };
        let mut inner = self.inner.lock().map_err(|_| ObserveError::Poisoned)?;
        if inner.generation != generation || inner.scope.as_ref() != Some(scope) {
            return Err(ObserveError::ConfigurationChanged);
        }
        let published = inner.sequence == sequence;
        if published {
            inner.status = Some(status.clone());
            inner.catalog = catalog.clone();
        }
        Ok(Observation {
            status,
            catalog,
            upstream_etag: etag,
            published,
        })
    }
}
fn chrono_now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or_else(
            |e| -(e.duration().as_millis().min(i64::MAX as u128) as i64),
            |d| d.as_millis().min(i64::MAX as u128) as i64,
        )
}
fn official_headers(incoming: &HeaderMap) -> std::result::Result<HeaderMap, CatalogFailure> {
    let raw = incoming
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or(CatalogFailure::MissingAuth)?;
    let token = raw
        .strip_prefix("Bearer ")
        .ok_or(CatalogFailure::MissingAuth)?;
    if token.trim().is_empty() || token.starts_with("aimami-") || token.starts_with("sk-aimami-") {
        return Err(CatalogFailure::MissingAuth);
    }
    let mut filtered = HeaderMap::new();
    for name in OFFICIAL_HEADERS {
        if let Some(value) = incoming.get(name) {
            filtered.insert(name, value.clone());
        }
    }
    Ok(filtered)
}
fn validate_official_models(mut value: Value) -> std::result::Result<(Value, u64), CatalogFailure> {
    let entries = value
        .get_mut("models")
        .and_then(Value::as_array_mut)
        .ok_or(CatalogFailure::InvalidCatalog)?;
    let mut seen = HashSet::new();
    let old_len = entries.len();
    entries.retain(|entry| {
        entry
            .get("slug")
            .and_then(Value::as_str)
            .is_some_and(|slug| {
                !slug.trim().is_empty()
                    && !slug.contains("::")
                    && !slug.starts_with("aimami_relay_")
                    && seen.insert(slug.to_owned())
            })
    });
    let dropped = (old_len - entries.len()) as u64;
    Ok((value, dropped))
}
async fn fetch(
    endpoint: &str,
    incoming: &HeaderMap,
    version: &str,
    timeout: Duration,
) -> std::result::Result<(Value, Option<HeaderValue>), CatalogFailure> {
    let headers = official_headers(incoming)?;
    #[cfg(test)]
    {
        let url = url::Url::parse(endpoint).map_err(|_| CatalogFailure::Network)?;
        if url.scheme() != "http" || url.host_str() != Some("127.0.0.1") {
            return Err(CatalogFailure::Network);
        }
    }
    let builder = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(timeout);
    #[cfg(test)]
    let builder = builder.no_proxy();
    let client = builder.build().map_err(|_| CatalogFailure::Network)?;
    let mut response = client
        .get(endpoint)
        .query(&[("client_version", version)])
        .headers(headers)
        .send()
        .await
        .map_err(classify_network)?;
    if response.status() != reqwest::StatusCode::OK {
        return Err(CatalogFailure::Http {
            status: response.status().as_u16(),
        });
    }
    if response
        .content_length()
        .is_some_and(|n| n > MAX_BYTES as u64)
    {
        return Err(CatalogFailure::TooLarge);
    }
    let etag = response.headers().get(ETAG).cloned();
    let mut body = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(classify_network)? {
        if chunk.len() > MAX_BYTES.saturating_sub(body.len()) {
            return Err(CatalogFailure::TooLarge);
        }
        body.extend_from_slice(&chunk);
    }
    let value = serde_json::from_slice(&body).map_err(|_| CatalogFailure::InvalidCatalog)?;
    Ok((value, etag))
}
fn classify_network(error: reqwest::Error) -> CatalogFailure {
    if error.is_timeout() {
        CatalogFailure::Timeout
    } else {
        CatalogFailure::Network
    }
}
pub(crate) struct FreshnessDiagnostic {
    pub(crate) item: DiagnosticItem,
    pub(crate) ready: bool,
}
pub(crate) fn diagnose(
    scope: &Scope,
    runtime: &OfficialCatalogRuntime,
) -> Result<FreshnessDiagnostic> {
    let (status, label, detail, ready) = if !scope.applicable() {
        (
            "ok",
            "官方目录（当前模式无需获取）",
            "当前模式不要求官方目录；不据此证明官方来源可用。".into(),
            false,
        )
    } else {
        match runtime.status(scope)? {
            None if scope.mode == SourceMode::Native => (
                "ok",
                "等待 ChatGPT 原生目录请求",
                "尚未观测到目录请求，不能据此确认官方目录可用。".into(),
                false,
            ),
            None => (
                "warning",
                "等待实际 ChatGPT 官方目录请求",
                "尚未观测到实际官方 HTTP 目录请求；没有用 Gateway 模型配置替代官方目录。".into(),
                false,
            ),
            Some(s) if s.available => (
                "ok",
                "最近一次官方目录获取成功",
                if scope.mode == SourceMode::Native {
                    format!(
                        "官方模型 {} 个；发现、缓存和刷新由 ChatGPT 原生管理。",
                        s.model_count
                    )
                } else {
                    format!("官方模型 {} 个；已由实际 ChatGPT 身份获取并校验，Atelier 仅保留本次运行态目录。", s.model_count)
                },
                true,
            ),
            Some(s) => (
                "warning",
                "官方目录刷新失败，未返回替代列表",
                if scope.mode == SourceMode::Native {
                    format!("最近检查={}，原因={:?}。已有有效目录由 ChatGPT 原生管理；等待下次原生刷新，无可用目录时不保证混合菜单。", s.checked_at, s.failure)
                } else {
                    format!("最近检查={}，原因={:?}。本次刷新未返回替代列表；Atelier 当前没有可验证的官方缓存。", s.checked_at, s.failure)
                },
                false,
            ),
        }
    };
    Ok(FreshnessDiagnostic {
        item: DiagnosticItem {
            id: "official_catalog_freshness".into(),
            group: "config".into(),
            status: status.into(),
            label: label.into(),
            detail,
            fixable: false,
        },
        ready,
    })
}
#[cfg(test)]
#[path = "official_catalog_tests.rs"]
mod tests;
