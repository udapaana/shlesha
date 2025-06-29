# crates.io Release Candidate Support

This document explains how Shlesha implements release candidate (RC) support for crates.io publishing.

## 🎯 Overview

**Yes, crates.io absolutely supports release candidates!** crates.io follows Semantic Versioning (SemVer) which includes pre-release versions. This allows us to publish RC versions for testing before stable releases.

## 📦 Pre-release Version Format

crates.io uses SemVer pre-release identifiers:

```
Stable:  1.0.0
RC:      1.0.0-rc.1, 1.0.0-rc.2, etc.
Beta:    1.0.0-beta.1, 1.0.0-beta.2, etc.
Alpha:   1.0.0-alpha.1, 1.0.0-alpha.2, etc.
```

**Shlesha uses the `rc.N` format** for consistency with industry standards.

## 🔄 Release Workflow

### Development Environment Matrix

| Release Type | PyPI Target | npm Tag | crates.io | Environment | Secrets |
|--------------|-------------|---------|-----------|-------------|---------|
| RC | TestPyPI | `@rc` | Pre-release | `dev` | `NPM_TOKEN`, `CARGO_REGISTRY_TOKEN` |
| Stable | Production PyPI | `@latest` | Stable | `prd` | `NPM_TOKEN`, `CARGO_REGISTRY_TOKEN` |

### Automatic Publishing

When you create tags:

```bash
# RC Release
git tag v0.1.0-rc.1
git push origin v0.1.0-rc.1

# Triggers:
# ✅ TestPyPI: shlesha==0.1.0rc1
# ✅ npm: shlesha-wasm@rc  
# ✅ crates.io: shlesha = "0.1.0-rc.1"

# Stable Release  
git tag v0.1.0
git push origin v0.1.0

# Triggers:
# ✅ PyPI: shlesha==0.1.0
# ✅ npm: shlesha-wasm (latest)
# ✅ crates.io: shlesha = "0.1.0"
```

## 📋 Installation Methods

### Using RC Versions

```toml
# Cargo.toml - Explicit RC version
[dependencies]
shlesha = "0.1.0-rc.1"
```

```bash
# Command line
cargo add shlesha@0.1.0-rc.1
```

### Using Stable Versions

```toml
# Cargo.toml - Stable version
[dependencies]  
shlesha = "0.1.0"
```

```bash
# Command line
cargo add shlesha@0.1.0
```

## ⚠️ Important Behaviors

### Pre-release Opt-in Required

**Key Point**: Pre-release versions are NOT automatically selected by Cargo unless explicitly requested.

```toml
# This will NOT match RC versions
shlesha = "0.1"         # Only matches stable 0.1.x releases

# This WILL match RC versions  
shlesha = "0.1.0-rc.1"  # Explicit RC version
```

### Version Update Rules

- **RC to RC**: `0.1.0-rc.1` can update to `0.1.0-rc.2`
- **RC to Stable**: `0.1.0-rc.1` can update to `0.1.0` 
- **Cross-version**: `0.1.0-rc.1` CANNOT update to `0.1.1-rc.1`

### Precedence Order

```
0.1.0-alpha.1 < 0.1.0-beta.1 < 0.1.0-rc.1 < 0.1.0
```

## 🔧 GitHub Actions Configuration

### RC Publishing (dev environment)

```yaml
publish-crates-rc:
  name: Publish to crates.io (RC)
  runs-on: ubuntu-latest
  if: ${{ contains(github.ref, 'rc') }}
  environment: dev
  steps:
    - name: Publish RC to crates.io
      run: cargo publish --token ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

### Stable Publishing (prd environment)

```yaml
publish-crates:
  name: Publish to crates.io (Stable)
  runs-on: ubuntu-latest
  if: ${{ !contains(github.ref, 'rc') }}
  environment: prd
  steps:
    - name: Publish to crates.io
      run: cargo publish --token ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

## 🔐 Enhanced Security Model

### Environment Isolation

The improved setup provides **enhanced security through environment separation**:

#### Token Separation
- **dev environment**: Separate `NPM_TOKEN` for RC releases
- **prd environment**: Separate `NPM_TOKEN` for stable releases
- **Both environments**: Same `CARGO_REGISTRY_TOKEN` (crates.io doesn't have separate RC/stable tokens)

#### Security Benefits
1. **Blast radius limitation**: Compromised RC token can't affect stable releases
2. **Access control**: Different team members can have different environment access
3. **Audit trails**: Clear separation between RC and production deployments
4. **Rollback capability**: Can disable RC publishing without affecting stable releases

#### Deployment Protection
- **dev environment**: Can have minimal protection for fast RC iteration
- **prd environment**: Can require approvals, reviews, or timing restrictions

## 🎨 Version Management Scripts

### Automatic RC Detection

Our release scripts automatically handle RC versioning:

```bash
# Tag Release Script
./scripts/tag-release.sh

# Options include:
1) Release candidate (RC) - for TestPyPI + crates.io
6) Minor RC release (new features, testing) 
7) Major RC release (breaking changes, testing)
```

### Version Preparation

```bash
# Prepare Release Script
./scripts/prepare-release.sh

# Suggests proper RC formats:
# v0.1.0-rc.1, v0.1.0-rc.2, etc.
```

## ✅ Testing RC Releases

### Installation Testing

```bash
# Create new project to test RC
cargo new test_shlesha_rc
cd test_shlesha_rc

# Add RC dependency
cargo add shlesha@0.1.0-rc.1

# Test functionality
echo 'fn main() { 
    let s = shlesha::Shlesha::new();
    println!("{}", s.transliterate("test", "iast", "devanagari").unwrap());
}' > src/main.rs

cargo run
```

### Dependency Resolution

```bash
# Check what version Cargo resolved
cargo tree | grep shlesha

# Expected output:
# └── shlesha v0.1.0-rc.1
```

## 🚫 Common Pitfalls

### 1. Forgetting Explicit RC Version

```toml
# ❌ Won't work - doesn't match RCs
shlesha = "0.1"

# ✅ Works - explicit RC version
shlesha = "0.1.0-rc.1"
```

### 2. Mixing RC and Stable Dependencies

```toml
# ⚠️ Be careful with mixed pre-release usage
shlesha = "0.1.0-rc.1"       # RC version
some_other_crate = "1.0"     # Stable version using shlesha
```

### 3. Version Requirement Conflicts

```toml
# ❌ This creates conflicts
shlesha = "0.1.0-rc.1"
other_crate = "1.0"  # If other_crate requires shlesha = "0.1"
```

## 📈 Best Practices

### For Library Authors

1. **Test RCs thoroughly** before promoting to stable
2. **Document breaking changes** between RC versions
3. **Use RC versions** for your own dependencies during development
4. **Avoid breaking changes** between RC and stable of same version

### For Library Users

1. **Pin exact RC versions** for reproducible builds
2. **Test RC versions** in isolated environments
3. **Update to stable** as soon as available
4. **Report issues** found in RC versions

## 🔗 Resources

- [Semantic Versioning Specification](https://semver.org/)
- [Cargo Book: Specifying Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
- [crates.io Documentation](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Rust Pre-release Discussion](https://users.rust-lang.org/t/alpha-beta-rc-with-cargo-semantic-versioning/69799)

## 🎉 Summary

crates.io's RC support enables:

- ✅ **Comprehensive testing** before stable releases
- ✅ **Multi-platform coordination** (PyPI, npm, crates.io all support RCs)
- ✅ **Safe dependency management** with explicit opt-in
- ✅ **Professional release workflow** following industry standards

The Shlesha release system now fully leverages this capability across all three package ecosystems!