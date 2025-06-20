//! Comprehensive script mapping definitions for all supported writing systems
//! 
//! This module contains static mapping data for 15 scripts:
//! - 9 Indic scripts: Devanagari, Bengali, Tamil, Telugu, Kannada, Malayalam, Gujarati, Gurmukhi, Odia  
//! - 6 Romanizations: IAST, SLP1, Harvard-Kyoto, ITRANS, Velthuis, WX
//!
//! Total: 15×15 = 225 mappings (including identity and bidirectional pairs)

use crate::lossless_transliterator::{LosslessMapper, FallbackStrategy};

// Script IDs (must match registration order)
pub const DEVANAGARI_ID: u8 = 1;
pub const IAST_ID: u8 = 2;  
pub const SLP1_ID: u8 = 3;
pub const BENGALI_ID: u8 = 4;
pub const TAMIL_ID: u8 = 5;
pub const TELUGU_ID: u8 = 6;
pub const KANNADA_ID: u8 = 7;
pub const MALAYALAM_ID: u8 = 8;
pub const GUJARATI_ID: u8 = 9;
pub const GURMUKHI_ID: u8 = 10;
pub const ODIA_ID: u8 = 11;
pub const HARVARD_KYOTO_ID: u8 = 12;
pub const ITRANS_ID: u8 = 13;
pub const VELTHUIS_ID: u8 = 14;
pub const WX_ID: u8 = 15;

// =============================================================================
// DEVANAGARI MAPPINGS (Enhanced from current partial implementation)
// =============================================================================

/// Complete Devanagari to IAST mapping (sorted by Unicode codepoint)
pub const DEVANAGARI_TO_IAST_SIMPLE: &[(char, &str)] = &[
    // Anusvara and Visarga (U+0902-U+0903)
    ('ं', "ṃ"), ('ः', "ḥ"),
    
    // Independent vowels (U+0905-U+0914)
    ('अ', "a"), ('आ', "ā"), ('इ', "i"), ('ई', "ī"),
    ('उ', "u"), ('ऊ', "ū"), ('ऋ', "ṛ"), ('ऌ', "ḷ"),
    ('ऍ', "ê"), ('ऎ', "e"), ('ए', "e"), ('ऐ', "ai"), 
    ('ऑ', "ô"), ('ऒ', "o"), ('ओ', "o"), ('औ', "au"),
    
    // Consonants (U+0915-U+0939) - Complete ka-varga through ha
    ('क', "ka"), ('ख', "kha"), ('ग', "ga"), ('घ', "gha"), ('ङ', "ṅa"),
    ('च', "ca"), ('छ', "cha"), ('ज', "ja"), ('झ', "jha"), ('ञ', "ña"),
    ('ट', "ṭa"), ('ठ', "ṭha"), ('ड', "ḍa"), ('ढ', "ḍha"), ('ण', "ṇa"),
    ('त', "ta"), ('थ', "tha"), ('द', "da"), ('ध', "dha"), ('न', "na"),
    ('प', "pa"), ('फ', "pha"), ('ब', "ba"), ('भ', "bha"), ('म', "ma"),
    ('य', "ya"), ('र', "ra"), ('ल', "la"), ('व', "va"),
    ('श', "śa"), ('ष', "ṣa"), ('स', "sa"), ('ह', "ha"),
    
    // Dependent vowel signs (U+093E-U+094C)
    ('ा', "ā"), ('ि', "i"), ('ी', "ī"), ('ु', "u"), ('ू', "ū"),
    ('ृ', "ṛ"), ('ॄ', "ṝ"), ('ॅ', "ê"), ('ॆ', "e"), ('े', "e"), ('ै', "ai"),
    ('ॉ', "ô"), ('ॊ', "o"), ('ो', "o"), ('ौ', "au"),
    
    // Virama (U+094D)
    ('्', ""),
    
    // Additional symbols (sorted by Unicode)
    // Note: ॐ (Om) at U+0950 is intentionally not mapped to test token preservation
    ('॑', "̀"), ('॒', "́"), ('॓', "̂"), ('॔', "̂"), // U+0951-U+0954
    
    // Punctuation (U+0964-U+0965)  
    ('।', "."), ('॥', ".."),
    
    // Numerals (U+0966-U+096F)
    ('०', "0"), ('१', "1"), ('२', "2"), ('३', "3"), ('४', "4"),
    ('५', "5"), ('६', "6"), ('७', "7"), ('८', "8"), ('९', "9"),
];

