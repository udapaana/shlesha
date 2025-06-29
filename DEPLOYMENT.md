# Deployment & Release Documentation

This document covers the complete deployment and release process for Shlesha, including environment setup, automation, and manual procedures.

## 🚀 Quick Release Guide

### Automated Release (Recommended)
```bash
# Complete guided release process
./scripts/release.sh
```

### Manual Step-by-Step
```bash
# 1. Test everything
./scripts/test-release.sh

# 2. Update version (optional)
./scripts/prepare-release.sh  

# 3. Create and push tags
./scripts/tag-release.sh
```

## 🏗️ Environment Setup

### GitHub Environments

The project uses two GitHub environments for deployment:

#### **dev** Environment
- **Purpose**: Release candidate (RC) testing
- **Triggers**: Tags containing 'rc' (e.g., `v0.1.0-rc.1`)
- **Deploys to**:
  - TestPyPI (Python packages)
  - npm with `@rc` tag (WASM packages)
  - crates.io pre-release (Rust packages)
- **Protection**: None (auto-deploy)

#### **prd** Environment  
- **Purpose**: Production stable releases
- **Triggers**: Stable version tags (e.g., `v0.1.0`)
- **Deploys to**:
  - Production PyPI (Python packages)
  - npm with `@latest` tag (WASM packages)
  - crates.io stable (Rust packages)
- **Protection**: Deployment protection rules (optional)

### Required Secrets

Configure these secrets in your GitHub environments:

#### dev Environment Secrets
```
NPM_TOKEN                 # npm publishing token for RC releases
CARGO_REGISTRY_TOKEN      # crates.io publishing token
```

#### prd Environment Secrets  
```
NPM_TOKEN                 # npm publishing token for stable releases
CARGO_REGISTRY_TOKEN      # crates.io publishing token
```

#### Token Scopes
The `CARGO_REGISTRY_TOKEN` should have these scopes:
- `publish-new` - Publish new crates
- `publish-update` - Update existing crates  
- `yank` - Yank/unlist versions

#### Security Benefits
- **Token isolation**: Separate npm tokens for RC vs stable releases
- **Environment protection**: Different access controls per environment
- **Audit trail**: Clear separation of RC and production deployments

## 📦 Publishing Targets

### Python Packages (PyPI)

#### TestPyPI (RC Releases)
- **Trigger**: RC tags (`v*-rc*`)
- **Environment**: `dev`
- **Authentication**: OIDC Trusted Publisher
- **Repository**: https://test.pypi.org/legacy/
- **Installation**: `pip install -i https://test.pypi.org/simple/ shlesha==0.1.0rc1`

#### Production PyPI (Stable Releases)
- **Trigger**: Stable tags (`v*` without `rc`)
- **Environment**: `prd` 
- **Authentication**: OIDC Trusted Publisher
- **Repository**: https://upload.pypi.org/legacy/
- **Installation**: `pip install shlesha==0.1.0`

### WASM Packages (npm)

#### Release Candidate
- **Trigger**: RC tags (`v*-rc*`)
- **Tag**: `@rc`
- **Installation**: `npm install shlesha-wasm@rc`

#### Stable Release
- **Trigger**: Stable tags (`v*` without `rc`)
- **Tag**: `@latest` (default)
- **Installation**: `npm install shlesha-wasm`

### Rust Packages (crates.io)

#### Release Candidate
- **Trigger**: RC tags (`v*-rc*`)
- **Environment**: `dev`
- **Authentication**: `CARGO_REGISTRY_TOKEN`
- **Installation**: Add to `Cargo.toml`: `shlesha = "0.1.0-rc.1"`
- **Cargo add**: `cargo add shlesha@0.1.0-rc.1`

#### Stable Release
- **Trigger**: Stable tags (`v*` without `rc`)
- **Environment**: `prd`
- **Authentication**: `CARGO_REGISTRY_TOKEN`
- **Installation**: Add to `Cargo.toml`: `shlesha = "0.1.0"`
- **Cargo add**: `cargo add shlesha@0.1.0`

## 🔄 Release Workflow

### 1. Development Phase
```bash
# Make changes, add features, fix bugs
git add .
git commit -m "feat: add new transliteration feature"
git push origin main
```

### 2. Pre-Release Testing  
```bash
# Run comprehensive tests
./scripts/test-release.sh

# ✅ Validates:
# - Git working directory status
# - Rust core and CLI builds
# - Python wheel building  
# - WASM package building
# - All test suites pass
```

### 3. Version Management
```bash
# Update version numbers
./scripts/prepare-release.sh

# Options:
# 1. Keep current version (just create commit)
# 2. Set specific version (e.g., 0.1.0-rc1)
# 3. Auto-increment based on git tags
```

### 4. Release Candidate
```bash
# Create RC tag for testing
./scripts/tag-release.sh

# Select: Release candidate (RC)
# Creates: v0.1.0-rc1
# Triggers: dev environment deployment
```

