use crate::providers::{AiProvider, AiResponse};
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Serialize)]
struct OpenAiRequest {
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
struct OpenAiResponse {
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Deserialize)]
struct Usage {
    total_tokens: u32,
}

pub struct OpenAiProvider {
    client: Client,
    api_key: String,
    model: String,
}

impl OpenAiProvider {
    pub fn new(model: String) -> Result<Self> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| anyhow::anyhow!("OPENAI_API_KEY environment variable not set"))?;

        let model = if model == "DEFAULT" {
            "gpt-4.1-mini".to_string()
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
impl AiProvider for OpenAiProvider {
    async fn query(&self, prompt: &str) -> Result<AiResponse> {
        let start = Instant::now();

        let request_body = OpenAiRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let duration = start.elapsed();

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            anyhow::bail!("OpenAI API request failed with status {}: {}", status, text);
        }

        let openai_response: OpenAiResponse = response.json().await?;

        let content = openai_response
            .choices
            .first()
            .ok_or_else(|| anyhow::anyhow!("No response from OpenAI"))?
            .message
            .content
            .clone();

        Ok(AiResponse {
            content,
            tokens_used: Some(openai_response.usage.total_tokens),
            duration,
        })
    }
}
