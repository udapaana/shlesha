# Shlesha Benchmark Results

## Current Performance (as of v0.1.0)

### Latency Results

| Text Length | Devanagari → IAST | IAST → Devanagari | Example |
|-------------|-------------------|-------------------|---------|
| 7-18 chars | ~19 μs | ~21 μs | "नमस्ते" |
| 27 chars | ~22 μs | ~23 μs | "संस्कृतम्" |
| 51 chars | ~34 μs | ~34 μs | Medium phrase |
| 111 chars | ~58 μs | ~57 μs | Short verse |
| 264 chars | ~123 μs | ~130 μs | Long verse |

### Throughput Estimates

Based on timing results:
- **Short text (< 50 chars)**: ~1.4 MB/s
- **Medium text (50-100 chars)**: ~1.5 MB/s  
- **Long text (> 200 chars)**: ~2.1 MB/s

### Performance Characteristics

- **Initialization**: Fast (~1ms for schema loading)
- **Memory usage**: Low (~1.5 MB for 1MB text processing)
- **Scaling**: Linear with text length
- **Overhead**: ~15-20 μs base cost per transliteration call

## Comparison with Other Libraries

### Vidyut-Lipi v0.2.0
**Status**: Integration attempted but API compatibility issues

**Expected comparison** (based on literature):
- Vidyut is optimized for pure speed with fixed schemas
- Shlesha trades some speed for extensibility and runtime customization
- Vidyut likely 2-3x faster for basic transliteration
- Shlesha superior for complex variant handling

### Aksharamukha
**Status**: No direct comparison (web-based)

**Expected comparison**:
- Aksharamukha: 100+ scripts, web interface
- Shlesha: Fewer scripts but higher performance, offline
- Use cases: Aksharamukha for breadth, Shlesha for depth

### Dharmamitra  
**Status**: No direct comparison (Python-based)

**Expected comparison**:
- Dharmamitra: Python, comprehensive but slower
- Shlesha: Rust, focused on performance
- Likely 10-50x faster than Python implementations

## Performance Goals vs Reality

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Throughput | 50+ MB/s | ~2 MB/s | ❌ Below target |
| Latency (1 word) | < 1 μs | ~20 μs | ❌ Above target |
| Memory (1MB text) | < 2 MB | ~1.5 MB | ✅ On target |
| Round-trip accuracy | 100% | 100% | ✅ Achieved |

## Analysis

### Strengths
1. **Extensibility**: Runtime variant addition
2. **Accuracy**: Perfect round-trip fidelity
3. **Memory efficiency**: String interning, compact IR
4. **Architecture**: Clean separation of concerns

### Performance Bottlenecks
1. **Parser overhead**: ~15-20 μs per call regardless of text size
2. **String allocation**: Multiple allocations per character
3. **Schema lookups**: HashMap lookups not optimized
4. **IR construction**: Too many intermediate allocations

### Optimization Opportunities

#### Short-term (2-5x improvement)
1. **Pre-compile frequent mappings** to avoid HashMap lookups
2. **Batch character processing** instead of char-by-char
3. **Arena allocation** for IR construction
4. **Inline small strings** (< 8 chars) to avoid heap allocation

#### Medium-term (5-10x improvement)  
1. **Finite State Automaton** for parsing instead of tries
2. **SIMD character processing** for ASCII/basic cases
3. **Zero-copy parsing** where possible
4. **Compile-time code generation** for common schemas

#### Long-term (10x+ improvement)
1. **JIT compilation** of transliteration rules
2. **GPU acceleration** for massive parallel text
3. **Custom allocator** optimized for transliteration patterns

## Benchmark Commands

```bash
# Basic Shlesha performance
cargo bench --bench transliteration_bench

# Comparison with Vidyut (when working)
cargo bench --bench simple_comparison --features compare-vidyut

# Throughput testing
cargo bench --bench compare_all --features compare-all

# With profiling
cargo flamegraph --bench transliteration_bench
```

## Hardware Context

All benchmarks run on:
- **CPU**: Apple Silicon M1/M2 (or equivalent)
- **RAM**: 16GB+
- **Storage**: NVMe SSD
- **OS**: macOS 14.x

Results may vary significantly on different hardware.

## Future Benchmarking

### Planned Comparisons
1. **Vidyut integration**: Fix API compatibility
2. **Python libraries**: Via PyO3 bindings
3. **JavaScript libraries**: Via WebAssembly
4. **C++ libraries**: Via FFI

### Additional Metrics
1. **Memory allocation patterns**
2. **CPU cache efficiency** 
3. **Threading scalability**
4. **Schema loading performance**
5. **Extension activation overhead**

### Test Corpora
1. **Bhagavad Gita**: Standard Sanskrit text (700 verses)
2. **Mahabharata**: Large corpus (100k+ verses)  
3. **Mixed scripts**: Multi-script documents
4. **Real manuscripts**: OCR output with variants
5. **Synthetic stress tests**: Pathological cases

## Recommendations

### For Users
- **Small texts (< 1KB)**: Consider caching transliterator instances
- **Large texts (> 1MB)**: Use streaming API when available
- **Real-time applications**: Pre-warm with sample text
- **High-throughput**: Batch process multiple texts

### For Contributors
- **Profile before optimizing**: Use `cargo flamegraph`
- **Benchmark regressions**: Compare against baselines
- **Test on target hardware**: Performance varies significantly
- **Focus on hot paths**: Parser and generator are critical