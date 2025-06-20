use std::collections::HashMap;
use crate::ir::{Element, ElementType, PropertyValue, AbugidaIR, AlphabetIR, IR};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransformError {
    #[error("Cannot transform between incompatible scripts")]
    IncompatibleScripts,
    
    #[error("Missing canonical mapping for element: {0}")]
    MissingCanonicalMapping(String),
    
    #[error("Invalid transformation: {0}")]
    InvalidTransformation(String),
}

pub struct Transformer {
    canonical_mappings: HashMap<String, CanonicalMapping>,
}

#[derive(Debug, Clone)]
struct CanonicalMapping {
    canonical_form: String,
    properties: HashMap<String, PropertyValue>,
}

impl Transformer {
    pub fn new() -> Self {
        Self {
            canonical_mappings: HashMap::new(),
        }
    }
    
    pub fn transform(&self, source: IR, target_type: &str) -> Result<IR, TransformError> {
        match (&source, target_type) {
            (IR::Abugida(abugida), "alphabet") => {
                self.abugida_to_alphabet(abugida)
            }
            (IR::Alphabet(alphabet), "abugida") => {
                self.alphabet_to_abugida(alphabet)
            }
            (IR::Abugida(source_abugida), "abugida") => {
                self.abugida_to_abugida(source_abugida)
            }
            (IR::Alphabet(source_alphabet), "alphabet") => {
                self.alphabet_to_alphabet(source_alphabet)
            }
            _ => Err(TransformError::IncompatibleScripts),
        }
    }
    
    fn abugida_to_alphabet(&self, abugida: &AbugidaIR) -> Result<IR, TransformError> {
        let mut alphabet = AlphabetIR::new("Romanized".to_string());
        
        for (i, element) in abugida.elements.iter().enumerate() {
            match element.element_type.0.as_str() {
                ElementType::CONSONANT => {
                    // In abugida, consonant has inherent 'a' (e.g., "ka")
                    // We need to split it for alphabet representation
                    let has_inherent_vowel = element.get_bool("has_inherent_vowel")
                        .unwrap_or(true);
                    
                    // Check if next element modifies this consonant
                    let next_is_vowel_modifier = i + 1 < abugida.elements.len() && 
                        matches!(abugida.elements[i + 1].element_type.0.as_str(),
                            ElementType::VOWEL_DEPENDENT | ElementType::VIRAMA);
                    
                    if next_is_vowel_modifier {
                        // Just output the pure consonant, the modifier will handle the vowel
                        let pure_consonant = self.extract_pure_consonant(&element.canonical);
                        alphabet.push(Element::new(
                            ElementType::CONSONANT,
                            pure_consonant.clone(),
                            pure_consonant
                        ));
                    } else if has_inherent_vowel {
                        // Output consonant + inherent 'a'
                        let pure_consonant = self.extract_pure_consonant(&element.canonical);
                        alphabet.push(Element::new(
                            ElementType::CONSONANT,
                            pure_consonant.clone(),
                            pure_consonant.clone()
                        ));
                        alphabet.push(Element::new(
                            ElementType::VOWEL,
                            "a",
                            "a"
                        ));
                    } else {
                        // Consonant without inherent vowel
                        let pure_consonant = self.extract_pure_consonant(&element.canonical);
                        alphabet.push(Element::new(
                            ElementType::CONSONANT,
                            pure_consonant.clone(),
                            pure_consonant
                        ));
                    }
                }
                
                ElementType::VOWEL_INDEPENDENT => {
                    alphabet.push(Element::new(
                        ElementType::VOWEL,
                        element.canonical.clone(),
                        element.canonical.clone()
                    ));
                }
                
                ElementType::VOWEL_DEPENDENT => {
                    alphabet.push(Element::new(
                        ElementType::VOWEL,
                        element.canonical.clone(),
                        element.canonical.clone()
                    ));
                }
                
                ElementType::VIRAMA => {
                    // Virama suppresses inherent vowel, already handled above
                }
                
                ElementType::ANUSVARA => {
                    alphabet.push(Element::new(
                        ElementType::MODIFIER,
                        "ṃ",
                        "ṃ"
                    ));
                }
                
                ElementType::VISARGA => {
                    alphabet.push(Element::new(
                        ElementType::MODIFIER,
                        "ḥ",
                        "ḥ"
                    ));
                }
                
                ElementType::WHITESPACE | ElementType::PUNCTUATION => {
                    alphabet.push(element.clone());
                }
                
                _ => {
                    // For other elements, use canonical form
                    alphabet.push(Element::new(
                        element.element_type.0.clone(),
                        element.canonical.clone(),
                        element.canonical.clone()
                    ));
                }
            }
        }
        
        Ok(IR::Alphabet(alphabet))
    }
    
