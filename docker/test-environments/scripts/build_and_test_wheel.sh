#!/bin/bash
set -e

echo "🔨 Building wheel from source in Docker environment"

# Clone the repository at the specific tag
cd /tmp
git clone https://github.com/udapaana/shlesha.git
cd shlesha
git checkout v0.1.12

echo "📋 Repository info:"
echo "Current commit: $(git rev-parse HEAD)"
echo "Tag: $(git describe --tags)"

# Install Rust and maturin
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Install uv and maturin
export PATH="$HOME/.local/bin:$PATH"
uv venv build_env
source build_env/bin/activate
uv pip install maturin

echo ""
echo "🔨 Building wheel..."
maturin build --features python --release --out wheels/

echo ""
echo "📦 Built wheel:"
ls -la wheels/

echo ""
echo "🔍 Analyzing built wheel..."
WHEEL_FILE=$(ls wheels/*.whl)
echo "Wheel file: $WHEEL_FILE"

# Extract and analyze
cd /tmp
mkdir wheel_analysis
cd wheel_analysis
unzip "$WHEEL_FILE"

SO_FILE=$(find . -name "*.so")
echo "SO file: $SO_FILE"

if [ -f "$SO_FILE" ]; then
    echo ""
    echo "📊 Symbol analysis:"
    echo "File size: $(ls -lh "$SO_FILE" | awk '{print $5}')"
    
    # Check for PyInit symbols
    echo "PyInit symbols:"
    nm -D "$SO_FILE" 2>/dev/null | grep -i pyinit || echo "  No PyInit symbols found with nm -D"
    nm "$SO_FILE" 2>/dev/null | grep -i pyinit || echo "  No PyInit symbols found with nm"
    readelf -s "$SO_FILE" 2>/dev/null | grep -i pyinit || echo "  No PyInit symbols found with readelf"
    
    echo ""
    echo "🧪 Testing the wheel..."
    cd /tmp
    uv venv test_wheel_env
    source test_wheel_env/bin/activate
    uv pip install "$WHEEL_FILE"
    
    python -c "
try:
    import shlesha
    print('✅ Wheel import successful!')
    result = shlesha.transliterate('धर्म', 'devanagari', 'iast')
    print(f'✅ Functionality test: धर्म → {result}')
except Exception as e:
    print(f'❌ Wheel test failed: {e}')
    import traceback
    traceback.print_exc()
"
else
    echo "❌ No .so file found!"
fi