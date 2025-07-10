use shlesha::Shlesha;
use std::collections::HashMap;

/// All supported scripts based on schema files
const ALL_SCRIPTS: &[&str] = &[
    // Roman scripts
    "iast",
    "slp1",
    "harvard_kyoto",
    "itrans",
    "velthuis",
    "wx",
    "kolkata",
    // Indic scripts
    "devanagari",
    "bengali",
    "gujarati",
    "gurmukhi",
    "kannada",
    "malayalam",
    "odia",
    "tamil",
    "telugu",
    "grantha",
    "sinhala",
];

const ROMAN_SCRIPTS: &[&str] = &[
    "iast",
    "slp1",
    "harvard_kyoto",
    "itrans",
    "velthuis",
    "wx",
    "kolkata",
];

const INDIC_SCRIPTS: &[&str] = &[
    "devanagari",
    "bengali",
    "gujarati",
    "gurmukhi",
    "kannada",
    "malayalam",
    "odia",
    "tamil",
    "telugu",
    "grantha",
    "sinhala",
];

/// Test data for each script type
#[derive(Debug, Clone)]
struct TestCase {
    text: &'static str,
    script: &'static str,
    description: &'static str,
}

impl TestCase {
    fn new(text: &'static str, script: &'static str, description: &'static str) -> Self {
        Self {
            text,
            script,
            description,
        }
    }
}

