use std::collections::HashMap;
use super::{ScriptConverter, ConverterError};
use crate::modules::hub::HubInput;

/// Bengali script converter
/// 
/// Bengali (বাংলা) is an Indic script used primarily for the Bengali language.
/// This converter handles Bengali text by first converting it to ISO-15919 format,
/// which can then be processed by the hub for cross-script conversion.
pub struct BengaliConverter {
    bengali_to_iso_map: HashMap<char, &'static str>,
    bengali_nukta_to_iso_map: HashMap<&'static str, &'static str>,
}

impl BengaliConverter {
    pub fn new() -> Self {
        let mut bengali_to_iso = HashMap::new();
        
        // Bengali has consonants with implicit 'a' vowel, similar to Devanagari
        // but with different Unicode codepoints and some script-specific variations
        
        // Vowels (স্বরবর্ণ)
        bengali_to_iso.insert('অ', "a");       // Bengali অ → ISO a
        bengali_to_iso.insert('আ', "ā");       // Bengali আ → ISO ā
        bengali_to_iso.insert('ই', "i");       // Bengali ই → ISO i
        bengali_to_iso.insert('ঈ', "ī");       // Bengali ঈ → ISO ī
        bengali_to_iso.insert('উ', "u");       // Bengali উ → ISO u
        bengali_to_iso.insert('ঊ', "ū");       // Bengali ঊ → ISO ū
        bengali_to_iso.insert('ঋ', "r̥");       // Bengali ঋ → ISO r̥
        bengali_to_iso.insert('ৠ', "r̥̄");      // Bengali ৠ → ISO r̥̄
        bengali_to_iso.insert('ঌ', "l̥");       // Bengali ঌ → ISO l̥
        bengali_to_iso.insert('ৡ', "l̥̄");      // Bengali ৡ → ISO l̥̄
        bengali_to_iso.insert('এ', "e");       // Bengali এ → ISO e
        bengali_to_iso.insert('ঐ', "ai");      // Bengali ঐ → ISO ai
        bengali_to_iso.insert('ও', "o");       // Bengali ও → ISO o
        bengali_to_iso.insert('ঔ', "au");      // Bengali ঔ → ISO au
        
        // Vowel diacritics (মাত্রা)
        bengali_to_iso.insert('া', "ā");       // Bengali া → ISO ā
        bengali_to_iso.insert('ি', "i");       // Bengali ি → ISO i
        bengali_to_iso.insert('ী', "ī");       // Bengali ী → ISO ī
        bengali_to_iso.insert('ু', "u");       // Bengali ু → ISO u
        bengali_to_iso.insert('ূ', "ū");       // Bengali ূ → ISO ū
        bengali_to_iso.insert('ৃ', "r̥");       // Bengali ৃ → ISO r̥
        bengali_to_iso.insert('ৄ', "r̥̄");      // Bengali ৄ → ISO r̥̄
        bengali_to_iso.insert('ৢ', "l̥");       // Bengali ৢ → ISO l̥
        bengali_to_iso.insert('ৣ', "l̥̄");      // Bengali ৣ → ISO l̥̄
        bengali_to_iso.insert('ে', "e");       // Bengali ে → ISO e
        bengali_to_iso.insert('ৈ', "ai");      // Bengali ৈ → ISO ai
        bengali_to_iso.insert('ো', "o");       // Bengali ো → ISO o
        bengali_to_iso.insert('ৌ', "au");      // Bengali ৌ → ISO au
        
        // Consonants (ব্যঞ্জনবর্ণ) - Bengali consonants have implicit 'a'
        // Velar consonants
        bengali_to_iso.insert('ক', "ka");      // Bengali ক → ISO ka
        bengali_to_iso.insert('খ', "kha");     // Bengali খ → ISO kha
        bengali_to_iso.insert('গ', "ga");      // Bengali গ → ISO ga
        bengali_to_iso.insert('ঘ', "gha");     // Bengali ঘ → ISO gha
        bengali_to_iso.insert('ঙ', "ṅa");      // Bengali ঙ → ISO ṅa
        
        // Palatal consonants
        bengali_to_iso.insert('চ', "ca");      // Bengali চ → ISO ca
        bengali_to_iso.insert('ছ', "cha");     // Bengali ছ → ISO cha
        bengali_to_iso.insert('জ', "ja");      // Bengali জ → ISO ja
        bengali_to_iso.insert('ঝ', "jha");     // Bengali ঝ → ISO jha
        bengali_to_iso.insert('ঞ', "ña");      // Bengali ঞ → ISO ña
        
        // Retroflex consonants
        bengali_to_iso.insert('ট', "ṭa");      // Bengali ট → ISO ṭa
        bengali_to_iso.insert('ঠ', "ṭha");     // Bengali ঠ → ISO ṭha
        bengali_to_iso.insert('ড', "ḍa");      // Bengali ড → ISO ḍa
        bengali_to_iso.insert('ঢ', "ḍha");     // Bengali ঢ → ISO ḍha
        bengali_to_iso.insert('ণ', "ṇa");      // Bengali ণ → ISO ṇa
        
        // Dental consonants
        bengali_to_iso.insert('ত', "ta");      // Bengali ত → ISO ta
        bengali_to_iso.insert('থ', "tha");     // Bengali থ → ISO tha
        bengali_to_iso.insert('দ', "da");      // Bengali দ → ISO da
        bengali_to_iso.insert('ধ', "dha");     // Bengali ধ → ISO dha
        bengali_to_iso.insert('ন', "na");      // Bengali ন → ISO na
        
        // Labial consonants
        bengali_to_iso.insert('প', "pa");      // Bengali প → ISO pa
        bengali_to_iso.insert('ফ', "pha");     // Bengali ফ → ISO pha
        bengali_to_iso.insert('ব', "ba");      // Bengali ব → ISO ba
        bengali_to_iso.insert('ভ', "bha");     // Bengali ভ → ISO bha
        bengali_to_iso.insert('ম', "ma");      // Bengali ম → ISO ma
        
        // Semivowels and liquids
        bengali_to_iso.insert('য', "ya");      // Bengali য → ISO ya
        bengali_to_iso.insert('র', "ra");      // Bengali র → ISO ra
        bengali_to_iso.insert('ল', "la");      // Bengali ল → ISO la
        bengali_to_iso.insert('ব', "va");      // Bengali ব can also be va (context dependent)
        
        // Sibilants and aspirate
        bengali_to_iso.insert('শ', "śa");      // Bengali শ → ISO śa
        bengali_to_iso.insert('ষ', "ṣa");      // Bengali ষ → ISO ṣa
        bengali_to_iso.insert('স', "sa");      // Bengali স → ISO sa
        bengali_to_iso.insert('হ', "ha");      // Bengali হ → ISO ha
        
        // Additional consonants (using nukta map for composite characters)
        let mut bengali_nukta_to_iso = HashMap::new();
        bengali_nukta_to_iso.insert("ড়", "ṛa");      // Bengali ড় → ISO ṛa (flapped)
        bengali_nukta_to_iso.insert("ঢ়", "ṛha");     // Bengali ঢ় → ISO ṛha (flapped aspirated)
        bengali_nukta_to_iso.insert("য়", "ẏa");      // Bengali য় → ISO ẏa (antahstha ya)
        
        // Special marks
        bengali_to_iso.insert('ং', "ṁ");       // Bengali ং → ISO ṁ (anusvara)
        bengali_to_iso.insert('ঃ', "ḥ");       // Bengali ঃ → ISO ḥ (visarga)
        bengali_to_iso.insert('্', "");        // Bengali ্ → hasanta/virama (removes inherent vowel)
        bengali_to_iso.insert('ঁ', "m̐");       // Bengali ঁ → ISO m̐ (candrabindu)
        bengali_to_iso.insert('ঽ', "'");       // Bengali ঽ → ISO ' (avagraha)
        
        // Digits
        bengali_to_iso.insert('০', "0");       // Bengali ০ → ISO 0
        bengali_to_iso.insert('১', "1");       // Bengali ১ → ISO 1
        bengali_to_iso.insert('২', "2");       // Bengali ২ → ISO 2
        bengali_to_iso.insert('৩', "3");       // Bengali ৩ → ISO 3
        bengali_to_iso.insert('৪', "4");       // Bengali ৪ → ISO 4
        bengali_to_iso.insert('৫', "5");       // Bengali ৫ → ISO 5
        bengali_to_iso.insert('৬', "6");       // Bengali ৬ → ISO 6
        bengali_to_iso.insert('৭', "7");       // Bengali ৭ → ISO 7
        bengali_to_iso.insert('৮', "8");       // Bengali ৮ → ISO 8
        bengali_to_iso.insert('৯', "9");       // Bengali ৯ → ISO 9
        
        // Punctuation
        bengali_to_iso.insert('।', "।");       // Bengali । → danda (same in ISO)
        bengali_to_iso.insert('॥', "॥");       // Bengali ॥ → double danda (same in ISO)
        
        Self {
            bengali_to_iso_map: bengali_to_iso,
            bengali_nukta_to_iso_map: bengali_nukta_to_iso,
        }
    }
    
