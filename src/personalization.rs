use crate::error::Result;
use serde::{Deserialize, Serialize};
use sysinfo::System;
use std::fmt;

// TODO: Add system context. This will include the current directory, the current shell, and the current user.

/// Represents the user's system context for personalized interactions
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserContext {
    /// Username for personalized responses
    pub username: String,
    
    /// Operating system information
    pub os_name: String,
    
    /// Operating system version
    pub os_version: String,
    
    /// Kernel version
    pub kernel_version: String,
    
    /// Host name
    pub hostname: String,
}

impl fmt::Display for UserContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User Context:\n  Username: {}\n  OS: {} {}\n  Kernel: {}\n  Hostname: {}",
            self.username,
            self.os_name,
            self.os_version,
            self.kernel_version,
            self.hostname
        )
    }
}

/// Manages personalization features
#[derive(Debug, Serialize, Deserialize)]
pub struct Personalization {
    /// Whether personalization is enabled
    enabled: bool,
    /// Whether debug mode is enabled
    debug: bool,
    /// User context information, if available
    user_context: Option<UserContext>,
}

impl Default for Personalization {
    fn default() -> Self {
        Self {
            enabled: false,
            debug: false,
            user_context: None,
        }
    }
}

impl Personalization {
    /// Create a new personalization module with the given enabled state
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            debug: false,
            user_context: None,
        }
    }

    /// Initialize user context if enabled
    ///
    /// Gets the username when personalization is enabled.
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn initialize(&mut self) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }
        
        let username = whoami::username();
        
        self.user_context = Some(UserContext {
            username,
            os_name: System::name().unwrap_or_else(|| String::from("Unknown")),
            os_version: System::os_version().unwrap_or_else(|| String::from("Unknown")),
            kernel_version: System::kernel_version().unwrap_or_else(|| String::from("Unknown")),
            hostname: System::host_name().unwrap_or_else(|| String::from("Unknown")),
        });
        
        Ok(())
    }
    
    /// Get the user context, if available
    ///
    /// # Returns
    /// * `Option<&UserContext>` - Reference to user context if available
    pub fn get_user_context(&self) -> Option<&UserContext> {
        self.user_context.as_ref()
    }
    
    /// Check if personalization is enabled
    ///
    /// # Returns
    /// * `bool` - Whether personalization is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Check if debug mode is enabled
    ///
    /// # Returns
    /// * `bool` - Whether debug mode is enabled
    pub fn is_debug(&self) -> bool {
        self.debug
    }
    
    /// Enable or disable personalization
    ///
    /// # Arguments
    /// * `enabled` - New enabled state
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.enabled = enabled;
        
        // If enabling and user context is not initialized, initialize it
        if enabled && self.user_context.is_none() {
            self.initialize()?;
        }
        
        Ok(())
    }

    /// Enable or disable debug mode
    ///
    /// # Arguments
    /// * `debug` - New debug state
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    /// Get a detailed debug representation of the context
    ///
    /// # Returns
    /// * `String` - Debug representation of the context
    pub fn debug_context(&self) -> String {
        let mut output = String::new();
        output.push_str("=== Debug Context ===\n");
        output.push_str(&format!("Personalization enabled: {}\n", self.enabled));
        
        if let Some(ctx) = &self.user_context {
            output.push_str("\nUser Context:\n");
            output.push_str(&format!("  Username: {}\n", ctx.username));
            output.push_str(&format!("  OS: {} {}\n", ctx.os_name, ctx.os_version));
            output.push_str(&format!("  Kernel: {}\n", ctx.kernel_version));
            output.push_str(&format!("  Hostname: {}\n", ctx.hostname));
        } else {
            output.push_str("\nNo user context available\n");
        }
        
        output.push_str("==================\n");
        output
    }
} 