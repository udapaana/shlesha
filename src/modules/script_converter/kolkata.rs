use std::collections::HashMap;
use super::{ScriptConverter, ConverterError};
use super::processors_optimized::OptimizedRomanScriptProcessor;
use crate::modules::hub::HubInput;

/// Kolkata (Calcutta) romanization scheme converter
/// 
/// The Kolkata romanization is a widely used transliteration system that follows
/// the conventions established by the Asiatic Society of Bengal and used in many
/// academic and reference works. It uses standard diacritics and is similar to
/// IAST but with some distinct conventions.
pub struct KolkataConverter {
    kolkata_to_iso_map: HashMap<&'static str, &'static str>,
    iso_to_kolkata_map: HashMap<&'static str, &'static str>,
}

impl KolkataConverter {
    pub fn new() -> Self {
        let mut kolkata_to_iso = HashMap::new();
        
        // Vowels - Kolkata uses standard diacritics similar to IAST
        kolkata_to_iso.insert("a", "a");
        kolkata_to_iso.insert("ā", "ā");
        kolkata_to_iso.insert("i", "i");
        kolkata_to_iso.insert("ī", "ī");
        kolkata_to_iso.insert("u", "u");
        kolkata_to_iso.insert("ū", "ū");
        kolkata_to_iso.insert("ṛ", "r̥");        // Kolkata ṛ → ISO r̥
        kolkata_to_iso.insert("ṝ", "r̥̄");       // Kolkata ṝ → ISO r̥̄
        kolkata_to_iso.insert("ḷ", "l̥");        // Kolkata ḷ → ISO l̥
        kolkata_to_iso.insert("ḹ", "l̥̄");       // Kolkata ḹ → ISO l̥̄
        kolkata_to_iso.insert("e", "e");
        kolkata_to_iso.insert("ai", "ai");
        kolkata_to_iso.insert("o", "o");
        kolkata_to_iso.insert("au", "au");
        
        // Consonants - Kolkata conventions
        kolkata_to_iso.insert("k", "k");
        kolkata_to_iso.insert("kh", "kh");
        kolkata_to_iso.insert("g", "g");
        kolkata_to_iso.insert("gh", "gh");
        kolkata_to_iso.insert("ṅ", "ṅ");
        
        kolkata_to_iso.insert("c", "c");
        kolkata_to_iso.insert("ch", "ch");
        kolkata_to_iso.insert("j", "j");
        kolkata_to_iso.insert("jh", "jh");
        kolkata_to_iso.insert("ñ", "ñ");
        
        // Retroflex - Kolkata uses dots under letters
        kolkata_to_iso.insert("ṭ", "ṭ");
        kolkata_to_iso.insert("ṭh", "ṭh");
        kolkata_to_iso.insert("ḍ", "ḍ");
        kolkata_to_iso.insert("ḍh", "ḍh");
        kolkata_to_iso.insert("ṇ", "ṇ");
        
        // Dental
        kolkata_to_iso.insert("t", "t");
        kolkata_to_iso.insert("th", "th");
        kolkata_to_iso.insert("d", "d");
        kolkata_to_iso.insert("dh", "dh");
        kolkata_to_iso.insert("n", "n");
        
        // Labial
        kolkata_to_iso.insert("p", "p");
        kolkata_to_iso.insert("ph", "ph");
        kolkata_to_iso.insert("b", "b");
        kolkata_to_iso.insert("bh", "bh");
        kolkata_to_iso.insert("m", "m");
        
        // Semivowels
        kolkata_to_iso.insert("y", "y");
        kolkata_to_iso.insert("r", "r");
        kolkata_to_iso.insert("l", "l");
        kolkata_to_iso.insert("v", "v");
        
        // Sibilants - Kolkata distinctive features
        kolkata_to_iso.insert("ś", "ś");        // Kolkata ś → ISO ś
        kolkata_to_iso.insert("ṣ", "ṣ");        // Kolkata ṣ → ISO ṣ
        kolkata_to_iso.insert("s", "s");
        kolkata_to_iso.insert("h", "h");
        
        // Additional consonants
        kolkata_to_iso.insert("ḷ", "ḷ");        // Retroflex l
        kolkata_to_iso.insert("kṣ", "kṣ");      // Compound kṣ
        kolkata_to_iso.insert("jñ", "jñ");      // Compound jñ
        
        // Nasalization and other marks
        kolkata_to_iso.insert("ṃ", "ṃ");        // Anusvara
        kolkata_to_iso.insert("ḥ", "ḥ");        // Visarga
        kolkata_to_iso.insert("'", "'");        // Avagraha
        
        // Special Kolkata conventions for conjuncts
        kolkata_to_iso.insert("kṣ", "kṣ");
        kolkata_to_iso.insert("tr", "tr");
        kolkata_to_iso.insert("śr", "śr");
        
        // Build reverse mapping for ISO → Kolkata
        let mut iso_to_kolkata = HashMap::new();
        for (&kolkata, &iso) in &kolkata_to_iso {
            iso_to_kolkata.insert(iso, kolkata);
        }
        
        // Handle special reverse mappings where ISO differs
        iso_to_kolkata.insert("r̥", "ṛ");        // ISO r̥ → Kolkata ṛ
        iso_to_kolkata.insert("r̥̄", "ṝ");       // ISO r̥̄ → Kolkata ṝ
        iso_to_kolkata.insert("l̥", "ḷ");        // ISO l̥ → Kolkata ḷ
        iso_to_kolkata.insert("l̥̄", "ḹ");       // ISO l̥̄ → Kolkata ḹ

        Self {
            kolkata_to_iso_map: kolkata_to_iso,
            iso_to_kolkata_map: iso_to_kolkata,
        }
    }
    
