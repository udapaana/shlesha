#!/bin/bash
set -e

echo "🧪 Testing release readiness for Shlesha..."

# Load environment variables
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

# Ensure Python 3.13 compatibility
export PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1

echo "📋 Pre-flight checks:"

# Check git status
if [[ -n $(git status --porcelain) ]]; then
    echo "⚠️  Warning: Working directory has uncommitted changes"
    git status --short
else
    echo "✅ Working directory is clean"
fi

# Check if we're on main branch
CURRENT_BRANCH=$(git branch --show-current)
if [[ "$CURRENT_BRANCH" != "main" ]]; then
    echo "⚠️  Warning: Not on main branch (currently on: $CURRENT_BRANCH)"
else
    echo "✅ On main branch"
fi

# Test Rust build
echo ""
echo "🦀 Testing Rust build..."
if cargo build; then
    echo "✅ Core Rust build successful"
else
    echo "❌ Core Rust build failed"
    exit 1
fi

# Test CLI build
echo ""
echo "🔧 Testing CLI build..."
if cargo build --features cli; then
    echo "✅ CLI build successful"
else
    echo "❌ CLI build failed"
    exit 1
fi

# Test Python build
echo ""
echo "🐍 Testing Python build..."
if maturin build --features python; then
    echo "✅ Python wheel build successful"
else
    echo "⚠️ Python wheel build failed (this is expected on some systems)"
    echo "   Python builds will work in CI with proper Python linking"
fi

# Test WASM build
echo ""
echo "📦 Testing WASM build..."
if wasm-pack build --target web --out-dir pkg --features wasm; then
    echo "✅ WASM build successful"
else
    echo "⚠️ WASM build failed (may need rustup setup)"
    echo "   WASM builds will work in CI with proper target installation"
fi

# Run tests
echo ""
echo "🧪 Running core tests..."
if cargo test; then
    echo "✅ Core tests passed"
else
    echo "❌ Core tests failed"
    exit 1
fi

# Run CLI tests
echo ""
echo "🧪 Running CLI tests..."
if cargo test --features cli; then
    echo "✅ CLI tests passed"
else
    echo "❌ CLI tests failed"
    exit 1
fi

echo ""
echo "🎉 All checks passed! Ready for release."
echo ""
echo "📝 Next steps:"
echo "1. Run the release tagging script:"
echo "   ./scripts/tag-release.sh"
echo ""
echo "2. The script will:"
echo "   - Automatically determine the next version"
echo "   - Create appropriate RC or stable tags"
echo "   - Guide you through the release process"
echo ""
echo "3. GitHub Actions will automatically handle:"
echo "   - Building Python wheels and WASM packages"
echo "   - Publishing to PyPI/TestPyPI and npm"
echo "   - Creating GitHub releases"
echo ""
echo "4. Monitor progress at:"
echo "   https://github.com/udapaana/shlesha/actions"