use std::collections::HashMap;
use super::{ScriptConverter, ConverterError};
use crate::modules::hub::HubInput;

/// Grantha script converter  
/// 
/// Grantha is a classical South Indian script used for writing Sanskrit,
/// particularly in Tamil Nadu and Kerala. It's closely related to Tamil script
/// but includes additional characters for Sanskrit sounds not found in Tamil.
/// This converter uses the optimal Indic script structure.
pub struct GranthaConverter {
    grantha_to_deva_map: HashMap<char, char>,
    deva_to_grantha_map: HashMap<char, char>,
}

impl GranthaConverter {
    pub fn new() -> Self {
        let mut grantha_to_deva = HashMap::new();
        
        // Independent vowels → Devanagari equivalents
        grantha_to_deva.insert('𑌅', 'अ');       // Grantha 𑌅 → Devanagari अ
        grantha_to_deva.insert('𑌆', 'आ');       // Grantha 𑌆 → Devanagari आ
        grantha_to_deva.insert('𑌇', 'इ');       // Grantha 𑌇 → Devanagari इ
        grantha_to_deva.insert('𑌈', 'ई');       // Grantha 𑌈 → Devanagari ई
        grantha_to_deva.insert('𑌉', 'उ');       // Grantha 𑌉 → Devanagari उ
        grantha_to_deva.insert('𑌊', 'ऊ');       // Grantha 𑌊 → Devanagari ऊ
        grantha_to_deva.insert('𑌋', 'ऋ');       // Grantha 𑌋 → Devanagari ऋ
        grantha_to_deva.insert('𑌌', 'ऌ');       // Grantha 𑌌 → Devanagari ऌ
        grantha_to_deva.insert('𑌏', 'ए');       // Grantha 𑌏 → Devanagari ए
        grantha_to_deva.insert('𑌐', 'ऐ');       // Grantha 𑌐 → Devanagari ऐ
        grantha_to_deva.insert('𑌓', 'ओ');       // Grantha 𑌓 → Devanagari ओ
        grantha_to_deva.insert('𑌔', 'औ');       // Grantha 𑌔 → Devanagari औ
        
        // Vowel diacritics → Devanagari equivalents
        grantha_to_deva.insert('𑌾', 'ा');       // Grantha 𑌾 → Devanagari ा
        grantha_to_deva.insert('𑌿', 'ि');       // Grantha 𑌿 → Devanagari ि
        grantha_to_deva.insert('𑍀', 'ी');       // Grantha 𑍀 → Devanagari ी
        grantha_to_deva.insert('𑍁', 'ु');       // Grantha 𑍁 → Devanagari ु
        grantha_to_deva.insert('𑍂', 'ू');       // Grantha 𑍂 → Devanagari ू
        grantha_to_deva.insert('𑍃', 'ृ');       // Grantha 𑍃 → Devanagari ृ
        grantha_to_deva.insert('𑍄', 'ॄ');       // Grantha 𑍄 → Devanagari ॄ
        grantha_to_deva.insert('𑍇', 'े');       // Grantha 𑍇 → Devanagari े
        grantha_to_deva.insert('𑍈', 'ै');       // Grantha 𑍈 → Devanagari ै
        grantha_to_deva.insert('𑍋', 'ो');       // Grantha 𑍋 → Devanagari ो
        grantha_to_deva.insert('𑍌', 'ौ');       // Grantha 𑍌 → Devanagari ौ
        
        // Consonants → Devanagari equivalents
        // Velar consonants
        grantha_to_deva.insert('𑌕', 'क');       // Grantha 𑌕 → Devanagari क
        grantha_to_deva.insert('𑌖', 'ख');       // Grantha 𑌖 → Devanagari ख
        grantha_to_deva.insert('𑌗', 'ग');       // Grantha 𑌗 → Devanagari ग
        grantha_to_deva.insert('𑌘', 'घ');       // Grantha 𑌘 → Devanagari घ
        grantha_to_deva.insert('𑌙', 'ङ');       // Grantha 𑌙 → Devanagari ङ
        
        // Palatal consonants
        grantha_to_deva.insert('𑌚', 'च');       // Grantha 𑌚 → Devanagari च
        grantha_to_deva.insert('𑌛', 'छ');       // Grantha 𑌛 → Devanagari छ
        grantha_to_deva.insert('𑌜', 'ज');       // Grantha 𑌜 → Devanagari ज
        grantha_to_deva.insert('𑌝', 'झ');       // Grantha 𑌝 → Devanagari झ
        grantha_to_deva.insert('𑌞', 'ञ');       // Grantha 𑌞 → Devanagari ञ
        
        // Retroflex consonants
        grantha_to_deva.insert('𑌟', 'ट');       // Grantha 𑌟 → Devanagari ट
        grantha_to_deva.insert('𑌠', 'ठ');       // Grantha 𑌠 → Devanagari ठ
        grantha_to_deva.insert('𑌡', 'ड');       // Grantha 𑌡 → Devanagari ड
        grantha_to_deva.insert('𑌢', 'ढ');       // Grantha 𑌢 → Devanagari ढ
        grantha_to_deva.insert('𑌣', 'ण');       // Grantha 𑌣 → Devanagari ण
        
        // Dental consonants
        grantha_to_deva.insert('𑌤', 'त');       // Grantha 𑌤 → Devanagari त
        grantha_to_deva.insert('𑌥', 'थ');       // Grantha 𑌥 → Devanagari थ
        grantha_to_deva.insert('𑌦', 'द');       // Grantha 𑌦 → Devanagari द
        grantha_to_deva.insert('𑌧', 'ध');       // Grantha 𑌧 → Devanagari ध
        grantha_to_deva.insert('𑌨', 'न');       // Grantha 𑌨 → Devanagari न
        
        // Labial consonants
        grantha_to_deva.insert('𑌪', 'प');       // Grantha 𑌪 → Devanagari प
        grantha_to_deva.insert('𑌫', 'फ');       // Grantha 𑌫 → Devanagari फ
        grantha_to_deva.insert('𑌬', 'ब');       // Grantha 𑌬 → Devanagari ब
        grantha_to_deva.insert('𑌭', 'भ');       // Grantha 𑌭 → Devanagari भ
        grantha_to_deva.insert('𑌮', 'म');       // Grantha 𑌮 → Devanagari म
        
        // Semivowels and liquids
        grantha_to_deva.insert('𑌯', 'य');       // Grantha 𑌯 → Devanagari य
        grantha_to_deva.insert('𑌰', 'र');       // Grantha 𑌰 → Devanagari र
        grantha_to_deva.insert('𑌲', 'ल');       // Grantha 𑌲 → Devanagari ल
        grantha_to_deva.insert('𑌵', 'व');       // Grantha 𑌵 → Devanagari व
        grantha_to_deva.insert('𑌳', 'ळ');       // Grantha 𑌳 → Devanagari ळ
        
        // Sibilants and aspirate
        grantha_to_deva.insert('𑌶', 'श');       // Grantha 𑌶 → Devanagari श
        grantha_to_deva.insert('𑌷', 'ष');       // Grantha 𑌷 → Devanagari ष
        grantha_to_deva.insert('𑌸', 'स');       // Grantha 𑌸 → Devanagari स
        grantha_to_deva.insert('𑌹', 'ह');       // Grantha 𑌹 → Devanagari ह
        
        // Special marks
        grantha_to_deva.insert('𑌂', 'ं');       // Grantha 𑌂 → Devanagari ं (anusvara)
        grantha_to_deva.insert('𑌃', 'ः');       // Grantha 𑌃 → Devanagari ः (visarga)
        grantha_to_deva.insert('𑍍', '्');       // Grantha 𑍍 → Devanagari ् (virama)
        
        // Grantha-specific characters for Sanskrit
        grantha_to_deva.insert('𑌐', 'ऐ');       // Grantha ai
        grantha_to_deva.insert('𑌔', 'औ');       // Grantha au
        
        // Build reverse mapping
        let mut deva_to_grantha = HashMap::new();
        for (&grantha, &deva) in &grantha_to_deva {
            deva_to_grantha.insert(deva, grantha);
        }

        Self {
            grantha_to_deva_map: grantha_to_deva,
            deva_to_grantha_map: deva_to_grantha,
        }
    }
    
