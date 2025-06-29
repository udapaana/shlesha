#[cfg(test)]
mod integration_tests {
    use crate::modules::hub::{Hub, HubInput, HubOutput, HubTrait};
    use crate::modules::script_converter::{
        BengaliConverter, HarvardKyotoConverter, ISO15919Converter, IastConverter, ItransConverter,
        ScriptConverter, ScriptConverterRegistry, Slp1Converter, TamilConverter, TeluguConverter,
        VelthuisConverter, WxConverter,
    };

    /// Test roundtrip conversion: Script → Hub → Devanagari
    #[test]
    fn test_iast_to_devanagari_roundtrip() {
        let hub = Hub::new();
        let iast_converter = IastConverter::new();

        let test_cases = vec![
            ("ā", "आ"),
            ("ṛ", "ऋ"),
            ("ṝ", "ॠ"),
            ("ḷ", "ऌ"),
            ("ḹ", "ॡ"),
            ("ṃ", "ं"),
            ("ḥ", "ः"),
            ("śa", "श"),
            ("ṣa", "ष"),
        ];

        for (iast_input, expected_deva) in test_cases {
            // IAST → ISO via script converter
            let hub_input = iast_converter.to_hub("iast", iast_input).unwrap();
            if let HubInput::Iso(iso_text) = hub_input {
                // ISO → Devanagari via hub
                let hub_output = hub.iso_to_deva(&iso_text).unwrap();
                if let HubOutput::Devanagari(deva_result) = hub_output {
                    assert_eq!(
                        deva_result, expected_deva,
                        "Roundtrip failed: {} → {} → {}",
                        iast_input, iso_text, deva_result
                    );
                } else {
                    panic!("Expected Devanagari output");
                }
            } else {
                panic!("Expected ISO hub input");
            }
        }
    }

    /// Test roundtrip conversion: Script → Hub → ISO
    #[test]
    fn test_itrans_to_iso_roundtrip() {
        let itrans_converter = ItransConverter::new();

        let test_cases = vec![
            ("A", "ā"),
            ("I", "ī"),
            ("U", "ū"),
            ("R", "r̥"),
            ("RR", "r̥̄"),
            ("M", "ṁ"),
            ("H", "ḥ"),
            ("kh", "kh"),
            ("gh", "gh"),
            ("ch", "ch"),
            ("Th", "ṭh"),
            ("Dh", "ḍh"),
            ("sh", "ś"),
            ("Sh", "ṣ"),
        ];

        for (itrans_input, expected_iso) in test_cases {
            let hub_input = itrans_converter.to_hub("itrans", itrans_input).unwrap();
            if let HubInput::Iso(iso_result) = hub_input {
                assert_eq!(
                    iso_result, expected_iso,
                    "ITRANS conversion failed: {} → {}",
                    itrans_input, iso_result
                );
            } else {
                panic!("Expected ISO hub input");
            }
        }
    }

    /// Test roundtrip conversion: Script → Hub → Devanagari
    #[test]
    fn test_slp1_to_devanagari_roundtrip() {
        let hub = Hub::new();
        let slp1_converter = Slp1Converter::new();

        let test_cases = vec![
            ("A", "आ"),
            ("I", "ई"),
            ("U", "ऊ"),
            ("f", "ऋ"),
            ("F", "ॠ"),
            ("x", "ऌ"),
            ("X", "ॡ"),
            ("E", "ऐ"),
            ("O", "औ"),
            ("M", "ं"),
            ("H", "ः"),
            ("K", "ख्"),
            ("G", "घ्"),
            ("C", "छ्"),
            ("W", "ठ्"),
            ("Q", "ढ्"),
            ("S", "श्"),
            ("z", "ष्"),
        ];

        for (slp1_input, expected_deva) in test_cases {
            // SLP1 → ISO via script converter
            let hub_input = slp1_converter.to_hub("slp1", slp1_input).unwrap();
            if let HubInput::Iso(iso_text) = hub_input {
                // ISO → Devanagari via hub
                let hub_output = hub.iso_to_deva(&iso_text).unwrap();
                if let HubOutput::Devanagari(deva_result) = hub_output {
                    assert_eq!(
                        deva_result, expected_deva,
                        "SLP1 roundtrip failed: {} → {} → {}",
                        slp1_input, iso_text, deva_result
                    );
                } else {
                    panic!("Expected Devanagari output");
                }
            } else {
                panic!("Expected ISO hub input");
            }
        }
    }

