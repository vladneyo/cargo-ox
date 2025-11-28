use anyhow::Result;
use serde::{Deserialize, Serialize};

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
    role: String,
    content: String,
}

pub async fn ask_ollama(system_prompt: &str, user_prompt: &str) -> Result<String> {
    let model = std::env::var("OX_MODEL").unwrap_or_else(|_| "llama3.1".to_string());

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
        stream: false,
    };

    let client = reqwest::Client::new();
    let resp = client
        .post("http://localhost:11434/api/chat")
        .json(&req)
        .send()
        .await?
        .error_for_status()?
        .json::<ChatResponse>()
        .await?;

    Ok(resp.message.content)
}