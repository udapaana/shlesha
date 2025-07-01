# Shlesha Integration Testing Platform

This directory contains a comprehensive Docker-based testing platform for Shlesha that ensures our Python extensions work correctly across different environments.

## Why This Exists

We built this after encountering the **PyInit_shlesha missing symbol** issue where:
- ✅ Local development worked fine
- ❌ Google Colab failed with `ImportError: dynamic module does not define module export function (PyInit_shlesha)`
- ❌ Arch Linux Docker containers failed with the same error

This platform prevents such environment-specific issues from reaching users.

## Test Environments

### 1. Ubuntu (`Dockerfile.ubuntu`)
- **Purpose**: Standard Linux environment testing
- **Base**: Ubuntu 22.04
- **Python**: System Python 3
- **Use case**: Most common Linux deployment environment

### 2. Arch Linux (`Dockerfile.archlinux`)
- **Purpose**: Rolling release, bleeding-edge testing
- **Base**: archlinux:latest
- **Python**: Latest Python from Arch repos
- **Use case**: Catches issues with newer Python versions and different build environments

### 3. Google Colab (`Dockerfile.colab`)
- **Purpose**: Mimic Google Colab environment
- **Base**: python:3.11-slim
- **Python**: Python 3.11 (Colab's version)
- **Use case**: Ensure compatibility with Jupyter/Colab environments

## Test Suites

### 1. PyPI Installation Test (`test_pypi_install.sh`)
- Installs latest shlesha from PyPI
- Tests basic import and functionality
- Verifies version information
- **Catches**: PyPI distribution issues

### 2. Wheel Installation Test (`test_wheel_install.sh`)
- Tests local wheel files (if available)
- Useful for pre-release testing
- **Catches**: Build-specific issues before PyPI upload

### 3. Functionality Test (`test_functionality.sh`)
- Comprehensive API testing
- Tests all major features:
  - Basic transliteration
  - Class methods
  - Metadata collection
  - Error handling
  - Convenience functions
- **Catches**: API regressions and functionality issues

### 4. Binary Analysis (`test_binary_analysis.sh`)
- Analyzes compiled `.so` files
- Checks for required symbols (PyInit_*)
- Provides debugging information
- **Catches**: Symbol export issues, binary problems

## Usage

### Local Testing

```bash
# Simple test (works anywhere)
python3 scripts/test_simple.py

# Full local testing with Docker
./scripts/test_local.sh

# Manual environment testing
cd docker/test-environments
./run_integration_tests.sh
```

### CI Integration

The platform automatically runs on:
- ✅ Pull requests
- ✅ Pushes to main
- ✅ Release tags

See `.github/workflows/integration-tests.yml` for configuration.

### Manual Environment Testing

```bash
cd docker/test-environments

# Test specific environment
docker build -f Dockerfile.ubuntu -t shlesha-test-ubuntu .
docker run --rm shlesha-test-ubuntu

# Test all environments
./run_integration_tests.sh
```

## Adding New Test Environments

1. Create `Dockerfile.<environment>` in this directory
2. Follow the pattern of existing Dockerfiles:
   - Install system dependencies
   - Install uv
   - Copy test scripts
   - Set executable permissions
3. Add the environment to `ENVIRONMENTS` array in `run_integration_tests.sh`
4. Add to CI matrix in `.github/workflows/integration-tests.yml`

## Test Output

### Success Example
```
🧪 Testing environment: ubuntu
✅ Docker image built successfully
✅ PyPI installation test passed
✅ Functionality tests completed
✅ Binary analysis completed
✅ Tests passed for ubuntu
```

### Failure Example
```
❌ Import failed: dynamic module does not define module export function (PyInit_shlesha)
❌ No PyInit symbols found!
❌ Tests failed for ubuntu
```

## Debugging Failed Tests

### Import Errors
1. Check binary analysis output for missing symbols
2. Verify PyO3 module definition in source code
3. Check compilation flags and environment differences

### Functionality Errors
1. Review test output for specific failing operations
2. Check version compatibility issues
3. Verify script support and mapping availability

### Environment Errors
1. Check Dockerfile for missing dependencies
2. Verify Python version compatibility
3. Review system-specific package installation

## Best Practices

1. **Always run integration tests** before releasing
2. **Test locally first** with `scripts/test_simple.py`
3. **Use Docker tests for comprehensive validation**
4. **Check CI status** before merging PRs
5. **Update test environments** when adding new features

## Future Enhancements

- [ ] Windows container testing
- [ ] Multiple Python version matrix per environment
- [ ] Performance regression testing
- [ ] Memory usage analysis
- [ ] Network-isolated testing
- [ ] PyPI upload validation hooks