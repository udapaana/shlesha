#!/bin/bash
# Start WASM demo with local server

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[DEMO]${NC} $1"
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

print_status "Starting Shlesha WASM demo..."

# Load configuration
if [[ -f ".env" ]]; then
    source .env
fi

# Check if WASM package exists
if [[ ! -d "pkg" ]]; then
    print_status "WASM package not found. Building..."
    if [[ "$RUST_SETUP" == "rustup" ]]; then
        ./scripts/build-all.sh
    else
        print_error "WASM build requires rustup for automatic setup."
        print_status "For Homebrew Rust, you can try manual setup:"
        echo "1. Install WASM target: rustup target add wasm32-unknown-unknown"
        echo "2. Build WASM: wasm-pack build --target web --features wasm --out-dir pkg"
        echo "3. Run this script again"
        exit 1
    fi
fi

# Check if demo.html exists
if [[ ! -f "examples/demo.html" ]]; then
    print_error "examples/demo.html not found. Please ensure you're in the project root."
    exit 1
fi

# Find available Python command (prefer uv's Python)
PYTHON_CMD=""
if command -v uv &> /dev/null && [[ -f "pyproject.toml" ]]; then
    PYTHON_CMD="uv run python"
    print_status "Using uv's Python for local server"
elif command -v python3 &> /dev/null; then
    PYTHON_CMD="python3"
elif command -v python &> /dev/null && python --version | grep -q "Python 3"; then
    PYTHON_CMD="python"
else
    print_error "Python 3 not found. Please install Python 3 or run ./scripts/setup-dev.sh"
    exit 1
fi

# Find an available port
PORT=8000
while netstat -an | grep -q ":$PORT "; do
    PORT=$((PORT + 1))
done

print_status "Starting local server on port $PORT..."
print_success "WASM demo will be available at: http://localhost:$PORT/examples/demo.html"
print_status "Press Ctrl+C to stop the server"

# Start server and open browser
if command -v open &> /dev/null; then
    # macOS
    (sleep 2 && open "http://localhost:$PORT/examples/demo.html") &
elif command -v xdg-open &> /dev/null; then
    # Linux
    (sleep 2 && xdg-open "http://localhost:$PORT/examples/demo.html") &
elif command -v start &> /dev/null; then
    # Windows
    (sleep 2 && start "http://localhost:$PORT/examples/demo.html") &
fi

# Start the server
$PYTHON_CMD -m http.server $PORT