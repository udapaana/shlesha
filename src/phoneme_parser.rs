use std::collections::HashMap;
use crate::indic_phoneme::{IndicPhoneme, IndicPhonemeRegistry, IndicConsonant};
use crate::ir::{IR, AbugidaIR, AlphabetIR, Element, ElementType, PropertyValue};
use crate::schema_parser::{Schema, ScriptType};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PhonemeParseError {
    #[error("Unknown script: {0}")]
    UnknownScript(String),
    
    #[error("Unsupported character sequence: {0}")]
    UnsupportedSequence(String),
    
    #[error("Invalid phoneme combination: {0}")]
    InvalidCombination(String),
    
    #[error("Schema error: {0}")]
    SchemaError(String),
}

/// High-performance parser that converts text directly to phonemes, then to appropriate IR
pub struct PhonemeParser {
    /// Pre-compiled phoneme lookup registry
    phoneme_registry: IndicPhonemeRegistry,
    
    /// Schema information for script types
    script_schemas: HashMap<String, Schema>,
    
    /// Performance statistics
    pub stats: PhonemeParseStats,
}

#[derive(Debug, Default)]
pub struct PhonemeParseStats {
    pub total_chars_processed: usize,
    pub enum_phonemes_used: usize,      // Fast path: 2-byte enum variants
    pub extension_phonemes_used: usize,  // Slow path: string-based extensions
    pub string_allocations: usize,       // Number of string allocations needed
    pub total_parse_time_ns: u64,        // Total parsing time in nanoseconds
}

impl PhonemeParseStats {
    pub fn allocation_efficiency(&self) -> f64 {
        if self.total_chars_processed == 0 { return 100.0; }
        (self.enum_phonemes_used as f64 / self.total_chars_processed as f64) * 100.0
    }
    
    pub fn avg_parse_time_per_char_ns(&self) -> f64 {
        if self.total_chars_processed == 0 { return 0.0; }
        self.total_parse_time_ns as f64 / self.total_chars_processed as f64
    }
}

impl PhonemeParser {
    pub fn new() -> Self {
        Self {
            phoneme_registry: IndicPhonemeRegistry::new(),
            script_schemas: HashMap::new(),
            stats: PhonemeParseStats::default(),
        }
    }
    
    /// Load a schema for a specific script
    pub fn load_schema(&mut self, schema: Schema) {
        self.script_schemas.insert(schema.name.clone(), schema);
    }
    
    /// Parse text to phonemes, then convert to appropriate IR type
    pub fn parse_to_ir(&mut self, text: &str, script_name: &str) -> Result<IR, PhonemeParseError> {
        let start_time = std::time::Instant::now();
        
        // Get schema for script type determination - clone to avoid borrow conflicts
        let script_type = self.script_schemas.get(script_name)
            .ok_or_else(|| PhonemeParseError::UnknownScript(script_name.to_string()))?
            .script_type.clone();
        
        // Parse text to phonemes (zero-allocation for known phonemes)
        let phonemes = self.parse_to_phonemes(text, script_name)?;
        
        // Convert phonemes to appropriate IR based on script type
        let ir = match script_type {
            ScriptType::Abugida => {
                IR::Abugida(self.phonemes_to_abugida_ir(&phonemes, script_name)?)
            },
            ScriptType::Alphabet => {
                IR::Alphabet(self.phonemes_to_alphabet_ir(&phonemes, script_name)?)
            },
        };
        
        // Update statistics
        self.stats.total_parse_time_ns += start_time.elapsed().as_nanos() as u64;
        
        Ok(ir)
    }
    
    /// Parse text directly to phonemes (the zero-allocation fast path)
    pub fn parse_to_phonemes(&mut self, text: &str, script_name: &str) -> Result<Vec<IndicPhoneme>, PhonemeParseError> {
        let mut phonemes = Vec::new();
        let mut chars = text.chars().peekable();
        
        while let Some(ch) = chars.next() {
            self.stats.total_chars_processed += 1;
            
            // Try fast enum-based lookup first
            let phoneme = match script_name {
                "Devanagari" => {
                    if let Some(phoneme) = self.phoneme_registry.lookup_devanagari(ch) {
                        self.stats.enum_phonemes_used += 1;
                        phoneme.clone()
                    } else {
                        // Handle conjuncts and compound characters
                        self.handle_devanagari_compound(&mut chars, ch)?
                    }
                },
                "Tamil" => {
                    if let Some(phoneme) = self.phoneme_registry.lookup_tamil(ch) {
                        self.stats.enum_phonemes_used += 1;
                        phoneme.clone()
                    } else {
                        self.handle_tamil_compound(&mut chars, ch)?
                    }
                },
                "IAST" => {
                    // For IAST, we need to look ahead for multi-character sequences
                    self.handle_iast_sequence(&mut chars, ch)?
                },
                _ => {
                    // Fallback to semantic annotation
                    self.create_semantic_phoneme(ch, script_name)
                }
            };
            
            phonemes.push(phoneme);
        }
        
        Ok(phonemes)
    }
    
