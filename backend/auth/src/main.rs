use axum::{routing::post, Router};
use redis::Client as RedisClient;
use sqlx::PgPool;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

mod config;
mod db;
mod handlers;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub redis: RedisClient,
    pub jwt_secret: String,
    pub sms_provider: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    dotenvy::dotenv().ok();
    let config = config::AppConfig::from_env();

    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    let redis_client = RedisClient::open(config.redis_url.as_str())
        .expect("Failed to create Redis client");

    let state = Arc::new(AppState {
        pool,
        redis: redis_client,
        jwt_secret: config.jwt_secret,
        sms_provider: config.sms_provider,
    });

    let app = Router::new()
        .route("/auth/send-code", post(handlers::send_code::send_code_handler))
        .route("/auth/verify", post(handlers::verify::verify_code_handler))
        .route("/auth/refresh", post(handlers::refresh::refresh_token_handler))
        .route("/auth/bind-apple", post(handlers::bind_apple::bind_apple_handler))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", config.port);
    tracing::info!("Auth service starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
