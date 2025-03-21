use chris::cli::Cli;
use clap::Parser;
use std::process;

#[tokio::main]
async fn main() {
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Run the CLI
    if let Err(e) = cli.run().await {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
