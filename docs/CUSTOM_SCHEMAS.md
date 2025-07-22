# Custom Schema Support in Shlesha

This document describes the runtime custom schema functionality that allows users to add new transliteration scripts without recompiling the library.

## Overview

Shlesha supports loading custom encoding schemes at runtime through YAML schema files. This enables users to:

- Add proprietary or specialized encoding schemes
- Create domain-specific transliteration systems
- Prototype new scripts quickly
- Handle legacy encoding formats

## Architecture

### Components

1. **SchemaRegistry**: Manages loaded schemas and provides access interface
2. **SchemaBasedConverter**: Generic converter that uses schema mappings
3. **Script Integration**: Seamless integration with existing converter system

### Flow

```
Custom Schema File → SchemaRegistry → SchemaBasedConverter → Hub → Target Script
```

The schema-based converter acts as a fallback when hardcoded converters don't support a script.

## Schema Format

### Basic Structure

```yaml
metadata:
  name: "my_custom_encoding"
  script_type: "roman"  # or "brahmic"
  has_implicit_a: false  # true for Brahmic scripts
  description: "Description of the encoding"
  author: "Author Name"  # optional
  version: "1.0.0"       # optional

target: "alphabet_tokens"  # or "abugida_tokens" for Brahmic scripts

mappings:
  consonants:
    ConsonantK: "k"      # Token name -> character(s)
    ConsonantKh: "kh"
    ConsonantG: "g"
    # ... more mappings
  
  vowels:
    VowelA: "a"
    VowelAa: "ā"
    VowelI: "i"
    # ... more mappings
  
  marks:
    MarkAnusvara: "ṃ"
    MarkVisarga: "ḥ"
    # ... more mappings
  
  digits:
    Digit0: "0"
    Digit1: "1"
    # ... more mappings

codegen:
  processor_type: "roman_token_based"  # or "brahmic_token_based"
```

### Script Types

- **`roman`**: Scripts without implicit vowels (uses alphabet tokens)
- **`brahmic`**: Scripts with implicit vowels (uses abugida tokens)

### Target Field

The `target` field specifies which internal token system your schema maps to. This determines how the transliteration engine interprets and processes your mappings:

- **`alphabet_tokens`**: For alphabetic scripts (Roman, Greek, Cyrillic, etc.)
  - Each character represents a distinct sound
  - No inherent vowels in consonants
  - Examples: IAST, ISO-15919, Harvard-Kyoto, SLP1
  
- **`abugida_tokens`**: For abugida scripts (Brahmic family)
  - Consonants have inherent vowels (usually 'a')
  - Vowel signs modify the inherent vowel
  - Requires virama/halant to remove inherent vowel
  - Examples: Devanagari, Tamil, Telugu, Bengali

The choice of target affects how your mappings are processed during transliteration.

### Mapping Categories

The `mappings` section uses predefined categories with specific token names:

#### Token Naming Convention
- **Consonants**: `ConsonantK`, `ConsonantKh`, `ConsonantG`, etc.
- **Vowels**: `VowelA`, `VowelAa`, `VowelI`, `VowelIi`, etc.
- **Marks**: `MarkAnusvara`, `MarkVisarga`, `MarkCandrabindu`, etc.
- **Digits**: `Digit0`, `Digit1`, `Digit2`, etc.
- **Vedic**: `MarkUdatta`, `MarkAnudatta`, etc.
- **Vowel Signs** (for abugida scripts): `VowelSignAa`, `VowelSignI`, etc.

Mappings can be:
- Single character: `ConsonantK: "k"`
- Multi-character: `ConsonantKh: "kh"`
- Array of alternatives: `MarkUdatta: ["́", "̍"]`

## Usage

### Loading a Schema

```rust
use shlesha::Shlesha;

let mut transliterator = Shlesha::new();
transliterator.load_schema("path/to/custom_encoding.yaml")?;
```

