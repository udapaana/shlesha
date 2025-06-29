# Release Scripts

This directory contains automated scripts for managing Shlesha releases.

## Quick Start

For most releases, simply run:

```bash
./scripts/release.sh
```

This comprehensive script will guide you through the entire release process.

## Individual Scripts

### 🧪 test-release.sh
**Pre-release testing and validation**

Runs comprehensive tests to ensure the codebase is ready for release:
- Checks git status and branch
- Tests Rust core build
- Tests CLI build with features
- Tests Python wheel building
- Tests WASM package building
- Runs all test suites

```bash
./scripts/test-release.sh
```

### 📝 prepare-release.sh
**Version management and preparation**

Updates version numbers and creates preparation commits:
- Reads current version from Cargo.toml
- Supports manual version entry or auto-increment
- Updates Cargo.toml with new version
- Creates release preparation commit

```bash
./scripts/prepare-release.sh
```

Options:
1. Keep current version (just create commit)
2. Set specific version (e.g., 0.1.0-rc1, 0.1.0)
3. Auto-increment based on git tags

### 🏷️ tag-release.sh
**Git tag creation and management**

Creates and pushes release tags with automatic version detection:
- Analyzes existing git tags
- Determines next logical version
- Supports RC and stable release workflows
- Creates annotated tags with proper messages
- Optionally pushes tags to trigger CI/CD

```bash
./scripts/tag-release.sh
```

Supported release types:
1. **Release Candidate (RC)** → TestPyPI + npm @rc
2. **Stable Release** → Production PyPI + npm latest  
3. **Patch Release** → Bug fixes (0.1.0 → 0.1.1)
4. **Minor Release** → New features (0.1.0 → 0.2.0)
5. **Major Release** → Breaking changes (0.1.0 → 1.0.0)

### 🚀 release.sh
**Comprehensive release manager**

Orchestrates the entire release process:
- Runs pre-release tests
- Guides through version updates
- Creates and pushes tags
- Provides post-release guidance

```bash
./scripts/release.sh
```

This script combines all the above scripts into a single, guided workflow.

## Legacy Scripts

### publish-pypi.sh
Manual PyPI publishing script (mostly superseded by GitHub Actions)

### publish-npm.sh  
Manual npm publishing script (mostly superseded by GitHub Actions)

## Workflow

The recommended release workflow:

1. **Development** → Make changes, add tests
2. **Testing** → Run `test-release.sh` to validate
3. **Preparation** → Run `prepare-release.sh` to update versions
4. **Tagging** → Run `tag-release.sh` to create and push tags
5. **Automation** → GitHub Actions handles building and publishing
6. **Post-release** → Edit GitHub release notes and announce

Or simply run `release.sh` for guided assistance through all steps.

## Environment Variables

GitHub Actions uses these environment secrets for automated publishing:

**dev Environment:**
- `NPM_TOKEN` - npm publishing token for RC releases
- `CARGO_REGISTRY_TOKEN` - crates.io token with publish permissions

**prd Environment:**
- `NPM_TOKEN` - npm publishing token for stable releases  
- `CARGO_REGISTRY_TOKEN` - crates.io token with publish permissions

Environment-based deployment:
- **dev environment** - RC releases → TestPyPI + npm @rc + crates.io pre-release
- **prd environment** - Stable releases → PyPI + npm latest + crates.io stable

## Exit Codes

All scripts follow standard exit code conventions:
- `0` - Success
- `1` - General error
- `2` - Misuse (wrong arguments, etc.)

## Safety Features

- **Git status checking** - Warns about uncommitted changes
- **Branch validation** - Recommends running from main branch
- **Tag conflict detection** - Prevents duplicate tags
- **Interactive confirmations** - Requires user approval for destructive actions
- **Dry-run support** - Most scripts show what they'll do before doing it

## Dependencies

Scripts require:
- `git` - Version control operations
- `cargo` - Rust building and testing
- `maturin` - Python wheel building (optional)
- `wasm-pack` - WASM package building (optional)

Optional tools are gracefully handled - scripts will warn but continue if they're missing.