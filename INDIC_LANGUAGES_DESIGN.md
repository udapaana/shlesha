# All Indic Languages Optimized Design

## Scope Expansion: From Sanskrit to All Indic Languages

### Language Families Covered
- **Indo-Aryan**: Sanskrit, Hindi, Bengali, Gujarati, Marathi, Punjabi, Assamese, Odia, etc.
- **Dravidian**: Tamil, Telugu, Kannada, Malayalam, etc.
- **Others**: Tibeto-Burman languages using Indic scripts

### Script Coverage
- **Primary**: Devanagari, Bengali, Gujarati, Gurmukhi, Tamil, Telugu, Kannada, Malayalam, Odia, Assamese
- **Extended**: Tibetan, Myanmar, Thai, Khmer (related abugida systems)

## Expanded Phoneme Analysis

### Common Indic Phoneme Inventory

#### 1. **Vowels (स्वर/स्वरवर्ण)**
```rust
enum IndicVowel {
    // Short vowels
    A, I, U, E, O,                    // Common to all
    Ri, Li,                           // Sanskrit/North Indian
    Ae, Oe,                           // Tamil/South Indian
    
    // Long vowels  
    Aa, Ii, Uu, Ee, Oo,             // Common to all
    Rii, Lii,                        // Sanskrit/North Indian
    Aae, Ooe,                        // Tamil/South Indian
    
    // Diphthongs
    Ai, Au,                          // Common
    
    // Regional vowels
    Schwa,                           // Hindi/Marathi (inherent vowel variation)
    CentralizedVowels(u8),           // Various regional vowels (1-10 variants)
}
```

#### 2. **Consonants by Articulatory Groups**
```rust
enum IndicConsonant {
    // === STOPS (स्पर्श) ===
    // Velars (कवर्ग)
    Ka, Kha, Ga, Gha, Nga,
    
    // Palatals (चवर्ग)  
    Ca, Cha, Ja, Jha, Nya,
    
    // Retroflexes (टवर्ग)
    Tta, Ttha, Dda, Ddha, Nna,
    
    // Dentals (तवर्ग)
    Ta, Tha, Da, Dha, Na,
    
    // Labials (पवर्ग)
    Pa, Pha, Ba, Bha, Ma,
    
    // === NASALS (अनुनासिक) ===
    // (Already covered in stop groups above)
    
    // === LIQUIDS (द्रव) ===
    // Semivowels (अन्तःस्थ)
    Ya, Ra, La, Va,
    
    // Additional liquids
    Lla,                             // Tamil/Telugu retroflex lateral
    Rra,                             // Tamil/Telugu trill variants
    Zha,                             // Malayalam/Tamil retroflex approximant
    
    // === FRICATIVES (ऊष्म) ===
    // Sibilants
    Sha_palatal,                     // श (śa)
    Sha_retroflex,                   // ष (ṣa)  
    Sa,                              // स (sa)
    
    // Regional fricatives
    Fa,                              // फ़ (Urdu/Hindi)
    Za,                              // ज़ (Urdu/Hindi)
    Kha_fricative,                   // ख़ (Urdu/Hindi)
    Gha_fricative,                   // ग़ (Urdu/Hindi)
    
    // === ASPIRATE ===
    Ha,
    
    // === REGIONAL CONSONANTS ===
    // Bengali/Assamese
    Wa,                              // Bengali va/wa distinction
    
    // Tamil/South Indian
    Nga_hard,                        // ங் (Tamil hard nga)
    Nya_hard,                        // ஞ் (Tamil hard nya)
    Nna_hard,                        // ண் (Tamil hard nna)
    Na_hard,                         // ந் (Tamil hard na)
    Ma_hard,                         // ம் (Tamil hard ma)
    
    // Dravidian-specific
    Ka_soft, Ga_soft,                // Tamil/Dravidian voice variations
    Ca_soft, Ja_soft,
    Tta_soft, Dda_soft,
    Ta_soft, Da_soft,
    Pa_soft, Ba_soft,
}
```

#### 3. **Modifiers and Diacritics**
```rust
enum IndicModifier {
    // === CORE MODIFIERS ===
    Virama,                          // ् (vowel killer)
    Anusvara,                        // ं (nasal)
    Visarga,                         // ः (aspiration)
    Candrabindu,                     // ँ (nasalization)
    
    // === VEDIC ACCENTS ===
    Udatta,                          // ॑ (high tone)
    Anudatta,                        // ॒ (low tone)
    Svarita,                         // (mid tone, usually unmarked)
    
    // === PUNCTUATION ===
    Danda,                           // । (single danda)
    DoubleDanda,                     // ॥ (double danda)
    Avagraha,                        // ऽ (elision mark)
    
    // === REGIONAL MODIFIERS ===
    // Tamil
    Pulli,                           // ் (Tamil virama equivalent)
    Aytham,                          // ஃ (Tamil visarga equivalent)
    
    // Telugu/Kannada
    Sunna,                           // Telugu/Kannada anusvara variants
    
    // Malayalam
    Samvruthokaram,                  // Malayalam vowel suppressor
    
    // Regional tone marks (1-20 variants for various languages)
    RegionalToneMark(u8),
}
```

#### 4. **Conjuncts and Ligatures**
```rust
enum CommonConjunct {
    // Sanskrit/Hindi common conjuncts
    Ksha,                            // क्ष
    Jnya,                            // ज्ञ
    Shri,                            // श्री
    
    // Language-specific common conjuncts (top 50-100)
    BengaliConjunct(u8),             // Bengali-specific conjuncts
    TamilConjunct(u8),               // Tamil-specific conjuncts
    TeluguConjunct(u8),              // Telugu-specific conjuncts
    // ... etc
}
```

