# Shlesha Architecture Documentation

## Overview

Shlesha implements a **hub-and-spoke architecture** for transliteration between Sanskrit and Indic scripts. This design provides optimal performance and maintainability by centralizing complex linguistic processing while enabling simple character-to-character mapping for individual script conversions.

## Core Architectural Principles

### 1. Hub-and-Spoke Design

```
Indic Scripts          Hub Scripts           Roman Scripts
     |                      |                      |
 Gujarati  ←→         Devanagari  ←→         ISO-15919  ←→  IAST
 Bengali   ←→              |                      |         ITRANS  
 Tamil     ←→         [Hub Processing]      [Hub Processing] SLP1
 Telugu    ←→              |                      |         HK
     |                     ↓                      |         Velthuis
     └─────────── Direct Character Mapping ──────┘         WX
```

### 2. Central Hub Processing

**Hub Scripts**: Devanagari ↔ ISO-15919
- All complex linguistic rules are handled here
- Virama processing, implicit vowel handling
- Consonant cluster normalization
- Script-specific phonetic rules

### 3. Script Classification

**Indic Scripts** (with implicit 'a'):
- Devanagari, Gujarati, Bengali, Tamil, Telugu
- Consonants inherently contain 'a' vowel
- Require virama (्) to suppress vowel

**Romanization Schemes** (without implicit 'a'):
- ISO-15919, IAST, ITRANS, SLP1, Harvard-Kyoto, Velthuis, WX
- Consonants do not contain implicit vowels
- Explicit vowel representation

## System Components

### 1. Core Library (`src/lib.rs`)

**Smart Routing Logic**:
```rust
match (&hub_input, to.to_lowercase().as_str()) {
    // Direct passthrough cases - no hub processing needed
    (HubInput::Devanagari(deva), "devanagari" | "deva") => deva.clone(),
    (HubInput::Iso(iso), "iso" | "iso15919" | "iso-15919") => iso.clone(),
    
    // Hub processing needed - convert between formats
    (HubInput::Devanagari(deva), _) => { /* route via hub */ },
    (HubInput::Iso(iso), _) => { /* route via hub */ },
}
```

### 2. Hub Module (`src/modules/hub/`)

**Central Processing Engine**:
- `iso_to_deva()`: ISO-15919 → Devanagari conversion
- `deva_to_iso()`: Devanagari → ISO-15919 conversion
- Handles all complex linguistic transformations
- Manages consonant clusters and virama placement

### 3. Script Converter Registry (`src/modules/script_converter/`)

**Modular Converter System**:
```rust
pub trait ScriptConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError>;
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError>;
    fn supported_scripts(&self) -> Vec<&'static str>;
    fn script_has_implicit_a(&self, script: &str) -> bool;
}
```

### 4. Schema-Based Converter (`src/modules/script_converter/schema_based.rs`)

**Runtime Extensibility System**:
```rust
pub struct SchemaBasedConverter {
    registry: Arc<SchemaRegistry>,
}

impl ScriptConverter for SchemaBasedConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError>;
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError>;
    fn supports_script(&self, script: &str) -> bool;
}
```

**Dynamic Script Support**:
- Loads custom encoding schemes from YAML files at runtime
- Character-level mapping with multi-character support
- Automatic hub routing based on script type
- Bidirectional conversion support

### 5. Unknown Token Handler (`src/modules/core/unknown_handler.rs`)

**Lightweight Metadata System**:
```rust
pub struct TransliterationResult {
    pub output: String,                           // Clean output
    pub metadata: Option<TransliterationMetadata>, // Optional metadata
}

pub struct UnknownToken {
    pub script: String,
    pub token: char,
    pub unicode: String,
    pub position: usize,
    pub is_extension: bool,
}
```

**Zero-overhead design**:
- Unknown characters pass through by default
- Metadata collection only when requested
- Position tracking for unknown locations
- Extension awareness for runtime scripts

## Conversion Flow Patterns

### 1. Indic Script Conversions

