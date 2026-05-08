use std::env;

pub struct AppConfig {
    pub port: u16,
    pub jwt_secret: String,
    pub auth_service_url: String,
    pub user_service_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            port: env::var("GATEWAY_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("GATEWAY_PORT must be a number"),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            auth_service_url: env::var("AUTH_SERVICE_URL")
                .unwrap_or_else(|_| "http://localhost:8081".to_string()),
            user_service_url: env::var("USER_SERVICE_URL")
                .unwrap_or_else(|_| "http://localhost:8082".to_string()),
        }
    }
}
