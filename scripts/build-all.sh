#!/bin/bash
# Build all Shlesha targets (CLI, Python, WASM)

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[BUILD]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_status "Building all Shlesha targets..."

# Load configuration
if [[ -f ".env" ]]; then
    source .env
fi

# Build CLI (release)
print_status "Building CLI binary..."
cargo build --release --features cli
print_success "CLI binary built: ./target/release/shlesha"

# Build Python bindings
print_status "Building Python bindings..."
if command -v uv &> /dev/null && [[ -f "pyproject.toml" ]]; then
    if [[ "$RUST_SETUP" == "homebrew" ]]; then
        PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 uv run maturin develop --features python
    else
        uv run maturin develop --features python
    fi
    print_success "Python bindings built and installed in uv environment"
else
    print_error "uv environment not found. Run ./scripts/setup-dev.sh first"
    exit 1
fi

# Build WASM
print_status "Building WASM package..."
if command -v wasm-pack &> /dev/null; then
    if [[ "$RUST_SETUP" == "rustup" ]]; then
        wasm-pack build --target web --features wasm --out-dir pkg
        print_success "WASM package built: ./pkg/"
    else
        print_error "WASM build requires rustup. Run with RUST_SETUP=homebrew for limited support:"
        print_status "cargo build --target wasm32-unknown-unknown --features wasm (requires manual target installation)"
    fi
else
    print_error "wasm-pack not found. Run ./scripts/setup-dev.sh first"
    exit 1
fi

print_success "All builds completed successfully!"
print_status "Available artifacts:"
echo "  • CLI: ./target/release/shlesha"
echo "  • Python: Installed in uv environment (uv run python -c 'import shlesha')"
echo "  • WASM: ./pkg/ (for web deployment)"