### 5. RC Validation
```bash
# Test RC packages
pip install -i https://test.pypi.org/simple/ shlesha==0.1.0rc1
npm install shlesha-wasm@rc
cargo add shlesha@0.1.0-rc.1

# Run integration tests
python -c "import shlesha; print(shlesha.transliterate('धर्म', 'devanagari', 'iso'))"
```

### 6. Stable Release
```bash
# Promote RC to stable or create new stable
./scripts/tag-release.sh

# Select: Stable release
# Creates: v0.1.0  
# Triggers: prd environment deployment
```

### 7. Post-Release
```bash
# Verify installations
pip install shlesha==0.1.0
npm install shlesha-wasm
cargo add shlesha@0.1.0

# Update GitHub release notes
# Announce release
```

## 🔐 Security & Authentication

### PyPI Trusted Publishing

**Setup** (one-time per environment):

1. **TestPyPI** (dev environment):
   - Go to https://test.pypi.org/manage/account/publishing/
   - Add publisher:
     - Repository: `udapaana/shlesha`
     - Workflow: `python.yml`
     - Environment: `dev`

2. **Production PyPI** (prd environment):
   - Go to https://pypi.org/manage/account/publishing/
   - Add publisher:
     - Repository: `udapaana/shlesha`
     - Workflow: `python.yml`  
     - Environment: `prd`

### npm Authentication

**Setup**:
1. Create npm account at https://www.npmjs.com
2. Generate access token (Classic Token with Publish access)
3. Add as `NPM_TOKEN` repository secret

### crates.io Authentication

**Setup**:
1. Login to https://crates.io with GitHub
2. Generate API token at https://crates.io/settings/tokens
3. Ensure token has required scopes:
   - `publish-new`
   - `publish-update` 
   - `yank`
4. Add as `CARGO_REGISTRY_TOKEN` repository secret

## 📊 Monitoring & Verification

### GitHub Actions
Monitor builds at: https://github.com/udapaana/shlesha/actions

### Package Registries
- **PyPI**: https://pypi.org/project/shlesha/
- **TestPyPI**: https://test.pypi.org/project/shlesha/
- **npm**: https://www.npmjs.com/package/shlesha-wasm
- **crates.io**: https://crates.io/crates/shlesha

### Installation Testing
```bash
# Python (production)
pip install shlesha
python -c "import shlesha; print(shlesha.__version__)"

# Python (test)  
pip install -i https://test.pypi.org/simple/ shlesha
python -c "import shlesha; print(shlesha.__version__)"

# WASM (stable)
npm install shlesha-wasm
node -e "const s = require('shlesha-wasm'); console.log(s.getVersion())"

# WASM (RC)
npm install shlesha-wasm@rc
node -e "const s = require('shlesha-wasm'); console.log(s.getVersion())"

# Rust (stable)
cargo add shlesha@0.1.0

# Rust (RC)
cargo add shlesha@0.1.0-rc.1
```

## 🚨 Troubleshooting

### Common Issues

#### PyPI Upload Failures
```bash
# Check trusted publisher configuration
# Verify environment name matches workflow
# Ensure OIDC permissions are set correctly
```

#### npm Publish Failures
```bash
# Check NPM_TOKEN validity
npm whoami
# Verify package name availability
npm info shlesha-wasm
```

#### crates.io Publish Failures
```bash
# Check token scopes
# Verify version doesn't already exist
# Check for dependency issues
```

#### Build Failures
```bash
# Run local tests first
./scripts/test-release.sh

# Check specific build logs in GitHub Actions
# Verify all dependencies are available
```

### Recovery Procedures

#### Failed RC Release
```bash
# Delete bad RC tag
git tag -d v0.1.0-rc1
git push origin :refs/tags/v0.1.0-rc1

# Fix issues and create new RC
./scripts/tag-release.sh
```

#### Failed Stable Release
```bash
# Cannot delete published packages
# Create patch version instead
./scripts/tag-release.sh  # Select patch release
```

## 📋 Checklist Templates

### Pre-Release Checklist
- [ ] All tests pass locally
- [ ] Documentation updated
- [ ] CHANGELOG.md updated  
- [ ] Version numbers consistent
- [ ] No pending security issues

### RC Release Checklist
- [ ] RC tag created and pushed
- [ ] TestPyPI package published
- [ ] npm RC package published
- [ ] Integration tests pass
- [ ] No critical issues found

### Stable Release Checklist  
- [ ] Stable tag created and pushed
- [ ] Production PyPI package published
- [ ] npm stable package published
- [ ] crates.io package published
- [ ] GitHub release notes updated
- [ ] Installation verified across platforms
- [ ] Release announced

## 🔗 Related Documentation

- [RELEASE.md](./RELEASE.md) - Release process overview
- [scripts/README.md](./scripts/README.md) - Script documentation
- [GitHub Actions Workflows](./.github/workflows/) - CI/CD pipeline definitions