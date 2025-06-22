use std::collections::HashMap;
use super::{ScriptConverter, ConverterError};
use crate::modules::hub::HubInput;

/// Velthuis to ISO-15919 converter
/// 
/// Velthuis is an ASCII-based transliteration scheme for Sanskrit developed by Frans Velthuis.
/// It's popular in academic contexts and uses specific ASCII character combinations
/// to represent Sanskrit sounds without requiring special fonts or diacritical marks.
pub struct VelthuisConverter {
    velthuis_to_iso_map: HashMap<&'static str, &'static str>,
    iso_to_velthuis_map: HashMap<&'static str, &'static str>,
}

impl VelthuisConverter {
    pub fn new() -> Self {
        let mut velthuis_to_iso = HashMap::new();
        
        // Velthuis uses specific ASCII conventions:
        // - Long vowels use doubled letters (aa, ii, uu) or capital letters
        // - Retroflex consonants use dots (.t, .d, .n, etc.)
        // - Aspirated consonants use 'h' after the consonant
        // - Special sounds use specific combinations ("s, ~n, etc.)
        
        // Vowels
        velthuis_to_iso.insert("a", "a");
        velthuis_to_iso.insert("aa", "ā");      // Velthuis aa → ISO ā
        velthuis_to_iso.insert("A", "ā");       // Alternative
        velthuis_to_iso.insert("i", "i");
        velthuis_to_iso.insert("ii", "ī");      // Velthuis ii → ISO ī
        velthuis_to_iso.insert("I", "ī");       // Alternative
        velthuis_to_iso.insert("u", "u");
        velthuis_to_iso.insert("uu", "ū");      // Velthuis uu → ISO ū
        velthuis_to_iso.insert("U", "ū");       // Alternative
        velthuis_to_iso.insert(".r", "r̥");      // Velthuis .r → ISO r̥
        velthuis_to_iso.insert(".R", "r̥̄");     // Velthuis .R → ISO r̥̄
        velthuis_to_iso.insert(".l", "l̥");      // Velthuis .l → ISO l̥
        velthuis_to_iso.insert(".L", "l̥̄");     // Velthuis .L → ISO l̥̄
        velthuis_to_iso.insert("e", "e");
        velthuis_to_iso.insert("ai", "ai");
        velthuis_to_iso.insert("o", "o");
        velthuis_to_iso.insert("au", "au");
        
        // Consonants - Velthuis uses ASCII combinations (no inherent 'a')
        velthuis_to_iso.insert("k", "k");
        velthuis_to_iso.insert("kh", "kh");
        velthuis_to_iso.insert("g", "g");
        velthuis_to_iso.insert("gh", "gh");
        velthuis_to_iso.insert("\"n", "ṅ");     // Velthuis "n → ISO ṅ
        velthuis_to_iso.insert("c", "c");
        velthuis_to_iso.insert("ch", "ch");
        velthuis_to_iso.insert("j", "j");
        velthuis_to_iso.insert("jh", "jh");
        velthuis_to_iso.insert("~n", "ñ");      // Velthuis ~n → ISO ñ
        velthuis_to_iso.insert(".t", "ṭ");      // Velthuis .t → ISO ṭ
        velthuis_to_iso.insert(".th", "ṭh");    // Velthuis .th → ISO ṭh
        velthuis_to_iso.insert(".d", "ḍ");      // Velthuis .d → ISO ḍ
        velthuis_to_iso.insert(".dh", "ḍh");    // Velthuis .dh → ISO ḍh
        velthuis_to_iso.insert(".n", "ṇ");      // Velthuis .n → ISO ṇ
        velthuis_to_iso.insert("t", "t");
        velthuis_to_iso.insert("th", "th");
        velthuis_to_iso.insert("d", "d");
        velthuis_to_iso.insert("dh", "dh");
        velthuis_to_iso.insert("n", "n");
        velthuis_to_iso.insert("p", "p");
        velthuis_to_iso.insert("ph", "ph");
        velthuis_to_iso.insert("b", "b");
        velthuis_to_iso.insert("bh", "bh");
        velthuis_to_iso.insert("m", "m");
        velthuis_to_iso.insert("y", "y");
        velthuis_to_iso.insert("r", "r");
        velthuis_to_iso.insert("l", "l");
        velthuis_to_iso.insert("v", "v");
        velthuis_to_iso.insert("\"s", "ś");     // Velthuis "s → ISO ś
        velthuis_to_iso.insert(".s", "ṣ");      // Velthuis .s → ISO ṣ
        velthuis_to_iso.insert("s", "s");
        velthuis_to_iso.insert("h", "h");
        
        // Additional consonants
        velthuis_to_iso.insert(".l", "ḷ");      // Velthuis .l → ISO ḷ (retroflex L - conflicts with vocalic l)
        
        // Handle the conflict: prioritize vocalic l over retroflex L
        velthuis_to_iso.insert(".l", "l̥");      // Prioritize vocalic l
        velthuis_to_iso.insert(".ll", "ḷ");     // Use .ll for retroflex L
        
        // Nukta consonants (Persian/Arabic sounds)
        velthuis_to_iso.insert("q", "q");
        velthuis_to_iso.insert("f", "f");
        velthuis_to_iso.insert("z", "z");
        
        // Special combinations
        velthuis_to_iso.insert("k.s", "kṣ");    // Velthuis k.s → ISO kṣ
        velthuis_to_iso.insert("j~n", "jñ");    // Velthuis j~n → ISO jñ
        
        // Special marks
        velthuis_to_iso.insert(".m", "ṁ");      // Velthuis .m → ISO ṁ (anusvara)
        velthuis_to_iso.insert(".h", "ḥ");      // Velthuis .h → ISO ḥ (visarga)
        velthuis_to_iso.insert("~", "m̐");       // Velthuis ~ → ISO m̐ (candrabindu)
        velthuis_to_iso.insert("'", "'");       // Avagraha (identical)
        
        // Digits (identical)
        for i in 0..=9 {
            let digit_str = i.to_string();
            let digit_key = digit_str.clone();
            velthuis_to_iso.insert(digit_key.leak(), digit_str.leak());
        }
        
        // Punctuation
        velthuis_to_iso.insert("|", "।");       // Velthuis | → danda
        velthuis_to_iso.insert("||", "॥");      // Velthuis || → double danda
        
        // Build reverse mapping for ISO → Velthuis conversion
        let mut iso_to_velthuis = HashMap::new();
        for (&velthuis, &iso) in &velthuis_to_iso {
            iso_to_velthuis.insert(iso, velthuis);
        }

        Self {
            velthuis_to_iso_map: velthuis_to_iso,
            iso_to_velthuis_map: iso_to_velthuis,
        }
    }
    
