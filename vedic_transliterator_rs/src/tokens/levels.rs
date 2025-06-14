//! Multi-level token architecture for Sanskrit transliteration
//! 
//! This module defines a three-level token system:
//! 1. Phoneme level - Pure consonants and vowels (e.g., K, A, T)
//! 2. Syllable level - Combined into pronounceable units (e.g., KA, TRA)  
//! 3. Surface level - Script-specific representations

/// The level at which tokens are represented
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenLevel {
    /// Pure phonemes: consonants without inherent vowels
    /// "कात्" → [K, AA, T]
    Phoneme,
    
    /// Syllabic units: consonants with vowels
    /// "कात्" → [KAA, T]
    Syllable,
    
    /// Script-specific surface forms
    /// "कात्" → [का, त्]
    Surface,
}

/// A token that knows its representation level
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeveledToken {
    /// The token identifier (e.g., "K", "KA", "क")
    pub token: String,
    
    /// The level this token represents
    pub level: TokenLevel,
    
    /// Original surface form (for round-trip fidelity)
    pub surface_form: Option<String>,
}

impl LeveledToken {
    /// Create a new leveled token
    pub fn new(token: String, level: TokenLevel) -> Self {
        Self {
            token,
            level,
            surface_form: None,
        }
    }
    
    /// Create with surface form preserved
    pub fn with_surface(token: String, level: TokenLevel, surface: String) -> Self {
        Self {
            token,
            level,
            surface_form: Some(surface),
        }
    }
}

/// Metrical weight of a syllable
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyllableWeight {
    /// Light syllable (laghu) - short vowel, no final consonant
    Light,
    /// Heavy syllable (guru) - long vowel or final consonant
    Heavy,
    /// Pluta - extra-long vowel (3+ moras)
    Pluta,
}

/// Represents a syllable boundary in the token stream
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyllableBoundary {
    /// Index in token stream where syllable starts
    pub start: usize,
    
    /// Index where syllable ends (exclusive)
    pub end: usize,
    
    /// Metrical weight for chanda analysis
    pub weight: SyllableWeight,
    
    /// Whether this syllable has an accent marker
    pub has_accent: bool,
    
    /// The type of accent (if any) - udātta, anudātta, svarita
    pub accent_type: Option<String>,
    
    /// Sama gana notation (numbers, symbols)
    pub sama_notation: Option<String>,
}

/// A multi-level token stream that can be viewed at different levels
#[derive(Debug, Clone)]
pub struct MultiLevelTokenStream {
    /// Phoneme-level tokens (most granular)
    pub phonemes: Vec<LeveledToken>,
    
    /// Syllable boundaries in the phoneme stream
    pub syllables: Vec<SyllableBoundary>,
    
    /// Cached syllable-level view
    syllable_tokens: Option<Vec<LeveledToken>>,
    
    /// Cached surface-level view  
    surface_tokens: Option<Vec<LeveledToken>>,
}

impl MultiLevelTokenStream {
    /// Create from phoneme tokens
    pub fn from_phonemes(phonemes: Vec<LeveledToken>) -> Self {
        Self {
            phonemes,
            syllables: Vec::new(),
            syllable_tokens: None,
            surface_tokens: None,
        }
    }
    
    /// Get tokens at specified level
    pub fn tokens_at_level(&mut self, level: TokenLevel) -> &[LeveledToken] {
        match level {
            TokenLevel::Phoneme => &self.phonemes,
            TokenLevel::Syllable => {
                if self.syllable_tokens.is_none() {
                    self.generate_syllable_tokens();
                }
                self.syllable_tokens.as_ref().unwrap()
            },
            TokenLevel::Surface => {
                if self.surface_tokens.is_none() {
                    self.generate_surface_tokens();
                }
                self.surface_tokens.as_ref().unwrap()
            },
        }
    }
    
    /// Generate syllable-level tokens from phonemes
    fn generate_syllable_tokens(&mut self) {
        let transformer = SanskritSyllableTransformer;
        self.syllable_tokens = Some(transformer.phonemes_to_syllables(&self.phonemes));
        self.syllables = transformer.find_syllable_boundaries(&self.phonemes);
    }
    
    /// Generate surface-level tokens
    fn generate_surface_tokens(&mut self) {
        // TODO: Implement syllable → surface transformation
        // For now, placeholder
        self.surface_tokens = Some(self.phonemes.clone());
    }
}

/// Rules for transforming between token levels
pub trait TokenTransformer {
    /// Transform phonemes to syllables
    fn phonemes_to_syllables(&self, phonemes: &[LeveledToken]) -> Vec<LeveledToken>;
    
