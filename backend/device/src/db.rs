use habo_core::error::AppError;
use habo_core::types::DeviceBindingResponse;
use sqlx::PgPool;

pub async fn bind_device(
    pool: &PgPool,
    user_id: &uuid::Uuid,
    device_type: &str,
    device_name: &str,
    config_json: &serde_json::Value,
) -> Result<DeviceBindingResponse, AppError> {
    let row = sqlx::query_as::<_, DeviceBindingResponse>(
        r#"
        INSERT INTO device_bindings (user_id, device_type, device_name, is_active, config_json)
        VALUES ($1, $2, $3, true, $4)
        RETURNING id, user_id, device_type, device_name, is_active, config_json, created_at
        "#,
    )
    .bind(user_id)
    .bind(device_type)
    .bind(device_name)
    .bind(config_json)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn list_devices(
    pool: &PgPool,
    user_id: &uuid::Uuid,
) -> Result<Vec<DeviceBindingResponse>, AppError> {
    let rows = sqlx::query_as::<_, DeviceBindingResponse>(
        r#"
        SELECT id, user_id, device_type, device_name, is_active, config_json, created_at
        FROM device_bindings
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn unbind_device(
    pool: &PgPool,
    user_id: &uuid::Uuid,
    device_id: &uuid::Uuid,
) -> Result<bool, AppError> {
    let result = sqlx::query(
        "DELETE FROM device_bindings WHERE id = $1 AND user_id = $2",
    )
    .bind(device_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
