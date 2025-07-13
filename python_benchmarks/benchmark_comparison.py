#!/usr/bin/env python3
"""
Fair comparison benchmarks between Shlesha and other transliteration tools.
Compares only on common supported scripts and features.
"""

import time
import statistics
import csv
import os
import importlib.util

# Import Shlesha
import shlesha

# Try to import other transliteration libraries
other_libs = {}

# Try importing vidyut
try:
    from vidyut.lipi import Scheme, transliterate
    other_libs['vidyut'] = {'Scheme': Scheme, 'transliterate': transliterate}
    print("✓ vidyut found")
except ImportError:
    print("✗ vidyut not found (pip install vidyut)")

# Try importing indic-transliteration
try:
    from indic_transliteration import sanscript
    other_libs['indic_transliteration'] = sanscript
    print("✓ indic-transliteration found")
except ImportError:
    print("✗ indic-transliteration not found (pip install indic-transliteration)")

# Try importing aksharamukha
try:
    from aksharamukha import transliterate as aksh_transliterate
    other_libs['aksharamukha'] = aksh_transliterate
    print("✓ aksharamukha found")
except ImportError:
    print("✗ aksharamukha not found (pip install aksharamukha)")

# Test data - using appropriate scripts for each conversion
TEST_DATA = {
    "devanagari": {
        "small": "धर्म",
        "medium": "धर्म योग भारत संस्कृत वेद उपनिषद् गीता रामायण महाभारत",
        "large": "धर्म योग भारत संस्कृत वेद उपनिषद् गीता रामायण महाभारत पुराण शास्त्र दर्शन आयुर्वेद ज्योतिष व्याकरण छन्द निरुक्त कल्प शिक्षा स्मृति श्रुति आचार विचार संस्कार परम्परा सत्य अहिंसा करुणा दया प्रेम शान्ति आनन्द मोक्ष निर्वाण समाधि ध्यान प्राणायाम आसन मन्त्र यन्त्र तन्त्र"
    },
    "iast": {
        "small": "dharma",
        "medium": "dharma yoga bhārata saṃskṛta veda upaniṣad gītā rāmāyaṇa mahābhārata",
        "large": "dharma yoga bhārata saṃskṛta veda upaniṣad gītā rāmāyaṇa mahābhārata purāṇa śāstra darśana āyurveda jyotiṣa vyākaraṇa chanda nirukta kalpa śikṣā smṛti śruti ācāra vicāra saṃskāra paramparā satya ahiṃsā karuṇā dayā prema śānti ānanda mokṣa nirvāṇa samādhi dhyāna prāṇāyāma āsana mantra yantra tantra"
    },
    "itrans": {
        "small": "dharma",
        "medium": "dharma yoga bhArata sa~nskR^ita veda upaniShad gItA rAmAyaNa mahAbhArata",
        "large": "dharma yoga bhArata sa~nskR^ita veda upaniShad gItA rAmAyaNa mahAbhArata purANa shAstra darshana Ayurveda jyotiSha vyAkaraNa chanda nirukta kalpa shikShA smR^iti shruti AchAra vichAra sa~nskAra paramparA satya ahi~nsA karuNA dayA prema shAnti Ananda mokSha nirvANa samAdhi dhyAna prANAyAma Asana mantra yantra tantra"
    },
    "slp1": {
        "small": "Darma",
        "medium": "Darma yoga BArata saMskfta veda upanizad gItA rAmAyaNa mahABArata",
        "large": "Darma yoga BArata saMskfta veda upanizad gItA rAmAyaNa mahABArata purANa SAstra darSana Ayurveda jyotiza vyAkaraNa Canda nirukta kalpa SikzA smfti Sruti AcAra vicAra saMskAra paramparA satya ahiMsA karuNA dayA prema SAnti Ananda mokza nirvANa samADi DyAna prANAyAma Asana mantra yantra tantra"
    }
}

