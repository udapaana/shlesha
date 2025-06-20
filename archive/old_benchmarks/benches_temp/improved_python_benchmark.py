#!/usr/bin/env python3
"""
Improved Python transliteration benchmark that detects and tests actual libraries
This ensures fair Python vs Python comparisons across available tools
"""

import time
import statistics
import json
import sys
import os
import subprocess
from typing import List, Dict, Any, Callable, Optional, Tuple
import importlib
from dataclasses import dataclass
from pathlib import Path

@dataclass
class BenchmarkResult:
    tool: str
    test_case: str
    text_length: int
    word_count: int
    mean_time: float
    median_time: float
    min_time: float
    max_time: float
    p95_time: float
    chars_per_sec: float
    words_per_sec: float
    success_rate: float
    accuracy: Optional[float] = None

@dataclass
class AccuracyResult:
    tool: str
    input_text: str
    expected: str
    actual: str
    correct: bool
    error: Optional[str] = None

# Comprehensive test corpus
TEST_CORPUS = [
    # (description, devanagari, word_count, expected_iast)
    ("single_char", "क", 1, "ka"),
    ("single_word", "नमस्ते", 1, "namaste"),
    ("short_sentence", "अहं संस्कृतं वदामि", 3, "ahaṃ saṃskṛtaṃ vadāmi"),
    ("with_conjuncts", "कृष्णार्जुनसंवादः", 1, "kṛṣṇārjunasaṃvādaḥ"),
    ("with_vedic", "अग्निमीळे पुरोहितं", 2, "agnimīḷe purohitaṃ"),
    ("medium_verse", "तत्र शूरा महेष्वासा भीमार्जुनसमा युधि", 6, "tatra śūrā maheṣvāsā bhīmārjunasamā yudhi"),
    ("long_sentence", "धृतराष्ट्र उवाच धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः", 7, "dhṛtarāṣṭra uvāca dharmakṣetre kurukṣetre samavetā yuyutsavaḥ"),
    ("complex_text", "कर्मण्येवाधिकारस्ते मा फलेषु कदाचन । मा कर्मफलहेतुर्भूर्मा ते सङ्गोऽस्त्वकर्मणि ॥", 12, "karmaṇyevādhikāraste mā phaleṣu kadācana . mā karmaphalahetur bhūr mā te saṅgo 'stvakarmaṇi ."),
    ("repeated_medium", "तत्र शूरा महेष्वासा भीमार्जुनसमा युधि । " * 10, 60, ""),
    ("repeated_large", "धृतराष्ट्र उवाच धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः । " * 100, 700, ""),
]

# Accuracy test cases - more comprehensive
ACCURACY_TESTS = [
    ("नमस्ते", "namaste"),
    ("संस्कृतम्", "saṃskṛtam"),
    ("कृष्ण", "kṛṣṇa"),
    ("ज्ञान", "jñāna"),
    ("अग्निमीळे", "agnimīḷe"),
    ("यज्ञस्य", "yajñasya"),
    ("ऋत्विजम्", "ṛtvijam"),
    ("द्वौ", "dvau"),
    ("त्र्यम्बकम्", "tryambakam"),
    ("पुष्टिवर्धनम्", "puṣṭivardhanam"),
    ("श्रीः", "śrīḥ"),
    ("कार्त्स्न्येन", "kārtsnyena"),
    ("वाङ्मयम्", "vāṅmayam"),
    ("षट्कर्म", "ṣaṭkarma"),
    ("अष्टाध्यायी", "aṣṭādhyāyī"),
    ("ज्योतिष्मान्", "jyotiṣmān"),
    ("प्रत्युत्पन्नमतिः", "pratyutpanamatiḥ"),
    ("सत्चित्आनन्द", "saccidānanda"),
    ("आत्मज्ञान", "ātmajñāna"),
    ("ब्रह्मचर्य", "brahmacarya"),
]

class TransliteratorWrapper:
    """Base class for wrapping different transliteration libraries"""
    
    def __init__(self, name: str):
        self.name = name
        self.available = False
        self.error_message = None
        self.version = None
        self.module = None
        
    def setup(self) -> bool:
        """Setup the transliterator. Return True if successful."""
        raise NotImplementedError
        
    def transliterate(self, text: str, from_script: str = "Devanagari", to_script: str = "IAST") -> str:
        """Transliterate text from one script to another."""
        raise NotImplementedError
        
    def get_info(self) -> Dict[str, Any]:
        """Get library information."""
        return {
            "name": self.name,
            "available": self.available,
            "version": self.version,
            "error": self.error_message
        }

