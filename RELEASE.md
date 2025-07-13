# Shlesha Release Documentation

## Table of Contents
- [Quick Start](#quick-start)
- [Release Process](#release-process)
- [Initial Setup](#initial-setup)
- [Release Types](#release-types)
- [Version Management](#version-management)
- [Platform Publishing](#platform-publishing)
- [Verification & Testing](#verification--testing)
- [Troubleshooting](#troubleshooting)
- [Architecture & Design](#architecture--design)

## Quick Start

### Automated Release (Recommended)
```bash
# For RC releases (from feature branch)
./scripts/release.sh rc

# For stable releases (from main branch)
./scripts/release.sh stable

# For patch releases
./scripts/release.sh patch
```

The automated script handles:
- Version bumping
- Tag creation
- GitHub Release creation
- CI/CD pipeline triggering
- Multi-platform publishing (PyPI, npm, crates.io)

### Manual Process
If you need to release manually:
```bash
# 1. Update version
./scripts/prepare-release.sh <version>

# 2. Commit and tag
git add -A
git commit -m "chore: release v<version>"
git tag -a v<version> -m "Release v<version>"

# 3. Push to trigger CI/CD
git push origin main --tags
```

## Release Process

### Pre-Release Checklist
- [ ] All tests passing locally
- [ ] Documentation updated
- [ ] CHANGELOG updated
- [ ] Version numbers synchronized across all manifests
- [ ] No uncommitted changes

### Release Workflow

1. **Prepare Release**
   ```bash
   # Updates versions in Cargo.toml, pyproject.toml, package.json
   ./scripts/prepare-release.sh <version>
   ```

2. **Create Release**
   ```bash
   # Creates GitHub release and triggers CI/CD
   ./scripts/release.sh <release-type>
   ```

3. **Monitor Pipeline**
   - Check GitHub Actions for build status
   - Verify all platform publications complete

4. **Post-Release Verification**
   ```bash
   # Verify PyPI
   pip install shlesha==$(cat Cargo.toml | grep version | head -1 | cut -d '"' -f 2)
   
   # Verify npm
   npm view shlesha version
   
   # Verify crates.io
   cargo search shlesha
   ```

## Initial Setup

### GitHub Configuration

1. **Environments**
   - Create `pypi` environment in Settings → Environments
   - Create `npm` environment
   - Add required reviewers if needed

2. **Secrets**
   ```yaml
   NPM_TOKEN: npm access token with publish permissions
   CARGO_REGISTRY_TOKEN: crates.io API token
   ```

3. **PyPI Trusted Publishing**
   - Configure in PyPI project settings
   - Add GitHub Actions as trusted publisher
   - Repository: `udapaana/shlesha`
   - Workflow: `release.yml`
   - Environment: `pypi`

### Local Development Setup

```bash
# Install all dependencies
./scripts/setup-dev.sh

# Verify setup
./scripts/test-all.sh
```

## Release Types

### RC (Release Candidate)
- Created from feature branches
- Format: `X.Y.Z-rc.N`
- For testing before stable release
- Published to all platforms with RC tag

### Stable Release
- Created from main branch only
- Format: `X.Y.Z`
- Full production release
- Triggers all platform publications

### Patch Release
- For bug fixes: `X.Y.Z` → `X.Y.(Z+1)`
- Must be backward compatible
- Created from main branch

### Minor Release
- For new features: `X.Y.Z` → `X.(Y+1).0`
- Backward compatible
- Created from main branch

### Major Release
- For breaking changes: `X.Y.Z` → `(X+1).0.0`
- May break backward compatibility
- Requires migration guide

## Version Management

### Version Synchronization
All version numbers must be synchronized across:
- `Cargo.toml` - Rust/Cargo version
- `pyproject.toml` - Python package version
- `package.json` - npm package version

### Version Format
- Follow Semantic Versioning (SemVer)
- Format: `MAJOR.MINOR.PATCH[-PRERELEASE]`
- Examples: `0.2.0`, `1.0.0-rc.1`, `2.1.3`

## Platform Publishing

### PyPI (Python)
- **Method**: Trusted Publishing via GitHub Actions
- **Package**: `shlesha`
- **Verification**: `pip install shlesha==VERSION`

### npm (JavaScript/WASM)
- **Method**: npm token authentication
- **Package**: `shlesha`
- **Verification**: `npm view shlesha version`

### crates.io (Rust)
- **Method**: Cargo token authentication
- **Package**: `shlesha`
- **Verification**: `cargo search shlesha`

## Verification & Testing

### Pre-Release Testing
```bash
# Run all tests
./scripts/test-all.sh

# Test release build
./scripts/test-release.sh

# Verify wheel building
./scripts/pre-release-check.sh
```

### Post-Release Verification
```bash
# Test PyPI installation
pip install shlesha==$(cat Cargo.toml | grep version | head -1 | cut -d '"' -f 2)
python -c "import shlesha; print(shlesha.__version__)"

# Test npm installation
npm install shlesha@latest
node -e "console.log(require('shlesha').version)"

# Test cargo installation
cargo install shlesha
shlesha --version
```

## Troubleshooting

### Common Issues

1. **Version Mismatch**
   ```bash
   # Fix: Re-run version sync
   ./scripts/prepare-release.sh <version>
   ```

2. **PyPI Upload Fails**
   - Check trusted publishing configuration
   - Verify environment name matches
   - Check PyPI project settings

3. **npm Publish Fails**
   - Verify NPM_TOKEN is set
   - Check token permissions
   - Ensure not publishing duplicate version

4. **Build Failures**
   - Check Rust toolchain version
   - Verify all dependencies resolved
   - Run `cargo clean` and retry

### Recovery Procedures

1. **Failed Release**
   ```bash
   # Delete local tag
   git tag -d v<version>
   
   # Delete remote tag (if pushed)
   git push origin :refs/tags/v<version>
   
   # Fix issues and retry
   ./scripts/release.sh <type>
   ```

2. **Partial Publication**
   - Check which platforms succeeded
   - Manually publish to failed platforms
   - Update GitHub release notes

## Architecture & Design

### Release Pipeline

```
Developer → GitHub → CI/CD → Platform Publishers
    |         |         |            |
    |         |         |            ├── PyPI
    |         |         |            ├── npm  
    |         |         |            └── crates.io
    |         |         |
    |         |         └── Tests, Builds, Checks
    |         |
    |         └── Release Creation, Tagging
    |
    └── Version Update, Commit
```

### Security Model

1. **Authentication**
   - PyPI: Trusted Publishing (no tokens)
   - npm: Secure token in GitHub Secrets
   - crates.io: API token in GitHub Secrets

2. **Authorization**
   - Release environment protection
   - Required reviewers for production
   - Branch protection on main

3. **Audit Trail**
   - All releases tagged in git
   - GitHub releases for changelog
   - Package registry history

### Maintenance

#### Regular Tasks
- Update dependencies monthly
- Review and rotate tokens quarterly
- Audit release permissions
- Monitor for security advisories

#### Scripts Reference
- `release.sh` - Main release automation
- `prepare-release.sh` - Version synchronization
- `test-release.sh` - Release validation
- `pre-release-check.sh` - Pre-flight checks
- `publish-pypi.sh` - Manual PyPI publish
- `publish-npm.sh` - Manual npm publish