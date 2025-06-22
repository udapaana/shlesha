use std::collections::HashMap;
use super::{ScriptConverter, ConverterError};
use crate::modules::hub::HubInput;

/// Malayalam script converter
/// 
/// Malayalam (മലയാളം) is a Dravidian language written in the Malayalam script,
/// primarily used in Kerala, India. The script is derived from the Brahmi script
/// and has unique characteristics including extensive use of conjunct consonants.
pub struct MalayalamConverter {
    malayalam_to_deva_map: HashMap<char, char>,
    deva_to_malayalam_map: HashMap<char, char>,
}

impl MalayalamConverter {
    pub fn new() -> Self {
        let mut malayalam_to_deva = HashMap::new();
        
        // Vowels (independent forms)
        malayalam_to_deva.insert('അ', 'अ');  // a
        malayalam_to_deva.insert('ആ', 'आ');  // ā
        malayalam_to_deva.insert('ഇ', 'इ');  // i
        malayalam_to_deva.insert('ഈ', 'ई');  // ī
        malayalam_to_deva.insert('ഉ', 'उ');  // u
        malayalam_to_deva.insert('ഊ', 'ऊ');  // ū
        malayalam_to_deva.insert('ഋ', 'ऋ');  // r̥
        malayalam_to_deva.insert('ൠ', 'ॠ');  // r̥̄
        malayalam_to_deva.insert('ഌ', 'ऌ');  // l̥
        malayalam_to_deva.insert('ൡ', 'ॡ');  // l̥̄
        malayalam_to_deva.insert('എ', 'ए');  // e
        malayalam_to_deva.insert('ഏ', 'ए');  // ē (mapped to e for simplicity)
        malayalam_to_deva.insert('ഐ', 'ऐ');  // ai
        malayalam_to_deva.insert('ഒ', 'ओ');  // o
        malayalam_to_deva.insert('ഓ', 'ओ');  // ō (mapped to o for simplicity)
        malayalam_to_deva.insert('ഔ', 'औ');  // au
        
        // Consonants (with inherent 'a')
        malayalam_to_deva.insert('ക', 'क');  // ka
        malayalam_to_deva.insert('ഖ', 'ख');  // kha
        malayalam_to_deva.insert('ഗ', 'ग');  // ga
        malayalam_to_deva.insert('ഘ', 'घ');  // gha
        malayalam_to_deva.insert('ങ', 'ङ');  // ṅa
        
        malayalam_to_deva.insert('ച', 'च');  // ca
        malayalam_to_deva.insert('ഛ', 'छ');  // cha
        malayalam_to_deva.insert('ജ', 'ज');  // ja
        malayalam_to_deva.insert('ഝ', 'झ');  // jha
        malayalam_to_deva.insert('ഞ', 'ञ');  // ña
        
        malayalam_to_deva.insert('ട', 'ट');  // ṭa
        malayalam_to_deva.insert('ഠ', 'ठ');  // ṭha
        malayalam_to_deva.insert('ഡ', 'ड');  // ḍa
        malayalam_to_deva.insert('ഢ', 'ढ');  // ḍha
        malayalam_to_deva.insert('ണ', 'ण');  // ṇa
        
        malayalam_to_deva.insert('ത', 'त');  // ta
        malayalam_to_deva.insert('ഥ', 'थ');  // tha
        malayalam_to_deva.insert('ദ', 'द');  // da
        malayalam_to_deva.insert('ധ', 'ध');  // dha
        malayalam_to_deva.insert('ന', 'न');  // na
        
        malayalam_to_deva.insert('പ', 'प');  // pa
        malayalam_to_deva.insert('ഫ', 'फ');  // pha
        malayalam_to_deva.insert('ബ', 'ब');  // ba
        malayalam_to_deva.insert('ഭ', 'भ');  // bha
        malayalam_to_deva.insert('മ', 'म');  // ma
        
        malayalam_to_deva.insert('യ', 'य');  // ya
        malayalam_to_deva.insert('ര', 'र');  // ra
        malayalam_to_deva.insert('ല', 'ल');  // la
        malayalam_to_deva.insert('വ', 'व');  // va
        
        malayalam_to_deva.insert('ശ', 'श');  // śa
        malayalam_to_deva.insert('ഷ', 'ष');  // ṣa
        malayalam_to_deva.insert('സ', 'स');  // sa
        malayalam_to_deva.insert('ഹ', 'ह');  // ha
        
        // Additional Malayalam consonants
        malayalam_to_deva.insert('ള', 'ळ');  // ḷa (retroflex L)
        malayalam_to_deva.insert('ഴ', 'ऴ');  // ḻa (retroflex/Tamil ḻa)
        malayalam_to_deva.insert('റ', 'र');  // ṟa (alveolar R, mapped to regular ra)
        
        // Vowel signs (dependent forms)
        malayalam_to_deva.insert('ാ', 'ा');  // ā sign
        malayalam_to_deva.insert('ി', 'ि');  // i sign  
        malayalam_to_deva.insert('ീ', 'ी');  // ī sign
        malayalam_to_deva.insert('ു', 'ु');  // u sign
        malayalam_to_deva.insert('ൂ', 'ू');  // ū sign
        malayalam_to_deva.insert('ൃ', 'ृ');  // r̥ sign
        malayalam_to_deva.insert('ൄ', 'ॄ');  // r̥̄ sign
        malayalam_to_deva.insert('െ', 'े');  // e sign
        malayalam_to_deva.insert('േ', 'े');  // ē sign (mapped to e)
        malayalam_to_deva.insert('ൈ', 'ै');  // ai sign
        malayalam_to_deva.insert('ൊ', 'ो');  // o sign
        malayalam_to_deva.insert('ോ', 'ो');  // ō sign (mapped to o)
        malayalam_to_deva.insert('ൌ', 'ौ');  // au sign
        
        // Special marks
        malayalam_to_deva.insert('ം', 'ं');  // anusvara
        malayalam_to_deva.insert('ഃ', 'ः');  // visarga
        malayalam_to_deva.insert('്', '्');  // virama (chandrakkala)
        
        // Digits
        malayalam_to_deva.insert('൦', '०');  // 0
        malayalam_to_deva.insert('൧', '१');  // 1
        malayalam_to_deva.insert('൨', '२');  // 2
        malayalam_to_deva.insert('൩', '३');  // 3
        malayalam_to_deva.insert('൪', '४');  // 4
        malayalam_to_deva.insert('൫', '५');  // 5
        malayalam_to_deva.insert('൬', '६');  // 6
        malayalam_to_deva.insert('൭', '७');  // 7
        malayalam_to_deva.insert('൮', '८');  // 8
        malayalam_to_deva.insert('൯', '९');  // 9
        
        // Build reverse mapping for Devanagari → Malayalam conversion
        let mut deva_to_malayalam = HashMap::new();
        for (&malayalam_char, &deva_char) in &malayalam_to_deva {
            deva_to_malayalam.insert(deva_char, malayalam_char);
        }
        
        // Override ambiguous mappings with preferred forms
        // For 'र', prefer standard RA (ര) over alveolar RRA (റ)
        deva_to_malayalam.insert('र', 'ര');
        
        Self {
            malayalam_to_deva_map: malayalam_to_deva,
            deva_to_malayalam_map: deva_to_malayalam,
        }
    }
    
