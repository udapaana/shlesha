# Shlesha Performance Benchmark Results

## 🎯 Executive Summary

**Shlesha achieves significant performance improvements with the new phoneme-based parser:**
- **1.93x faster** than the old O(n×m×c) parser
- **83.1% enum efficiency** (zero-allocation path usage)
- **Microsecond-level latency** for real-world text

## 📊 Core Performance Results

### Old Parser vs Phoneme Parser (Rust-native)

| Test Case | Old Parser | Phoneme Parser | Speedup |
|-----------|------------|----------------|---------|
| Single word (नमस्ते) | 7.97µs | 4.12µs | **1.93x** |
| Short sentence | 21.9µs | 10.8µs | **2.03x** |
| Medium text | 92.1µs | 40.4µs | **2.28x** |
| Long text | 235µs | 105µs | **2.24x** |
| 100KB throughput | 133.7ms | 69.9ms | **1.91x** |

**Key Insight**: Consistent ~2x performance improvement across all text sizes.

## 🏆 Comparison with Other Tools

### Python Library Comparison
| Tool | Time (6 chars) | Relative Speed |
|------|----------------|----------------|
| indic-transliteration | 0.9ms | **1.0x** (baseline) |
| Shlesha (CLI) | 12ms | 13.3x slower* |

*CLI includes process startup overhead (~11ms). Pure library performance is microsecond-level.

### Accuracy Results
**Devanagari → IAST: 100% accuracy** ✅

## 💡 Key Insights

1. **Algorithmic improvement matters**: O(n) vs O(n×m×c) gives consistent 2x speedup
2. **Zero-allocation design is effective**: 83.1% enum efficiency reduces memory pressure  
3. **One-way transliteration is production-ready**: 100% accuracy for Devanagari → IAST
4. **Round-trip needs work**: IAST → Devanagari transformation has issues

## 🚀 How to Run Benchmarks

```bash
# 1. Build release version
cargo build --release --example cli

# 2. Run performance benchmark
cargo bench --bench phoneme_vs_old_benchmark

# 3. Run accuracy tests  
cargo test round_trip_tests --test round_trip_tests

# 4. Run simple benchmark script
./run_benchmarks.sh

# 5. Python comparison (if available)
python3 benches/improved_python_benchmark.py
```
EOF < /dev/null