# Common script mappings across libraries
COMMON_CONVERSIONS = [
    # Devanagari to others (current tests)
    ("devanagari", "iast"),
    ("devanagari", "itrans"),
    ("devanagari", "slp1"),
    ("devanagari", "telugu"),
    ("devanagari", "tamil"),
    # Roman to Indic
    ("iast", "devanagari"),
    ("iast", "telugu"),
    ("iast", "tamil"),
    ("itrans", "devanagari"),
    ("slp1", "devanagari"),
    # Roman to Roman
    ("iast", "itrans"),
    ("iast", "slp1"),
    ("itrans", "slp1"),
]

class ComparisonResult:
    def __init__(self, library, from_script, to_script, text_size, throughput, latency, success=True, error=None):
        self.library = library
        self.from_script = from_script
        self.to_script = to_script
        self.text_size = text_size
        self.throughput = throughput
        self.latency = latency
        self.success = success
        self.error = error

def benchmark_shlesha(text, from_script, to_script, iterations=1000):
    """Benchmark Shlesha"""
    transliterator = shlesha.Shlesha()
    times = []
    
    # Warmup
    for _ in range(10):
        transliterator.transliterate(text, from_script, to_script)
    
    # Benchmark
    for _ in range(iterations):
        start = time.perf_counter_ns()
        transliterator.transliterate(text, from_script, to_script)
        end = time.perf_counter_ns()
        times.append(end - start)
    
    avg_time_ns = statistics.mean(times)
    throughput = len(text) / (avg_time_ns / 1_000_000_000)
    
    return throughput, avg_time_ns

def benchmark_vidyut(text, from_script, to_script, iterations=1000):
    """Benchmark vidyut"""
    if 'vidyut' not in other_libs:
        return None, None
    
    vidyut_lib = other_libs['vidyut']
    Scheme = vidyut_lib['Scheme']
    transliterate = vidyut_lib['transliterate']
    
    # Map script names to vidyut scheme names
    vidyut_map = {
        "devanagari": "Devanagari",
        "iast": "Iast", 
        "itrans": "Itrans",
        "slp1": "Slp1",
        "telugu": "Telugu",
        "tamil": "Tamil"
    }
    
    if from_script not in vidyut_map or to_script not in vidyut_map:
        return None, None
    
    try:
        from_scheme = getattr(Scheme, vidyut_map[from_script])
        to_scheme = getattr(Scheme, vidyut_map[to_script])
    except AttributeError as e:
        print(f"    vidyut AttributeError: {e}")
        return None, None
    
    times = []
    
    # Warmup
    try:
        for _ in range(10):
            transliterate(text, from_scheme, to_scheme)
    except Exception as e:
        print(f"    vidyut warmup error: {e}")
        return None, None
    
    # Benchmark
    for _ in range(iterations):
        start = time.perf_counter_ns()
        transliterate(text, from_scheme, to_scheme)
        end = time.perf_counter_ns()
        times.append(end - start)
    
    avg_time_ns = statistics.mean(times)
    throughput = len(text) / (avg_time_ns / 1_000_000_000)
    
    return throughput, avg_time_ns

def benchmark_indic_transliteration(text, from_script, to_script, iterations=1000):
    """Benchmark indic-transliteration"""
    if 'indic_transliteration' not in other_libs:
        return None, None
    
    sanscript = other_libs['indic_transliteration']
    
    # Map script names
    script_map = {
        "devanagari": sanscript.DEVANAGARI,
        "iast": sanscript.IAST,
        "itrans": sanscript.ITRANS,
        "slp1": sanscript.SLP1,
        "telugu": sanscript.TELUGU,
        "tamil": sanscript.TAMIL
    }
    
    if from_script not in script_map or to_script not in script_map:
        return None, None
    
    times = []
    
    # Warmup
    for _ in range(10):
        sanscript.transliterate(text, script_map[from_script], script_map[to_script])
    
    # Benchmark
    for _ in range(iterations):
        start = time.perf_counter_ns()
        sanscript.transliterate(text, script_map[from_script], script_map[to_script])
        end = time.perf_counter_ns()
        times.append(end - start)
    
    avg_time_ns = statistics.mean(times)
    throughput = len(text) / (avg_time_ns / 1_000_000_000)
    
    return throughput, avg_time_ns

