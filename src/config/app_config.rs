// file: src/config/app_config.rs
// description: application configuration management

use crate::types::AppResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub groq_api_key: String,
    pub groq_base_url: String,
    pub database_path: String,
    pub log_level: String,
}

impl AppConfig {
    pub fn load() -> AppResult<Self> {
        super::environment::load_env_file()?;

        let groq_api_key = std::env::var("GROQ_API_KEY").map_err(|_| {
            crate::types::AppError::Config("GROQ_API_KEY environment variable not set".to_string())
        })?;

        let groq_base_url = std::env::var("GROQ_BASE_URL")
            .unwrap_or_else(|_| "https://api.groq.com/openai/v1".to_string());

        let database_path =
            std::env::var("DATABASE_PATH").unwrap_or_else(|_| "reading_app.db".to_string());

        let log_level =
            std::env::var("RUST_LOG").unwrap_or_else(|_| "reading_app=info".to_string());

        Ok(Self {
            groq_api_key,
            groq_base_url,
            database_path,
            log_level,
        })
    }
}
