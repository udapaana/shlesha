# Shlesha - Sanskrit Transliteration Library

A high-performance, comprehensive transliteration library for Sanskrit and Indic scripts with bidirectional conversion support.

## 🚀 Quick Start for Developers

**New to Shlesha?** Get up and running in one command:

```bash
./scripts/quick-start.sh
```

This sets up everything: Rust environment, Python bindings, WASM support, and runs all tests.

**For detailed setup instructions**, see [DEVELOPER_SETUP.md](DEVELOPER_SETUP.md).

**📚 Complete Documentation**: See [DOCUMENTATION_INDEX.md](DOCUMENTATION_INDEX.md) for all guides and references.

---

## Overview

Shlesha implements a hub-and-spoke architecture for transliteration between Sanskrit/Indic scripts and romanization schemes. The system uses Devanagari and ISO-15919 as central hub formats, enabling efficient conversion between any supported script pair.

## Architecture

### Hub-and-Spoke Design

- **Hub Scripts**: Devanagari (for Indic scripts) ↔ ISO-15919 (for romanizations)
- **Indic Scripts**: Convert directly to/from Devanagari using character-to-character mapping
- **Romanization Schemes**: Convert directly to/from ISO-15919 using transliteration rules
- **Cross-conversion**: Indic ↔ Roman goes through the hub (Devanagari ↔ ISO-15919)

### Smart Routing

The system automatically determines the optimal conversion path:
- Same-script conversions: Direct passthrough
- Hub scripts: Direct passthrough when appropriate
- Cross-script: Automatic routing through hub formats

## Supported Scripts

### Indic Scripts (with implicit 'a')
- **Devanagari** (`devanagari`, `deva`) - Hindi, Sanskrit, Marathi
- **Bengali** (`bengali`, `bn`) - Bengali/Bangla script  
- **Tamil** (`tamil`, `ta`) - Tamil script
- **Telugu** (`telugu`, `te`) - Telugu script
- **Gujarati** (`gujarati`, `gu`) - Gujarati script
- **Kannada** (`kannada`, `kn`) - Kannada script
- **Malayalam** (`malayalam`, `ml`) - Malayalam script
- **Odia** (`odia`, `od`, `oriya`) - Odia/Oriya script

### Romanization Schemes (without implicit 'a')
- **ISO-15919** (`iso15919`, `iso_15919`, `iso`) - International standard
- **IAST** (`iast`) - International Alphabet of Sanskrit Transliteration
- **ITRANS** (`itrans`) - Indian languages TRANSliteration
- **SLP1** (`slp1`) - Sanskrit Library Phonetic Basic
- **Harvard-Kyoto** (`harvard_kyoto`, `hk`) - ASCII-based scheme
- **Velthuis** (`velthuis`) - TeX-compatible scheme  
- **WX** (`wx`) - ASCII-based notation

## Usage

### Rust Library

```rust
use shlesha::Shlesha;

let transliterator = Shlesha::new();

// Convert between any supported scripts
let result = transliterator.transliterate("धर्म", "devanagari", "gujarati")?;
println!("{}", result); // "ધર્મ"

let result = transliterator.transliterate("dharma", "iast", "devanagari")?;
println!("{}", result); // "धर्म"
```

### Python Bindings

Install and use via Python:

```bash
# Install (requires maturin)
pip install maturin
maturin develop --features python

# Or build wheel
maturin build --features python
```

```python
import shlesha

# Create transliterator
transliterator = shlesha.Shlesha()

# Basic transliteration
result = transliterator.transliterate("धर्म", "devanagari", "iast")
print(result)  # "dharma"

# Convenience function
result = shlesha.transliterate("धर्म", "devanagari", "iast") 
print(result)  # "dharma"

# With metadata for unknown tokens
result = transliterator.transliterate_with_metadata("धर्मkr", "devanagari", "iast")
print(result.output)  # "dharmakr"
print(len(result.metadata.unknown_tokens))  # Number of unknown chars

# List supported scripts
scripts = shlesha.get_supported_scripts()
print("devanagari" in scripts)  # True
```

### WebAssembly (JavaScript)

Build for web use:

```bash
# Install wasm-pack
cargo install wasm-pack

# Build for web
wasm-pack build --target web --out-dir pkg --features wasm

# Build for Node.js
wasm-pack build --target nodejs --out-dir pkg-node --features wasm
```

```javascript
import init, { WasmShlesha, transliterate } from './pkg/shlesha.js';

async function main() {
    // Initialize WASM module
    await init();
    
    // Create transliterator
    const transliterator = new WasmShlesha();
    
    // Basic transliteration
    const result = transliterator.transliterate("धर्म", "devanagari", "iast");
    console.log(result); // "dharma"
    
    // Convenience function
    const result2 = transliterate("धर्म", "devanagari", "iast");
    console.log(result2); // "dharma"
    
    // With metadata
    const resultWithMeta = transliterator.transliterateWithMetadata("धर्मkr", "devanagari", "iast");
    console.log(resultWithMeta.getOutput()); // "dharmakr"
    console.log(resultWithMeta.getUnknownTokenCount()); // Number of unknown chars
    
    // List scripts
    const scripts = transliterator.listSupportedScripts();
    console.log(scripts.includes("devanagari")); // true
}

main();
```

