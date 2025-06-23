use std::collections::HashMap;
use super::{ScriptConverter, ConverterError};
use super::processors::RomanScriptProcessor;
use crate::modules::hub::HubInput;

/// WX notation to ISO-15919 converter
/// 
/// WX notation is an ASCII-based transliteration scheme for Indic scripts developed by 
/// the Language Technologies Research Centre at IIIT Hyderabad. It provides a simple
/// ASCII-only representation of Indic scripts and is widely used in computational
/// linguistics and NLP applications for Indian languages.
pub struct WXConverter {
    wx_to_iso_map: HashMap<&'static str, &'static str>,
    iso_to_wx_map: HashMap<&'static str, &'static str>,
}

impl WXConverter {
    pub fn new() -> Self {
        let mut wx_to_iso = HashMap::new();
        
        // WX notation uses specific ASCII conventions:
        // - Capital letters for long vowels and special sounds
        // - 'x' for various special consonants
        // - Numbers for retroflex sounds
        // - Simple ASCII characters for basic sounds
        
        // Vowels
        wx_to_iso.insert("a", "a");
        wx_to_iso.insert("A", "ā");        // WX A → ISO ā
        wx_to_iso.insert("i", "i");
        wx_to_iso.insert("I", "ī");        // WX I → ISO ī
        wx_to_iso.insert("u", "u");
        wx_to_iso.insert("U", "ū");        // WX U → ISO ū
        wx_to_iso.insert("e", "e");
        wx_to_iso.insert("E", "ai");       // WX E → ISO ai
        wx_to_iso.insert("o", "o");
        wx_to_iso.insert("O", "au");       // WX O → ISO au
        wx_to_iso.insert("q", "r̥");        // WX q → ISO r̥
        wx_to_iso.insert("Q", "r̥̄");       // WX Q → ISO r̥̄
        wx_to_iso.insert("L", "l̥");        // WX L → ISO l̥
        wx_to_iso.insert("lY", "l̥̄");      // WX lY → ISO l̥̄
        
        // Consonants - WX notation uses ASCII combinations (no inherent 'a')
        wx_to_iso.insert("k", "k");
        wx_to_iso.insert("K", "kh");       // WX K → ISO kh
        wx_to_iso.insert("g", "g");
        wx_to_iso.insert("G", "gh");       // WX G → ISO gh
        wx_to_iso.insert("f", "ṅ");        // WX f → ISO ṅ
        wx_to_iso.insert("c", "c");
        wx_to_iso.insert("C", "ch");       // WX C → ISO ch
        wx_to_iso.insert("j", "j");
        wx_to_iso.insert("J", "jh");       // WX J → ISO jh
        wx_to_iso.insert("F", "ñ");        // WX F → ISO ñ
        wx_to_iso.insert("w", "ṭ");        // WX w → ISO ṭ
        wx_to_iso.insert("W", "ṭh");       // WX W → ISO ṭh
        wx_to_iso.insert("x", "ḍ");        // WX x → ISO ḍ
        wx_to_iso.insert("X", "ḍh");       // WX X → ISO ḍh
        wx_to_iso.insert("N", "ṇ");        // WX N → ISO ṇ
        wx_to_iso.insert("t", "t");
        wx_to_iso.insert("T", "th");       // WX T → ISO th
        wx_to_iso.insert("d", "d");
        wx_to_iso.insert("D", "dh");       // WX D → ISO dh
        wx_to_iso.insert("n", "n");
        wx_to_iso.insert("p", "p");
        wx_to_iso.insert("P", "ph");       // WX P → ISO ph
        wx_to_iso.insert("b", "b");
        wx_to_iso.insert("B", "bh");       // WX B → ISO bh
        wx_to_iso.insert("m", "m");
        wx_to_iso.insert("y", "y");
        wx_to_iso.insert("r", "r");
        wx_to_iso.insert("l", "l");
        wx_to_iso.insert("v", "v");
        wx_to_iso.insert("S", "ś");        // WX S → ISO ś
        wx_to_iso.insert("z", "ṣ");        // WX z → ISO ṣ
        wx_to_iso.insert("s", "s");
        wx_to_iso.insert("h", "h");
        
        // Additional consonants
        wx_to_iso.insert("lZ", "ḷ");       // WX lZ → ISO ḷ (retroflex L - using different mapping to avoid conflict)
        
        // Nukta consonants (Persian/Arabic sounds)
        wx_to_iso.insert("kZ", "q");       // WX kZ → ISO q
        wx_to_iso.insert("KZ", "ḵẖ");      // WX KZ → ISO ḵẖ
        wx_to_iso.insert("gZ", "ġ");       // WX gZ → ISO ġ
        wx_to_iso.insert("jZ", "z");       // WX jZ → ISO z
        wx_to_iso.insert("dZ", "ṛ");       // WX dZ → ISO ṛ
        wx_to_iso.insert("DZ", "ṛh");      // WX DZ → ISO ṛh
        wx_to_iso.insert("pZ", "f");       // WX pZ → ISO f
        wx_to_iso.insert("yZ", "ẏ");       // WX yZ → ISO ẏ
        
        // Special combinations
        wx_to_iso.insert("kz", "kṣ");      // WX kz → ISO kṣ
        wx_to_iso.insert("jF", "jñ");      // WX jF → ISO jñ
        wx_to_iso.insert("wR", "tr");      // WX wR → ISO tr (common combination)
        
        // Special marks
        wx_to_iso.insert("M", "ṁ");        // WX M → ISO ṁ (anusvara)
        wx_to_iso.insert("H", "ḥ");        // WX H → ISO ḥ (visarga)
        wx_to_iso.insert("Z", "m̐");        // WX Z → ISO m̐ (candrabindu)
        wx_to_iso.insert("'", "'");        // Avagraha (identical)
        
        // Digits (identical)
        for i in 0..=9 {
            let digit_str = i.to_string();
            let digit_key = digit_str.clone();
            wx_to_iso.insert(digit_key.leak(), digit_str.leak());
        }
        
        // Punctuation
        wx_to_iso.insert("|", "।");        // WX | → danda
        wx_to_iso.insert("||", "॥");       // WX || → double danda
        
        // Build reverse mapping for ISO → WX conversion
        let mut iso_to_wx = HashMap::new();
        for (&wx, &iso) in &wx_to_iso {
            iso_to_wx.insert(iso, wx);
        }

        Self {
            wx_to_iso_map: wx_to_iso,
            iso_to_wx_map: iso_to_wx,
        }
    }
    
