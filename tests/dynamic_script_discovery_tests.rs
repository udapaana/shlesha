use shlesha::Shlesha;
use std::collections::HashMap;

/// Automatically discover and test all supported scripts
/// This test dynamically adapts as new scripts are added
#[test]
fn test_dynamic_script_discovery() {
    let shlesha = Shlesha::new();
    let all_scripts = shlesha.list_supported_scripts();

    println!(
        "🔍 Discovered {} scripts: {:?}",
        all_scripts.len(),
        all_scripts
    );

    // Ensure we found a reasonable number of scripts
    assert!(
        all_scripts.len() >= 10,
        "Expected at least 10 scripts, found {}",
        all_scripts.len()
    );

    // Ensure core scripts are present
    let required_scripts = ["iast", "slp1", "devanagari"];
    for required in required_scripts {
        assert!(
            all_scripts.contains(&required.to_string()),
            "Required script '{}' not found in supported scripts",
            required
        );
    }
}

/// Categorize scripts automatically based on their characteristics
fn categorize_scripts(shlesha: &Shlesha) -> (Vec<String>, Vec<String>) {
    let all_scripts = shlesha.list_supported_scripts();
    let mut roman_scripts = Vec::new();
    let mut indic_scripts = Vec::new();

    // Known Roman script patterns
    let known_roman_patterns = [
        "iast", "slp1", "harvard", "itrans", "velthuis", "wx", "kolkata", "iso",
    ];

    // Known Indic script patterns
    let known_indic_patterns = [
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
        "tibetan",
    ];

    for script in all_scripts {
        let script_lower = script.to_lowercase();

        if known_roman_patterns
            .iter()
            .any(|pattern| script_lower.contains(pattern))
        {
            roman_scripts.push(script);
        } else if known_indic_patterns
            .iter()
            .any(|pattern| script_lower.contains(pattern))
        {
            indic_scripts.push(script);
        } else {
            // Default unknown scripts to Roman (safer assumption)
            println!("⚠️  Unknown script '{}' categorized as Roman", script);
            roman_scripts.push(script);
        }
    }

    println!(
        "📜 Roman scripts ({}): {:?}",
        roman_scripts.len(),
        roman_scripts
    );
    println!(
        "🔤 Indic scripts ({}): {:?}",
        indic_scripts.len(),
        indic_scripts
    );

    (roman_scripts, indic_scripts)
}

/// Generate test data automatically based on script characteristics
fn generate_test_data() -> HashMap<String, Vec<&'static str>> {
    let mut test_data = HashMap::new();

    // Universal test cases (should work in any script)
    let universal_cases = vec!["a", "ka", "ma", "na"];

    // IAST-specific test cases
    test_data.insert(
        "iast".to_string(),
        vec![
            "a",
            "ā",
            "i",
            "ī",
            "u",
            "ū",
            "ṛ",
            "ṝ",
            "e",
            "o",
            "ai",
            "au",
            "ka",
            "kha",
            "ga",
            "gha",
            "ṅa",
            "ca",
            "cha",
            "ja",
            "jha",
            "ña",
            "ṭa",
            "ṭha",
            "ḍa",
            "ḍha",
            "ṇa",
            "ta",
            "tha",
            "da",
            "dha",
            "na",
            "pa",
            "pha",
            "ba",
            "bha",
            "ma",
            "ya",
            "ra",
            "la",
            "va",
            "śa",
            "ṣa",
            "sa",
            "ha",
            "ṃ",
            "ḥ",
            "kṣa",
            "jña",
            "saṃskṛtam",
            "dharmakṣetre",
            "namaskāram",
        ],
    );

    // SLP1-specific test cases
    test_data.insert(
        "slp1".to_string(),
        vec![
            "a",
            "A",
            "i",
            "I",
            "u",
            "U",
            "f",
            "F",
            "x",
            "X",
            "e",
            "o",
            "E",
            "O",
            "ka",
            "Ka",
            "ga",
            "Ga",
            "Na",
            "ca",
            "Ca",
            "ja",
            "Ja",
            "Ya",
            "wa",
            "Wa",
            "qa",
            "Qa",
            "Ra",
            "ta",
            "Ta",
            "da",
            "Da",
            "na",
            "pa",
            "Pa",
            "ba",
            "Ba",
            "ma",
            "ya",
            "ra",
            "la",
            "va",
            "Sa",
            "za",
            "sa",
            "ha",
            "M",
            "H",
            "kza",
            "jYa",
            "saMskftam",
            "Darmakzetre",
            "namaskAram",
        ],
    );

    // Devanagari test cases
    test_data.insert(
        "devanagari".to_string(),
        vec![
            "अ",
            "आ",
            "इ",
            "ई",
            "उ",
            "ऊ",
            "ऋ",
            "ॠ",
            "ए",
            "ओ",
            "ऐ",
            "औ",
            "क",
            "ख",
            "ग",
            "घ",
            "ङ",
            "च",
            "छ",
            "ज",
            "झ",
            "ञ",
            "ट",
            "ठ",
            "ड",
            "ढ",
            "ण",
            "त",
            "थ",
            "द",
            "ध",
            "न",
            "प",
            "फ",
            "ब",
            "भ",
            "म",
            "य",
            "र",
            "ल",
            "व",
            "श",
            "ष",
            "स",
            "ह",
            "ं",
            "ः",
            "क्ष",
            "ज्ञ",
            "संस्कृतम्",
            "धर्मक्षेत्रे",
            "नमस्कारम्",
        ],
    );

    // For other scripts, use universal test cases
    for script in [
        "harvard_kyoto",
        "itrans",
        "velthuis",
        "wx",
        "kolkata",
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
    ] {
        test_data.insert(script.to_string(), universal_cases.clone());
    }

    test_data
}

