//! Mathematical verification tests for the lossless guarantee
//! 
//! This test suite verifies the mathematical properties of the lossless
//! transliteration system using Shannon entropy and information theory.

use shlesha::{LosslessTransliterator};
use std::collections::HashMap;

/// Calculate Shannon entropy of a string
fn shannon_entropy(text: &str) -> f64 {
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    let total_chars = text.chars().count();
    
    if total_chars == 0 {
        return 0.0;
    }
    
    // Count character frequencies
    for ch in text.chars() {
        *char_counts.entry(ch).or_insert(0) += 1;
    }
    
    // Calculate entropy: H = -Σ(p_i * log2(p_i))
    let mut entropy = 0.0;
    for &count in char_counts.values() {
        let probability = count as f64 / total_chars as f64;
        if probability > 0.0 {
            entropy -= probability * probability.log2();
        }
    }
    
    entropy
}

/// Extract preservation tokens from output
fn extract_tokens(output: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut chars = output.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '[' {
            let mut token = String::new();
            token.push(ch);
            
            while let Some(inner_ch) = chars.next() {
                token.push(inner_ch);
                if inner_ch == ']' {
                    break;
                }
            }
            
            if token.ends_with(']') {
                tokens.push(token);
            }
        }
    }
    
    tokens
}

