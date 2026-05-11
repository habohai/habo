use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    response::Response,
};
use std::sync::Arc;

use crate::AppState;

/// Proxy any request to the appropriate backend service
/// Forwards `X-Habo-User-Id` header if user_id extension is present
pub async fn proxy_handler(
    State(state): State<Arc<AppState>>,
    req: Request,
) -> Response {
    let path = req.uri().path();

    let target_url = if path.starts_with("/auth/") || path == "/auth" {
        format!("{}{}", state.auth_service_url, path)
    } else if path.starts_with("/user/") || path == "/user" {
        format!("{}{}", state.user_service_url, path)
    } else if path.starts_with("/device/") || path == "/device" {
        format!("{}{}", state.device_service_url, path)
    } else if path.starts_with("/workout/") || path == "/workout" {
        format!("{}{}", state.workout_service_url, path)
    } else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(format!("Unknown service path: {}", path)))
            .unwrap();
    };

    // Extract user_id from extension (set by JWT middleware) and Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let user_id = req
        .extensions()
        .get::<uuid::Uuid>()
        .map(|id| id.to_string());

    let client = reqwest::Client::new();
    let method = req.method().clone();
    let body_bytes = axum::body::to_bytes(req.into_body(), 10 * 1024 * 1024)
        .await
        .unwrap_or_default();

    let mut proxy_req = client
        .request(method, &target_url)
        .body(body_bytes)
        .header("Content-Type", "application/json");

    if let Some(auth) = auth_header {
        proxy_req = proxy_req.header("Authorization", auth);
    }
    if let Some(uid) = user_id {
        proxy_req = proxy_req.header("X-Habo-User-Id", uid);
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
