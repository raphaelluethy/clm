use crate::providers::{AiProvider, AiResponse};
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Serialize)]
struct OpenRouterRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct MessageResponse {
    content: String,
}

#[derive(Deserialize)]
struct OpenRouterResponse {
    choices: Vec<Choice>,
    usage: Option<Usage>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Deserialize)]
struct Usage {
    total_tokens: u32,
}

pub struct OpenRouterProvider {
    client: Client,
    api_key: String,
    model: String,
}

impl OpenRouterProvider {
    pub fn new(model: String) -> Result<Self> {
        let api_key = std::env::var("OPENROUTER_API_KEY")
            .map_err(|_| anyhow::anyhow!("OPENROUTER_API_KEY environment variable not set"))?;

        let model = if model == "DEFAULT" {
            "google/gemini-2.5-flash".to_string()
        } else {
            model
        };

        Ok(Self {
            client: Client::new(),
            api_key,
            model,
        })
    }
}

#[async_trait::async_trait]
impl AiProvider for OpenRouterProvider {
    async fn query(&self, prompt: &str) -> Result<AiResponse> {
        let start = Instant::now();

        let request_body = OpenRouterRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
        };

        let response = self
            .client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let duration = start.elapsed();

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            anyhow::bail!(
                "OpenRouter API request failed with status {}: {}",
                status,
                text
            );
        }

        let openrouter_response: OpenRouterResponse = response.json().await?;

        let content = openrouter_response
            .choices
            .first()
            .ok_or_else(|| anyhow::anyhow!("No response from OpenRouter"))?
            .message
            .content
            .clone();

        let tokens_used = openrouter_response.usage.map(|u| u.total_tokens);

        Ok(AiResponse {
            content,
            tokens_used,
            duration,
            model: self.model.clone(),
            provider: "openrouter".to_string(),
        })
    }
}
