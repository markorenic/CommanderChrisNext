//! API client for interacting with GPT models.
//!
//! This module provides functionality to communicate with various GPT
//! model providers like OpenAI and OpenRouter.

mod models;
mod client;

// Re-exports for public API
pub use client::{ApiClient, ApiClientTrait, OpenAIClient, OpenRouterClient, create_api_client};
pub use models::Message; 