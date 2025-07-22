# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Sharada script support (`sharada`, `shrd`) - Historical script of Kashmir, crucial for Vedic manuscripts
  - Full consonant, vowel, and mark mappings
  - Vedic accent support using standard Devanagari accent marks
  - Proper character ordering for yogavaha and vedic marks
- Tibetan script support (`tibetan`, `tibt`, `bo`) - Important for Buddhist Vedic transmission
  - Complete Sanskrit transliteration mappings
  - Aspirated consonants (གྷ, ཛྷ, ཌྷ, དྷ, བྷ) for accurate Sanskrit representation
  - Vedic accent support using standard combining marks
- Thai script support (`thai`, `th`) - Adapted from Grantha for Buddhist Vedic texts
  - Sanskrit consonant and vowel mappings using Thai characters
  - Vedic accent approximation using Thai tone marks
  - Special handling for vowel signs (pre-consonantal เ, ไ, โ)
- Token debug schemas (`abugida_tokens.yaml`, `alphabet_tokens.yaml`) for development
  - Document all possible tokens in the conversion system
  - Useful for understanding the token-based architecture

## [0.4.2] - 2025-07-22

### Fixed
- Character ordering for Vedic accents and yogavaha marks (anusvara/visarga) when converting between Roman and Indic scripts
  - Roman scripts: vedic accent + yogavaha (e.g., "ma̍ḥ")
  - Indic scripts: yogavaha + vedic accent (e.g., "मः॑")
  - Ensures proper rendering in target scripts without affecting transliteration accuracy

### Added
- Token categorization methods: `is_vedic_accent()` and `is_yogavaha()` for better mark classification
- Unit tests for mark reordering logic

## [0.4.1] - 2025-07-21

### Fixed
- SLP1 short vowel support and template escaping improvements
- Various minor bug fixes

## [0.4.0] - 2025-07-19

### Added
- Full SLP1 (Sanskrit Library Phonetic) encoding support:
  - Short vowels: `e1` for short e (ऎ), `o1` for short o (ऒ) - Dravidian support
  - Avagraha (ऽ) support using backtick (`) notation
  - Proper vowel length distinctions between short and long e/o
- General token representation system for unmapped characters
  - Tokens without mappings in target scripts display as `[TokenName]`
  - Enables lossless one-way conversions
- Template escaping improvements for special characters in schemas

### Changed
- SLP1 schema updated with proper vowel mappings
- Build process now properly escapes special characters in templates
- Hub generation prefers Devanagari as primary abugida schema

### Fixed
- Property test expectations for SLP1 to IAST conversions
- Template escaping issues preventing avagraha support
- Clippy warnings in source code
- Module naming and code quality issues

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