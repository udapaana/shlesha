use std::collections::HashMap;
use super::{ScriptConverter, ConverterError};
use crate::modules::hub::HubInput;

/// IAST (International Alphabet of Sanskrit Transliteration) to ISO-15919 converter
/// 
/// IAST is widely used in academic Sanskrit texts. It uses diacritical marks
/// and has some differences from ISO-15919 that need to be normalized.
pub struct IASTConverter {
    iast_to_iso_map: HashMap<&'static str, &'static str>,
}

impl IASTConverter {
    pub fn new() -> Self {
        let mut iast_to_iso = HashMap::new();
        
        // IAST and ISO-15919 are very similar, but have key differences:
        
        // Vowels - mostly identical
        iast_to_iso.insert("a", "a");
        iast_to_iso.insert("ā", "ā");
        iast_to_iso.insert("i", "i");
        iast_to_iso.insert("ī", "ī");
        iast_to_iso.insert("u", "u");
        iast_to_iso.insert("ū", "ū");
        iast_to_iso.insert("ṛ", "r̥");     // IAST ṛ → ISO r̥
        iast_to_iso.insert("ṝ", "r̥̄");    // IAST ṝ → ISO r̥̄
        iast_to_iso.insert("ḷ", "l̥");     // IAST ḷ → ISO l̥
        iast_to_iso.insert("ḹ", "l̥̄");    // IAST ḹ → ISO l̥̄
        iast_to_iso.insert("e", "e");
        iast_to_iso.insert("ai", "ai");
        iast_to_iso.insert("o", "o");
        iast_to_iso.insert("au", "au");
        
        // Consonants (bare consonants without vowels)
        iast_to_iso.insert("k", "k");
        iast_to_iso.insert("kh", "kh");
        iast_to_iso.insert("g", "g");
        iast_to_iso.insert("gh", "gh");
        iast_to_iso.insert("ṅ", "ṅ");
        iast_to_iso.insert("c", "c");
        iast_to_iso.insert("ch", "ch");
        iast_to_iso.insert("j", "j");
        iast_to_iso.insert("jh", "jh");
        iast_to_iso.insert("ñ", "ñ");
        iast_to_iso.insert("ṭ", "ṭ");
        iast_to_iso.insert("ṭh", "ṭh");
        iast_to_iso.insert("ḍ", "ḍ");
        iast_to_iso.insert("ḍh", "ḍh");
        iast_to_iso.insert("ṇ", "ṇ");
        iast_to_iso.insert("t", "t");
        iast_to_iso.insert("th", "th");
        iast_to_iso.insert("d", "d");
        iast_to_iso.insert("dh", "dh");
        iast_to_iso.insert("n", "n");
        iast_to_iso.insert("p", "p");
        iast_to_iso.insert("ph", "ph");
        iast_to_iso.insert("b", "b");
        iast_to_iso.insert("bh", "bh");
        iast_to_iso.insert("m", "m");
        iast_to_iso.insert("y", "y");
        iast_to_iso.insert("r", "r");
        iast_to_iso.insert("l", "l");
        iast_to_iso.insert("v", "v");
        iast_to_iso.insert("ś", "ś");
        iast_to_iso.insert("ṣ", "ṣ");
        iast_to_iso.insert("s", "s");
        iast_to_iso.insert("h", "h");
        
        // Consonant+vowel combinations (for compatibility)
        iast_to_iso.insert("ka", "ka");
        iast_to_iso.insert("kha", "kha");
        iast_to_iso.insert("ga", "ga");
        iast_to_iso.insert("gha", "gha");
        iast_to_iso.insert("ṅa", "ṅa");
        iast_to_iso.insert("ca", "ca");
        iast_to_iso.insert("cha", "cha");
        iast_to_iso.insert("ja", "ja");
        iast_to_iso.insert("jha", "jha");
        iast_to_iso.insert("ña", "ña");
        iast_to_iso.insert("ṭa", "ṭa");
        iast_to_iso.insert("ṭha", "ṭha");
        iast_to_iso.insert("ḍa", "ḍa");
        iast_to_iso.insert("ḍha", "ḍha");
        iast_to_iso.insert("ṇa", "ṇa");
        iast_to_iso.insert("ta", "ta");
        iast_to_iso.insert("tha", "tha");
        iast_to_iso.insert("da", "da");
        iast_to_iso.insert("dha", "dha");
        iast_to_iso.insert("na", "na");
        iast_to_iso.insert("pa", "pa");
        iast_to_iso.insert("pha", "pha");
        iast_to_iso.insert("ba", "ba");
        iast_to_iso.insert("bha", "bha");
        iast_to_iso.insert("ma", "ma");
        iast_to_iso.insert("ya", "ya");
        iast_to_iso.insert("ra", "ra");
        iast_to_iso.insert("la", "la");
        iast_to_iso.insert("va", "va");
        iast_to_iso.insert("śa", "śa");
        iast_to_iso.insert("ṣa", "ṣa");
        iast_to_iso.insert("sa", "sa");
        iast_to_iso.insert("ha", "ha");
        
        // Additional consonants
        iast_to_iso.insert("ḷa", "ḷa");   // Retroflex L
        
        // Nukta consonants (Persian/Arabic sounds)
        iast_to_iso.insert("qa", "qa");
        iast_to_iso.insert("ḵha", "ḵẖa");  // IAST uses ḵ for aspirated kha
        iast_to_iso.insert("ġa", "ġa");
        iast_to_iso.insert("za", "za");
        iast_to_iso.insert("ṛa", "ṛa");   // This is the flapped R (ड़), different from vocalic ṛ
        iast_to_iso.insert("ṛha", "ṛha");
        iast_to_iso.insert("fa", "fa");
        iast_to_iso.insert("ẏa", "ẏa");
        
        // Special marks
        iast_to_iso.insert("ṃ", "ṁ");     // IAST ṃ → ISO ṁ (anusvara)
        iast_to_iso.insert("ḥ", "ḥ");     // Visarga (identical)
        iast_to_iso.insert("m̐", "m̐");     // Candrabindu (identical)
        iast_to_iso.insert("'", "'");     // Avagraha (identical)
        
        // Digits (identical)
        for i in 0..=9 {
            let digit_str = i.to_string();
            let digit_key = digit_str.clone();
            iast_to_iso.insert(digit_key.leak(), digit_str.leak());
        }
        
        // Punctuation
        iast_to_iso.insert("।", "।");     // Danda
        iast_to_iso.insert("॥", "॥");     // Double danda
        
        Self {
            iast_to_iso_map: iast_to_iso,
        }
    }
    
