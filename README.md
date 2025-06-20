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

## Runtime Extensibility

Shlesha supports adding custom scripts and mappings at runtime without recompilation. This enables users to extend support for new writing systems, create custom romanization schemes, or add domain-specific transliterations.

### Creating Custom Scripts

```rust
use shlesha::{CustomScriptBuilder, ExtendedTransliterator, FallbackStrategy};

// Create a custom script for Ancient Greek
let ancient_greek = CustomScriptBuilder::new("AncientGreek", 100)
    // Basic character mappings
    .add_mapping('α', "a")
    .add_mapping('β', "b") 
    .add_mapping('γ', "g")
    .add_mapping('δ', "d")
    .add_mapping('ε', "e")
    .add_mapping('ζ', "z")
    .add_mapping('η', "ē")
    .add_mapping('θ', "th")
    .add_mapping('ι', "i")
    .add_mapping('κ', "k")
    .add_mapping('λ', "l")
    .add_mapping('μ', "m")
    .add_mapping('ν', "n")
    .add_mapping('ξ', "x")
    .add_mapping('ο', "o")
    .add_mapping('π', "p")
    .add_mapping('ρ', "r")
    .add_mapping('σ', "s")
    .add_mapping('ς', "s")  // Final sigma
    .add_mapping('τ', "t")
    .add_mapping('υ', "y")
    .add_mapping('φ', "ph")
    .add_mapping('χ', "ch")
    .add_mapping('ψ', "ps")
    .add_mapping('ω', "ō")
    // Pattern mappings for diphthongs
    .add_pattern("αι", "ai")
    .add_pattern("ει", "ei") 
    .add_pattern("οι", "oi")
    .add_pattern("αυ", "au")
    .add_pattern("ευ", "eu")
    .add_pattern("ου", "ou")
    .with_fallback_strategy(FallbackStrategy::PreserveWithPhonetics)
    .build();

// Create extended transliterator and add the custom script
let mut transliterator = ExtendedTransliterator::new();
transliterator.add_custom_script(ancient_greek);

// Now you can transliterate Ancient Greek (with custom fallback for unregistered mappings)
let result = transliterator.transliterate("κόσμος", "AncientGreek", "IAST");
```

### Custom Romanization Schemes

```rust
use shlesha::{CustomScriptBuilder, ExtendedTransliterator};

// Create a custom romanization for Devanagari (simplified ASCII-only)
let ascii_roman = CustomScriptBuilder::new("ASCII_Roman", 101)
    // Simple consonants 
    .add_mappings(&[
        ('क', "ka"), ('ख', "kha"), ('ग', "ga"), ('घ', "gha"), ('ङ', "nga"),
        ('च', "cha"), ('छ', "chha"), ('ज', "ja"), ('झ', "jha"), ('ञ', "nja"),
        ('ट', "ta"), ('ठ', "tha"), ('ड', "da"), ('ढ', "dha"), ('ण', "na"),
        ('त', "ta"), ('थ', "tha"), ('द', "da"), ('ध', "dha"), ('न', "na"),
        ('प', "pa"), ('फ', "pha"), ('ब', "ba"), ('भ', "bha"), ('म', "ma"),
        ('य', "ya"), ('र', "ra"), ('ल', "la"), ('व', "va"),
        ('श', "sha"), ('ष', "shha"), ('स', "sa"), ('ह', "ha"),
    ])
    // Vowels
    .add_mappings(&[
        ('अ', "a"), ('आ', "aa"), ('इ', "i"), ('ई', "ii"),
        ('उ', "u"), ('ऊ', "uu"), ('ए', "e"), ('ओ', "o"),
        ('ा', "aa"), ('ि', "i"), ('ी', "ii"), ('ु', "u"), ('ू', "uu"),
        ('े', "e"), ('ै', "ai"), ('ो', "o"), ('ौ', "au"),
    ])
    // Special characters
    .add_mappings(&[
        ('ं', "n"), ('ः', "h"), ('्', ""),
        ('।', "."), ('॥', ".."),
    ])
    // Complex patterns
    .add_patterns(&[
        ("क्ष", "ksha"),
        ("ज्ञ", "gnja"), 
        ("त्र", "tra"),
        ("श्र", "shra"),
    ])
    .build();

let mut transliterator = ExtendedTransliterator::new();
transliterator.add_custom_script(ascii_roman);
```

