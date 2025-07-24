# Adding New Schemas to Shlesha

This guide explains how to add new transliteration schemas to Shlesha. Schemas can be added either at compile-time (built into the library) or at runtime (dynamically loaded).

## Quick Start

### Compile-time Schema (Recommended)
1. Create a YAML file in the `/schemas/` directory
2. Define metadata and mappings using the appropriate tokens
3. Run `cargo build` to generate code
4. Test your new schema

### Runtime Schema
1. Create a YAML file anywhere
2. Load it using `transliterator.load_schema("path/to/schema.yaml")`
3. Use immediately without recompilation

## Schema File Format

### Basic Structure

```yaml
metadata:
  name: "your_script_name"           # Required: unique identifier
  script_type: "roman"               # Required: "roman" or "brahmic"
  has_implicit_a: false              # Required: true for abugida scripts
  description: "Script description"  # Optional but recommended
  aliases: ["alt_name"]              # Optional: alternative names
  version: "1.0.0"                   # Optional: schema version
  author: "Your Name"                # Optional: schema author

target: "alphabet_tokens"            # Required: token system to use
                                    # "alphabet_tokens" for roman scripts
                                    # "abugida_tokens" for brahmic scripts

mappings:                            # Required: character-to-token mappings
  vowels:
    VowelA: "a"
    VowelAa: "ā"
    # ... more vowel mappings
  
  consonants:
    ConsonantK: "k"
    ConsonantKh: "kh"
    # ... more consonant mappings
  
  marks:
    MarkAnusvara: "ṃ"
    MarkVisarga: "ḥ"
    # ... more mark mappings

codegen:                             # Optional: code generation options
  processor_type: "standard"         # "standard" or "extended"
```

## Compile-time vs Runtime Schemas

### Compile-time Schemas
**Advantages:**
- Optimal performance (no runtime overhead)
- Compile-time validation
- Integrated into the library
- Available to all users of the library

**Use when:**
- Adding widely-used transliteration schemes
- Performance is critical
- Schema is stable and well-tested
- Contributing to the main library

### Runtime Schemas
**Advantages:**
- No recompilation needed
- User-specific customizations
- Quick prototyping and testing
- Private/proprietary schemas

**Use when:**
- Experimenting with new mappings
- User-specific requirements
- Temporary or specialized schemas
- Cannot modify the library source

## Token Reference

### Vowel Tokens
- `VowelA`, `VowelAa` - Short and long 'a'
- `VowelI`, `VowelIi` - Short and long 'i'
- `VowelU`, `VowelUu` - Short and long 'u'
- `VowelR`, `VowelRr` - Vocalic r
- `VowelL`, `VowelLl` - Vocalic l
- `VowelE`, `VowelEe` - Short and long 'e'
- `VowelO`, `VowelOo` - Short and long 'o'
- `VowelAi`, `VowelAu` - Diphthongs 'ai' and 'au'

**Important Note on Short e/o:** Only map `VowelE` and `VowelO` if the script itself has distinct characters for short e and o sounds. For example:
- Telugu has distinct characters for short e (ఎ) and long e (ఏ), so both should be mapped
- Devanagari has short e (ऎ) and o (ऒ) as well as long e (ए) and o (ओ)
- Scripts like Grantha/Sharada that only have long e/o sounds should only map `VowelEe` and `VowelOo`
- Do not map tokens that don't exist in the script - they can be handled via runtime schemas if needed

**Important Note on Vedic Accent Marks:** Built-in schemas use visual token names to avoid Unicode naming confusion:
- Use `MarkVerticalLineAbove` for ॑ (U+0951) 
- Use `MarkLineBelow` for ॒ (U+0952)
- Use `MarkDoubleVerticalAbove` for ᳚ (U+1CDA)
- Use `MarkTripleVerticalAbove` for ᳛ (U+1CDB)

Avoid using linguistic names like `MarkUdatta` or `MarkSvarita` in built-in schemas, as the same visual mark can represent different linguistic functions across Vedic traditions. See [VEDIC_ACCENTS.md](VEDIC_ACCENTS.md) for details.

### Consonant Tokens
- **Velars**: `ConsonantK`, `ConsonantKh`, `ConsonantG`, `ConsonantGh`, `ConsonantNg`
- **Palatals**: `ConsonantC`, `ConsonantCh`, `ConsonantJ`, `ConsonantJh`, `ConsonantNy`
- **Retroflexes**: `ConsonantT`, `ConsonantTh`, `ConsonantD`, `ConsonantDh`, `ConsonantN`
- **Dentals**: `ConsonantDentalT`, `ConsonantDentalTh`, `ConsonantDentalD`, `ConsonantDentalDh`, `ConsonantDentalN`
- **Labials**: `ConsonantP`, `ConsonantPh`, `ConsonantB`, `ConsonantBh`, `ConsonantM`
- **Semivowels**: `ConsonantY`, `ConsonantR`, `ConsonantL`, `ConsonantV`
- **Sibilants**: `ConsonantPalatalS`, `ConsonantRetroflexS`, `ConsonantDentalS`
- **Aspirate**: `ConsonantH`

