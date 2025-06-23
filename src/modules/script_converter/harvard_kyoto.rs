use std::collections::HashMap;
use super::{ScriptConverter, ConverterError};
use super::processors::RomanScriptProcessor;
use crate::modules::hub::HubInput;

/// Harvard-Kyoto to ISO-15919 converter
/// 
/// Harvard-Kyoto is a popular ASCII-based transliteration scheme for Sanskrit.
/// It uses only basic ASCII characters and is designed to be easily typed
/// on standard keyboards while maintaining readability.
pub struct HarvardKyotoConverter {
    hk_to_iso_map: HashMap<&'static str, &'static str>,
    iso_to_hk_map: HashMap<&'static str, &'static str>,
}

impl HarvardKyotoConverter {
    pub fn new() -> Self {
        let mut hk_to_iso = HashMap::new();
        
        // Harvard-Kyoto uses straightforward ASCII mappings:
        // - Long vowels use capital letters (A, I, U) or doubled letters (aa, ii, uu)
        // - Retroflex consonants use capital letters (T, D, N, etc.)
        // - Aspirated consonants use 'h' after the consonant
        // - Special sounds use 'R' for vocalic r, 'z' for palatals, etc.
        
        // Vowels
        hk_to_iso.insert("a", "a");
        hk_to_iso.insert("A", "ā");        // HK A → ISO ā
        hk_to_iso.insert("aa", "ā");       // Alternative 
        hk_to_iso.insert("i", "i");
        hk_to_iso.insert("I", "ī");        // HK I → ISO ī
        hk_to_iso.insert("ii", "ī");       // Alternative
        hk_to_iso.insert("u", "u");
        hk_to_iso.insert("U", "ū");        // HK U → ISO ū
        hk_to_iso.insert("uu", "ū");       // Alternative
        hk_to_iso.insert("R", "r̥");        // HK R → ISO r̥
        hk_to_iso.insert("RR", "r̥̄");      // HK RR → ISO r̥̄
        hk_to_iso.insert("lR", "l̥");       // HK lR → ISO l̥
        hk_to_iso.insert("lRR", "l̥̄");     // HK lRR → ISO l̥̄
        hk_to_iso.insert("e", "e");
        hk_to_iso.insert("ai", "ai");
        hk_to_iso.insert("o", "o");
        hk_to_iso.insert("au", "au");
        
        // Consonants - Harvard-Kyoto uses ASCII combinations (no inherent 'a')
        hk_to_iso.insert("k", "k");
        hk_to_iso.insert("kh", "kh");
        hk_to_iso.insert("g", "g");
        hk_to_iso.insert("gh", "gh");
        hk_to_iso.insert("G", "ṅ");        // HK G → ISO ṅ
        hk_to_iso.insert("c", "c");
        hk_to_iso.insert("ch", "ch");
        hk_to_iso.insert("j", "j");
        hk_to_iso.insert("jh", "jh");
        hk_to_iso.insert("J", "ñ");        // HK J → ISO ñ
        hk_to_iso.insert("T", "ṭ");        // HK T → ISO ṭ
        hk_to_iso.insert("Th", "ṭh");      // HK Th → ISO ṭh
        hk_to_iso.insert("D", "ḍ");        // HK D → ISO ḍ
        hk_to_iso.insert("Dh", "ḍh");      // HK Dh → ISO ḍh
        hk_to_iso.insert("N", "ṇ");        // HK N → ISO ṇ
        hk_to_iso.insert("t", "t");
        hk_to_iso.insert("th", "th");
        hk_to_iso.insert("d", "d");
        hk_to_iso.insert("dh", "dh");
        hk_to_iso.insert("n", "n");
        hk_to_iso.insert("p", "p");
        hk_to_iso.insert("ph", "ph");
        hk_to_iso.insert("b", "b");
        hk_to_iso.insert("bh", "bh");
        hk_to_iso.insert("m", "m");
        hk_to_iso.insert("y", "y");
        hk_to_iso.insert("r", "r");
        hk_to_iso.insert("l", "l");
        hk_to_iso.insert("v", "v");
        hk_to_iso.insert("z", "ś");        // HK z → ISO ś
        hk_to_iso.insert("S", "ṣ");        // HK S → ISO ṣ
        hk_to_iso.insert("s", "s");
        hk_to_iso.insert("h", "h");
        
        // Additional consonants
        hk_to_iso.insert("L", "ḷ");        // HK L → ISO ḷ (retroflex L)
        
        // Nukta consonants (Persian/Arabic sounds)
        hk_to_iso.insert("q", "q");
        hk_to_iso.insert("K", "ḵẖ");       // HK K → ISO ḵẖ (aspirated kha with nukta)
        hk_to_iso.insert("x", "ḵẖ");       // Alternative
        hk_to_iso.insert("f", "f");
        hk_to_iso.insert("w", "v");        // Often mapped to v
        
        // Special combinations
        hk_to_iso.insert("kS", "kṣ");      // HK kS → ISO kṣ
        hk_to_iso.insert("jJ", "jñ");      // HK jJ → ISO jñ
        
        // Special marks
        hk_to_iso.insert("M", "ṁ");        // HK M → ISO ṁ (anusvara)
        hk_to_iso.insert("H", "ḥ");        // HK H → ISO ḥ (visarga)
        hk_to_iso.insert("~", "m̐");        // HK ~ → ISO m̐ (candrabindu)
        hk_to_iso.insert("'", "'");        // Avagraha (identical)
        
        // Digits (identical)
        for i in 0..=9 {
            let digit_str = i.to_string();
            let digit_key = digit_str.clone();
            hk_to_iso.insert(digit_key.leak(), digit_str.leak());
        }
        
        // Punctuation
        hk_to_iso.insert("|", "।");        // HK | → danda
        hk_to_iso.insert("||", "॥");       // HK || → double danda
        
        // Build reverse mapping for ISO → Harvard-Kyoto conversion
        let mut iso_to_hk = HashMap::new();
        for (&hk, &iso) in &hk_to_iso {
            iso_to_hk.insert(iso, hk);
        }

        Self {
            hk_to_iso_map: hk_to_iso,
            iso_to_hk_map: iso_to_hk,
        }
    }
    
