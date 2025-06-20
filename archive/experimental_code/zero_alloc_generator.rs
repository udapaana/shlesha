use std::collections::HashMap;
use crate::indic_phoneme::{IndicPhoneme, IndicVowel, IndicConsonant, IndicModifier};
use crate::semantic_annotation::SemanticResolver;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GenerateError {
    #[error("Unknown target script: {0}")]
    UnknownScript(String),
    
    #[error("Semantic resolution failed: {0}")]
    SemanticResolutionFailed(String),
    
    #[error("Invalid phoneme sequence: {0}")]
    InvalidSequence(String),
}

/// Zero-allocation generator that converts phonemes directly to target script representations
pub struct ZeroAllocGenerator {
    /// Pre-compiled lookup tables for each target script
    script_generators: HashMap<String, ScriptGenerator>,
    
    /// Semantic resolver for unknown phonemes
    semantic_resolver: SemanticResolver,
    
    /// Performance statistics
    stats: GenerateStats,
}

#[derive(Debug, Default)]
pub struct GenerateStats {
    pub total_phonemes_processed: usize,
    pub enum_lookups_used: usize,         // Fast path: direct enum → string lookup
    pub semantic_resolutions_used: usize, // Slow path: semantic resolution
    pub unknown_markers_used: usize,     // Fallback: [?:original] markers
    pub total_generate_time_ns: u64,     // Total generation time
}

impl GenerateStats {
    pub fn enum_efficiency(&self) -> f64 {
        if self.total_phonemes_processed == 0 { return 100.0; }
        (self.enum_lookups_used as f64 / self.total_phonemes_processed as f64) * 100.0
    }
    
    pub fn avg_generate_time_per_phoneme_ns(&self) -> f64 {
        if self.total_phonemes_processed == 0 { return 0.0; }
        self.total_generate_time_ns as f64 / self.total_phonemes_processed as f64
    }
}

/// Script-specific generator with pre-compiled lookup tables
struct ScriptGenerator {
    /// Direct phoneme → representation lookup (zero allocation)
    vowel_lookup: HashMap<IndicVowel, &'static str>,
    consonant_lookup: HashMap<IndicConsonant, &'static str>,
    modifier_lookup: HashMap<IndicModifier, &'static str>,
    
    /// Script-specific rules for complex phonemes
    script_rules: ScriptRules,
    
    /// Script name for fallback
    script_name: String,
}

/// Script-specific generation rules
#[derive(Debug, Clone)]
struct ScriptRules {
    /// How to handle inherent vowels in abugida scripts
    inherent_vowel_handling: InherentVowelHandling,
    
    /// How to combine consonants and vowels
    combination_rules: CombinationRules,
    
    /// How to handle conjuncts/clusters
    cluster_rules: ClusterRules,
    
    /// Unknown character fallback pattern
    unknown_pattern: String, // e.g., "[?:{original}]"
}

#[derive(Debug, Clone)]
enum InherentVowelHandling {
    /// Script has inherent vowels (Devanagari, Tamil, etc.)
    HasInherent { vowel: &'static str },
    
    /// Script explicitly marks all vowels (Latin, etc.)
    ExplicitVowels,
}

#[derive(Debug, Clone)]
enum CombinationRules {
    /// Abugida: consonant carries inherent vowel, modified by marks
    Abugida,
    
    /// Alphabet: each phoneme is separate character
    Alphabet,
    
    /// Abjad: consonants only, vowels optional
    Abjad,
}

#[derive(Debug, Clone)]
enum ClusterRules {
    /// Use virama + consonant (Devanagari style)
    Virama { virama_char: &'static str },
    
    /// Use conjunct characters when available
    Conjuncts { fallback_to_virama: bool },
    
    /// No special cluster handling (Latin style)
    None,
}

impl ZeroAllocGenerator {
    pub fn new() -> Self {
        let mut generator = Self {
            script_generators: HashMap::new(),
            semantic_resolver: SemanticResolver::new(),
            stats: GenerateStats::default(),
        };
        
        generator.setup_script_generators();
        generator
    }
    
