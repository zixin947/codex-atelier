use super::types::AccountMetadata;
use crate::gateway::types::now_ms;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use serde_json::Value;
use std::time::Duration;

fn text(value: &Value, max: usize) -> Option<String> {
    value
        .as_str()
        .map(str::trim)
        .filter(|s| !s.is_empty() && !s.chars().any(char::is_control))
        .map(|s| s.chars().take(max).collect())
}
pub fn timestamp(value: &Value) -> Option<u64> {
    if let Some(number) = value.as_u64().or_else(|| value.as_str()?.parse().ok()) {
        let millis = if number < 100_000_000_000 {
            number.checked_mul(1000)?
        } else {
            number
        };
        return (millis > 0 && millis < 4_102_444_800_000).then_some(millis);
    }
    let value = value.as_str()?;
    let date =
        time::OffsetDateTime::parse(value, &time::format_description::well_known::Rfc3339).ok()?;
    let millis = u64::try_from(date.unix_timestamp_nanos() / 1_000_000).ok()?;
    (millis > 0 && millis < 4_102_444_800_000).then_some(millis)
}
pub fn from_auth(auth: &Value) -> AccountMetadata {
    let claims = auth["tokens"]["id_token"]
        .as_str()
        .and_then(|t| t.split('.').nth(1))
        .and_then(|t| URL_SAFE_NO_PAD.decode(t).ok())
        .and_then(|b| serde_json::from_slice::<Value>(&b).ok())
        .unwrap_or(Value::Null);
    let namespace = &claims["https://api.openai.com/auth"];
    if namespace["chatgpt_account_id"]
        .as_str()
        .is_some_and(|id| Some(id) != auth["tokens"]["account_id"].as_str())
    {
        return AccountMetadata {
            error: Some("登录快照工作区不一致，未采用账户元数据。".into()),
            ..AccountMetadata::default()
        };
    }
    let name = text(&claims["name"], 100)
        .or_else(|| text(&claims["https://api.openai.com/profile"]["name"], 100))
        .or_else(|| text(&claims["nickname"], 100));
    let mut metadata = AccountMetadata {
        account_name: name,
        workspace_name: text(&namespace["organization_name"], 120),
        profile_name: text(&namespace["profile_name"], 100),
        plan_snapshot: text(&namespace["chatgpt_plan_type"], 60),
        subscription_expires_at: timestamp(&namespace["chatgpt_subscription_active_until"]),
        source: "tokenSnapshot".into(),
        source_at: timestamp(&namespace["chatgpt_subscription_last_checked"])
            .or_else(|| timestamp(&claims["iat"])),
        ..AccountMetadata::default()
    };
    // An ID token's exp is an authentication expiry, never a subscription expiry.
    metadata.has_active_subscription = namespace["has_active_subscription"].as_bool();
    metadata.subscription_will_renew = namespace["subscription_will_renew"].as_bool();
    metadata
}
pub fn merge(previous: &AccountMetadata, incoming: &AccountMetadata) -> AccountMetadata {
    let mut out = previous.clone();
    if incoming.account_name.is_some() {
        out.account_name = incoming.account_name.clone();
    }
    if incoming.workspace_name.is_some() {
        out.workspace_name = incoming.workspace_name.clone();
    }
    if incoming.profile_name.is_some() {
        out.profile_name = incoming.profile_name.clone();
    }
    if incoming.plan_snapshot.is_some()
        && (previous.source != "subscriptionApi"
            || incoming.source == "subscriptionApi"
            || previous.plan_snapshot.is_none())
    {
        out.plan_snapshot = incoming.plan_snapshot.clone();
    }
    // A newly decoded token does not replace a previously confirmed subscription response.
    if incoming.source == "subscriptionApi" {
        out.has_active_subscription = incoming.has_active_subscription;
        out.subscription_will_renew = incoming.subscription_will_renew;
        out.is_delinquent = incoming.is_delinquent;
    }
    if previous.source != "subscriptionApi" || incoming.source == "subscriptionApi" {
        if incoming.subscription_expires_at.is_some() {
            out.subscription_expires_at = incoming.subscription_expires_at;
        }
        if incoming.has_active_subscription.is_some() {
            out.has_active_subscription = incoming.has_active_subscription;
        }
        if incoming.subscription_will_renew.is_some() {
            out.subscription_will_renew = incoming.subscription_will_renew;
        }
        if incoming.is_delinquent.is_some() {
            out.is_delinquent = incoming.is_delinquent;
        }
        if incoming.source_at.is_some() {
            out.source_at = incoming.source_at;
        }
        if incoming.source != "none" {
            out.source = incoming.source.clone();
        }
    }
    if incoming.source == "subscriptionApi" || incoming.error.is_some() {
        out.error = incoming.error.clone();
    }
    out
}
pub async fn subscription(auth: &Value, user_agent: &str) -> Result<AccountMetadata, String> {
    let token = auth["tokens"]["access_token"]
        .as_str()
        .filter(|s| !s.is_empty())
        .ok_or("缺少本应用管理的 Codex 访问令牌。")?;
    let access_claims = token
        .split('.')
        .nth(1)
        .and_then(|part| URL_SAFE_NO_PAD.decode(part).ok())
        .and_then(|bytes| serde_json::from_slice::<Value>(&bytes).ok())
        .unwrap_or(Value::Null);
    let siwc_scope = access_claims["scope"].as_str().is_some_and(|s| {
        s.split_whitespace()
            .any(|v| v == "chatgpt.tokens.use.direct")
    }) || access_claims["scp"]
        .as_array()
        .is_some_and(|s| s.iter().any(|v| v == "chatgpt.tokens.use.direct"));
    if siwc_scope {
        return Err("网关的第三方套餐授权不能用于原生 Codex 订阅元数据接口。".into());
    }
    let id = auth["tokens"]["account_id"]
        .as_str()
        .filter(|s| !s.is_empty() && s.len() < 256)
        .ok_or("账号缺少工作区标识。")?;
    // Only ordinary Codex accounts reach this module, never gateway SIWC registrations.
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .http1_only()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|_| "无法初始化订阅读取。")?;
    let user_agent = if user_agent.is_empty() {
        "CodexAtelier/0.1.0"
    } else {
        user_agent
    };
    let r = client
        .get("https://chatgpt.com/backend-api/subscriptions")
        .query(&[("account_id", id)])
        .bearer_auth(token)
        .header("Accept", "application/json")
        .header("ChatGPT-Account-ID", id)
        .header("OpenAI-Beta", "codex-1")
        .header("Originator", "codex_atelier_accounts")
        .header("User-Agent", user_agent)
        .send()
        .await
        .map_err(|_| "订阅信息连接失败，已保留上次数据。")?;
    if !r.status().is_success() {
        return Err(format!(
            "订阅信息返回 HTTP {}，已保留上次数据。",
            r.status().as_u16()
        ));
    }
    let v = crate::gateway::oauth::json_response(r).await?;
    if !v.is_object() {
        return Err("订阅信息格式不兼容，已保留上次数据。".into());
    }
    if v["account_id"]
        .as_str()
        .is_some_and(|returned| returned != id)
    {
        return Err("订阅信息所属账号不一致，未采用响应。".into());
    }
    let expires = timestamp(&v["active_until"]);
    if expires.is_none() {
        return Err("订阅接口未提供可识别的到期时间。".into());
    }
    Ok(AccountMetadata {
        plan_snapshot: text(&v["plan_type"], 60),
        subscription_expires_at: expires,
        has_active_subscription: v["has_active_subscription"]
            .as_bool()
            .or_else(|| v["is_active"].as_bool()),
        subscription_will_renew: v["will_renew"].as_bool(),
        is_delinquent: v["is_delinquent"].as_bool(),
        source: "subscriptionApi".into(),
        source_at: Some(now_ms()),
        ..AccountMetadata::default()
    })
}