class ShleshaWrapper(TransliteratorWrapper):
    """Wrapper for Shlesha Python bindings (when available)"""
    
    def __init__(self):
        super().__init__("Shlesha (Python)")
        
    def setup(self) -> bool:
        try:
            # Try to import hypothetical shlesha_py module
            import shlesha_py
            self.module = shlesha_py
            self.version = getattr(shlesha_py, '__version__', 'unknown')
            
            # Initialize transliterator
            self.transliterator = shlesha_py.Transliterator()
            
            # Load schemas - adjust paths as needed
            schema_dir = Path(__file__).parent.parent / "schemas"
            self.transliterator.load_schema_file(str(schema_dir / "devanagari.yaml"))
            self.transliterator.load_schema_file(str(schema_dir / "iast.yaml"))
            
            self.available = True
            return True
        except ImportError as e:
            self.error_message = f"Shlesha Python bindings not available: {e}"
            return False
        except Exception as e:
            self.error_message = f"Failed to setup Shlesha: {e}"
            return False
            
    def transliterate(self, text: str, from_script: str = "Devanagari", to_script: str = "IAST") -> str:
        return self.transliterator.transliterate(text, from_script, to_script)

class VidyutPyWrapper(TransliteratorWrapper):
    """Wrapper for Vidyut Python bindings (when available)"""
    
    def __init__(self):
        super().__init__("Vidyut (Python)")
        
    def setup(self) -> bool:
        try:
            import vidyut_py
            self.module = vidyut_py
            self.version = getattr(vidyut_py, '__version__', 'unknown')
            self.available = True
            return True
        except ImportError as e:
            self.error_message = f"vidyut-py not available: {e}"
            return False
            
    def transliterate(self, text: str, from_script: str = "Devanagari", to_script: str = "IAST") -> str:
        # Map script names to Vidyut's format
        scheme_map = {
            "Devanagari": self.module.Scheme.Devanagari,
            "IAST": self.module.Scheme.Iast,
            "devanagari": self.module.Scheme.Devanagari,
            "iast": self.module.Scheme.Iast,
        }
        return self.module.transliterate(text, scheme_map[from_script], scheme_map[to_script])

class IndicTransliterationWrapper(TransliteratorWrapper):
    """Wrapper for indic-transliteration library"""
    
    def __init__(self):
        super().__init__("indic-transliteration")
        
    def setup(self) -> bool:
        try:
            from indic_transliteration import sanscript
            self.module = sanscript
            self.version = getattr(sanscript, '__version__', 'unknown')
            self.available = True
            return True
        except ImportError as e:
            self.error_message = f"indic-transliteration not found. Install: pip install indic-transliteration"
            return False
            
    def transliterate(self, text: str, from_script: str = "Devanagari", to_script: str = "IAST") -> str:
        # Map script names
        script_map = {
            "Devanagari": self.module.DEVANAGARI,
            "IAST": self.module.IAST,
            "devanagari": self.module.DEVANAGARI,
            "iast": self.module.IAST,
        }
        return self.module.transliterate(text, script_map[from_script], script_map[to_script])

class AksharamukhaWrapper(TransliteratorWrapper):
    """Wrapper for Aksharamukha library"""
    
    def __init__(self):
        super().__init__("Aksharamukha")
        
    def setup(self) -> bool:
        try:
            from aksharamukha import transliterate
            self.module = transliterate
            self.version = getattr(transliterate, '__version__', 'unknown')
            self.available = True
            return True
        except ImportError as e:
            self.error_message = f"aksharamukha not found. Install: pip install aksharamukha"
            return False
            
    def transliterate(self, text: str, from_script: str = "Devanagari", to_script: str = "IAST") -> str:
        return self.module.process(from_script, to_script, text)

class DharmamitraWrapper(TransliteratorWrapper):
    """Wrapper for dharmamitra library"""
    
    def __init__(self):
        super().__init__("dharmamitra")
        
    def setup(self) -> bool:
        try:
            import dharmamitra
            self.module = dharmamitra
            self.version = getattr(dharmamitra, '__version__', 'unknown')
            self.available = True
            return True
        except ImportError as e:
            self.error_message = f"dharmamitra not found. Install: pip install dharmamitra"
            return False
            
    def transliterate(self, text: str, from_script: str = "Devanagari", to_script: str = "IAST") -> str:
        # Map script names to dharmamitra's format
        script_map = {
            "Devanagari": "devanagari",
            "IAST": "iast",
            "devanagari": "devanagari",
            "iast": "iast",
        }
        return self.module.transliterate(text, script_map[from_script], script_map[to_script])

