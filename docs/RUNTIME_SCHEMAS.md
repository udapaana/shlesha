# Runtime Schema Loading

## Current Status

The Shlesha CLI currently uses only **built-in schemas** that are compiled into the binary. Runtime schema loading is supported at the library level but not yet exposed through the CLI.

## Current CLI Usage

```bash
# Basic transliteration with built-in schemas
shlesha transliterate --from devanagari --to iast "संस्कृत"

# List supported built-in scripts
shlesha scripts

# Read from stdin
echo "sanskrit" | shlesha transliterate --from iast --to devanagari

# With metadata
shlesha transliterate --from iast --to devanagari --verbose "unknown"
```

## Library-Level Runtime Loading

The `SchemaRegistry` module supports runtime schema loading:

```rust
use shlesha::modules::registry::{SchemaRegistry, SchemaRegistryTrait};

// Create a registry
let mut registry = SchemaRegistry::new();

// Load a custom schema at runtime
registry.load_schema("path/to/custom_script.yaml")?;

// Use the loaded schema
if let Some(schema) = registry.get_schema("custom_script") {
    // Access mapping data
    println!("Loaded {} categories", schema.mappings.len());
}
```

## Proposed CLI Enhancement

To support runtime schemas, the CLI could be enhanced with:

```bash
# Load a custom schema and use it
shlesha transliterate --from custom_script --to devanagari \
    --schema /path/to/custom_script.yaml \
    "custom text"

# Load multiple schemas
shlesha transliterate --from script1 --to script2 \
    --schema /path/to/script1.yaml \
    --schema /path/to/script2.yaml \
    "text"

# Mix built-in and custom schemas
shlesha transliterate --from custom_script --to devanagari \
    --schema /path/to/custom_script.yaml \
    "text"
```

## Implementation Requirements

To add CLI runtime schema support:

1. **Enhanced CLI Arguments**:
   ```rust
   #[derive(Parser)]
   struct Cli {
       /// Custom schema files to load
       #[arg(long, action = clap::ArgAction::Append)]
       schema: Vec<PathBuf>,
       // ... existing args
   }
   ```

2. **Registry-based Shlesha Instance**:
   ```rust
   // Instead of: let transliterator = Shlesha::new();
   let mut registry = SchemaRegistry::new();
   
   // Load custom schemas
   for schema_path in &args.schema {
       registry.load_schema(schema_path)?;
   }
   
   // Create transliterator with custom registry
   let transliterator = Shlesha::with_registry(registry);
   ```

3. **Error Handling**:
   - Schema file not found
   - Invalid YAML format
   - Schema validation errors
   - Script not found in loaded schemas

## Use Cases for Runtime Schemas

### 1. Custom Script Variants
```yaml
# custom_iast.yaml - IAST variant with different notation
metadata:
  name: "custom_iast"
  script_type: "roman"
  description: "Custom IAST variant"

target: "iso15919"

mappings:
  vowels:
    "ṛ": "r̥"
    "ṝ": "r̥̄"
    # Custom: use 'x' for vocalic l
    "x": "l̥"    
```

### 2. Experimental Scripts
```yaml
# experimental.yaml - Testing new script support
metadata:
  name: "experimental"
  script_type: "brahmic"
  description: "Experimental script for testing"

mappings:
  consonants:
    "𑀓": "क"    # Brahmi script
    "𑀔": "ख"
```

### 3. Historical Scripts
```yaml
# vedic.yaml - Vedic Sanskrit extensions
metadata:
  name: "vedic"
  script_type: "roman"
  description: "Vedic Sanskrit with accent marks"

target: "iso15919"

mappings:
  vowels:
    "á": "á"     # Acute accent
    "à": "à"     # Grave accent
```

## Benefits of Runtime Loading

1. **No Recompilation**: Add scripts without rebuilding the binary
2. **Experimentation**: Test new schema variations quickly
3. **Customization**: Users can modify mappings for their needs
4. **Distribution**: Share custom schemas as separate files

## Current Workaround

For now, users who need custom schemas can:

1. **Add to Built-in Schemas**: Place YAML in `schemas/` directory and rebuild
2. **Use Library Directly**: Write Rust code using the `SchemaRegistry`
3. **Fork and Modify**: Create a custom version with their schemas

## Example: Complete Runtime Schema Loading

```rust
use shlesha::modules::registry::{SchemaRegistry, SchemaRegistryTrait};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = SchemaRegistry::new();
    
    // Load custom schemas
    registry.load_schema("schemas/my_custom_script.yaml")?;
    registry.load_schema("schemas/experimental.yaml")?;
    
    // List loaded schemas
    println!("Loaded schemas:");
    for name in registry.list_schemas() {
        if let Some(schema) = registry.get_schema(&name) {
            println!("  {} - {}", name, schema.metadata.description);
        }
    }
    
    // Use in conversions (when Shlesha supports registry)
    // let transliterator = Shlesha::with_registry(registry);
    // let result = transliterator.transliterate("text", "custom", "devanagari")?;
    
    Ok(())
}
```

See `examples/test_runtime_schema_loading.rs` for a working example.