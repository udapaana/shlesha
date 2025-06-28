use std::collections::HashMap;
use super::{ScriptConverter, ConverterError};
use crate::modules::hub::HubInput;

/// Optimized Telugu script converter with reduced string allocations
/// 
/// This version uses the same logic as the original but with optimized string building
pub struct OptimizedTeluguConverter {
    telugu_to_deva_map: HashMap<char, char>,
    deva_to_telugu_map: HashMap<char, char>,
}

impl OptimizedTeluguConverter {
    pub fn new() -> Self {
        let mut telugu_to_deva = HashMap::new();
        
        // Same mappings as original Telugu converter but optimized construction
        
        // Independent vowels (అచ్చులు) → Devanagari equivalents
        telugu_to_deva.insert('అ', 'अ');       // Telugu అ → Devanagari अ
        telugu_to_deva.insert('ఆ', 'आ');       // Telugu ఆ → Devanagari आ
        telugu_to_deva.insert('ఇ', 'इ');       // Telugu ఇ → Devanagari इ
        telugu_to_deva.insert('ఈ', 'ई');       // Telugu ఈ → Devanagari ई
        telugu_to_deva.insert('ఉ', 'उ');       // Telugu ఉ → Devanagari उ
        telugu_to_deva.insert('ఊ', 'ऊ');       // Telugu ఊ → Devanagari ऊ
        telugu_to_deva.insert('ఋ', 'ऋ');       // Telugu ఋ → Devanagari ऋ
        telugu_to_deva.insert('ౠ', 'ॠ');       // Telugu ౠ → Devanagari ॠ
        telugu_to_deva.insert('ఌ', 'ऌ');       // Telugu ఌ → Devanagari ऌ
        telugu_to_deva.insert('ౡ', 'ॡ');       // Telugu ౡ → Devanagari ॡ
        telugu_to_deva.insert('ఎ', 'ए');       // Telugu ఎ → Devanagari ए
        telugu_to_deva.insert('ఏ', 'ए');       // Telugu ఏ → Devanagari ए (map to same)
        telugu_to_deva.insert('ఐ', 'ऐ');       // Telugu ఐ → Devanagari ऐ
        telugu_to_deva.insert('ఒ', 'ओ');       // Telugu ఒ → Devanagari ओ
        telugu_to_deva.insert('ఓ', 'ओ');       // Telugu ఓ → Devanagari ओ (map to same)
        telugu_to_deva.insert('ఔ', 'औ');       // Telugu ఔ → Devanagari औ
        
        // Vowel diacritics (పిల్లలు) → Devanagari equivalents
        telugu_to_deva.insert('ా', 'ा');       // Telugu ా → Devanagari ा
        telugu_to_deva.insert('ి', 'ि');       // Telugu ి → Devanagari ि
        telugu_to_deva.insert('ీ', 'ी');       // Telugu ీ → Devanagari ी
        telugu_to_deva.insert('ు', 'ु');       // Telugu ు → Devanagari ु
        telugu_to_deva.insert('ూ', 'ू');       // Telugu ూ → Devanagari ू
        telugu_to_deva.insert('ృ', 'ृ');       // Telugu ృ → Devanagari ृ
        telugu_to_deva.insert('ౄ', 'ॄ');       // Telugu ౄ → Devanagari ॄ
        telugu_to_deva.insert('ె', 'े');       // Telugu ె → Devanagari े
        telugu_to_deva.insert('ే', 'े');       // Telugu ే → Devanagari े
        telugu_to_deva.insert('ై', 'ै');       // Telugu ై → Devanagari ै
        telugu_to_deva.insert('ొ', 'ो');       // Telugu ొ → Devanagari ो
        telugu_to_deva.insert('ో', 'ो');       // Telugu ో → Devanagari ो
        telugu_to_deva.insert('ౌ', 'ौ');       // Telugu ౌ → Devanagari ौ
        
        // Consonants (హల్లులు) → Devanagari equivalents
        telugu_to_deva.insert('క', 'क');       // Telugu క → Devanagari क
        telugu_to_deva.insert('ఖ', 'ख');       // Telugu ఖ → Devanagari ख
        telugu_to_deva.insert('గ', 'ग');       // Telugu గ → Devanagari ग
        telugu_to_deva.insert('ఘ', 'घ');       // Telugu ఘ → Devanagari घ
        telugu_to_deva.insert('ఙ', 'ङ');       // Telugu ఙ → Devanagari ङ
        telugu_to_deva.insert('చ', 'च');       // Telugu చ → Devanagari च
        telugu_to_deva.insert('ఛ', 'छ');       // Telugu ఛ → Devanagari छ
        telugu_to_deva.insert('జ', 'ज');       // Telugu జ → Devanagari ज
        telugu_to_deva.insert('ఝ', 'झ');       // Telugu ఝ → Devanagari झ
        telugu_to_deva.insert('ఞ', 'ञ');       // Telugu ఞ → Devanagari ञ
        telugu_to_deva.insert('ట', 'ट');       // Telugu ట → Devanagari ट
        telugu_to_deva.insert('ఠ', 'ठ');       // Telugu ఠ → Devanagari ठ
        telugu_to_deva.insert('డ', 'ड');       // Telugu డ → Devanagari ड
        telugu_to_deva.insert('ఢ', 'ढ');       // Telugu ఢ → Devanagari ढ
        telugu_to_deva.insert('ణ', 'ण');       // Telugu ణ → Devanagari ण
        telugu_to_deva.insert('త', 'त');       // Telugu త → Devanagari त
        telugu_to_deva.insert('థ', 'थ');       // Telugu థ → Devanagari थ
        telugu_to_deva.insert('ద', 'द');       // Telugu ద → Devanagari द
        telugu_to_deva.insert('ధ', 'ध');       // Telugu ధ → Devanagari ध
        telugu_to_deva.insert('న', 'न');       // Telugu న → Devanagari न
        telugu_to_deva.insert('ప', 'प');       // Telugu ప → Devanagari प
        telugu_to_deva.insert('ఫ', 'फ');       // Telugu ఫ → Devanagari फ
        telugu_to_deva.insert('బ', 'ब');       // Telugu బ → Devanagari ब
        telugu_to_deva.insert('భ', 'भ');       // Telugu భ → Devanagari भ
        telugu_to_deva.insert('మ', 'म');       // Telugu మ → Devanagari म
        telugu_to_deva.insert('య', 'य');       // Telugu య → Devanagari य
        telugu_to_deva.insert('ర', 'र');       // Telugu ర → Devanagari र
        telugu_to_deva.insert('ల', 'ल');       // Telugu ల → Devanagari ल
        telugu_to_deva.insert('వ', 'व');       // Telugu వ → Devanagari व
        telugu_to_deva.insert('శ', 'श');       // Telugu శ → Devanagari श
        telugu_to_deva.insert('ష', 'ष');       // Telugu ష → Devanagari ष
        telugu_to_deva.insert('స', 'स');       // Telugu స → Devanagari स
        telugu_to_deva.insert('హ', 'ह');       // Telugu హ → Devanagari ह
        telugu_to_deva.insert('ళ', 'ळ');       // Telugu ళ → Devanagari ळ
        
        // Special marks
        telugu_to_deva.insert('ం', 'ं');       // Telugu ం → Devanagari ं (anusvara)
        telugu_to_deva.insert('ః', 'ः');       // Telugu ః → Devanagari ः (visarga)
        telugu_to_deva.insert('్', '्');       // Telugu ్ → Devanagari ् (virama)
        telugu_to_deva.insert('ఁ', 'ँ');       // Telugu ఁ → Devanagari ँ (candrabindu - rare)
        
        // Build reverse mapping
        let mut deva_to_telugu = HashMap::new();
        for (&telugu, &deva) in &telugu_to_deva {
            deva_to_telugu.insert(deva, telugu);
        }

        Self {
            telugu_to_deva_map: telugu_to_deva,
            deva_to_telugu_map: deva_to_telugu,
        }
    }
    
