use std::collections::HashMap;
use super::{ScriptConverter, ConverterError};
use crate::modules::hub::HubInput;

/// Telugu script converter
/// 
/// Telugu (తెలుగు) is an Indic script used primarily for the Telugu language.
/// This converter handles Telugu text by converting it directly to Devanagari equivalent,
/// which can then be processed by the hub. The hub handles all complex linguistic rules.
pub struct TeluguConverter {
    telugu_to_deva_map: HashMap<char, char>,
    deva_to_telugu_map: HashMap<char, char>,
}

impl TeluguConverter {
    pub fn new() -> Self {
        let mut telugu_to_deva = HashMap::new();
        
        // Simple character-to-character mapping from Telugu to Devanagari equivalents
        // Let the hub handle all complex linguistic rules (virama processing, etc.)
        
        // Independent vowels (అచ్చులు) → Devanagari equivalents
        telugu_to_deva.insert('అ', 'अ');       // Telugu అ → Devanagari अ
        telugu_to_deva.insert('ఆ', 'आ');       // Telugu ఆ → Devanagari आ
        telugu_to_deva.insert('ఇ', 'इ');       // Telugu ఇ → Devanagari इ
        telugu_to_deva.insert('ఈ', 'ई');       // Telugu ఈ → Devanagari ई
        telugu_to_deva.insert('ఉ', 'उ');       // Telugu ఉ → Devanagari उ
        telugu_to_deva.insert('ఊ', 'ऊ');       // Telugu ఊ → Devanagari ऊ
        telugu_to_deva.insert('ఋ', 'ऋ');       // Telugu ఋ → Devanagari ऋ
        telugu_to_deva.insert('ౠ', 'ॠ');       // Telugu ౠ → Devanagari ॠ
        telugu_to_deva.insert('ఌ', 'ऌ');       // Telugu ఌ → Devanagari ऌ
        telugu_to_deva.insert('ౡ', 'ॡ');       // Telugu ౡ → Devanagari ॡ
        telugu_to_deva.insert('ఎ', 'ए');       // Telugu ఎ → Devanagari ए
        telugu_to_deva.insert('ఏ', 'ए');       // Telugu ఏ → Devanagari ए (long e mapped to short e)
        telugu_to_deva.insert('ఐ', 'ऐ');       // Telugu ఐ → Devanagari ऐ
        telugu_to_deva.insert('ఒ', 'ओ');       // Telugu ఒ → Devanagari ओ
        telugu_to_deva.insert('ఓ', 'ओ');       // Telugu ఓ → Devanagari ओ (long o mapped to short o)
        telugu_to_deva.insert('ఔ', 'औ');       // Telugu ఔ → Devanagari औ
        
        // Vowel diacritics (మాత్రలు) → Devanagari equivalents
        telugu_to_deva.insert('ా', 'ा');       // Telugu ా → Devanagari ा
        telugu_to_deva.insert('ి', 'ि');       // Telugu ి → Devanagari ि
        telugu_to_deva.insert('ీ', 'ी');       // Telugu ీ → Devanagari ी
        telugu_to_deva.insert('ు', 'ु');       // Telugu ు → Devanagari ु
        telugu_to_deva.insert('ూ', 'ू');       // Telugu ూ → Devanagari ू
        telugu_to_deva.insert('ృ', 'ृ');       // Telugu ృ → Devanagari ृ
        telugu_to_deva.insert('ౄ', 'ॄ');       // Telugu ౄ → Devanagari ॄ
        telugu_to_deva.insert('ె', 'े');       // Telugu ె → Devanagari े
        telugu_to_deva.insert('ే', 'े');       // Telugu ే → Devanagari े
        telugu_to_deva.insert('ై', 'ै');       // Telugu ై → Devanagari ै
        telugu_to_deva.insert('ొ', 'ो');       // Telugu ొ → Devanagari ो
        telugu_to_deva.insert('ో', 'ो');       // Telugu ో → Devanagari ो
        telugu_to_deva.insert('ౌ', 'ौ');       // Telugu ౌ → Devanagari ौ
        
        // Consonants (హల్లులు) → Devanagari equivalents
        
        // Velar consonants
        telugu_to_deva.insert('క', 'क');       // Telugu క → Devanagari क
        telugu_to_deva.insert('ఖ', 'ख');       // Telugu ఖ → Devanagari ख
        telugu_to_deva.insert('గ', 'ग');       // Telugu గ → Devanagari ग
        telugu_to_deva.insert('ఘ', 'घ');       // Telugu ఘ → Devanagari घ
        telugu_to_deva.insert('ఙ', 'ङ');       // Telugu ఙ → Devanagari ङ
        
        // Palatal consonants
        telugu_to_deva.insert('చ', 'च');       // Telugu చ → Devanagari च
        telugu_to_deva.insert('ఛ', 'छ');       // Telugu ఛ → Devanagari छ
        telugu_to_deva.insert('జ', 'ज');       // Telugu జ → Devanagari ज
        telugu_to_deva.insert('ఝ', 'झ');       // Telugu ఝ → Devanagari झ
        telugu_to_deva.insert('ఞ', 'ञ');       // Telugu ఞ → Devanagari ञ
        
        // Retroflex consonants
        telugu_to_deva.insert('ట', 'ट');       // Telugu ట → Devanagari ट
        telugu_to_deva.insert('ఠ', 'ठ');       // Telugu ఠ → Devanagari ठ
        telugu_to_deva.insert('డ', 'ड');       // Telugu డ → Devanagari ड
        telugu_to_deva.insert('ఢ', 'ढ');       // Telugu ఢ → Devanagari ढ
        telugu_to_deva.insert('ణ', 'ण');       // Telugu ణ → Devanagari ण
        
        // Dental consonants
        telugu_to_deva.insert('త', 'त');       // Telugu త → Devanagari त
        telugu_to_deva.insert('థ', 'थ');       // Telugu థ → Devanagari थ
        telugu_to_deva.insert('ద', 'द');       // Telugu ద → Devanagari द
        telugu_to_deva.insert('ధ', 'ध');       // Telugu ధ → Devanagari ध
        telugu_to_deva.insert('న', 'न');       // Telugu న → Devanagari न
        
        // Labial consonants
        telugu_to_deva.insert('ప', 'प');       // Telugu ప → Devanagari प
        telugu_to_deva.insert('ఫ', 'फ');       // Telugu ఫ → Devanagari फ
        telugu_to_deva.insert('బ', 'ब');       // Telugu బ → Devanagari ब
        telugu_to_deva.insert('భ', 'भ');       // Telugu భ → Devanagari भ
        telugu_to_deva.insert('మ', 'म');       // Telugu మ → Devanagari म
        
        // Semivowels and liquids
        telugu_to_deva.insert('య', 'य');       // Telugu య → Devanagari य
        telugu_to_deva.insert('ర', 'र');       // Telugu ర → Devanagari र
        telugu_to_deva.insert('ల', 'ल');       // Telugu ల → Devanagari ल
        telugu_to_deva.insert('వ', 'व');       // Telugu వ → Devanagari व
        
        // Sibilants and aspirate
        telugu_to_deva.insert('శ', 'श');       // Telugu శ → Devanagari श
        telugu_to_deva.insert('ష', 'ष');       // Telugu ష → Devanagari ष
        telugu_to_deva.insert('స', 'स');       // Telugu స → Devanagari स
        telugu_to_deva.insert('హ', 'ह');       // Telugu హ → Devanagari ह
        
        // Additional consonants
        telugu_to_deva.insert('ళ', 'ळ');       // Telugu ळ → Devanagari ळ (retroflex l)
        
        // Special marks
        telugu_to_deva.insert('ం', 'ं');       // Telugu ం → Devanagari ं (anusvara)
        telugu_to_deva.insert('ః', 'ः');       // Telugu ః → Devanagari ः (visarga)
        telugu_to_deva.insert('్', '्');       // Telugu ్ → Devanagari ् (halanta/virama)
        telugu_to_deva.insert('ఁ', 'ँ');       // Telugu ఁ → Devanagari ँ (candrabindu)
        
        // Digits
        telugu_to_deva.insert('౦', '०');       // Telugu ౦ → Devanagari ०
        telugu_to_deva.insert('౧', '१');       // Telugu ౧ → Devanagari १
        telugu_to_deva.insert('౨', '२');       // Telugu ౨ → Devanagari २
        telugu_to_deva.insert('౩', '३');       // Telugu ౩ → Devanagari ३
        telugu_to_deva.insert('౪', '४');       // Telugu ౪ → Devanagari ४
        telugu_to_deva.insert('౫', '५');       // Telugu ౫ → Devanagari ५
        telugu_to_deva.insert('౬', '६');       // Telugu ౬ → Devanagari ६
        telugu_to_deva.insert('౭', '७');       // Telugu ౭ → Devanagari ७
        telugu_to_deva.insert('౮', '८');       // Telugu ౮ → Devanagari ८
        telugu_to_deva.insert('౯', '९');       // Telugu ౯ → Devanagari ९
        
        // Build reverse mapping for Devanagari → Telugu conversion
        let mut deva_to_telugu = HashMap::new();
        for (&telugu, &deva) in &telugu_to_deva {
            deva_to_telugu.insert(deva, telugu);
        }

        Self {
            telugu_to_deva_map: telugu_to_deva,
            deva_to_telugu_map: deva_to_telugu,
        }
    }
    
