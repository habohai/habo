use habo_core::error::AppError;
use habo_core::types::UserProfileResponse;
use sqlx::PgPool;

pub async fn get_profile(
    pool: &PgPool,
    user_id: &uuid::Uuid,
) -> Result<UserProfileResponse, AppError> {
    let row = sqlx::query_as::<_, UserProfileResponse>(
        r#"
        SELECT
            u.id as user_id,
            u.phone,
            u.nickname,
            u.avatar_url,
            COALESCE(p.gender, 0::smallint) as gender,
            p.height_cm,
            p.weight_kg,
            COALESCE(p.running_goal, '') as running_goal,
            p.weekly_goal_km,
            u.created_at
        FROM users u
        LEFT JOIN user_profiles p ON p.user_id = u.id
        WHERE u.id = $1
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn upsert_profile(
    pool: &PgPool,
    user_id: &uuid::Uuid,
    nickname: Option<&str>,
    gender: Option<i16>,
    height_cm: Option<i32>,
    weight_kg: Option<rust_decimal::Decimal>,
    running_goal: Option<&str>,
    weekly_goal_km: Option<rust_decimal::Decimal>,
) -> Result<(), AppError> {
    // Update users table nickname
    if let Some(n) = nickname {
        sqlx::query("UPDATE users SET nickname = $1, updated_at = now() WHERE id = $2")
            .bind(n)
            .bind(user_id)
            .execute(pool)
            .await?;
    }

    // Upsert user_profiles
    sqlx::query(
        r#"
        INSERT INTO user_profiles (user_id, gender, height_cm, weight_kg, running_goal, weekly_goal_km)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (user_id) DO UPDATE SET
            gender = COALESCE($2, user_profiles.gender),
            height_cm = COALESCE($3, user_profiles.height_cm),
            weight_kg = COALESCE($4, user_profiles.weight_kg),
            running_goal = COALESCE($5, user_profiles.running_goal),
            weekly_goal_km = COALESCE($6, user_profiles.weekly_goal_km)
        "#,
    )
    .bind(user_id)
    .bind(gender)
    .bind(height_cm)
    .bind(weight_kg)
    .bind(running_goal)
    .bind(weekly_goal_km)
    .execute(pool)
    .await?;

    Ok(())
}