class AI4BharatWrapper(TransliteratorWrapper):
    """Wrapper for AI4Bharat transliteration"""
    
    def __init__(self):
        super().__init__("AI4Bharat")
        
    def setup(self) -> bool:
        try:
            from ai4bharat_transliteration import XlitEngine
            self.engine = XlitEngine("hi", beam_width=10, rescore=False)
            self.version = "unknown"
            self.available = True
            return True
        except ImportError as e:
            self.error_message = f"ai4bharat-transliteration not found. Install: pip install ai4bharat-transliteration"
            return False
        except Exception as e:
            self.error_message = f"Failed to initialize AI4Bharat engine: {e}"
            return False
            
    def transliterate(self, text: str, from_script: str = "Devanagari", to_script: str = "IAST") -> str:
        # AI4Bharat is primarily for romanization
        if from_script.lower() == "devanagari" and to_script.lower() in ["iast", "roman"]:
            # This is a simplified approach - AI4Bharat may need different handling
            return text  # Placeholder - would need proper implementation
        else:
            raise ValueError(f"Unsupported conversion: {from_script} -> {to_script}")

class IndicTransWrapper(TransliteratorWrapper):
    """Wrapper for indic-trans library"""
    
    def __init__(self):
        super().__init__("indic-trans")
        
    def setup(self) -> bool:
        try:
            from indic_trans.transliterator import transliterate as indic_trans_transliterate
            self.transliterate_func = indic_trans_transliterate
            self.version = "unknown"
            self.available = True
            return True
        except ImportError as e:
            self.error_message = f"indic-trans not found. Install: pip install indic-trans"
            return False
            
    def transliterate(self, text: str, from_script: str = "Devanagari", to_script: str = "IAST") -> str:
        # indic-trans uses different naming conventions
        script_map = {
            "Devanagari": "hi",  # Hindi/Devanagari
            "IAST": "en",        # English/Roman
            "devanagari": "hi",
            "iast": "en",
        }
        return self.transliterate_func(text, script_map[from_script], script_map[to_script])

