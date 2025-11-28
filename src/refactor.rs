// src/refactor.rs
use anyhow::{Context, Result};
use std::fs;
use crate::llm::ask_ollama;
use crate::prompts::{SYSTEM_PROMPT, build_refactor_prompt};

pub async fn run_refactor(file_path: String) -> Result<()> {
    let content = fs::read_to_string(&file_path)
        .with_context(|| format!("Failed to read file: {file_path}"))?;

    let user_prompt = build_refactor_prompt(&content, &file_path);
    let answer = ask_ollama(SYSTEM_PROMPT, &user_prompt).await?;

    println!("\n=== ox · refactor suggestions ===\n");
    println!("{answer}");

    Ok(())
}