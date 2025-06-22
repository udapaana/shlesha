# Shlesha Benchmarks

## Overview

Shlesha includes comprehensive performance benchmarks across all APIs (Rust, Python, WASM) and script categories.

## Running Benchmarks

### Quick Start

Run all benchmarks with a single command:

```bash
./scripts/run-benchmarks.sh
```

This will:
1. Run Rust/Criterion benchmarks
2. Run Python API benchmarks
3. Generate consolidated reports
4. Create summary statistics

### Individual Benchmarks

#### Rust/Native Benchmarks

Run the focused performance benchmarks (recommended):

```bash
cargo bench --bench comprehensive_benchmark
```

This runs **23 representative conversions** covering all performance patterns:
- **Hub conversions** (2): Direct `devanagari` ↔ `iso15919` (fastest)
- **Indic → Roman** (6): `telugu` → all Roman scripts (2-hop via hub)
- **Roman → Indic** (7): `iast` → all Indic scripts (2-hop via hub)  
- **Roman → Roman** (4): Between Roman scripts (1-hop via ISO hub)
- **Indic → Indic** (4): Between Indic scripts (3-hop via hub)

Results are saved to:
- `target/benchmark_results_hub.csv`
- `target/benchmark_results_indic_to_roman.csv`
- `target/benchmark_results_roman_to_indic.csv`
- `target/benchmark_results_roman_to_roman.csv`
- `target/benchmark_results_indic_to_indic.csv`
- `target/BENCHMARK_RESULTS.md` (summary report)

#### Python API Benchmarks

```bash
uv run python benchmark_python.py
```

Results are saved to:
- `target/python_benchmark_results.csv`
- `target/PYTHON_BENCHMARK_RESULTS.md`

#### WASM Benchmarks

1. Build WASM package:
   ```bash
   wasm-pack build --target web --features wasm
   ```

2. Serve the benchmark page:
   ```bash
   python -m http.server 8000
   ```

3. Open http://localhost:8000/benchmark_wasm.html

4. Click "Start Benchmark" and download results

#### Library Comparison

Compare Shlesha with other transliteration libraries:

```bash
# Install comparison libraries first
pip install vidyut-py indic-transliteration aksharamukha

# Run comparison
uv run python benchmark_comparison.py
```

Results are saved to:
- `target/comparison_benchmark_results.csv`
- `target/COMPARISON_BENCHMARK_RESULTS.md`

## Benchmark Categories

### 1. Hub Scripts
Direct Devanagari ↔ ISO-15919 conversions (fastest path):
- `devanagari` ↔ `iso15919`

### 2. Standard Scripts
Indic script conversions:
- `bengali`, `tamil`, `telugu`, `gujarati`, `kannada`, `malayalam`, `odia`

### 3. Extension Scripts
Roman/ASCII scheme conversions:
- `iast`, `itrans`, `slp1`, `harvard_kyoto`, `velthuis`, `wx`

### 4. Cross-Category
Conversions between different categories:
- Hub → Standard (e.g., `devanagari` → `tamil`)
- Hub → Extension (e.g., `devanagari` → `iast`)

## Test Data Sizes

- **Small**: 5 characters ("धर्म")
- **Medium**: 54 characters (10 Sanskrit words)
- **Large**: 268 characters (50 Sanskrit words)

## Metrics

### Throughput
Characters processed per second (chars/sec). Higher is better.

### Latency
Time taken for a single conversion in nanoseconds (ns). Lower is better.

## Report Generation

Generate consolidated reports:

```bash
uv run python generate_benchmark_report.py
```

This creates:
- `target/BENCHMARK_DATA.md` - Complete benchmark data
- `target/BENCHMARK_SUMMARY.md` - Quick reference summary

## Performance Guidelines

### Expected Performance Ranges

| API | Expected Throughput |
|-----|-------------------|
| Rust Native | 1M - 10M chars/sec |
| Python Bindings | 100K - 1M chars/sec |
| WASM | 50K - 500K chars/sec |

### Optimization Tips

1. **Use Hub Scripts**: Direct Devanagari ↔ ISO-15919 is fastest
2. **Batch Processing**: Process multiple texts in a single call
3. **Reuse Instances**: Create one transliterator and reuse it
4. **Choose Right API**: Use native Rust for maximum performance

## Benchmark Output Format

All benchmark results are saved as:
1. **CSV files**: Raw data for analysis
2. **Markdown files**: Clean tables for documentation

### CSV Format

```csv
script_from,script_to,category,text_size,throughput_chars_per_sec,latency_ns
devanagari,iso15919,hub,small,5000000,1000
```

### Markdown Format

| From | To | Text Size | Throughput (chars/sec) | Latency (ns) |
|------|----|-----------|-----------------------|-------------|
| devanagari | iso15919 | small | 5,000,000 | 1,000 |

## Continuous Benchmarking

For tracking performance over time:

1. Run benchmarks before major changes
2. Save results with timestamp:
   ```bash
   cp target/BENCHMARK_DATA.md "benchmarks/results/$(date +%Y%m%d).md"
   ```
3. Compare with previous results to detect regressions

## Troubleshooting

### Criterion Not Found

```bash
cargo add --dev criterion
```

### Python Import Errors

```bash
uv sync --dev
uv run maturin develop --features python
```

### WASM Build Errors

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```