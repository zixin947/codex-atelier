//! Process lifecycle and native configuration for the vendored routing core.
use super::{
    routing,
    storage::{atomic_private, private_dir, read_private},
    types::*,
};
use fs2::FileExt;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs::{File, OpenOptions};
use std::{
    collections::VecDeque,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    process::{Child, ChildStdin, Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

struct Process {
    child: Child,
    input: Option<ChildStdin>,
    _lock: File,
}
impl Drop for Process {
    fn drop(&mut self) {
        self.input.take();
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
struct Live {
    runtime: RuntimeView,
    routes: VecDeque<routing::Route>,
    rerouted: u64,
    key: Option<String>,
    management_url: Option<String>,
    generation: u64,
    alive: Arc<AtomicBool>,
}
pub struct Engine {
    binary: PathBuf,
    root: PathBuf,
    process: Mutex<Option<Process>>,
    live: Arc<Mutex<Live>>,
}
impl Engine {
    pub fn new(binary: PathBuf, root: PathBuf) -> Self {
        Self {
            binary,
            root,
            process: Mutex::new(None),
            live: Arc::new(Mutex::new(Live {
                runtime: RuntimeView::default(),
                routes: VecDeque::new(),
                rerouted: 0,
                key: None,
                management_url: None,
                generation: 0,
                alive: Arc::new(AtomicBool::new(false)),
            })),
        }
    }
    pub fn management_url(&self) -> Result<String, String> {
        let live = self.live.lock().map_err(|_| "引擎状态锁异常。")?;
        if live.runtime.state != "running" {
            return Err("请先启动网关。".into());
        }
        live.management_url
            .clone()
            .ok_or("当前引擎未提供原版管理页，请重新启动网关。".into())
    }
    fn lock(&self) -> Result<File, String> {
        private_dir(&self.root)?;
        let mut options = OpenOptions::new();
        options.create(true).read(true).write(true).truncate(false);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600).custom_flags(libc::O_NOFOLLOW);
        }
        let file = options
            .open(self.root.join("engine.lock"))
            .map_err(|_| "无法打开引擎锁。")?;
        file.try_lock_exclusive()
            .map_err(|_| "另一个 Atelier 实例正在使用路由引擎。")?;
        Ok(file)
    }
    pub fn proxy_handle(
        &self,
    ) -> Option<atelier_maintenance_core::router_verification::ProxyHandle> {
        let live = self.live.lock().unwrap();
        let port = url::Url::parse(live.runtime.address.as_deref()?)
            .ok()?
            .port()?;
        Some(atelier_maintenance_core::router_verification::ProxyHandle {
            port,
            alive: live.alive.clone(),
        })
    }
    pub fn view(&self) -> RuntimeView {
        self.live.lock().unwrap().runtime.clone()
    }
    pub fn key(&self) -> Result<String, String> {
        self.live
            .lock()
            .unwrap()
            .key
            .clone()
            .ok_or("请先启动本地网关。".into())
    }
    pub fn snapshot(&self) -> routing::Snapshot {
        let live = self.live.lock().unwrap();
        routing::Snapshot {
            runtime: live.runtime.clone(),
            rerouted_requests: live.rerouted,
            routes: live.routes.iter().cloned().collect(),
        }
    }
    pub fn events(&self) -> Vec<RouteEvent> {
        self.live
            .lock()
            .unwrap()
            .routes
            .iter()
            .filter(|r| r.duration_ms.is_some())
            .map(|r| RouteEvent {
                at: r.at,
                profile_id: r.attempts.last().map(|a| a.profile_id.clone()),
                status: r.status.clone(),
                code: r.code.clone(),
                attempts: r.attempts.len(),
            })
            .collect()
    }
    pub fn configuration(&self) -> Result<Value, String> {
        let path = self.root.join("configuration.json");
        let bytes = match read_private(&path, 1024 * 1024) {
            Ok(b) => b,
            Err(_) if !path.exists() => Vec::new(),
            Err(e) => return Err(e),
        };
        let configuration = if bytes.is_empty() {
            json!({"providerOptions":{},"providers":[],"groups":[],"settings":{},"discoverAccounts":false})
        } else {
            serde_json::from_slice(&bytes).map_err(|_| "引擎配置 JSON 无法解析。")?
        };
        Ok(json!({"revision":format!("{:x}",Sha256::digest(&bytes)),"configuration":configuration}))
    }
    pub fn save_configuration(
        &self,
        configuration: Value,
        expected: &str,
    ) -> Result<Value, String> {
        let mut process = self.process.lock().map_err(|_| "引擎进程锁异常。")?;
        if !matches!(self.view().state.as_str(), "stopped" | "error") {
            return Err("请先停止网关，再修改协议与策略。".into());
        }
        // Reap the previous process and release its configuration lock.
        process.take();
        let _lock = self.lock()?;
        if self.configuration()?["revision"] != expected {
            return Err("引擎配置已被外部修改，请重新读取。".into());
        }
        if !configuration.is_object() {
            return Err("引擎配置必须是 JSON 对象。".into());
        }
        let bytes = serde_json::to_vec_pretty(&configuration).map_err(|e| e.to_string())?;
        if bytes.len() > 1024 * 1024 {
            return Err("引擎配置不能超过 1 MiB。".into());
        }
        private_dir(&self.root)?;
        atomic_private(&self.root.join("configuration.json"), &bytes)?;
        self.configuration()
    }
    pub fn start(&self, mut config: Value) -> Result<(), String> {
        let mut process = self.process.lock().map_err(|_| "引擎进程锁异常。")?;
        if let Some(p) = process.as_mut() {
            if p.child
                .try_wait()
                .map_err(|_| "无法读取引擎状态。")?
                .is_none()
            {
                return Err("网关已运行或正在停止。".into());
            }
        }
        process.take();
        let lock = self.lock()?;
        config["root"] = json!(self.root);
        let key = format!(
            "atelier_{}{}",
            uuid::Uuid::new_v4().simple(),
            uuid::Uuid::new_v4().simple()
        );
        config["key"] = json!(&key);
        let mut child = Command::new(&self.binary)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| "内置路由引擎无法启动，请重新构建 Atelier。")?;
        let mut input = child.stdin.take().unwrap();
        let output = child.stdout.take().unwrap();
        let mut bytes = serde_json::to_vec(&config).map_err(|e| e.to_string())?;
        bytes.push(b'\n');
        if input.write_all(&bytes).is_err() {
            let _ = child.kill();
            let _ = child.wait();
            return Err("无法向内置引擎传递配置。".into());
        }
        let live = self.live.clone();
        let (alive, generation) = {
            let mut l = live.lock().unwrap();
            l.generation += 1;
            l.alive = Arc::new(AtomicBool::new(false));
            (l.alive.clone(), l.generation)
        };
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut ready = false;
            for line in BufReader::new(output).lines() {
                let Ok(line) = line else { break };
                let Ok(v) = serde_json::from_str::<Value>(&line) else {
                    break;
                };
                if v["ready"] == true {
                    let mut l = live.lock().unwrap();
                    if l.generation != generation {
                        return;
                    }
                    l.runtime = RuntimeView {
                        state: "running".into(),
                        address: v["address"].as_str().map(String::from),
                        ..Default::default()
                    };
                    l.key = Some(key.clone());
                    l.management_url = v["managementURL"].as_str().map(String::from);
                    l.routes.clear();
                    l.rerouted = 0;
                    ready = true;
                    alive.store(true, Ordering::SeqCst);
                    let _ = tx.send(Ok(()));
                } else if let Some(trace) = v.get("trace") {
                    let mut l = live.lock().unwrap();
                    if l.generation == generation {
                        let snapshot = routing::native_snapshot(trace, l.runtime.clone());
                        l.runtime = snapshot.runtime;
                        l.routes = snapshot.routes.into();
                        l.rerouted = snapshot.rerouted_requests;
                    }
                } else if v.get("error").is_some() {
                    let _ = tx.send(Err(
                        "内置引擎拒绝了配置，请核对供应商、路由组与策略字段。".to_string()
                    ));
                    break;
                }
            }
            alive.store(false, Ordering::SeqCst);
            let mut l = live.lock().unwrap();
            if l.generation != generation {
                return;
            }
            l.runtime.active_requests = 0;
            l.runtime.state =
                if ready && matches!(l.runtime.state.as_str(), "stopping" | "stopped") {
                    "stopped"
                } else {
                    "error"
                }
                .into();
            l.runtime.address = None;
            l.key = None;
            if !ready {
                let _ = tx.send(Err("内置引擎未完成启动。".into()));
            }
        });
        *process = Some(Process {
            child,
            input: Some(input),
            _lock: lock,
        });
        match rx.recv_timeout(Duration::from_secs(20)) {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => {
                process.take();
                Err(e)
            }
            Err(_) => {
                process.take();
                Err("内置引擎启动超时。".into())
            }
        }
    }
    pub fn stop(&self) -> Result<(), String> {
        let mut process = self.process.lock().map_err(|_| "引擎进程锁异常。")?;
        if let Some(p) = process.as_mut() {
            if p.child
                .try_wait()
                .map_err(|_| "无法读取引擎状态。")?
                .is_some()
            {
                process.take();
                let mut l = self.live.lock().unwrap();
                l.runtime.state = "stopped".into();
                l.runtime.address = None;
                l.runtime.active_requests = 0;
                l.key = None;
            } else {
                self.live.lock().unwrap().runtime.state = "stopping".into();
                p.input.take();
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gateway::{oauth::AuthManager, plan_bridge, storage::Store};
    use axum::{routing::post, Json, Router};
    #[tokio::test]
    async fn vendored_engine_forwards_actual_http_and_preserves_live_image_filter() {
        let root = std::env::temp_dir().join(format!("atelier-core-host-{}", uuid::Uuid::new_v4()));
        let image_config = root.join("config.toml");
        let store =
            Arc::new(Store::new_with_config(root.join("gateway"), image_config.clone()).unwrap());
        let client = reqwest::Client::builder().no_proxy().build().unwrap();
        let auth = Arc::new(AuthManager::new(store.clone(), client.clone()));
        let bridge = plan_bridge::start(auth, store, client.clone(), None)
            .await
            .unwrap();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let upstream = listener.local_addr().unwrap();
        let captured = Arc::new(Mutex::new(Vec::<Value>::new()));
        let out = captured.clone();
        let app=Router::new().route("/v1/responses",post(move |Json(v):Json<Value>|{let out=out.clone();async move{out.lock().unwrap().push(v);Json(json!({"id":"fixture-response","object":"response","status":"completed","output":[{"type":"message","role":"assistant","content":[{"type":"output_text","text":"fixture answer"}]}],"usage":{"input_tokens":1,"output_tokens":1}}))}}));
        let upstream_task = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let engine = Engine::new(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("binaries/atelier-router"),
            root.join("engine"),
        );
        engine.start(json!({"port":0,"defaultProvider":"fixture","defaultModel":"fixture-model","defaultRoute":"fixture/fixture-model","providers":[{"id":"fixture","name":"fixture","key":"private-fixture-key","keyName":"fixture-account","responses":format!("http://{upstream}/v1"),"models":["fixture-model"]}],"groups":[],"settings":{},"prepareURL":format!("{}/prepare",bridge.address),"prepareKey":bridge.key})).unwrap();
        let first_handle = engine.proxy_handle().unwrap();
        assert!(first_handle.alive.load(Ordering::SeqCst));
        let address = engine.view().address.unwrap();
        let key = engine.key().unwrap();
        assert_eq!(
            client
                .get(format!("{address}/models"))
                .send()
                .await
                .unwrap()
                .status(),
            401
        );
        for enabled in [true, false] {
            atelier_maintenance_core::image_compat::set_image_compat(&image_config, enabled)
                .unwrap();
            let r=client.post(format!("{address}/responses")).bearer_auth(&key).json(&json!({"model":"fixture-model","input":"private-fixture-prompt","tools":[{"type":"image_generation"},{"type":"function","name":"keep","parameters":{"type":"object"}}]})).send().await.unwrap();
            let status = r.status();
            let body = r.text().await.unwrap();
            assert_eq!(status, 200, "{body}");
            assert!(body.contains("fixture answer"));
        }
        let bodies = captured.lock().unwrap();
        assert_eq!(bodies.len(), 2);
        assert_eq!(bodies[0]["model"], "fixture-model");
        assert_eq!(bodies[0]["tools"].as_array().unwrap().len(), 1);
        assert_eq!(bodies[1]["tools"].as_array().unwrap().len(), 2);
        drop(bodies);
        tokio::time::timeout(Duration::from_secs(4), async {
            while engine.snapshot().runtime.completed_requests < 2 {
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        })
        .await
        .unwrap();
        assert!(!serde_json::to_string(&engine.snapshot())
            .unwrap()
            .contains("private-fixture"));
        engine.stop().unwrap();
        tokio::time::timeout(Duration::from_secs(4), async {
            while engine.view().state != "stopped" {
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        })
        .await
        .unwrap();
        assert!(!first_handle.alive.load(Ordering::SeqCst));
        let doc = engine.configuration().unwrap();
        let saved = engine
            .save_configuration(json!({"groups":[]}), doc["revision"].as_str().unwrap())
            .unwrap();
        assert!(engine
            .save_configuration(json!({"groups":[]}), doc["revision"].as_str().unwrap())
            .is_err());
        let restart = json!({"port":0,"providers":[],"groups":[],"settings":{}});
        engine.start(restart.clone()).unwrap();
        assert!(!first_handle.alive.load(Ordering::SeqCst));
        let restarted = engine.proxy_handle().unwrap();
        assert!(restarted.alive.load(Ordering::SeqCst));
        let second = Engine::new(engine.binary.clone(), root.join("engine"));
        assert!(second
            .start(restart.clone())
            .unwrap_err()
            .contains("另一个 Atelier"));
        assert!(second
            .save_configuration(json!({}), saved["revision"].as_str().unwrap())
            .is_err());
        engine
            .process
            .lock()
            .unwrap()
            .as_mut()
            .unwrap()
            .child
            .kill()
            .unwrap();
        tokio::time::timeout(Duration::from_secs(4), async {
            while engine.view().state != "error" {
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        })
        .await
        .unwrap();
        assert!(engine.key().is_err());
        assert!(!restarted.alive.load(Ordering::SeqCst));
        engine.stop().unwrap();
        assert_eq!(engine.view().state, "stopped");
        engine.start(restart).unwrap();
        engine.stop().unwrap();
        tokio::time::timeout(Duration::from_secs(4), async {
            while engine.view().state != "stopped" {
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        })
        .await
        .unwrap();
        drop(engine);
        drop(bridge);
        upstream_task.abort();
        std::fs::remove_dir_all(root).unwrap();
    }
}
