use std::collections::HashMap;
use once_cell::sync::Lazy;
use super::{ScriptConverter, ConverterError};
use super::processors_optimized::OptimizedRomanScriptProcessor;
use crate::modules::hub::HubInput;

/// Optimized SLP1 (Sanskrit Library Phonetic) converter with eliminated allocations
pub struct OptimizedSLP1Converter {
    slp1_to_iso_map: &'static HashMap<&'static str, &'static str>,
    iso_to_slp1_map: &'static HashMap<&'static str, &'static str>,
}

// Pre-computed static mappings
static SLP1_TO_ISO_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // Simple vowels
    map.insert("a", "a");
    map.insert("A", "ā");
    map.insert("i", "i");
    map.insert("I", "ī");
    map.insert("u", "u");
    map.insert("U", "ū");
    map.insert("f", "r̥");
    map.insert("F", "r̥̄");
    map.insert("x", "l̥");
    map.insert("X", "l̥̄");
    map.insert("e", "e");
    map.insert("o", "o");
    map.insert("E", "ai");
    map.insert("O", "au");
    
    // Consonants
    map.insert("k", "k");
    map.insert("K", "kh");
    map.insert("g", "g");
    map.insert("G", "gh");
    map.insert("N", "ṅ");
    map.insert("c", "c");
    map.insert("C", "ch");
    map.insert("j", "j");
    map.insert("J", "jh");
    map.insert("Y", "ñ");
    map.insert("w", "ṭ");
    map.insert("W", "ṭh");
    map.insert("q", "ḍ");
    map.insert("Q", "ḍh");
    map.insert("R", "ṇ");
    map.insert("t", "t");
    map.insert("T", "th");
    map.insert("d", "d");
    map.insert("D", "dh");
    map.insert("n", "n");
    map.insert("p", "p");
    map.insert("P", "ph");
    map.insert("b", "b");
    map.insert("B", "bh");
    map.insert("m", "m");
    map.insert("y", "y");
    map.insert("r", "r");
    map.insert("l", "l");
    map.insert("v", "v");
    map.insert("S", "ś");
    map.insert("z", "ṣ");
    map.insert("s", "s");
    map.insert("h", "h");
    map.insert("L", "ḷ");
    
    // Special characters
    map.insert("M", "ṁ");
    map.insert("H", "ḥ");
    map.insert("~", "m̐");
    map.insert("|", " ");
    map.insert("||", " ");
    
    // Numbers (rare in SLP1 but included for completeness)
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

static ISO_TO_SLP1_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // Build reverse mapping with explicit precedence for multi-character sequences
    // Handle longer sequences first to avoid conflicts
    map.insert("r̥̄", "F");
    map.insert("l̥̄", "X");
    map.insert("r̥", "f");
    map.insert("l̥", "x");
    map.insert("kh", "K");
    map.insert("gh", "G");
    map.insert("ch", "C");
    map.insert("jh", "J");
    map.insert("ṭh", "W");
    map.insert("ḍh", "Q");
    map.insert("th", "T");
    map.insert("dh", "D");
    map.insert("ph", "P");
    map.insert("bh", "B");
    map.insert("ai", "E");
    map.insert("au", "O");
    
    // Then build the rest avoiding conflicts
    for (&slp1, &iso) in SLP1_TO_ISO_MAP.iter() {
        if !map.contains_key(iso) {
            map.insert(iso, slp1);
        }
    }
    
    map
});

impl OptimizedSLP1Converter {
    pub fn new() -> Self {
        Self {
            slp1_to_iso_map: &SLP1_TO_ISO_MAP,
            iso_to_slp1_map: &ISO_TO_SLP1_MAP,
        }
    }
    
    /// Convert SLP1 to ISO-15919 using optimized processor
    pub fn slp1_to_iso_optimized(&self, input: &str) -> Result<String, ConverterError> {
        OptimizedRomanScriptProcessor::process_auto(input, self.slp1_to_iso_map)
    }
    
    /// Convert ISO-15919 to SLP1 using optimized processor
    pub fn iso_to_slp1_optimized(&self, input: &str) -> Result<String, ConverterError> {
        OptimizedRomanScriptProcessor::process_auto(input, self.iso_to_slp1_map)
    }
}

impl ScriptConverter for OptimizedSLP1Converter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "slp1" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "SLP1 converter only supports 'slp1' script".to_string(),
            });
        }
        
        let iso_text = self.slp1_to_iso_optimized(input)?;
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
            HubInput::Iso(iso_text) => self.iso_to_slp1_optimized(iso_text),
            HubInput::Devanagari(_) => Err(ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: "SLP1 converter expects ISO input, got Devanagari".to_string(),
            }),
        }
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["slp1"]
    }
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        false // SLP1 is a romanization scheme without implicit vowels
    }
}

impl Default for OptimizedSLP1Converter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_optimized_slp1_basic() {
        let converter = OptimizedSLP1Converter::new();
        
        // Test simple conversions
        assert_eq!(converter.slp1_to_iso_optimized("a").unwrap(), "a");
        assert_eq!(converter.slp1_to_iso_optimized("A").unwrap(), "ā");
        assert_eq!(converter.slp1_to_iso_optimized("k").unwrap(), "k");
        assert_eq!(converter.slp1_to_iso_optimized("K").unwrap(), "kh");
    }
    
    #[test]
    fn test_optimized_slp1_complex() {
        let converter = OptimizedSLP1Converter::new();
        
        // Test complex words
        let result = converter.slp1_to_iso_optimized("dharmakSetrekukSAstramulamasUktamukhAdharmakSetrajYAnamukhAnamastubhyam").unwrap();
        assert!(result.contains("dharma"));
        assert!(result.contains("śāstra"));
    }
    
    #[test]
    fn test_optimized_round_trip() {
        let converter = OptimizedSLP1Converter::new();
        
        // Test individual character mappings first
        assert_eq!(converter.slp1_to_iso_optimized("D").unwrap(), "dh");
        assert_eq!(converter.iso_to_slp1_optimized("dh").unwrap(), "D");
        
        let original = "DarmakSetra"; // Fixed: use D instead of dh in SLP1
        let iso = converter.slp1_to_iso_optimized(original).unwrap();
        let back_to_slp1 = converter.iso_to_slp1_optimized(&iso).unwrap();
        
        assert_eq!(original, back_to_slp1);
    }
}