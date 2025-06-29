#[cfg(test)]
mod correctness_tests {
    use crate::modules::script_converter::{
        HarvardKyotoConverter, IastConverter, ItransConverter, Slp1Converter, VelthuisConverter,
        WxConverter,
    };

    /// Test that romanization schemes map correctly without adding inherent vowels
    /// Both ITRANS and ISO-15919 are explicit about vowels - no implicit 'a'
    #[test]
    fn test_itrans_consonant_mapping_correctness() {
        let converter = ItransConverter::new();

        // Basic consonants should map without inherent 'a'
        let consonant_tests = vec![
            ("k", "k"), // not "ka"
            ("g", "g"), // not "ga"
            ("c", "c"), // not "ca"
            ("j", "j"), // not "ja"
            ("t", "t"), // not "ta"
            ("d", "d"), // not "da"
            ("p", "p"), // not "pa"
            ("b", "b"), // not "ba"
            ("m", "m"), // not "ma"
            ("y", "y"), // not "ya"
            ("r", "r"), // not "ra"
            ("l", "l"), // not "la"
            ("v", "v"), // not "va"
            ("s", "s"), // not "sa"
            ("h", "h"), // not "ha"
        ];

        for (itrans, expected_iso) in consonant_tests {
            let result = converter.itrans_to_iso(itrans).unwrap();
            assert_eq!(
                result, expected_iso,
                "ITRANS '{}' should map to ISO '{}', got '{}'",
                itrans, expected_iso, result
            );
        }
    }

    #[test]
    fn test_itrans_aspirated_consonants() {
        let converter = ItransConverter::new();

        let aspirated_tests = vec![
            ("kh", "kh"), // not "kha"
            ("gh", "gh"), // not "gha"
            ("ch", "ch"), // not "cha"
            ("jh", "jh"), // not "jha"
            ("th", "th"), // not "tha"
            ("dh", "dh"), // not "dha"
            ("ph", "ph"), // not "pha"
            ("bh", "bh"), // not "bha"
        ];

        for (itrans, expected_iso) in aspirated_tests {
            let result = converter.itrans_to_iso(itrans).unwrap();
            assert_eq!(
                result, expected_iso,
                "ITRANS '{}' should map to ISO '{}', got '{}'",
                itrans, expected_iso, result
            );
        }
    }

    #[test]
    fn test_itrans_retroflex_consonants() {
        let converter = ItransConverter::new();

        let retroflex_tests = vec![
            ("T", "ṭ"),   // not "ṭa"
            ("Th", "ṭh"), // not "ṭha" - this was your correction!
            ("D", "ḍ"),   // not "ḍa"
            ("Dh", "ḍh"), // not "ḍha"
            ("N", "ṇ"),   // not "ṇa"
        ];

        for (itrans, expected_iso) in retroflex_tests {
            let result = converter.itrans_to_iso(itrans).unwrap();
            assert_eq!(
                result, expected_iso,
                "ITRANS '{}' should map to ISO '{}', got '{}'",
                itrans, expected_iso, result
            );
        }
    }

    #[test]
    fn test_itrans_sibilants() {
        let converter = ItransConverter::new();

        let sibilant_tests = vec![
            ("sh", "ś"), // not "śa" - this was your correction!
            ("Sh", "ṣ"), // not "ṣa"
        ];

        for (itrans, expected_iso) in sibilant_tests {
            let result = converter.itrans_to_iso(itrans).unwrap();
            assert_eq!(
                result, expected_iso,
                "ITRANS '{}' should map to ISO '{}', got '{}'",
                itrans, expected_iso, result
            );
        }
    }

    #[test]
    fn test_itrans_nasals() {
        let converter = ItransConverter::new();

        let nasal_tests = vec![
            ("~N", "ṅ"), // not "ṅa"
            ("N^", "ṅ"), // alternative
            ("~n", "ñ"), // not "ña"
            ("JN", "ñ"), // alternative
        ];

        for (itrans, expected_iso) in nasal_tests {
            let result = converter.itrans_to_iso(itrans).unwrap();
            assert_eq!(
                result, expected_iso,
                "ITRANS '{}' should map to ISO '{}', got '{}'",
                itrans, expected_iso, result
            );
        }
    }

