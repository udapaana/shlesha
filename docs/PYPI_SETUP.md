# PyPI Setup Guide for Shlesha

## Quick Setup Steps

### ✅ COMPLETED: Trusted Publishers

You've already set up trusted publishers, which is the recommended and most secure method:

- **Production PyPI**: `udapaana/shlesha` → workflow `python.yml` 
- **Test PyPI**: `udapaana/shlesha` → workflow `python.yml` → environment `dev`

No API tokens needed! GitHub Actions will authenticate automatically using OpenID Connect (OIDC).

### 5. Test Your Setup

Run the check script:
```bash
./scripts/check-tokens.sh
```

## Publishing Your First Release Candidate

### 1. Update Version

Edit `Cargo.toml`:
```toml
version = "0.1.0-rc1"
```

### 2. Create and Push Tag

```bash
# Commit your changes
git add -A
git commit -m "chore: prepare v0.1.0-rc1 release"

# Create tag
git tag -a v0.1.0-rc1 -m "Release candidate: initial PyPI package"

# Push changes and tag
git push origin main
git push origin v0.1.0-rc1
```

### 3. Automatic Publishing

GitHub Actions will automatically:
- Build wheels for multiple platforms
- Publish to Test PyPI (for rc tags)
- Publish to npm with 'rc' tag

### 4. Test Installation

```bash
# Install from TestPyPI
pip install -i https://test.pypi.org/simple/ shlesha==0.1.0rc1

# Install WASM from npm
npm install shlesha-wasm@rc

# Test it
python -c "import shlesha; print(shlesha.transliterate('धर्म', 'devanagari', 'iso'))"
```

### 5. Stable Release

After testing the RC:

```bash
# Update to stable version
# Edit Cargo.toml: version = "0.1.0"
git commit -am "chore: prepare v0.1.0 stable release"
git tag -a v0.1.0 -m "Stable release"
git push origin main && git push origin v0.1.0
```

This will publish to production PyPI and npm.

## Workflow Details

- **Workflow Name**: `Python`
- **Triggers**: 
  - Push to main
  - Version tags (v*)
  - Pull requests
- **Environment**: `test-pypi` for beta/alpha, `pypi` for production

## Troubleshooting

### "Package already exists" Error
- TestPyPI doesn't allow re-uploading the same version
- Increment the beta number: `v0.1.0-beta.2`

### Token Not Working
- Make sure you copied the entire token including `pypi-` prefix
- Tokens are shown only once - generate a new one if lost
- Check token scope - should be "Entire account" initially

### Build Failing
- Check GitHub Actions logs
- Ensure all tests pass locally first: `cargo test --all-features`
- Verify Python tests: `maturin develop && pytest python/tests`

## Next Steps

Once beta testing is complete:
1. Create a production PyPI account
2. Generate production token
3. Add `PYPI_API_TOKEN` to secrets
4. Tag a release without 'beta': `v0.1.0`
5. Package will auto-publish to production PyPI