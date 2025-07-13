#!/usr/bin/env python3
"""
Generate a clean, consolidated benchmark report from all benchmark results.
Produces a single markdown file with just data - no commentary.
"""

import os
import csv
import json
from datetime import datetime

def read_csv_results(filename):
    """Read CSV benchmark results"""
    results = []
    if os.path.exists(filename):
        with open(filename, 'r') as f:
            reader = csv.DictReader(f)
            results = list(reader)
    return results

def format_number(num_str):
    """Format number string for display"""
    try:
        return f"{float(num_str):,.0f}"
    except:
        return num_str

def generate_clean_report():
    """Generate consolidated benchmark report"""
    report = []
    report.append(f"# Shlesha Benchmark Results")
    report.append(f"*Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}*")
    report.append("")
    
    # Rust/Native benchmarks
    rust_results = []
    for category in ["hub", "standard", "extension", "cross_category"]:
        rust_results.extend(read_csv_results(f"target/benchmark_results_{category}.csv"))
    
    if rust_results:
        report.append("## Rust Native Performance")
        report.append("")
        report.append("### Hub Scripts (Devanagari ↔ ISO-15919)")
        report.append("| From | To | Text Size | Throughput (chars/sec) | Latency (ns) |")
        report.append("|------|----|-----------|-----------------------|-------------|")
        
        for r in rust_results:
            if r.get('category') == 'hub':
                report.append(f"| {r['script_from']} | {r['script_to']} | {r['text_size']} | {format_number(r['throughput_chars_per_sec'])} | {format_number(r['latency_ns'])} |")
        
        report.append("")
        report.append("### Standard Indic Scripts")
        report.append("| From | To | Text Size | Throughput (chars/sec) | Latency (ns) |")
        report.append("|------|----|-----------|-----------------------|-------------|")
        
        for r in rust_results:
            if r.get('category') == 'standard':
                report.append(f"| {r['script_from']} | {r['script_to']} | {r['text_size']} | {format_number(r['throughput_chars_per_sec'])} | {format_number(r['latency_ns'])} |")
        
        report.append("")
        report.append("### Extension Scripts (Roman/ASCII)")
        report.append("| From | To | Text Size | Throughput (chars/sec) | Latency (ns) |")
        report.append("|------|----|-----------|-----------------------|-------------|")
        
        for r in rust_results:
            if r.get('category') == 'extension':
                report.append(f"| {r['script_from']} | {r['script_to']} | {r['text_size']} | {format_number(r['throughput_chars_per_sec'])} | {format_number(r['latency_ns'])} |")
    
    # Python benchmarks
    python_results = read_csv_results("target/python_benchmark_results.csv")
    
    if python_results:
        report.append("")
        report.append("## Python API Performance")
        report.append("")
        
        # Group by API type
        api_types = set(r['api_type'] for r in python_results)
        
        for api_type in sorted(api_types):
            report.append(f"### {api_type.replace('_', ' ').title()}")
            report.append("| From | To | Category | Text Size | Throughput (chars/sec) | Latency (ns) |")
            report.append("|------|-----|----------|-----------|------------------------|-------------|")
            
            for r in python_results:
                if r['api_type'] == api_type:
                    report.append(f"| {r['script_from']} | {r['script_to']} | {r['category']} | {r['text_size']} | {format_number(r['throughput_chars_per_sec'])} | {format_number(r['latency_ns'])} |")
            report.append("")
    
    # Comparison benchmarks
    comparison_results = read_csv_results("target/comparison_benchmark_results.csv")
    
    if comparison_results:
        report.append("")
        report.append("## Library Comparison")
        report.append("")
        
        # Group by conversion
        conversions = {}
        for r in comparison_results:
            if r.get('success') == 'True':
                key = f"{r['from_script']}_{r['to_script']}"
                if key not in conversions:
                    conversions[key] = []
                conversions[key].append(r)
        
        for conversion, results in sorted(conversions.items()):
            from_script, to_script = conversion.split('_')
            report.append(f"### {from_script} → {to_script}")
            report.append("| Library | Text Size | Throughput (chars/sec) | Latency (ns) |")
            report.append("|---------|-----------|------------------------|-------------|")
            
            for r in sorted(results, key=lambda x: (x['text_size'], x['library'])):
                report.append(f"| {r['library']} | {r['text_size']} | {format_number(r['throughput_chars_per_sec'])} | {format_number(r['latency_ns'])} |")
            report.append("")
    
    # Performance Summary Tables
    report.append("")
    report.append("## Performance Summary")
    report.append("")
    
    # Calculate averages
    if rust_results:
        categories = {}
        for r in rust_results:
            cat = r['category']
            if cat not in categories:
                categories[cat] = []
            try:
                categories[cat].append(float(r['throughput_chars_per_sec']))
            except:
                pass
        
        report.append("### Average Throughput by Category (Rust)")
        report.append("| Category | Avg Throughput (chars/sec) |")
        report.append("|----------|----------------------------|")
        
        for cat, throughputs in sorted(categories.items()):
            if throughputs:
                avg = sum(throughputs) / len(throughputs)
                report.append(f"| {cat} | {avg:,.0f} |")
    
    if python_results:
        report.append("")
        report.append("### Average Throughput by API Type (Python)")
        report.append("| API Type | Avg Throughput (chars/sec) |")
        report.append("|----------|----------------------------|")
        
        api_throughputs = {}
        for r in python_results:
            api = r['api_type']
            if api not in api_throughputs:
                api_throughputs[api] = []
            try:
                api_throughputs[api].append(float(r['throughput_chars_per_sec']))
            except:
                pass
        
        for api, throughputs in sorted(api_throughputs.items()):
            if throughputs:
                avg = sum(throughputs) / len(throughputs)
                report.append(f"| {api} | {avg:,.0f} |")
    
    if comparison_results:
        report.append("")
        report.append("### Library Performance Ranking")
        report.append("| Library | Avg Throughput (chars/sec) | Success Rate |")
        report.append("|---------|----------------------------|--------------|")
        
        lib_stats = {}
        for r in comparison_results:
            lib = r['library']
            if lib not in lib_stats:
                lib_stats[lib] = {'throughputs': [], 'total': 0, 'success': 0}
            lib_stats[lib]['total'] += 1
            if r.get('success') == 'True':
                lib_stats[lib]['success'] += 1
                try:
                    lib_stats[lib]['throughputs'].append(float(r['throughput_chars_per_sec']))
                except:
                    pass
        
        # Sort by average throughput
        lib_ranking = []
        for lib, stats in lib_stats.items():
            if stats['throughputs']:
                avg = sum(stats['throughputs']) / len(stats['throughputs'])
                success_rate = (stats['success'] / stats['total']) * 100
                lib_ranking.append((lib, avg, success_rate))
        
        lib_ranking.sort(key=lambda x: x[1], reverse=True)
        
        for lib, avg, success_rate in lib_ranking:
            report.append(f"| {lib} | {avg:,.0f} | {success_rate:.0f}% |")
    
    return '\n'.join(report)

