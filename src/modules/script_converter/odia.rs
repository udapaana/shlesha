use std::collections::HashMap;
use super::{ScriptConverter, ConverterError};
use crate::modules::hub::HubInput;

/// Odia script converter
/// 
/// Odia (ଓଡ଼ିଆ) is an Indo-Aryan language written in the Odia script,
/// primarily used in Odisha, India. The script is derived from the Brahmi script
/// and has rounded characters similar to Telugu and Kannada scripts.
pub struct OdiaConverter {
    odia_to_deva_map: HashMap<char, char>,
    deva_to_odia_map: HashMap<char, char>,
    odia_nukta_to_deva: HashMap<String, String>,
    deva_nukta_to_odia: HashMap<String, String>,
}

impl OdiaConverter {
    pub fn new() -> Self {
        let mut odia_to_deva = HashMap::new();
        
        // Vowels (independent forms)
        odia_to_deva.insert('ଅ', 'अ');  // a
        odia_to_deva.insert('ଆ', 'आ');  // ā
        odia_to_deva.insert('ଇ', 'इ');  // i
        odia_to_deva.insert('ଈ', 'ई');  // ī
        odia_to_deva.insert('ଉ', 'उ');  // u
        odia_to_deva.insert('ଊ', 'ऊ');  // ū
        odia_to_deva.insert('ଋ', 'ऋ');  // r̥
        odia_to_deva.insert('ୠ', 'ॠ');  // r̥̄
        odia_to_deva.insert('ଌ', 'ऌ');  // l̥
        odia_to_deva.insert('ୡ', 'ॡ');  // l̥̄
        odia_to_deva.insert('ଏ', 'ए');  // e
        odia_to_deva.insert('ଐ', 'ऐ');  // ai
        odia_to_deva.insert('ଓ', 'ओ');  // o
        odia_to_deva.insert('ଔ', 'औ');  // au
        
        // Consonants (with inherent 'a')
        odia_to_deva.insert('କ', 'क');  // ka
        odia_to_deva.insert('ଖ', 'ख');  // kha
        odia_to_deva.insert('ଗ', 'ग');  // ga
        odia_to_deva.insert('ଘ', 'घ');  // gha
        odia_to_deva.insert('ଙ', 'ङ');  // ṅa
        
        odia_to_deva.insert('ଚ', 'च');  // ca
        odia_to_deva.insert('ଛ', 'छ');  // cha
        odia_to_deva.insert('ଜ', 'ज');  // ja
        odia_to_deva.insert('ଝ', 'झ');  // jha
        odia_to_deva.insert('ଞ', 'ञ');  // ña
        
        odia_to_deva.insert('ଟ', 'ट');  // ṭa
        odia_to_deva.insert('ଠ', 'ठ');  // ṭha
        odia_to_deva.insert('ଡ', 'ड');  // ḍa
        odia_to_deva.insert('ଢ', 'ढ');  // ḍha
        odia_to_deva.insert('ଣ', 'ण');  // ṇa
        
        odia_to_deva.insert('ତ', 'त');  // ta
        odia_to_deva.insert('ଥ', 'थ');  // tha
        odia_to_deva.insert('ଦ', 'द');  // da
        odia_to_deva.insert('ଧ', 'ध');  // dha
        odia_to_deva.insert('ନ', 'न');  // na
        
        odia_to_deva.insert('ପ', 'प');  // pa
        odia_to_deva.insert('ଫ', 'फ');  // pha
        odia_to_deva.insert('ବ', 'ब');  // ba
        odia_to_deva.insert('ଭ', 'भ');  // bha
        odia_to_deva.insert('ମ', 'म');  // ma
        
        odia_to_deva.insert('ଯ', 'य');  // ya
        odia_to_deva.insert('ର', 'र');  // ra
        odia_to_deva.insert('ଲ', 'ल');  // la
        odia_to_deva.insert('ଵ', 'व');  // va (Odia uses ଵ for va)
        odia_to_deva.insert('ୱ', 'व');  // alternative va
        
        odia_to_deva.insert('ଶ', 'श');  // śa
        odia_to_deva.insert('ଷ', 'ष');  // ṣa
        odia_to_deva.insert('ସ', 'स');  // sa
        odia_to_deva.insert('ହ', 'ह');  // ha
        
        // Additional Odia consonants
        odia_to_deva.insert('ଳ', 'ळ');  // ḷa (retroflex L)
        
        // Note: Nukta characters like ଡ଼, ଢ଼ are composite and would need special handling
        // For now, skipping these to keep the basic converter simple
        let odia_nukta_to_deva: HashMap<String, String> = HashMap::new();  // Empty for now
        
        // Vowel signs (dependent forms)
        odia_to_deva.insert('ା', 'ा');  // ā sign
        odia_to_deva.insert('ି', 'ि');  // i sign  
        odia_to_deva.insert('ୀ', 'ी');  // ī sign
        odia_to_deva.insert('ୁ', 'ु');  // u sign
        odia_to_deva.insert('ୂ', 'ू');  // ū sign
        odia_to_deva.insert('ୃ', 'ृ');  // r̥ sign
        odia_to_deva.insert('ୄ', 'ॄ');  // r̥̄ sign
        odia_to_deva.insert('େ', 'े');  // e sign
        odia_to_deva.insert('ୈ', 'ै');  // ai sign
        odia_to_deva.insert('ୋ', 'ो');  // o sign
        odia_to_deva.insert('ୌ', 'ौ');  // au sign
        
        // Special marks
        odia_to_deva.insert('ଂ', 'ं');  // anusvara
        odia_to_deva.insert('ଃ', 'ः');  // visarga
        odia_to_deva.insert('୍', '्');  // virama (halanta)
        
        // Digits
        odia_to_deva.insert('୦', '०');  // 0
        odia_to_deva.insert('୧', '१');  // 1
        odia_to_deva.insert('୨', '२');  // 2
        odia_to_deva.insert('୩', '३');  // 3
        odia_to_deva.insert('୪', '४');  // 4
        odia_to_deva.insert('୫', '५');  // 5
        odia_to_deva.insert('୬', '६');  // 6
        odia_to_deva.insert('୭', '७');  // 7
        odia_to_deva.insert('୮', '८');  // 8
        odia_to_deva.insert('୯', '९');  // 9
        
        // Build reverse mapping for Devanagari → Odia conversion
        let mut deva_to_odia = HashMap::new();
        for (&odia_char, &deva_char) in &odia_to_deva {
            deva_to_odia.insert(deva_char, odia_char);
        }
        
        // Build reverse nukta mapping
        let mut deva_nukta_to_odia = HashMap::new();
        for (odia_nukta, deva_nukta) in &odia_nukta_to_deva {
            deva_nukta_to_odia.insert(deva_nukta.clone(), odia_nukta.clone());
        }
        
        Self {
            odia_to_deva_map: odia_to_deva,
            deva_to_odia_map: deva_to_odia,
            odia_nukta_to_deva,
            deva_nukta_to_odia,
        }
    }
    