    /// Set up generators for major scripts
    fn setup_script_generators(&mut self) {
        self.script_generators.insert("Devanagari".to_string(), Self::create_devanagari_generator());
        self.script_generators.insert("Tamil".to_string(), Self::create_tamil_generator());
        self.script_generators.insert("IAST".to_string(), Self::create_iast_generator());
        self.script_generators.insert("SLP1".to_string(), Self::create_slp1_generator());
        self.script_generators.insert("Harvard-Kyoto".to_string(), Self::create_hk_generator());
        self.script_generators.insert("ISO15919".to_string(), Self::create_iso_generator());
    }
    
    /// Create Devanagari generator
    fn create_devanagari_generator() -> ScriptGenerator {
        let mut vowel_lookup = HashMap::new();
        let mut consonant_lookup = HashMap::new();
        let mut modifier_lookup = HashMap::new();
        
        // Vowels
        vowel_lookup.insert(IndicVowel::A, "अ");
        vowel_lookup.insert(IndicVowel::Aa, "आ");
        vowel_lookup.insert(IndicVowel::I, "इ");
        vowel_lookup.insert(IndicVowel::Ii, "ई");
        vowel_lookup.insert(IndicVowel::U, "उ");
        vowel_lookup.insert(IndicVowel::Uu, "ऊ");
        vowel_lookup.insert(IndicVowel::Ri, "ऋ");
        vowel_lookup.insert(IndicVowel::Rii, "ॠ");
        vowel_lookup.insert(IndicVowel::Li, "ऌ");
        vowel_lookup.insert(IndicVowel::Lii, "ॡ");
        vowel_lookup.insert(IndicVowel::E, "ए");
        vowel_lookup.insert(IndicVowel::Ai, "ऐ");
        vowel_lookup.insert(IndicVowel::O, "ओ");
        vowel_lookup.insert(IndicVowel::Au, "औ");
        
        // Consonants
        consonant_lookup.insert(IndicConsonant::Ka, "क");
        consonant_lookup.insert(IndicConsonant::Kha, "ख");
        consonant_lookup.insert(IndicConsonant::Ga, "ग");
        consonant_lookup.insert(IndicConsonant::Gha, "घ");
        consonant_lookup.insert(IndicConsonant::Nga, "ङ");
        consonant_lookup.insert(IndicConsonant::Ca, "च");
        consonant_lookup.insert(IndicConsonant::Cha, "छ");
        consonant_lookup.insert(IndicConsonant::Ja, "ज");
        consonant_lookup.insert(IndicConsonant::Jha, "झ");
        consonant_lookup.insert(IndicConsonant::Nya, "ञ");
        consonant_lookup.insert(IndicConsonant::Tta, "ट");
        consonant_lookup.insert(IndicConsonant::Ttha, "ठ");
        consonant_lookup.insert(IndicConsonant::Dda, "ड");
        consonant_lookup.insert(IndicConsonant::Ddha, "ढ");
        consonant_lookup.insert(IndicConsonant::Nna, "ण");
        consonant_lookup.insert(IndicConsonant::Ta, "त");
        consonant_lookup.insert(IndicConsonant::Tha, "थ");
        consonant_lookup.insert(IndicConsonant::Da, "द");
        consonant_lookup.insert(IndicConsonant::Dha, "ध");
        consonant_lookup.insert(IndicConsonant::Na, "न");
        consonant_lookup.insert(IndicConsonant::Pa, "प");
        consonant_lookup.insert(IndicConsonant::Pha, "फ");
        consonant_lookup.insert(IndicConsonant::Ba, "ब");
        consonant_lookup.insert(IndicConsonant::Bha, "भ");
        consonant_lookup.insert(IndicConsonant::Ma, "म");
        consonant_lookup.insert(IndicConsonant::Ya, "य");
        consonant_lookup.insert(IndicConsonant::Ra, "र");
        consonant_lookup.insert(IndicConsonant::La, "ल");
        consonant_lookup.insert(IndicConsonant::Va, "व");
        consonant_lookup.insert(IndicConsonant::ShaPalatal, "श");
        consonant_lookup.insert(IndicConsonant::ShaRetroflex, "ष");
        consonant_lookup.insert(IndicConsonant::Sa, "स");
        consonant_lookup.insert(IndicConsonant::Ha, "ह");
        consonant_lookup.insert(IndicConsonant::Ksha, "क्ष");
        consonant_lookup.insert(IndicConsonant::Gnya, "ज्ञ");
        
        // Modifiers
        modifier_lookup.insert(IndicModifier::Virama, "्");
        modifier_lookup.insert(IndicModifier::Anusvara, "ं");
        modifier_lookup.insert(IndicModifier::Visarga, "ः");
        modifier_lookup.insert(IndicModifier::Candrabindu, "ँ");
        modifier_lookup.insert(IndicModifier::Avagraha, "ऽ");
        modifier_lookup.insert(IndicModifier::Nukta, "़");
        
        ScriptGenerator {
            vowel_lookup,
            consonant_lookup,
            modifier_lookup,
            script_rules: ScriptRules {
                inherent_vowel_handling: InherentVowelHandling::HasInherent { vowel: "अ" },
                combination_rules: CombinationRules::Abugida,
                cluster_rules: ClusterRules::Virama { virama_char: "्" },
                unknown_pattern: "[?:{original}]".to_string(),
            },
            script_name: "Devanagari".to_string(),
        }
    }
    
