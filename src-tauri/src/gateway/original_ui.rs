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
    target.set_path(request.uri().path());
    target.set_query(request.uri().query());
    let mut outgoing = client
        .request(request.method().clone(), target)
        .header("Cookie", cookie)
        .header("Origin", &origin)
        .header("Sec-Fetch-Site", "same-origin");
    for name in [
        "content-type",
        "accept",
        "if-none-match",
        "if-modified-since",
        "range",
    ] {
        if decorated && matches!(name, "if-none-match" | "if-modified-since") {
            continue;
        }
        if let Some(value) = request.headers().get(name) {
            outgoing = outgoing.header(name, value);
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
    ] {
        if decorated && matches!(name, "etag" | "last-modified" | "cache-control") {
            continue;
        }
        if let Some(value) = upstream.headers().get(name) {
            response = response.header(name, value);
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
    response
        .header("Access-Control-Allow-Origin", "*")
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
                .body(b"{\"fixture\":true}".to_vec())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.body(), b"{\"fixture\":true}");
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
}