    /// Test Harvard-Kyoto → ISO → Devanagari roundtrip conversion
    #[test]
    fn test_harvard_kyoto_to_devanagari_roundtrip() {
        let hub = Hub::new();
        let hk_converter = HarvardKyotoConverter::new();

        let test_cases = vec![
            ("a", "अ"),
            ("A", "आ"),
            ("i", "इ"),
            ("I", "ई"),
            ("u", "उ"),
            ("U", "ऊ"),
            ("R", "ऋ"),
            ("RR", "ॠ"),
            ("e", "ए"),
            ("ai", "ऐ"),
            ("o", "ओ"),
            ("au", "औ"),
            ("M", "ं"),
            ("H", "ः"),
            ("T", "ट्"),  // bare consonant with virama
            ("Th", "ठ्"), // bare consonant with virama
            ("D", "ड्"),  // bare consonant with virama
            ("Dh", "ढ्"), // bare consonant with virama
            ("N", "ण्"),  // bare consonant with virama
            ("z", "श्"),  // HK z → ISO ś → Devanagari श्
            ("S", "ष्"),  // HK S → ISO ṣ → Devanagari ष्
        ];

        for (hk_input, expected_deva) in test_cases {
            // Harvard-Kyoto → ISO via script converter
            let hub_input = hk_converter.to_hub("harvard_kyoto", hk_input).unwrap();
            if let HubInput::Iso(iso_text) = hub_input {
                // ISO → Devanagari via hub
                let hub_output = hub.iso_to_deva(&iso_text).unwrap();
                if let HubOutput::Devanagari(deva_result) = hub_output {
                    assert_eq!(
                        deva_result, expected_deva,
                        "Harvard-Kyoto roundtrip failed: {} → {} → {}",
                        hk_input, iso_text, deva_result
                    );
                } else {
                    panic!("Expected Devanagari output");
                }
            } else {
                panic!("Expected ISO hub input");
            }
        }
    }

    /// Test Velthuis → ISO → Devanagari roundtrip conversion
    #[test]
    fn test_velthuis_to_devanagari_roundtrip() {
        let hub = Hub::new();
        let velthuis_converter = VelthuisConverter::new();

        let test_cases = vec![
            ("a", "अ"),
            ("aa", "आ"),
            ("i", "इ"),
            ("ii", "ई"),
            ("u", "उ"),
            ("uu", "ऊ"),
            (".r", "ऋ"),
            (".R", "ॠ"),
            ("e", "ए"),
            ("ai", "ऐ"),
            ("o", "ओ"),
            ("au", "औ"),
            (".m", "ं"),
            (".h", "ः"),
            (".t", "ट्"),  // bare consonant with virama
            (".th", "ठ्"), // bare consonant with virama
            (".d", "ड्"),  // bare consonant with virama
            (".dh", "ढ्"), // bare consonant with virama
            (".n", "ण्"),  // bare consonant with virama
            ("\"s", "श्"), // Velthuis "s → ISO ś → Devanagari श्
            (".s", "ष्"),  // Velthuis .s → ISO ṣ → Devanagari ष्
        ];

        for (velthuis_input, expected_deva) in test_cases {
            // Velthuis → ISO via script converter
            let hub_input = velthuis_converter
                .to_hub("velthuis", velthuis_input)
                .unwrap();
            if let HubInput::Iso(iso_text) = hub_input {
                // ISO → Devanagari via hub
                let hub_output = hub.iso_to_deva(&iso_text).unwrap();
                if let HubOutput::Devanagari(deva_result) = hub_output {
                    assert_eq!(
                        deva_result, expected_deva,
                        "Velthuis roundtrip failed: {} → {} → {}",
                        velthuis_input, iso_text, deva_result
                    );
                } else {
                    panic!("Expected Devanagari output");
                }
            } else {
                panic!("Expected ISO hub input");
            }
        }
    }