**Indic → Hub**:
```rust
// Simple character-to-character mapping
let hub_input = converter.to_hub("gujarati", "ધર્મ")?;
// Returns: HubInput::Devanagari("धर्म")
```

**Hub → Indic**:
```rust
let result = converter.from_hub("gujarati", &HubInput::Devanagari("धर्म"))?;
// Returns: "ધર્મ"
```

### 2. Roman Script Conversions

**Roman → Hub**:
```rust
let hub_input = converter.to_hub("iast", "dharma")?;
// Returns: HubInput::Iso("dharma")
```

**Hub → Roman**:
```rust
let result = converter.from_hub("itrans", &HubInput::Iso("dharma"))?;
// Returns: "dharma"
```

### 3. Cross-Script Conversions

**Indic → Roman** (via hub):
```
Gujarati "ધર્મ" → Devanagari "धर्म" → [Hub] → ISO "dharma" → IAST "dharma"
```

**Roman → Indic** (via hub):
```
ITRANS "dharma" → ISO "dharma" → [Hub] → Devanagari "धर्म" → Bengali "ধর্ম"
```

### 4. Custom Schema Conversions

**Custom Romanized → Indic** (via hub):
```
my_encoding "kam" → ISO "kam" → [Hub] → Devanagari "कअम" → Tamil "கअம்"
```

**Custom Indic → Roman** (via hub):
```
my_indic "क" → Devanagari "क" → [Hub] → ISO "ka" → IAST "ka"
```

**Runtime Loading**:
```rust
let mut shlesha = Shlesha::new();
shlesha.load_schema("custom_encoding.yaml")?;
let result = shlesha.transliterate("input", "my_encoding", "devanagari")?;
```

## Performance Optimizations

### 1. Zero-Copy Passthroughs

Hub scripts bypass conversion when source equals target:
```rust
// Devanagari → Devanagari: direct passthrough
(HubInput::Devanagari(deva), "devanagari") => deva.clone(),
```

### 2. Character-to-Character Mapping

Indic script conversions use simple HashMap lookups:
```rust
// O(1) character conversion
gujarati_to_deva_map.insert('ધ', 'ध');
```

### 3. Minimal String Allocation

- Reuse strings where possible
- Efficient Unicode handling
- Lazy evaluation of conversions

## Error Handling Strategy

### 1. Graceful Degradation

Unknown characters are preserved rather than causing failures:
```rust
// Preserve unknown characters as-is
if !matched {
    result.push(ch);
}
```

### 2. Clear Error Messages

Descriptive errors for debugging:
```rust
ConverterError::ConversionFailed {
    script: script.to_string(),
    reason: format!("Character '{}' not found in mapping", ch),
}
```

### 3. Early Validation

Script support validation before processing:
```rust
if !registry.supports_script(script) {
    return Err(ConverterError::UnsupportedScript { script });
}
```

## Testing Architecture

### 1. Unit Tests

Each converter has comprehensive unit tests:
- Character mapping validation
- Edge case handling
- Error condition testing

### 2. Integration Tests

Full pipeline testing:
- Script → Hub → Target conversions
- Roundtrip validation
- Cross-script compatibility

### 3. Property-Based Tests

Invariant validation:
- Roundtrip identity preservation
- Character set consistency
- Performance characteristics

## Extension Points

### 1. Adding New Scripts

Implement `ScriptConverter` trait:
```rust
impl ScriptConverter for NewScriptConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        // Convert to appropriate hub format
    }
    
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        // Convert from hub format
    }
}
```

### 2. Hub Format Extensions

Add new hub processing in hub module:
```rust
impl Hub {
    pub fn new_format_conversion(&self, input: &str) -> Result<HubOutput, HubError> {
        // New linguistic processing
    }
}
```

## Task Management Philosophy

### Centralized TODO Management

Shlesha maintains a centralized `TODO.md` file in the project root for all development tasks and module-level todos. This approach provides:

