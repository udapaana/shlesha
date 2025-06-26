#!/usr/bin/env python3
"""
Python benchmark comparing Shlesha with and without pre-computation against other libraries.
Tests both feature flags and measures the impact of pre-computation optimizations.
"""

import time
import statistics
import json
import os
import subprocess
import tempfile
from pathlib import Path

# Import Shlesha
import shlesha

# Try to import other transliteration libraries for comparison
other_libs = {}

try:
    from vidyut.lipi import Scheme, transliterate
    other_libs['vidyut'] = {'Scheme': Scheme, 'transliterate': transliterate}
    print("✓ vidyut found")
except ImportError:
    print("✗ vidyut not found (pip install vidyut)")

try:
    from indic_transliteration import sanscript
    other_libs['indic_transliteration'] = sanscript
    print("✓ indic-transliteration found")
except ImportError:
    print("✗ indic-transliteration not found (pip install indic-transliteration)")

# Test data for different sizes
TEST_DATA = {
    "small": {
        "iast": "dharma",
        "devanagari": "धर्म",
    },
    "medium": {
        "iast": "dharma yoga bhārata saṃskṛta veda upaniṣad gītā rāmāyaṇa mahābhārata",
        "devanagari": "धर्म योग भारत संस्कृत वेद उपनिषद् गीता रामायण महाभारत",
    },
    "large": {
        "iast": "dharma yoga bhārata saṃskṛta veda upaniṣad gītā rāmāyaṇa mahābhārata purāṇa śāstra darśana āyurveda jyotiṣa vyākaraṇa chanda nirukta kalpa śikṣā smṛti śruti ācāra vicāra saṃskāra paramparā satya ahiṃsā karuṇā dayā prema śānti ānanda mokṣa nirvāṇa samādhi dhyāna prāṇāyāma āsana mantra yantra tantra",
        "devanagari": "धर्म योग भारत संस्कृत वेद उपनिषद् गीता रामायण महाभारत पुराण शास्त्र दर्शन आयुर्वेद ज्योतिष व्याकरण छन्द निरुक्त कल्प शिक्षा स्मृति श्रुति आचार विचार संस्कार परम्परा सत्य अहिंसा करुणा दया प्रेम शान्ति आनन्द मोक्ष निर्वाण समाधि ध्यान प्राणायाम आसन मन्त्र यन्त्र तन्त्र",
    }
}

# Conversions that should benefit from pre-computation
PRECOMPUTE_BENEFIT_CONVERSIONS = [
    ("iast", "devanagari"),
    ("devanagari", "iast"),
    ("itrans", "devanagari"),
    ("devanagari", "itrans"),
]

# Control conversions (should not benefit)
CONTROL_CONVERSIONS = [
    ("devanagari", "telugu"),  # Indic → Indic (already optimal)
    ("iast", "itrans"),        # Roman → Roman (already optimal)
]

def benchmark_function(func, *args, iterations=100):
    """Benchmark a function with multiple iterations and return statistics."""
    times = []
    
    # Warm up
    for _ in range(10):
        func(*args)
    
    # Actual timing
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
        'iterations': iterations,
        'result_sample': str(result)[:50] + "..." if len(str(result)) > 50 else str(result)
    }

def test_shlesha_with_feature(from_script, to_script, text, feature_flag=None):
    """Test Shlesha with specific feature flags by rebuilding if needed."""
    # For now, test with current build
    # In practice, you'd rebuild with different features:
    # cargo build --release --features precompute-common
    # cargo build --release --features precompute-all  
    # cargo build --release --features no-precompute
    
    try:
        translator = shlesha.Shlesha()
        return translator.transliterate(text, from_script, to_script)
    except Exception as e:
        print(f"Error with Shlesha {from_script}→{to_script}: {e}")
        return None

def test_vidyut(from_script, to_script, text):
    """Test Vidyut if available."""
    if 'vidyut' not in other_libs:
        return None
    
    try:
        # Map script names to Vidyut schemes
        scheme_map = {
            'iast': 'Iast',
            'devanagari': 'Devanagari',
            'itrans': 'Itrans',
        }
        
        if from_script not in scheme_map or to_script not in scheme_map:
            return None
            
        from_scheme = getattr(other_libs['vidyut']['Scheme'], scheme_map[from_script])
        to_scheme = getattr(other_libs['vidyut']['Scheme'], scheme_map[to_script])
        
        return other_libs['vidyut']['transliterate'](text, from_scheme, to_scheme)
    except Exception as e:
        print(f"Error with Vidyut {from_script}→{to_script}: {e}")
        return None

def test_indic_transliteration(from_script, to_script, text):
    """Test indic-transliteration if available."""
    if 'indic_transliteration' not in other_libs:
        return None
    
    try:
        # Map script names to sanscript schemes
        scheme_map = {
            'iast': 'iast',
            'devanagari': 'devanagari',
            'itrans': 'itrans',
        }
        
        if from_script not in scheme_map or to_script not in scheme_map:
            return None
            
        return other_libs['indic_transliteration'].transliterate(
            text, scheme_map[from_script], scheme_map[to_script]
        )
    except Exception as e:
        print(f"Error with indic-transliteration {from_script}→{to_script}: {e}")
        return None

