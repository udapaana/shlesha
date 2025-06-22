use std::collections::HashMap;
use super::{ScriptConverter, ConverterError};
use crate::modules::hub::HubInput;

/// Kannada script converter
/// 
/// Kannada (ಕನ್ನಡ) is a Dravidian language written in the Kannada script,
/// primarily used in Karnataka, India. The script is derived from the Brahmi script
/// and shares similarities with Telugu script.
pub struct KannadaConverter {
    kannada_to_deva_map: HashMap<char, char>,
    deva_to_kannada_map: HashMap<char, char>,
}

impl KannadaConverter {
    pub fn new() -> Self {
        let mut kannada_to_deva = HashMap::new();
        
        // Vowels (independent forms)
        kannada_to_deva.insert('ಅ', 'अ');  // a
        kannada_to_deva.insert('ಆ', 'आ');  // ā
        kannada_to_deva.insert('ಇ', 'इ');  // i
        kannada_to_deva.insert('ಈ', 'ई');  // ī
        kannada_to_deva.insert('ಉ', 'उ');  // u
        kannada_to_deva.insert('ಊ', 'ऊ');  // ū
        kannada_to_deva.insert('ಋ', 'ऋ');  // r̥
        kannada_to_deva.insert('ೠ', 'ॠ');  // r̥̄
        kannada_to_deva.insert('ಌ', 'ऌ');  // l̥
        kannada_to_deva.insert('ೡ', 'ॡ');  // l̥̄
        kannada_to_deva.insert('ಎ', 'ए');  // e
        kannada_to_deva.insert('ಏ', 'ए');  // ē (mapped to e for simplicity)
        kannada_to_deva.insert('ಐ', 'ऐ');  // ai
        kannada_to_deva.insert('ಒ', 'ओ');  // o
        kannada_to_deva.insert('ಓ', 'ओ');  // ō (mapped to o for simplicity)
        kannada_to_deva.insert('ಔ', 'औ');  // au
        
        // Consonants (with inherent 'a')
        kannada_to_deva.insert('ಕ', 'क');  // ka
        kannada_to_deva.insert('ಖ', 'ख');  // kha
        kannada_to_deva.insert('ಗ', 'ग');  // ga
        kannada_to_deva.insert('ಘ', 'घ');  // gha
        kannada_to_deva.insert('ಙ', 'ङ');  // ṅa
        
        kannada_to_deva.insert('ಚ', 'च');  // ca
        kannada_to_deva.insert('ಛ', 'छ');  // cha
        kannada_to_deva.insert('ಜ', 'ज');  // ja
        kannada_to_deva.insert('ಝ', 'झ');  // jha
        kannada_to_deva.insert('ಞ', 'ञ');  // ña
        
        kannada_to_deva.insert('ಟ', 'ट');  // ṭa
        kannada_to_deva.insert('ಠ', 'ठ');  // ṭha
        kannada_to_deva.insert('ಡ', 'ड');  // ḍa
        kannada_to_deva.insert('ಢ', 'ढ');  // ḍha
        kannada_to_deva.insert('ಣ', 'ण');  // ṇa
        
        kannada_to_deva.insert('ತ', 'त');  // ta
        kannada_to_deva.insert('ಥ', 'थ');  // tha
        kannada_to_deva.insert('ದ', 'द');  // da
        kannada_to_deva.insert('ಧ', 'ध');  // dha
        kannada_to_deva.insert('ನ', 'न');  // na
        
        kannada_to_deva.insert('ಪ', 'प');  // pa
        kannada_to_deva.insert('ಫ', 'फ');  // pha
        kannada_to_deva.insert('ಬ', 'ब');  // ba
        kannada_to_deva.insert('ಭ', 'भ');  // bha
        kannada_to_deva.insert('ಮ', 'म');  // ma
        
        kannada_to_deva.insert('ಯ', 'य');  // ya
        kannada_to_deva.insert('ರ', 'र');  // ra
        kannada_to_deva.insert('ಲ', 'ल');  // la
        kannada_to_deva.insert('ವ', 'व');  // va
        
        kannada_to_deva.insert('ಶ', 'श');  // śa
        kannada_to_deva.insert('ಷ', 'ष');  // ṣa
        kannada_to_deva.insert('ಸ', 'स');  // sa
        kannada_to_deva.insert('ಹ', 'ह');  // ha
        
        // Additional consonants
        kannada_to_deva.insert('ಳ', 'ळ');  // ḷa (retroflex L)
        kannada_to_deva.insert('ೞ', 'ऴ');  // ḻa (Tamil/Malayalam ḻa in Kannada)
        
        // Vowel signs (dependent forms)
        kannada_to_deva.insert('ಾ', 'ा');  // ā sign
        kannada_to_deva.insert('ಿ', 'ि');  // i sign  
        kannada_to_deva.insert('ೀ', 'ी');  // ī sign
        kannada_to_deva.insert('ು', 'ु');  // u sign
        kannada_to_deva.insert('ೂ', 'ू');  // ū sign
        kannada_to_deva.insert('ೃ', 'ृ');  // r̥ sign
        kannada_to_deva.insert('ೄ', 'ॄ');  // r̥̄ sign
        kannada_to_deva.insert('ೆ', 'े');  // e sign
        kannada_to_deva.insert('ೇ', 'े');  // ē sign (mapped to e)
        kannada_to_deva.insert('ೈ', 'ै');  // ai sign
        kannada_to_deva.insert('ೊ', 'ो');  // o sign
        kannada_to_deva.insert('ೋ', 'ो');  // ō sign (mapped to o)
        kannada_to_deva.insert('ೌ', 'ौ');  // au sign
        
        // Special marks
        kannada_to_deva.insert('ಂ', 'ं');  // anusvara
        kannada_to_deva.insert('ಃ', 'ः');  // visarga
        kannada_to_deva.insert('್', '्');  // virama (halant)
        
        // Digits
        kannada_to_deva.insert('೦', '०');  // 0
        kannada_to_deva.insert('೧', '१');  // 1
        kannada_to_deva.insert('೨', '२');  // 2
        kannada_to_deva.insert('೩', '३');  // 3
        kannada_to_deva.insert('೪', '४');  // 4
        kannada_to_deva.insert('೫', '५');  // 5
        kannada_to_deva.insert('೬', '६');  // 6
        kannada_to_deva.insert('೭', '७');  // 7
        kannada_to_deva.insert('೮', '८');  // 8
        kannada_to_deva.insert('೯', '९');  // 9
        
        // Build reverse mapping for Devanagari → Kannada conversion
        let mut deva_to_kannada = HashMap::new();
        for (&kannada_char, &deva_char) in &kannada_to_deva {
            deva_to_kannada.insert(deva_char, kannada_char);
        }
        
        Self {
            kannada_to_deva_map: kannada_to_deva,
            deva_to_kannada_map: deva_to_kannada,
        }
    }
    
