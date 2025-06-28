# Shlesha Optimization Roadmap

## Current Competitive Position

- **Target**: Outperform Aksharamukha and Dharmamitra while maintaining extensibility benefits
- **Benchmark vs Vidyut**: Currently 18.9x slower on average (acceptable given extensibility)
- **Pre-computation removal**: Eliminated 1,500+ lines of complexity for only 1.5-6.5% gains

## Optimization Strategy Framework

For each optimization attempt:
1. **Commit baseline** with descriptive message
2. **Implement single optimization** strategy  
3. **Benchmark and measure** performance impact
4. **If unsuccessful** (< 20% improvement): revert to baseline commit
5. **If successful**: commit optimization and document strategy
6. **Document learnings** in this file regardless of outcome

## Phase 1: Memory & Allocation Optimizations (COMPLETED)

### ✅ Iterator-based Character Processing
- **Status**: Completed
- **Benefit**: Eliminated Vec allocations in character iteration
- **Impact**: Reduced memory overhead for character-by-character processing

### ✅ String Capacity Pre-calculation  
- **Status**: Completed
- **Benefit**: Reduced string reallocation overhead
- **Impact**: Pre-calculate result string size based on input characteristics

### ✅ HashMap Converter Lookup Cache
- **Status**: Completed  
- **Benefit**: O(1) script resolution instead of O(n) linear search
- **Impact**: Faster converter selection for script routing

## Phase 2: Hot Path Optimizations (HIGH IMPACT - NEXT)

### 1. ❌ Perfect Hash Tables for Fixed Mappings (ATTEMPTED - REVERTED)
**Status**: Attempted and reverted - complexity not justified by performance gains
**Target**: Character mapping lookups in script converters
**Strategy**: Replace HashMap with perfect hash tables for small, fixed character sets
**Expected Impact**: 30-50% faster character lookups ❌ **ACTUAL: 16.7% slower**
**Key Learnings**:
- Modern HashMap<char, char> is already highly optimized for 77-character mappings
- PHF overhead (complex hash function) outweighs benefits for small maps
- Added wrapper enums introduce indirection overhead
- Character lookups have excellent hash distribution making HashMap near-optimal
- FxHashMap provides minimal improvement due to wrapper overhead
- **Conclusion**: Keep simple HashMap<char, char> - premature optimization
**Implementation**:
```rust
// REVERTED: Complex generic optimization framework
// KEEPING: Simple HashMap<char, char> - already near-optimal
let mut char_map = HashMap::new();
char_map.insert('అ', 'अ'); // Simple and fast
```

### 2. String Allocation Reduction (HIGH PRIORITY - NEXT)
**Target**: Reduce string allocations in hot conversion paths
**Strategy**: Pre-calculate result string capacity, use string builders, reuse buffers
**Expected Impact**: 25-40% faster through reduced allocation overhead
**Implementation**:
```rust
// Pre-calculate result size to avoid reallocations
let mut result = String::with_capacity(input.len() * 2); // Conservative estimate
// Or use stack-allocated strings for small results
```

### 3. SIMD String Processing for ASCII
**Target**: Roman script processing (IAST, ITRANS, ISO-15919)
**Strategy**: Use SIMD instructions for ASCII character validation/conversion
**Expected Impact**: 40-60% faster for ASCII-heavy Roman texts
**Implementation**:
```rust
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

fn process_ascii_simd(input: &str) -> String {
    // Process 16 ASCII chars at once using SIMD
}
```

### 3. Stack-allocated Small Strings
**Target**: Short text conversions (< 64 characters)
**Strategy**: Use stack-allocated arrays for common small string operations
**Expected Impact**: 25-35% faster for short texts
**Implementation**:
```rust
use heapless::String as StackString;

fn convert_small_text<const N: usize>(input: &str) -> StackString<N> {
    // Avoid heap allocation for small results
}
```

### 4. Optimized Multi-character Sequence Matching
**Target**: Longest-match algorithms in roman script processing
**Strategy**: Replace linear search with optimized trie or automaton
**Expected Impact**: 20-40% faster for complex roman schemes
**Implementation**:
```rust
struct SequenceTrie {
    // Optimized trie for multi-character sequence matching
}
```

## Phase 3: Algorithmic Improvements (MEDIUM IMPACT)

### 5. Hub Processing Batching
**Target**: Character-by-character hub overhead
**Strategy**: Process characters in batches to reduce function call overhead
**Expected Impact**: 15-25% faster hub processing
**Implementation**:
```rust
fn batch_process_hub(chars: &[char]) -> Vec<char> {
    // Process multiple characters per function call
}
```

### 6. Memory Layout Optimization
**Target**: Cache efficiency in hot data structures
**Strategy**: Reorganize data structures for better cache locality
**Expected Impact**: 10-20% improvement through better cache usage
**Implementation**:
```rust
#[repr(C)]
struct OptimizedConverter {
    // Fields ordered by access frequency and size
}
```

