// file: src/config/environment.rs
// description: Environment variable handling

use crate::types::{AppError, AppResult};
use std::path::Path;
use tracing::{info, warn};

pub fn load_env_file() -> AppResult<()> {
    let env_file_exists = Path::new(".env").exists();

    if env_file_exists {
        info!("Loading environment variables from .env file");
        dotenvy::dotenv()
            .map_err(|e| AppError::Config(format!("Failed to load .env file: {}", e)))?;
    } else {
        warn!("No .env file found, using system environment variables");
    }

    Ok(())
}
