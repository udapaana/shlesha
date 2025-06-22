# Shlesha API Bindings

This document provides comprehensive information about using Shlesha across different programming languages and platforms.

## Overview

Shlesha offers full-featured bindings for multiple languages:
- **Rust** (native library)
- **Python** (via PyO3)
- **JavaScript/WebAssembly** (via wasm-bindgen)
- **Command Line Interface**

All bindings provide the same core functionality:
- Bidirectional transliteration between all supported scripts
- Metadata collection for unknown tokens
- Script discovery and validation
- Graceful error handling

## Python Bindings

### Installation

```bash
# Development installation
pip install maturin
maturin develop --features python

# Production build
maturin build --features python --release
pip install target/wheels/*.whl
```

### API Reference

#### Classes

##### `Shlesha`
Main transliterator class.

```python
transliterator = shlesha.Shlesha()
```

**Methods:**
- `transliterate(text, from_script, to_script) -> str`
- `transliterate_with_metadata(text, from_script, to_script) -> TransliterationResult`
- `list_supported_scripts() -> List[str]`
- `supports_script(script) -> bool`
- `load_schema(schema_path) -> None`
- `get_script_info() -> Dict[str, str]`

##### `TransliterationResult`
Result object containing output and metadata.

**Properties:**
- `output: str` - Transliterated text
- `metadata: Optional[TransliterationMetadata]` - Conversion metadata

##### `TransliterationMetadata`
Metadata about the transliteration process.

**Properties:**
- `source_script: str` - Source script name
- `target_script: str` - Target script name
- `used_extensions: str` - Extensions used
- `unknown_tokens: List[UnknownToken]` - Unknown tokens found

##### `UnknownToken`
Information about unknown/untranslatable tokens.

**Properties:**
- `script: str` - Script where token was found
- `token: str` - The unknown token
- `position: int` - Position in the text
- `unicode: str` - Unicode representation
- `is_extension: bool` - Whether from runtime extension

#### Convenience Functions

```python
# Direct transliteration
result = shlesha.transliterate("धर्म", "devanagari", "iast")

# Get supported scripts
scripts = shlesha.get_supported_scripts()

# Create transliterator instance
transliterator = shlesha.create_transliterator()
```

### Examples

```python
import shlesha

# Basic usage
transliterator = shlesha.Shlesha()
result = transliterator.transliterate("धर्म", "devanagari", "iast")
print(result)  # "dharma"

# With metadata
result = transliterator.transliterate_with_metadata("धर्मkr", "devanagari", "iast")
print(result.output)  # "dharmakr"
if result.metadata:
    print(f"Unknown tokens: {len(result.metadata.unknown_tokens)}")

# Script information
if transliterator.supports_script("gujarati"):
    result = transliterator.transliterate("dharma", "iast", "gujarati")
    print(result)  # "ધર્મ"
```

## WebAssembly Bindings

### Building

```bash
# Install wasm-pack
cargo install wasm-pack

# Build for web browsers
wasm-pack build --target web --out-dir pkg --features wasm

# Build for Node.js
wasm-pack build --target nodejs --out-dir pkg-node --features wasm

# Build for bundlers (webpack, etc.)
wasm-pack build --target bundler --out-dir pkg-bundler --features wasm
```

### API Reference

#### Classes

##### `WasmShlesha`
Main transliterator class for WebAssembly.

```javascript
const transliterator = new WasmShlesha();
```

**Methods:**
- `transliterate(text, fromScript, toScript) -> string`
- `transliterateWithMetadata(text, fromScript, toScript) -> WasmTransliterationResult`
- `listSupportedScripts() -> Array<string>`
- `supportsScript(script) -> boolean`
- `loadSchema(schemaPath) -> void`
- `getScriptInfo() -> Object`
- `getSupportedScriptCount() -> number`

##### `WasmTransliterationResult`
Result object with output and metadata access methods.

**Methods:**
- `getOutput() -> string`
- `hasMetadata() -> boolean`
- `getSourceScript() -> string | null`
- `getTargetScript() -> string | null`
- `getUnknownTokenCount() -> number`
- `getUnknownTokens() -> Array<Object>`

#### Convenience Functions

```javascript
// Direct transliteration
const result = transliterate("धर्म", "devanagari", "iast");

// Get supported scripts
const scripts = getSupportedScripts();

// Create transliterator instance
const transliterator = createTransliterator();

// Get version
const version = getVersion();
```

### Usage Examples

#### Web Browser

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Shlesha Demo</title>
</head>
<body>
    <script type="module">
        import init, { WasmShlesha, transliterate } from './pkg/shlesha.js';
        
        async function main() {
            await init();
            
            // Using class
            const transliterator = new WasmShlesha();
            const result = transliterator.transliterate("धर्म", "devanagari", "iast");
            console.log(result); // "dharma"
            
            // Using convenience function
            const result2 = transliterate("dharma", "iast", "devanagari");
            console.log(result2); // "धर्म"
            
            // With metadata
            const withMeta = transliterator.transliterateWithMetadata("धर्मkr", "devanagari", "iast");
            console.log(withMeta.getOutput()); // "dharmakr"
            console.log(withMeta.getUnknownTokenCount()); // 2
        }
        
        main();
    </script>
