# Shlesha Architecture Documentation

## Overview

Shlesha implements a token-based architecture for transliteration between Sanskrit and Indic scripts. The design uses compile-time code generation while maintaining runtime extensibility via dynamic compilation.

## Core Architectural Principles

### 1. Zero-Allocation Token Processing

```
Input String → Token Stream → Token Processing → Output String
     ↓              ↓               ↓               ↓
   "dharma"  →  [VowelA, ConsonantDh,  →  Pattern    →  "धर्म"
                 ConsonantR, ConsonantM,   Matching
                 VowelA]                   (Zero Alloc)
```

### 2. Dual-Hub Token Architecture

```
Alphabet Tokens          Hub Processing           Abugida Tokens
      |                       |                        |
  VowelA     ←→         Trait-Based            ←→    VowelA
  ConsonantK ←→         Conversion             ←→    ConsonantK
  MarkAnusvara ←→       (Stack-Based)          ←→    MarkAnusvara
      |                       |                        |
  Harvard-Kyoto              Hub                  Devanagari
  IAST, SLP1, etc.                               Bengali, etc.
```

The hub uses a trait-based converter that:
- Uses token traits (is_consonant(), is_vowel(), etc.) for classification
- Implements a stack-based algorithm for implicit 'a' handling
- Generates mappings from schemas at build time
- Handles consonant clusters with proper virama insertion

### 3. Build-Time Code Generation

**Static Processors**: Compile-time generated pattern matching
```rust
// Generated at compile time
pub fn string_to_token(&self, input: &str) -> Option<AlphabetToken> {
    match input {
        "a" => Some(AlphabetToken::VowelA),
        "ā" => Some(AlphabetToken::VowelAa),
        "RR" => Some(AlphabetToken::VowelRr),
        "lRR" => Some(AlphabetToken::VowelLl),
        "l̥̄" => Some(AlphabetToken::VowelLl),
        _ => None,
    }
}

pub fn token_to_string(&self, token: &AlphabetToken) -> &'static str {
    match token {
        AlphabetToken::VowelA => "a",
        AlphabetToken::VowelAa => "ā", 
        AlphabetToken::VowelLl => "lRR",
        // ...
    }
}
```

### 4. Runtime Compilation System

**Dynamic Processor Generation**
```rust
pub struct RuntimeCompiler {
    template_engine: Handlebars,    // Reuse build.rs templates
    cache_dir: PathBuf,             // Compiled schema cache
    build_tools: BuildTools,        // Same code generation
}

impl RuntimeCompiler {
    pub fn compile_schema(&self, schema: &Schema) -> Result<CompiledProcessor, Error> {
        // 1. Generate identical code to build.rs
        let code = self.template_engine.render("token_based_converter", &schema)?;
        
        // 2. Create temporary crate
        let crate_dir = self.create_temp_crate(&schema, &code)?;
        
        // 3. Cargo build --release
        self.cargo_build(&crate_dir)?;
        
        // 4. Load compiled dylib
        Ok(self.load_processor(&crate_dir)?)
    }
}
```

## Token System Design

### 1. Core Token Enums

**Abugida Tokens** (for Indic scripts):
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbugidaToken {
    // Vowels (independent)
    VowelA, VowelAa, VowelI, VowelIi, VowelU, VowelUu,
    VowelR, VowelRr, VowelL, VowelLl,
    VowelE, VowelAi, VowelO, VowelAu,
    
    // Vowel signs (dependent) 
    VowelSignAa, VowelSignI, VowelSignIi, VowelSignU, VowelSignUu,
    VowelSignR, VowelSignRr, VowelSignL, VowelSignLl,
    VowelSignE, VowelSignAi, VowelSignO, VowelSignAu,
    
    // Consonants
    ConsonantK, ConsonantKh, ConsonantG, ConsonantGh, ConsonantNg,
    // ... all Sanskrit consonants
    
    // Marks
    MarkAnusvara, MarkVisarga, MarkCandrabindu, MarkNukta, 
    MarkVirama, MarkAvagraha,
    
    // Special/Vedic
    MarkUdatta, MarkAnudatta, MarkDoubleSvarita, MarkTripleSvarita,
    
    // Digits
    Digit0, Digit1, Digit2, Digit3, Digit4, Digit5, Digit6, Digit7, Digit8, Digit9,
    
    // Unknown characters (rare)
    Unknown(char),
}
```

**Alphabet Tokens** (for Roman scripts):
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AlphabetToken {
    // Vowels
    VowelA, VowelAa, VowelI, VowelIi, VowelU, VowelUu,
    VowelR, VowelRr, VowelL, VowelLl,
    VowelE, VowelAi, VowelO, VowelAu,
    
    // Consonants  
    ConsonantK, ConsonantKh, ConsonantG, ConsonantGh, ConsonantNg,
    // ... all Sanskrit consonants
    
    // Marks
    MarkAnusvara, MarkVisarga, MarkCandrabindu, MarkAvagraha,
    
    // Extended nukta consonants
    ExtendedQ, ExtendedZ, ExtendedF, ExtendedGh, ExtendedKh,
    ExtendedRr, ExtendedRrh, ExtendedY,
    
    // Digits
    Digit0, Digit1, Digit2, Digit3, Digit4, Digit5, Digit6, Digit7, Digit8, Digit9,
}
```

