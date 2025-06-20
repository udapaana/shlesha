#!/bin/bash

echo "=== Shlesha Benchmark Suite ==="
echo

# Test if tools are available
echo "Checking available tools..."

# Check Shlesha
if [ -f "./target/release/examples/cli" ]; then
    echo "✓ Shlesha CLI found"
    SHLESHA_AVAILABLE=true
else
    echo "✗ Shlesha CLI not found - building..."
    cargo build --release --example cli
    SHLESHA_AVAILABLE=true
fi

# Check Python libraries
echo
echo "Python libraries:"
python3 -c "import indic_transliteration; print('✓ indic-transliteration available')" 2>/dev/null || echo "✗ indic-transliteration not found"
python3 -c "import aksharamukha; print('✓ aksharamukha available')" 2>/dev/null || echo "✗ aksharamukha not found"

echo
echo "=== Performance Benchmarks ==="
echo

# Test data
SHORT_TEXT="नमस्ते"
MEDIUM_TEXT="अहं संस्कृतं वदामि"
LONG_TEXT="तत्र शूरा महेष्वासा भीमार्जुनसमा युधि । युयुधानो विराटश्च द्रुपदश्च महारथः ॥"

# Function to measure time
measure_time() {
    local cmd="$1"
    local text="$2"
    local name="$3"
    
    # Run 10 times and measure
    local total_time=0
    for i in {1..10}; do
        start=$(date +%s.%N)
        eval "$cmd" > /dev/null 2>&1
        end=$(date +%s.%N)
        elapsed=$(echo "$end - $start" | bc)
        total_time=$(echo "$total_time + $elapsed" | bc)
    done
    
    avg_time=$(echo "scale=4; $total_time / 10" | bc)
    echo "$name: ${avg_time}s (avg of 10 runs)"
}

# Benchmark Shlesha
echo "Short text (${#SHORT_TEXT} chars):"
measure_time "./target/release/examples/cli -f devanagari -t iast '$SHORT_TEXT'" "$SHORT_TEXT" "  Shlesha"

echo
echo "Medium text (${#MEDIUM_TEXT} chars):"
measure_time "./target/release/examples/cli -f devanagari -t iast '$MEDIUM_TEXT'" "$MEDIUM_TEXT" "  Shlesha"

echo
echo "Long text (${#LONG_TEXT} chars):"
measure_time "./target/release/examples/cli -f devanagari -t iast '$LONG_TEXT'" "$LONG_TEXT" "  Shlesha"

# Python benchmark comparison
if python3 -c "import indic_transliteration" 2>/dev/null; then
    echo
    echo "=== Python Comparison ==="
    cat > /tmp/bench_python.py << 'EOF'
import sys
import time
from indic_transliteration import sanscript

text = sys.argv[1]
start = time.time()
result = sanscript.transliterate(text, sanscript.DEVANAGARI, sanscript.IAST)
end = time.time()
print(f"{end - start:.6f}")
EOF

    echo "Short text with indic-transliteration:"
    total_time=0
    for i in {1..10}; do
        elapsed=$(python3 /tmp/bench_python.py "$SHORT_TEXT")
        total_time=$(echo "$total_time + $elapsed" | bc)
    done
    avg_time=$(echo "scale=4; $total_time / 10" | bc)
    echo "  indic-transliteration: ${avg_time}s (avg of 10 runs)"
fi

echo
echo "=== Accuracy Test ==="
echo

# Test accuracy
test_accuracy() {
    local input="$1"
    local expected="$2"
    local result=$(./target/release/examples/cli -f devanagari -t iast "$input" 2>/dev/null)
    
    if [ "$result" = "$expected" ]; then
        echo "✓ $input → $result"
    else
        echo "✗ $input → $result (expected: $expected)"
    fi
}

test_accuracy "संस्कृत" "saṃskṛta"
test_accuracy "धर्म" "dharma"
test_accuracy "कर्म" "karma"
test_accuracy "अर्जुन" "arjuna"
test_accuracy "कृष्ण" "kṛṣṇa"

echo
echo "=== Benchmark Complete ==="