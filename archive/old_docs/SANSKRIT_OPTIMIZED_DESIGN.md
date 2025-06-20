# Sanskrit-Optimized IR Design

## Core Insight
**Sanskrit has a finite, well-defined set of phonemes.** We can enumerate these as compile-time enum variants with zero payload, and only use string interning for runtime extensions.

## Proposed IR Structure

### Core Sanskrit Phonemes (Zero-Cost Enums)

```rust
// Abugida elements - Sanskrit phonemes as simple variants
enum SanskritAbugida {
    // Vowels (स्वर)
    A, Aa, I, Ii, U, Uu, Ri, Rii, Li, Lii, E, Ai, O, Au,
    
    // Consonants (व्यञ्जन)
    // Velars (कवर्ग)
    Ka, Kha, Ga, Gha, Nga,
    // Palatals (चवर्ग)  
    Ca, Cha, Ja, Jha, Nya,
    // Retroflexes (टवर्ग)
    Ta, Tha, Da, Dha, Na,
    // Dentals (तवर्ग)
    Ta_dental, Tha_dental, Da_dental, Dha_dental, Na_dental,
    // Labials (पवर्ग)
    Pa, Pha, Ba, Bha, Ma,
    // Semivowels (अन्तःस्थ)
    Ya, Ra, La, Va,
    // Sibilants (ऊष्म)
    Sha_palatal, Sha_retroflex, Sa,
    // Aspirate
    Ha,
    
    // Modifiers
    Virama,        // ्
    Anusvara,      // ं
    Visarga,       // ः
    Candrabindu,   // ँ
    Avagraha,      // ऽ
    
    // Vedic accents
    Udatta,        // ॑
    Anudatta,      // ॒
    Svarita,       // (unmarked or ᳚)
    
    // Common conjuncts (optional optimization)
    Ksha,          // क्ष
    Jnya,          // ज्ञ
}

// Alphabet elements - Roman phonemes
enum RomanAlphabet {
    // Basic letters
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    
    // Diacritics for IAST
    A_macron,      // ā
    I_macron,      // ī  
    U_macron,      // ū
    R_dot_below,   // ṛ
    R_dot_below_macron, // ṝ
    L_dot_below,   // ḷ
    L_dot_below_macron, // ḹ
    M_dot_below,   // ṃ
    H_dot_below,   // ḥ
    N_dot_above,   // ṅ
    N_tilde,       // ñ
    N_dot_below,   // ṇ
    T_dot_below,   // ṭ
    D_dot_below,   // ḍ
    S_acute,       // ś
    S_dot_below,   // ṣ
    
    // Modifiers
    Apostrophe,    // '
}

// Unified IR with extension fallback
enum Element {
    SanskritAbugida(SanskritAbugida),           // Zero-cost variants
    RomanAlphabet(RomanAlphabet),               // Zero-cost variants
    Extension(Arc<str>, Arc<str>),              // Fallback for runtime additions
    Whitespace(char),                           // Simple char storage
    Unknown(char),                              // Simple char storage
}
```

## Performance Characteristics

### Memory Layout
```rust
// Current approach (per element)
struct CurrentElement {
    element_type: String,     // ~24 bytes (heap allocation)
    grapheme: String,         // ~24 bytes (heap allocation)  
    canonical: String,        // ~24 bytes (heap allocation)
    properties: HashMap<...>, // ~48+ bytes (heap allocation)
}
// Total: ~120+ bytes per element + heap allocations

// Proposed approach (per element)
enum Element {
    SanskritAbugida(SanskritAbugida), // 1 byte (enum discriminant)
    RomanAlphabet(RomanAlphabet),     // 1 byte (enum discriminant)
    Extension(Arc<str>, Arc<str>),    // 16 bytes (2 pointers)
    // ...
}
// Total: 1 byte for core elements, 16 bytes for extensions
```

### Performance Benefits
- **120x memory reduction** for core elements (120 bytes → 1 byte)
- **Zero allocations** for Sanskrit phonemes  
- **Cache-friendly**: Dense enum arrays vs scattered heap objects
- **Fast matching**: Integer comparisons vs string comparisons
- **Branch prediction**: Core elements likely in CPU cache

## Coverage Analysis

### Sanskrit Phoneme Coverage
Based on traditional Sanskrit phonology:
- **Vowels**: 14 variants (अ आ इ ई उ ऊ ऋ ॠ ऌ ॡ ए ऐ ओ औ)
- **Consonants**: 33 variants (standard वर्णमाला)
- **Modifiers**: 5-10 variants (virama, anusvara, etc.)
- **Vedic accents**: 3 variants
- **Total core**: ~60-70 enum variants

