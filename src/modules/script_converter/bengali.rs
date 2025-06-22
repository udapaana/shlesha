use std::collections::HashMap;
use super::{ScriptConverter, ConverterError};
use crate::modules::hub::HubInput;

/// Bengali script converter
/// 
/// Bengali (বাংলা) is an Indic script used primarily for the Bengali language.
/// This converter handles Bengali text by converting it directly to Devanagari equivalent,
/// which can then be processed by the hub. The hub handles all complex linguistic rules.
pub struct BengaliConverter {
    bengali_to_deva_map: HashMap<char, char>,
    bengali_nukta_to_deva_map: HashMap<String, &'static str>,
    deva_to_bengali_map: HashMap<char, char>,
    deva_nukta_to_bengali_map: HashMap<&'static str, String>,
}

impl BengaliConverter {
    pub fn new() -> Self {
        let mut bengali_to_deva = HashMap::new();
        
        // Simple character-to-character mapping from Bengali to Devanagari equivalents
        // Let the hub handle all complex linguistic rules (virama processing, etc.)
        
        // Independent vowels (স্বরবর্ণ) → Devanagari equivalents
        bengali_to_deva.insert('অ', 'अ');       // Bengali অ → Devanagari अ
        bengali_to_deva.insert('আ', 'आ');       // Bengali আ → Devanagari आ
        bengali_to_deva.insert('ই', 'इ');       // Bengali ই → Devanagari इ
        bengali_to_deva.insert('ঈ', 'ई');       // Bengali ঈ → Devanagari ई
        bengali_to_deva.insert('উ', 'उ');       // Bengali উ → Devanagari उ
        bengali_to_deva.insert('ঊ', 'ऊ');       // Bengali ঊ → Devanagari ऊ
        bengali_to_deva.insert('ঋ', 'ऋ');       // Bengali ঋ → Devanagari ऋ
        bengali_to_deva.insert('ৠ', 'ॠ');       // Bengali ৠ → Devanagari ॠ
        bengali_to_deva.insert('ঌ', 'ऌ');       // Bengali ঌ → Devanagari ऌ
        bengali_to_deva.insert('ৡ', 'ॡ');       // Bengali ৡ → Devanagari ॡ
        bengali_to_deva.insert('এ', 'ए');       // Bengali এ → Devanagari ए
        bengali_to_deva.insert('ঐ', 'ऐ');       // Bengali ঐ → Devanagari ऐ
        bengali_to_deva.insert('ও', 'ओ');       // Bengali ও → Devanagari ओ
        bengali_to_deva.insert('ঔ', 'औ');       // Bengali ঔ → Devanagari औ
        
        // Vowel diacritics (মাত্রা) → Devanagari equivalents
        bengali_to_deva.insert('া', 'ा');       // Bengali া → Devanagari ा
        bengali_to_deva.insert('ি', 'ि');       // Bengali ি → Devanagari ि
        bengali_to_deva.insert('ী', 'ी');       // Bengali ী → Devanagari ी
        bengali_to_deva.insert('ু', 'ु');       // Bengali ু → Devanagari ु
        bengali_to_deva.insert('ূ', 'ू');       // Bengali ূ → Devanagari ू
        bengali_to_deva.insert('ৃ', 'ृ');       // Bengali ৃ → Devanagari ृ
        bengali_to_deva.insert('ৄ', 'ॄ');       // Bengali ৄ → Devanagari ॄ
        bengali_to_deva.insert('ৢ', 'ॢ');       // Bengali ৢ → Devanagari ॢ
        bengali_to_deva.insert('ৣ', 'ॣ');       // Bengali ৣ → Devanagari ॣ
        bengali_to_deva.insert('ে', 'े');       // Bengali ে → Devanagari े
        bengali_to_deva.insert('ৈ', 'ै');       // Bengali ৈ → Devanagari ै
        bengali_to_deva.insert('ো', 'ो');       // Bengali ো → Devanagari ो
        bengali_to_deva.insert('ৌ', 'ौ');       // Bengali ৌ → Devanagari ौ
        
        // Consonants (ব্যঞ্জনবর্ণ) → Devanagari equivalents
        // Velar consonants
        bengali_to_deva.insert('ক', 'क');       // Bengali ক → Devanagari क
        bengali_to_deva.insert('খ', 'ख');       // Bengali খ → Devanagari ख
        bengali_to_deva.insert('গ', 'ग');       // Bengali গ → Devanagari ग
        bengali_to_deva.insert('ঘ', 'घ');       // Bengali ঘ → Devanagari घ
        bengali_to_deva.insert('ঙ', 'ङ');       // Bengali ঙ → Devanagari ङ
        
        // Palatal consonants
        bengali_to_deva.insert('চ', 'च');       // Bengali চ → Devanagari च
        bengali_to_deva.insert('ছ', 'छ');       // Bengali ছ → Devanagari छ
        bengali_to_deva.insert('জ', 'ज');       // Bengali জ → Devanagari ज
        bengali_to_deva.insert('ঝ', 'झ');       // Bengali ঝ → Devanagari झ
        bengali_to_deva.insert('ঞ', 'ञ');       // Bengali ঞ → Devanagari ञ
        
        // Retroflex consonants
        bengali_to_deva.insert('ট', 'ट');       // Bengali ট → Devanagari ट
        bengali_to_deva.insert('ঠ', 'ठ');       // Bengali ঠ → Devanagari ठ
        bengali_to_deva.insert('ড', 'ड');       // Bengali ড → Devanagari ड
        bengali_to_deva.insert('ঢ', 'ढ');       // Bengali ঢ → Devanagari ढ
        bengali_to_deva.insert('ণ', 'ण');       // Bengali ণ → Devanagari ण
        
        // Dental consonants
        bengali_to_deva.insert('ত', 'त');       // Bengali ত → Devanagari त
        bengali_to_deva.insert('থ', 'थ');       // Bengali থ → Devanagari थ
        bengali_to_deva.insert('দ', 'द');       // Bengali দ → Devanagari द
        bengali_to_deva.insert('ধ', 'ध');       // Bengali ধ → Devanagari ध
        bengali_to_deva.insert('ন', 'न');       // Bengali ন → Devanagari न
        
        // Labial consonants
        bengali_to_deva.insert('প', 'प');       // Bengali প → Devanagari प
        bengali_to_deva.insert('ফ', 'फ');       // Bengali ফ → Devanagari फ
        bengali_to_deva.insert('ব', 'ब');       // Bengali ব → Devanagari ब
        bengali_to_deva.insert('ভ', 'भ');       // Bengali ভ → Devanagari भ
        bengali_to_deva.insert('ম', 'म');       // Bengali ম → Devanagari म
        
        // Semivowels and liquids
        bengali_to_deva.insert('য', 'य');       // Bengali য → Devanagari य
        bengali_to_deva.insert('র', 'र');       // Bengali র → Devanagari र
        bengali_to_deva.insert('ল', 'ल');       // Bengali ল → Devanagari ल
        bengali_to_deva.insert('ব', 'व');       // Bengali ব → Devanagari व (duplicate mapping, but contextual)
        
        // Sibilants and aspirate
        bengali_to_deva.insert('শ', 'श');       // Bengali শ → Devanagari श
        bengali_to_deva.insert('ষ', 'ष');       // Bengali ষ → Devanagari ष
        bengali_to_deva.insert('স', 'स');       // Bengali স → Devanagari स
        bengali_to_deva.insert('হ', 'ह');       // Bengali হ → Devanagari ह
        
        // Special marks
        bengali_to_deva.insert('ং', 'ं');       // Bengali ং → Devanagari ं (anusvara)
        bengali_to_deva.insert('ঃ', 'ः');       // Bengali ঃ → Devanagari ः (visarga)
        bengali_to_deva.insert('্', '्');       // Bengali ্ → Devanagari ् (virama)
        bengali_to_deva.insert('ঽ', 'ऽ');       // Bengali ঽ → Devanagari ऽ (avagraha)
        
        // Digits
        bengali_to_deva.insert('০', '०');       // Bengali ০ → Devanagari ०
        bengali_to_deva.insert('১', '१');       // Bengali ১ → Devanagari १
        bengali_to_deva.insert('২', '२');       // Bengali ২ → Devanagari २
        bengali_to_deva.insert('৩', '३');       // Bengali ৩ → Devanagari ३
        bengali_to_deva.insert('৪', '४');       // Bengali ৪ → Devanagari ४
        bengali_to_deva.insert('৫', '५');       // Bengali ৫ → Devanagari ५
        bengali_to_deva.insert('৬', '६');       // Bengali ৬ → Devanagari ६
        bengali_to_deva.insert('৭', '७');       // Bengali ৭ → Devanagari ७
        bengali_to_deva.insert('৮', '८');       // Bengali ৮ → Devanagari ८
        bengali_to_deva.insert('৯', '९');       // Bengali ৯ → Devanagari ९
        
        // Nukta characters (special 2-character sequences) - keep as strings for now
        let mut bengali_nukta_to_deva = HashMap::new();
        bengali_nukta_to_deva.insert("ড়".to_string(), "ड़");  // Bengali ড় → Devanagari ड़
        bengali_nukta_to_deva.insert("ঢ়".to_string(), "ढ़");  // Bengali ঢ় → Devanagari ढ़
        bengali_nukta_to_deva.insert("য়".to_string(), "य़");  // Bengali য় → Devanagari य़
        
        // Build reverse mapping for Devanagari → Bengali conversion
        let mut deva_to_bengali = HashMap::new();
        for (&ben, &deva) in &bengali_to_deva {
            deva_to_bengali.insert(deva, ben);
        }
        
        // Build reverse nukta mapping
        let mut deva_nukta_to_bengali = HashMap::new();
        for (ben_str, &deva_str) in &bengali_nukta_to_deva {
            deva_nukta_to_bengali.insert(deva_str, ben_str.clone());
        }

        Self {
            bengali_to_deva_map: bengali_to_deva,
            bengali_nukta_to_deva_map: bengali_nukta_to_deva,
            deva_to_bengali_map: deva_to_bengali,
            deva_nukta_to_bengali_map: deva_nukta_to_bengali,
        }
    }
    
