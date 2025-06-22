use std::collections::HashMap;
use super::{ScriptConverter, ConverterError};
use crate::modules::hub::HubInput;

/// Tamil script converter
/// 
/// Tamil (தமிழ்) is an Indic script used primarily for the Tamil language.
/// This converter handles Tamil text by converting it directly to Devanagari equivalent,
/// which can then be processed by the hub. The hub handles all complex linguistic rules.
/// Tamil script has some unique features compared to other Indic scripts.
pub struct TamilConverter {
    tamil_to_deva_map: HashMap<char, char>,
    deva_to_tamil_map: HashMap<char, char>,
}

impl TamilConverter {
    pub fn new() -> Self {
        let mut tamil_to_deva = HashMap::new();
        
        // Simple character-to-character mapping from Tamil to Devanagari equivalents
        // Let the hub handle all complex linguistic rules (virama processing, etc.)
        
        // Independent vowels (உயிர்) → Devanagari equivalents
        tamil_to_deva.insert('அ', 'अ');       // Tamil அ → Devanagari अ
        tamil_to_deva.insert('ஆ', 'आ');       // Tamil ஆ → Devanagari आ
        tamil_to_deva.insert('இ', 'इ');       // Tamil இ → Devanagari इ
        tamil_to_deva.insert('ஈ', 'ई');       // Tamil ஈ → Devanagari ई
        tamil_to_deva.insert('உ', 'उ');       // Tamil உ → Devanagari उ
        tamil_to_deva.insert('ஊ', 'ऊ');       // Tamil ஊ → Devanagari ऊ
        tamil_to_deva.insert('எ', 'ए');       // Tamil எ → Devanagari ए
        tamil_to_deva.insert('ஏ', 'ए');       // Tamil ஏ → Devanagari ए (long e mapped to short e)
        tamil_to_deva.insert('ஐ', 'ऐ');       // Tamil ஐ → Devanagari ऐ
        tamil_to_deva.insert('ஒ', 'ओ');       // Tamil ஒ → Devanagari ओ
        tamil_to_deva.insert('ஓ', 'ओ');       // Tamil ஓ → Devanagari ओ (long o mapped to short o)
        tamil_to_deva.insert('ஔ', 'औ');       // Tamil ஔ → Devanagari औ
        
        // Vowel diacritics (உயிர்மெய்) → Devanagari equivalents
        tamil_to_deva.insert('ா', 'ा');       // Tamil ா → Devanagari ा
        tamil_to_deva.insert('ி', 'ि');       // Tamil ি → Devanagari ि
        tamil_to_deva.insert('ீ', 'ी');       // Tamil ீ → Devanagari ी
        tamil_to_deva.insert('ு', 'ु');       // Tamil ு → Devanagari ु
        tamil_to_deva.insert('ூ', 'ू');       // Tamil ூ → Devanagari ू
        tamil_to_deva.insert('ெ', 'े');       // Tamil ெ → Devanagari े
        tamil_to_deva.insert('ே', 'े');       // Tamil ே → Devanagari े
        tamil_to_deva.insert('ை', 'ै');       // Tamil ை → Devanagari ै
        tamil_to_deva.insert('ொ', 'ो');       // Tamil ொ → Devanagari ो
        tamil_to_deva.insert('ோ', 'ो');       // Tamil ோ → Devanagari ो
        tamil_to_deva.insert('ௌ', 'ौ');       // Tamil ௌ → Devanagari ौ
        
        // Consonants (மெய்) → Devanagari equivalents
        // Tamil has a limited set of consonants compared to other Indic scripts
        
        // Velar consonants
        tamil_to_deva.insert('க', 'क');       // Tamil க → Devanagari क
        tamil_to_deva.insert('ங', 'ङ');       // Tamil ங → Devanagari ङ
        
        // Palatal consonants  
        tamil_to_deva.insert('ச', 'च');       // Tamil ச → Devanagari च
        tamil_to_deva.insert('ஞ', 'ञ');       // Tamil ஞ → Devanagari ञ
        
        // Retroflex consonants (limited in Tamil)
        tamil_to_deva.insert('ட', 'ट');       // Tamil ட → Devanagari ट
        tamil_to_deva.insert('ண', 'ण');       // Tamil ண → Devanagari ण
        
        // Dental consonants
        tamil_to_deva.insert('த', 'त');       // Tamil த → Devanagari त
        tamil_to_deva.insert('ந', 'न');       // Tamil ந → Devanagari न
        
        // Labial consonants
        tamil_to_deva.insert('ப', 'प');       // Tamil ப → Devanagari प
        tamil_to_deva.insert('ம', 'म');       // Tamil ம → Devanagari म
        
        // Semivowels and liquids
        tamil_to_deva.insert('ய', 'य');       // Tamil ய → Devanagari य
        tamil_to_deva.insert('ர', 'र');       // Tamil ர → Devanagari र
        tamil_to_deva.insert('ல', 'ल');       // Tamil ல → Devanagari ल
        tamil_to_deva.insert('வ', 'व');       // Tamil வ → Devanagari व
        
        // Unique Tamil consonants (map to closest Devanagari equivalents)
        tamil_to_deva.insert('ழ', 'ळ');       // Tamil ழ → Devanagari ळ (retroflex l)
        tamil_to_deva.insert('ள', 'ळ');       // Tamil ள → Devanagari ळ (retroflex l variant)
        tamil_to_deva.insert('ற', 'र');       // Tamil ற → Devanagari र (alveolar trill → r)
        tamil_to_deva.insert('ன', 'न');       // Tamil ன → Devanagari न (alveolar n → n)
        
        // Sibilants and aspirate (limited in Tamil)
        tamil_to_deva.insert('ஶ', 'श');       // Tamil ஶ → Devanagari श (rare, used in Sanskrit loanwords)
        tamil_to_deva.insert('ஷ', 'ष');       // Tamil ஷ → Devanagari ष (rare, used in Sanskrit loanwords)
        tamil_to_deva.insert('ஸ', 'स');       // Tamil ஸ → Devanagari स (rare, used in Sanskrit loanwords)
        tamil_to_deva.insert('ஹ', 'ह');       // Tamil ஹ → Devanagari ह (rare, used in Sanskrit loanwords)
        
        // Additional characters for Sanskrit loanwords
        tamil_to_deva.insert('ஜ', 'ज');       // Tamil ஜ → Devanagari ज (Sanskrit loanword)
        tamil_to_deva.insert('ஃ', 'ः');       // Tamil ஃ → Devanagari ः (visarga - rare)
        
        // Special marks
        tamil_to_deva.insert('ஂ', 'ं');       // Tamil ஂ → Devanagari ं (anusvara - rare)
        tamil_to_deva.insert('்', '्');       // Tamil ் → Devanagari ् (pulli/virama)
        
        // Digits
        tamil_to_deva.insert('௦', '०');       // Tamil ௦ → Devanagari ०
        tamil_to_deva.insert('௧', '१');       // Tamil ௧ → Devanagari १
        tamil_to_deva.insert('௨', '२');       // Tamil ௨ → Devanagari २
        tamil_to_deva.insert('௩', '३');       // Tamil ௩ → Devanagari ३
        tamil_to_deva.insert('௪', '४');       // Tamil ௪ → Devanagari ४
        tamil_to_deva.insert('௫', '५');       // Tamil ௫ → Devanagari ५
        tamil_to_deva.insert('௬', '६');       // Tamil ௬ → Devanagari ६
        tamil_to_deva.insert('௭', '७');       // Tamil ௭ → Devanagari ७
        tamil_to_deva.insert('௮', '८');       // Tamil ௮ → Devanagari ८
        tamil_to_deva.insert('௯', '९');       // Tamil ௯ → Devanagari ९
        
        // Build reverse mapping for Devanagari → Tamil conversion
        let mut deva_to_tamil = HashMap::new();
        for (&tamil, &deva) in &tamil_to_deva {
            deva_to_tamil.insert(deva, tamil);
        }

        Self {
            tamil_to_deva_map: tamil_to_deva,
            deva_to_tamil_map: deva_to_tamil,
        }
    }
    
