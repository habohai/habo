use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::{Json, routing::{get, post, delete}, Router};
use sqlx::PgPool;
use tracing_subscriber::EnvFilter;

mod config;
mod db;
mod handlers;

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

    let app = Router::new()
        .route("/device/bind", post(handlers::device::bind_device_handler))
        .route("/device/list", get(handlers::device::list_devices_handler))
        .route("/device/unbind/:id", delete(handlers::device::unbind_device_handler))
        .with_state(pool);

    let addr = format!("0.0.0.0:{}", config.port);
    tracing::info!("Device service starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
