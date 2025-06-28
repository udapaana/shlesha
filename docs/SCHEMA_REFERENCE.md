# Schema Reference Guide

This document provides a comprehensive reference for Shlesha's YAML schema format used to define script converters.

## Overview

Shlesha uses YAML schemas to define how text should be converted between different scripts. These schemas serve dual purposes:

1. **Build-time code generation** - Processed during compilation to generate highly optimized Rust converters
2. **Runtime schema loading** - Can be dynamically loaded to add new script support without recompilation

Both systems use the **identical unified schema format** for consistency.

## Schema Format

All schemas follow this basic structure:

```yaml
metadata:
  name: "script_name"
  script_type: "roman" | "brahmic"
  description: "Human-readable description"
  # Optional fields
  aliases: ["alias1", "alias2"]
  has_implicit_a: true | false

target: "iso15919" | "devanagari"  # Optional, defaults to "devanagari"

mappings:
  vowels:
    "source": "target"
  consonants:
    "source": "target"
  marks:
    "source": "target"
  special:
    "source": "target"
  digits:
    "source": "target"

# Optional advanced configuration
codegen:
  type: "extended" | "standard"
```

## Metadata Section

### Required Fields

- **`name`**: Unique identifier for the script (lowercase, no spaces)
- **`script_type`**: Either `"roman"` for Latin-based scripts or `"brahmic"` for Indic scripts
- **`description`**: Human-readable description of the script

### Optional Fields

- **`aliases`**: Array of alternative names for the script
- **`has_implicit_a`**: Boolean indicating if the script has implicit 'a' vowels (relevant for Brahmic scripts)

## Target Field

Specifies the conversion target:

- **`"iso15919"`**: Convert TO ISO-15919 standard (typically for Roman scripts)
- **`"devanagari"`**: Convert TO Devanagari (typically for Indic scripts)
- **Omitted**: Defaults to `"devanagari"`

## Mappings Section

### Structure

All mappings use simple key-value pairs:

```yaml
mappings:
  category:
    "source_character": "target_character"
```

### Categories

#### vowels
Basic vowel sounds:
```yaml
vowels:
  "a": "a"
  "ā": "ā"
  "i": "i"
  "ī": "ī"
  # ... etc
```

#### consonants
Consonant characters:
```yaml
consonants:
  "k": "k"
  "kh": "kh"
  "g": "g"
  # ... etc
```

#### marks
Diacritical marks and special symbols:
```yaml
marks:
  "ṃ": "ṁ"      # anusvara
  "ḥ": "ḥ"      # visarga
  "'": "'"      # avagraha
```

#### special
Special character combinations:
```yaml
special:
  "kṣ": "kṣ"    # conjunct consonants
  "jñ": "jñ"
  "|": "।"      # punctuation
```

#### digits
Numeric characters:
```yaml
digits:
  "0": "0"
  "1": "1"
  # ... etc
```

## Schema Types

### Roman Scripts (`script_type: "roman"`)

Roman scripts typically convert TO ISO-15919:

```yaml
metadata:
  name: "iast"
  script_type: "roman"
  description: "International Alphabet of Sanskrit Transliteration"

target: "iso15919"

mappings:
  vowels:
    "ṛ": "r̥"     # IAST uses ṛ, ISO uses r̥
    "ṝ": "r̥̄"    # IAST uses ṝ, ISO uses r̥̄
  consonants:
    "k": "k"      # Most consonants are identical
  marks:
    "ṃ": "ṁ"      # IAST anusvara vs ISO anusvara
```

### Indic Scripts (`script_type: "brahmic"`)

Indic scripts typically convert TO Devanagari:

```yaml
metadata:
  name: "bengali"
  script_type: "brahmic"
  description: "Bengali/Bangla script"
  has_implicit_a: true

mappings:
  vowels:
    "অ": "अ"      # Bengali A → Devanagari A
    "আ": "आ"      # Bengali AA → Devanagari AA
  consonants:
    "ক": "क"      # Bengali KA → Devanagari KA
    "খ": "ख"      # Bengali KHA → Devanagari KHA
```

## Advanced Configuration

### CodeGen Options

For scripts requiring special processing:

