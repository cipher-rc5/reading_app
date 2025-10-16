// file: src/utils/logging.rs
// description: Logging configuration

use crate::types::AppResult;
use std::sync::Once;
use tracing_subscriber::{EnvFilter, fmt};

static INIT: Once = Once::new();

pub fn init() -> AppResult<()> {
    INIT.call_once(|| {
        if std::env::var("RUST_LOG").is_err() {
            unsafe {
                std::env::set_var("RUST_LOG", "reading_app=info,warn");
            }
        }

        let filter = EnvFilter::from_default_env();
        let subscriber = fmt()
            .with_env_filter(filter)
            .with_ansi(true)
            .with_target(false)
            .compact()
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set global default subscriber");
    });

    Ok(())
}
