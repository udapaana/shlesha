#!/usr/bin/env python3
"""
Unified Benchmark Runner for Shlesha Transliteration Comparisons
Orchestrates and aggregates results from all benchmark platforms:
- Rust vs Rust (native performance)
- Python vs Python (when bindings available)
- CLI vs CLI (command-line tools)
- WASM vs WASM (web assembly performance)
"""

import json
import time
import subprocess
import sys
import os
import shutil
import tempfile
from pathlib import Path
from typing import Dict, List, Any, Optional, Tuple
from dataclasses import dataclass, asdict
import statistics
import argparse

@dataclass
class BenchmarkConfig:
    """Configuration for benchmark execution"""
    run_rust: bool = True
    run_python: bool = True
    run_cli: bool = True
    run_wasm: bool = False  # Requires manual execution
    iterations: int = 50
    timeout: int = 300  # 5 minutes per benchmark
    output_dir: str = "unified_benchmark_results"
    include_accuracy: bool = True
    include_memory: bool = True

@dataclass
class ToolResult:
    """Results for a single tool"""
    name: str
    platform: str
    available: bool
    version: Optional[str] = None
    performance_metrics: Dict[str, Any] = None
    accuracy_score: Optional[float] = None
    memory_usage: Optional[Dict[str, float]] = None
    error_message: Optional[str] = None

@dataclass
class UnifiedResults:
    """Aggregated results from all platforms"""
    timestamp: str
    config: BenchmarkConfig
    system_info: Dict[str, Any]
    tools: List[ToolResult]
    summary: Dict[str, Any]
    recommendations: List[str]

class SystemInfo:
    """Collect system information for benchmark context"""
    
    @staticmethod
    def collect() -> Dict[str, Any]:
        info = {
            "platform": sys.platform,
            "python_version": sys.version,
            "timestamp": time.strftime('%Y-%m-%d %H:%M:%S'),
        }
        
        try:
            # Get Rust version
            result = subprocess.run(["rustc", "--version"], 
                                  capture_output=True, text=True, timeout=10)
            if result.returncode == 0:
                info["rust_version"] = result.stdout.strip()
        except:
            info["rust_version"] = "Not available"
        
        try:
            # Get system info
            if hasattr(os, 'uname'):
                uname = os.uname()
                info["system"] = {
                    "sysname": uname.sysname,
                    "release": uname.release,
                    "machine": uname.machine
                }
        except:
            pass
        
        try:
            # Get CPU info (Linux)
            if Path("/proc/cpuinfo").exists():
                with open("/proc/cpuinfo") as f:
                    cpu_info = f.read()
                    for line in cpu_info.split('\n'):
                        if line.startswith('model name'):
                            info["cpu"] = line.split(':')[1].strip()
                            break
        except:
            pass
        
        return info

