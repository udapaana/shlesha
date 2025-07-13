# Repository Structure

This document describes the organization of the Shlesha project repository.

## Directory Layout

```
shlesha/
├── src/                    # Rust source code
│   ├── lib.rs             # Library entry point
│   ├── main.rs            # CLI binary entry point
│   ├── modules/           # Core modules
│   │   ├── hub/           # Hub conversion engine
│   │   ├── script_converter/ # Script converter implementations
│   │   ├── core/          # Core utilities and handlers
│   │   └── registry/      # Schema registry management
│   ├── python_bindings.rs # PyO3 Python bindings
│   └── wasm_bindings.rs   # WebAssembly bindings
│
├── schemas/               # YAML schema definitions
│   ├── bengali.yaml      # Bengali script mappings
│   ├── devanagari.yaml   # Devanagari script mappings
│   └── ...               # Other script schemas
│
├── docs/                  # Documentation
│   ├── architecture/      # Architecture and design docs
│   │   ├── ARCHITECTURE.md
│   │   ├── DEVELOPMENT_PRINCIPLES.md
│   │   ├── MODULE_ARCHITECTURE.md
│   │   └── PERFORMANCE.md
│   ├── guides/            # User and developer guides
│   │   ├── CONTRIBUTING.md
│   │   └── DEVELOPER_SETUP.md
│   ├── reference/         # API and format references
│   │   ├── API_REFERENCE.md
│   │   ├── BINDINGS.md
│   │   └── SCHEMA_REFERENCE.md
│   ├── DOCUMENTATION_INDEX.md  # Documentation overview
│   └── REPOSITORY_STRUCTURE.md # This file
│
├── examples/              # Usage examples
│   ├── basic_usage.rs    # Basic Rust API usage
│   ├── hub_vs_direct_benchmark.rs
│   └── ...
│
├── tests/                 # Integration tests
│   ├── cli_integration_tests.rs
│   ├── comprehensive_bidirectional_tests.rs
│   └── verify_transliteration.py
│
├── benches/              # Rust/Criterion benchmarks
│   ├── comprehensive_benchmark.rs
│   └── profiling_benchmark.rs
│
├── python_benchmarks/    # Python performance tests
│   ├── memory_profile_benchmark.py
│   └── performance_tests.py
│
├── python/               # Python bindings package
│   ├── pyproject.toml
│   └── tests/
│
├── scripts/              # Development and utility scripts
│   ├── quick-start.sh   # One-command setup
│   ├── build-all.sh     # Build all targets
│   ├── test-all.sh      # Run all tests
│   └── ...
│
├── docker/               # Docker test environments
│   ├── test-python.dockerfile
│   └── ...
│
├── templates/            # Code generation templates
│   ├── brahmic_converter.hbs
│   ├── iso_converter.hbs
│   └── roman_converter.hbs
│
├── build.rs              # Build script for schema processing
├── Cargo.toml            # Rust package manifest
├── Makefile              # Development commands
├── README.md             # Project overview
├── LICENSE               # MIT license
└── RELEASE.md            # Release process documentation
```

## Key Directories

### Source Code (`src/`)
- Core library implementation in Rust
- Modular architecture with clear separation of concerns
- Language bindings for Python and WASM

### Schemas (`schemas/`)
- YAML definitions for all supported scripts
- Used by build.rs to generate optimized converters
- Can be loaded at runtime for custom scripts

### Documentation (`docs/`)
- **architecture/**: System design and technical architecture
- **guides/**: How-to guides for users and developers
- **reference/**: API documentation and format specifications

### Tests
- **tests/**: Integration tests
- **benches/**: Performance benchmarks (Rust)
- **python_benchmarks/**: Python-specific performance tests

### Scripts (`scripts/`)
- Automation for common development tasks
- Quick setup and deployment utilities
- Demo applications

## Build Artifacts

The following directories are created during build but not tracked in git:

- `target/`: Rust build output
- `pkg/`: WASM package output
- `wheels/`: Python wheel output
- `dist/`: Distribution packages

## Development Workflow

1. **Setup**: Run `./scripts/quick-start.sh`
2. **Build**: Use `cargo build` or `make build`
3. **Test**: Run `cargo test` or `./scripts/test-all.sh`
4. **Documentation**: Generate with `cargo doc --open`

See [DEVELOPER_SETUP.md](guides/DEVELOPER_SETUP.md) for detailed instructions.