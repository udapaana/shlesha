# Performance Design Options Analysis

## Current Problem
- **Dynamic variants**: Runtime string-based element creation
- **String copies**: Every element stores owned strings (grapheme, canonical)
- **HashMap lookups**: String keys require allocation for lookup
- **Flexibility cost**: Extensibility requires dynamic dispatch

## Option 1: Fixed Enum Variants with Contextual Mapping

### Concept
```rust
// Fixed compile-time variants
enum AbugidaElement {
    Element0, Element1, Element2, ..., Element999
}

enum AlphabetElement {
    Char0, Char1, Char2, ..., Char499
}

// Context-dependent mapping
struct TranslationContext {
    abugida_meanings: [&'static str; 1000],
    alphabet_meanings: [&'static str; 500],
}
```

### Benefits
- **Zero string storage**: Enums are just integers
- **Fast matching**: Simple integer comparisons
- **Cache-friendly**: Dense memory layout
- **No allocations**: All data is static

### Implementation Strategy
```rust
// Build-time generation
const DEVANAGARI_MEANINGS: [&'static str; 1000] = [
    "ka", "kha", "ga", "gha", "ṅa", // ...
];

// Runtime usage
match element {
    AbugidaElement::Element52 => context.abugida_meanings[52], // "ka"
    AbugidaElement::Element53 => context.abugida_meanings[53], // "kha"
}
```

### Challenges
- **Fixed size limits**: What if we need more than 1000 elements?
- **Context passing**: TranslationContext must flow through all operations
- **Schema coordination**: Build-time assignment of enum variants to meanings
- **Extension complexity**: How to map runtime extensions to fixed enums?

## Option 2: String Interning/Singleton Pointers

### Concept
```rust
// Global string pool
static STRING_POOL: OnceCell<StringPool> = OnceCell::new();

struct StringPool {
    strings: HashSet<&'static str>,
    // or Arc<str> for dynamic strings
}

enum Element {
    Canonical(&'static str),  // Pointer to singleton
    Grapheme(&'static str),   // No owned strings
}
```

### Benefits
- **Deduplication**: Each unique string stored once
- **Small elements**: Just pointers, not owned strings
- **Fast comparisons**: Pointer equality
- **Memory efficient**: Shared string storage

### Implementation Approaches

#### A. Static String Pool
```rust
// Build-time string interning
const STRINGS: &[&'static str] = &["ka", "kha", "ga", ...];
const STRING_MAP: phf::Map<&'static str, usize> = // build-time hash map

fn intern(s: &str) -> &'static str {
    STRINGS[STRING_MAP[s]]
}
```

#### B. Runtime String Pool with Arc
```rust
struct StringPool {
    pool: DashMap<String, Arc<str>>,
}

fn intern(&self, s: String) -> Arc<str> {
    self.pool.entry(s).or_insert_with(|s| s.into()).clone()
}
```

### Challenges
- **Initialization**: When/how to populate the pool?
- **Thread safety**: Concurrent access to global pool
- **Memory growth**: Pool grows indefinitely (unless with LRU)
- **Extension handling**: Dynamic strings require Arc approach

## Option 3: Hybrid Enum + String Fallback

### Concept
```rust
enum KnownElement {
    // Fast path: compile-time known elements
    Ka, Kha, Ga, Gha, // ... top 200-500 most common
}

enum Element {
    Known(KnownElement),           // Fast integer-based
    Extension(Arc<str>, Arc<str>), // Fallback for extensions
}
```

### Benefits
- **Best of both worlds**: Fast common case, flexible extensions
- **Graceful degradation**: Performance scales with extension usage
- **Incremental adoption**: Can start with small known set
- **Clear performance model**: Known = fast, extensions = slower

### Implementation Strategy
```rust
impl Element {
    fn canonical(&self, context: &TranslationContext) -> &str {
        match self {
            Element::Known(k) => context.get_canonical(*k), // Array lookup
            Element::Extension(_, canonical) => canonical,   // Arc deref
        }
    }
}
```

### Coverage Strategy
- **Core elements**: Top 200-500 most frequent graphemes
- **Long tail**: Everything else as extensions
- **Statistics-driven**: Analyze real corpora to determine optimal split

## Option 4: Optimized Current Approach

### Concept
Keep current architecture but optimize the bottlenecks:

```rust
// Pre-built lookup tables (no runtime string alloc)
struct OptimizedParser {
    char_to_element: HashMap<SmallVec<[char; 4]>, ElementId>,
    element_data: Vec<ElementData>,
}

// Compact element representation
#[derive(Copy, Clone)]
struct ElementId(u32);

struct ElementData {
    canonical: StringRef,  // Index into string pool
    grapheme: StringRef,
    properties: u32,       // Bitflags for common properties
}
```

## Comparative Analysis

| Approach | Performance | Extensibility | Complexity | Memory |
|----------|-------------|---------------|------------|---------|
| **Fixed Enums** | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **String Interning** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ |
| **Hybrid** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐ |
| **Optimized Current** | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |

## Specific Implementation Questions

### 1. Fixed Enum Sizing
- **How many variants?** Analysis of existing schemas needed
- **Growth strategy?** What happens when we hit the limit?
- **Assignment strategy?** Frequency-based vs alphabetical?

### 2. String Interning Details
- **When to intern?** Schema load time vs lazy?
- **Thread safety model?** Global pool vs per-transliterator?
- **Memory management?** Leak vs reference counting?

### 3. Hybrid Threshold
- **Coverage target?** 90%, 95%, 99% of common elements?
- **Runtime detection?** Can we measure extension usage?
- **Migration path?** Start small, grow the known set?

### 4. Context Passing
- **How much context?** Just mappings vs full schema info?
- **Performance impact?** Passing large contexts around?
- **Thread safety?** Immutable context vs synchronized?

## Recommendation Priority

1. **String Interning (Option 2)** - Least disruptive, immediate gains
2. **Hybrid Approach (Option 3)** - Best long-term performance/flexibility balance
3. **Fixed Enums (Option 1)** - Maximum performance, significant architecture change
4. **Optimized Current (Option 4)** - Incremental improvements

## Next Steps for Discussion

1. **Performance requirements**: What's the minimum acceptable speedup?
2. **Extension usage patterns**: How heavily are extensions used in practice?
3. **Implementation timeline**: How much refactoring is acceptable?
4. **Compatibility requirements**: Must maintain current API?

What's your intuition on these trade-offs? The hybrid approach feels most promising to me - we could start with top 200 Devanagari elements as known enums, measure the performance gain, then decide if it's worth extending further.