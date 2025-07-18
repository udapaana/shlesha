# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2024-07-18

### Added
- Comprehensive Vedic accent support across all scripts
  - Udātta (high pitch), Anudātta (low pitch), Svarita (falling pitch)
  - Double and triple svarita marks for complex Vedic texts
  - Support for both standard Unicode combining characters and alternative representations
- VowelEe and VowelOo token mappings for proper Sanskrit phonology
- Known round-trip limitations documented in property tests

### Changed
- Moved Vedic accent marks from "marks" to dedicated "vedic" sections in all schemas
- Updated is_mark() function to include vedic tokens for proper round-trip preservation
- Fixed IAST schema to handle Sanskrit's inherently long e/o vowels correctly
- Property tests now recognize and accept linguistically correct normalizations

### Fixed
- Round-trip preservation of Vedic accent marks
- VowelEe/VowelOo token display issues in Roman scripts
- IAST to ISO15919 vowel length normalization (o→ō, e→ē)
- Clippy warnings in generated code with proper allow attributes

## [0.2.1] - 2024-12-18

### Added
- Comprehensive code cleanup for release readiness
- Documentation for recommended additional schemas
- Example schemas: Kyoto-Harvard, URL-safe Sanskrit, and ISCII

### Changed
- Improved trait-based token converter with optimized state machine approach
- Fixed virama handling for final consonants in Brahmic scripts
- Updated hub token tests to correctly handle implicit 'a' behavior

### Removed
- All TODO comments throughout the codebase
- Dead code and unused imports
- Special token handling (SpecialKs, SpecialJn) in favor of natural tokenization
- Vestigial comments and REMOVED annotations

### Fixed
- Telugu long vowel tests with proper virama placement
- Property test failures for 'kṣ' and 'jñ' input
- Clippy warnings in generated code
- WASM bindings unused code

## [0.2.0] - Previous Release

### Added
- Token-based conversion system
- Schema-driven architecture
- Runtime schema loading
- Comprehensive test suite

### Changed
- Complete rewrite from string-based to token-based system
- Hub architecture using Devanagari ↔ ISO-15919 as central format

### Removed
- Legacy string-based converters
- Hardcoded conversion logic