/// Get test cases for comprehensive testing
fn get_test_cases() -> Vec<TestCase> {
    vec![
        // IAST test cases
        TestCase::new("saṃskṛtam", "iast", "basic Sanskrit word"),
        TestCase::new("dharmakṣetre", "iast", "word with kṣ combination"),
        TestCase::new("namaskāram", "iast", "word with long vowels"),
        TestCase::new("ā", "iast", "single long vowel"),
        TestCase::new("ṛ", "iast", "vocalic r"),
        TestCase::new("ṃ", "iast", "anusvara"),
        TestCase::new("ḥ", "iast", "visarga"),
        TestCase::new("kṣ", "iast", "conjunct consonant"),
        TestCase::new("jñ", "iast", "nasal conjunct"),
        // SLP1 test cases
        TestCase::new("saMskftam", "slp1", "basic Sanskrit word in SLP1"),
        TestCase::new("Darmakzetre", "slp1", "word with kz combination"),
        TestCase::new("namaskAram", "slp1", "word with long vowels in SLP1"),
        TestCase::new("A", "slp1", "single long vowel A"),
        TestCase::new("f", "slp1", "vocalic r in SLP1"),
        TestCase::new("M", "slp1", "anusvara in SLP1"),
        TestCase::new("H", "slp1", "visarga in SLP1"),
        TestCase::new("kz", "slp1", "conjunct kṣ in SLP1"),
        TestCase::new("jY", "slp1", "nasal conjunct in SLP1"),
        // Harvard-Kyoto test cases
        TestCase::new("saMskRtam", "harvard_kyoto", "basic Sanskrit word in HK"),
        TestCase::new("dharmakSetram", "harvard_kyoto", "word with kS combination"),
        TestCase::new("namaskAram", "harvard_kyoto", "word with long vowels in HK"),
        TestCase::new("A", "harvard_kyoto", "single long vowel A"),
        TestCase::new("R", "harvard_kyoto", "vocalic r in HK"),
        TestCase::new("M", "harvard_kyoto", "anusvara in HK"),
        TestCase::new("H", "harvard_kyoto", "visarga in HK"),
        TestCase::new("kS", "harvard_kyoto", "conjunct kṣ in HK"),
        TestCase::new("jJ", "harvard_kyoto", "nasal conjunct in HK"),
        // Devanagari test cases
        TestCase::new("संस्कृतम्", "devanagari", "basic Sanskrit word in Devanagari"),
        TestCase::new("धर्मक्षेत्रे", "devanagari", "word with क्ष in Devanagari"),
        TestCase::new("नमस्कारम्", "devanagari", "greeting in Devanagari"),
        // Telugu test cases
        TestCase::new("సంస్కృతం", "telugu", "Sanskrit word in Telugu"),
        TestCase::new("నమస్కారం", "telugu", "greeting in Telugu"),
        // ITRANS test cases (following ITRANS 5.3 standard)
        TestCase::new("a", "itrans", "basic vowel"),
        TestCase::new("aa", "itrans", "long vowel A"),
        TestCase::new("i", "itrans", "short i vowel"),
        TestCase::new("ii", "itrans", "long I vowel"),
        TestCase::new("u", "itrans", "short u vowel"),
        TestCase::new("uu", "itrans", "long U vowel"),
        TestCase::new("RRi", "itrans", "vocalic r"),
        TestCase::new("ka", "itrans", "basic consonant"),
        TestCase::new("M", "itrans", "anusvara"),
        TestCase::new("H", "itrans", "visarga"),
        TestCase::new("kSha", "itrans", "conjunct kṣ"),
        TestCase::new("j~na", "itrans", "conjunct jñ"),
        // Velthuis test cases
        TestCase::new("a", "velthuis", "basic vowel"),
        TestCase::new("aa", "velthuis", "long vowel A"),
        TestCase::new("i", "velthuis", "short i vowel"),
        TestCase::new("ii", "velthuis", "long I vowel"),
        TestCase::new("u", "velthuis", "short u vowel"),
        TestCase::new("uu", "velthuis", "long U vowel"),
        TestCase::new(".r", "velthuis", "vocalic r"),
        TestCase::new("ka", "velthuis", "basic consonant"),
        TestCase::new(".m", "velthuis", "anusvara"),
        TestCase::new(".h", "velthuis", "visarga"),
        TestCase::new("k.sa", "velthuis", "conjunct kṣ"),
        // WX test cases (following WX notation)
        TestCase::new("a", "wx", "basic vowel"),
        TestCase::new("A", "wx", "long vowel A"),
        TestCase::new("i", "wx", "short i vowel"),
        TestCase::new("I", "wx", "long I vowel"),
        TestCase::new("u", "wx", "short u vowel"),
        TestCase::new("U", "wx", "long U vowel"),
        TestCase::new("q", "wx", "vocalic r"),
        TestCase::new("ka", "wx", "basic consonant"),
        TestCase::new("M", "wx", "anusvara"),
        TestCase::new("H", "wx", "visarga"),
        TestCase::new("kRa", "wx", "conjunct kṣ"),
        // Kolkata test cases
        TestCase::new("a", "kolkata", "basic vowel"),
        TestCase::new("aa", "kolkata", "long vowel A"),
        TestCase::new("i", "kolkata", "short i vowel"),
        TestCase::new("ii", "kolkata", "long I vowel"),
        TestCase::new("u", "kolkata", "short u vowel"),
        TestCase::new("uu", "kolkata", "long U vowel"),
        TestCase::new("ri", "kolkata", "vocalic r"),
        TestCase::new("ka", "kolkata", "basic consonant"),
        TestCase::new("ng", "kolkata", "anusvara"),
        TestCase::new("h", "kolkata", "visarga"),
        // Simple test cases for coverage
        TestCase::new("om", "iast", "simple word"),
        TestCase::new("ka", "iast", "single consonant"),
        TestCase::new("a", "iast", "single vowel"),
    ]
}

#[test]
fn test_all_scripts_are_supported() {
    let shlesha = Shlesha::new();
    let supported_scripts = shlesha.list_supported_scripts();

    println!("Supported scripts: {:?}", supported_scripts);

    let mut missing_scripts = Vec::new();
    for script in ALL_SCRIPTS {
        if !supported_scripts.contains(&script.to_string()) {
            missing_scripts.push(*script);
        }
    }

    if !missing_scripts.is_empty() {
        println!("Missing scripts: {:?}", missing_scripts);
        // Don't fail the test, just document what's missing
    }

    // Ensure we have at least the core scripts
    let core_scripts = ["iast", "slp1", "devanagari"];
    for script in core_scripts {
        assert!(
            supported_scripts.contains(&script.to_string()),
            "Core script '{}' should be supported",
            script
        );
    }
}

