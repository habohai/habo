use axum::extract::State;
use axum::Json;
use habo_core::types::{ApiResponse, SendCodeRequest};
use rand::Rng;
use redis::AsyncCommands;
use std::sync::Arc;

use crate::db;
use crate::AppState;

pub async fn send_code_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SendCodeRequest>,
) -> Json<ApiResponse<String>> {
    let phone = req.phone.trim();
    if phone.len() < 5 {
        return Json(ApiResponse::err("Invalid phone number"));
    }

    // Generate 6-digit alphanumeric code
    let code: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(6)
        .map(|c| (c as char).to_ascii_uppercase())
        .collect();

    // Store in Redis with 5min TTL
    let redis_key = format!("sms:code:{}", phone);
    let mut conn = match state.redis.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(e) => return Json(ApiResponse::err(format!("Redis error: {}", e))),
    };

    if let Err(e) = conn.set_ex::<_, _, ()>(redis_key.clone(), &code, 300).await {
        return Json(ApiResponse::err(format!("Cache error: {}", e)));
    }

    // Log to PostgreSQL for audit trail
    let expired_at = chrono::Utc::now() + chrono::Duration::minutes(5);
    if let Err(e) = db::insert_verification_code(&state.pool, phone, &code, &expired_at).await {
        tracing::warn!("Failed to log verification code to DB: {}", e);
    }

    // In dev mode, print code to console instead of sending SMS
    if state.sms_provider == "console" {
        tracing::info!("[DEV] Verification code for {}: {}", phone, code);
    } else {
        tracing::info!("[DEV] Verification code for {}: {}", phone, code);
    }

    Json(ApiResponse::ok("Verification code sent".to_string()))
}
