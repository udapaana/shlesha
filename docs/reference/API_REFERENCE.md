# Shlesha API Reference

Complete API documentation for all Shlesha interfaces: Rust, Python, WASM, and CLI.

## 🦀 Rust API

### Core Library

```rust
use shlesha::Shlesha;

// Create transliterator
let transliterator = Shlesha::new();

// Basic transliteration
let result = transliterator.transliterate("धर्म", "devanagari", "iast")?;
// Returns: "dharma"

// Transliteration with metadata
let result = transliterator.transliterate_with_metadata("धर्मkr", "devanagari", "iast")?;
// result.output: "dharmakr"
// result.metadata: Contains unknown token info

// Script discovery
let scripts = transliterator.list_supported_scripts();
let supported = transliterator.supports_script("devanagari");

// Runtime schema loading  
transliterator.load_schema_from_file("path/to/schema.yaml")?;
transliterator.load_schema_from_string(yaml_content, "custom_script")?;
```

### Types

```rust
pub struct Shlesha { /* ... */ }

pub struct TransliterationResult {
    pub output: String,
    pub metadata: Option<TransliterationMetadata>,
}

pub struct TransliterationMetadata {
    pub source_script: String,
    pub target_script: String,
    pub used_extensions: String,
    pub unknown_tokens: Vec<UnknownToken>,
}

pub struct UnknownToken {
    pub script: String,
    pub token: char,
    pub position: usize,
    pub unicode: String,
    pub is_extension: bool,
}
```

### Runtime Schema Management

**NEW**: Direct schema management through the main `Shlesha` API:

```rust
use shlesha::{Shlesha, SchemaInfo};

let mut transliterator = Shlesha::new();

// Load schema from file
transliterator.load_schema_from_file("path/to/custom_script.yaml")?;

// Load schema from YAML string
let yaml_content = r#"
metadata:
  name: "my_script"
  script_type: "roman"
  has_implicit_a: false
  description: "My custom script"
target: "iso15919"
mappings:
  vowels:
    "a": "a"
  consonants:
    "k": "k"
"#;
transliterator.load_schema_from_string(yaml_content, "my_script")?;

// Get schema information
if let Some(info) = transliterator.get_schema_info("my_script") {
    println!("Name: {}", info.name);
    println!("Description: {}", info.description);
    println!("Type: {}", info.script_type);
    println!("Runtime loaded: {}", info.is_runtime_loaded);
    println!("Mapping count: {}", info.mapping_count);
}

// List all supported scripts (built-in + runtime)
let scripts = transliterator.list_supported_scripts();
println!("Available scripts: {:?}", scripts);

// Check if script is supported
if transliterator.supports_script("my_script") {
    println!("Custom script is ready to use!");
}

// Remove a runtime schema
let removed = transliterator.remove_schema("my_script");
println!("Schema removed: {}", removed);

// Clear all runtime schemas
transliterator.clear_runtime_schemas();

// Create with custom registry
use shlesha::modules::registry::SchemaRegistry;
let mut registry = SchemaRegistry::new();
registry.load_schema("custom.yaml")?;
let transliterator = Shlesha::with_registry(registry);
```

**Legacy Registry API** (still available for advanced use cases):

```rust
use shlesha::modules::registry::{SchemaRegistry, SchemaRegistryTrait};

let mut registry = SchemaRegistry::new();
registry.load_schema("path/to/custom_script.yaml")?;
registry.load_schemas_from_directory("schemas/")?;

if let Some(schema) = registry.get_schema("script_name") {
    // Access raw schema data
}
```

## 🐍 Python API

### Installation & Import

```python
# After building with uv and maturin
# uv run maturin develop --features python

import shlesha

# Or use convenience functions
from shlesha import transliterate, get_supported_scripts
```

### Basic Usage

```python
# Create transliterator
transliterator = shlesha.Shlesha()

# Basic transliteration
result = transliterator.transliterate("धर्म", "devanagari", "iast")
# Returns: "dharma"

# With metadata
result = transliterator.transliterate_with_metadata("धर्मkr", "devanagari", "iast")
print(result.output)  # "dharmakr"
print(len(result.metadata.unknown_tokens))  # 2

# Script discovery  
scripts = transliterator.list_supported_scripts()
supported = transliterator.supports_script("devanagari")

# Get script information
info = transliterator.get_script_info()
print(info["devanagari"])  # "Devanagari script (देवनागरी)"
```

### Convenience Functions

```python
# Direct transliteration
result = shlesha.transliterate("अ", "devanagari", "iast")

# Get all scripts
scripts = shlesha.get_supported_scripts()

# Create transliterator instance
t = shlesha.create_transliterator()
```