    #[test]
    fn test_itrans_with_explicit_vowels() {
        let converter = ItransConverter::new();

        // When vowels are explicit, they should be preserved
        let vowel_tests = vec![
            ("ka", "ka"), // k + a
            ("ki", "ki"), // k + i
            ("ku", "ku"), // k + u
            ("ke", "ke"), // k + e
            ("ko", "ko"), // k + o
            ("kA", "kā"), // k + ā
            ("kI", "kī"), // k + ī
            ("kU", "kū"), // k + ū
        ];

        for (itrans, expected_iso) in vowel_tests {
            let result = converter.itrans_to_iso(itrans).unwrap();
            assert_eq!(
                result, expected_iso,
                "ITRANS '{}' should map to ISO '{}', got '{}'",
                itrans, expected_iso, result
            );
        }
    }

    #[test]
    fn test_real_world_itrans_words() {
        let converter = ItransConverter::new();

        // Real ITRANS words should convert correctly
        let word_tests = vec![
            ("namaste", "namaste"), // simple word
            ("dharma", "dharma"),   // with 'r'
            ("shanti", "śanti"),    // with 'sh' → 'ś'
            ("Shiva", "ṣiva"),      // with 'Sh' → 'ṣ'
            ("kRShNa", "kr̥ṣṇa"),    // complex with 'k' + 'R' + 'Sh' + 'N' + 'a'
        ];

        for (itrans, expected_iso) in word_tests {
            let result = converter.itrans_to_iso(itrans).unwrap();
            assert_eq!(
                result, expected_iso,
                "ITRANS word '{}' should map to ISO '{}', got '{}'",
                itrans, expected_iso, result
            );
        }
    }

    #[test]
    fn test_slp1_consonant_mapping_correctness() {
        let converter = Slp1Converter::new();

        // SLP1 consonants should also map without inherent 'a'
        let consonant_tests = vec![
            ("k", "k"),  // not "ka"
            ("K", "kh"), // not "kha"
            ("g", "g"),  // not "ga"
            ("G", "gh"), // not "gha"
            ("c", "c"),  // not "ca"
            ("C", "ch"), // not "cha"
            ("j", "j"),  // not "ja"
            ("J", "jh"), // not "jha"
            ("w", "ṭ"),  // not "ṭa"
            ("W", "ṭh"), // not "ṭha"
            ("q", "ḍ"),  // not "ḍa"
            ("Q", "ḍh"), // not "ḍha"
            ("t", "t"),  // not "ta"
            ("T", "th"), // not "tha"
            ("d", "d"),  // not "da"
            ("D", "dh"), // not "dha"
            ("p", "p"),  // not "pa"
            ("P", "ph"), // not "pha"
            ("b", "b"),  // not "ba"
            ("B", "bh"), // not "bha"
            ("m", "m"),  // not "ma"
            ("y", "y"),  // not "ya"
            ("r", "r"),  // not "ra"
            ("l", "l"),  // not "la"
            ("v", "v"),  // not "va"
            ("S", "ś"),  // not "śa"
            ("z", "ṣ"),  // not "ṣa"
            ("s", "s"),  // not "sa"
            ("h", "h"),  // not "ha"
        ];

        for (slp1, expected_iso) in consonant_tests {
            let result = converter.slp1_to_iso(slp1).unwrap();
            assert_eq!(
                result, expected_iso,
                "SLP1 '{}' should map to ISO '{}', got '{}'",
                slp1, expected_iso, result
            );
        }
    }