### Using Custom Scripts

```rust
// After loading the schema
let result = transliterator.transliterate(
    "input_text", 
    "my_custom_encoding", 
    "devanagari"
)?;
```

### Checking Support

```rust
// Check if a script is supported (includes custom schemas)
let is_supported = transliterator.supports_script("my_custom_encoding");

// List all supported scripts (includes custom schemas)
let scripts = transliterator.list_supported_scripts();
```

## Examples

### Simple Roman Script Example

```yaml
metadata:
  name: "simple_ascii"
  script_type: "roman"
  has_implicit_a: false
  description: "Simple ASCII transliteration"

target: "alphabet_tokens"

mappings:
  vowels:
    VowelA: "a"
    VowelAa: "aa"
    VowelI: "i"
    VowelIi: "ii"
  
  consonants:
    ConsonantK: "k"
    ConsonantKh: "kh"
    ConsonantG: "g"
    ConsonantGh: "gh"
    ConsonantR: "r"
    ConsonantM: "m"
  
  marks:
    MarkAnusvara: "m"
    MarkVisarga: "h"

codegen:
  processor_type: "roman_token_based"
```

Usage:
```rust
transliterator.load_schema("simple_ascii.yaml")?;
let result = transliterator.transliterate("karma", "simple_ascii", "devanagari")?;
// Result: "कर्म" (proper token-based mapping)
```

### Legacy Encoding Support

```yaml
metadata:
  name: "legacy_encoding"
  script_type: "roman"
  has_implicit_a: false
  description: "Legacy system encoding"
  author: "Migration Team"

target: "alphabet_tokens"

mappings:
  consonants:
    ConsonantK: "K"
    ConsonantG: "G"
    ConsonantC: "CH"  # Multi-character mapping
    ConsonantJh: "JH"
    
  vowels:
    VowelA: "a"
    VowelAa: "A"     # Uppercase for long vowel
    VowelI: "i"
    VowelIi: "I"
  
  marks:
    MarkVirama: "&"  # Custom virama representation

codegen:
  processor_type: "roman_token_based"
```

## Conversion Behavior

### Token-Based Mapping

Custom schemas use token-based mapping:
- Input text is parsed into linguistic tokens (vowels, consonants, marks)
- Each token is mapped according to the schema definitions
- Multi-character mappings are supported (e.g., "kh" for aspirated consonants)
- The system understands linguistic units rather than just characters
- Proper handling of vowel signs, virama, and other diacritics

### Hub Routing

Custom scripts route through the token-based hub system:

1. **Roman scripts** (`script_type: "roman"`):
   ```
   Custom → Alphabet Tokens → Hub Processing → Target
   ```

2. **Brahmic scripts** (`script_type: "brahmic"`):
   ```
   Custom → Abugida Tokens → Hub Processing → Target
   ```

### Bidirectional Support

Reverse conversion is automatically supported:
```rust
// Forward: custom → devanagari
let deva = transliterator.transliterate("kam", "my_encoding", "devanagari")?;

// Reverse: devanagari → custom
let custom = transliterator.transliterate(&deva, "devanagari", "my_encoding")?;
```

## Performance Characteristics

### Runtime Performance

- **Character lookup**: O(1) HashMap access per character
- **Schema access**: Schema cloned per conversion (MVP implementation)
- **Hub routing**: Same performance as built-in scripts
- **Memory**: Each loaded schema stored in memory

### Optimization Opportunities

Current MVP implementation prioritizes simplicity over performance:

1. **Schema caching**: Schemas are cloned on each conversion
2. **No precompilation**: Character mappings rebuilt each time
3. **No optimization**: No special handling for common patterns

Future optimizations could include:
- Arc<Schema> sharing to avoid clones
- Precompiled mapping tables
- LRU cache for conversion results
- SIMD optimizations for character processing

## Error Handling

### Schema Loading Errors

