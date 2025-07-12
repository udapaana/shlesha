use shlesha::Shlesha;

#[test]
fn test_bijective_roundtrip_ll_character() {
    let shlesha = Shlesha::new();

    // Test: ळ (Devanagari) -> Bengali -> Devanagari
    // This character doesn't exist in Bengali, so should be preserved
    let original = "ळ";

    let bengali_result = shlesha
        .transliterate(original, "devanagari", "bengali")
        .expect("Failed to convert to Bengali");

    println!("Original: {} -> Bengali: {}", original, bengali_result);

    let roundtrip_result = shlesha
        .transliterate(&bengali_result, "bengali", "devanagari")
        .expect("Failed to convert back to Devanagari");

    println!(
        "Bengali: {} -> Roundtrip: {}",
        bengali_result, roundtrip_result
    );

    assert_eq!(
        original, roundtrip_result,
        "Bijective roundtrip failed for ळ character. Expected preservation."
    );
}

#[test]
fn test_bijective_roundtrip_multiple_scripts() {
    let shlesha = Shlesha::new();

    // Test characters that may not exist in all scripts
    let test_cases = vec![
        ("ळ", "devanagari"), // Special consonant
        ("ऴ", "devanagari"), // Extended consonant
    ];

    let target_scripts = vec!["bengali", "gujarati", "tamil"];

    for (original_char, source_script) in test_cases {
        for target_script in &target_scripts {
            // Convert to target script
            let converted = shlesha
                .transliterate(original_char, source_script, target_script)
                .expect(&format!(
                    "Failed to convert {} from {} to {}",
                    original_char, source_script, target_script
                ));

            // Convert back to source script
            let roundtrip = shlesha
                .transliterate(&converted, target_script, source_script)
                .expect(&format!(
                    "Failed to convert back from {} to {}",
                    target_script, source_script
                ));

            assert_eq!(
                original_char,
                roundtrip,
                "Bijective roundtrip failed: {} -> {} -> {} -> {}. Expected: {}, Got: {}",
                original_char,
                source_script,
                target_script,
                source_script,
                original_char,
                roundtrip
            );
        }
    }
}

#[test]
fn test_bijective_roundtrip_with_common_characters() {
    let shlesha = Shlesha::new();

    // Test that common characters still work correctly
    let common_text = "नमस्ते";

    let bengali_result = shlesha
        .transliterate(common_text, "devanagari", "bengali")
        .expect("Failed to convert to Bengali");

    let roundtrip_result = shlesha
        .transliterate(&bengali_result, "bengali", "devanagari")
        .expect("Failed to convert back to Devanagari");

    assert_eq!(
        common_text, roundtrip_result,
        "Bijective roundtrip failed for common text: {}",
        common_text
    );
}

#[test]
fn test_runtime_token_allocation() {
    // Test that runtime tokens are properly allocated
    // This is more of a compilation test - if it compiles, the tokens exist
    let shlesha = Shlesha::new();

    // Test basic functionality with runtime token architecture
    let result = shlesha.transliterate("a", "iast", "devanagari");
    assert!(
        result.is_ok(),
        "Basic transliteration should work with runtime token architecture"
    );
}

#[test]
fn test_preservation_priority_devanagari() {
    let shlesha = Shlesha::new();

    // Test that preservation mappings prefer Devanagari as canonical source
    // ळ exists in both Devanagari (ळ) and Gujarati (ળ) - should prefer Devanagari
    let original = "ळ"; // Devanagari ळ

    let bengali_result = shlesha
        .transliterate(original, "devanagari", "bengali")
        .expect("Failed to convert to Bengali");

    let roundtrip_result = shlesha
        .transliterate(&bengali_result, "bengali", "devanagari")
        .expect("Failed to convert back to Devanagari");

    // Check that we get back the Devanagari character, not the Gujarati one
    assert_eq!(
        original.as_bytes(),
        roundtrip_result.as_bytes(),
        "Should preserve Devanagari ळ (bytes: {:?}), not Gujarati ળ. Got bytes: {:?}",
        original.as_bytes(),
        roundtrip_result.as_bytes()
    );
}