class RustBenchmarkRunner:
    """Run Rust native benchmarks using Criterion"""
    
    def __init__(self, base_dir: Path):
        self.base_dir = base_dir
        
    def run(self, config: BenchmarkConfig) -> List[ToolResult]:
        """Run Rust benchmarks and parse results"""
        print("🦀 Running Rust vs Rust benchmarks...")
        
        results = []
        
        try:
            # Build benchmarks first
            build_cmd = ["cargo", "build", "--release", "--benches"]
            subprocess.run(build_cmd, cwd=self.base_dir, check=True, 
                          capture_output=True, timeout=config.timeout)
            
            # Run comprehensive Rust benchmark
            bench_cmd = [
                "cargo", "bench", "--bench", "comprehensive_rust_benchmark",
                "--", "--output-format", "json"
            ]
            
            result = subprocess.run(
                bench_cmd, 
                cwd=self.base_dir, 
                capture_output=True, 
                text=True, 
                timeout=config.timeout
            )
            
            if result.returncode == 0:
                # Parse Criterion output (simplified - actual parsing would be more complex)
                # For now, we'll extract basic info from stdout
                output_lines = result.stdout.split('\n')
                
                # Look for Shlesha results
                shlesha_result = ToolResult(
                    name="Shlesha",
                    platform="Rust",
                    available=True,
                    version=self._get_shlesha_version(),
                    performance_metrics=self._parse_rust_performance(output_lines, "Shlesha")
                )
                results.append(shlesha_result)
                
                # Look for Vidyut results
                vidyut_result = ToolResult(
                    name="Vidyut-lipi",
                    platform="Rust", 
                    available=True,
                    version=self._get_vidyut_version(),
                    performance_metrics=self._parse_rust_performance(output_lines, "Vidyut")
                )
                results.append(vidyut_result)
                
            else:
                print(f"Rust benchmark failed: {result.stderr}")
                
        except subprocess.TimeoutExpired:
            print("Rust benchmark timed out")
        except Exception as e:
            print(f"Rust benchmark error: {e}")
            
        return results
    
    def _get_shlesha_version(self) -> str:
        """Extract Shlesha version from Cargo.toml"""
        try:
            cargo_toml = self.base_dir / "Cargo.toml"
            with open(cargo_toml) as f:
                for line in f:
                    if line.startswith('version'):
                        return line.split('=')[1].strip().strip('"')
        except:
            pass
        return "unknown"
    
    def _get_vidyut_version(self) -> str:
        """Extract Vidyut version from dependencies"""
        try:
            cargo_toml = self.base_dir / "Cargo.toml"
            with open(cargo_toml) as f:
                content = f.read()
                # Look for vidyut-lipi version
                for line in content.split('\n'):
                    if 'vidyut-lipi' in line and '=' in line:
                        return line.split('=')[1].strip().strip('"')
        except:
            pass
        return "unknown"
    
    def _parse_rust_performance(self, output_lines: List[str], tool_name: str) -> Dict[str, Any]:
        """Parse performance metrics from Criterion output"""
        # Simplified parsing - real implementation would parse JSON output
        metrics = {
            "mean_time_ms": 0.0,
            "throughput_chars_per_sec": 0.0,
            "memory_efficiency": "unknown"
        }
        
        # Look for lines containing the tool name and performance data
        for line in output_lines:
            if tool_name.lower() in line.lower():
                # Extract timing information (this is simplified)
                if "time:" in line:
                    try:
                        # Parse timing data
                        parts = line.split()
                        for i, part in enumerate(parts):
                            if part.endswith("ms"):
                                metrics["mean_time_ms"] = float(part[:-2])
                            elif "chars/sec" in line:
                                # Extract throughput
                                pass
                    except:
                        pass
        
        return metrics

class PythonBenchmarkRunner:
    """Run Python benchmarks"""
    
    def __init__(self, base_dir: Path):
        self.base_dir = base_dir
        
    def run(self, config: BenchmarkConfig) -> List[ToolResult]:
        """Run Python benchmarks"""
        print("🐍 Running Python vs Python benchmarks...")
        
        results = []
        python_script = self.base_dir / "benches" / "improved_python_benchmark.py"
        
        if not python_script.exists():
            print("Python benchmark script not found")
            return results
        
        try:
            # Run Python benchmark
            cmd = [sys.executable, str(python_script)]
            result = subprocess.run(
                cmd,
                cwd=self.base_dir,
                capture_output=True,
                text=True,
                timeout=config.timeout
            )
            
            if result.returncode == 0:
                # Try to load results from generated JSON
                results_file = self.base_dir / "bench_data" / "python_benchmark_results.json"
                if results_file.exists():
                    with open(results_file) as f:
                        data = json.load(f)
                        results = self._parse_python_results(data)
                else:
                    # Parse from stdout
                    results = self._parse_python_stdout(result.stdout)
            else:
                print(f"Python benchmark failed: {result.stderr}")
                
        except subprocess.TimeoutExpired:
            print("Python benchmark timed out")
        except Exception as e:
            print(f"Python benchmark error: {e}")
            
        return results
    
    def _parse_python_results(self, data: Dict[str, Any]) -> List[ToolResult]:
        """Parse results from Python benchmark JSON output"""
        results = []
        
        # Group results by tool
        tools = {}
        for result in data.get('results', []):
            tool = result['tool']
            if tool not in tools:
                tools[tool] = []
            tools[tool].append(result)
        
        # Create ToolResult for each tool
        for tool_name, tool_results in tools.items():
            if tool_results:
                avg_time = statistics.mean(r['mean_time_ms'] for r in tool_results)
                avg_throughput = statistics.mean(r['chars_per_sec'] for r in tool_results)
                accuracy = tool_results[0].get('accuracy')
                
                result = ToolResult(
                    name=tool_name,
                    platform="Python",
                    available=True,
                    performance_metrics={
                        "mean_time_ms": avg_time,
                        "throughput_chars_per_sec": avg_throughput
                    },
                    accuracy_score=accuracy
                )
                results.append(result)
        
        return results
    
    def _parse_python_stdout(self, stdout: str) -> List[ToolResult]:
        """Parse results from Python benchmark stdout"""
        # Simplified parsing from stdout
        results = []
        
        # Look for tool availability lines
        for line in stdout.split('\n'):
            if '✓' in line and 'transliteration' in line.lower():
                tool_name = line.split('✓')[1].strip().split()[0]
                result = ToolResult(
                    name=tool_name,
                    platform="Python",
                    available=True
                )
                results.append(result)
        
        return results

