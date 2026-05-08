use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use habo_core::error::AppError;
use habo_core::jwt::verify_token;
use std::sync::Arc;

use crate::AppState;

fn extract_bearer_token(req: &Request) -> Option<String> {
    req.headers()
        .get("Authorization")?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
        .map(|s| s.to_string())
}

/// JWT auth middleware — validates token and injects user_id
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token =
        extract_bearer_token(&req).ok_or(AppError::Unauthorized("Missing Authorization header".into()))?;

    let claims = verify_token(&token, &state.jwt_secret)?;

    let user_id: uuid::Uuid = claims
        .sub
        .parse()
        .map_err(|_| AppError::Unauthorized("Invalid user id in token".into()))?;
    req.extensions_mut().insert(user_id);

    Ok(next.run(req).await)
}

/// Optional auth — attaches user if token present, does not reject
pub async fn optional_auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Response {
    if let Some(token) = extract_bearer_token(&req) {
        if let Ok(claims) = verify_token(&token, &state.jwt_secret) {
            if let Ok(user_id) = claims.sub.parse::<uuid::Uuid>() {
                req.extensions_mut().insert(user_id);
            }
        }
    }
    next.run(req).await
}
