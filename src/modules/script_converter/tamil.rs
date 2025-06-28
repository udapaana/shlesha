use std::collections::HashMap;
use super::{ScriptConverter, ConverterError};
use crate::modules::hub::HubInput;

/// Tamil script converter with Sanskrit superscript support
/// 
/// Tamil (தமிழ்) traditionally has a limited consonant set compared to Sanskrit.
/// This converter uses superscript notation to represent the full Sanskrit
/// phonetic inventory in Tamil script, following scholarly conventions.
/// 
/// Example: ध (dha) → த⁴ (ta with superscript 4 for aspiration)
/// Example: भ (bha) → ப⁴ (pa with superscript 4 for voiced aspiration)
pub struct TamilConverter {
    tamil_to_deva_map: HashMap<String, String>,
    deva_to_tamil_map: HashMap<String, String>,
}

impl TamilConverter {
    pub fn new() -> Self {
        let mut tamil_to_deva = HashMap::new();
        let mut deva_to_tamil = HashMap::new();
        
        // === BASIC VOWELS (no changes needed) ===
        tamil_to_deva.insert("அ".to_string(), "अ".to_string());
        tamil_to_deva.insert("ஆ".to_string(), "आ".to_string()); 
        tamil_to_deva.insert("இ".to_string(), "इ".to_string());
        tamil_to_deva.insert("ஈ".to_string(), "ई".to_string());
        tamil_to_deva.insert("உ".to_string(), "उ".to_string());
        tamil_to_deva.insert("ஊ".to_string(), "ऊ".to_string());
        tamil_to_deva.insert("எ".to_string(), "ए".to_string());
        tamil_to_deva.insert("ஏ".to_string(), "ए".to_string());
        tamil_to_deva.insert("ஐ".to_string(), "ऐ".to_string());
        tamil_to_deva.insert("ஒ".to_string(), "ओ".to_string());
        tamil_to_deva.insert("ஓ".to_string(), "ओ".to_string());
        tamil_to_deva.insert("ஔ".to_string(), "औ".to_string());
        
        // === CONSONANTS WITH SANSKRIT SUPERSCRIPT EXTENSIONS ===
        
        // Velar series: க (ka) base
        tamil_to_deva.insert("க".to_string(), "क".to_string());    // ka (unaspirated voiceless)
        tamil_to_deva.insert("க²".to_string(), "ख".to_string());   // kha (aspirated voiceless) 
        tamil_to_deva.insert("க³".to_string(), "ग".to_string());   // ga (unaspirated voiced)
        tamil_to_deva.insert("க⁴".to_string(), "घ".to_string());   // gha (aspirated voiced)
        tamil_to_deva.insert("ங".to_string(), "ङ".to_string());    // ṅa (nasal)
        
        // Palatal series: ச (ca) base  
        tamil_to_deva.insert("ச".to_string(), "च".to_string());    // ca (unaspirated voiceless)
        tamil_to_deva.insert("ச²".to_string(), "छ".to_string());   // cha (aspirated voiceless)
        tamil_to_deva.insert("ச³".to_string(), "ज".to_string());   // ja (unaspirated voiced) 
        tamil_to_deva.insert("ச⁴".to_string(), "झ".to_string());   // jha (aspirated voiced)
        tamil_to_deva.insert("ஞ".to_string(), "ञ".to_string());    // ña (nasal)
        
        // Retroflex series: ட (ṭa) base
        tamil_to_deva.insert("ட".to_string(), "ट".to_string());    // ṭa (unaspirated voiceless)
        tamil_to_deva.insert("ட²".to_string(), "ठ".to_string());   // ṭha (aspirated voiceless)
        tamil_to_deva.insert("ட³".to_string(), "ड".to_string());   // ḍa (unaspirated voiced)
        tamil_to_deva.insert("ட⁴".to_string(), "ढ".to_string());   // ḍha (aspirated voiced)
        tamil_to_deva.insert("ண".to_string(), "ण".to_string());    // ṇa (nasal)
        
        // Dental series: த (ta) base
        tamil_to_deva.insert("த".to_string(), "त".to_string());    // ta (unaspirated voiceless)
        tamil_to_deva.insert("த²".to_string(), "थ".to_string());   // tha (aspirated voiceless)
        tamil_to_deva.insert("த³".to_string(), "द".to_string());   // da (unaspirated voiced)
        tamil_to_deva.insert("த⁴".to_string(), "ध".to_string());   // dha (aspirated voiced)
        tamil_to_deva.insert("ந".to_string(), "न".to_string());    // na (nasal)
        
        // Labial series: ப (pa) base
        tamil_to_deva.insert("ப".to_string(), "प".to_string());    // pa (unaspirated voiceless)
        tamil_to_deva.insert("ப²".to_string(), "फ".to_string());   // pha (aspirated voiceless)
        tamil_to_deva.insert("ப³".to_string(), "ब".to_string());   // ba (unaspirated voiced)
        tamil_to_deva.insert("ப⁴".to_string(), "भ".to_string());   // bha (aspirated voiced)
        tamil_to_deva.insert("ம".to_string(), "म".to_string());    // ma (nasal)
        
        // Semivowels (mostly direct mappings)
        tamil_to_deva.insert("ய".to_string(), "य".to_string());    // ya
        tamil_to_deva.insert("ர".to_string(), "र".to_string());    // ra
        tamil_to_deva.insert("ல".to_string(), "ल".to_string());    // la
        tamil_to_deva.insert("வ".to_string(), "व".to_string());    // va
        
        // Sibilants: use ஸ (sa) as base with superscripts
        tamil_to_deva.insert("ஸ".to_string(), "स".to_string());    // sa (dental sibilant)
        tamil_to_deva.insert("ஸ²".to_string(), "श".to_string());   // śa (palatal sibilant)
        tamil_to_deva.insert("ஸ³".to_string(), "ष".to_string());   // ṣa (retroflex sibilant)
        tamil_to_deva.insert("ஹ".to_string(), "ह".to_string());    // ha (aspirate)
        
        // Additional Sanskrit sounds
        tamil_to_deva.insert("ர்²".to_string(), "ऋ".to_string());  // r̥ (vocalic r)
        tamil_to_deva.insert("ர்³".to_string(), "ॠ".to_string());  // r̥̄ (long vocalic r)
        tamil_to_deva.insert("ல்²".to_string(), "ऌ".to_string());  // l̥ (vocalic l)
        tamil_to_deva.insert("ல்³".to_string(), "ॡ".to_string());  // l̥̄ (long vocalic l)
        
        // Tamil-specific sounds (unchanged)
        tamil_to_deva.insert("ழ".to_string(), "ळ".to_string());    // ḻa (Tamil retroflex l)
        tamil_to_deva.insert("ள".to_string(), "ळ".to_string());    // ḷa (Tamil lateral)
        tamil_to_deva.insert("ற".to_string(), "र".to_string());    // ṟa (Tamil alveolar r)
        tamil_to_deva.insert("ன".to_string(), "न".to_string());    // ṉa (Tamil alveolar n)
        
        // Vowel diacritics (unchanged)
        tamil_to_deva.insert("ா".to_string(), "ा".to_string());
        tamil_to_deva.insert("ி".to_string(), "ि".to_string());
        tamil_to_deva.insert("ீ".to_string(), "ी".to_string());
        tamil_to_deva.insert("ு".to_string(), "ु".to_string());
        tamil_to_deva.insert("ூ".to_string(), "ू".to_string());
        tamil_to_deva.insert("ெ".to_string(), "े".to_string());
        tamil_to_deva.insert("ே".to_string(), "े".to_string());
        tamil_to_deva.insert("ை".to_string(), "ै".to_string());
        tamil_to_deva.insert("ொ".to_string(), "ो".to_string());
        tamil_to_deva.insert("ோ".to_string(), "ो".to_string());
        tamil_to_deva.insert("ௌ".to_string(), "ौ".to_string());
        
        // Special marks
        tamil_to_deva.insert("ஂ".to_string(), "ं".to_string());    // anusvara
        tamil_to_deva.insert("ஃ".to_string(), "ः".to_string());    // visarga
        tamil_to_deva.insert("்".to_string(), "्".to_string());    // virama/pulli
        
        // Build reverse mapping (Devanagari → Tamil with superscripts)
        for (tamil, deva) in &tamil_to_deva {
            deva_to_tamil.insert(deva.clone(), tamil.clone());
        }

        Self {
            tamil_to_deva_map: tamil_to_deva,
            deva_to_tamil_map: deva_to_tamil,
        }
    }
    
