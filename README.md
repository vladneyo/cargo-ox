# cargo-ox 🐂

**cargo-ox** is a Cargo subcommand that uses a local LLM (via [Ollama](https://ollama.com/)) to help you develop Rust code. It can explain compilation errors and suggest refactors.

## Prerequisites

- **Rust**: Ensure you have Rust and Cargo installed.
- **Ollama**: You need Ollama installed and running locally.

## Setup

We provide a setup script to automatically install Ollama (if missing), start the server, and pull the required model.

1.  Run the setup script:
    ```bash
    ./setup_ollama.sh
    ```
    This will prepare your environment and ensure the default model (`llama4:scout`) is available.

## Installation

To install `cargo-ox` locally:

```bash
cargo install --path .
```

## Usage

### Explain Compilation Errors

Run `cargo ox explain` to run `cargo check`, capture any errors, and get an AI explanation with fix suggestions.

```bash
cargo ox explain
```

You can also pass arguments to `cargo check`:

```bash
cargo ox explain --features my-feature
```

### Refactor Code

Run `cargo ox refactor` to get AI suggestions for improving a specific file.

```bash
cargo ox refactor --file src/main.rs
```

## Configuration

By default, `cargo-ox` uses the `llama4:scout` model. You can override this by setting the `OX_MODEL` environment variable:

```bash
export OX_MODEL=llama3
cargo ox explain
```