    /// Convert Odia text to Devanagari
    pub fn odia_to_devanagari(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        
        for ch in input.chars() {
            if let Some(&deva_char) = self.odia_to_deva_map.get(&ch) {
                result.push(deva_char);
            } else {
                // Preserve characters not in mapping (punctuation, spaces, etc.)
                result.push(ch);
            }
        }
        
        Ok(result)
    }
    
    /// Convert Devanagari text to Odia
    pub fn devanagari_to_odia(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        
        for ch in input.chars() {
            if let Some(&odia_char) = self.deva_to_odia_map.get(&ch) {
                result.push(odia_char);
            } else {
                // Preserve characters not in mapping (punctuation, spaces, etc.)
                result.push(ch);
            }
        }
        
        Ok(result)
    }
}

impl ScriptConverter for OdiaConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "odia" && script != "od" && script != "oriya" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Odia converter only supports 'odia', 'od', or 'oriya' script".to_string(),
            });
        }
        
        let deva_text = self.odia_to_devanagari(input)?;
        Ok(HubInput::Devanagari(deva_text))
    }
    
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        if script != "odia" && script != "od" && script != "oriya" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Odia converter only supports 'odia', 'od', or 'oriya' script".to_string(),
            });
        }
        
        match hub_input {
            HubInput::Devanagari(deva_text) => self.devanagari_to_odia(deva_text),
            HubInput::Iso(_) => Err(ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: "Odia converter expects Devanagari input, got ISO".to_string(),
            }),
        }
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["odia", "od", "oriya"]
    }
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        // Odia is an Indic script - consonants DO have implicit 'a'
        true
    }
}

impl Default for OdiaConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_odia_basic_vowels() {
        let converter = OdiaConverter::new();
        