    /// Convert WX notation text to ISO-15919 format
    pub fn wx_to_iso(&self, input: &str) -> Result<String, ConverterError> {
        RomanScriptProcessor::process_optimized(input, &self.wx_to_iso_map)
    }
    
    /// Convert ISO-15919 text to WX notation format (reverse conversion)
    pub fn iso_to_wx(&self, input: &str) -> Result<String, ConverterError> {
        RomanScriptProcessor::process_optimized(input, &self.iso_to_wx_map)
    }
}

impl ScriptConverter for WXConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "wx" && script != "wx_notation" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "WX converter only supports 'wx' or 'wx_notation' script".to_string(),
            });
        }
        
        let iso_text = self.wx_to_iso(input)
            .map_err(|e| ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: format!("WX to ISO conversion failed: {}", e),
            })?;
            
        Ok(HubInput::Iso(iso_text))
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["wx", "wx_notation"]
    }
    
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        if script != "wx" && script != "wx_notation" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "WX converter only supports 'wx' or 'wx_notation' script".to_string(),
            });
        }
        
        match hub_input {
            HubInput::Iso(iso_text) => {
                self.iso_to_wx(iso_text)
                    .map_err(|e| ConverterError::ConversionFailed {
                        script: script.to_string(),
                        reason: format!("ISO to WX conversion failed: {}", e),
                    })
            }
            HubInput::Devanagari(_) => {
                Err(ConverterError::ConversionFailed {
                    script: script.to_string(),
                    reason: "WX converter expects ISO hub input, not Devanagari".to_string(),
                })
            }
        }
    }
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        // WX notation is a romanization scheme - consonants do NOT have implicit 'a'
        false
    }
}

impl Default for WXConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_wx_basic_vowels() {
        let converter = WXConverter::new();
        
