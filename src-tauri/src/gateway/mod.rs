pub(crate) mod oauth;
mod server;
pub(crate) mod storage;
pub mod types;
use oauth::AuthManager;
use server::GatewayServer;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use storage::Store;
use tauri::{Manager, State};
use types::*;
use uuid::Uuid;

#[derive(Default)]
pub struct GatewayState(Mutex<Option<Arc<Gateway>>>);
struct Gateway {
    store: Arc<Store>,
    auth: Arc<AuthManager>,
    server: Arc<GatewayServer>,
    client: reqwest::Client,
}
impl GatewayState {
    fn get(&self, app: &tauri::AppHandle) -> Result<Arc<Gateway>, String> {
        let mut guard = self.0.lock().map_err(|_| "网关初始化锁异常。")?;
        if let Some(g) = &*guard {
            return Ok(g.clone());
        }
        let root = app
            .path()
            .app_data_dir()
            .map_err(|_| "无法定位 Atelier 数据目录。")?
            .join("gateway");
        let store = Arc::new(Store::new(root)?);
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .connect_timeout(Duration::from_secs(10))
            .read_timeout(Duration::from_secs(120))
            .user_agent("CodexAtelier/0.1.0")
            .build()
            .map_err(|_| "无法初始化 HTTPS 客户端。")?;
        let auth = Arc::new(AuthManager::new(store.clone(), client.clone()));
        let server = Arc::new(GatewayServer::new(
            store.clone(),
            auth.clone(),
            client.clone(),
        ));
        let gateway = Arc::new(Gateway {
            store,
            auth,
            server,
            client,
        });
        *guard = Some(gateway.clone());
        Ok(gateway)
    }
}
impl Gateway {
    fn snapshot(&self) -> Result<Snapshot, String> {
        let mut c = self.store.read()?;
        self.server.overlay_profiles(&mut c.profiles);
        Ok(Snapshot {
            available: true,
            reason: None,
            revision: c.revision,
            profiles: c.profiles,
            settings: c.settings,
            runtime: self.server.view(),
            events: self.server.events(),
        })
    }
    fn idle(&self) -> Result<(), String> {
        if self.server.view().state != "stopped" && self.server.view().state != "error"
            || self.server.active_requests() > 0
        {
            return Err("请先停止网关并等待在途请求完成。".into());
        }
        Ok(())
    }
}
fn unavailable(reason: String) -> Snapshot {
    Snapshot {
        available: false,
        reason: Some(reason),
        revision: String::new(),
        profiles: vec![],
        settings: Settings::default(),
        runtime: RuntimeView::default(),
        events: vec![],
    }
}
fn valid_name(name: &str) -> Result<String, String> {
    let n = name.trim();
    if n.is_empty() || n.chars().count() > 80 || n.chars().any(|c| c.is_control()) {
        Err("名称需要 1–80 个可显示字符。".into())
    } else {
        Ok(n.into())
    }
}
fn valid_model(model: &str) -> bool {
    !model.is_empty()
        && model.len() <= 200
        && !model.chars().any(|c| c.is_control() || c.is_whitespace())
}
fn base_url(raw: &str) -> Result<String, String> {
    let u = url::Url::parse(raw.trim()).map_err(|_| "请输入完整的 Provider Base URL。")?;
    let local = u
        .host_str()
        .is_some_and(|h| h == "localhost" || h == "127.0.0.1" || h == "[::1]");
    if !(u.scheme() == "https" || (u.scheme() == "http" && local))
        || u.host_str().is_none()
        || !u.username().is_empty()
        || u.password().is_some()
        || u.query().is_some()
        || u.fragment().is_some()
    {
        return Err(
            "Provider 需要 HTTPS 地址（本机回环可用 HTTP），不要在地址中放密钥、查询或片段。"
                .into(),
        );
    }
    Ok(u.as_str().trim_end_matches('/').into())
}
fn self_route(base: &str, port: u16) -> bool {
    url::Url::parse(base).ok().is_some_and(|u| {
        u.scheme() == "http"
            && u.port() == Some(port)
            && u.host_str()
                .is_some_and(|h| matches!(h, "localhost" | "127.0.0.1" | "[::1]"))
    })
}
#[tauri::command]
pub fn gateway_snapshot(app: tauri::AppHandle, state: State<'_, GatewayState>) -> Snapshot {
    state
        .get(&app)
        .and_then(|g| g.snapshot())
        .unwrap_or_else(unavailable)
}
#[tauri::command]
pub async fn gateway_provider_save(
    app: tauri::AppHandle,
    state: State<'_, GatewayState>,
    input: ProviderInput,
    expected_revision: String,
) -> Result<Snapshot, String> {
    let g = state.get(&app)?;
    let c = g.store.read()?;
    let name = valid_name(&input.name)?;
    let base = base_url(&input.base_url)?;
    if input.priority > 1000
        || input.models.len() > 100
        || input.models.iter().any(|m| !valid_model(m.trim()))
    {
        return Err("优先级需为 0–1000，模型 ID 最多 100 项且不能含空白。".into());
    }
    if self_route(&base, c.settings.port) {
        return Err("Provider 不能指向 Atelier 自己的监听端口。".into());
    }
    let id = input.id.unwrap_or_else(|| Uuid::new_v4().to_string());
    let _lock = g.store.credential_lock(&id)?;
    let old = c.profiles.iter().find(|p| p.id == id);
    if old.is_some_and(|p| p.kind != ProfileKind::ApiKey) {
        return Err("不能把订阅账号改为 API Provider。".into());
    }
    if old.is_some() {
        g.idle()?;
    }
    let old_secret = if old.is_some() {
        g.store.credential(&id)?
    } else {
        Credential::default()
    };
    let key = input
        .api_key
        .filter(|s| !s.trim().is_empty())
        .or(old_secret.api_key)
        .ok_or("新增 Provider 需要 API Key。")?;
    if key.len() > 16384 || key.chars().any(|c| c.is_control()) {
        return Err("API Key 格式无效。".into());
    }
    let mut ids = input.models;
    ids.iter_mut().for_each(|s| *s = s.trim().into());
    ids.sort();
    ids.dedup();
    let p = Profile {
        id: id.clone(),
        kind: ProfileKind::ApiKey,
        name,
        email: None,
        enabled: old.map(|p| p.enabled).unwrap_or(true),
        priority: input.priority,
        base_url: base,
        auth_status: "unknown".into(),
        models: ids
            .into_iter()
            .map(|id| GatewayModel {
                name: id.clone(),
                id,
            })
            .collect(),
        cooldown_until: None,
        last_error: None,
        requests: 0,
        input_tokens: 0,
        output_tokens: 0,
        created_at: old.map(|p| p.created_at).unwrap_or_else(now_ms),
    };
    g.store.upsert(
        p,
        Credential {
            api_key: Some(key),
            ..Credential::default()
        },
        Some(&expected_revision),
    )?;
    g.snapshot()
}
#[tauri::command]
pub async fn gateway_profile_update(
    app: tauri::AppHandle,
    state: State<'_, GatewayState>,
    input: ProfileUpdate,
    expected_revision: String,
) -> Result<Snapshot, String> {
    let g = state.get(&app)?;
    let name = valid_name(&input.name)?;
    if input.priority > 1000 {
        return Err("优先级需为 0–1000。".into());
    }
    g.store.update(Some(&expected_revision), |c| {
        let p = c
            .profiles
            .iter_mut()
            .find(|p| p.id == input.id)
            .ok_or("账号不存在。")?;
        if !p.enabled && input.enabled {
            p.cooldown_until = None;
            p.last_error = None;
        }
        p.name = name;
        p.enabled = input.enabled;
        p.priority = input.priority;
        Ok(())
    })?;
    g.snapshot()
}
#[tauri::command]
pub async fn gateway_profile_remove(
    app: tauri::AppHandle,
    state: State<'_, GatewayState>,
    id: String,
    expected_revision: String,
) -> Result<Snapshot, String> {
    let g = state.get(&app)?;
    g.idle()?;
    g.store.remove(&id, &expected_revision)?;
    g.snapshot()
}
#[tauri::command]
pub async fn gateway_settings_save(
    app: tauri::AppHandle,
    state: State<'_, GatewayState>,
    settings: Settings,
    expected_revision: String,
) -> Result<Snapshot, String> {
    let g = state.get(&app)?;
    if settings.port < 1024
        || !(1..=5).contains(&settings.max_attempts)
        || (!settings.model.is_empty() && !valid_model(&settings.model))
    {
        return Err("端口需为 1024–65535，尝试上限需为 1–5，模型 ID 不能含空白。".into());
    }
    let before = g.store.read()?;
    if before.settings.port != settings.port {
        g.idle()?;
    }
    g.store.update(Some(&expected_revision), |c| {
        if settings
            .preferred_profile_id
            .as_ref()
            .is_some_and(|id| !c.profiles.iter().any(|p| &p.id == id))
        {
            return Err("所选账号不存在。".into());
        }
        if c.profiles
            .iter()
            .any(|p| self_route(&p.base_url, settings.port))
        {
            return Err("网关端口与 Provider 地址冲突，会造成循环转发。".into());
        }
        c.settings = settings;
        Ok(())
    })?;
    g.snapshot()
}
#[tauri::command]
pub async fn gateway_profile_select(
    app: tauri::AppHandle,
    state: State<'_, GatewayState>,
    id: String,
    expected_revision: String,
) -> Result<Snapshot, String> {
    let g = state.get(&app)?;
    g.store.update(Some(&expected_revision), |c| {
        let p = c
            .profiles
            .iter()
            .find(|p| p.id == id)
            .ok_or("账号不存在。")?;
        if !p.enabled || !matches!(p.auth_status.as_str(), "ready" | "unknown") {
            return Err("请先启用并授权此账号。".into());
        }
        c.settings.preferred_profile_id = Some(id);
        if !p.models.iter().any(|m| m.id == c.settings.model) {
            c.settings.model = p.models.first().map(|m| m.id.clone()).unwrap_or_default();
        }
        Ok(())
    })?;
    g.snapshot()
}
#[tauri::command]
pub async fn gateway_start(
    app: tauri::AppHandle,
    state: State<'_, GatewayState>,
) -> Result<Snapshot, String> {
    let g = state.get(&app)?;
    g.server.start().await?;
    g.snapshot()
}
#[tauri::command]
pub async fn gateway_stop(
    app: tauri::AppHandle,
    state: State<'_, GatewayState>,
) -> Result<Snapshot, String> {
    let g = state.get(&app)?;
    g.server.stop().await?;
    g.snapshot()
}
#[tauri::command]
pub async fn gateway_local_key(
    app: tauri::AppHandle,
    state: State<'_, GatewayState>,
) -> Result<String, String> {
    state.get(&app)?.server.local_key()
}
#[tauri::command]
pub async fn gateway_models_refresh(
    app: tauri::AppHandle,
    state: State<'_, GatewayState>,
    id: String,
) -> Result<Snapshot, String> {
    let g = state.get(&app)?;
    let c = g.store.read()?;
    let p = c
        .profiles
        .iter()
        .find(|p| p.id == id)
        .cloned()
        .ok_or("账号不存在。")?;
    let token = g.auth.access_token(&id).await?;
    let r = g
        .client
        .get(format!("{}/models", p.base_url.trim_end_matches('/')))
        .bearer_auth(token)
        .timeout(Duration::from_secs(25))
        .send()
        .await
        .map_err(|_| "模型目录连接失败，原列表已保留。")?;
    if !r.status().is_success() {
        return Err(format!(
            "模型目录返回 HTTP {}，原列表已保留。",
            r.status().as_u16()
        ));
    }
    let v = oauth::json_response(r).await?;
    let rows = if p.kind == ProfileKind::Chatgpt {
        v["models"].as_array()
    } else {
        v["data"].as_array()
    }
    .ok_or("模型目录格式不兼容。")?;
    let list: Vec<GatewayModel> = rows
        .iter()
        .filter(|v| p.kind != ProfileKind::Chatgpt || v["visibility"] == "list")
        .filter_map(|v| {
            let id = if p.kind == ProfileKind::Chatgpt {
                v["slug"].as_str()
            } else {
                v["id"].as_str()
            }?;
            valid_model(id).then(|| GatewayModel {
                id: id.into(),
                name: v["display_name"]
                    .as_str()
                    .unwrap_or(id)
                    .chars()
                    .take(160)
                    .collect(),
            })
        })
        .take(100)
        .collect();
    g.store.update(Some(&c.revision), |c| {
        let current = c
            .profiles
            .iter_mut()
            .find(|x| x.id == id)
            .ok_or("账号已移除。")?;
        current.models = list;
        current.auth_status = "ready".into();
        current.last_error = None;
        Ok(())
    })?;
    g.snapshot()
}
#[tauri::command]
pub async fn gateway_oauth_begin(
    app: tauri::AppHandle,
    state: State<'_, GatewayState>,
    name: String,
    account_id: Option<String>,
) -> Result<LoginAttempt, String> {
    state.get(&app)?.auth.begin(name, account_id).await
}
#[tauri::command]
pub async fn gateway_oauth_poll(
    app: tauri::AppHandle,
    state: State<'_, GatewayState>,
    id: String,
) -> Result<LoginAttempt, String> {
    state.get(&app)?.auth.poll(&id)
}
#[tauri::command]
pub async fn gateway_oauth_cancel(
    app: tauri::AppHandle,
    state: State<'_, GatewayState>,
    id: String,
) -> Result<(), String> {
    state.get(&app)?.auth.cancel(&id).await
}
#[tauri::command]
pub async fn gateway_oauth_signout(
    app: tauri::AppHandle,
    state: State<'_, GatewayState>,
    id: String,
) -> Result<Snapshot, String> {
    let g = state.get(&app)?;
    g.idle()?;
    g.auth.sign_out(&id).await?;
    g.snapshot()
}
#[tauri::command]
pub async fn gateway_config_preview(
    app: tauri::AppHandle,
    state: State<'_, GatewayState>,
) -> Result<String, String> {
    let g = state.get(&app)?;
    let c = g.store.read()?;
    let address = g
        .server
        .view()
        .address
        .unwrap_or_else(|| format!("http://127.0.0.1:{}/v1", c.settings.port));
    Ok(format!("# 预览：由你手动接入，不会写入真实 Codex 配置\nmodel_provider = \"atelier_gateway\"\nmodel = {}\n\n[model_providers.atelier_gateway]\nname = \"Codex Atelier\"\nbase_url = {}\nwire_api = \"responses\"\nenv_key = \"ATELIER_GATEWAY_TOKEN\"\nrequires_openai_auth = false\nsupports_websockets = false\n\n# ATELIER_GATEWAY_TOKEN 使用本地网关密钥，不是上游 API Key。\n# 本地网关每次启动生成新密钥；手动接入需要重新设置环境变量。\n",serde_json::to_string(&c.settings.model).unwrap(),serde_json::to_string(&address).unwrap()))
}
#[tauri::command]
pub async fn gateway_launch_desktop(
    app: tauri::AppHandle,
    state: State<'_, GatewayState>,
) -> Result<String, String> {
    let g = state.get(&app)?;
    let v = g.server.view();
    if v.state != "running" {
        return Err("请先启动网关。".into());
    }
    let c = g.store.read()?;
    if !valid_model(&c.settings.model) {
        return Err("请先选择模型。".into());
    }
    crate::desktop_bridge::launch_with_gateway(
        v.address.as_deref().ok_or("网关地址不可用。")?,
        &g.server.local_key()?,
        &c.settings.model,
    )
}
#[tauri::command]
pub async fn gateway_open_login(
    app: tauri::AppHandle,
    state: State<'_, GatewayState>,
    id: String,
) -> Result<(), String> {
    let g = state.get(&app)?;
    let attempt = g.auth.poll(&id)?;
    let url = attempt
        .auth_url
        .filter(|_| attempt.state == "pending")
        .ok_or("授权链接已失效。")?;
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("/usr/bin/open")
            .arg(url)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map_err(|_| "无法打开默认浏览器，请复制授权链接。")?
            .success()
            .then_some(())
            .ok_or("打开浏览器失败。".into())
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = url;
        Err("请复制授权链接到系统浏览器完成登录。".into())
    }
}