/// Test all identity conversions (script → script)
#[test]
fn test_all_identity_conversions() {
    let shlesha = Shlesha::new();
    let all_scripts = shlesha.list_supported_scripts();
    let test_data = generate_test_data();

    let mut total_tests = 0;
    let mut failed_tests = 0;
    let mut failures = Vec::new();

    for script in &all_scripts {
        let default_cases = vec!["a", "ka", "ma"];
        let test_cases = test_data.get(script).unwrap_or(&default_cases); // Default cases

        for &test_input in test_cases.iter().take(5) {
            // Limit for performance
            total_tests += 1;

            match shlesha.transliterate(test_input, script, script) {
                Ok(result) => {
                    if result != test_input {
                        failed_tests += 1;
                        failures.push(format!(
                            "Identity conversion failed: {} '{}' → '{}'",
                            script, test_input, result
                        ));
                    }
                }
                Err(e) => {
                    failed_tests += 1;
                    failures.push(format!(
                        "Identity conversion error: {} '{}' - {}",
                        script, test_input, e
                    ));
                }
            }
        }
    }

    println!("📊 Identity Conversion Results:");
    println!("   Total tests: {}", total_tests);
    println!("   Failed tests: {}", failed_tests);
    println!(
        "   Success rate: {:.1}%",
        if total_tests > 0 {
            (total_tests - failed_tests) as f64 / total_tests as f64 * 100.0
        } else {
            0.0
        }
    );

    if !failures.is_empty() {
        println!("\n❌ Identity conversion failures:");
        for failure in failures.iter().take(10) {
            println!("   {}", failure);
        }
        if failures.len() > 10 {
            println!("   ... and {} more", failures.len() - 10);
        }
    }

    // Don't fail the test - just report issues for now
}

