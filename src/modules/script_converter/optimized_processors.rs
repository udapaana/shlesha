use std::collections::HashMap;
use super::ConverterError;

/// Optimized processor for Roman script conversions with reduced allocations
/// Focuses on eliminating intermediate string allocations in hot paths
pub struct OptimizedRomanScriptProcessor;

impl OptimizedRomanScriptProcessor {
    /// Process Roman script text using optimized allocation patterns
    pub fn process_optimized(
        input: &str, 
        mapping: &HashMap<&str, &str>
    ) -> Result<String, ConverterError> {
        // Pre-allocate with more aggressive sizing for better performance
        let mut result = String::with_capacity(input.len() * 3 / 2);
        let mut chars = input.char_indices();
        
        while let Some((i, ch)) = chars.next() {
            // Fast path for whitespace
            if ch.is_whitespace() {
                result.push(ch);
                continue;
            }
            
            let mut matched = false;
            let remaining = &input[i..];
            
            // Try to match sequences of decreasing length (4, 3, 2, 1)
            // Optimized: Use string slicing directly instead of Vec<char> allocation
            for len in (1..=4).rev() {
                if let Some(substring) = Self::get_char_substring(remaining, len) {
                    if let Some(&mapped_str) = mapping.get(substring) {
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
    
    /// Get substring of exactly `char_count` characters without allocation
    /// Returns None if not enough characters available
    fn get_char_substring(s: &str, char_count: usize) -> Option<&str> {
        let mut char_indices = s.char_indices();
        
        // Skip to the character at position char_count
        for _ in 0..char_count {
            if char_indices.next().is_none() {
                return None;
            }
        }
        
        // Get the byte index after char_count characters
        if let Some((end_byte_index, _)) = char_indices.next() {
            Some(&s[..end_byte_index])
        } else {
            // We reached the end of the string exactly at char_count characters
            Some(s)
        }
    }
}

/// Optimized processor for Indic script conversions with reduced allocations
pub struct OptimizedIndicScriptProcessor;

impl OptimizedIndicScriptProcessor {
    /// Process Indic script text to hub format with optimized allocations
    pub fn to_hub_optimized(
        input: &str,
        consonant_map: &HashMap<char, &str>,  // Changed to char key to avoid to_string()
        vowel_map: &HashMap<char, &str>,      // Changed to char key
        vowel_sign_map: &HashMap<char, &str>, // Changed to char key  
        misc_map: &HashMap<char, &str>,       // Changed to char key
        virama: char,
    ) -> Result<String, ConverterError> {
        let mut result = String::with_capacity(input.len() * 3 / 2);
        let mut chars = input.chars().peekable();
        
        while let Some(ch) = chars.next() {
            // Check for whitespace
            if ch.is_whitespace() {
                result.push(ch);
                continue;
            }
            
            // Check for consonants - no more to_string() allocations!
            if let Some(&cons) = consonant_map.get(&ch) {
                result.push_str(cons);
                
                // Check if followed by virama or vowel sign
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == virama {
                        // Consonant without vowel - skip the virama and don't add 'a'
                        chars.next();
                    } else if vowel_sign_map.contains_key(&next_ch) {
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
            // Check for independent vowels - no more to_string() allocations!
            else if let Some(&vowel) = vowel_map.get(&ch) {
                result.push_str(vowel);
            }
            // Check for vowel signs
            else if let Some(&vowel_sign) = vowel_sign_map.get(&ch) {
                result.push_str(vowel_sign);
            }
            // Check for misc characters (anusvara, visarga, etc.)
            else if let Some(&misc) = misc_map.get(&ch) {
                result.push_str(misc);
            }
            // Preserve unknown characters
            else {
                result.push(ch);
            }
        }
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_char_substring() {
        let text = "hello";
        
        assert_eq!(OptimizedRomanScriptProcessor::get_char_substring(text, 1), Some("h"));
        assert_eq!(OptimizedRomanScriptProcessor::get_char_substring(text, 2), Some("he"));
        assert_eq!(OptimizedRomanScriptProcessor::get_char_substring(text, 5), Some("hello"));
        assert_eq!(OptimizedRomanScriptProcessor::get_char_substring(text, 6), None);
        
        // Test with Unicode characters
        let unicode_text = "தெலుగు";
        assert_eq!(OptimizedRomanScriptProcessor::get_char_substring(unicode_text, 1), Some("த"));
        assert_eq!(OptimizedRomanScriptProcessor::get_char_substring(unicode_text, 2), Some("தெ"));
    }
    
    #[test]
    fn test_optimized_roman_processing() {
        let mut mapping = HashMap::new();
        mapping.insert("a", "a");
        mapping.insert("kh", "kh");
        mapping.insert("k", "k");
        
        let result = OptimizedRomanScriptProcessor::process_optimized("kha", &mapping).unwrap();
        assert_eq!(result, "kh");
        
        let result = OptimizedRomanScriptProcessor::process_optimized("ka", &mapping).unwrap();
        assert_eq!(result, "ka");
    }
}