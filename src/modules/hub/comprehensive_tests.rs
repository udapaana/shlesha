#[cfg(test)]
mod comprehensive_tests {
    use super::*;
    use crate::modules::hub::{Hub, HubOutput, HubTrait};

    /// Test all vowels in both directions
    #[test]
    fn test_all_vowels_bidirectional() {
        let hub = Hub::new();

        let vowel_pairs = vec![
            ("अ", "a"),
            ("आ", "ā"),
            ("इ", "i"),
            ("ई", "ī"),
            ("उ", "u"),
            ("ऊ", "ū"),
            ("ऋ", "r̥"),
            ("ए", "e"),
            ("ओ", "o"),
            ("औ", "au"),
        ];

        for (deva, iso) in vowel_pairs {
            // Test Devanagari → ISO
            let result = hub.deva_to_iso(deva).unwrap();
            if let HubOutput::Iso(iso_result) = result {
                assert_eq!(iso_result, iso, "Failed: {} → {}", deva, iso);
            } else {
                panic!("Expected ISO output for {}", deva);
            }

            // Test ISO → Devanagari
            let result = hub.iso_to_deva(iso).unwrap();
            if let HubOutput::Devanagari(deva_result) = result {
                assert_eq!(deva_result, deva, "Failed: {} → {}", iso, deva);
            } else {
                panic!("Expected Devanagari output for {}", iso);
            }
        }
    }

    /// Test all consonants in both directions
    #[test]
    fn test_all_consonants_bidirectional() {
        let hub = Hub::new();

        let consonant_pairs = vec![
            // Velars
            ("क", "ka"),
            ("ख", "kha"),
            ("ग", "ga"),
            ("घ", "gha"),
            ("ङ", "ṅa"),
            // Palatals
            ("च", "ca"),
            ("छ", "cha"),
            ("ज", "ja"),
            ("झ", "jha"),
            ("ञ", "ña"),
            // Retroflexes
            ("ट", "ṭa"),
            ("ठ", "ṭha"),
            ("ड", "ḍa"),
            ("ढ", "ḍha"),
            ("ण", "ṇa"),
            // Dentals
            ("त", "ta"),
            ("थ", "tha"),
            ("द", "da"),
            ("ध", "dha"),
            ("न", "na"),
            // Labials
            ("प", "pa"),
            ("फ", "pha"),
            ("ब", "ba"),
            ("भ", "bha"),
            ("म", "ma"),
            // Semivowels
            ("य", "ya"),
            ("र", "ra"),
            ("ल", "la"),
            ("व", "va"),
            // Sibilants
            ("श", "śa"),
            ("ष", "ṣa"),
            ("स", "sa"),
            // Aspirate
            ("ह", "ha"),
        ];

        for (deva, iso) in consonant_pairs {
            // Test Devanagari → ISO
            let result = hub.deva_to_iso(deva).unwrap();
            if let HubOutput::Iso(iso_result) = result {
                assert_eq!(iso_result, iso, "Failed: {} → {}", deva, iso);
            } else {
                panic!("Expected ISO output for {}", deva);
            }

            // Test ISO → Devanagari
            let result = hub.iso_to_deva(iso).unwrap();
            if let HubOutput::Devanagari(deva_result) = result {
                assert_eq!(deva_result, deva, "Failed: {} → {}", iso, deva);
            } else {
                panic!("Expected Devanagari output for {}", iso);
            }
        }
    }

    /// Test all vowel signs (mātrās) - forward direction only due to ambiguity with independent vowels
    #[test]
    fn test_all_vowel_signs_forward() {
        let hub = Hub::new();

        let matra_pairs = vec![
            ("ा", "ā"),  // ā-mātrā (ambiguous with आ)
            ("ि", "i"),  // i-mātrā (ambiguous with इ)
            ("ी", "ī"),  // ī-mātrā (ambiguous with ई)
            ("ु", "u"),   // u-mātrā (ambiguous with उ)
            ("ू", "ū"),   // ū-mātrā (ambiguous with ऊ)
            ("ृ", "r̥"),   // r̥-mātrā (ambiguous with ऋ)
            ("े", "e"),   // e-mātrā (ambiguous with ए)
            ("ो", "o"),  // o-mātrā (ambiguous with ओ)
            ("ौ", "au"), // au-mātrā (ambiguous with औ)
        ];

        for (deva, iso) in matra_pairs {
            // Test Devanagari → ISO (mātrās should convert correctly)
            let result = hub.deva_to_iso(deva).unwrap();
            if let HubOutput::Iso(iso_result) = result {
                assert_eq!(iso_result, iso, "Failed: {} → {}", deva, iso);
            } else {
                panic!("Expected ISO output for {}", deva);
            }
        }
    }

