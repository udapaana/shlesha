#!/bin/bash
# Shlesha Development Environment Setup Script
# This script sets up everything needed for local development

set -e  # Exit on any error

echo "🚀 Setting up Shlesha development environment..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]] || [[ ! -f "src/lib.rs" ]]; then
    print_error "Please run this script from the Shlesha project root directory"
    exit 1
fi

print_status "Checking Rust installation..."

# Check if rustup is installed
if command -v rustup &> /dev/null; then
    print_success "rustup is installed"
    RUST_SETUP="rustup"
else
    print_warning "rustup is not installed. Checking for Homebrew Rust..."
    
    if command -v rustc &> /dev/null; then
        RUST_PATH=$(which rustc)
        if [[ "$RUST_PATH" == *"Cellar"* ]]; then
            print_warning "Rust is installed via Homebrew. For WASM development, rustup is recommended."
            print_status "Would you like to:"
            echo "1) Install rustup (recommended for full functionality)"
            echo "2) Continue with Homebrew Rust (limited WASM support)"
            echo "3) Exit and install rustup manually"
            
            read -p "Enter your choice (1-3): " choice
            case $choice in
                1)
                    print_status "Installing rustup..."
                    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                    source $HOME/.cargo/env
                    RUST_SETUP="rustup"
                    ;;
                2)
                    print_warning "Continuing with Homebrew Rust. WASM functionality may be limited."
                    RUST_SETUP="homebrew"
                    ;;
                3)
                    print_status "Please install rustup manually: https://rustup.rs/"
                    exit 0
                    ;;
                *)
                    print_error "Invalid choice"
                    exit 1
                    ;;
            esac
        else
            RUST_SETUP="other"
        fi
    else
        print_error "Rust is not installed. Please install rustup: https://rustup.rs/"
        exit 1
    fi
fi

# Setup WASM target if using rustup
if [[ "$RUST_SETUP" == "rustup" ]]; then
    print_status "Adding WASM target..."
    rustup target add wasm32-unknown-unknown
    print_success "WASM target added"
fi

# Install wasm-pack
print_status "Checking for wasm-pack..."
if ! command -v wasm-pack &> /dev/null; then
    print_status "Installing wasm-pack..."
    if [[ "$RUST_SETUP" == "rustup" ]]; then
        curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    else
        # For non-rustup setups, install binary directly
        curl -L https://github.com/rustwasm/wasm-pack/releases/download/v0.13.1/wasm-pack-v0.13.1-x86_64-apple-darwin.tar.gz | tar -xz
        sudo mv wasm-pack-v0.13.1-x86_64-apple-darwin/wasm-pack /usr/local/bin/
        rm -rf wasm-pack-v0.13.1-x86_64-apple-darwin
    fi
    print_success "wasm-pack installed"
else
    print_success "wasm-pack is already installed"
fi

# Check for uv (modern Python package manager)
print_status "Checking for uv..."
if ! command -v uv &> /dev/null; then
    print_status "Installing uv (modern Python package manager)..."
    if command -v curl &> /dev/null; then
        curl -LsSf https://astral.sh/uv/install.sh | sh
        source $HOME/.cargo/env
    elif command -v brew &> /dev/null; then
        brew install uv
    else
        print_error "Please install uv manually: https://docs.astral.sh/uv/getting-started/installation/"
        exit 1
    fi
    print_success "uv installed"
else
    print_success "uv is already installed"
fi

# Setup Python environment with uv
print_status "Setting up Python environment with uv..."
if [[ ! -f ".python-version" ]]; then
    echo "3.11" > .python-version
fi

# Create/sync virtual environment
uv sync --dev
print_success "Python environment created and dependencies installed"

# Install maturin for building Python bindings
print_status "Installing maturin..."
uv add --dev maturin
print_success "Maturin installed"

# Check for Node.js (optional, for alternative WASM testing)
print_status "Checking for Node.js (optional)..."
if command -v node &> /dev/null; then
    print_success "Node.js is installed"
    HAS_NODE=true
else
    print_warning "Node.js not found. Consider installing for additional WASM testing options."
    HAS_NODE=false
fi

print_success "Development environment setup complete!"
print_status "Next steps:"
echo "  1. Run './scripts/build-all.sh' to build all targets"
echo "  2. Run './scripts/test-all.sh' to run all tests"
echo "  3. Run './scripts/demo-wasm.sh' to start the WASM demo"
echo "  4. Run './scripts/demo-python.sh' to test Python bindings"

# Create .env file with configuration
cat > .env << EOF
# Shlesha Development Configuration
RUST_SETUP=$RUST_SETUP
PYTHON_MANAGER=uv
HAS_NODE=$HAS_NODE
EOF

print_success "Configuration saved to .env"