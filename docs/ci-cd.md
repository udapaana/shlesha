# CI/CD Documentation

This document describes the continuous integration and deployment processes for the Shlesha project.

## Overview

Shlesha uses GitHub Actions for CI/CD with multiple workflows to ensure code quality, test coverage, and proper releases across different platforms.

## Workflows

### 1. Main CI Workflow (`.github/workflows/ci.yml`)

Runs on every push and pull request to ensure code quality and functionality.

#### Jobs:

- **Lint**: Checks code formatting and runs Clippy lints
- **Test Suite**: Runs tests across multiple OS (Ubuntu, macOS, Windows) and Rust versions (stable, beta)
  - **Note**: Python feature is excluded on Ubuntu due to PyO3 linking issues in CI environment
- **Minimal Versions**: Tests with minimal dependency versions to ensure correct version bounds
  - **Note**: Tests only CLI feature to avoid PyO3 linking issues
- **Security Audit**: Checks for known vulnerabilities in dependencies
- **Code Coverage**: Generates test coverage reports
  - **Note**: Excludes Python feature to avoid linking issues
- **Benchmarks**: Runs performance benchmarks (informational only)

### 2. Python Workflow (`.github/workflows/python.yml`)

Dedicated workflow for Python bindings that runs on every push and pull request.

#### Why Separate Python Testing?

The Python bindings require specific build tooling (maturin) and environment setup that differs from standard Rust testing. This workflow:
- Tests across multiple Python versions (3.8-3.12)
- Tests on all major platforms (Ubuntu, macOS, Windows)
- Uses maturin for proper wheel building
- Runs Python-specific tests in `python/tests/`

This comprehensive Python testing allows us to exclude the Python feature from the main CI workflow without losing test coverage.

### 3. WASM Workflow (`.github/workflows/wasm.yml`)

Tests WebAssembly builds and functionality.

#### Jobs:
- Builds WASM for both web and Node.js targets
- Runs WASM-specific tests
- Publishes to npm on release (if configured)

### 4. Release Workflow (`.github/workflows/release.yml`)

Handles automated releases when tags are pushed.

## Known Issues and Workarounds

### PyO3 Linking in GitHub Actions

**Issue**: PyO3 (Python bindings) fails to link in some CI environments due to missing Python libraries.

**Affected Jobs**:
- Test Suite (Ubuntu)
- Minimal Versions
- Code Coverage

**Solution**: These jobs exclude the Python feature and test only `cli,wasm,native-examples` features. Python functionality is thoroughly tested in the dedicated Python workflow.

**Root Cause**: GitHub Actions Ubuntu runners use a Python installation in `/opt/hostedtoolcache/` that doesn't properly expose libraries for linking when using standard `cargo test`.

### WASM Symbol Conflicts

**Issue**: Both `shlesha` and `vidyut-lipi` export WASM functions with the same names.

**Solution**: `vidyut-lipi` is configured as a target-specific dependency only for non-WASM builds:
```toml
[target.'cfg(not(target_arch = "wasm32"))'.dev-dependencies]
vidyut-lipi = "0.2.0"
```

## Local Development

To run the full test suite locally:

```bash
# Run all tests
cargo test --all-features

# Run tests without Python (mimics CI)
cargo test --features cli,wasm,native-examples

# Run Python tests
maturin develop --features python
pytest python/tests

# Run WASM tests
wasm-pack test --node --features wasm
```

## Future Improvements

1. Investigate proper Python library linking in GitHub Actions to enable Python feature in all tests
2. Consider using containers with pre-configured Python development environments
3. Add performance regression detection to benchmark jobs