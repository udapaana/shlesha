#!/bin/bash
set -e

echo "=== Shlesha Integration Tests ==="
echo "Platform: $(uname -a)"
echo "Python: $(python3 --version)"
echo ""

# Test 1: Install from PyPI
echo "🔍 Test 1: Install latest from PyPI"
./scripts/test_pypi_install.sh

# Test 2: Install from local wheel (if available)
if [ -d "wheels" ] && [ "$(ls -A wheels)" ]; then
    echo ""
    echo "🔍 Test 2: Install from local wheel"
    ./scripts/test_wheel_install.sh
fi

# Test 3: Comprehensive functionality tests
echo ""
echo "🔍 Test 3: Functionality tests"
./scripts/test_functionality.sh

# Test 4: Binary analysis
echo ""
echo "🔍 Test 4: Binary analysis"
./scripts/test_binary_analysis.sh

echo ""
echo "✅ All tests completed successfully!"