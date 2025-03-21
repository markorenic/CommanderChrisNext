# CommanderChrisNext

<div align="center">

![CommanderChrisNext Logo](https://img.shields.io/badge/💫-CommanderChrisNext-1A73E8)

[![Crates.io](https://img.shields.io/crates/v/chris.svg)](https://crates.io/crates/chris)
[![CI Status](https://github.com/markorenic/CommanderChrisNext/workflows/CI/badge.svg)](https://github.com/markorenic/CommanderChrisNext/actions?query=workflow%3ACI)
[![Security Audit](https://github.com/markorenic/CommanderChrisNext/workflows/Security%20Audit/badge.svg)](https://github.com/markorenic/CommanderChrisNext/actions?query=workflow%3A%22Security+Audit%22)
[![MIT Licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.70+-93450a.svg)](https://www.rust-lang.org/)

🚀 A powerful terminal-based AI assistant powered by GPT models, written in Rust for optimal performance

[Key Features](#features) •
[Installation](#installation) •
[Usage](#usage) •
[Configuration](#configuration) •
[Contributing](#contributing) •
[License](#license)

</div>

## Features

- 🧠 **Multiple AI Providers** - Support for OpenAI and OpenRouter APIs, with access to GPT-4, Claude, and more
- ⚡ **High Performance** - Built with Rust for speed, low memory usage, and reliability
- 💬 **Interactive Mode** - Full-featured REPL with command history and smart suggestions
- 🔄 **Context Awareness** - Maintain conversation context across interactions
- 🔧 **Extensive Configuration** - Fine-tune every aspect of the assistant
- 🔒 **Secure** - Local API key storage with safe handling
- 🖥️ **System Integration** - Optional system information personalization
- 📝 **Command History** - Persistent history with searchable entries

## Installation

### From Crates.io

```bash
cargo install chris
```

### From Binaries

Download pre-built binaries from the [Releases page](https://github.com/markorenic/CommanderChrisNext/releases).

**Linux/macOS**:
```bash
# Make executable and move to path
chmod +x chris-*
sudo mv chris-* /usr/local/bin/chris
```

**Windows**:
Download the executable and add its location to your PATH.

### From Source

```bash
# Clone the repository
git clone https://github.com/markorenic/CommanderChrisNext.git
cd CommanderChrisNext

# Build in release mode
cargo build --release

# Run the binary
./target/release/chris
```

## Quick Start

```bash
# Set your API key
export CHRIS_API_KEY=your_api_key_here

# Ask a question
chris "What are the primary features of Rust that make it good for systems programming?"

# Start interactive mode
chris
```

## Usage

### Single Query Mode

```bash
# Basic question
chris "What is the capital of France?"

# With personalization (includes system info)
chris --personalize "What are my system specs?"

# Advanced query with specific model
chris --model gpt-4 "Explain quantum computing in simple terms"

# Debug mode for troubleshooting
chris --verbose "Help me debug this error"
```

### Interactive Mode

Start an interactive session:

```bash
chris
```

Available commands in interactive mode:
- `help` - Display available commands
- `exit` or `quit` - Exit interactive mode
- `clear` - Clear the screen
- `context` - Show current conversation context
- `reset` - Clear conversation history
- `model [name]` - Change the model on the fly

## Configuration

### Configuration File

Create a default configuration file:

```bash
chris config --create
```

This creates a configuration file at `~/.config/chris/config.toml`.

### Configuration Options

```toml
# Required API key for authentication
api_key = "your-api-key-here"

# Provider to use: "openai" or "openrouter"
provider = "openai"

# API endpoint URL (default changes based on provider)
api_url = "https://api.openai.com/v1/chat/completions"

# OpenRouter specific options (only needed for OpenRouter)
site_url = "https://your-website.com"  # Your site URL for attribution
site_name = "My Application"           # Your app name for attribution

# Model to use for completions
model = "gpt-3.5-turbo"                # For OpenAI
# model = "openai/gpt-3.5-turbo"       # For OpenRouter format

# Maximum tokens in the completion
max_tokens = 1000

# Other options
enable_personalization = false
store_history = true
history_file = "/path/to/.chris_history"
log_level = "info"
```

### Using OpenRouter

[OpenRouter](https://openrouter.ai/) provides access to a wide variety of models from different providers. To use OpenRouter:

1. Create an account at [openrouter.ai](https://openrouter.ai/) and get your API key
2. Update your configuration:
   ```toml
   provider = "openrouter"
   api_key = "your-openrouter-key"
   model = "openai/gpt-3.5-turbo"  # Or any other model supported by OpenRouter
   ```

OpenRouter model IDs typically follow the format `provider/model-name`. For example:
- `openai/gpt-4o`
- `anthropic/claude-3-opus-20240229`
- `meta-llama/llama-3-70b-instruct`

## Advanced Usage

### Environment Variables

All configuration options can be set via environment variables:

```bash
export CHRIS_API_KEY="your-api-key"
export CHRIS_MODEL="gpt-4"
export CHRIS_PROVIDER="openai"
export CHRIS_MAX_TOKENS="2000"
```

### Aliases

Add useful aliases to your shell configuration:

```bash
# Add to .bashrc or .zshrc
alias ask="chris"
alias chat="chris"
alias gpt="chris --model gpt-4"
```

## Performance

CommanderChrisNext is built with Rust's performance in mind:

- **Memory Efficient** - Minimal resource usage even during long sessions
- **Fast Startup** - Cold start in milliseconds
- **Optimized Networking** - Asynchronous API interactions
- **Secure** - No unnecessary data persistence

## Contributing

Contributions are welcome! Here's how you can help:

1. **Report Issues** - Help improve the project by reporting bugs or suggesting features
2. **Submit PRs** - Fix bugs, add features, or improve documentation
3. **Spread the Word** - Star the repo and share with others

### Development Setup

```bash
# Clone the repository
git clone https://github.com/markorenic/CommanderChrisNext.git
cd CommanderChrisNext

# Install development dependencies
rustup component add clippy rustfmt

# Set up pre-commit hooks
cp .github/hooks/pre-commit .git/hooks/
chmod +x .git/hooks/pre-commit

# Run tests
cargo test

# Check formatting and lints
cargo fmt -- --check
cargo clippy
```

### Code Style

We follow standard Rust conventions:
- Format code with `rustfmt`
- Follow lints from `clippy`
- Add tests for new features
- Document public APIs

## Architecture

CommanderChrisNext is structured around these core components:

- **CLI** - Command line interface and REPL implementation
- **API Client** - Handles communication with language model providers
- **Configuration** - Manages user preferences and API keys
- **Personalization** - Gathers system information for context

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

<div align="center">
Built with ❤️ by the CommanderChrisNext community
</div>
