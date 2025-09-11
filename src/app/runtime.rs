// file: src/app/runtime.rs
// description: Global runtime management for async operations

use std::sync::OnceLock;
use tokio::runtime::{Handle, Runtime};
use tracing::info;

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

/// Get or create the global tokio runtime
pub fn get_or_create_runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| {
        info!("Creating global tokio runtime");
        Runtime::new().expect("Failed to create tokio runtime")
    })
}

/// Get the handle to the global runtime
pub fn get_runtime_handle() -> Handle {
    get_or_create_runtime().handle().clone()
}

/// Initialize the global runtime (called early in main)
pub fn init_runtime() {
    let _ = get_or_create_runtime();
    info!("Global runtime initialized");
}

// Legacy RuntimeManager for backward compatibility
pub struct RuntimeManager {
    runtime: Runtime,
}

impl RuntimeManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        info!("Creating async runtime");

        let runtime =
            Runtime::new().map_err(|e| format!("Failed to create Tokio runtime: {}", e))?;

        Ok(Self { runtime })
    }

    pub fn handle(&self) -> Handle {
        self.runtime.handle().clone()
    }

    pub fn block_on<F: std::future::Future>(&self, future: F) -> F::Output {
        self.runtime.block_on(future)
    }
}

impl Default for RuntimeManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default runtime")
    }
}
