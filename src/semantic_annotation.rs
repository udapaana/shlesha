use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Semantic annotation for phonemes that don't have direct enum representation
/// This captures the MEANING, not the representation - the representation is predetermined
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SemanticAnnotation {
    /// The core phonetic meaning - what sound this represents
    pub phonetic_meaning: PhoneticMeaning,
    
    /// Contextual information that affects representation
    pub context: PhoneticContext,
    
    /// Source language/script that provided this annotation
    pub source_language: String,
    
    /// Historical period (affects representation choices)
    pub era: Option<Era>,
}

/// The actual phonetic meaning - independent of script representation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PhoneticMeaning {
    /// A vowel with specific qualities
    Vowel {
        height: VowelHeight,        // High, Mid, Low
        backness: VowelBackness,    // Front, Central, Back
        roundedness: Roundedness,   // Rounded, Unrounded
        length: VowelLength,        // Short, Long, ExtraLong
        nasalization: Nasalization, // None, Partial, Full
        tone: Option<ToneSpec>,     // For tonal languages
    },
    
    /// A consonant with specific qualities
    Consonant {
        place: PlaceOfArticulation,  // Where it's made
        manner: MannerOfArticulation, // How it's made
        voicing: Voicing,            // Voiced, Voiceless
        aspiration: Aspiration,      // None, Weak, Strong, Breathy
        secondary: Vec<SecondaryArticulation>, // Palatalization, etc.
    },
    
    /// A modifier that affects adjacent sounds
    Modifier {
        modifier_type: ModifierType,
        scope: ModifierScope,        // What it modifies
        strength: ModifierStrength,  // How strong the effect
    },
    
    /// A complex sound (cluster, diphthong, etc.)
    Complex {
        components: Vec<PhoneticMeaning>,
        combination_type: CombinationType,
    },
}

