#!/bin/bash

# Comprehensive pre-computation benchmark runner
# Tests Shlesha with different feature flags to measure pre-computation impact

set -e

echo "🧪 Shlesha Pre-computation Benchmark Suite"
echo "=========================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Create results directory
RESULTS_DIR="benchmark_results/$(date +%Y%m%d_%H%M%S)"
mkdir -p "$RESULTS_DIR"

echo -e "${BLUE}📂 Results will be saved to: $RESULTS_DIR${NC}"

# Function to run Rust benchmarks with specific features
run_rust_benchmarks() {
    local feature_flag="$1"
    local description="$2"
    local output_file="$3"
    
    echo -e "\n${YELLOW}🔧 Building and benchmarking with $description${NC}"
    echo "Feature flag: $feature_flag"
    
    # Clean and build
    cargo clean
    if [ "$feature_flag" = "default" ]; then
        cargo build --release
    else
        cargo build --release --features "$feature_flag"
    fi
    
    # Run the pre-computation comparison benchmark
    echo "Running pre-computation comparison benchmark..."
    if [ "$feature_flag" = "default" ]; then
        cargo bench --bench precompute_comparison_benchmark > "$output_file" 2>&1
    else
        cargo bench --bench precompute_comparison_benchmark --features "$feature_flag" > "$output_file" 2>&1
    fi
    
    echo -e "${GREEN}✅ Completed: $description${NC}"
}

# Function to run Python benchmarks
run_python_benchmarks() {
    local feature_description="$1"
    local output_file="$2"
    
    echo -e "\n${YELLOW}🐍 Running Python comparison benchmarks${NC}"
    echo "Current build: $feature_description"
    
    # Build Python bindings with current features
    maturin develop --release
    
    # Run Python benchmark
    python3 benchmarks/precompute_python_benchmark.py > "$output_file" 2>&1
    
    echo -e "${GREEN}✅ Python benchmarks completed${NC}"
}

# Function to build WASM with features
build_wasm() {
    local feature_flag="$1"
    local description="$2"
    
    echo -e "\n${YELLOW}🌐 Building WASM with $description${NC}"
    
    if [ "$feature_flag" = "default" ]; then
        wasm-pack build --target web --out-dir examples/pkg
    else
        wasm-pack build --target web --out-dir examples/pkg --features "$feature_flag"
    fi
    
    echo -e "${GREEN}✅ WASM build completed: $description${NC}"
}

# Main benchmark execution
main() {
    echo -e "\n${BLUE}🚀 Starting comprehensive benchmark suite...${NC}"
    
    # Test different pre-computation configurations
    echo -e "\n${BLUE}📊 Testing Pre-computation Configurations${NC}"
    echo "=========================================="
    
    # 1. No pre-computation (baseline)
    run_rust_benchmarks "no-precompute" "No Pre-computation (Baseline)" "$RESULTS_DIR/rust_no_precompute.txt"
    run_python_benchmarks "No Pre-computation" "$RESULTS_DIR/python_no_precompute.txt"
    build_wasm "no-precompute" "No Pre-computation"
    cp examples/precompute_wasm_benchmark.html "$RESULTS_DIR/wasm_no_precompute.html"
    
    # 2. Common pre-computation
    run_rust_benchmarks "precompute-common" "Common Pre-computation" "$RESULTS_DIR/rust_precompute_common.txt"
    run_python_benchmarks "Common Pre-computation" "$RESULTS_DIR/python_precompute_common.txt"
    build_wasm "precompute-common" "Common Pre-computation"
    cp examples/precompute_wasm_benchmark.html "$RESULTS_DIR/wasm_precompute_common.html"
    
    # 3. All pre-computation
    run_rust_benchmarks "precompute-all" "All Pre-computation" "$RESULTS_DIR/rust_precompute_all.txt"
    run_python_benchmarks "All Pre-computation" "$RESULTS_DIR/python_precompute_all.txt"
    build_wasm "precompute-all" "All Pre-computation"
    cp examples/precompute_wasm_benchmark.html "$RESULTS_DIR/wasm_precompute_all.html"
    
    # 4. Default build (for comparison)
    run_rust_benchmarks "default" "Default Build" "$RESULTS_DIR/rust_default.txt"
    run_python_benchmarks "Default Build" "$RESULTS_DIR/python_default.txt"
    build_wasm "default" "Default Build"
    cp examples/precompute_wasm_benchmark.html "$RESULTS_DIR/wasm_default.html"
    
    # Generate comparison report
    generate_comparison_report
    
    echo -e "\n${GREEN}🎉 All benchmarks completed!${NC}"
    echo -e "${BLUE}📁 Results saved in: $RESULTS_DIR${NC}"
    echo -e "${YELLOW}📊 Check the comparison report: $RESULTS_DIR/comparison_report.md${NC}"
}