### Real-World Coverage Estimation
- **Classical Sanskrit**: ~95-98% coverage with core enum variants
- **Vedic Sanskrit**: ~90-95% coverage (more archaic forms)
- **Modern extensions**: Handled by Arc<str> fallback
- **Regional variants**: Handled by Arc<str> fallback

## Implementation Strategy

### Phase 1: Core Enum Infrastructure
```rust
impl SanskritAbugida {
    fn canonical_form(&self) -> &'static str {
        match self {
            Self::Ka => "ka",
            Self::Kha => "kha", 
            Self::Ga => "ga",
            // ... compile-time mapping
        }
    }
    
    fn devanagari_form(&self) -> &'static str {
        match self {
            Self::Ka => "क",
            Self::Kha => "ख",
            Self::Ga => "ग", 
            // ... compile-time mapping
        }
    }
}
```

### Phase 2: Optimized Parser
```rust
struct OptimizedParser {
    // Pre-built lookup: char sequence → core enum (zero-alloc)
    core_lookup: HashMap<&'static [char], Element>,
    
    // Fallback lookup: string → extension (string-interned)
    extension_lookup: HashMap<String, Arc<str>>,
}

impl OptimizedParser {
    fn parse_sequence(&self, chars: &[char]) -> Option<Element> {
        // Fast path: core Sanskrit elements (zero allocation)
        if let Some(element) = self.core_lookup.get(chars) {
            return Some(*element); // Copy tiny enum
        }
        
        // Slow path: extensions (string interning)
        let string_key = chars.iter().collect::<String>();
        if let Some(canonical) = self.extension_lookup.get(&string_key) {
            return Some(Element::Extension(
                Arc::from(string_key), 
                canonical.clone()
            ));
        }
        
        None
    }
}
```

### Phase 3: Schema Integration
```rust
// Build-time: Generate core lookups from Sanskrit phonology
const CORE_DEVANAGARI_LOOKUP: &[(&[char], Element)] = &[
    (&['क'], Element::SanskritAbugida(SanskritAbugida::Ka)),
    (&['ख'], Element::SanskritAbugida(SanskritAbugida::Kha)),
    // ... generated at compile time
];

// Runtime: Add extensions from YAML schemas
impl OptimizedParser {
    fn load_extensions(&mut self, schema: &Schema) {
        for (grapheme, mapping) in &schema.extension_mappings {
            let canonical = Arc::from(mapping.canonical.as_str());
            self.extension_lookup.insert(grapheme.clone(), canonical);
        }
    }
}
```

## Benefits of Sanskrit-Specific Design

### 1. Domain Knowledge Leverage
- **Finite phoneme set**: Sanskrit has well-defined sounds
- **Standardized romanization**: IAST is established
- **Predictable usage**: Core elements dominate real text

### 2. Performance Guarantees
- **Core elements**: Always zero-allocation, 1-byte storage
- **Extensions**: Bounded overhead (only for rare elements)
- **Predictable performance**: Degradation is proportional to extension usage

### 3. Extensibility Preservation
- **Full YAML schema support**: Extensions work exactly as before
- **Runtime extensions**: New elements can be added dynamically
- **Backward compatibility**: Existing schemas continue to work

## Questions for Implementation

### 1. Enum Completeness
- **Which Sanskrit elements to include?** Classical vs Vedic vs regional
- **Conjunct handling?** Common ones like क्ष as variants vs composition
- **Accent coverage?** How many Vedic accent variants needed

### 2. Lookup Table Strategy
- **Build-time generation?** Generate lookup tables from phoneme definitions
- **Static data structure?** `phf` perfect hash functions vs `HashMap`
- **Memory vs speed trade-off?** Compact tables vs fast lookups

### 3. Migration Path
- **Incremental implementation?** Start with Devanagari, add Roman later
- **Fallback strategy?** Graceful degradation when enum coverage insufficient
- **Testing approach?** Ensure no regression in functionality

This approach could potentially give us **the best of all worlds**: near-optimal performance for common Sanskrit text (95%+ coverage with 1-byte enums) while maintaining full extensibility for edge cases and regional variants.

What do you think about the phoneme coverage? Should we start with a minimal set (basic consonants/vowels) or go comprehensive from the start?