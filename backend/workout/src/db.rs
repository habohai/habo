use habo_core::error::AppError;
use habo_core::types::{CreateWorkoutRequest, UpdateWorkoutRequest, WorkoutListItem, WorkoutResponse};
use sqlx::PgPool;

pub async fn create_workout(
    pool: &PgPool,
    user_id: &uuid::Uuid,
    req: &CreateWorkoutRequest,
) -> Result<WorkoutResponse, AppError> {
    let sport_type = req.sport_type.as_deref().unwrap_or("running");
    let default_arr = serde_json::Value::Array(vec![]);
    let default_obj = serde_json::Value::Object(Default::default());
    let route = req.route_data.as_ref().unwrap_or(&default_arr);
    let splits = req.splits_data.as_ref().unwrap_or(&default_arr);
    let sensor = req.sensor_data.as_ref().unwrap_or(&default_obj);
    let notes = req.notes.as_deref().unwrap_or("");

    let row = sqlx::query_as::<_, WorkoutResponse>(
        r#"
        INSERT INTO workouts (user_id, device_id, sport_type, started_at, ended_at,
            duration_secs, distance_meters, avg_heart_rate, max_heart_rate,
            avg_pace_km, calories_kcal, elevation_gain_m,
            route_data, splits_data, sensor_data, notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
        RETURNING id, user_id, device_id, sport_type, status, started_at, ended_at,
            duration_secs, distance_meters, avg_heart_rate, max_heart_rate,
            avg_pace_km, calories_kcal, elevation_gain_m,
            route_data, splits_data, sensor_data, source, notes, created_at
        "#,
    )
    .bind(user_id)
    .bind(req.device_id)
    .bind(sport_type)
    .bind(req.started_at)
    .bind(req.ended_at)
    .bind(req.duration_secs)
    .bind(req.distance_meters)
    .bind(req.avg_heart_rate)
    .bind(req.max_heart_rate)
    .bind(req.avg_pace_km)
    .bind(req.calories_kcal)
    .bind(req.elevation_gain_m)
    .bind(route)
    .bind(splits)
    .bind(sensor)
    .bind(notes)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn get_workout(
    pool: &PgPool,
    user_id: &uuid::Uuid,
    workout_id: &uuid::Uuid,
) -> Result<WorkoutResponse, AppError> {
    let row = sqlx::query_as::<_, WorkoutResponse>(
        r#"
        SELECT id, user_id, device_id, sport_type, status, started_at, ended_at,
            duration_secs, distance_meters, avg_heart_rate, max_heart_rate,
            avg_pace_km, calories_kcal, elevation_gain_m,
            route_data, splits_data, sensor_data, source, notes, created_at
        FROM workouts
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(workout_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn list_workouts(
    pool: &PgPool,
    user_id: &uuid::Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<WorkoutListItem>, AppError> {
    let rows = sqlx::query_as::<_, WorkoutListItem>(
        r#"
        SELECT id, user_id, sport_type, status, started_at, ended_at,
            duration_secs, distance_meters, avg_heart_rate, max_heart_rate,
            avg_pace_km, calories_kcal, created_at
        FROM workouts
        WHERE user_id = $1
        ORDER BY started_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn update_workout(
    pool: &PgPool,
    user_id: &uuid::Uuid,
    workout_id: &uuid::Uuid,
    req: &UpdateWorkoutRequest,
) -> Result<WorkoutResponse, AppError> {
    sqlx::query(
        r#"
        UPDATE workouts SET
            ended_at = COALESCE($3, ended_at),
            duration_secs = COALESCE($4, duration_secs),
            distance_meters = COALESCE($5, distance_meters),
            avg_heart_rate = COALESCE($6, avg_heart_rate),
            max_heart_rate = COALESCE($7, max_heart_rate),
            avg_pace_km = COALESCE($8, avg_pace_km),
            calories_kcal = COALESCE($9, calories_kcal),
            elevation_gain_m = COALESCE($10, elevation_gain_m),
            route_data = COALESCE($11, route_data),
            splits_data = COALESCE($12, splits_data),
            notes = COALESCE($13, notes),
            updated_at = now()
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(workout_id)
    .bind(user_id)
    .bind(req.ended_at)
    .bind(req.duration_secs)
    .bind(req.distance_meters)
    .bind(req.avg_heart_rate)
    .bind(req.max_heart_rate)
    .bind(req.avg_pace_km)
    .bind(req.calories_kcal)
    .bind(req.elevation_gain_m)
    .bind(req.route_data.as_ref())
    .bind(req.splits_data.as_ref())
    .bind(req.notes.as_deref())
    .execute(pool)
    .await?;

    get_workout(pool, user_id, workout_id).await
}
