use std::collections::HashMap;
use super::ConverterError;

/// Optimized Indic script processor with eliminated string allocations
pub struct OptimizedIndicScriptProcessor;

impl OptimizedIndicScriptProcessor {
    /// Optimized process Indic script text to hub format eliminating String allocations
    /// Handles implicit 'a' vowel and virama logic efficiently
    pub fn to_hub(
        input: &str,
        consonant_map: &HashMap<&str, &str>,
        vowel_map: &HashMap<&str, &str>,
        vowel_sign_map: &HashMap<&str, &str>,
        misc_map: &HashMap<&str, &str>,
        virama: char,
    ) -> Result<String, ConverterError> {
        let mut result = String::with_capacity(input.len() * 2);
        let bytes = input.as_bytes();
        let mut byte_idx = 0;
        
        while byte_idx < bytes.len() {
            let ch = input[byte_idx..].chars().next().unwrap();
            let ch_len = ch.len_utf8();
            
            // Check for whitespace
            if ch.is_whitespace() {
                result.push(ch);
                byte_idx += ch_len;
                continue;
            }
            
            // Create a single-character string slice for lookup
            let ch_str = &input[byte_idx..byte_idx + ch_len];
            
            // Check for consonants
            if let Some(&cons) = consonant_map.get(ch_str) {
                result.push_str(cons);
                
                // Look ahead for virama or vowel sign
                let next_byte_idx = byte_idx + ch_len;
                if next_byte_idx < bytes.len() {
                    let next_ch = input[next_byte_idx..].chars().next().unwrap();
                    let next_ch_len = next_ch.len_utf8();
                    
                    if next_ch == virama {
                        // Consonant without vowel - skip the virama and don't add 'a'
                        byte_idx += ch_len + next_ch_len;
                        continue;
                    } else {
                        // Check for vowel sign using direct string slice
                        let next_ch_str = &input[next_byte_idx..next_byte_idx + next_ch_len];
                        if vowel_sign_map.contains_key(next_ch_str) {
                            // Consonant with explicit vowel sign - don't add 'a'
                        } else {
                            // Consonant with implicit 'a'
                            result.push('a');
                        }
                    }
                } else {
                    // End of string - add implicit 'a'
                    result.push('a');
                }
            }
            // Check for independent vowels
            else if let Some(&vowel) = vowel_map.get(ch_str) {
                result.push_str(vowel);
            }
            // Check for vowel signs (should not appear independently, but handle gracefully)
            else if let Some(&vowel_sign) = vowel_sign_map.get(ch_str) {
                result.push_str(vowel_sign);
            }
            // Check for misc characters (anusvara, visarga, etc.)
            else if let Some(&misc) = misc_map.get(ch_str) {
                result.push_str(misc);
            }
            // Preserve unknown characters
            else {
                result.push(ch);
            }
            
            byte_idx += ch_len;
        }
        
        Ok(result)
    }
    
