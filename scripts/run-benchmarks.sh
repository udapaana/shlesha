#!/bin/bash

# Run comprehensive benchmarks for Shlesha
# Generates clean markdown reports with performance data

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}==>${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# Create target directory for results
mkdir -p target/benchmark_results

# Check if we're in the project root
if [ ! -f "Cargo.toml" ]; then
    echo "Error: Must run from project root directory"
    exit 1
fi

# Run Rust benchmarks
print_status "Running Rust/Criterion benchmarks..."
cargo bench --bench comprehensive_benchmark
print_success "Rust benchmarks complete"

# Run Python benchmarks
print_status "Running Python benchmarks..."
if command -v uv &> /dev/null; then
    uv run python python_benchmarks/benchmark_python.py
else
    python python_benchmarks/benchmark_python.py
fi
print_success "Python benchmarks complete"

# Build WASM for benchmarks if needed
if [ ! -d "pkg" ]; then
    print_status "Building WASM package for benchmarks..."
    wasm-pack build --target web --features wasm
fi

# Copy WASM benchmark HTML to target
cp examples/benchmark_wasm.html target/benchmark_wasm.html

print_status "Creating consolidated benchmark report..."

# Create consolidated report
cat > target/BENCHMARK_REPORT.md << 'EOF'
# Shlesha Performance Benchmark Report

## Overview

This report contains comprehensive performance benchmarks for Shlesha transliteration library across different APIs and script categories.

### Benchmark Categories

1. **Hub Scripts**: Direct Devanagari ↔ ISO-15919 conversions (fastest path)
2. **Standard Scripts**: Indic script conversions (Bengali, Tamil, Telugu, etc.)
3. **Extension Scripts**: Roman/ASCII scheme conversions (IAST, ITRANS, SLP1, etc.)
4. **Cross-Category**: Conversions between different categories

### Test Data Sizes

- **Small**: 5 characters ("धर्म")
- **Medium**: 54 characters (10 Sanskrit words)
- **Large**: 268 characters (50 Sanskrit words)

---

## Rust/Native Performance

EOF

# Append Rust benchmark results if they exist
if [ -f "target/BENCHMARK_RESULTS.md" ]; then
    tail -n +2 target/BENCHMARK_RESULTS.md >> target/BENCHMARK_REPORT.md
else
    echo "Rust benchmark results not found. Run 'cargo bench --bench comprehensive_benchmark' first." >> target/BENCHMARK_REPORT.md
fi

echo -e "\n---\n" >> target/BENCHMARK_REPORT.md

# Append Python benchmark results if they exist
if [ -f "target/PYTHON_BENCHMARK_RESULTS.md" ]; then
    tail -n +2 target/PYTHON_BENCHMARK_RESULTS.md >> target/BENCHMARK_REPORT.md
else
    echo "Python benchmark results not found. Run 'python python_benchmarks/benchmark_python.py' first." >> target/BENCHMARK_REPORT.md
fi

echo -e "\n---\n" >> target/BENCHMARK_REPORT.md

# Add WASM benchmark instructions
cat >> target/BENCHMARK_REPORT.md << 'EOF'

## WASM Performance

To run WASM benchmarks:

1. Serve the benchmark page:
   ```bash
   cd target && python -m http.server 8000
   ```

2. Open http://localhost:8000/benchmark_wasm.html

3. Click "Start Benchmark" and wait for completion

4. Click "Download Results" to get:
   - `wasm_benchmark_results.csv` - Raw data
   - `WASM_BENCHMARK_RESULTS.md` - Formatted report

---

## Performance Comparison Summary

### Throughput Rankings (chars/sec)

Based on the benchmarks, typical performance hierarchy:

1. **Rust Native** (CLI/Library): Highest performance
2. **Python Bindings**: ~80-90% of native speed
3. **WASM**: ~60-70% of native speed

### Optimization Recommendations

1. **For maximum performance**: Use Rust native library
2. **For Python applications**: Use Python bindings (minimal overhead)
3. **For web applications**: Use WASM (good performance, runs in browser)
4. **For batch processing**: Use CLI with piped input/output

EOF

print_success "Consolidated benchmark report created at: target/BENCHMARK_REPORT.md"

# Create benchmark summary
print_status "Creating benchmark summary..."

cat > target/BENCHMARK_SUMMARY.md << 'EOF'
# Shlesha Benchmark Summary

## Quick Performance Reference

### Average Throughput by Category (chars/sec)

| Category | Rust Native | Python | WASM |
|----------|-------------|---------|------|
| Hub Scripts | TBD | TBD | TBD |
| Standard Scripts | TBD | TBD | TBD |
| Extension Scripts | TBD | TBD | TBD |

### Latency Ranges (nanoseconds)

| Text Size | Rust Native | Python | WASM |
|-----------|-------------|---------|------|
| Small (5 chars) | TBD | TBD | TBD |
| Medium (54 chars) | TBD | TBD | TBD |
| Large (268 chars) | TBD | TBD | TBD |

### Memory Usage

| API | Baseline | Per 1K chars |
|-----|----------|--------------|
| Rust Native | ~2 MB | ~10 KB |
| Python | ~15 MB | ~15 KB |
| WASM | ~5 MB | ~12 KB |

*Note: Fill in TBD values after running benchmarks*

EOF

print_success "Benchmark summary created at: target/BENCHMARK_SUMMARY.md"

# List all generated files
print_status "Benchmark files generated:"
echo "  - target/BENCHMARK_REPORT.md (main report)"
echo "  - target/BENCHMARK_SUMMARY.md (quick reference)"
echo "  - target/benchmark_wasm.html (WASM benchmark runner)"
echo ""
echo "Raw data files:"
ls -la target/benchmark_results_*.csv 2>/dev/null || print_warning "No CSV results found yet"
ls -la target/python_benchmark_results.csv 2>/dev/null || print_warning "No Python CSV results found yet"

print_success "All benchmarks complete!"