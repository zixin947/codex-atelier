//! Read-only credential observations. Never refresh, migrate, or discover a Keychain namespace.
use super::{native_catalog, Context, DiagnosticItem};
use atelier_maintenance_core::catalog::{credentials, state::RelayStateDocument};
use base64::Engine;
use serde_json::Value;
use std::path::Path;

fn item(id: &str, status: &str, detail: impl Into<String>) -> DiagnosticItem {
    DiagnosticItem {
        id: id.into(),
        group: "auth".into(),
        status: status.into(),
        label: if id == "auth_token_expiry" {
            if status == "ok" { "Token 过期检测（无 auth.json，跳过）" } else { "Token 到期检查未通过" }
        } else if status == "error" {
            "API 密钥缺失或读取失败"
        } else if status == "warning" {
            "API 密钥来源尚未确认"
        } else {
            "API 密钥完整"
        }
        .into(),
        detail: detail.into(),
        fixable: false,
    }
}

/// `auth` is the caller's already-read file observation. None is not evidence of health.
/// C run_diagnostics_with_mode:4010..4332: missing access=error; remaining<=0=error;
/// 1..3599=warning; >=3600=ok; undecodable claims=warning. Refresh affects detail only.
pub(super) fn token_expiry(
    auth: Option<&Value>,
    non_file: bool,
    router_on: bool,
    now_seconds: i64,
) -> DiagnosticItem {
    let make = |s, d: String| item("auth_token_expiry", s, d);
    if non_file {
        return make(
            "warning",
            "非文件认证来源未授权观察；未读取钥匙串或临时凭据，不能确认令牌到期".into(),
        );
    }
    if router_on {
        return make(
            "ok",
            "路由启用时原生到期检查不适用；未刷新或验证在线会话".into(),
        );
    }
    let Some(auth) = auth else {
        // Native missing/unreadable/invalid JSON branches return ok for non-applicability,
        // not a claim that a session is healthy (C:3979,3998,4328).
        return make(
            "ok",
            "没有可观察的认证文件；本项不判断文件完整性或登录有效性".into(),
        );
    };
    if auth["auth_mode"] == "apikey" {
        return make(
            "ok",
            "API Key 认证不适用 OAuth access_token 到期检查；未联网验证".into(),
        );
    }
    let Some(token) = auth
        .pointer("/tokens/access_token")
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
    else {
        return make(
            "error",
            "缺少 access_token；仅观察本地文件，未执行登录或刷新".into(),
        );
    };
    let refresh = auth
        .pointer("/tokens/refresh_token")
        .and_then(Value::as_str)
        .is_some_and(|s| !s.is_empty());
    let expiry = token
        .split('.')
        .nth(1)
        .and_then(|s| {
            base64::engine::general_purpose::URL_SAFE_NO_PAD
                .decode(s)
                .ok()
        })
        .and_then(|b| serde_json::from_slice::<Value>(&b).ok())
        .and_then(|v| v["exp"].as_i64());
    let Some(expiry) = expiry else {
        return make(
            "warning",
            "无法解析 access_token 的 exp 声明；未校验签名，未刷新".into(),
        );
    };
    let remaining = expiry.saturating_sub(now_seconds);
    let refresh_detail = if refresh {
        "存在 refresh_token，但本检查不会刷新或确认其有效性"
    } else {
        "缺少 refresh_token，需重新登录后恢复"
    };
    let (status, state) = if remaining <= 0 {
        (
            "error",
            format!("access_token 已过期 {} 秒", remaining.saturating_neg()),
        )
    } else if remaining < 3600 {
        ("warning", format!("access_token 将在 {remaining} 秒内到期"))
    } else {
        ("ok", format!("access_token 本地声明剩余 {remaining} 秒"))
    };
    make(
        status,
        format!("{state}；{refresh_detail}；仅解码声明，未校验签名或在线会话"),
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum KeyObservation {
    Present,
    Missing,
    Failed,
    Unknown,
}

fn key_summary(
    observations: impl IntoIterator<Item = KeyObservation>,
    source: &str,
) -> DiagnosticItem {
    let (mut total, mut missing, mut failed, mut unknown) = (0, 0, 0, 0);
    for observation in observations {
        total += 1;
        match observation {
            KeyObservation::Present => {}
            KeyObservation::Missing => missing += 1,
            KeyObservation::Failed => failed += 1,
            KeyObservation::Unknown => unknown += 1,
        }
    }
    item("api_key_integrity", if missing + failed > 0 { "error" } else if unknown > 0 { "warning" } else { "ok" },
        format!("{source}：检查 {total} 项，缺少 {missing} 项，读取失败 {failed} 项，来源未授权/未确认 {unknown} 项；仅检查、不迁移、不刷新、不联网、不显示密钥"))
}

/// Explicit read-only lookup injection. Production never supplies an implicit native Keychain.
/// Legacy lookup must preserve the core storage order without calling get_api_key (which writes).
pub(super) fn native_keys(
    state: &RelayStateDocument,
    root: &Path,
    mut legacy: impl FnMut(&str) -> KeyObservation,
) -> DiagnosticItem {
    let result = credentials::validate_credentials(state)
        .and_then(|_| credentials::provider_credentials(state));
    let Ok(providers) = result else {
        return item(
            "api_key_integrity",
            "warning",
            "原生凭据引用或状态schema无效；未猜测存储来源或访问钥匙串",
        );
    };
    key_summary(
        providers
            .iter()
            .filter(|p| p.api_key_stored)
            .map(|p| match &p.credential {
                credentials::RelayCredential::None => KeyObservation::Missing,
                credentials::RelayCredential::Legacy => legacy(&p.id),
                credentials::RelayCredential::Revision { id } => {
                    match credentials::read_credential_revision(root, &p.id, id) {
                        Ok(key) if !key.is_empty() => KeyObservation::Present,
                        Ok(_) => KeyObservation::Missing,
                        Err(_) => KeyObservation::Failed,
                    }
                }
            }),
        "Native RelayState",
    )
}

/// Only callers holding explicit source authorization may supply this store/backend.
/// The production inspector deliberately does not construct one for native legacy Keychain.
#[cfg_attr(not(test), allow(dead_code))]
pub(super) fn observe_authorized_legacy<K: credentials::LegacyKeychain>(
    store: &mut credentials::CredentialStore<K>,
    id: &str,
) -> KeyObservation {
    match store.observe_api_key(id) {
        Ok(Some(key)) if !key.is_empty() => KeyObservation::Present,
        Ok(_) => KeyObservation::Missing,
        Err(_) => KeyObservation::Failed,
    }
}

fn gateway_keys(ctx: &Context, catalog_path: &Path) -> DiagnosticItem {
    use crate::gateway::types::{Catalog, Credential, ProfileKind};
    let catalog = super::read_bounded(catalog_path, super::inspect::LIMIT)
        .ok()
        .and_then(|bytes| serde_json::from_slice::<Catalog>(&bytes).ok());
    let Some(catalog) = catalog.filter(|c| !c.host_id.is_empty() && c.profiles.len() <= 64) else {
        return item(
            "api_key_integrity",
            "warning",
            "Atelier Gateway 目录缺失或无效；未套用原生schema默认值",
        );
    };
    key_summary(
        catalog
            .profiles
            .iter()
            .filter(|p| p.enabled && p.kind == ProfileKind::ApiKey)
            .map(|p| {
                if uuid::Uuid::parse_str(&p.id).is_err() {
                    return KeyObservation::Failed;
                }
                let path = ctx
                    .app_root
                    .join("gateway/credentials")
                    .join(format!("{}.json", p.id));
                match super::read_bounded(&path, 256 * 1024)
                    .ok()
                    .and_then(|bytes| serde_json::from_slice::<Credential>(&bytes).ok())
                {
                    Some(c) if c.api_key.as_deref().is_some_and(|k| !k.trim().is_empty()) => {
                        KeyObservation::Present
                    }
                    Some(_) => KeyObservation::Missing,
                    None => KeyObservation::Failed,
                }
            }),
        "Atelier Gateway",
    )
}

pub(super) fn inspect(
    ctx: &Context,
    config: &toml::Value,
    auth: Option<&Value>,
    now_seconds: i64,
) -> Vec<DiagnosticItem> {
    let non_file = matches!(
        config
            .get("cli_auth_credentials_store")
            .and_then(toml::Value::as_str),
        Some("keyring" | "ephemeral")
    );
    let mode = super::route_mode::resolve(ctx, config);
    let expiry = match mode {
        Ok(mode) => token_expiry(auth, non_file, mode.enabled(), now_seconds),
        Err(_) => item(
            "auth_token_expiry",
            "warning",
            "路由模式无法确认，未猜测到期检查适用性",
        ),
    };
    // API key inventory is outside the maintenance repair scope.
    vec![expiry]
}

#[cfg(test)]
#[path = "credential_checks_tests.rs"]
mod tests;
