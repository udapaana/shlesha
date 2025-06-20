# Shlesha Architecture

This document defines the architecture, module boundaries, and development practices for Shlesha.

## Module Stability Tracker

### 🔒 Immutable Modules
These modules have stable interfaces and comprehensive tests. Changes require architectural review.

| Module | Status | Interface Stability | Test Coverage |
|--------|--------|-------------------|---------------|
| `tests/property_based_tests.rs` | 🔒 LOCKED | Stable | 100% |
| `tests/comprehensive_script_tests.rs` | 🔒 LOCKED | Stable | 100% |
| `benches/comprehensive_comparison.rs` | 🔒 LOCKED | Stable | N/A |
| `src/lossless_transliterator.rs` (tests) | 🔒 LOCKED | Stable | 100% |

### 🟡 Stable Interfaces
These modules have stable public APIs but internal implementation can be optimized.

| Module | Interface | Implementation Status |
|--------|-----------|---------------------|
| `src/lossless_transliterator.rs` | ✅ Stable | Open for optimization |
| `src/script_mappings.rs` | ✅ Stable | Needs completion |
| `src/lib.rs` | ✅ Stable | Minimal changes only |

### 🟢 Active Development
These modules are under active development.

| Module | Status | Notes |
|--------|--------|-------|
| Script implementations | 🚧 In Progress | 206/225 mappings remaining |
| Runtime extensions | ✅ Stable | Custom script creation and registration |
| Python bindings | 📋 Planned | PyO3 integration |
| WASM bindings | 📋 Planned | wasm-bindgen |

## Core Architecture

### Module Structure
```
src/
├── lib.rs                      // Public API exports
├── lossless_transliterator.rs  // Core engine with mathematical guarantees
├── script_mappings.rs          // Static mappings for all scripts
└── [additional modules...]     // Script-specific implementations
```

### Key Components

**LosslessTransliterator**: Main transliteration engine
- Script registration and mapping management
- Entropy-based verification
- Token extraction and reconstruction

**LosslessMapper**: High-performance character mapping
- Binary search for O(log n) character lookup
- Pattern matching for multi-character sequences
- Fallback token generation for unmapped characters

**PreservationToken**: Information preservation mechanism
- Encodes unmapped characters with metadata
- Enables perfect reconstruction
- Supports multiple fallback strategies

**ScriptRegistry**: Script and mapping management
- Centralizes script definitions
- Manages bidirectional mappings
- Handles reconstruction pathways

**ExtensionManager**: Runtime extensibility system
- Manages custom scripts and mappings
- Provides builder pattern for script creation
- Enables dynamic script registration without recompilation

### Processing Pipeline
```
Input Text → Pattern Matching → Binary Search → Token Generation → Output
    ↓                              O(log n)              ↓
    └─────────────── Entropy Verification ←──────────────┘
                    H(original) ≤ H(encoded) + H(tokens)
```

## Development Standards

### Performance Requirements
- Binary search for character lookup: O(log n)
- Pattern matching with longest-match-first precedence
- String capacity pre-allocation for performance
- Entropy verification overhead < 10% of transliteration time

### Losslessness Invariants
1. **Mathematical Guarantee**: H(original) ≤ H(encoded) + H(tokens)
2. **Token Format**: `[script_id:data:metadata]`
3. **Pattern Precedence**: Longest match first
4. **Preservation Ratio**: ≥ 0.95 for all scripts
5. **Unicode Ordering**: Mappings sorted by Unicode value

### Testing Requirements

**Unit Tests**
- All public methods must have tests
- Edge cases: empty string, single char, Unicode boundaries
- Pattern matching: precedence, overlapping patterns
- Token handling: encoding, decoding, malformed tokens

**Property-Based Tests**
- Losslessness for all valid input
- Token roundtrip correctness
- Entropy non-decreasing
- Pattern precedence maintained

**Benchmarks**
- Performance regression detection
- Memory allocation tracking
- Throughput measurement (MB/s)

## Development Workflow

### Making Changes
1. **Check Module Status**: Consult stability tracker
2. **Respect Interfaces**: Public APIs must remain stable
3. **Run Tests**: `cargo test` must pass before commit
4. **Run Benchmarks**: `cargo bench` to detect regressions
5. **Update Documentation**: Keep this file current

### Adding New Scripts
1. Add mappings to `src/script_mappings.rs`
2. Update `get_supported_scripts()`
3. Update `has_mapping()` and `get_mapper()`
4. Add tests in `tests/comprehensive_script_tests.rs`
5. Run full test suite

### Performance Optimization
1. Identify hot path with `cargo bench`
2. Profile with `perf` or `cargo flamegraph`
3. Implement optimization maintaining interface
4. Verify with benchmarks
5. Ensure all tests still pass

## Optimization Boundaries

### Areas Open for Optimization
1. **Character Lookup**: Current binary search can be optimized with SIMD or perfect hashing
2. **String Building**: Current allocation can be improved with zero-copy techniques
3. **Pattern Matching**: Current linear scan can use Trie or Aho-Corasick
4. **Entropy Calculation**: Current HashMap can use arrays for common scripts

### Protected Elements
- Public API signatures
- Test interfaces and behaviors
- Losslessness guarantees
- Token format specification
- Mathematical verification methods

## Git Workflow

### Protected Files
```
tests/**/*.rs          # All test files are immutable
benches/**/*.rs        # All benchmark files are immutable
src/**/tests.rs        # Test modules within source files
```

### Branch Strategy
- `main`: Stable releases
- Feature branches for new development
- PR required for protected files
- All tests must pass before merge

### Commit Standards
- Atomic commits with clear descriptions
- Performance impact noted in commit message
- Breaking changes marked with `BREAKING:`
- Documentation updates included with changes

---

**Note**: This architecture document defines the authoritative module boundaries and development practices for Shlesha.