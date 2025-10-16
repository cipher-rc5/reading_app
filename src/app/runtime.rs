// file: src/app/runtime.rs
// description: Application-scoped Tokio runtime with explicit ownership

use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::{
    runtime::{Handle, Runtime},
    task::JoinHandle,
};
use tracing::info;

#[derive(Clone)]
pub struct AppRuntime {
    runtime: Arc<Runtime>,
}

impl AppRuntime {
    pub fn new() -> Result<Self> {
        info!("Creating application async runtime");
        let runtime = Runtime::new().context("failed to create Tokio runtime")?;
        Ok(Self {
            runtime: Arc::new(runtime),
        })
    }

    pub fn handle(&self) -> Handle {
        self.runtime.handle().clone()
    }

    pub fn block_on<F>(&self, future: F) -> F::Output
    where
        F: std::future::Future,
    {
        self.runtime.block_on(future)
    }

    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.handle().spawn(future)
    }
}
