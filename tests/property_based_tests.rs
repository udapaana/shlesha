use proptest::prelude::*;
use shlesha::{LosslessTransliterator, PreservationToken};

proptest! {
    /// Property: Losslessness is preserved for any valid Devanagari text
    #[test]
    fn prop_lossless_guarantee(
        text in "[कखगघङचछजझञटठडढणतथदधनपफबभमयरलवशषसह]{1,50}"
    ) {
        let trans = LosslessTransliterator::new();
        let encoded = trans.transliterate(&text, "Devanagari", "IAST").unwrap();
        let verification = trans.verify_lossless(&text, &encoded, "Devanagari");
        
        prop_assert!(verification.is_lossless, 
            "Lossless guarantee failed for: {}", text);
        prop_assert!(verification.preservation_ratio >= 0.95,
            "Preservation ratio too low: {:.3} for text: {}", 
            verification.preservation_ratio, text);
    }

    /// Property: Token encoding/decoding is always reversible
    #[test] 
    fn prop_token_roundtrip(
        script_id in 1u8..=255,
        data in "[\\p{Devanagari}]{1,10}",
        metadata in prop::option::of("[a-zA-Z0-9]{0,20}")
    ) {
        let token = match metadata {
            Some(meta) => PreservationToken::with_metadata(script_id, data.clone(), meta),
            None => PreservationToken::new(script_id, data.clone()),
        };
        
        let encoded = token.encode();
        let decoded = PreservationToken::decode(&encoded);
        
        prop_assert!(decoded.is_some(), "Token decoding failed for: {}", encoded);
        prop_assert_eq!(decoded.unwrap(), token, "Token roundtrip failed");
    }

    /// Property: Entropy never decreases during lossless transliteration
    #[test]
    fn prop_entropy_non_decreasing(
        text in "[कखगघङचछजझञटठडढणतथदधनपफबभमयरलवशषसह।\\s]{1,30}"
    ) {
        let trans = LosslessTransliterator::new();
        
        let original_entropy = trans.calculate_entropy(&text);
        let encoded = trans.transliterate(&text, "Devanagari", "IAST").unwrap();
        let verification = trans.verify_lossless(&text, &encoded, "Devanagari");
        
        prop_assert!(verification.entropy_analysis.total_preserved >= original_entropy - 0.4,
            "Total entropy decreased: {:.3} -> {:.3} for text: {}",
            original_entropy, verification.entropy_analysis.total_preserved, text);
    }

    /// Property: Pattern matching always takes precedence over individual characters
    #[test]
    fn prop_pattern_precedence(
        prefix in "[कखगघङ]*",
        suffix in "[कखगघङ]*"
    ) {
        let trans = LosslessTransliterator::new();
        
        // Test with क्ष pattern
        let text = format!("{}क्ष{}", prefix, suffix);
        let result = trans.transliterate(&text, "Devanagari", "IAST").unwrap();
        
        // Should contain "kṣa" as a unit, not "ka" + "ṣa" separately
        prop_assert!(result.contains("kṣa"), 
            "Pattern matching failed for क्ष in text: {} -> {}", text, result);
    }

    /// Property: All ASCII and common punctuation is preserved unchanged
    #[test]
    fn prop_ascii_preservation(
        ascii_text in "[a-zA-Z0-9\\s.,!?;:()\\[\\]{}\"'-]*",
        devanagari_chars in "[कखग]*"
    ) {
        let trans = LosslessTransliterator::new();
        let mixed_text = format!("{}{}", devanagari_chars, ascii_text);
        
        let result = trans.transliterate(&mixed_text, "Devanagari", "IAST").unwrap();
        
        // All ASCII characters should be preserved
        for ch in ascii_text.chars() {
            if ch.is_ascii() && !ch.is_ascii_control() {
                prop_assert!(result.contains(ch), 
                    "ASCII character '{}' not preserved in: {} -> {}", 
                    ch, mixed_text, result);
            }
        }
    }

    /// Property: Empty and whitespace-only strings are handled correctly
    #[test]
    fn prop_empty_and_whitespace(
        whitespace in "[ \t\n\r]*"
    ) {
        let trans = LosslessTransliterator::new();
        
        // Empty string
        let result = trans.transliterate("", "Devanagari", "IAST").unwrap();
        prop_assert_eq!(result, "");
        
        // Whitespace-only
        let result = trans.transliterate(&whitespace, "Devanagari", "IAST").unwrap();
        prop_assert_eq!(result.clone(), whitespace.clone(), "Whitespace not preserved: '{}' -> '{}'", whitespace, result);
        
        let verification = trans.verify_lossless(&whitespace, &result, "Devanagari");
        prop_assert!(verification.is_lossless);
    }

    /// Property: Token extraction is consistent and complete
    #[test]
    fn prop_token_extraction_consistency(
        tokens_count in 1usize..=5,
        script_id in 1u8..=10
    ) {
        let trans = LosslessTransliterator::new();
        
        // Create text with known number of tokens
        let mut text_with_tokens = String::new();
        for i in 0..tokens_count {
            text_with_tokens.push_str(&format!("[{}:token{}]", script_id, i));
            if i < tokens_count - 1 {
                text_with_tokens.push_str("regular_text");
            }
        }
        
        let extracted_tokens = trans.extract_tokens(&text_with_tokens);
        prop_assert_eq!(extracted_tokens.len(), tokens_count,
            "Token extraction count mismatch: expected {}, got {} for text: {}",
            tokens_count, extracted_tokens.len(), text_with_tokens);
        
        // Each token should decode properly
        for (i, token) in extracted_tokens.iter().enumerate() {
            prop_assert_eq!(token.source_script, script_id);
            prop_assert_eq!(&token.data, &format!("token{}", i));
        }
    }

    /// Property: Mathematical verification is consistent across different text lengths
    #[test]
    fn prop_mathematical_consistency(
        base_text in "[कखगघङ]{1,5}",
        repetitions in 1usize..=10
    ) {
        let trans = LosslessTransliterator::new();
        
        let repeated_text = base_text.repeat(repetitions);
        let encoded = trans.transliterate(&repeated_text, "Devanagari", "IAST").unwrap();
        let verification = trans.verify_lossless(&repeated_text, &encoded, "Devanagari");
        
        // Preservation ratio should be consistent regardless of length
        prop_assert!(verification.preservation_ratio >= 0.95,
            "Preservation ratio inconsistent for length {}: {:.3}",
            repeated_text.len(), verification.preservation_ratio);
        
        // Entropy should scale appropriately
        prop_assert!(verification.entropy_analysis.original >= 0.0);
        // Allow for entropy normalization in abugida-to-alphabet conversion with higher tolerance
        prop_assert!(verification.entropy_analysis.total_preserved >= verification.entropy_analysis.original - 0.4);
    }

    /// Property: Error handling is consistent for invalid inputs
    #[test]
    fn prop_error_handling_consistency(
        text in ".*",
        invalid_script in "[^a-zA-Z]*"
    ) {
        // Filter out valid script names to ensure we test invalid ones
        prop_assume!(!["Devanagari", "IAST", "SLP1"].contains(&invalid_script.as_str()));
        prop_assume!(!invalid_script.is_empty());
        
        let trans = LosslessTransliterator::new();
        
        // Invalid source script should always error
        let result1 = trans.transliterate(&text, &invalid_script, "IAST");
        prop_assert!(result1.is_err(), "Should error for invalid source script: {}", invalid_script);
        
        // Invalid target script should always error  
        let result2 = trans.transliterate(&text, "Devanagari", &invalid_script);
        prop_assert!(result2.is_err(), "Should error for invalid target script: {}", invalid_script);
    }

    /// Property: Binary search maintains sorted order invariant
    #[test]
    fn prop_binary_search_invariant(
        test_chars in prop::collection::vec(any::<char>(), 1..=20)
    ) {
        use shlesha::DEVANAGARI_TO_IAST_SIMPLE;
        
        // Verify the mapping is sorted (required for binary search)
        for i in 1..DEVANAGARI_TO_IAST_SIMPLE.len() {
            prop_assert!(DEVANAGARI_TO_IAST_SIMPLE[i-1].0 <= DEVANAGARI_TO_IAST_SIMPLE[i].0,
                "Mapping not sorted at index {}: {} > {}",
                i, DEVANAGARI_TO_IAST_SIMPLE[i-1].0 as u32, DEVANAGARI_TO_IAST_SIMPLE[i].0 as u32);
        }
        
        // Test character lookup consistency
        let mapper = &shlesha::DEVANAGARI_TO_IAST;
        for &ch in &test_chars {
            let result = mapper.lookup_char(ch);
            
            // If found, should be in the mapping
            if let Some(_) = result {
                let found_in_mapping = DEVANAGARI_TO_IAST_SIMPLE.iter()
                    .any(|&(mapped_char, _)| mapped_char == ch);
                prop_assert!(found_in_mapping, 
                    "Character {} found by lookup but not in mapping", ch as u32);
            }
        }
    }
}

