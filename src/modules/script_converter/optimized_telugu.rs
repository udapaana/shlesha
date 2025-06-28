use std::collections::HashMap;
use super::{ScriptConverter, ConverterError};
use super::optimized_processors::OptimizedIndicScriptProcessor;
use crate::modules::hub::HubInput;

/// Optimized Telugu script converter with reduced string allocations
pub struct OptimizedTeluguConverter {
    telugu_to_deva_map: HashMap<char, char>,
    deva_to_telugu_map: HashMap<char, char>,
    // Pre-built maps for the optimized processor (char keys instead of string keys)
    telugu_consonants: HashMap<char, &'static str>,
    telugu_vowels: HashMap<char, &'static str>,
    telugu_vowel_signs: HashMap<char, &'static str>,
    telugu_misc: HashMap<char, &'static str>,
}

impl OptimizedTeluguConverter {
    pub fn new() -> Self {
        let mut telugu_to_deva = HashMap::new();
        
        // Same mappings as original TeluguConverter but organized for optimization
        
        // Independent vowels (అచ్చులు) → Devanagari equivalents
        telugu_to_deva.insert('అ', 'अ');
        telugu_to_deva.insert('ఆ', 'आ');
        telugu_to_deva.insert('ఇ', 'इ');
        telugu_to_deva.insert('ఈ', 'ई');
        telugu_to_deva.insert('ఉ', 'उ');
        telugu_to_deva.insert('ఊ', 'ऊ');
        telugu_to_deva.insert('ఋ', 'ऋ');
        telugu_to_deva.insert('ౠ', 'ॠ');
        telugu_to_deva.insert('ఌ', 'ऌ');
        telugu_to_deva.insert('ౡ', 'ॡ');
        telugu_to_deva.insert('ఎ', 'ए');
        telugu_to_deva.insert('ఏ', 'ए');
        telugu_to_deva.insert('ఐ', 'ऐ');
        telugu_to_deva.insert('ఒ', 'ओ');
        telugu_to_deva.insert('ఓ', 'ओ');
        telugu_to_deva.insert('ఔ', 'औ');
        
        // Vowel diacritics (పిల్లలు)
        telugu_to_deva.insert('ా', 'ा');
        telugu_to_deva.insert('ి', 'ि');
        telugu_to_deva.insert('ీ', 'ी');
        telugu_to_deva.insert('ు', 'ु');
        telugu_to_deva.insert('ూ', 'ू');
        telugu_to_deva.insert('ృ', 'ृ');
        telugu_to_deva.insert('ౄ', 'ॄ');
        telugu_to_deva.insert('ె', 'े');
        telugu_to_deva.insert('ే', 'े');
        telugu_to_deva.insert('ై', 'ै');
        telugu_to_deva.insert('ొ', 'ो');
        telugu_to_deva.insert('ో', 'ो');
        telugu_to_deva.insert('ౌ', 'ौ');
        
        // Consonants (హల్లులు)
        telugu_to_deva.insert('క', 'क');
        telugu_to_deva.insert('ఖ', 'ख');
        telugu_to_deva.insert('గ', 'ग');
        telugu_to_deva.insert('ఘ', 'घ');
        telugu_to_deva.insert('ఙ', 'ङ');
        telugu_to_deva.insert('చ', 'च');
        telugu_to_deva.insert('ఛ', 'छ');
        telugu_to_deva.insert('జ', 'ज');
        telugu_to_deva.insert('ఝ', 'झ');
        telugu_to_deva.insert('ఞ', 'ञ');
        telugu_to_deva.insert('ట', 'ट');
        telugu_to_deva.insert('ఠ', 'ठ');
        telugu_to_deva.insert('డ', 'ड');
        telugu_to_deva.insert('ఢ', 'ढ');
        telugu_to_deva.insert('ణ', 'ण');
        telugu_to_deva.insert('త', 'त');
        telugu_to_deva.insert('థ', 'थ');
        telugu_to_deva.insert('ద', 'द');
        telugu_to_deva.insert('ధ', 'ध');
        telugu_to_deva.insert('న', 'न');
        telugu_to_deva.insert('ప', 'प');
        telugu_to_deva.insert('ఫ', 'फ');
        telugu_to_deva.insert('బ', 'ब');
        telugu_to_deva.insert('భ', 'भ');
        telugu_to_deva.insert('మ', 'म');
        telugu_to_deva.insert('య', 'य');
        telugu_to_deva.insert('ర', 'र');
        telugu_to_deva.insert('ల', 'ल');
        telugu_to_deva.insert('వ', 'व');
        telugu_to_deva.insert('శ', 'श');
        telugu_to_deva.insert('ష', 'ष');
        telugu_to_deva.insert('స', 'स');
        telugu_to_deva.insert('హ', 'ह');
        telugu_to_deva.insert('ళ', 'ळ');
        
        // Special marks
        telugu_to_deva.insert('ం', 'ं');  // Anusvara
        telugu_to_deva.insert('ః', 'ः');  // Visarga
        telugu_to_deva.insert('్', '्');   // Halant/Virama
        
        // Build reverse mapping
        let mut deva_to_telugu = HashMap::new();
        for (&telugu_char, &deva_char) in &telugu_to_deva {
            deva_to_telugu.insert(deva_char, telugu_char);
        }
        
        // Build optimized maps for the processor (no string allocations!)
        let mut telugu_consonants = HashMap::new();
        let mut telugu_vowels = HashMap::new();
        let mut telugu_vowel_signs = HashMap::new();
        let mut telugu_misc = HashMap::new();
        
        // Consonants - map directly to single char string literals
        telugu_consonants.insert('క', "क");
        telugu_consonants.insert('ఖ', "ख");
        telugu_consonants.insert('గ', "ग");
        telugu_consonants.insert('ఘ', "घ");
        telugu_consonants.insert('ఙ', "ङ");
        telugu_consonants.insert('చ', "च");
        telugu_consonants.insert('ఛ', "छ");
        telugu_consonants.insert('జ', "ज");
        telugu_consonants.insert('ఝ', "झ");
        telugu_consonants.insert('ఞ', "ञ");
        telugu_consonants.insert('ట', "ट");
        telugu_consonants.insert('ఠ', "ठ");
        telugu_consonants.insert('డ', "ड");
        telugu_consonants.insert('ఢ', "ढ");
        telugu_consonants.insert('ణ', "ण");
        telugu_consonants.insert('త', "त");
        telugu_consonants.insert('థ', "थ");
        telugu_consonants.insert('ద', "द");
        telugu_consonants.insert('ధ', "ध");
        telugu_consonants.insert('న', "न");
        telugu_consonants.insert('ప', "प");
        telugu_consonants.insert('ఫ', "फ");
        telugu_consonants.insert('బ', "ब");
        telugu_consonants.insert('భ', "भ");
        telugu_consonants.insert('మ', "म");
        telugu_consonants.insert('య', "य");
        telugu_consonants.insert('ర', "र");
        telugu_consonants.insert('ల', "ल");
        telugu_consonants.insert('వ', "व");
        telugu_consonants.insert('శ', "श");
        telugu_consonants.insert('ష', "ष");
        telugu_consonants.insert('స', "स");
        telugu_consonants.insert('హ', "ह");
        telugu_consonants.insert('ళ', "ळ");
        
        // Independent vowels
        telugu_vowels.insert('అ', "अ");
        telugu_vowels.insert('ఆ', "आ");
        telugu_vowels.insert('ఇ', "इ");
        telugu_vowels.insert('ఈ', "ई");
        telugu_vowels.insert('ఉ', "उ");
        telugu_vowels.insert('ఊ', "ऊ");
        telugu_vowels.insert('ఋ', "ऋ");
        telugu_vowels.insert('ౠ', "ॠ");
        telugu_vowels.insert('ఌ', "ऌ");
        telugu_vowels.insert('ౡ', "ॡ");
        telugu_vowels.insert('ఎ', "ए");
        telugu_vowels.insert('ఏ', "ए");
        telugu_vowels.insert('ఐ', "ऐ");
        telugu_vowels.insert('ఒ', "ओ");
        telugu_vowels.insert('ఓ', "ओ");
        telugu_vowels.insert('ఔ', "औ");
        
        // Vowel signs (diacritics)
        telugu_vowel_signs.insert('ా', "ा");
        telugu_vowel_signs.insert('ి', "ि");
        telugu_vowel_signs.insert('ీ', "ी");
        telugu_vowel_signs.insert('ు', "ु");
        telugu_vowel_signs.insert('ూ', "ू");
        telugu_vowel_signs.insert('ృ', "ृ");
        telugu_vowel_signs.insert('ౄ', "ॄ");
        telugu_vowel_signs.insert('ె', "े");
        telugu_vowel_signs.insert('ే', "े");
        telugu_vowel_signs.insert('ై', "ै");
        telugu_vowel_signs.insert('ొ', "ो");
        telugu_vowel_signs.insert('ో', "ो");
        telugu_vowel_signs.insert('ౌ', "ौ");
        
        // Misc characters
        telugu_misc.insert('ం', "ं");
        telugu_misc.insert('ః', "ः");
        
        Self {
            telugu_to_deva_map: telugu_to_deva,
            deva_to_telugu_map: deva_to_telugu,
            telugu_consonants,
            telugu_vowels,
            telugu_vowel_signs,
            telugu_misc,
        }
    }
    
