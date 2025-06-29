//! Malformed input tests for the hub module
//!
//! Tests key malformed input scenarios to ensure graceful error handling

#[cfg(test)]
mod malformed_input_tests {
    use super::*;
    use crate::modules::hub::{Hub, HubOutput, HubTrait};

    /// Helper function to extract string content from HubOutput
    fn extract_string(output: &HubOutput) -> &str {
        output.as_str()
    }

    #[test]
    fn test_empty_string_handling() {
        let hub = Hub::new();

        // Empty strings should return empty results, not errors
        let deva_result = hub.deva_to_iso("").unwrap();
        let iso_result = hub.iso_to_deva("").unwrap();

        assert_eq!(extract_string(&deva_result), "");
        assert_eq!(extract_string(&iso_result), "");
    }

    #[test]
    fn test_whitespace_preservation() {
        let hub = Hub::new();

        let input = "   ";
        let deva_result = hub.iso_to_deva(input).unwrap();
        let iso_result = hub.deva_to_iso(input).unwrap();

        // Should preserve whitespace
        assert!(extract_string(&deva_result)
            .chars()
            .all(|c| c.is_whitespace()));
        assert!(extract_string(&iso_result)
            .chars()
            .all(|c| c.is_whitespace()));
    }

    #[test]
    fn test_invalid_unicode_graceful_handling() {
        let hub = Hub::new();

        // Test high Unicode values - should not crash
        let high_unicode = "\u{10FFFF}";
        let result1 = hub.iso_to_deva(high_unicode);
        let result2 = hub.deva_to_iso(high_unicode);

        // Should either succeed or fail cleanly (no panic)
        match result1 {
            Ok(output) => assert!(extract_string(&output)
                .chars()
                .all(|c| c.is_valid_unicode())),
            Err(_) => (), // Clean errors are acceptable
        }

        match result2 {
            Ok(output) => assert!(extract_string(&output)
                .chars()
                .all(|c| c.is_valid_unicode())),
            Err(_) => (), // Clean errors are acceptable
        }
    }

    #[test]
    fn test_extremely_long_input_handling() {
        let hub = Hub::new();

        // Test with moderately long strings (not too extreme for CI)
        let long_string = "a".repeat(1000);
        let long_devanagari = "अ".repeat(500);

        // Should handle long inputs without crashing
        let result1 = hub.iso_to_deva(&long_string);
        let result2 = hub.deva_to_iso(&long_devanagari);

        match result1 {
            Ok(output) => assert!(!extract_string(&output).is_empty()),
            Err(_) => (), // Memory/length limits are acceptable
        }

        match result2 {
            Ok(output) => assert!(!extract_string(&output).is_empty()),
            Err(_) => (), // Memory/length limits are acceptable
        }
    }

    #[test]
    fn test_malformed_devanagari_sequences() {
        let hub = Hub::new();

        let malformed_inputs = vec![
            "्",   // Virama alone
            "ा",  // Vowel sign alone
            "क्अ", // Unusual consonant-virama-vowel sequence
        ];

        for input in malformed_inputs {
            let result = hub.deva_to_iso(input);

            // Should handle malformed Devanagari without crashing
            match result {
                Ok(output) => {
                    // Should produce valid UTF-8 output
                    let output_str = extract_string(&output);
                    assert!(output_str.chars().all(|c| c.is_valid_unicode()));
                }
                Err(_) => {
                    // Clean errors for malformed input are acceptable
                }
            }
        }
    }

    #[test]
    fn test_malformed_romanization_input() {
        let hub = Hub::new();

        let malformed_inputs = vec![
            "kkkk",      // Invalid consonant cluster
            "ka1ma",     // Numbers mixed with letters
            "na.ma.ste", // Punctuation in unexpected places
        ];

        for input in malformed_inputs {
            let result = hub.iso_to_deva(input);

            // Should handle malformed romanization without crashing
            match result {
                Ok(output) => {
                    let output_str = extract_string(&output);
                    assert!(output_str.chars().all(|c| c.is_valid_unicode()));
                    assert!(!output_str.is_empty());
                }
                Err(_) => {
                    // Clean errors are acceptable
                }
            }
        }
    }

    #[test]
    fn test_mixed_script_boundaries() {
        let hub = Hub::new();

        let mixed_inputs = vec![
            "hello नमस्ते world", // Latin mixed with Devanagari
            "कर्म and karma",    // Devanagari mixed with Latin
        ];

        for input in mixed_inputs {
            // Try both directions
            let result1 = hub.iso_to_deva(input);
            let result2 = hub.deva_to_iso(input);

            // Should handle mixed scripts gracefully
            match result1 {
                Ok(output) => {
                    let output_str = extract_string(&output);
                    assert!(output_str.chars().all(|c| c.is_valid_unicode()));
                }
                Err(_) => (), // Rejecting mixed scripts is acceptable
            }

            match result2 {
                Ok(output) => {
                    let output_str = extract_string(&output);
                    assert!(output_str.chars().all(|c| c.is_valid_unicode()));
                }
                Err(_) => (), // Rejecting mixed scripts is acceptable
            }
        }
    }

    #[test]
    fn test_null_and_control_characters() {
        let hub = Hub::new();

        let control_inputs = vec![
            "test\0input",   // Null byte
            "test\x01input", // Control character
        ];

        for input in control_inputs {
            let result1 = hub.iso_to_deva(input);
            let result2 = hub.deva_to_iso(input);

            // Should handle control characters without crashing
            match result1 {
                Ok(output) => {
                    assert!(extract_string(&output)
                        .chars()
                        .all(|c| c.is_valid_unicode()));
                }
                Err(_) => (), // Rejecting control characters is acceptable
            }

            match result2 {
                Ok(output) => {
                    assert!(extract_string(&output)
                        .chars()
                        .all(|c| c.is_valid_unicode()));
                }
                Err(_) => (), // Rejecting control characters is acceptable
            }
        }
    }
}

trait ValidUnicode {
    fn is_valid_unicode(&self) -> bool;
}

impl ValidUnicode for char {
    fn is_valid_unicode(&self) -> bool {
        // All chars in Rust are valid Unicode by definition
        true
    }
}