    /// Convert Velthuis text to ISO-15919 format
    pub fn velthuis_to_iso(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            let ch = chars[i];
            
            if ch.is_whitespace() {
                result.push(ch);
                i += 1;
                continue;
            }
            
            // Handle punctuation that's not in our mapping
            if ch.is_ascii_punctuation() && ch != '|' && ch != '~' && ch != '\'' && ch != '.' && ch != '"' {
                result.push(ch);
                i += 1;
                continue;
            }
            
            let mut matched = false;
            
            // Try to match sequences of decreasing length (5, 4, 3, 2, 1)
            // Velthuis can have sequences like ".dh", "j~n", "k.s"
            for len in (1..=5).rev() {
                if i + len > chars.len() {
                    continue;
                }
                
                let seq: String = chars[i..i+len].iter().collect();
                if let Some(&iso_str) = self.velthuis_to_iso_map.get(seq.as_str()) {
                    result.push_str(iso_str);
                    i += len;
                    matched = true;
                    break;
                }
            }
            
            if !matched {
                // Character not found in mapping - preserve as-is
                result.push(ch);
                i += 1;
            }
        }
        
        Ok(result)
    }
    
    /// Convert ISO-15919 text to Velthuis format (reverse conversion)
    pub fn iso_to_velthuis(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            let ch = chars[i];
            
            if ch.is_whitespace() {
                result.push(ch);
                i += 1;
                continue;
            }
            
            // Handle ASCII punctuation that's not in our mapping
            if ch.is_ascii_punctuation() && ch != '।' && ch != '॥' && ch != '\'' {
                result.push(ch);
                i += 1;
                continue;
            }
            
            let mut matched = false;
            
            // Try to match sequences of decreasing length (5, 4, 3, 2, 1)
            // ISO can have combining characters like r̥̄, l̥̄
            for len in (1..=5).rev() {
                if i + len > chars.len() {
                    continue;
                }
                
                let seq: String = chars[i..i+len].iter().collect();
                if let Some(&velthuis_str) = self.iso_to_velthuis_map.get(seq.as_str()) {
                    result.push_str(velthuis_str);
                    i += len;
                    matched = true;
                    break;
                }
            }
            
            if !matched {
                // Character not found in mapping - preserve as-is
                result.push(ch);
                i += 1;
            }
        }
        
        Ok(result)
    }
}