    /// Convert Bengali text to Devanagari format (for hub processing)
    pub fn bengali_to_devanagari(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            let ch = chars[i];
            
            if ch.is_whitespace() || ch.is_ascii_punctuation() {
                result.push(ch);
                i += 1;
                continue;
            }
            
            // Check for nukta characters (2-char sequences)
            if i + 1 < chars.len() {
                let two_char: String = chars[i..i+2].iter().collect();
                if let Some(&deva_str) = self.bengali_nukta_to_deva_map.get(&two_char) {
                    result.push_str(deva_str);
                    i += 2;
                    continue;
                }
            }
            
            // Check single character mapping
            if let Some(&deva_char) = self.bengali_to_deva_map.get(&ch) {
                result.push(deva_char);
            } else {
                // Character not in mapping - preserve as-is for mixed content
                result.push(ch);
            }
            
            i += 1;
        }
        
        Ok(result)
    }
    
    /// Convert Devanagari text to Bengali format (reverse conversion)
    pub fn devanagari_to_bengali(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            let ch = chars[i];
            
            if ch.is_whitespace() || ch.is_ascii_punctuation() {
                result.push(ch);
                i += 1;
                continue;
            }
            
            // Check for nukta characters (2-char sequences)
            if i + 1 < chars.len() {
                let two_char: String = chars[i..i+2].iter().collect();
                if let Some(ben_str) = self.deva_nukta_to_bengali_map.get(two_char.as_str()) {
                    result.push_str(ben_str);
                    i += 2;
                    continue;
                }
            }
            
            // Check single character mapping
            if let Some(&ben_char) = self.deva_to_bengali_map.get(&ch) {
                result.push(ben_char);
            } else {
                // Character not in mapping - preserve as-is for mixed content
                result.push(ch);
            }
            
            i += 1;
        }
        
        Ok(result)
    }
}

