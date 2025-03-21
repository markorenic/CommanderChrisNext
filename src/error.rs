//! Error handling for the application.
//!
//! This module provides a centralized error type and utilities for
//! handling errors throughout the application.

use rustyline::error::ReadlineError;
use thiserror::Error;

/// Alias for Result with our custom error type
pub type Result<T> = std::result::Result<T, AppError>;

/// Application-wide error types
///
/// This enum encapsulates all possible errors that can occur within the application,
/// providing a centralized way to handle and propagate errors.
#[derive(Error, Debug)]
pub enum AppError {
    /// Error during I/O operations
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Error related to configuration
    #[error("Configuration error: {0}")]
    Config(String),

    /// Error from the API client
    #[error("API error: {0}")]
    Api(String),

    /// HTTP client error
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Missing API key
    #[error("Missing API key")]
    MissingApiKey,

    /// Error from the REPL
    #[error("REPL error: {0}")]
    Repl(String),

    /// Error retrieving system information
    #[error("System information error: {0}")]
    SystemInfo(String),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Unknown or unexpected error
    #[error("Unknown error: {0}")]
    Unknown(String),
}

// Implement From traits for common error conversions
impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError::Unknown(error)
    }
}

impl From<&str> for AppError {
    fn from(error: &str) -> Self {
        AppError::Unknown(error.to_string())
    }
}

impl From<ReadlineError> for AppError {
    fn from(error: ReadlineError) -> Self {
        AppError::Repl(error.to_string())
    }
}

impl From<toml::ser::Error> for AppError {
    fn from(err: toml::ser::Error) -> Self {
        Self::Config(err.to_string())
    }
}

/// Creates a config error with the given message
#[inline]
pub fn config_err<S: Into<String>>(msg: S) -> AppError {
    AppError::Config(msg.into())
}

/// Creates an API error with the given message
#[inline]
pub fn api_err<S: Into<String>>(msg: S) -> AppError {
    AppError::Api(msg.into())
}

/// Creates a validation error with the given message
#[inline]
pub fn validation_err<S: Into<String>>(msg: S) -> AppError {
    AppError::Validation(msg.into())
}
