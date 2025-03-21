use serde::{Deserialize, Serialize};

/// Represents a message in the chat completion API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Role of the message sender (e.g., "user", "assistant", "system")
    pub role: String,

    /// Content of the message
    pub content: String,
}

/// Request structure for the GPT API
#[derive(Debug, Serialize)]
pub(crate) struct CompletionRequest {
    /// The model to use for completions
    pub model: String,

    /// Messages to send to the model
    pub messages: Vec<Message>,

    /// Maximum number of tokens to generate
    pub max_tokens: usize,

    /// Temperature parameter for controlling randomness
    pub temperature: f32,

    /// Referer header for OpenRouter (http_referer variant)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_referer: Option<String>,

    /// Referer header for OpenRouter (http_referrer variant)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_referrer: Option<String>,
}

/// Response structure from the GPT API
#[derive(Debug, Deserialize)]
pub(crate) struct CompletionResponse {
    /// Array of completion choices
    pub choices: Vec<Choice>,
}

/// Represents a single completion choice
#[derive(Debug, Deserialize)]
pub(crate) struct Choice {
    /// The message containing the completion
    pub message: Message,
}
