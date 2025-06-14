#!/bin/bash
# Comprehensive benchmark runner for Shlesha transliteration engine
# Compares against Aksharamukha, Dharmamitra, and Vidyut-lipi

set -e

echo "🧪 Shlesha Transliteration Engine Benchmarks"
echo "============================================="

# Create results directory
mkdir -p results

# Activate virtual environment if it exists
if [ -d "../.venv" ]; then
    echo "🔧 Activating virtual environment..."
    source ../.venv/bin/activate
    echo "  ✅ Virtual environment activated"
else
    echo "⚠️  No virtual environment found at ../.venv"
fi

# Check if Python dependencies are available
echo "📦 Checking dependencies..."

# Optional packages (will be tested individually)
echo "  - Testing Aksharamukha availability..."
python3 -c "import aksharamukha" 2>/dev/null && echo "    ✅ Aksharamukha available" || echo "    ⚠️  Aksharamukha not available (install with: pip3 install aksharamukha)"

echo "  - Testing Dharmamitra availability..."
python3 -c "import indic_transliteration" 2>/dev/null && echo "    ✅ Dharmamitra available" || echo "    ⚠️  Dharmamitra not available (install with: pip3 install indic-transliteration)"

echo "  - Testing Vidyut-lipi availability..."
which vidyut-lipi >/dev/null 2>&1 && echo "    ✅ Vidyut-lipi available" || echo "    ⚠️  Vidyut-lipi not available (install with: cargo install vidyut-lipi)"

echo ""

# Run script testing
echo "🔤 Running script compatibility tests..."
if [ -f "accuracy/test_all_scripts.py" ]; then
    cd accuracy
    python3 test_all_scripts.py
    cd ..
    echo "  ✅ Script tests completed"
else
    echo "  ❌ Script test file not found"
fi

echo ""

# Run performance benchmarks
echo "⚡ Running performance benchmarks..."
if [ -f "performance/compare_engines.py" ]; then
    cd performance
    python3 compare_engines.py
    cd ..
    echo "  ✅ Performance tests completed"
else
    echo "  ❌ Performance test file not found"
fi

echo ""

# Run accuracy tests (if available)
echo "🎯 Running accuracy tests..."
if [ -f "accuracy/accuracy_test.py" ]; then
    cd accuracy
    python3 accuracy_test.py
    cd ..
    echo "  ✅ Accuracy tests completed"
else
    echo "  ⚠️  Accuracy test file not found (will be implemented)"
fi

echo ""

# Generate combined report
echo "📊 Generating combined report..."

TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
REPORT_FILE="results/combined_report_${TIMESTAMP}.md"

cat > "$REPORT_FILE" << EOF
# Shlesha Transliteration Engine - Comprehensive Benchmark Report

**Generated**: $(date)
**Version**: Shlesha v1.0 (Development)

## Overview

This report contains comprehensive benchmarks comparing Shlesha against other Sanskrit transliteration engines:

- **Shlesha** (This project) - Rust-based, Vedic-focused
- **Aksharamukha** - Python-based, 100+ scripts  
- **Dharmamitra** (indic-transliteration) - Python-based, multi-language
- **Vidyut-lipi** - Rust-based, Sanskrit-focused

## Test Categories

### 1. Script Compatibility
Tests all supported scripts for schema validation and basic functionality.

### 2. Performance Benchmarks  
Measures speed (chars/sec), memory usage (MB), and reliability across different text sizes.

### 3. Accuracy Tests
Validates transliteration accuracy against reference texts and edge cases.

## Individual Reports

EOF

# Link to individual reports if they exist
if ls results/script_test_*.md >/dev/null 2>&1; then
    echo "- [Script Test Report]($(ls results/script_test_*.md | tail -1 | xargs basename))" >> "$REPORT_FILE"
fi

if ls results/performance_report_*.md >/dev/null 2>&1; then
    echo "- [Performance Report]($(ls results/performance_report_*.md | tail -1 | xargs basename))" >> "$REPORT_FILE"
fi

echo "" >> "$REPORT_FILE"
echo "## Results Summary" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"
echo "*Individual test results are available in the linked reports above.*" >> "$REPORT_FILE"

echo "  ✅ Combined report generated: $REPORT_FILE"

echo ""
echo "🎉 All benchmarks completed!"
echo ""
echo "📁 Results available in: benchmarks/results/"
echo "📋 Combined report: $REPORT_FILE"
echo ""
echo "To view results:"
echo "  ls -la results/"
echo "  cat $REPORT_FILE"