    /// Convert Bengali text to ISO-15919 format
    pub fn bengali_to_iso(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            let ch = chars[i];
            
            if ch.is_whitespace() || ch.is_ascii_punctuation() {
                result.push(ch);
                i += 1;
                continue;
            }
            
            // Check for nukta characters (2-char sequences)
            if i + 1 < chars.len() {
                let two_char: String = chars[i..i+2].iter().collect();
                if let Some(&iso_str) = self.bengali_nukta_to_iso_map.get(two_char.as_str()) {
                    result.push_str(iso_str);
                    i += 2;
                    continue;
                }
            }
            
            // Check single character mapping
            if let Some(&iso_str) = self.bengali_to_iso_map.get(&ch) {
                result.push_str(iso_str);
            } else {
                // Character not in mapping - preserve as-is for mixed content
                result.push(ch);
            }
            i += 1;
        }
        
        Ok(result)
    }
}

impl ScriptConverter for BengaliConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "bengali" && script != "bengla" && script != "bn" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Bengali converter only supports 'bengali', 'bengla', or 'bn' script".to_string(),
            });
        }
        
        let iso_text = self.bengali_to_iso(input)
            .map_err(|e| ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: format!("Bengali to ISO conversion failed: {}", e),
            })?;
            
        Ok(HubInput::Iso(iso_text))
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["bengali", "bengla", "bn"]
    }
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        // Bengali is an Indic script - consonants DO have implicit 'a'
        // In Bengali, ক inherently represents "ka" and requires hasanta (্) to suppress the vowel: ক্
        true
    }
}

