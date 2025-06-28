use std::collections::HashMap;
use super::{ScriptConverter, ConverterError};
use crate::modules::hub::HubInput;

/// Gurmukhi (ਗੁਰਮੁਖੀ) script converter
/// 
/// Gurmukhi is the script used for writing Punjabi and is one of the most widely
/// used scripts in Northern India. This converter handles Gurmukhi text by converting
/// it to Devanagari equivalent, which can then be processed by the hub.
pub struct GurmukhiConverter {
    gurmukhi_to_deva_map: HashMap<String, String>,
    deva_to_gurmukhi_map: HashMap<String, String>,
}

impl GurmukhiConverter {
    pub fn new() -> Self {
        let mut gurmukhi_to_deva = HashMap::new();
        
        // Independent vowels (ਸੁਰ) → Devanagari equivalents
        gurmukhi_to_deva.insert("ਅ".to_string(), "अ".to_string());
        gurmukhi_to_deva.insert("ਆ".to_string(), "आ".to_string());
        gurmukhi_to_deva.insert("ਇ".to_string(), "इ".to_string());
        gurmukhi_to_deva.insert("ਈ".to_string(), "ई".to_string());
        gurmukhi_to_deva.insert("ਉ".to_string(), "उ".to_string());
        gurmukhi_to_deva.insert("ਊ".to_string(), "ऊ".to_string());
        gurmukhi_to_deva.insert("ਏ".to_string(), "ए".to_string());
        gurmukhi_to_deva.insert("ਐ".to_string(), "ऐ".to_string());
        gurmukhi_to_deva.insert("ਓ".to_string(), "ओ".to_string());
        gurmukhi_to_deva.insert("ਔ".to_string(), "औ".to_string());
        
        // Consonants - basic ones
        gurmukhi_to_deva.insert("ਕ".to_string(), "क".to_string());
        gurmukhi_to_deva.insert("ਖ".to_string(), "ख".to_string());
        gurmukhi_to_deva.insert("ਗ".to_string(), "ग".to_string());
        gurmukhi_to_deva.insert("ਘ".to_string(), "घ".to_string());
        gurmukhi_to_deva.insert("ਙ".to_string(), "ङ".to_string());
        gurmukhi_to_deva.insert("ਚ".to_string(), "च".to_string());
        gurmukhi_to_deva.insert("ਛ".to_string(), "छ".to_string());
        gurmukhi_to_deva.insert("ਜ".to_string(), "ज".to_string());
        gurmukhi_to_deva.insert("ਝ".to_string(), "झ".to_string());
        gurmukhi_to_deva.insert("ਞ".to_string(), "ञ".to_string());
        gurmukhi_to_deva.insert("ਟ".to_string(), "ट".to_string());
        gurmukhi_to_deva.insert("ਠ".to_string(), "ठ".to_string());
        gurmukhi_to_deva.insert("ਡ".to_string(), "ड".to_string());
        gurmukhi_to_deva.insert("ਢ".to_string(), "ढ".to_string());
        gurmukhi_to_deva.insert("ਣ".to_string(), "ण".to_string());
        gurmukhi_to_deva.insert("ਤ".to_string(), "त".to_string());
        gurmukhi_to_deva.insert("ਥ".to_string(), "थ".to_string());
        gurmukhi_to_deva.insert("ਦ".to_string(), "द".to_string());
        gurmukhi_to_deva.insert("ਧ".to_string(), "ध".to_string());
        gurmukhi_to_deva.insert("ਨ".to_string(), "न".to_string());
        gurmukhi_to_deva.insert("ਪ".to_string(), "प".to_string());
        gurmukhi_to_deva.insert("ਫ".to_string(), "फ".to_string());
        gurmukhi_to_deva.insert("ਬ".to_string(), "ब".to_string());
        gurmukhi_to_deva.insert("ਭ".to_string(), "भ".to_string());
        gurmukhi_to_deva.insert("ਮ".to_string(), "म".to_string());
        gurmukhi_to_deva.insert("ਯ".to_string(), "य".to_string());
        gurmukhi_to_deva.insert("ਰ".to_string(), "र".to_string());
        gurmukhi_to_deva.insert("ਲ".to_string(), "ल".to_string());
        gurmukhi_to_deva.insert("ਵ".to_string(), "व".to_string());
        gurmukhi_to_deva.insert("ਸ".to_string(), "स".to_string());
        gurmukhi_to_deva.insert("ਹ".to_string(), "ह".to_string());
        
        // Vowel diacritics
        gurmukhi_to_deva.insert("ਾ".to_string(), "ा".to_string());
        gurmukhi_to_deva.insert("ਿ".to_string(), "ि".to_string());
        gurmukhi_to_deva.insert("ੀ".to_string(), "ी".to_string());
        gurmukhi_to_deva.insert("ੁ".to_string(), "ु".to_string());
        gurmukhi_to_deva.insert("ੂ".to_string(), "ू".to_string());
        gurmukhi_to_deva.insert("ੇ".to_string(), "े".to_string());
        gurmukhi_to_deva.insert("ੈ".to_string(), "ै".to_string());
        gurmukhi_to_deva.insert("ੋ".to_string(), "ो".to_string());
        gurmukhi_to_deva.insert("ੌ".to_string(), "ौ".to_string());
        
        // Special marks
        gurmukhi_to_deva.insert("ਂ".to_string(), "ं".to_string());
        gurmukhi_to_deva.insert("ਃ".to_string(), "ः".to_string());
        gurmukhi_to_deva.insert("੍".to_string(), "्".to_string());
        gurmukhi_to_deva.insert("ੰ".to_string(), "ं".to_string()); // Tippi
        gurmukhi_to_deva.insert("ੱ".to_string(), "्".to_string()); // Addak
        
        // Digits  
        gurmukhi_to_deva.insert("੦".to_string(), "०".to_string());
        gurmukhi_to_deva.insert("੧".to_string(), "१".to_string());
        gurmukhi_to_deva.insert("੨".to_string(), "२".to_string());
        gurmukhi_to_deva.insert("੩".to_string(), "३".to_string());
        gurmukhi_to_deva.insert("੪".to_string(), "४".to_string());
        gurmukhi_to_deva.insert("੫".to_string(), "५".to_string());
        gurmukhi_to_deva.insert("੬".to_string(), "६".to_string());
        gurmukhi_to_deva.insert("੭".to_string(), "७".to_string());
        gurmukhi_to_deva.insert("੮".to_string(), "८".to_string());
        gurmukhi_to_deva.insert("੯".to_string(), "९".to_string());

        // Build reverse mapping
        let mut deva_to_gurmukhi = HashMap::new();
        for (gurmukhi, deva) in &gurmukhi_to_deva {
            deva_to_gurmukhi.insert(deva.clone(), gurmukhi.clone());
        }

        Self {
            gurmukhi_to_deva_map: gurmukhi_to_deva,
            deva_to_gurmukhi_map: deva_to_gurmukhi,
        }
    }
    
