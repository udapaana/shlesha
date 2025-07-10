# Shlesha Development Principles

## Core Philosophy

### Single Source of Truth
- **One Way to Do Things**: There should be exactly one fundamental way to perform any operation
- **No Redundant Implementations**: Multiple implementations of the same functionality create confusion and maintenance burden
- **Git is Your Safety Net**: Don't keep old code "just in case" - git history preserves everything

### Token-Based Architecture
- **Zero-Allocation Tokens**: All transliteration operations use compile-time generated token enums
- **Build-Time Generation**: Mappings are generated from schemas at build time, not runtime
- **Direct Token Mappings**: No intermediate string conversions - tokens map directly to tokens

### Code Organization
- **Clear Module Boundaries**: Each module has a single, well-defined responsibility
- **Minimal Dependencies**: Modules should not be tightly coupled
- **Explicit Interfaces**: Use traits to define clear contracts between modules

### Performance First
- **Measure, Don't Guess**: All performance claims must be backed by benchmarks
- **Zero-Cost Abstractions**: Abstractions should have no runtime overhead
- **Compile-Time Optimization**: Push as much work as possible to compile time

### Testing Strategy
- **Property-Based Testing**: Core functionality verified through property tests
- **Token-Based Tests**: All tests use the token-based API
- **No Legacy Test Paths**: Tests for old functionality should be deleted, not disabled

### Technical Debt Management
- **Delete, Don't Disable**: Remove unused code immediately
- **Refactor Fearlessly**: Git history preserves the old implementation
- **No TODOs Without Issues**: Every TODO must link to a tracked issue

## Implementation Guidelines

### Hub Architecture
- Hub converts between AbugidaToken ↔ AlphabetToken only
- No string-based hub methods
- All mappings generated from token schemas at build time

### Error Handling
- Use Result types for fallible operations
- Provide meaningful error messages with context
- Unknown tokens should be handled gracefully

### Schema Management
- Token schemas are the source of truth
- All converters generated from schemas
- No hardcoded mappings in Rust code

### Benchmarking
- Compare against established libraries (Vidyut)
- Test realistic workloads, not micro-benchmarks
- Track performance regressions in CI

## Code Review Checklist

- [ ] Does this add a second way to do something that already exists?
- [ ] Are there any string-based hub operations that could be tokens?
- [ ] Is there dead code that can be deleted?
- [ ] Are error messages helpful and contextual?
- [ ] Do benchmarks validate performance claims?
- [ ] Are module boundaries respected?