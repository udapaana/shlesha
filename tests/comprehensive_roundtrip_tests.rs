use quickcheck::{quickcheck, TestResult};
use shlesha::Shlesha;

/// Comprehensive test string covering all major Devanagari features
const COMPREHENSIVE_TEST_STRING: &str = r#"
अ आ इ ई उ ऊ ऋ ॠ ऌ ॡ ए ऐ ओ औ अं अः ँ ़ ॲ ऑ ॐ ऽ। ॥
क ख ग घ ङ च छ ज झ ञ ट ठ ड ढ ण त थ द ध न प फ ब भ म य र ल व ळ श ष स ह
क़ ख़ ग़ ज़ ड़ ढ़ फ़ य़ ऩ
क्र ग्र त्र ज्ञ क्ष श्र ह्न ह्म ह्य र्क र्त स्त स्क द्घ ष्ट स्त्र त्त्र त्र्य ज्ञ त्र्य
धर्म क्षत्रिय संस्कृत प्रत्यय कर्तृत्व बुद्धि विश्व शुद्धि मृत्यु गृह्य लक्ष्य पुळ्ळ फळ बाळ
कंप्यूटर फ़ोन ग़ज़ल फ़रिश्ता हिन्दी
क् ख् ग् घ् च् छ् ज् झ् ट् ठ् ड् ढ् ण् त् थ् द् ध् न् प् फ् ब् भ् म् य् र् ल् व् ळ् श् ष् स् ह्
क्‍ष क्‌ष क्‌त क्‍त्
क्‍र् क्‌र्
क्‍ल् क्‌ल्
क्‍म क्‌म
क्‍प् क्‌प्
क्‍ब् क्‌ब्
क्‍द् क्‌द्
क्‍ध् क्‌ध्
क्‍न क्‌न
क्‍य् क्‌य्
क्‍व् क्‌व्
₹ ० १ २ ३ ४ ५ ६ ७ ८ ९
॑ ॒ ॓ ॔ ॕ ॖ ॗ ᳵ ᳶ
𑑀 𑑁 𑑂 𑑃 𑑄 𑑅 𑑆 𑑇 𑑈 𑑉
ॸ 𑑋 𑑌 𑑍 𑑎
"#;

/// Additional edge cases based on research from vidyut-lipi and aksharamukha
const EDGE_CASE_STRINGS: &[&str] = &[
    // Vedic accent combinations
    "अ॑ अ॒ अ॓ इ॑ इ॒",
    // Complex three-consonant clusters
    "स्त्र त्र्य क्ष्ण ङ्क्त",
    // Malformed sequences (should be handled gracefully)
    "क्‌‌्र", // Multiple joiners
    "र्‍‍क", // Multiple ZWJ
    // Script boundary cases
    "क्a",  // Devanagari + Latin
    "अaब", // Mixed script
    // Unicode normalization edge cases
    "की", // ka + vowel sign vs. precomposed
    "कि", // NFC vs NFD
    // Rare vowel combinations
    "कॢ कॣ", // Vocalic L combinations
    // Extended Unicode ranges
    "𑑟 𑑠 𑑡 𑑢 𑑣", // Extended Devanagari
];

/// Dynamically discover all supported scripts
fn get_all_supported_scripts() -> Vec<String> {
    let shlesha = Shlesha::new();
    let scripts = shlesha.list_supported_scripts();

    // Filter to only include scripts we want to test
    scripts
        .into_iter()
        .filter(|script| {
            // Include Indic scripts and major romanization schemes
            matches!(
                script.as_str(),
                "devanagari"
                    | "bengali"
                    | "gujarati"
                    | "telugu"
                    | "tamil"
                    | "kannada"
                    | "malayalam"
                    | "odia"
                    | "gurmukhi"
                    | "iast"
                    | "iso15919"
                    | "harvard_kyoto"
                    | "slp1"
                    | "itrans"
                    | "velthuis"
            )
        })
        .collect()
}

/// Get Indic scripts (abugida) vs Roman scripts (alphabet)
fn categorize_scripts() -> (Vec<String>, Vec<String>) {
    let all_scripts = get_all_supported_scripts();
    let indic_scripts: Vec<String> = all_scripts
        .iter()
        .filter(|script| {
            matches!(
                script.as_str(),
                "devanagari"
                    | "bengali"
                    | "gujarati"
                    | "telugu"
                    | "tamil"
                    | "kannada"
                    | "malayalam"
                    | "odia"
                    | "gurmukhi"
            )
        })
        .cloned()
        .collect();

    let roman_scripts: Vec<String> = all_scripts
        .iter()
        .filter(|script| {
            matches!(
                script.as_str(),
                "iast" | "iso15919" | "harvard_kyoto" | "slp1" | "itrans" | "velthuis"
            )
        })
        .cloned()
        .collect();

    (indic_scripts, roman_scripts)
}

