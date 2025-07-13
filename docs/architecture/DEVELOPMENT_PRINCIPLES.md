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

### Performance First
- **Measure, Don't Guess**: All performance claims must be backed by benchmarks
- **Zero-Cost Abstractions**: Abstractions should have no runtime overhead
- **Compile-Time Optimization**: Push as much work as possible to compile time

### Documentation Standards
- **Just the Facts**: Documentation should contain only factual, technical information
- **No Self-Congratulation**: Avoid superlatives, marketing language, or self-praising terms
- **Objective Tone**: Present features and capabilities without embellishment
- **Focus on Utility**: Documentation exists to help users, not to sell or impress

## Modular Architecture

### Core Principles
1. **Complete Modularity**: Each module defines a clear interface and maintains its own todo list
2. **Single Module Mutability**: Only one module is mutable at any given time
3. **Clear Module Boundaries**: Each module has a single, well-defined responsibility
4. **Minimal Dependencies**: Modules should not be tightly coupled
5. **Explicit Interfaces**: Use traits to define clear contracts between modules

### Module Communication
- **Interface-Only Access**: Modules can only access other modules through their defined interfaces
- **Cross-Module Protocol**: Changes to external modules must be requested via their todo lists
- Internal implementation details remain hidden
- This enforces loose coupling and high cohesion

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

### Testing Strategy
- **Test-Driven Development**: Every module must have comprehensive tests
- **Property-Based Testing**: Core functionality verified through property tests
- **Token-Based Tests**: All tests use the token-based API
- **No Legacy Test Paths**: Tests for old functionality should be deleted, not disabled
- Modules are not considered "done" until tests pass

### Technical Debt Management
- **Delete, Don't Disable**: Remove unused code immediately
- **Refactor Fearlessly**: Git history preserves the old implementation
- **No TODOs Without Issues**: Every TODO must link to a tracked issue

## Development Workflow

### Version Control
1. **Frequent Small Commits**: Each commit should represent a single logical change
2. **Change Logging**: All changes are logged after each commit
3. **Clear Commit Messages**: Should clearly describe the module and changes made
4. **Strategic Checkpoints**: Use git commits to checkpoint progress

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
- [ ] Are all public interfaces properly tested?
- [ ] Have change logs been updated?