use std::collections::HashMap;
use super::{ScriptConverter, ConverterError};
use super::processors::RomanScriptProcessor;
use crate::modules::hub::HubInput;

/// ITRANS (Indian Language Transliteration) to ISO-15919 converter
/// 
/// ITRANS is a popular ASCII-based transliteration scheme that uses
/// combinations of ASCII characters to represent Sanskrit/Devanagari sounds.
/// It's widely used in digital contexts where diacritical marks are not supported.
pub struct ITRANSConverter {
    itrans_to_iso_map: HashMap<&'static str, &'static str>,
    iso_to_itrans_map: HashMap<&'static str, &'static str>,
}

impl ITRANSConverter {
    pub fn new() -> Self {
        let mut itrans_to_iso = HashMap::new();
        
        // ITRANS uses ASCII characters and combinations
        // Key differences from ISO-15919:
        // - Uses 'R' and 'RR' for vocalic r
        // - Uses 'L' and 'LL' for vocalic l  
        // - Uses 'x' for kṣa combination
        // - Uses '^' for various purposes
        // - Uses 'M' for anusvara, 'H' for visarga
        // - Uses combinations like 'ch', 'kh', etc.
        
        // Vowels
        itrans_to_iso.insert("a", "a");
        itrans_to_iso.insert("A", "ā");       // ITRANS A → ISO ā
        itrans_to_iso.insert("aa", "ā");      // Alternative for ā
        itrans_to_iso.insert("i", "i");
        itrans_to_iso.insert("I", "ī");       // ITRANS I → ISO ī
        itrans_to_iso.insert("ii", "ī");      // Alternative for ī
        itrans_to_iso.insert("u", "u");
        itrans_to_iso.insert("U", "ū");       // ITRANS U → ISO ū
        itrans_to_iso.insert("uu", "ū");      // Alternative for ū
        itrans_to_iso.insert("R", "r̥");       // ITRANS R → ISO r̥
        itrans_to_iso.insert("RR", "r̥̄");     // ITRANS RR → ISO r̥̄
        itrans_to_iso.insert("lR", "l̥");      // ITRANS lR → ISO l̥
        itrans_to_iso.insert("lRR", "l̥̄");    // ITRANS lRR → ISO l̥̄
        itrans_to_iso.insert("L", "l̥");       // Alternative L → ISO l̥
        itrans_to_iso.insert("LL", "l̥̄");     // Alternative LL → ISO l̥̄
        itrans_to_iso.insert("e", "e");
        itrans_to_iso.insert("ai", "ai");
        itrans_to_iso.insert("o", "o");
        itrans_to_iso.insert("au", "au");
        
        // Consonants - ITRANS uses ASCII combinations (no inherent 'a')
        itrans_to_iso.insert("k", "k");
        itrans_to_iso.insert("kh", "kh");
        itrans_to_iso.insert("g", "g");
        itrans_to_iso.insert("gh", "gh");
        itrans_to_iso.insert("~N", "ṅ");      // ITRANS ~N → ISO ṅ
        itrans_to_iso.insert("N^", "ṅ");      // Alternative
        itrans_to_iso.insert("c", "c");
        itrans_to_iso.insert("ch", "ch");
        itrans_to_iso.insert("j", "j");
        itrans_to_iso.insert("jh", "jh");
        itrans_to_iso.insert("~n", "ñ");      // ITRANS ~n → ISO ñ
        itrans_to_iso.insert("JN", "ñ");      // Alternative
        itrans_to_iso.insert("T", "ṭ");       // ITRANS T → ISO ṭ
        itrans_to_iso.insert("Th", "ṭh");     // ITRANS Th → ISO ṭh
        itrans_to_iso.insert("D", "ḍ");       // ITRANS D → ISO ḍ
        itrans_to_iso.insert("Dh", "ḍh");     // ITRANS Dh → ISO ḍh
        itrans_to_iso.insert("N", "ṇ");       // ITRANS N → ISO ṇ
        itrans_to_iso.insert("t", "t");
        itrans_to_iso.insert("th", "th");
        itrans_to_iso.insert("d", "d");
        itrans_to_iso.insert("dh", "dh");
        itrans_to_iso.insert("n", "n");
        itrans_to_iso.insert("p", "p");
        itrans_to_iso.insert("ph", "ph");
        itrans_to_iso.insert("b", "b");
        itrans_to_iso.insert("bh", "bh");
        itrans_to_iso.insert("m", "m");
        itrans_to_iso.insert("y", "y");
        itrans_to_iso.insert("r", "r");
        itrans_to_iso.insert("l", "l");
        itrans_to_iso.insert("v", "v");
        itrans_to_iso.insert("w", "v");       // Alternative w → v
        itrans_to_iso.insert("sh", "ś");      // ITRANS sh → ISO ś
        itrans_to_iso.insert("Sh", "ṣ");      // ITRANS Sh → ISO ṣ
        itrans_to_iso.insert("s", "s");
        itrans_to_iso.insert("h", "h");
        
        // Additional consonants
        itrans_to_iso.insert("L", "ḷ");       // ITRANS L → ISO ḷ (retroflex L)
        
        // Nukta consonants (Persian/Arabic sounds)
        itrans_to_iso.insert("q", "q");
        itrans_to_iso.insert("K", "ḵẖ");      // ITRANS K → ISO ḵẖ (aspirated kha with nukta)
        itrans_to_iso.insert("G", "ġ");       // ITRANS G → ISO ġ
        itrans_to_iso.insert("z", "z");
        itrans_to_iso.insert(".D", "ṛ");      // ITRANS .D → ISO ṛ
        itrans_to_iso.insert(".Dh", "ṛh");    // ITRANS .Dh → ISO ṛh
        itrans_to_iso.insert("f", "f");
        itrans_to_iso.insert("Y", "ẏ");       // ITRANS Y → ISO ẏ
        
        // Special combinations
        itrans_to_iso.insert("x", "kṣ");      // ITRANS x → ISO kṣ
        itrans_to_iso.insert("kSh", "kṣ");    // Alternative
        itrans_to_iso.insert("GY", "jñ");     // ITRANS GY → ISO jñ
        itrans_to_iso.insert("j~n", "jñ");    // Alternative
        
        // Special marks
        itrans_to_iso.insert("M", "ṁ");       // ITRANS M → ISO ṁ (anusvara)
        itrans_to_iso.insert("H", "ḥ");       // ITRANS H → ISO ḥ (visarga)
        itrans_to_iso.insert("~", "m̐");       // ITRANS ~ → ISO m̐ (candrabindu)
        itrans_to_iso.insert("'", "'");       // Avagraha (identical)
        
        // Digits (identical)
        for i in 0..=9 {
            let digit_str = i.to_string();
            let digit_key = digit_str.clone();
            itrans_to_iso.insert(digit_key.leak(), digit_str.leak());
        }
        
        // Punctuation
        itrans_to_iso.insert("|", "।");       // ITRANS | → danda
        itrans_to_iso.insert("||", "॥");      // ITRANS || → double danda
        
        // Build reverse mapping for ISO → ITRANS conversion
        let mut iso_to_itrans = HashMap::new();
        for (&itrans, &iso) in &itrans_to_iso {
            iso_to_itrans.insert(iso, itrans);
        }

        Self {
            itrans_to_iso_map: itrans_to_iso,
            iso_to_itrans_map: iso_to_itrans,
        }
    }
    