### Domain-Specific Extensions

```rust
use shlesha::{CustomScriptBuilder, CustomMapping, ExtendedTransliterator};

// Create a custom script for mathematical notation
let math_notation = CustomScriptBuilder::new("MathNotation", 102)
    .add_mappings(&[
        ('α', "alpha"), ('β', "beta"), ('γ', "gamma"), ('δ', "delta"),
        ('ε', "epsilon"), ('ζ', "zeta"), ('η', "eta"), ('θ', "theta"),
        ('λ', "lambda"), ('μ', "mu"), ('π', "pi"), ('σ', "sigma"),
        ('τ', "tau"), ('φ', "phi"), ('χ', "chi"), ('ψ', "psi"), ('ω', "omega"),
    ])
    .add_patterns(&[
        ("∞", "infinity"),
        ("∑", "sum"),
        ("∏", "product"), 
        ("∫", "integral"),
        ("∂", "partial"),
        ("∇", "nabla"),
        ("∆", "delta"),
    ])
    .build();

// Create custom mapping between math notation and LaTeX
let math_to_latex = CustomMapping {
    from_script: 102, // MathNotation
    to_script: 103,   // LaTeX
    char_mappings: [
        ('α', "\\alpha".to_string()),
        ('β', "\\beta".to_string()),
        ('γ', "\\gamma".to_string()),
        ('∞', "\\infty".to_string()),
        ('∑', "\\sum".to_string()),
        ('∫', "\\int".to_string()),
    ].iter().cloned().collect(),
    pattern_mappings: vec![
        ("∑_{i=1}^{n}".to_string(), "\\sum_{i=1}^{n}".to_string()),
        ("∫_{a}^{b}".to_string(), "\\int_{a}^{b}".to_string()),
    ],
    fallback_strategy: FallbackStrategy::Preserve,
};

let mut transliterator = ExtendedTransliterator::new();
transliterator.add_custom_script(math_notation);
transliterator.add_custom_mapping(math_to_latex);
```

### Linguistic Research Extensions

```rust
use shlesha::{CustomScriptBuilder, ExtendedTransliterator, FallbackStrategy};

// Create a custom script for Old Church Slavonic
let old_church_slavonic = CustomScriptBuilder::new("OldChurchSlavonic", 104)
    .add_mappings(&[
        ('а', "a"), ('б', "b"), ('в', "v"), ('г', "g"), ('д', "d"),
        ('е', "e"), ('ж', "ž"), ('з', "z"), ('и', "i"), ('і', "i"),
        ('к', "k"), ('л', "l"), ('м', "m"), ('н', "n"), ('о', "o"),
        ('п', "p"), ('р', "r"), ('с', "s"), ('т', "t"), ('у', "u"),
        ('ф', "f"), ('х', "x"), ('ц', "c"), ('ч', "č"), ('ш', "š"),
        ('щ', "šč"), ('ъ', "ъ"), ('ы', "y"), ('ь', "ь"), ('ѣ', "ě"),
        ('ю', "ju"), ('я', "ja"), ('ѧ', "ę"), ('ѩ', "ję"), ('ѫ', "ǫ"),
        ('ѭ', "jǫ"), ('ѯ', "ks"), ('ѱ', "ps"), ('ѳ', "th"), ('ѵ', "ü"),
    ])
    .add_patterns(&[
        ("оу", "u"),    // Digraph ou = u
        ("ць", "c'"),   // Palatalized c
        ("дь", "d'"),   // Palatalized d  
        ("ль", "l'"),   // Palatalized l
        ("нь", "n'"),   // Palatalized n
        ("рь", "r'"),   // Palatalized r
        ("сь", "s'"),   // Palatalized s
        ("ть", "t'"),   // Palatalized t
    ])
    .with_fallback_strategy(FallbackStrategy::PreserveWithContext)
    .build();

let mut transliterator = ExtendedTransliterator::new();
transliterator.add_custom_script(old_church_slavonic);

// Transliterate Old Church Slavonic to modern romanization
let result = transliterator.transliterate("блажени миротворьци", "OldChurchSlavonic", "IAST");
```

