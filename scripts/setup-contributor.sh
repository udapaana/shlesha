#!/bin/bash
set -e

echo "🚀 Setting up Shlesha development environment..."

# Check for required tools
echo "📋 Checking requirements..."

if ! command -v rustc &> /dev/null; then
    echo "❌ Rust not found. Please install from https://rustup.rs/"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install from https://rustup.rs/"
    exit 1
fi

echo "✅ Rust and Cargo found"

# Install Rust components
echo "📦 Installing Rust components..."
rustup component add rustfmt clippy

# Install development tools
echo "🔧 Installing development tools..."
cargo install cargo-audit cargo-tarpaulin wasm-pack || true

# Install pre-commit
if command -v pip &> /dev/null; then
    echo "📝 Installing pre-commit..."
    pip install pre-commit
    pre-commit install
    echo "✅ Pre-commit hooks installed"
else
    echo "⚠️  pip not found. Skipping pre-commit installation."
fi

# Install Python development dependencies
if command -v python3 &> /dev/null; then
    echo "🐍 Installing Python development dependencies..."
    pip install maturin pytest
else
    echo "⚠️  Python not found. Skipping Python setup."
fi

# Install Node.js dependencies
if command -v npm &> /dev/null; then
    echo "📦 Installing Node.js dependencies..."
    npm install
else
    echo "⚠️  npm not found. Skipping Node.js setup."
fi

# Build the project
echo "🔨 Building the project..."
cargo build --all-features

# Run tests
echo "🧪 Running tests..."
cargo test

echo "✅ Development environment setup complete!"
echo ""
echo "📚 Quick commands:"
echo "  make test    - Run all tests"
echo "  make fmt     - Format code"
echo "  make lint    - Run lints"
echo "  make docs    - Build documentation"
echo ""
echo "Happy coding! 🎉"