    /// Convert ITRANS text to ISO-15919 format
    pub fn itrans_to_iso(&self, input: &str) -> Result<String, ConverterError> {
        RomanScriptProcessor::process_optimized(input, &self.itrans_to_iso_map)
    }
    
    /// Convert ISO-15919 text to ITRANS format (reverse conversion)
    pub fn iso_to_itrans(&self, input: &str) -> Result<String, ConverterError> {
        RomanScriptProcessor::process_optimized(input, &self.iso_to_itrans_map)
    }
}

impl ScriptConverter for ITRANSConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "itrans" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "ITRANS converter only supports 'itrans' script".to_string(),
            });
        }
        
        let iso_text = self.itrans_to_iso(input)
            .map_err(|e| ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: format!("ITRANS to ISO conversion failed: {}", e),
            })?;
            
        Ok(HubInput::Iso(iso_text))
    }
    
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        if script != "itrans" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "ITRANS converter only supports 'itrans' script".to_string(),
            });
        }
        
        match hub_input {
            HubInput::Iso(iso_text) => self.iso_to_itrans(iso_text),
            HubInput::Devanagari(_) => Err(ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: "ITRANS converter expects ISO input, got Devanagari".to_string(),
            }),
        }
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["itrans"]
    }
    
    fn script_has_implicit_a(&self, script: &str) -> bool {
        // ITRANS is a romanization scheme - consonants do NOT have implicit 'a'
        false
    }
}

impl Default for ITRANSConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_itrans_basic_vowels() {
        let converter = ITRANSConverter::new();
        
