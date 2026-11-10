//! Synthetic request headers and loopback HTTP only. No discovery, app-server or authfile.
use super::*;
use serde_json::json;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
fn scope() -> Scope {
    Scope {
        mode: SourceMode::Native,
        revision: "fixture-revision".into(),
    }
}
fn headers() -> HeaderMap {
    let mut h = HeaderMap::new();
    h.insert(
        AUTHORIZATION,
        HeaderValue::from_static("Bearer synthetic-not-a-real-token"),
    );
    h.insert(
        "chatgpt-account-id",
        HeaderValue::from_static("synthetic-account"),
    );
    h
}
async fn server(
    status: u16,
    extra: &str,
    body: Vec<u8>,
    delay: Duration,
) -> (String, tokio::task::JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let extra = extra.to_owned();
    let handle = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut data = Vec::new();
        loop {
            let mut block = [0; 1024];
            let n = stream.read(&mut block).await.unwrap();
            if n == 0 {
                break;
            }
            data.extend_from_slice(&block[..n]);
            if data.windows(4).any(|w| w == b"\r\n\r\n") {
                break;
            }
        }
        tokio::time::sleep(delay).await;
        let response = format!(
            "HTTP/1.1 {status} Fixture\r\nContent-Length: {}\r\nConnection: close\r\n{extra}\r\n",
            body.len()
        );
        let _ = stream.write_all(response.as_bytes()).await;
        let _ = stream.write_all(&body).await;
        String::from_utf8(data).unwrap()
    });
    (format!("http://{address}/codex/models"), handle)
}
async fn observed(
    runtime: &OfficialCatalogRuntime,
    scope: &Scope,
    status: u16,
    body: &[u8],
) -> Observation {
    let (url, handle) = server(status, "", body.to_vec(), Duration::ZERO).await;
    let result = runtime
        .observe_at(scope, &headers(), "1.2.3", &url, FETCH_TIMEOUT)
        .await
        .unwrap();
    handle.await.unwrap();
    result
}
#[test]
fn native_modes_come_from_actual_document_and_unobserved_is_not_ready() {
    let mut json = RelayStateDocument::native_default().as_value().clone();
    let r = OfficialCatalogRuntime::default();
    for (on, no_account, expected) in [
        (false, false, SourceMode::NativeOff),
        (true, true, SourceMode::NativeNoAccount),
        (true, false, SourceMode::Native),
    ] {
        json["codexRouterEnabled"] = json!(on);
        json["codexRouterNoAccountMode"] = json!(no_account);
        let s = Scope::native(
            &RelayStateDocument::from_json(&serde_json::to_vec(&json).unwrap()).unwrap(),
        )
        .unwrap();
        assert_eq!(s.mode(), expected);
        let d = diagnose(&s, &r).unwrap();
        assert_eq!(d.item.status, "ok");
        assert!(!d.ready);
        assert!(!d.item.fixable);
        if expected == SourceMode::Native {
            assert!(d.item.detail.contains("不能据此确认"));
        }
    }
}
#[test]
fn typed_gateway_source_never_reads_raw_models_as_official() {
    let mut c = Catalog {
        revision: "r".into(),
        host_id: "h".into(),
        profiles: vec![],
        settings: Default::default(),
    };
    let p: crate::gateway::types::Profile = serde_json::from_value(json!({"id":"fake","kind":"apiKey","name":"fixture","enabled":true,"priority":0,"baseUrl":"https://fixture.invalid","authStatus":"warning","models":[{"id":"raw","name":"raw"}],"requests":0,"inputTokens":0,"outputTokens":0,"createdAt":0})).unwrap();
    c.profiles.push(p);
    assert_eq!(
        Scope::gateway(&c, "fake").unwrap().mode(),
        SourceMode::GatewayApiKey
    );
    assert!(Scope::gateway(&c, "missing").is_err());
    c.profiles[0].kind = ProfileKind::Chatgpt;
    let s = Scope::gateway(&c, "fake").unwrap();
    c.profiles[0].requests = 99;
    c.profiles[0].last_error = Some("runtime-only".into());
    assert_eq!(Scope::gateway(&c, "fake").unwrap(), s);
    c.settings.model = "actually-changed".into();
    assert_ne!(Scope::gateway(&c, "fake").unwrap(), s);
    assert_eq!(s.mode(), SourceMode::GatewayChatgpt);
    assert!(
        !diagnose(&s, &OfficialCatalogRuntime::default())
            .unwrap()
            .ready
    );
}
#[test]
fn header_filter_requires_real_bearer_and_copies_only_native_allowlist() {
    for value in [
        "",
        "Bearer ",
        "bearer fixture",
        "Basic fixture",
        "Bearer aimami-fixture",
        "Bearer sk-aimami-fixture",
    ] {
        let mut h = headers();
        h.insert(AUTHORIZATION, HeaderValue::from_str(value).unwrap());
        assert_eq!(
            official_headers(&h).unwrap_err(),
            CatalogFailure::MissingAuth
        );
    }
    let mut h = headers();
    for name in OFFICIAL_HEADERS {
        h.insert(name, HeaderValue::from_static("synthetic"));
    }
    h.insert(AUTHORIZATION, HeaderValue::from_static("Bearer fixture"));
    h.insert("cookie", HeaderValue::from_static("never-forward"));
    h.insert("x-api-key", HeaderValue::from_static("never-forward"));
    let out = official_headers(&h).unwrap();
    assert_eq!(out.len(), 8);
    assert!(!out.contains_key("cookie"));
    assert!(!out.contains_key("x-api-key"));
}
#[test]
fn validator_preserves_official_raw_fields_order_and_exact_slugs() {
    let input = json!({"other":{"native":true},"models":[{"slug":"a","hidden":true,"arbitrary":[1]},{"slug":"a"},{"slug":" a "},{"slug":""},{"slug":"\n "},{"slug":"aimami_relay_x"},{"slug":"p::x"},{"slug":1},null,{"slug":"b"}]});
    let (v, dropped) = validate_official_models(input).unwrap();
    assert_eq!(dropped, 7);
    assert_eq!(
        v["models"],
        json!([{"slug":"a","hidden":true,"arbitrary":[1]},{"slug":" a "},{"slug":"b"}])
    );
    assert_eq!(v["other"]["native"], true);
    assert!(validate_official_models(json!({"data":[{"id":"raw-gateway"}]})).is_err());
}
#[tokio::test]
async fn real_http_request_filters_headers_adds_version_and_caches_validated_response() {
    let runtime = OfficialCatalogRuntime::default();
    let s = scope();
    let (url, server) = server(200,"ETag: \"fixture-etag\"\r\n",br#"{"models":[{"slug":"official-a","object":"preserved"},{"slug":"official-a"},{"slug":"x::bad"}],"metadata":1}"#.to_vec(),Duration::ZERO).await;
    let mut h = headers();
    h.insert("cookie", HeaderValue::from_static("not-forwarded"));
    let o = runtime
        .observe_at(&s, &h, "1.2.3-beta+fixture", &url, FETCH_TIMEOUT)
        .await
        .unwrap();
    let request = server.await.unwrap();
    assert!(request.starts_with("GET /codex/models?client_version=1.2.3-beta%2Bfixture HTTP/1.1"));
    assert!(!request.contains("not-forwarded"));
    assert!(request.contains("synthetic-account"));
    assert_eq!(o.upstream_etag.unwrap(), "\"fixture-etag\"");
    assert!(o.published);
    assert_eq!(o.status.model_count(), 1);
    assert_eq!(o.status.dropped_entries, 2);
    assert!(o.status.available());
    assert!(diagnose(&s, &runtime).unwrap().ready);
    assert_eq!(runtime.cached_catalog(&s).unwrap().unwrap()["metadata"], 1);
    let wire = serde_json::to_value(o.status).unwrap();
    assert!(wire["checkedAt"].as_i64().unwrap() > 0);
    assert_eq!(wire.as_object().unwrap().len(), 5);
    assert!(wire["failure"].is_null());
}
#[tokio::test]
async fn missing_auth_is_observed_failure_without_any_http_and_no_secret_diagnostic() {
    let runtime = OfficialCatalogRuntime::default();
    let o = runtime
        .observe_at(
            &scope(),
            &HeaderMap::new(),
            "1.2.3",
            "http://127.0.0.1:1",
            FETCH_TIMEOUT,
        )
        .await
        .unwrap();
    assert_eq!(o.status.failure(), Some(&CatalogFailure::MissingAuth));
    assert!(o.catalog.is_none());
    let d = diagnose(&scope(), &runtime).unwrap();
    assert_eq!(d.item.status, "warning");
    assert!(!d.ready);
    assert!(d.item.detail.contains("MissingAuth"));
    assert!(!d.item.fixable);
}
#[tokio::test]
async fn invalid_queries_and_inactive_modes_do_not_create_an_observation() {
    let runtime = OfficialCatalogRuntime::default();
    for q in [
        "",
        "client_version=1.2.3&client_version=1.2.3",
        "client_version=not-version",
        "client_version=01.2.3",
    ] {
        assert_eq!(
            runtime
                .observe_query(&scope(), &headers(), q)
                .await
                .unwrap_err(),
            ObserveError::InvalidClientVersion
        );
    }
    assert_eq!(
        runtime
            .observe_at(
                &scope(),
                &headers(),
                &"1".repeat(81),
                "http://127.0.0.1:1",
                FETCH_TIMEOUT
            )
            .await
            .unwrap_err(),
        ObserveError::InvalidClientVersion
    );
    let mut off = scope();
    off.mode = SourceMode::NativeOff;
    assert_eq!(
        runtime
            .observe_request(&off, &headers(), "1.2.3")
            .await
            .unwrap_err(),
        ObserveError::NotApplicable
    );
    assert!(runtime.status(&scope()).unwrap().is_none());
}
#[tokio::test]
async fn failed_http_refresh_cannot_return_or_cache_previous_success_or_follow_redirect() {
    let r = OfficialCatalogRuntime::default();
    let s = scope();
    observed(&r, &s, 200, br#"{"models":[{"slug":"valid"}]}"#).await;
    let (url, handle) = server(
        302,
        "Location: http://127.0.0.1:1/must-not-follow\r\n",
        vec![],
        Duration::ZERO,
    )
    .await;
    let o = r
        .observe_at(&s, &headers(), "1.2.3", &url, FETCH_TIMEOUT)
        .await
        .unwrap();
    handle.await.unwrap();
    assert_eq!(
        o.status.failure(),
        Some(&CatalogFailure::Http { status: 302 })
    );
    assert!(o.catalog.is_none());
    assert!(r.cached_catalog(&s).unwrap().is_none());
    assert!(!diagnose(&s, &r).unwrap().ready);
    assert_eq!(o.response_status(), 502);
    assert_eq!(
        o.failure_body().unwrap()["error"]["code"],
        "official_catalog_unavailable"
    );
    assert_eq!(
        serde_json::to_value(o.status.failure.unwrap()).unwrap(),
        json!({"kind":"http","status":302})
    );
}
#[tokio::test]
async fn empty_all_invalid_wrong_schema_and_json_have_exact_failure_and_drop_counts() {
    let r = OfficialCatalogRuntime::default();
    let s = scope();
    for (body, drops) in [
        (b"not-json".as_slice(), 0),
        (br#"{"models":[]}"#, 0),
        (br#"{"models":[{"slug":"x::y"},null]}"#, 2),
        (br#"{"data":[{"id":"raw"}]}"#, 0),
    ] {
        let o = observed(&r, &s, 200, body).await;
        assert_eq!(o.status.failure(), Some(&CatalogFailure::InvalidCatalog));
        assert_eq!(o.status.dropped_entries, drops);
        assert_eq!(o.status.model_count(), 0);
    }
}
#[tokio::test]
async fn body_size_limit_checks_both_advertised_and_chunked_bytes() {
    let r = OfficialCatalogRuntime::default();
    let s = scope();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let handle = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut buf = [0; 4096];
        stream.read(&mut buf).await.unwrap();
        let _ = stream
            .write_all(
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
                    MAX_BYTES + 1
                )
                .as_bytes(),
            )
            .await;
    });
    let o = r
        .observe_at(
            &s,
            &headers(),
            "1.2.3",
            &format!("http://{address}"),
            FETCH_TIMEOUT,
        )
        .await
        .unwrap();
    handle.await.unwrap();
    assert_eq!(o.status.failure(), Some(&CatalogFailure::TooLarge));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let handle = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut buf = [0; 4096];
        stream.read(&mut buf).await.unwrap();
        let _ = stream
            .write_all(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n800001\r\n")
            .await;
        let _ = stream.write_all(&vec![b' '; MAX_BYTES + 1]).await;
        let _ = stream.write_all(b"\r\n0\r\n\r\n").await;
    });
    let o = r
        .observe_at(
            &s,
            &headers(),
            "1.2.3",
            &format!("http://{address}"),
            FETCH_TIMEOUT,
        )
        .await
        .unwrap();
    handle.await.unwrap();
    assert_eq!(o.status.failure(), Some(&CatalogFailure::TooLarge));
}
#[tokio::test]
async fn timeout_and_connection_failure_are_typed_actual_transport_failures() {
    let r = OfficialCatalogRuntime::default();
    let s = scope();
    let (url, handle) = server(
        200,
        "",
        br#"{"models":[{"slug":"late"}]}"#.to_vec(),
        Duration::from_millis(100),
    )
    .await;
    let o = r
        .observe_at(&s, &headers(), "1.2.3", &url, Duration::from_millis(10))
        .await
        .unwrap();
    assert_eq!(o.status.failure(), Some(&CatalogFailure::Timeout));
    handle.abort();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);
    let o = r
        .observe_at(
            &s,
            &headers(),
            "1.2.3",
            &format!("http://{address}"),
            FETCH_TIMEOUT,
        )
        .await
        .unwrap();
    assert_eq!(o.status.failure(), Some(&CatalogFailure::Network));
}
async fn wait_started(r: &OfficialCatalogRuntime, count: u64) {
    tokio::time::timeout(Duration::from_secs(1), async {
        loop {
            if r.inner.lock().unwrap().sequence >= count {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
}
#[tokio::test]
async fn configuration_publication_fences_inflight_result_and_newer_request_owns_status() {
    let r = std::sync::Arc::new(OfficialCatalogRuntime::default());
    let s = scope();
    let (url, h) = server(
        200,
        "",
        br#"{"models":[{"slug":"old"}]}"#.to_vec(),
        Duration::from_millis(250),
    )
    .await;
    let rr = r.clone();
    let ss = s.clone();
    let task = tokio::spawn(async move {
        rr.observe_at(&ss, &headers(), "1.2.3", &url, FETCH_TIMEOUT)
            .await
    });
    wait_started(&r, 1).await;
    observed(&r, &s, 200, br#"{"models":[{"slug":"new"}]}"#).await;
    assert!(!task.await.unwrap().unwrap().published);
    h.await.unwrap();
    assert_eq!(
        r.cached_catalog(&s).unwrap().unwrap()["models"][0]["slug"],
        "new"
    );
    let (url, h) = server(
        200,
        "",
        br#"{"models":[{"slug":"superseded"}]}"#.to_vec(),
        Duration::from_millis(250),
    )
    .await;
    let rr = r.clone();
    let ss = s.clone();
    let task = tokio::spawn(async move {
        rr.observe_at(&ss, &headers(), "1.2.3", &url, FETCH_TIMEOUT)
            .await
    });
    wait_started(&r, 3).await;
    let mut changed = s.clone();
    changed.revision = "changed-by-real-publication".into();
    r.configure(&changed).unwrap();
    assert_eq!(
        task.await.unwrap().unwrap_err(),
        ObserveError::ConfigurationChanged
    );
    h.await.unwrap();
    assert!(r.status(&changed).unwrap().is_none());
    assert_eq!(
        r.observe_at(&s, &headers(), "1.2.3", "http://127.0.0.1:1", FETCH_TIMEOUT)
            .await
            .unwrap_err(),
        ObserveError::ConfigurationChanged
    );
}

#[test]
fn native_failure_and_entry_http_status_mapping() {
    for (failure, status) in [
        (CatalogFailure::MissingAuth, 401),
        (CatalogFailure::Http { status: 403 }, 403),
        (CatalogFailure::Http { status: 599 }, 599),
        (CatalogFailure::Http { status: 302 }, 502),
        (CatalogFailure::Network, 502),
        (CatalogFailure::Timeout, 504),
        (CatalogFailure::InvalidCatalog, 502),
        (CatalogFailure::TooLarge, 502),
    ] {
        assert_eq!(failure.response_status(), status);
    }
    assert_eq!(ObserveError::InvalidClientVersion.response_status(), 400);
    assert_eq!(ObserveError::NotApplicable.response_status(), 409);
    assert_eq!(ObserveError::ConfigurationChanged.response_status(), 409);
}
