# Zero Data Loss Architecture

## Fundamental Principle: Information Preservation

### What is "No Data Loss"?
1. **Reversibility**: A→B→A must be possible (not necessarily identical)
2. **Information preservation**: All semantic content retained
3. **Lossless encoding**: Unknown elements preserved with sufficient context

### Data Loss Detection
```rust
// Formal verification approach
pub struct LossDetector {
    original_entropy: f64,
    encoded_entropy: f64,
    preserved_chars: HashSet<char>,
}

impl LossDetector {
    pub fn verify_lossless(original: &str, encoded: &str) -> LossAnalysis {
        LossAnalysis {
            is_lossless: self.can_reconstruct(original, encoded),
            preserved_information: self.calculate_preservation_ratio(original, encoded),
            reconstruction_method: self.find_reconstruction_path(original, encoded),
        }
    }
}
```

### Token-Based Preservation
```rust
// Simple, efficient token format
// Format: `[script_id:data]` where script_id is u8, data is base64
pub struct PreservationToken {
    source_script: u8,        // 1 byte
    data: SmallVec<[u8; 8]>,  // inline for small data
}

// Examples:
// Unknown char: [15:क] (script 15 = Devanagari)
// Complex cluster: [15:क्ष्म्य] (entire cluster preserved)
```

## Simplified Architecture

### Core Components (Only 3!)

```rust
// 1. Mapper: Pure function, zero allocation
pub struct Mapper {
    table: &'static [(u32, u32)], // (from_char, to_char) pairs
    fallback: fn(char, u8) -> Token, // For unknowns
}

// 2. Token: Minimal overhead preservation
#[repr(packed)]
pub struct Token {
    script_id: u8,
    len: u8,
    data: [u8; 14], // Total: 16 bytes
}

// 3. Registry: Extension point
pub struct MapperRegistry {
    mappers: HashMap<(u8, u8), &'static Mapper>,
    scripts: &'static [&'static str],
}
```

### Performance-First Design

```rust
pub struct FastTransliterator {
    registry: &'static MapperRegistry,
}

impl FastTransliterator {
    // Zero allocation transliteration
    pub fn transliterate_to<W: Write>(
        &self,
        input: &str,
        from: u8,
        to: u8,
        output: &mut W
    ) -> Result<(), Error> {
        let mapper = self.registry.get(from, to)?;
        
        for ch in input.chars() {
            match mapper.lookup(ch) {
                Some(mapped) => output.write_str(mapped)?,
                None => {
                    let token = (mapper.fallback)(ch, from);
                    write!(output, "[{}:{}]", from, token.encode_b64())?;
                }
            }
        }
        Ok(())
    }
}
```

## Extensibility Architecture

### 1. Compile-Time Extension System
```rust
// Define new scripts at compile time
#[derive(ScriptId)]
pub struct MyCustomScript;

// Auto-generate mapper
#[mapper(from = Devanagari, to = MyCustomScript)]
const MY_MAPPER: Mapper = mapper! {
    'क' => "ka",
    'ख' => "kha",
    // ... compile-time generation
};
```

### 2. Runtime Extension System  
```rust
// Hot-loadable mappers
pub struct DynamicMapper {
    base: &'static Mapper,
    overrides: HashMap<char, String>,
    fallback_handler: Box<dyn Fn(char, u8) -> Token>,
}

// Plugin interface
pub trait TransliterationPlugin {
    fn script_id(&self) -> u8;
    fn script_name(&self) -> &str;
    fn create_mapper(&self, target: u8) -> Option<DynamicMapper>;
    fn handle_unknown(&self, ch: char) -> Token;
}
```

### 3. Schema Evolution
```rust
// Forward-compatible schema format
#[derive(Serialize, Deserialize)]
pub struct SchemaV2 {
    version: u32,
    script_id: u8,
    mappings: CompactMappings,
    extensions: Vec<Extension>,
}

// Compact binary format for performance
pub struct CompactMappings {
    // Trie-compressed mapping table
    nodes: &'static [TrieNode],
    // Direct char->char mappings
    simple: &'static [(u32, u32)],
}
```

## Performance Optimizations

### 1. Zero-Copy Processing
```rust
// Process without intermediate allocations
pub fn transliterate_zero_copy(
    input: &str,
    mapper: &Mapper,
    output: &mut String
) {
    output.reserve(input.len() * 2); // Estimate
    
    let mut bytes = input.bytes();
    while let Some(ch) = decode_utf8(&mut bytes) {
        if let Some(mapped) = mapper.lookup_static(ch) {
            output.push_str(mapped); // Static str, no allocation
        } else {
            write_token(output, ch, mapper.script_id);
        }
    }
}
```

