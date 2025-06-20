#!/usr/bin/env python3
"""
Comprehensive Python benchmark comparing all available transliteration libraries
Ensures fair comparison: Python vs Python for all tools
"""

import time
import statistics
import json
import sys
from typing import List, Dict, Any, Callable, Optional
import importlib

# Test corpus with varying complexity
TEST_CORPUS = [
    # (Devanagari, word_count, description)
    ("नमस्ते", 1, "single_word"),
    ("अहं संस्कृतं वदामि", 3, "short_sentence"),
    ("तत्र शूरा महेष्वासा भीमार्जुनसमा युधि", 10, "medium_verse"),
    ("धृतराष्ट्र उवाच धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः", 15, "long_sentence"),
    ("तत्र शूरा महेष्वासा भीमार्जुनसमा युधि । युयुधानो विराटश्च द्रुपदश्च महारथः ॥" * 10, 150, "repeated_verse"),
    ("कर्मण्येवाधिकारस्ते मा फलेषु कदाचन । मा कर्मफलहेतुर्भूर्मा ते सङ्गोऽस्त्वकर्मणि ॥" * 100, 1500, "large_text"),
]

# Expected results for accuracy testing
ACCURACY_TEST_CASES = [
    ("नमस्ते", "namaste"),
    ("संस्कृतम्", "saṃskṛtam"),
    ("कृष्ण", "kṛṣṇa"),
    ("ज्ञान", "jñāna"),
    ("अग्निमीळे", "agnimīḷe"),
    ("यज्ञस्य", "yajñasya"),
    ("ऋत्विजम्", "ṛtvijam"),
]

class TransliteratorWrapper:
    """Base class for wrapping different transliteration libraries"""
    
    def __init__(self, name: str):
        self.name = name
        self.available = False
        self.error_message = None
        
    def setup(self) -> bool:
        """Setup the transliterator. Return True if successful."""
        raise NotImplementedError
        
    def transliterate(self, text: str, from_script: str, to_script: str) -> str:
        """Transliterate text from one script to another."""
        raise NotImplementedError
        
    def get_info(self) -> Dict[str, Any]:
        """Get library information."""
        return {
            "name": self.name,
            "available": self.available,
            "error": self.error_message
        }

class ShleshaWrapper(TransliteratorWrapper):
    def __init__(self):
        super().__init__("Shlesha (Python)")
        
    def setup(self) -> bool:
        try:
            # This would be the actual Python bindings
            import shlesha_py
            self.module = shlesha_py
            self.transliterator = shlesha_py.Transliterator()
            self.transliterator.load_schema_file("schemas/devanagari.yaml")
            self.transliterator.load_schema_file("schemas/iast.yaml")
            self.available = True
            return True
        except ImportError as e:
            self.error_message = f"Shlesha Python bindings not found: {e}"
            return False
            
    def transliterate(self, text: str, from_script: str, to_script: str) -> str:
        return self.transliterator.transliterate(text, from_script, to_script)

class VidyutWrapper(TransliteratorWrapper):
    def __init__(self):
        super().__init__("Vidyut (Python)")
        
    def setup(self) -> bool:
        try:
            import vidyut_py
            self.module = vidyut_py
            self.available = True
            return True
        except ImportError as e:
            self.error_message = f"vidyut-py not found: {e}"
            return False
            
    def transliterate(self, text: str, from_script: str, to_script: str) -> str:
        # Map script names to Vidyut's format
        script_map = {
            "Devanagari": self.module.Scheme.Devanagari,
            "IAST": self.module.Scheme.Iast
        }
        return self.module.transliterate(text, script_map[from_script], script_map[to_script])

class DharmamitraWrapper(TransliteratorWrapper):
    def __init__(self):
        super().__init__("dharmamitra")
        
    def setup(self) -> bool:
        try:
            import dharmamitra
            self.module = dharmamitra
            self.available = True
            return True
        except ImportError as e:
            self.error_message = f"dharmamitra not found. Install: pip install dharmamitra"
            return False
            
    def transliterate(self, text: str, from_script: str, to_script: str) -> str:
        # Map script names to dharmamitra's format
        script_map = {
            "Devanagari": "devanagari",
            "IAST": "iast"
        }
        return self.module.transliterate(text, script_map[from_script], script_map[to_script])

