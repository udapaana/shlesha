use std::collections::HashMap;
use super::ConverterError;

/// Optimized Roman script processor with eliminated string allocations
pub struct OptimizedRomanScriptProcessor;

impl OptimizedRomanScriptProcessor {
    /// Optimized process function that eliminates Vec<char> and String allocations
    pub fn process(
        input: &str, 
        mapping: &HashMap<&str, &str>
    ) -> Result<String, ConverterError> {
        let mut result = String::with_capacity(input.len() * 2);
        let bytes = input.as_bytes();
        let mut byte_idx = 0;
        
        while byte_idx < bytes.len() {
            let ch = input[byte_idx..].chars().next().unwrap();
            let ch_len = ch.len_utf8();
            
            // Fast path for whitespace
            if ch.is_whitespace() {
                result.push(ch);
                byte_idx += ch_len;
                continue;
            }
            
            let mut matched = false;
            
            // Try matching sequences by directly slicing the string
            // This avoids Vec<char> allocation and String::from_iter
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
    
    /// Get the byte offset for the end of the nth character from the start of a string slice
    /// Returns None if there are fewer than n characters
    fn get_char_boundary(s: &str, char_count: usize) -> Option<usize> {
        let mut chars = s.char_indices();
        
        // Take char_count characters and find the byte index after the last one
        for _ in 0..char_count {
            if chars.next().is_none() {
                return None; // Not enough characters
            }
        }
        
        // Get the next character's byte index, or end of string
        Some(chars.next().map(|(idx, _)| idx).unwrap_or(s.len()))
    }
    
    /// Specialized ASCII-only optimization for when input contains only ASCII
    pub fn process_ascii_only(
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
            
            let mut matched = false;
            
            // For ASCII, work directly with byte slices
            for len in (1..=4).rev() {
                if i + len <= bytes.len() {
                    if let Ok(seq) = std::str::from_utf8(&bytes[i..i + len]) {
                        if let Some(&mapped_str) = mapping.get(seq) {
                            result.push_str(mapped_str);
                            i += len;
                            matched = true;
                            break;
                        }
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
    
    /// Auto-detecting processor that chooses the best algorithm
    pub fn process_auto(
        input: &str,
        mapping: &HashMap<&str, &str>
    ) -> Result<String, ConverterError> {
        // Check if input is ASCII-only for fast path
        if input.is_ascii() {
            Self::process_ascii_only(input, mapping)
        } else {
            Self::process(input, mapping)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_char_boundary_helper() {
        let text = "kāñcana"; // Mix of ASCII and diacritics
        
        assert_eq!(OptimizedRomanScriptProcessor::get_char_boundary(text, 1), Some(1)); // 'k'
        assert_eq!(OptimizedRomanScriptProcessor::get_char_boundary(text, 2), Some(3)); // 'kā' (ā is 2 bytes)
        assert_eq!(OptimizedRomanScriptProcessor::get_char_boundary(text, 3), Some(5)); // 'kāñ' (ñ is 2 bytes)
        assert_eq!(OptimizedRomanScriptProcessor::get_char_boundary(text, 8), Some(text.len())); // Full string
        assert_eq!(OptimizedRomanScriptProcessor::get_char_boundary(text, 9), None); // Too many chars
    }
    
    #[test]
    fn test_ascii_optimization() {
        let mut mapping = HashMap::new();
        mapping.insert("k", "क");
        mapping.insert("a", "अ");
        mapping.insert("ka", "का");
        
        let result = OptimizedRomanScriptProcessor::process_ascii_only("ka", &mapping).unwrap();
        assert_eq!(result, "का");
        
        let result = OptimizedRomanScriptProcessor::process_ascii_only("k a", &mapping).unwrap();
        assert_eq!(result, "क अ");
    }
    
    #[test]
    fn test_unicode_processing() {
        let mut mapping = HashMap::new();
        mapping.insert("ā", "आ");
        mapping.insert("ñ", "ञ");
        mapping.insert("kā", "का");
        
        let result = OptimizedRomanScriptProcessor::process("kāñ", &mapping).unwrap();
        assert_eq!(result, "काञ");
    }
}