    /// Convert Telugu text to Devanagari format with optimized allocation
    pub fn telugu_to_devanagari_optimized(&self, input: &str) -> Result<String, ConverterError> {
        // Pre-allocate with better capacity estimation
        let mut result = String::with_capacity(input.len());
        
        for ch in input.chars() {
            if ch.is_whitespace() || ch.is_ascii_punctuation() {
                result.push(ch);
            } else if let Some(&deva_char) = self.telugu_to_deva_map.get(&ch) {
                result.push(deva_char);
            } else {
                // Character not in mapping - preserve as-is for mixed content
                result.push(ch);
            }
        }
        
        Ok(result)
    }
}

impl ScriptConverter for OptimizedTeluguConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        if script != "telugu" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Telugu converter only supports 'telugu' script".to_string(),
            });
        }
        
        let deva_text = self.telugu_to_devanagari_optimized(input)?;
        Ok(HubInput::Devanagari(deva_text))
    }
    
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        if script != "telugu" {
            return Err(ConverterError::InvalidInput {
                script: script.to_string(),
                message: "Telugu converter only supports 'telugu' script".to_string(),
            });
        }
        
        match hub_input {
            HubInput::Devanagari(deva_text) => {
                // Simple character mapping for reverse conversion
                let result: String = deva_text.chars()
                    .map(|ch| self.deva_to_telugu_map.get(&ch).copied().unwrap_or(ch))
                    .collect();
                Ok(result)
            }
            HubInput::Iso(_) => Err(ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: "Telugu converter expects Devanagari input, got ISO".to_string(),
            }),
        }
    }
    
    fn supported_scripts(&self) -> Vec<&'static str> {
        vec!["telugu"]
    }
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {
        true // Telugu has implicit 'a' vowel
    }
}

impl Default for OptimizedTeluguConverter {
    fn default() -> Self {
        Self::new()
    }
}