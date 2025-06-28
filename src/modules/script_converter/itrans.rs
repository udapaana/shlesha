use std::collections::HashMap;
use once_cell::sync::Lazy;
use super::{ScriptConverter, ConverterError};
use super::processors_optimized::OptimizedRomanScriptProcessor;
use crate::modules::hub::HubInput;

/// Optimized ITRANS (Indian Language Transliteration) to ISO-15919 converter
pub struct ITRANSConverter {
    itrans_to_iso_map: &'static HashMap<&'static str, &'static str>,
    iso_to_itrans_map: &'static HashMap<&'static str, &'static str>,
}

// Pre-computed static mappings
static ITRANS_TO_ISO_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // Vowels
    map.insert("a", "a");
    map.insert("A", "ā");       // ITRANS A → ISO ā
    map.insert("aa", "ā");      // Alternative for ā
    map.insert("i", "i");
    map.insert("I", "ī");       // ITRANS I → ISO ī
    map.insert("ii", "ī");      // Alternative for ī
    map.insert("u", "u");
    map.insert("U", "ū");       // ITRANS U → ISO ū
    map.insert("uu", "ū");      // Alternative for ū
    map.insert("R", "r̥");       // ITRANS R → ISO r̥
    map.insert("RR", "r̥̄");     // ITRANS RR → ISO r̥̄
    map.insert("lR", "l̥");      // ITRANS lR → ISO l̥
    map.insert("lRR", "l̥̄");    // ITRANS lRR → ISO l̥̄
    map.insert("L", "l̥");       // Alternative L → ISO l̥
    map.insert("LL", "l̥̄");     // Alternative LL → ISO l̥̄
    map.insert("e", "e");
    map.insert("ai", "ai");
    map.insert("o", "o");
    map.insert("au", "au");
    
    // Consonants - ITRANS uses ASCII combinations
    map.insert("k", "k");
    map.insert("kh", "kh");     // ITRANS kh → ISO kh
    map.insert("g", "g");
    map.insert("gh", "gh");     // ITRANS gh → ISO gh
    map.insert("~N", "ṅ");      // ITRANS ~N → ISO ṅ
    map.insert("N^", "ṅ");      // Alternative ~N → ISO ṅ
    map.insert("ch", "ch");     // ITRANS ch → ISO ch
    map.insert("Ch", "ch");     // ITRANS Ch → ISO ch
    map.insert("chh", "ch");    // Alternative Ch → ISO ch
    map.insert("j", "j");
    map.insert("jh", "jh");     // ITRANS jh → ISO jh
    map.insert("~n", "ñ");      // ITRANS ~n → ISO ñ
    map.insert("JN", "ñ");      // Alternative ~n → ISO ñ
    map.insert("T", "ṭ");       // ITRANS T → ISO ṭ
    map.insert("Th", "ṭh");     // ITRANS Th → ISO ṭh
    map.insert("D", "ḍ");       // ITRANS D → ISO ḍ
    map.insert("Dh", "ḍh");     // ITRANS Dh → ISO ḍh
    map.insert("N", "ṇ");       // ITRANS N → ISO ṇ
    map.insert("t", "t");
    map.insert("th", "th");     // ITRANS th → ISO th
    map.insert("d", "d");
    map.insert("dh", "dh");     // ITRANS dh → ISO dh
    map.insert("n", "n");
    map.insert("p", "p");
    map.insert("ph", "ph");     // ITRANS ph → ISO ph
    map.insert("b", "b");
    map.insert("bh", "bh");     // ITRANS bh → ISO bh
    map.insert("m", "m");
    map.insert("y", "y");
    map.insert("r", "r");
    map.insert("l", "l");
    map.insert("v", "v");
    map.insert("w", "v");       // Alternative w → ISO v
    map.insert("sh", "ś");      // ITRANS sh → ISO ś
    map.insert("Sh", "ṣ");      // ITRANS Sh → ISO ṣ
    map.insert("shh", "ṣ");     // Alternative Sh → ISO ṣ
    map.insert("s", "s");
    map.insert("h", "h");
    
    // Special consonants and combinations
    map.insert("x", "kṣ");      // ITRANS x → ISO kṣ (special combination)
    map.insert("GY", "jñ");     // ITRANS GY → ISO jñ (special combination)
    map.insert("j~n", "jñ");    // Alternative GY → ISO jñ
    
    // Special marks
    map.insert("M", "ṁ");       // ITRANS M → ISO ṁ (anusvara)
    map.insert(".m", "ṁ");      // Alternative M → ISO ṁ
    map.insert("H", "ḥ");       // ITRANS H → ISO ḥ (visarga)
    map.insert(".h", "ḥ");      // Alternative H → ISO ḥ
    map.insert("~", "m̐");       // ITRANS ~ → ISO m̐ (candrabindu)
    map.insert(".N", "m̐");      // Alternative ~ → ISO m̐
    
    // Vedic accents (rare)
    map.insert("\\_", "");      // ITRANS \_ → (anudātta - often omitted in ISO)
    map.insert("\\^", "");      // ITRANS \^ → (svarita - often omitted in ISO)
    
    // Punctuation and separators
    map.insert("|", " ");       // ITRANS | → space (phrase separator)
    map.insert("||", " ");      // ITRANS || → space (verse separator)
    map.insert(".", ".");       // Period remains
    map.insert(",", ",");       // Comma remains
    
    // Numbers (ITRANS sometimes uses these)
    map.insert("0", "0");
    map.insert("1", "1");
    map.insert("2", "2");
    map.insert("3", "3");
    map.insert("4", "4");
    map.insert("5", "5");
    map.insert("6", "6");
    map.insert("7", "7");
    map.insert("8", "8");
    map.insert("9", "9");
    
    map
});