    /// Test WX notation → ISO → Devanagari roundtrip conversion
    #[test]
    fn test_wx_to_devanagari_roundtrip() {
        let hub = Hub::new();
        let wx_converter = WxConverter::new();

        let test_cases = vec![
            ("a", "अ"),
            ("A", "आ"),
            ("i", "इ"),
            ("I", "ई"),
            ("u", "उ"),
            ("U", "ऊ"),
            ("q", "ऋ"),
            ("Q", "ॠ"),
            ("e", "ए"),
            ("E", "ऐ"),
            ("o", "ओ"),
            ("O", "औ"),
            ("M", "ं"),
            ("H", "ः"),
            ("w", "ट्"), // bare consonant with virama
            ("W", "ठ्"), // bare consonant with virama
            ("x", "ड्"), // bare consonant with virama
            ("X", "ढ्"), // bare consonant with virama
            ("N", "ण्"), // bare consonant with virama
            ("S", "श्"), // WX S → ISO ś → Devanagari श्
            ("z", "ष्"), // WX z → ISO ṣ → Devanagari ष्
        ];

        for (wx_input, expected_deva) in test_cases {
            // WX → ISO via script converter
            let hub_input = wx_converter.to_hub("wx", wx_input).unwrap();
            if let HubInput::Iso(iso_text) = hub_input {
                // ISO → Devanagari via hub
                let hub_output = hub.iso_to_deva(&iso_text).unwrap();
                if let HubOutput::Devanagari(deva_result) = hub_output {
                    assert_eq!(
                        deva_result, expected_deva,
                        "WX roundtrip failed: {} → {} → {}",
                        wx_input, iso_text, deva_result
                    );
                } else {
                    panic!("Expected Devanagari output");
                }
            } else {
                panic!("Expected ISO hub input");
            }
        }
    }

    /// Test Devanagari → Hub → ISO conversion (reverse direction)
    #[test]
    fn test_devanagari_to_iso_roundtrip() {
        let hub = Hub::new();
        // Devanagari is the hub format, so we test the hub directly

        let test_cases = vec![
            ("अ", "a"),
            ("आ", "ā"),
            ("इ", "i"),
            ("ई", "ī"),
            ("उ", "u"),
            ("ऊ", "ū"),
            ("ऋ", "r̥"),
            ("ॠ", "r̥̄"),
            ("ए", "e"),
            ("ऐ", "ai"),
            ("ओ", "o"),
            ("औ", "au"),
            ("ं", "ṁ"),
            ("ः", "ḥ"),
            ("क्", "k"),  // bare consonant with virama
            ("ख्", "kh"), // bare consonant with virama
            ("ग्", "g"),  // bare consonant with virama
            ("घ्", "gh"), // bare consonant with virama
            ("ङ्", "ṅ"),  // bare consonant with virama
            ("श्", "ś"),  // Devanagari श् → ISO ś
            ("ष्", "ṣ"),  // Devanagari ष् → ISO ṣ
        ];

        for (deva_input, expected_iso) in test_cases {
            // Devanagari is the hub format, so test direct hub conversion
            let hub_output = hub.deva_to_iso(deva_input).unwrap();
            if let HubOutput::Iso(iso_result) = hub_output {
                assert_eq!(
                    iso_result, expected_iso,
                    "Devanagari to ISO conversion failed: {} → {}",
                    deva_input, iso_result
                );
            } else {
                panic!("Hub should return ISO output for deva_to_iso");
            }
        }
    }