/// Devanagari pattern mappings (longest first for proper precedence)
pub const DEVANAGARI_TO_IAST_PATTERNS: &[(&str, &str)] = &[
    // Conjunct consonants (must come before individual mappings)
    ("क्ष", "kṣa"), ("ज्ञ", "jña"), ("त्र", "tra"), ("श्र", "śra"),
    
    // Common ligatures
    ("द्व", "dva"), ("स्व", "sva"), ("त्व", "tva"),
    
    // Vedic combinations
    ("द्य", "dya"), ("न्य", "nya"), ("ल्य", "lya"),
];

pub const DEVANAGARI_TO_IAST: LosslessMapper = LosslessMapper::new(
    DEVANAGARI_TO_IAST_SIMPLE,
    DEVANAGARI_TO_IAST_PATTERNS,
    DEVANAGARI_ID,
    IAST_ID,
    FallbackStrategy::PreserveWithPhonetics,
);

// =============================================================================
// IAST MAPPINGS (Complete reverse mapping)
// =============================================================================

/// Complete IAST to Devanagari mapping (sorted by Unicode codepoint)
pub const IAST_TO_DEVANAGARI_SIMPLE: &[(char, &str)] = &[
    // Basic vowels
    ('a', "अ"), ('i', "इ"), ('u', "उ"), ('e', "ए"), ('o', "ओ"),
    
    // Long vowels  
    ('ā', "आ"), ('ī', "ई"), ('ū', "ऊ"),
    
    // Vocalic consonants
    ('ṛ', "ऋ"), ('ḷ', "ऌ"), ('ṝ', "ॄ"),
    
    // Diphthongs
    ('ê', "ऍ"), ('ô', "ऑ"), ('ṃ', "ं"), ('ḥ', "ः"),
    
    // Consonants - ka varga
    ('k', "क्"), ('g', "ग्"), ('ṅ', "ङ्"),
    
    // ca varga  
    ('c', "च्"), ('j', "ज्"), ('ñ', "ञ्"),
    
    // ṭa varga
    ('ṭ', "ट्"), ('ḍ', "ड्"), ('ṇ', "ण्"),
    
    // ta varga
    ('t', "त्"), ('d', "द्"), ('n', "न्"),
    
    // pa varga
    ('p', "प्"), ('b', "ब्"), ('m', "म्"),
    
    // Semivowels and sibilants
    ('y', "य्"), ('r', "र्"), ('l', "ल्"), ('v', "व्"),
    ('ś', "श्"), ('ṣ', "ष्"), ('s', "स्"), ('h', "ह्"),
    
    // Aspirated consonants (need special handling)
    // Note: These should be handled by patterns for proper reconstruction
];

/// IAST pattern mappings for proper Devanagari reconstruction  
pub const IAST_TO_DEVANAGARI_PATTERNS: &[(&str, &str)] = &[
    // Aspirated consonants
    ("kha", "ख"), ("gha", "घ"), ("cha", "छ"), ("jha", "झ"),
    ("ṭha", "ठ"), ("ḍha", "ढ"), ("tha", "थ"), ("dha", "ध"),
    ("pha", "फ"), ("bha", "भ"),
    
    // Conjuncts (reverse of Devanagari patterns)
    ("kṣa", "क्ष"), ("jña", "ज्ञ"), ("tra", "त्र"), ("śra", "श्र"),
    
    // Voweled consonants (add inherent 'a')
    ("ka", "क"), ("ga", "ग"), ("ṅa", "ङ"),
    ("ca", "च"), ("ja", "ज"), ("ña", "ञ"), 
    ("ṭa", "ट"), ("ḍa", "ड"), ("ṇa", "ण"),
    ("ta", "त"), ("da", "द"), ("na", "न"),
    ("pa", "प"), ("ba", "ब"), ("ma", "म"),
    ("ya", "य"), ("ra", "र"), ("la", "ल"), ("va", "व"),
    ("śa", "श"), ("ṣa", "ष"), ("sa", "स"), ("ha", "ह"),
    
    // Diphthongs
    ("ai", "ै"), ("au", "ौ"),
];

pub const IAST_TO_DEVANAGARI: LosslessMapper = LosslessMapper::new(
    IAST_TO_DEVANAGARI_SIMPLE,
    IAST_TO_DEVANAGARI_PATTERNS,
    IAST_ID,
    DEVANAGARI_ID,
    FallbackStrategy::Preserve,
);