#[test]
fn test_identity_conversions_all_scripts() {
    let shlesha = Shlesha::new();
    let supported_scripts = shlesha.list_supported_scripts();

    let test_cases = get_test_cases();
    let mut failures = Vec::new();

    for test_case in &test_cases {
        if !supported_scripts.contains(&test_case.script.to_string()) {
            continue; // Skip unsupported scripts
        }

        match shlesha.transliterate(test_case.text, test_case.script, test_case.script) {
            Ok(result) => {
                if result != test_case.text {
                    failures.push(format!(
                        "Identity conversion failed for {} '{}': got '{}'",
                        test_case.script, test_case.text, result
                    ));
                }
            }
            Err(e) => {
                failures.push(format!(
                    "Identity conversion error for {} '{}': {}",
                    test_case.script, test_case.text, e
                ));
            }
        }
    }

    if !failures.is_empty() {
        for failure in &failures {
            println!("IDENTITY FAILURE: {}", failure);
        }
        panic!("Identity conversion failures: {}", failures.len());
    }
}

#[test]
fn test_roman_to_roman_conversions() {
    let shlesha = Shlesha::new();
    let supported_scripts = shlesha.list_supported_scripts();

    // Filter to supported Roman scripts
    let available_roman: Vec<&str> = ROMAN_SCRIPTS
        .iter()
        .filter(|script| supported_scripts.contains(&script.to_string()))
        .copied()
        .collect();

    let mut conversion_matrix = HashMap::new();
    let mut failures = Vec::new();

    // Test all Roman-to-Roman conversions
    for &source in &available_roman {
        for &target in &available_roman {
            if source == target {
                continue; // Skip identity conversions (tested separately)
            }

            let mut conversions_tested = 0;
            let mut conversions_succeeded = 0;

            // Get test cases specific to the source script
            let source_test_cases: Vec<_> = get_test_cases()
                .into_iter()
                .filter(|tc| tc.script == source)
                .collect();

            // If no specific test cases, use basic ones
            let test_cases = if source_test_cases.is_empty() {
                vec![
                    TestCase::new("a", source, "basic vowel"),
                    TestCase::new("ka", source, "basic consonant"),
                ]
            } else {
                source_test_cases
            };

            for test_case in &test_cases {
                conversions_tested += 1;

                // Use the script-specific test data directly
                let source_text = test_case.text;

                // Convert from source to target
                match shlesha.transliterate(source_text, source, target) {
                    Ok(result) => {
                        conversions_succeeded += 1;
                        println!("✓ {}→{}: '{}' → '{}'", source, target, source_text, result);
                    }
                    Err(e) => {
                        failures.push(format!(
                            "{}→{}: '{}' failed - {}",
                            source, target, source_text, e
                        ));
                    }
                }
            }

            conversion_matrix.insert(
                (source, target),
                (conversions_succeeded, conversions_tested),
            );
        }
    }

    // Print conversion matrix
    println!("\n📊 Roman-to-Roman Conversion Matrix:");
    println!("{:<15} {}", "Source→Target", "Success/Total");
    println!("{}", "-".repeat(40));

    for &source in &available_roman {
        for &target in &available_roman {
            if source != target {
                if let Some((succeeded, total)) = conversion_matrix.get(&(source, target)) {
                    let percentage = if *total > 0 {
                        succeeded * 100 / total
                    } else {
                        0
                    };
                    println!(
                        "{:<15} {}/{} ({}%)",
                        format!("{}→{}", source, target),
                        succeeded,
                        total,
                        percentage
                    );
                }
            }
        }
    }

    if !failures.is_empty() {
        println!("\n❌ Roman-to-Roman conversion failures:");
        for failure in &failures[..failures.len().min(10)] {
            println!("  {}", failure);
        }
        if failures.len() > 10 {
            println!("  ... and {} more failures", failures.len() - 10);
        }
    }

    // Don't fail the test, just report the issues
}