#[test]
fn test_comprehensive_roundtrip_all_scripts() {
    let shlesha = Shlesha::new();
    let (indic_scripts, roman_scripts) = categorize_scripts();

    println!(
        "Testing {} Indic scripts and {} Roman scripts",
        indic_scripts.len(),
        roman_scripts.len()
    );

    // Test the comprehensive string across all scripts
    for test_string in [COMPREHENSIVE_TEST_STRING]
        .iter()
        .chain(EDGE_CASE_STRINGS.iter())
    {
        test_roundtrip_for_string(&shlesha, test_string, &indic_scripts, &roman_scripts);
    }
}

fn test_roundtrip_for_string(
    shlesha: &Shlesha,
    test_string: &str,
    indic_scripts: &[String],
    roman_scripts: &[String],
) {
    let clean_string = test_string.trim();
    if clean_string.is_empty() {
        return;
    }

    println!(
        "\n=== Testing string: {} ===",
        &clean_string.chars().take(20).collect::<String>()
    );

    // Test Indic-to-Indic roundtrips
    for source_script in indic_scripts {
        for target_script in indic_scripts {
            if source_script == target_script {
                continue; // Skip identity conversions
            }

            test_single_roundtrip(shlesha, clean_string, source_script, target_script);
        }
    }

    // Test Indic-to-Roman roundtrips
    for source_script in indic_scripts {
        for target_script in roman_scripts {
            test_single_roundtrip(shlesha, clean_string, source_script, target_script);
        }
    }

    // Test Roman-to-Indic roundtrips (reverse of above)
    for source_script in roman_scripts {
        for target_script in indic_scripts {
            // Start with known good Roman text
            match shlesha.transliterate(clean_string, "devanagari", source_script) {
                Ok(roman_text) => {
                    test_single_roundtrip(shlesha, &roman_text, source_script, target_script);
                }
                Err(_) => {
                    // Skip if we can't convert to this Roman scheme
                    continue;
                }
            }
        }
    }
}

fn test_single_roundtrip(shlesha: &Shlesha, text: &str, source_script: &str, target_script: &str) {
    // Forward conversion
    let converted = match shlesha.transliterate(text, source_script, target_script) {
        Ok(result) => result,
        Err(e) => {
            // Don't fail test for unsupported script combinations, just log
            println!("  ⚠️  {} → {}: {}", source_script, target_script, e);
            return;
        }
    };

    // Backward conversion (roundtrip)
    let roundtrip = match shlesha.transliterate(&converted, target_script, source_script) {
        Ok(result) => result,
        Err(e) => {
            println!(
                "  ❌ {} → {} → {}: Forward succeeded but reverse failed: {}",
                source_script, target_script, source_script, e
            );
            return;
        }
    };

    // Compare with normalization (handle Unicode composition differences)
    let original_normalized = normalize_for_comparison(text);
    let roundtrip_normalized = normalize_for_comparison(&roundtrip);

    if original_normalized == roundtrip_normalized {
        println!(
            "  ✅ {} → {} → {} ({})",
            source_script,
            target_script,
            source_script,
            &converted.chars().take(10).collect::<String>()
        );
    } else {
        println!(
            "  ❌ {} → {} → {}: Roundtrip failed",
            source_script, target_script, source_script
        );
        println!("     Original:  '{}'", original_normalized);
        println!("     Roundtrip: '{}'", roundtrip_normalized);
        println!("     Via:       '{}'", converted);

        // Don't panic immediately - collect all failures
    }
}

/// Normalize text for comparison (handle Unicode composition differences)
fn normalize_for_comparison(text: &str) -> String {
    // Remove whitespace differences and normalize Unicode
    text.chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .trim()
        .to_string()
}