    /// Test Bengali → ISO → Devanagari conversion (cross-script via hub)
    #[test]
    fn test_bengali_to_devanagari_roundtrip() {
        let hub = Hub::new();
        let bengali_converter = BengaliConverter::new();

        let test_cases = vec![
            ("অ", "अ"), // Bengali অ → ISO a → Devanagari अ
            ("আ", "आ"), // Bengali আ → ISO ā → Devanagari आ
            ("ই", "इ"), // Bengali ই → ISO i → Devanagari इ
            ("ঈ", "ई"), // Bengali ঈ → ISO ī → Devanagari ई
            ("উ", "उ"), // Bengali উ → ISO u → Devanagari उ
            ("ঊ", "ऊ"), // Bengali ঊ → ISO ū → Devanagari ऊ
            ("ঋ", "ऋ"), // Bengali ঋ → ISO r̥ → Devanagari ऋ
            ("এ", "ए"), // Bengali এ → ISO e → Devanagari ए
            ("ঐ", "ऐ"), // Bengali ঐ → ISO ai → Devanagari ऐ
            ("ও", "ओ"), // Bengali ও → ISO o → Devanagari ओ
            ("ঔ", "औ"), // Bengali ঔ → ISO au → Devanagari औ
            ("ক", "क"), // Bengali ক → ISO ka → Devanagari क
            ("খ", "ख"), // Bengali খ → ISO kha → Devanagari ख
            ("গ", "ग"), // Bengali গ → ISO ga → Devanagari ग
            ("ঘ", "घ"), // Bengali ঘ → ISO gha → Devanagari घ
            ("শ", "श"), // Bengali শ → ISO śa → Devanagari श
            ("ষ", "ष"), // Bengali ষ → ISO ṣa → Devanagari ष
            ("স", "स"), // Bengali স → ISO sa → Devanagari स
        ];

        for (bengali_input, expected_deva) in test_cases {
            // Bengali → Devanagari via script converter (new architecture)
            let hub_input = bengali_converter.to_hub("bengali", bengali_input).unwrap();
            if let HubInput::Devanagari(deva_result) = hub_input {
                assert_eq!(
                    deva_result, expected_deva,
                    "Bengali to Devanagari conversion failed: {} → {}",
                    bengali_input, deva_result
                );
            } else {
                panic!("Expected Devanagari hub input");
            }
        }
    }

    /// Test ISO-15919 → Hub → Devanagari conversion (hub format passthrough)
    #[test]
    fn test_iso15919_to_devanagari_roundtrip() {
        let hub = Hub::new();
        let iso_converter = ISO15919Converter::new();

        let test_cases = vec![
            ("a", "अ"),
            ("ā", "आ"),
            ("i", "इ"),
            ("ī", "ई"),
            ("u", "उ"),
            ("ū", "ऊ"),
            ("r̥", "ऋ"),
            ("r̥̄", "ॠ"),
            ("e", "ए"),
            ("ai", "ऐ"),
            ("o", "ओ"),
            ("au", "औ"),
            ("ṁ", "ं"),
            ("ḥ", "ः"),
            ("k", "क्"),  // bare consonant with virama
            ("kh", "ख्"), // bare consonant with virama
            ("g", "ग्"),  // bare consonant with virama
            ("gh", "घ्"), // bare consonant with virama
            ("ṅ", "ङ्"),  // bare consonant with virama
            ("ś", "श्"),  // ISO ś → Devanagari श्
            ("ṣ", "ष्"),  // ISO ṣ → Devanagari ष्
        ];

        for (iso_input, expected_deva) in test_cases {
            // ISO-15919 → Hub (passthrough)
            let hub_input = iso_converter.to_hub("iso15919", iso_input).unwrap();
            if let HubInput::Iso(iso_text) = hub_input {
                // Hub: ISO → Devanagari
                let hub_output = hub.iso_to_deva(&iso_text).unwrap();
                if let HubOutput::Devanagari(deva_result) = hub_output {
                    assert_eq!(
                        deva_result, expected_deva,
                        "ISO-15919 roundtrip failed: {} → {} → {}",
                        iso_input, iso_text, deva_result
                    );
                } else {
                    panic!("Expected Devanagari output");
                }
            } else {
                panic!("Expected ISO hub input");
            }
        }
    }