    #[test]
    fn test_iast_consonant_mapping_correctness() {
        let converter = IastConverter::new();

        // IAST consonants should also map without inherent 'a'
        let consonant_tests = vec![
            ("k", "k"), // not "ka"
            ("g", "g"), // not "ga"
            ("c", "c"), // not "ca"
            ("j", "j"), // not "ja"
            ("t", "t"), // not "ta"
            ("d", "d"), // not "da"
            ("p", "p"), // not "pa"
            ("b", "b"), // not "ba"
            ("m", "m"), // not "ma"
            ("y", "y"), // not "ya"
            ("r", "r"), // not "ra"
            ("l", "l"), // not "la"
            ("v", "v"), // not "va"
            ("ś", "ś"), // identical
            ("ṣ", "ṣ"), // identical
            ("s", "s"), // not "sa"
            ("h", "h"), // not "ha"
        ];

        for (iast, expected_iso) in consonant_tests {
            let result = converter.iast_to_iso(iast).unwrap();
            assert_eq!(
                result, expected_iso,
                "IAST '{}' should map to ISO '{}', got '{}'",
                iast, expected_iso, result
            );
        }
    }

    #[test]
    fn test_harvard_kyoto_consonant_mapping_correctness() {
        let converter = HarvardKyotoConverter::new();

        // Harvard-Kyoto consonants should also map without inherent 'a'
        let consonant_tests = vec![
            ("k", "k"),   // not "ka"
            ("kh", "kh"), // not "kha"
            ("g", "g"),   // not "ga"
            ("gh", "gh"), // not "gha"
            ("G", "ṅ"),   // not "ṅa"
            ("c", "c"),   // not "ca"
            ("ch", "ch"), // not "cha"
            ("j", "j"),   // not "ja"
            ("jh", "jh"), // not "jha"
            ("J", "ñ"),   // not "ña"
            ("T", "ṭ"),   // not "ṭa"
            ("Th", "ṭh"), // not "ṭha"
            ("D", "ḍ"),   // not "ḍa"
            ("Dh", "ḍh"), // not "ḍha"
            ("N", "ṇ"),   // not "ṇa"
            ("t", "t"),   // not "ta"
            ("th", "th"), // not "tha"
            ("d", "d"),   // not "da"
            ("dh", "dh"), // not "dha"
            ("n", "n"),   // not "na"
            ("p", "p"),   // not "pa"
            ("ph", "ph"), // not "pha"
            ("b", "b"),   // not "ba"
            ("bh", "bh"), // not "bha"
            ("m", "m"),   // not "ma"
            ("y", "y"),   // not "ya"
            ("r", "r"),   // not "ra"
            ("l", "l"),   // not "la"
            ("v", "v"),   // not "va"
            ("z", "ś"),   // not "śa" - HK z → ISO ś
            ("S", "ṣ"),   // not "ṣa" - HK S → ISO ṣ
            ("s", "s"),   // not "sa"
            ("h", "h"),   // not "ha"
            ("L", "ḷ"),   // not "ḷa"
        ];

        for (hk, expected_iso) in consonant_tests {
            let result = converter.hk_to_iso(hk).unwrap();
            assert_eq!(
                result, expected_iso,
                "Harvard-Kyoto '{}' should map to ISO '{}', got '{}'",
                hk, expected_iso, result
            );
        }
    }

    #[test]
    fn test_harvard_kyoto_retroflex_consonants() {
        let converter = HarvardKyotoConverter::new();

        // Harvard-Kyoto uses capital letters for retroflex consonants
        let retroflex_tests = vec![
            ("T", "ṭ"),   // not "ṭa"
            ("Th", "ṭh"), // not "ṭha"
            ("D", "ḍ"),   // not "ḍa"
            ("Dh", "ḍh"), // not "ḍha"
            ("N", "ṇ"),   // not "ṇa"
            ("L", "ḷ"),   // not "ḷa"
        ];

        for (hk, expected_iso) in retroflex_tests {
            let result = converter.hk_to_iso(hk).unwrap();
            assert_eq!(
                result, expected_iso,
                "Harvard-Kyoto retroflex '{}' should map to ISO '{}', got '{}'",
                hk, expected_iso, result
            );
        }
    }

