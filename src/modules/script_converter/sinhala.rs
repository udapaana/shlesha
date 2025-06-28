use std::collections::HashMap;
use super::{ScriptConverter, ConverterError};
use crate::modules::hub::HubInput;

/// Sinhala (සිංහල) script converter
/// 
/// Sinhala is the script used for writing the Sinhala language, primarily in Sri Lanka.
/// This converter handles Sinhala text by converting it to Devanagari equivalent,
/// which can then be processed by the hub. Uses the optimal Indic script structure.
pub struct SinhalaConverter {
    sinhala_to_deva_map: HashMap<char, char>,
    deva_to_sinhala_map: HashMap<char, char>,
}

impl SinhalaConverter {
    pub fn new() -> Self {
        let mut sinhala_to_deva = HashMap::new();
        
        // Independent vowels → Devanagari equivalents
        sinhala_to_deva.insert('අ', 'अ');       // Sinhala අ → Devanagari अ
        sinhala_to_deva.insert('ආ', 'आ');       // Sinhala ආ → Devanagari आ
        sinhala_to_deva.insert('ඇ', 'ऍ');       // Sinhala ඇ → Devanagari ऍ (ae sound)
        sinhala_to_deva.insert('ඈ', 'ऑ');       // Sinhala ඈ → Devanagari ऑ (aae sound)
        sinhala_to_deva.insert('ඉ', 'इ');       // Sinhala ඉ → Devanagari इ
        sinhala_to_deva.insert('ඊ', 'ई');       // Sinhala ඊ → Devanagari ई
        sinhala_to_deva.insert('උ', 'उ');       // Sinhala උ → Devanagari उ
        sinhala_to_deva.insert('ඌ', 'ऊ');       // Sinhala ඌ → Devanagari ऊ
        sinhala_to_deva.insert('ඍ', 'ऋ');       // Sinhala ඍ → Devanagari ऋ
        sinhala_to_deva.insert('ඎ', 'ॠ');       // Sinhala ඎ → Devanagari ॠ
        sinhala_to_deva.insert('ඏ', 'ऌ');       // Sinhala ඏ → Devanagari ऌ
        sinhala_to_deva.insert('ඐ', 'ॡ');       // Sinhala ඐ → Devanagari ॡ
        sinhala_to_deva.insert('එ', 'ए');       // Sinhala එ → Devanagari ए
        sinhala_to_deva.insert('ඒ', 'ए');       // Sinhala ඒ → Devanagari ए (long e)
        sinhala_to_deva.insert('ඓ', 'ऐ');       // Sinhala ඓ → Devanagari ऐ
        sinhala_to_deva.insert('ඔ', 'ओ');       // Sinhala ඔ → Devanagari ओ
        sinhala_to_deva.insert('ඕ', 'ओ');       // Sinhala ඕ → Devanagari ओ (long o)
        sinhala_to_deva.insert('ඖ', 'औ');       // Sinhala ඖ → Devanagari औ
        
        // Vowel diacritics → Devanagari equivalents
        sinhala_to_deva.insert('ා', 'ा');       // Sinhala ා → Devanagari ा
        sinhala_to_deva.insert('ැ', 'ॅ');       // Sinhala ැ → Devanagari ॅ (ae sound)
        sinhala_to_deva.insert('ෑ', 'ॉ');       // Sinhala ෑ → Devanagari ॉ (aae sound)
        sinhala_to_deva.insert('ි', 'ि');       // Sinhala ි → Devanagari ि
        sinhala_to_deva.insert('ී', 'ी');       // Sinhala ී → Devanagari ी
        sinhala_to_deva.insert('ු', 'ु');       // Sinhala ු → Devanagari ु
        sinhala_to_deva.insert('ූ', 'ू');       // Sinhala ූ → Devanagari ू
        sinhala_to_deva.insert('ෘ', 'ृ');       // Sinhala ෘ → Devanagari ृ
        sinhala_to_deva.insert('ෲ', 'ॄ');       // Sinhala ෲ → Devanagari ॄ
        sinhala_to_deva.insert('ෟ', 'ॢ');       // Sinhala ෟ → Devanagari ॢ
        sinhala_to_deva.insert('ෳ', 'ॣ');       // Sinhala ෳ → Devanagari ॣ
        sinhala_to_deva.insert('ෙ', 'े');       // Sinhala ෙ → Devanagari े
        sinhala_to_deva.insert('ේ', 'े');       // Sinhala ේ → Devanagari े (long e)
        sinhala_to_deva.insert('ෛ', 'ै');       // Sinhala ෛ → Devanagari ै
        sinhala_to_deva.insert('ො', 'ो');       // Sinhala ො → Devanagari ो
        sinhala_to_deva.insert('ෝ', 'ो');       // Sinhala ෝ → Devanagari ो (long o)
        sinhala_to_deva.insert('ෞ', 'ौ');       // Sinhala ෞ → Devanagari ौ
        
        // Consonants → Devanagari equivalents
        // Velar consonants
        sinhala_to_deva.insert('ක', 'क');       // Sinhala ක → Devanagari क
        sinhala_to_deva.insert('ඛ', 'ख');       // Sinhala ඛ → Devanagari ख
        sinhala_to_deva.insert('ග', 'ग');       // Sinhala ග → Devanagari ग
        sinhala_to_deva.insert('ඝ', 'घ');       // Sinhala ඝ → Devanagari घ
        sinhala_to_deva.insert('ඞ', 'ङ');       // Sinhala ඞ → Devanagari ङ
        sinhala_to_deva.insert('ඟ', 'ग');       // Sinhala ඟ → Devanagari ग (prenasalized)
        
        // Palatal consonants
        sinhala_to_deva.insert('ච', 'च');       // Sinhala ච → Devanagari च
        sinhala_to_deva.insert('ඡ', 'छ');       // Sinhala ඡ → Devanagari छ
        sinhala_to_deva.insert('ජ', 'ज');       // Sinhala ජ → Devanagari ज
        sinhala_to_deva.insert('ඣ', 'झ');       // Sinhala ඣ → Devanagari झ
        sinhala_to_deva.insert('ඤ', 'ञ');       // Sinhala ඤ → Devanagari ञ
        sinhala_to_deva.insert('ඦ', 'ज');       // Sinhala ඦ → Devanagari ज (prenasalized)
        
        // Retroflex consonants
        sinhala_to_deva.insert('ට', 'ट');       // Sinhala ට → Devanagari ट
        sinhala_to_deva.insert('ඨ', 'ठ');       // Sinhala ඨ → Devanagari ठ
        sinhala_to_deva.insert('ඩ', 'ड');       // Sinhala ඩ → Devanagari ड
        sinhala_to_deva.insert('ඪ', 'ढ');       // Sinhala ඪ → Devanagari ढ
        sinhala_to_deva.insert('ණ', 'ण');       // Sinhala ණ → Devanagari ण
        sinhala_to_deva.insert('ඬ', 'ड');       // Sinhala ඬ → Devanagari ड (prenasalized)
        
        // Dental consonants
        sinhala_to_deva.insert('ත', 'त');       // Sinhala ත → Devanagari त
        sinhala_to_deva.insert('ථ', 'थ');       // Sinhala ථ → Devanagari थ
        sinhala_to_deva.insert('ද', 'द');       // Sinhala ද → Devanagari द
        sinhala_to_deva.insert('ධ', 'ध');       // Sinhala ධ → Devanagari ध
        sinhala_to_deva.insert('න', 'न');       // Sinhala න → Devanagari न
        sinhala_to_deva.insert('ඳ', 'द');       // Sinhala ඳ → Devanagari द (prenasalized)
        
        // Labial consonants
        sinhala_to_deva.insert('ප', 'प');       // Sinhala ප → Devanagari प
        sinhala_to_deva.insert('ඵ', 'फ');       // Sinhala ඵ → Devanagari फ
        sinhala_to_deva.insert('බ', 'ब');       // Sinhala බ → Devanagari ब
        sinhala_to_deva.insert('භ', 'भ');       // Sinhala භ → Devanagari भ
        sinhala_to_deva.insert('ම', 'म');       // Sinhala ම → Devanagari म
        sinhala_to_deva.insert('ඹ', 'ब');       // Sinhala ඹ → Devanagari ब (prenasalized)
        
        // Semivowels and liquids
        sinhala_to_deva.insert('ය', 'य');       // Sinhala ය → Devanagari य
        sinhala_to_deva.insert('ර', 'र');       // Sinhala ර → Devanagari र
        sinhala_to_deva.insert('ල', 'ल');       // Sinhala ල → Devanagari ल
        sinhala_to_deva.insert('ව', 'व');       // Sinhala ව → Devanagari व
        
        // Sibilants and aspirate
        sinhala_to_deva.insert('ශ', 'श');       // Sinhala ශ → Devanagari श
        sinhala_to_deva.insert('ෂ', 'ष');       // Sinhala ෂ → Devanagari ष
        sinhala_to_deva.insert('ස', 'स');       // Sinhala ස → Devanagari स
        sinhala_to_deva.insert('හ', 'ह');       // Sinhala හ → Devanagari ह
        sinhala_to_deva.insert('ළ', 'ळ');       // Sinhala ළ → Devanagari ळ
        
        // Special Sinhala consonants
        sinhala_to_deva.insert('ෆ', 'फ');       // Sinhala ෆ → Devanagari फ (for foreign sounds)
        
        // Special marks
        sinhala_to_deva.insert('ං', 'ं');       // Sinhala ං → Devanagari ं (anusvara)
        sinhala_to_deva.insert('ඃ', 'ः');       // Sinhala ඃ → Devanagari ः (visarga)
        sinhala_to_deva.insert('්', '्');       // Sinhala ් → Devanagari ् (virama/al-lakuna)
        
        // Digits
        sinhala_to_deva.insert('෦', '०');       // Sinhala ෦ → Devanagari ०
        sinhala_to_deva.insert('෧', '१');       // Sinhala ෧ → Devanagari १
        sinhala_to_deva.insert('෨', '२');       // Sinhala ෨ → Devanagari २
        sinhala_to_deva.insert('෩', '३');       // Sinhala ෩ → Devanagari ३
        sinhala_to_deva.insert('෪', '४');       // Sinhala ෪ → Devanagari ४
        sinhala_to_deva.insert('෫', '५');       // Sinhala ෫ → Devanagari ५
        sinhala_to_deva.insert('෬', '६');       // Sinhala ෬ → Devanagari ६
        sinhala_to_deva.insert('෭', '७');       // Sinhala ෭ → Devanagari ७
        sinhala_to_deva.insert('෮', '८');       // Sinhala ෮ → Devanagari ८
        sinhala_to_deva.insert('෯', '९');       // Sinhala ෯ → Devanagari ९
        
        // Build reverse mapping
        let mut deva_to_sinhala = HashMap::new();
        for (&sinhala, &deva) in &sinhala_to_deva {
            deva_to_sinhala.insert(deva, sinhala);
        }

        Self {
            sinhala_to_deva_map: sinhala_to_deva,
            deva_to_sinhala_map: deva_to_sinhala,
        }
    }
    
