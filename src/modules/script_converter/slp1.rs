use std::collections::HashMap;
use super::{ScriptConverter, ConverterError};
use crate::modules::hub::HubInput;

/// SLP1 (Sanskrit Library Phonetic Basic) to ISO-15919 converter
/// 
/// SLP1 is a strict ASCII-based transliteration scheme developed by the Sanskrit Library.
/// It's designed to be completely reversible and uses only basic ASCII characters.
/// Every Sanskrit sound has a unique ASCII representation.
pub struct SLP1Converter {
    slp1_to_iso_map: HashMap<&'static str, &'static str>,
    iso_to_slp1_map: HashMap<&'static str, &'static str>,
}

impl SLP1Converter {
    pub fn new() -> Self {
        let mut slp1_to_iso = HashMap::new();
        
        // SLP1 has very specific mappings that differ significantly from ISO-15919:
        // - Uses 'f', 'x', 'q' for specialized sounds
        // - Uses uppercase for retroflex and other sounds
        // - Completely ASCII-based with 1:1 character mapping where possible
        
        // Vowels
        slp1_to_iso.insert("a", "a");
        slp1_to_iso.insert("A", "ā");        // SLP1 A → ISO ā
        slp1_to_iso.insert("i", "i");
        slp1_to_iso.insert("I", "ī");        // SLP1 I → ISO ī
        slp1_to_iso.insert("u", "u");
        slp1_to_iso.insert("U", "ū");        // SLP1 U → ISO ū
        slp1_to_iso.insert("f", "r̥");        // SLP1 f → ISO r̥
        slp1_to_iso.insert("F", "r̥̄");       // SLP1 F → ISO r̥̄
        slp1_to_iso.insert("x", "l̥");        // SLP1 x → ISO l̥
        slp1_to_iso.insert("X", "l̥̄");       // SLP1 X → ISO l̥̄
        slp1_to_iso.insert("e", "e");
        slp1_to_iso.insert("E", "ai");       // SLP1 E → ISO ai
        slp1_to_iso.insert("o", "o");
        slp1_to_iso.insert("O", "au");       // SLP1 O → ISO au
        
        // Consonants (no inherent 'a')
        slp1_to_iso.insert("k", "k");
        slp1_to_iso.insert("K", "kh");       // SLP1 K → ISO kh
        slp1_to_iso.insert("g", "g");
        slp1_to_iso.insert("G", "gh");       // SLP1 G → ISO gh
        slp1_to_iso.insert("N", "ṅ");        // SLP1 N → ISO ṅ
        slp1_to_iso.insert("c", "c");
        slp1_to_iso.insert("C", "ch");       // SLP1 C → ISO ch
        slp1_to_iso.insert("j", "j");
        slp1_to_iso.insert("J", "jh");       // SLP1 J → ISO jh
        slp1_to_iso.insert("Y", "ñ");        // SLP1 Y → ISO ñ
        slp1_to_iso.insert("w", "ṭ");        // SLP1 w → ISO ṭ
        slp1_to_iso.insert("W", "ṭh");       // SLP1 W → ISO ṭh
        slp1_to_iso.insert("q", "ḍ");        // SLP1 q → ISO ḍ
        slp1_to_iso.insert("Q", "ḍh");       // SLP1 Q → ISO ḍh
        slp1_to_iso.insert("R", "ṇ");        // SLP1 R → ISO ṇ
        slp1_to_iso.insert("t", "t");
        slp1_to_iso.insert("T", "th");       // SLP1 T → ISO th
        slp1_to_iso.insert("d", "d");
        slp1_to_iso.insert("D", "dh");       // SLP1 D → ISO dh
        slp1_to_iso.insert("n", "n");
        slp1_to_iso.insert("p", "p");
        slp1_to_iso.insert("P", "ph");       // SLP1 P → ISO ph
        slp1_to_iso.insert("b", "b");
        slp1_to_iso.insert("B", "bh");       // SLP1 B → ISO bh
        slp1_to_iso.insert("m", "m");
        slp1_to_iso.insert("y", "y");
        slp1_to_iso.insert("r", "r");
        slp1_to_iso.insert("l", "l");
        slp1_to_iso.insert("v", "v");
        slp1_to_iso.insert("S", "ś");        // SLP1 S → ISO ś
        slp1_to_iso.insert("z", "ṣ");        // SLP1 z → ISO ṣ
        slp1_to_iso.insert("s", "s");
        slp1_to_iso.insert("h", "h");
        
        // Additional consonants
        slp1_to_iso.insert("L", "ḷ");        // SLP1 L → ISO ḷ
        
        // Nukta consonants (Persian/Arabic sounds) - SLP1 extensions
        slp1_to_iso.insert("q", "qa");       // Note: conflicts with ḍa above - context dependent
        slp1_to_iso.insert("K", "ḵẖa");      // Note: conflicts with kha above
        slp1_to_iso.insert("G", "ġa");       // Note: conflicts with gha above
        slp1_to_iso.insert("z", "za");       // Note: conflicts with ṣa above
        slp1_to_iso.insert("q", "ṛa");       // Note: multiple conflicts
        slp1_to_iso.insert("Q", "ṛha");      // Note: conflicts with ḍha above
        slp1_to_iso.insert("f", "fa");       // Note: conflicts with r̥ above
        slp1_to_iso.insert("Y", "ẏa");       // Note: conflicts with ña above
        
        // NOTE: SLP1 has character conflicts that need context resolution
        // For now, we'll prioritize the core Sanskrit sounds over nukta sounds
        // since SLP1 was primarily designed for Sanskrit, not Persian/Arabic
        
        // Re-insert core Sanskrit mappings to override conflicts (no inherent 'a')
        slp1_to_iso.insert("f", "r̥");        // Prioritize vocalic r over f
        slp1_to_iso.insert("z", "ṣ");        // Prioritize ṣ over z
        slp1_to_iso.insert("q", "ḍ");        // Prioritize ḍ over q
        slp1_to_iso.insert("Q", "ḍh");       // Prioritize ḍh over ṛh
        slp1_to_iso.insert("K", "kh");       // Prioritize kh over ḵẖ
        slp1_to_iso.insert("G", "gh");       // Prioritize gh over ġ
        slp1_to_iso.insert("Y", "ñ");        // Prioritize ñ over ẏ
        
        // Special marks
        slp1_to_iso.insert("M", "ṁ");        // SLP1 M → ISO ṁ (anusvara)
        slp1_to_iso.insert("H", "ḥ");        // SLP1 H → ISO ḥ (visarga)
        slp1_to_iso.insert("~", "m̐");        // SLP1 ~ → ISO m̐ (candrabindu) 
        slp1_to_iso.insert("'", "'");        // Avagraha (identical)
        
        // Digits (identical)
        for i in 0..=9 {
            let digit_str = i.to_string();
            let digit_key = digit_str.clone();
            slp1_to_iso.insert(digit_key.leak(), digit_str.leak());
        }
        
        // Punctuation - SLP1 typically doesn't change these
        slp1_to_iso.insert("|", "।");        // Danda
        slp1_to_iso.insert("||", "॥");       // Double danda
        
        // Build reverse mapping for ISO → SLP1 conversion
        let mut iso_to_slp1 = HashMap::new();
        for (&slp1, &iso) in &slp1_to_iso {
            iso_to_slp1.insert(iso, slp1);
        }

        Self {
            slp1_to_iso_map: slp1_to_iso,
            iso_to_slp1_map: iso_to_slp1,
        }
    }
    