### Runtime Schema Loading

**NEW**: Python bindings now support comprehensive runtime schema loading:

```python
import shlesha

# Create transliterator
transliterator = shlesha.Shlesha()

# Load schema from file
transliterator.load_schema_from_file("path/to/custom_script.yaml")

# Load schema from YAML string
yaml_content = """
metadata:
  name: "my_script"
  script_type: "roman"
  has_implicit_a: false
  description: "My custom script"
target: "iso15919"
mappings:
  vowels:
    "a": "a"
  consonants:
    "k": "k"
"""
transliterator.load_schema_from_string(yaml_content, "my_script")

# Get schema information
info = transliterator.get_schema_info("my_script")
if info:
    print(f"Name: {info['name']}")
    print(f"Description: {info['description']}")
    print(f"Type: {info['script_type']}")
    print(f"Runtime loaded: {info['is_runtime_loaded']}")
    print(f"Mapping count: {info['mapping_count']}")

# Use the runtime schema
result = transliterator.transliterate("ka", "my_script", "devanagari")
print(result)  # "क"

# Remove a runtime schema
removed = transliterator.remove_schema("my_script")
print(f"Schema removed: {removed}")

# Clear all runtime schemas
transliterator.clear_runtime_schemas()
```

### Classes

```python
class Shlesha:
    def transliterate(self, text: str, from_script: str, to_script: str) -> str
    def transliterate_with_metadata(self, text: str, from_script: str, to_script: str) -> TransliterationResult
    def list_supported_scripts(self) -> List[str]
    def supports_script(self, script: str) -> bool
    def load_schema(self, schema_path: str) -> None  # Legacy method
    def get_script_info(self) -> Dict[str, str]
    
    # Runtime schema loading methods
    def load_schema_from_file(self, file_path: str) -> None
    def load_schema_from_string(self, yaml_content: str, schema_name: str) -> None
    def get_schema_info(self, script_name: str) -> Optional[Dict[str, Any]]
    def remove_schema(self, script_name: str) -> bool
    def clear_runtime_schemas(self) -> None

class TransliterationResult:
    output: str
    metadata: Optional[TransliterationMetadata]

class TransliterationMetadata:
    source_script: str
    target_script: str
    used_extensions: str
    unknown_tokens: List[UnknownToken]

class UnknownToken:
    script: str
    token: str
    position: int
    unicode: str
    is_extension: bool
```

## 🕸️ WASM/JavaScript API

### Installation

```html
<!-- In HTML -->
<script type="module">
import init, { WasmShlesha, transliterate, getSupportedScripts } from './pkg/shlesha.js';

await init(); // Initialize WASM module
</script>
```

### Basic Usage

```javascript
// Initialize WASM (required)
await init();

// Direct function
const result = transliterate("धर्म", "devanagari", "iast");
console.log(result); // "dharma"

// Class usage
const transliterator = new WasmShlesha();
const result2 = transliterator.transliterate("अ", "devanagari", "iast");

// With metadata
const metaResult = transliterator.transliterateWithMetadata("धर्मkr", "devanagari", "iast");
console.log(metaResult.getOutput()); // "dharmakr"
console.log(metaResult.getUnknownTokenCount()); // 2

// Script discovery
const scripts = transliterator.listSupportedScripts();
const supported = transliterator.supportsScript("devanagari");
const info = transliterator.getScriptInfo();
```

### Runtime Schema Loading

**NEW**: WASM bindings now support comprehensive runtime schema loading:

```javascript
import init, { WasmShlesha } from './pkg/shlesha.js';

async function loadCustomScript() {
    await init();
    const transliterator = new WasmShlesha();
    
    // Load schema from YAML string
    const yamlContent = `
metadata:
  name: "custom_script"
  script_type: "roman"
  has_implicit_a: false
  description: "Custom script"
target: "iso15919"
mappings:
  vowels:
    "a": "a"
  consonants:
    "k": "k"
`;
    
    // Load at runtime
    transliterator.loadSchemaFromString(yamlContent, "custom_script");
    
    // Use immediately
    const result = transliterator.transliterate("ka", "custom_script", "devanagari");
    console.log(result); // "क"
    
    // Get schema information
    const info = transliterator.getSchemaInfo("custom_script");
    if (info) {
        console.log(`Name: ${info.name}`);
        console.log(`Description: ${info.description}`);
        console.log(`Type: ${info.script_type}`);
        console.log(`Runtime loaded: ${info.is_runtime_loaded}`);
        console.log(`Mapping count: ${info.mapping_count}`);
    }
    
    // Remove schema
    const removed = transliterator.removeSchema("custom_script");
    console.log(`Schema removed: ${removed}`);
    
    // Clear all runtime schemas
    transliterator.clearRuntimeSchemas();
}
```

