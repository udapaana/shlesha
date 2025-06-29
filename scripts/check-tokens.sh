#!/bin/bash

# Load environment variables
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
else
    echo "❌ .env file not found. Copy .env.example to .env and fill in your tokens."
    exit 1
fi

echo "🔐 Checking publishing tokens..."
echo ""

# Check PyPI tokens
if [ -z "$TEST_PYPI_API_TOKEN" ]; then
    echo "❌ TEST_PYPI_API_TOKEN is not set"
    echo "   Get one from: https://test.pypi.org/manage/account/token/"
else
    echo "✅ TEST_PYPI_API_TOKEN is set"
fi

if [ -z "$PYPI_API_TOKEN" ]; then
    echo "⚠️  PYPI_API_TOKEN is not set (optional for production releases)"
    echo "   Get one from: https://pypi.org/manage/account/token/"
else
    echo "✅ PYPI_API_TOKEN is set"
fi

echo ""

# Check npm token
if [ -z "$NPM_TOKEN" ]; then
    echo "❌ NPM_TOKEN is not set"
    echo "   Get one from: https://www.npmjs.com/settings/YOUR_USERNAME/tokens"
else
    echo "✅ NPM_TOKEN is set"
fi

echo ""

# Check crates.io token
if [ -z "$CRATES_TOKEN" ]; then
    echo "⚠️  CRATES_TOKEN is not set (optional)"
    echo "   Get one from: https://crates.io/settings/tokens"
else
    echo "✅ CRATES_TOKEN is set"
fi

echo ""

# Check GitHub token
if [ -z "$GITHUB_TOKEN" ]; then
    echo "⚠️  GITHUB_TOKEN is not set (usually automatic in GitHub Actions)"
    echo "   Get one from: https://github.com/settings/tokens"
else
    echo "✅ GITHUB_TOKEN is set"
fi

echo ""

# Summary
if [ -z "$NPM_TOKEN" ]; then
    echo "❌ NPM_TOKEN is missing for release candidate publishing."
    echo "   Please set NPM_TOKEN in your .env file."
    exit 1
else
    echo "✅ All tokens for release candidates are configured!"
    echo ""
    echo "📝 Next steps for release candidate:"
    echo "   1. Create an RC tag: git tag -a v0.1.0-rc1 -m 'Release candidate'"
    echo "   2. Push the tag: git push origin v0.1.0-rc1"
    echo "   3. GitHub Actions will auto-publish to Test PyPI and npm with 'rc' tag"
    echo ""
    echo "📝 For stable release:"
    echo "   1. Create a stable tag: git tag -a v0.1.0 -m 'Stable release'"
    echo "   2. Push the tag: git push origin v0.1.0"
    echo "   3. GitHub Actions will auto-publish to PyPI and npm"
fi