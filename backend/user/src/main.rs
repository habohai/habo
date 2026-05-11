use axum::{routing::get, Router};
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
        .route(
            "/user/profile",
            get(handlers::profile::get_my_profile).put(handlers::profile::update_profile),
        )
        .route("/user/profile/:id", get(handlers::profile::get_user_profile))
        .with_state(pool);

    let addr = format!("0.0.0.0:{}", config.port);
    tracing::info!("User service starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
