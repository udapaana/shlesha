# Performance Optimization Strategy: Relaxing Bidirectionality

## Current State Analysis

### Performance Metrics (Current System)
- **Round-trip time**: 975 μs for 43 characters (~23 μs/character)
- **Memory usage**: ~144 bytes per character (IR + metadata)
- **Success rate**: 96.62% (670/19,800 test failures)
- **Allocations**: 129 per text (3 per character)

### Key Bottlenecks
1. **IR Generation**: Every character becomes an Element with metadata
2. **Bidirectional Mappings**: Reverse lookup tables consume memory
3. **Round-trip Testing**: Adds 2x performance overhead
4. **Token Unwrapping**: Complex logic for script-aware tokens

## Optimization Strategies

### 1. **Asymmetric Design** 🚀
**Impact**: 3-5x performance improvement

```rust
// Fast path for common translations
pub enum TransliterationPath {
    Direct(DirectMapper),     // Pre-compiled state machine
    Fallback(IRTranslator),   // Current IR-based approach
}
```

**Benefits**:
- No IR generation for common paths
- Single allocation for output
- ~2 bytes per character vs 144 bytes

### 2. **Mode-Based Architecture** ⚡
```rust
pub enum Mode {
    Fast,        // ~5x faster, may lose edge cases
    Lossless,    // Current behavior with tokens
    Balanced,    // Smart fallback to tokens only when needed
}
```

### 3. **Streaming/Zero-Copy Processing** 💾
```rust
// Process text in chunks without full IR generation
pub fn transliterate_streaming<W: Write>(
    input: &str, 
    output: &mut W,
    mapper: &DirectMapper
) -> Result<()>
```

## Implementation Roadmap

### Phase 1: Dual Mode Support (2 weeks)
- Add `FastTransliterator` alongside current `Transliterator`
- Implement direct mapping for top 5 script pairs:
  - Devanagari ↔ IAST
  - Devanagari ↔ SLP1  
  - Devanagari ↔ Harvard-Kyoto
  - IAST ↔ Harvard-Kyoto
  - IAST ↔ SLP1

### Phase 2: Smart Mode Selection (1 week)
- Auto-detect common vs rare script pairs
- Fallback to IR mode for unsupported paths
- Performance monitoring and metrics

### Phase 3: Advanced Optimizations (2 weeks)
- State machine compilation from schemas
- SIMD/vectorized processing for ASCII-heavy romanizations
- Memory pool for IR allocations

## Practical Trade-offs

### Use Case Prioritization

#### High-Performance Use Cases (Use Fast Mode)
- **Real-time input methods**: Sub-millisecond response needed
- **Bulk text processing**: Processing thousands of documents  
- **Web services**: Low latency API responses
- **Search indexing**: Speed over perfect fidelity

#### High-Fidelity Use Cases (Use Lossless Mode)
- **Academic publishing**: Perfect round-trips required
- **Digital preservation**: No data loss acceptable
- **Linguistic analysis**: Edge cases matter
- **Reference implementations**: Standards compliance

### Performance vs Fidelity Matrix

| Mode | Speed | Memory | Round-trip | Use Case |
|------|-------|--------|------------|----------|
| Fast | 5x | 72x less | ~85% | Web apps, IME |
| Balanced | 2x | 10x less | ~95% | General purpose |
| Lossless | 1x | Baseline | 96.62% | Academic, archival |

## Implementation Examples

### Fast Mode API
```rust
let fast = FastTransliterator::new()
    .add_path("Devanagari", "IAST")
    .add_path("IAST", "Devanagari");

// ~200μs instead of 975μs
let result = fast.transliterate(text, "Devanagari", "IAST")?;
```

### Balanced Mode API  
```rust
let balanced = Transliterator::builder()
    .mode(Mode::Balanced)
    .fallback_to_tokens(true)
    .build();

// ~400μs with 95% round-trip success
let result = balanced.transliterate(text, "Devanagari", "IAST")?;
```

### Streaming API
```rust
let mut output = Vec::new();
fast.transliterate_streaming(input_reader, &mut output, "Devanagari", "IAST")?;
```

## Memory Optimization Details

### Current Memory Usage
```
Per Character: 144 bytes
├── Element struct: 72 bytes
├── Properties HashMap: 48 bytes  
├── String allocations: 24 bytes
└── IR metadata: Variable
```

### Optimized Memory Usage
```
Per Character: 2 bytes
├── Output buffer: 1-4 bytes
└── Stack variables: ~0 bytes
```

**Result**: 72x memory reduction for common paths

## Benchmarking Plan

### Target Metrics
- **Latency**: Sub-100μs for common translations
- **Throughput**: 1M+ characters/second
- **Memory**: <10MB peak usage
- **Accuracy**: 99%+ for balanced mode

### Test Scenarios
1. **Micro**: Single words (धर्म → dharma)
2. **Small**: Sentences (43 characters)
3. **Medium**: Paragraphs (500 characters)  
4. **Large**: Documents (10K+ characters)
5. **Bulk**: Multiple documents concurrently

## Migration Strategy

### Backward Compatibility
- Keep current API as `Transliterator::legacy()`
- New default: `Transliterator::balanced()`
- Opt-in fast mode: `Transliterator::fast()`

### Schema Compatibility
- Existing schemas work unchanged
- Optional fast-path annotations
- Auto-generation of direct mappings

## Conclusion

**Relaxing bidirectionality constraint enables**:
- 🚀 **5x performance improvement** for common paths
- 💾 **72x memory reduction** 
- 🎯 **Use-case optimized modes**
- 🔄 **Graceful degradation** to lossless mode
- ↩️ **Full backward compatibility**

**Key insight**: Most real-world usage follows 80/20 rule - optimize the 20% of paths that handle 80% of traffic, while maintaining lossless fallback for the long tail.