        // Test basic vowels
        assert_eq!(converter.itrans_to_iso("a").unwrap(), "a");
        assert_eq!(converter.itrans_to_iso("A").unwrap(), "ā");
        assert_eq!(converter.itrans_to_iso("aa").unwrap(), "ā");
        assert_eq!(converter.itrans_to_iso("i").unwrap(), "i");
        assert_eq!(converter.itrans_to_iso("I").unwrap(), "ī");
        assert_eq!(converter.itrans_to_iso("ii").unwrap(), "ī");
        assert_eq!(converter.itrans_to_iso("u").unwrap(), "u");
        assert_eq!(converter.itrans_to_iso("U").unwrap(), "ū");
        assert_eq!(converter.itrans_to_iso("uu").unwrap(), "ū");
    }
    
    #[test]
    fn test_itrans_vocalic_vowels() {
        let converter = ITRANSConverter::new();
        
        // Test ITRANS → ISO vocalic vowels
        assert_eq!(converter.itrans_to_iso("R").unwrap(), "r̥");
        assert_eq!(converter.itrans_to_iso("RR").unwrap(), "r̥̄");
        // L is ambiguous in ITRANS - context dependent
        // For now, prioritize retroflex L consonant over vocalic l
        assert_eq!(converter.itrans_to_iso("L").unwrap(), "ḷ");  
        assert_eq!(converter.itrans_to_iso("LL").unwrap(), "l̥̄");
        assert_eq!(converter.itrans_to_iso("lR").unwrap(), "l̥");
        assert_eq!(converter.itrans_to_iso("lRR").unwrap(), "l̥̄");
    }
    
    #[test]
    fn test_itrans_consonants() {
        let converter = ITRANSConverter::new();
        
        // Test basic consonants (no inherent 'a')
        assert_eq!(converter.itrans_to_iso("k").unwrap(), "k");
        assert_eq!(converter.itrans_to_iso("kh").unwrap(), "kh");
        assert_eq!(converter.itrans_to_iso("g").unwrap(), "g");
        assert_eq!(converter.itrans_to_iso("gh").unwrap(), "gh");
        
        // Test retroflex consonants (no inherent 'a')
        assert_eq!(converter.itrans_to_iso("T").unwrap(), "ṭ");
        assert_eq!(converter.itrans_to_iso("Th").unwrap(), "ṭh");
        assert_eq!(converter.itrans_to_iso("D").unwrap(), "ḍ");
        assert_eq!(converter.itrans_to_iso("Dh").unwrap(), "ḍh");
        assert_eq!(converter.itrans_to_iso("N").unwrap(), "ṇ");
        
        // Test sibilants (no inherent 'a')
        assert_eq!(converter.itrans_to_iso("sh").unwrap(), "ś");
        assert_eq!(converter.itrans_to_iso("Sh").unwrap(), "ṣ");
        assert_eq!(converter.itrans_to_iso("s").unwrap(), "s");
    }
    
    #[test]
    fn test_itrans_special_marks() {
        let converter = ITRANSConverter::new();
        
        // Test special marks
        assert_eq!(converter.itrans_to_iso("M").unwrap(), "ṁ");
        assert_eq!(converter.itrans_to_iso("H").unwrap(), "ḥ");
        assert_eq!(converter.itrans_to_iso("~").unwrap(), "m̐");
    }
    
    #[test]
    fn test_itrans_special_combinations() {
        let converter = ITRANSConverter::new();
        
        // Test special combinations
        assert_eq!(converter.itrans_to_iso("x").unwrap(), "kṣ");
        assert_eq!(converter.itrans_to_iso("kSh").unwrap(), "kṣ");
        assert_eq!(converter.itrans_to_iso("GY").unwrap(), "jñ");
        assert_eq!(converter.itrans_to_iso("j~n").unwrap(), "jñ");
    }
    
    #[test]
    fn test_itrans_complex_text() {
        let converter = ITRANSConverter::new();
        
        // Test ITRANS-specific features (not mixed romanization)
        let itrans_text = "kSh x M H";
        let expected_iso = "kṣ kṣ ṁ ḥ";
        assert_eq!(converter.itrans_to_iso(itrans_text).unwrap(), expected_iso);
        
        // Test combinations that are specifically ITRANS
        let itrans_text2 = "GY Th Dh";
        let expected_iso2 = "jñ ṭh ḍh";
        assert_eq!(converter.itrans_to_iso(itrans_text2).unwrap(), expected_iso2);
    }
    
    #[test]
    fn test_itrans_punctuation() {
        let converter = ITRANSConverter::new();
        
        // Test punctuation conversion
        assert_eq!(converter.itrans_to_iso("|").unwrap(), "।");
        assert_eq!(converter.itrans_to_iso("||").unwrap(), "॥");
    }
    
    #[test]
    fn test_script_converter_interface() {
        let converter = ITRANSConverter::new();
        
        // Test the ScriptConverter interface
        assert!(converter.supports_script("itrans"));
        assert!(!converter.supports_script("iast"));
        
        let result = converter.to_hub("itrans", "k").unwrap();
        if let HubInput::Iso(iso_text) = result {
            assert_eq!(iso_text, "k");
        } else {
            panic!("Expected ISO hub input");
        }
    }
}