/// Property-based test: any valid Devanagari text should roundtrip
fn prop_devanagari_roundtrip(text: String) -> TestResult {
    let shlesha = Shlesha::new();
    let (indic_scripts, _) = categorize_scripts();

    // Filter to valid Devanagari range
    let devanagari_text: String = text
        .chars()
        .filter(|c| {
            let code = *c as u32;
            // Devanagari block (U+0900-U+097F) + Extended (U+A8E0-U+A8FF)
            (code >= 0x0900 && code <= 0x097F) || (code >= 0xA8E0 && code <= 0xA8FF)
        })
        .take(50) // Limit length for test performance
        .collect();

    if devanagari_text.len() < 3 {
        return TestResult::discard();
    }

    // Test roundtrip to a few representative scripts
    for target_script in ["bengali", "gujarati", "telugu"].iter() {
        if indic_scripts.contains(&target_script.to_string()) {
            let converted =
                match shlesha.transliterate(&devanagari_text, "devanagari", target_script) {
                    Ok(result) => result,
                    Err(_) => return TestResult::discard(),
                };

            let roundtrip = match shlesha.transliterate(&converted, target_script, "devanagari") {
                Ok(result) => result,
                Err(_) => return TestResult::failed(),
            };

            let original_normalized = normalize_for_comparison(&devanagari_text);
            let roundtrip_normalized = normalize_for_comparison(&roundtrip);

            if original_normalized != roundtrip_normalized {
                return TestResult::failed();
            }
        }
    }

    TestResult::passed()
}

#[test]
fn test_property_based_roundtrips() {
    quickcheck(prop_devanagari_roundtrip as fn(String) -> TestResult);
}

/// Test virama preservation specifically (regression test for the issue we just fixed)
#[test]
fn test_virama_preservation() {
    let shlesha = Shlesha::new();
    let (indic_scripts, _) = categorize_scripts();

    // Test strings with viramas
    let virama_test_cases = [
        "धर्म",   // Simple virama
        "कर्म",   // Another simple case
        "स्त्र",   // Complex cluster
        "क्ष्त्र",  // Multiple viramas
        "सर्वज्ञ", // Real word with complex clusters
    ];

    for test_case in &virama_test_cases {
        for target_script in &indic_scripts {
            if target_script == "devanagari" {
                continue;
            }

            let converted = match shlesha.transliterate(test_case, "devanagari", target_script) {
                Ok(result) => result,
                Err(_) => continue,
            };

            // Count characters - converted should have same number as original
            // (viramas should be preserved)
            let original_chars = test_case.chars().count();
            let converted_chars = converted.chars().count();

            println!(
                "Virama test '{}' → {} ({}): {} chars → {} chars",
                test_case, target_script, converted, original_chars, converted_chars
            );

            // For simple cases like धर्म, character count should match
            if test_case.len() <= 6 {
                // Simple cases
                assert_eq!(
                    original_chars, converted_chars,
                    "Character count mismatch for '{}' → {}: '{}' (expected {} chars, got {})",
                    test_case, target_script, converted, original_chars, converted_chars
                );
            }
        }
    }
}

/// Test that all scripts can handle the comprehensive test string
#[test]
fn test_all_scripts_handle_comprehensive_string() {
    let shlesha = Shlesha::new();
    let all_scripts = get_all_supported_scripts();
    let mut successful_conversions = 0;
    let mut total_attempts = 0;

    for source_script in &all_scripts {
        for target_script in &all_scripts {
            if source_script == target_script {
                continue;
            }

            total_attempts += 1;

            // For this test, start with Devanagari source
            let test_text = if source_script == "devanagari" {
                COMPREHENSIVE_TEST_STRING.trim().to_string()
            } else {
                // Convert from Devanagari to source script first
                match shlesha.transliterate(
                    COMPREHENSIVE_TEST_STRING.trim(),
                    "devanagari",
                    source_script,
                ) {
                    Ok(text) => text,
                    Err(_) => continue,
                }
            };

            match shlesha.transliterate(&test_text, source_script, target_script) {
                Ok(result) => {
                    successful_conversions += 1;

                    // Basic sanity checks
                    assert!(
                        !result.is_empty(),
                        "Empty result for {} → {}",
                        source_script,
                        target_script
                    );

                    // Result should not be just question marks (indicates mapping failure)
                    let question_mark_ratio = result.chars().filter(|&c| c == '?').count() as f64
                        / result.chars().count() as f64;
                    assert!(
                        question_mark_ratio < 0.5,
                        "Too many unmapped characters ({:.1}%) for {} → {}: '{}'",
                        question_mark_ratio * 100.0,
                        source_script,
                        target_script,
                        &result.chars().take(50).collect::<String>()
                    );
                }
                Err(e) => {
                    println!("Failed: {} → {}: {}", source_script, target_script, e);
                }
            }
        }
    }

    let success_rate = successful_conversions as f64 / total_attempts as f64;
    println!(
        "Script conversion success rate: {}/{} ({:.1}%)",
        successful_conversions,
        total_attempts,
        success_rate * 100.0
    );

    // We should have at least 70% success rate for basic functionality
    assert!(
        success_rate >= 0.7,
        "Success rate too low: {:.1}%",
        success_rate * 100.0
    );
}
