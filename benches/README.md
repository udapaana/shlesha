# Rust Benchmarks

This directory contains Rust performance benchmarks using the Criterion benchmarking framework.

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench comprehensive_benchmark

# Run with profiling output
cargo bench --bench profiling_benchmark
```

## Benchmark Files

- `comprehensive_benchmark.rs` - Main benchmark suite covering all conversion patterns
- `comparison.rs` - Comparison benchmarks between different implementations
- `fast_optimization_benchmark.rs` - Tests for specific optimizations
- `profile_roman_processing.rs` - Focused benchmarks for Roman script processing
- `profiling_benchmark.rs` - Benchmarks designed for profiling
- `runtime_vs_builtin_benchmark.rs` - Compares runtime-loaded vs built-in schemas

## Results

Benchmark results are saved to `target/criterion/` with HTML reports.