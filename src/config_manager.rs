use crate::error::{AppError, Result};
use config::{Environment, File};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Supported API providers
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    #[default]
    OpenAI,
    OpenRouter,
}

/// Configuration for the Chris Terminal application
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    
    /// Provider to use (OpenAI or OpenRouter)
    pub provider: Provider,
    
    /// Model to use for completions when using OpenRouter
    pub openrouter_model: String,

    /// Model to use for completions when using OpenAI
    pub openai_model: String,
    
    /// Maximum number of tokens in the completion
    pub max_tokens: usize,

    /// OpenRouter API key
    pub openrouter_api_key: String,
    
    /// OpenRouter base URL
    pub openrouter_base_url: String,

    /// OpenAI API key
    pub openai_api_key: String,
    
    /// API endpoint URL for OpenAI (used only when provider is OpenAI)
    pub openai_api_url: String,
    
    /// Whether to enable personalization features
    pub enable_personalization: bool,
    
    /// Whether to store conversation history
    pub store_history: bool,
    
    /// Path to the history file
    #[serde(default = "default_history_file")]
    pub history_file: PathBuf,
    
    /// Log level (error, warn, info, debug, trace)
    pub log_level: String,
}

/// Get the default Chris directory in the user's home folder
fn get_chris_dir() -> PathBuf {
    home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".chris")
}

/// Default function for the history file (used by Serde)
fn default_history_file() -> PathBuf {
    get_chris_dir().join("history")
}

/// Get the default path for the configuration file
fn get_config_path() -> PathBuf {
    get_chris_dir().join("config.toml")
}

/// Create directories necessary for config file if they don't exist
fn ensure_config_dir_exists() -> Result<()> {
    let config_path = get_config_path();
    let config_dir = config_path.parent().ok_or_else(|| {
        AppError::Config("Could not determine config directory".to_string())
    })?;
    
    if !config_dir.exists() {
        std::fs::create_dir_all(config_dir)
            .map_err(AppError::Io)?;
    }
    
    Ok(())
}

impl Default for Config {
    fn default() -> Self {
        // Default is OpenRouter with deepseek-r1:free model
        Self {
            provider: Provider::OpenRouter,
            openai_api_key: std::env::var("OPENAI_API_KEY").unwrap_or_default(),
            openai_model: "gpt-3.5-turbo".to_string(),
            openrouter_api_key: std::env::var("OPENROUTER_API_KEY").unwrap_or_default(),
            openrouter_base_url: "https://openrouter.ai/api/v1".to_string(),
            openai_api_url: "https://api.openai.com/v1/chat/completions".to_string(),
            openrouter_model: "deepseek/deepseek-r1:free".to_string(),
            max_tokens: 1000,
            enable_personalization: true,
            store_history: true,
            history_file: default_history_file(),
            log_level: "info".to_string(),
        }
    }
}

impl Config {
    /// Load configuration from files and environment variables
    pub fn load(config_path: Option<&Path>) -> Result<Self> {
        // Create default config if it doesn't exist
        Self::create_default_if_missing()?;
        
        // Create a new builder using config::builder
        let mut builder = config::Config::builder();
            
        // Add default config file
        builder = builder.add_source(
            File::from(get_config_path())
                .required(false)
        );
                
        // If a custom config path is provided, use it
        if let Some(path) = config_path {
            builder = builder.add_source(File::from(path).required(true));
        }
        
        // Add environment variables
        builder = builder.add_source(
            Environment::default()
                .try_parsing(true)
        );
        
        // Build the configuration
        let config = builder
            .build()
            .map_err(|e| AppError::Config(e.to_string()))?;
            
        // Deserialize into our config struct
        let cfg: Config = config
            .try_deserialize()
            .map_err(|e| AppError::Config(e.to_string()))?;
        
        // Validate required fields based on provider
        match cfg.provider {
            Provider::OpenAI => {
                if cfg.openai_api_key.is_empty() {
                    return Err(AppError::Config("OpenAI API key is required. Set it in the config file or use the OPENAI_API_KEY environment variable.".to_string()));
                }
            },
            Provider::OpenRouter => {
                if cfg.openrouter_api_key.is_empty() {
                    return Err(AppError::Config("OpenRouter API key is required. Set it in the config file or use the OPENROUTER_API_KEY environment variable.".to_string()));
                }
            }
        }
        
        Ok(cfg)
    }
    
