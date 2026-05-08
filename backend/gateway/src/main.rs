use axum::routing::{any, get, post};
use axum::{middleware, Router};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

mod config;
mod auth_mw;
mod routes;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub jwt_secret: String,
    pub auth_service_url: String,
    pub user_service_url: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    dotenvy::dotenv().ok();
    let cfg = config::AppConfig::from_env();

    let state = Arc::new(AppState {
        jwt_secret: cfg.jwt_secret,
        auth_service_url: cfg.auth_service_url,
        user_service_url: cfg.user_service_url,
    });

    // Public routes (no auth required)
    let public_routes = Router::new()
        .route("/health", get(routes::health::health_check))
        .route("/auth/send-code", post(routes::proxy::proxy_handler))
        .route("/auth/verify", post(routes::proxy::proxy_handler))
        .route("/auth/refresh", post(routes::proxy::proxy_handler));

    // Protected routes (auth required)
    let protected_routes = Router::new()
        .route("/user/*path", any(routes::proxy::proxy_handler))
        .route("/auth/bind-apple", post(routes::proxy::proxy_handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_mw::auth::auth_middleware,
        ));

    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = format!("0.0.0.0:{}", cfg.port);
    tracing::info!("Gateway starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