impl Default for BengaliConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bengali_basic_vowels() {
        let converter = BengaliConverter::new();
        
        // Test basic vowels
        assert_eq!(converter.bengali_to_iso("অ").unwrap(), "a");
        assert_eq!(converter.bengali_to_iso("আ").unwrap(), "ā");
        assert_eq!(converter.bengali_to_iso("ই").unwrap(), "i");
        assert_eq!(converter.bengali_to_iso("ঈ").unwrap(), "ī");
        assert_eq!(converter.bengali_to_iso("উ").unwrap(), "u");
        assert_eq!(converter.bengali_to_iso("ঊ").unwrap(), "ū");
        assert_eq!(converter.bengali_to_iso("এ").unwrap(), "e");
        assert_eq!(converter.bengali_to_iso("ঐ").unwrap(), "ai");
        assert_eq!(converter.bengali_to_iso("ও").unwrap(), "o");
        assert_eq!(converter.bengali_to_iso("ঔ").unwrap(), "au");
    }
    
    #[test]
    fn test_bengali_vocalic_vowels() {
        let converter = BengaliConverter::new();
        
        // Test Bengali → ISO vocalic vowels
        assert_eq!(converter.bengali_to_iso("ঋ").unwrap(), "r̥");    // Bengali ঋ → ISO r̥
        assert_eq!(converter.bengali_to_iso("ৠ").unwrap(), "r̥̄");   // Bengali ৠ → ISO r̥̄
        assert_eq!(converter.bengali_to_iso("ঌ").unwrap(), "l̥");    // Bengali ঌ → ISO l̥
        assert_eq!(converter.bengali_to_iso("ৡ").unwrap(), "l̥̄");   // Bengali ৡ → ISO l̥̄
    }
    
    #[test]
    fn test_bengali_consonants() {
        let converter = BengaliConverter::new();
        
        // Test basic consonants (with implicit 'a')
        assert_eq!(converter.bengali_to_iso("ক").unwrap(), "ka");
        assert_eq!(converter.bengali_to_iso("খ").unwrap(), "kha");
        assert_eq!(converter.bengali_to_iso("গ").unwrap(), "ga");
        assert_eq!(converter.bengali_to_iso("ঘ").unwrap(), "gha");
        assert_eq!(converter.bengali_to_iso("ঙ").unwrap(), "ṅa");
        
        // Test retroflex consonants
        assert_eq!(converter.bengali_to_iso("ট").unwrap(), "ṭa");
        assert_eq!(converter.bengali_to_iso("ঠ").unwrap(), "ṭha");
        assert_eq!(converter.bengali_to_iso("ড").unwrap(), "ḍa");
        assert_eq!(converter.bengali_to_iso("ঢ").unwrap(), "ḍha");
        assert_eq!(converter.bengali_to_iso("ণ").unwrap(), "ṇa");
        
        // Test sibilants
        assert_eq!(converter.bengali_to_iso("শ").unwrap(), "śa");   // Bengali শ → ISO śa
        assert_eq!(converter.bengali_to_iso("ষ").unwrap(), "ṣa");   // Bengali ষ → ISO ṣa
        assert_eq!(converter.bengali_to_iso("স").unwrap(), "sa");
    }
    
    #[test]
    fn test_bengali_special_marks() {
        let converter = BengaliConverter::new();
        
        // Test special marks
        assert_eq!(converter.bengali_to_iso("ং").unwrap(), "ṁ");
        assert_eq!(converter.bengali_to_iso("ঃ").unwrap(), "ḥ");
        assert_eq!(converter.bengali_to_iso("্").unwrap(), "");     // virama
        assert_eq!(converter.bengali_to_iso("ঁ").unwrap(), "m̐");
    }
    
    #[test]
    fn test_bengali_digits() {
        let converter = BengaliConverter::new();
        
        // Test Bengali digits
        assert_eq!(converter.bengali_to_iso("০১২৩৪৫৬৭৮৯").unwrap(), "0123456789");
    }
    
    #[test]
    fn test_bengali_complex_text() {
        let converter = BengaliConverter::new();
        
        // Test Bengali words
        let bengali_text = "নমস্কার";  // "namaskara" (greeting)
        
        // Just test that conversion works without error
        let result = converter.bengali_to_iso(bengali_text);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_script_converter_interface() {
        let converter = BengaliConverter::new();
        
        // Test the ScriptConverter interface
        assert!(converter.supports_script("bengali"));
        assert!(converter.supports_script("bengla"));
        assert!(converter.supports_script("bn"));
        assert!(!converter.supports_script("hindi"));
        
        // Test script_has_implicit_a
        assert!(converter.script_has_implicit_a("bengali"));
        assert!(converter.script_has_implicit_a("bn"));
        
        let result = converter.to_hub("bengali", "ক").unwrap();
        if let HubInput::Iso(iso_text) = result {
            assert_eq!(iso_text, "ka");
        } else {
            panic!("Expected ISO hub input");
        }
    }
    
    #[test]
    fn test_invalid_script_error() {
        let converter = BengaliConverter::new();
        
        // Should reject invalid script names
        let result = converter.to_hub("hindi", "test");
        assert!(result.is_err());
        
        if let Err(ConverterError::InvalidInput { script, message }) = result {
            assert_eq!(script, "hindi");
            assert!(message.contains("Bengali converter only supports"));
        } else {
            panic!("Expected InvalidInput error");
        }
    }
    
    #[test]
    fn test_mixed_content() {
        let converter = BengaliConverter::new();
        
        // Should handle mixed Bengali and other characters
        let mixed_input = "নমস্কার 123 hello";
        let result = converter.bengali_to_iso(mixed_input).unwrap();
        
        // Should contain the converted Bengali part plus preserved non-Bengali content
        assert!(result.contains("123"));
        assert!(result.contains("hello"));
    }
}