### 2. SIMD-Accelerated Processing
```rust
// Use SIMD for ASCII portions
#[cfg(target_feature = "avx2")]
pub fn transliterate_ascii_simd(input: &[u8], output: &mut Vec<u8>) {
    use std::arch::x86_64::*;
    
    // Process 32 bytes at once for ASCII romanizations
    for chunk in input.chunks_exact(32) {
        let chars = unsafe { _mm256_loadu_si256(chunk.as_ptr() as *const __m256i) };
        let result = process_ascii_chunk(chars);
        output.extend_from_slice(&result);
    }
}
```

### 3. Constant-Time Lookups
```rust
// Perfect hash tables for small alphabets
pub struct PerfectHashMapper {
    hash_fn: fn(char) -> u8,
    table: &'static [Option<&'static str>; 256],
}

// Trie for larger scripts
pub struct TrieMapper {
    root: &'static TrieNode,
}

struct TrieNode {
    children: &'static [(char, &'static TrieNode)],
    value: Option<&'static str>,
}
```

## Simplified Implementation

### Core API (5 functions only!)
```rust
pub struct Shlesha {
    registry: &'static Registry,
}

impl Shlesha {
    // 1. Create with built-in scripts
    pub fn new() -> Self;
    
    // 2. Add custom script
    pub fn with_plugin<P: Plugin>(self, plugin: P) -> Self;
    
    // 3. Fast transliteration  
    pub fn transliterate(&self, text: &str, from: &str, to: &str) -> String;
    
    // 4. Streaming transliteration
    pub fn transliterate_stream<R, W>(&self, input: R, output: W, from: &str, to: &str);
    
    // 5. Verify lossless
    pub fn verify_lossless(&self, original: &str, encoded: &str) -> bool;
}
```

### Usage Examples
```rust
// Simple usage
let shlesha = Shlesha::new();
let result = shlesha.transliterate("धर्म", "Devanagari", "IAST");

// Custom script
let shlesha = Shlesha::new()
    .with_plugin(MyCustomScript::new());

// Streaming (zero allocation)
let mut output = Vec::new();
shlesha.transliterate_stream(input_file, &mut output, "Devanagari", "IAST");

// Verify preservation
assert!(shlesha.verify_lossless("धर्म", "[15:धर्म]"));
```

## Data Loss Verification

### Mathematical Approach
```rust
// Information-theoretic verification
pub fn verify_information_preservation(original: &str, encoded: &str) -> f64 {
    let original_entropy = calculate_entropy(original);
    let encoded_entropy = calculate_entropy(encoded); 
    let token_recovery_entropy = calculate_token_entropy(encoded);
    
    (encoded_entropy + token_recovery_entropy) / original_entropy
}

// Ratio of 1.0 = perfect preservation
// Ratio > 0.95 = acceptable preservation
// Ratio < 0.95 = data loss detected
```

### Practical Verification
```rust
// Round-trip testing
pub fn test_preservation(text: &str, path: &[&str]) -> PreservationResult {
    let mut current = text.to_string();
    let mut tokens_created = 0;
    
    for i in 0..path.len()-1 {
        let result = transliterate(&current, path[i], path[i+1]);
        tokens_created += count_tokens(&result);
        current = result;
    }
    
    PreservationResult {
        final_text: current,
        tokens_created,
        can_reconstruct: tokens_created == 0 || has_reconstruction_path(&current),
        preservation_ratio: calculate_similarity(text, &current),
    }
}
```

## Performance Targets

| Operation | Target | Current | Improvement |
|-----------|--------|---------|-------------|
| Simple transliteration | 50 ns/char | 500 ns/char | 10x |
| With tokens | 100 ns/char | 1000 ns/char | 10x |
| Memory usage | 16 bytes/text | 144 bytes/char | 100x |
| Startup time | 1 ms | 45 ms | 45x |
| Binary size | 2 MB | 8 MB | 4x |

## Extension Points

### 1. Script Plugins
```rust
pub trait ScriptPlugin {
    fn id(&self) -> u8;
    fn name(&self) -> &str;
    fn create_mappers(&self) -> Vec<(u8, Mapper)>;
}
```

### 2. Processing Plugins  
```rust
pub trait ProcessorPlugin {
    fn can_handle(&self, from: u8, to: u8) -> bool;
    fn process(&self, input: &str, from: u8, to: u8) -> String;
}
```

### 3. Token Handlers
```rust
pub trait TokenHandler {
    fn encode(&self, ch: char, script: u8) -> Token;
    fn decode(&self, token: &Token) -> (char, u8);
    fn can_reconstruct(&self, token: &Token, target_script: u8) -> bool;
}
```

This architecture provides:
- 🚀 **10x performance improvement**
- 🔧 **Simple extensibility** via plugins
- 🛡️ **Guaranteed no data loss** via tokens
- 📦 **Minimal complexity** (3 core types)
- 🎯 **Use-case optimization** without architectural complexity