    /// Convert Telugu text to Devanagari using optimized processor
    pub fn telugu_to_deva_optimized(&self, input: &str) -> Result<String, ConverterError> {
        OptimizedIndicScriptProcessor::to_hub_optimized(
            input,
            &self.telugu_consonants,
            &self.telugu_vowels,
            &self.telugu_vowel_signs,
            &self.telugu_misc,
            '్', // Telugu virama
        )
    }
}

impl ScriptConverter for OptimizedTeluguConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "telugu" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Telugu converter only supports 'telugu' script".to_string(),
            });
        }
        
        let deva_text = self.telugu_to_deva_optimized(input)?;
        Ok(HubInput::Devanagari(deva_text))
    }
    
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        if script != "telugu" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Telugu converter only supports 'telugu' script".to_string(),
            });
        }
        
        match hub_input {
            HubInput::Devanagari(deva_text) => {
                // Simple character-by-character reverse mapping
                let mut result = String::with_capacity(deva_text.len());
                for ch in deva_text.chars() {
                    if let Some(&telugu_char) = self.deva_to_telugu_map.get(&ch) {
                        result.push(telugu_char);
                    } else {
                        result.push(ch); // Preserve unknown characters
                    }
                }
                Ok(result)
            }
            HubInput::Iso(_) => Err(ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: "Telugu converter expects Devanagari hub input, got ISO".to_string(),
            }),
        }
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["telugu"]
    }
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        true // Telugu has implicit 'a' vowel in consonants
    }
}

