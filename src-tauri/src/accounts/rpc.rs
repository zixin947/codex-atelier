use serde_json::{json, Value};
use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
    process::Stdio,
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{Child, ChildStdin, ChildStdout, Command},
};
pub struct Rpc {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    next: u64,
    events: VecDeque<Value>,
    pub user_agent: String,
}
impl Rpc {
    pub async fn start(home: &Path) -> Result<Self, String> {
        let binary = binary().ok_or("未找到 Codex CLI。请安装 CLI；Atelier 不会自动升级。")?;
        let mut command = Command::new(&binary);
        command
            .args([
                "app-server",
                "--listen",
                "stdio://",
                "-c",
                "cli_auth_credentials_store=\"file\"",
                "-c",
                "model_provider=\"openai\"",
            ])
            .current_dir(home)
            .env("CODEX_HOME", home)
            .env_remove("OPENAI_API_KEY")
            .env_remove("OPENAI_BASE_URL")
            .env_remove("CODEX_AUTH_JSON")
            .env_remove("ATELIER_DESKTOP_BRIDGE")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true);
        let mut paths = vec![binary
            .parent()
            .unwrap_or(Path::new("/usr/bin"))
            .to_path_buf()];
        paths.extend(std::env::split_paths(
            &std::env::var_os("PATH").unwrap_or_default(),
        ));
        paths.extend([PathBuf::from("/usr/bin"), PathBuf::from("/bin")]);
        if let Ok(path) = std::env::join_paths(paths) {
            command.env("PATH", path);
        }
        let mut child = command
            .spawn()
            .map_err(|_| "无法启动隔离的 Codex 账户接口。")?;
        let stdin = child.stdin.take().ok_or("账户接口输入不可用。")?;
        let stdout = BufReader::new(child.stdout.take().ok_or("账户接口输出不可用。")?);
        let mut rpc = Self {
            child,
            stdin,
            stdout,
            next: 0,
            events: VecDeque::new(),
            user_agent: String::new(),
        };
        let initialized = rpc
            .call(
                "initialize",
                json!({"clientInfo":{"name":"codex_atelier_accounts","version":"0.1.0"}}),
            )
            .await?;
        rpc.user_agent = initialized["userAgent"]
            .as_str()
            .filter(|s| s.len() < 512 && !s.chars().any(char::is_control))
            .unwrap_or("")
            .into();
        rpc.send(json!({"method":"initialized"})).await?;
        Ok(rpc)
    }
    async fn send(&mut self, v: Value) -> Result<(), String> {
        let mut bytes = serde_json::to_vec(&v).map_err(|_| "账户请求编码失败。")?;
        bytes.push(b'\n');
        self.stdin
            .write_all(&bytes)
            .await
            .map_err(|_| "账户接口已关闭。")?;
        self.stdin
            .flush()
            .await
            .map_err(|_| "账户请求发送失败。".into())
    }
    async fn line(&mut self) -> Result<Value, String> {
        let mut bytes = Vec::new();
        loop {
            let available = self
                .stdout
                .fill_buf()
                .await
                .map_err(|_| "账户接口读取失败。")?;
            if available.is_empty() {
                return Err("账户接口已退出。".into());
            }
            let count = available
                .iter()
                .position(|b| *b == b'\n')
                .map(|n| n + 1)
                .unwrap_or(available.len());
            if bytes.len() + count > 2 * 1024 * 1024 {
                return Err("账户接口响应过大。".into());
            }
            let done = available[count - 1] == b'\n';
            bytes.extend_from_slice(&available[..count]);
            self.stdout.consume(count);
            if done {
                return serde_json::from_slice(&bytes)
                    .map_err(|_| "账户接口响应格式不兼容。".into());
            }
        }
    }
    pub async fn call(&mut self, method: &str, params: Value) -> Result<Value, String> {
        self.next += 1;
        let id = self.next;
        self.send(json!({"id":id,"method":method,"params":params}))
            .await?;
        let deadline = tokio::time::Instant::now() + Duration::from_secs(40);
        loop {
            let v = tokio::time::timeout_at(deadline, self.line())
                .await
                .map_err(|_| "账户接口超时；未确认操作完成。")??;
            if v.get("method").is_some() && v.get("id").is_some() {
                self.send(json!({"id":v["id"],"error":{"code":-32601,"message":"Only account management is allowed"}})).await?;
                continue;
            }
            if v["id"] == id {
                if v.get("error").is_some() {
                    return Err(format!("Codex 拒绝 {method}；请检查授权或 CLI 版本。"));
                }
                return v.get("result").cloned().ok_or("账户响应缺少结果。".into());
            }
            if self.events.len() < 32 {
                self.events.push_back(v);
            }
        }
    }
    pub async fn event(&mut self) -> Result<Value, String> {
        if let Some(v) = self.events.pop_front() {
            return Ok(v);
        }
        self.line().await
    }
    pub async fn close(&mut self) {
        let _ = self.child.kill().await;
        let _ = self.child.wait().await;
    }
}
fn binary() -> Option<PathBuf> {
    let mut bins: Vec<PathBuf> =
        std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
            .map(|p| p.join("codex"))
            .collect();
    bins.extend(
        [
            "/opt/homebrew/bin/codex",
            "/usr/local/bin/codex",
            "/Applications/ChatGPT.app/Contents/Resources/codex",
        ]
        .map(PathBuf::from),
    );
    if let Some(home) = std::env::var_os("HOME") {
        if let Ok(entries) = std::fs::read_dir(PathBuf::from(home).join(".nvm/versions/node")) {
            let mut paths: Vec<_> = entries
                .flatten()
                .map(|e| e.path().join("bin/codex"))
                .collect();
            paths.sort();
            paths.reverse();
            bins.extend(paths);
        }
    }
    bins.into_iter().find(|p| p.is_file())
}