    /// Convert Grantha text to Devanagari format (for hub processing)
    pub fn grantha_to_devanagari(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        
        for ch in input.chars() {
            if ch.is_whitespace() || ch.is_ascii_punctuation() {
                result.push(ch);
            } else if let Some(&deva_char) = self.grantha_to_deva_map.get(&ch) {
                result.push(deva_char);
            } else {
                // Character not in mapping - preserve as-is for mixed content
                result.push(ch);
            }
        }
        
        Ok(result)
    }
    
    /// Convert Devanagari text to Grantha format (reverse conversion)
    pub fn devanagari_to_grantha(&self, input: &str) -> Result<String, ConverterError> {
        let mut result = String::new();
        
        for ch in input.chars() {
            if ch.is_whitespace() || ch.is_ascii_punctuation() {
                result.push(ch);
            } else if let Some(&grantha_char) = self.deva_to_grantha_map.get(&ch) {
                result.push(grantha_char);
            } else {
                // Character not in mapping - preserve as-is for mixed content
                result.push(ch);
            }
        }
        
        Ok(result)
    }
}

impl ScriptConverter for GranthaConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "grantha" && script != "gran" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Grantha converter only supports 'grantha' or 'gran' script".to_string(),
            });
        }
        
        let deva_text = self.grantha_to_devanagari(input)
            .map_err(|e| ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: format!("Grantha to Devanagari conversion failed: {}", e),
            })?;
            
        Ok(HubInput::Devanagari(deva_text))
    }
    
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        if script != "grantha" && script != "gran" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Grantha converter only supports 'grantha' or 'gran' script".to_string(),
            });
        }
        
        match hub_input {
            HubInput::Devanagari(deva_text) => self.devanagari_to_grantha(deva_text),
            HubInput::Iso(_) => Err(ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: "Grantha converter expects Devanagari input, got ISO".to_string(),
            }),
        }
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["grantha", "gran"]
    }
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        // Grantha is an Indic script - consonants DO have implicit 'a'
        true
    }
}

