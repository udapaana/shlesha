# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- **Grantha**: Retroflex and dental stop series were swapped. The single-letter
  consonant tokens (`ConsonantT/Th/D/Dh/N`) are retroflex (ṭ ṭh ḍ ḍh ṇ) and the
  doubled tokens (`ConsonantTt/Tth/Dd/Ddh/Nn`) are dental (t th d dh n) per the hub
  convention shared with Devanagari, Kannada, etc. In `schemas/grantha.yaml` these
  were assigned to the wrong glyphs, so `ṭa` rendered as 𑌤 (GRANTHA LETTER TA,
  dental) and `ta` rendered as 𑌟 (GRANTHA LETTER TTA, retroflex). Round-trips still
  succeeded because the swap was internally consistent. Added `tests/grantha_test.rs`
  pinning the exact glyphs to prevent regression.

## [0.5.5] - 2026-02-28

### Added
- **Baraha script support** (`baraha`, aliases: `baraha_north`, `baraha_south`) — popular
  South Indian romanisation scheme with case-based retroflex/dental distinction
  (uppercase T/D/N/C/S/P/B/G/K/J/Sh for retroflex, lowercase for dental), long vowel
  aliases (A/aa, I/ee, U/oo), Anusvara (M), Visarga (H), Candrabindu (~m), Avagraha (~),
  and basic Vedic accent marks.
- **Runtime source schema support** — schemas loaded at runtime via
  `load_schema_from_string()` or `load_schema_from_file()` can now be used as the
  *source* (input) script, not just the target. Uses inverted mappings with greedy
  longest-match parsing.

### Fixed
- **Kannada (#47)**: Added missing independent long vowels `VowelEe` (ಏ U+0C8F) and
  `VowelOo` (ಓ U+0C93). Previously these output literal `[VowelEe]`/`[VowelOo]`
  placeholders. Short vowels (ಎ/ಒ) and dependent vowel signs were already correct.
- **Tamil (#46)**: Added missing independent vowels `VowelEe` (ஏ) and `VowelOo` (ஓ),
  and dependent vowel signs `VowelSignEe` (ே U+0BC7) and `VowelSignOo` (ோ U+0BCB).
  Previously these leaked as `[VowelSignEe]`/`[VowelSignOo]` placeholders.

## [0.5.1] - 2025-01-24

### Fixed
- Generate is_indic_script/is_roman_script at build time from schema metadata
- Token converter bug causing "????" output for Roman → new Indic script conversions
- Add direct converters for new Vedic scripts for better performance

## [0.5.0] - 2025-01-23

### Added
- Grantha script support (`grantha`) - Historical script of Tamil Nadu for Sanskrit
  - Complete character mappings including Grantha-specific marks
  - Full Vedic accent support with additional marks (kampa, prachaya, etc.)
- Nine additional historical Vedic scripts:
  - Sharada (`sharada`, `shrd`) - Historical script of Kashmir
  - Siddham (`siddham`) - Buddhist/tantric script
  - Modi (`modi`) - Maharashtra historical script
  - Newa/Newari (`newa`) - Nepal historical script with OM symbol
  - Bhaiksuki (`bhaiksuki`) - Buddhist manuscript script
  - Kaithi (`kaithi`) - North Indian historical script
  - Takri (`takri`) - Western Himalayan script
  - Dogra (`dogra`) - Jammu & Kashmir script
  - Nandinagari (`nandinagari`) - South Indian Sanskrit script
- Visual-based Vedic accent token names to resolve Unicode ambiguities:
  - `MarkVerticalLineAbove` for ॑ (U+0951)
  - `MarkLineBelow` for ॒ (U+0952)
  - `MarkDoubleVerticalAbove` for ᳚ (U+1CDA)
  - `MarkTripleVerticalAbove` for ᳛ (U+1CDB)
- Documentation for Vedic accent Unicode ambiguities (`docs/VEDIC_ACCENTS.md`)
- Tibetan script support (`tibetan`, `tibt`, `bo`) - Important for Buddhist Vedic transmission
  - Complete Sanskrit transliteration mappings
  - Aspirated consonants (གྷ, ཛྷ, ཌྷ, དྷ, བྷ) for accurate Sanskrit representation
  - Vedic accent support using standard combining marks
- Thai script support (`thai`, `th`) - Adapted from Grantha for Buddhist Vedic texts
  - Sanskrit consonant and vowel mappings using Thai characters
  - Vedic accent approximation using Thai tone marks
  - Special handling for vowel signs (pre-consonantal เ, ไ, โ)

### Changed
- All built-in schemas now use visual token names for Vedic accents instead of linguistic names
- Updated documentation to explain visual token naming approach
- Fixed duplicate character mappings in multiple schemas

### Fixed
- Short e/o vowel mapping convention - only map if script has distinct characters
- Vedic accent mappings to reflect actual usage vs Unicode naming
- Duplicate character mappings causing unreachable pattern warnings

### Removed
- Unused GeneratedHub implementation (~150 lines of dead code)
- Legacy linguistic token names (MarkUdatta, MarkSvarita, etc.) from built-in schemas

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