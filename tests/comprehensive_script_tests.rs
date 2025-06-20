//! Comprehensive tests for multi-script support

use shlesha::LosslessTransliterator;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_devanagari_basic() {
        let transliterator = LosslessTransliterator::new();
        
        let test_cases = vec![
            "धर्म",
            "अहिंसा",
            "सत्य",
            "क्ष्म्य",
            "ॐ",
        ];
        
        for text in test_cases {
            println!("Testing Devanagari: {}", text);
            
            let result = transliterator
                .transliterate(text, "Devanagari", "IAST")
                .unwrap();
            
            let verification = transliterator
                .verify_lossless(text, &result, "Devanagari");
            
            println!("  → {}", result);
            println!("  Lossless: {} ({:.1}%)", 
                     verification.is_lossless,
                     verification.preservation_ratio * 100.0);
            
            assert!(verification.is_lossless,
                   "Lossless verification failed for '{}'", text);
            println!();
        }
    }
    
    #[test]
    fn test_mixed_script_handling() {
        let transliterator = LosslessTransliterator::new();
        
        let mixed_cases = vec![
            ("English + Devanagari", "Hello धर्म world"),
            ("Numbers + Devanagari", "123 धर्म 456"),
            ("Punctuation + Devanagari", "धर्म, अहिंसा! सत्य?"),
        ];
        
        for (description, text) in mixed_cases {
            println!("Mixed script test: {} - '{}'", description, text);
            
            let result = transliterator
                .transliterate(text, "Mixed", "IAST")
                .unwrap();
            
            let verification = transliterator
                .verify_lossless(text, &result, "Mixed");
            
            println!("  Output: '{}'", result);
            println!("  Lossless: {} ({:.1}% preservation)",
                     verification.is_lossless,
                     verification.preservation_ratio * 100.0);
            
            assert!(verification.is_lossless,
                   "Mixed script lossless guarantee failed: '{}'", text);
                   
            println!();
        }
    }
}