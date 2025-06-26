#!/usr/bin/env python3
"""
Analyze Shlesha vs Vidyut performance by conversion pattern types
"""

import csv
import sys
import os

# Add parent directory to path to import shlesha
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'python'))

def load_benchmark_data():
    """Load benchmark results from CSV"""
    results = {}
    
    with open('../target/comparison_benchmark_results.csv', 'r') as f:
        reader = csv.DictReader(f)
        for row in reader:
            if row['library'] in ['shlesha', 'vidyut']:
                key = (row['from_script'], row['to_script'], row['text_size'])
                if key not in results:
                    results[key] = {}
                results[key][row['library']] = {
                    'throughput': float(row['throughput_chars_per_sec']),
                    'latency': float(row['latency_ns'])
                }
    
    return results

def categorize_conversions():
    """Categorize conversion types based on Shlesha's architecture"""
    
    # Script classifications
    hub_scripts = ['devanagari']  # Hub format (can convert directly between hub scripts)
    iso_scripts = ['iso15919']   # ISO format (hub format for Roman)
    roman_scripts = ['iast', 'itrans', 'slp1']  # Roman scripts (go through ISO hub)
    indic_scripts = ['devanagari', 'telugu', 'tamil', 'bengali', 'gujarati', 'kannada', 'malayalam', 'odia']  # Indic scripts (go through Devanagari hub)
    
    categories = {
        'hub_to_roman': [],      # Devanagari → Roman (should benefit from pre-computation)
        'roman_to_hub': [],      # Roman → Devanagari (should benefit from pre-computation)  
        'roman_to_roman': [],    # Roman → Roman (Roman → ISO → Roman, no pre-computation benefit)
        'hub_to_indic': [],      # Devanagari → Indic (direct hub conversion, no pre-computation)
        'indic_to_hub': [],      # Indic → Devanagari (direct hub conversion, no pre-computation)
        'roman_to_indic': [],    # Roman → Indic (Roman → ISO → Devanagari → Indic, potential pre-computation)
        'indic_to_roman': [],    # Indic → Roman (Indic → Devanagari → ISO → Roman, potential pre-computation)
    }
    
    # All conversion pairs from the benchmark
    all_conversions = [
        ('devanagari', 'iast'), ('devanagari', 'itrans'), ('devanagari', 'slp1'),
        ('devanagari', 'telugu'), ('devanagari', 'tamil'),
        ('iast', 'devanagari'), ('iast', 'telugu'), ('iast', 'tamil'),
        ('itrans', 'devanagari'), ('slp1', 'devanagari'),
        ('iast', 'itrans'), ('iast', 'slp1'), ('itrans', 'slp1')
    ]
    
    for from_script, to_script in all_conversions:
        if from_script in hub_scripts and to_script in roman_scripts:
            categories['hub_to_roman'].append((from_script, to_script))
        elif from_script in roman_scripts and to_script in hub_scripts:
            categories['roman_to_hub'].append((from_script, to_script))
        elif from_script in roman_scripts and to_script in roman_scripts:
            categories['roman_to_roman'].append((from_script, to_script))
        elif from_script in hub_scripts and to_script in indic_scripts and to_script != from_script:
            categories['hub_to_indic'].append((from_script, to_script))
        elif from_script in indic_scripts and to_script in hub_scripts and from_script != to_script:
            categories['indic_to_hub'].append((from_script, to_script))
        elif from_script in roman_scripts and to_script in indic_scripts and to_script not in hub_scripts:
            categories['roman_to_indic'].append((from_script, to_script))
        elif from_script in indic_scripts and from_script not in hub_scripts and to_script in roman_scripts:
            categories['indic_to_roman'].append((from_script, to_script))
    
    return categories

def analyze_performance_by_category(results, categories):
    """Analyze performance by conversion category"""
    
    print("# Shlesha vs Vidyut Performance Analysis by Conversion Pattern\n")
    
    for category, conversions in categories.items():
        if not conversions:
            continue
            
        print(f"## {category.replace('_', ' ').title()}")
        print()
        print("| Conversion | Text Size | Shlesha (chars/sec) | Vidyut (chars/sec) | Vidyut Advantage |")
        print("|------------|-----------|---------------------|-------------------|------------------|")
        
        category_ratios = []
        
        for from_script, to_script in conversions:
            for size in ['small', 'medium', 'large']:
                key = (from_script, to_script, size)
                if key in results and 'shlesha' in results[key] and 'vidyut' in results[key]:
                    shlesha_perf = results[key]['shlesha']['throughput']
                    vidyut_perf = results[key]['vidyut']['throughput']
                    ratio = vidyut_perf / shlesha_perf
                    category_ratios.append(ratio)
                    
                    print(f"| {from_script} → {to_script} | {size} | {shlesha_perf:,.0f} | {vidyut_perf:,.0f} | {ratio:.1f}x |")
        
        if category_ratios:
            avg_ratio = sum(category_ratios) / len(category_ratios)
            print(f"\n**Average Vidyut advantage: {avg_ratio:.1f}x**\n")
        
        print()

def analyze_precomputation_potential(categories):
    """Analyze which conversions could benefit from pre-computation"""
    
    print("## Pre-computation Impact Analysis\n")
    
    high_benefit = ['hub_to_roman', 'roman_to_hub']
    medium_benefit = ['roman_to_indic', 'indic_to_roman'] 
    no_benefit = ['roman_to_roman', 'hub_to_indic', 'indic_to_hub']
    
    print("### Expected Pre-computation Benefits\n")
    
    print("**High Benefit (Direct mappings possible)**:")
    for category in high_benefit:
        if category in categories and categories[category]:
            print(f"- **{category.replace('_', ' ').title()}**: {len(categories[category])} conversions")
            for from_script, to_script in categories[category]:
                print(f"  - {from_script} → {to_script}")
    print()
    
    print("**Medium Benefit (Partial optimization possible)**:")
    for category in medium_benefit:
        if category in categories and categories[category]:
            print(f"- **{category.replace('_', ' ').title()}**: {len(categories[category])} conversions")
            for from_script, to_script in categories[category]:
                print(f"  - {from_script} → {to_script}")
    print()
    
    print("**No Benefit (Hub-based conversions remain optimal)**:")
    for category in no_benefit:
        if category in categories and categories[category]:
            print(f"- **{category.replace('_', ' ').title()}**: {len(categories[category])} conversions")
            for from_script, to_script in categories[category]:
                print(f"  - {from_script} → {to_script}")
    print()

def main():
    print("Loading benchmark data...")
    results = load_benchmark_data()
    
    print("Categorizing conversions...")
    categories = categorize_conversions()
    
    print("Analyzing performance patterns...\n")
    
    analyze_performance_by_category(results, categories)
    analyze_precomputation_potential(categories)
    
    print("## Current Pre-computation Status\n")
    print("**Currently Implemented**:")
    print("- Direct mappings: `iso15919 ↔ devanagari` (character-level)")
    print("- Feature flags: `precompute-common` vs `no-precompute`")
    print("- Zero overhead: Falls back to hub system when no direct mapping")
    print()
    
    print("**Next Steps for Optimization**:")
    print("- Add direct mappings for `iast ↔ devanagari`")
    print("- Add direct mappings for `itrans ↔ devanagari`") 
    print("- Implement `precompute-all` feature")
    print("- Add multi-character pattern optimization (handle conjuncts)")

if __name__ == "__main__":
    main()