- **File not found**: Clear error message with file path
- **YAML parsing**: Detailed parsing error information
- **Schema validation**: Validation failures with specific field errors

### Runtime Errors

- **Unknown scripts**: Standard "No converter found" error
- **Mapping failures**: Unknown characters pass through silently
- **Hub errors**: Propagated from underlying hub processing

## Integration with Existing Features

### Metadata Collection

Custom schemas support metadata collection:
```rust
let result = transliterator.transliterate_with_metadata(
    "text", 
    "my_encoding", 
    "devanagari"
)?;
// result.metadata contains unknown character information
```

### CLI Support

Custom schemas work with the CLI:
```bash
# Load schema and convert
shlesha --load-schema custom.yaml transliterate --from my_encoding --to devanagari "text"
```

### Python/WASM Bindings

Custom schemas are supported in all language bindings:
```python
import shlesha
transliterator = shlesha.Shlesha()
transliterator.load_schema("custom.yaml")
result = transliterator.transliterate("text", "my_encoding", "devanagari")
```

## Limitations and Considerations

### Current Limitations

1. **Token set limitations**: Must use predefined token names
2. **Performance overhead**: Schema cloning and rebuilding per conversion
3. **Basic validation**: Only validates required fields and script types
4. **Memory usage**: All schemas kept in memory permanently

### Best Practices

1. **Keep mappings simple**: Complex linguistic rules not supported
2. **Use appropriate script_type**: Affects hub routing behavior
3. **Test thoroughly**: Verify all expected character mappings work
4. **Document schemas**: Include clear descriptions and examples

## Built-in Schemas for Vedic Texts

### Recently Added Scripts

The following scripts have been added specifically for Vedic text support:

1. **Sharada** (`sharada`, `shrd`)
   - Historical script of Kashmir
   - Crucial for Vedic manuscript preservation
   - Full Vedic accent support
   - Proper yogavaha and accent ordering

2. **Tibetan** (`tibetan`, `tibt`, `bo`)
   - Important for Buddhist Vedic transmission
   - Complete Sanskrit transliteration support
   - Aspirated consonants for accurate Sanskrit
   - Vedic accent marks using standard combiners

3. **Thai** (`thai`, `th`)
   - Adapted from Grantha for Buddhist texts
   - Sanskrit consonant and vowel mappings
   - Tone marks for Vedic accent approximation
   - Pre-consonantal vowel handling

### Debug Schemas

Two special schemas are provided for debugging the token-based architecture:

1. **abugida_tokens.yaml** - Shows all Abugida (Brahmic) tokens
2. **alphabet_tokens.yaml** - Shows all Alphabet (Roman) tokens

These output token names instead of characters (e.g., `[ConsonantK][VowelA]`), useful for:
- Understanding the conversion pipeline
- Debugging character ordering issues
- Learning the token architecture

## Future Enhancements

### Planned Features

1. **Performance optimization**: Reduce schema access overhead
2. **Advanced validation**: More comprehensive schema validation
3. **Linguistic rules**: Support for context-aware conversions
4. **Schema versioning**: Handle multiple versions of same schema
5. **Schema discovery**: Auto-discovery of schema files in directories

### Advanced Use Cases

1. **Historical scripts**: Support for ancient or historical writing systems
2. **Phonetic systems**: Specialized phonetic notation schemes
3. **Domain-specific**: Scientific, mathematical, or technical notations
4. **Regional variants**: Local variations of standard scripts

## Migration and Compatibility

### Upgrading from MVP

The current MVP implementation provides a stable foundation. Future optimizations will maintain API compatibility while improving performance.

### Schema Evolution

Schema format is designed to be extensible:
- New fields can be added without breaking existing schemas
- Version field allows for format evolution
- Validation rules can be enhanced over time

---

This custom schema system enables Shlesha to handle any text encoding that can be expressed as character mappings, making it truly extensible for diverse transliteration needs.