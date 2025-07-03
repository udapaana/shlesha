#!/usr/bin/env python3
"""
Fixed Processor Comparison Test
Shows the actual performance of different Shlesha processor implementations
"""

import time
import statistics

def get_test_mappings():
    """Get IAST to ISO mappings for testing."""
    return {
        # Vowels
        "a": "a", "ā": "ā", "i": "i", "ī": "ī", "u": "u", "ū": "ū",
        "ṛ": "r̥", "ṝ": "r̥̄", "ḷ": "l̥", "ḹ": "l̥̄",
        "e": "e", "ai": "ai", "o": "o", "au": "au",
        
        # Consonants  
        "k": "k", "kh": "kh", "g": "g", "gh": "gh", "ṅ": "ṅ",
        "c": "c", "ch": "ch", "j": "j", "jh": "jh", "ñ": "ñ",
        "ṭ": "ṭ", "ṭh": "ṭh", "ḍ": "ḍ", "ḍh": "ḍh", "ṇ": "ṇ",
        "t": "t", "th": "th", "d": "d", "dh": "dh", "n": "n",
        "p": "p", "ph": "ph", "b": "b", "bh": "bh", "m": "m",
        "y": "y", "r": "r", "l": "l", "v": "v",
        "ś": "ś", "ṣ": "ṣ", "s": "s", "h": "h",
        
        # Marks
        "ṃ": "ṁ", "ḥ": "ḥ", "m̐": "m̐", "'": "'",
        
        # Special combinations
        "kṣ": "kṣ", "jñ": "jñ"
    }

def benchmark_raw_processor(processor_func, iterations=5000):
    """Benchmark a raw processor function."""
    times = []
    result = None
    
    for _ in range(iterations):
        start = time.perf_counter()
        result = processor_func()
        times.append(time.perf_counter() - start)
    
    avg_time = statistics.mean(times)
    return {
        'avg_time': avg_time,
        'ops_per_sec': 1 / avg_time,
        'result': result,
        'iterations': iterations
    }

def compare_processors_direct():
    """Compare processors using direct implementation."""
    print("🔬 Direct Processor Implementation Comparison")
    print("=" * 70)
    print("Testing raw processor performance with IAST → ISO conversion")
    print()
    
    # Import what we need
    from shlesha import PyShlesha
    import shlesha
    
    # Test data
    test_cases = [
        ("Simple", "dharma"),
        ("Diacritics", "śrī"),
        ("Medium", "namaskāram"),
        ("Complex", "saṃskṛtabhāṣā"),
        ("Multi-char", "kṣatriya"),
        ("Long", "dharmakṣetre kurukṣetre")
    ]
    
    # Get mappings
    mappings = get_test_mappings()
    
    # Create Shlesha instance
    s = PyShlesha()
    
    print("Running benchmarks...")
    print("-" * 70)
    
    for test_name, test_text in test_cases:
        print(f"\n📝 Test: {test_name}")
        print(f"   Input: '{test_text}'")
        print(f"   Expected: ISO-15919 format")
        print()
        
        # Define processor functions
        def fx_hashmap():
            return s.benchmark_processor(test_text, "fx_hashmap", mappings)
        
        def aho_corasick():
            return s.benchmark_processor(test_text, "aho_corasick", mappings)
        
        def fast_lookup():
            return s.benchmark_processor(test_text, "fast_lookup", mappings)
        
        def standard_api():
            # This uses the production path through converters
            return shlesha.transliterate(test_text, "iast", "iso")
        
        # Benchmark each processor
        processors = [
            ("FxHashMap", fx_hashmap),
            ("Aho-Corasick", aho_corasick),
            ("Fast Lookup", fast_lookup),
            ("Standard API", standard_api)
        ]
        
        results = []
        baseline = None
        
        for name, func in processors:
            try:
                result = benchmark_raw_processor(func, iterations=3000)
                results.append((name, result))
                
                if baseline is None:
                    baseline = result['ops_per_sec']
                
                speedup = result['ops_per_sec'] / baseline
                print(f"   {name:<15}: {result['ops_per_sec']:>8,.0f} ops/sec ({speedup:>5.2f}x) → '{result['result']}'")
            except Exception as e:
                print(f"   {name:<15}: ERROR - {e}")
                results.append((name, {'ops_per_sec': 0, 'result': f'ERROR: {e}'}))
        
        # Find best processor
        valid_results = [(n, r) for n, r in results if r['ops_per_sec'] > 0]
        if valid_results:
            best = max(valid_results, key=lambda x: x[1]['ops_per_sec'])
            print(f"\n   🏆 Best: {best[0]} at {best[1]['ops_per_sec']:,.0f} ops/sec")

def test_production_paths():
    """Test the actual production paths that users would use."""
    print("\n\n🚀 Production Path Performance")
    print("=" * 70)
    print("Testing the actual transliteration paths users would use")
    print()
    
    import shlesha
    from vidyut.lipi import transliterate, Scheme
    
    test_cases = [
        {
            'name': 'IAST → SLP1',
            'text': 'dharmakṣetre kurukṣetre',
            'shlesha_args': ('iast', 'slp1'),
            'vidyut_args': (Scheme.Iast, Scheme.Slp1),
            'note': 'Roman → Roman (should use Aho-Corasick)'
        },
        {
            'name': 'Telugu → Devanagari',
            'text': 'సంస్కృతం',
            'shlesha_args': ('telugu', 'devanagari'),
            'vidyut_args': (Scheme.Telugu, Scheme.Devanagari),
            'note': 'Indic → Indic (hub optimization)'
        }
    ]
    
    for test in test_cases:
        print(f"📝 {test['name']}")
        print(f"   Input: '{test['text']}'")
        print(f"   Note: {test['note']}")
        
        # Benchmark functions
        def shlesha_func():
            return shlesha.transliterate(test['text'], test['shlesha_args'][0], test['shlesha_args'][1])
        
        def vidyut_func():
            return transliterate(test['text'], test['vidyut_args'][0], test['vidyut_args'][1])
        
        # Run benchmarks
        shlesha_result = benchmark_raw_processor(shlesha_func, iterations=5000)
        vidyut_result = benchmark_raw_processor(vidyut_func, iterations=5000)
        
        print(f"   Shlesha: {shlesha_result['ops_per_sec']:>8,.0f} ops/sec → '{shlesha_result['result']}'")
        print(f"   Vidyut:  {vidyut_result['ops_per_sec']:>8,.0f} ops/sec → '{vidyut_result['result']}'")
        
        if shlesha_result['ops_per_sec'] > vidyut_result['ops_per_sec']:
            speedup = shlesha_result['ops_per_sec'] / vidyut_result['ops_per_sec']
            print(f"   🏆 Shlesha is {speedup:.2f}x faster!\n")
        else:
            speedup = vidyut_result['ops_per_sec'] / shlesha_result['ops_per_sec']
            print(f"   🎯 Vidyut is {speedup:.2f}x faster\n")

def main():
    print("🔧 Shlesha Processor Implementation Analysis")
    print("=" * 70)
    print("This test shows the performance of different internal processors")
    print("and how they compare in real-world usage.")
    print()
    
    # First, compare raw processors
    compare_processors_direct()
    
    # Then show production performance
    test_production_paths()
    
    print("\n💡 Key Insights:")
    print("   • The Standard API uses the optimal processor internally")
    print("   • Raw processor benchmarks show implementation differences")
    print("   • Production paths benefit from compile-time optimizations")
    print("   • Different processors excel at different pattern types")

if __name__ == "__main__":
    main()