# Generate a comparison report
generate_comparison_report() {
    echo -e "\n${YELLOW}📈 Generating comparison report...${NC}"
    
    cat > "$RESULTS_DIR/comparison_report.md" << EOF
# Shlesha Pre-computation Benchmark Results

Generated on: $(date)

## Overview

This report compares Shlesha's performance with different pre-computation configurations:

1. **No Pre-computation**: Baseline performance using runtime hub conversions
2. **Common Pre-computation**: Pre-compute IAST, ITRANS, SLP1 ↔ Devanagari conversions
3. **All Pre-computation**: Pre-compute all Roman ↔ Indic conversions
4. **Default Build**: Standard build configuration

## Expected Results

### Pre-computation should improve:
- Roman → Indic conversions (e.g., IAST → Devanagari)
- Indic → Roman conversions (e.g., Devanagari → IAST)
- These normally require 3 steps, optimized to 2 steps

### Pre-computation should NOT affect:
- Indic → Indic conversions (already optimal via Devanagari hub)
- Roman → Roman conversions (already optimal via ISO hub)

## Files Generated

### Rust Benchmarks (Criterion)
- \`rust_no_precompute.txt\` - Baseline performance
- \`rust_precompute_common.txt\` - Common pre-computation
- \`rust_precompute_all.txt\` - All pre-computation  
- \`rust_default.txt\` - Default build

### Python Benchmarks
- \`python_no_precompute.txt\` - Python bindings without pre-computation
- \`python_precompute_common.txt\` - Python bindings with common pre-computation
- \`python_precompute_all.txt\` - Python bindings with all pre-computation
- \`python_default.txt\` - Python bindings default build

### WASM Benchmarks
- \`wasm_no_precompute.html\` - WASM without pre-computation
- \`wasm_precompute_common.html\` - WASM with common pre-computation
- \`wasm_precompute_all.html\` - WASM with all pre-computation
- \`wasm_default.html\` - WASM default build

## Analysis Instructions

1. **Compare Rust benchmarks**: Look for improvements in "precomputation_impact" group
2. **Check Python results**: Compare times between feature flag configurations
3. **Test WASM**: Open HTML files in browser and run interactive benchmarks
4. **Measure step reduction**: Focus on Roman↔Indic conversions showing 3→2 step optimization

## Key Metrics to Examine

- **Throughput improvements** for pre-computed conversions
- **Consistent performance** for control group conversions
- **Build size impact** of different pre-computation levels
- **Cross-platform consistency** between Rust, Python, and WASM

## Performance Interpretation

- **Improvement > 10%**: Significant pre-computation benefit
- **Improvement 5-10%**: Moderate benefit
- **Improvement < 5%**: Minimal impact
- **Negative improvement**: Possible overhead (investigate)

EOF
    
    echo -e "${GREEN}✅ Comparison report generated${NC}"
}

# Check prerequisites
check_prerequisites() {
    echo -e "${BLUE}🔍 Checking prerequisites...${NC}"
    
    # Check for required tools
    command -v cargo >/dev/null 2>&1 || { echo -e "${RED}❌ cargo not found${NC}"; exit 1; }
    command -v python3 >/dev/null 2>&1 || { echo -e "${RED}❌ python3 not found${NC}"; exit 1; }
    command -v wasm-pack >/dev/null 2>&1 || { echo -e "${RED}❌ wasm-pack not found${NC}"; exit 1; }
    command -v maturin >/dev/null 2>&1 || { echo -e "${RED}❌ maturin not found${NC}"; exit 1; }
    
    # Check for Python packages
    python3 -c "import shlesha" 2>/dev/null || {
        echo -e "${YELLOW}⚠️  Shlesha Python package not found, will build during benchmark${NC}"
    }
    
    echo -e "${GREEN}✅ Prerequisites check passed${NC}"
}

# Cleanup function
cleanup() {
    echo -e "\n${YELLOW}🧹 Cleaning up...${NC}"
    # Clean any temporary files if needed
}

# Trap cleanup on exit
trap cleanup EXIT

# Main execution
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    check_prerequisites
    main "$@"
fi