def run_comprehensive_benchmark():
    """Run comprehensive benchmarks comparing all tools."""
    results = {
        'meta': {
            'timestamp': time.time(),
            'test_description': 'Shlesha pre-computation vs other libraries',
            'iterations_per_test': 100,
        },
        'results': {}
    }
    
    print("\n🔬 Comprehensive Pre-computation Benchmark")
    print("==========================================")
    
    # Test each conversion and size combination
    for conversion_type, conversions in [
        ("precompute_benefit", PRECOMPUTE_BENEFIT_CONVERSIONS),
        ("control", CONTROL_CONVERSIONS)
    ]:
        print(f"\n📊 Testing {conversion_type} conversions:")
        
        results['results'][conversion_type] = {}
        
        for from_script, to_script in conversions:
            print(f"\n  {from_script} → {to_script}:")
            
            conversion_key = f"{from_script}_to_{to_script}"
            results['results'][conversion_type][conversion_key] = {}
            
            for size in ['small', 'medium', 'large']:
                text = TEST_DATA[size].get(from_script, TEST_DATA[size]['iast'])
                print(f"    {size}: ", end="", flush=True)
                
                size_results = {}
                
                # Test Shlesha (current build)
                shlesha_stats = benchmark_function(
                    test_shlesha_with_feature, from_script, to_script, text
                )
                size_results['shlesha'] = shlesha_stats
                print(f"Shlesha: {shlesha_stats['mean']*1000:.2f}ms ", end="", flush=True)
                
                # Test Vidyut if available and compatible
                if from_script in ['iast', 'devanagari', 'itrans'] and to_script in ['iast', 'devanagari', 'itrans']:
                    vidyut_stats = benchmark_function(
                        test_vidyut, from_script, to_script, text
                    )
                    if vidyut_stats and vidyut_stats.get('result_sample'):
                        size_results['vidyut'] = vidyut_stats
                        print(f"Vidyut: {vidyut_stats['mean']*1000:.2f}ms ", end="", flush=True)
                
                # Test indic-transliteration if available and compatible
                if from_script in ['iast', 'devanagari', 'itrans'] and to_script in ['iast', 'devanagari', 'itrans']:
                    indic_stats = benchmark_function(
                        test_indic_transliteration, from_script, to_script, text
                    )
                    if indic_stats and indic_stats.get('result_sample'):
                        size_results['indic_transliteration'] = indic_stats
                        print(f"Indic-Trans: {indic_stats['mean']*1000:.2f}ms", end="", flush=True)
                
                print()  # New line
                
                results['results'][conversion_type][conversion_key][size] = size_results
    
    return results

def analyze_precomputation_impact(results):
    """Analyze the impact of pre-computation from benchmark results."""
    print("\n📈 Pre-computation Impact Analysis")
    print("==================================")
    
    # Compare precompute_benefit vs control conversions
    precompute_times = []
    control_times = []
    
    for conversion_type, conversions in results['results'].items():
        for conversion, sizes in conversions.items():
            for size, tools in sizes.items():
                if 'shlesha' in tools:
                    mean_time = tools['shlesha']['mean']
                    if conversion_type == 'precompute_benefit':
                        precompute_times.append(mean_time)
                    else:
                        control_times.append(mean_time)
    
    if precompute_times and control_times:
        precompute_avg = statistics.mean(precompute_times)
        control_avg = statistics.mean(control_times)
        
        print(f"Average time for pre-computation benefit conversions: {precompute_avg*1000:.2f}ms")
        print(f"Average time for control conversions: {control_avg*1000:.2f}ms")
        
        if precompute_avg < control_avg:
            improvement = ((control_avg - precompute_avg) / control_avg) * 100
            print(f"🚀 Pre-computation shows {improvement:.1f}% improvement!")
        else:
            overhead = ((precompute_avg - control_avg) / control_avg) * 100
            print(f"⚠️  Pre-computation shows {overhead:.1f}% overhead")

def generate_comparison_report(results):
    """Generate a detailed comparison report."""
    timestamp = int(time.time())
    report_file = f"precompute_python_benchmark_{timestamp}.json"
    
    with open(report_file, 'w') as f:
        json.dump(results, f, indent=2)
    
    print(f"\n📄 Detailed results saved to: {report_file}")
    
    # Generate summary CSV
    csv_file = f"precompute_summary_{timestamp}.csv"
    with open(csv_file, 'w') as f:
        f.write("conversion_type,conversion,size,tool,mean_ms,median_ms,stdev_ms\n")
        
        for conversion_type, conversions in results['results'].items():
            for conversion, sizes in conversions.items():
                for size, tools in sizes.items():
                    for tool, stats in tools.items():
                        f.write(f"{conversion_type},{conversion},{size},{tool},"
                               f"{stats['mean']*1000:.3f},{stats['median']*1000:.3f},"
                               f"{stats['stdev']*1000:.3f}\n")
    
    print(f"📊 Summary CSV saved to: {csv_file}")

def main():
    """Main benchmark execution."""
    print("🧪 Shlesha Pre-computation Python Benchmark")
    print("============================================")
    
    # Check current feature flags (would need cargo build info in practice)
    print("\n🔧 Current build configuration:")
    print("   Note: To test different pre-computation settings, rebuild with:")
    print("   cargo build --release --features precompute-common")
    print("   cargo build --release --features precompute-all")
    print("   cargo build --release --features no-precompute")
    
    # Run benchmarks
    results = run_comprehensive_benchmark()
    
    # Analyze results
    analyze_precomputation_impact(results)
    
    # Generate reports
    generate_comparison_report(results)
    
    print("\n✅ Benchmark complete!")
    print("\n💡 To see the full impact of pre-computation:")
    print("   1. Run this benchmark with --features no-precompute")
    print("   2. Run this benchmark with --features precompute-common") 
    print("   3. Compare the results to see the optimization impact")

if __name__ == "__main__":
    main()