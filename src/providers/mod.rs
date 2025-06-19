use anyhow::Result;
use std::time::Duration;

pub mod anthropic;
pub mod google;
pub mod ollama;
pub mod openai;
pub mod openrouter;

#[derive(Debug, Clone)]
pub struct AiResponse {
    pub content: String,
    pub tokens_used: Option<u32>,
    pub duration: Duration,
}

#[async_trait::async_trait]
pub trait AiProvider {
    async fn query(&self, prompt: &str) -> Result<AiResponse>;
}

pub fn get_provider() -> Result<Box<dyn AiProvider + Send + Sync>> {
    let provider = std::env::var("CLM_PROVIDER")
        .unwrap_or_else(|_| "google".to_string())
        .to_lowercase();

    let model = std::env::var("CLM_MODEL").unwrap_or_else(|_| "DEFAULT".to_string());

    match provider.as_str() {
        "openai" => Ok(Box::new(openai::OpenAiProvider::new(model)?)),
        "google" => Ok(Box::new(google::GoogleProvider::new(model)?)),
        "anthropic" => Ok(Box::new(anthropic::AnthropicProvider::new(model)?)),
        "ollama" => Ok(Box::new(ollama::OllamaProvider::new(model)?)),
        "openrouter" => Ok(Box::new(openrouter::OpenRouterProvider::new(model)?)),
        _ => anyhow::bail!("Unsupported provider: {}", provider),
    }
}
