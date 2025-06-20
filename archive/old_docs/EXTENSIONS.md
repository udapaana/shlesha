# Shlesha Extensions Guide

Extensions allow you to modify transliteration behavior at runtime without changing the core library code. This is crucial for handling manuscript variations, special notation systems, and domain-specific requirements.

## How Extensions Work

Extensions operate through a priority-based overlay system:

1. **Base Schema**: Defines core mappings (e.g., क → ka)
2. **Extensions**: Override or add mappings (e.g., क़ → qa)
3. **Priority Resolution**: Higher priority extensions win conflicts

## Extension Structure

```yaml
name: "extension_name"
description: "What this extension does"
applies_to: ["Devanagari", "IAST"]  # Which schemas it modifies

extensions:
  feature_name:
    description: "Specific feature within extension"
    priority: 20  # Higher numbers override lower
    mappings:
      "source_character":
        to: "target_character"
        element_type: "type_name"
        properties:
          key1: value1
          key2: value2

conditions:
  feature_name:
    requires_any: ["char1", "char2"]  # Activate if text contains these
```

## Available Extensions

### 1. Musical Notation (`musical_notation.yaml`)

Handles various musical notation systems:

- **Sama Vedic Tones**: 7-tone system (१-७ → ₁-₇)
- **Western Staff**: ♩, ♪, ♫, ♭, ♮, ♯
- **Rhythmic Patterns**: Tala markers (|, ||, ○, ●)
- **Byzantine Neumes**: Ancient notation symbols

Example:
```bash
cargo run --example cli -- -f devanagari -t iast -e musical_notation "संगीत१२३"
```

### 2. Vedic Accents (to be created)

```yaml
name: "vedic_accents"
description: "Vedic accent marks and tonal indicators"
applies_to: ["Devanagari", "IAST", "Harvard-Kyoto"]

extensions:
  basic_accents:
    description: "Three-tone accent system"
    priority: 30
    mappings:
      "॑":  # Udatta (high tone)
        to: "́"
        element_type: "accent_udatta"
        properties:
          tone: "high"
          combines_with_previous: true
      "॒":  # Anudatta (low tone)
        to: "̀"
        element_type: "accent_anudatta"
        properties:
          tone: "low"
          combines_with_previous: true
      "᳚":  # Svarita (falling tone)
        to: "̌"
        element_type: "accent_svarita"
        properties:
          tone: "falling"
```

### 3. Manuscript Variants (to be created)

```yaml
name: "manuscript_variants"
description: "Common scribal variations in manuscripts"
applies_to: ["Devanagari", "IAST"]

extensions:
  archaic_forms:
    description: "Older forms of characters"
    priority: 15
    mappings:
      "ळ":  # Vedic retroflex l
        to: "ḻ"
        element_type: "consonant_variant"
        properties:
          variant_of: "la"
          period: "vedic"
```

## Creating Custom Extensions

### Step 1: Create YAML File

Create `schemas/extensions/my_extension.yaml`:

```yaml
name: "my_custom_extension"
description: "Handles special requirements for my project"
applies_to: ["Devanagari", "IAST"]

extensions:
  special_marks:
    description: "Project-specific notation"
    priority: 25
    mappings:
      "◌":
        to: "[placeholder]"
        element_type: "custom_mark"
        properties:
          purpose: "indicates missing text"
```

### Step 2: Use in Code

```rust
use shlesha::TransliteratorBuilder;

let mut transliterator = TransliteratorBuilder::new()
    .with_schema_directory("schemas")?
    .build();

// Add extension
transliterator.add_extension("my_custom_extension")?;

// Or use directly
let result = transliterator.transliterate_with_extensions(
    "text◌here",
    "Devanagari",
    "IAST",
    &["my_custom_extension"]
)?;
```

### Step 3: Use in CLI

```bash
cargo run --example cli -- -f devanagari -t iast -e my_custom_extension "input"
```

## Runtime Variant System

For non-technical users who need to add variants without writing YAML:

```rust
use shlesha::runtime_extension::RuntimeExtensionManager;

let mut manager = RuntimeExtensionManager::new();

// Simple variant addition
manager.add_simple_variant(
    "qa_variant",           // name
    "Arabic qa sound",      // description
    "ka",                   // similar to
    "क़",                    // Devanagari form
    "qa",                   // IAST form
    "Manuscript ABC.123"    // source
)?;

// Activate the variant
manager.activate_variant("qa_variant")?;
```

## Extension Properties

### Priority System

- **0-10**: Low priority (defaults, fallbacks)
- **11-20**: Normal priority (standard extensions)
- **21-30**: High priority (overrides)
- **31+**: Critical (always applies)

### Conditional Activation

Extensions can activate automatically based on text content:

```yaml
conditions:
  feature_name:
    requires_any: ["char1", "char2"]     # OR condition
    requires_all: ["char3", "char4"]     # AND condition
    excludes: ["char5"]                  # NOT condition
```

### Property Types

```yaml
properties:
  # Boolean
  is_variant: true
  
  # Numeric
  tone_height: 2.5
  frequency: 440
  
  # String
  manuscript_source: "MS Delhi 1234"
  
  # Lists
  alternate_forms: ["form1", "form2"]
  
  # Nested
  phonetic_features:
    voiced: true
    aspirated: false
```

## Best Practices

1. **Namespace Your Extensions**: Use prefixes to avoid conflicts
   ```yaml
   name: "myproject_special_marks"  # Not just "special_marks"
   ```

2. **Document Sources**: Always include manuscript/text references
   ```yaml
   properties:
     source: "Wilson Sanskrit-English Dictionary, 1819"
     page: 127
   ```

3. **Version Extensions**: Track changes over time
   ```yaml
   metadata:
     version: "1.2.0"
     updated: "2024-01-15"
     author: "Your Name"
   ```

4. **Test Round-trips**: Ensure bidirectional conversion works
   ```bash
   echo "क़" | cargo run --example cli -- -f devanagari -t iast -e my_ext | \
     cargo run --example cli -- -f iast -t devanagari -e my_ext
   ```

5. **Use Appropriate Priorities**: Don't use high priority unless necessary

## Advanced Features

### Contextual Rules

Future support for context-sensitive mappings:

```yaml
contextual_rules:
  - pattern: "क्ष"
    context_before: "संस्"
    context_after: "त"
    replacement: "kṣ"
    properties:
      note: "Special sandhi case"
```

### Multi-character Mappings

```yaml
mappings:
  "क्ष":  # Conjunct
    to: "kṣa"
    element_type: "conjunct"
    components: ["ka", "ṣa"]
```

### Metadata Preservation

```yaml
preserve_metadata:
  - "manuscript_line_number"
  - "folio_side"
  - "scribe_mark"
```

## Debugging Extensions

Enable verbose mode to see extension application:

```bash
cargo run --example cli -- -v -f devanagari -t iast -e my_extension "test"
```

Check which extensions are loaded:

```rust
let active = transliterator.list_active_extensions();
println!("Active extensions: {:?}", active);
```

## Contributing Extensions

To contribute an extension to the Shlesha project:

1. Create extension YAML in `schemas/extensions/`
2. Add tests in `tests/extension_tests/`
3. Document in this file
4. Submit PR with example usage

Extensions should be:
- **General purpose** (not project-specific)
- **Well documented** with sources
- **Tested** with example texts
- **Performant** (avoid complex rules)