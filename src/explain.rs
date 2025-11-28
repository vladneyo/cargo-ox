// src/explain.rs
use anyhow::{Context, Result};
use tokio::process::Command;
use crate::llm::ask_ollama;
use crate::prompts::{SYSTEM_PROMPT, build_explain_prompt};

pub async fn run_explain(cargo_args: Vec<String>, project_path: Option<String>) -> Result<()> {
    // Build `cargo check` command
    let mut cmd = Command::new("cargo");
    cmd.arg("check");
    // Make output less noisy & consistent
    cmd.arg("--color").arg("never");
    
    if let Some(path) = project_path {
        cmd.current_dir(path);
    }

    // pass through any extra args
    cmd.args(&cargo_args);

    // Capture stderr (where Rust errors go)
    let output = cmd.output().await.context("failed to run `cargo check`")?;

    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        if stderr.trim().is_empty() {
            println!("✅ `cargo check` passed with no errors.");
            return Ok(());
        } else {
            // Sometimes warnings go to stderr even on success
            println!("`cargo check` succeeded, but there were warnings.\n");
        }
    } else if stderr.trim().is_empty() {
        println!("`cargo check` failed, but no stderr output was captured.");
        return Ok(());
    }

    let user_prompt = build_explain_prompt(&stderr);
    let answer = ask_ollama(SYSTEM_PROMPT, &user_prompt).await?;

    println!("\n=== ox · explanation ===\n");
    println!("{answer}");

    Ok(())
}