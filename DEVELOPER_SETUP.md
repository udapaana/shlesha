# 🚀 Shlesha Developer Setup Guide

Easy setup for local development of the Shlesha transliteration library.

## 🎯 Quick Start (Recommended)

For new developers, run this single command to set up everything:

```bash
./scripts/quick-start.sh
```

This will:
- ✅ Set up Rust/Python environment  
- ✅ Build all targets (CLI, Python, WASM)
- ✅ Run all tests
- ✅ Show you what to do next

## 📋 Manual Setup

If you prefer to set up step by step:

### 1. Environment Setup
```bash
./scripts/setup-dev.sh
```

### 2. Build All Targets  
```bash
./scripts/build-all.sh
```

### 3. Run Tests
```bash
./scripts/test-all.sh
```

## 🛠️ Available Scripts

| Script | Purpose | Usage |
|--------|---------|-------|
| `quick-start.sh` | Complete setup for new devs | `./scripts/quick-start.sh` |
| `setup-dev.sh` | Environment setup only | `./scripts/setup-dev.sh` |
| `build-all.sh` | Build CLI + Python + WASM | `./scripts/build-all.sh` |
| `test-all.sh` | Run all test suites | `./scripts/test-all.sh` |
| `demo-python.sh` | Interactive Python demo | `./scripts/demo-python.sh` |
| `demo-wasm.sh` | Start WASM web demo | `./scripts/demo-wasm.sh` |
| `fix-wasm-target.sh` | Fix WASM issues | `./scripts/fix-wasm-target.sh` |

## 🎮 Demos

### Python Demo
Interactive command-line demo showing all Python API features:
```bash
./scripts/demo-python.sh
```

### WASM Demo  
Web-based demo with Google Noto fonts:
```bash
./scripts/demo-wasm.sh
# Opens http://localhost:8000/demo.html
```

### CLI Demo
```bash
# Basic transliteration
./target/release/shlesha transliterate --from devanagari --to iast "धर्म"

# With metadata
./target/release/shlesha transliterate --from devanagari --to iast --verbose "धर्मkr"

# List scripts
./target/release/shlesha scripts
```

## 🐛 Troubleshooting

### WASM Target Issues

If you get this error:
```
Error: wasm32-unknown-unknown target not found in sysroot: "/usr/local/Cellar/rust/1.86.0"
```

**Fix 1: Switch to rustup (recommended)**
```bash
./scripts/fix-wasm-target.sh
# Choose option 1
```

**Fix 2: Manual fix for Homebrew Rust**
```bash
# Install rustup alongside Homebrew Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup target add wasm32-unknown-unknown
```

### Python Version Issues

For Python 3.13 compatibility issues:
```bash
# The scripts automatically handle this, but if you see PyO3 errors:
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 uv run maturin develop --features python
```

### uv Installation Issues

If uv is not found:
```bash
# Install uv
curl -LsSf https://astral.sh/uv/install.sh | sh
# Or with Homebrew
brew install uv
```

### Permission Issues

If you get permission errors:
```bash
chmod +x scripts/*.sh
```

## 📦 What Gets Built

After running the build scripts, you'll have:

- **CLI Binary**: `./target/release/shlesha`
- **Python Package**: Installed in uv environment (`uv run python -c "import shlesha"`)  
- **WASM Package**: `./pkg/` directory for web deployment
- **Demos**: Ready-to-run interactive examples

## 🧪 Testing

Run different test suites:

```bash
# All tests
./scripts/test-all.sh

# Individual test suites
cargo test                                    # Rust unit tests
cargo test --test cli_integration_tests      # CLI tests  
cargo test --test comprehensive_bidirectional_tests  # Cross-script tests
uv run pytest python/tests/                 # Python tests (after setup)
```

## 🎯 Development Workflow

1. **First time setup**: `./scripts/quick-start.sh`
2. **Daily development**: 
   - Make changes
   - `./scripts/build-all.sh` (if needed)
   - `./scripts/test-all.sh`
3. **Testing specific features**:
   - CLI: `./scripts/demo-python.sh`
   - WASM: `./scripts/demo-wasm.sh` 
   - Python: `./scripts/demo-python.sh`

## 📝 Requirements

The setup scripts will install/check for:

- **Rust**: Either rustup (recommended) or Homebrew Rust
- **uv**: Modern Python package manager and environment manager
- **wasm-pack**: For WebAssembly builds
- **maturin**: For Python binding builds (installed via uv)

## 🎉 Ready to Develop!

After setup, you'll have a fully functional development environment for:
- 🦀 Rust core library
- 🐍 Python bindings  
- 🕸️ WebAssembly bindings
- ⚡ CLI application

Happy coding! 🎯