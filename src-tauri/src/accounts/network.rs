//! Subscription API transport only. Never configures Codex or the model gateway.
use super::storage::{hash, read_external};
use crate::gateway::{storage::atomic_private, types::now_ms};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex,
    },
    time::{Duration, Instant},
};

type Result<T> = std::result::Result<T, String>;
const ENDPOINT: &str = "https://chatgpt.com/backend-api/subscriptions";
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    #[default]
    System,
    Direct,
    Manual,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Settings {
    pub mode: Mode,
    pub url: Option<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Saved {
    version: u8,
    generation: String,
    settings: Settings,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Observation {
    pub state: String,
    pub message: String,
    pub checked_at: u64,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct View {
    pub revision: String,
    pub settings: Settings,
    pub subscription: Option<Observation>,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Detection {
    pub candidates: Vec<String>,
    pub message: String,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Probe {
    pub code: String,
    pub reachable: bool,
    pub status_code: Option<u16>,
    pub elapsed_ms: u64,
    pub message: String,
}
#[derive(Clone)]
pub struct Request {
    pub revision: String,
    pub settings: Settings,
    sequence: u64,
}
pub struct Network {
    path: PathBuf,
    gate: Mutex<Option<(String, Observation)>>,
    sequence: AtomicU64,
}
impl Network {
    pub fn new(root: &Path) -> Self {
        Self {
            path: root.join("api-network.json"),
            gate: Mutex::new(None),
            sequence: AtomicU64::new(0),
        }
    }
    fn read(&self) -> Result<Request> {
        let bytes = read_external(&self.path, 16 * 1024)?;
        let settings = match &bytes {
            None => Settings::default(),
            Some(bytes) => {
                let saved: Saved = serde_json::from_slice(bytes)
                    .map_err(|_| "API网络设置损坏，未覆盖；请检查本地设置")?;
                if saved.version != 1 {
                    return Err("API网络设置版本不受支持".into());
                }
                normalize(saved.settings)?
            }
        };
        Ok(Request {
            sequence: 0,
            revision: bytes
                .as_deref()
                .map(hash)
                .unwrap_or_else(|| "absent".into()),
            settings,
        })
    }
    pub fn view(&self) -> Result<View> {
        let observation = self.gate.lock().map_err(|_| "API网络状态不可用")?;
        let request = self.read()?;
        Ok(View {
            revision: request.revision.clone(),
            settings: request.settings,
            subscription: observation
                .as_ref()
                .filter(|(r, _)| r == &request.revision)
                .map(|(_, o)| o.clone()),
        })
    }
    pub fn save(&self, settings: Settings, revision: &str) -> Result<View> {
        let mut observation = self.gate.lock().map_err(|_| "API网络状态不可用")?;
        if self.read()?.revision != revision {
            return Err("API网络设置已变化，请重新打开后保存".into());
        }
        let settings = normalize(settings)?;
        atomic_private(
            &self.path,
            &serde_json::to_vec(&Saved {
                version: 1,
                generation: uuid::Uuid::new_v4().to_string(),
                settings,
            })
            .map_err(|_| "网络设置编码失败")?,
        )?;
        *observation = None;
        let request = self.read()?;
        Ok(View {
            revision: request.revision,
            settings: request.settings,
            subscription: None,
        })
    }
    pub fn begin(&self) -> Result<Request> {
        let _gate = self.gate.lock().map_err(|_| "API网络状态不可用")?;
        let mut request = self.read()?;
        request.sequence = self.sequence.fetch_add(1, Ordering::SeqCst) + 1;
        Ok(request)
    }
    pub fn record(
        &self,
        request: &Request,
        result: &std::result::Result<super::types::AccountMetadata, String>,
    ) -> Result<bool> {
        let mut observation = self.gate.lock().map_err(|_| "API网络状态不可用")?;
        if self.read()?.revision != request.revision {
            return Ok(false);
        }
        // An older request may still update its own account data, but cannot
        // overwrite the latest subscription-transport observation.
        if self.sequence.load(Ordering::SeqCst) != request.sequence {
            return Ok(true);
        }
        *observation = Some((
            request.revision.clone(),
            Observation {
                state: if result.is_ok() { "ok" } else { "error" }.into(),
                message: result
                    .as_ref()
                    .map(|_| "订阅API响应已校验，资料已读取".into())
                    .unwrap_or_else(|e| e.clone()),
                checked_at: now_ms(),
            },
        ));
        Ok(true)
    }
}
pub fn normalize(mut settings: Settings) -> Result<Settings> {
    if settings.mode != Mode::Manual {
        settings.url = None;
        return Ok(settings);
    }
    settings.url = Some(proxy_url(settings.url.as_deref().unwrap_or(""))?);
    Ok(settings)
}
fn proxy_url(raw: &str) -> Result<String> {
    let raw = raw.trim();
    if raw.is_empty() || raw.len() > 2048 || raw.chars().any(char::is_control) {
        return Err("请输入有效代理地址".into());
    }
    let url = url::Url::parse(raw).map_err(|_| "代理地址格式无效")?;
    if !matches!(url.scheme(), "http" | "https" | "socks5" | "socks5h")
        || url.host_str().is_none()
        || url.port_or_known_default().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || !matches!(url.path(), "" | "/")
    {
        return Err(
            "代理仅支持http/https/socks5/socks5h主机和端口，不接受账号密码、路径、查询或片段"
                .into(),
        );
    }
    Ok(url.to_string())
}
fn environment() -> BTreeMap<String, String> {
    [
        "https_proxy",
        "HTTPS_PROXY",
        "all_proxy",
        "ALL_PROXY",
        "no_proxy",
        "NO_PROXY",
    ]
    .into_iter()
    .filter_map(|k| {
        std::env::var(k)
            .ok()
            .filter(|v| !v.is_empty())
            .map(|v| (k.into(), v))
    })
    .collect()
}
fn env_proxy(env: &BTreeMap<String, String>) -> Result<Option<String>> {
    for key in ["https_proxy", "HTTPS_PROXY", "all_proxy", "ALL_PROXY"] {
        if let Some(value) = env.get(key) {
            return proxy_url(value).map(Some);
        }
    }
    Ok(None)
}
#[derive(Default)]
struct System {
    candidates: Vec<String>,
    https_proxy: Option<String>,
    bypass: Vec<String>,
    pac: bool,
}
fn parse_system(text: &str) -> Result<System> {
    let mut pairs = BTreeMap::new();
    let mut bypass = vec![];
    let mut in_exceptions = false;
    for line in text.lines().map(str::trim) {
        if line.starts_with("ExceptionsList") {
            in_exceptions = true;
            continue;
        }
        if in_exceptions {
            if line == "}" {
                in_exceptions = false;
            } else if let Some((_, value)) = line.split_once(':') {
                bypass.push(value.trim().trim_start_matches("*.").to_owned());
            }
            continue;
        }
        if let Some((key, value)) = line.split_once(':') {
            pairs.insert(key.trim(), value.trim());
        }
    }
    let mut result = System {
        candidates: vec![],
        https_proxy: None,
        bypass,
        pac: pairs.get("ProxyAutoConfigEnable") == Some(&"1")
            || pairs.get("ProxyAutoDiscoveryEnable") == Some(&"1"),
    };
    for (enabled, host, port, scheme) in [
        ("HTTPSEnable", "HTTPSProxy", "HTTPSPort", "http"),
        ("HTTPEnable", "HTTPProxy", "HTTPPort", "http"),
        ("SOCKSEnable", "SOCKSProxy", "SOCKSPort", "socks5h"),
    ] {
        if pairs.get(enabled) == Some(&"1") {
            let host = pairs.get(host).ok_or("系统代理缺少主机")?;
            let port = pairs.get(port).ok_or("系统代理缺少端口")?;
            let host = if host.contains(':') && !host.starts_with('[') {
                format!("[{host}]")
            } else {
                host.to_string()
            };
            let url = proxy_url(&format!("{scheme}://{host}:{port}"))?;
            if enabled != "HTTPEnable" && result.https_proxy.is_none() {
                result.https_proxy = Some(url.clone());
            }
            if !result.candidates.contains(&url) {
                result.candidates.push(url);
            }
        }
    }
    Ok(result)
}
async fn system() -> Result<System> {
    #[cfg(target_os = "macos")]
    {
        let mut command = tokio::process::Command::new("/usr/sbin/scutil");
        command.arg("--proxy").kill_on_drop(true);
        let output = tokio::time::timeout(Duration::from_secs(2), command.output())
            .await
            .map_err(|_| "读取系统代理超时")?
            .map_err(|_| "无法读取系统代理")?;
        if !output.status.success() || output.stdout.len() > 64 * 1024 {
            return Err("系统代理读取失败或超过上限".into());
        }
        parse_system(std::str::from_utf8(&output.stdout).map_err(|_| "系统代理响应不是UTF8")?)
    }
    #[cfg(not(target_os = "macos"))]
    {
        Ok(System::default())
    }
}
pub async fn detect() -> Result<Detection> {
    let env = environment();
    let system = system().await?;
    let mut candidates = system.candidates;
    if let Some(proxy) = env_proxy(&env)? {
        if !candidates.contains(&proxy) {
            candidates.insert(0, proxy);
        }
    }
    Ok(Detection {
        message: if candidates.is_empty() {
            if system.pac {
                "仅检测到PAC/自动发现；不下载执行PAC，请手动填写代理地址"
            } else {
                "未检测到明确代理；未扫描端口，未自动保存"
            }
        } else {
            "已读取本机代理候选；仅填入草稿，测试/保存需分别操作"
        }
        .into(),
        candidates,
    })
}
async fn resolve(settings: &Settings) -> Result<(Option<String>, Option<String>)> {
    let settings = normalize(settings.clone())?;
    match settings.mode {
        Mode::Direct => Ok((None, None)),
        Mode::Manual => Ok((settings.url, None)),
        Mode::System => {
            let env = environment();
            let bypass = env.get("no_proxy").or_else(|| env.get("NO_PROXY")).cloned();
            if let Some(proxy) = env_proxy(&env)? {
                return Ok((Some(proxy), bypass));
            }
            let system = system().await?;
            if system.pac {
                return Err("系统仅配置PAC/自动发现，本客户端不执行PAC；请选择手动代理".into());
            }
            Ok((
                system.https_proxy,
                Some([bypass.unwrap_or_default(), system.bypass.join(",")].join(",")),
            ))
        }
    }
}
pub async fn client(settings: &Settings) -> Result<reqwest::Client> {
    let (proxy, bypass) = resolve(settings).await?;
    build_client(proxy.as_deref(), bypass.as_deref(), Duration::from_secs(15))
}
fn build_client(
    proxy: Option<&str>,
    bypass: Option<&str>,
    timeout: Duration,
) -> Result<reqwest::Client> {
    let mut builder = reqwest::Client::builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .connect_timeout(Duration::from_secs(6))
        .timeout(timeout);
    if let Some(proxy) = proxy {
        let proxy = reqwest::Proxy::all(proxy)
            .map_err(|_| "代理地址无法使用")?
            .no_proxy(bypass.and_then(reqwest::NoProxy::from_string));
        builder = builder.proxy(proxy);
    }
    builder
        .build()
        .map_err(|_| "无法初始化订阅API网络客户端".into())
}
pub fn transport_error(error: &reqwest::Error) -> String {
    let reason = if error.is_timeout() {
        "请求超时"
    } else if error.is_connect() {
        "连接建立失败（网络、代理或TLS原因尚未细分）"
    } else if error.is_builder() {
        "请求/代理配置无效"
    } else if error.is_body() || error.is_decode() {
        "响应读取或格式错误"
    } else {
        "请求发送失败"
    };
    format!("订阅API{reason}；已保留旧订阅资料，额度结果独立处理")
}
pub async fn test(settings: Settings) -> Result<Probe> {
    let client = client(&normalize(settings)?).await?;
    Ok(probe(&client, ENDPOINT).await)
}
async fn probe(client: &reqwest::Client, endpoint: &str) -> Probe {
    let started = Instant::now();
    let result = client
        .get(endpoint)
        .header("Accept", "application/json")
        .header("User-Agent", "CodexAtelier/0.1.0")
        .send()
        .await;
    match result {
        Ok(response) => {
            let status = response.status();
            Probe {
                code: if status.is_success() {
                    "reachable"
                } else {
                    "http_response"
                }
                .into(),
                reachable: true,
                status_code: Some(status.as_u16()),
                elapsed_ms: started.elapsed().as_millis() as u64,
                message: format!(
                    "收到HTTP {}，网络已到达HTTP响应阶段；无令牌测试，不代表账号授权或订阅读取成功",
                    status.as_u16()
                ),
            }
        }
        Err(error) => Probe {
            code: if error.is_timeout() {
                "timeout"
            } else {
                "network_error"
            }
            .into(),
            reachable: false,
            status_code: None,
            elapsed_ms: started.elapsed().as_millis() as u64,
            message: transport_error(&error),
        },
    }
}

#[cfg(test)]
#[path = "network_tests.rs"]
mod tests;
