use sqlx::PgPool;
use sqlx::Row;

pub async fn create_user(pool: &PgPool, phone: &str) -> Result<uuid::Uuid, sqlx::Error> {
    let row = sqlx::query(
        "INSERT INTO users (phone) VALUES ($1)
         ON CONFLICT (phone) DO UPDATE SET updated_at = now()
         RETURNING id",
    )
    .bind(phone)
    .fetch_one(pool)
    .await?;

    Ok(row.get("id"))
}

pub async fn find_user_by_phone(
    pool: &PgPool,
    phone: &str,
) -> Result<Option<uuid::Uuid>, sqlx::Error> {
    let row = sqlx::query("SELECT id FROM users WHERE phone = $1")
        .bind(phone)
        .fetch_optional(pool)
        .await?;

    Ok(row.map(|r| r.get("id")))
}

pub async fn insert_verification_code(
    pool: &PgPool,
    phone: &str,
    code: &str,
    expired_at: &chrono::DateTime<chrono::Utc>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO verification_codes (phone, code, expired_at) VALUES ($1, $2, $3)",
    )
    .bind(phone)
    .bind(code)
    .bind(expired_at)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn verify_and_use_code(
    pool: &PgPool,
    phone: &str,
    code: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE verification_codes SET used = true
         WHERE phone = $1 AND code = $2 AND used = false AND expired_at > now()
         RETURNING id",
    )
    .bind(phone)
    .bind(code)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn bind_apple_id(
    pool: &PgPool,
    user_id: &uuid::Uuid,
    apple_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET apple_id = $1 WHERE id = $2")
        .bind(apple_id)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}
