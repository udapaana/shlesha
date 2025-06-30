use super::{ConverterError, ScriptConverter};
use crate::modules::hub::HubInput;

/// ISO-15919 script converter
///
/// ISO-15919 is one of the hub formats in our architecture. This converter simply passes
/// ISO-15919 text directly to the hub without any transformation, since the hub
/// already handles ISO-15919 ↔ Devanagari conversion internally.
pub struct ISO15919Converter;

impl ISO15919Converter {
    #[inline]
    pub fn new() -> Self {
        Self
    }
}

impl ScriptConverter for ISO15919Converter {
    #[inline]
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "iso15919" && script != "iso_15919" && script != "iso" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message:
                    "ISO-15919 converter only supports 'iso15919', 'iso_15919', or 'iso' script"
                        .to_string(),
            });
        }

        // ISO-15919 is one of the hub formats - pass directly to hub
        Ok(HubInput::Iso(input.to_string()))
    }

    #[inline]
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        if script != "iso15919" && script != "iso_15919" && script != "iso" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message:
                    "ISO-15919 converter only supports 'iso15919', 'iso_15919', or 'iso' script"
                        .to_string(),
            });
        }

        match hub_input {
            HubInput::Iso(iso_text) => Ok(iso_text.clone()),
            HubInput::Devanagari(_) => Err(ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: "ISO-15919 converter expects ISO input, got Devanagari".to_string(),
            }),
        }
    }

    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["iso15919", "iso_15919", "iso"]
    }

    #[inline]
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        // ISO-15919 is a romanization scheme - consonants do NOT have implicit 'a'
        // ISO-15919 explicitly represents consonants without vowels
        false
    }
}

impl Default for ISO15919Converter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso15919_passthrough() {
        let converter = ISO15919Converter::new();

        // ISO-15919 text should be passed through as HubInput::Iso
        let test_cases = vec![
            "a",       // single vowel
            "k",       // single consonant
            "ka",      // consonant with vowel
            "namaste", // complete word
            "dharma",  // word with 'r'
            "kr̥ṣṇa",   // complex word with vocalic r and retroflex
            "śānti",   // word with palatal sibilant
            "ṛṣi",     // word with vocalic r
        ];

        for input in test_cases {
            let result = converter.to_hub("iso15919", input).unwrap();
            if let HubInput::Iso(iso_text) = result {
                assert_eq!(
                    iso_text, input,
                    "ISO-15919 input '{}' should pass through unchanged",
                    input
                );
            } else {
                panic!("Expected HubInput::Iso, got something else");
            }
        }
    }

    #[test]
    fn test_script_converter_interface() {
        let converter = ISO15919Converter::new();

        // Test the ScriptConverter interface
        assert!(converter.supports_script("iso15919"));
        assert!(converter.supports_script("iso_15919"));
        assert!(converter.supports_script("iso"));
        assert!(!converter.supports_script("iast"));

        // Test script_has_implicit_a
        assert!(!converter.script_has_implicit_a("iso15919"));
        assert!(!converter.script_has_implicit_a("iso"));

        let result = converter.to_hub("iso", "k").unwrap();
        if let HubInput::Iso(iso_text) = result {
            assert_eq!(iso_text, "k");
        } else {
            panic!("Expected HubInput::Iso");
        }
    }

    #[test]
    fn test_invalid_script_error() {
        let converter = ISO15919Converter::new();

        // Should reject invalid script names
        let result = converter.to_hub("latin", "test");
        assert!(result.is_err());

        if let Err(ConverterError::InvalidInput { script, message }) = result {
            assert_eq!(script, "latin");
            assert!(message.contains("ISO-15919 converter only supports"));
        } else {
            panic!("Expected InvalidInput error");
        }
    }

    #[test]
    fn test_empty_input() {
        let converter = ISO15919Converter::new();

        // Should handle empty input gracefully
        let result = converter.to_hub("iso15919", "").unwrap();
        if let HubInput::Iso(iso_text) = result {
            assert_eq!(iso_text, "");
        } else {
            panic!("Expected HubInput::Iso");
        }
    }

    #[test]
    fn test_mixed_content() {
        let converter = ISO15919Converter::new();

        // Should handle mixed ISO-15919 and other characters
        let mixed_input = "namaste 123 hello";
        let result = converter.to_hub("iso15919", mixed_input).unwrap();
        if let HubInput::Iso(iso_text) = result {
            assert_eq!(iso_text, mixed_input);
        } else {
            panic!("Expected HubInput::Iso");
        }
    }

    #[test]
    fn test_complex_iso15919_text() {
        let converter = ISO15919Converter::new();

        // Test complex ISO-15919 text with all character types
        let complex_input = "śrī kr̥ṣṇa bhagavān uvāca"; // Sanskrit text in ISO-15919
        let result = converter.to_hub("iso15919", complex_input).unwrap();
        if let HubInput::Iso(iso_text) = result {
            assert_eq!(iso_text, complex_input);
        } else {
            panic!("Expected HubInput::Iso");
        }
    }
}