    /// Convert Tamil text (with superscripts) to Devanagari
    pub fn tamil_to_devanagari(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        let mut chars = input.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch.is_whitespace() || ch.is_ascii_punctuation() {
                result.push(ch);
                continue;
            }
            
            // Check for superscript combinations (base + superscript)
            if let Some(&next_ch) = chars.peek() {
                if is_superscript(next_ch) {
                    let combined = format!("{}{}", ch, next_ch);
                    if let Some(mapped) = self.tamil_to_deva_map.get(&combined) {
                        result.push_str(mapped);
                        chars.next(); // consume the superscript
                        continue;
                    }
                }
            }
            
            // Single character lookup
            let single_char = ch.to_string();
            if let Some(mapped) = self.tamil_to_deva_map.get(&single_char) {
                result.push_str(mapped);
            } else {
                // Preserve unmapped characters
                result.push(ch);
            }
        }
        
        Ok(result)
    }
    
    /// Convert Devanagari text to Tamil (with superscripts)
    pub fn devanagari_to_tamil(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        
        for ch in input.chars() {
            if ch.is_whitespace() || ch.is_ascii_punctuation() {
                result.push(ch);
            } else {
                let single_char = ch.to_string();
                if let Some(mapped) = self.deva_to_tamil_map.get(&single_char) {
                    result.push_str(mapped);
                } else {
                    // Preserve unmapped characters
                    result.push(ch);
                }
            }
        }
        
        Ok(result)
    }
}

