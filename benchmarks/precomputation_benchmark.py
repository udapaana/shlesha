#!/usr/bin/env python3
"""
Benchmark script to compare Shlesha performance with and without pre-computation optimization.
This script builds Shlesha with different feature flags and compares performance.
"""

import subprocess
import os
import time
import statistics
import csv
import json
from pathlib import Path

# Test data for different script types
TEST_DATA = {
    "devanagari": {
        "small": "धर्म",
        "medium": "धर्म योग भारत संस्कृत वेद उपनिषद् गीता रामायण महाभारत",
        "large": "धर्म योग भारत संस्कृत वेद उपनिषद् गीता रामायण महाभारत पुराण शास्त्र दर्शन आयुर्वेद ज्योतिष व्याकरण छन्द निरुक्त कल्प शिक्षा स्मृति श्रुति आचार विचार संस्कार परम्परा सत्य अहिंसा करुणा दया प्रेम शान्ति आनन्द मोक्ष निर्वाण समाधि ध्यान प्राणायाम आसन मन्त्र यन्त्र तन्त्र",
        "very_large": "धर्म योग भारत संस्कृत वेद उपनिषद् गीता रामायण महाभारत पुराण शास्त्र दर्शन आयुर्वेद ज्योतिष व्याकरण छन्द निरुक्त कल्प शिक्षा स्मृति श्रुति आचार विचार संस्कार परम्परा सत्य अहिंसा करुणा दया प्रेम शान्ति आनन्द मोक्ष निर्वाण समाधि ध्यान प्राणायाम आसन मन्त्र यन्त्र तन्त्र ब्रह्म आत्मा जीव प्रकृति पुरुष गुण कर्म फल संसार मुक्ति लीला माया इच्छा संकल्प विकल्प निर्विकल्प सत्चित्आनन्द ॐ गायत्री मन्त्र हरि ॐ नमो भगवते वासुदेवाय"
    },
    "iast": {
        "small": "dharma",
        "medium": "dharma yoga bhārata saṃskṛta veda upaniṣad gītā rāmāyaṇa mahābhārata",
        "large": "dharma yoga bhārata saṃskṛta veda upaniṣad gītā rāmāyaṇa mahābhārata purāṇa śāstra darśana āyurveda jyotiṣa vyākaraṇa chanda nirukta kalpa śikṣā smṛti śruti ācāra vicāra saṃskāra paramparā satya ahiṃsā karuṇā dayā prema śānti ānanda mokṣa nirvāṇa samādhi dhyāna prāṇāyāma āsana mantra yantra tantra",
        "very_large": "dharma yoga bhārata saṃskṛta veda upaniṣad gītā rāmāyaṇa mahābhārata purāṇa śāstra darśana āyurveda jyotiṣa vyākaraṇa chanda nirukta kalpa śikṣā smṛti śruti ācāra vicāra saṃskāra paramparā satya ahiṃsā karuṇā dayā prema śānti ānanda mokṣa nirvāṇa samādhi dhyāna prāṇāyāma āsana mantra yantra tantra brahma ātmā jīva prakṛti puruṣa guṇa karma phala saṃsāra mukti līlā māyā icchā saṅkalpa vikalpa nirvikalpa saccidānanda oṃ gāyatrī mantra hari oṃ namo bhagavate vāsudevāya"
    },
    "itrans": {
        "small": "dharma",
        "medium": "dharma yoga bhArata sa~nskR^ita veda upaniShad gItA rAmAyaNa mahAbhArata", 
        "large": "dharma yoga bhArata sa~nskR^ita veda upaniShad gItA rAmAyaNa mahAbhArata purANa shAstra darshana Ayurveda jyotiSha vyAkaraNa chanda nirukta kalpa shikShA smR^iti shruti AchAra vichAra sa~nskAra paramparA satya ahi~nsA karuNA dayA prema shAnti Ananda mokSha nirvANa samAdhi dhyAna prANAyAma Asana mantra yantra tantra",
        "very_large": "dharma yoga bhArata sa~nskR^ita veda upaniShad gItA rAmAyaNa mahAbhArata purANa shAstra darshana Ayurveda jyotiSha vyAkaraNa chanda nirukta kalpa shikShA smR^iti shruti AchAra vichAra sa~nskAra paramparA satya ahi~nsA karuNA dayA prema shAnti Ananda mokSha nirvANa samAdhi dhyAna prANAyAma Asana mantra yantra tantra brahma AtmA jIva prakR^iti puruSha guNa karma phala sa~nsAra mukti lIlA mAyA icchA sa~Nkalpa vikalpa nirvikalpa saccidAnanda om gAyatrI mantra hari om namo bhagavate vAsudevAya"
    },
    "slp1": {
        "small": "Darma",
        "medium": "Darma yoga BArata saMskfta veda upanizad gItA rAmAyaNa mahABArata",
        "large": "Darma yoga BArata saMskfta veda upanizad gItA rAmAyaNa mahABArata purANa SAstra darSana Ayurveda jyotiza vyAkaraNa Canda nirukta kalpa SikzA smfti Sruti AcAra vicAra saMskAra paramparA satya ahiMsA karuNA dayA prema SAnti Ananda mokza nirvANa samADi DyAna prANAyAma Asana mantra yantra tantra",
        "very_large": "Darma yoga BArata saMskfta veda upanizad gItA rAmAyaNa mahABArata purANa SAstra darSana Ayurveda jyotiza vyAkaraNa Canda nirukta kalpa SikzA smfti Sruti AcAra vicAra saMskAra paramparA satya ahiMsA karuNA dayA prema SAnti Ananda mokza nirvANa samADi DyAna prANAyAma Asana mantra yantra tantra brahma AtmA jIva prakfti puruza guNa karma Pala saMsAra mukti lIlA mAyA icCA saNkalpa vikalpa nirvikalpa saccidAnanda om gAyatrI mantra hari om namo Bagavate vAsudevAya"
    }
}