class CLIBenchmarkRunner:
    """Run CLI benchmarks"""
    
    def __init__(self, base_dir: Path):
        self.base_dir = base_dir
        
    def run(self, config: BenchmarkConfig) -> List[ToolResult]:
        """Run CLI benchmarks"""
        print("⚡ Running CLI vs CLI benchmarks...")
        
        results = []
        cli_script = self.base_dir / "benches" / "improved_cli_benchmark.sh"
        
        if not cli_script.exists():
            print("CLI benchmark script not found")
            return results
        
        try:
            # Make script executable
            os.chmod(cli_script, 0o755)
            
            # Run CLI benchmark
            result = subprocess.run(
                [str(cli_script)],
                cwd=self.base_dir,
                capture_output=True,
                text=True,
                timeout=config.timeout
            )
            
            if result.returncode == 0:
                # Parse results from CSV files
                results = self._parse_cli_results()
            else:
                print(f"CLI benchmark failed: {result.stderr}")
                
        except subprocess.TimeoutExpired:
            print("CLI benchmark timed out")
        except Exception as e:
            print(f"CLI benchmark error: {e}")
            
        return results
    
    def _parse_cli_results(self) -> List[ToolResult]:
        """Parse CLI benchmark results from CSV files"""
        results = []
        
        # Parse detailed results
        results_file = self.base_dir / "bench_data" / "detailed_results.csv"
        if results_file.exists():
            try:
                import csv
                tools_data = {}
                
                with open(results_file) as f:
                    reader = csv.reader(f)
                    next(reader)  # Skip header
                    
                    for row in reader:
                        if len(row) >= 8:
                            tool, file_name, avg_time, min_time, max_time, median_time, throughput_mb, throughput_chars = row[:8]
                            
                            if tool not in tools_data:
                                tools_data[tool] = []
                            
                            tools_data[tool].append({
                                'avg_time': float(avg_time),
                                'throughput_chars': float(throughput_chars)
                            })
                
                # Create ToolResult for each tool
                for tool_name, data in tools_data.items():
                    if data:
                        avg_time = statistics.mean(d['avg_time'] for d in data)
                        avg_throughput = statistics.mean(d['throughput_chars'] for d in data)
                        
                        result = ToolResult(
                            name=tool_name,
                            platform="CLI",
                            available=True,
                            performance_metrics={
                                "mean_time_ms": avg_time * 1000,  # Convert to ms
                                "throughput_chars_per_sec": avg_throughput
                            }
                        )
                        results.append(result)
                        
            except Exception as e:
                print(f"Error parsing CLI results: {e}")
        
        return results

class WASMBenchmarkRunner:
    """WASM benchmark runner (mostly provides instructions)"""
    
    def __init__(self, base_dir: Path):
        self.base_dir = base_dir
        
    def run(self, config: BenchmarkConfig) -> List[ToolResult]:
        """Provide instructions for WASM benchmarks"""
        print("🌐 WASM benchmarks require manual execution:")
        print(f"1. Open {self.base_dir}/benches/improved_wasm_benchmark.html in a browser")
        print("2. Load available WASM modules")
        print("3. Run performance and accuracy tests")
        print("4. Download results JSON")
        print("5. Place the JSON file in bench_data/wasm_results.json")
        
        # Try to load existing WASM results
        wasm_results_file = self.base_dir / "bench_data" / "wasm_results.json"
        if wasm_results_file.exists():
            try:
                with open(wasm_results_file) as f:
                    data = json.load(f)
                    return self._parse_wasm_results(data)
            except Exception as e:
                print(f"Error loading WASM results: {e}")
        
        return []
    
    def _parse_wasm_results(self, data: Dict[str, Any]) -> List[ToolResult]:
        """Parse WASM benchmark results"""
        results = []
        
        for tool_data in data.get('tools', []):
            if tool_data.get('status') == 'loaded':
                result = ToolResult(
                    name=tool_data['name'],
                    platform="WASM",
                    available=True,
                    performance_metrics=tool_data.get('performance_metrics', {})
                )
                results.append(result)
        
        return results