impl ScriptConverter for VelthuisConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "velthuis" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Velthuis converter only supports 'velthuis' script".to_string(),
            });
        }
        
        let iso_text = self.velthuis_to_iso(input)
            .map_err(|e| ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: format!("Velthuis to ISO conversion failed: {}", e),
            })?;
            
        Ok(HubInput::Iso(iso_text))
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["velthuis"]
    }
    
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        if script != "velthuis" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Velthuis converter only supports 'velthuis' script".to_string(),
            });
        }
        
        match hub_input {
            HubInput::Iso(iso_text) => {
                self.iso_to_velthuis(iso_text)
                    .map_err(|e| ConverterError::ConversionFailed {
                        script: script.to_string(),
                        reason: format!("ISO to Velthuis conversion failed: {}", e),
                    })
            }
            HubInput::Devanagari(_) => {
                Err(ConverterError::ConversionFailed {
                    script: script.to_string(),
                    reason: "Velthuis converter expects ISO hub input, not Devanagari".to_string(),
                })
            }
        }
    }
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        // Velthuis is a romanization scheme - consonants do NOT have implicit 'a'
        false
    }
}

impl Default for VelthuisConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_velthuis_basic_vowels() {
        let converter = VelthuisConverter::new();
        
        // Test basic vowels
        assert_eq!(converter.velthuis_to_iso("a").unwrap(), "a");
        assert_eq!(converter.velthuis_to_iso("aa").unwrap(), "ā");
        assert_eq!(converter.velthuis_to_iso("A").unwrap(), "ā");
        assert_eq!(converter.velthuis_to_iso("i").unwrap(), "i");
        assert_eq!(converter.velthuis_to_iso("ii").unwrap(), "ī");
        assert_eq!(converter.velthuis_to_iso("I").unwrap(), "ī");
        assert_eq!(converter.velthuis_to_iso("u").unwrap(), "u");
        assert_eq!(converter.velthuis_to_iso("uu").unwrap(), "ū");
        assert_eq!(converter.velthuis_to_iso("U").unwrap(), "ū");
    }
    
    #[test]
    fn test_velthuis_vocalic_vowels() {
        let converter = VelthuisConverter::new();
        
        // Test Velthuis → ISO vocalic vowels
        assert_eq!(converter.velthuis_to_iso(".r").unwrap(), "r̥");    // Velthuis .r → ISO r̥
        assert_eq!(converter.velthuis_to_iso(".R").unwrap(), "r̥̄");   // Velthuis .R → ISO r̥̄
        assert_eq!(converter.velthuis_to_iso(".l").unwrap(), "l̥");    // Velthuis .l → ISO l̥
        assert_eq!(converter.velthuis_to_iso(".L").unwrap(), "l̥̄");   // Velthuis .L → ISO l̥̄
    }
    
    #[test]
    fn test_velthuis_consonants() {
        let converter = VelthuisConverter::new();
        
        // Test basic consonants (no inherent 'a')
        assert_eq!(converter.velthuis_to_iso("k").unwrap(), "k");
        assert_eq!(converter.velthuis_to_iso("kh").unwrap(), "kh");
        assert_eq!(converter.velthuis_to_iso("g").unwrap(), "g");
        assert_eq!(converter.velthuis_to_iso("gh").unwrap(), "gh");
        
        // Test retroflex consonants with dots
        assert_eq!(converter.velthuis_to_iso(".t").unwrap(), "ṭ");
        assert_eq!(converter.velthuis_to_iso(".th").unwrap(), "ṭh");
        assert_eq!(converter.velthuis_to_iso(".d").unwrap(), "ḍ");
        assert_eq!(converter.velthuis_to_iso(".dh").unwrap(), "ḍh");
        assert_eq!(converter.velthuis_to_iso(".n").unwrap(), "ṇ");
        
        // Test sibilants with quotes and dots
        assert_eq!(converter.velthuis_to_iso("\"s").unwrap(), "ś");   // Velthuis "s → ISO ś
        assert_eq!(converter.velthuis_to_iso(".s").unwrap(), "ṣ");    // Velthuis .s → ISO ṣ
        assert_eq!(converter.velthuis_to_iso("s").unwrap(), "s");
    }
    
    #[test]
    fn test_velthuis_special_marks() {
        let converter = VelthuisConverter::new();
        
        // Test special marks
        assert_eq!(converter.velthuis_to_iso(".m").unwrap(), "ṁ");
        assert_eq!(converter.velthuis_to_iso(".h").unwrap(), "ḥ");
        assert_eq!(converter.velthuis_to_iso("~").unwrap(), "m̐");
    }
    
    #[test]
    fn test_velthuis_special_combinations() {
        let converter = VelthuisConverter::new();
        
        // Test special combinations
        assert_eq!(converter.velthuis_to_iso("k.s").unwrap(), "kṣ");   // Velthuis k.s → ISO kṣ
        assert_eq!(converter.velthuis_to_iso("j~n").unwrap(), "jñ");   // Velthuis j~n → ISO jñ
    }
    
    #[test]
    fn test_velthuis_complex_text() {
        let converter = VelthuisConverter::new();
        
        // Test Velthuis specific features
        let velthuis_text = "k.s \"s .s .m .h";
        let expected_iso = "kṣ ś ṣ ṁ ḥ";
        assert_eq!(converter.velthuis_to_iso(velthuis_text).unwrap(), expected_iso);
    }
    
    #[test]
    fn test_velthuis_bidirectional_conversion() {
        let converter = VelthuisConverter::new();
        
        // Test Velthuis → ISO → Velthuis roundtrip
        let original = "dharma .r k.s j~n \"s .s .m .h";
        let iso_result = converter.velthuis_to_iso(original).unwrap();
        let velthuis_result = converter.iso_to_velthuis(&iso_result).unwrap();
        assert_eq!(original, velthuis_result);
        
        // Test specific bidirectional conversions
        assert_eq!(converter.iso_to_velthuis("ṭ").unwrap(), ".t");
        assert_eq!(converter.iso_to_velthuis("ṭh").unwrap(), ".th");
        assert_eq!(converter.iso_to_velthuis("ḍ").unwrap(), ".d");
        assert_eq!(converter.iso_to_velthuis("ḍh").unwrap(), ".dh");
        assert_eq!(converter.iso_to_velthuis("ṇ").unwrap(), ".n");
        assert_eq!(converter.iso_to_velthuis("ś").unwrap(), "\"s");
        assert_eq!(converter.iso_to_velthuis("ṣ").unwrap(), ".s");
        assert_eq!(converter.iso_to_velthuis("r̥").unwrap(), ".r");
        assert_eq!(converter.iso_to_velthuis("r̥̄").unwrap(), ".R");
        assert_eq!(converter.iso_to_velthuis("l̥").unwrap(), ".l");
        assert_eq!(converter.iso_to_velthuis("l̥̄").unwrap(), ".L");
        assert_eq!(converter.iso_to_velthuis("ṁ").unwrap(), ".m");
        assert_eq!(converter.iso_to_velthuis("ḥ").unwrap(), ".h");
        assert_eq!(converter.iso_to_velthuis("kṣ").unwrap(), "k.s");
        assert_eq!(converter.iso_to_velthuis("jñ").unwrap(), "j~n");
    }
    
    #[test]
    fn test_script_converter_interface() {
        let converter = VelthuisConverter::new();
        
        // Test the ScriptConverter interface
        assert!(converter.supports_script("velthuis"));
        assert!(!converter.supports_script("itrans"));
        
        // Test script_has_implicit_a
        assert!(!converter.script_has_implicit_a("velthuis"));
        
        // Test to_hub
        let result = converter.to_hub("velthuis", "k").unwrap();
        if let HubInput::Iso(iso_text) = result {
            assert_eq!(iso_text, "k");
        } else {
            panic!("Expected ISO hub input");
        }
        
        // Test from_hub
        let hub_input = HubInput::Iso("ṭ".to_string());
        let result = converter.from_hub("velthuis", &hub_input).unwrap();
        assert_eq!(result, ".t");
    }
}