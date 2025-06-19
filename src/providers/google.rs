use crate::providers::{AiProvider, AiResponse};
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
    #[serde(rename = "usageMetadata")]
    usage_metadata: Option<UsageMetadata>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ContentResponse,
}

#[derive(Deserialize)]
struct ContentResponse {
    parts: Vec<PartResponse>,
}

#[derive(Deserialize)]
struct PartResponse {
    text: String,
}

#[derive(Deserialize)]
struct UsageMetadata {
    #[serde(rename = "totalTokenCount")]
    total_token_count: Option<u32>,
}

pub struct GoogleProvider {
    client: Client,
    api_key: String,
    model: String,
}

impl GoogleProvider {
    pub fn new(model: String) -> Result<Self> {
        let model = if model == "DEFAULT" {
            "gemini-2.5-flash".to_string()
        } else {
            model
        };

        let api_key = std::env::var("GOOGLE_AI_API_KEY")
            .map_err(|_| anyhow::anyhow!("GOOGLE_AI_API_KEY environment variable not set"))?;

        Ok(Self {
            client: Client::new(),
            api_key,
            model,
        })
    }
}

#[async_trait::async_trait]
impl AiProvider for GoogleProvider {
    async fn query(&self, prompt: &str) -> Result<AiResponse> {
        let start = Instant::now();

        let request_body = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: prompt.to_string(),
                }],
            }],
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let duration = start.elapsed();

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            anyhow::bail!(
                "Google AI API request failed with status {}: {}",
                status,
                text
            );
        }

        let gemini_response: GeminiResponse = response.json().await?;

        let content = gemini_response
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from Google AI"))?;

        let tokens_used = gemini_response
            .usage_metadata
            .and_then(|u| u.total_token_count);

        Ok(AiResponse {
            content,
            tokens_used,
            duration,
        })
    }
}
