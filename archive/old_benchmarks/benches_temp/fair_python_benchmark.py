#!/usr/bin/env python3
"""
Fair Python vs Python benchmark comparing Shlesha with other Python transliteration libraries.
This benchmark ensures fair comparison by using the same input data and measurement methodology.
"""

import time
import json
import sys
import os
from pathlib import Path

# Add the parent directory to sys.path to import shlesha
sys.path.insert(0, str(Path(__file__).parent.parent))

# Import libraries
try:
    import shlesha
    HAS_SHLESHA = True
except ImportError:
    HAS_SHLESHA = False
    print("Warning: Could not import shlesha")

try:
    from indic_transliteration import sanscript
    HAS_INDIC_TRANS = True
except ImportError:
    HAS_INDIC_TRANS = False
    print("Warning: Could not import indic_transliteration")

try:
    from aksharamukha import transliterate
    HAS_AKSHARAMUKHA = True
except ImportError:
    HAS_AKSHARAMUKHA = False
    print("Warning: Could not import aksharamukha")


def load_test_data():
    """Load test data from bench_data directory"""
    data_files = {
        'small': 'bench_data/small.txt',
        'medium': 'bench_data/medium.txt',
        'large': 'bench_data/large.txt',
        'very_large': 'bench_data/very_large.txt'
    }
    
    test_data = {}
    for size, filename in data_files.items():
        try:
            with open(filename, 'r', encoding='utf-8') as f:
                test_data[size] = f.read().strip()
        except FileNotFoundError:
            print(f"Warning: {filename} not found")
    
    return test_data


def benchmark_shlesha(text, iterations=100):
    """Benchmark Shlesha Python API"""
    if not HAS_SHLESHA:
        return None
    
    transliterator = shlesha.Transliterator()
    
    # Warm up
    for _ in range(10):
        _ = transliterator.transliterate(text, "devanagari", "iast")
    
    # Benchmark
    start_time = time.perf_counter()
    for _ in range(iterations):
        result = transliterator.transliterate(text, "devanagari", "iast")
    end_time = time.perf_counter()
    
    return {
        'total_time': end_time - start_time,
        'avg_time': (end_time - start_time) / iterations,
        'iterations': iterations,
        'chars_per_second': len(text) * iterations / (end_time - start_time)
    }


def benchmark_indic_transliteration(text, iterations=100):
    """Benchmark indic-transliteration library"""
    if not HAS_INDIC_TRANS:
        return None
    
    # Warm up
    for _ in range(10):
        _ = sanscript.transliterate(text, sanscript.DEVANAGARI, sanscript.IAST)
    
    # Benchmark
    start_time = time.perf_counter()
    for _ in range(iterations):
        result = sanscript.transliterate(text, sanscript.DEVANAGARI, sanscript.IAST)
    end_time = time.perf_counter()
    
    return {
        'total_time': end_time - start_time,
        'avg_time': (end_time - start_time) / iterations,
        'iterations': iterations,
        'chars_per_second': len(text) * iterations / (end_time - start_time)
    }


def benchmark_aksharamukha(text, iterations=100):
    """Benchmark Aksharamukha library"""
    if not HAS_AKSHARAMUKHA:
        return None
    
    # Warm up
    for _ in range(10):
        _ = transliterate.process('Devanagari', 'IAST', text)
    
    # Benchmark
    start_time = time.perf_counter()
    for _ in range(iterations):
        result = transliterate.process('Devanagari', 'IAST', text)
    end_time = time.perf_counter()
    
    return {
        'total_time': end_time - start_time,
        'avg_time': (end_time - start_time) / iterations,
        'iterations': iterations,
        'chars_per_second': len(text) * iterations / (end_time - start_time)
    }


