# Shlesha (श्लेष)

A high-performance, linguistically-accurate transliteration engine for Vedic Sanskrit texts, supporting multiple scripts and encoding schemes.

> Named after the Sanskrit concept of śleṣa (श्लेष), meaning "embrace" or "union" - symbolizing the binding together of different script systems.

## Overview

This project provides a Rust-based transliteration engine specifically designed for Vedic texts. A key challenge in working with Vedic sources is that different traditions and manuscript repositories often develop custom encoding extensions to accurately represent their specific corpus. These extensions may include unique accent marks, special characters, or tradition-specific conventions that go beyond standard transliteration schemes.

Shlesha addresses this by providing:

- **Runtime Extensibility**: Both input parsing and output generation can be customized at runtime through configuration
- **Source-Aware Processing**: Each text source can have its own encoding conventions without affecting others
- **Multiple Scripts**: Devanagari, IAST, ISO 15919, Harvard-Kyoto, ITRANS, SLP1, Telugu, and more
- **Vedic Accents**: Proper handling of Udatta, Anudatta, and Svarita marks with source-specific variations
- **Tradition-Specific Conventions**: Support for VedaNidhi, VedaVMS, and other repository-specific encoding schemes
- **High Performance**: Built in Rust for speed and memory efficiency
- **Extensible Schema System**: TOML-based configuration allows easy addition of new traditions and conventions

## Features

### Base Schemas

Shlesha includes base transliteration schemas organized by script families:

#### Brahmic Scripts (Indian subcontinent)
- **devanagari** - Standard Devanagari script (Hindi, Sanskrit, Marathi)
- **bengali** - Bengali (Bangla) script (Bengali, Assamese, Sanskrit)
- **gujarati** - Gujarati script (Gujarati, Sanskrit)
- **gurmukhi** - Gurmukhi script (Punjabi)
- **kannada** - Kannada script (Kannada, Sanskrit)
- **telugu** - Telugu script (Telugu, Sanskrit)

#### Romanization Systems
- **iast** - International Alphabet of Sanskrit Transliteration
- **iso15919** - ISO 15919 standard for Indic transliteration
- **harvard_kyoto** - Harvard-Kyoto ASCII convention
- **itrans** - ITRANS encoding scheme
- **slp1** - Sanskrit Library Phonetic encoding
- **velthuis** - Velthuis ASCII system for TeX
- **wx** - WX notation (IIIT Hyderabad)

#### Other Scripts
- **arabic** - Arabic script (Arabic, Persian, Urdu)
- **cyrillic** - Cyrillic script (Russian, other Slavic languages)

### Vedic Extensions

Vedic-specific extensions and source-specific schemas are maintained separately with the corpus data. These include:

- **Vedic accent marks** and special characters
- **Tradition-specific conventions** (VedaNidhi, VedaVMS, etc.)
- **Source-specific overrides** for individual texts

When using Shlesha with Vedic texts, specify the path to these additional schemas.

### Vedic Features

- Accurate representation of Vedic accents (स्वर)
- Support for Anunasika and other Vedic-specific marks
- Handling of Samhita, Pada, and other text types
- Source-specific encoding conventions

## Project Structure

```
shlesha/
├── vedic_transliterator_rs/     # Rust transliteration engine
│   ├── src/                     # Source code
│   ├── Cargo.toml               # Rust dependencies
│   └── build_python_module.py   # Python bindings builder
└── schemas/                     # Base encoding schemes
    ├── brahmic/                 # Indian subcontinent scripts
    │   ├── devanagari.toml
    │   ├── bengali.toml
    │   ├── gujarati.toml
    │   ├── gurmukhi.toml
    │   ├── kannada.toml
    │   └── telugu.toml
    ├── romanization/            # Latin-based systems
    │   ├── iast.toml
    │   ├── iso15919.toml
    │   ├── harvard_kyoto.toml
    │   ├── itrans.toml
    │   ├── slp1.toml
    │   ├── velthuis.toml
    │   └── wx.toml
    └── other/                   # Other script families
        ├── arabic.toml
        └── cyrillic.toml
```

