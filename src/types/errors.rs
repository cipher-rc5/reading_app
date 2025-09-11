// file: src/types/errors.rs
// description: Comprehensive error types

use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug, Clone)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] DatabaseError),
    #[error("Network error: {0}")]
    Network(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
    #[error("IO error: {0}")]
    Io(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
}

#[derive(Error, Debug, Clone)]
pub enum DatabaseError {
    #[error("Connection failed: {0}")]
    Connection(String),
    #[error("Query failed: {0}")]
    Query(String),
    #[error("Transaction failed: {0}")]
    Transaction(String),
    #[error("Schema error: {0}")]
    Schema(String),
}

#[derive(Error, Debug, Clone)]
pub enum ValidationError {
    #[error("Input is empty")]
    EmptyInput,
    #[error("Input too long: {current} characters, max {max}")]
    InputTooLong { current: usize, max: usize },
    #[error("Invalid characters in input")]
    InvalidCharacters,
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Serialization(err.to_string())
    }
}
