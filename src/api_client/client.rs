use crate::api_client::models::{CompletionRequest, CompletionResponse, Message};
use crate::config_manager::{Config, Provider};
use crate::error::{api_err, Result};
use crate::personalization::UserContext;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use std::time::Duration;

/// Trait defining the interface for API clients
#[async_trait::async_trait]
pub trait ApiClientTrait {
    /// Send a query to the model
    async fn send_query(&self, query: &str, user_context: Option<&UserContext>) -> Result<String>;
    /// Get a reference to the configuration
    fn config(&self) -> &Config;
}

/// Trait for creating model-specific clients
#[async_trait::async_trait]
pub trait ModelClient: Send + Sync {
    /// Send a request to the model and get a response
    async fn send_request(&self, messages: Vec<Message>, config: &Config) -> Result<String>;
}

/// HTTP-based model client implementation
#[derive(Debug)]
struct HttpModelClient {
    client: reqwest::Client,
}

impl HttpModelClient {
    fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| api_err(format!("Failed to create HTTP client: {}", e)))?;
            
        Ok(Self { client })
    }
}

#[async_trait::async_trait]
impl ModelClient for HttpModelClient {
    async fn send_request(&self, messages: Vec<Message>, config: &Config) -> Result<String> {
        let response = match config.provider {
            Provider::OpenAI => self.send_openai_request(messages, config).await?,
            Provider::OpenRouter => self.send_openrouter_request(messages, config).await?,
        };
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(api_err(format!(
                "API returned error ({}): {}", 
                status, error_text
            )));
        }
        
        let completion: CompletionResponse = response
            .json()
            .await
            .map_err(|e| api_err(format!("Failed to parse API response: {}", e)))?;
            
        completion.choices.first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| api_err("API returned no completion choices"))
    }
}

impl HttpModelClient {
    async fn send_openai_request(&self, messages: Vec<Message>, config: &Config) -> Result<reqwest::Response> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", config.get_api_key()))
                .map_err(|e| api_err(format!("Invalid API key format: {}", e)))?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        
        let request = CompletionRequest {
            model: config.openai_model.clone(),
            messages,
            max_tokens: config.max_tokens,
            temperature: 0.7,
            http_referer: None,
            http_referrer: None,
        };
        
        self.client
            .post(config.get_api_url())
            .headers(headers)
            .json(&request)
            .send()
            .await
            .map_err(|e| api_err(format!("API request failed: {}", e)))
    }
    
    async fn send_openrouter_request(&self, messages: Vec<Message>, config: &Config) -> Result<reqwest::Response> {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("http_referer"),
            HeaderValue::from_str(&config.get_site_url())
                .map_err(|e| api_err(format!("Invalid site URL: {}", e)))?,
        );
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", config.get_api_key()))
                .map_err(|e| api_err(format!("Invalid API key format: {}", e)))?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        
        let request = CompletionRequest {
            model: config.openrouter_model.clone(),
            messages,
            max_tokens: config.max_tokens,
            temperature: 0.7,
            http_referer: Some(config.get_site_url()),
            http_referrer: Some(config.get_site_url()),
        };
        
        self.client
            .post(config.get_api_url())
            .headers(headers)
            .json(&request)
            .send()
            .await
            .map_err(|e| api_err(format!("API request failed: {}", e)))
    }
}

/// Base API client implementation with shared functionality
#[derive(Debug)]
struct BaseApiClient<T: ModelClient> {
    config: Config,
    client: T,
}

impl<T: ModelClient> BaseApiClient<T> {
    fn new(config: Config, client: T) -> Self {
        Self { config, client }
    }
    
    fn create_messages(&self, query: &str, user_context: Option<&UserContext>) -> Vec<Message> {
        let mut messages = Vec::new();
        
        // Add system message
        let system_content = if let Some(context) = user_context {
            format!(
                "You are Chris, a helpful AI assistant. You are talking to {} who is using {} {} with kernel version {}. \
                Always provide responses specific to their operating system and environment. \
                If you need additional system information, ask the user or suggest specific commands they can run. \
                Never provide instructions for other operating systems unless explicitly asked.",
                context.username, context.os_name, context.os_version, context.kernel_version
            )
        } else {
            "You are Chris, a helpful AI assistant.".to_string()
        };
        
        messages.push(Message {
            role: "system".to_string(),
            content: system_content,
        });

        // Add user message
        messages.push(Message {
            role: "user".to_string(),
            content: query.to_string(),
        });

        messages
    }
}

/// OpenAI-specific API client implementation
#[derive(Debug)]
pub struct OpenAIClient {
    base: BaseApiClient<HttpModelClient>,
}

#[async_trait::async_trait]
impl ApiClientTrait for OpenAIClient {
    async fn send_query(&self, query: &str, user_context: Option<&UserContext>) -> Result<String> {
        let messages = self.base.create_messages(query, user_context);
        self.base.client.send_request(messages, self.config()).await
    }
    
    fn config(&self) -> &Config {
        &self.base.config
    }
}

/// OpenRouter-specific API client implementation
#[derive(Debug)]
pub struct OpenRouterClient {
    base: BaseApiClient<HttpModelClient>,
}

#[async_trait::async_trait]
impl ApiClientTrait for OpenRouterClient {
    async fn send_query(&self, query: &str, user_context: Option<&UserContext>) -> Result<String> {
        let messages = self.base.create_messages(query, user_context);
        self.base.client.send_request(messages, self.config()).await
    }
    
    fn config(&self) -> &Config {
        &self.base.config
    }
}

/// Factory function to create the appropriate API client based on the provider
pub fn create_api_client(config: Config) -> Result<Box<dyn ApiClientTrait>> {
    let http_client = HttpModelClient::new()?;
    
    match config.provider {
        Provider::OpenAI => Ok(Box::new(OpenAIClient { 
            base: BaseApiClient::new(config, http_client)
        })),
        Provider::OpenRouter => Ok(Box::new(OpenRouterClient { 
            base: BaseApiClient::new(config, http_client)
        })),
    }
}

// Public type alias for backward compatibility
pub type ApiClient = Box<dyn ApiClientTrait>; 