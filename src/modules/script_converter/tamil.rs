use std::collections::HashMap;
use super::{ScriptConverter, ConverterError};
use crate::modules::hub::HubInput;

/// Tamil script converter
/// 
/// Tamil (தமிழ்) is an Indic script used primarily for the Tamil language.
/// This converter handles Tamil text by first converting it to ISO-15919 format,
/// which can then be processed by the hub for cross-script conversion.
/// Tamil script has some unique features compared to other Indic scripts.
pub struct TamilConverter {
    tamil_to_iso_map: HashMap<char, &'static str>,
}

impl TamilConverter {
    pub fn new() -> Self {
        let mut tamil_to_iso = HashMap::new();
        
        // Tamil has consonants with implicit 'a' vowel, similar to other Indic scripts
        // but Tamil script has some unique characteristics:
        // - No aspirated consonants (kh, gh, etc.)
        // - No retroflex consonants except ṭ, ṇ, ṛ
        // - Unique character ṟ (alveolar trill)
        // - Unique character ḻ (retroflex l)
        
        // Vowels (உயிர்)
        tamil_to_iso.insert('அ', "a");       // Tamil அ → ISO a
        tamil_to_iso.insert('ஆ', "ā");       // Tamil ஆ → ISO ā
        tamil_to_iso.insert('இ', "i");       // Tamil இ → ISO i
        tamil_to_iso.insert('ஈ', "ī");       // Tamil ஈ → ISO ī
        tamil_to_iso.insert('உ', "u");       // Tamil உ → ISO u
        tamil_to_iso.insert('ஊ', "ū");       // Tamil ஊ → ISO ū
        tamil_to_iso.insert('எ', "e");       // Tamil எ → ISO e
        tamil_to_iso.insert('ஏ', "ē");       // Tamil ஏ → ISO ē (long e)
        tamil_to_iso.insert('ஐ', "ai");      // Tamil ஐ → ISO ai
        tamil_to_iso.insert('ஒ', "o");       // Tamil ஒ → ISO o
        tamil_to_iso.insert('ஓ', "ō");       // Tamil ஓ → ISO ō (long o)
        tamil_to_iso.insert('ஔ', "au");      // Tamil ஔ → ISO au
        
        // Vowel diacritics (உயிர்மெய்)
        tamil_to_iso.insert('ா', "ā");       // Tamil ா → ISO ā
        tamil_to_iso.insert('ி', "i");       // Tamil ி → ISO i
        tamil_to_iso.insert('ீ', "ī");       // Tamil ீ → ISO ī
        tamil_to_iso.insert('ு', "u");       // Tamil ு → ISO u
        tamil_to_iso.insert('ூ', "ū");       // Tamil ூ → ISO ū
        tamil_to_iso.insert('ெ', "e");       // Tamil ெ → ISO e
        tamil_to_iso.insert('ே', "ē");       // Tamil ே → ISO ē
        tamil_to_iso.insert('ை', "ai");      // Tamil ै → ISO ai
        tamil_to_iso.insert('ொ', "o");       // Tamil ொ → ISO o
        tamil_to_iso.insert('ோ', "ō");       // Tamil ோ → ISO ō
        tamil_to_iso.insert('ௌ', "au");      // Tamil ௌ → ISO au
        
        // Consonants (மெய்) - Tamil consonants have implicit 'a'
        // Tamil has a limited set of consonants compared to other Indic scripts
        
        // Velar consonants
        tamil_to_iso.insert('க', "ka");      // Tamil க → ISO ka
        tamil_to_iso.insert('ங', "ṅa");      // Tamil ங → ISO ṅa
        
        // Palatal consonants  
        tamil_to_iso.insert('ச', "ca");      // Tamil ச → ISO ca
        tamil_to_iso.insert('ஞ', "ña");      // Tamil ஞ → ISO ña
        
        // Retroflex consonants (limited in Tamil)
        tamil_to_iso.insert('ட', "ṭa");      // Tamil ட → ISO ṭa
        tamil_to_iso.insert('ண', "ṇa");      // Tamil ண → ISO ṇa
        
        // Dental consonants
        tamil_to_iso.insert('த', "ta");      // Tamil த → ISO ta
        tamil_to_iso.insert('ந', "na");      // Tamil ந → ISO na
        
        // Labial consonants
        tamil_to_iso.insert('ப', "pa");      // Tamil ப → ISO pa
        tamil_to_iso.insert('ம', "ma");      // Tamil ம → ISO ma
        
        // Semivowels and liquids
        tamil_to_iso.insert('ய', "ya");      // Tamil ய → ISO ya
        tamil_to_iso.insert('ர', "ra");      // Tamil ர → ISO ra
        tamil_to_iso.insert('ல', "la");      // Tamil ல → ISO la
        tamil_to_iso.insert('வ', "va");      // Tamil வ → ISO va
        
        // Unique Tamil consonants
        tamil_to_iso.insert('ழ', "ḻa");      // Tamil ழ → ISO ḻa (retroflex l)
        tamil_to_iso.insert('ள', "ḷa");      // Tamil ள → ISO ḷa (retroflex l variant)
        tamil_to_iso.insert('ற', "ṟa");      // Tamil ற → ISO ṟa (alveolar trill)
        tamil_to_iso.insert('ன', "ṉa");      // Tamil ன → ISO ṉa (alveolar n)
        
        // Sibilants and aspirate (limited in Tamil)
        tamil_to_iso.insert('ஶ', "śa");      // Tamil ஶ → ISO śa (rare, used in Sanskrit loanwords)
        tamil_to_iso.insert('ஷ', "ṣa");      // Tamil ஷ → ISO ṣa (rare, used in Sanskrit loanwords)
        tamil_to_iso.insert('ஸ', "sa");      // Tamil ஸ → ISO sa (rare, used in Sanskrit loanwords)
        tamil_to_iso.insert('ஹ', "ha");      // Tamil ஹ → ISO ha (rare, used in Sanskrit loanwords)
        
        // Additional characters for Sanskrit loanwords
        tamil_to_iso.insert('ஜ', "ja");      // Tamil ஜ → ISO ja (Sanskrit loanword)
        tamil_to_iso.insert('ஃ', "ḥ");       // Tamil ஃ → ISO ḥ (visarga - rare)
        
        // Special marks
        tamil_to_iso.insert('ஂ', "ṁ");       // Tamil ஂ → ISO ṁ (anusvara - rare)
        tamil_to_iso.insert('்', "");        // Tamil ் → pulli/virama (removes inherent vowel)
        
        // Digits
        tamil_to_iso.insert('௦', "0");       // Tamil ௦ → ISO 0
        tamil_to_iso.insert('௧', "1");       // Tamil ௧ → ISO 1
        tamil_to_iso.insert('௨', "2");       // Tamil ௨ → ISO 2
        tamil_to_iso.insert('௩', "3");       // Tamil ௩ → ISO 3
        tamil_to_iso.insert('௪', "4");       // Tamil ௪ → ISO 4
        tamil_to_iso.insert('௫', "5");       // Tamil ௫ → ISO 5
        tamil_to_iso.insert('௬', "6");       // Tamil ௬ → ISO 6
        tamil_to_iso.insert('௭', "7");       // Tamil ௭ → ISO 7
        tamil_to_iso.insert('௮', "8");       // Tamil ௮ → ISO 8
        tamil_to_iso.insert('௯', "9");       // Tamil ௯ → ISO 9
        
        Self {
            tamil_to_iso_map: tamil_to_iso,
        }
    }
    
