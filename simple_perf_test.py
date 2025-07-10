#!/usr/bin/env python3
import time
import shlesha

def benchmark_conversion(text, from_script, to_script, iterations=1000):
    """Benchmark a specific conversion"""
    
    # Warm up
    for _ in range(10):
        result = shlesha.transliterate(text, from_script, to_script)
    
    # Actual benchmark
    start_time = time.time()
    for _ in range(iterations):
        result = shlesha.transliterate(text, from_script, to_script)
    end_time = time.time()
    
    total_time = end_time - start_time
    ops_per_sec = iterations / total_time
    chars_per_sec = (len(text) * iterations) / total_time
    
    return {
        'result': result,
        'time': total_time,
        'ops_per_sec': ops_per_sec,
        'chars_per_sec': chars_per_sec
    }

def main():
    print("🚀 PERFORMANCE TEST - Token-based Architecture")
    print("=" * 60)
    
    test_cases = [
        ("संस्कृतम्", "devanagari", "slp1", "Indic → Roman"),
        ("saṃskṛtam", "iast", "slp1", "Roman → Roman"),  
        ("saṃskṛtam", "iast", "devanagari", "Roman → Indic"),
        ("సంస్కృతం", "telugu", "slp1", "Telugu → Roman"),
        ("సంస్కృతం", "telugu", "devanagari", "Telugu → Devanagari"),
        ("saṃskṛtam", "iast", "itrans", "IAST → ITRANS"),
        ("রমনিয়া", "bengali", "iast", "Bengali → IAST"),
    ]
    
    for text, from_script, to_script, category in test_cases:
        try:
            result = benchmark_conversion(text, from_script, to_script, 1000)
            print(f"{category:20} | {from_script:15} → {to_script:15} | {result['ops_per_sec']:8.0f} ops/s | {result['chars_per_sec']:8.0f} chars/s")
            print(f"                     | Result: {result['result']}")
            print("-" * 100)
        except Exception as e:
            print(f"{category:20} | {from_script:15} → {to_script:15} | ERROR: {e}")
            print("-" * 100)

if __name__ == "__main__":
    main()