# Conversion types to test (focusing on Roman ↔ Indic where pre-computation helps)
PRECOMPUTATION_CONVERSIONS = [
    # Roman → Indic (2-step → 1-step optimization)
    ("iast", "devanagari"),
    ("itrans", "devanagari"), 
    ("slp1", "devanagari"),
    # Indic → Roman (2-step → 1-step optimization)
    ("devanagari", "iast"),
    ("devanagari", "itrans"),
    ("devanagari", "slp1"),
    # For comparison: conversions that don't benefit from pre-computation
    ("devanagari", "telugu"),  # Indic → Indic (already 1-step)
    ("iast", "itrans"),        # Roman → Roman (already 1-step)
]

# Build configurations to test
BUILD_CONFIGS = [
    {
        "name": "no-precompute",
        "features": ["no-precompute"],
        "no_default": True,
        "description": "Hub-and-spoke only (no pre-computation)"
    },
    {
        "name": "precompute-common", 
        "features": ["precompute-common"],
        "no_default": False,
        "description": "Common conversions pre-computed (IAST, ITRANS, SLP1 ↔ Devanagari)"
    },
    {
        "name": "precompute-roman-indic",
        "features": ["precompute-roman-indic"],
        "no_default": False, 
        "description": "All Roman → Indic conversions pre-computed"
    },
    {
        "name": "precompute-indic-roman",
        "features": ["precompute-indic-roman"],
        "no_default": False,
        "description": "All Indic → Roman conversions pre-computed"
    }
]

class PrecomputationBenchmarkResult:
    def __init__(self, config_name, from_script, to_script, text_size, throughput, latency, binary_size_mb=None, success=True, error=None):
        self.config_name = config_name
        self.from_script = from_script
        self.to_script = to_script
        self.text_size = text_size
        self.throughput = throughput
        self.latency = latency
        self.binary_size_mb = binary_size_mb
        self.success = success
        self.error = error

def build_shlesha_with_config(config):
    """Build Shlesha with specific feature configuration"""
    print(f"Building Shlesha with config: {config['name']}...")
    
    cmd = ["cargo", "build", "--release"]
    
    if config["no_default"]:
        cmd.append("--no-default-features")
    
    if config["features"]:
        cmd.extend(["--features", ",".join(config["features"])])
    
    try:
        # Build the library
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        
        # Build Python bindings
        maturin_cmd = ["uv", "run", "maturin", "develop", "--release"]
        if config["no_default"]:
            maturin_cmd.append("--no-default-features")
        if config["features"]:
            maturin_cmd.extend(["--features", ",".join(config["features"])])
        
        subprocess.run(maturin_cmd, capture_output=True, text=True, check=True)
        
        return True, None
    except subprocess.CalledProcessError as e:
        return False, f"Build failed: {e.stderr}"