    /// Create a default configuration file if it doesn't exist
    pub fn create_default_if_missing() -> Result<()> {
        let config_path = get_config_path();
        
        if !config_path.exists() {
            // Create directory structure
            ensure_config_dir_exists()?;
            
            // Create default config with OpenRouter and deepseek-r1:free
            let default_config = Config::default();
            let toml_string = toml::to_string_pretty(&default_config)
                .map_err(|e| AppError::Config(e.to_string()))?;
                
            std::fs::write(&config_path, toml_string)
                .map_err(AppError::Io)?;
                
            log::info!("Created default configuration file at {:?}", config_path);
        }
        
        Ok(())
    }
    
    /// Get the resolved API URL based on the provider
    pub fn get_api_url(&self) -> String {
        match self.provider {
            Provider::OpenAI => self.openai_api_url.clone(),
            Provider::OpenRouter => format!("{}/chat/completions", self.openrouter_base_url),
        }
    }
    
    /// Get the API key based on the provider
    pub fn get_api_key(&self) -> String {
        match self.provider {
            Provider::OpenAI => self.openai_api_key.clone(),
            Provider::OpenRouter => self.openrouter_api_key.clone(),
        }
    }
    
    /// Get the site URL (hardcoded)
    pub fn get_site_url(&self) -> String {
        "example.com".to_string()
    }
    
    /// Get the site name (hardcoded)
    pub fn get_site_name(&self) -> String {
        "CommanderChris".to_string()
    }

    /// Gets the default config path
    pub fn get_config_path() -> PathBuf {
        let mut config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        config_dir.push("chris");
        config_dir.push("config.toml");
        config_dir
    }
    
    /// Save configuration to a file
    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        // Make sure the parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        // Serialize to TOML
        let toml_string = toml::to_string_pretty(self)?;
        
        // Write to file
        std::fs::write(path, toml_string)?;
        
        Ok(())
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Provider: {}", match self.provider {
            Provider::OpenAI => "OpenAI",
            Provider::OpenRouter => "OpenRouter",
        })?;
        
        if self.provider == Provider::OpenAI {
            writeln!(f, "Model: {}", self.openai_model)?;
        } else {
            writeln!(f, "Model: {}", self.openrouter_model)?;
        }
        
        writeln!(f, "Max Tokens: {}", self.max_tokens)?;
        
        // API connection settings
        if self.provider == Provider::OpenAI {
            writeln!(f, "API URL: {}", self.openai_api_url)?;
            writeln!(f, "OpenAI API Key: {}", if self.openai_api_key.is_empty() { 
                "Not set" 
            } else { 
                "[REDACTED]" 
            })?;
        } else {
            writeln!(f, "API URL: {}", self.get_api_url())?;
            writeln!(f, "OpenRouter Site URL: {}", self.get_site_url())?;
            writeln!(f, "OpenRouter Site Name: {}", self.get_site_name())?;
            writeln!(f, "OpenRouter API Key: {}", if self.openrouter_api_key.is_empty() { 
                "Not set" 
            } else { 
                "[REDACTED]" 
            })?;
        }
        
        // User experience settings
        writeln!(f, "Personalization: {}", if self.enable_personalization { "Enabled" } else { "Disabled" })?;
        writeln!(f, "History Storage: {}", if self.store_history { "Enabled" } else { "Disabled" })?;
        writeln!(f, "History File: {:?}", self.history_file)?;
        
        // System settings
        writeln!(f, "Log Level: {}", self.log_level)?;
        
        Ok(())
    }
}
