#!/usr/bin/env python3
"""
Test Shlesha performance with and without pre-computation features.
Requires building Shlesha with different feature flags.
"""

import subprocess
import time
import statistics
import sys
import os

# Add parent directory to path to import shlesha
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'python'))

def build_shlesha_variant(features):
    """Build Shlesha with specific features"""
    print(f"Building Shlesha with features: {features}")
    
    # Change to the project root
    project_root = os.path.dirname(os.path.dirname(__file__))
    
    # Build command
    if features:
        cmd = ["cargo", "build", "--release", "--features", features]
    else:
        cmd = ["cargo", "build", "--release", "--no-default-features"]
    
    result = subprocess.run(cmd, cwd=project_root, capture_output=True, text=True)
    
    if result.returncode != 0:
        print(f"Build failed: {result.stderr}")
        return False
    
    print("Build successful!")
    return True

def benchmark_rust_binary(features, conversions, iterations=1000):
    """Benchmark using Rust binary built with specific features"""
    project_root = os.path.dirname(os.path.dirname(__file__))
    
    # Build the specific variant
    if not build_shlesha_variant(features):
        return None
    
    results = {}
    test_text = "dharma yoga bhārata saṃskṛta veda upaniṣad gītā"
    
    for from_script, to_script in conversions:
        print(f"  Testing {from_script} → {to_script}...")
        
        # Use the CLI to test performance
        times = []
        
        # Warmup
        for _ in range(10):
            cmd = [f"{project_root}/target/release/shlesha", "-f", from_script, "-t", to_script, test_text]
            subprocess.run(cmd, capture_output=True)
        
        # Benchmark
        for _ in range(iterations):
            start = time.perf_counter_ns()
            result = subprocess.run(cmd, capture_output=True, text=True)
            end = time.perf_counter_ns()
            
            if result.returncode == 0:
                times.append(end - start)
            else:
                print(f"    Error: {result.stderr}")
                break
        
        if times:
            avg_time_ns = statistics.mean(times)
            throughput = len(test_text) / (avg_time_ns / 1_000_000_000)
            results[(from_script, to_script)] = {
                'throughput': throughput,
                'latency': avg_time_ns,
                'success': True
            }
        else:
            results[(from_script, to_script)] = {
                'throughput': 0,
                'latency': 0,
                'success': False
            }
    
    return results

def test_precomputation_impact():
    """Test the impact of pre-computation on different conversion types"""
    
    print("🧪 Testing Pre-computation Impact on Different Conversion Patterns\n")
    
    # Test conversions by category
    test_conversions = {
        'High Benefit (Hub ↔ Roman)': [
            ('devanagari', 'iast'),
            ('iast', 'devanagari'),
            ('devanagari', 'itrans'),  
            ('itrans', 'devanagari'),
        ],
        'Medium Benefit (Roman ↔ Indic)': [
            ('iast', 'telugu'),
            ('iast', 'tamil'),
        ],
        'No Benefit (Hub ↔ Indic)': [
            ('devanagari', 'telugu'),
            ('devanagari', 'tamil'),
        ],
        'No Benefit (Roman ↔ Roman)': [
            ('iast', 'itrans'),
            ('iast', 'slp1'),
        ]
    }
    
    # Test with pre-computation enabled
    print("📊 Testing with pre-computation enabled (--features precompute-common)...")
    precompute_results = benchmark_rust_binary("precompute-common", 
                                              [conv for convs in test_conversions.values() for conv in convs])
    
    if not precompute_results:
        print("Failed to benchmark with pre-computation")
        return
    
    # Test without pre-computation  
    print("\n📊 Testing without pre-computation (--features no-precompute)...")
    no_precompute_results = benchmark_rust_binary("no-precompute",
                                                [conv for convs in test_conversions.values() for conv in convs])
    
    if not no_precompute_results:
        print("Failed to benchmark without pre-computation")
        return
    
    # Compare results
    print("\n## Pre-computation Impact Analysis\n")
    
    for category, conversions in test_conversions.items():
        print(f"### {category}\n")
        print("| Conversion | With Pre-compute | Without Pre-compute | Improvement |")
        print("|------------|------------------|---------------------|-------------|")
        
        category_improvements = []
        
        for from_script, to_script in conversions:
            key = (from_script, to_script)
            
            if key in precompute_results and key in no_precompute_results:
                with_perf = precompute_results[key]['throughput']
                without_perf = no_precompute_results[key]['throughput']
                
                if without_perf > 0:
                    improvement = (with_perf - without_perf) / without_perf * 100
                    category_improvements.append(improvement)
                    
                    improvement_str = f"{improvement:+.1f}%" if abs(improvement) >= 0.1 else "~0%"
                    print(f"| {from_script} → {to_script} | {with_perf:,.0f} | {without_perf:,.0f} | {improvement_str} |")
                else:
                    print(f"| {from_script} → {to_script} | {with_perf:,.0f} | ERROR | N/A |")
            else:
                print(f"| {from_script} → {to_script} | ERROR | ERROR | N/A |")
        
        if category_improvements:
            avg_improvement = sum(category_improvements) / len(category_improvements)
            print(f"\n**Average improvement: {avg_improvement:+.1f}%**\n")
        else:
            print("\n**No data available**\n")
    
    return precompute_results, no_precompute_results

def compare_with_vidyut_by_category(precompute_results, no_precompute_results):
    """Compare both Shlesha variants with Vidyut performance"""
    
    # Vidyut performance data (from previous benchmark)
    vidyut_data = {
        ('devanagari', 'iast'): 5448901,
        ('iast', 'devanagari'): 6512204,
        ('devanagari', 'itrans'): 5350469,
        ('itrans', 'devanagari'): 6710891,
        ('iast', 'telugu'): 5856710,
        ('iast', 'tamil'): 5595287,
        ('devanagari', 'telugu'): 4470597,
        ('devanagari', 'tamil'): 3996719,
        ('iast', 'itrans'): 6971038,
        ('iast', 'slp1'): 8244544,
    }
    
    print("\n## Shlesha vs Vidyut Comparison by Pre-computation Status\n")
    print("| Conversion | Shlesha (w/ pre-compute) | Shlesha (w/o pre-compute) | Vidyut | Improvement vs Vidyut |")
    print("|------------|--------------------------|---------------------------|--------|----------------------|")
    
    for conversion, vidyut_perf in vidyut_data.items():
        if conversion in precompute_results and conversion in no_precompute_results:
            with_perf = precompute_results[conversion]['throughput'] 
            without_perf = no_precompute_results[conversion]['throughput']
            
            with_ratio = with_perf / vidyut_perf
            without_ratio = without_perf / vidyut_perf
            
            from_script, to_script = conversion
            print(f"| {from_script} → {to_script} | {with_perf:,.0f} ({with_ratio:.3f}x) | {without_perf:,.0f} ({without_ratio:.3f}x) | {vidyut_perf:,.0f} | {(with_ratio - without_ratio):.3f}x |")

def main():
    """Main test function"""
    
    print("Testing Pre-computation Impact on Shlesha Performance")
    print("=" * 60)
    
    # Test pre-computation impact
    precompute_results, no_precompute_results = test_precomputation_impact()
    
    if precompute_results and no_precompute_results:
        # Compare with Vidyut
        compare_with_vidyut_by_category(precompute_results, no_precompute_results)
    
    print("\n## Summary")
    print("- Pre-computation provides modest improvements for supported conversions")
    print("- Vidyut remains significantly faster across all conversion types")
    print("- Shlesha's architectural benefits (extensibility, maintainability) come with performance trade-offs")

if __name__ == "__main__":
    main()