#!/usr/bin/env python3
"""Test script to reproduce the import issue"""

import sys
print(f"Python version: {sys.version}")
print(f"Platform: {sys.platform}")

try:
    import shlesha
    print("✅ Import successful!")
    
    # Test basic functionality
    result = shlesha.transliterate('धर्म', 'devanagari', 'iast')
    print(f"✅ Transliteration works: '{result}'")
    
    # Test version
    print(f"✅ Version: {shlesha.__version__}")
    
except ImportError as e:
    print(f"❌ Import failed: {e}")
    print("\nDebugging info:")
    
    # Check if the package is installed
    try:
        import pkg_resources
        dist = pkg_resources.get_distribution('shlesha')
        print(f"Package found: {dist.location}")
        print(f"Version: {dist.version}")
        
        # Try to find the actual .so file
        import os
        import glob
        
        shlesha_path = os.path.join(dist.location, 'shlesha')
        if os.path.exists(shlesha_path):
            print(f"Shlesha directory: {shlesha_path}")
            files = glob.glob(os.path.join(shlesha_path, '*'))
            print(f"Files in shlesha/: {files}")
            
            # Look for .so files
            so_files = glob.glob(os.path.join(shlesha_path, '*.so'))
            print(f"SO files: {so_files}")
            
            if so_files:
                import subprocess
                for so_file in so_files:
                    print(f"\nChecking symbols in {so_file}:")
                    try:
                        result = subprocess.run(['nm', so_file], capture_output=True, text=True)
                        if 'PyInit' in result.stdout:
                            print("✅ PyInit symbol found")
                            pyinit_lines = [line for line in result.stdout.split('\n') if 'PyInit' in line]
                            for line in pyinit_lines:
                                print(f"  {line}")
                        else:
                            print("❌ No PyInit symbol found")
                    except Exception as ex:
                        print(f"  Error checking symbols: {ex}")
        
    except Exception as ex:
        print(f"Error getting package info: {ex}")

except Exception as e:
    print(f"❌ Unexpected error: {e}")
    import traceback
    traceback.print_exc()