    /// Convert Tamil text to Devanagari format (for hub processing)
    pub fn tamil_to_devanagari(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        
        // Simple character-to-character mapping - let hub handle complex rules
        for ch in input.chars() {
            if ch.is_whitespace() || ch.is_ascii_punctuation() {
                result.push(ch);
            } else if let Some(&deva_char) = self.tamil_to_deva_map.get(&ch) {
                result.push(deva_char);
            } else {
                // Character not in mapping - preserve as-is for mixed content
                result.push(ch);
            }
        }
        
        Ok(result)
    }
    
    /// Convert Devanagari text to Tamil format (reverse conversion)
    pub fn devanagari_to_tamil(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        
        // Simple character-to-character mapping
        for ch in input.chars() {
            if ch.is_whitespace() || ch.is_ascii_punctuation() {
                result.push(ch);
            } else if let Some(&tamil_char) = self.deva_to_tamil_map.get(&ch) {
                result.push(tamil_char);
            } else {
                // Character not in mapping - preserve as-is for mixed content
                result.push(ch);
            }
        }
        
        Ok(result)
    }
}

impl ScriptConverter for TamilConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "tamil" && script != "ta" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Tamil converter only supports 'tamil' or 'ta' script".to_string(),
            });
        }
        
        let deva_text = self.tamil_to_devanagari(input)
            .map_err(|e| ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: format!("Tamil to Devanagari conversion failed: {}", e),
            })?;
            
        // Return Devanagari for hub processing, not ISO
        Ok(HubInput::Devanagari(deva_text))
    }
    
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        if script != "tamil" && script != "ta" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Tamil converter only supports 'tamil' or 'ta' script".to_string(),
            });
        }
        
        match hub_input {
            HubInput::Devanagari(deva_text) => self.devanagari_to_tamil(deva_text),
            HubInput::Iso(_) => Err(ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: "Tamil converter expects Devanagari input, got ISO".to_string(),
            }),
        }
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["tamil", "ta"]
    }
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        // Tamil is an Indic script - consonants DO have implicit 'a'
        // But now the hub will handle this complexity
        true
    }
}

