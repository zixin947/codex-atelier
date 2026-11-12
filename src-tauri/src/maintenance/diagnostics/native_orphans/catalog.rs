//! Official model/list snapshot; a Gateway inventory is not an official model pool.
use super::*;
use serde_json::{json, Value};
use std::{
    io::Write,
    process::{Child, ChildStdin},
    sync::mpsc,
    time::{Duration, Instant},
};
#[cfg(not(test))]
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};
pub(crate) trait Rpc {
    fn request(&mut self, method: &str, params: Value, timeout: Duration) -> Result<Value>;
}
struct StdioRpc {
    child: Child,
    input: Option<ChildStdin>,
    output: mpsc::Receiver<Result<Value>>,
    next: u64,
}
impl Drop for StdioRpc {
    fn drop(&mut self) {
        // EOF requests normal app-server shutdown; never force-kill a Codex process.
        drop(self.input.take());
        let deadline = Instant::now() + Duration::from_secs(2);
        while Instant::now() < deadline {
            match self.child.try_wait() {
                Ok(Some(_)) | Err(_) => return,
                Ok(None) => std::thread::sleep(Duration::from_millis(20)),
            }
        }
        eprintln!("maintenance app-server has not exited after stdin EOF; left running");
    }
}
impl StdioRpc {
    #[cfg(test)]
    fn connect(_ctx: &Context) -> Result<Self> {
        Err("测试默认禁止发现或启动真实 Codex；必须显式注入 FakeRpc".into())
    }
    #[cfg(not(test))]
    fn connect(ctx: &Context) -> Result<Self> {
        use atelier_maintenance_core::process::{self, ProcessContext};
        let processes = ProcessContext {
            user_home: std::env::var_os("HOME").map(PathBuf::from),
            codex_home: ctx.home.clone(),
            sqlite_home_env: ctx.sqlite_home_env.clone(),
            cwd: ctx.working_directory.clone(),
        };
        let bundle = process::codex_app_bundle_path(&process::SystemRunner::default(), &processes)
            .ok_or("未找到通过原生校验的 Desktop bundle")?;
        let cli = atelier_maintenance_core::catalog::publication::codex_bundled_cli_path(&bundle)
            .ok_or("未找到 bundle 内 Codex CLI")?;
        // Native AppServerClient::connect @10077a610: fixed argv and explicit CODEX_HOME.
        let mut child = Command::new(cli)
            .args(["app-server", "--stdio"])
            .env("CODEX_HOME", &ctx.home)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(err)?;
        let input = child.stdin.take().ok_or("app-server stdin unavailable")?;
        let stdout = child.stdout.take().ok_or("app-server stdout unavailable")?;
        let (send, output) = mpsc::channel();
        std::thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            loop {
                let mut line = String::new();
                match reader.read_line(&mut line) {
                    Ok(0) => break,
                    Ok(_) => {
                        if send.send(serde_json::from_str(&line).map_err(err)).is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        let _ = send.send(Err(err(e)));
                        break;
                    }
                }
            }
        });
        let mut rpc = Self {
            child,
            input: Some(input),
            output,
            next: 1,
        };
        rpc.request(
            "initialize",
            json!({"clientInfo":{"name":"codex_atelier_maintenance","version":"0.1.0"},"capabilities":{"experimentalApi":true}}),
            Duration::from_secs(10),
        )?;
        let input = rpc.input.as_mut().unwrap();
        serde_json::to_writer(&mut *input, &json!({"method":"initialized"})).map_err(err)?;
        input.write_all(b"\n").map_err(err)?;
        input.flush().map_err(err)?;
        Ok(rpc)
    }
}
impl Rpc for StdioRpc {
    fn request(&mut self, method: &str, params: Value, timeout: Duration) -> Result<Value> {
        let id = self.next;
        self.next += 1;
        let input = self.input.as_mut().unwrap();
        serde_json::to_writer(
            &mut *input,
            &json!({"id":id,"method":method,"params":params}),
        )
        .map_err(err)?;
        input.write_all(b"\n").map_err(err)?;
        input.flush().map_err(err)?;
        let deadline = Instant::now() + timeout;
        loop {
            let v = self
                .output
                .recv_timeout(deadline.saturating_duration_since(Instant::now()))
                .map_err(|e| format!("app-server request failed: {e}"))??;
            if v.get("id").and_then(Value::as_u64) != Some(id) {
                continue;
            }
            if v.get("error").is_some() {
                let code = v["error"]["code"].as_i64().unwrap_or(0);
                return Err(format!(
                    "Codex RPC {method} 被拒绝（{code}），尚未执行孤儿线程修复。"
                ));
            }
            return v
                .get("result")
                .cloned()
                .ok_or("app-server response has no result".into());
        }
    }
}
#[derive(Clone)]
pub(crate) struct OfficialCatalog {
    pub(super) provider: String,
    pub(super) models: BTreeSet<String>,
    pub(super) default: Option<String>,
    pub(super) raw: Vec<Value>,
    pub(super) dependencies: BTreeMap<PathBuf, Option<Stamp>>,
}
impl OfficialCatalog {
    pub(crate) fn provider_target(&self) -> &str {
        &self.provider
    }
}
fn validate_source(config: &Value, native: Option<&RelayStateDocument>) -> Result<()> {
    if !config.is_object() {
        return Err("配置不是有效对象，无法确认官方目录来源".into());
    }
    if config.get("model_catalog_json").is_some_and(|v| !v.is_null()) {
        return Err("自定义 model_catalog_json 不能作为官方孤儿接管依据".into());
    }
    let provider = match config.get("model_provider") {
        None | Some(Value::Null) => "openai",
        Some(Value::String(s)) => s.as_str(),
        _ => return Err("官方目录来源 model_provider 类型无效".into()),
    };
    if provider != "aimai1" {
        return atelier_maintenance_core::router_verification::official_provider_target(config).map(|_| ());
    } else {
        if let Some(native) = native {
            let i = native.catalog_inputs().map_err(err)?;
            if i.router_enabled && !i.no_account_mode {
                let p = &config["model_providers"]["aimai1"];
                let url = p["base_url"].as_str().and_then(|s| url::Url::parse(s).ok());
                let expected = atelier_maintenance_core::manager::native_catalog_revision(native)
                    .map_err(err)?;
                let port = native.proxy_status().map_err(err)?.port;
                if url.is_some_and(|u| {
                    u.scheme() == "http"
                        && u.host_str() == Some("127.0.0.1")
                        && u.port() == Some(port)
                        && u.path() == "/codex/router/v1"
                        && u.query().is_none()
                        && u.fragment().is_none()
                        && u.username().is_empty()
                        && u.password().is_none()
                }) && p["requires_openai_auth"] == true
                    && p["http_headers"]["x-aimami-catalog-revision"] == expected
                {
                    return Ok(());
                }
            }
        }
    }
    Err("无法确证官方 model/list 来源；不使用第三方/Gateway 模型池接管孤儿线程".into())
}
/// Whole operation 20s; each RPC is bounded by min(10s, remaining), max 20 pages.
pub(super) fn prepare_with(ctx: &Context, rpc: &mut impl Rpc) -> Result<OfficialCatalog> {
    let mut dependencies = BTreeMap::new();
    for path in [
        ctx.config(),
        ctx.home.join("auth.json"),
        ctx.home.join("codexmate/relay/state.json"),
    ] {
        dependencies.insert(path.clone(), stamp(&path)?);
    }
    let native = optional(
        &ctx.home.join("codexmate/relay/state.json"),
        8 * 1024 * 1024,
    )?
    .map(|b| RelayStateDocument::from_json(&b).map_err(err))
    .transpose()?;
    if native
        .as_ref()
        .map(|d| d.catalog_inputs().map_err(err))
        .transpose()?
        .is_some_and(|i| i.router_enabled && i.no_account_mode)
    {
        return Err("无账号路由模式不能准备官方孤儿接管目录".into());
    }
    let deadline = Instant::now() + Duration::from_secs(20);
    let request = |rpc: &mut dyn Rpc, method: &str, params: Value| {
        let left = deadline.saturating_duration_since(Instant::now());
        if left.is_zero() {
            return Err("官方 model/list 操作超时".into());
        }
        rpc.request(method, params, left.min(Duration::from_secs(10)))
    };
    let before = request(rpc, "config/read", json!({"includeLayers":true}))?;
    let config = before.get("config").ok_or("config/read has no config")?;
    validate_source(config, native.as_ref())?;
    let mut cursor = None;
    let mut seen_cursors = BTreeSet::new();
    let mut ids = BTreeSet::new();
    let mut raw = vec![];
    for _ in 0..20 {
        let mut params = json!({"includeHidden":true,"limit":100});
        if let Some(c) = cursor.take() {
            params["cursor"] = Value::String(c);
        }
        let page = request(rpc, "model/list", params)?;
        let data = page
            .get("data")
            .and_then(Value::as_array)
            .ok_or("model/list page has no data array")?;
        for value in data {
            let model = value
                .get("model")
                .and_then(Value::as_str)
                .ok_or("model/list model field invalid")?;
            if model.trim().is_empty()
                || value["hidden"] == true
                || model.starts_with("aimami_relay_")
                || model.contains("::")
            {
                continue;
            }
            if ids.insert(model.to_owned()) {
                raw.push(value.clone());
            }
        }
        let next = page
            .get("nextCursor")
            .filter(|v| !v.is_null())
            .map(|v| v.as_str().ok_or("model/list cursor invalid"))
            .transpose()?;
        match next {
            Some(c) if !c.is_empty() => {
                if !seen_cursors.insert(c.to_string()) {
                    return Err("model/list cursor repeated".into());
                }
                cursor = Some(c.to_string());
            }
            _ => break,
        }
    }
    if cursor.is_some() {
        return Err("model/list exceeded native page limit".into());
    }
    let after = request(rpc, "config/read", json!({"includeLayers":true}))?;
    let actual = after.get("config").ok_or("config/read has no config")?;
    validate_source(actual, native.as_ref())?;
    if actual != config {
        return Err("官方目录读取期间 effective config 变化".into());
    }
    if raw.is_empty() {
        return Err("官方模型列表为空，未接管孤儿线程".into());
    }
    // @100510110: hidden/relay-qualified models excluded; isDefault gets priority 0, others index+1.
    let default = raw
        .iter()
        .find(|v| v.get("isDefault").and_then(Value::as_bool) == Some(true))
        .or_else(|| raw.first())
        .and_then(|v| v.get("model"))
        .and_then(Value::as_str)
        .map(str::to_owned);
    for (path, before) in &dependencies {
        if &stamp(path)? != before {
            return Err("官方目录准备期间配置或认证来源变化".into());
        }
    }
    Ok(OfficialCatalog {
        provider: match config["model_provider"].as_str() {
            Some("aimai1") | None => "openai",
            Some(provider) => provider,
        }.to_owned(),
        models: ids,
        default,
        raw,
        dependencies,
    })
}
pub(super) fn prepare(ctx: &Context) -> Result<OfficialCatalog> {
    if let Some(bytes) = optional(
        &ctx.home.join("codexmate/relay/state.json"),
        8 * 1024 * 1024,
    )? {
        let d = RelayStateDocument::from_json(&bytes).map_err(err)?;
        let i = d.catalog_inputs().map_err(err)?;
        if i.router_enabled && i.no_account_mode {
            return Err("无账号路由模式不能准备官方孤儿接管目录".into());
        }
    }
    prepare_with(ctx, &mut StdioRpc::connect(ctx)?)
}

#[cfg(test)]
mod source_tests {
    use super::*;
    #[test]
    fn accepts_official_provider_alias_but_rejects_redirected_or_custom_auth() {
        let official = json!({"model_provider":"openai_https","model_providers":{"openai_https":{"requires_openai_auth":true,"base_url":null}}});
        assert!(validate_source(&official, None).is_ok());
        for (key, value) in [
            ("base_url", json!("https://example.invalid/v1")),
            ("env_key", json!("THIRD_PARTY_KEY")),
            ("http_headers", json!({"Authorization":"fixture"})),
            ("model_catalog_url", json!("https://example.invalid/models")),
        ] {
            let mut config = official.clone();
            config["model_providers"]["openai_https"][key] = value;
            assert!(validate_source(&config, None).is_err());
        }
    }
}