    #[test]
    fn test_harvard_kyoto_aspirated_consonants() {
        let converter = HarvardKyotoConverter::new();

        // Harvard-Kyoto aspirated consonants should not have inherent 'a'
        let aspirated_tests = vec![
            ("kh", "kh"), // not "kha"
            ("gh", "gh"), // not "gha"
            ("ch", "ch"), // not "cha"
            ("jh", "jh"), // not "jha"
            ("Th", "ṭh"), // not "ṭha"
            ("Dh", "ḍh"), // not "ḍha"
            ("th", "th"), // not "tha"
            ("dh", "dh"), // not "dha"
            ("ph", "ph"), // not "pha"
            ("bh", "bh"), // not "bha"
        ];

        for (hk, expected_iso) in aspirated_tests {
            let result = converter.hk_to_iso(hk).unwrap();
            assert_eq!(
                result, expected_iso,
                "Harvard-Kyoto aspirated '{}' should map to ISO '{}', got '{}'",
                hk, expected_iso, result
            );
        }
    }

    #[test]
    fn test_harvard_kyoto_nasals() {
        let converter = HarvardKyotoConverter::new();

        // Harvard-Kyoto nasal consonants should not have inherent 'a'
        let nasal_tests = vec![
            ("G", "ṅ"), // not "ṅa" - HK G → ISO ṅ
            ("J", "ñ"), // not "ña" - HK J → ISO ñ
            ("N", "ṇ"), // not "ṇa" - HK N → ISO ṇ
            ("n", "n"), // not "na"
            ("m", "m"), // not "ma"
        ];

        for (hk, expected_iso) in nasal_tests {
            let result = converter.hk_to_iso(hk).unwrap();
            assert_eq!(
                result, expected_iso,
                "Harvard-Kyoto nasal '{}' should map to ISO '{}', got '{}'",
                hk, expected_iso, result
            );
        }
    }

    #[test]
    fn test_harvard_kyoto_sibilants() {
        let converter = HarvardKyotoConverter::new();

        // Harvard-Kyoto sibilants should not have inherent 'a'
        let sibilant_tests = vec![
            ("z", "ś"), // not "śa" - HK z → ISO ś
            ("S", "ṣ"), // not "ṣa" - HK S → ISO ṣ
            ("s", "s"), // not "sa"
        ];

        for (hk, expected_iso) in sibilant_tests {
            let result = converter.hk_to_iso(hk).unwrap();
            assert_eq!(
                result, expected_iso,
                "Harvard-Kyoto sibilant '{}' should map to ISO '{}', got '{}'",
                hk, expected_iso, result
            );
        }
    }

    #[test]
    fn test_harvard_kyoto_with_explicit_vowels() {
        let converter = HarvardKyotoConverter::new();

        // Test that explicit consonant+vowel combinations work correctly
        let cv_tests = vec![
            ("ka", "ka"), // consonant + vowel should have the vowel
            ("ki", "ki"),
            ("ku", "ku"),
            ("kA", "kā"), // HK A → ISO ā
            ("kI", "kī"), // HK I → ISO ī
            ("kU", "kū"), // HK U → ISO ū
            ("kR", "kr̥"), // HK R → ISO r̥
            ("ke", "ke"),
            ("ko", "ko"),
            ("kai", "kai"),
            ("kau", "kau"),
        ];

        for (hk, expected_iso) in cv_tests {
            let result = converter.hk_to_iso(hk).unwrap();
            assert_eq!(
                result, expected_iso,
                "Harvard-Kyoto '{}' should map to ISO '{}', got '{}'",
                hk, expected_iso, result
            );
        }
    }