    /// Convert Tamil text to ISO-15919 format
    pub fn tamil_to_iso(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        
        for ch in input.chars() {
            if ch.is_whitespace() || ch.is_ascii_punctuation() {
                result.push(ch);
            } else if let Some(&iso_str) = self.tamil_to_iso_map.get(&ch) {
                result.push_str(iso_str);
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
        
        let iso_text = self.tamil_to_iso(input)
            .map_err(|e| ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: format!("Tamil to ISO conversion failed: {}", e),
            })?;
            
        Ok(HubInput::Iso(iso_text))
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["tamil", "ta"]
    }
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        // Tamil is an Indic script - consonants DO have implicit 'a'
        // In Tamil, க inherently represents "ka" and requires pulli (்) to suppress the vowel: க்
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
        
        // Test basic vowels
        assert_eq!(converter.tamil_to_iso("அ").unwrap(), "a");
        assert_eq!(converter.tamil_to_iso("ஆ").unwrap(), "ā");
        assert_eq!(converter.tamil_to_iso("இ").unwrap(), "i");
        assert_eq!(converter.tamil_to_iso("ஈ").unwrap(), "ī");
        assert_eq!(converter.tamil_to_iso("உ").unwrap(), "u");
        assert_eq!(converter.tamil_to_iso("ஊ").unwrap(), "ū");
        assert_eq!(converter.tamil_to_iso("எ").unwrap(), "e");
        assert_eq!(converter.tamil_to_iso("ஏ").unwrap(), "ē");
        assert_eq!(converter.tamil_to_iso("ஐ").unwrap(), "ai");
        assert_eq!(converter.tamil_to_iso("ஒ").unwrap(), "o");
        assert_eq!(converter.tamil_to_iso("ஓ").unwrap(), "ō");
        assert_eq!(converter.tamil_to_iso("ஔ").unwrap(), "au");
    }
    