class UnifiedBenchmarkRunner:
    """Main benchmark orchestrator"""
    
    def __init__(self, base_dir: Path):
        self.base_dir = base_dir
        self.rust_runner = RustBenchmarkRunner(base_dir)
        self.python_runner = PythonBenchmarkRunner(base_dir)
        self.cli_runner = CLIBenchmarkRunner(base_dir)
        self.wasm_runner = WASMBenchmarkRunner(base_dir)
        
    def run_all(self, config: BenchmarkConfig) -> UnifiedResults:
        """Run all benchmarks and aggregate results"""
        print("🚀 Starting Unified Benchmark Suite")
        print("=" * 60)
        
        all_tools = []
        
        # Create output directory
        output_dir = Path(config.output_dir)
        output_dir.mkdir(exist_ok=True)
        
        # Run platform-specific benchmarks
        if config.run_rust:
            rust_tools = self.rust_runner.run(config)
            all_tools.extend(rust_tools)
            
        if config.run_python:
            python_tools = self.python_runner.run(config)
            all_tools.extend(python_tools)
            
        if config.run_cli:
            cli_tools = self.cli_runner.run(config)
            all_tools.extend(cli_tools)
            
        if config.run_wasm:
            wasm_tools = self.wasm_runner.run(config)
            all_tools.extend(wasm_tools)
        
        # Collect system info
        system_info = SystemInfo.collect()
        
        # Generate summary and recommendations
        summary = self._generate_summary(all_tools)
        recommendations = self._generate_recommendations(all_tools)
        
        # Create unified results
        results = UnifiedResults(
            timestamp=time.strftime('%Y-%m-%d %H:%M:%S'),
            config=config,
            system_info=system_info,
            tools=all_tools,
            summary=summary,
            recommendations=recommendations
        )
        
        # Save results
        self._save_results(results, output_dir)
        
        return results
    
    def _generate_summary(self, tools: List[ToolResult]) -> Dict[str, Any]:
        """Generate summary statistics"""
        summary = {
            "total_tools_tested": len(tools),
            "platforms": list(set(tool.platform for tool in tools)),
            "available_tools": len([t for t in tools if t.available]),
            "performance_leaders": {},
            "accuracy_leaders": {},
            "platform_comparison": {}
        }
        
        # Find performance leaders
        available_tools = [t for t in tools if t.available and t.performance_metrics]
        if available_tools:
            # Fastest tool overall
            fastest = min(available_tools, 
                         key=lambda t: t.performance_metrics.get('mean_time_ms', float('inf')))
            summary["performance_leaders"]["fastest_overall"] = {
                "name": fastest.name,
                "platform": fastest.platform,
                "time_ms": fastest.performance_metrics.get('mean_time_ms')
            }
            
            # Highest throughput
            highest_throughput = max(available_tools,
                                   key=lambda t: t.performance_metrics.get('throughput_chars_per_sec', 0))
            summary["performance_leaders"]["highest_throughput"] = {
                "name": highest_throughput.name,
                "platform": highest_throughput.platform,
                "throughput": highest_throughput.performance_metrics.get('throughput_chars_per_sec')
            }
        
        # Find accuracy leaders
        accurate_tools = [t for t in tools if t.available and t.accuracy_score is not None]
        if accurate_tools:
            most_accurate = max(accurate_tools, key=lambda t: t.accuracy_score)
            summary["accuracy_leaders"]["most_accurate"] = {
                "name": most_accurate.name,
                "platform": most_accurate.platform,
                "accuracy": most_accurate.accuracy_score
            }
        
        # Platform comparison
        platforms = {}
        for tool in available_tools:
            platform = tool.platform
            if platform not in platforms:
                platforms[platform] = []
            platforms[platform].append(tool)
        
        for platform, platform_tools in platforms.items():
            if platform_tools:
                avg_time = statistics.mean(
                    t.performance_metrics.get('mean_time_ms', 0) 
                    for t in platform_tools 
                    if t.performance_metrics
                ) if any(t.performance_metrics for t in platform_tools) else 0
                
                summary["platform_comparison"][platform] = {
                    "tool_count": len(platform_tools),
                    "avg_performance_ms": avg_time
                }
        
        return summary
    
    def _generate_recommendations(self, tools: List[ToolResult]) -> List[str]:
        """Generate recommendations based on results"""
        recommendations = []
        
        available_tools = [t for t in tools if t.available]
        
        if not available_tools:
            recommendations.append("❌ No transliteration tools were successfully benchmarked")
            return recommendations
        
        # Performance recommendations
        rust_tools = [t for t in available_tools if t.platform == "Rust" and t.performance_metrics]
        if rust_tools:
            fastest_rust = min(rust_tools, key=lambda t: t.performance_metrics.get('mean_time_ms', float('inf')))
            recommendations.append(f"🦀 For maximum performance: Use {fastest_rust.name} (Rust native)")
        
        # Python recommendations
        python_tools = [t for t in available_tools if t.platform == "Python"]
        if python_tools:
            recommendations.append(f"🐍 For Python integration: {len(python_tools)} Python libraries available")
        
        # CLI recommendations
        cli_tools = [t for t in available_tools if t.platform == "CLI"]
        if cli_tools:
            recommendations.append(f"⚡ For command-line usage: {len(cli_tools)} CLI tools available")
        
        # Accuracy recommendations
        accurate_tools = [t for t in available_tools if t.accuracy_score is not None]
        if accurate_tools:
            most_accurate = max(accurate_tools, key=lambda t: t.accuracy_score)
            if most_accurate.accuracy_score >= 90:
                recommendations.append(f"🎯 For highest accuracy: {most_accurate.name} ({most_accurate.accuracy_score:.1f}%)")
        
        # General recommendations
        if len(available_tools) > 1:
            recommendations.append("✅ Multiple options available - choose based on your specific needs")
        
        # Shlesha-specific recommendations
        shlesha_tools = [t for t in available_tools if "Shlesha" in t.name]
        if shlesha_tools:
            recommendations.append("🌟 Shlesha is available and ready for production use")
        else:
            recommendations.append("⚠️  Shlesha not detected - ensure proper installation")
        
        return recommendations
    
    def _save_results(self, results: UnifiedResults, output_dir: Path):
        """Save unified results to various formats"""
        timestamp = time.strftime('%Y%m%d_%H%M%S')
        
        # Save as JSON
        json_file = output_dir / f"unified_results_{timestamp}.json"
        with open(json_file, 'w') as f:
            json.dump(asdict(results), f, indent=2, default=str)
        
        # Save as human-readable report
        report_file = output_dir / f"benchmark_report_{timestamp}.txt"
        with open(report_file, 'w') as f:
            self._write_text_report(results, f)
        
        # Save as CSV summary
        csv_file = output_dir / f"tool_comparison_{timestamp}.csv"
        self._write_csv_summary(results, csv_file)
        
        print(f"\n📁 Results saved to {output_dir}/")
        print(f"   - JSON: {json_file.name}")
        print(f"   - Report: {report_file.name}")
        print(f"   - CSV: {csv_file.name}")
    
    def _write_text_report(self, results: UnifiedResults, file_handle):
        """Write human-readable text report"""
        f = file_handle
        
        f.write("SHLESHA TRANSLITERATION BENCHMARK REPORT\n")
        f.write("=" * 60 + "\n\n")
        
        f.write(f"Generated: {results.timestamp}\n")
        f.write(f"System: {results.system_info.get('platform', 'unknown')}\n")
        f.write(f"Python: {results.system_info.get('python_version', 'unknown')}\n")
        f.write(f"Rust: {results.system_info.get('rust_version', 'unknown')}\n\n")
        
        # Summary
        f.write("SUMMARY\n")
        f.write("-" * 30 + "\n")
        f.write(f"Total tools tested: {results.summary['total_tools_tested']}\n")
        f.write(f"Available tools: {results.summary['available_tools']}\n")
        f.write(f"Platforms tested: {', '.join(results.summary['platforms'])}\n\n")
        
        # Performance leaders
        if results.summary.get('performance_leaders'):
            f.write("PERFORMANCE LEADERS\n")
            f.write("-" * 30 + "\n")
            
            fastest = results.summary['performance_leaders'].get('fastest_overall')
            if fastest:
                f.write(f"Fastest: {fastest['name']} ({fastest['platform']}) - {fastest['time_ms']:.2f}ms\n")
            
            throughput = results.summary['performance_leaders'].get('highest_throughput')
            if throughput:
                f.write(f"Highest throughput: {throughput['name']} ({throughput['platform']}) - {throughput['throughput']:.0f} chars/sec\n")
            f.write("\n")
        
        # Accuracy leaders
        if results.summary.get('accuracy_leaders'):
            f.write("ACCURACY LEADERS\n")
            f.write("-" * 30 + "\n")
            
            accurate = results.summary['accuracy_leaders'].get('most_accurate')
            if accurate:
                f.write(f"Most accurate: {accurate['name']} ({accurate['platform']}) - {accurate['accuracy']:.1f}%\n")
            f.write("\n")
        
        # Tool details
        f.write("DETAILED RESULTS\n")
        f.write("-" * 30 + "\n")
        
        for tool in results.tools:
            f.write(f"\n{tool.name} ({tool.platform})\n")
            f.write(f"  Available: {'Yes' if tool.available else 'No'}\n")
            
            if tool.version:
                f.write(f"  Version: {tool.version}\n")
            
            if tool.performance_metrics:
                f.write(f"  Performance:\n")
                f.write(f"    Mean time: {tool.performance_metrics.get('mean_time_ms', 'N/A')} ms\n")
                f.write(f"    Throughput: {tool.performance_metrics.get('throughput_chars_per_sec', 'N/A')} chars/sec\n")
            
            if tool.accuracy_score is not None:
                f.write(f"  Accuracy: {tool.accuracy_score:.1f}%\n")
            
            if tool.error_message:
                f.write(f"  Error: {tool.error_message}\n")
        
        # Recommendations
        f.write("\nRECOMMENDATIONS\n")
        f.write("-" * 30 + "\n")
        for rec in results.recommendations:
            f.write(f"{rec}\n")
    
    def _write_csv_summary(self, results: UnifiedResults, csv_file: Path):
        """Write CSV summary for easy analysis"""
        import csv
        
        with open(csv_file, 'w', newline='') as f:
            writer = csv.writer(f)
            
            # Header
            writer.writerow([
                'Tool', 'Platform', 'Available', 'Version',
                'Mean_Time_ms', 'Throughput_chars_per_sec', 'Accuracy_%'
            ])
            
            # Data rows
            for tool in results.tools:
                writer.writerow([
                    tool.name,
                    tool.platform,
                    'Yes' if tool.available else 'No',
                    tool.version or 'Unknown',
                    tool.performance_metrics.get('mean_time_ms', '') if tool.performance_metrics else '',
                    tool.performance_metrics.get('throughput_chars_per_sec', '') if tool.performance_metrics else '',
                    tool.accuracy_score if tool.accuracy_score is not None else ''
                ])