    /// Convert SLP1 text to ISO-15919 format
    pub fn slp1_to_iso(&self, input: &str) -> Result<String, ConverterError> {
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
            if ch.is_ascii_punctuation() && ch != '|' && ch != '~' && ch != '\'' {
                result.push(ch);
                i += 1;
                continue;
            }
            
            let mut matched = false;
            
            // Try to match sequences - SLP1 mostly has single-character mappings
            // but we check for || first
            for len in (1..=2).rev() {
                if i + len > chars.len() {
                    continue;
                }
                
                let seq: String = chars[i..i+len].iter().collect();
                if let Some(&iso_str) = self.slp1_to_iso_map.get(seq.as_str()) {
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
    
    /// Convert ISO-15919 text to SLP1 format (reverse conversion)
    pub fn iso_to_slp1(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            let ch = chars[i];
            
            // Handle whitespace (preserve as-is)
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
                if let Some(&slp1_str) = self.iso_to_slp1_map.get(seq.as_str()) {
                    result.push_str(slp1_str);
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

impl ScriptConverter for SLP1Converter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "slp1" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "SLP1 converter only supports 'slp1' script".to_string(),
            });
        }
        
        let iso_text = self.slp1_to_iso(input)
            .map_err(|e| ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: format!("SLP1 to ISO conversion failed: {}", e),
            })?;
            
        Ok(HubInput::Iso(iso_text))
    }
    
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        if script != "slp1" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "SLP1 converter only supports 'slp1' script".to_string(),
            });
        }
        
        match hub_input {
            HubInput::Iso(iso_text) => self.iso_to_slp1(iso_text),
            HubInput::Devanagari(_) => Err(ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: "SLP1 converter expects ISO input, got Devanagari".to_string(),
            }),
        }
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["slp1"]
    }
    
    fn script_has_implicit_a(&self, script: &str) -> bool {
        // SLP1 is a romanization scheme - consonants do NOT have implicit 'a'
        false
    }
}

impl Default for SLP1Converter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_slp1_basic_vowels() {
        let converter = SLP1Converter::new();
        
