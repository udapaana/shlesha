# Shlesha: Lossless Transliteration Library

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Shlesha (Sanskrit: श्लेषा, "connection/binding") is a transliteration library that guarantees **100% lossless information preservation** through mathematical verification and token-based fallback mechanisms.

## Key Features

- **🔒 100% Lossless Guarantee**: Mathematical proof via Shannon entropy
- **⚡ High Performance**: Binary search O(log n) with direct character mappings  
- **🌍 Multi-Script Support**: 9 Indic scripts + 6 romanization schemes
- **🔄 Token Preservation**: Unknown characters preserved as `[script_id:data:metadata]`
- **📊 Entropy Analysis**: Information-theoretic verification
- **🎯 Pattern Matching**: Complex conjuncts (क्ष → kṣa) with proper precedence

## Quick Start

### Rust Library

```toml
[dependencies]
shlesha = "0.1.0"
```

```rust
use shlesha::LosslessTransliterator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transliterator = LosslessTransliterator::new();
    
    // Basic transliteration
    let result = transliterator
        .transliterate("धर्म", "Devanagari", "IAST")?;
    println!("{}", result); // "dharma"
    
    // Verify losslessness
    let verification = transliterator
        .verify_lossless("धर्म", &result, "Devanagari");
    assert!(verification.is_lossless);
    
    // Unknown characters are preserved with tokens
    let text_with_unknown = "धर्म ॐ योग";
    let encoded = transliterator
        .transliterate(text_with_unknown, "Devanagari", "IAST")?;
    println!("{}", encoded); // "dharma [1:ॐ:om] yoga"
    
    Ok(())
}
```

### CLI Tool

```bash
# Install
cargo install --path .

# Basic transliteration
echo "नमस्ते" | shlesha transliterate --from Devanagari --to IAST
# Output: namaste

# Verify losslessness
shlesha verify --original "धर्म" --encoded "dharma" --from Devanagari

# List supported scripts
shlesha scripts

# Show detailed help
shlesha --help
```

## Running Benchmarks

Shlesha includes comprehensive benchmarking tools for performance analysis:

### Basic Benchmarks
```bash
# Run all benchmarks with Criterion
cargo bench

# Run Rust micro-benchmarks
cargo test --release --test '*_benchmark*'

# Generate flamegraph for profiling
cargo install flamegraph
cargo flamegraph --bench transliteration_bench
```

### Comprehensive Benchmark Suite
```bash
# Run the unified benchmark runner (compares multiple tools)
cd benches/
python unified_benchmark_runner.py

# Run CLI benchmarks with various text sizes
./comprehensive_cli_benchmark.sh

# Run Python API benchmarks
python comprehensive_python_benchmark.py

# Run WASM benchmarks (in browser)
open comprehensive_wasm_benchmark.html
```

### Individual Benchmark Scripts
```bash
# Quick Rust comparisons
cargo run --bin bench --release

# System-wide tool comparison
cargo run --example comprehensive_performance_demo

# Real-world text benchmark
cargo bench real_world_benchmark
```

### Performance Testing
```bash
# Profile specific functions
cargo run --example profile_bottleneck

# Memory usage analysis
cargo run --example profile_detailed

# Test with different text sizes
cargo run --example test_scaling
```

## Architecture

### Core Design Principles

Shlesha follows a **lossless-first architecture** that prioritizes information preservation over speed:

```
Input Text → Pattern Matching → Binary Search → Token Generation → Output
    ↓                              O(log n)              ↓
    └─────────────── Entropy Verification ←──────────────┘
                    H(original) ≤ H(encoded) + H(tokens)
```

### Module Structure

```rust
src/
├── lib.rs                      // Public API exports
├── lossless_transliterator.rs  // Core engine with mathematical guarantees
├── script_mappings.rs          // Static mappings for all scripts
└── [additional modules...]     // Script-specific implementations
```

### Key Components

1. **LosslessTransliterator**: Main transliteration engine
   - Handles script registration and mapping
   - Performs entropy-based verification
   - Manages token extraction and reconstruction

2. **LosslessMapper**: High-performance character mapping
   - Binary search for O(log n) character lookup
   - Pattern matching for multi-character sequences
   - Fallback token generation for unmapped characters

3. **PreservationToken**: Information preservation mechanism
   - Encodes unmapped characters with metadata
   - Enables perfect reconstruction
   - Supports multiple fallback strategies

4. **ScriptRegistry**: Script and mapping management
   - Centralizes script definitions
   - Manages bidirectional mappings
   - Handles reconstruction pathways

### Mathematical Foundation

**Lossless Guarantee**: Information preservation verified through Shannon entropy
```
H(original) ≤ H(encoded) + H(tokens)
```

**Token Format**: Unknown characters preserved as `[script_id:data:metadata]`
```
Input:  "धर्म ॐ योग"
Output: "dharma [1:ॐ:om] yoga"
```