static ISO_TO_ITRANS_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // Build reverse mapping, preferring simpler ITRANS forms
    for (&itrans, &iso) in ITRANS_TO_ISO_MAP.iter() {
        // Skip alternatives and use the primary forms
        if !map.contains_key(iso) {
            map.insert(iso, itrans);
        }
    }
    
    // Override with preferred forms for reverse conversion
    map.insert("ā", "A");
    map.insert("ī", "I");
    map.insert("ū", "U");
    map.insert("r̥", "R");
    map.insert("r̥̄", "RR");
    map.insert("l̥", "L");
    map.insert("l̥̄", "LL");
    map.insert("ṅ", "~N");
    map.insert("c", "ch");
    map.insert("ch", "Ch");
    map.insert("ñ", "~n");
    map.insert("ṭ", "T");
    map.insert("ṭh", "Th");
    map.insert("ḍ", "D");
    map.insert("ḍh", "Dh");
    map.insert("ṇ", "N");
    map.insert("ś", "sh");
    map.insert("ṣ", "Sh");
    map.insert("ṁ", "M");
    map.insert("ḥ", "H");
    map.insert("m̐", "~");
    map.insert("kṣ", "x");
    map.insert("jñ", "GY");
    
    map
});

impl ITRANSConverter {
    pub fn new() -> Self {
        Self {
            itrans_to_iso_map: &ITRANS_TO_ISO_MAP,
            iso_to_itrans_map: &ISO_TO_ITRANS_MAP,
        }
    }
    
    /// Convert ITRANS to ISO-15919 using optimized processor
    pub fn itrans_to_iso_optimized(&self, input: &str) -> Result<String, ConverterError> {
        OptimizedRomanScriptProcessor::process_auto(input, self.itrans_to_iso_map)
    }
    
    /// Alias for the optimized version to maintain clean API
    pub fn itrans_to_iso(&self, input: &str) -> Result<String, ConverterError> {
        self.itrans_to_iso_optimized(input)
    }
    
    /// Convert ISO-15919 to ITRANS using optimized processor
    pub fn iso_to_itrans_optimized(&self, input: &str) -> Result<String, ConverterError> {
        OptimizedRomanScriptProcessor::process_auto(input, self.iso_to_itrans_map)
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
        
        let iso_text = self.itrans_to_iso_optimized(input)?;
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
            HubInput::Iso(iso_text) => self.iso_to_itrans_optimized(iso_text),
            HubInput::Devanagari(_) => Err(ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: "ITRANS converter expects ISO input, got Devanagari".to_string(),
            }),
        }
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["itrans"]
    }
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        false // ITRANS is a romanization scheme without implicit vowels
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
    fn test_optimized_itrans_basic() {
        let converter = ITRANSConverter::new();
        
        // Test simple conversions
        assert_eq!(converter.itrans_to_iso_optimized("a").unwrap(), "a");
        assert_eq!(converter.itrans_to_iso_optimized("A").unwrap(), "ā");
        assert_eq!(converter.itrans_to_iso_optimized("k").unwrap(), "k");
        assert_eq!(converter.itrans_to_iso_optimized("kh").unwrap(), "kh");
    }
    
    #[test]
    fn test_optimized_itrans_special() {
        let converter = ITRANSConverter::new();
        
        // Test special combinations
        assert_eq!(converter.itrans_to_iso_optimized("x").unwrap(), "kṣ");
        assert_eq!(converter.itrans_to_iso_optimized("GY").unwrap(), "jñ");
        assert_eq!(converter.itrans_to_iso_optimized("M").unwrap(), "ṁ");
        assert_eq!(converter.itrans_to_iso_optimized("H").unwrap(), "ḥ");
    }
    
    #[test]
    fn test_optimized_round_trip() {
        let converter = ITRANSConverter::new();
        
        let original = "dharma";
        let iso = converter.itrans_to_iso_optimized(original).unwrap();
        let back_to_itrans = converter.iso_to_itrans_optimized(&iso).unwrap();
        
        // Note: May not be exact due to alternative representations
        assert!(back_to_itrans.contains("dharma") || back_to_itrans == "dharma");
    }
}