def benchmark_aksharamukha(text, from_script, to_script, iterations=100):  # Fewer iterations as it's slower
    """Benchmark aksharamukha"""
    if 'aksharamukha' not in other_libs:
        return None, None
    
    aksh_transliterate = other_libs['aksharamukha']
    
    # Map script names
    script_map = {
        "devanagari": "Devanagari",
        "iast": "IAST",
        "itrans": "ITRANS", 
        "slp1": "SLP1",
        "telugu": "Telugu",
        "tamil": "Tamil"
    }
    
    if from_script not in script_map or to_script not in script_map:
        return None, None
    
    times = []
    
    # Warmup
    for _ in range(5):
        aksh_transliterate.process(script_map[from_script], script_map[to_script], text)
    
    # Benchmark
    for _ in range(iterations):
        start = time.perf_counter_ns()
        aksh_transliterate.process(script_map[from_script], script_map[to_script], text)
        end = time.perf_counter_ns()
        times.append(end - start)
    
    avg_time_ns = statistics.mean(times)
    throughput = len(text) / (avg_time_ns / 1_000_000_000)
    
    return throughput, avg_time_ns

def run_comparison_benchmarks():
    """Run fair comparison benchmarks"""
    results = []
    
    for from_script, to_script in COMMON_CONVERSIONS:
        for size_name in ["small", "medium", "large"]:
            # Get appropriate test data for the source script
            if from_script in TEST_DATA:
                text = TEST_DATA[from_script][size_name]
            else:
                text = TEST_DATA["devanagari"][size_name]  # fallback
            
            print(f"\nBenchmarking {from_script} → {to_script} ({size_name})...")
            
            # Benchmark Shlesha
            try:
                throughput, latency = benchmark_shlesha(text, from_script, to_script)
                results.append(ComparisonResult(
                    "shlesha", from_script, to_script, size_name, 
                    throughput, latency
                ))
                print(f"  Shlesha: {throughput:.0f} chars/sec")
            except Exception as e:
                results.append(ComparisonResult(
                    "shlesha", from_script, to_script, size_name,
                    0, 0, False, str(e)
                ))
            
            # Benchmark vidyut
            if 'vidyut' in other_libs:
                try:
                    throughput, latency = benchmark_vidyut(text, from_script, to_script)
                    if throughput:
                        results.append(ComparisonResult(
                            "vidyut", from_script, to_script, size_name,
                            throughput, latency
                        ))
                        print(f"  vidyut: {throughput:.0f} chars/sec")
                except Exception as e:
                    results.append(ComparisonResult(
                        "vidyut", from_script, to_script, size_name,
                        0, 0, False, str(e)
                    ))
            
            # Benchmark indic-transliteration
            if 'indic_transliteration' in other_libs:
                try:
                    throughput, latency = benchmark_indic_transliteration(text, from_script, to_script)
                    if throughput:
                        results.append(ComparisonResult(
                            "indic_transliteration", from_script, to_script, size_name,
                            throughput, latency
                        ))
                        print(f"  indic-transliteration: {throughput:.0f} chars/sec")
                except Exception as e:
                    results.append(ComparisonResult(
                        "indic_transliteration", from_script, to_script, size_name,
                        0, 0, False, str(e)
                    ))
            
            # Benchmark aksharamukha
            if 'aksharamukha' in other_libs:
                try:
                    throughput, latency = benchmark_aksharamukha(text, from_script, to_script)
                    if throughput:
                        results.append(ComparisonResult(
                            "aksharamukha", from_script, to_script, size_name,
                            throughput, latency
                        ))
                        print(f"  aksharamukha: {throughput:.0f} chars/sec")
                except Exception as e:
                    results.append(ComparisonResult(
                        "aksharamukha", from_script, to_script, size_name,
                        0, 0, False, str(e)
                    ))
    
    return results