1. **Single Source of Truth**: All tasks in one location for easy tracking
2. **Better Visibility**: Team members can quickly see all pending work
3. **Organized Structure**: Tasks grouped by module and priority
4. **Progress Tracking**: Clear status indicators for each task

The `TODO.md` file replaces scattered TODO comments across module files, ensuring that development priorities and pending work are always visible and well-organized.

## Design Benefits

### 1. Scalability

Adding new scripts requires only:
- Character mapping (for Indic scripts)
- Transliteration rules (for Roman scripts)
- No changes to existing converters

### 2. Maintainability

- Complex linguistic rules centralized in hub
- Simple, testable character mappings
- Clear separation of concerns

### 3. Performance

- O(n) conversion complexity
- Minimal memory allocation
- Optimized common paths

### 4. Correctness

- Single source of truth for linguistic rules
- Comprehensive test coverage
- Bidirectional validation

## Future Enhancements

### 1. Additional Scripts

- Kannada, Malayalam, Odia
- Additional romanization schemes
- Historical script variants

### 2. Advanced Features

- Phonetic similarity matching
- Context-aware conversion rules
- Statistical transliteration models

### 3. Performance Strategy & Competitive Position

#### Performance Philosophy

Shlesha prioritizes **extensibility, maintainability, and correctness** over raw performance. While specialized libraries like Vidyut achieve 18.9x faster performance on average, Shlesha's hub-and-spoke architecture provides unique benefits:

- **Runtime Extensibility**: Add new scripts without recompilation via schema loading
- **Architectural Consistency**: Centralized linguistic rules ensure uniform behavior
- **Maintainability**: Clean separation of concerns enables rapid development
- **Future-proofing**: Easy to adapt to new transliteration requirements

#### Current Performance Analysis

**Competitive Positioning vs Vidyut:**
- **Roman → Devanagari**: 75x performance gap (biggest optimization target)
- **Roman ↔ Roman**: 44x performance gap
- **Devanagari → Roman**: 27x performance gap  
- **Indic ↔ Indic**: 3.3x performance gap (most competitive area)

**Root Causes of Performance Gap:**
1. **Hub Architecture Overhead**: Multi-step routing (Roman → ISO → Devanagari → Target)
2. **Generality Tax**: Generic framework vs specialized implementation
3. **Safety/Validation Layers**: Additional error handling and validation
4. **Memory Allocation Patterns**: String operations and character processing

#### Pre-computation System Decision

**Removed after analysis showing:**
- **Minimal gains**: Only 1.5-6.5% performance improvement
- **High complexity**: 1,500+ lines of build system and generated code
- **Maintenance burden**: Every hub change requires regenerating mappings
- **Better alternatives**: Simple optimizations can achieve similar benefits

The pre-computation system was technically impressive but over-engineered for its modest benefits.

#### Optimization Roadmap

**Phase 1: Memory & Allocation Optimizations** (Completed)
- ✅ Iterator-based character processing (eliminated Vec allocations)
- ✅ String capacity pre-calculation (reduced reallocation overhead)
- ✅ HashMap converter lookup cache (O(1) vs O(n) script resolution)

**Phase 2: Hot Path Optimizations** (High Impact)
- Perfect hash tables for small, fixed mappings
- SIMD-based string processing for ASCII-heavy text
- Stack allocation for small string operations
- Optimized longest-match algorithms for multi-character sequences

**Phase 3: Algorithmic Improvements** (Medium Impact)
- Trie-based sequence matching for complex patterns
- Batch character processing to reduce function call overhead
- Memory layout optimization for cache efficiency
- Specialized fast paths for common conversion patterns

**Performance Goals:**
- **Short-term**: Close gap from 18.9x to ~10x through simple optimizations
- **Medium-term**: Achieve competitive performance on Indic ↔ Indic conversions
- **Long-term**: Maintain architectural advantages while minimizing performance tax

#### Future Performance Enhancements

- **SIMD optimizations** for character processing
- **Parallel processing** for large texts
- **Memory-mapped file processing** for bulk operations
- **GPU acceleration** for batch processing scenarios