    /// Test that ISO → Devanagari prefers independent vowels (by design)
    #[test]
    fn test_iso_prefers_independent_vowels() {
        let hub = Hub::new();

        let vowel_pairs = vec![
            ("ā", "आ"),  // Should prefer आ over ा
            ("i", "इ"),  // Should prefer इ over ि
            ("ī", "ई"),  // Should prefer ई over ी
            ("u", "उ"),  // Should prefer उ over ु
            ("ū", "ऊ"),  // Should prefer ऊ over ू
            ("r̥", "ऋ"),  // Should prefer ऋ over ृ
            ("e", "ए"),  // Should prefer ए over े
            ("o", "ओ"),  // Should prefer ओ over ो
            ("au", "औ"), // Should prefer औ over ौ
        ];

        for (iso, expected_deva) in vowel_pairs {
            let result = hub.iso_to_deva(iso).unwrap();
            if let HubOutput::Devanagari(deva_result) = result {
                assert_eq!(
                    deva_result, expected_deva,
                    "Failed: {} → {} (expected independent vowel)",
                    iso, expected_deva
                );
            } else {
                panic!("Expected Devanagari output for {}", iso);
            }
        }
    }

    /// Test special marks in both directions
    #[test]
    fn test_special_marks_bidirectional() {
        let hub = Hub::new();

        let special_pairs = vec![
            ("ं", "ṁ"), // anusvara
            ("ः", "ḥ"), // visarga
                       // Note: virama ("्", "") is tested separately due to special handling
        ];

        for (deva, iso) in special_pairs {
            // Test Devanagari → ISO
            let result = hub.deva_to_iso(deva).unwrap();
            if let HubOutput::Iso(iso_result) = result {
                assert_eq!(iso_result, iso, "Failed: {} → {}", deva, iso);
            } else {
                panic!("Expected ISO output for {}", deva);
            }

            // Test ISO → Devanagari
            let result = hub.iso_to_deva(iso).unwrap();
            if let HubOutput::Devanagari(deva_result) = result {
                assert_eq!(deva_result, deva, "Failed: {} → {}", iso, deva);
            } else {
                panic!("Expected Devanagari output for {}", iso);
            }
        }
    }

    /// Test roundtrip conversion for characters with unambiguous mappings
    #[test]
    fn test_unambiguous_roundtrip_conversion() {
        let hub = Hub::new();

        // Only test characters that have unambiguous roundtrip behavior
        let unambiguous_chars = vec![
            // Independent vowels (these should roundtrip correctly)
            "अ", "आ", "इ", "ई", "उ", "ऊ", "ऋ", "ए", "ओ", "औ",
            // All consonants (these should roundtrip correctly)
            "क", "ख", "ग", "घ", "ङ", "च", "छ", "ज", "झ", "ञ", "ट", "ठ", "ड", "ढ", "ण", "त", "थ",
            "द", "ध", "न", "प", "फ", "ब", "भ", "म", "य", "र", "ल", "व", "श", "ष", "स", "ह",
            // Special marks
            "ं", "ः", "।",
        ];

        for original_deva in unambiguous_chars {
            // Roundtrip: Deva → ISO → Deva
            let to_iso = hub.deva_to_iso(original_deva).unwrap();
            if let HubOutput::Iso(iso_text) = to_iso {
                let back_to_deva = hub.iso_to_deva(&iso_text).unwrap();
                if let HubOutput::Devanagari(final_deva) = back_to_deva {
                    assert_eq!(
                        final_deva, original_deva,
                        "Roundtrip failed: {} → {} → {}",
                        original_deva, iso_text, final_deva
                    );
                } else {
                    panic!(
                        "Expected Devanagari output in roundtrip for {}",
                        original_deva
                    );
                }
            } else {
                panic!("Expected ISO output in roundtrip for {}", original_deva);
            }
        }
    }