/// Contextual information that affects how the meaning gets rendered
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PhoneticContext {
    /// Position in syllable
    pub syllable_position: SyllablePosition,
    
    /// Surrounding sounds (for allophonic variation)
    pub preceding_sound: Option<PhoneticClass>,
    pub following_sound: Option<PhoneticClass>,
    
    /// Morphological context
    pub morpheme_boundary: bool,
    pub word_boundary: bool,
    
    /// Register/style
    pub register: Register,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VowelHeight {
    Close,      // i, u
    NearClose,  // ɪ, ʊ  
    CloseMid,   // e, o
    Mid,        // e̞, o̞
    OpenMid,    // ɛ, ɔ
    NearOpen,   // æ
    Open,       // a, ɑ
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VowelBackness {
    Front,      // i, e
    NearFront,  // ɪ
    Central,    // ə, ɨ
    NearBack,   // ʊ
    Back,       // u, o
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Roundedness {
    Rounded,
    Unrounded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VowelLength {
    ExtraShort,  // ə̆
    Short,       // a
    HalfLong,    // aˑ  
    Long,        // aː
    ExtraLong,   // aːː (pluti)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlaceOfArticulation {
    Bilabial,           // p, b, m
    Labiodental,        // f, v
    Dental,             // t̪, d̪ (Indic dentals)
    Alveolar,           // t, d, n (English-style)
    PostAlveolar,       // ʃ, ʒ
    Retroflex,          // ṭ, ḍ, ṇ (Indic)
    Palatal,            // c, j, ñ
    Velar,              // k, g, ŋ
    Uvular,             // q, ɢ
    Pharyngeal,         // ħ, ʕ
    Glottal,            // ʔ, h
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MannerOfArticulation {
    Stop,               // p, t, k
    Fricative,          // f, s, ʃ
    Affricate,         // tʃ, dʒ
    Nasal,             // m, n, ŋ
    Trill,             // r
    Tap,               // ɾ
    Approximant,       // w, j, ɹ
    LateralApproximant, // l
    LateralFricative,  // ɬ
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Voicing {
    Voiceless,
    Voiced,
    PartialVoicing,    // Partially voiced
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Aspiration {
    None,
    Weak,              // Slight aspiration
    Strong,            // kʰ, pʰ (Indic aspirated)
    Breathy,           // Breathy voice
    Creaky,            // Creaky voice
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecondaryArticulation {
    Palatalization,    // ʲ
    Velarization,      // ˠ
    Pharyngealization, // ˤ
    Labialization,     // ʷ
    Nasalization,      // ̃
    Rhotacization,     // ˞
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModifierType {
    VowelSuppressor,   // Virama/halant
    Nasalizer,         // Anusvara, candrabindu
    Aspirator,         // Visarga
    Lengthener,        // Length marks
    ToneMarker,        // Udatta, anudatta
    Geminator,         // Doubling marker
    Boundary,          // Word/morpheme boundary
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModifierScope {
    PrecedingVowel,
    FollowingVowel,
    PrecedingConsonant,
    FollowingConsonant,
    EntireSyllable,
    EntireWord,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModifierStrength {
    Weak,
    Medium,
    Strong,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CombinationType {
    Cluster,           // Consonant cluster
    Diphthong,         // Vowel sequence
    Triphthong,        // Triple vowel
    Affricate,         // Single phoneme from two
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Era {
    Vedic,             // ~1500-500 BCE
    Classical,         // ~500 BCE - 1000 CE
    Medieval,          // ~1000-1500 CE
    EarlyModern,       // ~1500-1800 CE
    Modern,            // ~1800-1947 CE
    Contemporary,      // ~1947-present
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SyllablePosition {
    Onset,             // Beginning of syllable
    Nucleus,           // Vowel center
    Coda,              // End of syllable
    OnsetCluster,      // Part of onset cluster
    CodaCluster,       // Part of coda cluster
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PhoneticClass {
    Vowel,
    Consonant,
    Glide,
    Liquid,
    Nasal,
    Fricative,
    Stop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Register {
    Formal,            // Formal/literary register
    Colloquial,        // Everyday speech
    Ritual,            // Religious/ritual context
    Archaic,           // Old-fashioned
    Regional,          // Regional variant
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Nasalization {
    None,
    Partial,           // Slight nasalization
    Full,              // Complete nasalization
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToneSpec {
    Level(u8),         // Level tone (1-5)
    Contour(u8, u8),   // Start-end tone
    Complex(u8, u8, u8), // Three-point contour
}

/// Resolution engine that maps semantic annotations to predetermined script representations
pub struct SemanticResolver {
    /// Script-specific resolution rules
    script_resolvers: HashMap<String, ScriptResolver>,
}

/// Resolves semantic annotations to specific script representations
pub struct ScriptResolver {
    /// Maps phonetic meanings to script characters/sequences
    phonetic_mappings: HashMap<PhoneticMeaning, String>,
    
    /// Context-sensitive rules
    context_rules: Vec<ContextRule>,
    
    /// Default fallback patterns
    fallback_patterns: HashMap<PhoneticClass, String>,
}

/// Context-sensitive resolution rule
#[derive(Debug, Clone)]
pub struct ContextRule {
    /// Condition for when this rule applies
    pub condition: ContextCondition,
    
    /// What to output when condition matches
    pub output: String,
    
    /// Priority (higher = more specific)
    pub priority: u32,
}

#[derive(Debug, Clone)]
pub enum ContextCondition {
    /// Match specific phonetic meaning in specific context
    SpecificContext {
        meaning: PhoneticMeaning,
        context: PhoneticContext,
    },
    
    /// Match phonetic class with context
    ClassContext {
        class: PhoneticClass,
        context: PhoneticContext,
    },
    
    /// Match sequence patterns
    Sequence {
        pattern: Vec<PhoneticClass>,
        position: usize, // Which element in pattern we're resolving
    },
}

impl SemanticResolver {
    pub fn new() -> Self {
        let mut resolver = Self {
            script_resolvers: HashMap::new(),
        };
        
        resolver.setup_default_resolvers();
        resolver
    }
    
    /// Set up resolvers for major scripts
    fn setup_default_resolvers(&mut self) {
        self.script_resolvers.insert("Devanagari".to_string(), Self::create_devanagari_resolver());
        self.script_resolvers.insert("Tamil".to_string(), Self::create_tamil_resolver());
        self.script_resolvers.insert("IAST".to_string(), Self::create_iast_resolver());
        self.script_resolvers.insert("Harvard-Kyoto".to_string(), Self::create_hk_resolver());
        self.script_resolvers.insert("SLP1".to_string(), Self::create_slp1_resolver());
    }
    
    /// Create Devanagari-specific resolver
    fn create_devanagari_resolver() -> ScriptResolver {
        let mut mappings = HashMap::new();
        
        // Example: Map semantic meaning to Devanagari
        mappings.insert(
            PhoneticMeaning::Consonant {
                place: PlaceOfArticulation::Velar,
                manner: MannerOfArticulation::Stop,
                voicing: Voicing::Voiceless,
                aspiration: Aspiration::None,
                secondary: vec![],
            },
            "क".to_string() // ka
        );
        
        mappings.insert(
            PhoneticMeaning::Consonant {
                place: PlaceOfArticulation::Velar,
                manner: MannerOfArticulation::Stop,
                voicing: Voicing::Voiceless,
                aspiration: Aspiration::Strong,
                secondary: vec![],
            },
            "ख".to_string() // kha
        );
        
        // TODO: Complete mapping for all phonetic meanings
        
        ScriptResolver {
            phonetic_mappings: mappings,
            context_rules: vec![],
            fallback_patterns: HashMap::new(),
        }
    }
    
    /// Create Tamil-specific resolver
    fn create_tamil_resolver() -> ScriptResolver {
        let mut mappings = HashMap::new();
        
        // Tamil has different representations for the same semantic meanings
        mappings.insert(
            PhoneticMeaning::Consonant {
                place: PlaceOfArticulation::Velar,
                manner: MannerOfArticulation::Stop,
                voicing: Voicing::Voiceless,
                aspiration: Aspiration::None,
                secondary: vec![],
            },
            "க".to_string() // Tamil ka
        );
        
        // TODO: Complete Tamil mappings
        
        ScriptResolver {
            phonetic_mappings: mappings,
            context_rules: vec![],
            fallback_patterns: HashMap::new(),
        }
    }
    
    /// Create IAST resolver
    fn create_iast_resolver() -> ScriptResolver {
        let mut mappings = HashMap::new();
        
        mappings.insert(
            PhoneticMeaning::Consonant {
                place: PlaceOfArticulation::Velar,
                manner: MannerOfArticulation::Stop,
                voicing: Voicing::Voiceless,
                aspiration: Aspiration::None,
                secondary: vec![],
            },
            "k".to_string()
        );
        
        mappings.insert(
            PhoneticMeaning::Consonant {
                place: PlaceOfArticulation::Velar,
                manner: MannerOfArticulation::Stop,
                voicing: Voicing::Voiceless,
                aspiration: Aspiration::Strong,
                secondary: vec![],
            },
            "kh".to_string()
        );
        
        // TODO: Complete IAST mappings
        
        ScriptResolver {
            phonetic_mappings: mappings,
            context_rules: vec![],
            fallback_patterns: HashMap::new(),
        }
    }
    
    /// Create Harvard-Kyoto resolver
    fn create_hk_resolver() -> ScriptResolver {
        let mut mappings = HashMap::new();
        
        // Harvard-Kyoto uses ASCII-only representations
        mappings.insert(
            PhoneticMeaning::Consonant {
                place: PlaceOfArticulation::Palatal,
                manner: MannerOfArticulation::Fricative,
                voicing: Voicing::Voiceless,
                aspiration: Aspiration::None,
                secondary: vec![],
            },
            "z".to_string() // श -> z in HK
        );
        
        // TODO: Complete HK mappings
        
        ScriptResolver {
            phonetic_mappings: mappings,
            context_rules: vec![],
            fallback_patterns: HashMap::new(),
        }
    }
    
    /// Create SLP1 resolver  
    fn create_slp1_resolver() -> ScriptResolver {
        let mut mappings = HashMap::new();
        
        // SLP1 also uses ASCII-only
        mappings.insert(
            PhoneticMeaning::Consonant {
                place: PlaceOfArticulation::Palatal,
                manner: MannerOfArticulation::Fricative,
                voicing: Voicing::Voiceless,
                aspiration: Aspiration::None,
                secondary: vec![],
            },
            "S".to_string() // श -> S in SLP1
        );
        
        // TODO: Complete SLP1 mappings
        
        ScriptResolver {
            phonetic_mappings: mappings,
            context_rules: vec![],
            fallback_patterns: HashMap::new(),
        }
    }
    
    /// Resolve semantic annotation to script representation
    pub fn resolve(&self, annotation: &SemanticAnnotation, target_script: &str) -> Option<String> {
        if let Some(resolver) = self.script_resolvers.get(target_script) {
            resolver.resolve_meaning(&annotation.phonetic_meaning, &annotation.context)
        } else {
            None
        }
    }
}

impl ScriptResolver {
    /// Resolve phonetic meaning to script representation
    fn resolve_meaning(&self, meaning: &PhoneticMeaning, context: &PhoneticContext) -> Option<String> {
        // First try context-sensitive rules
        for rule in &self.context_rules {
            if rule.matches(meaning, context) {
                return Some(rule.output.clone());
            }
        }
        
        // Then try direct phonetic mapping
        if let Some(output) = self.phonetic_mappings.get(meaning) {
            return Some(output.clone());
        }
        
        // Finally try fallback patterns
        let phonetic_class = meaning.to_phonetic_class();
        self.fallback_patterns.get(&phonetic_class).cloned()
    }
}

impl ContextRule {
    fn matches(&self, meaning: &PhoneticMeaning, context: &PhoneticContext) -> bool {
        match &self.condition {
            ContextCondition::SpecificContext { meaning: rule_meaning, context: rule_context } => {
                meaning == rule_meaning && context == rule_context
            },
            ContextCondition::ClassContext { class, context: rule_context } => {
                meaning.to_phonetic_class() == *class && context == rule_context
            },
            ContextCondition::Sequence { .. } => {
                // TODO: Implement sequence matching
                false
            },
        }
    }
}

impl PhoneticMeaning {
    fn to_phonetic_class(&self) -> PhoneticClass {
        match self {
            PhoneticMeaning::Vowel { .. } => PhoneticClass::Vowel,
            PhoneticMeaning::Consonant { manner, .. } => {
                match manner {
                    MannerOfArticulation::Stop => PhoneticClass::Stop,
                    MannerOfArticulation::Fricative => PhoneticClass::Fricative,
                    MannerOfArticulation::Nasal => PhoneticClass::Nasal,
                    MannerOfArticulation::Trill | MannerOfArticulation::Tap | 
                    MannerOfArticulation::LateralApproximant => PhoneticClass::Liquid,
                    MannerOfArticulation::Approximant => PhoneticClass::Glide,
                    _ => PhoneticClass::Consonant,
                }
            },
            PhoneticMeaning::Modifier { .. } => PhoneticClass::Consonant, // Default
            PhoneticMeaning::Complex { .. } => PhoneticClass::Consonant, // Default
        }
    }
}

impl Default for SemanticResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_resolution() {
        let resolver = SemanticResolver::new();
        
        // Create semantic annotation for /k/ sound
        let annotation = SemanticAnnotation {
            phonetic_meaning: PhoneticMeaning::Consonant {
                place: PlaceOfArticulation::Velar,
                manner: MannerOfArticulation::Stop,
                voicing: Voicing::Voiceless,
                aspiration: Aspiration::None,
                secondary: vec![],
            },
            context: PhoneticContext {
                syllable_position: SyllablePosition::Onset,
                preceding_sound: None,
                following_sound: Some(PhoneticClass::Vowel),
                morpheme_boundary: false,
                word_boundary: false,
                register: Register::Formal,
            },
            source_language: "Sanskrit".to_string(),
            era: Some(Era::Classical),
        };
        
        // Should resolve to different representations in different scripts
        assert_eq!(resolver.resolve(&annotation, "Devanagari"), Some("क".to_string()));
        assert_eq!(resolver.resolve(&annotation, "Tamil"), Some("க".to_string()));
        assert_eq!(resolver.resolve(&annotation, "IAST"), Some("k".to_string()));
    }
}