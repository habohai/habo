use axum::extract::State;
use axum::Json;
use habo_core::error::AppError;
use habo_core::jwt::{create_access_token, create_refresh_token};
use habo_core::types::{ApiResponse, VerifyCodeRequest, VerifyCodeResponse};
use redis::AsyncCommands;
use std::sync::Arc;

use crate::db;
use crate::AppState;

pub async fn verify_code_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<VerifyCodeRequest>,
) -> Result<Json<ApiResponse<VerifyCodeResponse>>, AppError> {
    let phone = req.phone.trim();
    let code = req.code.trim();

    // Check Redis cache first
    let redis_key = format!("sms:code:{}", phone);
    let mut conn = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| AppError::Internal(format!("Redis error: {}", e)))?;

    let cached_code: Option<String> = conn
        .get(redis_key)
        .await
        .map_err(|e| AppError::Internal(format!("Redis get error: {}", e)))?;

    let valid = match cached_code {
        Some(ref stored) if stored == code => true,
        _ => {
            // Fallback to DB check if Redis missed or expired
            db::verify_and_use_code(&state.pool, phone, code)
                .await
                .map_err(AppError::from)?
        }
    };

    if !valid {
        return Err(AppError::BadRequest("Invalid or expired code".into()));
    }

    // Delete code from Redis after successful verification
    let _: () = conn
        .del(format!("sms:code:{}", phone))
        .await
        .unwrap_or_default();

    // Create or get user
    let user_id = db::find_user_by_phone(&state.pool, phone)
        .await
        .map_err(AppError::from)?
        .unwrap_or_else(|| {
            // No existing user — create one
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(db::create_user(&state.pool, phone))
            })
            .unwrap()
        });

    // Generate JWT tokens
    let access_token = create_access_token(&user_id, phone, &state.jwt_secret)?;
    let refresh_token = create_refresh_token(&user_id, &state.jwt_secret)?;

    Ok(Json(ApiResponse::ok(VerifyCodeResponse {
        access_token,
        refresh_token,
        user_id,
    })))
}
