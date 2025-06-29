#!/bin/bash
set -e

# Load environment variables
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

echo "📦 Publishing Shlesha WASM to npm..."

# Check if we're on a tag
if ! git describe --exact-match --tags HEAD 2>/dev/null; then
    echo "❌ Error: Not on a tagged commit. Please tag your release first."
    echo "   Example: git tag -a v0.1.0-beta.1 -m 'Beta release'"
    exit 1
fi

VERSION=$(git describe --exact-match --tags HEAD | sed 's/^v//')
echo "📦 Version: $VERSION"

# Clean previous builds
echo "🧹 Cleaning previous builds..."
rm -rf pkg pkg-node

# Build WASM package
echo "🔨 Building WASM package..."
wasm-pack build --target web --out-dir pkg --features wasm

# Update version in package.json
echo "📝 Updating package version..."
cd pkg
npm version $VERSION --no-git-tag-version --allow-same-version

# For release candidates, add rc tag
if [[ $VERSION == *"rc"* ]]; then
    echo "📤 Publishing to npm with rc tag..."
    npm publish --access public --tag rc
    
    echo "✅ Published to npm (rc)!"
    echo "📥 Install with: npm install shlesha-wasm@rc"
else
    echo "📤 Publishing to npm..."
    read -p "Are you sure you want to publish to production npm? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        npm publish --access public
        
        echo "✅ Published to npm!"
        echo "📥 Install with: npm install shlesha-wasm@$VERSION"
    else
        echo "❌ Publishing cancelled"
        exit 1
    fi
fi