    /// Convert IAST text to ISO-15919 format
    pub fn iast_to_iso(&self, input: &str) -> Result<String, ConverterError> {
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
            
            // Handle ASCII punctuation (preserve as-is)
            if ch.is_ascii_punctuation() && ch != '\'' {
                result.push(ch);
                i += 1;
                continue;
            }
            
            let mut matched = false;
            
            // Try to match sequences of decreasing length (4, 3, 2, 1)
            for len in (1..=4).rev() {
                if i + len > chars.len() {
                    continue;
                }
                
                let seq: String = chars[i..i+len].iter().collect();
                if let Some(&iso_str) = self.iast_to_iso_map.get(seq.as_str()) {
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
}

impl ScriptConverter for IASTConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "iast" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "IAST converter only supports 'iast' script".to_string(),
            });
        }
        
        let iso_text = self.iast_to_iso(input)
            .map_err(|e| ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: format!("IAST to ISO conversion failed: {}", e),
            })?;
            
        Ok(HubInput::Iso(iso_text))
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["iast"]
    }
    
    fn script_has_implicit_a(&self, script: &str) -> bool {
        // IAST is a romanization scheme - consonants do NOT have implicit 'a'
        false
    }
}

impl Default for IASTConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_iast_basic_vowels() {
        let converter = IASTConverter::new();
        
        // Test basic vowels
        assert_eq!(converter.iast_to_iso("a").unwrap(), "a");
        assert_eq!(converter.iast_to_iso("ā").unwrap(), "ā");
        assert_eq!(converter.iast_to_iso("i").unwrap(), "i");
        assert_eq!(converter.iast_to_iso("ī").unwrap(), "ī");
        assert_eq!(converter.iast_to_iso("u").unwrap(), "u");
        assert_eq!(converter.iast_to_iso("ū").unwrap(), "ū");
    }
    
    #[test]
    fn test_iast_vocalic_vowels() {
        let converter = IASTConverter::new();
        
        // Test key IAST → ISO differences for vocalic vowels
        assert_eq!(converter.iast_to_iso("ṛ").unwrap(), "r̥");   // IAST ṛ → ISO r̥
        assert_eq!(converter.iast_to_iso("ṝ").unwrap(), "r̥̄");  // IAST ṝ → ISO r̥̄
        assert_eq!(converter.iast_to_iso("ḷ").unwrap(), "l̥");   // IAST ḷ → ISO l̥
        assert_eq!(converter.iast_to_iso("ḹ").unwrap(), "l̥̄");  // IAST ḹ → ISO l̥̄
    }
    
    #[test]
    fn test_iast_consonants() {
        let converter = IASTConverter::new();
        
        // Test basic consonants (should be identical)
        assert_eq!(converter.iast_to_iso("ka").unwrap(), "ka");
        assert_eq!(converter.iast_to_iso("ma").unwrap(), "ma");
        assert_eq!(converter.iast_to_iso("śa").unwrap(), "śa");
        assert_eq!(converter.iast_to_iso("ṣa").unwrap(), "ṣa");
    }
    
    #[test]
    fn test_iast_special_marks() {
        let converter = IASTConverter::new();
        
        // Test special mark differences
        assert_eq!(converter.iast_to_iso("ṃ").unwrap(), "ṁ");   // IAST ṃ → ISO ṁ
        assert_eq!(converter.iast_to_iso("ḥ").unwrap(), "ḥ");   // Visarga (identical)
    }
    
    #[test]
    fn test_iast_complex_text() {
        let converter = IASTConverter::new();
        
        // Test complex text with IAST-specific characters
        let iast_text = "dharmaṃ śāśvataṃ";
        let expected_iso = "dharmaṁ śāśvataṁ";
        assert_eq!(converter.iast_to_iso(iast_text).unwrap(), expected_iso);
    }
    
    #[test]
    fn test_script_converter_interface() {
        let converter = IASTConverter::new();
        
        // Test the ScriptConverter interface
        assert!(converter.supports_script("iast"));
        assert!(!converter.supports_script("itrans"));
        
        let result = converter.to_hub("iast", "dharma").unwrap();
        if let HubInput::Iso(iso_text) = result {
            assert_eq!(iso_text, "dharma");
        } else {
            panic!("Expected ISO hub input");
        }
    }
}