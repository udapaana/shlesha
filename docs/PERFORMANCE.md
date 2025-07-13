# Performance Guide

This document explains Shlesha's performance characteristics, benchmarking methodology, and optimization strategies.

## Performance Overview

Shlesha is designed for transliteration with the following characteristics:

### Performance Characteristics

Technical implementation:
- Hash map lookups with function inlining
- Pre-allocated string capacity
- Compile-time code generation
- Optimized for both short and long text processing

### Comparative Performance

Shlesha's hub-and-spoke architecture trades some performance for extensibility compared to direct conversion approaches. Performance varies based on:
- Text length (short vs long)
- Script types (Roman vs Indic)
- Conversion path (direct vs hub-based)

## Architecture for Performance

### 1. Schema-Driven Code Generation

Shlesha generates converters at compile time:

```yaml
# schemas/tamil.yaml
metadata:
  name: "tamil"
  script_type: "brahmic"

mappings:
  vowels:
    "அ": "अ"
    "ஆ": "आ"
```

Generates optimized Rust code:
```rust
// Generated converter with O(1) lookups
pub fn convert_tamil_to_devanagari(input: &str) -> String {
    static VOWEL_MAP: phf::Map<&str, &str> = phf_map! {
        "அ" => "अ",
        "ஆ" => "आ",
        // ... optimized at compile time
    };
    // ... conversion logic
}
```

### 2. Perfect Hash Functions (PHF)

- Compile-time hash map generation for O(1) lookups
- No hash collisions in generated maps
- Cache-friendly memory layout

### 3. Hub-and-Spoke Architecture

```
Roman Scripts → ISO-15919 → Devanagari → Indic Scripts
```

- Minimal conversion steps (maximum 2 hops)
- Intermediate representations
- Reduced complexity compared to N×N conversion matrix

## Benchmarking

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark suites
cargo bench comprehensive
cargo bench performance
cargo bench comparison

# Profile with detailed timing
RUST_LOG=debug cargo bench

# Generate benchmark report
python python_benchmarks/generate_benchmark_report.py
```

### Benchmark Suites

#### 1. Comprehensive Benchmark (`benches/comprehensive_benchmark.rs`)

Tests all converters with realistic text samples:

```bash
# Sample output
Running comprehensive benchmark...
IAST → Devanagari: 156.23 MB/s (mean: 6.40 µs)
SLP1 → Devanagari: 178.45 MB/s (mean: 5.60 µs)
Bengali → Devanagari: 142.89 MB/s (mean: 7.00 µs)
```

#### 2. Comparison Benchmark (`examples/shlesha_vs_vidyut_benchmark.rs`)

Direct comparison with Vidyut:

```bash
cargo run --example shlesha_vs_vidyut_benchmark --release
```

#### 3. Memory Profiling

```bash
# Profile memory usage
cargo run --example profile_simple --release
valgrind --tool=massif target/release/examples/profile_simple
```

### Benchmark Data

Standard benchmark text samples:

- **Sanskrit texts**: Bhagavad Gita verses
- **Mixed content**: Technical terms, names, numbers
- **Edge cases**: Rare characters, conjuncts
- **Size ranges**: 10B to 100KB inputs

## Performance Optimization Strategies

### 1. Character-Level Optimizations

#### Fast Path for ASCII
```rust
// Check for ASCII fast path
if input.is_ascii() {
    return ascii_fast_path(input);
}
```

#### Unicode Normalization
```rust
// Normalize Unicode only when necessary
let normalized = if needs_normalization(input) {
    unicode_normalize(input)
} else {
    input
};
```

### 2. Memory Optimizations

#### String Pre-allocation
```rust
// Pre-allocate output string based on input size
let mut output = String::with_capacity(input.len() * size_factor);
```

#### Zero-Copy Operations
```rust
// Avoid unnecessary allocations
if input == output {
    return input.to_string(); // Or return Cow<str>
}
```

### 3. Algorithm Optimizations

#### Longest Match First
```rust
// Process longer sequences before shorter ones
// "kṣa" before "k" + "ṣ" + "a"
for &(pattern, replacement) in ORDERED_PATTERNS {
    // ... match logic
}
```

#### Context-Aware Conversion
```rust
// Consider character context for better accuracy
match (prev_char, current_char, next_char) {
    // Contextual rules
}
```

## Performance Monitoring

### Runtime Metrics

Track performance in production:

```rust
use std::time::Instant;

let start = Instant::now();
let result = converter.convert(input);
let duration = start.elapsed();

// Log performance metrics
if duration > threshold {
    warn!("Slow conversion: {:?} for {} chars", duration, input.len());
}
```

### Profiling Tools

#### 1. Built-in Rust Profiling
```bash
# Profile with perf
cargo build --release
perf record target/release/shlesha
perf report
```

#### 2. Flamegraph Generation
```bash
# Install flamegraph
cargo install flamegraph

