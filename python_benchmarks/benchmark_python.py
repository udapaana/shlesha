#!/usr/bin/env python3
"""
Comprehensive Python API benchmarks for Shlesha.
Generates clean markdown output without commentary.
"""

import time
import statistics
import csv
import os
import shlesha

# Test data sets
SMALL_TEXT = "धर्म"
MEDIUM_TEXT = "धर्म योग भारत संस्कृत वेद उपनिषद् गीता रामायण महाभारत"
LARGE_TEXT = "धर्म योग भारत संस्कृत वेद उपनिषद् गीता रामायण महाभारत पुराण शास्त्र दर्शन आयुर्वेद ज्योतिष व्याकरण छन्द निरुक्त कल्प शिक्षा स्मृति श्रुति आचार विचार संस्कार परम्परा सत्य अहिंसा करुणा दया प्रेम शान्ति आनन्द मोक्ष निर्वाण समाधि ध्यान प्राणायाम आसन मन्त्र यन्त्र तन्त्र"

# Script categories
HUB_SCRIPTS = ["devanagari", "iso15919"]
STANDARD_SCRIPTS = ["bengali", "tamil", "telugu", "gujarati", "kannada", "malayalam", "odia"]
EXTENSION_SCRIPTS = ["iast", "itrans", "slp1", "harvard_kyoto", "velthuis", "wx"]

class BenchmarkResult:
    def __init__(self, script_from, script_to, category, text_size, throughput_chars_per_sec, latency_ns, api_type):
        self.script_from = script_from
        self.script_to = script_to
        self.category = category
        self.text_size = text_size
        self.throughput_chars_per_sec = throughput_chars_per_sec
        self.latency_ns = latency_ns
        self.api_type = api_type

def benchmark_api_method(method, text, from_script, to_script, iterations=100):
    """Benchmark a specific API method"""
    times = []
    
    # Warmup
    for _ in range(5):
        method(text, from_script, to_script)
    
    # Actual benchmark
    for _ in range(iterations):
        start = time.perf_counter_ns()
        method(text, from_script, to_script)
        end = time.perf_counter_ns()
        times.append(end - start)
    
    avg_time_ns = statistics.mean(times)
    chars_count = len(text)
    throughput = chars_count / (avg_time_ns / 1_000_000_000)
    
    return throughput, avg_time_ns

def benchmark_category(category_name, scripts, transliterator, results):
    """Benchmark a category of scripts"""
    
    for from_script in scripts:
        for to_script in scripts:
            if from_script == to_script:
                continue
                
            for size_name, text in [("small", SMALL_TEXT), ("medium", MEDIUM_TEXT), ("large", LARGE_TEXT)]:
                # Instance method benchmark
                throughput, latency = benchmark_api_method(
                    transliterator.transliterate, text, from_script, to_script
                )
                results.append(BenchmarkResult(
                    from_script, to_script, category_name, size_name,
                    throughput, latency, "instance_method"
                ))
                
                # Convenience function benchmark
                throughput, latency = benchmark_api_method(
                    shlesha.transliterate, text, from_script, to_script
                )
                results.append(BenchmarkResult(
                    from_script, to_script, category_name, size_name,
                    throughput, latency, "convenience_function"
                ))
                
                # With metadata benchmark
                def transliterate_with_metadata(text, from_script, to_script):
                    return transliterator.transliterate_with_metadata(text, from_script, to_script)
                
                throughput, latency = benchmark_api_method(
                    transliterate_with_metadata, text, from_script, to_script
                )
                results.append(BenchmarkResult(
                    from_script, to_script, category_name, size_name,
                    throughput, latency, "with_metadata"
                ))

def benchmark_cross_category(transliterator, results):
    """Benchmark cross-category conversions"""
    
    # Hub to Standard
    for hub_script in HUB_SCRIPTS:
        for standard_script in STANDARD_SCRIPTS:
            for size_name, text in [("small", SMALL_TEXT), ("medium", MEDIUM_TEXT), ("large", LARGE_TEXT)]:
                throughput, latency = benchmark_api_method(
                    transliterator.transliterate, text, hub_script, standard_script
                )
                results.append(BenchmarkResult(
                    hub_script, standard_script, "cross_hub_to_standard", size_name,
                    throughput, latency, "instance_method"
                ))
    
    # Hub to Extension
    for hub_script in HUB_SCRIPTS:
        for ext_script in EXTENSION_SCRIPTS:
            for size_name, text in [("small", SMALL_TEXT), ("medium", MEDIUM_TEXT), ("large", LARGE_TEXT)]:
                throughput, latency = benchmark_api_method(
                    transliterator.transliterate, text, hub_script, ext_script
                )
                results.append(BenchmarkResult(
                    hub_script, ext_script, "cross_hub_to_extension", size_name,
                    throughput, latency, "instance_method"
                ))

def write_csv_results(results, filename):
    """Write results to CSV file"""
    with open(filename, 'w', newline='') as csvfile:
        writer = csv.writer(csvfile)
        writer.writerow(['script_from', 'script_to', 'category', 'text_size', 'throughput_chars_per_sec', 'latency_ns', 'api_type'])
        
        for result in results:
            writer.writerow([
                result.script_from, result.script_to, result.category, result.text_size,
                f"{result.throughput_chars_per_sec:.0f}", f"{result.latency_ns:.0f}", result.api_type
            ])

