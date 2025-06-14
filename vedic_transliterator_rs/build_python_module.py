#!/usr/bin/env python3
"""
Build script for the Rust-based Vedic transliteration Python module
"""

import subprocess
import sys
import os
from pathlib import Path

def build_rust_module():
    """Build the Rust module with Python bindings"""
    
    # Change to the Rust project directory
    rust_dir = Path(__file__).parent
    os.chdir(rust_dir)
    
    print("Building Rust transliteration module...")
    
    # Build the module
    try:
        result = subprocess.run([
            "cargo", "build", "--release", "--features", "python-bindings"
        ], check=True, capture_output=True, text=True)
        
        print("✓ Rust module built successfully")
        
        # Find the built library
        target_dir = rust_dir / "target" / "release"
        
        # Look for the shared library
        lib_files = list(target_dir.glob("libvedic_transliterator.*"))
        if not lib_files:
            lib_files = list(target_dir.glob("vedic_transliterator.*"))
        
        if lib_files:
            lib_file = lib_files[0]
            print(f"✓ Library built: {lib_file}")
            
            # Copy to parent directory for Python import
            import shutil
            parent_dir = rust_dir.parent
            
            # Determine the correct extension for the platform
            if sys.platform == "darwin":
                target_name = "vedic_transliterator.so"
            elif sys.platform == "win32":
                target_name = "vedic_transliterator.pyd"
            else:
                target_name = "vedic_transliterator.so"
            
            target_path = parent_dir / target_name
            shutil.copy2(lib_file, target_path)
            print(f"✓ Module copied to {target_path}")
            
            return True
        else:
            print("✗ No library file found in target directory")
            return False
            
    except subprocess.CalledProcessError as e:
        print(f"✗ Build failed: {e}")
        print("STDOUT:", e.stdout)
        print("STDERR:", e.stderr)
        return False
    except Exception as e:
        print(f"✗ Unexpected error: {e}")
        return False

def test_module():
    """Test the built module"""
    try:
        # Add parent directory to path
        parent_dir = str(Path(__file__).parent.parent)
        if parent_dir not in sys.path:
            sys.path.insert(0, parent_dir)
        
        import vedic_transliterator
        
        # Test basic functionality
        scheme = vedic_transliterator.TargetScheme("slp1")
        result = vedic_transliterator.transliterate("पुरोहितं", scheme)
        
        print(f"✓ Module test passed: '{result.text}' (confidence: {result.confidence})")
        return True
        
    except Exception as e:
        print(f"✗ Module test failed: {e}")
        return False

if __name__ == "__main__":
    print("=== Building Vedic Transliterator Rust Module ===")
    
    if build_rust_module():
        print("\n=== Testing Module ===")
        if test_module():
            print("\n✓ Build and test completed successfully!")
        else:
            print("\n✗ Build succeeded but test failed")
            sys.exit(1)
    else:
        print("\n✗ Build failed")
        sys.exit(1)