def get_binary_size():
    """Get the size of the compiled binary in MB"""
    binary_path = Path("target/release/libshlesha.rlib")
    if binary_path.exists():
        size_bytes = binary_path.stat().st_size
        return size_bytes / (1024 * 1024)  # Convert to MB
    return None

def benchmark_config(config, text, from_script, to_script, iterations=1000):
    """Benchmark a specific configuration"""
    try:
        # Import shlesha (will use the currently built version)
        import importlib
        import sys
        if 'shlesha' in sys.modules:
            importlib.reload(sys.modules['shlesha'])
        import shlesha
        
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
        
        binary_size = get_binary_size()
        
        return PrecomputationBenchmarkResult(
            config["name"], from_script, to_script, "medium", 
            throughput, avg_time_ns, binary_size
        )
        
    except Exception as e:
        return PrecomputationBenchmarkResult(
            config["name"], from_script, to_script, "medium",
            0, 0, None, False, str(e)
        )

def run_comprehensive_benchmark():
    """Run comprehensive pre-computation benchmarks"""
    results = []
    
    for config in BUILD_CONFIGS:
        print(f"\n{'='*60}")
        print(f"Testing configuration: {config['name']}")
        print(f"Description: {config['description']}")
        print(f"{'='*60}")
        
        # Build with this configuration
        success, error = build_shlesha_with_config(config)
        if not success:
            print(f"Failed to build {config['name']}: {error}")
            continue
        
        print(f"✓ Build successful for {config['name']}")
        
        # Test all conversions
        for from_script, to_script in PRECOMPUTATION_CONVERSIONS:
            # Get appropriate test data
            if from_script in TEST_DATA:
                text = TEST_DATA[from_script]["medium"]
            else:
                text = TEST_DATA["devanagari"]["medium"]  # fallback
            
            print(f"  Benchmarking {from_script} → {to_script}...")
            
            result = benchmark_config(config, text, from_script, to_script)
            results.append(result)
            
            if result.success:
                print(f"    {result.throughput:.0f} chars/sec")
            else:
                print(f"    ERROR: {result.error}")
    
    return results

def analyze_performance_improvements(results):
    """Analyze performance improvements from pre-computation"""
    analysis = {}
    
    # Group results by conversion
    by_conversion = {}
    for result in results:
        if not result.success:
            continue
        
        conversion_key = f"{result.from_script}_{result.to_script}"
        if conversion_key not in by_conversion:
            by_conversion[conversion_key] = {}
        by_conversion[conversion_key][result.config_name] = result
    
    # Calculate improvements
    for conversion, configs in by_conversion.items():
        if "no-precompute" in configs:
            baseline = configs["no-precompute"]
            baseline_throughput = baseline.throughput
            
            conversion_analysis = {
                "baseline_throughput": baseline_throughput,
                "improvements": {}
            }
            
            for config_name, result in configs.items():
                if config_name != "no-precompute":
                    improvement = result.throughput / baseline_throughput
                    conversion_analysis["improvements"][config_name] = {
                        "throughput": result.throughput,
                        "improvement_factor": improvement,
                        "improvement_percent": (improvement - 1) * 100
                    }
            
            analysis[conversion] = conversion_analysis
    
    return analysis

