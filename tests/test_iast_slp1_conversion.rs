use shlesha::Shlesha;

#[test]
fn test_iast_to_slp1_direct_conversion() {
    // Test the specific issue we found: IAST → SLP1 conversions not working
    let shlesha = Shlesha::new();

    let test_cases = vec![
        // (iast_input, expected_slp1_output)
        ("ā", "A"),
        ("ī", "I"),
        ("ū", "U"),
        ("ṛ", "f"),
        ("ṝ", "F"),
        ("ṁ", "M"),
        ("ṃ", "M"), // Also accept underdot variant
        ("ḥ", "H"),
        ("ś", "S"),
        ("ṣ", "z"),
        ("ṅ", "N"),
        ("ñ", "Y"),
        ("ṇ", "R"),
        ("ṭ", "w"),
        ("ḍ", "q"),
        ("kṣ", "kz"),
        ("ai", "E"),
        ("au", "O"),
        ("saṁskṛtam", "saMskftam"),
        ("saṃskṛtam", "saMskftam"), // Also accept underdot variant
        ("dharmakṣetre", "Darmakzetre"),
        ("namaskāram", "namaskAram"),
    ];

    for (iast_input, expected_slp1) in test_cases {
        let result = shlesha
            .transliterate(iast_input, "iast", "slp1")
            .unwrap_or_else(|e| panic!("IAST→SLP1 conversion failed for '{}': {}", iast_input, e));

        assert_eq!(
            result, expected_slp1,
            "IAST '{}' should convert to SLP1 '{}', got '{}'",
            iast_input, expected_slp1, result
        );
    }
}

#[test]
fn test_slp1_to_iast_reverse_conversion() {
    // Test that SLP1 → IAST works correctly
    let shlesha = Shlesha::new();

    let test_cases = vec![
        // (slp1_input, expected_iast_output)
        ("A", "ā"),
        ("I", "ī"),
        ("U", "ū"),
        ("f", "ṛ"),
        ("F", "ṝ"),
        ("M", "ṁ"),
        ("H", "ḥ"),
        ("S", "ś"),
        ("z", "ṣ"),
        ("N", "ṅ"),
        ("Y", "ñ"),
        ("R", "ṇ"),
        ("w", "ṭ"),
        ("q", "ḍ"),
        ("kz", "kṣ"),
        ("E", "ai"),
        ("O", "au"),
        ("saMskftam", "saṁskṛtam"),
        ("Darmakzetre", "dharmakṣetre"),
        ("namaskAram", "namaskāram"),
    ];

    for (slp1_input, expected_iast) in test_cases {
        let result = shlesha
            .transliterate(slp1_input, "slp1", "iast")
            .unwrap_or_else(|e| panic!("SLP1→IAST conversion failed for '{}': {}", slp1_input, e));

        assert_eq!(
            result, expected_iast,
            "SLP1 '{}' should convert to IAST '{}', got '{}'",
            slp1_input, expected_iast, result
        );
    }
}

#[test]
fn test_iast_slp1_roundtrip() {
    // Test that IAST → SLP1 → IAST preserves the original
    let shlesha = Shlesha::new();

    let test_inputs = vec![
        "saṁskṛtam",
        "dharmakṣetre",
        "namaskāram",
        "bhagavadgītā",
        "ā",
        "ī",
        "ū",
        "ṛ",
        "ṁ",
        "ḥ",
        "ś",
        "ṣ",
    ];

    for original_iast in test_inputs {
        // IAST → SLP1
        let slp1_result = shlesha
            .transliterate(original_iast, "iast", "slp1")
            .unwrap_or_else(|e| panic!("IAST→SLP1 failed for '{}': {}", original_iast, e));

        // SLP1 → IAST
        let back_to_iast = shlesha
            .transliterate(&slp1_result, "slp1", "iast")
            .unwrap_or_else(|e| panic!("SLP1→IAST failed for '{}': {}", slp1_result, e));

        assert_eq!(
            back_to_iast, original_iast,
            "Round-trip failed: '{}' → '{}' → '{}'",
            original_iast, slp1_result, back_to_iast
        );
    }
}

#[test]
fn test_simple_conversions() {
    // Test simple conversions that should definitely work
    let shlesha = Shlesha::new();

    // Test that identity conversions work
    assert_eq!(
        shlesha.transliterate("test", "iast", "iast").unwrap(),
        "test"
    );
    assert_eq!(
        shlesha.transliterate("test", "slp1", "slp1").unwrap(),
        "test"
    );

    // Test basic ASCII preservation
    assert_eq!(
        shlesha.transliterate("abc123", "iast", "slp1").unwrap(),
        "abc123"
    );

    // Test that IAST to ISO works (this should work)
    let iast_to_iso = shlesha.transliterate("saṁskṛtam", "iast", "iso").unwrap();
    println!("IAST → ISO: '{}'", iast_to_iso);
    assert!(iast_to_iso.contains("saṁ"), "Should contain ISO anusvara");
    assert!(iast_to_iso.contains("r̥"), "Should contain ISO vocalic r");
}
