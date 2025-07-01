#!/bin/bash
set -e

# Shlesha Integration Test Runner
# This script runs comprehensive tests across multiple environments

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "🚀 Starting Shlesha Integration Tests"
echo "===================================="

# Test environments to run
ENVIRONMENTS=("ubuntu" "archlinux" "colab")

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Results tracking
RESULTS=()
FAILED=0

# Function to run tests in a specific environment
run_environment_test() {
    local env=$1
    echo ""
    echo -e "${YELLOW}🧪 Testing environment: $env${NC}"
    echo "----------------------------------------"
    
    # Build the Docker image
    echo "🔨 Building Docker image for $env..."
    if docker build -f "Dockerfile.$env" -t "shlesha-test-$env" .; then
        echo "✅ Docker image built successfully"
    else
        echo -e "${RED}❌ Failed to build Docker image for $env${NC}"
        RESULTS+=("$env: BUILD_FAILED")
        ((FAILED++))
        return 1
    fi
    
    # Run the tests
    echo "🏃 Running tests in $env environment..."
    if docker run --rm "shlesha-test-$env"; then
        echo -e "${GREEN}✅ Tests passed for $env${NC}"
        RESULTS+=("$env: PASSED")
    else
        echo -e "${RED}❌ Tests failed for $env${NC}"
        RESULTS+=("$env: FAILED")
        ((FAILED++))
        return 1
    fi
}

# Copy wheels if they exist (for local testing)
if [ -d "../../target/wheels" ]; then
    echo "📦 Copying wheels for testing..."
    mkdir -p wheels
    cp ../../target/wheels/*.whl wheels/ 2>/dev/null || true
fi

# Make scripts executable
chmod +x scripts/*.sh

# Run tests for each environment
for env in "${ENVIRONMENTS[@]}"; do
    if run_environment_test "$env"; then
        echo -e "${GREEN}Environment $env completed successfully${NC}"
    else
        echo -e "${RED}Environment $env failed${NC}"
    fi
done

# Print summary
echo ""
echo "🏁 Integration Test Summary"
echo "=========================="
for result in "${RESULTS[@]}"; do
    env=$(echo "$result" | cut -d: -f1)
    status=$(echo "$result" | cut -d: -f2)
    
    case $status in
        "PASSED")
            echo -e "  ${GREEN}✅ $env: $status${NC}"
            ;;
        "FAILED"|"BUILD_FAILED")
            echo -e "  ${RED}❌ $env: $status${NC}"
            ;;
    esac
done

echo ""
if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 All integration tests passed!${NC}"
    exit 0
else
    echo -e "${RED}💥 $FAILED environment(s) failed${NC}"
    exit 1
fi