    /// Convert Malayalam text to Devanagari
    pub fn malayalam_to_devanagari(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        
        for ch in input.chars() {
            if let Some(&deva_char) = self.malayalam_to_deva_map.get(&ch) {
                result.push(deva_char);
            } else {
                // Preserve characters not in mapping (punctuation, spaces, etc.)
                result.push(ch);
            }
        }
        
        Ok(result)
    }
    
    /// Convert Devanagari text to Malayalam
    pub fn devanagari_to_malayalam(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        
        for ch in input.chars() {
            if let Some(&malayalam_char) = self.deva_to_malayalam_map.get(&ch) {
                result.push(malayalam_char);
            } else {
                // Preserve characters not in mapping (punctuation, spaces, etc.)
                result.push(ch);
            }
        }
        
        Ok(result)
    }
}

impl ScriptConverter for MalayalamConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "malayalam" && script != "ml" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Malayalam converter only supports 'malayalam' or 'ml' script".to_string(),
            });
        }
        
        let deva_text = self.malayalam_to_devanagari(input)?;
        Ok(HubInput::Devanagari(deva_text))
    }
    
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        if script != "malayalam" && script != "ml" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Malayalam converter only supports 'malayalam' or 'ml' script".to_string(),
            });
        }
        
        match hub_input {
            HubInput::Devanagari(deva_text) => self.devanagari_to_malayalam(deva_text),
            HubInput::Iso(_) => Err(ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: "Malayalam converter expects Devanagari input, got ISO".to_string(),
            }),
        }
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["malayalam", "ml"]
    }
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        // Malayalam is an Indic script - consonants DO have implicit 'a'
        true
    }
}

impl Default for MalayalamConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_malayalam_basic_vowels() {
        let converter = MalayalamConverter::new();
        