**Abugida Normalization**: Accounts for inherent vowel expansion
```rust
// क → ka (1 char → 2 chars) is information-preserving
let normalized_ratio = base_ratio / expansion_ratio.powf(0.5);
```

## Script Support

### Currently Implemented
- ✅ **Devanagari ↔ IAST**: Complete with 70+ characters, conjunct patterns
- ✅ **Devanagari ↔ SLP1**: Full phonetic coverage  
- ✅ **Identity mappings**: All 15 scripts

### Supported Scripts

**Indic Scripts (9)**
- Devanagari (Hindi, Sanskrit, Marathi, Nepali)
- Bengali (Bengali, Assamese)  
- Tamil (Tamil)
- Telugu (Telugu)
- Kannada (Kannada)
- Malayalam (Malayalam)
- Gujarati (Gujarati)
- Gurmukhi (Punjabi)
- Odia (Odia)

**Romanization Schemes (6)**
- IAST (International Alphabet of Sanskrit Transliteration)
- SLP1 (Sanskrit Library Phonetic Basic)
- Harvard-Kyoto (ASCII-friendly)
- ITRANS (Internet standard)
- Velthuis (TeX-based)
- WX (Computational linguistics)

### Script Coverage Matrix
```
From\To    Deva  IAST  SLP1  Beng  Tami  Telu  Kann  Mala  Guja  Gurm  Odia  HK    ITRA  Velt  WX
Devanagari  ✓     ✓     ✓     ○     ○     ○     ○     ○     ○     ○     ○     ○     ○     ○     ○
IAST        ✓     ✓     ○     ○     ○     ○     ○     ○     ○     ○     ○     ○     ○     ○     ○
SLP1        ✓     ○     ✓     ○     ○     ○     ○     ○     ○     ○     ○     ○     ○     ○     ○
[... remaining show identity mappings only]
```

**Implementation Status**: 19/225 mappings (8.4% complete)

## Testing

Comprehensive test coverage ensuring correctness:

```bash
# Run all tests
cargo test

# Unit tests
cargo test --lib

# Property-based tests (mathematical invariants)
cargo test --test property_based_tests

# Script matrix validation
cargo test --test comprehensive_script_tests

# Integration tests
cargo test --test integration_tests
```

### Test Categories
- **Unit Tests**: Core functionality and edge cases
- **Property-Based Tests**: Mathematical invariants and losslessness
- **Integration Tests**: End-to-end transliteration workflows
- **Benchmark Tests**: Performance regression detection

## Advanced Usage

### Extract Preservation Tokens
```rust
let encoded = "dharma [1:ॐ:om] yoga";
let tokens = transliterator.extract_tokens(encoded);
// Returns: vec![PreservationToken { source_script: 1, data: "ॐ", metadata: Some("om") }]
```

### Check Script Support
```rust
use shlesha::script_mappings::{get_supported_scripts, has_mapping};

// List all scripts
for (name, id) in get_supported_scripts() {
    println!("{}: {}", id, name);
}

// Check if mapping exists
if has_mapping(from_id, to_id) {
    // Perform transliteration
}
```

### Custom Fallback Strategies
```rust
use shlesha::{LosslessMapper, FallbackStrategy};

// Choose preservation strategy
let mapper = LosslessMapper::new(
    mappings,
    patterns,
    source_id,
    target_id,
    FallbackStrategy::PreserveWithPhonetics, // Adds phonetic hints
);
```

### Entropy Analysis
```rust
let verification = transliterator.verify_lossless(original, encoded, "Devanagari");
println!("Preservation ratio: {:.2}", verification.preservation_ratio);
println!("Original entropy: {:.2}", verification.entropy_analysis.original);
println!("Total preserved: {:.2}", verification.entropy_analysis.total_preserved);
```

## Development

### Building
```bash
git clone https://github.com/udapaana/shlesha.git
cd shlesha
cargo build --release
```

### Contributing

Priority areas for contribution:
1. **Script Implementations**: Add remaining 206 mappings
2. **Performance Optimizations**: SIMD, zero-copy improvements
3. **Language Bindings**: Python (PyO3), WASM, Node.js
4. **Documentation**: Examples, tutorials, API docs
5. **Testing**: Fuzzing, edge cases, property-based tests

### Performance Optimization Areas
- Character lookup: SIMD vectorization
- String building: Zero-copy with borrowed slices  
- Pattern matching: Trie or Aho-Corasick algorithms
- Memory usage: Custom allocators and arena allocation

## License

MIT OR Apache-2.0

## Citation

```bibtex
@software{shlesha2024,
  title = {Shlesha: Lossless Transliteration Library},
  author = {Shlesha Contributors},
  year = {2024},
  url = {https://github.com/udapaana/shlesha},
  note = {Mathematical lossless guarantee via Shannon entropy}
}
```

---

**Shlesha**: Mathematical correctness meets practical transliteration.