        // Test basic vowel conversion
        let test_cases = vec![
            ("ଅ", "अ"),  // a
            ("ଆ", "आ"),  // ā
            ("ଇ", "इ"),  // i
            ("ଈ", "ई"),  // ī
            ("ଉ", "उ"),  // u
            ("ଊ", "ऊ"),  // ū
            ("ଋ", "ऋ"),  // r̥
            ("ଏ", "ए"),  // e
            ("ଐ", "ऐ"),  // ai
            ("ଓ", "ओ"),  // o
            ("ଔ", "औ"),  // au
        ];
        
        for (odia_input, expected_deva) in test_cases {
            let result = converter.odia_to_devanagari(odia_input).unwrap();
            assert_eq!(result, expected_deva, 
                "Odia vowel '{}' should convert to Devanagari '{}'", 
                odia_input, expected_deva);
        }
    }
    
    #[test]
    fn test_odia_consonants() {
        let converter = OdiaConverter::new();
        
        // Test consonant conversion (with inherent 'a')
        let test_cases = vec![
            ("କ", "क"),  // ka
            ("ଖ", "ख"),  // kha
            ("ଗ", "ग"),  // ga
            ("ଘ", "घ"),  // gha
            ("ଚ", "च"),  // ca
            ("ଜ", "ज"),  // ja
            ("ଟ", "ट"),  // ṭa
            ("ଡ", "ड"),  // ḍa
            ("ତ", "त"),  // ta
            ("ଦ", "द"),  // da
            ("ନ", "न"),  // na
            ("ପ", "प"),  // pa
            ("ବ", "ब"),  // ba
            ("ମ", "म"),  // ma
            ("ଯ", "य"),  // ya
            ("ର", "र"),  // ra
            ("ଲ", "ल"),  // la
            ("ଵ", "व"),  // va
            ("ଶ", "श"),  // śa
            ("ଷ", "ष"),  // ṣa
            ("ସ", "स"),  // sa
            ("ହ", "ह"),  // ha
        ];
        
        for (odia_input, expected_deva) in test_cases {
            let result = converter.odia_to_devanagari(odia_input).unwrap();
            assert_eq!(result, expected_deva, 
                "Odia consonant '{}' should convert to Devanagari '{}'", 
                odia_input, expected_deva);
        }
    }
    
    #[test]
    fn test_odia_dharma_word() {
        let converter = OdiaConverter::new();
        
        // Test the word "dharma" in Odia: ଧର୍ମ
        let odia_dharma = "ଧର୍ମ";
        let expected_deva = "धर्म";
        
        let result = converter.odia_to_devanagari(odia_dharma).unwrap();
        assert_eq!(result, expected_deva, 
            "Odia 'ଧର୍ମ' should convert to Devanagari 'धर्म'");
            
        // Test reverse conversion
        let reverse_result = converter.devanagari_to_odia(expected_deva).unwrap();
        assert_eq!(reverse_result, odia_dharma,
            "Devanagari 'धर्म' should convert back to Odia 'ଧର୍ମ'");
    }
    
    #[test]
    fn test_odia_vowel_signs() {
        let converter = OdiaConverter::new();
        
        // Test vowel signs (dependent forms)
        let test_cases = vec![
            ("କା", "का"),  // kā
            ("କି", "कि"),  // ki
            ("କୀ", "की"),  // kī
            ("କୁ", "कु"),  // ku
            ("କୂ", "कू"),  // kū
            ("କେ", "के"),  // ke
            ("କୈ", "कै"),  // kai
            ("କୋ", "को"),  // ko
            ("କୌ", "कौ"),  // kau
        ];
        
        for (odia_input, expected_deva) in test_cases {
            let result = converter.odia_to_devanagari(odia_input).unwrap();
            assert_eq!(result, expected_deva, 
                "Odia '{}' should convert to Devanagari '{}'", 
                odia_input, expected_deva);
        }
    }
    
    #[test]
    fn test_script_converter_interface() {
        let converter = OdiaConverter::new();
        
        // Test the ScriptConverter interface
        assert!(converter.supports_script("odia"));
        assert!(converter.supports_script("od"));
        assert!(converter.supports_script("oriya"));
        assert!(!converter.supports_script("tamil"));
        
        // Test script_has_implicit_a
        assert!(converter.script_has_implicit_a("odia"));
        assert!(converter.script_has_implicit_a("od"));
        assert!(converter.script_has_implicit_a("oriya"));
        
        let result = converter.to_hub("odia", "କ").unwrap();
        if let HubInput::Devanagari(deva_text) = result {
            assert_eq!(deva_text, "क");
        } else {
            panic!("Expected HubInput::Devanagari");
        }
    }
}