    /// Transform syllables to surface forms
    fn syllables_to_surface(&self, syllables: &[LeveledToken], target_script: &str) -> Vec<LeveledToken>;
    
    /// Identify syllable boundaries in phoneme stream
    fn find_syllable_boundaries(&self, phonemes: &[LeveledToken]) -> Vec<SyllableBoundary>;
}

/// Sanskrit-specific syllable rules
pub struct SanskritSyllableTransformer;

impl TokenTransformer for SanskritSyllableTransformer {
    fn phonemes_to_syllables(&self, phonemes: &[LeveledToken]) -> Vec<LeveledToken> {
        let mut syllables = Vec::new();
        let mut i = 0;
        
        while i < phonemes.len() {
            // Collect consonant cluster
            let mut consonants = Vec::new();
            while i < phonemes.len() && self.is_consonant(&phonemes[i].token) {
                consonants.push(&phonemes[i]);
                i += 1;
            }
            
            // Must have a vowel (or add inherent 'a')
            if i < phonemes.len() && self.is_vowel(&phonemes[i].token) {
                // Explicit vowel
                let vowel = &phonemes[i];
                i += 1;
                
                // Create syllable token
                let syllable_name = if consonants.is_empty() {
                    vowel.token.clone()
                } else {
                    format!("{}{}", 
                        consonants.iter().map(|c| &c.token[..]).collect::<String>(),
                        vowel.token
                    )
                };
                
                syllables.push(LeveledToken::new(syllable_name, TokenLevel::Syllable));
            } else if !consonants.is_empty() {
                // Consonant cluster at end - add inherent 'a' for each
                for consonant in consonants {
                    let syllable_name = format!("{}A", consonant.token);
                    syllables.push(LeveledToken::new(syllable_name, TokenLevel::Syllable));
                }
            }
        }
        
        syllables
    }
    
    fn syllables_to_surface(&self, syllables: &[LeveledToken], _target_script: &str) -> Vec<LeveledToken> {
        // TODO: Implement based on target script rules
        syllables.to_vec()
    }
    
    fn find_syllable_boundaries(&self, phonemes: &[LeveledToken]) -> Vec<SyllableBoundary> {
        let mut boundaries = Vec::new();
        let mut i = 0;
        
        while i < phonemes.len() {
            let start = i;
            let mut vowel_token: Option<&str> = None;
            let mut has_final_consonant = false;
            
            // Skip consonant cluster
            while i < phonemes.len() && self.is_consonant(&phonemes[i].token) {
                i += 1;
            }
            
            // Include vowel if present
            if i < phonemes.len() && self.is_vowel(&phonemes[i].token) {
                vowel_token = Some(&phonemes[i].token);
                i += 1;
            }
            
            // Include anusvara/visarga if present
            if i < phonemes.len() && self.is_syllable_final(&phonemes[i].token) {
                has_final_consonant = true;
                i += 1;
            }
            
            if i > start {
                // Calculate metrical weight
                let weight = self.calculate_syllable_weight(vowel_token, has_final_consonant);
                
                boundaries.push(SyllableBoundary {
                    start,
                    end: i,
                    weight,
                    has_accent: false,
                    accent_type: None,
                    sama_notation: None,
                });
            }
        }
        
        boundaries
    }
}

impl SanskritSyllableTransformer {
    fn is_consonant(&self, token: &str) -> bool {
        // Pure consonants: K, KH, G, GH, etc.
        matches!(token, 
            "K" | "KH" | "G" | "GH" | "NG" |
            "C" | "CH" | "J" | "JH" | "NY" |
            "TT" | "TTH" | "DD" | "DDH" | "NN" |
            "T" | "TH" | "D" | "DH" | "N" |
            "P" | "PH" | "B" | "BH" | "M" |
            "Y" | "R" | "L" | "LL" | "V" |
            "SH" | "SS" | "S" | "H"
        )
    }
    
    fn is_vowel(&self, token: &str) -> bool {
        matches!(token,
            "A" | "AA" | "I" | "II" | "U" | "UU" |
            "RI" | "RII" | "LI" | "LII" |
            "E" | "AI" | "O" | "AU"
        )
    }
    
    fn is_syllable_final(&self, token: &str) -> bool {
        matches!(token, "ANUSVARA" | "VISARGA" | "M" | "H") // Anusvara, Visarga
    }
    