/// Check if a character is a superscript used for Sanskrit notation
fn is_superscript(ch: char) -> bool {
    matches!(ch, '²' | '³' | '⁴' | '⁵' | '⁶' | '⁷' | '⁸' | '⁹')
}

impl ScriptConverter for TamilConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "tamil" && script != "ta" && script != "tamil-extended" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Enhanced Tamil converter supports 'tamil', 'ta', or 'tamil-extended' script".to_string(),
            });
        }
        
        let deva_text = self.tamil_to_devanagari(input)
            .map_err(|e| ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: format!("Tamil to Devanagari conversion failed: {}", e),
            })?;
            
        Ok(HubInput::Devanagari(deva_text))
    }
    
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        if script != "tamil" && script != "ta" && script != "tamil-extended" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Enhanced Tamil converter supports 'tamil', 'ta', or 'tamil-extended' script".to_string(),
            });
        }
        
        match hub_input {
            HubInput::Devanagari(deva_text) => self.devanagari_to_tamil(deva_text),
            HubInput::Iso(_) => Err(ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: "Enhanced Tamil converter expects Devanagari input, got ISO".to_string(),
            }),
        }
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["tamil", "ta", "tamil-extended"]
    }
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        // Tamil is an Indic script - consonants DO have implicit 'a'
        true
    }
}

impl Default for TamilConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_tamil_consonants() {
        let converter = TamilConverter::new();
        
        // Basic unaspirated consonants
        assert_eq!(converter.tamil_to_devanagari("க").unwrap(), "क");
        assert_eq!(converter.tamil_to_devanagari("ச").unwrap(), "च");
        assert_eq!(converter.tamil_to_devanagari("ட").unwrap(), "ट");
        assert_eq!(converter.tamil_to_devanagari("த").unwrap(), "त");
        assert_eq!(converter.tamil_to_devanagari("ப").unwrap(), "प");
    }
    
    #[test]
    fn test_sanskrit_superscript_aspirated() {
        let converter = TamilConverter::new();
        
        // Aspirated consonants with superscripts
        assert_eq!(converter.tamil_to_devanagari("க²").unwrap(), "ख");  // kha
        assert_eq!(converter.tamil_to_devanagari("த⁴").unwrap(), "ध");  // dha
        assert_eq!(converter.tamil_to_devanagari("ப³").unwrap(), "ब");  // bha
    }
    
    #[test]
    fn test_sanskrit_sibilants() {
        let converter = TamilConverter::new();
        
        // Sibilant variations
        assert_eq!(converter.tamil_to_devanagari("ஸ").unwrap(), "स");   // sa
        assert_eq!(converter.tamil_to_devanagari("ஸ²").unwrap(), "श");  // śa
        assert_eq!(converter.tamil_to_devanagari("ஸ³").unwrap(), "ष");  // ṣa
    }
    
    #[test]
    fn test_reverse_conversion() {
        let converter = TamilConverter::new();
        
        // Test Devanagari → Tamil with superscripts
        assert_eq!(converter.devanagari_to_tamil("ध").unwrap(), "த⁴");  // dha → த⁴
        assert_eq!(converter.devanagari_to_tamil("भ").unwrap(), "ப⁴");  // bha → ப⁴  
        assert_eq!(converter.devanagari_to_tamil("ख").unwrap(), "க²");  // kha → க²
    }
    
    #[test]
    fn test_dharma_example() {
        let converter = TamilConverter::new();
        
        // Test the example: dharma should become த⁴ர்ம
        // Note: this requires proper handling of vowel suppression
        let result = converter.devanagari_to_tamil("धर्म").unwrap();
        // The exact result depends on how virama is handled, but should contain த⁴
        assert!(result.contains("த⁴"));
    }
    
    #[test]
    fn test_script_converter_interface() {
        let converter = TamilConverter::new();
        
        assert!(converter.supports_script("tamil"));
        assert!(converter.supports_script("ta"));
        assert!(converter.supports_script("tamil-extended"));
        assert!(!converter.supports_script("hindi"));
        
        assert!(converter.script_has_implicit_a("tamil"));
        
        let result = converter.to_hub("tamil", "க²").unwrap();
        if let HubInput::Devanagari(deva_text) = result {
            assert_eq!(deva_text, "ख");
        } else {
            panic!("Expected Devanagari hub input");
        }
    }
}