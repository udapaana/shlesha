use std::collections::HashMap;
use super::{ScriptConverter, ConverterError};
use crate::modules::hub::HubInput;

/// Gujarati script converter
/// 
/// Gujarati (ગુજરાતી) is an Indic script used primarily for the Gujarati language.
/// This converter handles Gujarati text by converting it directly to Devanagari equivalent,
/// which can then be processed by the hub. The hub handles all complex linguistic rules.
pub struct GujaratiConverter {
    gujarati_to_deva_map: HashMap<char, char>,
    deva_to_gujarati_map: HashMap<char, char>,
}

impl GujaratiConverter {
    pub fn new() -> Self {
        let mut gujarati_to_deva = HashMap::new();
        
        // Simple character-to-character mapping from Gujarati to Devanagari equivalents
        // Let the hub handle all complex linguistic rules (virama processing, etc.)
        
        // Independent vowels (સ્વર) → Devanagari equivalents
        gujarati_to_deva.insert('અ', 'अ');       // Gujarati અ → Devanagari अ
        gujarati_to_deva.insert('આ', 'आ');       // Gujarati આ → Devanagari आ
        gujarati_to_deva.insert('ઇ', 'इ');       // Gujarati ઇ → Devanagari इ
        gujarati_to_deva.insert('ઈ', 'ई');       // Gujarati ઈ → Devanagari ई
        gujarati_to_deva.insert('ઉ', 'उ');       // Gujarati ઉ → Devanagari उ
        gujarati_to_deva.insert('ઊ', 'ऊ');       // Gujarati ઊ → Devanagari ऊ
        gujarati_to_deva.insert('ઋ', 'ऋ');       // Gujarati ઋ → Devanagari ऋ
        gujarati_to_deva.insert('ૠ', 'ॠ');       // Gujarati ૠ → Devanagari ॠ
        gujarati_to_deva.insert('ઌ', 'ऌ');       // Gujarati ઌ → Devanagari ऌ
        gujarati_to_deva.insert('ૡ', 'ॡ');       // Gujarati ૡ → Devanagari ॡ
        gujarati_to_deva.insert('એ', 'ए');       // Gujarati એ → Devanagari ए
        gujarati_to_deva.insert('ઐ', 'ऐ');       // Gujarati ઐ → Devanagari ऐ
        gujarati_to_deva.insert('ઓ', 'ओ');       // Gujarati ઓ → Devanagari ओ
        gujarati_to_deva.insert('ઔ', 'औ');       // Gujarati ઔ → Devanagari औ
        
        // Vowel diacritics (માત્રા) → Devanagari equivalents
        gujarati_to_deva.insert('ા', 'ा');       // Gujarati ા → Devanagari ा
        gujarati_to_deva.insert('િ', 'ि');       // Gujarati િ → Devanagari ि
        gujarati_to_deva.insert('ી', 'ी');       // Gujarati ી → Devanagari ी
        gujarati_to_deva.insert('ુ', 'ु');       // Gujarati ુ → Devanagari ु
        gujarati_to_deva.insert('ૂ', 'ू');       // Gujarati ૂ → Devanagari ू
        gujarati_to_deva.insert('ૃ', 'ृ');       // Gujarati ૃ → Devanagari ृ
        gujarati_to_deva.insert('ૄ', 'ॄ');       // Gujarati ૄ → Devanagari ॄ
        gujarati_to_deva.insert('ૢ', 'ॢ');       // Gujarati ૢ → Devanagari ॢ
        gujarati_to_deva.insert('ૣ', 'ॣ');       // Gujarati ૣ → Devanagari ॣ
        gujarati_to_deva.insert('ે', 'े');       // Gujarati ે → Devanagari े
        gujarati_to_deva.insert('ૈ', 'ै');       // Gujarati ૈ → Devanagari ै
        gujarati_to_deva.insert('ો', 'ो');       // Gujarati ો → Devanagari ो
        gujarati_to_deva.insert('ૌ', 'ौ');       // Gujarati ૌ → Devanagari ौ
        
        // Consonants (વ્યંજન) → Devanagari equivalents
        // Velar consonants
        gujarati_to_deva.insert('ક', 'क');       // Gujarati ક → Devanagari क
        gujarati_to_deva.insert('ખ', 'ख');       // Gujarati ખ → Devanagari ख
        gujarati_to_deva.insert('ગ', 'ग');       // Gujarati ગ → Devanagari ग
        gujarati_to_deva.insert('ઘ', 'घ');       // Gujarati ઘ → Devanagari घ
        gujarati_to_deva.insert('ઙ', 'ङ');       // Gujarati ઙ → Devanagari ङ
        
        // Palatal consonants
        gujarati_to_deva.insert('ચ', 'च');       // Gujarati ચ → Devanagari च
        gujarati_to_deva.insert('છ', 'छ');       // Gujarati છ → Devanagari छ
        gujarati_to_deva.insert('જ', 'ज');       // Gujarati જ → Devanagari ज
        gujarati_to_deva.insert('ઝ', 'झ');       // Gujarati ઝ → Devanagari झ
        gujarati_to_deva.insert('ઞ', 'ञ');       // Gujarati ઞ → Devanagari ञ
        
        // Retroflex consonants
        gujarati_to_deva.insert('ટ', 'ट');       // Gujarati ટ → Devanagari ट
        gujarati_to_deva.insert('ઠ', 'ठ');       // Gujarati ઠ → Devanagari ठ
        gujarati_to_deva.insert('ડ', 'ड');       // Gujarati ડ → Devanagari ड
        gujarati_to_deva.insert('ઢ', 'ढ');       // Gujarati ઢ → Devanagari ढ
        gujarati_to_deva.insert('ણ', 'ण');       // Gujarati ણ → Devanagari ण
        
        // Dental consonants
        gujarati_to_deva.insert('ત', 'त');       // Gujarati ત → Devanagari त
        gujarati_to_deva.insert('થ', 'थ');       // Gujarati થ → Devanagari थ
        gujarati_to_deva.insert('દ', 'द');       // Gujarati દ → Devanagari द
        gujarati_to_deva.insert('ધ', 'ध');       // Gujarati ધ → Devanagari ध
        gujarati_to_deva.insert('ન', 'न');       // Gujarati ન → Devanagari न
        
        // Labial consonants
        gujarati_to_deva.insert('પ', 'प');       // Gujarati પ → Devanagari प
        gujarati_to_deva.insert('ફ', 'फ');       // Gujarati ફ → Devanagari फ
        gujarati_to_deva.insert('બ', 'ब');       // Gujarati બ → Devanagari ब
        gujarati_to_deva.insert('ભ', 'भ');       // Gujarati ભ → Devanagari भ
        gujarati_to_deva.insert('મ', 'म');       // Gujarati મ → Devanagari म
        
        // Semivowels and liquids
        gujarati_to_deva.insert('ય', 'य');       // Gujarati ય → Devanagari य
        gujarati_to_deva.insert('ર', 'र');       // Gujarati ર → Devanagari र
        gujarati_to_deva.insert('લ', 'ल');       // Gujarati લ → Devanagari ल
        gujarati_to_deva.insert('વ', 'व');       // Gujarati વ → Devanagari व
        
        // Sibilants and aspirate
        gujarati_to_deva.insert('શ', 'श');       // Gujarati શ → Devanagari श
        gujarati_to_deva.insert('ષ', 'ष');       // Gujarati ષ → Devanagari ष
        gujarati_to_deva.insert('સ', 'स');       // Gujarati સ → Devanagari स
        gujarati_to_deva.insert('હ', 'ह');       // Gujarati હ → Devanagari ह
        
        // Additional consonants
        gujarati_to_deva.insert('ળ', 'ळ');       // Gujarati ળ → Devanagari ळ (retroflex l)
        
        // Special marks
        gujarati_to_deva.insert('ં', 'ं');       // Gujarati ં → Devanagari ं (anusvara)
        gujarati_to_deva.insert('ઃ', 'ः');       // Gujarati ઃ → Devanagari ः (visarga)
        gujarati_to_deva.insert('્', '्');       // Gujarati ્ → Devanagari ् (virama)
        gujarati_to_deva.insert('ઽ', 'ऽ');       // Gujarati ઽ → Devanagari ऽ (avagraha)
        
        // Digits
        gujarati_to_deva.insert('૦', '०');       // Gujarati ૦ → Devanagari ०
        gujarati_to_deva.insert('૧', '१');       // Gujarati ૧ → Devanagari १
        gujarati_to_deva.insert('૨', '२');       // Gujarati ૨ → Devanagari २
        gujarati_to_deva.insert('૩', '३');       // Gujarati ૩ → Devanagari ३
        gujarati_to_deva.insert('૪', '४');       // Gujarati ૪ → Devanagari ४
        gujarati_to_deva.insert('૫', '५');       // Gujarati ૫ → Devanagari ५
        gujarati_to_deva.insert('૬', '६');       // Gujarati ૬ → Devanagari ६
        gujarati_to_deva.insert('૭', '७');       // Gujarati ૭ → Devanagari ७
        gujarati_to_deva.insert('૮', '८');       // Gujarati ૮ → Devanagari ८
        gujarati_to_deva.insert('૯', '९');       // Gujarati ૯ → Devanagari ९
        
        // Build reverse mapping for Devanagari → Gujarati conversion
        let mut deva_to_gujarati = HashMap::new();
        for (&guj, &deva) in &gujarati_to_deva {
            deva_to_gujarati.insert(deva, guj);
        }

        Self {
            gujarati_to_deva_map: gujarati_to_deva,
            deva_to_gujarati_map: deva_to_gujarati,
        }
    }
    