        // Test basic vowel conversion
        let test_cases = vec![
            ("അ", "अ"),  // a
            ("ആ", "आ"),  // ā
            ("ഇ", "इ"),  // i
            ("ഈ", "ई"),  // ī
            ("ഉ", "उ"),  // u
            ("ഊ", "ऊ"),  // ū
            ("ഋ", "ऋ"),  // r̥
            ("എ", "ए"),  // e
            ("ഐ", "ऐ"),  // ai
            ("ഒ", "ओ"),  // o
            ("ഔ", "औ"),  // au
        ];
        
        for (malayalam_input, expected_deva) in test_cases {
            let result = converter.malayalam_to_devanagari(malayalam_input).unwrap();
            assert_eq!(result, expected_deva, 
                "Malayalam vowel '{}' should convert to Devanagari '{}'", 
                malayalam_input, expected_deva);
        }
    }
    
    #[test]
    fn test_malayalam_consonants() {
        let converter = MalayalamConverter::new();
        
        // Test consonant conversion (with inherent 'a')
        let test_cases = vec![
            ("ക", "क"),  // ka
            ("ഖ", "ख"),  // kha
            ("ഗ", "ग"),  // ga
            ("ഘ", "घ"),  // gha
            ("ച", "च"),  // ca
            ("ജ", "ज"),  // ja
            ("ട", "ट"),  // ṭa
            ("ഡ", "ड"),  // ḍa
            ("ത", "त"),  // ta
            ("ദ", "द"),  // da
            ("ന", "न"),  // na
            ("പ", "प"),  // pa
            ("ബ", "ब"),  // ba
            ("മ", "म"),  // ma
            ("യ", "य"),  // ya
            ("ര", "र"),  // ra
            ("ല", "ल"),  // la
            ("വ", "व"),  // va
            ("ശ", "श"),  // śa
            ("ഷ", "ष"),  // ṣa
            ("സ", "स"),  // sa
            ("ഹ", "ह"),  // ha
        ];
        
        for (malayalam_input, expected_deva) in test_cases {
            let result = converter.malayalam_to_devanagari(malayalam_input).unwrap();
            assert_eq!(result, expected_deva, 
                "Malayalam consonant '{}' should convert to Devanagari '{}'", 
                malayalam_input, expected_deva);
        }
    }
    
    #[test]
    fn test_malayalam_dharma_word() {
        let converter = MalayalamConverter::new();
        
        // Test the word "dharma" in Malayalam: ധര്‍മ or ധര്മ
        let malayalam_dharma = "ധര്മ";
        let expected_deva = "धर्म";
        
        let result = converter.malayalam_to_devanagari(malayalam_dharma).unwrap();
        assert_eq!(result, expected_deva, 
            "Malayalam 'ധര്മ' should convert to Devanagari 'धर्म'");
            
        // Test reverse conversion
        let reverse_result = converter.devanagari_to_malayalam(expected_deva).unwrap();
        assert_eq!(reverse_result, malayalam_dharma,
            "Devanagari 'धर्म' should convert back to Malayalam 'ധര്മ'");
    }
    
    #[test]
    fn test_malayalam_vowel_signs() {
        let converter = MalayalamConverter::new();
        
        // Test vowel signs (dependent forms)
        let test_cases = vec![
            ("കാ", "का"),  // kā
            ("കി", "कि"),  // ki
            ("കീ", "की"),  // kī
            ("കു", "कु"),  // ku
            ("കൂ", "कू"),  // kū
            ("കെ", "के"),  // ke
            ("കൈ", "कै"),  // kai
            ("കൊ", "को"),  // ko
            ("കൌ", "कौ"),  // kau
        ];
        
        for (malayalam_input, expected_deva) in test_cases {
            let result = converter.malayalam_to_devanagari(malayalam_input).unwrap();
            assert_eq!(result, expected_deva, 
                "Malayalam '{}' should convert to Devanagari '{}'", 
                malayalam_input, expected_deva);
        }
    }
    
    #[test]
    fn test_script_converter_interface() {
        let converter = MalayalamConverter::new();
        
        // Test the ScriptConverter interface
        assert!(converter.supports_script("malayalam"));
        assert!(converter.supports_script("ml"));
        assert!(!converter.supports_script("tamil"));
        
        // Test script_has_implicit_a
        assert!(converter.script_has_implicit_a("malayalam"));
        assert!(converter.script_has_implicit_a("ml"));
        
        let result = converter.to_hub("malayalam", "ക").unwrap();
        if let HubInput::Devanagari(deva_text) = result {
            assert_eq!(deva_text, "क");
        } else {
            panic!("Expected HubInput::Devanagari");
        }
    }
}