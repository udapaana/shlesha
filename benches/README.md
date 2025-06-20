# Shlesha Comprehensive Benchmarking Suite

This directory contains a comprehensive benchmarking suite that fairly compares Shlesha with other transliteration systems across multiple platforms and environments.

## 🎯 Overview

The benchmarking suite ensures **fair comparisons** by testing tools in their native environments:
- **Rust vs Rust**: Native performance comparison using Criterion
- **Python vs Python**: Python library comparison when bindings are available
- **CLI vs CLI**: Command-line tool comparison
- **WASM vs WASM**: WebAssembly performance comparison in browsers

## 📁 Files Structure

```
benches/
├── README.md                           # This file
├── unified_benchmark_runner.py         # 🚀 Main orchestrator - run this!
├── comprehensive_rust_benchmark.rs     # 🦀 Rust vs Rust benchmarks
├── extensibility_performance_benchmark.rs # 🔧 Runtime extensibility overhead
├── improved_python_benchmark.py        # 🐍 Python vs Python benchmarks  
├── improved_cli_benchmark.sh           # ⚡ CLI vs CLI benchmarks
├── improved_wasm_benchmark.html        # 🌐 WASM vs WASM benchmarks
└── [legacy files...]                   # Previous benchmark iterations
```

## 🚀 Quick Start

### Run All Benchmarks (Recommended)

```bash
# Run comprehensive benchmarks across all platforms
python3 benches/unified_benchmark_runner.py

# Skip specific platforms if needed
python3 benches/unified_benchmark_runner.py --skip-python --skip-cli

# Include WASM benchmarks (requires manual execution)
python3 benches/unified_benchmark_runner.py --include-wasm
```

### Run Individual Benchmarks

```bash
# Rust benchmarks (requires Rust/Cargo)
cargo bench --bench comprehensive_rust_benchmark

# Runtime extensibility performance
cargo bench --bench extensibility_performance_benchmark

# Compare with Vidyut (if available)
cargo bench --bench extensibility_performance_benchmark --features compare-vidyut

# Python benchmarks
python3 benches/improved_python_benchmark.py

# CLI benchmarks (requires bash)
./benches/improved_cli_benchmark.sh

# WASM benchmarks (open in browser)
open benches/improved_wasm_benchmark.html
```

## 🔧 Prerequisites

### Core Requirements
- **Rust**: For building Shlesha and running Rust benchmarks
- **Python 3.7+**: For Python benchmarks and the unified runner
- **Bash**: For CLI benchmarks (Linux/macOS)

### Optional Tools (for comparison)
Install these to get more comprehensive comparisons:

#### Python Libraries
```bash
pip install indic-transliteration    # Indic transliteration library
pip install aksharamukha             # Aksharamukha transliterator
pip install dharmamitra              # Dharmamitra (if available)
pip install ai4bharat-transliteration # AI4Bharat transliterator
```

#### CLI Tools
```bash
# Vidyut CLI (if available)
cargo install vidyut-cli

# Or build from source
git clone https://github.com/vidyut-org/vidyut
cd vidyut && cargo build --release
```

#### System Tools
```bash
# Ubuntu/Debian
sudo apt-get install bc

# macOS (if not already installed)
brew install bc
```

## 📊 Benchmark Categories

### 1. Performance Benchmarks
- **Execution Time**: Mean, median, p95, p99 response times
- **Throughput**: Characters per second, words per second
- **Memory Usage**: Allocation patterns and efficiency
- **Scalability**: Performance across different text sizes

### 2. Accuracy Tests
- **Standard Test Cases**: Curated Devanagari → IAST conversions
- **Complex Conjuncts**: Sanskrit grammatical forms
- **Vedic Text**: Traditional Sanskrit with accents
- **Round-trip Accuracy**: Devanagari → IAST → Devanagari

### 3. Platform Comparisons
- **Native vs Bindings**: Rust native vs Python bindings performance
- **CLI Overhead**: Command-line invocation costs
- **WASM Performance**: Browser environment performance
- **Memory Efficiency**: Allocation patterns across platforms

## 📈 Understanding Results

### Performance Metrics

```
Tool Performance Summary:
Tool                 Avg Time    Throughput        Accuracy
Shlesha (Rust)       2.34ms      45,673 c/s       98.5%
Vidyut-lipi (Rust)   3.12ms      34,221 c/s       97.2%
indic-trans (Python) 15.67ms     6,834 c/s        95.1%
```

**Interpreting Results:**
- **Lower time is better** (faster execution)
- **Higher throughput is better** (more characters/second)
- **Higher accuracy is better** (more correct transliterations)

### Relative Performance
```
Relative Performance (vs Shlesha):
  Vidyut-lipi: 1.33x slower, accuracy -1.3%
  indic-trans: 6.69x slower, accuracy -3.4%
```

