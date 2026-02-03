use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub server_host: String,
    pub server_port: u16,
    pub access_token_ttl_minutes: i64,
    pub refresh_token_ttl_days: i64,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost/learner".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "dev-secret-change-in-production".to_string()),
            server_host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            access_token_ttl_minutes: env::var("ACCESS_TOKEN_TTL_MINUTES")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(15),
            refresh_token_ttl_days: env::var("REFRESH_TOKEN_TTL_DAYS")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(7),
        }
    }

    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::from_env()
    }
}