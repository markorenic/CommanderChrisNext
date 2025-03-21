use crate::api_client::{ApiClient, create_api_client};
use crate::config_manager::Config;
use crate::error::Result;
use crate::personalization::Personalization;
use crate::util;
use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::path::PathBuf;
use std::io::Write;

/// Enum to track which reader mode we're in
#[derive(Debug, Clone, Copy)]
pub enum ReaderMode {
    /// Single query mode
    SingleQuery,
    /// Interactive REPL mode
    Interactive,
}

/// Subcommands for the CLI
#[derive(Parser, Debug)]
pub enum Command {
    /// Configuration commands
    Config {
        /// Create a new configuration file
        #[clap(long)]
        create: bool,
        
        /// Show current configuration
        #[clap(long)]
        show: bool,
        
        /// Path to configuration file
        #[clap(short, long, value_name = "FILE")]
        config: Option<PathBuf>,
    },
}

/// Chris - A terminal interface for GPT models
#[derive(Parser, Debug)]
#[clap(author, version, about = "Chris - Your friendly terminal assistant")]
pub struct Cli {
    /// Optional subcommand
    #[clap(subcommand)]
    pub command: Option<Command>,
    
    /// Query to send to the model
    #[clap(index = 1)]
    pub query: Option<String>,
    
    /// Path to configuration file
    #[clap(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,
    
    /// Enable verbose output
    #[clap(short, long)]
    pub verbose: bool,
    
    /// Show current configuration
    #[clap(long)]
    pub show_config: bool,

    /// Enable debug mode to show context sent to the model
    #[clap(long)]
    pub debug: bool,
    
    /// Enable personalization to include system information
    #[clap(long)]
    pub personalize: bool,
    
    /// Specify the model to use
    #[clap(long)]
    pub model: Option<String>,
}

impl Cli {
    /// Run the CLI with the provided arguments
    pub async fn run(&self) -> Result<()> {
        // Initialize logging
        if self.verbose {
            std::env::set_var("RUST_LOG", "debug");
        } else {
            std::env::set_var("RUST_LOG", "info");
        }
        env_logger::init();

        // Handle subcommands
        if let Some(command) = &self.command {
            match command {
                Command::Config { create, show, config } => {
                    if *create {
                        use crate::config_manager::Config;
                        let config_path = config.as_ref().map(|p| p.as_path());
                        
                        // If no path specified, use default
                        let actual_path = if let Some(path) = config_path {
                            path.to_path_buf()
                        } else {
                            crate::config_manager::Config::get_config_path()
                        };
                        
                        // Create default config
                        let default_config = Config::default();
                        default_config.save_to_file(&actual_path)?;
                        
                        println!("Created default configuration at: {}", actual_path.display());
                        return Ok(());
                    }
                    
                    if *show {
                        let config_path = config.as_ref().map(|p| p.as_path());
                        let loaded_config = crate::config_manager::Config::load(config_path)?;
                        println!("{}", loaded_config);
                        return Ok(());
                    }
                    
                    // If neither option was specified
                    println!("Please specify either --create or --show with the config command");
                    return Ok(());
                }
            }
        }
        
        // Show configuration if requested
        if self.show_config {
            let config_path = self.config.as_deref();
            let loaded_config = crate::config_manager::Config::load(config_path)?;
            println!("{}", loaded_config);
            return Ok(());
        }

        // Load configuration
        let config_path = self.config.as_deref();
        let mut config = crate::config_manager::Config::load(config_path)?;
        
        // Override config with command line arguments
        if let Some(model) = &self.model {
            if config.provider == crate::config_manager::Provider::OpenAI {
                config.openai_model = model.clone();
            } else {
                config.openrouter_model = model.clone();
            }
        }
        
        // Enable personalization if requested
        let enable_personalization = self.personalize || config.enable_personalization;
        
        // Create API client
        let api_client = create_api_client(config)?;
        
        // Create personalization module
        let mut personalization = Personalization::new(enable_personalization);
        
        // Handle query
        match &self.query {
            Some(query) => {
                self.handle_query(&api_client, query, &mut personalization).await?;
            }
            None => {
                self.run_interactive_mode(&api_client, &mut personalization).await?;
            }
        }

        Ok(())
    }
    