    fn alphabet_to_abugida(&self, alphabet: &AlphabetIR) -> Result<IR, TransformError> {
        let mut abugida = AbugidaIR::new("Devanagari".to_string());
        let mut i = 0;
        let elements = &alphabet.elements;
        
        while i < elements.len() {
            let element = &elements[i];
            
            match element.element_type.0.as_str() {
                ElementType::CONSONANT => {
                    // For alphabet to abugida, we need to look ahead for vowels
                    let consonant_element = Element::new(
                        ElementType::CONSONANT,
                        self.get_abugida_consonant(&element.canonical),
                        format!("{}a", element.canonical) // Abugida consonants have inherent 'a'
                    ).with_property("has_inherent_vowel", PropertyValue::Bool(true));
                    
                    // Check if followed by a vowel
                    if i + 1 < elements.len() {
                        if let Some(next) = elements.get(i + 1) {
                            if next.element_type.0 == ElementType::VOWEL {
                                if next.canonical != "a" {
                                    // Non-'a' vowel: add consonant + dependent vowel
                                    abugida.push(consonant_element);
                                    
                                    let dependent_vowel = Element::new(
                                        ElementType::VOWEL_DEPENDENT,
                                        self.get_dependent_vowel(&next.canonical),
                                        next.canonical.clone()
                                    );
                                    abugida.push(dependent_vowel);
                                    i += 2;
                                    continue;
                                } else {
                                    // 'a' is inherent, just add consonant
                                    abugida.push(consonant_element);
                                    i += 2;
                                    continue;
                                }
                            }
                        }
                    }
                    
                    // No vowel follows, pure consonant needs virama
                    abugida.push(consonant_element);
                    abugida.push(Element::new(
                        ElementType::VIRAMA,
                        "्",
                        ""
                    ));
                }
                
                ElementType::VOWEL => {
                    // Independent vowel
                    abugida.push(Element::new(
                        ElementType::VOWEL_INDEPENDENT,
                        self.get_independent_vowel(&element.canonical),
                        element.canonical.clone()
                    ));
                }
                
                ElementType::MODIFIER => {
                    match element.canonical.as_str() {
                        "ṃ" => abugida.push(Element::new(ElementType::ANUSVARA, "ं", "ṃ")),
                        "ḥ" => abugida.push(Element::new(ElementType::VISARGA, "ः", "ḥ")),
                        _ => abugida.push(element.clone()),
                    }
                }
                
                ElementType::WHITESPACE | ElementType::PUNCTUATION => {
                    abugida.push(element.clone());
                }
                
                _ => {
                    abugida.push(element.clone());
                }
            }
            
            i += 1;
        }
        
        Ok(IR::Abugida(abugida))
    }
    
    fn abugida_to_abugida(&self, source: &AbugidaIR) -> Result<IR, TransformError> {
        // For now, just clone with different script name
        let mut target = AbugidaIR::new("TargetAbugida".to_string());
        
        for element in &source.elements {
            // Transform based on canonical mappings
            target.push(element.clone());
        }
        
        Ok(IR::Abugida(target))
    }
    
    fn alphabet_to_alphabet(&self, source: &AlphabetIR) -> Result<IR, TransformError> {
        // For now, just clone with different scheme name
        let mut target = AlphabetIR::new("TargetAlphabet".to_string());
        
        for element in &source.elements {
            target.push(element.clone());
        }
        
        Ok(IR::Alphabet(target))
    }
    
    // Helper methods for getting script-specific representations
    fn extract_pure_consonant(&self, canonical: &str) -> String {
        // Remove trailing 'a' from canonical form
        if canonical.ends_with('a') && canonical.len() > 1 {
            canonical[..canonical.len() - 1].to_string()
        } else {
            canonical.to_string()
        }
    }
    
