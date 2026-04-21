use std::env;
use serde::Deserialize;
use crate::error::AppError;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        dotenv::dotenv().ok();

        let port = env::var("PORT")
            .unwrap_or_else(|_| "3009".to_string())
            .parse()
            .map_err(|e| AppError::Config(format!("Invalid PORT: {}", e)))?;

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://cn:cn@localhost/ze_db".to_string());

        Ok(Config {
            port,
            database_url,
        })
    }
}