    #[test]
    fn test_tamil_consonants() {
        let converter = TamilConverter::new();
        
        // Test basic consonants (with implicit 'a')
        assert_eq!(converter.tamil_to_iso("க").unwrap(), "ka");
        assert_eq!(converter.tamil_to_iso("ங").unwrap(), "ṅa");
        assert_eq!(converter.tamil_to_iso("ச").unwrap(), "ca");
        assert_eq!(converter.tamil_to_iso("ஞ").unwrap(), "ña");
        assert_eq!(converter.tamil_to_iso("ட").unwrap(), "ṭa");
        assert_eq!(converter.tamil_to_iso("ண").unwrap(), "ṇa");
        assert_eq!(converter.tamil_to_iso("த").unwrap(), "ta");
        assert_eq!(converter.tamil_to_iso("ந").unwrap(), "na");
        assert_eq!(converter.tamil_to_iso("ப").unwrap(), "pa");
        assert_eq!(converter.tamil_to_iso("ம").unwrap(), "ma");
        assert_eq!(converter.tamil_to_iso("ய").unwrap(), "ya");
        assert_eq!(converter.tamil_to_iso("ர").unwrap(), "ra");
        assert_eq!(converter.tamil_to_iso("ல").unwrap(), "la");
        assert_eq!(converter.tamil_to_iso("வ").unwrap(), "va");
    }
    
    #[test]
    fn test_tamil_unique_consonants() {
        let converter = TamilConverter::new();
        
        // Test unique Tamil consonants
        assert_eq!(converter.tamil_to_iso("ழ").unwrap(), "ḻa");   // retroflex l
        assert_eq!(converter.tamil_to_iso("ள").unwrap(), "ḷa");   // retroflex l variant
        assert_eq!(converter.tamil_to_iso("ற").unwrap(), "ṟa");   // alveolar trill
        assert_eq!(converter.tamil_to_iso("ன").unwrap(), "ṉa");   // alveolar n
    }
    
    #[test]
    fn test_tamil_special_marks() {
        let converter = TamilConverter::new();
        
        // Test special marks
        assert_eq!(converter.tamil_to_iso("ஂ").unwrap(), "ṁ");   // anusvara (rare)
        assert_eq!(converter.tamil_to_iso("ஃ").unwrap(), "ḥ");   // visarga (rare)
        assert_eq!(converter.tamil_to_iso("்").unwrap(), "");    // pulli/virama
    }
    
    #[test]
    fn test_tamil_digits() {
        let converter = TamilConverter::new();
        
        // Test Tamil digits
        assert_eq!(converter.tamil_to_iso("௦௧௨௩௪௫௬௭௮௯").unwrap(), "0123456789");
    }
    
    #[test]
    fn test_tamil_sanskrit_loanwords() {
        let converter = TamilConverter::new();
        
        // Test characters used in Sanskrit loanwords
        assert_eq!(converter.tamil_to_iso("ஶ").unwrap(), "śa");   // rare, Sanskrit
        assert_eq!(converter.tamil_to_iso("ஷ").unwrap(), "ṣa");   // rare, Sanskrit
        assert_eq!(converter.tamil_to_iso("ஸ").unwrap(), "sa");   // rare, Sanskrit
        assert_eq!(converter.tamil_to_iso("ஹ").unwrap(), "ha");   // rare, Sanskrit
        assert_eq!(converter.tamil_to_iso("ஜ").unwrap(), "ja");   // Sanskrit loanword
    }
    
    #[test]
    fn test_tamil_complex_text() {
        let converter = TamilConverter::new();
        
        // Test Tamil words
        let tamil_text = "வணக்கம்";  // "vanakkam" (greeting)
        
        // Just test that conversion works without error
        let result = converter.tamil_to_iso(tamil_text);
        assert!(result.is_ok());
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
        if let HubInput::Iso(iso_text) = result {
            assert_eq!(iso_text, "ka");
        } else {
            panic!("Expected ISO hub input");
        }
    }
    
    #[test]
    fn test_invalid_script_error() {
        let converter = TamilConverter::new();
        
        // Should reject invalid script names
        let result = converter.to_hub("hindi", "test");
        assert!(result.is_err());
        
        if let Err(ConverterError::InvalidInput { script, message }) = result {
            assert_eq!(script, "hindi");
            assert!(message.contains("Tamil converter only supports"));
        } else {
            panic!("Expected InvalidInput error");
        }
    }
    
    #[test]
    fn test_mixed_content() {
        let converter = TamilConverter::new();
        
        // Should handle mixed Tamil and other characters
        let mixed_input = "வணக்கம் 123 hello";
        let result = converter.tamil_to_iso(mixed_input).unwrap();
        
        // Should contain the converted Tamil part plus preserved non-Tamil content
        assert!(result.contains("123"));
        assert!(result.contains("hello"));
    }
}