use clap::{Parser, Subcommand};
use anyhow::Result;

mod llm;
mod explain;
mod refactor;
mod prompts;



#[derive(Parser)]
#[command(
    name = "cargo-ox",
    version,
    about = "Cargo subcommand: AI helper for Rust using Ollama"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run cargo check, capture errors, and explain them
    Explain {
        /// Path to the project directory
        #[arg(short, long)]
        project: Option<String>,

        /// Optional: pass extra args to `cargo check` (e.g. --features xyz)
        #[arg(trailing_var_arg = true)]
        cargo_args: Vec<String>,
    },

    /// Suggest refactors for a Rust file
    Refactor {
        /// Path to a .rs file
        #[arg(short, long)]
        file: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Explain { cargo_args, project } => {
            explain::run_explain(cargo_args, project).await?;
        }
        Commands::Refactor { file } => {
            refactor::run_refactor(file).await?;
        }
    }

    Ok(())
}