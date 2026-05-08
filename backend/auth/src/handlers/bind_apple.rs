use axum::extract::State;
use axum::Json;
use habo_core::error::AppError;
use habo_core::types::{ApiResponse, BindAppleRequest};
use std::sync::Arc;

use crate::db;
use crate::AppState;

pub async fn bind_apple_handler(
    State(state): State<Arc<AppState>>,
    axum::Extension(user_id): axum::Extension<uuid::Uuid>,
    Json(req): Json<BindAppleRequest>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    // TODO: Verify Apple identity token with Apple's API in production
    db::bind_apple_id(&state.pool, &user_id, &req.apple_id)
        .await
        .map_err(AppError::from)?;

    Ok(Json(ApiResponse::ok(
        "Apple ID bound successfully".to_string(),
    )))
}
