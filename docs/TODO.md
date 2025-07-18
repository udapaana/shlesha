# Shlesha Project TODO List

This file centralizes all module-level todos and development tasks for the Shlesha project.
All new tasks should be added here rather than scattered across module files.

## Project Philosophy Update
- We maintain a single centralized TODO.md file for better task tracking and visibility
- Module-specific tasks are organized by sections in this file
- This approach replaces scattered TODO comments across modules

## Core Architecture & Design

### Performance Optimization
- [ ] Implement linguistic rule pre-computation (not just character mappings)
- [ ] Create specialized fast paths for common conversions
- [ ] Add caching layer for frequently converted patterns
- [ ] Implement hybrid approach: pre-computed for known patterns, hub for unknown
- [ ] Profile and optimize hot paths in conversion pipeline
- [ ] Consider lazy loading of mapping data

### Design & Architecture
- [ ] Document the trade-offs between hub-and-spoke vs direct conversion
- [ ] Create architecture decision records (ADRs) for major design choices
- [ ] Implement plugin system for custom conversion rules
- [x] Design API for runtime script registration (completed with custom schema system)

## Module-Specific Tasks

### Hub Module (`src/modules/hub/`)
- [ ] Add support for conjunct consonants
- [ ] Implement proper handling of complex Devanagari sequences
- [ ] Add support for Vedic accents
- [ ] Optimize mapping lookup performance
- [ ] Add comprehensive Unicode normalization
- [ ] Implement preservation tokens for unknown mappings: [<script>:<token>:<unicode_point>]

### Script Converter Module (`src/modules/script_converter/`)
- [ ] Handle ambiguous mappings with superscripted numerals when:
    - One character in source script maps to multiple characters in destination script
    - Multiple characters in source script map to one character in destination script
    - Example: Tamil ப could map to ப² (pha), ப³ (ba), or ப⁴ (bha) to disambiguate
    - This would help preserve information in bidirectional conversions
- [ ] Add support for Grantha script used for Sanskrit in Tamil Nadu
- [ ] Add support for Sinhala script
- [ ] Add support for Tibetan script
- [ ] Add support for Thai/Lao scripts (for Sanskrit/Pali texts)
- [ ] Implement contextual conversion rules for better accuracy
- [ ] Add script-specific validation rules
- [ ] Implement script detection for automatic source script identification

### Registry Module (`src/modules/registry/`)
- [x] Implement YAML schema file loading
- [x] Add schema validation
- [x] Implement dynamic schema registration (load from directory)
- [ ] Add schema versioning support (handle multiple versions)
- [x] Implement basic schema caching (HashMap cache)

### Mapping Data Module (`src/modules/mapping_data/`)
- [ ] Implement actual mapping lookup (currently returns empty mappings)
- [ ] Implement mapping validation logic
- [ ] Implement mapping composition for multi-step conversions
- [ ] Add support for loading mappings from external files
- [ ] Implement mapping data versioning

### Processor Optimizations (`src/modules/script_converter/processors.rs`)
- [ ] Re-enable Roman script processor optimizations after fixing logic
- [ ] Implement proper Indic-specific logic for vowel marks
- [ ] Add specialized processors for common patterns
- [ ] Implement streaming processors for large texts

## Features & Functionality

### New Script Support
- [ ] Grantha script (for Sanskrit in Tamil Nadu)
- [ ] Sinhala script
- [ ] Tibetan script
- [ ] Thai script (for Sanskrit/Pali)
- [ ] Lao script (for Sanskrit/Pali)
- [ ] Myanmar/Burmese script
- [ ] Javanese script
- [ ] Balinese script

### Language Bindings & APIs
- [ ] Improve Python API with more Pythonic interfaces
- [ ] Create npm package for JavaScript/TypeScript
- [ ] Add C API for embedding in other applications
- [ ] Create Ruby gem
- [ ] Add Go bindings
- [ ] Implement gRPC service for network access

### Developer Experience
- [ ] Create comprehensive documentation site
- [ ] Add interactive web demo using WASM
- [ ] Create tutorial series for common use cases
- [ ] Add more code examples in multiple languages
- [ ] Create VS Code extension for live transliteration
- [ ] Build online playground for testing conversions

## Testing & Quality

### Test Coverage
- [ ] Add property-based testing for all script pairs
- [ ] Create comprehensive Unicode test suite
- [ ] Add fuzzing tests for robustness
- [ ] Implement performance regression tests
- [ ] Add memory usage benchmarks
- [ ] Create validation suite against reference implementations

### Benchmarking
- [ ] Create standardized benchmark suite
- [ ] Compare against all major transliteration libraries
- [ ] Add benchmarks for streaming/chunked processing
- [ ] Profile memory allocation patterns
- [ ] Add benchmarks for parallel processing

## Documentation

### User Documentation
- [ ] Write getting started guide
- [ ] Create script-specific documentation
- [ ] Document all supported Unicode ranges
- [ ] Add troubleshooting guide
- [ ] Create FAQ section

### Technical Documentation
- [ ] Document internal architecture
- [ ] Create contributor guide
- [ ] Document performance characteristics
- [ ] Add API reference documentation
- [ ] Create decision log for design choices

## Infrastructure & Tooling

### Build System
- [ ] Optimize build times
- [ ] Add cross-compilation support
- [ ] Create Docker images
- [ ] Set up automated benchmarking
- [ ] Add size optimization for WASM builds

### CI/CD
- [ ] Set up comprehensive GitHub Actions
- [ ] Add automated performance testing
- [ ] Implement security scanning
- [ ] Add dependency update automation
- [ ] Create release automation

## Future Considerations

### Advanced Features
- [ ] Machine learning-based script detection
- [ ] Context-aware transliteration
- [ ] Support for historical scripts
- [ ] Implement reversible transliteration with metadata
- [ ] Add support for mixed-script documents

### Research & Development
- [ ] Investigate GPU acceleration for batch processing
- [ ] Research optimal data structures for mapping storage
- [ ] Explore SIMD optimizations
- [ ] Study linguistic patterns for better conversion rules
- [ ] Investigate WebAssembly SIMD support

---

## Completed Tasks

### Custom Schema System (Runtime Extensibility)
- [x] Implement SchemaBasedConverter for runtime-loaded scripts
- [x] Connect Schema Registry to Script Converter Registry
- [x] Add fallback logic: hardcoded converters → schema-based converter
- [x] Update routing logic to check schema registry for custom scripts
- [x] Fix script detection to include loaded schemas
- [x] Create example custom schema files and test cases
- [x] Document custom schema functionality comprehensively
- [x] Enable runtime script registration without recompilation

### Registry Module
- [x] Implement YAML schema file loading
- [x] Add schema validation
- [x] Implement dynamic schema registration (load from directory)
- [x] Implement basic schema caching (HashMap cache)

### Core Architecture
- [x] Implement hub-and-spoke architecture
- [x] Add support for 15 scripts (8 Indic + 7 Roman)
- [x] Implement bidirectional conversion for all script pairs
- [x] Add compile-time pre-computation with feature flags
- [x] Create multi-language bindings (Rust, Python, WASM, CLI)

---

## Notes

- Tasks are roughly ordered by priority within each section
- Performance optimization remains important for competitive benchmarks
- New script support should focus on commonly requested scripts first
- All tasks should maintain the project's focus on correctness and extensibility