Vedic extensions and source-specific schemas reside with the corpus data and can be loaded at runtime.

## Installation

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Python 3.8+ (for Python bindings)
- wasm-pack (for WASM builds) - `cargo install wasm-pack`

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/shlesha.git
cd shlesha

# Build the Rust CLI tool
cd vedic_transliterator_rs
cargo build --release

# Build Python bindings (requires PyO3)
cargo build --release --features python
python build_python_module.py

# Build WASM module
wasm-pack build --target web --features wasm
```

### Platform Support

- **Native CLI**: Full performance on Linux, macOS, Windows
- **Python Module**: Available via PyO3 bindings for Python 3.8+
- **WASM/Browser**: Run directly in web browsers
- **Node.js**: Use via WASM bindings

## Usage

### Command Line

```bash
# Transliterate a file
shlesha --from devanagari --to iast input.txt output.txt

# Specify source-specific schema from corpus
shlesha --from devanagari --to iast --schema-dir ../data/schemas/vedic input.txt
```

### Rust API

```rust
use shlesha::{Transliterator, Script};

let transliterator = Transliterator::new(Script::Devanagari, Script::IAST)?;
let result = transliterator.transliterate("अग्निमीळे पुरोहितं")?;
// Output: "agnimīḻe purohitaṃ"
```

### Python API

```python
from shlesha import Transliterator

# Basic transliteration
trans = Transliterator(from_script="devanagari", to_script="iast")
result = trans.transliterate("अग्निमीळे पुरोहितं")
# Output: "agnimīḻe purohitaṃ"

# With Vedic schema directory
trans = Transliterator(
    from_script="devanagari", 
    to_script="iast",
    schema_dir="../data/schemas/vedic"
)
result = trans.transliterate("अग्निमीळे पुरोहितं")
```

### JavaScript/WASM API

```javascript
import init, { Transliterator } from './shlesha_wasm.js';

async function transliterate() {
    await init();
    
    const trans = new Transliterator("devanagari", "iast");
    const result = trans.transliterate("अग्निमीळे पुरोहितं");
    console.log(result); // "agnimīḻe purohitaṃ"
}
```

### Browser Usage

```html
<script type="module">
import init, { Transliterator } from './shlesha_wasm.js';

await init();
const trans = new Transliterator("devanagari", "iast");

// Use in your web application
document.getElementById('output').textContent = 
    trans.transliterate(document.getElementById('input').value);
</script>
```

## Schema System

The transliteration system uses TOML-based schemas that define mappings at multiple levels, allowing precise control over how each tradition's texts are processed:

1. **Base Schemas**: Core character mappings for each script (Devanagari, IAST, etc.)
2. **Vedic Extensions**: Additional mappings for Vedic-specific characters and accents
3. **Source Overrides**: Custom conventions for specific text sources and traditions

This layered approach ensures that:
- Common transliterations are defined once and reused
- Tradition-specific variations can override base mappings
- New sources can be added without modifying core code
- Runtime configuration allows dynamic adaptation to different text sources

Example schema structure:

```toml
[metadata]
name = "devanagari_vedic"
description = "Devanagari with Vedic extensions"
base = "devanagari"

[mappings]
"अ" = "a"
"आ" = "ā"

[vedic_marks]
udatta = "\u0951"
anudatta = "\u0952"
```

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Install development dependencies
cargo install cargo-watch

# Run tests
cargo test

# Watch for changes
cargo watch -x test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Based on transliteration principles from traditional Vedic scholarship
- Inspired by existing Sanskrit processing tools
- Schema design influenced by Unicode standards for Vedic texts

## Related Projects

- [Vedic Texts Corpus](https://github.com/yourusername/vedic-texts) - Source texts for transliteration
- [Sanskrit NLP Tools](https://github.com/sanskrit/tools) - General Sanskrit processing

## Citation

If you use this software in your research, please cite:

```bibtex
@software{shlesha,
  title = {Shlesha: A High-Performance Transliteration Engine for Vedic Sanskrit},
  author = {Your Name},
  year = {2024},
  url = {https://github.com/yourusername/shlesha}
}
```