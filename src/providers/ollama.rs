use crate::providers::{AiProvider, AiResponse};
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
    done: bool,
    eval_count: Option<u32>,
    prompt_eval_count: Option<u32>,
}

pub struct OllamaProvider {
    client: Client,
    base_url: String,
    model: String,
}

impl OllamaProvider {
    pub fn new(model: String) -> Result<Self> {
        let base_url = std::env::var("OLLAMA_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:11434".to_string());

        let model = if model == "DEFAULT" {
            "llama3.2".to_string()
        } else {
            model
        };

        Ok(Self {
            client: Client::new(),
            base_url,
            model,
        })
    }
}

#[async_trait::async_trait]
impl AiProvider for OllamaProvider {
    async fn query(&self, prompt: &str) -> Result<AiResponse> {
        let start = Instant::now();

        let request_body = OllamaRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            stream: false,
        };

        let url = format!("{}/api/generate", self.base_url);

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
            anyhow::bail!("Ollama API request failed with status {}: {}", status, text);
        }

        let ollama_response: OllamaResponse = response.json().await?;

        if !ollama_response.done {
            anyhow::bail!("Incomplete response from Ollama");
        }

        let tokens_used = ollama_response.eval_count.and_then(|eval| {
            ollama_response
                .prompt_eval_count
                .map(|prompt| eval + prompt)
        });

        Ok(AiResponse {
            content: ollama_response.response,
            tokens_used,
            duration,
            model: self.model.clone(),
            provider: "ollama".to_string(),
        })
    }
}
