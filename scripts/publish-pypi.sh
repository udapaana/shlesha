#!/bin/bash
set -e

# Load environment variables
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

# Ensure Python 3.13 compatibility
export PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1

echo "🐍 Publishing Shlesha to PyPI..."

# Check if we're on a tag
if ! git describe --exact-match --tags HEAD 2>/dev/null; then
    echo "❌ Error: Not on a tagged commit. Please tag your release first."
    echo "   Example: git tag -a v0.1.0-beta.1 -m 'Beta release'"
    exit 1
fi

VERSION=$(git describe --exact-match --tags HEAD)
echo "📦 Version: $VERSION"

# Clean previous builds
echo "🧹 Cleaning previous builds..."
rm -rf target/wheels dist

# Build wheels
echo "🔨 Building wheels with maturin..."
maturin build --release --features python

# For release candidates, use TestPyPI
if [[ $VERSION == *"rc"* ]]; then
    echo "📤 Publishing to TestPyPI (release candidate)..."
    maturin publish \
        --repository-url https://test.pypi.org/legacy/ \
        --username __token__ \
        --password "${TEST_PYPI_API_TOKEN}"
    
    echo "✅ Published to TestPyPI!"
    echo "📥 Install with: pip install -i https://test.pypi.org/simple/ shlesha==$VERSION"
else
    echo "📤 Publishing to PyPI..."
    read -p "Are you sure you want to publish to production PyPI? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        maturin publish \
            --username __token__ \
            --password "${PYPI_API_TOKEN}"
        
        echo "✅ Published to PyPI!"
        echo "📥 Install with: pip install shlesha==$VERSION"
    else
        echo "❌ Publishing cancelled"
        exit 1
    fi
fi