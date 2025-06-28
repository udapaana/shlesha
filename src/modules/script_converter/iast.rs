use std::collections::HashMap;
use once_cell::sync::Lazy;
use super::{ScriptConverter, ConverterError};
use super::processors::RomanScriptProcessor;
use crate::modules::hub::HubInput;

/// Optimized IAST (International Alphabet of Sanskrit Transliteration) to ISO-15919 converter
pub struct IASTConverter {
    iast_to_iso_map: &'static HashMap<&'static str, &'static str>,
    iso_to_iast_map: &'static HashMap<&'static str, &'static str>,
}

// Pre-computed static mappings
static IAST_TO_ISO_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // IAST and ISO-15919 are very similar, but have key differences:
    
    // Vowels - mostly identical
    map.insert("a", "a");
    map.insert("ā", "ā");
    map.insert("i", "i");
    map.insert("ī", "ī");
    map.insert("u", "u");
    map.insert("ū", "ū");
    map.insert("ṛ", "r̥");     // IAST ṛ → ISO r̥
    map.insert("ṝ", "r̥̄");    // IAST ṝ → ISO r̥̄
    map.insert("ḷ", "l̥");     // IAST ḷ → ISO l̥
    map.insert("ḹ", "l̥̄");    // IAST ḹ → ISO l̥̄
    map.insert("e", "e");
    map.insert("ai", "ai");
    map.insert("o", "o");
    map.insert("au", "au");
    
    // Consonants - mostly identical but some key differences
    map.insert("k", "k");
    map.insert("kh", "kh");
    map.insert("g", "g");
    map.insert("gh", "gh");
    map.insert("ṅ", "ṅ");
    map.insert("c", "c");
    map.insert("ch", "ch");
    map.insert("j", "j");
    map.insert("jh", "jh");
    map.insert("ñ", "ñ");
    map.insert("ṭ", "ṭ");
    map.insert("ṭh", "ṭh");
    map.insert("ḍ", "ḍ");
    map.insert("ḍh", "ḍh");
    map.insert("ṇ", "ṇ");
    map.insert("t", "t");
    map.insert("th", "th");
    map.insert("d", "d");
    map.insert("dh", "dh");
    map.insert("n", "n");
    map.insert("p", "p");
    map.insert("ph", "ph");
    map.insert("b", "b");
    map.insert("bh", "bh");
    map.insert("m", "m");
    map.insert("y", "y");
    map.insert("r", "r");
    map.insert("l", "l");
    map.insert("v", "v");
    map.insert("ś", "ś");
    map.insert("ṣ", "ṣ");
    map.insert("s", "s");
    map.insert("h", "h");
    
    // Special marks and punctuation
    map.insert("ṃ", "ṁ");      // IAST ṃ → ISO ṁ (anusvara)
    map.insert("ḥ", "ḥ");      // Same (visarga)
    map.insert("m̐", "m̐");     // Same (candrabindu)
    map.insert("'", "'");      // Avagraha
    
    // Special combinations
    map.insert("kṣ", "kṣ");
    map.insert("jñ", "jñ");
    
    // Punctuation
    map.insert(".", ".");
    map.insert(",", ",");
    map.insert(";", ";");
    map.insert(":", ":");
    map.insert("!", "!");
    map.insert("?", "?");
    
    // Numbers
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

static ISO_TO_IAST_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // Build reverse mapping
    for (&iast, &iso) in IAST_TO_ISO_MAP.iter() {
        map.insert(iso, iast);
    }
    
    // Override key differences
    map.insert("r̥", "ṛ");     // ISO r̥ → IAST ṛ
    map.insert("r̥̄", "ṝ");    // ISO r̥̄ → IAST ṝ
    map.insert("l̥", "ḷ");     // ISO l̥ → IAST ḷ
    map.insert("l̥̄", "ḹ");    // ISO l̥̄ → IAST ḹ
    map.insert("ṁ", "ṃ");     // ISO ṁ → IAST ṃ
    
    map
});

impl IASTConverter {
    pub fn new() -> Self {
        Self {
            iast_to_iso_map: &IAST_TO_ISO_MAP,
            iso_to_iast_map: &ISO_TO_IAST_MAP,
        }
    }
    
    /// Convert IAST to ISO-15919
    pub fn iast_to_iso(&self, input: &str) -> Result<String, ConverterError> {
        RomanScriptProcessor::process(input, self.iast_to_iso_map)
    }
    
    /// Convert ISO-15919 to IAST
    pub fn iso_to_iast(&self, input: &str) -> Result<String, ConverterError> {
        RomanScriptProcessor::process(input, self.iso_to_iast_map)
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
        
        let iso_text = self.iast_to_iso(input)?;
        Ok(HubInput::Iso(iso_text))
    }
    
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        if script != "iast" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "IAST converter only supports 'iast' script".to_string(),
            });
        }
        
        match hub_input {
            HubInput::Iso(iso_text) => self.iso_to_iast(iso_text),
            HubInput::Devanagari(_) => Err(ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: "IAST converter expects ISO input, got Devanagari".to_string(),
            }),
        }
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["iast"]
    }
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        false // IAST is a romanization scheme without implicit vowels
    }
}

impl Default for IASTConverter {
    fn default() -> Self {
        Self::new()
    }
}