```yaml
codegen:
  type: "extended"  # vs "standard"
```

- **`"standard"`**: Basic character-by-character conversion (default)
- **`"extended"`**: Advanced processing with context awareness

## Complete Examples

### Roman Script Example (IAST)

```yaml
metadata:
  name: "iast"
  script_type: "roman"
  has_implicit_a: false
  description: "International Alphabet of Sanskrit Transliteration"

target: "iso15919"

mappings:
  vowels:
    "a": "a"
    "ā": "ā"
    "i": "i"
    "ī": "ī"
    "u": "u"
    "ū": "ū"
    "ṛ": "r̥"     # Key difference: IAST ṛ → ISO r̥
    "ṝ": "r̥̄"    # Key difference: IAST ṝ → ISO r̥̄
    "ḷ": "l̥"
    "ḹ": "l̥̄"
    "e": "e"
    "ai": "ai"
    "o": "o"
    "au": "au"
  
  consonants:
    "k": "k"
    "kh": "kh"
    # ... (most are identical)
  
  marks:
    "ṃ": "ṁ"      # IAST ṃ → ISO ṁ (anusvara)
    "ḥ": "ḥ"      # Same (visarga)
    "'": "'"      # Avagraha
  
  special:
    "kṣ": "kṣ"
    "jñ": "jñ"
```

### Indic Script Example (Bengali)

```yaml
metadata:
  name: "bengali"
  script_type: "brahmic"
  has_implicit_a: true
  description: "Bengali/Bangla script"
  aliases: ["bangla"]

mappings:
  vowels:
    "অ": "अ"      # A
    "আ": "आ"      # AA
    "ই": "इ"      # I
    "ঈ": "ई"      # II
    "উ": "उ"      # U
    "ঊ": "ऊ"      # UU
    "ঋ": "ऋ"      # VOCALIC R
    "এ": "ए"      # E
    "ঐ": "ऐ"      # AI
    "ও": "ओ"      # O
    "ঔ": "औ"      # AU
  
  consonants:
    "ক": "क"      # KA
    "খ": "ख"      # KHA
    "গ": "ग"      # GA
    "ঘ": "घ"      # GHA
    "ঙ": "ङ"      # NGA
    # ... etc
  
  dependent_vowels:
    "া": "ा"      # AA matra
    "ি": "ि"      # I matra
    "ী": "ी"      # II matra
    # ... etc
  
  marks:
    "ং": "ं"      # Anusvara
    "ঃ": "ः"      # Visarga
```

## Validation Rules

1. **Character Uniqueness**: No duplicate source characters within a script
2. **Unicode Validity**: All characters must be valid Unicode
3. **Target Consistency**: Target characters should be appropriate for the target script
4. **Completeness**: Core character sets should be complete for the script

## Best Practices

1. **Comprehensive Coverage**: Include all common characters for the script
2. **Consistent Naming**: Use standard script names and descriptions
3. **Clear Comments**: Add comments for non-obvious mappings
4. **Test Coverage**: Ensure comprehensive test cases exist
5. **Documentation**: Update this reference for new schema features

## Testing Your Schema

### Build-time Testing

After creating a schema for build-time use:

1. **Build the library**: `cargo build`
2. **Run tests**: `cargo test`
3. **Test conversion**: Use examples to verify correct conversion
4. **Add test cases**: Create comprehensive test cases in the test suite

### Runtime Testing

To test runtime schema loading:

```rust
use shlesha::modules::registry::{SchemaRegistry, SchemaRegistryTrait};

let mut registry = SchemaRegistry::new();

// Load your schema
registry.load_schema("path/to/your/schema.yaml")?;

// Verify it's loaded
if let Some(schema) = registry.get_schema("your_script") {
    println!("Loaded {} mappings", schema.mappings.len());
}
```

See `examples/test_runtime_schema_loading.rs` for a complete example.

## Error Handling

Common schema errors:

- **Invalid YAML**: Syntax errors in the YAML file
- **Missing required fields**: Metadata fields are missing
- **Invalid target**: Unknown target script specified
- **Character conflicts**: Duplicate source characters
- **Unicode issues**: Invalid Unicode sequences

The build system will report these errors during compilation.