    /// Convert Kannada text to Devanagari
    pub fn kannada_to_devanagari(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        
        for ch in input.chars() {
            if let Some(&deva_char) = self.kannada_to_deva_map.get(&ch) {
                result.push(deva_char);
            } else {
                // Preserve characters not in mapping (punctuation, spaces, etc.)
                result.push(ch);
            }
        }
        
        Ok(result)
    }
    
    /// Convert Devanagari text to Kannada
    pub fn devanagari_to_kannada(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        
        for ch in input.chars() {
            if let Some(&kannada_char) = self.deva_to_kannada_map.get(&ch) {
                result.push(kannada_char);
            } else {
                // Preserve characters not in mapping (punctuation, spaces, etc.)
                result.push(ch);
            }
        }
        
        Ok(result)
    }
}

impl ScriptConverter for KannadaConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "kannada" && script != "kn" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Kannada converter only supports 'kannada' or 'kn' script".to_string(),
            });
        }
        
        let deva_text = self.kannada_to_devanagari(input)?;
        Ok(HubInput::Devanagari(deva_text))
    }
    
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        if script != "kannada" && script != "kn" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Kannada converter only supports 'kannada' or 'kn' script".to_string(),
            });
        }
        
        match hub_input {
            HubInput::Devanagari(deva_text) => self.devanagari_to_kannada(deva_text),
            HubInput::Iso(_) => Err(ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: "Kannada converter expects Devanagari input, got ISO".to_string(),
            }),
        }
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["kannada", "kn"]
    }
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        // Kannada is an Indic script - consonants DO have implicit 'a'
        true
    }
}

impl Default for KannadaConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_kannada_basic_vowels() {
        let converter = KannadaConverter::new();
        
