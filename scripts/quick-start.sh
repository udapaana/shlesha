#!/bin/bash
# Quick start script for new developers

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[QUICK-START]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

clear
echo "🚀 Shlesha Quick Start"
echo "====================="
echo
echo "This script will set up everything you need to start developing with Shlesha."
echo

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]] || [[ ! -f "src/lib.rs" ]]; then
    print_error "Please run this script from the Shlesha project root directory"
    exit 1
fi

print_status "Step 1: Setting up development environment..."
./scripts/setup-dev.sh

echo
print_status "Step 2: Building all targets..."
./scripts/build-all.sh

echo  
print_status "Step 3: Running tests to verify everything works..."
./scripts/test-all.sh

echo
print_success "🎉 Quick start complete!"
echo
echo "What you can do now:"
echo "==================="
echo
echo "1. 🖥️  Test CLI tool:"
echo "   ./target/release/shlesha transliterate --from devanagari --to iast 'धर्म'"
echo
echo "2. 🐍 Try Python bindings:"
echo "   ./scripts/demo-python.sh"
echo
echo "3. 🕸️  Run WASM demo:"
echo "   ./scripts/demo-wasm.sh"
echo
echo "4. 🧪 Run specific tests:"
echo "   cargo test                     # All Rust tests"
echo "   cargo test --test cli_integration_tests  # CLI tests"
echo
echo "5. 📚 Learn more:"
echo "   ./target/release/shlesha --help           # CLI help"
echo "   ./target/release/shlesha scripts          # List supported scripts"
echo
echo "Happy coding! 🎯"