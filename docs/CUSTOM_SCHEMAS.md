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
name: "my_custom_encoding"
version: "1.0.0"
script_type: "romanized"  # or "indic"/"brahmic"
description: "Description of the encoding"
author: "Author Name"

metadata:
  has_implicit_a: false    # true for Indic scripts
  direction: "ltr"         # text direction
  case_sensitive: true     # case sensitivity

mappings:
  consonants:
    "k": "क"
    "g": "ग"
    # ... more mappings
  
  vowels:
    "a": "अ"
    "i": "इ"
    # ... more mappings

validation:
  patterns: {}  # Optional validation rules
```

### Script Types

- **`romanized`**: Scripts without implicit vowels (routes through ISO-15919 hub)
- **`indic`/`brahmic`**: Scripts with implicit vowels (routes through Devanagari hub)

### Mapping Categories

The `mappings` section can contain any categorization:
- `consonants`: Consonant characters
- `vowels`: Vowel characters  
- `digits`: Numeric characters
- `punctuation`: Punctuation marks
- `special`: Special characters

Categories are flattened internally, so organization is for clarity only.

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

### Simple ASCII-to-Devanagari Mapping

```yaml
name: "simple_ascii"
version: "1.0.0"
script_type: "romanized"
description: "Simple ASCII to Devanagari mapping"
author: "User"

metadata:
  has_implicit_a: false
  direction: "ltr"
  case_sensitive: false

mappings:
  basic:
    "a": "अ"
    "k": "क"
    "r": "र"
    "m": "म"
```

Usage:
```rust
transliterator.load_schema("simple_ascii.yaml")?;
let result = transliterator.transliterate("karma", "simple_ascii", "devanagari")?;
// Result: "कअर्मअ" (character-by-character mapping)
```

### Legacy Encoding Support

```yaml
name: "legacy_encoding"
version: "1.0.0"
script_type: "romanized"
description: "Legacy system encoding"
author: "Migration Team"

metadata:
  has_implicit_a: false
  
mappings:
  consonants:
    "K": "क"
    "G": "ग"
    "CH": "च"  # Multi-character input
    "JH": "झ"
    
  vowels:
    "A": "आ"
    "I": "ई"
```

## Conversion Behavior

### Character-Level Mapping

Custom schemas perform character-level mapping:
- Each character/string in the input is looked up in the mappings
- Multi-character mappings are supported (e.g., "ch" → "च")
- Unknown characters pass through unchanged
- Longer mappings are tried first

### Hub Routing

Custom scripts route through the hub system:

1. **Romanized scripts** (`script_type: "romanized"`):
   ```
   Custom → ISO-15919 → Hub Processing → Target
   ```

2. **Indic scripts** (`script_type: "indic"`):
   ```
   Custom → Devanagari → Hub Processing → Target
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

1. **No linguistic rules**: Only character-level mapping, no context awareness
2. **Performance overhead**: Schema cloning and rebuilding per conversion
3. **No validation**: Limited schema validation during loading
4. **Memory usage**: All schemas kept in memory permanently

### Best Practices

1. **Keep mappings simple**: Complex linguistic rules not supported
2. **Use appropriate script_type**: Affects hub routing behavior
3. **Test thoroughly**: Verify all expected character mappings work
4. **Document schemas**: Include clear descriptions and examples

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