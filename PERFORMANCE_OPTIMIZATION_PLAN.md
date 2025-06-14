# Shlesha Performance Optimization Plan

## Current Performance Baseline

**Current Shlesha Performance:**
- Small text: 4,849 chars/sec
- Medium text: 52,394 chars/sec  
- Large text: 85,310 chars/sec
- XLarge text: 98,383 chars/sec

**Competition Performance:**
- **Vidyut-lipi**: 952K - 21.8M chars/sec (220x faster)
- **Dharmamitra**: 207K - 933K chars/sec (9x faster)
- **Aksharamukha**: 14K - 2.7M chars/sec (27x faster)

**Performance Goal:** Achieve 2-10M chars/sec while maintaining 100% round-trip accuracy

---

## Phase 1: Architectural Optimizations (Critical Path)

### 1.1 Build Python Bindings 
**Current State:** CLI subprocess calls via `subprocess.run()`  
**Target:** Direct Rust library bindings using PyO3

**Implementation:**
- Add PyO3 dependencies to Cargo.toml
- Create Python wrapper functions around core Rust APIs
- Build wheel packages for distribution
- Update benchmark tests to use Python bindings

**Expected Performance Gain:** 10-50x speedup
- Eliminates process spawning overhead (~1-10ms per call)
- Removes serialization/deserialization of text data
- Enables persistent memory state between calls

**Accuracy Risk:** ⭐ MINIMAL
- Same core algorithms, just different interface
- No changes to transliteration logic

**Implementation Complexity:** 🟡 MEDIUM
- Well-established PyO3 patterns
- ~2-3 days development time
- Need build system setup

**Tradeoffs:**
- ✅ Massive performance improvement
- ✅ Better memory management
- ❌ Additional build complexity
- ❌ Platform-specific compilation requirements

---

### 1.2 Schema Caching & Precompilation
**Current State:** Loading TOML files from disk on every transliteration call  
**Target:** In-memory cached, precompiled schema objects

**Implementation:**
- Global schema cache with lazy initialization
- Compile parsing patterns into optimized data structures
- Schema versioning for cache invalidation
- Optional ahead-of-time compilation of schemas

**Expected Performance Gain:** 2-5x speedup
- Eliminates file I/O on hot path (~0.1-1ms per call)
- Faster pattern matching with precompiled structures
- Reduced memory allocations

**Accuracy Risk:** ⭐ MINIMAL
- Same schema data, just cached representation
- Need careful cache invalidation logic

**Implementation Complexity:** 🟡 MEDIUM
- Cache management logic
- Thread-safe access patterns
- ~1-2 days development time

**Tradeoffs:**
- ✅ Significant performance improvement
- ✅ Reduced I/O pressure
- ✅ Better resource utilization
- ❌ Increased memory usage
- ❌ Cache invalidation complexity
- ❌ Cold start penalty

---

### 1.3 Token Registry Optimization
**Current State:** Global RwLock-protected HashMap for token registration  
**Target:** Lock-free data structures with pre-populated common tokens

**Implementation:**
- Replace RwLock with atomic operations or lock-free maps
- Pre-populate registry with common Sanskrit tokens at startup
- Use numeric token IDs for faster comparisons
- Consider immutable token sets for read-heavy workloads

**Expected Performance Gain:** 1.5-2x speedup
- Reduces lock contention (~10-100μs per lookup)
- Faster token comparison operations
- Better cache locality

**Accuracy Risk:** ⭐ MINIMAL
- Same token semantics, different implementation
- Need atomic consistency guarantees

**Implementation Complexity:** 🔴 HIGH
- Complex concurrent data structures
- Requires careful memory ordering
- ~3-5 days development time

**Tradeoffs:**
- ✅ Better concurrency performance
- ✅ Reduced CPU overhead
- ❌ Complex implementation
- ❌ Platform-specific atomic operations
- ❌ Debugging difficulty

---

## Phase 2: Algorithm Optimizations (High Impact)

### 2.1 Optimized String Matching
**Current State:** HashMap lookups for pattern matching  
**Target:** Aho-Corasick automaton for simultaneous multi-pattern matching

**Implementation:**
- Replace parsing_patterns Vec + HashMap with Aho-Corasick DFA
- Use string interning for reduced allocations
- SIMD string operations where applicable
- Prefix tree optimization for common prefixes

**Expected Performance Gain:** 2-3x speedup
- O(n) vs O(n*m) pattern matching complexity
- Better cache locality with DFA traversal
- Reduced string allocations

**Accuracy Risk:** ⭐⭐ LOW
- Well-tested algorithm with same matching semantics
- Edge cases around Unicode normalization

**Implementation Complexity:** 🟡 MEDIUM  
- Well-established libraries (aho-corasick crate)
- Need careful Unicode handling
- ~2-3 days development time

**Tradeoffs:**
- ✅ Asymptotically better performance
- ✅ Memory efficient for large pattern sets
- ✅ Well-tested algorithms
- ❌ Higher memory usage for DFA construction
- ❌ Complex Unicode edge cases

---

### 2.2 Memory Pool Allocation
**Current State:** Individual heap allocations for strings and tokens  
**Target:** Object pools and arena allocators for hot path data