    /// Convert Sinhala text to Devanagari format (for hub processing)
    pub fn sinhala_to_devanagari(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        
        for ch in input.chars() {
            if ch.is_whitespace() || ch.is_ascii_punctuation() {
                result.push(ch);
            } else if let Some(&deva_char) = self.sinhala_to_deva_map.get(&ch) {
                result.push(deva_char);
            } else {
                // Character not in mapping - preserve as-is for mixed content
                result.push(ch);
            }
        }
        
        Ok(result)
    }
    
    /// Convert Devanagari text to Sinhala format (reverse conversion)
    pub fn devanagari_to_sinhala(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        
        for ch in input.chars() {
            if ch.is_whitespace() || ch.is_ascii_punctuation() {
                result.push(ch);
            } else if let Some(&sinhala_char) = self.deva_to_sinhala_map.get(&ch) {
                result.push(sinhala_char);
            } else {
                // Character not in mapping - preserve as-is for mixed content
                result.push(ch);
            }
        }
        
        Ok(result)
    }
}

impl ScriptConverter for SinhalaConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "sinhala" && script != "sinh" && script != "si" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Sinhala converter only supports 'sinhala', 'sinh', or 'si' script".to_string(),
            });
        }
        
        let deva_text = self.sinhala_to_devanagari(input)
            .map_err(|e| ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: format!("Sinhala to Devanagari conversion failed: {}", e),
            })?;
            
        Ok(HubInput::Devanagari(deva_text))
    }
    
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        if script != "sinhala" && script != "sinh" && script != "si" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Sinhala converter only supports 'sinhala', 'sinh', or 'si' script".to_string(),
            });
        }
        
        match hub_input {
            HubInput::Devanagari(deva_text) => self.devanagari_to_sinhala(deva_text),
            HubInput::Iso(_) => Err(ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: "Sinhala converter expects Devanagari input, got ISO".to_string(),
            }),
        }
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["sinhala", "sinh", "si"]
    }
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        // Sinhala is an Indic script - consonants DO have implicit 'a'
        true
    }
}

