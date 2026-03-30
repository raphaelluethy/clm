use crate::providers::{AiProvider, AiResponse};
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning_effort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_completion_tokens: Option<u32>,
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
    reasoning_effort: Option<String>,
    max_completion_tokens: Option<u32>,
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

        let reasoning_effort = std::env::var("OPENAI_REASONING_EFFORT")
            .ok()
            .map(|v| v.to_lowercase())
            .and_then(|v| match v.as_str() {
                "minimal" | "low" | "medium" | "high" => Some(v),
                _ => None,
            });

        let max_completion_tokens = std::env::var("OPENAI_MAX_COMPLETION_TOKENS")
            .ok()
            .and_then(|v| v.parse::<u32>().ok());

        Ok(Self {
            client: Client::new(),
            api_key,
            model,
            reasoning_effort,
            max_completion_tokens,
        })
    }
}

#[async_trait::async_trait]
impl AiProvider for OpenAiProvider {
    async fn query(&self, prompt: &str) -> Result<AiResponse> {
        let start = Instant::now();

        let supports_reasoning = self.model.to_lowercase().starts_with('o');

        let request_body = OpenAiRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            reasoning_effort: if supports_reasoning {
                self.reasoning_effort.clone()
            } else {
                None
            },
            max_completion_tokens: if supports_reasoning {
                self.max_completion_tokens
            } else {
                None
            },
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
            model: self.model.clone(),
            provider: "openai".to_string(),
        })
    }
}
