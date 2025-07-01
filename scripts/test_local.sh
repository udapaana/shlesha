#!/bin/bash
set -e

# Local testing script for Shlesha
# This script builds wheels and runs integration tests locally

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "🚀 Shlesha Local Integration Testing"
echo "===================================="

cd "$PROJECT_ROOT"

# Build wheels first
echo "🔨 Building wheels..."
maturin build --features python --release

# Run integration tests
echo "🧪 Running integration tests..."
cd docker/test-environments

# Copy fresh wheels
rm -rf wheels
mkdir -p wheels
cp ../../target/wheels/*.whl wheels/

# Run the integration tests
./run_integration_tests.sh

echo "✅ Local integration testing completed!"