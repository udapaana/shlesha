#!/bin/bash

# Shlesha Performance Test Setup and Runner
# This script will set up everything and run the performance comparison

set -e  # Exit on any error

echo "🚀 Shlesha Performance Test Setup & Runner"
echo "=========================================="

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the shlesha project root directory"
    exit 1
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust/Cargo is not installed. Please install from https://rustup.rs/"
    exit 1
fi

# Check if Python is installed
if ! command -v python3 &> /dev/null; then
    echo "❌ Error: Python 3 is not installed"
    exit 1
fi

echo "✅ Prerequisites check passed"

# Clean up any existing virtual environment
if [ -d "venv" ]; then
    echo "🧹 Cleaning up existing virtual environment..."
    rm -rf venv
fi

# Create virtual environment
echo "🐍 Creating Python virtual environment..."
python3 -m venv venv

# Activate virtual environment
echo "🔧 Activating virtual environment..."
source venv/bin/activate

# Upgrade pip
echo "📦 Upgrading pip..."
python -m pip install --upgrade pip

# Install maturin
echo "🔨 Installing maturin..."
python -m pip install maturin

# Build Shlesha with optimizations
echo "⚡ Building Shlesha with Aho-Corasick optimizations..."
cargo build --release

# Install Shlesha Python wheel
echo "📦 Installing Shlesha Python package..."
maturin develop --release --features python

# Check for demo scripts
echo "📝 Checking for test scripts..."
if [ -f "optimization_showcase.py" ]; then
    echo "✅ Optimization showcase script found"
else
    echo "❌ Showcase script not found. Please make sure optimization_showcase.py exists."
    exit 1
fi

# Install Vidyut for comparison
echo "📥 Installing Vidyut for comparison..."
python -m pip install vidyut

# Create the performance test script
echo "📝 Creating performance test script..."
cat > performance_test.py << 'EOF'
#!/usr/bin/env python3
"""
Quick Performance Test: Shlesha vs Vidyut
Shows the impact of Aho-Corasick optimization
"""

import time
import statistics

def test_conversion(name, shlesha_func, vidyut_func, iterations=3000):
    """Test and compare a specific conversion."""
    print(f"\n🧪 Testing: {name}")
    print("-" * 50)
    
    # Test Shlesha
    times = []
    for _ in range(iterations):
        start = time.perf_counter()
        shlesha_func()
        times.append(time.perf_counter() - start)
    
    shlesha_avg = statistics.mean(times)
    shlesha_throughput = 1 / shlesha_avg
    
    # Test Vidyut
    times = []
    for _ in range(iterations):
        start = time.perf_counter()
        vidyut_func()
        times.append(time.perf_counter() - start)
    
    vidyut_avg = statistics.mean(times)
    vidyut_throughput = 1 / vidyut_avg
    
    # Compare
    if shlesha_avg < vidyut_avg:
        faster = "Shlesha"
        speedup = vidyut_avg / shlesha_avg
    else:
        faster = "Vidyut"
        speedup = shlesha_avg / vidyut_avg
    
    print(f"Shlesha: {shlesha_throughput:,.0f} conversions/second")
    print(f"Vidyut:  {vidyut_throughput:,.0f} conversions/second")
    print(f"🏆 {faster} is {speedup:.2f}x faster!")
    
    return {
        'name': name,
        'faster': faster,
        'speedup': speedup,
        'shlesha_throughput': shlesha_throughput,
        'vidyut_throughput': vidyut_throughput
    }