    #[test]
    fn test_real_world_harvard_kyoto_words() {
        let converter = HarvardKyotoConverter::new();

        // Real Harvard-Kyoto words should convert correctly
        let word_tests = vec![
            ("namaste", "namaste"), // simple word
            ("dharma", "dharma"),   // with 'r'
            ("zAnti", "śānti"),     // with 'z' → 'ś' and 'A' → 'ā'
            ("Siva", "ṣiva"),       // with 'S' → 'ṣ'
            ("kRSNa", "kr̥ṣṇa"),     // complex with 'R' + 'S' + 'N'
        ];

        for (hk, expected_iso) in word_tests {
            let result = converter.hk_to_iso(hk).unwrap();
            assert_eq!(
                result, expected_iso,
                "Harvard-Kyoto word '{}' should map to ISO '{}', got '{}'",
                hk, expected_iso, result
            );
        }
    }

    #[test]
    fn test_velthuis_consonant_mapping_correctness() {
        let converter = VelthuisConverter::new();

        // Velthuis consonants should also map without inherent 'a'
        let consonant_tests = vec![
            ("k", "k"),    // not "ka"
            ("kh", "kh"),  // not "kha"
            ("g", "g"),    // not "ga"
            ("gh", "gh"),  // not "gha"
            ("\"n", "ṅ"),  // not "ṅa" - Velthuis "n → ISO ṅ
            ("c", "c"),    // not "ca"
            ("ch", "ch"),  // not "cha"
            ("j", "j"),    // not "ja"
            ("jh", "jh"),  // not "jha"
            ("~n", "ñ"),   // not "ña" - Velthuis ~n → ISO ñ
            (".t", "ṭ"),   // not "ṭa" - Velthuis .t → ISO ṭ
            (".th", "ṭh"), // not "ṭha" - Velthuis .th → ISO ṭh
            (".d", "ḍ"),   // not "ḍa" - Velthuis .d → ISO ḍ
            (".dh", "ḍh"), // not "ḍha" - Velthuis .dh → ISO ḍh
            (".n", "ṇ"),   // not "ṇa" - Velthuis .n → ISO ṇ
            ("t", "t"),    // not "ta"
            ("th", "th"),  // not "tha"
            ("d", "d"),    // not "da"
            ("dh", "dh"),  // not "dha"
            ("n", "n"),    // not "na"
            ("p", "p"),    // not "pa"
            ("ph", "ph"),  // not "pha"
            ("b", "b"),    // not "ba"
            ("bh", "bh"),  // not "bha"
            ("m", "m"),    // not "ma"
            ("y", "y"),    // not "ya"
            ("r", "r"),    // not "ra"
            ("l", "l"),    // not "la"
            ("v", "v"),    // not "va"
            ("\"s", "ś"),  // not "śa" - Velthuis "s → ISO ś
            (".s", "ṣ"),   // not "ṣa" - Velthuis .s → ISO ṣ
            ("s", "s"),    // not "sa"
            ("h", "h"),    // not "ha"
        ];

        for (velthuis, expected_iso) in consonant_tests {
            let result = converter.velthuis_to_iso(velthuis).unwrap();
            assert_eq!(
                result, expected_iso,
                "Velthuis '{}' should map to ISO '{}', got '{}'",
                velthuis, expected_iso, result
            );
        }
    }

    #[test]
    fn test_velthuis_retroflex_consonants() {
        let converter = VelthuisConverter::new();

        // Velthuis uses dots for retroflex consonants
        let retroflex_tests = vec![
            (".t", "ṭ"),   // not "ṭa"
            (".th", "ṭh"), // not "ṭha"
            (".d", "ḍ"),   // not "ḍa"
            (".dh", "ḍh"), // not "ḍha"
            (".n", "ṇ"),   // not "ṇa"
            (".s", "ṣ"),   // not "ṣa"
        ];

        for (velthuis, expected_iso) in retroflex_tests {
            let result = converter.velthuis_to_iso(velthuis).unwrap();
            assert_eq!(
                result, expected_iso,
                "Velthuis retroflex '{}' should map to ISO '{}', got '{}'",
                velthuis, expected_iso, result
            );
        }
    }

