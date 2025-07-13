# Python Benchmarks

This directory contains Python performance benchmarks and comparison scripts for testing Shlesha's Python bindings.

## Running Benchmarks

```bash
# Run main Python benchmark suite
python python_benchmarks/benchmark_python.py

# Run comparison with other libraries
python python_benchmarks/benchmark_comparison.py

# Generate benchmark report
python python_benchmarks/generate_benchmark_report.py
```

## Benchmark Files

- `benchmark_python.py` - Main Python API benchmark suite
- `benchmark_comparison.py` - Compares Shlesha with other transliteration libraries
- `generate_benchmark_report.py` - Generates consolidated benchmark reports
- `python/` - Additional Python performance test scripts

## Results

Results are typically saved to `target/` directory with various output formats.