def generate_precomputation_report(results, analysis):
    """Generate detailed pre-computation performance report"""
    md = "# Shlesha Pre-computation Performance Analysis\n\n"
    md += "This report compares Shlesha's performance with and without compile-time pre-computation optimization.\n\n"
    
    # Configuration overview
    md += "## Build Configurations Tested\n\n"
    md += "| Configuration | Description | Binary Size (MB) |\n"
    md += "|---------------|-------------|------------------|\n"
    
    size_by_config = {}
    for result in results:
        if result.binary_size_mb is not None:
            size_by_config[result.config_name] = result.binary_size_mb
            break
    
    for config in BUILD_CONFIGS:
        size = size_by_config.get(config["name"], "N/A")
        if isinstance(size, float):
            size = f"{size:.1f}"
        md += f"| `{config['name']}` | {config['description']} | {size} |\n"
    
    # Performance comparison
    md += "\n## Performance Comparison by Conversion Type\n\n"
    
    for conversion, conv_analysis in analysis.items():
        from_script, to_script = conversion.split("_", 1)
        md += f"### {from_script} → {to_script}\n\n"
        
        md += "| Configuration | Throughput (chars/sec) | vs Baseline | Improvement |\n"
        md += "|---------------|----------------------|-------------|-------------|\n"
        
        baseline = conv_analysis["baseline_throughput"]
        md += f"| `no-precompute` | {baseline:.0f} | 1.00x | baseline |\n"
        
        for config_name, improvement_data in conv_analysis["improvements"].items():
            throughput = improvement_data["throughput"]
            factor = improvement_data["improvement_factor"]
            percent = improvement_data["improvement_percent"]
            md += f"| `{config_name}` | {throughput:.0f} | {factor:.2f}x | +{percent:.1f}% |\n"
        
        md += "\n"
    
    # Performance impact analysis
    md += "## Performance Impact Analysis\n\n"
    
    roman_to_indic = []
    indic_to_roman = []
    other_conversions = []
    
    for conversion, conv_analysis in analysis.items():
        from_script, to_script = conversion.split("_", 1)
        
        # Get best improvement
        best_improvement = 0
        for improvement_data in conv_analysis["improvements"].values():
            best_improvement = max(best_improvement, improvement_data["improvement_factor"])
        
        if from_script in ["iast", "itrans", "slp1"] and to_script == "devanagari":
            roman_to_indic.append((conversion, best_improvement))
        elif from_script == "devanagari" and to_script in ["iast", "itrans", "slp1"]:
            indic_to_roman.append((conversion, best_improvement))
        else:
            other_conversions.append((conversion, best_improvement))
    
    if roman_to_indic:
        avg_improvement = statistics.mean([imp for _, imp in roman_to_indic])
        md += f"### Roman → Indic Conversions\n"
        md += f"- **Average improvement**: {avg_improvement:.2f}x ({(avg_improvement-1)*100:.1f}%)\n"
        md += f"- **Conversions tested**: {len(roman_to_indic)}\n"
        md += f"- **Optimization mechanism**: Direct Roman→Devanagari mapping bypasses ISO-15919 hub\n\n"
    
    if indic_to_roman:
        avg_improvement = statistics.mean([imp for _, imp in indic_to_roman])
        md += f"### Indic → Roman Conversions\n"
        md += f"- **Average improvement**: {avg_improvement:.2f}x ({(avg_improvement-1)*100:.1f}%)\n"
        md += f"- **Conversions tested**: {len(indic_to_roman)}\n"
        md += f"- **Optimization mechanism**: Direct Devanagari→Roman mapping bypasses ISO-15919 hub\n\n"
    
    if other_conversions:
        avg_improvement = statistics.mean([imp for _, imp in other_conversions])
        md += f"### Other Conversions (Control Group)\n"
        md += f"- **Average change**: {avg_improvement:.2f}x ({(avg_improvement-1)*100:.1f}%)\n"
        md += f"- **Conversions tested**: {len(other_conversions)}\n"
        md += f"- **Expected**: No significant improvement (already optimized)\n\n"
    
    # Binary size vs performance trade-off
    md += "## Binary Size vs Performance Trade-off\n\n"
    md += "| Configuration | Binary Size | Performance Gain | Size/Performance Ratio |\n"
    md += "|---------------|-------------|------------------|------------------------|\n"
    
    baseline_size = size_by_config.get("no-precompute", 0)
    for config in BUILD_CONFIGS:
        config_name = config["name"]
        size = size_by_config.get(config_name, 0)
        
        if config_name == "no-precompute":
            md += f"| `{config_name}` | {size:.1f} MB | baseline | baseline |\n"
        else:
            # Calculate average performance gain for this config
            total_improvement = 0
            count = 0
            for conv_analysis in analysis.values():
                if config_name in conv_analysis["improvements"]:
                    total_improvement += conv_analysis["improvements"][config_name]["improvement_factor"]
                    count += 1
            
            if count > 0:
                avg_improvement = total_improvement / count
                size_increase = size - baseline_size
                ratio = size_increase / (avg_improvement - 1) if avg_improvement > 1 else float('inf')
                md += f"| `{config_name}` | {size:.1f} MB | {avg_improvement:.2f}x | {ratio:.1f} MB/x |\n"
    
    # Recommendations
    md += "\n## Recommendations\n\n"
    md += "### For Applications\n\n"
    md += "- **General purpose**: Use `precompute-common` (default) - balances performance and binary size\n"
    md += "- **Roman↔Indic heavy**: Use `precompute-roman-indic` or `precompute-indic-roman` based on primary direction\n"
    md += "- **Memory constrained**: Use `no-precompute` - minimal binary size with standard performance\n"
    md += "- **Performance critical**: Use the specific feature flag matching your conversion patterns\n\n"
    
    md += "### For Libraries\n\n"
    md += "- **Default distribution**: Ship with `no-precompute` - let users choose optimization level\n"
    md += "- **Application bundles**: Include `precompute-common` for broad performance improvement\n"
    md += "- **Specialized use cases**: Enable specific flags based on usage patterns\n\n"
    
    # Technical details
    md += "## Technical Implementation\n\n"
    md += "### How Pre-computation Works\n\n"
    md += "1. **Build-time generation**: `build.rs` composes Roman→ISO and ISO→Devanagari mappings\n"
    md += "2. **Direct lookup tables**: Generated converters use single HashMap lookup\n"
    md += "3. **Runtime selection**: Registry automatically uses direct converters when available\n"
    md += "4. **Fallback guarantee**: Always falls back to hub-and-spoke for missing converters\n\n"
    
    md += "### Performance Characteristics\n\n"
    md += "- **Roman→Indic**: ~2x improvement (2 lookups → 1 lookup)\n"
    md += "- **Indic→Roman**: ~2x improvement (2 lookups → 1 lookup)\n"
    md += "- **Indic→Indic**: No change (already direct)\n"
    md += "- **Roman→Roman**: No change (already direct)\n"
    md += "- **Memory usage**: Increases with number of pre-computed converters\n"
    md += "- **Compile time**: Increases with scope of pre-computation\n\n"
    
    return md

