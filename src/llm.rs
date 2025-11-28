use anyhow::Result;
use serde::{Deserialize, Serialize};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use std::io::{self, Write};

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
}

#[derive(Deserialize)]
struct ChatResponse {
    message: ChatResponseMessage,
}

#[derive(Deserialize)]
struct ChatResponseMessage {
    #[allow(dead_code)]
    role: String,
    content: String,
}

pub async fn ask_ollama(system_prompt: &str, user_prompt: &str) -> Result<String> {
    let model = std::env::var("OX_MODEL").unwrap_or_else(|_| "llama4:scout".to_string());

    let client = reqwest::Client::new();
    
    // Pre-flight check
    check_ollama(&client, &model).await?;

    let req = ChatRequest {
        model,
        messages: vec![
            ChatMessage {
                role: "system".into(),
                content: system_prompt.into(),
            },
            ChatMessage {
                role: "user".into(),
                content: user_prompt.into(),
            },
        ],
        stream: true,
    };

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.set_message("Thinking...");
    spinner.enable_steady_tick(Duration::from_millis(100));

    let mut resp = client
        .post("http://localhost:11434/api/chat")
        .json(&req)
        .send()
        .await?
        .error_for_status()?;

    let mut full_response = String::new();

    // Stop spinner before streaming starts
    spinner.finish_and_clear();

    while let Some(chunk) = resp.chunk().await? {
        let s = String::from_utf8_lossy(&chunk);
        
        // Ollama sends multiple JSON objects in one chunk sometimes
        for line in s.lines() {
            if line.trim().is_empty() { continue; }
            if let Ok(part) = serde_json::from_str::<ChatResponse>(line) {
                print!("{}", part.message.content);
                io::stdout().flush()?;
                full_response.push_str(&part.message.content);
            }
        }
    }

    Ok(full_response)
}

#[derive(Deserialize)]
struct TagsResponse {
    models: Vec<ModelInfo>,
}

#[derive(Deserialize)]
struct ModelInfo {
    name: String,
}

async fn check_ollama(client: &reqwest::Client, model: &str) -> Result<()> {
    // 1. Check if Ollama is running
    let resp = client.get("http://localhost:11434/api/tags").send().await;

    let resp = match resp {
        Ok(r) => r,
        Err(_) => {
            anyhow::bail!("Could not connect to Ollama at http://localhost:11434. Is it running?");
        }
    };

    if !resp.status().is_success() {
        anyhow::bail!("Ollama returned an error: {}", resp.status());
    }

    // 2. Check if the model exists
    let tags: TagsResponse = resp.json().await?;
    let model_exists = tags.models.iter().any(|m| m.name.starts_with(model));

    if !model_exists {
        anyhow::bail!(
            "Model '{}' not found in Ollama. Available models: {}",
            model,
            tags.models
                .iter()
                .map(|m| m.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    Ok(())
}