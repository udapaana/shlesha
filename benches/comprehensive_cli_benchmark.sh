#!/bin/bash

# Comprehensive CLI Benchmark Script
# Compares Shlesha with Vidyut, dharmamitra, and Aksharamukha CLIs

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Comprehensive CLI Transliteration Benchmark ===${NC}"
echo "Comparing: Shlesha vs Vidyut vs dharmamitra vs Aksharamukha"
echo

# Create test files
mkdir -p bench_data
cat > bench_data/small.txt << 'EOF'
नमस्ते संस्कृतम्
EOF

cat > bench_data/medium.txt << 'EOF'
धृतराष्ट्र उवाच
धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः
मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय
EOF

# Generate large file (1000 lines)
for i in {1..1000}; do
    echo "तत्र शूरा महेष्वासा भीमार्जुनसमा युधि । युयुधानो विराटश्च द्रुपदश्च महारथः ॥" >> bench_data/large.txt
done

# Generate very large file (10000 lines)
for i in {1..10000}; do
    echo "कर्मण्येवाधिकारस्ते मा फलेषु कदाचन । मा कर्मफलहेतुर्भूर्मा ते सङ्गोऽस्त्वकर्मणि ॥" >> bench_data/very_large.txt
done

# Build Shlesha CLI
echo -e "${GREEN}Building Shlesha CLI...${NC}"
cargo build --release --example cli 2>/dev/null || {
    echo -e "${RED}Failed to build Shlesha CLI${NC}"
    exit 1
}

# Function to benchmark a command
benchmark() {
    local name=$1
    local cmd=$2
    local input=$3
    
    echo -e "${BLUE}Benchmarking $name with $(basename $input)...${NC}"
    
    # Check if command exists
    if ! eval "$cmd --help > /dev/null 2>&1"; then
        echo -e "${YELLOW}$name not available${NC}"
        return
    fi
    
    # Warm up
    eval "$cmd < $input > /dev/null" 2>/dev/null || true
    
    # Run benchmark (10 iterations)
    local total_time=0
    local successful_runs=0
    
    for i in {1..10}; do
        local start=$(date +%s.%N)
        if eval "$cmd < $input > /dev/null" 2>/dev/null; then
            local end=$(date +%s.%N)
            local elapsed=$(echo "$end - $start" | bc)
            total_time=$(echo "$total_time + $elapsed" | bc)
            ((successful_runs++))
        fi
    done
    
    if [ $successful_runs -gt 0 ]; then
        local avg_time=$(echo "scale=6; $total_time / $successful_runs" | bc)
        echo "Average time: ${avg_time}s (${successful_runs}/10 successful)"
        
        # Calculate throughput
        local file_size=$(wc -c < $input)
        local throughput=$(echo "scale=2; $file_size / $avg_time / 1048576" | bc)
        echo "Throughput: ${throughput} MB/s"
        
        # Save to results file
        echo "$name,$(basename $input),$avg_time,$throughput,$successful_runs" >> bench_data/results.csv
    else
        echo -e "${RED}All runs failed${NC}"
    fi
    echo
}

# Initialize results file
echo "Tool,File,Time(s),Throughput(MB/s),SuccessfulRuns" > bench_data/results.csv

# Test files
files=(bench_data/small.txt bench_data/medium.txt bench_data/large.txt bench_data/very_large.txt)

# Benchmark Shlesha
echo -e "${GREEN}=== Shlesha CLI ===${NC}"
for file in "${files[@]}"; do
    benchmark "Shlesha" "./target/release/examples/cli -f Devanagari -t IAST" "$file"
done

# Benchmark Vidyut CLI
echo -e "${GREEN}=== Vidyut CLI ===${NC}"
# Check if vidyut-cli exists or can be built
if command -v vidyut-cli &> /dev/null; then
    for file in "${files[@]}"; do
        benchmark "Vidyut" "vidyut-cli --from devanagari --to iast" "$file"
    done
