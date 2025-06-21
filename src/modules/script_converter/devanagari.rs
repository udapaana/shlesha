use super::{ScriptConverter, ConverterError};
use crate::modules::hub::HubInput;

/// Devanagari script converter
/// 
/// Devanagari is the hub script in our architecture. This converter simply passes
/// Devanagari text directly to the hub without any transformation, since the hub
/// already handles Devanagari ↔ ISO-15919 conversion internally.
pub struct DevanagariConverter;

impl DevanagariConverter {
    pub fn new() -> Self {
        Self
    }
}

impl ScriptConverter for DevanagariConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "devanagari" && script != "deva" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Devanagari converter only supports 'devanagari' or 'deva' script".to_string(),
            });
        }
        
        // Devanagari is the hub script - pass directly to hub
        Ok(HubInput::Devanagari(input.to_string()))
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["devanagari", "deva"]
    }
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        // Devanagari is an Indic script - consonants DO have implicit 'a'
        // In Devanagari, क inherently represents "ka" and requires virama (्) to suppress the vowel: क्
        true
    }
}

impl Default for DevanagariConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_devanagari_passthrough() {
        let converter = DevanagariConverter::new();
        
        // Devanagari text should be passed through as HubInput::Devanagari
        let test_cases = vec![
            "अ",        // single vowel
            "क",        // single consonant
            "का",       // consonant with vowel
            "नमस्ते",    // complete word
            "धर्म",      // word with virama
            "कृष्ण",     // complex word
        ];
        
        for input in test_cases {
            let result = converter.to_hub("devanagari", input).unwrap();
            if let HubInput::Devanagari(deva_text) = result {
                assert_eq!(deva_text, input, 
                    "Devanagari input '{}' should pass through unchanged", input);
            } else {
                panic!("Expected HubInput::Devanagari, got something else");
            }
        }
    }
    
    #[test]
    fn test_script_converter_interface() {
        let converter = DevanagariConverter::new();
        
        // Test the ScriptConverter interface
        assert!(converter.supports_script("devanagari"));
        assert!(converter.supports_script("deva"));
        assert!(!converter.supports_script("bengali"));
        
        // Test script_has_implicit_a
        assert!(converter.script_has_implicit_a("devanagari"));
        assert!(converter.script_has_implicit_a("deva"));
        
        let result = converter.to_hub("deva", "क").unwrap();
        if let HubInput::Devanagari(deva_text) = result {
            assert_eq!(deva_text, "क");
        } else {
            panic!("Expected HubInput::Devanagari");
        }
    }
    
    #[test]
    fn test_invalid_script_error() {
        let converter = DevanagariConverter::new();
        
        // Should reject invalid script names
        let result = converter.to_hub("hindi", "test");
        assert!(result.is_err());
        
        if let Err(ConverterError::InvalidInput { script, message }) = result {
            assert_eq!(script, "hindi");
            assert!(message.contains("Devanagari converter only supports"));
        } else {
            panic!("Expected InvalidInput error");
        }
    }
    
    #[test]
    fn test_empty_input() {
        let converter = DevanagariConverter::new();
        
        // Should handle empty input gracefully
        let result = converter.to_hub("devanagari", "").unwrap();
        if let HubInput::Devanagari(deva_text) = result {
            assert_eq!(deva_text, "");
        } else {
            panic!("Expected HubInput::Devanagari");
        }
    }
    
    #[test]
    fn test_mixed_content() {
        let converter = DevanagariConverter::new();
        
        // Should handle mixed Devanagari and other characters
        let mixed_input = "नमस्ते 123 hello";
        let result = converter.to_hub("devanagari", mixed_input).unwrap();
        if let HubInput::Devanagari(deva_text) = result {
            assert_eq!(deva_text, mixed_input);
        } else {
            panic!("Expected HubInput::Devanagari");
        }
    }
}