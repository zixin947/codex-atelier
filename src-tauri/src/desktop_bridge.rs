//! Opt-in Desktop CLI transport. No app patching, TCP listener or persistent service.
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BridgeStatus {
    pub state: String,
    pub hosts: usize,
    pub loaded_threads: usize,
    pub message: String,
}
impl BridgeStatus {
    pub fn disconnected() -> Self {
        Self {
            state: "disconnected".into(),
            hosts: 0,
            loaded_threads: 0,
            message: "当前 Desktop 尚未提供可确认的会话控制入口，暂不能修改当前会话的 Hook 开关。"
                .into(),
        }
    }
}

pub fn directory() -> PathBuf {
    std::env::var_os("ATELIER_BRIDGE_DIRECTORY")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(std::env::var_os("HOME").unwrap_or_default())
                .join("Library/Application Support/com.codexatelier.desktop/desktop-bridge")
        })
}

#[cfg(unix)]
mod native {
    use super::*;
    use std::{
        fs::{self, OpenOptions},
        io::{self, BufRead, BufReader, Write},
        os::{
            fd::AsRawFd,
            unix::{
                fs::{FileTypeExt, MetadataExt, OpenOptionsExt, PermissionsExt},
                net::UnixStream,
            },
        },
        process::{Child, Command, Stdio},
        sync::{
            atomic::{AtomicBool, Ordering},
            mpsc, Arc,
        },
        time::{Duration, Instant},
    };
    use tungstenite::{client, Message, WebSocket};
    const MAX: usize = 16 * 1024 * 1024;

    #[derive(Serialize, Deserialize)]
    struct Descriptor {
        protocol: u32,
        engine_pid: u32,
        socket: PathBuf,
    }