    /// Multi-character sequence aware Indic processor for complex scripts
    pub fn to_hub_with_sequences(
        input: &str,
        consonant_map: &HashMap<&str, &str>,
        vowel_map: &HashMap<&str, &str>,
        vowel_sign_map: &HashMap<&str, &str>,
        misc_map: &HashMap<&str, &str>,
        virama: char,
    ) -> Result<String, ConverterError> {
        let mut result = String::with_capacity(input.len() * 2);
        let bytes = input.as_bytes();
        let mut byte_idx = 0;
        
        while byte_idx < bytes.len() {
            let ch = input[byte_idx..].chars().next().unwrap();
            let ch_len = ch.len_utf8();
            
            // Check for whitespace
            if ch.is_whitespace() {
                result.push(ch);
                byte_idx += ch_len;
                continue;
            }
            
            let mut matched = false;
            
            // Try to match multi-character sequences (up to 3 chars for complex scripts)
            for seq_len in (1..=3).rev() {
                if let Some(end_idx) = Self::get_char_boundary(&input[byte_idx..], seq_len) {
                    let seq = &input[byte_idx..byte_idx + end_idx];
                    
                    // Check all mapping types for this sequence
                    if let Some(&cons) = consonant_map.get(seq) {
                        result.push_str(cons);
                        
                        // Handle implicit 'a' for consonants
                        let next_byte_idx = byte_idx + end_idx;
                        if next_byte_idx < bytes.len() {
                            let next_ch = input[next_byte_idx..].chars().next().unwrap();
                            let next_ch_len = next_ch.len_utf8();
                            
                            if next_ch == virama {
                                byte_idx += end_idx + next_ch_len;
                                matched = true;
                                break;
                            } else {
                                let next_ch_str = &input[next_byte_idx..next_byte_idx + next_ch_len];
                                if !vowel_sign_map.contains_key(next_ch_str) {
                                    result.push('a');
                                }
                            }
                        } else {
                            result.push('a');
                        }
                        
                        byte_idx += end_idx;
                        matched = true;
                        break;
                    }
                    else if let Some(&vowel) = vowel_map.get(seq) {
                        result.push_str(vowel);
                        byte_idx += end_idx;
                        matched = true;
                        break;
                    }
                    else if let Some(&vowel_sign) = vowel_sign_map.get(seq) {
                        result.push_str(vowel_sign);
                        byte_idx += end_idx;
                        matched = true;
                        break;
                    }
                    else if let Some(&misc) = misc_map.get(seq) {
                        result.push_str(misc);
                        byte_idx += end_idx;
                        matched = true;
                        break;
                    }
                }
            }
            
            if !matched {
                result.push(ch);
                byte_idx += ch_len;
            }
        }
        
        Ok(result)
    }
    
    /// Get the byte offset for the end of the nth character from the start of a string slice
    fn get_char_boundary(s: &str, char_count: usize) -> Option<usize> {
        let mut chars = s.char_indices();
        
        for _ in 0..char_count {
            if chars.next().is_none() {
                return None;
            }
        }
        
        Some(chars.next().map(|(idx, _)| idx).unwrap_or(s.len()))
    }
    
    /// Process hub format to Indic script (reverse conversion)
    /// This is similar to Roman script processing but handles Indic-specific logic
    pub fn from_hub(
        input: &str,
        mapping: &HashMap<&str, &str>,
        _has_implicit_a: bool,
    ) -> Result<String, ConverterError> {
        // For now, use optimized Roman-style processing
        // TODO: Add Indic-specific logic for vowel mark generation
        let mut result = String::with_capacity(input.len() * 2);
        let bytes = input.as_bytes();
        let mut byte_idx = 0;
        
        while byte_idx < bytes.len() {
            let ch = input[byte_idx..].chars().next().unwrap();
            let ch_len = ch.len_utf8();
            
            if ch.is_whitespace() {
                result.push(ch);
                byte_idx += ch_len;
                continue;
            }
            
            let mut matched = false;
            
            // Try matching sequences by directly slicing the string
            for seq_len in (1..=4).rev() {
                if let Some(end_idx) = Self::get_char_boundary(&input[byte_idx..], seq_len) {
                    let seq = &input[byte_idx..byte_idx + end_idx];
                    if let Some(&mapped_str) = mapping.get(seq) {
                        result.push_str(mapped_str);
                        byte_idx += end_idx;
                        matched = true;
                        break;
                    }
                }
            }
            
            if !matched {
                result.push(ch);
                byte_idx += ch_len;
            }
        }
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_char_boundary_helper() {
        let text = "धर्म"; // Devanagari with virama
        
        assert_eq!(OptimizedIndicScriptProcessor::get_char_boundary(text, 1), Some(3)); // 'ध' (3 bytes)
        assert_eq!(OptimizedIndicScriptProcessor::get_char_boundary(text, 2), Some(6)); // 'धर' 
        assert_eq!(OptimizedIndicScriptProcessor::get_char_boundary(text, 3), Some(9)); // 'धर्' (with virama)
        assert_eq!(OptimizedIndicScriptProcessor::get_char_boundary(text, 4), Some(text.len())); // Full string
        assert_eq!(OptimizedIndicScriptProcessor::get_char_boundary(text, 5), None); // Too many chars
    }
}