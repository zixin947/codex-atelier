//! Real local gateway -> local official-HTTP fixture -> shared diagnostic runtime.
use super::*;
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU16, Ordering},
};
use tokio::sync::Notify;
struct Fixture {
    root: PathBuf,
    server: Arc<GatewayServer>,
    store: Arc<Store>,
    client: reqwest::Client,
    address: String,
    key: String,
    token: String,
    captured: Arc<Mutex<Vec<(HeaderMap, String)>>>,
    status: Arc<AtomicU16>,
    entered: Arc<Notify>,
    release: Arc<Notify>,
    delay: Arc<AtomicBool>,
    upstream: tokio::task::JoinHandle<()>,
}
impl Fixture {
    async fn new() -> Self {
        let root = std::env::temp_dir().join(format!("atelier-official-http-{}", Uuid::new_v4()));
        let config = root.join("codex/config.toml");
        fs::create_dir_all(config.parent().unwrap()).unwrap();
        fs::write(&config, "model_provider='atelier'\n").unwrap();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let upstream_address = listener.local_addr().unwrap();
        let captured = Arc::new(Mutex::new(vec![]));
        let captured_handler = captured.clone();
        let status = Arc::new(AtomicU16::new(200));
        let status_handler = status.clone();
        let entered = Arc::new(Notify::new());
        let e = entered.clone();
        let release = Arc::new(Notify::new());
        let r = release.clone();
        let delay = Arc::new(AtomicBool::new(false));
        let d = delay.clone();
        let app = Router::new().route("/official/models", get(move |headers: HeaderMap, RawQuery(query): RawQuery| {
            let (captured, status, entered, release, delay) = (captured_handler.clone(), status_handler.clone(), e.clone(), r.clone(), d.clone());
            async move {
                captured.lock().unwrap().push((headers, query.unwrap_or_default())); entered.notify_one();
                if delay.load(Ordering::Acquire) { release.notified().await; }
                let status = StatusCode::from_u16(status.load(Ordering::Acquire)).unwrap();
                let value = if status == StatusCode::OK { json!({"models":[{"slug":"real-official-fixture","native_field":{"preserved":true}},{"slug":"hidden::local"}],"owned":"upstream-fixture"}) }
                    else { json!({"secret":"do not forward arbitrary upstream failure content"}) };
                let mut response = (status, Json(value)).into_response(); response.headers_mut().insert("etag", "\"official-fixture\"".parse().unwrap()); response
            }
        }));
        let upstream = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        let store = Arc::new(Store::new_with_config(root.join("gateway"), config).unwrap());
        let reservation = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = reservation.local_addr().unwrap().port();
        drop(reservation);
        let id = Uuid::new_v4().to_string();
        use base64::Engine;
        let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(
            br#"{"https://api.openai.com/auth":{"chatgpt_account_id":"selected-account-fixture"}}"#,
        );
        let token = format!("header.{payload}.synthetic-signature");
        store
            .upsert(
                Profile {
                    id: id.clone(),
                    kind: ProfileKind::Chatgpt,
                    name: "fixture".into(),
                    email: None,
                    enabled: true,
                    priority: 0,
                    base_url: "https://upstream.invalid/v1".into(),
                    auth_status: "ready".into(),
                    models: vec![GatewayModel {
                        id: "configured-local-only".into(),
                        name: "Local fixture".into(),
                    }],
                    cooldown_until: None,
                    last_error: None,
                    requests: 0,
                    input_tokens: 0,
                    output_tokens: 0,
                    created_at: 0,
                },
                Credential {
                    access_token: Some(token.clone()),
                    expires_at: Some(now_ms() + 3_600_000),
                    ..Default::default()
                },
                None,
            )
            .unwrap();
        store
            .update(None, |c| {
                c.settings.port = port;
                c.settings.preferred_profile_id = Some(id);
                c.settings.model = "configured-local-only".into();
                Ok(())
            })
            .unwrap();
        let client = reqwest::Client::builder().no_proxy().build().unwrap();
        let auth = Arc::new(AuthManager::new(store.clone(), client.clone()));
        let mut server = GatewayServer::new(store.clone(), auth, client.clone());
        server.official_catalog = Arc::new(OfficialCatalogRuntime::with_test_endpoint(format!(
            "http://{upstream_address}/official/models"
        )));
        let server = Arc::new(server);
        server.start().await.unwrap();
        let key = server.local_key().unwrap();
        Self {
            root,
            server,
            store,
            client,
            address: format!("http://127.0.0.1:{port}/v1/models"),
            key,
            token,
            captured,
            status,
            entered,
            release,
            delay,
            upstream,
        }
    }
    async fn request(&self, suffix: &str) -> reqwest::Response {
        self.client
            .get(format!("{}{suffix}", self.address))
            .bearer_auth(&self.key)
            .header("chatgpt-account-id", "wrong-desktop-account")
            .send()
            .await
            .unwrap()
    }
    fn scope(&self) -> Scope {
        self.server.capture_official_scope().unwrap().0
    }
    async fn stop(&self) {
        self.server.stop().await.unwrap();
        for _ in 0..100 {
            if self.server.view().state == "stopped" {
                return;
            }
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        }
        panic!("fixture gateway failed to stop");
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        self.upstream.abort();
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[tokio::test]
async fn actual_gateway_request_uses_selected_identity_and_publishes_official_status() {
    let f = Fixture::new().await;
    let response = f.request("?client_version=0.137.0").await;
    assert_eq!(response.status(), 200);
    assert_eq!(response.headers()["etag"], "\"official-fixture\"");
    let value: Value = response.json().await.unwrap();
    assert_eq!(value["models"].as_array().unwrap().len(), 1);
    assert_eq!(value["models"][0]["slug"], "real-official-fixture");
    assert!(value.get("data").is_none());
    let captured = f.captured.lock().unwrap();
    assert_eq!(captured.len(), 1);
    assert_eq!(
        captured[0].0["authorization"],
        format!("Bearer {}", f.token)
    );
    assert_ne!(captured[0].0["authorization"], format!("Bearer {}", f.key));
    assert_eq!(
        captured[0].0["chatgpt-account-id"],
        "selected-account-fixture"
    );
    assert_eq!(captured[0].1, "client_version=0.137.0");
    drop(captured);
    let scope = f.scope();
    let diagnostic =
        official_catalog::diagnose(&scope, &f.server.official_catalog_runtime()).unwrap();
    assert_eq!(diagnostic.item.status, "ok");
    assert!(diagnostic.ready);
    assert_eq!(
        f.server
            .official_catalog
            .status(&scope)
            .unwrap()
            .unwrap()
            .model_count(),
        1
    );
    f.stop().await;
}
#[tokio::test]
async fn failed_refresh_returns_failure_instead_of_local_or_previous_official_catalog() {
    let f = Fixture::new().await;
    assert_eq!(f.request("?client_version=1.2.3").await.status(), 200);
    f.status.store(500, Ordering::Release);
    let response = f.request("?client_version=1.2.3").await;
    assert_eq!(response.status(), 500);
    let value: Value = response.json().await.unwrap();
    assert!(value.get("models").is_none());
    assert!(value.get("data").is_none());
    assert!(value.get("secret").is_none());
    let scope = f.scope();
    assert!(f
        .server
        .official_catalog
        .cached_catalog(&scope)
        .unwrap()
        .is_none());
    let diagnostic = official_catalog::diagnose(&scope, &f.server.official_catalog).unwrap();
    assert_eq!(diagnostic.item.status, "warning");
    assert!(!diagnostic.ready);
    assert_eq!(f.captured.lock().unwrap().len(), 2);
    f.stop().await;
}
#[tokio::test]
async fn invalid_or_absent_client_version_never_fabricates_an_official_request() {
    let f = Fixture::new().await;
    for suffix in [
        "?client_version=",
        "?client_version=latest",
        "?client_version=1.2.3&client_version=2.3.4",
    ] {
        assert_eq!(f.request(suffix).await.status(), 400);
    }
    let local: Value = f.request("").await.json().await.unwrap();
    assert_eq!(local["data"][0]["id"], "configured-local-only");
    assert!(f.captured.lock().unwrap().is_empty());
    assert!(f
        .server
        .official_catalog
        .status(&f.scope())
        .unwrap()
        .is_none());
    f.stop().await;
}
#[tokio::test]
async fn changing_the_actual_source_while_http_is_in_flight_cannot_publish_old_status() {
    let f = Fixture::new().await;
    f.delay.store(true, Ordering::Release);
    let client = f.client.clone();
    let address = f.address.clone();
    let key = f.key.clone();
    let request = tokio::spawn(async move {
        client
            .get(format!("{address}?client_version=1.2.3"))
            .bearer_auth(key)
            .send()
            .await
            .unwrap()
    });
    tokio::time::timeout(std::time::Duration::from_secs(3), f.entered.notified()).await.expect("fixture request reached loopback");
    f.store
        .update(None, |c| {
            c.settings.model = "changed-model".into();
            Ok(())
        })
        .unwrap();
    f.server.synchronize_official_catalog().unwrap();
    f.release.notify_one();
    let response = request.await.unwrap();
    assert_eq!(response.status(), 409);
    let current = f.scope();
    assert!(f
        .server
        .official_catalog
        .status(&current)
        .unwrap()
        .is_none());
    assert!(f
        .server
        .official_catalog
        .cached_catalog(&current)
        .unwrap()
        .is_none());
    f.stop().await;
}