## Unified IR Structure

```rust
enum IndicElement {
    // === ZERO-COST CORE ELEMENTS ===
    Vowel(IndicVowel),               // ~30 variants
    Consonant(IndicConsonant),       // ~80 variants  
    Modifier(IndicModifier),         // ~30 variants
    CommonConjunct(CommonConjunct),  // ~100 variants
    
    // === ROMAN/LATIN ELEMENTS ===
    Roman(RomanElement),             // IAST, ISO 15919, etc.
    
    // === WHITESPACE/PUNCTUATION ===
    Whitespace(char),                // Space, newline, etc.
    Punctuation(char),               // ASCII punctuation
    
    // === EXTENSION FALLBACK ===
    Extension {
        grapheme: Arc<str>,          // Original character(s)
        canonical: Arc<str>,         // Canonical representation
        script: Arc<str>,            // Script identifier
        category: ElementCategory,   // Vowel, consonant, etc.
    },
    
    // === UNKNOWN ===
    Unknown(char),
}

enum ElementCategory {
    Vowel, Consonant, Modifier, Conjunct, Punctuation, Other
}
```

## Coverage Analysis by Language

### **Hindi/Devanagari**
- **Core coverage**: ~98% (standard Devanagari + Urdu additions)
- **Extensions needed**: Regional variations, borrowed words

### **Bengali/Assamese** 
- **Core coverage**: ~95% (shares most phonemes with Hindi)
- **Extensions needed**: Bengali-specific conjuncts, ya-phalaa variants

### **Tamil**
- **Core coverage**: ~90-95% (distinctive phonology but mappable)
- **Extensions needed**: Grantha script elements, Sanskrit borrowings

### **Telugu/Kannada**
- **Core coverage**: ~95% (similar to Tamil but with more Sanskrit influence)
- **Extensions needed**: Language-specific conjuncts

### **Gujarati/Marathi/Punjabi**
- **Core coverage**: ~97% (close to Hindi/Devanagari)
- **Extensions needed**: Script-specific marks, regional variations

### **Malayalam**
- **Core coverage**: ~85-90% (most complex script)
- **Extensions needed**: Complex conjuncts, chillu letters

## Performance Characteristics

### Memory Layout
```rust
// Per element storage:
enum IndicElement {
    Vowel(IndicVowel),        // 1 byte (discriminant + enum)
    Consonant(IndicConsonant), // 1 byte (discriminant + enum)
    // ... other core types     // 1 byte each
    
    Extension { ... },         // ~40 bytes (4 Arc<str> + category)
}
```

### Expected Performance
- **90-98% of text**: 1-byte elements, zero allocation
- **2-10% of text**: Extension fallback with string interning
- **Overall**: ~10-20x performance improvement vs current approach

## Implementation Strategy

### Phase 1: Core Phoneme Inventory
1. **Research**: Analyze phoneme inventories of major Indic languages
2. **Enumerate**: Create comprehensive but finite enum variants
3. **Validate**: Check coverage against real corpora

### Phase 2: Script Mapping Tables
```rust
// Build-time generated lookup tables
const DEVANAGARI_TO_ELEMENT: &[(&str, IndicElement)] = &[
    ("क", IndicElement::Consonant(IndicConsonant::Ka)),
    ("ख", IndicElement::Consonant(IndicConsonant::Kha)),
    // ... thousands of mappings
];

const TAMIL_TO_ELEMENT: &[(&str, IndicElement)] = &[
    ("க", IndicElement::Consonant(IndicConsonant::Ka)),
    ("ச", IndicElement::Consonant(IndicConsonant::Ca)),
    // ... Tamil script mappings
];
```

### Phase 3: Multi-Script Parser
```rust
struct IndicParser {
    // Pre-built lookup tables per script
    script_lookups: HashMap<&'static str, &'static ScriptLookup>,
    
    // Extension fallback
    extension_pool: StringPool,
}

impl IndicParser {
    fn parse(&self, text: &str, script: &str) -> Vec<IndicElement> {
        let lookup = self.script_lookups[script];
        // Fast path: core elements (zero allocation)
        // Slow path: extensions (string interning)
    }
}
```

## Challenges and Solutions

### 1. **Phoneme Mapping Complexity**
- **Challenge**: Same phoneme, different scripts (क vs ক vs க)
- **Solution**: Unified phoneme enum + script-specific lookup tables

### 2. **Conjunct Explosion**
- **Challenge**: Thousands of possible conjuncts per language
- **Solution**: Common conjuncts as enum variants + fallback for rare ones

### 3. **Regional Variations**
- **Challenge**: Script variations within same language
- **Solution**: Extension system handles regional differences

### 4. **Build-Time Complexity**
- **Challenge**: Generating lookup tables for all scripts
- **Solution**: Data-driven build scripts + Unicode database integration

## Questions for Implementation

### 1. **Scope Definition**
- **How many languages?** Top 10? All major Indic scripts?
- **Historical vs modern?** Include archaic forms?
- **Precision vs coverage?** Perfect mapping vs good-enough approximation?

### 2. **Phoneme Granularity**
- **Allophone handling?** Same phoneme with different realizations?
- **Tone marking?** How detailed for tonal languages?
- **Conjunct strategy?** Compositional vs enumerated?

### 3. **Extension Strategy**
- **Runtime loading?** Can users add new script mappings?
- **Versioning?** How to handle phoneme inventory updates?
- **Fallback quality?** What happens when coverage insufficient?

This is a **much more ambitious and exciting project** than just Sanskrit optimization! The potential impact is enormous - a high-performance, extensible transliterator for all major Indic languages.

What's your priority order for language/script support? Should we start with the most common ones (Hindi, Bengali, Tamil, Telugu) and expand from there?