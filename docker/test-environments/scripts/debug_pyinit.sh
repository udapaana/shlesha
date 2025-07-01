#!/bin/bash
set -e

echo "🔍 Debugging PyInit_shlesha issue..."

# Create virtual environment
uv venv debug_venv
source debug_venv/bin/activate

# Install shlesha
echo "Installing shlesha..."
uv pip install shlesha==0.1.12 --no-cache-dir

# Find the .so file
echo ""
echo "Looking for .so files..."
SHLESHA_DIR=$(python -c "import site; print(site.getsitepackages()[0])")/shlesha
echo "Shlesha directory: $SHLESHA_DIR"

if [ -d "$SHLESHA_DIR" ]; then
    echo "Contents of shlesha directory:"
    ls -la "$SHLESHA_DIR"
    
    # Find .so files
    SO_FILES=$(find "$SHLESHA_DIR" -name "*.so" 2>/dev/null || true)
    
    if [ -n "$SO_FILES" ]; then
        for so_file in $SO_FILES; do
            echo ""
            echo "Analyzing: $so_file"
            echo "File size: $(ls -lh "$so_file" | awk '{print $5}')"
            
            echo ""
            echo "Checking for PyInit symbols:"
            nm -D "$so_file" 2>/dev/null | grep -i pyinit || echo "No PyInit symbols found with nm -D"
            
            echo ""
            echo "Checking all symbols:"
            nm "$so_file" 2>/dev/null | grep -i pyinit || echo "No PyInit symbols found with nm"
            
            echo ""
            echo "Using readelf:"
            readelf -s "$so_file" 2>/dev/null | grep -i pyinit || echo "No PyInit symbols found with readelf"
            
            echo ""
            echo "Check if it's actually a Python extension:"
            python -c "
import ctypes
try:
    lib = ctypes.CDLL('$so_file')
    print('✅ Library loaded with ctypes')
    # Try to find PyInit_shlesha
    if hasattr(lib, 'PyInit_shlesha'):
        print('✅ PyInit_shlesha found via ctypes')
    else:
        print('❌ PyInit_shlesha not found via ctypes')
except Exception as e:
    print(f'❌ Failed to load library: {e}')
"
        done
    else
        echo "❌ No .so files found!"
    fi
else
    echo "❌ Shlesha directory not found!"
fi

# Try to see what Python expects
echo ""
echo "What Python is looking for:"
python -c "
import importlib.machinery
import importlib.util

# Find the module spec
spec = importlib.util.find_spec('shlesha.shlesha')
if spec:
    print(f'Module spec: {spec}')
    print(f'Origin: {spec.origin}')
    print(f'Loader: {spec.loader}')
    
    # Try to load it manually
    try:
        loader = importlib.machinery.ExtensionFileLoader('shlesha', spec.origin)
        print(f'Expected init function: {loader.create_module.__name__}')
    except Exception as e:
        print(f'Loader error: {e}')
else:
    print('No spec found for shlesha.shlesha')
"