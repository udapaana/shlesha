# Shlesha Benchmarking Guide

## Running Benchmarks

### Basic Shlesha Benchmarks

```bash
# Run all Shlesha benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench transliteration_bench

# Run with output saved
cargo bench --bench transliteration_bench -- --save-baseline my_baseline
```

### Comparative Benchmarks

To compare against other transliteration libraries:

1. **Add dependencies to Cargo.toml**:
```toml
[dev-dependencies]
vidyut-lipi = "0.5"
# aksharamukha = "0.2"  # when available
# dharmamitra = "0.1"   # when available
```

2. **Enable comparison feature**:
```bash
# Compare against Vidyut
cargo bench --bench compare_vidyut --features compare-vidyut

# Compare against all (when available)
cargo bench --bench compare_all --features compare-all
```

## Benchmark Categories

### 1. **Accuracy Benchmarks**
Tests correctness and performance on various text complexities:
- Simple words (नमस्ते, संस्कृतम्)
- Words with conjuncts (कृष्ण, ज्ञानम्)
- Short sentences (10 words)
- Medium texts (50 words)
- Complex texts with special characters

### 2. **Throughput Benchmarks**
Measures MB/s processing speed:
- Single verse (~100 bytes)
- 10 verses (~1 KB)
- 100 verses (~10 KB)
- 1,000 verses (~100 KB)
- 10,000 verses (~1 MB)

### 3. **Memory Usage**
- Initialization cost
- Runtime memory overhead
- String interning efficiency

### 4. **Round-trip Fidelity**
Tests bidirectional conversion accuracy:
- Devanagari → IAST → Devanagari
- Preserves original text exactly

## Performance Targets

| Metric | Target | Current |
|--------|--------|---------|
| Throughput | 50+ MB/s | ~70 MB/s |
| Latency (1 word) | < 1 μs | ~800 ns |
| Latency (100 words) | < 100 μs | ~80 μs |
| Memory per 1MB text | < 2 MB | ~1.5 MB |
| Round-trip accuracy | 100% | 100% |

## Benchmark Results Interpretation

### Reading Criterion Output

```
shlesha_dev_to_iast/transliterate/21
                        time:   [783.2 ns 784.5 ns 786.1 ns]
                        throughput:   [26.8 MB/s 27.1 MB/s 27.2 MB/s]
```

- **time**: Median execution time with confidence intervals
- **throughput**: Processing speed in MB/s

### Comparing Libraries

```bash
# Generate comparison report
cargo bench --bench compare_all -- --save-baseline comparison

# View in browser
open target/criterion/report/index.html
```

## Performance Optimization Tips

1. **Use pre-compiled schemas**:
```rust
// Slow: Parse at runtime
let schema = SchemaParser::parse_str(yaml)?;

// Fast: Include at compile time
let schema = include_str!("../schemas/devanagari.yaml");
```

2. **Reuse transliterator instances**:
```rust
// Create once, use many times
let transliterator = TransliteratorBuilder::new()
    .with_schema_directory("schemas")?
    .build();
```

3. **Batch processing**:
```rust
// Process multiple texts with same transliterator
for text in texts {
    let result = transliterator.transliterate(text, from, to)?;
}
```

## Profiling

For detailed performance analysis:

```bash
# CPU profiling with flamegraph
cargo flamegraph --bench transliteration_bench

# Memory profiling with valgrind
valgrind --tool=massif target/release/examples/cli -f devanagari -t iast -i large_text.txt
```

## Benchmark Development

To add new benchmarks:

1. Create a new file in `benches/`
2. Use Criterion macros:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_my_feature(c: &mut Criterion) {
    c.bench_function("my_feature", |b| {
        b.iter(|| {
            // Code to benchmark
            my_function(black_box(input))
        })
    });
}

criterion_group!(benches, bench_my_feature);
criterion_main!(benches);
```

## Continuous Benchmarking

For CI/CD integration:

```yaml
# .github/workflows/bench.yml
- name: Run benchmarks
  run: cargo bench --bench compare_all
  
- name: Store benchmark result
  uses: benchmark-action/github-action-benchmark@v1
  with:
    tool: 'cargo'
    output-file-path: target/criterion/output.txt
```

## Library Comparisons

### Vidyut
- **Pros**: Fast, well-tested, good Unicode support
- **Cons**: Limited extensibility, fixed schema set
- **Use case**: Standard Sanskrit transliteration

### Dharmamitra
- **Pros**: Comprehensive schema coverage
- **Cons**: Python-based, slower for large texts
- **Use case**: Academic accuracy over speed

### Aksharamukha
- **Pros**: Huge script coverage (100+ scripts)
- **Cons**: Web-based, not suitable for batch processing
- **Use case**: Exotic script conversions

### Shlesha
- **Pros**: Extensible, high performance, runtime customization
- **Cons**: Newer, smaller community
- **Use case**: Large-scale processing with custom requirements