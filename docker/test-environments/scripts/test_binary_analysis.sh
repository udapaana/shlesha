#!/bin/bash
set -e

echo "🔍 Running binary analysis..."

# Use existing venv from PyPI test
source test_pypi_venv/bin/activate

python3 -c "
import shlesha
import os
import glob
import subprocess
import sys

def analyze_binary():
    '''Analyze the compiled binary for debugging'''
    
    # Find the shlesha module location
    shlesha_file = shlesha.__file__
    shlesha_dir = os.path.dirname(shlesha_file)
    
    print(f'Shlesha module location: {shlesha_dir}')
    print(f'Shlesha __file__: {shlesha_file}')
    
    # List all files in the shlesha directory
    files = os.listdir(shlesha_dir)
    print(f'Files in shlesha directory: {files}')
    
    # Find .so files (compiled extensions)
    so_files = glob.glob(os.path.join(shlesha_dir, '*.so'))
    
    if not so_files:
        print('❌ No .so files found!')
        return False
        
    for so_file in so_files:
        print(f'\\nAnalyzing binary: {so_file}')
        
        # Check file info
        stat_info = os.stat(so_file)
        print(f'File size: {stat_info.st_size} bytes')
        print(f'File permissions: {oct(stat_info.st_mode)}')
        
        # Check if file is executable/readable
        if os.access(so_file, os.R_OK):
            print('✅ File is readable')
        else:
            print('❌ File is not readable')
            
        # Try to check symbols with nm
        try:
            result = subprocess.run(['nm', so_file], capture_output=True, text=True, timeout=10)
            if result.returncode == 0:
                # Look for PyInit symbols
                pyinit_symbols = [line for line in result.stdout.split('\\n') if 'PyInit' in line]
                if pyinit_symbols:
                    print('✅ PyInit symbols found:')
                    for symbol in pyinit_symbols:
                        print(f'  {symbol.strip()}')
                else:
                    print('❌ No PyInit symbols found!')
                    print('Available symbols (first 10):')
                    lines = result.stdout.split('\\n')[:10]
                    for line in lines:
                        if line.strip():
                            print(f'  {line.strip()}')
            else:
                print(f'nm command failed: {result.stderr}')
        except subprocess.TimeoutExpired:
            print('nm command timed out')
        except FileNotFoundError:
            print('nm command not available')
        except Exception as e:
            print(f'Error running nm: {e}')
            
        # Try to check with objdump if available
        try:
            result = subprocess.run(['objdump', '-T', so_file], capture_output=True, text=True, timeout=10)
            if result.returncode == 0 and 'PyInit' in result.stdout:
                print('✅ PyInit symbols found with objdump')
                pyinit_lines = [line for line in result.stdout.split('\\n') if 'PyInit' in line]
                for line in pyinit_lines[:3]:  # Show first 3 matches
                    print(f'  {line.strip()}')
        except:
            pass  # objdump might not be available
            
        # Try to load the module directly to see if symbols are accessible
        try:
            import ctypes
            lib = ctypes.CDLL(so_file)
            if hasattr(lib, 'PyInit_shlesha'):
                print('✅ PyInit_shlesha symbol accessible via ctypes')
            else:
                print('❌ PyInit_shlesha not accessible via ctypes')
        except Exception as e:
            print(f'ctypes analysis failed: {e}')
    
    return True

def check_python_import_internals():
    '''Check Python's internal view of the module'''
    
    print('\\nPython import internals:')
    
    # Check module spec
    import importlib.util
    spec = importlib.util.find_spec('shlesha.shlesha')
    if spec:
        print(f'Module spec found: {spec}')
        print(f'Origin: {spec.origin}')
        print(f'Loader: {spec.loader}')
    else:
        print('❌ No module spec found for shlesha.shlesha')
        
    # Check sys.modules
    import sys
    if 'shlesha.shlesha' in sys.modules:
        mod = sys.modules['shlesha.shlesha']
        print(f'Module in sys.modules: {mod}')
        print(f'Module file: {getattr(mod, \"__file__\", \"No __file__\")}')
    else:
        print('Module not in sys.modules')

# Run analysis
print('Starting binary analysis...')
try:
    if analyze_binary():
        check_python_import_internals()
        print('✅ Binary analysis completed')
    else:
        print('❌ Binary analysis failed')
        sys.exit(1)
except Exception as e:
    print(f'❌ Binary analysis failed with exception: {e}')
    import traceback
    traceback.print_exc()
    sys.exit(1)
"

echo "✅ Binary analysis completed"