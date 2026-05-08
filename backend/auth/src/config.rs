use std::env;

pub struct AppConfig {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub sms_provider: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            redis_url: env::var("REDIS_URL")
                .expect("REDIS_URL must be set"),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            sms_provider: env::var("SMS_PROVIDER")
                .unwrap_or_else(|_| "console".to_string()),
            port: env::var("AUTH_PORT")
                .unwrap_or_else(|_| "8081".to_string())
                .parse()
                .expect("AUTH_PORT must be a number"),
        }
    }
}
