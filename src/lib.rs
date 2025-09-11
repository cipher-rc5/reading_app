// file: src/lib.rs
// description: Library root with public exports

pub mod app;
pub mod client;
pub mod config;
pub mod database;
pub mod services;
pub mod types;
pub mod ui;
pub mod utils;

// Re-export commonly used types
pub use types::{
    article::{Article, ArticleSubject},
    errors::{AppError, AppResult},
    settings::UISettings,
};

// Re-export main application
pub use app::App;
