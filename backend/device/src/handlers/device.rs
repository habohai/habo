use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::Json;
use habo_core::error::AppError;
use habo_core::types::{ApiResponse, BindDeviceRequest, DeviceBindingResponse};
use sqlx::PgPool;

use crate::db;

fn extract_user_id(headers: &HeaderMap) -> Result<uuid::Uuid, AppError> {
    headers
        .get("X-Habo-User-Id")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<uuid::Uuid>().ok())
        .ok_or(AppError::Unauthorized("Missing user identification".into()))
}

pub async fn bind_device_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(req): Json<BindDeviceRequest>,
) -> Result<Json<ApiResponse<DeviceBindingResponse>>, AppError> {
    let user_id = extract_user_id(&headers)?;
    let config = req.config_json.unwrap_or(serde_json::Value::Object(Default::default()));
    let device = db::bind_device(&pool, &user_id, &req.device_type, &req.device_name, &config).await?;
    Ok(Json(ApiResponse::ok(device)))
}

pub async fn list_devices_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<Vec<DeviceBindingResponse>>>, AppError> {
    let user_id = extract_user_id(&headers)?;
    let devices = db::list_devices(&pool, &user_id).await?;
    Ok(Json(ApiResponse::ok(devices)))
}

pub async fn unbind_device_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Path(device_id): Path<uuid::Uuid>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    let user_id = extract_user_id(&headers)?;
    let deleted = db::unbind_device(&pool, &user_id, &device_id).await?;
    if deleted {
        Ok(Json(ApiResponse::ok("Device unbound".to_string())))
    } else {
        Err(AppError::NotFound("Device binding not found".into()))
    }
}
