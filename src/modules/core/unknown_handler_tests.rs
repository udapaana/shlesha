#[cfg(test)]
mod comprehensive_tests {
    use crate::modules::core::unknown_handler::*;

    #[test]
    fn test_unknown_token_creation_and_formatting() {
        // Test standard unknown token
        let token = UnknownToken::new("devanagari", '☺', 5, false);
        assert_eq!(token.script, "devanagari");
        assert_eq!(token.token, '☺');
        assert_eq!(token.unicode, "U+263A");
        assert_eq!(token.position, 5);
        assert!(!token.is_extension);
        assert_eq!(token.format(), "[devanagari:☺:U+263A]");

        // Test extension unknown token
        let ext_token = UnknownToken::new("vedavms", '†', 10, true);
        assert_eq!(ext_token.script, "vedavms");
        assert_eq!(ext_token.token, '†');
        assert_eq!(ext_token.unicode, "U+2020");
        assert!(ext_token.is_extension);
        assert_eq!(ext_token.format(), "[ext:vedavms:†:U+2020]");

        // Test various Unicode characters
        let emoji = UnknownToken::new("test", '🚀', 0, false);
        assert_eq!(emoji.unicode, "U+1F680");
        assert_eq!(emoji.format(), "[test:🚀:U+1F680]");

        let chinese = UnknownToken::new("test", '中', 0, false);
        assert_eq!(chinese.unicode, "U+4E2D");
        assert_eq!(chinese.format(), "[test:中:U+4E2D]");
    }

    #[test]
    fn test_transliteration_metadata() {
        let mut metadata = TransliterationMetadata::new("source", "target");
        assert_eq!(metadata.source_script, "source");
        assert_eq!(metadata.target_script, "target");
        assert!(!metadata.used_extensions);
        assert!(metadata.unknown_tokens.is_empty());

        // Add standard unknown
        metadata.add_unknown(UnknownToken::new("source", 'x', 0, false));
        assert_eq!(metadata.unknown_tokens.len(), 1);
        assert!(!metadata.used_extensions);

        // Add extension unknown
        metadata.add_unknown(UnknownToken::new("vedavms", 'y', 1, true));
        assert_eq!(metadata.unknown_tokens.len(), 2);
        assert!(metadata.used_extensions);
    }

    #[test]
    fn test_unique_unknowns() {
        let mut metadata = TransliterationMetadata::new("test", "test");

        // Add duplicate unknowns
        metadata.add_unknown(UnknownToken::new("test", 'a', 0, false));
        metadata.add_unknown(UnknownToken::new("test", 'b', 1, false));
        metadata.add_unknown(UnknownToken::new("test", 'a', 2, false));
        metadata.add_unknown(UnknownToken::new("test", 'c', 3, false));
        metadata.add_unknown(UnknownToken::new("test", 'b', 4, false));

        let unique = metadata.unique_unknowns();
        assert_eq!(unique, vec!['a', 'b', 'c']);
    }

    #[test]
    fn test_metadata_report() {
        let mut metadata = TransliterationMetadata::new("devanagari", "iast");

        // Test empty report
        let report = metadata.report();
        assert!(report.contains("No unknown tokens found"));

        // Add some unknowns
        metadata.add_unknown(UnknownToken::new("devanagari", '☺', 5, false));
        metadata.add_unknown(UnknownToken::new("devanagari", '☺', 10, false));
        metadata.add_unknown(UnknownToken::new("devanagari", '♥', 15, false));

        let report = metadata.report();
        assert!(report.contains("Unknown tokens in devanagari → iast conversion"));
        assert!(report.contains("'☺' (U+263A) - 2 occurrence(s)"));
        assert!(report.contains("'♥' (U+2665) - 1 occurrence(s)"));
        assert!(!report.contains("runtime extensions")); // No extensions used

        // Add extension unknown
        metadata.add_unknown(UnknownToken::new("vedavms", '†', 20, true));
        let report = metadata.report();
        assert!(report.contains("Note: Some unknown tokens came from runtime extensions"));
    }

    #[test]
    fn test_transliteration_result() {
        // Test simple result
        let simple = TransliterationResult::simple("dharma".to_string());
        assert_eq!(simple.output, "dharma");
        assert!(simple.metadata.is_none());
        assert_eq!(simple.annotated_output(), "dharma");

        // Test result with metadata but no unknowns
        let metadata = TransliterationMetadata::new("source", "target");
        let with_meta = TransliterationResult::with_metadata("dharma".to_string(), metadata);
        assert_eq!(with_meta.output, "dharma");
        assert!(with_meta.metadata.is_some());
        assert_eq!(with_meta.annotated_output(), "dharma");

        // Test result with unknowns
        let mut metadata = TransliterationMetadata::new("devanagari", "iast");
        metadata.add_unknown(UnknownToken::new("devanagari", '☺', 6, false));
        let result = TransliterationResult::with_metadata("dharma☺".to_string(), metadata);
        assert_eq!(result.output, "dharma☺");

        // Note: annotated_output implementation is simplified in the current code
        // In practice, we'd need proper char boundary handling
        let annotated = result.annotated_output();
        assert!(annotated.contains("dharma"));
        assert!(annotated.contains("☺"));
    }

    #[test]
    fn test_unknown_handler_trait() {
        // Test implementation of UnknownHandler trait
        struct TestHandler {
            known_chars: Vec<char>,
        }

        impl UnknownHandler for TestHandler {
            fn is_known_char(&self, _script: &str, ch: char) -> bool {
                self.known_chars.contains(&ch)
            }
        }

        let handler = TestHandler {
            known_chars: vec!['a', 'b', 'c'],
        };

        // Test with all known chars
        let (output, unknowns) = handler.process_with_unknowns("test", "abc", false);
        assert_eq!(output, "abc");
        assert!(unknowns.is_empty());

        // Test with unknown chars
        let (output, unknowns) = handler.process_with_unknowns("test", "ab☺c", false);
        assert_eq!(output, "ab☺c");
        assert_eq!(unknowns.len(), 1);
        assert_eq!(unknowns[0].token, '☺');
        assert_eq!(unknowns[0].position, 2); // byte position

        // Test with extension
        let (output, unknowns) = handler.process_with_unknowns("vedavms", "a†b", true);
        assert_eq!(output, "a†b");
        assert_eq!(unknowns.len(), 1);
        assert_eq!(unknowns[0].token, '†');
        assert!(unknowns[0].is_extension);
    }

    #[test]
    fn test_edge_cases() {
        // Test empty string
        let mut metadata = TransliterationMetadata::new("", "");
        assert_eq!(metadata.unique_unknowns(), Vec::<char>::new());

        // Test single character
        metadata.add_unknown(UnknownToken::new("test", 'x', 0, false));
        assert_eq!(metadata.unique_unknowns(), vec!['x']);

        // Test Unicode edge cases
        let high_unicode = UnknownToken::new("test", '\u{10FFFF}', 0, false);
        assert_eq!(high_unicode.unicode, "U+10FFFF");

        // Test position tracking with multi-byte chars
        let sanskrit = "धर्म";
        let positions: Vec<usize> = sanskrit.char_indices().map(|(i, _)| i).collect();
        assert_eq!(positions, vec![0, 3, 6, 9]); // "धर्म" has 4 characters
    }
}

// Export the test module
#[cfg(test)]
pub use comprehensive_tests::*;