    /// Convert Harvard-Kyoto text to ISO-15919 format
    pub fn hk_to_iso(&self, input: &str) -> Result<String, ConverterError> {
        RomanScriptProcessor::process_optimized(input, &self.hk_to_iso_map)
    }
    
    /// Convert ISO-15919 text to Harvard-Kyoto format (reverse conversion)
    pub fn iso_to_hk(&self, input: &str) -> Result<String, ConverterError> {
        RomanScriptProcessor::process_optimized(input, &self.iso_to_hk_map)
    }
}

impl ScriptConverter for HarvardKyotoConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "harvard_kyoto" && script != "hk" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Harvard-Kyoto converter only supports 'harvard_kyoto' or 'hk' script".to_string(),
            });
        }
        
        let iso_text = self.hk_to_iso(input)
            .map_err(|e| ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: format!("Harvard-Kyoto to ISO conversion failed: {}", e),
            })?;
            
        Ok(HubInput::Iso(iso_text))
    }
    
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        if script != "harvard_kyoto" && script != "hk" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Harvard-Kyoto converter only supports 'harvard_kyoto' or 'hk' script".to_string(),
            });
        }
        
        match hub_input {
            HubInput::Iso(iso_text) => self.iso_to_hk(iso_text),
            HubInput::Devanagari(_) => Err(ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: "Harvard-Kyoto converter expects ISO input, got Devanagari".to_string(),
            }),
        }
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["harvard_kyoto", "hk"]
    }
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        // Harvard-Kyoto is a romanization scheme - consonants do NOT have implicit 'a'
        false
    }
}

impl Default for HarvardKyotoConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hk_basic_vowels() {
        let converter = HarvardKyotoConverter::new();
        