    /// Create SLP1 generator (ASCII-based)
    fn create_slp1_generator() -> ScriptGenerator {
        let mut vowel_lookup = HashMap::new();
        let mut consonant_lookup = HashMap::new();
        let mut modifier_lookup = HashMap::new();
        
        // Vowels (SLP1)
        vowel_lookup.insert(IndicVowel::A, "a");
        vowel_lookup.insert(IndicVowel::Aa, "A");
        vowel_lookup.insert(IndicVowel::I, "i");
        vowel_lookup.insert(IndicVowel::Ii, "I");
        vowel_lookup.insert(IndicVowel::U, "u");
        vowel_lookup.insert(IndicVowel::Uu, "U");
        vowel_lookup.insert(IndicVowel::Ri, "f");
        vowel_lookup.insert(IndicVowel::Rii, "F");
        vowel_lookup.insert(IndicVowel::Li, "x");
        vowel_lookup.insert(IndicVowel::Lii, "X");
        vowel_lookup.insert(IndicVowel::E, "e");
        vowel_lookup.insert(IndicVowel::Ai, "Y");
        vowel_lookup.insert(IndicVowel::O, "o");
        vowel_lookup.insert(IndicVowel::Au, "V");
        
        // Consonants (SLP1)
        consonant_lookup.insert(IndicConsonant::Ka, "k");
        consonant_lookup.insert(IndicConsonant::Kha, "K");
        consonant_lookup.insert(IndicConsonant::Ga, "g");
        consonant_lookup.insert(IndicConsonant::Gha, "G");
        consonant_lookup.insert(IndicConsonant::Nga, "N");
        consonant_lookup.insert(IndicConsonant::Ca, "c");
        consonant_lookup.insert(IndicConsonant::Cha, "C");
        consonant_lookup.insert(IndicConsonant::Ja, "j");
        consonant_lookup.insert(IndicConsonant::Jha, "J");
        consonant_lookup.insert(IndicConsonant::Nya, "Y");
        consonant_lookup.insert(IndicConsonant::Tta, "w");
        consonant_lookup.insert(IndicConsonant::Ttha, "W");
        consonant_lookup.insert(IndicConsonant::Dda, "q");
        consonant_lookup.insert(IndicConsonant::Ddha, "Q");
        consonant_lookup.insert(IndicConsonant::Nna, "R");
        consonant_lookup.insert(IndicConsonant::Ta, "t");
        consonant_lookup.insert(IndicConsonant::Tha, "T");
        consonant_lookup.insert(IndicConsonant::Da, "d");
        consonant_lookup.insert(IndicConsonant::Dha, "D");
        consonant_lookup.insert(IndicConsonant::Na, "n");
        consonant_lookup.insert(IndicConsonant::Pa, "p");
        consonant_lookup.insert(IndicConsonant::Pha, "P");
        consonant_lookup.insert(IndicConsonant::Ba, "b");
        consonant_lookup.insert(IndicConsonant::Bha, "B");
        consonant_lookup.insert(IndicConsonant::Ma, "m");
        consonant_lookup.insert(IndicConsonant::Ya, "y");
        consonant_lookup.insert(IndicConsonant::Ra, "r");
        consonant_lookup.insert(IndicConsonant::La, "l");
        consonant_lookup.insert(IndicConsonant::Va, "v");
        consonant_lookup.insert(IndicConsonant::ShaPalatal, "S");
        consonant_lookup.insert(IndicConsonant::ShaRetroflex, "z");
        consonant_lookup.insert(IndicConsonant::Sa, "s");
        consonant_lookup.insert(IndicConsonant::Ha, "h");
        consonant_lookup.insert(IndicConsonant::Ksha, "kz");
        consonant_lookup.insert(IndicConsonant::Gnya, "jY");
        
        // Modifiers (SLP1)
        modifier_lookup.insert(IndicModifier::Anusvara, "M");
        modifier_lookup.insert(IndicModifier::Visarga, "H");
        modifier_lookup.insert(IndicModifier::Avagraha, "'");
        
        ScriptGenerator {
            vowel_lookup,
            consonant_lookup,
            modifier_lookup,
            script_rules: ScriptRules {
                inherent_vowel_handling: InherentVowelHandling::ExplicitVowels,
                combination_rules: CombinationRules::Alphabet,
                cluster_rules: ClusterRules::None,
                unknown_pattern: "[?:{original}]".to_string(),
            },
            script_name: "SLP1".to_string(),
        }
    }
    