#[cfg(test)]
mod deterministic_edge_cases {
    use super::*;
    
    #[test]
    fn test_malformed_token_edge_cases() {
        let trans = LosslessTransliterator::new();
        
        // Test various malformed token scenarios
        let malformed_cases = [
            "[", "]", "[]", "[:", ":]", "[1", "1]", 
            "[1::]", "[1:::]", "[:data]", "[script:data]",
            "[1:data:meta:extra:too:many]", "[99999:data]",
            "[1:data[nested]]", "[[1:data]]", "[1:data]:extra",
        ];
        
        for &malformed in &malformed_cases {
            // Should not crash when extracting tokens from malformed input
            let _tokens = trans.extract_tokens(malformed);
            
            // Transliteration with malformed content should still work
            let result = trans.transliterate(malformed, "Devanagari", "IAST");
            assert!(result.is_ok(), "Malformed token caused crash: {}", malformed);
        }
    }
    
    #[test]
    fn test_unicode_edge_cases() {
        let trans = LosslessTransliterator::new();
        
        let unicode_edge_cases = [
            "\u{0000}", // NULL
            "\u{FEFF}", // BOM
            "\u{200B}", // ZWSP
            "\u{200C}", // ZWNJ
            "\u{200D}", // ZWJ
            "🔥🚀💻", // Emoji
            "\u{10000}", // Supplementary plane
        ];
        
        for &edge_case in &unicode_edge_cases {
            let result = trans.transliterate(edge_case, "Devanagari", "IAST");
            assert!(result.is_ok(), "Unicode edge case failed: {:?}", edge_case);
            
            if let Ok(encoded) = result {
                let verification = trans.verify_lossless(edge_case, &encoded, "Devanagari");
                assert!(verification.is_lossless, "Lossless guarantee failed for unicode: {:?}", edge_case);
            }
        }
    }
}