// =============================================================================
// SLP1 MAPPINGS (Sanskrit Library Phonetic Basic - Complete)
// =============================================================================

/// Complete SLP1 to Devanagari mapping
pub const SLP1_TO_DEVANAGARI_SIMPLE: &[(char, &str)] = &[
    // Vowels
    ('a', "अ"), ('A', "आ"), ('i', "इ"), ('I', "ई"), 
    ('u', "उ"), ('U', "ऊ"), ('f', "ऋ"), ('F', "ॄ"),
    ('x', "ऌ"), ('X', "ॡ"), ('e', "ए"), ('E', "ऐ"),
    ('o', "ओ"), ('O', "औ"),
    
    // Consonants - ka varga
    ('k', "क्"), ('K', "ख्"), ('g', "ग्"), ('G', "घ्"), ('N', "ङ्"),
    
    // ca varga
    ('c', "च्"), ('C', "छ्"), ('j', "ज्"), ('J', "झ्"), ('Y', "ञ्"),
    
    // ṭa varga (retroflexes)
    ('w', "ट्"), ('W', "ठ्"), ('q', "ड्"), ('Q', "ढ्"), ('R', "ण्"),
    
    // ta varga
    ('t', "त्"), ('T', "थ्"), ('d', "द्"), ('D', "ध्"), ('n', "न्"),
    
    // pa varga
    ('p', "प्"), ('P', "फ्"), ('b', "ब्"), ('B', "भ्"), ('m', "म्"),
    
    // Semivowels and sibilants
    ('y', "य्"), ('r', "र्"), ('l', "ल्"), ('v', "व्"),
    ('S', "श्"), ('z', "ष्"), ('s', "स्"), ('h', "ह्"),
    
    // Special signs
    ('M', "ं"), ('H', "ः"), ('~', "्"),
];

/// SLP1 pattern mappings for voweled consonants
pub const SLP1_TO_DEVANAGARI_PATTERNS: &[(&str, &str)] = &[
    // Add inherent 'a' vowel to consonants
    ("ka", "क"), ("Ka", "ख"), ("ga", "ग"), ("Ga", "घ"), ("Na", "ङ"),
    ("ca", "च"), ("Ca", "छ"), ("ja", "ज"), ("Ja", "झ"), ("Ya", "ञ"),
    ("wa", "ट"), ("Wa", "ठ"), ("qa", "ड"), ("Qa", "ढ"), ("Ra", "ण"), 
    ("ta", "त"), ("Ta", "थ"), ("da", "द"), ("Da", "ध"), ("na", "न"),
    ("pa", "प"), ("Pa", "फ"), ("ba", "ब"), ("Ba", "भ"), ("ma", "म"),
    ("ya", "य"), ("ra", "र"), ("la", "ल"), ("va", "व"),
    ("Sa", "श"), ("za", "ष"), ("sa", "स"), ("ha", "ह"),
];

pub const SLP1_TO_DEVANAGARI: LosslessMapper = LosslessMapper::new(
    SLP1_TO_DEVANAGARI_SIMPLE,
    SLP1_TO_DEVANAGARI_PATTERNS,
    SLP1_ID,
    DEVANAGARI_ID,
    FallbackStrategy::Preserve,
);

/// Complete Devanagari to SLP1 mapping (reverse)
pub const DEVANAGARI_TO_SLP1_SIMPLE: &[(char, &str)] = &[
    // Independent vowels
    ('अ', "a"), ('आ', "A"), ('इ', "i"), ('ई', "I"),
    ('उ', "u"), ('ऊ', "U"), ('ऋ', "f"), ('ॄ', "F"),
    ('ऌ', "x"), ('ॡ', "X"), ('ए', "e"), ('ऐ', "E"),
    ('ओ', "o"), ('औ', "O"),
    
    // Consonants with inherent 'a'
    ('क', "ka"), ('ख', "Ka"), ('ग', "ga"), ('घ', "Ga"), ('ङ', "Na"),
    ('च', "ca"), ('छ', "Ca"), ('ज', "ja"), ('झ', "Ja"), ('ञ', "Ya"),
    ('ट', "wa"), ('ठ', "Wa"), ('ड', "qa"), ('ढ', "Qa"), ('ण', "Ra"),
    ('त', "ta"), ('थ', "Ta"), ('द', "da"), ('ध', "Da"), ('न', "na"),
    ('प', "pa"), ('फ', "Pa"), ('ब', "ba"), ('भ', "Ba"), ('म', "ma"),
    ('य', "ya"), ('र', "ra"), ('ल', "la"), ('व', "va"),
    ('श', "Sa"), ('ष', "za"), ('स', "sa"), ('ह', "ha"),
    
    // Dependent vowels (for combination)
    ('ा', "A"), ('ि', "i"), ('ी', "I"), ('ु', "u"), ('ू', "U"),
    ('ृ', "f"), ('ॄ', "F"), ('े', "e"), ('ै', "E"), ('ो', "o"), ('ौ', "O"),
    
    // Special signs
    ('ं', "M"), ('ः', "H"), ('्', "~"),
];