impl Default for SinhalaConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sinhala_basic_vowels() {
        let converter = SinhalaConverter::new();
        
        assert_eq!(converter.sinhala_to_devanagari("අ").unwrap(), "अ");
        assert_eq!(converter.sinhala_to_devanagari("ආ").unwrap(), "आ");
        assert_eq!(converter.sinhala_to_devanagari("ඉ").unwrap(), "इ");
        assert_eq!(converter.sinhala_to_devanagari("ඊ").unwrap(), "ई");
    }
    
    #[test]
    fn test_sinhala_consonants() {
        let converter = SinhalaConverter::new();
        
        assert_eq!(converter.sinhala_to_devanagari("ක").unwrap(), "क");
        assert_eq!(converter.sinhala_to_devanagari("ඛ").unwrap(), "ख");
        assert_eq!(converter.sinhala_to_devanagari("ග").unwrap(), "ग");
        assert_eq!(converter.sinhala_to_devanagari("ඝ").unwrap(), "घ");
    }
    
    #[test]
    fn test_sinhala_special_characters() {
        let converter = SinhalaConverter::new();
        
        // Test prenasalized consonants (unique to Sinhala)
        assert_eq!(converter.sinhala_to_devanagari("ඟ").unwrap(), "ग");
        assert_eq!(converter.sinhala_to_devanagari("ඦ").unwrap(), "ज");
        assert_eq!(converter.sinhala_to_devanagari("ඬ").unwrap(), "ड");
    }
    
    #[test]
    fn test_script_converter_interface() {
        let converter = SinhalaConverter::new();
        
        assert!(converter.supports_script("sinhala"));
        assert!(converter.supports_script("sinh"));
        assert!(converter.supports_script("si"));
        assert!(!converter.supports_script("tamil"));
        
        assert!(converter.script_has_implicit_a("sinhala"));
        
        let result = converter.to_hub("sinhala", "ක").unwrap();
        if let HubInput::Devanagari(deva_text) = result {
            assert_eq!(deva_text, "क");
        } else {
            panic!("Expected Devanagari hub input");
        }
    }
}