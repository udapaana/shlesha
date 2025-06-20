# Asymmetric Transliteration Design

## Current Bidirectional Constraint Issues

1. **Performance overhead**: Every mapping must be reversible, requiring complex lookups
2. **Schema complexity**: Need to maintain perfect 1:1 mappings
3. **Token proliferation**: Many tokens created for edge cases that rarely occur in practice

## Proposed Asymmetric Design

### Core Principles

1. **No data loss**: All information is preserved (via tokens if needed)
2. **Optimized common paths**: Fast, direct mappings for common use cases
3. **Graceful degradation**: Less common paths may use tokens but remain functional

### Implementation Strategy

#### 1. Direction-Aware Schemas

```yaml
# devanagari_to_iast.yaml
name: "Devanagari_to_IAST"
direction: "forward"  # Optimized direction
reverse_direction: "fallback"  # Uses tokens when needed

mappings:
  # Direct, fast mappings
  "क": "ka"
  "ख": "kha"
  # Compounds get special handling
  "क्ष": "kṣa"  # Preserved as unit
  "ज्ञ": "jña"  # Preserved as unit
```

#### 2. Lazy Reverse Mapping Generation

```rust
pub struct AsymmetricTransliterator {
    forward_mappings: HashMap<(String, String), ForwardMapper>,
    reverse_mappings: Option<HashMap<(String, String), ReverseMapper>>,
}

impl AsymmetricTransliterator {
    pub fn transliterate(&self, text: &str, from: &str, to: &str) -> Result<String> {
        if let Some(forward) = self.forward_mappings.get(&(from.to_string(), to.to_string())) {
            // Fast path - direct mapping
            forward.transliterate(text)
        } else {
            // Slow path - may need to build reverse mapping or use tokens
            self.transliterate_with_fallback(text, from, to)
        }
    }
}
```

#### 3. Smart Token Strategy

Instead of always creating tokens, use them only when:
- No direct mapping exists
- User explicitly requests lossless mode
- Detecting ambiguous cases

```rust
pub enum TransliterationMode {
    Fast,        // May lose some information, but fast
    Lossless,    // Always preserves information via tokens
    Smart,       // Uses heuristics to decide
}
```

### Performance Optimizations

#### 1. Unidirectional Parsing

```rust
// Current: Parse → Transform → Generate
// Optimized: Parse+Transform → Generate (fused operations)

pub struct DirectTransliterator {
    // Pre-compiled state machines for common paths
    machines: HashMap<(String, String), StateMachine>,
}
```

#### 2. Zero-Copy Operations

```rust
// Avoid intermediate IR for common cases
pub fn transliterate_direct(input: &str, machine: &StateMachine) -> String {
    let mut output = String::with_capacity(input.len() * 2);
    let mut chars = input.chars();
    
    while let Some(ch) = chars.next() {
        match machine.process(ch, &mut chars) {
            Action::Emit(s) => output.push_str(s),
            Action::Token(t) => output.push_str(&format!("[{}]", t)),
        }
    }
    
    output
}
```

#### 3. Streaming Processing

```rust
pub struct StreamingTransliterator {
    buffer: Vec<u8>,
    state: State,
}

impl Write for StreamingTransliterator {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Process chunks without loading entire text
        self.process_chunk(buf)
    }
}
```

### Use Case Optimization

#### Common Paths (Optimized)
- Devanagari → IAST (academic papers)
- Devanagari → Harvard-Kyoto (digital texts)
- IAST → Devanagari (input methods)

#### Rare Paths (Use tokens)
- Velthuis → Tamil
- SLP1 → Gujarati
- WX → Malayalam

### Benchmark Targets

| Operation | Current (Bidirectional) | Optimized (Asymmetric) | Improvement |
|-----------|------------------------|------------------------|-------------|
| Deva→IAST | 3.5 μs/word | 0.8 μs/word | 4.4x |
| Round-trip | 7.0 μs/word | N/A | - |
| Memory usage | 15 MB | 5 MB | 3x |
| Startup time | 45 ms | 10 ms | 4.5x |

### Migration Path

1. **Phase 1**: Add asymmetric mode alongside current bidirectional mode
2. **Phase 2**: Identify and optimize common paths
3. **Phase 3**: Make asymmetric mode default, bidirectional optional
4. **Phase 4**: Provide tools to generate reverse mappings on demand

### API Design

```rust
// Simple API for common use
let output = transliterate(text, "Devanagari", "IAST");

// Advanced API with options
let output = Transliterator::builder()
    .mode(TransliterationMode::Fast)
    .fallback_tokens(false)
    .build()
    .transliterate(text, "Devanagari", "IAST");

// Bidirectional when needed
let output = Transliterator::builder()
    .mode(TransliterationMode::Lossless)
    .bidirectional(true)
    .build()
    .transliterate(text, "Devanagari", "IAST");
```