    #[test]
    fn test_velthuis_aspirated_consonants() {
        let converter = VelthuisConverter::new();

        // Velthuis aspirated consonants should not have inherent 'a'
        let aspirated_tests = vec![
            ("kh", "kh"),  // not "kha"
            ("gh", "gh"),  // not "gha"
            ("ch", "ch"),  // not "cha"
            ("jh", "jh"),  // not "jha"
            (".th", "ṭh"), // not "ṭha"
            (".dh", "ḍh"), // not "ḍha"
            ("th", "th"),  // not "tha"
            ("dh", "dh"),  // not "dha"
            ("ph", "ph"),  // not "pha"
            ("bh", "bh"),  // not "bha"
        ];

        for (velthuis, expected_iso) in aspirated_tests {
            let result = converter.velthuis_to_iso(velthuis).unwrap();
            assert_eq!(
                result, expected_iso,
                "Velthuis aspirated '{}' should map to ISO '{}', got '{}'",
                velthuis, expected_iso, result
            );
        }
    }

    #[test]
    fn test_velthuis_nasals() {
        let converter = VelthuisConverter::new();

        // Velthuis nasal consonants should not have inherent 'a'
        let nasal_tests = vec![
            ("\"n", "ṅ"), // not "ṅa" - Velthuis "n → ISO ṅ
            ("~n", "ñ"),  // not "ña" - Velthuis ~n → ISO ñ
            (".n", "ṇ"),  // not "ṇa" - Velthuis .n → ISO ṇ
            ("n", "n"),   // not "na"
            ("m", "m"),   // not "ma"
        ];

        for (velthuis, expected_iso) in nasal_tests {
            let result = converter.velthuis_to_iso(velthuis).unwrap();
            assert_eq!(
                result, expected_iso,
                "Velthuis nasal '{}' should map to ISO '{}', got '{}'",
                velthuis, expected_iso, result
            );
        }
    }

    #[test]
    fn test_velthuis_sibilants() {
        let converter = VelthuisConverter::new();

        // Velthuis sibilants should not have inherent 'a'
        let sibilant_tests = vec![
            ("\"s", "ś"), // not "śa" - Velthuis "s → ISO ś
            (".s", "ṣ"),  // not "ṣa" - Velthuis .s → ISO ṣ
            ("s", "s"),   // not "sa"
        ];

        for (velthuis, expected_iso) in sibilant_tests {
            let result = converter.velthuis_to_iso(velthuis).unwrap();
            assert_eq!(
                result, expected_iso,
                "Velthuis sibilant '{}' should map to ISO '{}', got '{}'",
                velthuis, expected_iso, result
            );
        }
    }

    #[test]
    fn test_velthuis_with_explicit_vowels() {
        let converter = VelthuisConverter::new();

        // Test that explicit consonant+vowel combinations work correctly
        let cv_tests = vec![
            ("ka", "ka"), // consonant + vowel should have the vowel
            ("ki", "ki"),
            ("ku", "ku"),
            ("kaa", "kā"), // Velthuis aa → ISO ā
            ("kii", "kī"), // Velthuis ii → ISO ī
            ("kuu", "kū"), // Velthuis uu → ISO ū
            ("k.r", "kr̥"), // Velthuis .r → ISO r̥
            ("ke", "ke"),
            ("ko", "ko"),
            ("kai", "kai"),
            ("kau", "kau"),
        ];

        for (velthuis, expected_iso) in cv_tests {
            let result = converter.velthuis_to_iso(velthuis).unwrap();
            assert_eq!(
                result, expected_iso,
                "Velthuis '{}' should map to ISO '{}', got '{}'",
                velthuis, expected_iso, result
            );
        }
    }