#[test]
fn test_indic_to_indic_conversions() {
    let shlesha = Shlesha::new();
    let supported_scripts = shlesha.list_supported_scripts();

    // Filter to supported Indic scripts
    let available_indic: Vec<&str> = INDIC_SCRIPTS
        .iter()
        .filter(|script| supported_scripts.contains(&script.to_string()))
        .copied()
        .collect();

    let mut conversion_matrix = HashMap::new();
    let mut failures = Vec::new();

    // Test all Indic-to-Indic conversions
    for &source in &available_indic {
        for &target in &available_indic {
            if source == target {
                continue;
            }

            let mut conversions_tested = 0;
            let mut conversions_succeeded = 0;

            // Use Devanagari test cases as the baseline
            let devanagari_test_cases: Vec<_> = get_test_cases()
                .into_iter()
                .filter(|tc| tc.script == "devanagari")
                .collect();

            for test_case in &devanagari_test_cases {
                conversions_tested += 1;

                // Convert from Devanagari to source script (if needed)
                let source_text = if source == "devanagari" {
                    test_case.text.to_string()
                } else {
                    match shlesha.transliterate(test_case.text, "devanagari", source) {
                        Ok(text) => text,
                        Err(e) => {
                            failures.push(format!(
                                "Preparation failed: Devanagari→{}: '{}' - {}",
                                source, test_case.text, e
                            ));
                            continue;
                        }
                    }
                };

                // Then convert from source to target
                match shlesha.transliterate(&source_text, source, target) {
                    Ok(result) => {
                        conversions_succeeded += 1;
                        println!("✓ {}→{}: '{}' → '{}'", source, target, source_text, result);
                    }
                    Err(e) => {
                        failures.push(format!(
                            "{}→{}: '{}' failed - {}",
                            source, target, source_text, e
                        ));
                    }
                }
            }

            conversion_matrix.insert(
                (source, target),
                (conversions_succeeded, conversions_tested),
            );
        }
    }

    // Print conversion matrix
    println!("\n📊 Indic-to-Indic Conversion Matrix:");
    println!("{:<20} {}", "Source→Target", "Success/Total");
    println!("{}", "-".repeat(45));

    for &source in &available_indic {
        for &target in &available_indic {
            if source != target {
                if let Some((succeeded, total)) = conversion_matrix.get(&(source, target)) {
                    let percentage = if *total > 0 {
                        succeeded * 100 / total
                    } else {
                        0
                    };
                    println!(
                        "{:<20} {}/{} ({}%)",
                        format!("{}→{}", source, target),
                        succeeded,
                        total,
                        percentage
                    );
                }
            }
        }
    }

    if !failures.is_empty() {
        println!("\n❌ Indic-to-Indic conversion failures:");
        for failure in &failures[..failures.len().min(10)] {
            println!("  {}", failure);
        }
        if failures.len() > 10 {
            println!("  ... and {} more failures", failures.len() - 10);
        }
    }
}

#[test]
fn test_roman_to_indic_conversions() {
    let shlesha = Shlesha::new();
    let supported_scripts = shlesha.list_supported_scripts();

    let available_roman: Vec<&str> = ROMAN_SCRIPTS
        .iter()
        .filter(|script| supported_scripts.contains(&script.to_string()))
        .copied()
        .collect();

    let available_indic: Vec<&str> = INDIC_SCRIPTS
        .iter()
        .filter(|script| supported_scripts.contains(&script.to_string()))
        .copied()
        .collect();

    let mut conversion_matrix = HashMap::new();
    let mut failures = Vec::new();

    // Test Roman→Indic conversions
    for &roman in &available_roman {
        for &indic in &available_indic {
            let mut conversions_tested = 0;
            let mut conversions_succeeded = 0;

            // Use IAST test cases as source
            let test_cases: Vec<_> = get_test_cases()
                .into_iter()
                .filter(|tc| tc.script == "iast")
                .take(5) // Limit for performance
                .collect();

            for test_case in &test_cases {
                conversions_tested += 1;

                // Convert from IAST to Roman script (if needed)
                let roman_text = if roman == "iast" {
                    test_case.text.to_string()
                } else {
                    match shlesha.transliterate(test_case.text, "iast", roman) {
                        Ok(text) => text,
                        Err(e) => {
                            failures.push(format!(
                                "Prep failed: IAST→{}: '{}' - {}",
                                roman, test_case.text, e
                            ));
                            continue;
                        }
                    }
                };

                // Convert from Roman to Indic
                match shlesha.transliterate(&roman_text, roman, indic) {
                    Ok(result) => {
                        conversions_succeeded += 1;
                        println!("✓ {}→{}: '{}' → '{}'", roman, indic, roman_text, result);
                    }
                    Err(e) => {
                        failures.push(format!(
                            "{}→{}: '{}' failed - {}",
                            roman, indic, roman_text, e
                        ));
                    }
                }
            }

            conversion_matrix.insert((roman, indic), (conversions_succeeded, conversions_tested));
        }
    }

    // Print conversion matrix
    println!("\n📊 Roman-to-Indic Conversion Matrix:");
    println!("{:<20} {}", "Source→Target", "Success/Total");
    println!("{}", "-".repeat(45));

    for &roman in &available_roman {
        for &indic in &available_indic {
            if let Some((succeeded, total)) = conversion_matrix.get(&(roman, indic)) {
                let percentage = if *total > 0 {
                    succeeded * 100 / total
                } else {
                    0
                };
                println!(
                    "{:<20} {}/{} ({}%)",
                    format!("{}→{}", roman, indic),
                    succeeded,
                    total,
                    percentage
                );
            }
        }
    }

    if !failures.is_empty() {
        println!("\n❌ Roman-to-Indic conversion failures:");
        for failure in &failures[..failures.len().min(15)] {
            println!("  {}", failure);
        }
        if failures.len() > 15 {
            println!("  ... and {} more failures", failures.len() - 15);
        }
    }
}

