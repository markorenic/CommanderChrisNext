//! # Chris - Terminal Interface for GPT models
//! 
//! This library provides functionality to interact with GPT models
//! through a terminal interface.

/// Contains API client functionality for communicating with GPT providers
pub mod api_client;

/// Command-line interface and REPL implementation
pub mod cli;

/// Configuration management
pub mod config_manager;

/// Error handling types and utilities
pub mod error;

/// System personalization features
pub mod personalization;

/// Utility functions
pub mod util;

// Re-export commonly used components for easier access
pub use api_client::ApiClient;
pub use cli::{Cli, ReaderMode};
pub use config_manager::Config;
pub use error::{AppError, Result}; 