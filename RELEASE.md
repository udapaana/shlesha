# Release Process

This document outlines the automated release process for Shlesha.

## Quick Start (Recommended)

For most releases, simply run the comprehensive release script:

```bash
./scripts/release.sh
```

This script will guide you through the entire process automatically.

## Manual Process

If you prefer manual control, you can run individual scripts:

### 1. Test Release Readiness

```bash
./scripts/test-release.sh
```

### 2. Prepare Version (Optional)

```bash
./scripts/prepare-release.sh
```

### 3. Create and Push Tags

```bash
./scripts/tag-release.sh
```

## Release Types

The automated scripts support several release types:

### Release Candidates (RC)
- **Purpose**: Testing on TestPyPI before stable release
- **Tag format**: `v0.1.0-rc1`, `v0.1.0-rc2`, etc.
- **Publishes to**: TestPyPI + npm with `@rc` tag
- **Automatic detection**: Script detects existing RCs and increments

### Stable Releases
- **Purpose**: Production releases
- **Tag format**: `v0.1.0`, `v0.2.0`, etc.
- **Publishes to**: Production PyPI + npm latest
- **Can promote**: Existing RC to stable (same version)

### Patch/Minor/Major Releases
- **Patch**: Bug fixes (v0.1.0 → v0.1.1)
- **Minor**: New features (v0.1.0 → v0.2.0) 
- **Major**: Breaking changes (v0.1.0 → v1.0.0)

## Automated Process

### GitHub Actions Pipeline

When you push a tag, GitHub Actions automatically:

1. **Testing**: Runs comprehensive tests across all platforms
2. **Building**: Creates Python wheels and WASM packages
3. **Publishing**: 
   - RC tags → TestPyPI + npm @rc
   - Stable tags → Production PyPI + npm latest
4. **Releases**: Creates GitHub release draft

### Trusted Publishing

The project uses PyPI trusted publishing (no API tokens needed):
- **TestPyPI**: Environment `dev` for RC releases
- **Production PyPI**: Default environment for stable releases

## Verification

After release, verify installation:

#### Python Packages
```bash
# Release candidates (TestPyPI)
pip install -i https://test.pypi.org/simple/ shlesha==0.1.0rc1

# Stable releases (PyPI)
pip install shlesha==0.1.0

# Test functionality
python -c "import shlesha; print(shlesha.transliterate('धर्म', 'devanagari', 'iso'))"
```

#### WASM Packages
```bash
# Release candidates
npm install shlesha-wasm@rc

# Stable releases  
npm install shlesha-wasm

# Test in Node.js
node -e "const shlesha = require('shlesha-wasm'); console.log(shlesha.transliterate('dharma', 'iso', 'devanagari'))"
```

## Post-Release Tasks

- [ ] Edit and publish GitHub release notes
- [ ] Test installation from PyPI/npm
- [ ] Update documentation if needed
- [ ] Announce release on relevant channels

## Script Reference

### release.sh
**Main release script** - Guides you through the complete process:
- Runs tests
- Updates versions  
- Creates tags
- Provides post-release guidance

### test-release.sh
**Pre-release testing** - Validates release readiness:
- Checks git status
- Tests Rust, Python, and WASM builds
- Runs test suites
- Ensures compatibility

### prepare-release.sh  
**Version management** - Updates version numbers:
- Reads current version from Cargo.toml
- Supports manual or auto-increment
- Creates release preparation commit

### tag-release.sh
**Tag creation** - Handles git tagging:
- Analyzes existing tags
- Determines next version automatically
- Creates appropriate RC or stable tags
- Pushes to trigger CI/CD

## Initial Setup

### PyPI Trusted Publishing

Set up trusted publishers (one-time setup):

1. **TestPyPI** (for RC releases):
   - Repository: `udapaana/shlesha`
   - Workflow: `python.yml` 
   - Environment: `dev`

2. **Production PyPI** (for stable releases):
   - Repository: `udapaana/shlesha`
   - Workflow: `python.yml`
   - Environment: `prd`

### GitHub Environments

The project uses GitHub environments for deployment control:

- **dev** - Release candidates → TestPyPI + npm @rc
- **prd** - Stable releases → Production PyPI + npm latest + crates.io

### Required Secrets

Add these to GitHub environment secrets:

**dev Environment:**
- `NPM_TOKEN` - npm publishing token for RC releases
- `CARGO_REGISTRY_TOKEN` - crates.io token with scopes:
  - `publish-new`, `publish-update`, `yank`

**prd Environment:**  
- `NPM_TOKEN` - npm publishing token for stable releases
- `CARGO_REGISTRY_TOKEN` - crates.io token (same as dev)

## Versioning

We follow [Semantic Versioning](https://semver.org/):
- MAJOR version for incompatible API changes
- MINOR version for backwards-compatible functionality
- PATCH version for backwards-compatible bug fixes

For pre-releases:
- Release Candidate: `v0.1.0-rc1`, `v0.1.0-rc2`, etc.