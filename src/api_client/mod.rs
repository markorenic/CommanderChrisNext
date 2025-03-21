//! API client for interacting with GPT models.
//!
//! This module provides functionality to communicate with various GPT
//! model providers like OpenAI and OpenRouter.

mod client;
mod models;

// Re-exports for public API
pub use client::{create_api_client, ApiClient, ApiClientTrait, OpenAIClient, OpenRouterClient};
pub use models::Message;