### Classes

```javascript
class WasmShlesha {
    transliterate(text, fromScript, toScript) // -> string
    transliterateWithMetadata(text, fromScript, toScript) // -> WasmTransliterationResult
    listSupportedScripts() // -> Array<string>
    supportsScript(script) // -> boolean
    loadSchema(schemaPath) // -> void  // Legacy method
    getScriptInfo() // -> Object
    getSupportedScriptCount() // -> number
    
    // Runtime schema loading methods
    loadSchemaFromFile(filePath) // -> void
    loadSchemaFromString(yamlContent, schemaName) // -> void
    getSchemaInfo(scriptName) // -> Object|undefined
    removeSchema(scriptName) // -> boolean
    clearRuntimeSchemas() // -> void
}

class WasmTransliterationResult {
    getOutput() // -> string
    hasMetadata() // -> boolean
    getSourceScript() // -> string|null
    getTargetScript() // -> string|null
    getUnknownTokenCount() // -> number
    getUnknownTokens() // -> Array<Object>
}

// Convenience functions
transliterate(text, fromScript, toScript) // -> string
getSupportedScripts() // -> Array<string>
createTransliterator() // -> WasmShlesha
getVersion() // -> string
```

## ⚡ CLI API

### Installation

```bash
# Build from source
cargo build --release --features cli
# Binary: ./target/release/shlesha
```

### Basic Usage

```bash
# Basic transliteration
shlesha transliterate --from devanagari --to iast "धर्म"
# Output: dharma

# With metadata (inline format)
shlesha transliterate --from devanagari --to iast --show-metadata "धर्मkr"
# Output: dharmak[devanagari:k]r[devanagari:r]

# With detailed metadata
shlesha transliterate --from devanagari --to iast --verbose "धर्मkr"
# Output:
# dharmakr
# 
# Metadata:
#   Source: devanagari -> Target: iast
#   Extensions used: false
#   Unknown tokens: 2
#     1. 'k' at position 6 (U+006B)
#     2. 'r' at position 7 (U+0072)

# List supported scripts
shlesha scripts

# From stdin
echo "धर्म" | shlesha transliterate --from devanagari --to iast

# Script aliases
shlesha transliterate --from deva --to iso "धर्म"  # Same as devanagari -> iso15919
```

### Commands

```bash
shlesha transliterate [OPTIONS] --from <FROM> --to <TO> [TEXT]
shlesha scripts
shlesha help [COMMAND]
```

### Options

```bash
# transliterate command options:
-f, --from <FROM>         Source script (e.g., devanagari, iso)
-t, --to <TO>             Target script (e.g., devanagari, iso)  
    --show-metadata       Show unknown tokens inline: output[script:token]
-v, --verbose             Show detailed metadata breakdown
-h, --help                Print help

# Global options:
-h, --help                Print help
```

## 📋 Supported Scripts

### Indic Scripts
- **devanagari** (deva) - Hindi, Sanskrit, Marathi  
- **bengali** (bn) - Bengali/Bangla
- **tamil** (ta) - Tamil
- **telugu** (te) - Telugu  
- **gujarati** (gu) - Gujarati
- **kannada** (kn) - Kannada
- **malayalam** (ml) - Malayalam
- **odia** (od, oriya) - Odia

### Roman/ASCII Schemes
- **iast** - International Alphabet of Sanskrit Transliteration
- **itrans** - ASCII transliteration scheme
- **slp1** - Sanskrit Library Phonetic scheme  
- **harvard_kyoto** (hk) - Harvard-Kyoto convention
- **velthuis** - TeX-based notation
- **wx** - Computational notation
- **iso15919** (iso) - ISO-15919 international standard

## 🔗 Integration Examples

### Web Integration

```html
<!DOCTYPE html>
<html>
<head>
    <title>Shlesha Demo</title>
    <script type="module">
        import init, { WasmShlesha } from './pkg/shlesha.js';
        
        async function setupShlesha() {
            await init();
            const t = new WasmShlesha();
            
            document.getElementById('convert').onclick = () => {
                const text = document.getElementById('input').value;
                const result = t.transliterate(text, 'devanagari', 'iast');
                document.getElementById('output').textContent = result;
            };
        }
        
        setupShlesha();
    </script>
</head>
<body>
    <input id="input" placeholder="Enter Devanagari text">
    <button id="convert">Convert to IAST</button>
    <div id="output"></div>
</body>
</html>
```