    /// Convert Telugu text to Devanagari format (for hub processing)
    pub fn telugu_to_devanagari(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        
        // Simple character-to-character mapping - let hub handle complex rules
        for ch in input.chars() {
            if ch.is_whitespace() || ch.is_ascii_punctuation() {
                result.push(ch);
            } else if let Some(&deva_char) = self.telugu_to_deva_map.get(&ch) {
                result.push(deva_char);
            } else {
                // Character not in mapping - preserve as-is for mixed content
                result.push(ch);
            }
        }
        
        Ok(result)
    }
    
    /// Convert Devanagari text to Telugu format (reverse conversion)
    pub fn devanagari_to_telugu(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        
        // Simple character-to-character mapping
        for ch in input.chars() {
            if ch.is_whitespace() || ch.is_ascii_punctuation() {
                result.push(ch);
            } else if let Some(&telugu_char) = self.deva_to_telugu_map.get(&ch) {
                result.push(telugu_char);
            } else {
                // Character not in mapping - preserve as-is for mixed content
                result.push(ch);
            }
        }
        
        Ok(result)
    }
}

impl ScriptConverter for TeluguConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "telugu" && script != "te" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Telugu converter only supports 'telugu' or 'te' script".to_string(),
            });
        }
        
        let deva_text = self.telugu_to_devanagari(input)
            .map_err(|e| ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: format!("Telugu to Devanagari conversion failed: {}", e),
            })?;
            
        // Return Devanagari for hub processing, not ISO
        Ok(HubInput::Devanagari(deva_text))
    }
    
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        if script != "telugu" && script != "te" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Telugu converter only supports 'telugu' or 'te' script".to_string(),
            });
        }
        
        match hub_input {
            HubInput::Devanagari(deva_text) => self.devanagari_to_telugu(deva_text),
            HubInput::Iso(_) => Err(ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: "Telugu converter expects Devanagari input, got ISO".to_string(),
            }),
        }
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["telugu", "te"]
    }
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        // Telugu is an Indic script - consonants DO have implicit 'a'
        // But now the hub will handle this complexity
        true
    }
}

