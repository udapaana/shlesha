#!/usr/bin/env python3
"""
Fair Performance Comparison
Ensures both libraries are doing equivalent work
"""

import time
import statistics
import shlesha
from vidyut.lipi import transliterate, Scheme

def benchmark_function(func, iterations=5000):
    """Benchmark a function."""
    times = []
    result = None
    
    for _ in range(iterations):
        start = time.perf_counter()
        result = func()
        times.append(time.perf_counter() - start)
    
    avg_time = statistics.mean(times)
    return {
        'avg_time': avg_time,
        'ops_per_sec': 1 / avg_time,
        'result': result
    }

def verify_output(input_text, output1, output2, conversion_type):
    """Verify that outputs are equivalent."""
    if output1 == output2:
        return True, "Exact match"
    
    # For IAST → SLP1, check if Shlesha produced ISO instead
    if conversion_type == "IAST → SLP1":
        # Check for telltale signs of ISO vs SLP1
        iso_markers = ['ṁ', 'r̥', 'l̥', 'ā', 'ī', 'ū', 'ś', 'ṣ', 'ñ', 'ṅ']
        slp1_markers = ['M', 'f', 'x', 'A', 'I', 'U', 'S', 'z', 'Y', 'N']
        
        has_iso = any(marker in output1 for marker in iso_markers)
        has_slp1 = any(marker in output2 for marker in slp1_markers)
        
        if has_iso and has_slp1:
            return False, "Shlesha produced ISO-15919, Vidyut produced SLP1"
    
    return False, "Different outputs"

def main():
    print("🎯 Fair Performance Comparison: Shlesha vs Vidyut")
    print("=" * 80)
    print("Testing only conversions where both libraries produce equivalent output")
    print()
    
    test_cases = [
        {
            'name': 'Telugu → Devanagari',
            'input': 'సంస్కృతం',
            'shlesha_args': ('telugu', 'devanagari'),
            'vidyut_args': (Scheme.Telugu, Scheme.Devanagari),
            'type': 'Indic → Indic'
        },
        {
            'name': 'IAST → Devanagari',
            'input': 'saṃskṛtam',
            'shlesha_args': ('iast', 'devanagari'),
            'vidyut_args': (Scheme.Iast, Scheme.Devanagari),
            'type': 'Roman → Indic'
        },
        {
            'name': 'Devanagari → Telugu',
            'input': 'संस्कृतम्',
            'shlesha_args': ('devanagari', 'telugu'),
            'vidyut_args': (Scheme.Devanagari, Scheme.Telugu),
            'type': 'Indic → Indic'
        },
        {
            'name': 'IAST → Telugu',
            'input': 'namaskāram',
            'shlesha_args': ('iast', 'telugu'),
            'vidyut_args': (Scheme.Iast, Scheme.Telugu),
            'type': 'Roman → Indic'
        },
        {
            'name': 'IAST → SLP1',
            'input': 'dharmakṣetre',
            'shlesha_args': ('iast', 'slp1'),
            'vidyut_args': (Scheme.Iast, Scheme.Slp1),
            'type': 'Roman → Roman'
        }
    ]
    
    valid_results = []
    
    print("🔍 Verifying output equivalence first...")
    print("-" * 80)
    
    for test in test_cases:
        # Get outputs
        shlesha_output = shlesha.transliterate(
            test['input'], 
            test['shlesha_args'][0], 
            test['shlesha_args'][1]
        )
        
        vidyut_output = transliterate(
            test['input'],
            test['vidyut_args'][0],
            test['vidyut_args'][1]
        )
        
        # Verify equivalence
        equivalent, reason = verify_output(test['input'], shlesha_output, vidyut_output, test['name'])
        
        print(f"{test['name']}:")
        print(f"  Input:   '{test['input']}'")
        print(f"  Shlesha: '{shlesha_output}'")
        print(f"  Vidyut:  '{vidyut_output}'")
        print(f"  Status:  {'✓ Valid' if equivalent else '✗ Invalid'} - {reason}")
        
        if equivalent:
            valid_results.append(test)
        
        print()
    
    if not valid_results:
        print("❌ No valid comparisons found! The libraries are producing different outputs.")
        return
    
    print(f"\n✅ Found {len(valid_results)} valid comparisons")
    print("\n🏃 Running performance tests on valid comparisons only...")
    print("=" * 80)
    
    summary = []
    
    for test in valid_results:
        print(f"\n{test['name']} ({test['type']})")
        print(f"Input: '{test['input']}'")
        
        # Define benchmark functions
        def shlesha_func():
            return shlesha.transliterate(
                test['input'], 
                test['shlesha_args'][0], 
                test['shlesha_args'][1]
            )
        
        def vidyut_func():
            return transliterate(
                test['input'],
                test['vidyut_args'][0],
                test['vidyut_args'][1]
            )
        
        # Run benchmarks
        shlesha_result = benchmark_function(shlesha_func)
        vidyut_result = benchmark_function(vidyut_func)
        
        print(f"Shlesha: {shlesha_result['ops_per_sec']:>8,.0f} ops/sec")
        print(f"Vidyut:  {vidyut_result['ops_per_sec']:>8,.0f} ops/sec")
        
        if shlesha_result['ops_per_sec'] > vidyut_result['ops_per_sec']:
            speedup = shlesha_result['ops_per_sec'] / vidyut_result['ops_per_sec']
            winner = "Shlesha"
            print(f"🏆 Shlesha wins by {speedup:.2f}x")
        else:
            speedup = vidyut_result['ops_per_sec'] / shlesha_result['ops_per_sec']
            winner = "Vidyut"
            print(f"🎯 Vidyut wins by {speedup:.2f}x")
        
        summary.append({
            'test': test['name'],
            'type': test['type'],
            'winner': winner,
            'speedup': speedup,
            'shlesha_ops': shlesha_result['ops_per_sec'],
            'vidyut_ops': vidyut_result['ops_per_sec']
        })
    
    # Summary
    print(f"\n🏆 FAIR COMPARISON SUMMARY")
    print("=" * 80)
    print("Only comparing conversions where both produce equivalent output")
    print()
    print(f"{'Test':<25} {'Type':<15} {'Winner':<8} {'Speedup':<8}")
    print("-" * 60)
    
    shlesha_wins = 0
    for result in summary:
        if result['winner'] == 'Shlesha':
            shlesha_wins += 1
        print(f"{result['test']:<25} {result['type']:<15} {result['winner']:<8} {result['speedup']:<6.2f}x")
    
    print(f"\n📊 Fair Results: Shlesha wins {shlesha_wins}/{len(summary)} tests")
    
    # Type analysis
    print(f"\n📈 By Conversion Type:")
    types = {}
    for result in summary:
        conv_type = result['type']
        if conv_type not in types:
            types[conv_type] = {'shlesha': 0, 'vidyut': 0, 'tests': 0}
        types[conv_type]['tests'] += 1
        types[conv_type][result['winner'].lower()] += 1
    
    for conv_type, stats in types.items():
        win_rate = stats['shlesha'] / stats['tests'] * 100
        print(f"  {conv_type:<15}: Shlesha wins {stats['shlesha']}/{stats['tests']} ({win_rate:.0f}%)")
    
    print(f"\n⚠️  Note: Tests where Shlesha produces ISO-15919 instead of the")
    print(f"   requested format (like SLP1) have been excluded as invalid.")

if __name__ == "__main__":
    main()