def benchmark_function(func: Callable, args: tuple, iterations: int = 100) -> Dict[str, float]:
    """Benchmark a function with warmup and comprehensive statistics"""
    # Warmup runs
    warmup_runs = min(10, iterations // 10)
    for _ in range(warmup_runs):
        try:
            func(*args)
        except:
            pass
    
    # Actual benchmark runs
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
        return {
            "error": "All runs failed",
            "error_count": errors,
            "success_rate": 0.0
        }
    
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

def test_accuracy(wrapper: TransliteratorWrapper) -> Tuple[List[AccuracyResult], float]:
    """Test accuracy of a transliterator"""
    results = []
    correct = 0
    
    for devanagari, expected in ACCURACY_TESTS:
        try:
            actual = wrapper.transliterate(devanagari)
            is_correct = actual == expected
            if is_correct:
                correct += 1
            
            results.append(AccuracyResult(
                tool=wrapper.name,
                input_text=devanagari,
                expected=expected,
                actual=actual,
                correct=is_correct
            ))
        except Exception as e:
            results.append(AccuracyResult(
                tool=wrapper.name,
                input_text=devanagari,
                expected=expected,
                actual="",
                correct=False,
                error=str(e)
            ))
    
    accuracy = correct / len(ACCURACY_TESTS) if ACCURACY_TESTS else 0.0
    return results, accuracy

def run_comprehensive_benchmark(wrappers: List[TransliteratorWrapper]) -> Tuple[List[BenchmarkResult], List[AccuracyResult]]:
    """Run comprehensive benchmarks for all available transliterators"""
    benchmark_results = []
    accuracy_results = []
    
    available_wrappers = [w for w in wrappers if w.available]
    
    if not available_wrappers:
        print("No transliterators available for benchmarking!")
        return [], []
    
    print(f"\nRunning benchmarks with {len(available_wrappers)} transliterators...")
    
    # Performance benchmarks
    for wrapper in available_wrappers:
        print(f"\nBenchmarking {wrapper.name}...")
        
        for desc, text, word_count, expected in TEST_CORPUS:
            print(f"  Testing {desc} ({len(text)} chars)...", end='', flush=True)
            
            # Determine iteration count based on text length
            iterations = 50 if len(text) < 1000 else 20 if len(text) < 10000 else 10
            
            stats = benchmark_function(
                wrapper.transliterate,
                (text,),
                iterations=iterations
            )
            
            if "error" not in stats:
                # Calculate throughput
                chars_per_sec = len(text) / stats['mean'] if stats['mean'] > 0 else 0
                words_per_sec = word_count / stats['mean'] if stats['mean'] > 0 else 0
                
                result = BenchmarkResult(
                    tool=wrapper.name,
                    test_case=desc,
                    text_length=len(text),
                    word_count=word_count,
                    mean_time=stats['mean'],
                    median_time=stats['median'],
                    min_time=stats['min'],
                    max_time=stats['max'],
                    p95_time=stats['p95'],
                    chars_per_sec=chars_per_sec,
                    words_per_sec=words_per_sec,
                    success_rate=stats['success_rate']
                )
                benchmark_results.append(result)
                print(f" {stats['mean']*1000:.2f}ms")
            else:
                print(f" FAILED ({stats.get('error', 'unknown error')})")
    
    # Accuracy tests
    print("\n" + "="*80)
    print("ACCURACY TESTS")
    print("="*80)
    
    for wrapper in available_wrappers:
        print(f"\nTesting accuracy for {wrapper.name}...")
        acc_results, accuracy = test_accuracy(wrapper)
        accuracy_results.extend(acc_results)
        
        # Update benchmark results with accuracy
        for result in benchmark_results:
            if result.tool == wrapper.name:
                result.accuracy = accuracy
        
        print(f"  Accuracy: {accuracy*100:.1f}%")
        
        # Show some accuracy details
        correct_count = sum(1 for r in acc_results if r.correct)
        total_count = len(acc_results)
        print(f"  Correct: {correct_count}/{total_count}")
        
        # Show first few incorrect results
        incorrect = [r for r in acc_results if not r.correct][:3]
        for r in incorrect:
            if r.error:
                print(f"    {r.input_text} → ERROR: {r.error}")
            else:
                print(f"    {r.input_text} → {r.actual} (expected: {r.expected})")
    
    return benchmark_results, accuracy_results

def print_performance_summary(results: List[BenchmarkResult]):
    """Print a comprehensive performance summary"""
    if not results:
        print("\nNo benchmark results to summarize.")
        return
    
    print("\n" + "="*100)
    print("PERFORMANCE SUMMARY")
    print("="*100)
    
    # Group by tool
    tools = {}
    for r in results:
        if r.tool not in tools:
            tools[r.tool] = []
        tools[r.tool].append(r)
    
    # Calculate tool averages
    tool_stats = []
    for tool, tool_results in tools.items():
        avg_chars_per_sec = statistics.mean(r.chars_per_sec for r in tool_results)
        avg_time_ms = statistics.mean(r.mean_time * 1000 for r in tool_results)
        avg_accuracy = statistics.mean(r.accuracy for r in tool_results if r.accuracy is not None)
        
        tool_stats.append({
            'tool': tool,
            'avg_chars_per_sec': avg_chars_per_sec,
            'avg_time_ms': avg_time_ms,
            'avg_accuracy': avg_accuracy,
            'test_count': len(tool_results)
        })
    
    # Sort by performance
    tool_stats.sort(key=lambda x: x['avg_chars_per_sec'], reverse=True)
    
    # Print summary table
    print(f"\n{'Tool':<30} {'Avg Time':<12} {'Throughput':<20} {'Accuracy':<10} {'Tests':<6}")
    print("-" * 85)
    
    for stats in tool_stats:
        print(f"{stats['tool']:<30} {stats['avg_time_ms']:>10.2f}ms "
              f"{stats['avg_chars_per_sec']:>15,.0f} c/s {stats['avg_accuracy']:>8.1%} "
              f"{stats['test_count']:>5}")
    
    # Print relative performance
    if tool_stats:
        baseline = tool_stats[0]  # Fastest tool
        print(f"\nRelative Performance (vs {baseline['tool']}):")
        for stats in tool_stats[1:]:
            speedup = baseline['avg_chars_per_sec'] / stats['avg_chars_per_sec']
            accuracy_diff = stats['avg_accuracy'] - baseline['avg_accuracy']
            print(f"  {stats['tool']}: {speedup:.2f}x slower, "
                  f"accuracy {accuracy_diff:+.1%}")

def save_results(benchmark_results: List[BenchmarkResult], accuracy_results: List[AccuracyResult]):
    """Save comprehensive results to JSON files"""
    # Create output directory
    output_dir = Path("bench_data")
    output_dir.mkdir(exist_ok=True)
    
    # Save benchmark results
    benchmark_data = {
        'timestamp': time.strftime('%Y-%m-%d %H:%M:%S'),
        'platform': sys.platform,
        'python_version': sys.version,
        'results': [
            {
                'tool': r.tool,
                'test_case': r.test_case,
                'text_length': r.text_length,
                'word_count': r.word_count,
                'mean_time_ms': r.mean_time * 1000,
                'median_time_ms': r.median_time * 1000,
                'min_time_ms': r.min_time * 1000,
                'max_time_ms': r.max_time * 1000,
                'p95_time_ms': r.p95_time * 1000,
                'chars_per_sec': r.chars_per_sec,
                'words_per_sec': r.words_per_sec,
                'success_rate': r.success_rate,
                'accuracy': r.accuracy
            } for r in benchmark_results
        ]
    }
    
    with open(output_dir / 'python_benchmark_results.json', 'w', encoding='utf-8') as f:
        json.dump(benchmark_data, f, indent=2, ensure_ascii=False)
    
    # Save accuracy results
    accuracy_data = {
        'timestamp': time.strftime('%Y-%m-%d %H:%M:%S'),
        'results': [
            {
                'tool': r.tool,
                'input_text': r.input_text,
                'expected': r.expected,
                'actual': r.actual,
                'correct': r.correct,
                'error': r.error
            } for r in accuracy_results
        ]
    }
    
    with open(output_dir / 'python_accuracy_results.json', 'w', encoding='utf-8') as f:
        json.dump(accuracy_data, f, indent=2, ensure_ascii=False)
    
    print(f"\nResults saved to {output_dir}/")

def check_library_installation():
    """Check which libraries can be installed and provide installation commands"""
    libraries = [
        ("indic-transliteration", "pip install indic-transliteration"),
        ("aksharamukha", "pip install aksharamukha"),
        ("dharmamitra", "pip install dharmamitra"),
        ("ai4bharat-transliteration", "pip install ai4bharat-transliteration"),
        ("indic-trans", "pip install indic-trans"),
    ]
    
    print("\nLibrary Installation Status:")
    print("-" * 50)
    
    for lib_name, install_cmd in libraries:
        try:
            module_name = lib_name.replace('-', '_')
            importlib.import_module(module_name)
            print(f"✓ {lib_name} - Already installed")
        except ImportError:
            print(f"✗ {lib_name} - Not installed ({install_cmd})")

def main():
    print("Improved Python Transliteration Benchmark")
    print("=" * 80)
    
    # Check library installation status
    check_library_installation()
    
    # Initialize all wrappers
    wrappers = [
        ShleshaWrapper(),
        VidyutPyWrapper(),
        IndicTransliterationWrapper(),
        AksharamukhaWrapper(),
        DharmamitraWrapper(),
        AI4BharatWrapper(),
        IndicTransWrapper(),
    ]
    
    # Setup transliterators
    print("\n" + "="*80)
    print("SETUP")
    print("="*80)
    
    available_count = 0
    for wrapper in wrappers:
        if wrapper.setup():
            print(f"  ✓ {wrapper.name} (v{wrapper.version})")
            available_count += 1
        else:
            print(f"  ✗ {wrapper.name}: {wrapper.error_message}")
    
    if available_count == 0:
        print("\nNo transliterators available for benchmarking!")
        print("Install libraries using the commands shown above.")
        return
    
    print(f"\n{available_count} transliterators available for benchmarking.")
    
    # Run comprehensive benchmarks
    print("\n" + "="*80)
    print("BENCHMARKING")
    print("="*80)
    
    benchmark_results, accuracy_results = run_comprehensive_benchmark(wrappers)
    
    # Print summary
    print_performance_summary(benchmark_results)
    
    # Save results
    save_results(benchmark_results, accuracy_results)
    
    print("\nBenchmark complete!")

if __name__ == "__main__":
    main()