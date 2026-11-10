use super::*;
const FIRST: &str = "00000000-0000-4000-8000-000000000001";
const SECOND: &str = "00000000-0000-4000-8000-000000000002";

fn profile(id: &str, base: &str, priority: u16) -> Profile {
    Profile {
        id: id.into(),
        kind: ProfileKind::ApiKey,
        name: format!("fixture-{id}"),
        email: None,
        enabled: true,
        priority,
        base_url: base.into(),
        auth_status: "ready".into(),
        models: vec![GatewayModel {
            id: "fixture-model".into(),
            name: "fixture-model".into(),
        }],
        cooldown_until: None,
        last_error: None,
        requests: 0,
        input_tokens: 0,
        output_tokens: 0,
        created_at: 0,
    }
}

#[tokio::test]
async fn routing_records_actual_failover_and_stream_failure_without_replay() {
    let root = std::env::temp_dir().join(format!("atelier-routing-test-{}", Uuid::new_v4()));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let mock = Router::new()
        .route(
            "/first/responses",
            post(|| async {
                (
                    StatusCode::TOO_MANY_REQUESTS,
                    Json(json!({"error":{"code":"rate_limited"}})),
                )
            }),
        )
        .route(
            "/second/responses",
            post(|Json(v): Json<Value>| async move {
                if v["stream"] == true {
                    (
                        [("content-type", "text/event-stream")],
                        "event: response.incomplete\ndata: {\"type\":\"response.incomplete\"}\n\n",
                    )
                        .into_response()
                } else {
                    Json(json!({"id":"fixture-response","status":"completed","output":[]}))
                        .into_response()
                }
            }),
        );
    let upstream = tokio::spawn(async move { axum::serve(listener, mock).await.unwrap() });
    let store = Arc::new(Store::new(root.join("gateway")).unwrap());
    for p in [
        profile(FIRST, &format!("http://{addr}/first"), 100),
        profile(SECOND, &format!("http://{addr}/second"), 0),
    ] {
        store
            .upsert(
                p,
                Credential {
                    api_key: Some("private-fixture-key".into()),
                    ..Default::default()
                },
                None,
            )
            .unwrap();
    }
    store
        .update(None, |c| {
            c.settings.port = 0;
            c.settings.model = "fixture-model".into();
            c.settings.preferred_profile_id = Some(FIRST.into());
            c.settings.auto_switch = true;
            Ok(())
        })
        .unwrap();
    let client = reqwest::Client::builder().no_proxy().build().unwrap();
    let auth = Arc::new(AuthManager::new(store.clone(), client.clone()));
    let server = Arc::new(GatewayServer::new(store.clone(), auth, client.clone()));
    server.start().await.unwrap();
    let key = server.local_key().unwrap();
    let url = format!("{}/responses", server.view().address.unwrap());
    let completed = client
        .post(&url)
        .bearer_auth(&key)
        .json(&json!({"model":"fixture-model", "input":"private-fixture-prompt"}))
        .send()
        .await
        .unwrap();
    assert_eq!(
        completed.json::<Value>().await.unwrap()["status"],
        "completed"
    );
    let snapshot = server.routing_snapshot();
    assert_eq!(
        snapshot.routes[0]
            .attempts
            .iter()
            .map(|a| a.profile_id.as_str())
            .collect::<Vec<_>>(),
        [FIRST, SECOND]
    );
    assert_eq!(snapshot.routes[0].candidates[0].reason, "当前首选账号");
    assert_eq!(snapshot.routes[0].attempts[0].http_status, Some(429));
    assert_eq!(snapshot.routes[0].attempts[0].code, "rate_limited");
    assert_eq!(snapshot.routes[0].status, "completed");
    assert_eq!(snapshot.rerouted_requests, 1);
    let wire = serde_json::to_string(&snapshot).unwrap();
    assert!(!wire.contains("private-fixture"));
    store
        .update(None, |c| {
            c.settings.preferred_profile_id = Some(SECOND.into());
            Ok(())
        })
        .unwrap();
    let stream = client
        .post(&url)
        .bearer_auth(&key)
        .json(&json!({"model":"fixture-model","input":[],"stream":true}))
        .send()
        .await
        .unwrap();
    assert_eq!(stream.status(), StatusCode::OK);
    assert!(stream.text().await.unwrap().contains("response.incomplete"));
    let snapshot = server.routing_snapshot();
    assert_eq!(snapshot.routes[0].status, "failed");
    assert_eq!(snapshot.routes[0].code, "response.incomplete");
    assert_eq!(snapshot.routes[0].attempts.len(), 1);
    assert_eq!(snapshot.runtime.completed_requests, 1);
    assert_eq!(snapshot.runtime.failed_requests, 1);
    let blocked = client
        .post(&url)
        .bearer_auth(&key)
        .json(&json!({"model":"fixture-model","previous_response_id":"unknown"}))
        .send()
        .await
        .unwrap();
    assert_eq!(blocked.status(), StatusCode::CONFLICT);
    let snapshot = server.routing_snapshot();
    assert!(snapshot.routes[0].attempts.is_empty());
    assert_eq!(snapshot.routes[0].code, "unknown_response_owner");
    server.stop().await.unwrap();
    upstream.abort();
    std::fs::remove_dir_all(root).unwrap();
}

#[tokio::test]
async fn routing_keeps_inflight_request_when_finished_history_is_pruned_and_marks_drop() {
    let root = std::env::temp_dir().join(format!("atelier-routing-retention-{}", Uuid::new_v4()));
    let store = Arc::new(Store::new(root.join("gateway")).unwrap());
    let client = reqwest::Client::new();
    let auth = Arc::new(AuthManager::new(store.clone(), client.clone()));
    let server = Arc::new(GatewayServer::new(store, auth, client));
    let pending = RequestGuard::new(
        server.clone(),
        server.slots.clone().acquire_owned().await.unwrap(),
        "responses",
    );
    let pending_id = pending.route_id.clone();
    for _ in 0..110 {
        let mut guard = RequestGuard::new(
            server.clone(),
            server.slots.clone().acquire_owned().await.unwrap(),
            "responses",
        );
        guard.finish(true, "response.completed", None);
    }
    let snapshot = server.routing_snapshot();
    assert_eq!(snapshot.routes.len(), 101);
    assert!(snapshot
        .routes
        .iter()
        .any(|r| r.id == pending_id && r.duration_ms.is_none()));
    assert_eq!(snapshot.runtime.active_requests, 1);
    drop(pending);
    let snapshot = server.routing_snapshot();
    assert_eq!(snapshot.routes.len(), 100);
    let cancelled = snapshot.routes.iter().find(|r| r.id == pending_id).unwrap();
    assert_eq!(cancelled.code, "stream_interrupted");
    assert_eq!(snapshot.runtime.active_requests, 0);
    std::fs::remove_dir_all(root).unwrap();
}
