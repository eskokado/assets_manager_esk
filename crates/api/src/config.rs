use std::env;

#[derive(Debug, Clone)]
pub struct Settings {
    pub bind_addr: String,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in_secs: i64,
}

impl Settings {
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            bind_addr: env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:4000".into()),
            database_url: env::var("DATABASE_URL")
                .map_err(|_| "DATABASE_URL is required".to_string())?,
            jwt_secret: env::var("JWT_SECRET")
                .map_err(|_| "JWT_SECRET is required".to_string())?,
            jwt_expires_in_secs: env::var("JWT_EXPIRES_IN_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3600),
        })
    }
}
