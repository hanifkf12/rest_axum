use std::env;

/// Application configuration loaded from environment variables.
/// This is an infrastructure concern - the application doesn't care
/// where config comes from.
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingVar("DATABASE_URL".to_string()))?;

        let redis_url = env::var("REDIS_URL")
            .map_err(|_| ConfigError::MissingVar("REDIS_URL".to_string()))?;

        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let server_port = env::var("SERVER_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);

        Ok(Self {
            database_url,
            redis_url,
            server_host,
            server_port,
        })
    }
}

#[derive(Debug)]
pub enum ConfigError {
    MissingVar(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::MissingVar(var) => write!(f, "Missing environment variable: {}", var),
        }
    }
}

impl std::error::Error for ConfigError {}