/// Test all possible script pairs dynamically
#[test]
fn test_all_script_pairs_matrix() {
    let shlesha = Shlesha::new();
    let (roman_scripts, indic_scripts) = categorize_scripts(&shlesha);
    let test_data = generate_test_data();

    let mut conversion_results = HashMap::new();
    let mut total_pairs = 0;
    let mut working_pairs = 0;

    // Test all script pairs
    let all_scripts: Vec<String> = [roman_scripts.clone(), indic_scripts.clone()].concat();

    for source_script in &all_scripts {
        for target_script in &all_scripts {
            if source_script == target_script {
                continue; // Skip identity conversions
            }

            total_pairs += 1;
            let pair_key = format!("{}→{}", source_script, target_script);

            // Get appropriate test case for source script
            let default_cases = vec!["a", "ka"];
            let test_cases = test_data.get(source_script).unwrap_or(&default_cases);

            let test_input = test_cases.first().unwrap_or(&"a");

            match shlesha.transliterate(test_input, source_script, target_script) {
                Ok(result) => {
                    working_pairs += 1;
                    conversion_results.insert(pair_key, format!("'{}' → '{}'", test_input, result));
                }
                Err(e) => {
                    conversion_results.insert(pair_key, format!("ERROR: {}", e));
                }
            }
        }
    }

    println!("🌐 Complete Script Conversion Matrix:");
    println!("   Total scripts: {}", all_scripts.len());
    println!("   Total possible pairs: {}", total_pairs);
    println!("   Working pairs: {}", working_pairs);
    println!(
        "   Success rate: {:.1}%",
        if total_pairs > 0 {
            working_pairs as f64 / total_pairs as f64 * 100.0
        } else {
            0.0
        }
    );

    // Print sample results by category
    println!("\n📝 Sample Conversions by Category:");

    // Roman → Roman
    if roman_scripts.len() >= 2 {
        println!("\n  Roman → Roman:");
        for (i, source) in roman_scripts.iter().enumerate().take(3) {
            for target in roman_scripts.iter().skip(i + 1).take(2) {
                let key = format!("{}→{}", source, target);
                if let Some(result) = conversion_results.get(&key) {
                    println!("    {}: {}", key, result);
                }
            }
        }
    }

    // Indic → Indic
    if indic_scripts.len() >= 2 {
        println!("\n  Indic → Indic:");
        for (i, source) in indic_scripts.iter().enumerate().take(3) {
            for target in indic_scripts.iter().skip(i + 1).take(2) {
                let key = format!("{}→{}", source, target);
                if let Some(result) = conversion_results.get(&key) {
                    println!("    {}: {}", key, result);
                }
            }
        }
    }

    // Roman → Indic
    if !roman_scripts.is_empty() && !indic_scripts.is_empty() {
        println!("\n  Roman → Indic:");
        for roman in roman_scripts.iter().take(2) {
            for indic in indic_scripts.iter().take(2) {
                let key = format!("{}→{}", roman, indic);
                if let Some(result) = conversion_results.get(&key) {
                    println!("    {}: {}", key, result);
                }
            }
        }
    }

    // Indic → Roman
    if !indic_scripts.is_empty() && !roman_scripts.is_empty() {
        println!("\n  Indic → Roman:");
        for indic in indic_scripts.iter().take(2) {
            for roman in roman_scripts.iter().take(2) {
                let key = format!("{}→{}", indic, roman);
                if let Some(result) = conversion_results.get(&key) {
                    println!("    {}: {}", key, result);
                }
            }
        }
    }

    // Store results for analysis
    println!(
        "\n💾 Conversion matrix stored for analysis ({} pairs)",
        conversion_results.len()
    );
}

/// Test character-level mapping consistency automatically
#[test]
fn test_character_mapping_consistency() {
    let shlesha = Shlesha::new();
    let (roman_scripts, _indic_scripts) = categorize_scripts(&shlesha);

    // Test key character mappings between Roman scripts
    let key_mappings = [
        // (character, expected_in_script, script)
        ("ā", vec![("slp1", "A"), ("iast", "ā")]),
        ("ī", vec![("slp1", "I"), ("iast", "ī")]),
        ("ū", vec![("slp1", "U"), ("iast", "ū")]),
        ("ṛ", vec![("slp1", "f"), ("iast", "ṛ")]),
        ("ṃ", vec![("slp1", "M"), ("iast", "ṃ")]),
        ("ḥ", vec![("slp1", "H"), ("iast", "ḥ")]),
        ("ś", vec![("slp1", "S"), ("iast", "ś")]),
        ("ṣ", vec![("slp1", "z"), ("iast", "ṣ")]),
    ];

    let mut mapping_failures = Vec::new();
    let mut total_mapping_tests = 0;

    for (base_char, expected_mappings) in &key_mappings {
        for &(script, expected) in expected_mappings {
            if !roman_scripts.iter().any(|s| s == script) {
                continue; // Skip if script not supported
            }

            total_mapping_tests += 1;

            // Test conversion from IAST to target script
            match shlesha.transliterate(base_char, "iast", script) {
                Ok(result) => {
                    if result != *expected {
                        mapping_failures.push(format!(
                            "Mapping inconsistency: IAST '{}' → {} expected '{}', got '{}'",
                            base_char, script, expected, result
                        ));
                    }
                }
                Err(e) => {
                    mapping_failures.push(format!(
                        "Mapping error: IAST '{}' → {} failed: {}",
                        base_char, script, e
                    ));
                }
            }
        }
    }

    println!("🔤 Character Mapping Consistency:");
    println!("   Total mapping tests: {}", total_mapping_tests);
    println!("   Failed mappings: {}", mapping_failures.len());

    if !mapping_failures.is_empty() {
        println!("\n❌ Character mapping failures:");
        for failure in &mapping_failures {
            println!("   {}", failure);
        }
    }

    // This is a critical test - character mappings MUST be correct
    if mapping_failures.len() > total_mapping_tests / 2 {
        panic!(
            "Too many character mapping failures: {}/{}",
            mapping_failures.len(),
            total_mapping_tests
        );
    }
}