    /// Test Tamil → ISO → Devanagari conversion (cross-script via hub)
    #[test]
    fn test_tamil_to_devanagari_roundtrip() {
        let hub = Hub::new();
        let tamil_converter = TamilConverter::new();

        let test_cases = vec![
            ("அ", "अ"), // Tamil அ → ISO a → Devanagari अ
            ("ஆ", "आ"), // Tamil ஆ → ISO ā → Devanagari आ
            ("இ", "इ"), // Tamil இ → ISO i → Devanagari इ
            ("ஈ", "ई"), // Tamil ஈ → ISO ī → Devanagari ई
            ("உ", "उ"), // Tamil உ → ISO u → Devanagari उ
            ("ஊ", "ऊ"), // Tamil ஊ → ISO ū → Devanagari ऊ
            ("எ", "ए"), // Tamil எ → ISO e → Devanagari ए
            ("ஐ", "ऐ"), // Tamil ஐ → ISO ai → Devanagari ऐ
            ("ஒ", "ओ"), // Tamil ஒ → ISO o → Devanagari ओ
            // Note: Tamil ஏ (ē) and ஓ (ō) don't have direct Devanagari equivalents in the hub
            ("ஔ", "औ"), // Tamil ஔ → ISO au → Devanagari औ
            ("க", "क"), // Tamil க → ISO ka → Devanagari क
            ("ச", "च"), // Tamil ச → ISO ca → Devanagari च
            ("ட", "ट"), // Tamil ட → ISO ṭa → Devanagari ट
            ("த", "त"), // Tamil த → ISO ta → Devanagari त
            ("ப", "प"), // Tamil ப → ISO pa → Devanagari प
            ("ம", "म"), // Tamil ம → ISO ma → Devanagari म
        ];

        for (tamil_input, expected_deva) in test_cases {
            // Tamil → Devanagari via script converter (new architecture)
            let hub_input = tamil_converter.to_hub("tamil", tamil_input).unwrap();
            if let HubInput::Devanagari(deva_result) = hub_input {
                assert_eq!(
                    deva_result, expected_deva,
                    "Tamil to Devanagari conversion failed: {} → {}",
                    tamil_input, deva_result
                );
            } else {
                panic!("Expected Devanagari hub input");
            }
        }
    }