    /// Create IAST generator
    fn create_iast_generator() -> ScriptGenerator {
        // TODO: Implement comprehensive IAST lookup tables
        // For now, create minimal version
        let vowel_lookup = HashMap::new();
        let consonant_lookup = HashMap::new();
        let modifier_lookup = HashMap::new();
        
        ScriptGenerator {
            vowel_lookup,
            consonant_lookup,
            modifier_lookup,
            script_rules: ScriptRules {
                inherent_vowel_handling: InherentVowelHandling::ExplicitVowels,
                combination_rules: CombinationRules::Alphabet,
                cluster_rules: ClusterRules::None,
                unknown_pattern: "[?:{original}]".to_string(),
            },
            script_name: "IAST".to_string(),
        }
    }
    
    /// Create Tamil generator
    fn create_tamil_generator() -> ScriptGenerator {
        // TODO: Implement Tamil-specific lookup tables
        let vowel_lookup = HashMap::new();
        let consonant_lookup = HashMap::new();
        let modifier_lookup = HashMap::new();
        
        ScriptGenerator {
            vowel_lookup,
            consonant_lookup,
            modifier_lookup,
            script_rules: ScriptRules {
                inherent_vowel_handling: InherentVowelHandling::HasInherent { vowel: "அ" },
                combination_rules: CombinationRules::Abugida,
                cluster_rules: ClusterRules::Virama { virama_char: "்" },
                unknown_pattern: "[?:{original}]".to_string(),
            },
            script_name: "Tamil".to_string(),
        }
    }
    
    /// Create Harvard-Kyoto generator
    fn create_hk_generator() -> ScriptGenerator {
        // TODO: Implement HK lookup tables
        let vowel_lookup = HashMap::new();
        let consonant_lookup = HashMap::new();
        let modifier_lookup = HashMap::new();
        
        ScriptGenerator {
            vowel_lookup,
            consonant_lookup,
            modifier_lookup,
            script_rules: ScriptRules {
                inherent_vowel_handling: InherentVowelHandling::ExplicitVowels,
                combination_rules: CombinationRules::Alphabet,
                cluster_rules: ClusterRules::None,
                unknown_pattern: "[?:{original}]".to_string(),
            },
            script_name: "Harvard-Kyoto".to_string(),
        }
    }
    
    /// Create ISO 15919 generator
    fn create_iso_generator() -> ScriptGenerator {
        // TODO: Implement ISO 15919 lookup tables
        let vowel_lookup = HashMap::new();
        let consonant_lookup = HashMap::new();
        let modifier_lookup = HashMap::new();
        
        ScriptGenerator {
            vowel_lookup,
            consonant_lookup,
            modifier_lookup,
            script_rules: ScriptRules {
                inherent_vowel_handling: InherentVowelHandling::ExplicitVowels,
                combination_rules: CombinationRules::Alphabet,
                cluster_rules: ClusterRules::None,
                unknown_pattern: "[?:{original}]".to_string(),
            },
            script_name: "ISO15919".to_string(),
        }
    }
    