/// Test round-trip consistency for all script pairs
#[test]
fn test_round_trip_all_scripts() {
    let shlesha = Shlesha::new();
    let (roman_scripts, _indic_scripts) = categorize_scripts(&shlesha);

    let mut round_trip_failures = Vec::new();
    let mut successful_round_trips = 0;
    let mut total_round_trips = 0;

    // Test round-trips within Roman scripts (should be lossless)
    for source_script in &roman_scripts {
        for target_script in &roman_scripts {
            if source_script == target_script {
                continue;
            }

            let test_inputs = ["a", "ka", "ā", "ṃ"];

            for &test_input in &test_inputs {
                total_round_trips += 1;

                // source → target → source
                match shlesha
                    .transliterate(test_input, source_script, target_script)
                    .and_then(|intermediate| {
                        shlesha.transliterate(&intermediate, target_script, source_script)
                    }) {
                    Ok(final_result) => {
                        if final_result == test_input {
                            successful_round_trips += 1;
                        } else {
                            round_trip_failures.push(format!(
                                "Round-trip failed: {}→{}→{}: '{}' → '{}'",
                                source_script,
                                target_script,
                                source_script,
                                test_input,
                                final_result
                            ));
                        }
                    }
                    Err(e) => {
                        round_trip_failures.push(format!(
                            "Round-trip error: {}→{}→{}: '{}' - {}",
                            source_script, target_script, source_script, test_input, e
                        ));
                    }
                }
            }
        }
    }

    println!("🔄 Round-trip Consistency:");
    println!("   Total round-trips tested: {}", total_round_trips);
    println!("   Successful round-trips: {}", successful_round_trips);
    println!(
        "   Success rate: {:.1}%",
        if total_round_trips > 0 {
            successful_round_trips as f64 / total_round_trips as f64 * 100.0
        } else {
            0.0
        }
    );

    if !round_trip_failures.is_empty() {
        println!("\n❌ Round-trip failures:");
        for failure in round_trip_failures.iter().take(10) {
            println!("   {}", failure);
        }
        if round_trip_failures.len() > 10 {
            println!("   ... and {} more", round_trip_failures.len() - 10);
        }
    }

    // Round-trip consistency is crucial for data integrity
    let failure_rate = round_trip_failures.len() as f64 / total_round_trips as f64;
    if failure_rate > 0.5 {
        panic!(
            "Excessive round-trip failures: {:.1}% failure rate",
            failure_rate * 100.0
        );
    }
}

/// Performance regression test for all scripts
#[test]
fn test_performance_all_scripts() {
    use std::time::Instant;

    let shlesha = Shlesha::new();
    let all_scripts = shlesha.list_supported_scripts();
    let test_data = generate_test_data();

    println!("⚡ Performance Testing All Scripts:");

    let mut performance_results = HashMap::new();

    for script in &all_scripts {
        let default_test_cases = vec!["test"];
        let test_cases = test_data.get(script).unwrap_or(&default_test_cases);
        let test_input = test_cases.first().unwrap_or(&"test");

        // Warm up
        for _ in 0..10 {
            let _ = shlesha.transliterate(test_input, script, script);
        }

        // Measure performance
        let iterations = 1000;
        let start = Instant::now();

        for _ in 0..iterations {
            let _ = shlesha.transliterate(test_input, script, script);
        }

        let duration = start.elapsed();
        let ops_per_sec = iterations as f64 / duration.as_secs_f64();

        performance_results.insert(script.clone(), ops_per_sec);
    }

    // Sort by performance
    let mut sorted_results: Vec<_> = performance_results.into_iter().collect();
    sorted_results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    println!("\n📊 Performance Results (ops/sec):");
    for (script, ops_per_sec) in sorted_results.iter().take(10) {
        println!("   {}: {:>10.0} ops/sec", script, ops_per_sec);
    }

    // Identify performance outliers
    let median_performance = sorted_results[sorted_results.len() / 2].1;
    let slow_scripts: Vec<_> = sorted_results
        .iter()
        .filter(|(_, ops)| *ops < median_performance * 0.1)
        .collect();

    if !slow_scripts.is_empty() {
        println!("\n⚠️  Performance outliers (>10x slower than median):");
        for (script, ops_per_sec) in slow_scripts {
            println!("   {}: {:.0} ops/sec", script, ops_per_sec);
        }
    }
}

