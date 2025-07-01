#!/bin/bash
set -e

echo "📦 Testing PyPI installation..."

# Create fresh virtual environment
uv venv test_pypi_venv
source test_pypi_venv/bin/activate

# Install latest shlesha from PyPI
echo "Installing shlesha from PyPI..."
uv pip install shlesha --no-cache-dir

# Test import
echo "Testing import..."
python3 -c "
import sys
print(f'Python: {sys.version}')
print(f'Platform: {sys.platform}')

try:
    import shlesha
    print('✅ Import successful')
    print(f'Version: {shlesha.__version__}')
    
    # Test basic functionality
    result = shlesha.transliterate('धर्म', 'devanagari', 'iast')
    print(f'✅ Transliteration: {result}')
    
    # Test class instantiation
    translator = shlesha.Shlesha()
    result2 = translator.transliterate('योग', 'devanagari', 'iast')
    print(f'✅ Class method: {result2}')
    
    # Test script support
    scripts = shlesha.get_supported_scripts()
    print(f'✅ Supported scripts: {len(scripts)} found')
    assert 'devanagari' in scripts
    assert 'iast' in scripts
    
except Exception as e:
    print(f'❌ Error: {e}')
    import traceback
    traceback.print_exc()
    exit(1)
"

echo "✅ PyPI installation test passed"