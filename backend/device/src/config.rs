use std::env;

pub struct AppConfig {
    pub database_url: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            port: env::var("DEVICE_PORT")
                .unwrap_or_else(|_| "8083".to_string())
                .parse()
                .expect("DEVICE_PORT must be a number"),
        }
    }
}