class AksharamukhaWrapper(TransliteratorWrapper):
    def __init__(self):
        super().__init__("Aksharamukha")
        
    def setup(self) -> bool:
        try:
            from aksharamukha import transliterate
            self.transliterate_func = transliterate.process
            self.available = True
            return True
        except ImportError as e:
            self.error_message = f"aksharamukha not found. Install: pip install aksharamukha"
            return False
            
    def transliterate(self, text: str, from_script: str, to_script: str) -> str:
        return self.transliterate_func(from_script, to_script, text)

class IndicTransliterationWrapper(TransliteratorWrapper):
    def __init__(self):
        super().__init__("indic-transliteration")
        
    def setup(self) -> bool:
        try:
            from indic_transliteration import sanscript
            self.sanscript = sanscript
            self.available = True
            return True
        except ImportError as e:
            self.error_message = f"indic-transliteration not found. Install: pip install indic-transliteration"
            return False
            
    def transliterate(self, text: str, from_script: str, to_script: str) -> str:
        # Map script names
        script_map = {
            "Devanagari": self.sanscript.DEVANAGARI,
            "IAST": self.sanscript.IAST
        }
        return self.sanscript.transliterate(text, script_map[from_script], script_map[to_script])

def benchmark_function(func: Callable, args: tuple, iterations: int = 100) -> Dict[str, float]:
    """Benchmark a function with warmup"""
    # Warmup
    for _ in range(min(10, iterations // 10)):
        try:
            func(*args)
        except:
            pass
    
    # Actual benchmark
    times = []
    errors = 0
    
    for _ in range(iterations):
        try:
            start = time.perf_counter()
            result = func(*args)
            end = time.perf_counter()
            times.append(end - start)
        except Exception as e:
            errors += 1
    
    if not times:
        return {"error": "All runs failed", "error_count": errors}
    
    times.sort()
    return {
        'mean': statistics.mean(times),
        'median': statistics.median(times),
        'stdev': statistics.stdev(times) if len(times) > 1 else 0,
        'min': min(times),
        'max': max(times),
        'p95': times[int(len(times) * 0.95)] if times else 0,
        'p99': times[int(len(times) * 0.99)] if times else 0,
        'success_rate': (iterations - errors) / iterations
    }

def run_benchmarks(wrappers: List[TransliteratorWrapper]) -> List[Dict[str, Any]]:
    """Run benchmarks for all available transliterators"""
    results = []
    
    for wrapper in wrappers:
        if not wrapper.available:
            continue
            
        print(f"\nBenchmarking {wrapper.name}...")
        
        for text, word_count, desc in TEST_CORPUS:
            print(f"  Testing {desc} ({len(text)} chars)...", end='', flush=True)
            
            stats = benchmark_function(
                wrapper.transliterate,
                (text, "Devanagari", "IAST"),
                iterations=50 if len(text) < 1000 else 20
            )
            
            if "error" not in stats:
                # Calculate throughput
                chars_per_sec = len(text) / stats['mean']
                words_per_sec = word_count / stats['mean']
                
                result = {
                    'tool': wrapper.name,
                    'description': desc,
                    'text_length': len(text),
                    'word_count': word_count,
                    'chars_per_sec': chars_per_sec,
                    'words_per_sec': words_per_sec,
                    **stats
                }
                results.append(result)
                print(f" {stats['mean']*1000:.2f}ms")
            else:
                print(f" FAILED")
    
    return results

def test_accuracy(wrappers: List[TransliteratorWrapper]) -> Dict[str, List[Dict[str, Any]]]:
    """Test accuracy of transliterations"""
    accuracy_results = {}
    
    for wrapper in wrappers:
        if not wrapper.available:
            continue
            
        print(f"\nTesting accuracy for {wrapper.name}...")
        wrapper_results = []
        
        for devanagari, expected in ACCURACY_TEST_CASES:
            try:
                result = wrapper.transliterate(devanagari, "Devanagari", "IAST")
                correct = result == expected
                wrapper_results.append({
                    'input': devanagari,
                    'expected': expected,
                    'actual': result,
                    'correct': correct
                })
                print(f"  {devanagari} → {result} {'✓' if correct else '✗ (expected: ' + expected + ')'}")
            except Exception as e:
                wrapper_results.append({
                    'input': devanagari,
                    'expected': expected,
                    'actual': f"ERROR: {e}",
                    'correct': False
                })
                print(f"  {devanagari} → ERROR: {e}")
        
        accuracy_rate = sum(1 for r in wrapper_results if r['correct']) / len(wrapper_results)
        print(f"  Accuracy: {accuracy_rate*100:.1f}%")
        
        accuracy_results[wrapper.name] = wrapper_results
    
    return accuracy_results

def print_summary(results: List[Dict[str, Any]], accuracy_results: Dict[str, List[Dict[str, Any]]]):
    """Print summary of benchmark results"""
    if not results:
        print("\nNo benchmark results to summarize.")
        return
    
    print("\n" + "="*80)
    print("PERFORMANCE SUMMARY")
    print("="*80)
    
    # Group by tool
    tools = {}
    for r in results:
        tool = r['tool']
        if tool not in tools:
            tools[tool] = []
        tools[tool].append(r)
    
    # Calculate averages for each tool
    tool_stats = []
    for tool, tool_results in tools.items():
        avg_chars_per_sec = statistics.mean(r['chars_per_sec'] for r in tool_results)
        avg_time_ms = statistics.mean(r['mean'] * 1000 for r in tool_results)
        
        # Get accuracy
        accuracy = 0
        if tool in accuracy_results:
            accuracy = sum(1 for r in accuracy_results[tool] if r['correct']) / len(accuracy_results[tool])
        
        tool_stats.append({
            'tool': tool,
            'avg_chars_per_sec': avg_chars_per_sec,
            'avg_time_ms': avg_time_ms,
            'accuracy': accuracy
        })
    
    # Sort by performance
    tool_stats.sort(key=lambda x: x['avg_chars_per_sec'], reverse=True)
    
    # Print table
    print(f"\n{'Tool':<25} {'Avg Time':<12} {'Throughput':<20} {'Accuracy':<10}")
    print("-" * 70)
    
    for stats in tool_stats:
        print(f"{stats['tool']:<25} {stats['avg_time_ms']:>10.2f}ms "
              f"{stats['avg_chars_per_sec']:>15,.0f} c/s {stats['accuracy']:>8.1%}")
    
    # Calculate relative performance
    if tool_stats:
        baseline = tool_stats[0]  # Fastest tool
        print(f"\nRelative Performance (vs {baseline['tool']}):")
        for stats in tool_stats[1:]:
            speedup = baseline['avg_chars_per_sec'] / stats['avg_chars_per_sec']
            print(f"  {stats['tool']}: {speedup:.2f}x slower")

def save_results(results: List[Dict[str, Any]], accuracy_results: Dict[str, List[Dict[str, Any]]]):
    """Save results to JSON file"""
    output = {
        'timestamp': time.strftime('%Y-%m-%d %H:%M:%S'),
        'platform': sys.platform,
        'python_version': sys.version,
        'performance_results': results,
        'accuracy_results': accuracy_results
    }
    
    with open('bench_data/python_benchmark_results.json', 'w') as f:
        json.dump(output, f, indent=2)
    
    print(f"\nResults saved to bench_data/python_benchmark_results.json")

def main():
    print("Comprehensive Python Transliteration Library Benchmark")
    print("=" * 80)
    
    # Initialize all wrappers
    wrappers = [
        ShleshaWrapper(),
        VidyutWrapper(),
        DharmamitraWrapper(),
        AksharamukhaWrapper(),
        IndicTransliterationWrapper(),
    ]
    
    # Setup transliterators
    print("\nSetting up transliterators...")
    available_count = 0
    for wrapper in wrappers:
        if wrapper.setup():
            print(f"  ✓ {wrapper.name}")
            available_count += 1
        else:
            print(f"  ✗ {wrapper.name}: {wrapper.error_message}")
    
    if available_count == 0:
        print("\nNo transliterators available for benchmarking!")
        return
    
    print(f"\n{available_count} transliterators available for benchmarking.")
    
    # Run performance benchmarks
    print("\n" + "="*80)
    print("PERFORMANCE BENCHMARKS")
    print("="*80)
    results = run_benchmarks(wrappers)
    
    # Test accuracy
    print("\n" + "="*80)
    print("ACCURACY TESTS")
    print("="*80)
    accuracy_results = test_accuracy(wrappers)
    
    # Print summary
    print_summary(results, accuracy_results)
    
    # Save results
    save_results(results, accuracy_results)

if __name__ == "__main__":
    main()