    /// Test that vowel signs have expected forward-only behavior
    #[test]
    fn test_vowel_sign_behavior() {
        let hub = Hub::new();

        // Vowel signs should convert to ISO but ISO should prefer independent vowels
        let vowel_signs = vec!["ा", "ि", "ी", "ु", "ू", "ृ", "े", "ो", "ौ"];

        for sign in vowel_signs {
            // Forward conversion should work
            let to_iso = hub.deva_to_iso(sign).unwrap();
            if let HubOutput::Iso(iso_text) = to_iso {
                // Reverse conversion should give independent vowel, not sign
                let back_to_deva = hub.iso_to_deva(&iso_text).unwrap();
                if let HubOutput::Devanagari(final_deva) = back_to_deva {
                    // Should NOT be the same as original sign
                    assert_ne!(
                        final_deva, sign,
                        "Vowel sign {} should not roundtrip to itself (got {})",
                        sign, final_deva
                    );
                }
            }
        }
    }

    /// Test virama handling with all consonants
    #[test]
    fn test_virama_with_all_consonants() {
        let hub = Hub::new();

        let consonants = vec![
            ("क", "k"),
            ("ख", "kh"),
            ("ग", "g"),
            ("घ", "gh"),
            ("ङ", "ṅ"),
            ("च", "c"),
            ("छ", "ch"),
            ("ज", "j"),
            ("झ", "jh"),
            ("ञ", "ñ"),
            ("ट", "ṭ"),
            ("ठ", "ṭh"),
            ("ड", "ḍ"),
            ("ढ", "ḍh"),
            ("ण", "ṇ"),
            ("त", "t"),
            ("थ", "th"),
            ("द", "d"),
            ("ध", "dh"),
            ("न", "n"),
            ("प", "p"),
            ("फ", "ph"),
            ("ब", "b"),
            ("भ", "bh"),
            ("म", "m"),
            ("य", "y"),
            ("र", "r"),
            ("ल", "l"),
            ("व", "v"),
            ("श", "ś"),
            ("ष", "ṣ"),
            ("स", "s"),
            ("ह", "h"),
        ];

        for (consonant, expected_bare) in consonants {
            let with_virama = format!("{}्", consonant);

            // Test consonant + virama → bare consonant
            let result = hub.deva_to_iso(&with_virama).unwrap();
            if let HubOutput::Iso(iso_result) = result {
                assert_eq!(
                    iso_result, expected_bare,
                    "Failed virama: {} → {}",
                    with_virama, expected_bare
                );
            } else {
                panic!("Expected ISO output for {}", with_virama);
            }
        }
    }

    /// Test complex words and phrases
    #[test]
    fn test_complex_words_bidirectional() {
        let hub = Hub::new();

        let word_pairs = vec![
            ("धर्म", "dharma"),    // dharma (religion)
            ("कर्म", "karma"),     // karma (action)
            ("अर्थ", "artha"),     // artha (meaning)
            ("मोक्ष", "mokṣa"),    // moksha (liberation)
            ("सत्य", "satya"),     // truth
            ("अहिंसा", "ahiṁsā"),  // non-violence
            ("संस्कृत", "saṁskr̥ta"), // Sanskrit
            ("भारत", "bhārata"),  // India
            ("गुरु", "guru"),       // teacher
            ("योग", "yoga"),      // yoga
        ];

        for (deva_word, iso_word) in word_pairs {
            // Test Devanagari → ISO
            let result = hub.deva_to_iso(deva_word).unwrap();
            if let HubOutput::Iso(iso_result) = result {
                assert_eq!(iso_result, iso_word, "Failed: {} → {}", deva_word, iso_word);
            } else {
                panic!("Expected ISO output for {}", deva_word);
            }

            // Test ISO → Devanagari
            let result = hub.iso_to_deva(iso_word).unwrap();
            if let HubOutput::Devanagari(deva_result) = result {
                assert_eq!(
                    deva_result, deva_word,
                    "Failed: {} → {}",
                    iso_word, deva_word
                );
            } else {
                panic!("Expected Devanagari output for {}", iso_word);
            }

            // Test roundtrip
            let roundtrip_result = hub.deva_to_iso(deva_word).unwrap();
            if let HubOutput::Iso(iso_intermediate) = roundtrip_result {
                let back_result = hub.iso_to_deva(&iso_intermediate).unwrap();
                if let HubOutput::Devanagari(final_deva) = back_result {
                    assert_eq!(
                        final_deva, deva_word,
                        "Roundtrip failed: {} → {} → {}",
                        deva_word, iso_intermediate, final_deva
                    );
                }
            }
        }
    }