    /// Handle Devanagari compound characters and conjuncts
    fn handle_devanagari_compound(&mut self, chars: &mut std::iter::Peekable<std::str::Chars>, ch: char) -> Result<IndicPhoneme, PhonemeParseError> {
        // Check for common conjuncts
        if ch == 'क' && chars.peek() == Some(&'्') {
            chars.next(); // consume virama
            if chars.peek() == Some(&'ष') {
                chars.next(); // consume ष
                self.stats.enum_phonemes_used += 1;
                return Ok(IndicPhoneme::Consonant(IndicConsonant::Ksha));
            }
        }
        
        if ch == 'ज' && chars.peek() == Some(&'्') {
            chars.next(); // consume virama
            if chars.peek() == Some(&'ञ') {
                chars.next(); // consume ञ
                self.stats.enum_phonemes_used += 1;
                return Ok(IndicPhoneme::Consonant(IndicConsonant::Gnya));
            }
        }
        
        // Fallback to semantic annotation
        Ok(self.create_semantic_phoneme(ch, "Devanagari"))
    }
    
    /// Handle Tamil compound characters
    fn handle_tamil_compound(&mut self, _chars: &mut std::iter::Peekable<std::str::Chars>, ch: char) -> Result<IndicPhoneme, PhonemeParseError> {
        // TODO: Implement Tamil-specific compound handling
        Ok(self.create_semantic_phoneme(ch, "Tamil"))
    }
    
    /// Handle IAST multi-character sequences
    fn handle_iast_sequence(&mut self, chars: &mut std::iter::Peekable<std::str::Chars>, ch: char) -> Result<IndicPhoneme, PhonemeParseError> {
        // Build potential multi-character sequence
        let mut sequence = String::new();
        sequence.push(ch);
        
        // Look ahead for diacritics and multi-char sequences
        while let Some(&next_ch) = chars.peek() {
            if next_ch.is_ascii_alphabetic() || "āīūṛṝḷḹṅñṭḍṇśṣḥṃ".contains(next_ch) {
                sequence.push(next_ch);
                chars.next();
                
                // Check if we have a valid phoneme
                if let Some(phoneme) = self.phoneme_registry.lookup_iast(&sequence) {
                    self.stats.enum_phonemes_used += 1;
                    return Ok(phoneme.clone());
                }
                
                // Limit sequence length to prevent runaway parsing
                if sequence.len() > 4 {
                    break;
                }
            } else {
                break;
            }
        }
        
        // Try the final sequence
        if let Some(phoneme) = self.phoneme_registry.lookup_iast(&sequence) {
            self.stats.enum_phonemes_used += 1;
            Ok(phoneme.clone())
        } else {
            // Single character fallback
            let single_char = ch.to_string();
            if let Some(phoneme) = self.phoneme_registry.lookup_iast(&single_char) {
                self.stats.enum_phonemes_used += 1;
                Ok(phoneme.clone())
            } else {
                Ok(self.create_semantic_phoneme(ch, "IAST"))
            }
        }
    }
    
    /// Create semantic annotation for unknown characters
    fn create_semantic_phoneme(&mut self, _ch: char, script_name: &str) -> IndicPhoneme {
        self.stats.extension_phonemes_used += 1;
        self.stats.string_allocations += 1; // Only for the source annotation
        
        // Create a semantic annotation that captures the meaning
        let annotation = crate::semantic_annotation::SemanticAnnotation {
            // For unknown characters, we mark them as unknown consonants by default
            // This could be made smarter with character class analysis
            phonetic_meaning: crate::semantic_annotation::PhoneticMeaning::Complex {
                components: vec![], // Unknown internal structure
                combination_type: crate::semantic_annotation::CombinationType::Cluster,
            },
            context: crate::semantic_annotation::PhoneticContext {
                syllable_position: crate::semantic_annotation::SyllablePosition::Onset,
                preceding_sound: None,
                following_sound: None,
                morpheme_boundary: false,
                word_boundary: false,
                register: crate::semantic_annotation::Register::Formal,
            },
            source_language: script_name.to_string(),
            era: None,
        };
        
        IndicPhoneme::Semantic(annotation)
    }
    