    fn decompose_compound_consonant(&self, canonical: &str) -> Vec<String> {
        // Handle common Sanskrit conjuncts by decomposing them into constituent parts
        match canonical {
            "kṣa" => vec!["k".to_string(), "ṣa".to_string()],
            "jña" => vec!["j".to_string(), "ña".to_string()],
            "kṣ" => vec!["k".to_string(), "ṣ".to_string()],
            "jñ" => vec!["j".to_string(), "ñ".to_string()],
            // Add more conjuncts as needed
            _ => {
                // For non-compound consonants, don't decompose at all
                vec![canonical.to_string()]
            }
        }
    }
    
    fn get_abugida_consonant(&self, pure: &str) -> String {
        // Temporary hardcoded mappings - should come from schema
        match pure {
            "k" => "क",
            "kh" => "ख",
            "g" => "ग",
            "gh" => "घ",
            "c" => "च",
            "ch" => "छ",
            "j" => "ज",
            "jh" => "झ",
            "ṭ" => "ट",
            "ṭh" => "ठ",
            "ḍ" => "ड",
            "ḍh" => "ढ",
            "t" => "त",
            "th" => "थ",
            "d" => "द",
            "dh" => "ध",
            "n" => "न",
            "p" => "प",
            "ph" => "फ",
            "b" => "ब",
            "bh" => "भ",
            "m" => "म",
            "y" => "य",
            "r" => "र",
            "l" => "ल",
            "v" => "व",
            "ś" => "श",
            "ṣ" => "ष",
            "s" => "स",
            "h" => "ह",
            _ => pure,
        }.to_string()
    }
    
    fn get_dependent_vowel(&self, vowel: &str) -> String {
        match vowel {
            "ā" => "ा",
            "i" => "ि",
            "ī" => "ी",
            "u" => "ु",
            "ū" => "ू",
            "ṛ" => "ृ",
            "ṝ" => "ॄ",
            "e" => "े",
            "ai" => "ै",
            "o" => "ो",
            "au" => "ौ",
            _ => vowel,
        }.to_string()
    }
    
    fn get_independent_vowel(&self, vowel: &str) -> String {
        match vowel {
            "a" => "अ",
            "ā" => "आ",
            "i" => "इ",
            "ī" => "ई",
            "u" => "उ",
            "ū" => "ऊ",
            "ṛ" => "ऋ",
            "ṝ" => "ॠ",
            "e" => "ए",
            "ai" => "ऐ",
            "o" => "ओ",
            "au" => "औ",
            _ => vowel,
        }.to_string()
    }
}

pub struct TransformerBuilder {
    transformer: Transformer,
}

impl TransformerBuilder {
    pub fn new() -> Self {
        Self {
            transformer: Transformer::new(),
        }
    }
    
    pub fn with_canonical_mappings(mut self, mappings: HashMap<String, CanonicalMapping>) -> Self {
        self.transformer.canonical_mappings = mappings;
        self
    }
    
    pub fn build(self) -> Transformer {
        self.transformer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_consonant_vowel_transformation() {
        let transformer = Transformer::new();
        
        // Create a simple abugida IR with "ka"
        let mut abugida = AbugidaIR::new("Devanagari".to_string());
        abugida.push(Element::new(ElementType::CONSONANT, "क", "ka")
            .with_property("has_inherent_vowel", PropertyValue::Bool(true)));
        
        let result = transformer.transform(IR::Abugida(abugida), "alphabet").unwrap();
        
        match result {
            IR::Alphabet(alphabet) => {
                assert_eq!(alphabet.elements.len(), 2);
                assert_eq!(alphabet.elements[0].canonical, "k");
                assert_eq!(alphabet.elements[1].canonical, "a");
            }
            _ => panic!("Expected Alphabet IR"),
        }
    }
    
    #[test]
    fn test_virama_handling() {
        let transformer = Transformer::new();
        
        // Create abugida IR with "k" (क्)
        let mut abugida = AbugidaIR::new("Devanagari".to_string());
        abugida.push(Element::new(ElementType::CONSONANT, "क", "ka")
            .with_property("has_inherent_vowel", PropertyValue::Bool(true)));
        abugida.push(Element::new(ElementType::VIRAMA, "्", ""));
        
        let result = transformer.transform(IR::Abugida(abugida), "alphabet").unwrap();
        
        match result {
            IR::Alphabet(alphabet) => {
                assert_eq!(alphabet.elements.len(), 1);
                assert_eq!(alphabet.elements[0].canonical, "k");
            }
            _ => panic!("Expected Alphabet IR"),
        }
    }
}