impl Default for GranthaConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_grantha_basic_vowels() {
        let converter = GranthaConverter::new();
        
        assert_eq!(converter.grantha_to_devanagari("𑌅").unwrap(), "अ");
        assert_eq!(converter.grantha_to_devanagari("𑌆").unwrap(), "आ");
        assert_eq!(converter.grantha_to_devanagari("𑌇").unwrap(), "इ");
        assert_eq!(converter.grantha_to_devanagari("𑌈").unwrap(), "ई");
    }
    
    #[test]
    fn test_grantha_consonants() {
        let converter = GranthaConverter::new();
        
        assert_eq!(converter.grantha_to_devanagari("𑌕").unwrap(), "क");
        assert_eq!(converter.grantha_to_devanagari("𑌖").unwrap(), "ख");
        assert_eq!(converter.grantha_to_devanagari("𑌗").unwrap(), "ग");
        assert_eq!(converter.grantha_to_devanagari("𑌘").unwrap(), "घ");
    }
    
    #[test]
    fn test_script_converter_interface() {
        let converter = GranthaConverter::new();
        
        assert!(converter.supports_script("grantha"));
        assert!(converter.supports_script("gran"));
        assert!(!converter.supports_script("tamil"));
        
        assert!(converter.script_has_implicit_a("grantha"));
        
        let result = converter.to_hub("grantha", "𑌕").unwrap();
        if let HubInput::Devanagari(deva_text) = result {
            assert_eq!(deva_text, "क");
        } else {
            panic!("Expected Devanagari hub input");
        }
    }
}