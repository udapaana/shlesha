# Contributing to Shlesha

Thank you for your interest in contributing to Shlesha! This document provides guidelines and information for contributors.

## Quick Start for Contributors

1. **Set up the development environment:**
   ```bash
   ./scripts/quick-start.sh
   ```

2. **Run tests to ensure everything works:**
   ```bash
   cargo test
   ```

3. **Make your changes and test them thoroughly**

4. **Submit a pull request**

## Development Workflow

### Prerequisites

- Rust 1.70+ (latest stable recommended)
- Python 3.8+ (for Python bindings)
- Node.js (for WASM bindings)

See [DEVELOPER_SETUP.md](DEVELOPER_SETUP.md) for detailed setup instructions.

### Building and Testing

```bash
# Build the library
cargo build

# Run all tests
cargo test

# Run benchmarks
cargo bench

# Test Python bindings
cd python && python -m pytest

# Build WASM bindings
wasm-pack build --target web
```

### Code Standards

1. **Rust Code:**
   - Follow `rustfmt` formatting (run `cargo fmt`)
   - Pass all `clippy` lints (run `cargo clippy`)
   - Add tests for new functionality
   - Document public APIs

2. **Schema Contributions:**
   - Place new schemas in `schemas/` directory
   - Use YAML format consistently
   - Include comprehensive test cases
   - Follow the schema format documented in [SCHEMA_REFERENCE.md](../reference/SCHEMA_REFERENCE.md)

3. **Documentation:**
   - Update relevant documentation for changes
   - Add examples for new features
   - Keep README.md current

## Adding New Script Support

To add support for a new script:

1. **Create a schema file** in `schemas/[script_name].yaml`
2. **Follow the schema format:**
   ```yaml
   metadata:
     name: "script_name"
     script_type: "roman" | "brahmic"
     description: "Description of the script"
   
   target: "iso15919" | "devanagari"  # conversion target
   
   mappings:
     vowels:
       "source": "target"
     consonants:
       "source": "target"
   ```

3. **Add comprehensive tests** in appropriate test files
4. **Update documentation** if needed

## Schema Format Guidelines

### For Roman Scripts (target: "iso15919")
- Convert TO ISO-15919 standard
- Use `script_type: "roman"`
- Include vowels, consonants, marks, and special characters

### For Indic Scripts (target: "devanagari" or omitted)
- Convert TO Devanagari
- Use `script_type: "brahmic"`
- Include comprehensive Unicode mappings

## Performance Considerations

- Schema-generated converters use compile-time optimization
- All converters use O(1) hash map lookups
- Maintain performance parity with hand-coded implementations
- Run benchmarks to verify performance impact

## Testing Requirements

### Unit Tests
- Test basic conversion functionality
- Test edge cases and error conditions
- Test roundtrip conversions where applicable

### Integration Tests
- Test schema loading and validation
- Test converter generation
- Test API compatibility

### Benchmark Tests
- Verify performance meets standards
- Compare against existing implementations
- Document performance characteristics

## Documentation Standards

- Keep all documentation current with code changes
- Use clear, concise language
- Provide practical examples
- Update the [DOCUMENTATION_INDEX.md](../DOCUMENTATION_INDEX.md) for new docs

## Submission Guidelines

### Pull Requests

1. **Branch naming:** Use descriptive names like `feature/new-script-support` or `fix/conversion-bug`

2. **Commit messages:** Use clear, descriptive commit messages
   ```
   feat: Add support for Tamil script conversion
   fix: Correct IAST vowel mapping for ṛ character
   docs: Update schema format documentation
   ```

3. **Testing:** Ensure all tests pass and add new tests for your changes

4. **Documentation:** Update relevant documentation

### Code Review Process

1. All submissions require review
2. Address reviewer feedback promptly
3. Maintain backwards compatibility unless explicitly breaking
4. Follow established patterns and conventions

## Getting Help

- **Documentation:** See [docs/](../) directory for comprehensive guides
- **Issues:** Create GitHub issues for bugs or feature requests
- **Discussions:** Use GitHub discussions for questions and ideas

## Project Structure

```
shlesha/
├── src/                    # Rust source code
├── schemas/                # YAML schema definitions
├── docs/                   # Documentation
├── examples/               # Usage examples
├── tests/                  # Integration tests
├── benches/                # Rust/Criterion benchmarks
├── python_benchmarks/      # Python performance tests
├── python/                 # Python bindings
└── scripts/                # Development scripts
```

## License

By contributing to Shlesha, you agree that your contributions will be licensed under the MIT License.