### 2. Ambiguity Resolution

**Harvard-Kyoto Schema** (token-first format):
```yaml
mappings:
  vowels:
    VowelLl: ["lRR", "l̥̄"]  # Multiple inputs → same token
    VowelRr: ["RR", "r̥̄"]   # First = preferred output
```

**Conversion Flow**:
```
Input "lRR" → VowelLl → Output "lRR" ✅
Input "l̥̄"  → VowelLl → Output "lRR" ✅ (normalized to preferred)
```

This eliminates round-trip ambiguity.

## System Components

### 1. Hub Token Processing (`src/modules/hub/`)

**Token-to-Token Conversion**:
```rust
impl HubTrait for Hub {
    fn abugida_to_alphabet_tokens(&self, tokens: &HubTokenSequence) -> Result<HubTokenSequence, HubError> {
        tokens.iter().map(|token| match token {
            HubToken::Abugida(AbugidaToken::VowelA) => HubToken::Alphabet(AlphabetToken::VowelA),
            HubToken::Abugida(AbugidaToken::ConsonantK) => HubToken::Alphabet(AlphabetToken::ConsonantK),
            // Direct 1:1 token mapping
        }).collect()
    }
    
    fn alphabet_to_abugida_tokens(&self, tokens: &HubTokenSequence) -> Result<HubTokenSequence, HubError> {
        // Reverse conversion handling implicit vowels and virama placement
        // e.g., consonant sequences require virama between them
    }
}
```

### 2. Generated Token Processors (`build.rs` + Runtime)

**Template-Based Code Generation**:
```handlebars
// Generated token-based converter for {{script_name}}
pub struct {{struct_name}};

impl {{struct_name}} {
    pub fn string_to_token(&self, input: &str) -> Option<{{token_type}}> {
        match input {
            {{#each mappings}}
            {{#each entries}}
            {{#each all_inputs}}
            "{{this}}" => Some({{token_type}}::{{../token}}),
            {{/each}}
            {{/each}}
            {{/each}}
            _ => None,
        }
    }
    
    pub fn token_to_string(&self, token: &{{token_type}}) -> &'static str {
        match token {
            {{#each mappings}}
            {{#each entries}}
            {{token_type}}::{{token}} => "{{preferred}}",
            {{/each}}
            {{/each}}
        }
    }
}
```

### 3. Dual Extensibility System

**ProcessorSource** - Unified performance:
```rust
pub enum ProcessorSource {
    // Compile-time generated (built-in scripts)
    Static(&'static dyn TokenProcessor),
    
    // Runtime compiled
    RuntimeCompiled(Box<dyn TokenProcessor>),
    
    // Fallback only (development/testing)
    Dynamic(DynamicProcessor),
}

impl Shlesha {
    pub fn add_runtime_schema(&mut self, schema: Schema) -> Result<(), Error> {
        match self.runtime_compiler.compile_schema(&schema) {
            Ok(compiled) => {
                self.processors.insert(schema.name.clone(), ProcessorSource::RuntimeCompiled(compiled));
            }
            Err(_) => {
                // Graceful fallback
                let dynamic = DynamicProcessor::from_schema(schema)?;
                self.processors.insert(schema.name.clone(), ProcessorSource::Dynamic(dynamic));
            }
        }
        Ok(())
    }
}
```

### 4. Caching System

**Persistent Compilation Cache**:
```
~/.cache/shlesha/
├── compiled/
│   ├── abc123def.dylib      # Compiled processor
│   ├── abc123def.meta       # Schema metadata  
│   └── xyz789abc.dylib      # Another compiled schema
├── source/
│   ├── abc123def.rs         # Generated source (debugging)
│   └── xyz789abc.rs
└── index.json               # Cache index
```

**Cache Strategy**:
- **Cache Key**: Blake3 hash of schema content
- **Versioning**: Automatic invalidation on schema changes
- **Sharing**: Cache across processes/applications
- **Cleanup**: LRU eviction for disk space management

## Performance Characteristics

### 1. Zero-Allocation Token Processing

**Memory Profile**:
```
Operation           | Before (String)    | After (Token)
--------------------|-------------------|---------------
Parse "dharma"      | ~40 bytes heap   | 0 bytes heap
Token storage       | Vec<String>       | Vec<enum> (stack)
Pattern matching    | HashMap lookup    | Match statement
String output       | Concatenation     | Static &str refs
```

### 2. Performance Comparison

| Processor Type | First Load | Subsequent Use | Memory Overhead |
|----------------|------------|----------------|-----------------|
| **Static (Built-in)** | 0ms | Match statement | Minimal |
| **Runtime Compiled** | ~100ms compile | Match statement | Minimal |
| **Dynamic HashMap** | ~1ms load | Hash lookup | Higher |

### 3. Optimization Techniques

