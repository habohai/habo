use axum::{routing::{get, post, put}, Router};
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
        .route("/workout", post(handlers::workout::create_workout_handler)
            .get(handlers::workout::list_workouts_handler))
        .route("/workout/:id", get(handlers::workout::get_workout_handler)
            .put(handlers::workout::update_workout_handler))
        .with_state(pool);

    let addr = format!("0.0.0.0:{}", config.port);
    tracing::info!("Workout service starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