def generate_comparison_markdown(results):
    """Generate markdown comparison report"""
    md = "# Transliteration Library Performance Comparison\n\n"
    
    # Group by conversion type
    conversions = {}
    for result in results:
        key = f"{result.from_script}_to_{result.to_script}"
        if key not in conversions:
            conversions[key] = {}
        if result.text_size not in conversions[key]:
            conversions[key][result.text_size] = {}
        conversions[key][result.text_size][result.library] = result
    
    # Performance comparison by conversion
    for conversion, sizes in conversions.items():
        from_script, to_script = conversion.replace("_to_", " → ").split(" → ")
        md += f"\n## {from_script} → {to_script}\n\n"
        
        md += "| Text Size | Library | Throughput (chars/sec) | Latency (ns) | Relative Speed |\n"
        md += "|-----------|---------|----------------------|--------------|----------------|\n"
        
        for size_name in ["small", "medium", "large"]:
            if size_name in sizes:
                # Find baseline (shlesha)
                baseline = sizes[size_name].get("shlesha")
                if baseline and baseline.success:
                    baseline_throughput = baseline.throughput
                else:
                    baseline_throughput = None
                
                for library, result in sorted(sizes[size_name].items()):
                    if result.success:
                        relative = ""
                        if baseline_throughput and library != "shlesha":
                            ratio = result.throughput / baseline_throughput
                            relative = f"{ratio:.2f}x"
                        elif library == "shlesha":
                            relative = "1.00x (baseline)"
                        
                        md += f"| {size_name} | {library} | {result.throughput:.0f} | {result.latency:.0f} | {relative} |\n"
                    else:
                        md += f"| {size_name} | {library} | ERROR | ERROR | N/A |\n"
    
    # Overall performance summary
    md += "\n## Overall Performance Summary\n\n"
    md += "| Library | Average Throughput (chars/sec) | Conversions Tested | Success Rate |\n"
    md += "|---------|-------------------------------|-------------------|-------------|\n"
    
    library_stats = {}
    for result in results:
        if result.library not in library_stats:
            library_stats[result.library] = {"throughputs": [], "total": 0, "success": 0}
        library_stats[result.library]["total"] += 1
        if result.success:
            library_stats[result.library]["success"] += 1
            library_stats[result.library]["throughputs"].append(result.throughput)
    
    for library, stats in sorted(library_stats.items()):
        if stats["throughputs"]:
            avg_throughput = statistics.mean(stats["throughputs"])
        else:
            avg_throughput = 0
        success_rate = (stats["success"] / stats["total"]) * 100
        md += f"| {library} | {avg_throughput:.0f} | {stats['total']} | {success_rate:.0f}% |\n"
    
    return md

def main():
    """Run comparison benchmarks"""
    print("Running transliteration library comparison benchmarks...")
    print(f"Libraries found: {list(other_libs.keys())}")
    
    if not other_libs:
        print("\nNo other libraries found for comparison!")
        print("Install comparison libraries with:")
        print("  pip install vidyut-py indic-transliteration aksharamukha")
        return
    
    results = run_comparison_benchmarks()
    
    # Save results
    os.makedirs("target", exist_ok=True)
    
    # Save CSV
    with open("target/comparison_benchmark_results.csv", "w", newline="") as f:
        writer = csv.writer(f)
        writer.writerow(["library", "from_script", "to_script", "text_size", "throughput_chars_per_sec", "latency_ns", "success"])
        for result in results:
            writer.writerow([
                result.library, result.from_script, result.to_script, result.text_size,
                f"{result.throughput:.0f}", f"{result.latency:.0f}", result.success
            ])
    
    # Generate and save markdown
    md = generate_comparison_markdown(results)
    with open("target/COMPARISON_BENCHMARK_RESULTS.md", "w") as f:
        f.write(md)
    
    print(f"\nComparison benchmarks complete!")
    print(f"Results saved to:")
    print(f"  target/comparison_benchmark_results.csv")
    print(f"  target/COMPARISON_BENCHMARK_RESULTS.md")

if __name__ == "__main__":
    main()