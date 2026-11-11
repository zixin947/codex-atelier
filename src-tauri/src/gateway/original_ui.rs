//! Transports the unchanged upstream page through an authenticated native scheme.
use super::GatewayState;
use tauri::{
    http::{Request, Response},
    Manager,
};

pub fn protocol(
    app: tauri::AppHandle,
    request: Request<Vec<u8>>,
    responder: tauri::UriSchemeResponder,
) {
    tauri::async_runtime::spawn(async move {
        let result = async {
            let gateway = app.state::<GatewayState>().get(&app)?;
            let address = gateway.engine.management_url()?;
            forward(&gateway.ui_client, &address, request).await
        }
        .await;
        responder.respond(match result {
            Ok(response) => response,
            Err(message) => Response::builder()
                .status(503)
                .header("Content-Type", "text/plain; charset=utf-8")
                .body(message.into_bytes())
                .unwrap(),
        });
    });
}

async fn forward(
    client: &reqwest::Client,
    address: &str,
    request: Request<Vec<u8>>,
) -> Result<Response<Vec<u8>>, String> {
    let native_origin = match (
        request.uri().scheme_str(),
        request.uri().authority().map(|value| value.as_str()),
    ) {
        (Some("magpie"), Some("localhost")) => "magpie://localhost",
        (Some("http"), Some("magpie.localhost")) => "http://magpie.localhost",
        _ => "",
    };
    let api = request.uri().path().starts_with("/api/");
    // Sandboxed plugin frames have an opaque Origin ("null"). Check it
    // before attaching the management cookie or translating our own origin.
    if native_origin.is_empty()
        || api
            && (request
                .headers()
                .get("origin")
                .is_some_and(|value| value != native_origin)
                || request
                    .headers()
                    .get("sec-fetch-site")
                    .is_some_and(|value| value != "same-origin" && value != "none")
                || request
                    .headers()
                    .get("sec-fetch-dest")
                    .is_some_and(|value| {
                        matches!(value.to_str(), Ok("iframe" | "frame" | "embed" | "object"))
                    }))
    {
        return Response::builder()
            .status(403)
            .body(b"Management API is available to the native page only".to_vec())
            .map_err(|_| "原版管理页响应无效。".into());
    }
    let plugin_asset = request.uri().path().starts_with("/_magpie/plugin/");
    let opaque_plugin_asset = plugin_asset
        && request
            .headers()
            .get("origin")
            .is_some_and(|value| value == "null");
    let styled = request.uri().path() == "/"
        && url::form_urlencoded::parse(request.uri().query().unwrap_or("").as_bytes())
            .any(|(key, value)| key == "atelierStyle" && value == "endfield");
    let content_only = request.uri().path() == "/"
        && url::form_urlencoded::parse(request.uri().query().unwrap_or("").as_bytes())
            .any(|(key, value)| key == "atelierContent" && value == "1");
    let decorated = styled || content_only;
    let mut target = url::Url::parse(address).map_err(|_| "原版管理页地址无效。")?;
    if target.scheme() != "http"
        || target.host_str() != Some("127.0.0.1")
        || target.port().is_none()
    {
        return Err("原版管理页必须由本机引擎提供。".into());
    }
    let key = target
        .query_pairs()
        .find(|(name, _)| name == "k")
        .map(|(_, value)| value.into_owned())
        .ok_or("管理页认证材料缺失。")?;
    let cookie = format!("magpie_web_{}={key}", target.port().unwrap());
    let origin = target.origin().ascii_serialization();
    let secure_origin = format!("https://127.0.0.1:{}", target.port().unwrap());
    target.set_path(request.uri().path());
    target.set_query(request.uri().query());
    let mut outgoing = client.request(request.method().clone(), target);
    if !plugin_asset {
        outgoing = outgoing.header("Cookie", cookie);
    }
    for name in [
        "content-type",
        "accept",
        "if-none-match",
        "if-modified-since",
        "range",
        "origin",
        "sec-fetch-site",
        "sec-fetch-dest",
    ] {
        if decorated && matches!(name, "if-none-match" | "if-modified-since") {
            continue;
        }
        if let Some(value) = request.headers().get(name) {
            if name == "origin" && value == native_origin {
                outgoing = outgoing.header(name, &origin);
            } else {
                outgoing = outgoing.header(name, value);
            }
        }
    }
    let upstream = outgoing
        .body(request.into_body())
        .send()
        .await
        .map_err(|_| "原版管理页连接失败，请重新打开网关。")?;
    let mut response = Response::builder().status(upstream.status());
    let html = upstream
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.starts_with("text/html"));
    for name in [
        "content-type",
        "cache-control",
        "etag",
        "last-modified",
        "content-range",
        "accept-ranges",
        "content-security-policy",
        "x-content-type-options",
        "referrer-policy",
    ] {
        if decorated && matches!(name, "etag" | "last-modified" | "cache-control") {
            continue;
        }
        for value in upstream.headers().get_all(name) {
            if name == "content-security-policy" {
                // Keep upstream's restrictions, mapping only its own resource
                // URLs to the native transport used to load this document.
                let policy = value.to_str().map_err(|_| "插件安全策略无效。")?;
                response = response.header(
                    name,
                    policy
                        .replace(&origin, native_origin)
                        .replace(&secure_origin, native_origin),
                );
            } else {
                response = response.header(name, value);
            }
        }
    }
    let mut bytes = upstream
        .bytes()
        .await
        .map_err(|_| "原版管理页响应中断。")?
        .to_vec();
    if decorated && html {
        let page = String::from_utf8(bytes).map_err(|_| "原版页面不是 UTF-8。")?;
        let mut css = String::new();
        if styled {
            css.push_str(include_str!(
                "../../../src/features/connections/native-theme.css"
            ));
        }
        if content_only {
            css.push_str(include_str!(
                "../../../src/features/connections/native-content.css"
            ));
        }
        bytes = page
            .replacen(
                "</head>",
                &format!("<style id=\"atelier-skin\">{css}</style></head>"),
                1,
            )
            .into_bytes();
        response = response.header("Cache-Control", "no-store");
    }
    if opaque_plugin_asset {
        response = response.header("Access-Control-Allow-Origin", "null");
    }
    response
        .body(bytes)
        .map_err(|_| "原版管理页响应无效。".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        http::HeaderMap,
        routing::{get, post},
        Router,
    };
    #[tokio::test]
    async fn native_transport_authenticates_assets_and_api_without_browser_cookies() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let app = Router::new()
            .route(
                "/",
                get(|| async {
                    axum::response::Html(
                        "<html><head></head><body><main>original-layout</main></body></html>",
                    )
                }),
            )
            .route("/app.js", get(|| async { "unchanged-script" }))
            .route(
                "/api/agents/test",
                post(move |headers: HeaderMap, body: String| async move {
                    assert_eq!(headers["cookie"], format!("magpie_web_{port}=fixture"));
                    assert_eq!(headers["origin"], format!("http://127.0.0.1:{port}"));
                    body
                }),
            );
        let task = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let client = reqwest::Client::builder().no_proxy().build().unwrap();
        let address = format!("http://127.0.0.1:{port}/?k=fixture");
        let response = forward(
            &client,
            &address,
            Request::builder()
                .uri("magpie://localhost/app.js")
                .body(vec![])
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.body(), b"unchanged-script");
        let response = forward(
            &client,
            &address,
            Request::builder()
                .method("POST")
                .uri("magpie://localhost/api/agents/test")
                .header("content-type", "application/json")
                .header("origin", "magpie://localhost")
                .body(b"{\"fixture\":true}".to_vec())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.body(), b"{\"fixture\":true}");
        assert!(!response
            .headers()
            .contains_key("access-control-allow-origin"));
        let response = forward(
            &client,
            &address,
            Request::builder()
                .uri("magpie://localhost/?view=groups&atelierStyle=endfield")
                .body(vec![])
                .unwrap(),
        )
        .await
        .unwrap();
        let html = String::from_utf8(response.body().clone()).unwrap();
        assert!(html.contains("<style id=\"atelier-skin\">"));
        assert!(html.contains("<body><main>original-layout</main></body>"));
        assert_eq!(response.headers()["cache-control"], "no-store");
        task.abort();
    }

    #[tokio::test]
    async fn management_rejects_untrusted_requests_before_contacting_upstream() {
        let client = reqwest::Client::builder().no_proxy().build().unwrap();
        for (uri, header, value) in [
            ("magpie://localhost/api/settings", "origin", "null"),
            (
                "magpie://localhost/api/settings",
                "origin",
                "https://evil.invalid",
            ),
            (
                "magpie://localhost/api/settings",
                "sec-fetch-site",
                "cross-site",
            ),
            (
                "magpie://localhost/api/settings",
                "sec-fetch-site",
                "same-site",
            ),
            (
                "magpie://localhost/api/settings",
                "sec-fetch-dest",
                "iframe",
            ),
            (
                "magpie://evil.invalid/api/settings",
                "origin",
                "magpie://evil.invalid",
            ),
        ] {
            // A dead upstream makes any accidental forwarding fail the test.
            let response = forward(
                &client,
                "http://127.0.0.1:1/?k=fixture",
                Request::builder()
                    .method("POST")
                    .uri(uri)
                    .header(header, value)
                    .body(vec![])
                    .unwrap(),
            )
            .await
            .unwrap();
            assert_eq!(response.status(), 403);
            assert!(!response
                .headers()
                .contains_key("access-control-allow-origin"));
        }
    }

    #[tokio::test]
    async fn plugin_assets_keep_security_headers_and_their_own_resource_scope() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let path = "/_magpie/plugin/fixture/index.html";
        let app = Router::new().route(
            path,
            get(move |headers: HeaderMap| async move {
                assert_eq!(headers["origin"], "null");
                assert_eq!(headers["sec-fetch-dest"], "iframe");
                assert!(!headers.contains_key("cookie"));
                let mut headers = HeaderMap::new();
                headers.append("content-security-policy", format!("default-src 'none'; script-src http://127.0.0.1:{port}/_magpie/plugin/fixture/ https://127.0.0.1:{port}/_magpie/plugin/fixture/; connect-src 'none'; frame-src 'none'").parse().unwrap());
                headers.append("content-security-policy", "sandbox allow-scripts; frame-ancestors 'self'".parse().unwrap());
                headers.insert("x-content-type-options", "nosniff".parse().unwrap());
                headers.insert("referrer-policy", "no-referrer".parse().unwrap());
                (headers, axum::response::Html("<html>plugin fixture</html>"))
            }),
        );
        let task = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let client = reqwest::Client::builder().no_proxy().build().unwrap();
        for origin in ["magpie://localhost", "http://magpie.localhost"] {
            let response = forward(
                &client,
                &format!("http://127.0.0.1:{port}/?k=fixture"),
                Request::builder()
                    .uri(format!("{origin}{path}"))
                    .header("origin", "null")
                    .header("sec-fetch-dest", "iframe")
                    .body(vec![])
                    .unwrap(),
            )
            .await
            .unwrap();
            assert_eq!(response.status(), 200);
            assert_eq!(response.body(), b"<html>plugin fixture</html>");
            assert_eq!(response.headers()["x-content-type-options"], "nosniff");
            assert_eq!(response.headers()["referrer-policy"], "no-referrer");
            assert_eq!(response.headers()["access-control-allow-origin"], "null");
            let policies: Vec<_> = response
                .headers()
                .get_all("content-security-policy")
                .iter()
                .map(|value| value.to_str().unwrap())
                .collect();
            assert_eq!(policies.len(), 2);
            let policy = policies.join("; ");
            assert!(policy.contains("sandbox allow-scripts"));
            assert!(policy.contains("connect-src 'none'"));
            assert!(policy.contains("frame-src 'none'"));
            assert!(policy.contains(&format!("{origin}/_magpie/plugin/fixture/")));
            assert!(!policy.contains("127.0.0.1"));
        }
        task.abort();
    }
}