else
    echo -e "${YELLOW}vidyut-cli not found. Install from: https://github.com/vidyut-org/vidyut${NC}"
fi

# Benchmark dharmamitra CLI
echo -e "${GREEN}=== dharmamitra CLI ===${NC}"
if command -v dharmamitra &> /dev/null; then
    for file in "${files[@]}"; do
        benchmark "dharmamitra" "dharmamitra --from devanagari --to iast" "$file"
    done
else
    echo -e "${YELLOW}dharmamitra not found. Install with: pip install dharmamitra${NC}"
fi

# Benchmark Aksharamukha CLI
echo -e "${GREEN}=== Aksharamukha CLI ===${NC}"
if command -v aksharamukha &> /dev/null; then
    for file in "${files[@]}"; do
        benchmark "Aksharamukha" "aksharamukha -s Devanagari -t IAST" "$file"
    done
else
    echo -e "${YELLOW}aksharamukha CLI not found. Install with: pip install aksharamukha-cli${NC}"
fi

# Generate comparison report
cat > bench_data/generate_report.py << 'EOF'
#!/usr/bin/env python3
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

# Read results
df = pd.read_csv('bench_data/results.csv')

# Create comparison plots
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(14, 6))

# Time comparison
pivot_time = df.pivot(index='File', columns='Tool', values='Time(s)')
pivot_time.plot(kind='bar', ax=ax1)
ax1.set_title('Execution Time Comparison')
ax1.set_ylabel('Time (seconds)')
ax1.set_xlabel('Test File')
ax1.legend(title='Tool')

# Throughput comparison
pivot_throughput = df.pivot(index='File', columns='Tool', values='Throughput(MB/s)')
pivot_throughput.plot(kind='bar', ax=ax2)
ax2.set_title('Throughput Comparison')
ax2.set_ylabel('Throughput (MB/s)')
ax2.set_xlabel('Test File')
ax2.legend(title='Tool')

plt.tight_layout()
plt.savefig('bench_data/cli_comparison.png', dpi=300)

# Generate summary statistics
print("\n=== Performance Summary ===")
for tool in df['Tool'].unique():
    tool_data = df[df['Tool'] == tool]
    avg_time = tool_data['Time(s)'].mean()
    avg_throughput = tool_data['Throughput(MB/s)'].mean()
    print(f"\n{tool}:")
    print(f"  Average time: {avg_time:.4f}s")
    print(f"  Average throughput: {avg_throughput:.2f} MB/s")

# Calculate relative performance
baseline_tool = 'Shlesha'
if baseline_tool in df['Tool'].values:
    print(f"\n=== Relative Performance (vs {baseline_tool}) ===")
    baseline_data = df[df['Tool'] == baseline_tool].set_index('File')
    
    for tool in df['Tool'].unique():
        if tool != baseline_tool:
            tool_data = df[df['Tool'] == tool].set_index('File')
            
            # Calculate speedup for each file
            speedups = []
            for file in tool_data.index:
                if file in baseline_data.index:
                    baseline_time = baseline_data.loc[file, 'Time(s)']
                    tool_time = tool_data.loc[file, 'Time(s)']
                    speedup = tool_time / baseline_time
                    speedups.append(speedup)
            
            if speedups:
                avg_speedup = sum(speedups) / len(speedups)
                print(f"{tool}: {1/avg_speedup:.2f}x {'faster' if avg_speedup > 1 else 'slower'}")
EOF

chmod +x bench_data/generate_report.py

echo -e "${GREEN}Generating report...${NC}"
if command -v python3 &> /dev/null && python3 -c "import pandas, matplotlib" 2>/dev/null; then
    python3 bench_data/generate_report.py
else
    echo -e "${YELLOW}Python with pandas and matplotlib not available for report generation${NC}"
fi

echo -e "${GREEN}Benchmark complete!${NC}"
echo "Results saved in bench_data/results.csv"
echo "Report (if generated) saved in bench_data/cli_comparison.png"