    /// Test Telugu → ISO → Devanagari conversion (cross-script via hub)
    #[test]
    fn test_telugu_to_devanagari_roundtrip() {
        let hub = Hub::new();
        let telugu_converter = TeluguConverter::new();

        let test_cases = vec![
            ("అ", "अ"), // Telugu అ → ISO a → Devanagari अ
            ("ఆ", "आ"), // Telugu ఆ → ISO ā → Devanagari आ
            ("ఇ", "इ"), // Telugu ఇ → ISO i → Devanagari इ
            ("ఈ", "ई"), // Telugu ఈ → ISO ī → Devanagari ई
            ("ఉ", "उ"), // Telugu ఉ → ISO u → Devanagari उ
            ("ఊ", "ऊ"), // Telugu ఊ → ISO ū → Devanagari ऊ
            ("ఋ", "ऋ"), // Telugu ఋ → ISO r̥ → Devanagari ऋ
            ("ఎ", "ए"), // Telugu ఎ → ISO e → Devanagari ए
            ("ఐ", "ऐ"), // Telugu ఐ → ISO ai → Devanagari ऐ
            ("ఒ", "ओ"), // Telugu ఒ → ISO o → Devanagari ओ
            // Note: Telugu ఏ (ē) and ఓ (ō) don't have direct Devanagari equivalents in the hub
            ("ఔ", "औ"), // Telugu ఔ → ISO au → Devanagari औ
            ("క", "क"), // Telugu క → ISO ka → Devanagari क
            ("ఖ", "ख"), // Telugu ఖ → ISO kha → Devanagari ख
            ("గ", "ग"), // Telugu గ → ISO ga → Devanagari ग
            ("ఘ", "घ"), // Telugu ఘ → ISO gha → Devanagari घ
            ("చ", "च"), // Telugu చ → ISO ca → Devanagari च
            ("జ", "ज"), // Telugu జ → ISO ja → Devanagari ज
            ("ట", "ट"), // Telugu ట → ISO ṭa → Devanagari ट
            ("డ", "ड"), // Telugu డ → ISO ḍa → Devanagari ड
            ("త", "त"), // Telugu త → ISO ta → Devanagari त
            ("ద", "द"), // Telugu ద → ISO da → Devanagari द
            ("ప", "प"), // Telugu ప → ISO pa → Devanagari प
            ("బ", "ब"), // Telugu బ → ISO ba → Devanagari ब
            ("మ", "म"), // Telugu మ → ISO ma → Devanagari म
            ("శ", "श"), // Telugu శ → ISO śa → Devanagari श
            ("ష", "ष"), // Telugu ష → ISO ṣa → Devanagari ष
            ("స", "स"), // Telugu స → ISO sa → Devanagari स
        ];

        for (telugu_input, expected_deva) in test_cases {
            // Telugu → Devanagari via script converter (new architecture)
            let hub_input = telugu_converter.to_hub("telugu", telugu_input).unwrap();
            if let HubInput::Devanagari(deva_result) = hub_input {
                assert_eq!(
                    deva_result, expected_deva,
                    "Telugu to Devanagari conversion failed: {} → {}",
                    telugu_input, deva_result
                );
            } else {
                panic!("Expected Devanagari hub input");
            }
        }
    }

    /// Test script converter registry functionality
    #[test]
    fn test_script_converter_registry() {
        let mut registry = ScriptConverterRegistry::new();
        registry.register_converter(Box::new(IastConverter::new()));
        registry.register_converter(Box::new(ItransConverter::new()));
        registry.register_converter(Box::new(Slp1Converter::new()));

        // Test supported scripts
        let supported = registry.list_supported_scripts();
        assert!(supported.contains(&"iast"));
        assert!(supported.contains(&"itrans"));
        assert!(supported.contains(&"slp1"));

        // Test conversions through registry
        let result = registry.to_hub("iast", "ā").unwrap();
        if let HubInput::Iso(iso_text) = result {
            assert_eq!(iso_text, "ā");
        } else {
            panic!("Expected ISO hub input");
        }

        let result = registry.to_hub("itrans", "A").unwrap();
        if let HubInput::Iso(iso_text) = result {
            assert_eq!(iso_text, "ā");
        } else {
            panic!("Expected ISO hub input");
        }

        let result = registry.to_hub("slp1", "A").unwrap();
        if let HubInput::Iso(iso_text) = result {
            assert_eq!(iso_text, "ā");
        } else {
            panic!("Expected ISO hub input");
        }
    }