### Mark Tokens
- `MarkAnusvara` - Nasal mark (ṃ)
- `MarkVisarga` - Aspiration mark (ḥ)
- `MarkCandrabindu` - Nasalization (m̐)
- `MarkVirama` - Vowel cancellation

### Special Tokens
- `SpecialKs` - kṣa conjunct
- `SpecialJny` - jña conjunct
- `SpecialAvagraha` - Elision mark (')
- `SpecialNukta` - Dot below
- `SpecialJihvamuliya` - Velar fricative
- `SpecialUpadhmaniya` - Bilabial fricative

### Digit Tokens
- `Digit0` through `Digit9`

### Vedic Accent Tokens (Visual Names)
- `MarkVerticalLineAbove` - Vertical line above (॑)
- `MarkLineBelow` - Line below (॒)
- `MarkDoubleVerticalAbove` - Double vertical above (᳚)
- `MarkTripleVerticalAbove` - Triple vertical above (᳛)

### For Abugida Scripts Only
- **Vowel Signs**: `VowelSignAa`, `VowelSignI`, `VowelSignIi`, etc.
- **Consonant Marks**: `ConsonantAnusvara`, `ConsonantVisarga`

## Step-by-Step Guide

### 1. Choose Your Script Type

**Roman-based scripts** (IAST, Harvard-Kyoto, SLP1):
- Set `script_type: "roman"`
- Set `has_implicit_a: false`
- Use `target: "alphabet_tokens"`

**Brahmic scripts** (Devanagari, Tamil, Bengali):
- Set `script_type: "brahmic"`
- Set `has_implicit_a: true`
- Use `target: "abugida_tokens"`

### 2. Create Character Mappings

Map each character or character sequence to the appropriate token:

```yaml
mappings:
  consonants:
    ConsonantK: "k"      # Single character
    ConsonantKh: "kh"    # Multi-character sequence
    ConsonantK: ["k", "q"]  # Multiple representations (alternatives)
```

### 3. Handle Special Cases

#### Multi-character Mappings
```yaml
ConsonantKh: "kh"    # "kh" is treated as one unit
ConsonantC: "ch"
```

#### Alternative Representations
```yaml
MarkUdatta: ["́", "̍"]    # Both represent the same token
VowelAa: ["ā", "aa"]     # Long vowel alternatives
```

#### Case Sensitivity
Most Roman schemas are case-sensitive:
```yaml
ConsonantT: "T"          # Retroflex
ConsonantDentalT: "t"    # Dental
```

### 4. Test Your Schema

#### For Compile-time Schemas

Create a test file to verify your schema works correctly:

```rust
#[test]
fn test_my_schema() {
    let transliterator = Shlesha::new();
    
    // Test basic conversion
    assert_eq!(
        transliterator.transliterate("test", "my_schema", "devanagari").unwrap(),
        "expected_output"
    );
    
    // Test round-trip
    let devanagari = transliterator.transliterate("test", "my_schema", "devanagari").unwrap();
    let back = transliterator.transliterate(&devanagari, "devanagari", "my_schema").unwrap();
    assert_eq!(back, "test");
}
```

#### For Runtime Schemas

```rust
use shlesha::Shlesha;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut transliterator = Shlesha::new();
    
    // Load the schema
    transliterator.load_schema("path/to/my_schema.yaml")?;
    
    // Use immediately
    let result = transliterator.transliterate(
        "test text", 
        "my_schema", 
        "devanagari"
    )?;
    println!("{}", result);
    
    Ok(())
}
```

### 5. Build and Verify

#### For Compile-time Schemas
```bash
# Build to generate code
cargo build

# Run tests
cargo test

# Try your schema
echo "test text" | cargo run -- transliterate --from my_schema --to devanagari
```

#### For Runtime Schemas
```bash
# No build needed! Just run with the schema file
cargo run -- --load-schema my_schema.yaml transliterate --from my_schema --to devanagari "test"
```

## Runtime Schema Usage

### Loading Schemas

#### From File
```rust
transliterator.load_schema("path/to/schema.yaml")?;
```

#### From String
```rust
let schema_content = r#"
metadata:
  name: "custom"
  script_type: "roman"
  has_implicit_a: false
target: "alphabet_tokens"
mappings:
  vowels:
    VowelA: "a"
"#;
transliterator.load_schema_from_string(schema_content)?;
```

### CLI Usage
```bash
# Load and use in one command
shlesha --load-schema custom.yaml transliterate --from custom --to devanagari "text"

# Load multiple schemas
shlesha --load-schema schema1.yaml --load-schema schema2.yaml list-scripts
```

### Python Bindings
```python
import shlesha

t = shlesha.Shlesha()
t.load_schema("custom_schema.yaml")
result = t.transliterate("text", "custom_schema", "devanagari")
```

### Schema Registry

Runtime schemas are stored in memory and available until the program exits:

```rust
// Check if schema is loaded
if transliterator.supports_script("my_schema") {
    // Use the schema
}

// List all scripts (includes runtime schemas)
let scripts = transliterator.list_supported_scripts();
```

## Common Patterns

### Aspirated Consonants
Group aspirated consonants logically:
```yaml
ConsonantK: "k"
ConsonantKh: "kh"
ConsonantG: "g"
ConsonantGh: "gh"
```

### Vowel Length
Distinguish short and long vowels:
```yaml
VowelA: "a"
VowelAa: ["ā", "A", "aa"]  # Multiple representations
```

### Retroflex vs Dental
Use specific tokens for different articulation points:
```yaml
ConsonantT: "T"              # Retroflex
ConsonantDentalT: "t"        # Dental
```

## Example: Adding Baraha Schema

```yaml
metadata:
  name: "baraha"
  script_type: "roman"
  has_implicit_a: false
  description: "Baraha transliteration scheme"
  aliases: ["baraha_kannada"]

target: "alphabet_tokens"

mappings:
  vowels:
    VowelA: "a"
    VowelAa: ["A", "aa"]
    VowelI: "i"
    VowelIi: ["I", "ee", "ii"]
    VowelU: "u"
    VowelUu: ["U", "oo", "uu"]
    VowelR: ["Ru", "R^i"]
    VowelRr: ["RU", "R^I"]
    VowelE: ["e", "E"]
    VowelAi: "ai"
    VowelO: ["o", "O"]
    VowelAu: ["au", "ou"]
  
  consonants:
    ConsonantK: "k"
    ConsonantKh: ["kh", "K"]
    ConsonantG: "g"
    ConsonantGh: ["gh", "G"]
    # ... more mappings
  
  marks:
    MarkAnusvara: ["M", "m"]
    MarkVisarga: "H"
    MarkVirama: "&"

codegen:
  processor_type: "standard"
```

## Runtime Schema Limitations

### Performance
- Runtime schemas are slower than compile-time schemas
- Each conversion rebuilds the token mappings
- No Aho-Corasick optimization for pattern matching

### Validation
- Basic validation only (required fields, valid script types)
- No compile-time type checking
- Errors discovered at runtime

### Features
- No direct converter generation
- Always routes through hub
- Cannot override built-in schemas

## Debugging Tips

### 1. Check Token Names
Ensure you're using the correct token names. Run:
```bash
grep "pub enum" src/modules/hub/tokens.rs
```

### 2. Verify Build Output (Compile-time only)
After building, check generated code:
```bash
# Check if your schema was processed
grep "your_schema_name" target/debug/build/*/out/schema_generated.rs
```

### 3. Test Incrementally
Start with a minimal schema and add mappings gradually:
```yaml
# Start simple
mappings:
  vowels:
    VowelA: "a"
  consonants:
    ConsonantK: "k"
```

### 4. Use Existing Schemas as Reference
Look at similar existing schemas in `/schemas/` for patterns and conventions.

### 5. Debug Runtime Loading
```rust
// Enable debug logging
match transliterator.load_schema("schema.yaml") {
    Ok(_) => println!("Schema loaded successfully"),
    Err(e) => eprintln!("Failed to load schema: {}", e),
}
```

## Advanced Features

### Custom Processor Types
Use `codegen.processor_type` for special handling:
- `"standard"` - Default processing
- `"extended"` - For schemas with special requirements

### Handling Conjuncts
For complex conjuncts, use Special tokens:
```yaml
special:
  SpecialKs: ["kSh", "kS", "x"]
  SpecialJny: ["jny", "gny", "gy"]
```

### Contextual Variations
For context-dependent mappings, consider the longest match first:
```yaml
ConsonantN: ["ng", "n"]  # "ng" will be tried before "n"
```

## Troubleshooting

### Schema Not Recognized
- Ensure file is in `/schemas/` directory (compile-time)
- Check YAML syntax is valid
- Verify `metadata.name` is unique
- For runtime: ensure `load_schema()` was called successfully

### Mappings Not Working
- Check token names are correct
- Ensure proper script_type and target alignment
- Look for conflicting mappings
- Test with simple inputs first

### Build Errors (Compile-time)
- Run `cargo clean` and rebuild
- Check for YAML syntax errors
- Ensure all required fields are present

### Runtime Loading Errors
- Check file path is correct
- Verify YAML syntax
- Ensure schema name doesn't conflict with built-in schemas
- Check for missing required fields

## Best Practices

### For Compile-time Schemas
1. Thoroughly test before adding to `/schemas/`
2. Include comprehensive test cases
3. Document any special behavior
4. Consider performance implications

### For Runtime Schemas
1. Use for experimentation and prototyping
2. Validate thoroughly before production use
3. Consider converting to compile-time for performance
4. Keep schemas versioned and documented

## Contributing Your Schema

### For Compile-time Schemas
1. Ensure your schema is well-tested
2. Add documentation in the schema description
3. Include test cases
4. Submit a pull request with:
   - The schema YAML file
   - Test file demonstrating usage
   - Any necessary documentation updates

### For Runtime Schemas
1. Share schema files with documentation
2. Include example usage code
3. Note any special requirements
4. Consider proposing for compile-time inclusion if widely useful