    #[test]
    fn test_real_world_velthuis_words() {
        let converter = VelthuisConverter::new();

        // Real Velthuis words should convert correctly
        let word_tests = vec![
            ("namaste", "namaste"), // simple word
            ("dharma", "dharma"),   // with 'r'
            ("\"saanti", "śānti"),  // with "s → 'ś' and 'aa' → 'ā'
            (".siva", "ṣiva"),      // with .s → 'ṣ'
            ("k.r.s.na", "kr̥ṣṇa"),  // complex with .r + .s + .n
        ];

        for (velthuis, expected_iso) in word_tests {
            let result = converter.velthuis_to_iso(velthuis).unwrap();
            assert_eq!(
                result, expected_iso,
                "Velthuis word '{}' should map to ISO '{}', got '{}'",
                velthuis, expected_iso, result
            );
        }
    }

    #[test]
    fn test_wx_consonant_mapping_correctness() {
        let converter = WxConverter::new();

        // WX consonants should also map without inherent 'a'
        let consonant_tests = vec![
            ("k", "k"),  // not "ka"
            ("K", "kh"), // not "kha"
            ("g", "g"),  // not "ga"
            ("G", "gh"), // not "gha"
            ("f", "ṅ"),  // not "ṅa" - WX f → ISO ṅ
            ("c", "c"),  // not "ca"
            ("C", "ch"), // not "cha"
            ("j", "j"),  // not "ja"
            ("J", "jh"), // not "jha"
            ("F", "ñ"),  // not "ña" - WX F → ISO ñ
            ("w", "ṭ"),  // not "ṭa" - WX w → ISO ṭ
            ("W", "ṭh"), // not "ṭha" - WX W → ISO ṭh
            ("x", "ḍ"),  // not "ḍa" - WX x → ISO ḍ
            ("X", "ḍh"), // not "ḍha" - WX X → ISO ḍh
            ("N", "ṇ"),  // not "ṇa" - WX N → ISO ṇ
            ("t", "t"),  // not "ta"
            ("T", "th"), // not "tha"
            ("d", "d"),  // not "da"
            ("D", "dh"), // not "dha"
            ("n", "n"),  // not "na"
            ("p", "p"),  // not "pa"
            ("P", "ph"), // not "pha"
            ("b", "b"),  // not "ba"
            ("B", "bh"), // not "bha"
            ("m", "m"),  // not "ma"
            ("y", "y"),  // not "ya"
            ("r", "r"),  // not "ra"
            ("l", "l"),  // not "la"
            ("v", "v"),  // not "va"
            ("S", "ś"),  // not "śa" - WX S → ISO ś
            ("z", "ṣ"),  // not "ṣa" - WX z → ISO ṣ
            ("s", "s"),  // not "sa"
            ("h", "h"),  // not "ha"
        ];

        for (wx, expected_iso) in consonant_tests {
            let result = converter.wx_to_iso(wx).unwrap();
            assert_eq!(
                result, expected_iso,
                "WX '{}' should map to ISO '{}', got '{}'",
                wx, expected_iso, result
            );
        }
    }

    #[test]
    fn test_wx_retroflex_consonants() {
        let converter = WxConverter::new();

        // WX uses specific characters for retroflex consonants
        let retroflex_tests = vec![
            ("w", "ṭ"),  // not "ṭa" - WX w → ISO ṭ
            ("W", "ṭh"), // not "ṭha" - WX W → ISO ṭh
            ("x", "ḍ"),  // not "ḍa" - WX x → ISO ḍ
            ("X", "ḍh"), // not "ḍha" - WX X → ISO ḍh
            ("N", "ṇ"),  // not "ṇa" - WX N → ISO ṇ
        ];

        for (wx, expected_iso) in retroflex_tests {
            let result = converter.wx_to_iso(wx).unwrap();
            assert_eq!(
                result, expected_iso,
                "WX retroflex '{}' should map to ISO '{}', got '{}'",
                wx, expected_iso, result
            );
        }
    }