def run_benchmarks():
    """Run all benchmarks and collect results"""
    test_data = load_test_data()
    results = {}
    
    # Different iteration counts for different sizes
    iteration_counts = {
        'small': 1000,
        'medium': 100,
        'large': 10,
        'very_large': 1
    }
    
    for size, text in test_data.items():
        if not text:
            continue
            
        iterations = iteration_counts.get(size, 100)
        print(f"\nBenchmarking {size} text ({len(text)} chars) with {iterations} iterations...")
        
        results[size] = {
            'text_length': len(text),
            'iterations': iterations
        }
        
        # Benchmark Shlesha
        print("  - Benchmarking Shlesha...")
        shlesha_result = benchmark_shlesha(text, iterations)
        if shlesha_result:
            results[size]['shlesha'] = shlesha_result
            print(f"    Avg time: {shlesha_result['avg_time']*1000:.3f}ms, "
                  f"Chars/sec: {shlesha_result['chars_per_second']:.0f}")
        
        # Benchmark indic-transliteration
        print("  - Benchmarking indic-transliteration...")
        indic_result = benchmark_indic_transliteration(text, iterations)
        if indic_result:
            results[size]['indic_transliteration'] = indic_result
            print(f"    Avg time: {indic_result['avg_time']*1000:.3f}ms, "
                  f"Chars/sec: {indic_result['chars_per_second']:.0f}")
        
        # Benchmark Aksharamukha
        print("  - Benchmarking Aksharamukha...")
        akshara_result = benchmark_aksharamukha(text, iterations)
        if akshara_result:
            results[size]['aksharamukha'] = akshara_result
            print(f"    Avg time: {akshara_result['avg_time']*1000:.3f}ms, "
                  f"Chars/sec: {akshara_result['chars_per_second']:.0f}")
    
    return results


def print_comparison_table(results):
    """Print a comparison table of results"""
    print("\n" + "="*80)
    print("FAIR PYTHON BENCHMARK RESULTS")
    print("="*80)
    
    libraries = ['shlesha', 'indic_transliteration', 'aksharamukha']
    
    for size in ['small', 'medium', 'large', 'very_large']:
        if size not in results:
            continue
            
        size_results = results[size]
        print(f"\n{size.upper()} Text ({size_results['text_length']} chars)")
        print("-" * 60)
        print(f"{'Library':<25} {'Avg Time (ms)':<15} {'Chars/sec':<15} {'Relative'}")
        print("-" * 60)
        
        # Get baseline (shlesha) performance
        baseline_time = None
        if 'shlesha' in size_results:
            baseline_time = size_results['shlesha']['avg_time']
        
        for lib in libraries:
            if lib in size_results:
                result = size_results[lib]
                avg_time_ms = result['avg_time'] * 1000
                chars_per_sec = result['chars_per_second']
                
                if baseline_time and lib != 'shlesha':
                    relative = result['avg_time'] / baseline_time
                    relative_str = f"{relative:.2f}x"
                else:
                    relative_str = "1.00x (baseline)"
                
                print(f"{lib:<25} {avg_time_ms:<15.3f} {chars_per_sec:<15.0f} {relative_str}")


def save_results(results):
    """Save benchmark results to JSON file"""
    timestamp = time.strftime("%Y%m%d_%H%M%S")
    filename = f"unified_benchmark_results/python_fair_benchmark_{timestamp}.json"
    
    os.makedirs("unified_benchmark_results", exist_ok=True)
    
    with open(filename, 'w') as f:
        json.dump(results, f, indent=2)
    
    print(f"\nResults saved to {filename}")
    return filename


def main():
    """Main function"""
    print("Starting Fair Python vs Python Benchmark")
    print("=" * 80)
    
    # Check which libraries are available
    print("\nAvailable libraries:")
    print(f"  - Shlesha: {'✓' if HAS_SHLESHA else '✗'}")
    print(f"  - indic-transliteration: {'✓' if HAS_INDIC_TRANS else '✗'}")
    print(f"  - Aksharamukha: {'✓' if HAS_AKSHARAMUKHA else '✗'}")
    
    # Run benchmarks
    results = run_benchmarks()
    
    # Print comparison table
    print_comparison_table(results)
    
    # Save results
    save_results(results)


if __name__ == "__main__":
    main()