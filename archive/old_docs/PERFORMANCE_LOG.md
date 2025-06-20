# Shlesha Performance Optimization Log

## Baseline Performance (After Generator Cache Fix)
- **Simple text (6 chars)**: 8-15µs 
- **Complex text (17 chars)**: 22µs
- **Throughput**: ~2.5 MB/s
- **Target**: 50+ MB/s (20x improvement needed)

## Root Cause Analysis

### Profiling Results (17-char text: "कृष्णार्जुनसंवादः")
| Component | Time | Percentage |
|-----------|------|------------|
| Parsing | 17µs | 77% |
| Transformation | 21µs | 95% |
| Generation | 0µs | 0% |
| **Total Pipeline** | **22µs** | **100%** |

### Critical Bottlenecks Identified

1. **String Allocation Overhead (CRITICAL)**
   - ~408 temporary strings allocated per 17-character word
   - Only 4.2% allocation efficiency (17 matches / 408 attempts)
   - Each failed lookup: allocate string → HashMap lookup → discard
   - String allocations alone: ~4µs

2. **Algorithmic Complexity (CRITICAL)**
   - Current: O(text_len × 4_lengths × N_categories)
   - Nested loops: for each position, try 4 lengths across 6 categories
   - Vidyut likely: O(text_len) with finite state automaton

3. **Architecture Overhead (MEDIUM)**
   - Bidirectional transformation adds 21µs
   - Generic IR with runtime type checking
   - HashMap indirection: String → HashMap → ElementMapping
   - Property bags and dynamic element creation

## Optimization Attempts

### ✅ 1. Generator Reverse Mapping Cache (SUCCESSFUL)
**Change**: Cache reverse mappings during schema load instead of rebuilding per call
**Result**: 
- Single char: 13µs → 2µs (85% improvement)
- Simple word: 18µs → 8µs (56% improvement)
- Throughput: 2.0 → 2.5 MB/s

### ❌ 2. Trie-Based Parser (FAILED - REVERTED)
**Change**: Replace nested HashMap lookups with trie structure
**Result**: 
- Simple text: 14.8µs → 18.5µs (25% regression)
- Long text: 91.8µs → 174.4µs (90% regression)
**Reason**: Trie traversal + cloning overhead > HashMap benefit

### 🔬 3. Performance Potential Analysis
**Test**: Simulated zero-allocation approach using char slices as keys
**Result**: 22µs → 2µs (11x improvement potential)
**Key Finding**: String allocations are the primary bottleneck

## Comparison with Fast Libraries

### Vidyut-Lipi Analysis
**Why Vidyut is faster**:
1. **Compiled lookup tables** vs runtime HashMap traversal
2. **Single-direction** vs bidirectional transformation
3. **Fixed schema** vs extensible property system
4. **Finite state automaton** vs nested loops
5. **No intermediate IR** vs full IR construction

### Simple Approach Benchmark
**Test**: Character-by-character HashMap lookup (no extensions, no IR)
**Result**: 22µs → 1µs (22x faster)
**Trade-offs**: No extensibility, no bidirectionality, no property preservation

## Architectural Issues Identified

### 1. Impedance Mismatch
```
YAML Schema:    HashMap<String, ElementMapping>  (string keys)
Parser:         chars → String → lookup → IR     (allocation required)
IR Goal:        Work with structured data        (no strings)
```

### 2. Design Philosophy Tension
- **Shlesha**: Prioritizes extensibility and bidirectionality
- **Vidyut**: Prioritizes speed with fixed schemas
- **Trade-off**: Flexibility vs Performance

### 3. Memory Allocation Patterns
- **Current**: 408 allocations per 17-char word (4.2% efficiency)
- **Optimal**: Zero allocations during parsing
- **Problem**: Schema design forces string-key lookups

## Proposed Solutions (Not Yet Implemented)

### A. Pre-Compiled Character Lookup Tables
**Concept**: Build `HashMap<Vec<char>, ElementId>` during schema load
**Benefits**: 
- Zero runtime string allocations
- Maintains full extensibility
- ~11x performance improvement potential
**Implementation**: Extend schema loading phase

### B. Compile-Time Schema Serialization
**Concept**: Serialize optimized lookup tables into binary at compile time
**Benefits**:
- No runtime schema loading
- Maximum performance
- Predictable startup time
**Challenges**: 
- Dynamic extension loading
- Build-time complexity

### C. String/Char-Based IR
**Concept**: Replace complex IR with simpler string-based representation
**Benefits**:
- Simpler data structures
- Potentially faster processing
- Less memory overhead
**Challenges**:
- Loss of type safety
- Property preservation complexity

### D. Hybrid Approach
**Concept**: Fast path for common cases + full IR for complex cases
**Benefits**:
- Best of both worlds
- Graceful degradation
**Challenges**:
- Code complexity
- Maintenance burden

## Next Steps for Discussion

### 1. Compile-Time Schema Serialization
**Question**: Can we serialize optimized lookup tables into the binary?
- Use build scripts to process YAML → optimized binary format
- Include lookup tables as static data in executable
- Trade-off: Binary size vs runtime performance

### 2. String/Char-Based IR Simplification
**Question**: Would `&str`/`char`-based IR be simpler and faster?
- Current IR: Complex typed structures with properties
- Proposed: String-based with minimal metadata
- Trade-off: Type safety vs performance

### 3. Performance Target Validation
**Question**: Is 50+ MB/s realistic with extensibility requirements?
- Simple approach: 22x faster but no extensibility
- Vidyut approach: Fixed schema, compiled tables
- Our goal: Extensible but fast

## Key Insights

1. **String allocations dominate cost** - solving this gives 11x improvement
2. **Extensibility has real performance cost** - but may be acceptable with optimizations
3. **Bidirectionality requires transformation step** - 21µs overhead
4. **YAML-driven schema design creates allocation pressure** - need pre-compilation
5. **Algorithmic complexity matters** - O(n×m×c) vs O(n) is significant

## Success Metrics
- [x] **Identify bottlenecks**: String allocations (408 per word)
- [x] **Measure improvement potential**: 11x with zero allocations
- [ ] **Implement zero-allocation parsing**: TBD
- [ ] **Achieve target throughput**: 50+ MB/s
- [ ] **Maintain extensibility**: Full schema/extension support