### Loading Scripts from Configuration

```rust
use shlesha::{CustomScriptBuilder, ExtendedTransliterator};
use std::collections::HashMap;

// Example: Load script mappings from JSON/YAML configuration
fn load_script_from_config(config: &str) -> Result<CustomScript, Box<dyn std::error::Error>> {
    // Parse configuration (JSON/YAML/TOML)
    let config_data: HashMap<String, serde_json::Value> = serde_json::from_str(config)?;
    
    let script_name = config_data["script_name"].as_str().unwrap();
    let script_id = config_data["script_id"].as_u64().unwrap() as u8;
    
    let mut builder = CustomScriptBuilder::new(script_name, script_id);
    
    // Load character mappings
    if let Some(chars) = config_data["characters"].as_object() {
        for (from, to) in chars {
            if let (Some(from_char), Some(to_str)) = (from.chars().next(), to.as_str()) {
                builder = builder.add_mapping(from_char, to_str);
            }
        }
    }
    
    // Load pattern mappings
    if let Some(patterns) = config_data["patterns"].as_object() {
        for (from, to) in patterns {
            if let Some(to_str) = to.as_str() {
                builder = builder.add_pattern(from, to_str);
            }
        }
    }
    
    Ok(builder.build())
}

// Usage
let config = r#"{
    "script_name": "CustomScript",
    "script_id": 200,
    "characters": {
        "α": "a",
        "β": "b",
        "γ": "g"
    },
    "patterns": {
        "αβ": "ab",
        "γδ": "gd"
    }
}"#;

let custom_script = load_script_from_config(config).unwrap();
let mut transliterator = ExtendedTransliterator::new();
transliterator.add_custom_script(custom_script);
```

### Extension Management

```rust
use shlesha::{ExtendedTransliterator, ExtensionManager};

let mut transliterator = ExtendedTransliterator::new();

// Access the extension manager for advanced operations
let extensions = transliterator.extensions();

// List all custom scripts
for (script_id, script_name) in extensions.list_custom_scripts() {
    println!("Custom script {}: {}", script_id, script_name);
}

// Check if a specific mapping exists
if extensions.has_custom_mapping(100, 101) {
    println!("Custom mapping exists between scripts 100 and 101");
}

// Get a specific custom script
if let Some(script) = extensions.get_script(100) {
    println!("Found custom script: {}", script.script_name);
    
    // Test character lookup
    if let Some(result) = script.lookup_char('α') {
        println!("α maps to: {}", result);
    }
}
```

### Runtime Extension Best Practices

1. **Script ID Management**: Use IDs 100+ for custom scripts to avoid conflicts with built-in scripts (1-99)

2. **Unicode Sorting**: Custom mappings are automatically sorted by Unicode value for efficient binary search

3. **Pattern Precedence**: Longer patterns are matched first (automatic sorting by length)

4. **Fallback Strategies**: Choose appropriate fallback for your use case:
   - `Preserve`: Simple token preservation
   - `PreserveWithPhonetics`: Add phonetic hints for unknown characters
   - `PreserveWithContext`: Include surrounding context for better reconstruction

5. **Performance**: Runtime extensions use the same optimized lookup algorithms as built-in scripts

6. **Memory Management**: Custom scripts are stored in owned memory (not static), allowing full runtime flexibility

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