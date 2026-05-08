use axum::extract::State;
use axum::Json;
use habo_core::error::AppError;
use habo_core::jwt::{create_access_token, create_refresh_token, verify_token};
use habo_core::types::{ApiResponse, RefreshTokenRequest, VerifyCodeResponse};
use std::sync::Arc;

use crate::AppState;

pub async fn refresh_token_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RefreshTokenRequest>,
) -> Result<Json<ApiResponse<VerifyCodeResponse>>, AppError> {
    let claims = verify_token(&req.refresh_token, &state.jwt_secret)?;

    let user_id: uuid::Uuid = claims
        .sub
        .parse()
        .map_err(|_| AppError::Unauthorized("Invalid user ID in token".into()))?;

    let access_token = create_access_token(&user_id, &claims.phone, &state.jwt_secret)?;
    let refresh_token = create_refresh_token(&user_id, &state.jwt_secret)?;

    Ok(Json(ApiResponse::ok(VerifyCodeResponse {
        access_token,
        refresh_token,
        user_id,
    })))
}