    /// Generate text from phonemes (zero allocation for known phonemes)
    pub fn generate(&mut self, phonemes: &[IndicPhoneme], target_script: &str) -> Result<String, GenerateError> {
        let start_time = std::time::Instant::now();
        
        let generator = self.script_generators.get(target_script)
            .ok_or_else(|| GenerateError::UnknownScript(target_script.to_string()))?;
        
        let mut result = String::new();
        
        for phoneme in phonemes {
            self.stats.total_phonemes_processed += 1;
            
            let output = match phoneme {
                IndicPhoneme::Vowel(vowel) => {
                    if let Some(&repr) = generator.vowel_lookup.get(vowel) {
                        self.stats.enum_lookups_used += 1;
                        repr.to_string()
                    } else {
                        self.stats.unknown_markers_used += 1;
                        format!("[?:{}]", vowel.canonical())
                    }
                },
                
                IndicPhoneme::Consonant(consonant) => {
                    if let Some(&repr) = generator.consonant_lookup.get(consonant) {
                        self.stats.enum_lookups_used += 1;
                        repr.to_string()
                    } else {
                        self.stats.unknown_markers_used += 1;
                        format!("[?:{}]", consonant.canonical())
                    }
                },
                
                IndicPhoneme::Modifier(modifier) => {
                    if let Some(&repr) = generator.modifier_lookup.get(modifier) {
                        self.stats.enum_lookups_used += 1;
                        repr.to_string()
                    } else {
                        self.stats.unknown_markers_used += 1;
                        format!("[?:{}]", modifier.canonical())
                    }
                },
                
                IndicPhoneme::Semantic(annotation) => {
                    self.stats.semantic_resolutions_used += 1;
                    if let Some(resolved) = self.semantic_resolver.resolve(annotation, target_script) {
                        resolved
                    } else {
                        self.stats.unknown_markers_used += 1;
                        format!("[?:semantic]")
                    }
                },
                
                _ => {
                    // Handle other phoneme types (clusters, toned, etc.)
                    self.stats.unknown_markers_used += 1;
                    format!("[?:{}]", phoneme.canonical())
                }
            };
            
            result.push_str(&output);
        }
        
        self.stats.total_generate_time_ns += start_time.elapsed().as_nanos() as u64;
        
        Ok(result)
    }
    
    /// Get performance statistics
    pub fn get_stats(&self) -> &GenerateStats {
        &self.stats
    }
    
    /// Reset performance statistics
    pub fn reset_stats(&mut self) {
        self.stats = GenerateStats::default();
    }
}

impl Default for ZeroAllocGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_devanagari_generation() {
        let mut generator = ZeroAllocGenerator::new();
        
        let phonemes = vec![
            IndicPhoneme::Consonant(IndicConsonant::Ka),
            IndicPhoneme::Consonant(IndicConsonant::Ra),
            IndicPhoneme::Modifier(IndicModifier::Anusvara),
        ];
        
        let result = generator.generate(&phonemes, "Devanagari").unwrap();
        assert_eq!(result, "करं");
        
        let stats = generator.get_stats();
        assert_eq!(stats.total_phonemes_processed, 3);
        assert_eq!(stats.enum_lookups_used, 3);
        assert_eq!(stats.unknown_markers_used, 0);
        assert!(stats.enum_efficiency() > 99.0);
    }
    
    #[test]
    fn test_slp1_generation() {
        let mut generator = ZeroAllocGenerator::new();
        
        let phonemes = vec![
            IndicPhoneme::Consonant(IndicConsonant::Ka),
            IndicPhoneme::Vowel(IndicVowel::Aa),
            IndicPhoneme::Consonant(IndicConsonant::Ra),
            IndicPhoneme::Modifier(IndicModifier::Anusvara),
        ];
        
        let result = generator.generate(&phonemes, "SLP1").unwrap();
        assert_eq!(result, "kArM");
        
        let stats = generator.get_stats();
        assert_eq!(stats.enum_lookups_used, 4);
    }
}