### 7. Specialized Fast Paths
**Target**: Common conversion patterns (e.g., Roman → Devanagari)
**Strategy**: Detect common patterns and use specialized converters
**Expected Impact**: 30-50% faster for detected patterns
**Implementation**:
```rust
fn detect_fast_path(from: &str, to: &str, text: &str) -> Option<String> {
    // Specialized converters for common patterns
}
```

## Phase 4: Advanced Optimizations (EXPERIMENTAL)

### 8. Compile-time Script Specialization
**Target**: Generate optimized code paths at compile time
**Strategy**: Macro-generated specialized functions for each script pair
**Expected Impact**: 20-40% improvement for supported pairs
**Risk**: High complexity, maintenance burden

### 9. Parallel Character Processing
**Target**: Large text processing
**Strategy**: Split text into chunks and process in parallel
**Expected Impact**: 2-4x faster for large texts (>1KB)
**Risk**: Overhead may hurt small text performance

### 10. GPU Acceleration via Compute Shaders
**Target**: Batch processing scenarios
**Strategy**: Offload character mapping to GPU for massive parallelism
**Expected Impact**: 10-100x faster for batch operations
**Risk**: High implementation complexity, limited applicability

## Phase 5: Additional High-Impact Optimizations

### 11. Unsafe Zero-Copy String Operations
**Target**: String manipulation overhead in hot paths
**Strategy**: Use `unsafe` code for zero-copy string slicing and character access
**Expected Impact**: 30-50% faster string operations
**Implementation**:
```rust
unsafe fn fast_char_at(s: &str, idx: usize) -> char {
    // Direct byte access without bounds checking
    s.as_bytes().get_unchecked(idx)
}
```

### 12. Branch Prediction Optimization
**Target**: Conditional logic in character mapping
**Strategy**: Reorder conditions by frequency, use `likely`/`unlikely` hints
**Expected Impact**: 15-25% improvement through better CPU prediction
**Implementation**:
```rust
#[inline(always)]
fn convert_char_optimized(ch: char) -> char {
    if likely(ch.is_ascii()) {
        // Most common path first
        fast_ascii_convert(ch)
    } else {
        slow_unicode_convert(ch)
    }
}
```

### 13. Lookup Table Compression
**Target**: Memory usage and cache efficiency
**Strategy**: Compress sparse lookup tables using range-based encoding
**Expected Impact**: 20-40% better cache performance
**Implementation**:
```rust
struct CompressedLookup {
    ranges: &'static [(char, char, u16)], // start, end, offset
    values: &'static [char],
}
```

### 14. Inline Assembly for Critical Loops
**Target**: Character iteration and conversion loops
**Strategy**: Hand-optimized assembly for innermost loops
**Expected Impact**: 25-60% faster critical sections
**Risk**: Platform-specific, maintenance complexity

### 15. Custom String Interning
**Target**: Repeated string allocations for common words
**Strategy**: Intern frequently used strings and return references
**Expected Impact**: 40-70% reduction in allocations for repeated content
**Implementation**:
```rust
static STRING_INTERNER: once_cell::sync::Lazy<StringInterner> = 
    once_cell::sync::Lazy::new(StringInterner::new);

fn intern_result(s: String) -> &'static str {
    STRING_INTERNER.get_or_intern(s)
}
```

### 16. Finite State Automaton for Complex Rules
**Target**: Hub processing complex linguistic rules
**Strategy**: Compile linguistic rules into optimized state machines
**Expected Impact**: 30-50% faster rule processing
**Implementation**:
```rust
struct CompiledRules {
    states: &'static [State],
    transitions: &'static [Transition],
}
```

### 17. Profile-Guided Optimization (PGO)
**Target**: Overall binary optimization
**Strategy**: Use Rust's PGO to optimize based on actual usage patterns
**Expected Impact**: 10-30% improvement across all operations
**Implementation**:
```toml
[profile.release]
lto = "fat"
codegen-units = 1
panic = "abort"
```

### 18. Memory Pool Allocation
**Target**: Reduce allocation overhead
**Strategy**: Pre-allocate memory pools for common string sizes
**Expected Impact**: 20-40% reduction in allocation overhead
**Implementation**:
```rust
struct StringPool {
    small: Vec<String>,  // < 64 chars
    medium: Vec<String>, // 64-256 chars
    large: Vec<String>,  // > 256 chars
}
```

### 19. Hot/Cold Code Splitting
**Target**: Instruction cache efficiency
**Strategy**: Separate hot paths from error handling and rare cases
**Expected Impact**: 15-25% better instruction cache utilization
**Implementation**:
```rust
#[cold]
#[inline(never)]
fn handle_rare_case() { /* ... */ }

#[hot]
#[inline(always)]
fn handle_common_case() { /* ... */ }
```

