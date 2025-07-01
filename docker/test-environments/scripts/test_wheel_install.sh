#!/bin/bash
set -e

echo "🎯 Testing local wheel installation..."

# Find wheel files
WHEEL_FILES=$(find wheels -name "*.whl" 2>/dev/null || true)

if [ -z "$WHEEL_FILES" ]; then
    echo "⚠️ No wheel files found, skipping local wheel test"
    exit 0
fi

for wheel in $WHEEL_FILES; do
    echo "Testing wheel: $wheel"
    
    # Create fresh virtual environment for each wheel
    VENV_NAME="test_wheel_$(basename $wheel .whl)"
    uv venv "$VENV_NAME"
    source "$VENV_NAME/bin/activate"
    
    # Install the wheel
    echo "Installing $wheel..."
    uv pip install "$wheel" --force-reinstall
    
    # Test import and functionality
    echo "Testing functionality..."
    python3 -c "
import sys
import os
print(f'Testing wheel: $wheel')
print(f'Python: {sys.version}')

try:
    import shlesha
    print('✅ Import successful')
    print(f'Version: {shlesha.__version__}')
    
    # Test functionality
    result = shlesha.transliterate('धर्म', 'devanagari', 'iast')
    if result == 'dharma':
        print('✅ Transliteration test passed')
    else:
        print(f'❌ Unexpected result: {result}')
        exit(1)
        
except Exception as e:
    print(f'❌ Error with wheel $wheel: {e}')
    import traceback
    traceback.print_exc()
    exit(1)
"
    
    deactivate
    echo "✅ Wheel $wheel test passed"
done

echo "✅ All wheel installation tests passed"