def generate_markdown_report(results):
    """Generate markdown report from results"""
    
    md_content = "# Shlesha Python API Performance Benchmark Results\n\n"
    
    # Hub Scripts Performance
    md_content += "## Hub Scripts (Devanagari ↔ ISO-15919)\n\n"
    md_content += "| From | To | Text Size | API Type | Throughput (chars/sec) | Latency (ns) |\n"
    md_content += "|------|----|-----------|---------|-----------------------|-------------|\n"
    
    for result in results:
        if result.category == "hub":
            md_content += f"| {result.script_from} | {result.script_to} | {result.text_size} | {result.api_type} | {result.throughput_chars_per_sec:.0f} | {result.latency_ns:.0f} |\n"
    
    # Standard Scripts Performance
    md_content += "\n## Standard Indic Scripts\n\n"
    md_content += "| From | To | Text Size | API Type | Throughput (chars/sec) | Latency (ns) |\n"
    md_content += "|------|----|-----------|---------|-----------------------|-------------|\n"
    
    for result in results:
        if result.category == "standard":
            md_content += f"| {result.script_from} | {result.script_to} | {result.text_size} | {result.api_type} | {result.throughput_chars_per_sec:.0f} | {result.latency_ns:.0f} |\n"
    
    # Extension Scripts Performance
    md_content += "\n## Extension Scripts (Roman/ASCII)\n\n"
    md_content += "| From | To | Text Size | API Type | Throughput (chars/sec) | Latency (ns) |\n"
    md_content += "|------|----|-----------|---------|-----------------------|-------------|\n"
    
    for result in results:
        if result.category == "extension":
            md_content += f"| {result.script_from} | {result.script_to} | {result.text_size} | {result.api_type} | {result.throughput_chars_per_sec:.0f} | {result.latency_ns:.0f} |\n"
    
    # Cross-Category Performance
    md_content += "\n## Cross-Category Performance\n\n"
    md_content += "| From | To | Category | Text Size | API Type | Throughput (chars/sec) | Latency (ns) |\n"
    md_content += "|------|----|-----------|-----------|---------|-----------------------|-------------|\n"
    
    for result in results:
        if result.category.startswith("cross_"):
            md_content += f"| {result.script_from} | {result.script_to} | {result.category} | {result.text_size} | {result.api_type} | {result.throughput_chars_per_sec:.0f} | {result.latency_ns:.0f} |\n"
    
    # API Method Comparison
    md_content += "\n## API Method Performance Comparison\n\n"
    
    api_stats = {}
    for result in results:
        if result.api_type not in api_stats:
            api_stats[result.api_type] = []
        api_stats[result.api_type].append(result.throughput_chars_per_sec)
    
    md_content += "| API Method | Average Throughput (chars/sec) | Count |\n"
    md_content += "|------------|-------------------------------|-------|\n"
    
    for api_type, throughputs in api_stats.items():
        avg_throughput = statistics.mean(throughputs)
        count = len(throughputs)
        md_content += f"| {api_type} | {avg_throughput:.0f} | {count} |\n"
    
    # Category Summary
    md_content += "\n## Category Summary\n\n"
    
    category_stats = {}
    for result in results:
        if result.category not in category_stats:
            category_stats[result.category] = []
        category_stats[result.category].append(result.throughput_chars_per_sec)
    
    md_content += "| Category | Average Throughput (chars/sec) | Count |\n"
    md_content += "|----------|-------------------------------|-------|\n"
    
    for category, throughputs in category_stats.items():
        avg_throughput = statistics.mean(throughputs)
        count = len(throughputs)
        md_content += f"| {category} | {avg_throughput:.0f} | {count} |\n"
    
    return md_content

def main():
    """Run comprehensive Python API benchmarks"""
    
    # Create transliterator instance
    transliterator = shlesha.Shlesha()
    
    # Collect all results
    results = []
    
    print("Running Hub Scripts benchmarks...")
    benchmark_category("hub", HUB_SCRIPTS, transliterator, results)
    
    print("Running Standard Scripts benchmarks...")
    benchmark_category("standard", STANDARD_SCRIPTS, transliterator, results)
    
    print("Running Extension Scripts benchmarks...")
    benchmark_category("extension", EXTENSION_SCRIPTS, transliterator, results)
    
    print("Running Cross-Category benchmarks...")
    benchmark_cross_category(transliterator, results)
    
    # Write results
    os.makedirs("target", exist_ok=True)
    write_csv_results(results, "target/python_benchmark_results.csv")
    
    # Generate markdown report
    md_content = generate_markdown_report(results)
    with open("target/PYTHON_BENCHMARK_RESULTS.md", "w") as f:
        f.write(md_content)
    
    print(f"Benchmarks complete. Results written to:")
    print("  target/python_benchmark_results.csv")
    print("  target/PYTHON_BENCHMARK_RESULTS.md")
    print(f"Total measurements: {len(results)}")

if __name__ == "__main__":
    main()