    /// Calculate metrical weight based on Sanskrit prosody rules
    fn calculate_syllable_weight(&self, vowel: Option<&str>, has_final_consonant: bool) -> SyllableWeight {
        match vowel {
            Some(v) if self.is_long_vowel(v) => SyllableWeight::Heavy,
            Some(v) if self.is_pluta_vowel(v) => SyllableWeight::Pluta,
            Some(_) if has_final_consonant => SyllableWeight::Heavy, // Short vowel + consonant = heavy
            Some(_) => SyllableWeight::Light, // Short vowel alone = light
            None => SyllableWeight::Heavy, // Pure consonant = heavy (rare but possible)
        }
    }
    
    fn is_long_vowel(&self, vowel: &str) -> bool {
        matches!(vowel, "AA" | "II" | "UU" | "RII" | "LII" | "E" | "AI" | "O" | "AU")
    }
    
    fn is_pluta_vowel(&self, _vowel: &str) -> bool {
        // Pluta vowels would need special notation - for now, none
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_syllable_detection() {
        let transformer = SanskritSyllableTransformer;
        
        // Test: "kat" → KA-T
        let phonemes = vec![
            LeveledToken::new("K".to_string(), TokenLevel::Phoneme),
            LeveledToken::new("A".to_string(), TokenLevel::Phoneme),
            LeveledToken::new("T".to_string(), TokenLevel::Phoneme),
        ];
        
        let boundaries = transformer.find_syllable_boundaries(&phonemes);
        assert_eq!(boundaries.len(), 2);
        assert_eq!(boundaries[0].start, 0);
        assert_eq!(boundaries[0].end, 2); // K-A
        assert_eq!(boundaries[1].start, 2);
        assert_eq!(boundaries[1].end, 3); // T
    }
    
    #[test] 
    fn test_conjunct_syllable() {
        let transformer = SanskritSyllableTransformer;
        
        // Test: "kta" → KTA
        let phonemes = vec![
            LeveledToken::new("K".to_string(), TokenLevel::Phoneme),
            LeveledToken::new("T".to_string(), TokenLevel::Phoneme),
            LeveledToken::new("A".to_string(), TokenLevel::Phoneme),
        ];
        
        let syllables = transformer.phonemes_to_syllables(&phonemes);
        assert_eq!(syllables.len(), 1);
        assert_eq!(syllables[0].token, "KTA");
    }
    
    #[test]
    fn test_gni_conjunct() {
        let transformer = SanskritSyllableTransformer;
        
        // Test: "gni" → GNI (single syllable)
        let phonemes = vec![
            LeveledToken::new("G".to_string(), TokenLevel::Phoneme),
            LeveledToken::new("N".to_string(), TokenLevel::Phoneme),
            LeveledToken::new("I".to_string(), TokenLevel::Phoneme),
        ];
        
        let syllables = transformer.phonemes_to_syllables(&phonemes);
        assert_eq!(syllables.len(), 1);
        assert_eq!(syllables[0].token, "GNI");
    }
    
    #[test]
    fn test_metrical_analysis() {
        let transformer = SanskritSyllableTransformer;
        
        // Test metrical weights: "agni" → A-GNI (light-light)
        let phonemes = vec![
            LeveledToken::new("A".to_string(), TokenLevel::Phoneme),   // Short vowel alone = light
            LeveledToken::new("G".to_string(), TokenLevel::Phoneme),
            LeveledToken::new("N".to_string(), TokenLevel::Phoneme),   
            LeveledToken::new("I".to_string(), TokenLevel::Phoneme),   // Short vowel alone = light
        ];
        
        let boundaries = transformer.find_syllable_boundaries(&phonemes);
        assert_eq!(boundaries.len(), 2);
        
        // First syllable: A (light)
        assert_eq!(boundaries[0].weight, SyllableWeight::Light);
        
        // Second syllable: GNI (light - short vowel, no final consonant)
        assert_eq!(boundaries[1].weight, SyllableWeight::Light);
    }
    
    #[test]
    fn test_heavy_syllables() {
        let transformer = SanskritSyllableTransformer;
        
        // Test: "kāra" → KAA-RA (heavy-light)
        let phonemes = vec![
            LeveledToken::new("K".to_string(), TokenLevel::Phoneme),
            LeveledToken::new("AA".to_string(), TokenLevel::Phoneme),  // Long vowel = heavy
            LeveledToken::new("R".to_string(), TokenLevel::Phoneme),
            LeveledToken::new("A".to_string(), TokenLevel::Phoneme),   // Short vowel = light
        ];
        
        let boundaries = transformer.find_syllable_boundaries(&phonemes);
        assert_eq!(boundaries.len(), 2);
        
        assert_eq!(boundaries[0].weight, SyllableWeight::Heavy); // KAA
        assert_eq!(boundaries[1].weight, SyllableWeight::Light); // RA
    }
}