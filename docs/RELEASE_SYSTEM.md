# Release System Overview

This document provides a high-level overview of the Shlesha automated release system.

## 🎯 System Goals

1. **Automated Versioning** - Intelligent tag incrementing based on git history
2. **Multi-Target Publishing** - Python (PyPI), WASM (npm), Rust (crates.io)
3. **Environment Isolation** - Separate dev/prd environments for RC/stable releases
4. **Safety & Validation** - Comprehensive testing before release
5. **Developer Experience** - Simple, guided release process

## 🏗️ Architecture

### Release Scripts
```
scripts/
├── release.sh           # 🚀 Main orchestrator - guided workflow
├── test-release.sh      # 🧪 Pre-release validation 
├── prepare-release.sh   # 📝 Version management
├── tag-release.sh       # 🏷️  Tag creation and pushing
├── publish-pypi.sh      # 🐍 Manual PyPI (legacy)
├── publish-npm.sh       # 📦 Manual npm (legacy)
└── README.md           # 📚 Script documentation
```

### GitHub Environments
```
udapaana/shlesha repository:
├── dev environment      # RC releases
│   ├── TestPyPI publishing
│   └── npm @rc tag
└── prd environment      # Stable releases
    ├── Production PyPI
    ├── npm @latest
    └── crates.io
```

### CI/CD Workflows
```
.github/workflows/
├── ci.yml              # Core testing & validation
├── python.yml          # Python builds & PyPI publishing
├── wasm.yml            # WASM builds & npm publishing
└── release.yml         # CLI binaries & crates.io
```

## 🔄 Release Flow

### 1. Development
```bash
# Regular development cycle
git add . && git commit -m "feat: new feature"
git push origin main
```

### 2. Release Candidate
```bash
# Create RC for testing
./scripts/release.sh
# → Creates v0.1.0-rc1
# → Publishes to TestPyPI + npm @rc
```

### 3. Validation
```bash
# Test RC packages
pip install -i https://test.pypi.org/simple/ shlesha==0.1.0rc1
npm install shlesha-wasm@rc
```

### 4. Stable Release
```bash
# Promote to stable
./scripts/tag-release.sh
# → Creates v0.1.0
# → Publishes to PyPI + npm + crates.io
```

## 🔐 Security Model

### Authentication Methods

| Target | Method | Environment | Secrets |
|--------|--------|-------------|---------|
| TestPyPI | OIDC Trusted Publisher | dev | None (OIDC) |
| PyPI | OIDC Trusted Publisher | prd | None (OIDC) |
| npm (RC) | API Token | dev | `NPM_TOKEN` |
| npm (stable) | API Token | prd | `NPM_TOKEN` |
| crates.io (RC) | API Token | dev | `CARGO_REGISTRY_TOKEN` |
| crates.io (stable) | API Token | prd | `CARGO_REGISTRY_TOKEN` |

### Access Control
- **dev environment**: Automatic deployment for RC tags
- **prd environment**: Automatic deployment for stable tags
- **Protection rules**: Optional approval workflows
- **Scoped tokens**: Minimal required permissions

## 📊 Version Management

### Tag Strategy
```
v0.1.0-rc.1  → Release candidate 1
v0.1.0-rc.2  → Release candidate 2
v0.1.0       → Stable release
v0.1.1       → Patch release
v0.2.0       → Minor release
v1.0.0       → Major release
```

### Auto-Increment Logic
```bash
# From existing: v0.1.0-rc.1
./scripts/tag-release.sh
Options:
1. Next RC     → v0.1.0-rc.2
2. Stable      → v0.1.0
3. Patch       → v0.1.1
4. Minor       → v0.2.0
5. Major       → v1.0.0
6. Minor RC    → v0.2.0-rc.1
7. Major RC    → v1.0.0-rc.1
```

## 🎛️ Control Points

### Pre-Release Gates
- ✅ All tests pass
- ✅ Code formatting
- ✅ Linting checks
- ✅ Security audit
- ✅ Build validation

### Environment Gates
- ✅ RC testing on TestPyPI
- ✅ Integration validation
- ✅ Manual approval (optional)
- ✅ Stable release approval

### Post-Release Validation
- ✅ Package installation tests
- ✅ Functionality verification
- ✅ Documentation updates
- ✅ Release announcement

## 🚀 Package Distribution

### Python Packages
```
TestPyPI (RC):  pip install -i https://test.pypi.org/simple/ shlesha==0.1.0rc1
PyPI (Stable):  pip install shlesha==0.1.0
```

### WASM Packages  
```
npm (RC):       npm install shlesha-wasm@rc
npm (Stable):   npm install shlesha-wasm
```

### Rust Packages
```
crates.io (RC):     shlesha = "0.1.0-rc.1"  # Cargo.toml
crates.io (stable): shlesha = "0.1.0"       # Cargo.toml
```

### CLI Binaries
```
GitHub Releases: Pre-built binaries for:
- Linux (x86_64)
- macOS (x86_64, ARM64)  
- Windows (x86_64)
```

## 🔧 Maintenance

### Regular Tasks
- Monitor CI/CD pipeline health
- Update dependencies
- Rotate authentication tokens
- Review security configurations

### Emergency Procedures
- Yank problematic releases
- Emergency patches
- Rollback mechanisms
- Security incident response

### Performance Optimization
- Build time optimization
- Package size reduction
- Parallel publishing
- Caching strategies

## 📈 Metrics & Monitoring

### Success Metrics
- Release frequency
- Time to release
- Test coverage
- Package download counts

### Error Tracking
- Build failures
- Publishing failures  
- Test failures
- Security vulnerabilities

### Performance Metrics
- Build duration
- Package sizes
- Installation time
- Memory usage

## 🔗 Documentation Links

- [DEPLOYMENT.md](../DEPLOYMENT.md) - Complete deployment guide
- [RELEASE.md](../RELEASE.md) - Release process overview  
- [scripts/README.md](../scripts/README.md) - Script documentation
- [GitHub Actions](.github/workflows/) - CI/CD definitions

## 🎓 Best Practices

### For Developers
1. Run `./scripts/test-release.sh` before any release
2. Use RC releases for testing major changes
3. Keep CHANGELOG.md updated
4. Test packages before promoting to stable

### For Maintainers  
1. Monitor GitHub Actions for failures
2. Verify trusted publisher configurations
3. Keep secrets up to date
4. Review security configurations regularly

### For Contributors
1. Understand the release flow before contributing
2. Test locally before submitting PRs
3. Follow semantic versioning guidelines
4. Update documentation with changes