    /// Convert Kolkata romanization to ISO-15919 (optimized)
    pub fn kolkata_to_iso_optimized(&self, input: &str) -> Result<String, ConverterError> {
        OptimizedRomanScriptProcessor::process_auto(input, &self.kolkata_to_iso_map)
    }
    
    /// Convert ISO-15919 to Kolkata romanization (optimized)
    pub fn iso_to_kolkata_optimized(&self, input: &str) -> Result<String, ConverterError> {
        OptimizedRomanScriptProcessor::process_auto(input, &self.iso_to_kolkata_map)
    }
}

impl ScriptConverter for KolkataConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "kolkata" && script != "calcutta" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Kolkata converter only supports 'kolkata' or 'calcutta' script".to_string(),
            });
        }
        
        let iso_text = self.kolkata_to_iso_optimized(input)
            .map_err(|e| ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: format!("Kolkata to ISO conversion failed: {}", e),
            })?;
            
        Ok(HubInput::Iso(iso_text))
    }
    
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        if script != "kolkata" && script != "calcutta" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Kolkata converter only supports 'kolkata' or 'calcutta' script".to_string(),
            });
        }
        
        match hub_input {
            HubInput::Iso(iso_text) => self.iso_to_kolkata_optimized(iso_text),
            HubInput::Devanagari(_) => Err(ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: "Kolkata converter expects ISO input, got Devanagari".to_string(),
            }),
        }
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["kolkata", "calcutta"]
    }
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        // Kolkata is a Roman script - consonants do NOT have implicit 'a'
        false
    }
}

impl Default for KolkataConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_kolkata_vowels() {
        let converter = KolkataConverter::new();
        
        assert_eq!(converter.kolkata_to_iso_optimized("a").unwrap(), "a");
        assert_eq!(converter.kolkata_to_iso_optimized("ā").unwrap(), "ā");
        assert_eq!(converter.kolkata_to_iso_optimized("ṛ").unwrap(), "r̥");
        assert_eq!(converter.kolkata_to_iso_optimized("ṝ").unwrap(), "r̥̄");
    }
    
    #[test]
    fn test_kolkata_consonants() {
        let converter = KolkataConverter::new();
        
        assert_eq!(converter.kolkata_to_iso_optimized("k").unwrap(), "k");
        assert_eq!(converter.kolkata_to_iso_optimized("kh").unwrap(), "kh");
        assert_eq!(converter.kolkata_to_iso_optimized("ṭ").unwrap(), "ṭ");
        assert_eq!(converter.kolkata_to_iso_optimized("ṣ").unwrap(), "ṣ");
    }
    
    #[test]
    fn test_kolkata_compounds() {
        let converter = KolkataConverter::new();
        
        assert_eq!(converter.kolkata_to_iso_optimized("kṣ").unwrap(), "kṣ");
        assert_eq!(converter.kolkata_to_iso_optimized("jñ").unwrap(), "jñ");
    }
    
    #[test]
    fn test_script_converter_interface() {
        let converter = KolkataConverter::new();
        
        assert!(converter.supports_script("kolkata"));
        assert!(converter.supports_script("calcutta"));
        assert!(!converter.supports_script("iast"));
        
        assert!(!converter.script_has_implicit_a("kolkata"));
        
        let result = converter.to_hub("kolkata", "dharma").unwrap();
        if let HubInput::Iso(iso_text) = result {
            assert_eq!(iso_text, "dharma");
        } else {
            panic!("Expected ISO hub input");
        }
    }
    
    #[test]
    fn test_reverse_conversion() {
        let converter = KolkataConverter::new();
        
        // Test bidirectional conversion
        let original = "dharmaṣāstra";
        let to_iso = converter.kolkata_to_iso_optimized(original).unwrap();
        let back_to_kolkata = converter.iso_to_kolkata_optimized(&to_iso).unwrap();
        assert_eq!(original, back_to_kolkata);
    }
}