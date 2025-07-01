#!/usr/bin/env python3
"""
Simple integration test that can run anywhere to verify basic functionality
"""

import sys
import subprocess
import tempfile
import os

def test_current_installation():
    """Test the currently installed version of shlesha"""
    print("🧪 Testing current shlesha installation...")
    
    try:
        import shlesha
        print(f"✅ Import successful - version {shlesha.__version__}")
        
        # Test basic functionality
        result = shlesha.transliterate('धर्म', 'devanagari', 'iast')
        if result == 'dharma':
            print(f"✅ Basic transliteration: धर्म → {result}")
        else:
            print(f"❌ Unexpected result: {result}")
            return False
            
        # Test class methods
        translator = shlesha.Shlesha()
        result2 = translator.transliterate('योग', 'devanagari', 'iast')
        if result2 == 'yoga':
            print(f"✅ Class method: योग → {result2}")
        else:
            print(f"❌ Unexpected class result: {result2}")
            return False
            
        return True
        
    except ImportError as e:
        print(f"❌ Import failed: {e}")
        return False
    except Exception as e:
        print(f"❌ Unexpected error: {e}")
        return False

def test_fresh_installation():
    """Test installing shlesha in a fresh environment"""
    print("\n🔄 Testing fresh installation...")
    
    with tempfile.TemporaryDirectory() as temp_dir:
        venv_path = os.path.join(temp_dir, 'test_venv')
        
        # Create virtual environment
        subprocess.run([sys.executable, '-m', 'venv', venv_path], check=True)
        
        # Get python executable in venv
        if sys.platform == 'win32':
            python_exe = os.path.join(venv_path, 'Scripts', 'python.exe')
        else:
            python_exe = os.path.join(venv_path, 'bin', 'python')
            
        # Install shlesha
        subprocess.run([python_exe, '-m', 'pip', 'install', 'shlesha', '--no-cache-dir'], 
                      check=True, capture_output=True)
        
        # Test in fresh environment
        test_script = '''
import shlesha
result = shlesha.transliterate("धर्म", "devanagari", "iast")
print(f"Fresh install test: {result}")
assert result == "dharma", f"Expected dharma, got {result}"
print("✅ Fresh installation test passed")
'''
        
        result = subprocess.run([python_exe, '-c', test_script], 
                              capture_output=True, text=True)
        
        if result.returncode == 0:
            print("✅ Fresh installation test passed")
            print(result.stdout.strip())
            return True
        else:
            print("❌ Fresh installation test failed")
            print(f"stdout: {result.stdout}")
            print(f"stderr: {result.stderr}")
            return False

def main():
    print("🚀 Shlesha Simple Integration Test")
    print("================================")
    print(f"Python: {sys.version}")
    print(f"Platform: {sys.platform}")
    
    tests_passed = 0
    total_tests = 2
    
    # Test 1: Current installation
    if test_current_installation():
        tests_passed += 1
    
    # Test 2: Fresh installation
    if test_fresh_installation():
        tests_passed += 1
    
    print(f"\n📊 Results: {tests_passed}/{total_tests} tests passed")
    
    if tests_passed == total_tests:
        print("🎉 All tests passed!")
        return 0
    else:
        print("💥 Some tests failed!")
        return 1

if __name__ == '__main__':
    sys.exit(main())