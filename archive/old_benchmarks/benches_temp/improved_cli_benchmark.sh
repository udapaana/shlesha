#!/bin/bash

# Improved CLI Transliteration Benchmark
# Detects and benchmarks actual CLI tools for fair CLI vs CLI comparison

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Improved CLI Transliteration Benchmark ===${NC}"
echo "Detects and compares available CLI transliteration tools"
echo

# Create benchmark data directory
mkdir -p bench_data

# Create test files with varying sizes
create_test_files() {
    echo -e "${GREEN}Creating test files...${NC}"
    
    # Small test (single word)
    cat > bench_data/small.txt << 'EOF'
नमस्ते
EOF
    
    # Medium test (sentence)
    cat > bench_data/medium.txt << 'EOF'
धृतराष्ट्र उवाच धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः
EOF
    
    # Large test (paragraph)
    cat > bench_data/large.txt << 'EOF'
कर्मण्येवाधिकारस्ते मा फलेषु कदाचन ।
मा कर्मफलहेतुर्भूर्मा ते सङ्गोऽस्त्वकर्मणि ॥
योगस्थः कुरु कर्माणि सङ्गं त्यक्त्वा धनञ्जय ।
सिद्ध्यसिद्ध्योः समो भूत्वा समत्वं योग उच्यते ॥
दूरेण ह्यवरं कर्म बुद्धियोगाद्धनञ्जय ।
बुद्धौ शरणमन्विच्छ कृपणाः फलहेतवः ॥
EOF
    
    # Very large test (repeated text)
    for i in {1..500}; do
        echo "तत्र शूरा महेष्वासा भीमार्जुनसमा युधि । युयुधानो विराटश्च द्रुपदश्च महारथः ॥" >> bench_data/very_large.txt
    done
    
    # Extremely large test (for stress testing)
    for i in {1..2000}; do
        echo "कर्मण्येवाधिकारस्ते मा फलेषु कदाचन । मा कर्मफलहेतुर्भूर्मा ते सङ्गोऽस्त्वकर्मणि ॥" >> bench_data/extreme.txt
    done
    
    echo "Test files created:"
    for file in bench_data/*.txt; do
        if [[ -f "$file" ]]; then
            size=$(wc -c < "$file")
            lines=$(wc -l < "$file")
            echo "  $(basename "$file"): ${size} bytes, ${lines} lines"
        fi
    done
}

# Check if a command exists and is working
check_command() {
    local cmd="$1"
    local test_args="$2"
    
    if command -v "$cmd" &> /dev/null; then
        if eval "$cmd $test_args" &> /dev/null; then
            return 0
        else
            return 1
        fi
    else
        return 1
    fi
}

# Detect available CLI tools
detect_cli_tools() {
    echo -e "${GREEN}Detecting available CLI tools...${NC}"
    
    declare -A CLI_TOOLS
    
    # Build Shlesha CLI first
    echo -e "${CYAN}Building Shlesha CLI...${NC}"
    if cargo build --release --example cli 2>/dev/null; then
        if [[ -f "./target/release/examples/cli" ]]; then
            CLI_TOOLS["shlesha"]="./target/release/examples/cli -f Devanagari -t IAST"
            echo "  ✓ Shlesha CLI built successfully"
        else
            echo "  ✗ Shlesha CLI binary not found"
        fi
    else
        echo "  ✗ Failed to build Shlesha CLI"
    fi
    
    # Check for Vidyut CLI
    if check_command "vidyut-cli" "--help"; then
        CLI_TOOLS["vidyut"]="vidyut-cli --from devanagari --to iast"
        echo "  ✓ vidyut-cli found"
    elif check_command "vidyut" "--help"; then
        CLI_TOOLS["vidyut"]="vidyut transliterate --from devanagari --to iast"
        echo "  ✓ vidyut found"
    else
        echo "  ✗ vidyut-cli not found"
    fi
    
    # Check for dharmamitra CLI
    if check_command "dharmamitra" "--help"; then
        CLI_TOOLS["dharmamitra"]="dharmamitra transliterate --from devanagari --to iast"
        echo "  ✓ dharmamitra CLI found"
    else
        echo "  ✗ dharmamitra CLI not found"
    fi
    
    # Check for Aksharamukha CLI
    if check_command "aksharamukha" "--help"; then
        CLI_TOOLS["aksharamukha"]="aksharamukha -s Devanagari -t IAST"
        echo "  ✓ aksharamukha CLI found"
    elif check_command "am-cli" "--help"; then
        CLI_TOOLS["aksharamukha"]="am-cli -s Devanagari -t IAST"
        echo "  ✓ am-cli found"
    else
        echo "  ✗ aksharamukha CLI not found"
    fi
    
    # Check for indic-transliteration CLI
    if check_command "indic-transliterate" "--help"; then
        CLI_TOOLS["indic_transliteration"]="indic-transliterate --from devanagari --to iast"
        echo "  ✓ indic-transliteration CLI found"
    else
        echo "  ✗ indic-transliteration CLI not found"
    fi
    
    # Check for Python-based CLI wrappers
    if python3 -c "import sys; from indic_transliteration import sanscript; print('OK')" 2>/dev/null | grep -q "OK"; then
        # Create a temporary Python CLI wrapper
        cat > bench_data/indic_transliteration_cli.py << 'EOF'
#!/usr/bin/env python3
import sys
from indic_transliteration import sanscript

if __name__ == "__main__":
    text = sys.stdin.read().strip()
    result = sanscript.transliterate(text, sanscript.DEVANAGARI, sanscript.IAST)
    print(result)
EOF
        chmod +x bench_data/indic_transliteration_cli.py
        CLI_TOOLS["indic_trans_py"]="python3 bench_data/indic_transliteration_cli.py"
        echo "  ✓ indic-transliteration (Python wrapper) created"
    fi
    
    # Check for AI4Bharat CLI
    if python3 -c "from ai4bharat_transliteration import XlitEngine; print('OK')" 2>/dev/null | grep -q "OK"; then
        cat > bench_data/ai4bharat_cli.py << 'EOF'
#!/usr/bin/env python3
import sys
from ai4bharat_transliteration import XlitEngine

engine = XlitEngine("hi", beam_width=10, rescore=False)

if __name__ == "__main__":
    text = sys.stdin.read().strip()
    # AI4Bharat is mainly for romanization, this is a simplified wrapper
    result = engine.translit_word(text, topk=1)[0]
    print(result)
EOF
        chmod +x bench_data/ai4bharat_cli.py
        CLI_TOOLS["ai4bharat"]="python3 bench_data/ai4bharat_cli.py"
        echo "  ✓ AI4Bharat (Python wrapper) created"
    fi
    
    echo
    echo "Available CLI tools: ${#CLI_TOOLS[@]}"
    for tool in "${!CLI_TOOLS[@]}"; do
        echo "  - $tool: ${CLI_TOOLS[$tool]}"
    done
    
    # Export CLI_TOOLS for use in other functions
    for tool in "${!CLI_TOOLS[@]}"; do
        declare -g "CLI_${tool}=${CLI_TOOLS[$tool]}"
    done
    
    # Store available tools in a file for later processing
    for tool in "${!CLI_TOOLS[@]}"; do
        echo "${tool}:${CLI_TOOLS[$tool]}" >> bench_data/available_tools.txt
    done
}

# Benchmark a single tool
benchmark_tool() {
    local tool_name="$1"
    local tool_cmd="$2"
    local input_file="$3"
    local iterations="${4:-10}"
    
    echo -e "${CYAN}Benchmarking $tool_name with $(basename $input_file)...${NC}"
    
    # Verify tool works
    if ! eval "echo 'क' | $tool_cmd > /dev/null 2>&1"; then
        echo "  ✗ Tool validation failed"
        return 1
    fi
    
    local times=()
    local successful_runs=0
    local total_time=0
    
    # Warmup run
    eval "$tool_cmd < $input_file > /dev/null 2>&1" || true
    
    # Benchmark runs
    for i in $(seq 1 $iterations); do
        local start_time=$(date +%s.%N)
        
        if eval "$tool_cmd < $input_file > /dev/null 2>&1"; then
            local end_time=$(date +%s.%N)
            local elapsed=$(echo "$end_time - $start_time" | bc -l)
            times+=("$elapsed")
            total_time=$(echo "$total_time + $elapsed" | bc -l)
            ((successful_runs++))
        fi
    done
    
    if [[ $successful_runs -gt 0 ]]; then
        local avg_time=$(echo "scale=6; $total_time / $successful_runs" | bc -l)
        local file_size=$(wc -c < "$input_file")
        local char_count=$(wc -m < "$input_file" 2>/dev/null || echo "$file_size")
        
        # Calculate throughput
        local throughput_mb=$(echo "scale=3; $file_size / $avg_time / 1048576" | bc -l)
        local throughput_chars=$(echo "scale=0; $char_count / $avg_time" | bc -l)
        
        # Calculate statistics
        local min_time=$(printf '%s\n' "${times[@]}" | sort -n | head -1)
        local max_time=$(printf '%s\n' "${times[@]}" | sort -n | tail -1)
        
        # Calculate median (simple approach)
        local sorted_times=($(printf '%s\n' "${times[@]}" | sort -n))
        local mid_index=$((${#sorted_times[@]} / 2))
        local median_time=${sorted_times[$mid_index]}
        
        echo "  ✓ Average time: ${avg_time}s"
        echo "    Min: ${min_time}s, Max: ${max_time}s, Median: ${median_time}s"
        echo "    Throughput: ${throughput_mb} MB/s, ${throughput_chars} chars/s"
        echo "    Success rate: ${successful_runs}/${iterations}"
        
        # Save detailed results
        echo "$tool_name,$(basename $input_file),$avg_time,$min_time,$max_time,$median_time,$throughput_mb,$throughput_chars,$successful_runs,$iterations,$file_size,$char_count" >> bench_data/detailed_results.csv
        
        return 0
    else
        echo "  ✗ All runs failed"
        return 1
    fi
}

# Run accuracy test
test_accuracy() {
    local tool_name="$1"
    local tool_cmd="$2"
    
    echo -e "${CYAN}Testing accuracy for $tool_name...${NC}"
    
    # Test cases with expected outputs
    declare -A test_cases=(
        ["नमस्ते"]="namaste"
        ["संस्कृतम्"]="saṃskṛtam"
        ["कृष्ण"]="kṛṣṇa"
        ["ज्ञान"]="jñāna"
        ["अग्निमीळे"]="agnimīḷe"
    )
    
    local correct=0
    local total=0
    
    for input in "${!test_cases[@]}"; do
        local expected="${test_cases[$input]}"
        local actual
        
        if actual=$(echo "$input" | eval "$tool_cmd" 2>/dev/null); then
            ((total++))
            # Remove trailing whitespace
            actual=$(echo "$actual" | tr -d '\n\r' | sed 's/[[:space:]]*$//')
            
            if [[ "$actual" == "$expected" ]]; then
                ((correct++))
                echo "  ✓ $input → $actual"
            else
                echo "  ✗ $input → $actual (expected: $expected)"
            fi
        else
            ((total++))
            echo "  ✗ $input → ERROR"
        fi
    done
    
    if [[ $total -gt 0 ]]; then
        local accuracy=$(echo "scale=1; $correct * 100 / $total" | bc -l)
        echo "  Accuracy: ${accuracy}% (${correct}/${total})"
        
        # Save accuracy result
        echo "$tool_name,$accuracy,$correct,$total" >> bench_data/accuracy_results.csv
    else
        echo "  No successful tests"
    fi
}

# Generate comprehensive report
generate_report() {
    echo -e "${GREEN}Generating comprehensive report...${NC}"
    
    cat > bench_data/generate_report.py << 'EOF'
#!/usr/bin/env python3
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import json
from pathlib import Path

def load_data():
    try:
        # Load detailed results
        df = pd.read_csv('bench_data/detailed_results.csv', 
                        names=['tool', 'file', 'avg_time', 'min_time', 'max_time', 
                              'median_time', 'throughput_mb', 'throughput_chars', 
                              'successful_runs', 'total_runs', 'file_size', 'char_count'])
        
        # Load accuracy results
        acc_df = pd.read_csv('bench_data/accuracy_results.csv',
                           names=['tool', 'accuracy', 'correct', 'total'])
        
        return df, acc_df
    except Exception as e:
        print(f"Error loading data: {e}")
        return None, None

def create_visualizations(df, acc_df):
    if df is None:
        return
    
    # Set up the plotting style
    plt.style.use('seaborn-v0_8')
    fig, axes = plt.subplots(2, 2, figsize=(16, 12))
    fig.suptitle('CLI Transliteration Tools Benchmark Results', fontsize=16, fontweight='bold')
    
    # 1. Execution Time Comparison
    ax1 = axes[0, 0]
    pivot_time = df.pivot(index='file', columns='tool', values='avg_time')
    pivot_time.plot(kind='bar', ax=ax1, width=0.8)
    ax1.set_title('Average Execution Time Comparison')
    ax1.set_ylabel('Time (seconds)')
    ax1.set_xlabel('Test File')
    ax1.legend(title='Tool', bbox_to_anchor=(1.05, 1), loc='upper left')
    ax1.tick_params(axis='x', rotation=45)
    
    # 2. Throughput Comparison
    ax2 = axes[0, 1]
    pivot_throughput = df.pivot(index='file', columns='tool', values='throughput_chars')
    pivot_throughput.plot(kind='bar', ax=ax2, width=0.8)
    ax2.set_title('Throughput Comparison')
    ax2.set_ylabel('Characters/second')
    ax2.set_xlabel('Test File')
    ax2.legend(title='Tool', bbox_to_anchor=(1.05, 1), loc='upper left')
    ax2.tick_params(axis='x', rotation=45)
    
    # 3. Success Rate
    ax3 = axes[1, 0]
    df['success_rate'] = df['successful_runs'] / df['total_runs'] * 100
    pivot_success = df.pivot(index='file', columns='tool', values='success_rate')
    pivot_success.plot(kind='bar', ax=ax3, width=0.8)
    ax3.set_title('Success Rate Comparison')
    ax3.set_ylabel('Success Rate (%)')
    ax3.set_xlabel('Test File')
    ax3.legend(title='Tool', bbox_to_anchor=(1.05, 1), loc='upper left')
    ax3.tick_params(axis='x', rotation=45)
    
    # 4. Accuracy Comparison
    ax4 = axes[1, 1]
    if acc_df is not None and not acc_df.empty:
        acc_df.plot(x='tool', y='accuracy', kind='bar', ax=ax4, width=0.6)
        ax4.set_title('Accuracy Comparison')
        ax4.set_ylabel('Accuracy (%)')
        ax4.set_xlabel('Tool')
        ax4.tick_params(axis='x', rotation=45)
    else:
        ax4.text(0.5, 0.5, 'No accuracy data available', 
                ha='center', va='center', transform=ax4.transAxes)
        ax4.set_title('Accuracy Comparison - No Data')
    
    plt.tight_layout()
    plt.savefig('bench_data/cli_benchmark_report.png', dpi=300, bbox_inches='tight')
    print("Visualization saved as 'bench_data/cli_benchmark_report.png'")

def generate_summary(df, acc_df):
    if df is None:
        return
    
    print("\n" + "="*80)
    print("COMPREHENSIVE BENCHMARK SUMMARY")
    print("="*80)
    
    # Overall performance ranking
    tool_performance = df.groupby('tool').agg({
        'avg_time': 'mean',
        'throughput_chars': 'mean',
        'successful_runs': 'sum',
        'total_runs': 'sum'
    }).round(4)
    
    tool_performance['success_rate'] = (tool_performance['successful_runs'] / 
                                      tool_performance['total_runs'] * 100).round(1)
    
    # Add accuracy data
    if acc_df is not None and not acc_df.empty:
        tool_performance = tool_performance.merge(
            acc_df[['tool', 'accuracy']].set_index('tool'), 
            left_index=True, right_index=True, how='left'
        )
    
    # Sort by throughput (higher is better)
    tool_performance = tool_performance.sort_values('throughput_chars', ascending=False)
    
    print("\nOverall Performance Ranking:")
    print("-" * 80)
    print(f"{'Tool':<20} {'Avg Time':<12} {'Throughput':<15} {'Success%':<10} {'Accuracy%':<10}")
    print("-" * 80)
    
    for tool, row in tool_performance.iterrows():
        acc_str = f"{row.get('accuracy', 0):.1f}" if 'accuracy' in row and pd.notna(row['accuracy']) else "N/A"
        print(f"{tool:<20} {row['avg_time']:<12.4f} {row['throughput_chars']:<15.0f} "
              f"{row['success_rate']:<10.1f} {acc_str:<10}")
    
    # Best tool analysis
    fastest_tool = tool_performance.index[0]
    print(f"\n🏆 Fastest Tool: {fastest_tool}")
    print(f"   Average throughput: {tool_performance.loc[fastest_tool, 'throughput_chars']:.0f} chars/sec")
    
    if 'accuracy' in tool_performance.columns:
        most_accurate = tool_performance.dropna(subset=['accuracy']).sort_values('accuracy', ascending=False)
        if not most_accurate.empty:
            accurate_tool = most_accurate.index[0]
            print(f"🎯 Most Accurate Tool: {accurate_tool}")
            print(f"   Accuracy: {most_accurate.loc[accurate_tool, 'accuracy']:.1f}%")
    
    # Performance insights
    print(f"\n📊 Performance Insights:")
    speed_range = tool_performance['throughput_chars'].max() / tool_performance['throughput_chars'].min()
    print(f"   Speed difference: {speed_range:.1f}x between fastest and slowest")
    
    if 'accuracy' in tool_performance.columns:
        acc_data = tool_performance.dropna(subset=['accuracy'])['accuracy']
        if len(acc_data) > 1:
            acc_range = acc_data.max() - acc_data.min()
            print(f"   Accuracy range: {acc_range:.1f}% difference")

if __name__ == "__main__":
    df, acc_df = load_data()
    if df is not None:
        generate_summary(df, acc_df)
        try:
            create_visualizations(df, acc_df)
        except ImportError:
            print("Matplotlib not available, skipping visualizations")
        except Exception as e:
            print(f"Error creating visualizations: {e}")
    else:
        print("No data available for report generation")
EOF
    
    chmod +x bench_data/generate_report.py
}

# Main benchmark execution
run_benchmarks() {
    echo -e "${GREEN}Running comprehensive CLI benchmarks...${NC}"
    
    # Initialize result files
    echo "tool,file,avg_time,min_time,max_time,median_time,throughput_mb,throughput_chars,successful_runs,total_runs,file_size,char_count" > bench_data/detailed_results.csv
    echo "tool,accuracy,correct,total" > bench_data/accuracy_results.csv
    
    # Test files to benchmark against
    local test_files=(
        "bench_data/small.txt"
        "bench_data/medium.txt"
        "bench_data/large.txt"
        "bench_data/very_large.txt"
    )
    
    # Read available tools
    if [[ ! -f "bench_data/available_tools.txt" ]]; then
        echo "No available tools found!"
        return 1
    fi
    
    local tool_count=0
    
    # Benchmark each available tool
    while IFS=':' read -r tool_name tool_cmd; do
        if [[ -n "$tool_name" && -n "$tool_cmd" ]]; then
            echo -e "\n${BLUE}=== Benchmarking $tool_name ===${NC}"
            
            # Performance benchmarks
            for test_file in "${test_files[@]}"; do
                if [[ -f "$test_file" ]]; then
                    # Adjust iterations based on file size
                    local file_size=$(wc -c < "$test_file")
                    local iterations=20
                    
                    if [[ $file_size -gt 100000 ]]; then
                        iterations=5
                    elif [[ $file_size -gt 10000 ]]; then
                        iterations=10
                    fi
                    
                    benchmark_tool "$tool_name" "$tool_cmd" "$test_file" "$iterations"
                fi
            done
            
            # Accuracy test
            test_accuracy "$tool_name" "$tool_cmd"
            
            ((tool_count++))
        fi
    done < bench_data/available_tools.txt
    
    echo -e "\n${GREEN}Benchmarked $tool_count tools${NC}"
}

# Installation suggestions
suggest_installations() {
    echo -e "\n${YELLOW}=== Installation Suggestions ===${NC}"
    echo "To get more tools for comparison, you can install:"
    echo
    
    echo "1. Vidyut (Rust-based):"
    echo "   cargo install vidyut-cli"
    echo "   # or build from source: https://github.com/vidyut-org/vidyut"
    echo
    
    echo "2. Python-based tools:"
    echo "   pip install indic-transliteration"
    echo "   pip install aksharamukha"
    echo "   pip install dharmamitra"
    echo "   pip install ai4bharat-transliteration"
    echo
    
    echo "3. System packages (Ubuntu/Debian):"
    echo "   sudo apt-get install transliteration-tools"
    echo
    
    echo "4. Check for distro-specific packages:"
    echo "   # Arch: pacman -S transliteration-tools"
    echo "   # Fedora: dnf install transliteration-tools"
}

# Main execution
main() {
    echo "Starting improved CLI benchmark..."
    
    create_test_files
    detect_cli_tools
    
    if [[ ! -f "bench_data/available_tools.txt" ]] || [[ ! -s "bench_data/available_tools.txt" ]]; then
        echo -e "${RED}No CLI tools detected!${NC}"
        suggest_installations
        exit 1
    fi
    
    run_benchmarks
    generate_report
    
    echo -e "\n${GREEN}Running report generation...${NC}"
    if command -v python3 &> /dev/null; then
        python3 bench_data/generate_report.py
    else
        echo "Python3 not found, skipping automated report generation"
    fi
    
    echo -e "\n${GREEN}=== Benchmark Complete ===${NC}"
    echo "Results saved in bench_data/"
    echo "- detailed_results.csv: Raw benchmark data"
    echo "- accuracy_results.csv: Accuracy test results"
    echo "- cli_benchmark_report.png: Visual comparison (if generated)"
    
    suggest_installations
}

# Run main function
main "$@"