### 20. Vectorized Character Classification
**Target**: Character type detection (vowel, consonant, etc.)
**Strategy**: Use bit manipulation and lookup tables for character classification
**Expected Impact**: 40-80% faster character type detection
**Implementation**:
```rust
const CHAR_CLASS_TABLE: [u8; 256] = [...]; // Bit flags for character types

#[inline(always)]
fn is_vowel(ch: char) -> bool {
    (CHAR_CLASS_TABLE[ch as u8 as usize] & VOWEL_MASK) != 0
}
```

## Phase 6: Architectural Optimizations

### 21. Copy-on-Write Hub Caching
**Target**: Repeated hub conversions
**Strategy**: Cache hub conversion results with copy-on-write semantics
**Expected Impact**: 50-90% faster for repeated conversions
**Risk**: Memory usage increase

### 22. Lazy Schema Loading
**Target**: Startup time and memory usage
**Strategy**: Load schemas only when first accessed
**Expected Impact**: 60-80% faster startup, lower memory footprint
**Risk**: First-use latency penalty

### 23. Binary Schema Format
**Target**: Schema loading performance
**Strategy**: Pre-compile YAML schemas to optimized binary format
**Expected Impact**: 5-10x faster schema loading
**Implementation**: Custom binary serialization format

### 24. Template Specialization for Common Pairs
**Target**: Most frequent script conversions
**Strategy**: Generate specialized code for top 10 script pairs
**Expected Impact**: 40-70% faster for common conversions
**Risk**: Binary size increase

## Implementation Priority Queue

### Immediate Next Steps (Highest ROI)
1. **Perfect Hash Tables** - Low risk, high impact for character lookups
2. **Unsafe Zero-Copy Operations** - Medium risk, very high impact for string ops
3. **SIMD ASCII Processing** - Medium risk, very high impact for Roman scripts
4. **Vectorized Character Classification** - Low risk, very high impact for type detection
5. **Stack String Allocation** - Low risk, good impact for small texts

### Secondary Targets (Good ROI)
6. **Branch Prediction Optimization** - Low risk, good impact for conditionals
7. **Multi-char Sequence Optimization** - Medium risk, medium-high impact
8. **Hub Processing Batching** - Low risk, medium impact
9. **String Interning** - Medium risk, high impact for repeated content
10. **Lookup Table Compression** - Low risk, good cache performance

### High-Impact Architectural Changes
11. **Copy-on-Write Hub Caching** - Medium risk, very high impact for repeated ops
12. **Template Specialization for Common Pairs** - Medium risk, very high impact
13. **Finite State Automaton** - High risk, high impact for complex rules
14. **Binary Schema Format** - Low risk, extreme impact for schema loading

### Advanced Techniques (High Risk/Reward)
15. **Profile-Guided Optimization** - Low risk, moderate impact across board
16. **Memory Pool Allocation** - Medium risk, good impact on allocation overhead
17. **Hot/Cold Code Splitting** - Low risk, moderate cache improvement
18. **Compile-time Specialization** - High risk, potentially high reward
19. **Inline Assembly** - Very high risk, extreme impact for critical loops
20. **Parallel Processing** - Medium risk, high impact for large texts only
21. **GPU Acceleration** - Very high risk, extreme impact for batch scenarios

## Success Metrics

### Performance Targets
- **Short-term Goal**: 50% performance improvement (reduce 18.9x gap to ~10x)
- **Medium-term Goal**: Competitive with general-purpose libraries
- **Benchmark Threshold**: Each optimization must show >20% improvement to be kept

### Quality Metrics
- **Correctness**: All existing tests must pass
- **Maintainability**: Code complexity should not increase significantly
- **Extensibility**: Runtime schema loading must remain functional

## Documentation Requirements

### For Each Optimization Attempt
1. **Baseline commit** with current benchmark results
2. **Implementation details** and architectural changes
3. **Performance measurements** before/after
4. **Decision rationale** (keep/revert) with supporting data
5. **Lessons learned** for future optimization efforts

### Failed Optimization Archive
Document strategies that didn't work to avoid repeating them:
- What was tried
- Why it failed
- Performance impact (if any)
- What we learned

## Next Action Items

1. **Select first optimization target** (recommend: Perfect Hash Tables)
2. **Create baseline commit** with current benchmark suite
3. **Implement optimization** in isolated branch
4. **Measure performance impact** using fast benchmark suite
5. **Make keep/revert decision** based on >20% improvement threshold
6. **Document results** and proceed to next optimization

This roadmap provides a structured approach to achieving competitive performance while maintaining Shlesha's architectural advantages and extensibility benefits.