impl Default for TamilConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tamil_basic_vowels() {
        let converter = TamilConverter::new();
        
        // Test basic vowels - now should map to Devanagari
        assert_eq!(converter.tamil_to_devanagari("அ").unwrap(), "अ");
        assert_eq!(converter.tamil_to_devanagari("ஆ").unwrap(), "आ");
        assert_eq!(converter.tamil_to_devanagari("இ").unwrap(), "इ");
        assert_eq!(converter.tamil_to_devanagari("ஈ").unwrap(), "ई");
        assert_eq!(converter.tamil_to_devanagari("உ").unwrap(), "उ");
        assert_eq!(converter.tamil_to_devanagari("ஊ").unwrap(), "ऊ");
    }
    
    #[test]
    fn test_tamil_consonants() {
        let converter = TamilConverter::new();
        
        // Test basic consonants - should map to Devanagari equivalents
        assert_eq!(converter.tamil_to_devanagari("க").unwrap(), "क");
        assert_eq!(converter.tamil_to_devanagari("ச").unwrap(), "च");
        assert_eq!(converter.tamil_to_devanagari("த").unwrap(), "त");
        assert_eq!(converter.tamil_to_devanagari("ப").unwrap(), "प");
        assert_eq!(converter.tamil_to_devanagari("ம").unwrap(), "म");
    }
    
    #[test]
    fn test_tamil_unique_consonants() {
        let converter = TamilConverter::new();
        
        // Test unique Tamil consonants
        assert_eq!(converter.tamil_to_devanagari("ழ").unwrap(), "ळ");   // retroflex l
        assert_eq!(converter.tamil_to_devanagari("ள").unwrap(), "ळ");   // retroflex l variant
        assert_eq!(converter.tamil_to_devanagari("ற").unwrap(), "र");   // alveolar trill → r
        assert_eq!(converter.tamil_to_devanagari("ன").unwrap(), "न");   // alveolar n → n
    }
    
    #[test]
    fn test_tamil_special_marks() {
        let converter = TamilConverter::new();
        
        // Test special marks
        assert_eq!(converter.tamil_to_devanagari("ஂ").unwrap(), "ं");   // anusvara (rare)
        assert_eq!(converter.tamil_to_devanagari("ஃ").unwrap(), "ः");   // visarga (rare)
        assert_eq!(converter.tamil_to_devanagari("்").unwrap(), "्");    // pulli/virama
    }
    
    #[test]
    fn test_script_converter_interface() {
        let converter = TamilConverter::new();
        
        // Test the ScriptConverter interface
        assert!(converter.supports_script("tamil"));
        assert!(converter.supports_script("ta"));
        assert!(!converter.supports_script("telugu"));
        
        // Test script_has_implicit_a
        assert!(converter.script_has_implicit_a("tamil"));
        assert!(converter.script_has_implicit_a("ta"));
        
        let result = converter.to_hub("tamil", "க").unwrap();
        if let HubInput::Devanagari(deva_text) = result {
            assert_eq!(deva_text, "क");
        } else {
            panic!("Expected Devanagari hub input");
        }
    }
}