    /// Handle a single query
    async fn handle_query(
        &self,
        api_client: &ApiClient,
        query: &str,
        personalization: &mut Personalization,
    ) -> Result<()> {
        let context = personalization.get_user_context();
        
        util::print_header("Query");
        println!("{}", query);

        if self.debug {
            util::print_header("Debug Context");
            println!("{}", personalization.debug_context());
        }
        
        util::print_header("Response");
        let response = api_client.send_query(query, context).await?;
        let formatted_response = util::format_response(&response);
        println!("{}", formatted_response);
        
        // Handle command execution
        let commands = util::extract_commands(&response);
        if !commands.is_empty() {
            println!("\nDetected commands:");
            for (i, cmd) in commands.iter().enumerate() {
                print!("\nCommand {}: {}\nWould you like to execute this command? (y/N): ", i + 1, cmd);
                std::io::stdout().flush()?;
                
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                
                if input.trim().eq_ignore_ascii_case("y") {
                    match util::execute_command(cmd) {
                        Ok(output) => {
                            println!("\nCommand Output:");
                            println!("┌── output ────────────────────────────────────────────────");
                            for line in output.lines() {
                                println!("│ {}", line);
                            }
                            println!("└{}", "─".repeat(60));
                        }
                        Err(e) => {
                            eprintln!("Error executing command: {}", e);
                        }
                    }
                } else {
                    println!("Command execution skipped.");
                }
            }
        }
        
        Ok(())
    }
    
    /// Run the interactive REPL mode
    async fn run_interactive_mode(
        &self,
        api_client: &ApiClient,
        personalization: &mut Personalization,
    ) -> Result<()> {
        util::print_header("Chris Interactive Mode");
        println!("Type your queries and press Enter. Use Ctrl+D or type 'exit' to quit.");
        println!("Type 'help' for available commands.\n");
        
        let mut rl = DefaultEditor::new()?;
        let history_file = api_client.config().history_file.clone();
        
        // Try to load history file
        if api_client.config().store_history {
            let _ = rl.load_history(&history_file);
        }
        
        loop {
            let readline = rl.readline("chris> ");
            match readline {
                Ok(line) => {
                    let line = line.trim();
                    
                    // Skip empty lines
                    if line.is_empty() {
                        continue;
                    }
                    
                    // Add to history
                    if api_client.config().store_history {
                        let _ = rl.add_history_entry(line);
                    }
                    
                    // Handle special commands
                    match line.to_lowercase().as_str() {
                        "exit" | "quit" => break,
                        "help" => {
                            println!("Available commands:");
                            println!("  help    - Show this help message");
                            println!("  exit    - Exit the program");
                            println!("  clear   - Clear the screen");
                            println!("  context - Show user context (if personalization is enabled)");
                            println!("  debug   - Toggle debug mode");
                            println!("  Any other input will be sent as a query to the model");
                        }
                        "clear" => {
                            print!("\x1B[2J\x1B[1;1H");
                        }
                        "context" => {
                            if let Some(context) = personalization.get_user_context() {
                                println!("{}", context);
                            } else {
                                println!("Personalization is disabled. No context available.");
                            }
                        }
                        "debug" => {
                            personalization.set_debug(!personalization.is_debug());
                            println!("Debug mode {}", if personalization.is_debug() { "enabled" } else { "disabled" });
                        }
                        _ => {
                            // Regular query
                            if self.debug || personalization.is_debug() {
                                util::print_header("Debug Context");
                                println!("{}", personalization.debug_context());
                            }

                            match api_client
                                .send_query(line, personalization.get_user_context())
                                .await
                            {
                                Ok(response) => {
                                    util::print_header("Response");
                                    println!("{}", util::format_response(&response));
                                }
                                Err(e) => {
                                    eprintln!("Error: {}", e);
                                }
                            }
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("Ctrl+C pressed. Press Ctrl+D or type 'exit' to quit.");
                }
                Err(ReadlineError::Eof) => {
                    println!("Exiting...");
                    break;
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    break;
                }
            }
        }
        
        // Save history on exit
        if api_client.config().store_history {
            if let Err(e) = rl.save_history(&history_file) {
                log::warn!("Failed to save history: {}", e);
            }
        }
        
        Ok(())
    }
} 