# Shlesha vs Vidyut: Fair Performance Comparison

## Executive Summary

Direct benchmark comparison shows **Vidyut is 16-30x faster than Shlesha** for transliteration tasks:
- Single word: Vidyut 470ns vs Shlesha 7.9µs (16.9x faster)
- Short sentence: Vidyut 1.3µs vs Shlesha 22.6µs (17.4x faster)
- Medium text: Vidyut 4.8µs vs Shlesha 97.0µs (20.2x faster)
- Large text: Vidyut 13.0µs vs Shlesha 240µs (18.5x faster)
- Throughput: Vidyut 45 MiB/s vs Shlesha 1.5 MiB/s (30x faster)

## Detailed Benchmark Results

### Performance by Text Size

| Text Size | Shlesha | Vidyut | Vidyut Speedup |
|-----------|---------|---------|----------------|
| Single word (7 chars) | 7.95 µs | 0.47 µs | **16.9x** |
| Short sentence (30 chars) | 22.59 µs | 1.26 µs | **17.9x** |
| Medium text (150 chars) | 97.01 µs | 4.83 µs | **20.1x** |
| Long text (450 chars) | 239.54 µs | 12.99 µs | **18.4x** |

### Throughput Comparison

| Corpus Size | Shlesha | Vidyut | Vidyut Advantage |
|-------------|---------|---------|------------------|
| 100KB | 1.47 MiB/s | 44.65 MiB/s | **30.4x** |
| 1MB | 1.53 MiB/s | 45.49 MiB/s | **29.7x** |

## Why is Vidyut Faster?

Based on profiling analysis and architecture comparison:

### 1. **Algorithmic Complexity**
- **Vidyut**: O(n) - Linear scan with finite state automaton
- **Shlesha**: O(n × m × c) - Nested loops for multiple pattern lengths and categories
  - n = text length
  - m = pattern lengths to check (1-4)
  - c = element categories

### 2. **String Allocation Overhead**
- **Shlesha**: Creates ~408 temporary strings per 17-character word
  - Only 4.2% allocation efficiency (17 matches / 408 attempts)
  - Each failed lookup allocates and discards a string
- **Vidyut**: Likely uses pre-compiled lookup tables with minimal allocations

### 3. **Architecture Differences**

#### Vidyut Architecture:
```rust
// Simple, direct mapping
let mapping = Mapping::new(Scheme::Devanagari, Scheme::Iast);
transliterate(text, &mapping)
```
- Pre-compiled mappings
- Direct character-to-character transformation
- Optimized for specific script pairs
- No intermediate representation

#### Shlesha Architecture:
```rust
// Complex, extensible pipeline
transliterator.transliterate(text, "Devanagari", "IAST")
// Internally: Parse → IR → Transform → Generate
```
- Dynamic schema loading
- Intermediate representation (IR) for bidirectionality
- Property preservation and semantic understanding
- Runtime type checking and HashMap lookups

### 4. **Memory Layout**
- **Vidyut**: Compact lookup tables, cache-friendly access patterns
- **Shlesha**: Multiple indirections through HashMap → HashMap → ElementMapping

## Trade-offs Analysis

### Vidyut Strengths:
- ⚡ **Speed**: 16-30x faster for transliteration
- 🎯 **Focused**: Optimized for Sanskrit computational linguistics
- 📦 **Lightweight**: Minimal overhead, direct transformations
- 🔧 **Compiled**: Pre-compiled mappings for known script pairs

### Shlesha Strengths:
- 🌍 **Universal**: "For any script Sanskrit has been transliterated to"
- ↔️ **Bidirectional**: Same system works both directions
- 🔌 **Extensible**: Runtime schema loading and extensions
- 🎭 **Semantic**: Preserves linguistic properties and unknown sounds
- 🏗️ **Flexible**: YAML-based schemas, easy to add new scripts

## When to Use Which?

### Use Vidyut When:
- Maximum performance is critical
- Working with well-known script pairs (Devanagari ↔ IAST/SLP1)
- Building Sanskrit-specific applications
- Throughput matters (30x difference!)

### Use Shlesha When:
- Need support for rare or custom transliteration schemes
- Require bidirectional transliteration with same codebase
- Want to preserve semantic information and properties
- Need runtime extensibility and custom schemas
- Working with multiple Indic languages beyond Sanskrit

## Performance Improvement Opportunities for Shlesha

Based on the comparison, potential optimizations:

1. **String Allocation Reduction** (High Impact)
   - Pre-allocate pattern strings
   - Use string interning for common patterns
   - Implement zero-copy parsing where possible

2. **Algorithm Optimization** (High Impact)
   - Implement finite state automaton for known patterns
   - Use trie or similar data structure for pattern matching
   - Compile schemas to more efficient runtime representation

3. **Fast Path for Common Cases** (Medium Impact)
   - Direct lookup table for common script pairs
   - Skip IR transformation for simple cases
   - Cache compiled transformations

4. **Memory Layout Optimization** (Medium Impact)
   - Replace nested HashMaps with flat arrays where possible
   - Use compact representations for mappings
   - Improve cache locality

## Conclusion

The performance difference reflects fundamental architectural choices:

- **Vidyut**: Chose speed through specialization
- **Shlesha**: Chose flexibility through generalization

This is a classic software engineering trade-off. Vidyut's 16-30x performance advantage comes from:
1. Simpler algorithm (O(n) vs O(n×m×c))
2. No string allocations during processing
3. Pre-compiled mappings
4. Direct transformations without intermediate representation

Shlesha's slower performance is the cost of:
1. Universal script support
2. Bidirectional capability
3. Runtime extensibility
4. Semantic preservation

For most Sanskrit transliteration tasks where performance matters, **Vidyut is the clear winner**. For complex, multi-script, or extensible transliteration needs, Shlesha's flexibility may justify the performance cost.