### Platform Comparison
- **Rust**: Fastest execution, lowest memory usage
- **Python**: Good for integration, moderate performance
- **CLI**: Includes process startup overhead
- **WASM**: Browser-dependent, good for web applications

## 🎛️ Customization

### Benchmark Configuration

```bash
# Adjust number of iterations
python3 benches/unified_benchmark_runner.py --iterations 100

# Set timeout for slow systems
python3 benches/unified_benchmark_runner.py --timeout 600

# Custom output directory
python3 benches/unified_benchmark_runner.py --output-dir my_results
```

### Adding New Tools

To add a new transliteration tool to the benchmarks:

1. **For Rust tools**: Add to `comprehensive_rust_benchmark.rs`
2. **For Python tools**: Add wrapper class to `improved_python_benchmark.py`
3. **For CLI tools**: Modify detection logic in `improved_cli_benchmark.sh`
4. **For WASM tools**: Add configuration to `improved_wasm_benchmark.html`

## 📋 Test Corpus

The benchmarks use a standardized test corpus:

### Performance Tests
- Single character: `क` → `ka`
- Simple word: `नमस्ते` → `namaste`
- Short sentence: `अहं संस्कृतं वदामि` → `ahaṃ saṃskṛtaṃ vadāmi`
- Complex conjuncts: `कृष्णार्जुनसंवादः` → `kṛṣṇārjunasaṃvādaḥ`
- Large text: Repeated verses for stress testing

### Accuracy Tests
- Standard characters and conjuncts
- Vedic accents and special marks
- Sanskrit grammatical forms
- Edge cases and difficult combinations

## 🔍 Troubleshooting

### Common Issues

**"No tools detected"**
```bash
# Ensure Shlesha is built
cargo build --release

# Check Python tool availability
python3 -c "from indic_transliteration import sanscript; print('OK')"
```

**"Benchmark timeout"**
```bash
# Increase timeout for slow systems
python3 benches/unified_benchmark_runner.py --timeout 900
```

**"Permission denied" (CLI benchmark)**
```bash
# Make script executable
chmod +x benches/improved_cli_benchmark.sh
```

**"WASM modules not loading"**
- WASM benchmarks require actual WASM builds
- Check browser console for errors
- Ensure WASM files are served over HTTP (not file://)

### Debugging

Enable verbose output:
```bash
# Python benchmarks
PYTHONUNBUFFERED=1 python3 benches/improved_python_benchmark.py

# Rust benchmarks with more detail
RUST_LOG=debug cargo bench --bench comprehensive_rust_benchmark
```

## 📊 Result Formats

The unified runner generates multiple output formats:

### JSON Results
```json
{
  "timestamp": "2024-01-15 14:30:25",
  "tools": [...],
  "summary": {...},
  "recommendations": [...]
}
```

### CSV Summary
```csv
Tool,Platform,Available,Mean_Time_ms,Throughput_chars_per_sec,Accuracy_%
Shlesha,Rust,Yes,2.34,45673,98.5
Vidyut-lipi,Rust,Yes,3.12,34221,97.2
```

### Human-Readable Report
```
SHLESHA TRANSLITERATION BENCHMARK REPORT
========================================

Generated: 2024-01-15 14:30:25
Total tools tested: 8
Available tools: 6

PERFORMANCE LEADERS
------------------
Fastest: Shlesha (Rust) - 2.34ms
Highest throughput: Shlesha (Rust) - 45,673 chars/sec

RECOMMENDATIONS
--------------
🦀 For maximum performance: Use Shlesha (Rust native)
🐍 For Python integration: 3 Python libraries available
⚡ For command-line usage: 4 CLI tools available
🎯 For highest accuracy: Shlesha (98.5%)
```

## 🤝 Contributing

To improve the benchmarking suite:

1. **Add test cases**: Extend the test corpus with challenging examples
2. **Add tools**: Include new transliteration libraries
3. **Improve accuracy**: Enhance parsing of tool outputs
4. **Add metrics**: Include new performance measurements
5. **Platform support**: Add support for Windows CLI tools

### Submitting Results

If you run benchmarks on different systems, consider sharing:
- System specifications (CPU, RAM, OS)
- Tool versions and availability
- Performance results
- Any issues encountered

## 📚 References

- [Criterion.rs](https://bheisler.github.io/criterion.rs/book/) - Rust benchmarking
- [Vidyut Project](https://github.com/vidyut-org/vidyut) - Sanskrit/Pali tools
- [Indic Transliteration](https://github.com/indic-transliteration/indic_transliteration_py) - Python library
- [Aksharamukha](https://www.aksharamukha.com/) - Universal transliterator

## 📄 License

This benchmarking suite is part of the Shlesha project and follows the same license terms.