    #[test]
    fn test_wx_aspirated_consonants() {
        let converter = WxConverter::new();

        // WX aspirated consonants should not have inherent 'a'
        let aspirated_tests = vec![
            ("K", "kh"), // not "kha"
            ("G", "gh"), // not "gha"
            ("C", "ch"), // not "cha"
            ("J", "jh"), // not "jha"
            ("W", "ṭh"), // not "ṭha"
            ("X", "ḍh"), // not "ḍha"
            ("T", "th"), // not "tha"
            ("D", "dh"), // not "dha"
            ("P", "ph"), // not "pha"
            ("B", "bh"), // not "bha"
        ];

        for (wx, expected_iso) in aspirated_tests {
            let result = converter.wx_to_iso(wx).unwrap();
            assert_eq!(
                result, expected_iso,
                "WX aspirated '{}' should map to ISO '{}', got '{}'",
                wx, expected_iso, result
            );
        }
    }

    #[test]
    fn test_wx_nasals() {
        let converter = WxConverter::new();

        // WX nasal consonants should not have inherent 'a'
        let nasal_tests = vec![
            ("f", "ṅ"), // not "ṅa" - WX f → ISO ṅ
            ("F", "ñ"), // not "ña" - WX F → ISO ñ
            ("N", "ṇ"), // not "ṇa" - WX N → ISO ṇ
            ("n", "n"), // not "na"
            ("m", "m"), // not "ma"
        ];

        for (wx, expected_iso) in nasal_tests {
            let result = converter.wx_to_iso(wx).unwrap();
            assert_eq!(
                result, expected_iso,
                "WX nasal '{}' should map to ISO '{}', got '{}'",
                wx, expected_iso, result
            );
        }
    }

    #[test]
    fn test_wx_sibilants() {
        let converter = WxConverter::new();

        // WX sibilants should not have inherent 'a'
        let sibilant_tests = vec![
            ("S", "ś"), // not "śa" - WX S → ISO ś
            ("z", "ṣ"), // not "ṣa" - WX z → ISO ṣ
            ("s", "s"), // not "sa"
        ];

        for (wx, expected_iso) in sibilant_tests {
            let result = converter.wx_to_iso(wx).unwrap();
            assert_eq!(
                result, expected_iso,
                "WX sibilant '{}' should map to ISO '{}', got '{}'",
                wx, expected_iso, result
            );
        }
    }

    #[test]
    fn test_wx_with_explicit_vowels() {
        let converter = WxConverter::new();

        // Test that explicit consonant+vowel combinations work correctly
        let cv_tests = vec![
            ("ka", "ka"), // consonant + vowel should have the vowel
            ("ki", "ki"),
            ("ku", "ku"),
            ("kA", "kā"), // WX A → ISO ā
            ("kI", "kī"), // WX I → ISO ī
            ("kU", "kū"), // WX U → ISO ū
            ("kq", "kr̥"), // WX q → ISO r̥
            ("ke", "ke"),
            ("ko", "ko"),
            ("kE", "kai"), // WX E → ISO ai
            ("kO", "kau"), // WX O → ISO au
        ];

        for (wx, expected_iso) in cv_tests {
            let result = converter.wx_to_iso(wx).unwrap();
            assert_eq!(
                result, expected_iso,
                "WX '{}' should map to ISO '{}', got '{}'",
                wx, expected_iso, result
            );
        }
    }

    #[test]
    fn test_real_world_wx_words() {
        let converter = WxConverter::new();

        // Real WX words should convert correctly
        let word_tests = vec![
            ("namaste", "namaste"), // simple word
            ("Karma", "kharma"),    // with 'K' → 'kh' and 'A' → 'ā'
            ("SAnti", "śānti"),     // with 'S' → 'ś', 'A' → 'ā', and 't' → 't'
            ("ziva", "ṣiva"),       // with 'z' → 'ṣ'
            ("kqzNa", "kr̥ṣṇa"),     // complex with 'q' + 'z' + 'N'
        ];

        for (wx, expected_iso) in word_tests {
            let result = converter.wx_to_iso(wx).unwrap();
            assert_eq!(
                result, expected_iso,
                "WX word '{}' should map to ISO '{}', got '{}'",
                wx, expected_iso, result
            );
        }
    }
}
