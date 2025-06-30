#!/bin/bash

# Pre-release verification script for Shlesha
# This script ensures all tests pass and builds work before releasing

set -e

echo "🔍 Pre-Release Verification for Shlesha"
echo "======================================="

# Function to print status
print_status() {
    if [ $? -eq 0 ]; then
        echo "✅ $1"
    else
        echo "❌ $1"
        exit 1
    fi
}

# 1. Rust tests
echo ""
echo "1️⃣ Running Rust tests..."
cargo test --lib
print_status "Rust library tests"

cargo test --bins 
print_status "Rust binary tests"

# 2. Rust builds
echo ""
echo "2️⃣ Testing Rust builds..."
cargo build --release
print_status "Release build"

cargo build --release --features cli
print_status "CLI build"

# 3. Python builds and tests
echo ""
echo "3️⃣ Testing Python builds..."
if command -v maturin &> /dev/null; then
    maturin build --release
    print_status "Python wheel build"
    
    # Install and test wheel locally
    if [ -d "target/wheels" ]; then
        pip install --force-reinstall target/wheels/*.whl
        print_status "Python wheel installation"
        
        # Run Python tests
        python -c "import shlesha; print('✅ Python import works')"
        
        # Basic functionality test
        python -c "
import shlesha
t = shlesha.Shlesha()
result = t.transliterate('namaste', 'iast', 'devanagari')
expected = 'नमस्ते'
if expected in result or result == expected:
    print('✅ Basic Python transliteration works')
else:
    print(f'❌ Python transliteration failed: expected {expected}, got {result}')
    exit(1)
"
        print_status "Python functionality test"
    else
        echo "⚠️ No wheels found to test"
    fi
else
    echo "⚠️ Maturin not found, skipping Python tests"
fi

# 4. WASM builds
echo ""
echo "4️⃣ Testing WASM builds..."
if command -v wasm-pack &> /dev/null; then
    if rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
        wasm-pack build --target web --out-dir pkg-web --release
        print_status "WASM web build"
        
        wasm-pack build --target nodejs --out-dir pkg-node --release  
        print_status "WASM Node.js build"
    else
        echo "⚠️ WASM target not installed, skipping WASM tests"
    fi
else
    echo "⚠️ wasm-pack not found, skipping WASM tests"
fi

# 5. Critical functionality tests
echo ""
echo "5️⃣ Running critical functionality tests..."

# Test core conversions
echo "Testing core conversions..."
./target/release/shlesha transliterate --from devanagari --to iast "धर्म" > /tmp/test_output.txt
if grep -q "dharma" /tmp/test_output.txt; then
    echo "✅ Devanagari → IAST works"
else
    echo "❌ Devanagari → IAST failed"
    cat /tmp/test_output.txt
    exit 1
fi

# Test reverse conversion
./target/release/shlesha transliterate --from iast --to devanagari "dharma" > /tmp/test_output.txt
if grep -q "धर्म" /tmp/test_output.txt; then
    echo "✅ IAST → Devanagari works"
else
    echo "❌ IAST → Devanagari failed"
    echo "Expected: धर्म"
    echo "Got: $(cat /tmp/test_output.txt)"
    exit 1
fi

# Test cross-hub conversion  
./target/release/shlesha transliterate --from slp1 --to bengali "dharma" > /tmp/test_output.txt
if grep -q "ধর্ম" /tmp/test_output.txt; then
    echo "✅ Cross-hub conversion works"
else
    echo "❌ Cross-hub conversion failed"
    cat /tmp/test_output.txt
    exit 1
fi

# 6. Version consistency check
echo ""
echo "6️⃣ Checking version consistency..."
CARGO_VERSION=$(grep -E '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
PACKAGE_VERSION=$(grep -E '"version":' package.json | sed 's/.*"version": "\(.*\)".*/\1/')

echo "Cargo.toml version: $CARGO_VERSION"
echo "package.json version: $PACKAGE_VERSION"

if [ "$CARGO_VERSION" = "$PACKAGE_VERSION" ]; then
    echo "✅ Version consistency check"
else
    echo "❌ Version mismatch between Cargo.toml and package.json"
    exit 1
fi

# 7. Git status check
echo ""
echo "7️⃣ Checking git status..."
if git diff --quiet && git diff --cached --quiet; then
    echo "✅ Working directory is clean"
else
    echo "❌ Working directory has uncommitted changes"
    git status --short
    exit 1
fi

echo ""
echo "🎉 All pre-release checks passed!"
echo "Ready to create release tag v$CARGO_VERSION"
echo ""
echo "Next steps:"
echo "  git tag v$CARGO_VERSION"
echo "  git push origin v$CARGO_VERSION"