    /// Convert Gurmukhi text to Devanagari format (for hub processing)
    pub fn gurmukhi_to_devanagari(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        
        for ch in input.chars() {
            if ch.is_whitespace() || ch.is_ascii_punctuation() {
                result.push(ch);
            } else if let Some(deva_char) = self.gurmukhi_to_deva_map.get(&ch.to_string()) {
                result.push_str(deva_char);
            } else {
                // Character not in mapping - preserve as-is for mixed content
                result.push(ch);
            }
        }
        
        Ok(result)
    }
    
    /// Convert Devanagari text to Gurmukhi format (reverse conversion)
    pub fn devanagari_to_gurmukhi(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        
        for ch in input.chars() {
            if ch.is_whitespace() || ch.is_ascii_punctuation() {
                result.push(ch);
            } else if let Some(gurmukhi_char) = self.deva_to_gurmukhi_map.get(&ch.to_string()) {
                result.push_str(gurmukhi_char);
            } else {
                // Character not in mapping - preserve as-is for mixed content
                result.push(ch);
            }
        }
        
        Ok(result)
    }
}

impl ScriptConverter for GurmukhiConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "gurmukhi" && script != "guru" && script != "punjabi" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Gurmukhi converter only supports 'gurmukhi', 'guru', or 'punjabi' script".to_string(),
            });
        }
        
        let deva_text = self.gurmukhi_to_devanagari(input)
            .map_err(|e| ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: format!("Gurmukhi to Devanagari conversion failed: {}", e),
            })?;
            
        Ok(HubInput::Devanagari(deva_text))
    }
    
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        if script != "gurmukhi" && script != "guru" && script != "punjabi" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Gurmukhi converter only supports 'gurmukhi', 'guru', or 'punjabi' script".to_string(),
            });
        }
        
        match hub_input {
            HubInput::Devanagari(deva_text) => self.devanagari_to_gurmukhi(deva_text),
            HubInput::Iso(_) => Err(ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: "Gurmukhi converter expects Devanagari input, got ISO".to_string(),
            }),
        }
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["gurmukhi", "guru", "punjabi"]
    }
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        // Gurmukhi is an Indic script - consonants DO have implicit 'a'
        true
    }
}

impl Default for GurmukhiConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gurmukhi_basic_vowels() {
        let converter = GurmukhiConverter::new();
        
        assert_eq!(converter.gurmukhi_to_devanagari("ਅ").unwrap(), "अ");
        assert_eq!(converter.gurmukhi_to_devanagari("ਆ").unwrap(), "आ");
        assert_eq!(converter.gurmukhi_to_devanagari("ਇ").unwrap(), "इ");
        assert_eq!(converter.gurmukhi_to_devanagari("ਈ").unwrap(), "ई");
    }
    
    #[test]
    fn test_gurmukhi_consonants() {
        let converter = GurmukhiConverter::new();
        
        assert_eq!(converter.gurmukhi_to_devanagari("ਕ").unwrap(), "क");
        assert_eq!(converter.gurmukhi_to_devanagari("ਖ").unwrap(), "ख");
        assert_eq!(converter.gurmukhi_to_devanagari("ਗ").unwrap(), "ग");
        assert_eq!(converter.gurmukhi_to_devanagari("ਘ").unwrap(), "घ");
    }
    
    #[test]
    fn test_gurmukhi_special_characters() {
        let converter = GurmukhiConverter::new();
        
        // Test Tippi and Addak
        assert_eq!(converter.gurmukhi_to_devanagari("ੰ").unwrap(), "ं");
        assert_eq!(converter.gurmukhi_to_devanagari("ੱ").unwrap(), "्");
    }
    
    #[test]
    fn test_script_converter_interface() {
        let converter = GurmukhiConverter::new();
        
        assert!(converter.supports_script("gurmukhi"));
        assert!(converter.supports_script("guru"));
        assert!(converter.supports_script("punjabi"));
        assert!(!converter.supports_script("hindi"));
        
        assert!(converter.script_has_implicit_a("gurmukhi"));
        
        let result = converter.to_hub("gurmukhi", "ਕ").unwrap();
        if let HubInput::Devanagari(deva_text) = result {
            assert_eq!(deva_text, "क");
        } else {
            panic!("Expected Devanagari hub input");
        }
    }
}