    /// Test edge cases and boundary conditions
    #[test]
    fn test_edge_cases() {
        let hub = Hub::new();

        // Empty strings
        let empty_deva = hub.deva_to_iso("").unwrap();
        if let HubOutput::Iso(result) = empty_deva {
            assert_eq!(result, "");
        }

        let empty_iso = hub.iso_to_deva("").unwrap();
        if let HubOutput::Devanagari(result) = empty_iso {
            assert_eq!(result, "");
        }

        // Whitespace preservation
        let spaced_deva = hub.deva_to_iso("क म").unwrap();
        if let HubOutput::Iso(result) = spaced_deva {
            assert_eq!(result, "ka ma");
        }

        let spaced_iso = hub.iso_to_deva("ka ma").unwrap();
        if let HubOutput::Devanagari(result) = spaced_iso {
            assert_eq!(result, "क म");
        }

        // Punctuation preservation
        let punct_deva = hub.deva_to_iso("क, म।").unwrap();
        if let HubOutput::Iso(result) = punct_deva {
            assert_eq!(result, "ka, ma।");
        }

        // Multiple spaces
        let multi_space_deva = hub.deva_to_iso("क   म").unwrap();
        if let HubOutput::Iso(result) = multi_space_deva {
            assert_eq!(result, "ka   ma");
        }
    }

    /// Test mathematical completeness accounting for natural script ambiguities
    #[test]
    fn test_mapping_completeness() {
        let hub = Hub::new();

        // Count mappings in each direction
        let deva_to_iso_count = hub.deva_to_iso_map.len();
        let iso_to_deva_count = hub.iso_to_deva_map.len();

        // Devanagari has more characters due to vowel signs vs independent vowels
        // This is natural and expected - not all characters can have bijective mapping
        assert!(deva_to_iso_count > iso_to_deva_count,
            "Deva→ISO count ({}) should be greater than ISO→Deva count ({}) due to vowel sign ambiguity", 
            deva_to_iso_count, iso_to_deva_count);

        // Verify virama is present in deva_to_iso but not in iso_to_deva
        assert!(hub.deva_to_iso_map.contains_key(&'्'));
        assert_eq!(hub.deva_to_iso_map[&'्'], "");
        assert!(!hub.iso_to_deva_map.contains_key(""));

        // All ISO→Deva mappings should have corresponding Deva→ISO mappings
        for (&iso_str, &deva_char) in &hub.iso_to_deva_map {
            assert!(
                hub.deva_to_iso_map.contains_key(&deva_char),
                "Missing forward mapping for: {} → {}",
                iso_str,
                deva_char
            );
            assert_eq!(
                hub.deva_to_iso_map[&deva_char], iso_str,
                "Inconsistent forward mapping for: {} → {}",
                iso_str, deva_char
            );
        }

        // Verify ambiguous mappings are handled correctly
        // These ISO sequences should map to independent vowels, not vowel signs
        let ambiguous_iso = vec!["ā", "i", "ī", "u", "ū", "r̥", "e", "o", "au"];
        for iso in ambiguous_iso {
            if let Some(&deva_char) = hub.iso_to_deva_map.get(iso) {
                // Should be independent vowel (U+0905-U+0914), not vowel sign (U+093E-U+094C)
                let unicode_val = deva_char as u32;
                assert!(
                    unicode_val >= 0x0905 && unicode_val <= 0x0914,
                    "ISO '{}' should map to independent vowel, got {} (U+{:04X})",
                    iso,
                    deva_char,
                    unicode_val
                );
            }
        }
    }
}