    /// Convert Gujarati text to Devanagari format (for hub processing)
    pub fn gujarati_to_devanagari(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        
        // Simple character-to-character mapping - let hub handle complex rules
        for ch in input.chars() {
            if ch.is_whitespace() || ch.is_ascii_punctuation() {
                result.push(ch);
            } else if let Some(&deva_char) = self.gujarati_to_deva_map.get(&ch) {
                result.push(deva_char);
            } else {
                // Character not in mapping - preserve as-is for mixed content
                result.push(ch);
            }
        }
        
        Ok(result)
    }
    
    /// Convert Devanagari text to Gujarati format (reverse conversion)
    pub fn devanagari_to_gujarati(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        
        // Simple character-to-character mapping
        for ch in input.chars() {
            if ch.is_whitespace() || ch.is_ascii_punctuation() {
                result.push(ch);
            } else if let Some(&guj_char) = self.deva_to_gujarati_map.get(&ch) {
                result.push(guj_char);
            } else {
                // Character not in mapping - preserve as-is for mixed content
                result.push(ch);
            }
        }
        
        Ok(result)
    }
}

impl ScriptConverter for GujaratiConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "gujarati" && script != "gu" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Gujarati converter only supports 'gujarati' or 'gu' script".to_string(),
            });
        }
        
        let deva_text = self.gujarati_to_devanagari(input)
            .map_err(|e| ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: format!("Gujarati to Devanagari conversion failed: {}", e),
            })?;
            
        // Return Devanagari for hub processing, not ISO
        Ok(HubInput::Devanagari(deva_text))
    }
    
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        if script != "gujarati" && script != "gu" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Gujarati converter only supports 'gujarati' or 'gu' script".to_string(),
            });
        }
        
        match hub_input {
            HubInput::Devanagari(deva_text) => self.devanagari_to_gujarati(deva_text),
            HubInput::Iso(_) => Err(ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: "Gujarati converter expects Devanagari input, got ISO".to_string(),
            }),
        }
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["gujarati", "gu"]
    }
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        // Gujarati is an Indic script - consonants DO have implicit 'a'
        // But now the hub will handle this complexity
        true
    }
}