def main():
    """Generate consolidated benchmark report"""
    os.makedirs("target", exist_ok=True)
    
    report = generate_clean_report()
    
    with open("target/BENCHMARK_DATA.md", "w") as f:
        f.write(report)
    
    print("Generated consolidated benchmark report: target/BENCHMARK_DATA.md")
    
    # Also create a minimal summary
    summary = []
    summary.append("# Shlesha Performance Summary")
    summary.append("")
    summary.append("## Quick Reference")
    summary.append("")
    summary.append("| Metric | Value |")
    summary.append("|--------|-------|")
    
    # Add key metrics
    rust_results = []
    for category in ["hub", "standard", "extension"]:
        rust_results.extend(read_csv_results(f"target/benchmark_results_{category}.csv"))
    
    if rust_results:
        throughputs = []
        for r in rust_results:
            try:
                throughputs.append(float(r['throughput_chars_per_sec']))
            except:
                pass
        if throughputs:
            avg_throughput = sum(throughputs) / len(throughputs)
            summary.append(f"| Rust Avg Throughput | {avg_throughput:,.0f} chars/sec |")
            summary.append(f"| Rust Max Throughput | {max(throughputs):,.0f} chars/sec |")
            summary.append(f"| Rust Min Throughput | {min(throughputs):,.0f} chars/sec |")
    
    python_results = read_csv_results("target/python_benchmark_results.csv")
    if python_results:
        throughputs = []
        for r in python_results:
            try:
                throughputs.append(float(r['throughput_chars_per_sec']))
            except:
                pass
        if throughputs:
            avg_throughput = sum(throughputs) / len(throughputs)
            summary.append(f"| Python Avg Throughput | {avg_throughput:,.0f} chars/sec |")
    
    summary.append("")
    summary.append("*See target/BENCHMARK_DATA.md for complete results*")
    
    with open("target/BENCHMARK_SUMMARY.md", "w") as f:
        f.write('\n'.join(summary))
    
    print("Generated benchmark summary: target/BENCHMARK_SUMMARY.md")

if __name__ == "__main__":
    main()