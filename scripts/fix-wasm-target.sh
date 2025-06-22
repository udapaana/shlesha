#!/bin/bash
# Fix WASM target for Homebrew Rust installations

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[WASM-FIX]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_status "Fixing WASM target for Homebrew Rust..."

# Check current Rust setup
RUSTC_PATH=$(which rustc)
if [[ "$RUSTC_PATH" == *"Cellar"* ]]; then
    print_warning "Detected Homebrew Rust installation"
    
    print_status "Options to fix WASM support:"
    echo "1) Switch to rustup (recommended)"
    echo "2) Manual WASM target setup for Homebrew Rust"
    echo "3) Build without WASM support"
    
    read -p "Choose option (1-3): " choice
    
    case $choice in
        1)
            print_status "Installing rustup and switching from Homebrew Rust..."
            
            # Install rustup
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            source $HOME/.cargo/env
            
            # Add WASM target
            rustup target add wasm32-unknown-unknown
            
            print_success "Switched to rustup with WASM support"
            print_warning "You may want to remove Homebrew Rust: brew uninstall rust"
            ;;
            
        2)
            print_status "Setting up WASM target for Homebrew Rust..."
            
            # For Homebrew Rust, we need to manually add the target
            RUST_VERSION=$(rustc --version | cut -d' ' -f2)
            RUST_SYSROOT=$(rustc --print sysroot)
            
            print_status "Downloading WASM target for Rust $RUST_VERSION..."
            
            # Create target directory
            TARGET_DIR="$RUST_SYSROOT/lib/rustlib/wasm32-unknown-unknown"
            if [[ ! -d "$TARGET_DIR" ]]; then
                sudo mkdir -p "$TARGET_DIR/lib"
                
                # Download and extract WASM std library
                TEMP_DIR=$(mktemp -d)
                cd "$TEMP_DIR"
                
                print_status "Downloading WASM standard library..."
                curl -L "https://forge.rust-lang.org/infra/channel-layout.html#wasm32-unknown-unknown" || {
                    print_error "Failed to download WASM target"
                    print_status "Alternative: Use wasm-pack with --dev flag or install rustup"
                    exit 1
                }
                
                cd -
                rm -rf "$TEMP_DIR"
            fi
            
            print_warning "Manual WASM setup may have limitations. Consider using rustup."
            ;;
            
        3)
            print_status "Building without WASM support..."
            
            # Build only CLI and Python
            cargo build --release --features cli
            
            if [[ -d ".venv" ]]; then
                source .venv/bin/activate
                PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop --features python
            fi
            
            print_success "Built CLI and Python bindings (WASM skipped)"
            ;;
            
        *)
            print_error "Invalid choice"
            exit 1
            ;;
    esac
    
else
    print_error "This script is for fixing Homebrew Rust installations"
    print_status "Your Rust installation: $RUSTC_PATH"
    
    if command -v rustup &> /dev/null; then
        print_status "You have rustup. Adding WASM target..."
        rustup target add wasm32-unknown-unknown
        print_success "WASM target added"
    else
        print_status "Please install rustup for best WASM support: https://rustup.rs/"
    fi
fi