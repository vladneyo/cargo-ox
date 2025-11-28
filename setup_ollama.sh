#!/bin/bash
set -e

# Extract model name from Rust configuration
MODEL_NAME=$(grep 'const DEFAULT_MODEL' src/configuration.rs | cut -d '"' -f 2)

if [ -z "$MODEL_NAME" ]; then
    echo "Error: Could not extract DEFAULT_MODEL from src/configuration.rs"
    exit 1
fi

echo "=== Cargo-Ex Setup: Ollama & Model ==="

# 1. Check if Ollama is installed
if ! command -v ollama &> /dev/null; then
    echo ">> Ollama not found. Installing..."
    # Install Ollama using the official script
    curl -fsSL https://ollama.com/install.sh | sh
else
    echo ">> Ollama is already installed."
fi

# 2. Check if Ollama server is running
if ! pgrep -x "ollama" > /dev/null; then
    echo ">> Ollama server is not running. Starting it in the background..."
    ollama serve &
    OLLAMA_PID=$!
    echo ">> Waiting for Ollama to start..."
    # Wait for the server to be responsive
    while ! curl -s http://localhost:11434/api/tags > /dev/null; do
        sleep 1
    done
    echo ">> Ollama started."
else
    echo ">> Ollama server is running."
fi

# 3. Check if the model exists
echo ">> Checking for model '$MODEL_NAME'..."
if ! ollama list | grep -q "$MODEL_NAME"; then
    echo ">> Model '$MODEL_NAME' not found. Pulling..."
    ollama pull "$MODEL_NAME"
    echo ">> Model pulled successfully."
else
    echo ">> Model '$MODEL_NAME' is already available."
fi

echo "=== Setup Complete ==="
echo "You can now run: cargo run -- explain"
