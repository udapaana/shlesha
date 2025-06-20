# Shlesha Usage Guide

Shlesha is a high-performance, extensible transliterator for Indic languages built with an LLVM-inspired architecture.

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
shlesha = { path = "path/to/shlesha" }
```

## Command Line Interface

### Basic Usage

```bash
# Simple text transliteration
cargo run --example cli -- -f devanagari -t iast "नमस्ते"
# Output: namaste

# From a file
cargo run --example cli -- -f devanagari -t iast -i input.txt

# To a file
cargo run --example cli -- -f devanagari -t iast -i input.txt -o output.txt

# Via pipe
echo "संस्कृतम्" | cargo run --example cli -- -f devanagari -t iast
# Output: saṃskṛtam
```

### Supported Scripts

**Indic Scripts (9):**
- `devanagari` - Devanagari script (देवनागरी)
- `bengali` - Bengali script (বাংলা)
- `tamil` - Tamil script (தமிழ்)
- `telugu` - Telugu script (తెలుగు)
- `kannada` - Kannada script (ಕನ್ನಡ)
- `malayalam` - Malayalam script (മലയാളം)
- `gujarati` - Gujarati script (ગુજરાતી)
- `odia` - Odia script (ଓଡ଼ିଆ)
- `gurmukhi` - Gurmukhi script (ਗੁਰਮੁਖੀ)

**Roman Schemes (6):**
- `iast` - International Alphabet of Sanskrit Transliteration
- `harvard-kyoto` - Harvard-Kyoto convention
- `itrans` - ITRANS scheme (ASCII-based)
- `slp1` - Sanskrit Library Phonetic Basic
- `velthuis` - Velthuis TeX transliteration
- `wx` - WX notation (IIIT Hyderabad)

### Options

- `-f, --from <SCRIPT>` - Source script (required)
- `-t, --to <SCRIPT>` - Target script (required)
- `-i, --input <FILE>` - Input file (default: stdin)
- `-o, --output <FILE>` - Output file (default: stdout)
- `-s, --schemas <DIR>` - Schema directory (default: ./schemas)
- `-e, --extension <NAME>` - Enable extension (can be repeated)
- `-v, --verbose` - Show performance metrics

### Examples

```bash
# Devanagari to IAST
cargo run --example cli -- -f devanagari -t iast "धर्मक्षेत्रे कुरुक्षेत्रे"
# Output: dharmakṣetre kurukṣetre

# IAST to Devanagari
cargo run --example cli -- -f iast -t devanagari "śāntam"
# Output: शान्तम्

# Cross-script transliteration via IAST
cargo run --example cli -- -f devanagari -t tamil "नमः"
# Output: நம꞉

# ASCII schemes
cargo run --example cli -- -f devanagari -t harvard-kyoto "धर्म"
# Output: dharma

cargo run --example cli -- -f slp1 -t devanagari "Darma"
# Output: धर्म

# With verbose metrics
cargo run --example cli -- -f devanagari -t iast -v "कर्म"
# Output includes timing and throughput data

# Large text processing
cargo run --example cli -- -f devanagari -t iast -i bhagavad_gita.txt -o gita_iast.txt -v
```

## Library Usage

### Basic Example

```rust
use shlesha::{TransliteratorBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build transliterator
    let transliterator = TransliteratorBuilder::new()
        .with_schema_directory("schemas")?
        .build();
    
    // Transliterate
    let input = "नमस्ते";
    let output = transliterator.transliterate(input, "Devanagari", "IAST")?;
    println!("{} → {}", input, output);
    // Output: नमस्ते → namaste
    
    Ok(())
}
```

### With Extensions

```rust
use shlesha::{TransliteratorBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut transliterator = TransliteratorBuilder::new()
        .with_schema_directory("schemas")?
        .build();
    
    // Add extension
    transliterator.add_extension("vedic_accents")?;
    
    let input = "वै॒श्वा॒न॒राय॑";
    let output = transliterator.transliterate(input, "Devanagari", "IAST")?;
    println!("{} → {}", input, output);
    
    Ok(())
}
```

## Performance

Shlesha is designed for high performance with:
- Zero-copy parsing where possible
- Efficient string interning
- Typed intermediate representation
- Minimal allocations

Typical throughput: 50-100 MB/s on modern hardware.

## Architecture

Shlesha uses a dual intermediate representation (IR) system:
- **Abugida IR**: For scripts like Devanagari where consonants have inherent vowels
- **Alphabet IR**: For scripts like IAST where each sound is a separate character

The pipeline: Parse → Transform → Generate

## Extensions

Extensions allow runtime modification of transliteration behavior without recompilation.

### Available Extensions
- `vedic_accents` - Support for Vedic accent marks
- `musical_notation` - Support for musical notation symbols

### Creating Extensions

See [EXTENSIONS.md](EXTENSIONS.md) for details on creating custom extensions.

## Testing

```bash
# Run all tests
cargo test

# Run comprehensive matrix tests (all 15×15 = 225 script pairs)
cargo test --test full_matrix_tests -- --nocapture

# Run specific test suite
cargo test parser

# Verify expected test values
cargo test --test verify_expected_values -- --nocapture

# Run benchmarks
cargo bench
```

### Matrix Testing

Shlesha includes comprehensive round-trip testing across all supported scripts:

- **Identity tests**: Same script conversions (15 tests)
- **Round-trip tests**: Source → Target → Source (210 tests)  
- **Total coverage**: 18,675 test cases (225 pairs × 83 test cases)
- **Test categories**: 
  - All consonants (velars, palatals, retroflexes, dentals, labials, semi-vowels, sibilants)
  - Independent vowels and vowel combinations
  - Common conjuncts and complex clusters
  - Anusvara, visarga, and modifiers
  - Real Sanskrit/Hindi words
  - Numerals and punctuation
  - Script-specific features

### Specialized Testing

- **Conjunct coverage**: 23 complex conjuncts with 100% success rate
- **Vowel combinations**: 28 vowel forms with 100% success rate  
- **Real-world words**: 24 Sanskrit terms with 100% success rate
- **Numerals/punctuation**: 14 symbols with 92.9% success rate
- **Script-specific features**: Tamil, Bengali, Telugu, Devanagari specializations

## Notes

- The transliterator preserves round-trip fidelity where possible
- Unknown characters are passed through unchanged
- Case sensitivity depends on the target script