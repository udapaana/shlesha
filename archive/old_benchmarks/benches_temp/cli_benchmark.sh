#!/bin/bash

# CLI Benchmark Script for comparing Shlesha with dharmamitra
# This ensures fair comparison: CLI vs CLI

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== CLI Transliteration Benchmark ===${NC}"
echo "Comparing Shlesha CLI with dharmamitra CLI"
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
    
    # Warm up
    eval "$cmd < $input > /dev/null" 2>/dev/null
    
    # Run benchmark (10 iterations)
    local total_time=0
    for i in {1..10}; do
        local start=$(date +%s.%N)
        eval "$cmd < $input > /dev/null" 2>/dev/null
        local end=$(date +%s.%N)
        local elapsed=$(echo "$end - $start" | bc)
        total_time=$(echo "$total_time + $elapsed" | bc)
    done
    
    local avg_time=$(echo "scale=6; $total_time / 10" | bc)
    echo "Average time: ${avg_time}s"
    
    # Calculate throughput
    local file_size=$(wc -c < $input)
    local throughput=$(echo "scale=2; $file_size / $avg_time / 1048576" | bc)
    echo "Throughput: ${throughput} MB/s"
    echo
}

# Check if dharmamitra is installed
if command -v dharmamitra &> /dev/null; then
    echo -e "${GREEN}dharmamitra found${NC}"
    COMPARE_DHARMAMITRA=true
else
    echo -e "${RED}dharmamitra not found. Install with: pip install dharmamitra${NC}"
    COMPARE_DHARMAMITRA=false
fi

# Benchmark Shlesha
echo -e "${GREEN}=== Shlesha CLI Benchmarks ===${NC}"
for file in bench_data/*.txt; do
    benchmark "Shlesha" "./target/release/examples/cli -f Devanagari -t IAST" "$file"
done

# Benchmark dharmamitra if available
if [ "$COMPARE_DHARMAMITRA" = true ]; then
    echo -e "${GREEN}=== dharmamitra CLI Benchmarks ===${NC}"
    for file in bench_data/*.txt; do
        benchmark "dharmamitra" "dharmamitra --from devanagari --to iast" "$file"
    done
fi

# Python script for statistical comparison
cat > bench_data/analyze_results.py << 'EOF'
#!/usr/bin/env python3
import sys
import statistics

def analyze_benchmark_output(filename):
    with open(filename, 'r') as f:
        lines = f.readlines()
    
    shlesha_times = []
    dharmamitra_times = []
    
    current_tool = None
    for line in lines:
        if "Shlesha CLI Benchmarks" in line:
            current_tool = "shlesha"
        elif "dharmamitra CLI Benchmarks" in line:
            current_tool = "dharmamitra"
        elif "Average time:" in line:
            time = float(line.split(":")[1].strip().rstrip('s'))
            if current_tool == "shlesha":
                shlesha_times.append(time)
            elif current_tool == "dharmamitra":
                dharmamitra_times.append(time)
    
    if shlesha_times and dharmamitra_times:
        print("\n=== Performance Comparison ===")
        print(f"Shlesha average: {statistics.mean(shlesha_times):.4f}s")
        print(f"dharmamitra average: {statistics.mean(dharmamitra_times):.4f}s")
        speedup = statistics.mean(dharmamitra_times) / statistics.mean(shlesha_times)
        print(f"Shlesha is {speedup:.2f}x faster")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        analyze_benchmark_output(sys.argv[1])
EOF

chmod +x bench_data/analyze_results.py

echo -e "${GREEN}Benchmark complete!${NC}"
echo "Results saved in bench_data/"