#!/usr/bin/env python3
"""
Simple Fair Benchmark: Shlesha vs Other Transliteration Tools
Provides platform-specific comparisons (Python vs Python, CLI vs CLI)
"""

import time
import subprocess
import sys
import os
from typing import Dict, List, Tuple, Optional

# Test cases with expected SLP1 output
TEST_CASES = [
    ("क", "k"),
    ("कर", "kr"), 
    ("कर्म", "krm"),
    ("धर्म", "Drm"),
    ("संस्कृत", "sAskft"),
    ("प्रकृति", "prakft"), 
    ("भगवद्गीता", "BagavdgItA"),
    ("नमस्ते", "namaste"),
    ("योग", "yog"),
    ("गुरु", "guru"),
]

def benchmark_function(func, *args, iterations=100):
    """Benchmark a function with given arguments"""
    # Warmup
    for _ in range(min(10, iterations // 10)):
        try:
            func(*args)
        except:
            pass
    
    # Actual benchmark
    times = []
    for _ in range(iterations):
        start = time.perf_counter()
        try:
            result = func(*args)
            end = time.perf_counter()
            times.append(end - start)
        except Exception as e:
            # Count failures but don't include timing
            pass
    
    if not times:
        return None
    
    times.sort()
    return {
        'mean': sum(times) / len(times),
        'median': times[len(times) // 2],
        'min': min(times),
        'max': max(times),
        'success_rate': len(times) / iterations
    }

class ToolBenchmark:
    def __init__(self, name: str):
        self.name = name
        self.available = False
        self.version = "unknown"
        self.setup_error = None
        
    def setup(self) -> bool:
        """Setup the tool. Return True if successful."""
        return False
        
    def transliterate(self, text: str) -> str:
        """Transliterate text. Should be implemented by subclasses."""
        raise NotImplementedError
        
    def test_accuracy(self) -> Dict[str, float]:
        """Test accuracy on standard test cases"""
        correct = 0
        total = 0
        details = []
        
        for devanagari, expected_slp1 in TEST_CASES:
            try:
                result = self.transliterate(devanagari).strip()
                total += 1
                is_correct = result == expected_slp1
                if is_correct:
                    correct += 1
                    
                details.append({
                    'input': devanagari,
                    'expected': expected_slp1,
                    'actual': result,
                    'correct': is_correct
                })
            except Exception as e:
                total += 1
                details.append({
                    'input': devanagari,
                    'expected': expected_slp1,
                    'actual': f"ERROR: {str(e)}",
                    'correct': False
                })
        
        accuracy = (correct / total * 100) if total > 0 else 0
        return {
            'accuracy': accuracy,
            'correct': correct,
            'total': total,
            'details': details
        }
    
    def benchmark_performance(self) -> Dict[str, float]:
        """Benchmark performance on test cases"""
        results = {}
        
        # Test different text sizes
        test_texts = {
            'single_char': 'क',
            'word': 'संस्कृत',
            'sentence': 'अहं संस्कृतं वदामि',
            'large_text': 'कर्म धर्म योग गुरु शांति प्रकृति संस्कृत वेद उपनिषद् भगवद्गीता रामायण महाभारत' * 10
        }
        
        for test_name, text in test_texts.items():
            iterations = 100 if len(text) < 100 else 20
            stats = benchmark_function(self.transliterate, text, iterations=iterations)
            
            if stats:
                chars_per_sec = len(text) / stats['mean'] if stats['mean'] > 0 else 0
                results[test_name] = {
                    'mean_time_ms': stats['mean'] * 1000,
                    'chars_per_sec': chars_per_sec,
                    'success_rate': stats['success_rate']
                }
            else:
                results[test_name] = {
                    'mean_time_ms': float('inf'),
                    'chars_per_sec': 0,
                    'success_rate': 0
                }
        
        return results

class ShleshaCLI(ToolBenchmark):
    def __init__(self):
        super().__init__("Shlesha CLI")
        
    def setup(self) -> bool:
        try:
            # Build CLI example
            result = subprocess.run(['cargo', 'build', '--release', '--example', 'cli'], 
                                  cwd='.', capture_output=True, text=True, timeout=120)
            if result.returncode == 0:
                cli_path = './target/release/examples/cli'
                if os.path.exists(cli_path):
                    self.cli_path = cli_path
                    self.available = True
                    self.version = "built from source"
                    return True
            
            self.setup_error = f"Build failed: {result.stderr}"
            return False
        except Exception as e:
            self.setup_error = str(e)
            return False
    
    def transliterate(self, text: str) -> str:
        if not self.available:
            raise Exception("Tool not available")
            
        result = subprocess.run([self.cli_path, '-f', 'devanagari', '-t', 'slp1', text],
                              capture_output=True, text=True, timeout=10)
        if result.returncode == 0:
            return result.stdout.strip()
        else:
            raise Exception(f"CLI error: {result.stderr}")

class IndicTransliterationPython(ToolBenchmark):
    def __init__(self):
        super().__init__("indic-transliteration (Python)")
        
    def setup(self) -> bool:
        try:
            import sys
            from indic_transliteration import sanscript
            self.sanscript = sanscript
            self.available = True
            
            # Try to get version
            try:
                import indic_transliteration
                self.version = getattr(indic_transliteration, '__version__', 'unknown')
            except:
                self.version = "unknown"
                
            return True
        except ImportError as e:
            self.setup_error = f"Import failed: {e}. Install with: pip install indic-transliteration"
            return False
        except Exception as e:
            self.setup_error = str(e)
            return False
    
    def transliterate(self, text: str) -> str:
        if not self.available:
            raise Exception("Tool not available")
            
        try:
            # Map to SLP1 (closest equivalent)
            return self.sanscript.transliterate(text, self.sanscript.DEVANAGARI, self.sanscript.SLP1)
        except Exception as e:
            raise Exception(f"Transliteration error: {e}")

class AksharamukhaPython(ToolBenchmark):
    def __init__(self):
        super().__init__("Aksharamukha (Python)")
        
    def setup(self) -> bool:
        try:
            import aksharamukha
            self.aksharamukha = aksharamukha
            self.available = True
            self.version = getattr(aksharamukha, '__version__', 'unknown')
            return True
        except ImportError as e:
            self.setup_error = f"Import failed: {e}. Install with: pip install aksharamukha"
            return False
        except Exception as e:
            self.setup_error = str(e)
            return False
    
    def transliterate(self, text: str) -> str:
        if not self.available:
            raise Exception("Tool not available")
            
        try:
            # Use SLP1 output
            return self.aksharamukha.transliterate("Devanagari", "slp1", text)
        except Exception as e:
            raise Exception(f"Transliteration error: {e}")

def main():
    print("🚀 Simple Fair Transliteration Benchmark")
    print("=" * 60)
    print()
    
    # Initialize tools
    tools = [
        ShleshaCLI(),
        IndicTransliterationPython(),
        AksharamukhaPython(),
    ]
    
    # Setup tools
    print("🔧 Setting up tools...")
    available_tools = []
    
    for tool in tools:
        if tool.setup():
            print(f"  ✅ {tool.name} - v{tool.version}")
            available_tools.append(tool)
        else:
            print(f"  ❌ {tool.name} - {tool.setup_error}")
    
    if not available_tools:
        print("\n❌ No tools available for benchmarking!")
        sys.exit(1)
    
    print(f"\n📊 Testing {len(available_tools)} available tools")
    print()
    
    # Run accuracy tests
    print("🎯 Accuracy Tests")
    print("-" * 40)
    
    accuracy_results = {}
    for tool in available_tools:
        print(f"\nTesting {tool.name}...")
        acc_result = tool.test_accuracy()
        accuracy_results[tool.name] = acc_result
        
        print(f"  Accuracy: {acc_result['accuracy']:.1f}% ({acc_result['correct']}/{acc_result['total']})")
        
        # Show first few test cases
        for detail in acc_result['details'][:3]:
            status = "✅" if detail['correct'] else "❌"
            print(f"    {status} {detail['input']} → {detail['actual']}")
    
    # Run performance tests  
    print(f"\n⚡ Performance Tests")
    print("-" * 40)
    
    performance_results = {}
    for tool in available_tools:
        print(f"\nBenchmarking {tool.name}...")
        perf_result = tool.benchmark_performance()
        performance_results[tool.name] = perf_result
        
        for test_name, stats in perf_result.items():
            if stats['success_rate'] > 0:
                print(f"  {test_name}: {stats['mean_time_ms']:.2f}ms, {stats['chars_per_sec']:.0f} chars/sec")
            else:
                print(f"  {test_name}: FAILED")
    
    # Generate summary
    print(f"\n📋 Summary")
    print("=" * 60)
    
    # Accuracy ranking
    if accuracy_results:
        print("\n🎯 Accuracy Ranking:")
        sorted_acc = sorted(accuracy_results.items(), key=lambda x: x[1]['accuracy'], reverse=True)
        for i, (name, result) in enumerate(sorted_acc, 1):
            print(f"  {i}. {name}: {result['accuracy']:.1f}%")
    
    # Performance ranking (using word test as representative)
    if performance_results:
        print("\n⚡ Performance Ranking (word test):")
        valid_perf = [(name, result['word']) for name, result in performance_results.items() 
                     if 'word' in result and result['word']['success_rate'] > 0]
        
        if valid_perf:
            sorted_perf = sorted(valid_perf, key=lambda x: x[1]['mean_time_ms'])
            for i, (name, stats) in enumerate(sorted_perf, 1):
                print(f"  {i}. {name}: {stats['mean_time_ms']:.2f}ms ({stats['chars_per_sec']:.0f} chars/sec)")
            
            # Calculate relative performance
            if len(sorted_perf) > 1:
                fastest_time = sorted_perf[0][1]['mean_time_ms']
                print(f"\n📈 Relative Performance:")
                for name, stats in sorted_perf:
                    speedup = stats['mean_time_ms'] / fastest_time
                    print(f"  {name}: {speedup:.1f}x {'(baseline)' if speedup == 1.0 else f'slower than {sorted_perf[0][0]}'}")
    
    # Recommendations
    print(f"\n💡 Recommendations:")
    
    if accuracy_results:
        best_accuracy = max(accuracy_results.items(), key=lambda x: x[1]['accuracy'])
        print(f"  🎯 For highest accuracy: {best_accuracy[0]} ({best_accuracy[1]['accuracy']:.1f}%)")
    
    if valid_perf:
        fastest_tool = min(valid_perf, key=lambda x: x[1]['mean_time_ms'])
        print(f"  ⚡ For fastest processing: {fastest_tool[0]} ({fastest_tool[1]['mean_time_ms']:.2f}ms)")
    
    print(f"\n🔧 Installation commands for missing tools:")
    print(f"  pip install indic-transliteration")
    print(f"  pip install aksharamukha")
    print(f"  cargo install vidyut-cli  # If available")

if __name__ == "__main__":
    main()