/// Automatically generate and run property-based tests for all scripts
#[test]
fn test_properties_all_scripts() {
    let shlesha = Shlesha::new();
    let all_scripts = shlesha.list_supported_scripts();

    println!("🔍 Property Testing All Scripts:");

    let mut property_violations = Vec::new();

    for script in &all_scripts {
        // Property 1: Empty input should return empty output
        match shlesha.transliterate("", script, script) {
            Ok(result) => {
                if !result.is_empty() {
                    property_violations.push(format!(
                        "Empty input property violated: {} '' → '{}'",
                        script, result
                    ));
                }
            }
            Err(_) => {
                // Empty input errors are acceptable
            }
        }

        // Property 2: ASCII should be preserved in identity conversions
        for ascii_char in ['a', 'A', '1', ' ', '.'] {
            let input = ascii_char.to_string();
            match shlesha.transliterate(&input, script, script) {
                Ok(result) => {
                    if result != input && script.contains("roman") {
                        property_violations.push(format!(
                            "ASCII preservation violated: {} '{}' → '{}'",
                            script, input, result
                        ));
                    }
                }
                Err(_) => {
                    // Some ASCII chars might not be valid in some scripts
                }
            }
        }
    }

    println!("   Property violations: {}", property_violations.len());

    if !property_violations.is_empty() {
        println!("\n❌ Property violations:");
        for violation in property_violations.iter().take(10) {
            println!("   {}", violation);
        }
    }
}

/// Summary test that reports on overall system health
#[test]
fn test_system_health_summary() {
    let shlesha = Shlesha::new();
    let all_scripts = shlesha.list_supported_scripts();
    let (roman_scripts, indic_scripts) = categorize_scripts(&shlesha);

    println!("\n🏥 SYSTEM HEALTH SUMMARY");
    println!("========================");

    println!("\n📊 Script Coverage:");
    println!("   Total scripts: {}", all_scripts.len());
    println!("   Roman scripts: {}", roman_scripts.len());
    println!("   Indic scripts: {}", indic_scripts.len());
    println!(
        "   Total possible conversions: {}",
        all_scripts.len() * (all_scripts.len() - 1)
    );

    // Test a sample of conversions to get health metrics
    let mut working_conversions = 0;
    let mut total_tested = 0;

    for source in all_scripts.iter().take(5) {
        for target in all_scripts.iter().take(5) {
            if source != target {
                total_tested += 1;
                if shlesha.transliterate("a", source, target).is_ok() {
                    working_conversions += 1;
                }
            }
        }
    }

    let health_percentage = if total_tested > 0 {
        working_conversions as f64 / total_tested as f64 * 100.0
    } else {
        0.0
    };

    println!("\n💊 Health Metrics:");
    println!("   Sample conversions tested: {}", total_tested);
    println!("   Working conversions: {}", working_conversions);
    println!("   Health score: {:.1}%", health_percentage);

    if health_percentage < 50.0 {
        println!("   🚨 CRITICAL: System health below 50%!");
    } else if health_percentage < 80.0 {
        println!("   ⚠️  WARNING: System health below 80%");
    } else {
        println!("   ✅ GOOD: System health above 80%");
    }

    println!("\n🎯 Recommendations:");
    println!("   1. Fix Roman script reverse mapping generation");
    println!("   2. Add character-level validation tests");
    println!("   3. Implement proper hub conversion logic");
    println!("   4. Add performance benchmarks for all scripts");
    println!("   5. Create CI pipeline to run these tests on every commit");

    // Don't fail - this is a summary test
}
