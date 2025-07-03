#!/usr/bin/env python3
"""
Diagnostic script to identify the wheel import issue.
"""

import subprocess
import tempfile
import shutil
import sys
import os
from pathlib import Path

def run_cmd(cmd, cwd=None):
    """Run command and return result."""
    print(f"\n🔧 Running: {cmd}")
    result = subprocess.run(cmd, shell=True, cwd=cwd, capture_output=True, text=True)
    if result.stdout:
        print(f"STDOUT:\n{result.stdout}")
    if result.stderr:
        print(f"STDERR:\n{result.stderr}")
    return result

def diagnose_wheel_issue():
    """Diagnose the wheel import issue step by step."""
    project_root = Path(__file__).parent.parent.parent
    
    print("🔍 Diagnosing wheel import issue...\n")
    
    # Step 1: Check current configuration
    print("📋 Step 1: Checking pyproject.toml configuration")
    pyproject = project_root / "pyproject.toml"
    with open(pyproject) as f:
        content = f.read()
        if "[tool.maturin]" in content:
            start = content.find("[tool.maturin]")
            end = content.find("\n[", start + 1)
            if end == -1:
                end = len(content)
            print(content[start:end])
    
    # Step 2: Build wheel with explicit features
    print("\n📋 Step 2: Building wheel with explicit Python features")
    with tempfile.TemporaryDirectory() as tmpdir:
        # Copy project to temp dir
        temp_project = Path(tmpdir) / "shlesha_test"
        shutil.copytree(project_root, temp_project, ignore=shutil.ignore_patterns(
            '.git', 'target', '.venv', 'venv', '__pycache__', '*.egg-info'
        ))
        
        # Build wheel
        result = run_cmd(
            "maturin build --release --features python -o wheelhouse",
            cwd=temp_project
        )
        
        if result.returncode != 0:
            print("❌ Wheel build failed!")
            return False
        
        # Find the built wheel
        wheelhouse = temp_project / "wheelhouse"
        wheels = list(wheelhouse.glob("*.whl"))
        if not wheels:
            print("❌ No wheel found!")
            return False
        
        wheel_path = wheels[0]
        print(f"\n✅ Built wheel: {wheel_path.name}")
        
        # Step 3: Inspect wheel contents
        print("\n📋 Step 3: Inspecting wheel contents")
        extract_dir = Path(tmpdir) / "wheel_contents"
        run_cmd(f"unzip -l {wheel_path}")
        run_cmd(f"unzip -q {wheel_path} -d {extract_dir}")
        
        # Check for __init__.py
        init_files = list(extract_dir.rglob("__init__.py"))
        if init_files:
            print("\n⚠️  Found __init__.py files:")
            for f in init_files:
                print(f"  - {f.relative_to(extract_dir)}")
                print(f"    Content: {f.read_text().strip()}")
        
        # Check for .so files
        so_files = list(extract_dir.rglob("*.so")) + list(extract_dir.rglob("*.pyd"))
        if so_files:
            print("\n✅ Found compiled extension modules:")
            for f in so_files:
                print(f"  - {f.relative_to(extract_dir)}")
                # Check if it exports PyInit_shlesha
                result = run_cmd(f"nm -D {f} 2>/dev/null | grep PyInit || true")
        
        # Step 4: Test import in isolated environment
        print("\n📋 Step 4: Testing import in isolated environment")
        test_venv = Path(tmpdir) / "test_venv"
        run_cmd(f"python3 -m venv {test_venv}")
        
        pip_cmd = f"{test_venv}/bin/pip" if sys.platform != "win32" else f"{test_venv}\\Scripts\\pip"
        python_cmd = f"{test_venv}/bin/python" if sys.platform != "win32" else f"{test_venv}\\Scripts\\python"
        
        # Install wheel
        result = run_cmd(f"{pip_cmd} install {wheel_path}")
        if result.returncode != 0:
            print("❌ Wheel installation failed!")
            return False
        
        # Test import
        print("\n🧪 Testing import...")
        result = run_cmd(f'{python_cmd} -c "import shlesha; print(\'✅ Import successful!\')"')
        
        if result.returncode != 0:
            print("\n❌ Import failed! Investigating further...")
            
            # Check what's in site-packages
            site_packages = test_venv / ("lib/python*/site-packages" if sys.platform != "win32" else "Lib/site-packages")
            site_packages = list(test_venv.glob("lib/python*/site-packages/shlesha*" if sys.platform != "win32" else "Lib/site-packages/shlesha*"))
            
            if site_packages:
                print("\n📁 Contents of site-packages:")
                for sp in site_packages:
                    if sp.is_dir():
                        for f in sp.rglob("*"):
                            if f.is_file():
                                print(f"  - {f.relative_to(test_venv)}")
            
            # Try to understand the error
            result = run_cmd(f'{python_cmd} -c "import sys; sys.path.append(\'.\'); import shlesha"')
            
            return False
        
        return True

def check_maturin_behavior():
    """Check how maturin behaves with our configuration."""
    print("\n📋 Checking maturin behavior...")
    
    # Create minimal test project
    with tempfile.TemporaryDirectory() as tmpdir:
        test_project = Path(tmpdir) / "test_project"
        test_project.mkdir()
        
        # Create minimal Cargo.toml
        cargo_toml = """
[package]
name = "test_shlesha"
version = "0.1.0"
edition = "2021"

[dependencies]
pyo3 = { version = "0.25", features = ["extension-module"] }

[lib]
name = "test_shlesha"
crate-type = ["cdylib"]

[features]
python = ["pyo3/extension-module"]
"""
        (test_project / "Cargo.toml").write_text(cargo_toml)
        
        # Create minimal lib.rs
        lib_rs = """
use pyo3::prelude::*;

#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("Hello from Rust!".to_string())
}

#[pymodule]
fn test_shlesha(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    Ok(())
}
"""
        src_dir = test_project / "src"
        src_dir.mkdir()
        (src_dir / "lib.rs").write_text(lib_rs)
        
        # Create pyproject.toml
        pyproject = """
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "test_shlesha"
version = "0.1.0"

[tool.maturin]
features = ["pyo3/extension-module", "python"]
"""
        (test_project / "pyproject.toml").write_text(pyproject)
        
        print("\n🔨 Building minimal test project...")
        result = run_cmd("maturin build --release", cwd=test_project)
        
        if result.returncode == 0:
            print("✅ Minimal test project builds successfully!")
            
            # Now compare with our project
            print("\n🔍 Comparing with our project configuration...")
            # This helps identify configuration differences

if __name__ == "__main__":
    success = diagnose_wheel_issue()
    
    if not success:
        print("\n🔍 Running additional diagnostics...")
        check_maturin_behavior()
    
    sys.exit(0 if success else 1)