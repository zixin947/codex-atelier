use super::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[test]
fn validates_proxy_scope_and_never_accepts_embedded_credentials() {
    for address in [
        "http://127.0.0.1:7890",
        "https://proxy.example.invalid:443",
        "socks5://127.0.0.1:1080",
        "socks5h://[::1]:1080",
    ] {
        assert!(normalize(Settings {
            mode: Mode::Manual,
            url: Some(address.into())
        })
        .is_ok());
    }
    for address in [
        "",
        "file:///tmp/socket",
        "http://u:p@localhost:1",
        "http://localhost:1/path",
        "http://localhost:1?token=fixture",
        "http://localhost:1#fragment",
        "http://localhost:1\nX: y",
    ] {
        assert!(
            normalize(Settings {
                mode: Mode::Manual,
                url: Some(address.into())
            })
            .is_err(),
            "{address:?}"
        );
    }
    assert_eq!(
        normalize(Settings {
            mode: Mode::Direct,
            url: Some("untrusted".into())
        })
        .unwrap()
        .url,
        None
    );
}

#[test]
fn system_parser_keeps_https_socks_and_bypass_and_identifies_pac() {
    let system = parse_system("<dictionary> {\nHTTPSEnable : 1\nHTTPSProxy : 127.0.0.1\nHTTPSPort : 7890\nSOCKSEnable : 1\nSOCKSProxy : ::1\nSOCKSPort : 1080\nExceptionsList : <array> {\n0 : *.local\n1 : 127.0.0.1\n}\nProxyAutoConfigEnable : 1\n}").unwrap();
    assert_eq!(
        system.candidates,
        ["http://127.0.0.1:7890/", "socks5h://[::1]:1080"]
    );
    assert!(system.bypass.contains(&"local".into()));
    assert!(system.pac);
    assert!(parse_system("HTTPSEnable : 1\nHTTPSProxy : localhost").is_err());
}

#[test]
fn settings_are_read_only_until_save_and_old_observations_cannot_turn_green() {
    let root = std::env::temp_dir().join(format!("atelier-api-network-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&root).unwrap();
    let network = Network::new(&root);
    let first = network.view().unwrap();
    assert_eq!(first.settings.mode, Mode::System);
    assert_eq!(std::fs::read_dir(&root).unwrap().count(), 0);
    let request = network.begin().unwrap();
    network
        .record(
            &request,
            &Ok(super::super::types::AccountMetadata::default()),
        )
        .unwrap();
    assert_eq!(network.view().unwrap().subscription.unwrap().state, "ok");
    let next = network
        .save(
            Settings {
                mode: Mode::Direct,
                url: None,
            },
            &first.revision,
        )
        .unwrap();
    assert!(next.subscription.is_none());
    assert!(!network
        .record(
            &request,
            &Ok(super::super::types::AccountMetadata::default())
        )
        .unwrap());
    assert!(network.save(Settings::default(), &first.revision).is_err());
    assert!(network.view().unwrap().subscription.is_none());
    let older = network.begin().unwrap();
    let newer = network.begin().unwrap();
    network
        .record(&newer, &Err("latest failure".into()))
        .unwrap();
    network
        .record(&older, &Ok(super::super::types::AccountMetadata::default()))
        .unwrap();
    assert_eq!(network.view().unwrap().subscription.unwrap().state, "error");
    let repeated = network
        .save(
            Settings {
                mode: Mode::Direct,
                url: None,
            },
            &next.revision,
        )
        .unwrap();
    assert_ne!(repeated.revision, next.revision);
    assert!(!network
        .record(&newer, &Ok(super::super::types::AccountMetadata::default()))
        .unwrap());
    std::fs::write(&network.path, b"invalid").unwrap();
    assert!(network.view().is_err());
    std::fs::remove_dir_all(root).unwrap();
}

async fn server(status: &str) -> (String, tokio::task::JoinHandle<String>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}/probe", listener.local_addr().unwrap());
    let response = format!("HTTP/1.1 {status}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
    let task = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut bytes = vec![0; 8192];
        let n = stream.read(&mut bytes).await.unwrap();
        stream.write_all(response.as_bytes()).await.unwrap();
        String::from_utf8_lossy(&bytes[..n]).to_string()
    });
    (url, task)
}
#[tokio::test]
async fn tokenless_probe_distinguishes_http_response_from_subscription_success() {
    for status in ["200 OK", "401 Unauthorized", "403 Forbidden", "302 Found"] {
        let (url, task) = server(status).await;
        let client = build_client(None, None, Duration::from_secs(1)).unwrap();
        let result = probe(&client, &url).await;
        let request = task.await.unwrap().to_lowercase();
        assert!(result.reachable);
        assert!(result.message.contains("不代表账号授权"));
        assert!(!request.contains("authorization"));
        assert!(!request.contains("chatgpt-account-id"));
        assert_eq!(
            result.code,
            if status.starts_with("200") {
                "reachable"
            } else {
                "http_response"
            }
        );
    }
}
#[tokio::test]
async fn explicit_proxy_receives_request_and_no_proxy_bypass_is_respected() {
    let (proxy, task) = server("200 OK").await;
    let client = build_client(
        Some(proxy.trim_end_matches("/probe")),
        None,
        Duration::from_secs(1),
    )
    .unwrap();
    let result = probe(&client, "http://remote.example.invalid/test").await;
    assert!(result.reachable);
    assert!(task
        .await
        .unwrap()
        .starts_with("GET http://remote.example.invalid/test"));
    let (target, task) = server("200 OK").await;
    let client = build_client(
        Some("http://127.0.0.1:1"),
        Some("127.0.0.1"),
        Duration::from_secs(1),
    )
    .unwrap();
    assert!(probe(&client, &target).await.reachable);
    task.await.unwrap();
}
#[tokio::test]
async fn connection_failure_is_sanitized_and_timeout_is_classified() {
    let client = build_client(None, None, Duration::from_millis(50)).unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}/", listener.local_addr().unwrap());
    let hold = tokio::spawn(async move {
        let (_s, _) = listener.accept().await.unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
    });
    let result = probe(&client, &endpoint).await;
    assert!(!result.reachable);
    assert_eq!(result.code, "timeout");
    assert!(!result.message.contains(&endpoint));
    hold.abort();
}
#[test]
fn refresh_partial_success_is_structural_not_error_text_matching() {
    let result = super::super::refresh_result(
        "fixture".into(),
        Ok(super::super::types::RefreshOutcome {
            subscription_error: Some("connection".into()),
        }),
    );
    assert!(result.ok && result.partial);
    assert!(result.message.contains("额度已更新"));
    let result = super::super::refresh_result("fixture".into(), Err("quota failed".into()));
    assert!(!result.ok && !result.partial);
    let result = super::super::refresh_result(
        "fixture".into(),
        Ok(super::super::types::RefreshOutcome {
            subscription_error: None,
        }),
    );
    assert!(result.ok && !result.partial);
}