def main():
    """Run the performance comparison."""
    import shlesha
    from vidyut.lipi import transliterate, Scheme
    
    print("⚡ Shlesha vs Vidyut Performance Comparison")
    print("=" * 60)
    print("Testing the impact of Aho-Corasick optimization")
    
    results = []
    
    # Test 1: Indic → Indic (where Shlesha should win)
    def shlesha_indic(): 
        return shlesha.transliterate("నమస్కారం", "telugu", "devanagari")
    def vidyut_indic(): 
        return transliterate("నమస్కారం", Scheme.Telugu, Scheme.Devanagari)
    
    results.append(test_conversion(
        "Indic → Indic (Telugu → Devanagari)",
        shlesha_indic, vidyut_indic
    ))
    
    # Test 2: Roman → Roman (Aho-Corasick optimization)
    def shlesha_roman(): 
        return shlesha.transliterate("namaskāram", "iast", "slp1")
    def vidyut_roman(): 
        return transliterate("namaskāram", Scheme.Iast, Scheme.Slp1)
    
    results.append(test_conversion(
        "Roman → Roman (IAST → SLP1)",
        shlesha_roman, vidyut_roman
    ))
    
    # Test 3: Roman → Indic (complex pipeline)
    def shlesha_roman_indic(): 
        return shlesha.transliterate("saṃskṛtam", "iast", "telugu")
    def vidyut_roman_indic(): 
        return transliterate("saṃskṛtam", Scheme.Iast, Scheme.Telugu)
    
    results.append(test_conversion(
        "Roman → Indic (IAST → Telugu)",
        shlesha_roman_indic, vidyut_roman_indic
    ))
    
    # Test 4: Indic → Roman
    def shlesha_indic_roman(): 
        return shlesha.transliterate("సంస్కృతం", "telugu", "slp1")
    def vidyut_indic_roman(): 
        return transliterate("సంస్కృతం", Scheme.Telugu, Scheme.Slp1)
    
    results.append(test_conversion(
        "Indic → Roman (Telugu → SLP1)",
        shlesha_indic_roman, vidyut_indic_roman
    ))
    
    # Summary
    print(f"\n🏆 FINAL RESULTS")
    print("=" * 70)
    print(f"{'Test':<35} {'Winner':<8} {'Speedup':<8} {'Shlesha/s':<12} {'Vidyut/s':<12}")
    print("-" * 70)
    
    shlesha_wins = 0
    for result in results:
        if result['faster'] == 'Shlesha':
            shlesha_wins += 1
        
        print(f"{result['name']:<35} {result['faster']:<8} {result['speedup']:<8.1f} "
              f"{result['shlesha_throughput']:<12.0f} {result['vidyut_throughput']:<12.0f}")
    
    print(f"\n🎯 Shlesha wins {shlesha_wins}/{len(results)} categories!")
    
    if shlesha_wins >= 2:
        print("🚀 Excellent! Aho-Corasick optimization is working!")
    else:
        print("🤔 Hmm, performance could be better. Check your build.")
    
    # Show sample outputs
    print(f"\n📝 Sample Conversions:")
    print(f"Telugu → Devanagari: నమస్కారం → {shlesha.transliterate('నమస్కారం', 'telugu', 'devanagari')}")
    print(f"IAST → SLP1: namaskāram → {shlesha.transliterate('namaskāram', 'iast', 'slp1')}")
    print(f"IAST → Telugu: saṃskṛtam → {shlesha.transliterate('saṃskṛtam', 'iast', 'telugu')}")

if __name__ == "__main__":
    main()
EOF

# Make sure we're still in the virtual environment and run the test
echo "🏃 Running comprehensive optimization showcase..."
echo ""
python optimization_showcase.py

echo ""
echo "✅ Optimization showcase completed!"
echo ""
echo "💡 Available tests to run:"
echo "   source venv/bin/activate"
echo "   python performance_test.py           # Basic comparison test"
echo "   python demo_optimization.py          # Simple optimization demo"
echo "   python optimization_showcase.py      # Comprehensive showcase"
echo "   python processor_comparison.py       # Internal processor comparison"
echo ""
echo "🔍 The comprehensive showcase shows:"
echo "   - Shlesha vs Vidyut across 6 different transliteration scenarios"
echo "   - Real-world performance with complex text patterns"
echo "   - Where Shlesha wins (complex Roman → Roman patterns)"
echo "   - Detailed category-by-category analysis"
echo "   - Technical implementation details of optimizations"