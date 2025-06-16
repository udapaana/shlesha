#!/usr/bin/env python3
"""
Python benchmark comparing Shlesha Python bindings with Aksharamukha
Ensures fair comparison: Python vs Python
"""

import time
import statistics
import json
from typing import List, Tuple

# Test corpus
TEST_CORPUS = [
    # (Devanagari, word_count, description)
    ("नमस्ते", 1, "single_word"),
    ("अहं संस्कृतं वदामि", 3, "short_sentence"),
    ("तत्र शूरा महेष्वासा भीमार्जुनसमा युधि । युयुधानो विराटश्च द्रुपदश्च महारथः ॥", 15, "medium_verse"),
    ("धृतराष्ट्र उवाच । धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः । " * 5, 50, "long_text"),
]

def benchmark_function(func, args, iterations=100):
    """Benchmark a function with warmup"""
    # Warmup
    for _ in range(10):
        func(*args)
    
    # Actual benchmark
    times = []
    for _ in range(iterations):
        start = time.perf_counter()
        result = func(*args)
        end = time.perf_counter()
        times.append(end - start)
    
    return {
        'mean': statistics.mean(times),
        'median': statistics.median(times),
        'stdev': statistics.stdev(times) if len(times) > 1 else 0,
        'min': min(times),
        'max': max(times),
    }

def benchmark_shlesha_python():
    """Benchmark Shlesha Python bindings"""
    try:
        import shlesha_py  # This would be the Python bindings
        
        results = []
        for text, word_count, desc in TEST_CORPUS:
            # Create transliterator
            transliterator = shlesha_py.Transliterator()
            transliterator.load_schema_file("schemas/devanagari.yaml")
            transliterator.load_schema_file("schemas/iast.yaml")
            
            # Benchmark
            stats = benchmark_function(
                transliterator.transliterate,
                (text, "Devanagari", "IAST")
            )
            
            results.append({
                'tool': 'shlesha',
                'description': desc,
                'word_count': word_count,
                'text_length': len(text),
                **stats
            })
        
        return results
    except ImportError:
        print("Shlesha Python bindings not found. Please build them first.")
        return []

def benchmark_aksharamukha():
    """Benchmark Aksharamukha"""
    try:
        from aksharamukha import transliterate
        
        results = []
        for text, word_count, desc in TEST_CORPUS:
            # Benchmark
            stats = benchmark_function(
                transliterate.process,
                ("Devanagari", "IAST", text)
            )
            
            results.append({
                'tool': 'aksharamukha',
                'description': desc,
                'word_count': word_count,
                'text_length': len(text),
                **stats
            })
        
        return results
    except ImportError:
        print("Aksharamukha not found. Install with: pip install aksharamukha")
        return []

def benchmark_indic_transliteration():
    """Benchmark indic-transliteration library"""
    try:
        from indic_transliteration import sanscript
        from indic_transliteration.sanscript import transliterate
        
        results = []
        for text, word_count, desc in TEST_CORPUS:
            # Benchmark
            stats = benchmark_function(
                transliterate,
                (text, sanscript.DEVANAGARI, sanscript.IAST)
            )
            
            results.append({
                'tool': 'indic-transliteration',
                'description': desc,
                'word_count': word_count,
                'text_length': len(text),
                **stats
            })
        
        return results
    except ImportError:
        print("indic-transliteration not found. Install with: pip install indic-transliteration")
        return []

def format_results(results: List[dict]):
    """Format benchmark results as a table"""
    if not results:
        return
    
    # Group by tool
    tools = {}
    for r in results:
        tool = r['tool']
        if tool not in tools:
            tools[tool] = []
        tools[tool].append(r)
    
    # Print header
    print("\n" + "="*80)
    print(f"{'Description':<20} {'Words':<10} " + " ".join(f"{tool:<15}" for tool in tools.keys()))
    print("="*80)
    
    # Print results
    descriptions = set(r['description'] for r in results)
    for desc in descriptions:
        row = f"{desc:<20}"
        
        # Get word count
        word_count = next(r['word_count'] for r in results if r['description'] == desc)
        row += f"{word_count:<10}"
        
        # Get times for each tool
        for tool in tools.keys():
            tool_results = [r for r in tools[tool] if r['description'] == desc]
            if tool_results:
                mean_time = tool_results[0]['mean'] * 1000  # Convert to ms
                row += f"{mean_time:>13.3f}ms"
            else:
                row += f"{'N/A':>15}"
        
        print(row)
    
    print("\n" + "="*80)
    print("Detailed Statistics:")
    print("="*80)
    
    for tool, tool_results in tools.items():
        print(f"\n{tool}:")
        total_time = sum(r['mean'] for r in tool_results)
        total_words = sum(r['word_count'] for r in tool_results)
        total_chars = sum(r['text_length'] for r in tool_results)
        
        print(f"  Total time: {total_time*1000:.3f}ms")
        print(f"  Words/second: {total_words/total_time:,.0f}")
        print(f"  Chars/second: {total_chars/total_time:,.0f}")
        print(f"  Avg time/word: {total_time/total_words*1000:.3f}ms")

def compare_accuracy():
    """Compare accuracy of different tools"""
    print("\n" + "="*80)
    print("Accuracy Comparison")
    print("="*80)
    
    test_cases = [
        ("नमस्ते", "namaste"),
        ("संस्कृतम्", "saṃskṛtam"),
        ("कृष्ण", "kṛṣṇa"),
        ("ज्ञान", "jñāna"),
    ]
    
    # Test each tool
    tools_output = {}
    
    try:
        import shlesha_py
        trans = shlesha_py.Transliterator()
        trans.load_schema_file("schemas/devanagari.yaml")
        trans.load_schema_file("schemas/iast.yaml")
        
        tools_output['shlesha'] = []
        for deva, _ in test_cases:
            result = trans.transliterate(deva, "Devanagari", "IAST")
            tools_output['shlesha'].append(result)
    except:
        pass
    
    try:
        from aksharamukha import transliterate
        tools_output['aksharamukha'] = []
        for deva, _ in test_cases:
            result = transliterate.process("Devanagari", "IAST", deva)
            tools_output['aksharamukha'].append(result)
    except:
        pass
    
    # Compare outputs
    print(f"{'Input':<15} {'Expected':<15} " + " ".join(f"{tool:<20}" for tool in tools_output.keys()))
    print("-" * 80)
    
    for i, (deva, expected) in enumerate(test_cases):
        row = f"{deva:<15} {expected:<15}"
        for tool in tools_output.keys():
            if i < len(tools_output[tool]):
                output = tools_output[tool][i]
                if output == expected:
                    row += f" ✓ {output:<18}"
                else:
                    row += f" ✗ {output:<18}"
        print(row)

def main():
    print("Python Transliteration Library Benchmark")
    print("=" * 80)
    
    all_results = []
    
    # Run benchmarks
    print("\nBenchmarking Shlesha Python bindings...")
    all_results.extend(benchmark_shlesha_python())
    
    print("Benchmarking Aksharamukha...")
    all_results.extend(benchmark_aksharamukha())
    
    print("Benchmarking indic-transliteration...")
    all_results.extend(benchmark_indic_transliteration())
    
    # Format and display results
    format_results(all_results)
    
    # Compare accuracy
    compare_accuracy()
    
    # Save results
    with open('bench_data/python_benchmark_results.json', 'w') as f:
        json.dump(all_results, f, indent=2)
    
    print(f"\nResults saved to bench_data/python_benchmark_results.json")

if __name__ == "__main__":
    main()