# Claude Development Notes for Shlesha

This file contains development philosophy, architectural decisions, and Claude-specific development practices for the Shlesha transliteration library.

## 🏗️ Modular/Component-Based Development Philosophy

### Core Principles

1. **Interface Stability Over Implementation**
   - Public APIs are immutable contracts
   - Internal implementations can be freely optimized
   - Changes require interface versioning

2. **Protected Test Boundaries**
   - Test files are treated as immutable specifications
   - Benchmarks serve as regression detection
   - Property-based tests encode mathematical invariants

3. **Opaque Component Design**
   - Modules expose minimal public surface
   - Internal complexity is hidden behind traits
   - Components can be swapped without affecting callers

### File Stability Classification

```
🔒 IMMUTABLE (Breaking Changes Require Review)
- tests/**/*.rs - Test specifications
- benches/**/*.rs - Performance baselines  
- Public trait definitions in lib.rs

🟡 STABLE INTERFACE (Implementation May Change)
- src/lossless_transliterator.rs - Core algorithms
- src/script_mappings.rs - Static data structures
- Module public APIs

🟢 ACTIVE DEVELOPMENT
- Internal implementation details
- Private functions and methods
- Performance optimizations
```

## 🎯 Development Workflow

### Making Changes

1. **Check Module Status**: Consult `ARCHITECTURE.md` stability tracker
2. **Respect Interfaces**: Never break public APIs without version bump
3. **Test-Driven**: All changes must pass existing test suite
4. **Benchmark-Aware**: Monitor performance impact with `cargo bench`
5. **Document Changes**: Update architecture notes for design decisions

### Adding Features

1. **Design Interface First**: Define public API before implementation
2. **Write Tests**: Property-based tests for mathematical guarantees
3. **Implement Behind Interface**: Keep internals flexible
4. **Benchmark**: Measure performance impact
5. **Document**: Update both code and architecture docs

### Optimization Strategy

1. **Profile First**: Use `cargo bench` to identify bottlenecks
2. **Target Highest Impact**: Focus on largest performance gaps first
3. **Maintain Invariants**: Never break losslessness guarantee
4. **Test Coverage**: Ensure optimizations don't break edge cases
5. **Interface Preservation**: Keep public APIs unchanged
6. **Regression Detection**: Compare against baseline benchmarks
7. **Commit Frequently**: Small, atomic changes with performance tracking

### Performance-Focused Development

1. **Measure Before Optimize**: Always benchmark current state
2. **Isolate Changes**: One optimization per commit for easy rollback
3. **Track Performance Impact**: Include benchmark results in commit messages
4. **Rollback Criteria**: Any regression >5% without equivalent gain elsewhere
5. **Document Tradeoffs**: Explain why specific approaches were chosen

## 🧪 Testing Philosophy

### Mathematical Guarantees

Shlesha's correctness is based on information theory:

```rust
// Core invariant: Losslessness
H(original) ≤ H(encoded) + H(tokens)

// Property-based testing ensures this holds for all valid inputs
proptest! {
    fn prop_lossless_guarantee(text in valid_devanagari()) {
        let result = transliterate(text);
        assert!(verify_lossless(text, result));
    }
}
```

### Test Categories

1. **Unit Tests** - Component behavior verification
2. **Property-Based Tests** - Mathematical invariant checking  
3. **Integration Tests** - Cross-module interaction validation
4. **Regression Tests** - Prevention of performance/correctness degradation

### Test Immutability

Tests serve as **executable specifications**:
- They define what the system must do
- Changes indicate specification changes (require review)
- They protect against regression during optimization

## 🚀 Performance Philosophy

### Optimization Hierarchy

1. **Algorithmic Efficiency** - O(log n) vs O(n) improvements
2. **Data Structure Optimization** - Cache-friendly layouts
3. **Micro-optimizations** - SIMD, branch prediction, etc.
4. **Parallelization** - Multi-threading for large inputs

### Performance Measurement

```bash
# Core performance benchmarks
cargo bench --bench comprehensive_comparison

# Cross-tool comparison (with Vidyut)  
cargo bench --features compare-vidyut

# Memory profiling
cargo bench --features profiling
```

### Optimization Targets

Current performance vs Vidyut:
- ✅ **Medium text**: Shlesha 20% faster
- 🎯 **Large text**: Vidyut 83% faster (optimization target)
- 🎯 **Memory usage**: Both efficient, room for improvement

## 🌍 Extensibility Design

### Script Addition Pattern

```rust
// 1. Add mappings to script_mappings.rs
pub const NEW_SCRIPT_TO_IAST: &[(char, &str)] = &[...];

// 2. Register in setup_builtin_scripts()
registry.register_script("NewScript".to_string(), NEW_SCRIPT_ID);

// 3. Add tests in comprehensive_script_tests.rs
ScriptTestData {
    script_name: "NewScript".to_string(),
    basic_chars: vec![...],
    // ...
}
```

### Interface Extension Pattern

```rust
// Extend traits, never break them
pub trait TransliteratorInterface {
    // Existing methods (never remove/change)
    fn transliterate(&self, ...) -> Result<...>;
    
    // New methods (additive only)
    fn transliterate_with_options(&self, ...) -> Result<...>;
}
```

## 🔧 Claude-Specific Development Notes

### Assistance Patterns

1. **Architecture Consultation**: Use `ARCHITECTURE.md` for module boundaries
2. **Test-First Development**: Always check test implications before changes
3. **Performance Analysis**: Use benchmark results to guide optimization
4. **Documentation Updates**: Keep Claude notes current with changes

### Code Review Checklist

- [ ] Does this preserve the losslessness guarantee?
- [ ] Are all existing tests still passing?
- [ ] Is the public interface unchanged?
- [ ] Have benchmarks been run for performance impact?
- [ ] Is documentation updated?

### Future Development Areas

1. **Script Completion**: Implement remaining 206/225 mappings
2. **SIMD Optimization**: Vectorize character lookup operations
3. **Zero-Copy Processing**: Eliminate unnecessary string allocations
4. **Parallel Processing**: Multi-thread large document processing
5. **Language Bindings**: Python (PyO3) and WASM implementations

## 📚 References

- `ARCHITECTURE.md` - Module boundaries and interfaces
- `README.md` - User-facing documentation and examples
- `tests/` - Executable specifications and mathematical proofs
- `benches/` - Performance baselines and regression detection

---

**Philosophy**: "Make interfaces immutable, implementations fluid, and tests the source of truth."

*Last Updated: December 2024*
*Commit: 894413c - Modular architecture establishment*