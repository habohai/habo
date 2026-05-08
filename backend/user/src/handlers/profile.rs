use axum::extract::{Path, State};
use axum::Json;
use habo_core::error::AppError;
use habo_core::types::{ApiResponse, UpdateProfileRequest, UserProfileResponse};
use sqlx::PgPool;

use crate::db;

pub async fn get_my_profile(
    State(pool): State<PgPool>,
    axum::Extension(user_id): axum::Extension<uuid::Uuid>,
) -> Result<Json<ApiResponse<UserProfileResponse>>, AppError> {
    let profile = db::get_profile(&pool, &user_id).await?;
    Ok(Json(ApiResponse::ok(profile)))
}

pub async fn get_user_profile(
    State(pool): State<PgPool>,
    Path(target_id): Path<uuid::Uuid>,
) -> Result<Json<ApiResponse<UserProfileResponse>>, AppError> {
    let profile = db::get_profile(&pool, &target_id).await?;
    Ok(Json(ApiResponse::ok(profile)))
}

pub async fn update_profile(
    State(pool): State<PgPool>,
    axum::Extension(user_id): axum::Extension<uuid::Uuid>,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<Json<ApiResponse<UserProfileResponse>>, AppError> {
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