        // Test basic vowels
        assert_eq!(converter.slp1_to_iso("a").unwrap(), "a");
        assert_eq!(converter.slp1_to_iso("A").unwrap(), "ā");
        assert_eq!(converter.slp1_to_iso("i").unwrap(), "i");
        assert_eq!(converter.slp1_to_iso("I").unwrap(), "ī");
        assert_eq!(converter.slp1_to_iso("u").unwrap(), "u");
        assert_eq!(converter.slp1_to_iso("U").unwrap(), "ū");
    }
    
    #[test]
    fn test_slp1_vocalic_vowels() {
        let converter = SLP1Converter::new();
        
        // Test SLP1 → ISO vocalic vowels
        assert_eq!(converter.slp1_to_iso("f").unwrap(), "r̥");    // SLP1 f → ISO r̥
        assert_eq!(converter.slp1_to_iso("F").unwrap(), "r̥̄");   // SLP1 F → ISO r̥̄
        assert_eq!(converter.slp1_to_iso("x").unwrap(), "l̥");    // SLP1 x → ISO l̥
        assert_eq!(converter.slp1_to_iso("X").unwrap(), "l̥̄");   // SLP1 X → ISO l̥̄
    }
    
    #[test]
    fn test_slp1_diphthongs() {
        let converter = SLP1Converter::new();
        
        // Test SLP1 diphthongs
        assert_eq!(converter.slp1_to_iso("e").unwrap(), "e");
        assert_eq!(converter.slp1_to_iso("E").unwrap(), "ai");   // SLP1 E → ISO ai
        assert_eq!(converter.slp1_to_iso("o").unwrap(), "o");
        assert_eq!(converter.slp1_to_iso("O").unwrap(), "au");   // SLP1 O → ISO au
    }
    
    #[test]
    fn test_slp1_consonants() {
        let converter = SLP1Converter::new();
        
        // Test basic consonants (no inherent 'a')
        assert_eq!(converter.slp1_to_iso("k").unwrap(), "k");
        assert_eq!(converter.slp1_to_iso("K").unwrap(), "kh");
        assert_eq!(converter.slp1_to_iso("g").unwrap(), "g");
        assert_eq!(converter.slp1_to_iso("G").unwrap(), "gh");
        assert_eq!(converter.slp1_to_iso("N").unwrap(), "ṅ");
        
        // Test retroflex consonants (no inherent 'a')
        assert_eq!(converter.slp1_to_iso("w").unwrap(), "ṭ");   // SLP1 w → ISO ṭ
        assert_eq!(converter.slp1_to_iso("W").unwrap(), "ṭh");  // SLP1 W → ISO ṭh
        assert_eq!(converter.slp1_to_iso("q").unwrap(), "ḍ");   // SLP1 q → ISO ḍ
        assert_eq!(converter.slp1_to_iso("Q").unwrap(), "ḍh");  // SLP1 Q → ISO ḍh
        assert_eq!(converter.slp1_to_iso("R").unwrap(), "ṇ");   // SLP1 R → ISO ṇ
        
        // Test sibilants (no inherent 'a')
        assert_eq!(converter.slp1_to_iso("S").unwrap(), "ś");   // SLP1 S → ISO ś
        assert_eq!(converter.slp1_to_iso("z").unwrap(), "ṣ");   // SLP1 z → ISO ṣ
        assert_eq!(converter.slp1_to_iso("s").unwrap(), "s");
    }
    
    #[test]
    fn test_slp1_special_marks() {
        let converter = SLP1Converter::new();
        
        // Test special marks
        assert_eq!(converter.slp1_to_iso("M").unwrap(), "ṁ");
        assert_eq!(converter.slp1_to_iso("H").unwrap(), "ḥ");
        assert_eq!(converter.slp1_to_iso("~").unwrap(), "m̐");
    }
    
    #[test]
    fn test_slp1_complex_text() {
        let converter = SLP1Converter::new();
        
        // Test SLP1-specific mappings
        let slp1_text = "M H w z S";
        let expected_iso = "ṁ ḥ ṭ ṣ ś";
        assert_eq!(converter.slp1_to_iso(slp1_text).unwrap(), expected_iso);
    }
    
    #[test]
    fn test_slp1_unique_mappings() {
        let converter = SLP1Converter::new();
        
        // Test unique SLP1 mappings (no inherent 'a')
        assert_eq!(converter.slp1_to_iso("Y").unwrap(), "ñ");   // SLP1 Y → ISO ñ
        assert_eq!(converter.slp1_to_iso("L").unwrap(), "ḷ");   // SLP1 L → ISO ḷ
    }
    
    #[test]
    fn test_script_converter_interface() {
        let converter = SLP1Converter::new();
        
        // Test the ScriptConverter interface
        assert!(converter.supports_script("slp1"));
        assert!(!converter.supports_script("iast"));
        
        let result = converter.to_hub("slp1", "k").unwrap();
        if let HubInput::Iso(iso_text) = result {
            assert_eq!(iso_text, "k");
        } else {
            panic!("Expected ISO hub input");
        }
    }
}