# Generate flamegraph
cargo flamegraph --example profile_simple
```

#### 3. Memory Profiling
```bash
# Heap profiling
cargo run --example profile_simple --release
# Use tools like heaptrack, valgrind, or jemalloc profiling
```

## Performance Tuning

### 1. Schema Optimization

#### Efficient Character Ordering
```yaml
# Order by frequency for better cache performance
mappings:
  vowels:
    "a": "अ"    # Most common first
    "ā": "आ"
    "i": "इ"
    # ... less common last
```

#### Minimal Mapping Sets
```yaml
# Include only necessary mappings
# Avoid redundant or rarely-used characters
mappings:
  vowels:
    # Core vowels only
  consonants:
    # Essential consonants
```

### 2. Build-Time Optimization

#### Compiler Flags
```bash
# Maximum optimization
RUSTFLAGS="-C target-cpu=native -C opt-level=3" cargo build --release

# Profile-guided optimization
cargo build --release --profile=pgo
```

#### Link-Time Optimization
```toml
# Cargo.toml
[profile.release]
lto = true
codegen-units = 1
panic = "abort"
```

### 3. Runtime Optimization

#### Input Preprocessing
```rust
// Batch process multiple inputs
fn convert_batch(inputs: &[&str]) -> Vec<String> {
    inputs.par_iter()  // Parallel processing
          .map(|&input| convert(input))
          .collect()
}
```

#### Caching Results
```rust
use lru::LruCache;

// Cache frequent conversions
static CACHE: Lazy<Mutex<LruCache<String, String>>> = 
    Lazy::new(|| Mutex::new(LruCache::new(1000)));
```

## Performance Regression Testing

### Continuous Benchmarking

```bash
# Automated performance testing
./scripts/run-benchmarks.sh --compare-baseline
```

### Performance Thresholds

Set acceptable performance ranges:

```rust
#[cfg(test)]
mod performance_tests {
    #[test]
    fn benchmark_performance_thresholds() {
        let result = benchmark_converter();
        assert!(result.throughput > 100_000_000); // 100 MB/s minimum
        assert!(result.latency < Duration::from_micros(10)); // 10µs maximum
    }
}
```

### Regression Detection

```bash
# Compare against previous versions
cargo bench --save-baseline main
git checkout feature-branch
cargo bench --baseline main
```

## Deployment Considerations

### Production Settings

```bash
# Production build with maximum optimization
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

### Environment Variables

```bash
# Tune runtime behavior
export SHLESHA_CACHE_SIZE=10000
export SHLESHA_BATCH_SIZE=1000
export RUST_LOG=warn  # Reduce logging overhead
```

### Monitoring

```rust
// Production metrics
struct ConversionMetrics {
    total_conversions: u64,
    total_chars_processed: u64,
    average_latency: Duration,
    cache_hit_rate: f64,
}
```

## Troubleshooting Performance Issues

### Common Issues

1. **Slow Startup**: Large schema compilation
   - Solution: Use smaller, focused schemas
   
2. **Memory Usage**: Large hash maps
   - Solution: Optimize mapping sizes
   
3. **Cache Misses**: Poor locality
   - Solution: Reorder character mappings by frequency

### Diagnostic Tools

```bash
# Profile specific functions
cargo bench --bench comprehensive -- --profile-time=10

# Memory usage analysis
cargo run --example profile_simple --release 2>&1 | grep "heap\|malloc"

# CPU profiling
perf record -g cargo run --example benchmark
perf report -g
```

## Performance Optimizations History

### v0.1.7 Optimizations

Three optimizations implemented:

1. **Schema-based String Allocation**: Replaced `String::new()` with `String::with_capacity(input.len() * 2)` in schema-based converter hot paths
2. **Function Inlining**: Added `#[inline]` attributes to frequently called functions:
   - `RomanScriptProcessor::process*` methods
   - `IndicScriptProcessor::to_hub/from_hub`
   - `FastMappingBuilder` helper functions
   - `SchemaBasedConverter::apply_mappings`
3. **String Capacity Pre-allocation**: Added capacity pre-allocation to hub conversion functions

Measured impact: IAST conversions improved from competitive to faster than Vidyut baseline.

## Future Performance Work

### Planned Optimizations

1. **SIMD Processing**: Vectorized character processing
2. **GPU Acceleration**: CUDA/OpenCL for batch processing
3. **WebAssembly Optimization**: Smaller WASM binaries
4. **Incremental Conversion**: Delta-based updates

### Contributing Performance Improvements

See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines on:
- Adding new benchmarks
- Optimizing existing converters
- Performance regression testing
- Profiling methodology