use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::Json;
use habo_core::error::AppError;
use habo_core::types::{ApiResponse, CreateWorkoutRequest, UpdateWorkoutRequest, WorkoutListItem, WorkoutResponse};
use serde::Deserialize;
use sqlx::PgPool;

use crate::db;

fn extract_user_id(headers: &HeaderMap) -> Result<uuid::Uuid, AppError> {
    headers
        .get("X-Habo-User-Id")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<uuid::Uuid>().ok())
        .ok_or(AppError::Unauthorized("Missing user identification".into()))
}

#[derive(Deserialize)]
pub struct ListParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn create_workout_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(req): Json<CreateWorkoutRequest>,
) -> Result<Json<ApiResponse<WorkoutResponse>>, AppError> {
    let user_id = extract_user_id(&headers)?;
    let workout = db::create_workout(&pool, &user_id, &req).await?;
    Ok(Json(ApiResponse::ok(workout)))
}

pub async fn get_workout_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Path(workout_id): Path<uuid::Uuid>,
) -> Result<Json<ApiResponse<WorkoutResponse>>, AppError> {
    let user_id = extract_user_id(&headers)?;
    let workout = db::get_workout(&pool, &user_id, &workout_id).await?;
    Ok(Json(ApiResponse::ok(workout)))
}

pub async fn list_workouts_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<Vec<WorkoutListItem>>>, AppError> {
    let user_id = extract_user_id(&headers)?;
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);
    let workouts = db::list_workouts(&pool, &user_id, limit, offset).await?;
    Ok(Json(ApiResponse::ok(workouts)))
}

pub async fn update_workout_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Path(workout_id): Path<uuid::Uuid>,
    Json(req): Json<UpdateWorkoutRequest>,
) -> Result<Json<ApiResponse<WorkoutResponse>>, AppError> {
    let user_id = extract_user_id(&headers)?;
    let workout = db::update_workout(&pool, &user_id, &workout_id, &req).await?;
    Ok(Json(ApiResponse::ok(workout)))
}