**Implementation:**
- String pools for common text patterns
- Token object pools to reduce allocations
- Arena allocators for parsing temporary data
- Stack-allocated small string optimization

**Expected Performance Gain:** 1.5-2x speedup
- Reduces allocation overhead (~10-100μs per allocation)
- Better cache locality
- Lower GC pressure in host language

**Accuracy Risk:** ⭐ MINIMAL
- Same data, different memory management
- Need careful lifetime management

**Implementation Complexity:** 🔴 HIGH
- Complex memory management
- Lifetime and borrowing issues in Rust
- ~4-6 days development time

**Tradeoffs:**
- ✅ Reduced allocation overhead
- ✅ Better memory locality
- ✅ Lower memory fragmentation
- ❌ Complex lifetime management
- ❌ Memory leak potential
- ❌ Higher cognitive load

---

### 2.3 Smart Parsing Strategy Selection
**Current State:** Always uses complex syllable-aware parsing for Devanagari  
**Target:** Fast path for simple cases, complex parsing only when needed

**Implementation:**
- Fast romanization parser for ASCII input
- Heuristics to detect when syllable parsing is needed
- Caching of parsing strategy decisions
- Fallback chain: simple → complex → error handling

**Expected Performance Gain:** 1.5-3x speedup
- Avoids unnecessary complexity for simple inputs
- Better average-case performance
- Reduced parsing overhead

**Accuracy Risk:** ⭐⭐ LOW-MEDIUM
- Need careful heuristics to maintain accuracy
- Edge cases around strategy switching

**Implementation Complexity:** 🟡 MEDIUM
- Heuristic development and testing
- Strategy selection logic
- ~2-3 days development time

**Tradeoffs:**
- ✅ Better average-case performance
- ✅ Adaptive complexity
- ✅ Maintains accuracy for complex cases
- ❌ Additional code complexity
- ❌ Heuristic tuning required
- ❌ Potential edge case bugs

---

## Phase 3: Implementation Optimizations (Fine-tuning)

### 3.1 Compilation Optimizations
**Current State:** Standard Rust release build  
**Target:** Profile-guided optimization and link-time optimization

**Implementation:**
- Enable PGO with representative workloads
- Link-time optimization (LTO) for cross-crate inlining
- CPU-specific optimizations (AVX2, etc.)
- Binary size vs performance tradeoffs

**Expected Performance Gain:** 1.2-1.5x speedup
- Better inlining decisions
- CPU-specific optimizations
- Dead code elimination

**Accuracy Risk:** ⭐ MINIMAL
- Same code, different compilation
- Need testing across platforms

**Implementation Complexity:** 🟢 LOW
- Compiler flag changes
- Build system modifications
- ~0.5-1 day development time

**Tradeoffs:**
- ✅ Easy to implement
- ✅ No code changes required
- ✅ Broad performance improvements
- ❌ Platform-specific builds
- ❌ Longer compilation times
- ❌ Larger binary sizes

---

### 3.2 Parallel Processing
**Current State:** Single-threaded processing  
**Target:** Parallelized parsing and batch processing

**Implementation:**
- Rayon-based parallel token processing
- Batch API for multiple texts
- SIMD operations for character classification
- Work-stealing for load balancing

**Expected Performance Gain:** 1.5-4x speedup (on multi-core)
- Better CPU utilization
- Throughput improvements for batch workloads
- SIMD acceleration

**Accuracy Risk:** ⭐⭐ LOW-MEDIUM
- Need to ensure order-independent operations
- Synchronization complexity

**Implementation Complexity:** 🔴 HIGH
- Parallel algorithm design
- Race condition prevention
- ~3-5 days development time

**Tradeoffs:**
- ✅ Excellent scalability on multi-core
- ✅ Better throughput for batch processing
- ❌ Complex synchronization
- ❌ Overhead for small inputs
- ❌ Platform-dependent scaling

---

## Implementation Priority & Risk Assessment

### Recommended Implementation Order:

1. **Phase 1.1 - Python Bindings** (Week 1)
   - Highest ROI, lowest risk
   - Enables accurate performance measurement

2. **Phase 1.2 - Schema Caching** (Week 2)  
   - High impact, medium complexity
   - Foundation for other optimizations

3. **Phase 2.1 - Optimized String Matching** (Week 3)
   - Good algorithmic improvement
   - Well-tested approach

4. **Phase 2.3 - Smart Parsing Strategy** (Week 4)
   - Adaptive performance gains
   - Requires careful testing

5. **Phase 3.1 - Compilation Optimizations** (Week 5)
   - Low effort, guaranteed gains
   - Platform validation needed

**Later phases:** 1.3, 2.2, 3.2 (higher risk/complexity)

### Risk Mitigation Strategies:

- **Accuracy Preservation:** Comprehensive round-trip tests after each optimization
- **Performance Regression:** Automated benchmark suite in CI/CD
- **Rollback Plan:** Feature flags for each optimization level
- **Gradual Deployment:** A/B testing infrastructure

### Expected Combined Performance:

**Conservative Estimate:** 20-50x improvement (2-5M chars/sec)  
**Optimistic Estimate:** 50-100x improvement (5-10M chars/sec)

This would position Shlesha as the fastest transliteration engine while maintaining its accuracy advantage.