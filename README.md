# CLM - Command Line LLM Tool

A unified command-line interface for interacting with multiple Large Language Model providers including OpenAI, Anthropic, Google, and Ollama.

## Features

- **Multi-provider support**: Switch between OpenAI, Anthropic, Google, and Ollama
- **Simple CLI interface**: Just pass your prompt as arguments
- **Usage tracking**: See token usage, response time, model, and provider for each query
- **Environment-based configuration**: Easy provider switching via environment variables

## Installation

### Prerequisites
- Rust 1.70+ (uses 2024 edition)

### Build from source
```bash
git clone <repository-url>
cd clm
cargo build --release
```

The binary will be available at `target/release/clm`.

### Install globally
```bash
make install # or `cargo install --path .`
```

### Uninstall
```bash
make uninstall # or `cargo uninstall clm`
```


## Usage

### Basic usage
```bash
clm What is the capital of France
clm Explain quantum computing in simple terms
```

### Multi-word prompts using punctuation with quotes
```bash
clm "How do I implement a binary search tree in Rust?"
```

### Response Format
Each response includes metadata at the bottom showing:
```
[Tokens: 150 | Time: 2.34s | Model: gemini-2.5-flash | Provider: google]
```

This provides visibility into:
- **Tokens**: Number of tokens used (when available from the provider)
- **Time**: Response time in seconds
- **Model**: The specific model that processed the request
- **Provider**: Which AI provider was used

## Configuration

CLM uses environment variables for configuration:

### Provider Selection
Set the `CLM_PROVIDER` environment variable to choose your provider:

```bash
export CLM_PROVIDER=google     # Default
export CLM_PROVIDER=openai
export CLM_PROVIDER=anthropic
export CLM_PROVIDER=openrouter
export CLM_PROVIDER=ollama
```

### API Keys
Each provider requires its respective API key:

```bash
# OpenAI
export OPENAI_API_KEY="your-openai-api-key"

# Anthropic
export ANTHROPIC_API_KEY="your-anthropic-api-key"

# Google
export GOOGLE_AI_API_KEY="your-google-api-key"

# OpenRouter
export OPENROUTER_API_KEY="your-openrouter-api-key"

# Ollama (runs locally, no API key needed)
```

### Model Configuration
Set the `CLM_MODEL` environment variable to specify which model to use:

```bash
# Global model setting (applies to all providers)
export CLM_MODEL="gemini-2.5-flash"      # Default
export CLM_MODEL="gpt-4.1-mini"          # For OpenAI
export CLM_MODEL="claude-4-sonnet"  # For Anthropic  
export CLM_MODEL="google/gemini-2.5-flash"            # For OpenRouter
export CLM_MODEL="llama3.2"              # For Ollama

# Ollama base URL (optional)
export OLLAMA_BASE_URL="http://localhost:11434"  # Default
```

### Provider-Specific Defaults
When using the global default model (gemini-2.5-flash), each provider maps to its appropriate default:
- **Google**: Uses the specified model directly
- **OpenAI**: Defaults to "gpt-4.1-mini"
- **Anthropic**: Defaults to "claude-4-sonnet"
- **OpenRouter**: Defaults to "google/gemini-2.5-flash"
- **Ollama**: Defaults to "llama3.2"

## Project Structure

```
src/
├── main.rs              # CLI entry point and argument parsing
└── providers/
    ├── mod.rs           # Provider trait and factory
    ├── openai.rs        # OpenAI GPT integration
    ├── anthropic.rs     # Anthropic Claude integration
    ├── google.rs        # Google AI integration
    └── ollama.rs        # Ollama local model integration
```

## Contributions
This project was partially enhanced with ClaudeCode.