</body>
</html>
```

#### Node.js

```javascript
const { WasmShlesha, transliterate } = require('./pkg-node/shlesha.js');

// Direct usage
const result = transliterate("धर्म", "devanagari", "iast");
console.log(result); // "dharma"

// Class usage
const transliterator = new WasmShlesha();
const scripts = transliterator.listSupportedScripts();
console.log(`Supports ${scripts.length} scripts`);
```

## Command Line Interface

### Installation

```bash
cargo install --path . --features cli
```

### Usage

```bash
# Basic transliteration
shlesha transliterate --from devanagari --to iast "धर्म"

# With metadata display (inline)
shlesha transliterate --from devanagari --to iast --show-metadata "धर्मkr"

# With detailed metadata
shlesha transliterate --from devanagari --to iast --verbose "धर्मkr"

# List supported scripts
shlesha scripts

# Read from stdin
echo "धर्म" | shlesha transliterate --from devanagari --to iast

# Process file
shlesha transliterate --from devanagari --to iast < input.txt > output.txt
```

### Flags

- `--from`, `-f`: Source script name
- `--to`, `-t`: Target script name  
- `--show-metadata`: Show unknown tokens inline as `[script:token]`
- `--verbose`, `-v`: Show detailed metadata breakdown
- `--help`, `-h`: Show help information

## Error Handling

All bindings implement graceful error handling:

### Python
```python
try:
    result = transliterator.transliterate("text", "invalid_script", "iast")
except RuntimeError as e:
    print(f"Error: {e}")
```

### JavaScript
```javascript
try {
    const result = transliterator.transliterate("text", "invalid_script", "iast");
} catch (error) {
    console.error(`Error: ${error.message}`);
}
```

### CLI
Exit codes:
- `0`: Success
- `1`: Transliteration error or invalid arguments

## Performance Considerations

### Python
- Use the class instance for multiple conversions to avoid repeated initialization
- Metadata collection has minimal overhead when not used

### WebAssembly
- WASM module initialization is asynchronous - await `init()` before use
- Consider using Web Workers for large batch processing
- Module size is optimized for web delivery

### CLI
- Supports stdin/stdout for pipeline integration
- Efficient for single conversions and batch processing

## Building from Source

### Requirements
- Rust 1.70+
- Python 3.8+ (for Python bindings)
- Node.js 14+ (for WASM testing)

### Python Development
```bash
# Install development dependencies
pip install maturin pytest

# Build and install in development mode
maturin develop --features python

# Run tests
pytest python/tests/
```

### WASM Development
```bash
# Install wasm-pack
cargo install wasm-pack

# Build and test
wasm-pack build --features wasm
wasm-pack test --node --features wasm

# Serve demo
python3 -m http.server 8000
# Open http://localhost:8000/demo.html
```

## Integration Examples

### Python Data Processing
```python
import pandas as pd
import shlesha

transliterator = shlesha.Shlesha()

# Process DataFrame column
df['transliterated'] = df['sanskrit_text'].apply(
    lambda x: transliterator.transliterate(x, "devanagari", "iast")
)
```

### JavaScript Web App
```javascript
// Async transliteration function
async function translateText(text, from, to) {
    if (!window.shleshaReady) {
        await init();
        window.transliterator = new WasmShlesha();
        window.shleshaReady = true;
    }
    
    return window.transliterator.transliterate(text, from, to);
}

// Use in form handler
document.getElementById('translateBtn').onclick = async () => {
    const text = document.getElementById('inputText').value;
    const result = await translateText(text, 'devanagari', 'iast');
    document.getElementById('output').textContent = result;
};
```

### Shell Script Processing
```bash
#!/bin/bash

# Batch convert files
for file in *.txt; do
    echo "Converting $file..."
    shlesha transliterate --from devanagari --to iast < "$file" > "converted_$file"
done

# Process with metadata
echo "धर्मkr" | shlesha transliterate --from devanagari --to iast --verbose > conversion_report.txt
```

## Troubleshooting

### Python Issues
- **Import errors**: Ensure maturin development install completed successfully
- **Runtime errors**: Check Python version compatibility (3.8+)
- **Performance**: Use class instances for multiple conversions

### WASM Issues  
- **Module not loading**: Ensure proper async initialization with `await init()`
- **CORS errors**: Serve files over HTTP, not file:// protocol
- **Memory issues**: Consider chunking large text processing

### CLI Issues
- **Command not found**: Ensure `~/.cargo/bin` is in your PATH
- **Permission errors**: Check file permissions for input/output files
- **Encoding issues**: Ensure terminal supports UTF-8 encoding

## Contributing

See the main README.md for contribution guidelines. When adding binding features:

1. Update all three binding implementations (Python, WASM, CLI)
2. Add corresponding tests for each binding
3. Update this documentation
4. Ensure backward compatibility