    /// Convert phonemes to Abugida IR (for Indic scripts)
    fn phonemes_to_abugida_ir(&self, phonemes: &[IndicPhoneme], script_name: &str) -> Result<AbugidaIR, PhonemeParseError> {
        let mut elements = Vec::new();
        let mut current_consonant: Option<Element> = None;
        
        for phoneme in phonemes {
            match phoneme {
                IndicPhoneme::Consonant(consonant) => {
                    // If we have a pending consonant, push it with inherent vowel
                    if let Some(pending) = current_consonant.take() {
                        elements.push(pending);
                    }
                    
                    // Create new consonant element
                    current_consonant = Some(Element {
                        grapheme: consonant.canonical().to_string(),
                        element_type: ElementType(ElementType::CONSONANT.to_string()),
                        canonical: consonant.canonical().to_string(),
                        properties: HashMap::new(),
                    });
                },
                
                IndicPhoneme::Vowel(vowel) => {
                    if let Some(mut consonant) = current_consonant.take() {
                        // Attach vowel to consonant
                        consonant.properties.insert(
                            "vowel".to_string(), 
                            PropertyValue::String(vowel.canonical().to_string())
                        );
                        elements.push(consonant);
                    } else {
                        // Independent vowel
                        elements.push(Element {
                            grapheme: vowel.canonical().to_string(),
                            element_type: ElementType(ElementType::VOWEL.to_string()),
                            canonical: vowel.canonical().to_string(),
                            properties: HashMap::new(),
                        });
                    }
                },
                
                IndicPhoneme::Modifier(modifier) => {
                    if let Some(current) = current_consonant.as_mut() {
                        // Apply modifier to current consonant
                        current.properties.insert(
                            "modifier".to_string(),
                            PropertyValue::String(modifier.canonical().to_string())
                        );
                    } else if let Some(last_element) = elements.last_mut() {
                        // Apply modifier to last element
                        last_element.properties.insert(
                            "modifier".to_string(),
                            PropertyValue::String(modifier.canonical().to_string())
                        );
                    } else {
                        // Standalone modifier
                        elements.push(Element {
                            grapheme: modifier.canonical().to_string(),
                            element_type: ElementType(ElementType::MODIFIER.to_string()),
                            canonical: modifier.canonical().to_string(),
                            properties: HashMap::new(),
                        });
                    }
                },
                
                IndicPhoneme::Semantic(_annotation) => {
                    // Handle semantic annotation phonemes
                    if let Some(pending) = current_consonant.take() {
                        elements.push(pending);
                    }
                    
                    elements.push(Element {
                        grapheme: "[semantic]".to_string(),
                        element_type: ElementType(ElementType::UNKNOWN.to_string()),
                        canonical: "[semantic]".to_string(),
                        properties: HashMap::new(),
                    });
                },
                
                _ => {
                    // Handle other phoneme types (clusters, toned, etc.)
                    if let Some(pending) = current_consonant.take() {
                        elements.push(pending);
                    }
                    
                    elements.push(Element {
                        grapheme: phoneme.canonical().to_string(),
                        element_type: ElementType(ElementType::UNKNOWN.to_string()),
                        canonical: phoneme.canonical().to_string(),
                        properties: HashMap::new(),
                    });
                }
            }
        }
        
        // Push any remaining consonant with inherent vowel
        if let Some(consonant) = current_consonant {
            elements.push(consonant);
        }
        
        Ok(AbugidaIR {
            script: script_name.to_string(),
            elements,
            metadata: crate::ir::Metadata::default(),
            extensions: indexmap::IndexMap::new(),
        })
    }
    