impl Default for GujaratiConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gujarati_basic_vowels() {
        let converter = GujaratiConverter::new();
        
        // Test basic vowels - now should map to Devanagari
        assert_eq!(converter.gujarati_to_devanagari("અ").unwrap(), "अ");
        assert_eq!(converter.gujarati_to_devanagari("આ").unwrap(), "आ");
        assert_eq!(converter.gujarati_to_devanagari("ઇ").unwrap(), "इ");
        assert_eq!(converter.gujarati_to_devanagari("ઈ").unwrap(), "ई");
        assert_eq!(converter.gujarati_to_devanagari("ઉ").unwrap(), "उ");
        assert_eq!(converter.gujarati_to_devanagari("ઊ").unwrap(), "ऊ");
    }
    
    #[test]
    fn test_gujarati_consonants() {
        let converter = GujaratiConverter::new();
        
        // Test basic consonants - should map to Devanagari equivalents
        assert_eq!(converter.gujarati_to_devanagari("ક").unwrap(), "क");
        assert_eq!(converter.gujarati_to_devanagari("ખ").unwrap(), "ख");
        assert_eq!(converter.gujarati_to_devanagari("ગ").unwrap(), "ग");
        assert_eq!(converter.gujarati_to_devanagari("ઘ").unwrap(), "घ");
        assert_eq!(converter.gujarati_to_devanagari("ઙ").unwrap(), "ङ");
    }
    
    #[test]
    fn test_gujarati_dharma_word() {
        let converter = GujaratiConverter::new();
        
        // Test the specific word "ધર્મ" (dharma)
        // Should map directly to Devanagari "धर्म" 
        // Hub will handle the virama processing correctly
        let result = converter.gujarati_to_devanagari("ધર્મ").unwrap();
        println!("Gujarati 'ધર્મ' maps to Devanagari: '{}'", result);
        assert_eq!(result, "धर्म");
    }
    
    #[test]
    fn test_script_converter_interface() {
        let converter = GujaratiConverter::new();
        
        // Test the ScriptConverter interface
        assert!(converter.supports_script("gujarati"));
        assert!(converter.supports_script("gu"));
        assert!(!converter.supports_script("hindi"));
        
        // Test script_has_implicit_a
        assert!(converter.script_has_implicit_a("gujarati"));
        assert!(converter.script_has_implicit_a("gu"));
        
        let result = converter.to_hub("gujarati", "ક").unwrap();
        if let HubInput::Devanagari(deva_text) = result {
            assert_eq!(deva_text, "क");
        } else {
            panic!("Expected Devanagari hub input");
        }
    }
}