def main():
    parser = argparse.ArgumentParser(description='Unified Transliteration Benchmark Runner')
    parser.add_argument('--skip-rust', action='store_true', help='Skip Rust benchmarks')
    parser.add_argument('--skip-python', action='store_true', help='Skip Python benchmarks')
    parser.add_argument('--skip-cli', action='store_true', help='Skip CLI benchmarks')
    parser.add_argument('--include-wasm', action='store_true', help='Include WASM benchmarks (manual)')
    parser.add_argument('--iterations', type=int, default=50, help='Number of iterations per test')
    parser.add_argument('--timeout', type=int, default=300, help='Timeout per benchmark in seconds')
    parser.add_argument('--output-dir', default='unified_benchmark_results', help='Output directory')
    
    args = parser.parse_args()
    
    # Create configuration
    config = BenchmarkConfig(
        run_rust=not args.skip_rust,
        run_python=not args.skip_python,
        run_cli=not args.skip_cli,
        run_wasm=args.include_wasm,
        iterations=args.iterations,
        timeout=args.timeout,
        output_dir=args.output_dir
    )
    
    # Find project root (directory containing Cargo.toml)
    current_dir = Path.cwd()
    project_root = current_dir
    
    # Look for Cargo.toml in current dir and parents
    for path in [current_dir] + list(current_dir.parents):
        if (path / "Cargo.toml").exists():
            project_root = path
            break
    
    print(f"Project root: {project_root}")
    
    # Create and run benchmark
    runner = UnifiedBenchmarkRunner(project_root)
    results = runner.run_all(config)
    
    # Print summary
    print("\n" + "=" * 60)
    print("BENCHMARK COMPLETE")
    print("=" * 60)
    
    print(f"\nTested {results.summary['total_tools_tested']} tools across {len(results.summary['platforms'])} platforms")
    print(f"Available tools: {results.summary['available_tools']}")
    
    print("\nRecommendations:")
    for rec in results.recommendations:
        print(f"  {rec}")
    
    print(f"\nDetailed results saved to: {config.output_dir}/")

if __name__ == "__main__":
    main()