**Compile-Time Optimizations**:
- Pattern matching optimization by compiler
- Dead code elimination for unused tokens
- Inline expansion of hot paths
- Static string interning

**Runtime Optimizations**:
- Zero-copy token processing
- Stack-allocated token sequences for small inputs
- Batch token conversion
- SIMD-friendly data layouts (future)

## Error Handling & Robustness

### 1. Graceful Degradation

**Compilation Fallbacks**:
```rust
// Try runtime compilation first
if let Ok(compiled) = runtime_compiler.compile_schema(&schema) {
    return ProcessorSource::RuntimeCompiled(compiled);
}

// Fallback to HashMap-based processing
ProcessorSource::Dynamic(DynamicProcessor::from_schema(schema)?)
```

### 2. Unknown Token Handling

**Robust Token Processing**:
```rust
match self.string_to_token(input) {
    Some(token) => process_token(token),
    None => {
        // Unknown sequences pass through unchanged
        // Metadata tracks unknown positions for debugging
        unknown_handler.record_unknown(input, position);
        input.to_string()
    }
}
```

## Testing Architecture

### 1. Property-Based Round-Trip Testing

**Ambiguity Resolution Validation**:
```rust
#[quickcheck]
fn prop_round_trip_conversion(input: SanskritText) -> bool {
    let tokens = processor.string_to_tokens(&input.text);
    let preferred = processor.tokens_to_string(&tokens);
    let round_trip_tokens = processor.string_to_tokens(&preferred);
    
    // Round-trip should be stable through preferred form
    tokens == round_trip_tokens
}
```

### 2. Performance Regression Testing

**Zero-Allocation Validation**:
```rust
#[test]
fn test_zero_allocation_conversion() {
    let initial_allocs = get_allocation_count();
    
    let tokens = processor.string_to_tokens("test_input");
    let output = processor.tokens_to_string(&tokens);
    
    let final_allocs = get_allocation_count();
    assert_eq!(initial_allocs, final_allocs, "Token processing should not allocate");
}
```

### 3. Schema Validation Testing

**Runtime Compilation Testing**:
```rust
#[test]
fn test_runtime_schema_compilation() {
    let schema = load_test_schema("custom_script.yaml");
    let compiled = runtime_compiler.compile_schema(&schema)?;
    
    // Runtime compiled processor should behave identically to static
    assert_conversion_equivalence(&compiled, &expected_static_behavior);
}
```

## Extension Points

### 1. Adding Static Scripts

**Build-Time Integration**:
```yaml
# schemas/new_script.yaml
metadata:
  name: "new_script"
  script_type: "roman"
  
target: "alphabet_tokens"

mappings:
  vowels:
    VowelA: "a"
    VowelAa: ["aa", "ā"]  # Multiple inputs supported
```

**Automatic Code Generation**: Build system generates processor automatically

### 2. Runtime Script Addition

**Dynamic Loading**:
```rust
let mut shlesha = Shlesha::new();

// Load schema from file
shlesha.load_schema_file("custom_encoding.yaml")?;

// Or create schema programmatically
let schema = SchemaBuilder::new("my_script")
    .add_vowel_mapping("VowelA", &["a", "A"])
    .add_consonant_mapping("ConsonantK", "k")
    .build();

shlesha.add_runtime_schema(schema)?;
```

### 3. Performance Tuning

**Optimization Hooks**:
```rust
// Custom optimization strategies
impl TokenProcessor for MyCustomProcessor {
    fn optimize_for_frequency(&mut self, char_frequencies: &FrequencyMap) {
        // Reorder pattern matching based on usage
    }
    
    fn enable_simd(&mut self) -> bool {
        // SIMD-optimized token processing
    }
}
```

## Future Enhancements

### 1. Advanced Compilation Strategies

**LLVM Backend**: Direct LLVM IR generation for maximum performance
**GPU Compilation**: CUDA/OpenCL for massive parallel processing
**WebAssembly**: Browser-optimized token processing

### 2. Linguistic Features

**Context-Aware Tokens**: Tokens that change behavior based on surrounding context
**Probabilistic Disambiguation**: ML-based ambiguity resolution
**Phonetic Similarity**: Token-level edit distance for fuzzy matching

### 3. Ecosystem Integration

**Language Bindings**: Zero-copy FFI for Python, JavaScript, etc.
**IDE Support**: Language server for schema editing and validation
**Visual Tools**: GUI schema editor with real-time compilation

## Design Benefits

### 1. Performance

- Token processing without memory overhead
- Compile-time optimization
- No hash lookup variance
- Compact token representation

### 2. Correctness

- Explicit preferred forms in schemas
- Compile-time token validation
- Stable round-trip conversion
- Build-time schema validation

### 3. Developer Experience

- Consistent API across static and runtime processors
- Runtime schema loading capability
- Automatic recompilation on schema changes
- Source-level debugging of generated code

### 4. Scalability

- No performance penalty for additional scripts
- Independent processor generation
- Shareable compiled processors
- Only recompile changed schemas

This architecture provides token-based processing with runtime extensibility through compile-time and runtime code generation.