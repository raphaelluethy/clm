# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

CLM is a command-line tool for interacting with multiple LLM providers (OpenAI, Google, Anthropic, Ollama). It provides a unified interface for querying different AI providers with automatic provider selection based on environment variables.

## Architecture

The project follows a provider pattern architecture:

- `src/main.rs`: CLI entry point using clap for argument parsing
- `src/providers/mod.rs`: Defines the `AiProvider` trait and provider factory function
- `src/providers/`: Individual provider implementations (openai.rs, anthropic.rs, google.rs, ollama.rs)

Each provider implements the `AiProvider` trait with a `query` method that returns an `AiResponse` containing the response content, token usage, and request duration.

## Common Commands

### Build and Run
```bash
cargo build
cargo run -- "your prompt here"
```

### Development
```bash
cargo check         # Fast syntax/type checking
cargo clippy        # Linting
cargo fmt           # Code formatting
```

### Testing
```bash
cargo test
```

## Environment Configuration

The tool uses environment variables for provider selection, model configuration, and API keys:

- `CLM_PROVIDER`: Sets the AI provider (google, openai, anthropic, ollama, openrouter). Defaults to "google"
- `CLM_MODEL`: Sets the model to use. Defaults to "gemini-2.5-flash"
- `GOOGLE_AI_API_KEY`: Required for Google provider
- `OPENAI_API_KEY`: Required for OpenAI provider
- `ANTHROPIC_API_KEY`: Required for Anthropic provider
- `OPENROUTER_API_KEY`: Required for OpenRouter provider
- `OLLAMA_BASE_URL`: Optional for Ollama (defaults to "http://localhost:11434")

### Model Defaults
When using the global default model (gemini-2.5-flash), each provider maps to its appropriate default:
- Google: Uses the specified model directly
- OpenAI: Defaults to "gpt-4.1-mini"
- Anthropic: Defaults to "claude-3-5-sonnet-20241022"
- OpenRouter: Defaults to "google/gemini-2.5-flash"
- Ollama: Defaults to "llama3.2"

## Provider Implementation Notes

When adding new providers:
1. Create a new module in `src/providers/`
2. Implement the `AiProvider` trait
3. Add the provider to the factory function in `mod.rs`
4. Handle provider-specific authentication and request/response formats

The `AiResponse` struct tracks content, optional token usage, and request duration for consistent metrics across providers.