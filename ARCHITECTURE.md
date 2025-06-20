# Shlesha Architecture & Module Interfaces

This document defines the architecture, module boundaries, and interfaces for Shlesha.

## Module Stability Tracker

### 🔒 Immutable Modules (Do Not Modify)

These modules have stable interfaces and comprehensive tests. Changes require architectural review.

| Module | Status | Interface Stability | Test Coverage |
|--------|--------|-------------------|---------------|
| `tests/property_based_tests.rs` | 🔒 LOCKED | Stable | 100% |
| `tests/comprehensive_script_tests.rs` | 🔒 LOCKED | Stable | 100% |
| `benches/comprehensive_comparison.rs` | 🔒 LOCKED | Stable | N/A |
| `src/lossless_transliterator.rs` (tests) | 🔒 LOCKED | Stable | 100% |

### 🟡 Stable Interfaces (Implementation May Change)

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
| Python bindings | 📋 Planned | PyO3 integration |
| WASM bindings | 📋 Planned | wasm-bindgen |

## Module Interfaces

### Core Transliterator Interface

```rust
// src/lossless_transliterator.rs
pub trait TransliteratorInterface {
    /// Core transliteration function
    fn transliterate(&self, text: &str, from: &str, to: &str) -> Result<String, String>;
    
    /// Verify losslessness with mathematical proof
    fn verify_lossless(&self, original: &str, encoded: &str, from_script: &str) -> LosslessResult;
    
    /// Extract preservation tokens
    fn extract_tokens(&self, text: &str) -> Vec<PreservationToken>;
    
    /// Calculate Shannon entropy
    fn calculate_entropy(&self, text: &str) -> f64;
}
```

### Script Mapping Interface

```rust
// src/script_mappings.rs
pub trait ScriptMappingInterface {
    /// Get all supported scripts
    fn get_supported_scripts() -> Vec<(String, u8)>;
    
    /// Check if mapping exists
    fn has_mapping(from_id: u8, to_id: u8) -> bool;
    
    /// Get mapper for script pair
    fn get_mapper(from_id: u8, to_id: u8) -> Option<&'static LosslessMapper>;
}
```

### Mapper Interface

```rust
// src/lossless_transliterator.rs
pub trait MapperInterface {
    /// Binary search character lookup
    fn lookup_char(&self, ch: char) -> Option<&'static str>;
    
    /// Pattern matching for sequences
    fn lookup_pattern(&self, text: &str, pos: usize) -> Option<(&'static str, usize)>;
    
    /// Create preservation token
    fn create_preservation_token(&self, text: &str, pos: usize) -> PreservationToken;
}
```

### Token Interface

```rust
// src/lossless_transliterator.rs
pub trait TokenInterface {
    /// Encode token to string
    fn encode(&self) -> String;
    
    /// Decode token from string
    fn decode(s: &str) -> Option<Self> where Self: Sized;
    
    /// Check reconstruction capability
    fn can_reconstruct(&self, target_script: u8, registry: &ScriptRegistry) -> bool;
}
```

## Performance Optimization Boundaries

### Areas Open for Optimization

1. **Character Lookup** (`lookup_char`)
   - Current: Binary search O(log n)
   - Potential: SIMD, perfect hashing, compile-time lookup

2. **String Building** (`transliterate_with_mapper`)
   - Current: String allocation with capacity hint
   - Potential: Zero-copy with borrowed slices

3. **Pattern Matching** (`lookup_pattern`)
   - Current: Linear scan of patterns
   - Potential: Trie, Aho-Corasick, SIMD

4. **Entropy Calculation** (`calculate_entropy`)
   - Current: HashMap for frequency counting
   - Potential: Array-based for common scripts

### Invariants to Maintain

1. **Losslessness**: H(original) ≤ H(encoded) + H(tokens)
2. **Token Format**: `[script_id:data:metadata]`
3. **Pattern Precedence**: Longest match first
4. **Binary Search Order**: Mappings sorted by Unicode value
5. **Preservation Ratio**: ≥ 0.95 for all scripts

## Testing Requirements

### Unit Test Invariants
- All public methods must have tests
- Edge cases: empty string, single char, Unicode boundaries
- Pattern matching: precedence, overlapping patterns
- Token handling: encoding, decoding, malformed tokens

### Property-Based Test Invariants
- Losslessness for all valid input
- Token roundtrip correctness
- Entropy non-decreasing
- Pattern precedence maintained

### Benchmark Requirements
- Compare against Vidyut for regression detection
- Measure throughput in MB/s
- Track memory allocations
- Profile hot paths

## Development Workflow

### Making Changes

1. **Check Module Status**: Consult the stability tracker above
2. **Respect Interfaces**: Public APIs must remain stable
3. **Run Tests**: `cargo test` must pass before any commit
4. **Run Benchmarks**: `cargo bench` to detect performance regressions
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

## Version Control Strategy

### Protected Files
```
tests/**/*.rs          # All test files are immutable
benches/**/*.rs        # All benchmark files are immutable
src/**/tests.rs        # Test modules within source files
```

### Semantic Versioning
- Major: Breaking API changes
- Minor: New features, backward compatible
- Patch: Bug fixes, performance improvements

### Git Workflow
1. Feature branches for new development
2. PR required for protected files
3. Benchmarks must not regress >5%
4. All tests must pass

---

**Note**: This architecture document is the source of truth for module boundaries and development practices. Update it when making architectural changes.