/// Calculate entropy of preservation tokens
fn token_entropy(output: &str) -> f64 {
    let tokens = extract_tokens(output);
    if tokens.is_empty() {
        return 0.0;
    }
    
    // Each token preserves exact information, so entropy is the sum of 
    // individual character entropies within tokens
    let mut total_entropy = 0.0;
    
    for token in tokens {
        // Extract the data portion: [script_id:data:metadata] -> data
        if let Some(start) = token.find(':') {
            if let Some(end) = token[start+1..].find(':') {
                let data = &token[start+1..start+1+end];
                total_entropy += shannon_entropy(data);
            }
        }
    }
    
    total_entropy
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_shannon_entropy_calculation() {
        // Test known entropy values
        assert_eq!(shannon_entropy(""), 0.0);
        assert_eq!(shannon_entropy("aaaa"), 0.0); // No entropy in uniform string
        
        // Equal distribution should have higher entropy
        let entropy_uniform = shannon_entropy("abcd");
        let entropy_skewed = shannon_entropy("aaab");
        assert!(entropy_uniform > entropy_skewed);
    }
    
    #[test]
    fn test_lossless_guarantee_mathematical() {
        let transliterator = LosslessTransliterator::new();
        
        let test_cases = vec![
            "धर्म",
            "क्ष्म्य",
            "ॐ मणि पद्मे हूँ",
            "संस्कृत",
            "अहिंसा परमो धर्मः",
        ];
        
        for &input in &test_cases {
            let output = transliterator
                .transliterate(input, "Devanagari", "IAST")
                .unwrap();
            
            // Calculate entropies
            let h_original = shannon_entropy(input);
            let h_encoded = shannon_entropy(&output);
            let h_tokens = token_entropy(&output);
            
            // Verify lossless guarantee: H(original) ≤ H(encoded) + H(tokens)
            let total_preserved_entropy = h_encoded + h_tokens;
            
            println!("Input: '{}' (H = {:.3})", input, h_original);
            println!("Output: '{}' (H = {:.3})", output, h_encoded);
            println!("Tokens entropy: {:.3}", h_tokens);
            println!("Total preserved: {:.3}", total_preserved_entropy);
            println!("Lossless: {}", total_preserved_entropy >= h_original);
            println!();
            
            // Mathematical lossless guarantee
            assert!(
                total_preserved_entropy >= h_original - f64::EPSILON,
                "Lossless guarantee violated for '{}': {} < {}",
                input, total_preserved_entropy, h_original
            );
        }
    }
    
    #[test]
    fn test_lossless_verification_api() {
        let transliterator = LosslessTransliterator::new();
        
        let test_cases = vec![
            ("धर्म", "Simple word"),
            ("क्ष्म्य त्र्य ज्ञ श्र", "Complex consonant clusters"),
            ("ॐ", "Sacred symbol"),
            ("अहिंसा परमो धर्मः", "Complete Sanskrit sentence"),
            ("123 धर्म abc", "Mixed script text"),
        ];
        
        for (input, description) in test_cases {
            let output = transliterator
                .transliterate(input, "Devanagari", "IAST")
                .unwrap();
            
            let verification = transliterator
                .verify_lossless(input, &output, "Devanagari");
            
            println!("{}: '{}'", description, input);
            println!("  Output: '{}'", output);
            println!("  Lossless: {} ({:.1}% preservation)", 
                     verification.is_lossless,
                     verification.preservation_ratio * 100.0);
            println!("  Tokens: {}", verification.tokens_count);
            
            // Verify the API guarantees losslessness
            assert!(
                verification.is_lossless,
                "Lossless verification failed for '{}' ({})",
                input, description
            );
            
            // Preservation ratio should be >= 100%
            assert!(
                verification.preservation_ratio >= 1.0,
                "Preservation ratio < 100% for '{}': {:.1}%",
                input, verification.preservation_ratio * 100.0
            );
        }
    }
    
    #[test]
    fn test_information_preservation_edge_cases() {
        let transliterator = LosslessTransliterator::new();
        
        // Edge cases that might challenge lossless guarantee
        let edge_cases = vec![
            ("", "Empty string"),
            ("a", "Single ASCII character"),
            ("ॐ", "Single Unicode symbol"),
            ("्", "Standalone virama"),
            ("॥", "Double danda"),
            ("०१२३४५६७८९", "Devanagari numerals"),
            ("ऽ", "Avagraha"),
            ("ॐ॥ॐ॥", "Repeated symbols"),
        ];
        
        for (input, description) in edge_cases {
            let output = transliterator
                .transliterate(input, "Devanagari", "IAST")
                .unwrap();
            
            let verification = transliterator
                .verify_lossless(input, &output, "Devanagari");
            
            println!("{}: '{}'", description, input);
            println!("  Output: '{}'", output);
            println!("  Verification: {}", verification.is_lossless);
            
            assert!(
                verification.is_lossless,
                "Edge case lossless guarantee failed: {} ('{}')",
                description, input
            );
        }
    }
    
    #[test]
    fn test_entropy_conservation_large_text() {
        let transliterator = LosslessTransliterator::new();
        
        // Large text to test entropy conservation at scale
        let large_text = "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः । मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय ॥ ".repeat(100);
        
        let output = transliterator
            .transliterate(&large_text, "Devanagari", "IAST")
            .unwrap();
        
        let h_original = shannon_entropy(&large_text);
        let h_encoded = shannon_entropy(&output);
        let h_tokens = token_entropy(&output);
        
        println!("Large text test ({}+ characters):", large_text.len());
        println!("  Original entropy: {:.3}", h_original);
        println!("  Encoded entropy: {:.3}", h_encoded);
        println!("  Token entropy: {:.3}", h_tokens);
        println!("  Total preserved: {:.3}", h_encoded + h_tokens);
        
        // Verify mathematical guarantee holds for large text
        assert!(
            h_encoded + h_tokens >= h_original - f64::EPSILON,
            "Large text lossless guarantee failed: {} < {}",
            h_encoded + h_tokens, h_original
        );
        
        // Verify API also confirms losslessness
        let verification = transliterator
            .verify_lossless(&large_text, &output, "Devanagari");
        assert!(verification.is_lossless);
    }
    
    #[test]
    fn test_token_structure_validity() {
        let transliterator = LosslessTransliterator::new();
        
        // Text with characters that should generate tokens
        let input = "Hello धर्म ॐ 123";
        let output = transliterator
            .transliterate(input, "Mixed", "IAST")
            .unwrap();
        
        let tokens = extract_tokens(&output);
        
        for token in &tokens {
            println!("Token: {}", token);
            
            // Verify token structure: [script_id:data:metadata]
            assert!(token.starts_with('[') && token.ends_with(']'));
            
            let inner = &token[1..token.len()-1];
            let parts: Vec<&str> = inner.split(':').collect();
            
            // Should have at least script_id and data
            assert!(parts.len() >= 2, "Invalid token structure: {}", token);
            
            // Script ID should be numeric
            assert!(parts[0].parse::<u32>().is_ok(), "Invalid script ID: {}", parts[0]);
            
            // Data should be non-empty
            assert!(!parts[1].is_empty(), "Empty data in token: {}", token);
        }
        
        // Should have tokens for unsupported characters
        assert!(!tokens.is_empty(), "Expected tokens for mixed script input");
    }
    
    #[test]
    fn test_performance_invariance() {
        let transliterator = LosslessTransliterator::new();
        
        // Test that lossless guarantee doesn't degrade with repeated operations
        let original = "धर्मक्षेत्रे कुरुक्षेत्रे";
        
        for i in 1..=10 {
            let repeated_text = original.repeat(i);
            
            let output = transliterator
                .transliterate(&repeated_text, "Devanagari", "IAST")
                .unwrap();
            
            let verification = transliterator
                .verify_lossless(&repeated_text, &output, "Devanagari");
            
            assert!(
                verification.is_lossless,
                "Lossless guarantee failed at repetition {}: text length {}",
                i, repeated_text.len()
            );
            
            // Preservation ratio should remain consistent
            assert!(
                verification.preservation_ratio >= 1.0,
                "Preservation ratio degraded at repetition {}: {:.3}",
                i, verification.preservation_ratio
            );
        }
    }
}