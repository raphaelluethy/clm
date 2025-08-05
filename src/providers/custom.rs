use crate::providers::{AiProvider, AiResponse};
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Serialize)]
struct CustomProviderRequest {
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
struct CustomProviderResponse {
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

pub struct CustomProvider {
    client: Client,
    api_key: String,
    model: String,
    api_url: String,
    provider_name: String,
}

impl CustomProvider {
    pub fn new(model: String) -> Result<Self> {
        let api_key = std::env::var("CUSTOM_PROVIDER_API_KEY")
            .map_err(|_| anyhow::anyhow!("CUSTOM_PROVIDER_API_KEY environment variable not set"))?;

        let model = if model == "DEFAULT" {
            "google/gemini-2.5-flash".to_string()
        } else {
            model
        };

        let env_api_url = std::env::var("CUSTOM_PROVIDER_API_URL")
            .map_err(|_| anyhow::anyhow!("CUSTOM_PROVIDER_API_URL environment variable not set"))?;

        let provider_name =
            std::env::var("CUSTOM_PROVIDER_NAME").unwrap_or_else(|_| "Custom Provider".to_string());

        Ok(Self {
            client: Client::new(),
            api_key,
            model,
            api_url: env_api_url,
            provider_name,
        })
    }
}

#[async_trait::async_trait]
impl AiProvider for CustomProvider {
    async fn query(&self, prompt: &str) -> Result<AiResponse> {
        let start = Instant::now();

        let request_body = CustomProviderRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
        };

        let response = self
            .client
            .post(&self.api_url)
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
                "{} Provider API request failed with status {}: {}",
                self.provider_name,
                status,
                text
            );
        }

        let custom_respone: CustomProviderResponse = response.json().await?;

        let content = custom_respone
            .choices
            .first()
            .ok_or_else(|| anyhow::anyhow!("No response from {} provider", self.provider_name))?
            .message
            .content
            .clone();

        let tokens_used = custom_respone.usage.map(|u| u.total_tokens);

        Ok(AiResponse {
            content,
            tokens_used,
            duration,
            model: self.model.clone(),
            provider: self.provider_name.clone(),
        })
    }
}