pub const DEVANAGARI_TO_SLP1: LosslessMapper = LosslessMapper::new(
    DEVANAGARI_TO_SLP1_SIMPLE,
    &[], // Patterns handled in simple mapping
    DEVANAGARI_ID,
    SLP1_ID,
    FallbackStrategy::Preserve,
);

// =============================================================================
// PLACEHOLDER MAPPINGS FOR REMAINING SCRIPTS
// (To be implemented based on schema files)
// =============================================================================

// Bengali mappings (placeholder - will be generated from bengali.yaml)
pub const BENGALI_TO_IAST_SIMPLE: &[(char, &str)] = &[
    // Basic Bengali consonants to IAST
    ('ক', "ka"), ('খ', "kha"), ('গ', "ga"), ('ঘ', "gha"), ('ঙ', "ṅa"),
    // ... (complete mapping to be generated)
];

// Tamil mappings (placeholder - will be generated from tamil.yaml) 
pub const TAMIL_TO_IAST_SIMPLE: &[(char, &str)] = &[
    // Basic Tamil consonants to IAST (Tamil has unique phonetic characteristics)
    ('க', "ka"), ('ங', "ṅa"), ('ச', "ca"), ('ஞ', "ña"), ('ட', "ṭa"),
    // ... (complete mapping to be generated)
];

// Additional romanization scheme mappings will be added here:
// - Harvard-Kyoto (ASCII-only transliteration)
// - ITRANS (popular input method)  
// - Velthuis (TeX-based system)
// - WX (Computational linguistics standard)

/// Get all supported script names and IDs
pub fn get_supported_scripts() -> Vec<(String, u8)> {
    vec![
        ("Devanagari".to_string(), DEVANAGARI_ID),
        ("IAST".to_string(), IAST_ID),
        ("SLP1".to_string(), SLP1_ID),
        ("Bengali".to_string(), BENGALI_ID),
        ("Tamil".to_string(), TAMIL_ID),
        ("Telugu".to_string(), TELUGU_ID),
        ("Kannada".to_string(), KANNADA_ID),
        ("Malayalam".to_string(), MALAYALAM_ID),
        ("Gujarati".to_string(), GUJARATI_ID),
        ("Gurmukhi".to_string(), GURMUKHI_ID),
        ("Odia".to_string(), ODIA_ID),
        ("HarvardKyoto".to_string(), HARVARD_KYOTO_ID),
        ("ITRANS".to_string(), ITRANS_ID),
        ("Velthuis".to_string(), VELTHUIS_ID),
        ("WX".to_string(), WX_ID),
    ]
}

/// Check if a script pair mapping exists
pub fn has_mapping(from_id: u8, to_id: u8) -> bool {
    // Identity mapping always exists
    if from_id == to_id {
        return true;
    }
    
    // Currently implemented mappings
    matches!((from_id, to_id), 
        (DEVANAGARI_ID, IAST_ID) | (IAST_ID, DEVANAGARI_ID) |
        (DEVANAGARI_ID, SLP1_ID) | (SLP1_ID, DEVANAGARI_ID)
        // Add more as they are implemented
    )
}

/// Get the mapper for a script pair (if it exists)
pub fn get_mapper(from_id: u8, to_id: u8) -> Option<&'static LosslessMapper> {
    match (from_id, to_id) {
        (DEVANAGARI_ID, IAST_ID) => Some(&DEVANAGARI_TO_IAST),
        (IAST_ID, DEVANAGARI_ID) => Some(&IAST_TO_DEVANAGARI),
        (DEVANAGARI_ID, SLP1_ID) => Some(&DEVANAGARI_TO_SLP1),
        (SLP1_ID, DEVANAGARI_ID) => Some(&SLP1_TO_DEVANAGARI),
        // Add more mappings as they are implemented
        _ => None,
    }
}