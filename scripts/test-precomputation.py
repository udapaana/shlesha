#!/usr/bin/env python3
"""
Quick test script to verify pre-computation is working correctly.
This script builds with different feature flags and runs basic tests.
"""

import subprocess
import time
import os
import sys

def run_command(cmd, description):
    """Run a command and return success status"""
    print(f"  {description}...")
    try:
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True, cwd="/Users/skmnktl/Projects/udapaana/shlesha")
        if result.returncode == 0:
            print(f"    ✓ Success")
            return True
        else:
            print(f"    ✗ Failed: {result.stderr.strip()}")
            return False
    except Exception as e:
        print(f"    ✗ Error: {e}")
        return False

def test_build_config(config_name, features, no_default=False):
    """Test building with a specific configuration"""
    print(f"\n{'='*50}")
    print(f"Testing {config_name}")
    print(f"{'='*50}")
    
    # Build command
    cmd = "cargo build --release"
    if no_default:
        cmd += " --no-default-features"
    if features:
        cmd += f" --features {','.join(features)}"
    
    # Test build
    success = run_command(cmd, f"Building with {config_name}")
    if not success:
        return False
    
    # Quick test CLI
    test_cmd = './target/release/shlesha transliterate --from iast --to devanagari "dharma"'
    success = run_command(test_cmd, f"Testing CLI with {config_name}")
    
    return success

def main():
    """Test different pre-computation configurations"""
    print("Shlesha Pre-computation Configuration Test")
    print("==========================================")
    
    configs = [
        ("no-precompute", ["no-precompute"], True),
        ("precompute-common (default)", ["precompute-common"], False),
        ("precompute-roman-indic", ["precompute-roman-indic"], False),
        ("precompute-indic-roman", ["precompute-indic-roman"], False),
    ]
    
    results = {}
    
    for config_name, features, no_default in configs:
        success = test_build_config(config_name, features, no_default)
        results[config_name] = success
    
    # Summary
    print(f"\n{'='*50}")
    print("SUMMARY")
    print(f"{'='*50}")
    
    for config_name, success in results.items():
        status = "✓ PASS" if success else "✗ FAIL"
        print(f"  {config_name:<30} {status}")
    
    # Overall result
    all_passed = all(results.values())
    print(f"\nOverall: {'✓ ALL TESTS PASSED' if all_passed else '✗ SOME TESTS FAILED'}")
    
    if all_passed:
        print("\nPre-computation infrastructure is working correctly!")
        print("You can now run benchmarks to measure performance improvements:")
        print("  cargo bench --bench comprehensive_benchmark")
        print("  python benchmarks/benchmark_comparison.py")
    
    return 0 if all_passed else 1

if __name__ == "__main__":
    sys.exit(main())