    fn private_dir(path: &Path) -> Result<(), String> {
        fs::create_dir_all(path).map_err(|_| "无法创建桥接目录。")?;
        let m = fs::symlink_metadata(path).map_err(|_| "无法检查桥接目录。")?;
        if !m.is_dir() || m.uid() != unsafe { libc::getuid() } {
            return Err("桥接目录所有者或类型不安全。".into());
        }
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))
            .map_err(|_| "无法保护桥接目录。".into())
    }
    fn socket_directory() -> PathBuf {
        // Keep below macOS sockaddr_un's 104-byte path limit.
        PathBuf::from(format!("/tmp/atelier-desktop-{}", unsafe {
            libc::getuid()
        }))
    }
    fn secure(path: &Path, socket: bool) -> Result<(), String> {
        let m = fs::symlink_metadata(path).map_err(|_| "Desktop 桥接尚未连接。")?;
        let parent = fs::symlink_metadata(path.parent().ok_or("桥接路径无效。")?)
            .map_err(|_| "桥接目录无效。")?;
        let uid = unsafe { libc::getuid() };
        // Codex materializes Unix listeners as a symlink to its private,
        // hashed daemon socket, even when an explicit short path is supplied.
        if socket && m.file_type().is_symlink() {
            if m.uid() != uid
                || parent.uid() != uid
                || !parent.is_dir()
                || parent.mode() & 0o077 != 0
            {
                return Err("桥接 socket 链接权限不安全。".into());
            }
            let target = fs::canonicalize(path).map_err(|_| "桥接 socket 已失效。")?;
            let expected = PathBuf::from(format!("/private/tmp/codex-daemon-{uid}"));
            if target.parent() != Some(expected.as_path()) {
                return Err("桥接 socket 不指向 Codex 私有目录。".into());
            }
            return secure(&target, true);
        }
        if m.uid() != uid
            || parent.uid() != uid
            || !parent.is_dir()
            || parent.mode() & 0o077 != 0
            || m.mode() & 0o077 != 0
            || if socket {
                !m.file_type().is_socket()
            } else {
                !m.is_file() || m.len() > 4096
            }
        {
            return Err("桥接路径权限或类型不安全，已拒绝连接。".into());
        }
        Ok(())
    }
    fn transient(e: &tungstenite::Error) -> bool {
        matches!(e, tungstenite::Error::Io(e) if matches!(e.kind(), io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut))
    }
    fn connect(path: &Path, pid: u32) -> Result<WebSocket<UnixStream>, String> {
        connect_existing(path, Some(pid), None).map(|(ws, _)| ws)
    }
    fn connect_existing(
        path: &Path,
        expected_pid: Option<u32>,
        desktop: Option<(u32, &Path)>,
    ) -> Result<(WebSocket<UnixStream>, u32), String> {
        secure(path, true)?;
        let stream = UnixStream::connect(path).map_err(|_| "Desktop 桥接连接失败。")?;
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .map_err(|_| "无法设置桥接超时。")?;
        stream
            .set_write_timeout(Some(Duration::from_secs(5)))
            .map_err(|_| "无法设置桥接超时。")?;
        // Socket ownership alone does not prove the expected process owns it.
        #[cfg(target_os = "macos")]
        let pid = {
            let mut actual: libc::pid_t = 0;
            let mut length = std::mem::size_of_val(&actual) as libc::socklen_t;
            let result = unsafe {
                libc::getsockopt(
                    stream.as_raw_fd(),
                    0,
                    libc::LOCAL_PEERPID,
                    (&mut actual as *mut libc::pid_t).cast(),
                    &mut length,
                )
            };
            if result != 0 || actual <= 0 || expected_pid.is_some_and(|pid| actual as u32 != pid) {
                return Err("桥接宿主身份不匹配。".into());
            }
            if let Some((desktop_pid, home)) = desktop {
                crate::desktop_attach::verify(desktop_pid, actual as u32, home)?;
            }
            actual as u32
        };
        #[cfg(not(target_os = "macos"))]
        let pid = {
            if desktop.is_some() {
                return Err("不重启连接目前仅支持 macOS。".into());
            }
            expected_pid.ok_or("缺少宿主身份。")?
        };
        let (ws, _) = client("ws://localhost/rpc", stream).map_err(|_| "Desktop 桥接握手失败。")?;
        ws.get_ref()
            .set_read_timeout(Some(Duration::from_millis(50)))
            .map_err(|_| "无法设置桥接超时。")?;
        Ok((ws, pid))
    }
    pub struct Owner {
        ws: WebSocket<UnixStream>,
        next: u64,
        pub threads: usize,
    }
    impl Owner {
        pub fn call(&mut self, method: &str, params: Value) -> Result<Value, String> {
            self.next += 1;
            let id = self.next;
            self.ws
                .send(Message::Text(
                    json!({"id":id,"method":method,"params":params})
                        .to_string()
                        .into(),
                ))
                .map_err(|_| "Desktop 宿主连接已断开；未确认当前会话生效。")?;
            let until = Instant::now() + Duration::from_secs(8);
            while Instant::now() < until {
                match self.ws.read() {
                    Ok(Message::Text(text)) => {
                        let v: Value =
                            serde_json::from_str(&text).map_err(|_| "宿主协议不兼容。")?;
                        if v["id"] == id {
                            if v.get("error").is_some() {
                                return Err(
                                    "Desktop 拒绝操作：配置冲突或接口不兼容。请刷新。".into()
                                );
                            }
                            return v.get("result").cloned().ok_or("宿主响应不完整。".into());
                        }
                        // Never answer approvals, tool calls or forward thread contents.
                    }
                    Ok(Message::Close(_)) => break,
                    Ok(_) => {}
                    Err(e) if transient(&e) => {}
                    Err(_) => break,
                }
            }
            Err("Desktop 宿主响应超时；未确认当前会话生效。".into())
        }
        fn from_descriptor(path: &Path, codex_home: &Path) -> Result<Self, String> {
            secure(path, false)?;
            let d: Descriptor =
                serde_json::from_slice(&fs::read(path).map_err(|_| "桥接记录不可读。")?)
                    .map_err(|_| "桥接记录格式错误。")?;
            if d.protocol != 1 || d.socket.parent() != Some(socket_directory().as_path()) {
                return Err("桥接记录版本或路径不兼容。".into());
            }
            let ws = connect(&d.socket, d.engine_pid)?;
            Self::initialize(ws, codex_home)
        }
        fn initialize(ws: WebSocket<UnixStream>, codex_home: &Path) -> Result<Self, String> {
            let mut owner = Self {
                ws,
                next: 0,
                threads: 0,
            };
            let init = owner.call("initialize", json!({"clientInfo":{"name":"codex_atelier_control","version":"0.1.0"},"capabilities":{"experimentalApi":true}}))?;
            if init["codexHome"].as_str() != codex_home.to_str() {
                return Err("Desktop 与 Atelier 使用不同的 Codex 用户目录，已停止写入。".into());
            }
            owner
                .ws
                .send(Message::Text(
                    json!({"method":"initialized"}).to_string().into(),
                ))
                .map_err(|_| "桥接连接已关闭。")?;
            let mut cursor = Value::Null;
            for _ in 0..64 {
                let result =
                    owner.call("thread/loaded/list", json!({"cursor":cursor,"limit":100}))?;
                owner.threads += result["data"]
                    .as_array()
                    .ok_or("宿主会话清单格式不兼容。")?
                    .len();
                cursor = result["nextCursor"].clone();
                if cursor.is_null() {
                    return Ok(owner);
                }
            }
            Err("宿主会话清单超过检查上限，未确认范围。".into())
        }
    }
    fn control_socket(home: &Path) -> PathBuf {
        home.join("app-server-control/app-server-control.sock")
    }
    #[cfg(target_os = "macos")]
    fn running_owner(codex_home: &Path) -> Result<Owner, String> {
        let socket = control_socket(codex_home);
        let mut last_error = "当前 Desktop 会话控制入口不可用。".to_string();
        for desktop in crate::desktop_attach::desktops()? {
            let result = connect_existing(&socket, None, Some((desktop, codex_home)))
                .and_then(|(ws, _)| Owner::initialize(ws, codex_home));
            match result {
                Ok(owner) => return Ok(owner),
                Err(error) => last_error = error,
            }
        }
        Err(last_error)
    }
    impl Drop for Owner {
        fn drop(&mut self) {
            let _ = self.ws.close(None);
        }
    }

    pub fn owners(codex_home: &Path) -> Result<Vec<Owner>, String> {
        // Discover the running Desktop directly; no pairing file or process launch.
        #[cfg(target_os = "macos")]
        if control_socket(codex_home).exists() {
            match running_owner(codex_home) {
                Ok(owner) => return Ok(vec![owner]),
                Err(native_error) => {
                    let hosts = recorded_owners(codex_home)?;
                    return if hosts.is_empty() {
                        Err(native_error)
                    } else {
                        Ok(hosts)
                    };
                }
            }
        }
        recorded_owners(codex_home)
    }
    fn recorded_owners(codex_home: &Path) -> Result<Vec<Owner>, String> {
        let root = directory();
        if !root.exists() {
            return Ok(Vec::new());
        }
        // Do not silently treat a stale/unsafe known host as disconnected and write anyway.
        let mut paths: Vec<_> = fs::read_dir(&root)
            .map_err(|_| "桥接目录读取失败。")?
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|v| v == "json"))
            .collect();
        paths.sort();
        if paths.len() > 16 {
            return Err("桥接记录过多，请检查后重试。".into());
        }
        paths
            .iter()
            .map(|path| Owner::from_descriptor(path, codex_home))
            .collect()
    }
    pub fn status(codex_home: &Path) -> BridgeStatus {
        match owners(codex_home) {
            Ok(hosts) if !hosts.is_empty() => BridgeStatus { state: "connected".into(), hosts: hosts.len(),
                loaded_threads: hosts.iter().map(|h| h.threads).sum(),
                message: "已连接 Desktop 宿主。切换后会等待热重载确认；影响此宿主的全部已加载会话，其他独立宿主不在此范围。".into() },
            Ok(_) => BridgeStatus::disconnected(),
            Err(message) => BridgeStatus { state: "error".into(), hosts: 0, loaded_threads: 0, message },
        }
    }

    struct Engine {
        child: Child,
        descriptor: PathBuf,
        socket: PathBuf,
    }
    impl Drop for Engine {
        fn drop(&mut self) {
            // Give the engine a normal shutdown opportunity before forcing exit.
            unsafe {
                libc::kill(self.child.id() as libc::pid_t, libc::SIGTERM);
            }
            let until = Instant::now() + Duration::from_secs(2);
            while Instant::now() < until {
                if self.child.try_wait().ok().flatten().is_some() {
                    break;
                }
                std::thread::sleep(Duration::from_millis(20));
            }
            if self.child.try_wait().ok().flatten().is_none() {
                let _ = self.child.kill();
            }
            let _ = self.child.wait();
            let _ = fs::remove_file(&self.descriptor);
            let _ = fs::remove_file(&self.socket);
        }
    }
    fn relay(binary: &Path, args: &[std::ffi::OsString]) -> Result<i32, String> {
        let root = directory();
        private_dir(&root)?;
        let sockets = socket_directory();
        private_dir(&sockets)?;
        let socket = sockets.join(format!("host-{}.sock", std::process::id()));
        if socket.exists() {
            return Err("桥接 socket 已存在，未覆盖。".into());
        }
        let mut launch_args = args.to_vec();
        if let Ok(base) = std::env::var("ATELIER_GATEWAY_URL") {
            let url = url::Url::parse(&base).map_err(|_| "网关地址格式错误。")?;
            if url.scheme() != "http"
                || url.host_str() != Some("127.0.0.1")
                || url.path() != "/v1"
                || url.port().is_none()
            {
                return Err("只允许连接本机 Atelier 网关。".into());
            }
            let model = std::env::var("ATELIER_GATEWAY_MODEL").map_err(|_| "网关模型未指定。")?;
            if std::env::var("ATELIER_GATEWAY_TOKEN")
                .unwrap_or_default()
                .is_empty()
            {
                return Err("网关访问密钥缺失。".into());
            }
            let overrides = [
                "model_provider=\"atelier_gateway\"".to_string(),
                format!("model={}", serde_json::to_string(&model).unwrap()),
                "model_providers.atelier_gateway.name=\"Codex Atelier\"".to_string(),
                format!(
                    "model_providers.atelier_gateway.base_url={}",
                    serde_json::to_string(&base).unwrap()
                ),
                "model_providers.atelier_gateway.env_key=\"ATELIER_GATEWAY_TOKEN\"".to_string(),
                "model_providers.atelier_gateway.wire_api=\"responses\"".to_string(),
                "model_providers.atelier_gateway.requires_openai_auth=false".to_string(),
                "model_providers.atelier_gateway.supports_websockets=false".to_string(),
            ];
            for value in overrides {
                launch_args.push("-c".into());
                launch_args.push(value.into());
            }
        }
        let child = Command::new(binary)
            .args(&launch_args)
            .arg("--listen")
            .arg(format!("unix://{}", socket.display()))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|_| "无法启动 Desktop 引擎。")?;
        let mut engine = Engine {
            descriptor: root.join(format!("host-{}.json", std::process::id())),
            socket: socket.clone(),
            child,
        };
        let stop = Arc::new(AtomicBool::new(false));
        for sig in [
            signal_hook::consts::SIGTERM,
            signal_hook::consts::SIGINT,
            signal_hook::consts::SIGHUP,
        ] {
            signal_hook::flag::register(sig, Arc::clone(&stop))
                .map_err(|_| "无法注册桥接退出处理。")?;
        }
        let until = Instant::now() + Duration::from_secs(12);
        while !socket.exists() {
            if stop.load(Ordering::Relaxed)
                || Instant::now() > until
                || engine
                    .child
                    .try_wait()
                    .map_err(|_| "引擎状态未知。")?
                    .is_some()
            {
                return Err("Desktop 引擎启动失败或超时。".into());
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        // Codex creates the socket within our 0700 directory; narrow the socket too.
        fs::set_permissions(&socket, fs::Permissions::from_mode(0o600))
            .map_err(|_| "无法保护宿主 socket。")?;
        let mut ws = connect(&socket, engine.child.id())?;
        let (tx, rx) = mpsc::sync_channel(16);
        std::thread::spawn(move || {
            let mut input = BufReader::new(io::stdin());
            loop {
                let mut line = String::new();
                match input.read_line(&mut line) {
                    Ok(0) | Err(_) => break,
                    Ok(_) if line.len() <= MAX => {
                        if tx.send(line).is_err() {
                            break;
                        }
                    }
                    _ => break,
                }
            }
        });
        let mut initialized_id = Value::Null;
        let mut published = false;
        let mut output = io::stdout().lock();
        loop {
            if stop.load(Ordering::Relaxed) {
                break;
            }
            loop {
                match rx.try_recv() {
                    Ok(line) => {
                        let v: Value =
                            serde_json::from_str(&line).map_err(|_| "Desktop 发出无效协议帧。")?;
                        if v["method"] == "initialize" {
                            initialized_id = v["id"].clone();
                        }
                        ws.send(Message::Text(line.trim_end().into()))
                            .map_err(|_| "宿主连接已关闭。")?;
                    }
                    Err(mpsc::TryRecvError::Empty) => break,
                    Err(mpsc::TryRecvError::Disconnected) => return Ok(0),
                }
            }
            match ws.read() {
                Ok(Message::Text(text)) => {
                    if text.len() > MAX {
                        return Err("Desktop 协议帧超过上限。".into());
                    }
                    if !published && !initialized_id.is_null() {
                        let v: Value =
                            serde_json::from_str(&text).map_err(|_| "引擎协议帧错误。")?;
                        if v["id"] == initialized_id && v.get("result").is_some() {
                            let d = Descriptor {
                                protocol: 1,
                                engine_pid: engine.child.id(),
                                socket: socket.clone(),
                            };
                            let bytes = serde_json::to_vec(&d).unwrap();
                            let mut file = OpenOptions::new()
                                .create_new(true)
                                .write(true)
                                .mode(0o600)
                                .open(&engine.descriptor)
                                .map_err(|_| "无法保存桥接连接记录。")?;
                            file.write_all(&bytes)
                                .map_err(|_| "无法保存桥接连接记录。")?;
                            published = true;
                        }
                    }
                    writeln!(output, "{text}")
                        .and_then(|_| output.flush())
                        .map_err(|_| "Desktop 已关闭连接。")?;
                }
                Ok(Message::Close(_)) => break,
                Ok(_) => {}
                Err(e) if transient(&e) => {}
                Err(_) => return Err("Desktop 宿主连接中断。".into()),
            }
        }
        Ok(0)
    }
    pub fn cli() -> i32 {
        let binary = std::env::var_os("ATELIER_REAL_CODEX").map(PathBuf::from);
        let Some(binary) = binary.filter(|p| {
            p.is_absolute() && p.is_file() && Some(p) != std::env::current_exe().ok().as_ref()
        }) else {
            eprintln!("Atelier: missing actual Codex executable");
            return 1;
        };
        let mut args: Vec<_> = std::env::args_os().skip(1).collect();
        let is_server = args.iter().any(|a| a == "app-server")
            && !args.iter().any(|a| {
                [
                    "daemon",
                    "proxy",
                    "generate-ts",
                    "generate-json-schema",
                    "--print-startup-config",
                    "--help",
                ]
                .iter()
                .any(|s| a == *s)
            });
        if !is_server {
            return Command::new(binary)
                .args(args)
                .status()
                .ok()
                .and_then(|s| s.code())
                .unwrap_or(1);
        }
        // --stdio selects the default transport and conflicts with --listen.
        args.retain(|a| a != "--stdio");
        if args
            .iter()
            .any(|a| a == "--listen" || a.to_string_lossy().starts_with("--listen="))
        {
            eprintln!("Atelier: refusing a second listener override");
            return 1;
        }
        match relay(&binary, &args) {
            Ok(code) => code,
            Err(e) => {
                eprintln!("Atelier: {e}");
                1
            }
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn rejects_symlinks_and_world_accessible_records() {
            let root = std::env::temp_dir()
                .join(format!("atelier-bridge-security-{}", std::process::id()));
            private_dir(&root).unwrap();
            let record = root.join("host.json");
            fs::write(&record, "{}").unwrap();
            fs::set_permissions(&record, fs::Permissions::from_mode(0o600)).unwrap();
            assert!(secure(&record, false).is_ok());
            assert!(secure(&record, true).is_err());
            fs::set_permissions(&record, fs::Permissions::from_mode(0o644)).unwrap();
            assert!(secure(&record, false).is_err());
            let link = root.join("link.json");
            std::os::unix::fs::symlink(&record, &link).unwrap();
            assert!(secure(&link, false).is_err());
            fs::remove_dir_all(root).unwrap();
        }
        #[test]
        fn rejects_unknown_or_redirected_legacy_records_before_connecting() {
            let root = std::env::temp_dir()
                .join(format!("atelier-bridge-descriptor-{}", std::process::id()));
            private_dir(&root).unwrap();
            let record = root.join("host.json");
            for descriptor in [
                Descriptor {
                    protocol: 2,
                    engine_pid: 1,
                    socket: control_socket(&root),
                },
                Descriptor {
                    protocol: 1,
                    engine_pid: 1,
                    socket: root.join("other.sock"),
                },
                Descriptor {
                    protocol: 99,
                    engine_pid: 1,
                    socket: socket_directory().join("other.sock"),
                },
            ] {
                fs::write(&record, serde_json::to_vec(&descriptor).unwrap()).unwrap();
                fs::set_permissions(&record, fs::Permissions::from_mode(0o600)).unwrap();
                let result = Owner::from_descriptor(&record, &root);
                assert!(matches!(result, Err(message) if message.contains("版本或路径不兼容")));
            }
            fs::remove_dir_all(root).unwrap();
        }
    }
}
#[cfg(unix)]
pub use native::{cli, owners, status, Owner};
#[cfg(not(unix))]
pub fn cli() -> i32 {
    1
}
#[cfg(not(unix))]
pub fn status(_: &Path) -> BridgeStatus {
    BridgeStatus::disconnected()
}

pub fn launch_with_gateway(base: &str, token: &str, model: &str) -> Result<String, String> {
    launch_desktop(Some((base, token, model)))
}
fn launch_desktop(gateway: Option<(&str, &str, &str)>) -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        use std::process::{Command, Stdio};
        // No surprise termination: the active Desktop turn owns our execution.
        let running = Command::new("/usr/bin/pgrep")
            .args(["-x", "ChatGPT"])
            .stdout(Stdio::null())
            .status()
            .map_err(|_| "无法检查 Desktop 运行状态。")?;
        if running.success() {
            return Err("请先退出 Codex Desktop，再从 Atelier 点击连接。首次桥接需要重新启动 Desktop；原有聊天不删除。".into());
        }
        let real = PathBuf::from("/Applications/ChatGPT.app/Contents/Resources/codex-cli/CodexCLI.app/Contents/MacOS/codex");
        if !real.is_file() {
            return Err("未找到受支持的 Desktop 安装路径。".into());
        }
        let mut command = Command::new("/Applications/ChatGPT.app/Contents/MacOS/ChatGPT");
        command
            .env(
                "CODEX_CLI_PATH",
                std::env::current_exe().map_err(|_| "无法定位 Atelier 桥接程序。")?,
            )
            .env("ATELIER_DESKTOP_BRIDGE", "1")
            .env("ATELIER_REAL_CODEX", real);
        if let Some((base, token, model)) = gateway {
            command
                .env("ATELIER_GATEWAY_URL", base)
                .env("ATELIER_GATEWAY_TOKEN", token)
                .env("ATELIER_GATEWAY_MODEL", model);
        } else {
            command
                .env_remove("ATELIER_GATEWAY_URL")
                .env_remove("ATELIER_GATEWAY_TOKEN")
                .env_remove("ATELIER_GATEWAY_MODEL");
        }
        command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| "无法启动 Desktop。")?;
        Ok("已请求通过桥接启动 Desktop，请打开原有聊天并刷新连接状态。".into())
    }
    #[cfg(not(target_os = "macos"))]
    Err("Desktop 桥接启动目前仅支持 macOS。".into())
}