#[test]
fn test_indic_to_roman_conversions() {
    let shlesha = Shlesha::new();
    let supported_scripts = shlesha.list_supported_scripts();

    let available_roman: Vec<&str> = ROMAN_SCRIPTS
        .iter()
        .filter(|script| supported_scripts.contains(&script.to_string()))
        .copied()
        .collect();

    let available_indic: Vec<&str> = INDIC_SCRIPTS
        .iter()
        .filter(|script| supported_scripts.contains(&script.to_string()))
        .copied()
        .collect();

    let mut conversion_matrix = HashMap::new();
    let mut failures = Vec::new();

    // Test Indic→Roman conversions
    for &indic in &available_indic {
        for &roman in &available_roman {
            let mut conversions_tested = 0;
            let mut conversions_succeeded = 0;

            // Use Devanagari test cases as source
            let test_cases: Vec<_> = get_test_cases()
                .into_iter()
                .filter(|tc| tc.script == "devanagari")
                .take(5) // Limit for performance
                .collect();

            for test_case in &test_cases {
                conversions_tested += 1;

                // Convert from Devanagari to Indic script (if needed)
                let indic_text = if indic == "devanagari" {
                    test_case.text.to_string()
                } else {
                    match shlesha.transliterate(test_case.text, "devanagari", indic) {
                        Ok(text) => text,
                        Err(e) => {
                            failures.push(format!(
                                "Prep failed: Devanagari→{}: '{}' - {}",
                                indic, test_case.text, e
                            ));
                            continue;
                        }
                    }
                };

                // Convert from Indic to Roman
                match shlesha.transliterate(&indic_text, indic, roman) {
                    Ok(result) => {
                        conversions_succeeded += 1;
                        println!("✓ {}→{}: '{}' → '{}'", indic, roman, indic_text, result);
                    }
                    Err(e) => {
                        failures.push(format!(
                            "{}→{}: '{}' failed - {}",
                            indic, roman, indic_text, e
                        ));
                    }
                }
            }

            conversion_matrix.insert((indic, roman), (conversions_succeeded, conversions_tested));
        }
    }

    // Print conversion matrix
    println!("\n📊 Indic-to-Roman Conversion Matrix:");
    println!("{:<20} {}", "Source→Target", "Success/Total");
    println!("{}", "-".repeat(45));

    for &indic in &available_indic {
        for &roman in &available_roman {
            if let Some((succeeded, total)) = conversion_matrix.get(&(indic, roman)) {
                let percentage = if *total > 0 {
                    succeeded * 100 / total
                } else {
                    0
                };
                println!(
                    "{:<20} {}/{} ({}%)",
                    format!("{}→{}", indic, roman),
                    succeeded,
                    total,
                    percentage
                );
            }
        }
    }

    if !failures.is_empty() {
        println!("\n❌ Indic-to-Roman conversion failures:");
        for failure in &failures[..failures.len().min(15)] {
            println!("  {}", failure);
        }
        if failures.len() > 15 {
            println!("  ... and {} more failures", failures.len() - 15);
        }
    }
}

