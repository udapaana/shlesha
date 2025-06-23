use std::collections::HashMap;
use super::ConverterError;

/// Shared processor for Roman script conversions (IAST, ITRANS, SLP1, etc.)
/// Handles the common logic for all Roman transliteration schemes
pub struct RomanScriptProcessor;

impl RomanScriptProcessor {
    /// Process Roman script text using the provided mapping table
    /// This is the optimized implementation that all Roman converters will use
    pub fn process(
        input: &str, 
        mapping: &HashMap<&str, &str>
    ) -> Result<String, ConverterError> {
        let mut result = String::with_capacity(input.len() * 2); // Pre-allocate for worst case
        let mut chars = input.char_indices();
        
        while let Some((i, ch)) = chars.next() {
            // Fast path for whitespace
            if ch.is_whitespace() {
                result.push(ch);
                continue;
            }
            
            // Note: Don't preserve punctuation as-is since some schemes (Velthuis, WX) 
            // use punctuation characters as part of their encoding
            
            let mut matched = false;
            let remaining = &input[i..];
            
            // Try to match sequences of decreasing length (4, 3, 2, 1)
            // This handles multi-character sequences like "kh", "gh", "r̥̄", etc.
            for len in (1..=4).rev() {
                // Get the substring containing the next 'len' characters
                let chars_to_take: Vec<char> = remaining.chars().take(len).collect();
                if chars_to_take.len() == len {
                    let seq: String = chars_to_take.iter().collect();
                    if let Some(&mapped_str) = mapping.get(seq.as_str()) {
                        result.push_str(mapped_str);
                        // Skip the matched characters (len - 1 because we already have the first one)
                        for _ in 1..len {
                            chars.next();
                        }
                        matched = true;
                        break;
                    }
                }
            }
            
            if !matched {
                // Character not found in mapping - preserve as-is
                result.push(ch);
            }
        }
        
        Ok(result)
    }
    
    /// Optimized version for processing with early exit on ASCII-only text
    pub fn process_optimized(
        input: &str,
        mapping: &HashMap<&str, &str>
    ) -> Result<String, ConverterError> {
        // For now, just use the general processor to ensure it works correctly
        // TODO: Re-enable optimizations after fixing the logic
        Self::process(input, mapping)
    }
    
    /// Fast path for pure ASCII text without diacritics
    fn process_ascii_fast(
        input: &str,
        mapping: &HashMap<&str, &str>
    ) -> Result<String, ConverterError> {
        let mut result = String::with_capacity(input.len() * 2);
        let bytes = input.as_bytes();
        let mut i = 0;
        
        while i < bytes.len() {
            let ch = bytes[i] as char;
            
            if ch.is_whitespace() {
                result.push(ch);
                i += 1;
                continue;
            }
            
            // Note: Don't preserve punctuation as-is since some schemes (Velthuis, WX) 
            // use punctuation characters as part of their encoding
            
            let mut matched = false;
            
            // For ASCII, we can work directly with byte slices
            for len in (1..=4).rev() {
                if i + len > bytes.len() {
                    continue;
                }
                
                if let Ok(seq) = std::str::from_utf8(&bytes[i..i + len]) {
                    if let Some(&mapped_str) = mapping.get(seq) {
                        result.push_str(mapped_str);
                        i += len;
                        matched = true;
                        break;
                    }
                }
            }
            
            if !matched {
                result.push(ch);
                i += 1;
            }
        }
        
        Ok(result)
    }
}

/// Shared processor for Indic script conversions (Devanagari, Bengali, Tamil, etc.)
/// Handles the common logic for all Indic scripts with implicit 'a' vowel
pub struct IndicScriptProcessor;

impl IndicScriptProcessor {
    /// Process Indic script text to hub format (ISO or Devanagari)
    /// Handles implicit 'a' vowel and virama logic
    pub fn to_hub(
        input: &str,
        consonant_map: &HashMap<&str, &str>,
        vowel_map: &HashMap<&str, &str>,
        vowel_sign_map: &HashMap<&str, &str>,
        misc_map: &HashMap<&str, &str>,
        virama: char,
    ) -> Result<String, ConverterError> {
        let mut result = String::with_capacity(input.len() * 2);
        let mut chars = input.chars().peekable();
        
        while let Some(ch) = chars.next() {
            // Check for whitespace
            if ch.is_whitespace() {
                result.push(ch);
                continue;
            }
            
            // Check for consonants
            if let Some(&cons) = consonant_map.get(&ch.to_string().as_str()) {
                result.push_str(cons);
                
                // Check if followed by virama or vowel sign
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == virama {
                        // Consonant without vowel - skip the virama and don't add 'a'
                        chars.next();
                    } else if vowel_sign_map.contains_key(&next_ch.to_string().as_str()) {
                        // Consonant with explicit vowel sign - don't add 'a'
                    } else {
                        // Consonant with implicit 'a'
                        result.push('a');
                    }
                } else {
                    // End of string - add implicit 'a'
                    result.push('a');
                }
            }
            // Check for independent vowels
            else if let Some(&vowel) = vowel_map.get(&ch.to_string().as_str()) {
                result.push_str(vowel);
            }
            // Check for vowel signs (should not appear independently, but handle gracefully)
            else if let Some(&vowel_sign) = vowel_sign_map.get(&ch.to_string().as_str()) {
                result.push_str(vowel_sign);
            }
            // Check for misc characters (anusvara, visarga, etc.)
            else if let Some(&misc) = misc_map.get(&ch.to_string().as_str()) {
                result.push_str(misc);
            }
            // Preserve unknown characters
            else {
                result.push(ch);
            }
        }
        
        Ok(result)
    }
    
    /// Process hub format (ISO or Devanagari) to Indic script
    /// Handles vowel mark generation and consonant cluster formation
    pub fn from_hub(
        input: &str,
        mapping: &HashMap<&str, &str>,
        has_implicit_a: bool,
    ) -> Result<String, ConverterError> {
        // For now, use the same logic as Roman scripts
        // TODO: Implement proper Indic-specific logic for vowel marks
        RomanScriptProcessor::process(input, mapping)
    }
}