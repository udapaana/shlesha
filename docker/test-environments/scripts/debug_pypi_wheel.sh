#!/bin/bash
set -e

echo "🔍 Debugging PyPI wheel issue specifically"

# Create clean environment
uv venv clean_env --python 3.11
source clean_env/bin/activate

# Clear any pip cache and force fresh download
echo "Clearing uv cache..."
uv cache clean

# Download the exact wheel from PyPI
echo "Force downloading shlesha==0.1.12 from PyPI..."
uv pip install shlesha==0.1.12 --no-cache-dir --force-reinstall --no-deps

# Get the exact wheel that was installed
SHLESHA_DIR=$(python -c "import shlesha; import os; print(os.path.dirname(shlesha.__file__))")
echo "Shlesha installed at: $SHLESHA_DIR"

SO_FILE="$SHLESHA_DIR/shlesha.cpython-311-x86_64-linux-gnu.so"

if [ -f "$SO_FILE" ]; then
    echo ""
    echo "Analyzing .so file: $SO_FILE"
    echo "File size: $(ls -lh "$SO_FILE" | awk '{print $5}')"
    
    echo ""
    echo "=== Symbol Analysis ==="
    echo "Looking for PyInit symbols (all methods):"
    
    # Method 1: nm with dynamic symbols
    echo "1. nm -D:"
    nm -D "$SO_FILE" 2>/dev/null | grep -i pyinit || echo "   No PyInit symbols found"
    
    # Method 2: nm with all symbols
    echo "2. nm (all symbols):"
    nm "$SO_FILE" 2>/dev/null | grep -i pyinit || echo "   No PyInit symbols found"
    
    # Method 3: readelf
    echo "3. readelf -s:"
    readelf -s "$SO_FILE" 2>/dev/null | grep -i pyinit || echo "   No PyInit symbols found"
    
    # Method 4: objdump
    echo "4. objdump -T:"
    objdump -T "$SO_FILE" 2>/dev/null | grep -i pyinit || echo "   No PyInit symbols found"
    
    # Method 5: strings (look for the actual string)
    echo "5. strings (looking for PyInit_shlesha):"
    strings "$SO_FILE" | grep -i pyinit || echo "   No PyInit strings found"
    
    echo ""
    echo "=== File Analysis ==="
    echo "File type:"
    file "$SO_FILE" 2>/dev/null || echo "file command not available"
    
    echo ""
    echo "File headers:"
    readelf -h "$SO_FILE" 2>/dev/null | head -10 || echo "readelf not available"
    
    echo ""
    echo "=== Import Test ==="
    echo "Testing Python import:"
    python -c "
try:
    # Try to import the module directly
    import importlib.util
    spec = importlib.util.spec_from_file_location('shlesha_module', '$SO_FILE')
    if spec and spec.loader:
        try:
            module = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(module)
            print('✅ Direct .so import successful')
        except Exception as e:
            print(f'❌ Direct .so import failed: {e}')
    else:
        print('❌ Could not create module spec')
        
    # Try the normal import
    import shlesha
    print('✅ Normal shlesha import successful')
except Exception as e:
    print(f'❌ Normal import failed: {e}')
    import traceback
    traceback.print_exc()
"
else
    echo "❌ .so file not found: $SO_FILE"
    echo "Available files in shlesha directory:"
    ls -la "$SHLESHA_DIR"
fi