impl Default for TeluguConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_telugu_basic_vowels() {
        let converter = TeluguConverter::new();
        
        // Test basic vowels - now should map to Devanagari
        assert_eq!(converter.telugu_to_devanagari("అ").unwrap(), "अ");
        assert_eq!(converter.telugu_to_devanagari("ఆ").unwrap(), "आ");
        assert_eq!(converter.telugu_to_devanagari("ఇ").unwrap(), "इ");
        assert_eq!(converter.telugu_to_devanagari("ఈ").unwrap(), "ई");
        assert_eq!(converter.telugu_to_devanagari("ఉ").unwrap(), "उ");
        assert_eq!(converter.telugu_to_devanagari("ఊ").unwrap(), "ऊ");
    }
    
    #[test]
    fn test_telugu_vocalic_vowels() {
        let converter = TeluguConverter::new();
        
        // Test Telugu → Devanagari vocalic vowels
        assert_eq!(converter.telugu_to_devanagari("ఋ").unwrap(), "ऋ");    // Telugu ఋ → Devanagari ऋ
        assert_eq!(converter.telugu_to_devanagari("ౠ").unwrap(), "ॠ");   // Telugu ౠ → Devanagari ॠ
        assert_eq!(converter.telugu_to_devanagari("ఌ").unwrap(), "ऌ");    // Telugu ఌ → Devanagari ऌ
        assert_eq!(converter.telugu_to_devanagari("ౡ").unwrap(), "ॡ");   // Telugu ౡ → Devanagari ॡ
    }
    
    #[test]
    fn test_telugu_consonants() {
        let converter = TeluguConverter::new();
        
        // Test basic consonants - should map to Devanagari equivalents
        assert_eq!(converter.telugu_to_devanagari("క").unwrap(), "क");
        assert_eq!(converter.telugu_to_devanagari("ఖ").unwrap(), "ख");
        assert_eq!(converter.telugu_to_devanagari("గ").unwrap(), "ग");
        assert_eq!(converter.telugu_to_devanagari("ఘ").unwrap(), "घ");
        assert_eq!(converter.telugu_to_devanagari("ఙ").unwrap(), "ङ");
        
        // Test retroflex consonants
        assert_eq!(converter.telugu_to_devanagari("ట").unwrap(), "ट");
        assert_eq!(converter.telugu_to_devanagari("ఠ").unwrap(), "ठ");
        assert_eq!(converter.telugu_to_devanagari("డ").unwrap(), "ड");
        assert_eq!(converter.telugu_to_devanagari("ఢ").unwrap(), "ढ");
        assert_eq!(converter.telugu_to_devanagari("ణ").unwrap(), "ण");
        
        // Test sibilants
        assert_eq!(converter.telugu_to_devanagari("శ").unwrap(), "श");   // Telugu శ → Devanagari श
        assert_eq!(converter.telugu_to_devanagari("ష").unwrap(), "ष");   // Telugu ష → Devanagari ष
        assert_eq!(converter.telugu_to_devanagari("స").unwrap(), "स");
    }
    
    #[test]
    fn test_telugu_special_marks() {
        let converter = TeluguConverter::new();
        
        // Test special marks
        assert_eq!(converter.telugu_to_devanagari("ం").unwrap(), "ं");
        assert_eq!(converter.telugu_to_devanagari("ః").unwrap(), "ः");
        assert_eq!(converter.telugu_to_devanagari("్").unwrap(), "्");     // halanta/virama
        assert_eq!(converter.telugu_to_devanagari("ఁ").unwrap(), "ँ");
    }
    
    #[test]
    fn test_telugu_dharma_word() {
        let converter = TeluguConverter::new();
        
        // Test the specific word "ధర్మ" (dharma)
        // Should map directly to Devanagari "धर्म" 
        // Hub will handle the virama processing correctly
        let result = converter.telugu_to_devanagari("ధర్మ").unwrap();
        println!("Telugu 'ధర్మ' maps to Devanagari: '{}'", result);
        assert_eq!(result, "धर्म");
    }
    
    #[test]
    fn test_script_converter_interface() {
        let converter = TeluguConverter::new();
        
        // Test the ScriptConverter interface
        assert!(converter.supports_script("telugu"));
        assert!(converter.supports_script("te"));
        assert!(!converter.supports_script("tamil"));
        
        // Test script_has_implicit_a
        assert!(converter.script_has_implicit_a("telugu"));
        assert!(converter.script_has_implicit_a("te"));
        
        let result = converter.to_hub("telugu", "క").unwrap();
        if let HubInput::Devanagari(deva_text) = result {
            assert_eq!(deva_text, "क");
        } else {
            panic!("Expected Devanagari hub input");
        }
    }
}