# 📚 Shlesha Documentation Index

Complete guide to all Shlesha documentation and resources.

## 🎯 Quick Navigation

| Document | Purpose | Audience |
|----------|---------|----------|
| [README.md](../README.md) | Project overview & quick start | All users |
| [CHANGELOG.md](../CHANGELOG.md) | Version history & changes | All users |
| [CONTRIBUTING.md](guides/CONTRIBUTING.md) | Contribution guidelines | Contributors |
| [DEVELOPER_SETUP.md](guides/DEVELOPER_SETUP.md) | Development environment setup | Developers |
| [API_REFERENCE.md](reference/API_REFERENCE.md) | Complete API documentation | API users |
| [SCHEMA_REFERENCE.md](reference/SCHEMA_REFERENCE.md) | YAML schema format guide | Schema authors |
| [PERFORMANCE.md](architecture/PERFORMANCE.md) | Performance guide & benchmarks | Performance engineers |
| [ARCHITECTURE.md](architecture/ARCHITECTURE.md) | System design & architecture | Architects |
| [BINDINGS.md](reference/BINDINGS.md) | Language bindings guide | Integration developers |

## 📖 Documentation Structure

### 🚀 Getting Started
1. **[README.md](../README.md)** - Start here
   - Project overview
   - Supported scripts
   - Basic usage examples
   - Quick start command

2. **[CHANGELOG.md](../CHANGELOG.md)** - Version history
   - Recent changes and fixes
   - Breaking changes
   - Migration notes

3. **[CONTRIBUTING.md](guides/CONTRIBUTING.md)** - Contribution guidelines
   - Development workflow
   - Adding new scripts
   - Code standards
   - Testing requirements

4. **[DEVELOPER_SETUP.md](guides/DEVELOPER_SETUP.md)** - Environment setup
   - One-command setup (`./scripts/quick-start.sh`)
   - Manual setup instructions
   - Troubleshooting guides
   - Development workflow

### 🔧 Technical Reference
5. **[API_REFERENCE.md](reference/API_REFERENCE.md)** - Complete API docs
   - Rust native API
   - Python bindings API
   - WASM/JavaScript API
   - CLI interface
   - Error handling
   - Integration examples

6. **[SCHEMA_REFERENCE.md](reference/SCHEMA_REFERENCE.md)** - Schema format guide
   - YAML schema structure
   - Roman vs Indic script schemas
   - Complete examples
   - Validation rules
   - Best practices

7. **[PERFORMANCE.md](architecture/PERFORMANCE.md)** - Performance guide
   - Benchmarking methodology
   - Performance metrics
   - Optimization strategies
   - Profiling tools
   - Tuning recommendations

8. **[ARCHITECTURE.md](architecture/ARCHITECTURE.md)** - System design
   - Hub-and-spoke architecture
   - Performance optimizations
   - Module structure
   - Design decisions

9. **[BINDINGS.md](reference/BINDINGS.md)** - Language bindings
   - Python integration details
   - WASM deployment guide
   - CLI usage patterns
   - Cross-platform considerations

## 🛠️ Development Resources

### Scripts & Tools
- **`./scripts/quick-start.sh`** - Complete environment setup
- **`./scripts/build-all.sh`** - Build all targets
- **`./scripts/test-all.sh`** - Run all test suites
- **`./scripts/demo-python.sh`** - Interactive Python demo
- **`./scripts/demo-wasm.sh`** - Web-based WASM demo

### Test Suites
- **Unit Tests**: `cargo test` - 193 Rust tests
- **CLI Tests**: `cargo test --test cli_integration_tests` - 12 CLI tests  
- **Bidirectional Tests**: `cargo test --test comprehensive_bidirectional_tests` - 5 comprehensive tests
- **Python Tests**: Built-in validation + comprehensive test suite
- **WASM Tests**: Node.js and browser testing

### Demo Applications
- **Python Demo**: Interactive CLI showcasing all Python API features
- **WASM Demo**: Web application with Google Noto fonts (`demo.html`)
- **CLI Examples**: Command-line usage patterns

## 🎓 Learning Path

### For New Users
1. Read [README.md](README.md) for overview
2. Run `./scripts/quick-start.sh` to set up environment
3. Try the demos:
   - `./scripts/demo-python.sh` for Python
   - `./scripts/demo-wasm.sh` for web
   - CLI examples in [API_REFERENCE.md](API_REFERENCE.md)

### For Developers
1. Complete [DEVELOPER_SETUP.md](guides/DEVELOPER_SETUP.md)
2. Study [ARCHITECTURE.md](architecture/ARCHITECTURE.md) for system design
3. Review [API_REFERENCE.md](reference/API_REFERENCE.md) for implementation details
4. Run test suites with `./scripts/test-all.sh`

### For Integrators
1. Choose your platform in [API_REFERENCE.md](reference/API_REFERENCE.md)
2. Follow integration examples
3. Review [BINDINGS.md](reference/BINDINGS.md) for platform-specific details
4. Test with provided demo applications

## 🧩 Module Documentation

### Core Modules
- **Hub Module** (`src/modules/hub/`) - Central conversion engine
- **Script Converters** (`src/modules/script_converter/`) - Individual script implementations
- **Unknown Handler** (`src/modules/core/unknown_handler.rs`) - Metadata collection
- **Registry** (`src/modules/registry/`) - Schema management

### API Bindings
- **Python** (`src/python_bindings.rs`) - PyO3-based Python API
- **WASM** (`src/wasm_bindings.rs`) - WebAssembly JavaScript API
- **CLI** (`src/main.rs`) - Command-line interface

## 📋 Reference Materials

### Supported Scripts
- **Indic Scripts**: Devanagari, Bengali, Tamil, Telugu, Gujarati, Kannada, Malayalam, Odia
- **Roman Schemes**: IAST, ITRANS, SLP1, Harvard-Kyoto, Velthuis, WX, ISO-15919

### Schema Format
- YAML-based script definitions
- Examples in `schemas/` directory
- Runtime loading supported

### Performance Data
- Hub-and-spoke architecture for O(1) script addition
- Zero-overhead metadata collection (when not requested)
- Comprehensive benchmarking suite (upcoming)

## 🎯 Quick Commands

```bash
# Complete setup
./scripts/quick-start.sh

# Build everything
./scripts/build-all.sh

# Test everything
./scripts/test-all.sh

# Try demos
./scripts/demo-python.sh
./scripts/demo-wasm.sh

# Basic CLI usage
./target/release/shlesha transliterate --from devanagari --to iast "धर्म"
```

## 📞 Support & Contributing

- **Issues**: Report via project repository
- **Development**: Follow [DEVELOPER_SETUP.md](guides/DEVELOPER_SETUP.md)
- **Testing**: Use `./scripts/test-all.sh` before contributing
- **Documentation**: Update this index when adding new docs

---

**All documentation is comprehensive and up-to-date as of the latest release.**