### Python Data Pipeline

```python
import shlesha
import pandas as pd

def transliterate_column(df, column, from_script, to_script):
    """Transliterate a pandas column"""
    t = shlesha.Shlesha()
    df[f'{column}_{to_script}'] = df[column].apply(
        lambda x: t.transliterate(str(x), from_script, to_script)
    )
    return df

# Usage
df = pd.DataFrame({'sanskrit': ['धर्म', 'योग', 'भारत']})
df = transliterate_column(df, 'sanskrit', 'devanagari', 'iast')
print(df)
```

### Rust Library Integration

```rust
use shlesha::Shlesha;

fn batch_transliterate(texts: Vec<&str>, from: &str, to: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let transliterator = Shlesha::new();
    
    texts.into_iter()
        .map(|text| transliterator.transliterate(text, from, to))
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sanskrit_words = vec!["धर्म", "योग", "भारत"];
    let iast_words = batch_transliterate(sanskrit_words, "devanagari", "iast")?;
    
    for word in iast_words {
        println!("{}", word);
    }
    
    Ok(())
}
```

## 🎯 Error Handling

### Rust
```rust
match transliterator.transliterate("text", "invalid", "iast") {
    Ok(result) => println!("{}", result),
    Err(e) => eprintln!("Error: {}", e),
}
```

### Python
```python
try:
    result = transliterator.transliterate("text", "invalid", "iast")
except RuntimeError as e:
    print(f"Error: {e}")
```

### JavaScript
```javascript
try {
    const result = transliterator.transliterate("text", "invalid", "iast");
} catch (error) {
    console.error("Error:", error);
}
```

### CLI
```bash
# Returns non-zero exit code on error
shlesha transliterate --from invalid --to iast "text"
echo $?  # Non-zero exit code
```

## 🔧 Configuration

### Runtime Schema Loading
All APIs support comprehensive runtime schema loading for custom scripts. You can load schemas from files or directly from YAML strings:

```rust
// Rust API
let mut transliterator = Shlesha::new();

// From file
transliterator.load_schema_from_file("path/to/custom.yaml")?;

// From YAML string
let yaml_content = r#"
metadata:
  name: "custom"
  script_type: "roman"
  has_implicit_a: false
target: "iso15919"
mappings:
  vowels:
    "a": "a"
"#;
transliterator.load_schema_from_string(yaml_content, "custom")?;

// Schema management
let info = transliterator.get_schema_info("custom");
transliterator.remove_schema("custom");
transliterator.clear_runtime_schemas();
```

```python
# Python API
import shlesha
transliterator = shlesha.Shlesha()

# From file
transliterator.load_schema_from_file("path/to/custom.yaml")

# From YAML string
yaml_content = """
metadata:
  name: "custom"
  script_type: "roman"
  has_implicit_a: false
target: "iso15919"
mappings:
  vowels:
    "a": "a"
"""
transliterator.load_schema_from_string(yaml_content, "custom")

# Schema management
info = transliterator.get_schema_info("custom")
transliterator.remove_schema("custom")
transliterator.clear_runtime_schemas()
```

```javascript
// JavaScript/WASM API
import init, { WasmShlesha } from './pkg/shlesha.js';
await init();
const transliterator = new WasmShlesha();

// From YAML string
const yamlContent = `
metadata:
  name: "custom"
  script_type: "roman"
  has_implicit_a: false
target: "iso15919"
mappings:
  vowels:
    "a": "a"
`;
transliterator.loadSchemaFromString(yamlContent, "custom");

// Schema management
const info = transliterator.getSchemaInfo("custom");
transliterator.removeSchema("custom");
transliterator.clearRuntimeSchemas();
```

### Environment Variables
- `PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1` - For Python 3.13+ compatibility

### uv Commands
```bash
# Setup environment
uv sync --dev

# Build Python bindings
uv run maturin develop --features python

# Run Python with bindings
uv run python -c "import shlesha; print('Works!')"

# Run tests
uv run pytest python/tests/
```

## 🧪 Testing

All APIs include comprehensive test suites:

```bash
# All tests
./scripts/test-all.sh

# Individual API tests
cargo test                          # Rust
uv run pytest python/tests/           # Python  
wasm-pack test --node --features wasm  # WASM
cargo test --test cli_integration_tests  # CLI
```

This reference covers all public APIs and usage patterns for Shlesha across all supported platforms.