### Command Line Interface

```bash
# Install CLI
cargo install --path . --features cli

# Basic conversion
shlesha transliterate --from devanagari --to iast "धर्म"

# With metadata display
shlesha transliterate --from devanagari --to iast --show-metadata "धर्मkr"
shlesha transliterate --from devanagari --to iast --verbose "धर्मkr"

# List supported scripts
shlesha scripts
```

### Script Discovery

```rust
// List all supported scripts
let scripts = transliterator.list_supported_scripts();

// Check if a script is supported
let is_supported = transliterator.supports_script("gujarati");
```

### Metadata Collection

```rust
// Convert with metadata collection for unknown tokens
let result = transliterator.transliterate_with_metadata("धर्म", "devanagari", "iast")?;
println!("Output: {}", result.output);

if let Some(metadata) = result.metadata {
    println!("Source: {} -> Target: {}", metadata.source_script, metadata.target_script);
    for unknown in metadata.unknown_tokens {
        println!("Unknown token: {} at position {}", unknown.token, unknown.position);
    }
}
```

### CLI Usage

```bash
# Basic conversion
shlesha convert "धर्म" --from devanagari --to gujarati

# List supported scripts  
shlesha scripts

# Convert files
shlesha convert-file input.txt --from iast --to devanagari --output output.txt
```

## Bidirectional Conversion

All supported scripts have full bidirectional conversion capability:

### Script Pairs Supported
- **110 total conversion pairs** (11 scripts × 10 other scripts)
- All Indic ↔ Indic conversions
- All Indic ↔ Roman conversions  
- All Roman ↔ Roman conversions

### Examples

```rust
// Indic to Roman
transliterator.transliterate("ધર્મ", "gujarati", "iast")?; // "dharma"

// Roman to Indic  
transliterator.transliterate("dharma", "itrans", "bengali")?; // "ধর্ম"

// Cross-Indic (via hub)
transliterator.transliterate("ధర్మ", "telugu", "tamil")?; // "தர்மம்"
```

## Features

### Core Capabilities
- ✅ **Bidirectional conversion** between all script pairs
- ✅ **Hub-and-spoke architecture** for optimal performance
- ✅ **Character-to-character mapping** for Indic scripts
- ✅ **Smart routing** with automatic path detection
- ✅ **Virama handling** for proper consonant representation
- ✅ **Zero-copy optimizations** where possible
- ✅ **Graceful unknown character handling** with metadata tracking

### Multi-Language Support
- ✅ **Rust library** - Native performance and safety
- ✅ **Python bindings** - PyO3-based with full feature parity
- ✅ **WebAssembly bindings** - Browser and Node.js support
- ✅ **Command Line Interface** - Easy integration with shell scripts
- ✅ **Metadata collection** - Track unknown tokens and conversion details

### Script Classification
- ✅ **Implicit 'a' detection** - Indic scripts vs. romanizations
- ✅ **Script aliases** - Multiple names per script
- ✅ **Comprehensive coverage** - Major Sanskrit/Indic scripts

### Quality Assurance
- ✅ **193 passing tests** for all conversion pairs and features
- ✅ **Property-based testing** for edge cases
- ✅ **Roundtrip validation** for data integrity
- ✅ **Comprehensive benchmarks** for performance

## Performance

- **Character-to-character mapping** for O(n) complexity
- **Zero-allocation paths** for hub script passthroughs
- **Optimized string handling** with minimal copying
- **Benchmarked against** other transliteration libraries

## Testing

The library includes comprehensive test suites:

```bash
# Run all tests
cargo test

# Run specific test suites
cargo test comprehensive_bidirectional_tests
cargo test integration_tests
cargo test property_based_tests
cargo test unknown_handler_tests

# Run benchmarks
cargo bench
```

### Test Coverage
- **Integration tests**: All script converter functionality
- **Bidirectional tests**: Complete conversion matrix (110 pairs)
- **Property-based tests**: Edge cases and invariants
- **Roundtrip tests**: Data integrity validation

## Technical Details

### Hub Processing
- **Devanagari ↔ ISO-15919**: Central conversion in hub module
- **Virama handling**: Proper consonant cluster processing  
- **Implicit vowel processing**: Script-aware 'a' insertion/removal

### Character Mapping
- **Simple lookups**: HashMap-based character conversion
- **Nukta support**: Extended character sets (Bengali ড়, etc.)
- **Bidirectional maps**: Reverse conversion capability

### Unknown Token Handling
- **Zero-overhead tracking**: Unknown characters pass through by default
- **Optional metadata collection**: Track unknown tokens when needed
- **Position tracking**: Know where unknowns occurred
- **Extension awareness**: Distinguish runtime extension unknowns

### Error Handling
- **Graceful degradation**: Unknown characters preserved
- **Clear error messages**: Descriptive conversion failures
- **Script validation**: Early detection of unsupported formats

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please see CONTRIBUTING.md for guidelines.

## Acknowledgments

- Based on Unicode standards for Indic script processing
- ISO-15919 standard for romanization
- Sanskrit Library for SLP1 encoding
- Harvard-Kyoto conventions for ASCII transliteration