        // Test basic vowels
        assert_eq!(converter.hk_to_iso("a").unwrap(), "a");
        assert_eq!(converter.hk_to_iso("A").unwrap(), "ā");
        assert_eq!(converter.hk_to_iso("aa").unwrap(), "ā");
        assert_eq!(converter.hk_to_iso("i").unwrap(), "i");
        assert_eq!(converter.hk_to_iso("I").unwrap(), "ī");
        assert_eq!(converter.hk_to_iso("ii").unwrap(), "ī");
        assert_eq!(converter.hk_to_iso("u").unwrap(), "u");
        assert_eq!(converter.hk_to_iso("U").unwrap(), "ū");
        assert_eq!(converter.hk_to_iso("uu").unwrap(), "ū");
    }
    
    #[test]
    fn test_hk_vocalic_vowels() {
        let converter = HarvardKyotoConverter::new();
        
        // Test Harvard-Kyoto → ISO vocalic vowels
        assert_eq!(converter.hk_to_iso("R").unwrap(), "r̥");    // HK R → ISO r̥
        assert_eq!(converter.hk_to_iso("RR").unwrap(), "r̥̄");   // HK RR → ISO r̥̄
        assert_eq!(converter.hk_to_iso("lR").unwrap(), "l̥");   // HK lR → ISO l̥
        assert_eq!(converter.hk_to_iso("lRR").unwrap(), "l̥̄");  // HK lRR → ISO l̥̄
    }
    
    #[test]
    fn test_hk_consonants() {
        let converter = HarvardKyotoConverter::new();
        
        // Test basic consonants (no inherent 'a')
        assert_eq!(converter.hk_to_iso("k").unwrap(), "k");
        assert_eq!(converter.hk_to_iso("kh").unwrap(), "kh");
        assert_eq!(converter.hk_to_iso("g").unwrap(), "g");
        assert_eq!(converter.hk_to_iso("gh").unwrap(), "gh");
        assert_eq!(converter.hk_to_iso("G").unwrap(), "ṅ");
        
        // Test retroflex consonants
        assert_eq!(converter.hk_to_iso("T").unwrap(), "ṭ");
        assert_eq!(converter.hk_to_iso("Th").unwrap(), "ṭh");
        assert_eq!(converter.hk_to_iso("D").unwrap(), "ḍ");
        assert_eq!(converter.hk_to_iso("Dh").unwrap(), "ḍh");
        assert_eq!(converter.hk_to_iso("N").unwrap(), "ṇ");
        
        // Test sibilants
        assert_eq!(converter.hk_to_iso("z").unwrap(), "ś");    // HK z → ISO ś
        assert_eq!(converter.hk_to_iso("S").unwrap(), "ṣ");    // HK S → ISO ṣ
        assert_eq!(converter.hk_to_iso("s").unwrap(), "s");
    }
    
    #[test]
    fn test_hk_special_marks() {
        let converter = HarvardKyotoConverter::new();
        
        // Test special marks
        assert_eq!(converter.hk_to_iso("M").unwrap(), "ṁ");
        assert_eq!(converter.hk_to_iso("H").unwrap(), "ḥ");
        assert_eq!(converter.hk_to_iso("~").unwrap(), "m̐");
    }
    
    #[test]
    fn test_hk_special_combinations() {
        let converter = HarvardKyotoConverter::new();
        
        // Test special combinations
        assert_eq!(converter.hk_to_iso("kS").unwrap(), "kṣ");   // HK kS → ISO kṣ
        assert_eq!(converter.hk_to_iso("jJ").unwrap(), "jñ");   // HK jJ → ISO jñ
    }
    
    #[test]
    fn test_hk_complex_text() {
        let converter = HarvardKyotoConverter::new();
        
        // Test Harvard-Kyoto specific features
        let hk_text = "kS z S M H";
        let expected_iso = "kṣ ś ṣ ṁ ḥ";
        assert_eq!(converter.hk_to_iso(hk_text).unwrap(), expected_iso);
    }
    
    #[test]
    fn test_script_converter_interface() {
        let converter = HarvardKyotoConverter::new();
        
        // Test the ScriptConverter interface
        assert!(converter.supports_script("harvard_kyoto"));
        assert!(converter.supports_script("hk"));
        assert!(!converter.supports_script("itrans"));
        
        // Test script_has_implicit_a
        assert!(!converter.script_has_implicit_a("harvard_kyoto"));
        assert!(!converter.script_has_implicit_a("hk"));
        
        let result = converter.to_hub("hk", "k").unwrap();
        if let HubInput::Iso(iso_text) = result {
            assert_eq!(iso_text, "k");
        } else {
            panic!("Expected ISO hub input");
        }
    }
}