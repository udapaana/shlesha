# Shlesha Transliterator Design Document

## Overview

Shlesha is a bidirectional, extensible, and highly-performant transliterator for Indic languages designed with a compiler-like architecture inspired by LLVM. It addresses the fundamental differences between Indic scripts (abugidas) and Roman scripts (alphabets) through a dual intermediate representation system.

## Core Architecture

### 1. Intermediate Representations (IR)

The system maintains two distinct intermediate representations:

#### 1.1 Abugida IR
- Represents Indic scripts where consonants have an inherent vowel (typically 'a')
- Handles consonant clusters, vowel marks, and special characters
- Maintains the syllabic structure of Indic languages

#### 1.2 Alphabet IR  
- Represents Roman/Latin scripts where all sounds are explicitly written
- Supports both ASCII-only romanizations (e.g., Harvard-Kyoto, SLP1) and Unicode-based romanizations (e.g., IAST, ISO 15919)

### 2. Compilation Pipeline

```
Source Script → Parser → Source IR → Transformer → Target IR → Generator → Target Script
```

- **Parser**: Converts input text to appropriate IR (Abugida or Alphabet)
- **Transformer**: Maps between Abugida IR ↔ Alphabet IR
- **Generator**: Converts IR to output script

### 3. Transliteration Modes

The system supports four transliteration modes:
1. **Abugida → Abugida**: e.g., Devanagari → Telugu
2. **Abugida → Alphabet**: e.g., Devanagari → IAST
3. **Alphabet → Abugida**: e.g., Harvard-Kyoto → Devanagari
4. **Alphabet → Alphabet**: e.g., IAST → Harvard-Kyoto

## Key Features

### 1. Runtime Extensibility
- Schema-based configuration using YAML/TOML
- Allows handling of idiosyncratic sources and edge cases
- Dynamic loading of custom mappings without recompilation

### 2. Performance Optimization
- Zero-copy parsing where possible
- Efficient string building using pre-allocated buffers
- Lazy evaluation for complex transformations
- SIMD optimizations for batch processing

### 3. Quality Assurance
- Comprehensive round-trip testing for all script pairs (n² tests for n scripts)
- Property-based testing for edge cases
- Benchmarking against existing solutions (Vidyut, Aksharamukha, Dharmamitra)

## Implementation Details

### 1. Core Data Structures

```rust
// Simplified representation
enum AbugidaElement {
    Consonant { base: char, inherent_vowel: bool },
    VowelMark(char),
    Virama,
    Nukta,
    // ... other elements
}

enum AlphabetElement {
    Consonant(String),
    Vowel(String),
    Modifier(String),
    // ... other elements
}

struct AbugidaIR {
    elements: Vec<AbugidaElement>,
    metadata: Metadata,
}

struct AlphabetIR {
    elements: Vec<AlphabetElement>,
    metadata: Metadata,
}
```

### 2. Schema Format

```yaml
# Example schema for a script
name: "Devanagari"
type: "abugida"
mappings:
  vowels:
    "अ": { canonical: "a", inherent: true }
    "आ": { canonical: "ā" }
  consonants:
    "क": { canonical: "ka", pure: "k" }
    "ख": { canonical: "kha", pure: "kh" }
  modifiers:
    "्": { type: "virama" }
    "़": { type: "nukta" }
extensions:
  # Custom mappings for specific use cases
```

### 3. API Design

```rust
// Core API
pub struct Transliterator {
    // Internal state
}

impl Transliterator {
    pub fn new() -> Self;
    pub fn load_schema(&mut self, schema: Schema) -> Result<(), Error>;
    pub fn transliterate(&self, input: &str, from: Script, to: Script) -> Result<String, Error>;
    pub fn with_extensions(&mut self, extensions: Extensions) -> &mut Self;
}
```

## Language Bindings

### 1. Python Bindings (PyO3)
- Pythonic API with type hints
- Support for streaming transliteration
- Integration with popular NLP libraries

### 2. WebAssembly Bindings
- Browser-compatible builds
- Minimal bundle size
- TypeScript definitions

## Testing Strategy

### 1. Unit Tests
- Individual component testing
- Edge case coverage
- Error handling verification

### 2. Integration Tests
- Full pipeline testing
- Round-trip verification
- Performance regression tests

### 3. Property-Based Tests
- Invariant verification
- Fuzz testing for robustness

## Performance Targets

- **Throughput**: > 1GB/s for simple transliterations
- **Latency**: < 1μs for single word transliteration
- **Memory**: O(n) space complexity with minimal allocations

## Supported Scripts (Initial)

### Indic Scripts (Abugidas)
1. Devanagari
2. Telugu
3. Tamil (with special handling for pure consonants)
4. Kannada
5. Malayalam

### Romanization Schemes (Alphabets)
1. IAST (International Alphabet of Sanskrit Transliteration)
2. ISO 15919
3. Harvard-Kyoto
4. SLP1 (Sanskrit Library Phonetic)
5. ITRANS

## Future Enhancements

1. Support for additional scripts (Bengali, Gujarati, etc.)
2. Context-aware transliteration for ambiguous cases
3. Machine learning models for handling non-standard text
4. Parallel processing for large documents
5. Real-time transliteration with incremental updates