def main():
    """Run pre-computation benchmarks and generate report"""
    print("Shlesha Pre-computation Performance Benchmark")
    print("=" * 50)
    
    # Run comprehensive benchmarks
    results = run_comprehensive_benchmark()
    
    # Analyze performance improvements
    analysis = analyze_performance_improvements(results)
    
    # Generate report
    report = generate_precomputation_report(results, analysis)
    
    # Save results
    os.makedirs("target", exist_ok=True)
    
    # Save CSV data
    with open("target/precomputation_benchmark_results.csv", "w", newline="") as f:
        writer = csv.writer(f)
        writer.writerow([
            "config_name", "from_script", "to_script", "text_size", 
            "throughput_chars_per_sec", "latency_ns", "binary_size_mb", "success"
        ])
        for result in results:
            writer.writerow([
                result.config_name, result.from_script, result.to_script, result.text_size,
                f"{result.throughput:.0f}", f"{result.latency:.0f}", 
                result.binary_size_mb if result.binary_size_mb else "N/A",
                result.success
            ])
    
    # Save JSON analysis
    with open("target/precomputation_analysis.json", "w") as f:
        json.dump(analysis, f, indent=2)
    
    # Save markdown report
    with open("target/PRECOMPUTATION_BENCHMARK_REPORT.md", "w") as f:
        f.write(report)
    
    print(f"\n{'='*50}")
    print("Pre-computation benchmarks complete!")
    print(f"Results saved to:")
    print(f"  target/precomputation_benchmark_results.csv")
    print(f"  target/precomputation_analysis.json")
    print(f"  target/PRECOMPUTATION_BENCHMARK_REPORT.md")
    print(f"{'='*50}")

if __name__ == "__main__":
    main()