impl ScriptConverter for BengaliConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "bengali" && script != "bn" && script != "bangla" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Bengali converter only supports 'bengali', 'bn', or 'bangla' script".to_string(),
            });
        }
        
        let deva_text = self.bengali_to_devanagari(input)
            .map_err(|e| ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: format!("Bengali to Devanagari conversion failed: {}", e),
            })?;
            
        // Return Devanagari for hub processing, not ISO
        Ok(HubInput::Devanagari(deva_text))
    }
    
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        if script != "bengali" && script != "bn" && script != "bangla" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Bengali converter only supports 'bengali', 'bn', or 'bangla' script".to_string(),
            });
        }
        
        match hub_input {
            HubInput::Devanagari(deva_text) => self.devanagari_to_bengali(deva_text),
            HubInput::Iso(_) => Err(ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: "Bengali converter expects Devanagari input, got ISO".to_string(),
            }),
        }
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["bengali", "bn", "bangla"]
    }
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        // Bengali is an Indic script - consonants DO have implicit 'a'
        // But now the hub will handle this complexity
        true
    }
}

impl Default for BengaliConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bengali_basic_vowels() {
        let converter = BengaliConverter::new();
        
        // Test basic vowels - now should map to Devanagari
        assert_eq!(converter.bengali_to_devanagari("অ").unwrap(), "अ");
        assert_eq!(converter.bengali_to_devanagari("আ").unwrap(), "आ");
        assert_eq!(converter.bengali_to_devanagari("ই").unwrap(), "इ");
        assert_eq!(converter.bengali_to_devanagari("ঈ").unwrap(), "ई");
        assert_eq!(converter.bengali_to_devanagari("উ").unwrap(), "उ");
        assert_eq!(converter.bengali_to_devanagari("ঊ").unwrap(), "ऊ");
    }
    
    #[test]
    fn test_bengali_consonants() {
        let converter = BengaliConverter::new();
        
        // Test basic consonants - should map to Devanagari equivalents
        assert_eq!(converter.bengali_to_devanagari("ক").unwrap(), "क");
        assert_eq!(converter.bengali_to_devanagari("খ").unwrap(), "ख");
        assert_eq!(converter.bengali_to_devanagari("গ").unwrap(), "ग");
        assert_eq!(converter.bengali_to_devanagari("ঘ").unwrap(), "घ");
        assert_eq!(converter.bengali_to_devanagari("ঙ").unwrap(), "ङ");
    }
    
    #[test]
    fn test_bengali_dharma_word() {
        let converter = BengaliConverter::new();
        
        // Test the specific word "ধর্ম" (dharma)
        // Should map directly to Devanagari "धर्म" 
        // Hub will handle the virama processing correctly
        let result = converter.bengali_to_devanagari("ধর্ম").unwrap();
        println!("Bengali 'ধর্ম' maps to Devanagari: '{}'", result);
        assert_eq!(result, "धर्म");
    }
    
    #[test]
    fn test_bengali_nukta_characters() {
        let converter = BengaliConverter::new();
        
        // Test Bengali nukta characters
        assert_eq!(converter.bengali_to_devanagari("ড়").unwrap(), "ड़");
        assert_eq!(converter.bengali_to_devanagari("ঢ়").unwrap(), "ढ़");
        assert_eq!(converter.bengali_to_devanagari("য়").unwrap(), "य़");
    }
    
    #[test]
    fn test_script_converter_interface() {
        let converter = BengaliConverter::new();
        
        // Test the ScriptConverter interface
        assert!(converter.supports_script("bengali"));
        assert!(converter.supports_script("bn"));
        assert!(converter.supports_script("bangla"));
        assert!(!converter.supports_script("hindi"));
        
        // Test script_has_implicit_a
        assert!(converter.script_has_implicit_a("bengali"));
        assert!(converter.script_has_implicit_a("bn"));
        
        let result = converter.to_hub("bengali", "ক").unwrap();
        if let HubInput::Devanagari(deva_text) = result {
            assert_eq!(deva_text, "क");
        } else {
            panic!("Expected Devanagari hub input");
        }
    }
}