    /// Convert phonemes to Alphabet IR (for Roman scripts)
    fn phonemes_to_alphabet_ir(&self, phonemes: &[IndicPhoneme], script_name: &str) -> Result<AlphabetIR, PhonemeParseError> {
        let mut elements = Vec::new();
        
        for phoneme in phonemes {
            let element = Element {
                grapheme: phoneme.canonical().to_string(),
                element_type: ElementType(match phoneme {
                    IndicPhoneme::Vowel(_) | IndicPhoneme::VowelNasalized(_) | IndicPhoneme::VowelToned(_, _) => ElementType::VOWEL.to_string(),
                    IndicPhoneme::Consonant(_) | IndicPhoneme::ConsonantPalatalized(_) | IndicPhoneme::ConsonantVelarized(_) | IndicPhoneme::ConsonantCluster(_) => ElementType::CONSONANT.to_string(),
                    IndicPhoneme::Modifier(_) => ElementType::MODIFIER.to_string(),
                    IndicPhoneme::Semantic(_) => ElementType::UNKNOWN.to_string(),
                }),
                canonical: phoneme.canonical().to_string(),
                properties: HashMap::new(),
            };
            
            elements.push(element);
        }
        
        Ok(AlphabetIR {
            scheme: script_name.to_string(),
            elements,
            metadata: crate::ir::Metadata::default(),
            extensions: indexmap::IndexMap::new(),
        })
    }
    
    /// Get performance statistics
    pub fn get_stats(&self) -> &PhonemeParseStats {
        &self.stats
    }
    
    /// Reset performance statistics
    pub fn reset_stats(&mut self) {
        self.stats = PhonemeParseStats::default();
    }
}

impl Default for PhonemeParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema_parser::ScriptType;

    #[test]
    fn test_devanagari_phoneme_parsing() {
        let mut parser = PhonemeParser::new();
        
        // Load a minimal Devanagari schema
        let schema = Schema {
            name: "Devanagari".to_string(),
            script_type: ScriptType::Abugida,
            element_types: HashMap::new(),
            mappings: HashMap::new(),
            extensions: HashMap::new(),
            metadata: None,
        };
        parser.load_schema(schema);
        
        // Test basic parsing
        let result = parser.parse_to_ir("क", "Devanagari").unwrap();
        
        match result {
            IR::Abugida(abugida_ir) => {
                assert_eq!(abugida_ir.script, "Devanagari");
                assert_eq!(abugida_ir.elements.len(), 1);
                assert_eq!(abugida_ir.elements[0].canonical, "ka");
            },
            _ => panic!("Expected Abugida IR"),
        }
        
        // Check statistics
        let stats = parser.get_stats();
        assert_eq!(stats.total_chars_processed, 1);
        assert_eq!(stats.enum_phonemes_used, 1);
        assert_eq!(stats.extension_phonemes_used, 0);
        assert!(stats.allocation_efficiency() > 99.0);
    }
    
    #[test]
    fn test_iast_phoneme_parsing() {
        let mut parser = PhonemeParser::new();
        
        // Load a minimal IAST schema
        let schema = Schema {
            name: "IAST".to_string(),
            script_type: ScriptType::Alphabet,
            element_types: HashMap::new(),
            mappings: HashMap::new(),
            extensions: HashMap::new(),
            metadata: None,
        };
        parser.load_schema(schema);
        
        // Test basic parsing
        let result = parser.parse_to_ir("ka", "IAST").unwrap();
        
        match result {
            IR::Alphabet(alphabet_ir) => {
                assert_eq!(alphabet_ir.scheme, "IAST");
                assert!(alphabet_ir.elements.len() >= 1);
            },
            _ => panic!("Expected Alphabet IR"),
        }
    }
    
    #[test]
    fn test_performance_statistics() {
        let mut parser = PhonemeParser::new();
        
        let schema = Schema {
            name: "Devanagari".to_string(),
            script_type: ScriptType::Abugida,
            element_types: HashMap::new(),
            mappings: HashMap::new(),
            extensions: HashMap::new(),
            metadata: None,
        };
        parser.load_schema(schema);
        
        // Parse known characters (should use enum path)
        parser.parse_to_ir("कखग", "Devanagari").unwrap();
        
        let stats = parser.get_stats();
        assert_eq!(stats.total_chars_processed, 3);
        assert_eq!(stats.enum_phonemes_used, 3);
        assert_eq!(stats.extension_phonemes_used, 0);
        assert_eq!(stats.string_allocations, 0);
    }
}