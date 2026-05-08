use axum::{
    body::Body,
    extract::{Path, Request, State},
    http::StatusCode,
    response::Response,
};
use std::sync::Arc;

use crate::AppState;

/// Proxy requests to internal services
pub async fn proxy_handler(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request,
) -> Response {
    let target_url = if path.starts_with("auth/") {
        format!("{}/{}", state.auth_service_url, &path)
    } else if path.starts_with("user/") {
        format!("{}/{}", state.user_service_url, &path)
    } else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(format!("Unknown service path: {}", path)))
            .unwrap();
    };

    // Extract headers before consuming body
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let client = reqwest::Client::new();
    let method = req.method().clone();
    let body_bytes = axum::body::to_bytes(req.into_body(), 10 * 1024 * 1024)
        .await
        .unwrap_or_default();

    let mut proxy_req = client
        .request(method, &target_url)
        .body(body_bytes)
        .header("Content-Type", "application/json");

    // Forward Authorization header
    if let Some(auth) = auth_header {
        proxy_req = proxy_req.header("Authorization", auth);
    }

    match proxy_req.send().await {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.bytes().await.unwrap_or_default();
            Response::builder()
                .status(status)
                .body(Body::from(body))
                .unwrap()
        }
        Err(e) => Response::builder()
            .status(StatusCode::BAD_GATEWAY)
            .body(Body::from(format!("Proxy error: {}", e)))
            .unwrap(),
    }
}