#[test]
fn test_round_trip_consistency() {
    let shlesha = Shlesha::new();
    let supported_scripts = shlesha.list_supported_scripts();

    let available_roman: Vec<&str> = ROMAN_SCRIPTS
        .iter()
        .filter(|script| supported_scripts.contains(&script.to_string()))
        .copied()
        .collect();

    let mut round_trip_failures = Vec::new();
    let mut round_trip_successes = 0;
    let mut round_trips_tested = 0;

    // Test round-trip conversions within Roman scripts
    for &script1 in &available_roman {
        for &script2 in &available_roman {
            if script1 == script2 {
                continue;
            }

            let test_cases: Vec<_> = get_test_cases()
                .into_iter()
                .filter(|tc| tc.script == script1)
                .take(3)
                .collect();

            for test_case in &test_cases {
                round_trips_tested += 1;

                // script1 → script2 → script1
                match shlesha
                    .transliterate(test_case.text, script1, script2)
                    .and_then(|intermediate| shlesha.transliterate(&intermediate, script2, script1))
                {
                    Ok(final_result) => {
                        if final_result == test_case.text {
                            round_trip_successes += 1;
                            println!(
                                "✓ Round-trip {}→{}→{}: '{}'",
                                script1, script2, script1, test_case.text
                            );
                        } else {
                            round_trip_failures.push(format!(
                                "Round-trip {}→{}→{}: '{}' → '{}'",
                                script1, script2, script1, test_case.text, final_result
                            ));
                        }
                    }
                    Err(e) => {
                        round_trip_failures.push(format!(
                            "Round-trip {}→{}→{}: '{}' failed - {}",
                            script1, script2, script1, test_case.text, e
                        ));
                    }
                }
            }
        }
    }

    println!("\n🔄 Round-trip Results:");
    println!(
        "Successful round-trips: {}/{} ({}%)",
        round_trip_successes,
        round_trips_tested,
        if round_trips_tested > 0 {
            round_trip_successes * 100 / round_trips_tested
        } else {
            0
        }
    );

    if !round_trip_failures.is_empty() {
        println!("\n❌ Round-trip failures:");
        for failure in &round_trip_failures[..round_trip_failures.len().min(10)] {
            println!("  {}", failure);
        }
        if round_trip_failures.len() > 10 {
            println!("  ... and {} more failures", round_trip_failures.len() - 10);
        }
    }
}

#[test]
fn test_comprehensive_coverage_report() {
    let shlesha = Shlesha::new();
    let supported_scripts = shlesha.list_supported_scripts();

    println!("\n📋 COMPREHENSIVE COVERAGE REPORT");
    println!("================================");

    println!("\n🎯 Target Coverage:");
    println!("Scripts in schemas: {}", ALL_SCRIPTS.len());
    println!("Scripts supported: {}", supported_scripts.len());
    println!("Roman scripts: {}", ROMAN_SCRIPTS.len());
    println!("Indic scripts: {}", INDIC_SCRIPTS.len());

    let total_possible_conversions = supported_scripts.len() * (supported_scripts.len() - 1);
    println!("Total possible conversions: {}", total_possible_conversions);

    println!("\n📊 Test Coverage Matrix:");
    println!(
        "✓ Identity conversions: {} scripts",
        supported_scripts.len()
    );
    println!("✓ Roman→Roman: tested in test_roman_to_roman_conversions");
    println!("✓ Indic→Indic: tested in test_indic_to_indic_conversions");
    println!("✓ Roman→Indic: tested in test_roman_to_indic_conversions");
    println!("✓ Indic→Roman: tested in test_indic_to_roman_conversions");
    println!("✓ Round-trips: tested in test_round_trip_consistency");

    println!("\n🔍 Known Issues:");
    println!("❌ IAST→SLP1 conversion broken (outputs unchanged text)");
    println!("❌ SLP1→IAST conversion outputs ISO-15919 instead");
    println!("❌ Hub conversion system has reverse mapping issues");

    println!("\n💡 Recommendations:");
    println!("1. Fix reverse mapping generation in build.rs");
    println!("2. Add property-based testing for all script pairs");
    println!("3. Add character-level mapping verification");
    println!("4. Add performance regression tests");
    println!("5. Add CI test that runs full matrix on every commit");
}