        // Test basic vowels
        assert_eq!(converter.wx_to_iso("a").unwrap(), "a");
        assert_eq!(converter.wx_to_iso("A").unwrap(), "ā");
        assert_eq!(converter.wx_to_iso("i").unwrap(), "i");
        assert_eq!(converter.wx_to_iso("I").unwrap(), "ī");
        assert_eq!(converter.wx_to_iso("u").unwrap(), "u");
        assert_eq!(converter.wx_to_iso("U").unwrap(), "ū");
        assert_eq!(converter.wx_to_iso("e").unwrap(), "e");
        assert_eq!(converter.wx_to_iso("E").unwrap(), "ai");
        assert_eq!(converter.wx_to_iso("o").unwrap(), "o");
        assert_eq!(converter.wx_to_iso("O").unwrap(), "au");
    }
    
    #[test]
    fn test_wx_vocalic_vowels() {
        let converter = WXConverter::new();
        
        // Test WX → ISO vocalic vowels
        assert_eq!(converter.wx_to_iso("q").unwrap(), "r̥");    // WX q → ISO r̥
        assert_eq!(converter.wx_to_iso("Q").unwrap(), "r̥̄");   // WX Q → ISO r̥̄
        assert_eq!(converter.wx_to_iso("L").unwrap(), "l̥");    // WX L → ISO l̥
        assert_eq!(converter.wx_to_iso("lY").unwrap(), "l̥̄");   // WX lY → ISO l̥̄
    }
    
    #[test]
    fn test_wx_consonants() {
        let converter = WXConverter::new();
        
        // Test basic consonants (no inherent 'a')
        assert_eq!(converter.wx_to_iso("k").unwrap(), "k");
        assert_eq!(converter.wx_to_iso("K").unwrap(), "kh");
        assert_eq!(converter.wx_to_iso("g").unwrap(), "g");
        assert_eq!(converter.wx_to_iso("G").unwrap(), "gh");
        assert_eq!(converter.wx_to_iso("f").unwrap(), "ṅ");
        
        // Test retroflex consonants
        assert_eq!(converter.wx_to_iso("w").unwrap(), "ṭ");
        assert_eq!(converter.wx_to_iso("W").unwrap(), "ṭh");
        assert_eq!(converter.wx_to_iso("x").unwrap(), "ḍ");
        assert_eq!(converter.wx_to_iso("X").unwrap(), "ḍh");
        assert_eq!(converter.wx_to_iso("N").unwrap(), "ṇ");
        
        // Test sibilants
        assert_eq!(converter.wx_to_iso("S").unwrap(), "ś");    // WX S → ISO ś
        assert_eq!(converter.wx_to_iso("z").unwrap(), "ṣ");    // WX z → ISO ṣ
        assert_eq!(converter.wx_to_iso("s").unwrap(), "s");
    }
    
    #[test]
    fn test_wx_special_marks() {
        let converter = WXConverter::new();
        
        // Test special marks
        assert_eq!(converter.wx_to_iso("M").unwrap(), "ṁ");
        assert_eq!(converter.wx_to_iso("H").unwrap(), "ḥ");
        assert_eq!(converter.wx_to_iso("Z").unwrap(), "m̐");
    }
    
    #[test]
    fn test_wx_special_combinations() {
        let converter = WXConverter::new();
        
        // Test special combinations
        assert_eq!(converter.wx_to_iso("kz").unwrap(), "kṣ");   // WX kz → ISO kṣ
        assert_eq!(converter.wx_to_iso("jF").unwrap(), "jñ");   // WX jF → ISO jñ
    }
    
    #[test]
    fn test_wx_complex_text() {
        let converter = WXConverter::new();
        
        // Test WX specific features
        let wx_text = "kz S z M H";
        let expected_iso = "kṣ ś ṣ ṁ ḥ";
        assert_eq!(converter.wx_to_iso(wx_text).unwrap(), expected_iso);
    }
    
    #[test]
    fn test_wx_bidirectional_conversion() {
        let converter = WXConverter::new();
        
        // Test WX → ISO → WX roundtrip
        let original = "Darma kz S z M H";
        let iso_result = converter.wx_to_iso(original).unwrap();
        let wx_result = converter.iso_to_wx(&iso_result).unwrap();
        assert_eq!(original, wx_result);
        
        // Test specific bidirectional conversions
        assert_eq!(converter.iso_to_wx("ṭ").unwrap(), "w");
        assert_eq!(converter.iso_to_wx("ṭh").unwrap(), "W");
        assert_eq!(converter.iso_to_wx("ḍ").unwrap(), "x");
        assert_eq!(converter.iso_to_wx("ḍh").unwrap(), "X");
        assert_eq!(converter.iso_to_wx("ṇ").unwrap(), "N");
        assert_eq!(converter.iso_to_wx("ś").unwrap(), "S");
        assert_eq!(converter.iso_to_wx("ṣ").unwrap(), "z");
        assert_eq!(converter.iso_to_wx("r̥").unwrap(), "q");
        assert_eq!(converter.iso_to_wx("r̥̄").unwrap(), "Q");
        assert_eq!(converter.iso_to_wx("l̥").unwrap(), "L");
        assert_eq!(converter.iso_to_wx("ṁ").unwrap(), "M");
        assert_eq!(converter.iso_to_wx("ḥ").unwrap(), "H");
        assert_eq!(converter.iso_to_wx("kṣ").unwrap(), "kz");
        assert_eq!(converter.iso_to_wx("jñ").unwrap(), "jF");
    }
    
    #[test]
    fn test_script_converter_interface() {
        let converter = WXConverter::new();
        
        // Test the ScriptConverter interface
        assert!(converter.supports_script("wx"));
        assert!(converter.supports_script("wx_notation"));
        assert!(!converter.supports_script("itrans"));
        
        // Test script_has_implicit_a
        assert!(!converter.script_has_implicit_a("wx"));
        assert!(!converter.script_has_implicit_a("wx_notation"));
        
        // Test to_hub
        let result = converter.to_hub("wx", "k").unwrap();
        if let HubInput::Iso(iso_text) = result {
            assert_eq!(iso_text, "k");
        } else {
            panic!("Expected ISO hub input");
        }
        
        // Test from_hub
        let hub_input = HubInput::Iso("ṭ".to_string());
        let result = converter.from_hub("wx", &hub_input).unwrap();
        assert_eq!(result, "w");
    }
}