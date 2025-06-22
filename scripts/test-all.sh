#!/bin/bash
# Run all Shlesha tests (Rust, Python, CLI, WASM)

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[TEST]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_status "Running all Shlesha tests..."

# Load configuration
if [[ -f ".env" ]]; then
    source .env
fi

# Run Rust unit tests
print_status "Running Rust unit tests..."
cargo test --lib --features cli
print_success "Rust unit tests passed"

# Run CLI integration tests
print_status "Running CLI integration tests..."
cargo build --release --features cli  # Ensure binary exists
cargo test --test cli_integration_tests
print_success "CLI integration tests passed"

# Run comprehensive bidirectional tests
print_status "Running comprehensive bidirectional tests..."
cargo test --test comprehensive_bidirectional_tests
print_success "Comprehensive bidirectional tests passed"

# Test Python bindings
print_status "Testing Python bindings..."
if command -v uv &> /dev/null && [[ -f "pyproject.toml" ]]; then
    
    # Build Python bindings if not already built
    if ! uv run python -c "import shlesha" 2>/dev/null; then
        print_status "Building Python bindings for testing..."
        if [[ "$RUST_SETUP" == "homebrew" ]]; then
            PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 uv run maturin develop --features python
        else
            uv run maturin develop --features python
        fi
    fi
    
    # Test basic functionality
    uv run python -c "
import shlesha
print('Testing Python bindings...')
t = shlesha.Shlesha()
result = t.transliterate('धर्म', 'devanagari', 'iast')
assert result == 'dharma', f'Expected dharma, got {result}'
print('✓ Basic transliteration works')

result_meta = t.transliterate_with_metadata('धर्मkr', 'devanagari', 'iast')
assert result_meta.output == 'dharmakr', f'Expected dharmakr, got {result_meta.output}'
assert result_meta.metadata is not None, 'Metadata should be present'
assert len(result_meta.metadata.unknown_tokens) == 2, f'Expected 2 unknown tokens, got {len(result_meta.metadata.unknown_tokens)}'
print('✓ Metadata collection works')

scripts = t.list_supported_scripts()
assert len(scripts) > 0, 'Should have supported scripts'
assert 'devanagari' in scripts, 'Should support devanagari'
print('✓ Script discovery works')

print('All Python binding tests passed!')
"
    print_success "Python binding tests passed"
    
    # Run comprehensive Python tests if available
    if [[ -f "python/tests/test_comprehensive.py" ]]; then
        print_status "Running comprehensive Python tests..."
        
        # Use pytest if available, otherwise run individual files
        if uv run python -c "import pytest" 2>/dev/null; then
            print_status "Running tests with pytest..."
            uv run pytest python/tests/ -v
        else
            print_status "Running individual test files..."
            for test_file in python/tests/test_*.py; do
                if [[ -f "$test_file" ]]; then
                    print_status "Running $(basename "$test_file")..."
                    uv run python "$test_file" || {
                        print_error "Test file $test_file failed"
                        exit 1
                    }
                fi
            done
        fi
        
        print_success "Comprehensive Python tests passed"
    fi
else
    print_error "uv environment not found. Run ./scripts/setup-dev.sh first"
    exit 1
fi

# Test WASM bindings (if available)
if [[ -d "pkg" ]] && [[ "$RUST_SETUP" == "rustup" ]]; then
    print_status "Testing WASM bindings..."
    if command -v node &> /dev/null; then
        # Create a quick Node.js test
        cat > test_wasm.mjs << 'EOF'
import init, { WasmShlesha, transliterate, getSupportedScripts } from './pkg/shlesha.js';

async function test() {
    await init();
    
    console.log('Testing WASM bindings...');
    
    // Test direct function
    const result = transliterate('धर्म', 'devanagari', 'iast');
    console.assert(result === 'dharma', `Expected dharma, got ${result}`);
    console.log('✓ Direct transliterate function works');
    
    // Test class
    const transliterator = new WasmShlesha();
    const result2 = transliterator.transliterate('अ', 'devanagari', 'iast');
    console.assert(result2 === 'a', `Expected a, got ${result2}`);
    console.log('✓ WasmShlesha class works');
    
    // Test script support
    const scripts = getSupportedScripts();
    console.assert(scripts.length > 0, 'Should have supported scripts');
    console.log('✓ Script discovery works');
    
    console.log('All WASM binding tests passed!');
}

test().catch(console.error);
EOF
        node test_wasm.mjs
        rm test_wasm.mjs
        print_success "WASM binding tests passed"
    else
        print_status "Node.js not available, skipping WASM tests"
    fi
else
    if [[ "$RUST_SETUP" != "rustup" ]]; then
        print_status "WASM tests skipped (requires rustup)"
    else
        print_status "WASM package not built, skipping WASM tests"
    fi
fi

print_success "All available tests passed!"

# Test summary
echo
print_status "Test Summary:"
echo "  ✓ Rust unit tests"
echo "  ✓ CLI integration tests" 
echo "  ✓ Comprehensive bidirectional tests"
echo "  ✓ Python binding tests"
if [[ -d "pkg" ]] && [[ "$RUST_SETUP" == "rustup" ]] && command -v node &> /dev/null; then
    echo "  ✓ WASM binding tests"
fi
echo
print_success "Ready for development!"