    /// Test full transliteration pipeline: Different scripts → Common target
    #[test]
    fn test_multi_script_to_common_target() {
        let hub = Hub::new();
        let mut registry = ScriptConverterRegistry::new();
        registry.register_converter(Box::new(IastConverter::new()));
        registry.register_converter(Box::new(ItransConverter::new()));
        registry.register_converter(Box::new(Slp1Converter::new()));
        registry.register_converter(Box::new(HarvardKyotoConverter::new()));
        registry.register_converter(Box::new(VelthuisConverter::new()));
        registry.register_converter(Box::new(WxConverter::new()));

        // All these should produce the same Devanagari output (bare consonant)
        let test_cases = vec![
            ("iast", "ś"),          // bare consonant
            ("itrans", "sh"),       // bare consonant
            ("slp1", "S"),          // bare consonant
            ("harvard_kyoto", "z"), // bare consonant - HK z → ISO ś
            ("velthuis", "\"s"),    // bare consonant - Velthuis "s → ISO ś
            ("wx", "S"),            // bare consonant - WX S → ISO ś
        ];

        for (script, input) in test_cases {
            let hub_input = registry.to_hub(script, input).unwrap();
            if let HubInput::Iso(iso_text) = hub_input {
                let hub_output = hub.iso_to_deva(&iso_text).unwrap();
                if let HubOutput::Devanagari(deva_result) = hub_output {
                    assert_eq!(
                        deva_result, "श्",
                        "Script {} input '{}' should produce 'श्', got '{}'",
                        script, input, deva_result
                    );
                } else {
                    panic!("Expected Devanagari output");
                }
            } else {
                panic!("Expected ISO hub input");
            }
        }
    }

    /// Test the clean public interface for callers
    #[test]
    fn test_clean_script_converter_interface() {
        // Easy setup - all converters pre-registered
        let registry = ScriptConverterRegistry::default();

        // Discovery - what scripts are supported?
        let supported_scripts = registry.list_supported_scripts();
        assert!(supported_scripts.contains(&"itrans"));
        assert!(supported_scripts.contains(&"harvard_kyoto"));
        assert!(supported_scripts.contains(&"velthuis"));
        assert!(supported_scripts.contains(&"slp1"));
        assert!(supported_scripts.contains(&"iast"));
        assert!(supported_scripts.contains(&"wx"));
        assert!(supported_scripts.contains(&"devanagari"));
        assert!(supported_scripts.contains(&"bengali"));
        assert!(supported_scripts.contains(&"iso15919"));
        assert!(supported_scripts.contains(&"tamil"));
        assert!(supported_scripts.contains(&"telugu"));

        // Check support for specific scripts
        assert!(registry.supports_script("itrans"));
        assert!(registry.supports_script("hk")); // Harvard-Kyoto alias
        assert!(registry.supports_script("wx")); // WX notation
        assert!(registry.supports_script("devanagari")); // Devanagari
        assert!(registry.supports_script("deva")); // Devanagari alias
        assert!(registry.supports_script("bengali")); // Bengali
        assert!(registry.supports_script("bn")); // Bengali alias
        assert!(registry.supports_script("iso15919")); // ISO-15919
        assert!(registry.supports_script("iso")); // ISO-15919 alias
        assert!(registry.supports_script("tamil")); // Tamil
        assert!(registry.supports_script("ta")); // Tamil alias
        assert!(registry.supports_script("telugu")); // Telugu
        assert!(registry.supports_script("te")); // Telugu alias
        assert!(!registry.supports_script("unknown_script"));

        // Convert text using clean interface
        let result = registry.convert_to_hub("itrans", "namaste").unwrap();
        if let HubInput::Iso(iso_text) = result {
            assert_eq!(iso_text, "namaste");
        } else {
            panic!("Expected ISO hub input");
        }

        // Check script characteristics
        assert!(!registry.script_has_implicit_vowels("itrans").unwrap()); // Romanization
        assert!(!registry.script_has_implicit_vowels("velthuis").unwrap()); // Romanization
        assert!(!registry.script_has_implicit_vowels("wx").unwrap()); // Romanization
        assert!(registry.script_has_implicit_vowels("devanagari").unwrap()); // Indic script
        assert!(registry.script_has_implicit_vowels("bengali").unwrap()); // Indic script
        assert!(!registry.script_has_implicit_vowels("iso15919").unwrap()); // Romanization
        assert!(registry.script_has_implicit_vowels("tamil").unwrap()); // Indic script
        assert!(registry.script_has_implicit_vowels("telugu").unwrap()); // Indic script

        // Error handling for unsupported script
        assert!(registry.convert_to_hub("unsupported", "test").is_err());
        assert!(registry.script_has_implicit_vowels("unsupported").is_err());
    }
}