impl Default for OptimizedTeluguConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_optimized_telugu_basic_vowels() {
        let converter = OptimizedTeluguConverter::new();
        
        // Test basic vowels
        assert_eq!(converter.telugu_to_deva_optimized("అ").unwrap(), "अ");
        assert_eq!(converter.telugu_to_deva_optimized("ఆ").unwrap(), "आ");
        assert_eq!(converter.telugu_to_deva_optimized("ఇ").unwrap(), "इ");
        assert_eq!(converter.telugu_to_deva_optimized("ఈ").unwrap(), "ई");
    }
    
    #[test]
    fn test_optimized_telugu_consonants() {
        let converter = OptimizedTeluguConverter::new();
        
        // Test consonants (should add implicit 'a')
        assert_eq!(converter.telugu_to_deva_optimized("క").unwrap(), "का");
        assert_eq!(converter.telugu_to_deva_optimized("త").unwrap(), "ता");
        assert_eq!(converter.telugu_to_deva_optimized("మ").unwrap(), "मा");
    }
    
    #[test]
    fn test_optimized_script_converter_interface() {
        let converter = OptimizedTeluguConverter::new();
        
        assert!(converter.supports_script("telugu"));
        assert!(!converter.supports_script("hindi"));
        assert!(converter.script_has_implicit_a("telugu"));
        
        let result = converter.to_hub("telugu", "తెలుగు").unwrap();
        match result {
            HubInput::Devanagari(text) => {
                assert!(!text.is_empty());
            }
            _ => panic!("Expected Devanagari output"),
        }
    }
}