use axum::extract::State;
use axum::Json;
use habo_core::error::AppError;
use habo_core::types::{ApiResponse, UpdateProfileRequest, UserProfileResponse};
use sqlx::PgPool;
use axum::http::HeaderMap;

use crate::db;

/// Extract user ID from X-Habo-User-Id header (set by gateway proxy)
fn extract_user_id(headers: &HeaderMap) -> Result<uuid::Uuid, AppError> {
    headers
        .get("X-Habo-User-Id")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<uuid::Uuid>().ok())
        .ok_or(AppError::Unauthorized("Missing user identification".into()))
}

pub async fn get_my_profile(
    State(pool): State<PgPool>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<UserProfileResponse>>, AppError> {
    let user_id = extract_user_id(&headers)?;
    let profile = db::get_profile(&pool, &user_id).await?;
    Ok(Json(ApiResponse::ok(profile)))
}

pub async fn get_user_profile(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    axum::extract::Path(target_id): axum::extract::Path<uuid::Uuid>,
) -> Result<Json<ApiResponse<UserProfileResponse>>, AppError> {
    let profile = db::get_profile(&pool, &target_id).await?;
    Ok(Json(ApiResponse::ok(profile)))
}

pub async fn update_profile(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<Json<ApiResponse<UserProfileResponse>>, AppError> {
    let user_id = extract_user_id(&headers)?;
    db::upsert_profile(
        &pool,
        &user_id,
        req.nickname.as_deref(),
        req.gender,
        req.height_cm,
        req.weight_kg,
        req.running_goal.as_deref(),
        req.weekly_goal_km,
    )
    .await?;

    let profile = db::get_profile(&pool, &user_id).await?;
    Ok(Json(ApiResponse::ok(profile)))
}
