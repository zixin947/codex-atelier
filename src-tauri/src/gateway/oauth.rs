use super::{storage::Store, types::*};
use axum::{
    extract::{Query, State},
    response::Html,
    routing::get,
    Router,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use jsonwebtoken::{decode, decode_header, jwk::JwkSet, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};
use tokio::sync::Notify;
use uuid::Uuid;

const AUTH: &str = "https://auth.openai.com/api/accounts/authorize";
const TOKEN: &str = "https://auth.openai.com/api/accounts/oauth/token";
const RESOURCE: &str = "https://api.openai.com/v1";
const ISSUER: &str = "https://auth.openai.com";
const SCOPES: &str =
    "openid profile email offline_access resource.invoke chatgpt.tokens.use.direct";
struct Pending {
    view: Mutex<LoginAttempt>,
    stop: Notify,
    consumed: AtomicBool,
    name: String,
    account: Option<String>,
    client_id: Option<String>,
    state: String,
    nonce: String,
    verifier: String,
    redirect: String,
}
pub struct AuthManager {
    store: Arc<Store>,
    client: reqwest::Client,
    pending: Mutex<HashMap<String, Arc<Pending>>>,
}
impl AuthManager {
    pub fn new(store: Arc<Store>, client: reqwest::Client) -> Self {
        Self {
            store,
            client,
            pending: Mutex::new(HashMap::new()),
        }
    }
    pub async fn begin(
        self: &Arc<Self>,
        name: String,
        account: Option<String>,
    ) -> Result<LoginAttempt, String> {
        if name.trim().is_empty() || name.chars().count() > 80 {
            return Err("账号名称需要 1–80 个字符。".into());
        }
        let c = self.store.read()?;
        let client_id = if let Some(id) = &account {
            let p = c
                .profiles
                .iter()
                .find(|p| p.id == *id && p.kind == ProfileKind::Chatgpt)
                .ok_or("订阅账号不存在。")?;
            let cred = self.store.credential(&p.id)?;
            Some(cred.client_id.ok_or("账号注册信息缺失。")?)
        } else {
            if c.profiles.len() >= 64 {
                return Err("账号档案已达上限。".into());
            }
            None
        };
        {
            let mut attempts = self.pending.lock().map_err(|_| "登录状态锁异常。")?;
            attempts.retain(|_, p| p.view.lock().is_ok_and(|v| v.expires_at > now_ms()));
            if attempts
                .values()
                .any(|p| p.account == account && p.view.lock().is_ok_and(|v| v.state == "pending"))
            {
                return Err("此账号已有待完成的授权，请完成或取消后重试。".into());
            }
            if attempts.len() >= 8 {
                return Err("待处理登录过多，请稍后重试。".into());
            }
        }
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .map_err(|_| "无法创建本地登录回调。")?;
        let port = listener
            .local_addr()
            .map_err(|_| "无法读取回调端口。")?
            .port();
        let redirect = format!("http://127.0.0.1:{port}/auth/callback");
        let verifier = random_string();
        let state = random_string();
        let nonce = random_string();
        let mut url = url::Url::parse(AUTH).map_err(|_| "授权地址错误。")?;
        {
            let mut q = url.query_pairs_mut();
            q.append_pair(
                "client_id",
                client_id.as_deref().unwrap_or("dynamic_agent_client"),
            );
            if client_id.is_none() {
                q.append_pair("agent_name_hint", "Codex Atelier");
            }
            q.append_pair("ext_agent_host_id", &c.host_id)
                .append_pair("response_type", "code")
                .append_pair("redirect_uri", &redirect)
                .append_pair("scope", SCOPES)
                .append_pair("resource", RESOURCE)
                .append_pair("state", &state)
                .append_pair("nonce", &nonce)
                .append_pair("code_challenge_method", "S256")
                .append_pair(
                    "code_challenge",
                    &URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes())),
                );
        }
        let view = LoginAttempt {
            id: Uuid::new_v4().to_string(),
            state: "pending".into(),
            auth_url: Some(url.to_string()),
            expires_at: now_ms() + 600_000,
            message: None,
        };
        let pending = Arc::new(Pending {
            view: Mutex::new(view.clone()),
            stop: Notify::new(),
            consumed: AtomicBool::new(false),
            name: name.trim().into(),
            account,
            client_id,
            state,
            nonce,
            verifier,
            redirect,
        });
        self.pending
            .lock()
            .map_err(|_| "登录状态锁异常。")?
            .insert(view.id.clone(), pending.clone());
        let manager = self.clone();
        let router = Router::new()
            .route("/auth/callback", get(callback))
            .with_state((manager, pending.clone()));
        tokio::spawn(async move {
            let timeout_pending = pending.clone();
            let shutdown = async move {
                tokio::select! {_ = timeout_pending.stop.notified()=>{},_ = tokio::time::sleep(Duration::from_secs(600))=>{finish(&timeout_pending,"error",Some("授权已超时，请重新开始。"));}}
            };
            if axum::serve(listener, router)
                .with_graceful_shutdown(shutdown)
                .await
                .is_err()
            {
                finish(&pending, "error", Some("登录回调服务已关闭。"));
            }
        });
        Ok(view)
    }
    pub fn poll(&self, id: &str) -> Result<LoginAttempt, String> {
        let p = self
            .pending
            .lock()
            .map_err(|_| "登录状态锁异常。")?
            .get(id)
            .cloned()
            .ok_or("登录记录不存在或已过期。")?;
        let result = p.view.lock().map_err(|_| "登录状态锁异常。")?.clone();
        Ok(result)
    }
    pub async fn cancel(&self, id: &str) -> Result<(), String> {
        let p = self
            .pending
            .lock()
            .map_err(|_| "登录状态锁异常。")?
            .get(id)
            .cloned()
            .ok_or("登录记录不存在。")?;
        finish(&p, "cancelled", Some("本次授权已取消。"));
        p.stop.notify_one();
        Ok(())
    }
    async fn complete(&self, p: &Pending, query: HashMap<String, String>) -> Result<(), String> {
        if query.get("state") != Some(&p.state) {
            return Err("授权状态不匹配。".into());
        }
        if p.view.lock().map_err(|_| "登录状态锁异常。")?.state != "pending" {
            return Err("本次授权已结束。".into());
        }
        if query.contains_key("error") {
            return Err("授权未完成或已被拒绝。".into());
        }
        let client_id = match &p.client_id {
            Some(id) => {
                if query.get("client_id").is_some_and(|v| v != id) {
                    return Err("授权返回了不同的账号注册标识。".into());
                }
                id.clone()
            }
            None => query
                .get("client_id")
                .filter(|id| id.starts_with("oaiapp_") && id.len() < 256)
                .cloned()
                .ok_or("新授权未返回有效的注册标识。")?,
        };
        let code = query
            .get("code")
            .filter(|v| !v.is_empty() && v.len() < 16384)
            .ok_or("授权回调缺少 code。")?;
        let response = self
            .client
            .post(TOKEN)
            .timeout(Duration::from_secs(25))
            .form(&[
                ("grant_type", "authorization_code"),
                ("client_id", client_id.as_str()),
                ("code", code.as_str()),
                ("code_verifier", p.verifier.as_str()),
                ("redirect_uri", p.redirect.as_str()),
                ("resource", RESOURCE),
            ])
            .send()
            .await
            .map_err(|_| "授权兑换连接失败，请重新发起登录。")?;
        if !response.status().is_success() {
            return Err("授权兑换失败，请重新发起登录。".into());
        }
        let tokens = json_response(response).await?;
        let id_token = tokens["id_token"]
            .as_str()
            .ok_or("授权没有返回身份令牌。")?;
        let claims = self
            .validate_id(id_token, &client_id, Some(&p.nonce))
            .await?;
        let id = p
            .account
            .clone()
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        let _lock = self.store.credential_lock(&id)?;
        let catalog = self.store.read()?;
        let previous = catalog.profiles.iter().find(|v| v.id == id);
        if p.account.is_some() {
            let old = self.store.credential(&id)?;
            if previous.is_none()
                || old.subject.as_deref() != Some(&claims.sub)
                || old.client_id.as_deref() != Some(&client_id)
            {
                return Err("重新授权身份与所选账号不一致，原凭据未替换。".into());
            }
        }
        let credential = credential_from_tokens(
            &tokens,
            Credential {
                client_id: Some(client_id),
                subject: Some(claims.sub),
                issuer: Some(ISSUER.into()),
                ..Credential::default()
            },
        )?;
        let allowed = credential
            .scopes
            .iter()
            .any(|v| v == "chatgpt.tokens.use.direct")
            && credential.scopes.iter().any(|v| v == "resource.invoke");
        // Cancellation can happen while network/JWKS work is in progress. Commit under the attempt lock.
        let mut view = p.view.lock().map_err(|_| "登录状态锁异常。")?;
        if view.state != "pending" || view.expires_at < now_ms() {
            return Err("授权已取消或过期，未保存。".into());
        }
        let profile = Profile {
            id: id.clone(),
            kind: ProfileKind::Chatgpt,
            name: p.name.clone(),
            email: claims.email,
            enabled: allowed && previous.map(|v| v.enabled).unwrap_or(true),
            priority: previous.map(|v| v.priority).unwrap_or(100),
            base_url: RESOURCE.into(),
            auth_status: if allowed { "ready" } else { "planDisabled" }.into(),
            models: previous.map(|v| v.models.clone()).unwrap_or_default(),
            cooldown_until: None,
            last_error: None,
            requests: 0,
            input_tokens: 0,
            output_tokens: 0,
            created_at: previous.map(|v| v.created_at).unwrap_or_else(now_ms),
        };
        self.store
            .upsert(profile, credential, Some(&catalog.revision))?;
        view.state = "completed".into();
        view.auth_url = None;
        view.message = Some(
            if allowed {
                "授权已保存，请手动同步模型并选择当前账号。"
            } else {
                "已登录，但未获得套餐使用权限。"
            }
            .into(),
        );
        Ok(())
    }
    async fn discovery(&self) -> Result<Value, String> {
        let r = self
            .client
            .get("https://auth.openai.com/.well-known/openid-configuration")
            .timeout(Duration::from_secs(20))
            .send()
            .await
            .map_err(|_| "无法读取官方授权元数据。")?;
        if !r.status().is_success() {
            return Err("官方授权元数据暂不可用。".into());
        }
        json_response(r).await
    }
    async fn validate_id(
        &self,
        token: &str,
        client_id: &str,
        nonce: Option<&str>,
    ) -> Result<Identity, String> {
        let meta = self.discovery().await?;
        let jwks = official_url(
            meta["jwks_uri"]
                .as_str()
                .ok_or("官方授权元数据缺少 JWKS。")?,
        )?;
        let r = self
            .client
            .get(jwks)
            .timeout(Duration::from_secs(20))
            .send()
            .await
            .map_err(|_| "无法读取官方签名密钥。")?;
        if !r.status().is_success() {
            return Err("官方签名密钥暂不可用。".into());
        }
        let keys: JwkSet =
            serde_json::from_value(json_response(r).await?).map_err(|_| "签名密钥格式错误。")?;
        let header = decode_header(token).map_err(|_| "身份令牌格式错误。")?;
        if header.alg != Algorithm::RS256 {
            return Err("不支持此身份签名算法。".into());
        }
        let key = keys
            .find(header.kid.as_deref().ok_or("身份令牌缺少密钥标识。")?)
            .ok_or("未找到身份签名密钥。")?;
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[ISSUER]);
        validation.set_audience(&[client_id]);
        validation.set_required_spec_claims(&["exp", "iss", "aud", "sub"]);
        let identity = decode::<Identity>(
            token,
            &DecodingKey::from_jwk(key).map_err(|_| "签名密钥不兼容。")?,
            &validation,
        )
        .map_err(|_| "身份签名、有效期或受众校验失败。")?
        .claims;
        if identity.sub.is_empty() || nonce.is_some_and(|n| identity.nonce.as_deref() != Some(n)) {
            return Err("身份 nonce 校验失败。".into());
        }
        Ok(identity)
    }
    pub async fn access_token(&self, id: &str) -> Result<String, String> {
        let profile = self
            .store
            .read()?
            .profiles
            .into_iter()
            .find(|v| v.id == id)
            .ok_or("账号不存在。")?;
        if profile.kind == ProfileKind::ApiKey {
            return self
                .store
                .credential(id)?
                .api_key
                .filter(|k| !k.is_empty())
                .ok_or("API Key 尚未配置。".into());
        }
        if profile.auth_status != "ready" {
            return Err("账号需要授权或未启用套餐权限。".into());
        }
        let _lock = self.store.credential_lock(id)?;
        let old = self.store.credential(id)?;
        if old.expires_at.is_some_and(|t| t > now_ms() + 60_000) {
            return old.access_token.ok_or("账号缺少访问令牌。".into());
        }
        let refresh = old.refresh_token.as_deref().ok_or("账号需要重新登录。")?;
        let client_id = old.client_id.as_deref().ok_or("账号注册标识缺失。")?;
        let r = self
            .client
            .post(TOKEN)
            .timeout(Duration::from_secs(25))
            .form(&[
                ("grant_type", "refresh_token"),
                ("client_id", client_id),
                ("refresh_token", refresh),
                ("resource", RESOURCE),
            ])
            .send()
            .await
            .map_err(|_| "刷新授权暂时失败，已保留原凭据。")?;
        let status = r.status();
        let value = json_response(r).await?;
        if !status.is_success() {
            let code = value["error"]
                .as_str()
                .or_else(|| value["error"]["code"].as_str())
                .unwrap_or("");
            if [
                "invalid_grant",
                "invalid_refresh_token",
                "token_expired",
                "refresh_token_expired",
                "refresh_token_invalidated",
                "refresh_token_reused",
            ]
            .contains(&code)
            {
                self.store.save_credential(
                    id,
                    &Credential {
                        access_token: None,
                        refresh_token: None,
                        id_token: None,
                        ..old
                    },
                )?;
                self.store.update(None, |c| {
                    if let Some(p) = c.profiles.iter_mut().find(|p| p.id == id) {
                        p.auth_status = "needsLogin".into();
                        p.last_error = Some("授权已失效，请重新登录。".into());
                    }
                    Ok(())
                })?;
                return Err("授权已失效，请重新登录。".into());
            }
            return Err("刷新授权失败，已保留凭据；请稍后重试。".into());
        }
        if let Some(id_token) = value["id_token"].as_str() {
            let identity = self.validate_id(id_token, client_id, None).await?;
            if old.subject.as_deref() != Some(&identity.sub) {
                return Err("刷新返回身份不一致，未替换凭据。".into());
            }
        }
        let fresh = credential_from_tokens(&value, old)?;
        let token = fresh.access_token.clone().ok_or("刷新未返回访问令牌。")?;
        self.store.save_credential(id, &fresh)?;
        if !fresh
            .scopes
            .iter()
            .any(|s| s == "chatgpt.tokens.use.direct")
            || !fresh.scopes.iter().any(|s| s == "resource.invoke")
        {
            self.store.update(None, |c| {
                if let Some(p) = c.profiles.iter_mut().find(|p| p.id == id) {
                    p.auth_status = "planDisabled".into();
                    p.last_error = Some("刷新后的授权未包含套餐访问权限。".into());
                }
                Ok(())
            })?;
            return Err("套餐访问权限已不可用，请重新授权。".into());
        }
        Ok(token)
    }
    pub async fn sign_out(&self, id: &str) -> Result<Option<String>, String> {
        if !self
            .store
            .read()?
            .profiles
            .iter()
            .any(|p| p.id == id && p.kind == ProfileKind::Chatgpt)
        {
            return Err("仅订阅账号支持退出授权。".into());
        }
        let _lock = self.store.credential_lock(id)?;
        let old = self.store.credential(id)?;
        let revoked = if let (Some(refresh), Some(client)) = (&old.refresh_token, &old.client_id) {
            if let Ok(meta) = self.discovery().await {
                if let Some(endpoint) = meta["revocation_endpoint"]
                    .as_str()
                    .and_then(|s| official_url(s).ok())
                {
                    self.client
                        .post(endpoint)
                        .timeout(Duration::from_secs(20))
                        .form(&[
                            ("token", refresh.as_str()),
                            ("token_type_hint", "refresh_token"),
                            ("client_id", client.as_str()),
                        ])
                        .send()
                        .await
                        .is_ok_and(|r| r.status().as_u16() == 200)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            true
        };
        self.store.save_credential(
            id,
            &Credential {
                access_token: None,
                refresh_token: None,
                id_token: None,
                ..old
            },
        )?;
        self.store.update(None, |c| {
            let p = c
                .profiles
                .iter_mut()
                .find(|p| p.id == id)
                .ok_or("账号不存在。")?;
            p.auth_status = "needsLogin".into();
            p.enabled = false;
            p.last_error = if revoked {
                None
            } else {
                Some("已清除本地令牌；远程撤销未确认，请到 ChatGPT 设置管理授权。".into())
            };
            Ok(())
        })?;
        Ok(if revoked {
            None
        } else {
            Some("远程撤销未确认。".into())
        })
    }
}
#[derive(Clone, Deserialize)]
struct Identity {
    sub: String,
    email: Option<String>,
    nonce: Option<String>,
}
fn random_string() -> String {
    format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple())
}
fn finish(p: &Pending, state: &str, message: Option<&str>) {
    if let Ok(mut v) = p.view.lock() {
        if v.state == "pending" {
            v.state = state.into();
            v.auth_url = None;
            v.message = message.map(String::from);
        }
    }
}
async fn callback(
    State((manager, p)): State<(Arc<AuthManager>, Arc<Pending>)>,
    Query(q): Query<HashMap<String, String>>,
) -> Html<&'static str> {
    if q.get("state") != Some(&p.state) {
        return Html("Invalid authorization state. Close this tab and return to Atelier.");
    }
    if p.consumed.swap(true, Ordering::SeqCst) {
        return Html("This authorization callback was already used.");
    }
    let result = manager.complete(&p, q).await;
    if let Err(e) = &result {
        finish(&p, "error", Some(e));
    }
    p.stop.notify_one();
    Html(if result.is_ok() {
        "Authorization received. Return to Codex Atelier. You may close this tab."
    } else {
        "Authorization could not be completed. Return to Codex Atelier for details."
    })
}
fn official_url(s: &str) -> Result<url::Url, String> {
    let u = url::Url::parse(s).map_err(|_| "官方授权地址格式错误。")?;
    if u.scheme() != "https"
        || u.host_str() != Some("auth.openai.com")
        || !u.username().is_empty()
        || u.password().is_some()
    {
        return Err("授权元数据包含不受信任的地址。".into());
    }
    Ok(u)
}
fn credential_from_tokens(v: &Value, mut previous: Credential) -> Result<Credential, String> {
    if v["token_type"]
        .as_str()
        .is_some_and(|s| !s.eq_ignore_ascii_case("bearer"))
    {
        return Err("不支持此授权令牌类型。".into());
    }
    let access = v["access_token"]
        .as_str()
        .filter(|s| !s.is_empty() && s.len() < 32768)
        .ok_or("授权未返回访问令牌。")?;
    let expires = v["expires_in"]
        .as_u64()
        .filter(|n| *n > 0 && *n <= 86400)
        .ok_or("授权未返回有效期限。")?;
    previous.access_token = Some(access.into());
    previous.expires_at = Some(now_ms() + expires * 1000);
    if let Some(s) = v["refresh_token"].as_str() {
        previous.refresh_token = Some(s.into());
    }
    if let Some(s) = v["id_token"].as_str() {
        previous.id_token = Some(s.into());
    }
    if let Some(s) = v["scope"].as_str() {
        previous.scopes = s.split_whitespace().map(String::from).collect();
    }
    Ok(previous)
}
pub async fn json_response(mut response: reqwest::Response) -> Result<Value, String> {
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|_| "上游响应读取失败。")? {
        if bytes.len() + chunk.len() > 256 * 1024 {
            return Err("上游元数据超过大小上限。".into());
        }
        bytes.extend_from_slice(&chunk);
    }
    serde_json::from_slice(&bytes).map_err(|_| "上游未返回有效 JSON。".into())
}