        // Test basic vowel conversion
        let test_cases = vec![
            ("ಅ", "अ"),  // a
            ("ಆ", "आ"),  // ā
            ("ಇ", "इ"),  // i
            ("ಈ", "ई"),  // ī
            ("ಉ", "उ"),  // u
            ("ಊ", "ऊ"),  // ū
            ("ಋ", "ऋ"),  // r̥
            ("ಎ", "ए"),  // e
            ("ಐ", "ऐ"),  // ai
            ("ಒ", "ओ"),  // o
            ("ಔ", "औ"),  // au
        ];
        
        for (kannada_input, expected_deva) in test_cases {
            let result = converter.kannada_to_devanagari(kannada_input).unwrap();
            assert_eq!(result, expected_deva, 
                "Kannada vowel '{}' should convert to Devanagari '{}'", 
                kannada_input, expected_deva);
        }
    }
    
    #[test]
    fn test_kannada_consonants() {
        let converter = KannadaConverter::new();
        
        // Test consonant conversion (with inherent 'a')
        let test_cases = vec![
            ("ಕ", "क"),  // ka
            ("ಖ", "ख"),  // kha
            ("ಗ", "ग"),  // ga
            ("ಘ", "घ"),  // gha
            ("ಚ", "च"),  // ca
            ("ಜ", "ज"),  // ja
            ("ಟ", "ट"),  // ṭa
            ("ಡ", "ड"),  // ḍa
            ("ತ", "त"),  // ta
            ("ದ", "द"),  // da
            ("ನ", "न"),  // na
            ("ಪ", "प"),  // pa
            ("ಬ", "ब"),  // ba
            ("ಮ", "म"),  // ma
            ("ಯ", "य"),  // ya
            ("ರ", "र"),  // ra
            ("ಲ", "ल"),  // la
            ("ವ", "व"),  // va
            ("ಶ", "श"),  // śa
            ("ಷ", "ष"),  // ṣa
            ("ಸ", "स"),  // sa
            ("ಹ", "ह"),  // ha
        ];
        
        for (kannada_input, expected_deva) in test_cases {
            let result = converter.kannada_to_devanagari(kannada_input).unwrap();
            assert_eq!(result, expected_deva, 
                "Kannada consonant '{}' should convert to Devanagari '{}'", 
                kannada_input, expected_deva);
        }
    }
    
    #[test]
    fn test_kannada_dharma_word() {
        let converter = KannadaConverter::new();
        
        // Test the word "dharma" in Kannada: ಧರ್ಮ
        let kannada_dharma = "ಧರ್ಮ";
        let expected_deva = "धर्म";
        
        let result = converter.kannada_to_devanagari(kannada_dharma).unwrap();
        assert_eq!(result, expected_deva, 
            "Kannada 'ಧರ್ಮ' should convert to Devanagari 'धर्म'");
            
        // Test reverse conversion
        let reverse_result = converter.devanagari_to_kannada(expected_deva).unwrap();
        assert_eq!(reverse_result, kannada_dharma,
            "Devanagari 'धर्म' should convert back to Kannada 'ಧರ್ಮ'");
    }
    
    #[test]
    fn test_kannada_vowel_signs() {
        let converter = KannadaConverter::new();
        
        // Test vowel signs (dependent forms)
        let test_cases = vec![
            ("ಕಾ", "का"),  // kā
            ("ಕಿ", "कि"),  // ki
            ("ಕೀ", "की"),  // kī
            ("ಕು", "कु"),  // ku
            ("ಕೂ", "कू"),  // kū
            ("ಕೆ", "के"),  // ke
            ("ಕೈ", "कै"),  // kai
            ("ಕೊ", "को"),  // ko
            ("ಕೌ", "कौ"),  // kau
        ];
        
        for (kannada_input, expected_deva) in test_cases {
            let result = converter.kannada_to_devanagari(kannada_input).unwrap();
            assert_eq!(result, expected_deva, 
                "Kannada '{}' should convert to Devanagari '{}'", 
                kannada_input, expected_deva);
        }
    }
    
    #[test]
    fn test_script_converter_interface() {
        let converter = KannadaConverter::new();
        
        // Test the ScriptConverter interface
        assert!(converter.supports_script("kannada"));
        assert!(converter.supports_script("kn"));
        assert!(!converter.supports_script("tamil"));
        
        // Test script_has_implicit_a
        assert!(converter.script_has_implicit_a("kannada"));
        assert!(converter.script_has_implicit_a("kn"));
        
        let result = converter.to_hub("kannada", "ಕ").unwrap();
        if let HubInput::Devanagari(deva_text) = result {
            assert_eq!(deva_text, "क");
        } else {
            panic!("Expected HubInput::Devanagari");
        }
    }
}