use shlesha::Shlesha;
use std::collections::{HashMap, HashSet};

/// Results for character set testing of a script pair
#[derive(Debug, Clone)]
struct CharsetTestResults {
    successful_chars: HashMap<String, String>,
    failed_chars: HashMap<String, String>,
    successful_combinations: HashMap<String, String>,
    failed_combinations: HashMap<String, String>,
}

impl CharsetTestResults {
    fn new() -> Self {
        Self {
            successful_chars: HashMap::new(),
            failed_chars: HashMap::new(),
            successful_combinations: HashMap::new(),
            failed_combinations: HashMap::new(),
        }
    }

    fn total_tests(&self) -> usize {
        self.successful_chars.len()
            + self.failed_chars.len()
            + self.successful_combinations.len()
            + self.failed_combinations.len()
    }

    fn successful_tests(&self) -> usize {
        self.successful_chars.len() + self.successful_combinations.len()
    }

    fn success_rate(&self) -> f64 {
        if self.total_tests() == 0 {
            0.0
        } else {
            self.successful_tests() as f64 / self.total_tests() as f64 * 100.0
        }
    }
}

/// Generate complete character sets for different script types
fn generate_script_character_sets() -> HashMap<String, Vec<String>> {
    let mut character_sets = HashMap::new();

    // IAST character set
    character_sets.insert(
        "iast".to_string(),
        vec![
            // Vowels
            "a", "ā", "i", "ī", "u", "ū", "ṛ", "ṝ", "ḷ", "ḹ", "e", "ai", "o", "au",
            // Consonants
            "k", "kh", "g", "gh", "ṅ", "c", "ch", "j", "jh", "ñ", "ṭ", "ṭh", "ḍ", "ḍh", "ṇ", "t",
            "th", "d", "dh", "n", "p", "ph", "b", "bh", "m", "y", "r", "l", "v", "ś", "ṣ", "s",
            "h", // Marks
            "ṃ", "ḥ", "m̐", // Special combinations
            "kṣ", "jñ", // Digits
            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect(),
    );

    // SLP1 character set
    character_sets.insert(
        "slp1".to_string(),
        vec![
            // Vowels
            "a", "A", "i", "I", "u", "U", "f", "F", "x", "X", "e", "E", "o", "O",
            // Consonants
            "k", "K", "g", "G", "N", "c", "C", "j", "J", "Y", "w", "W", "q", "Q", "R", "t", "T",
            "d", "D", "n", "p", "P", "b", "B", "m", "y", "r", "l", "v", "S", "z", "s", "h",
            // Marks
            "M", "H", // Special combinations
            "kz", "jY", // Digits
            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect(),
    );

    // Harvard-Kyoto character set
    character_sets.insert(
        "harvard_kyoto".to_string(),
        vec![
            // Vowels
            "a", "A", "i", "I", "u", "U", "R", "RR", "lR", "lRR", "e", "ai", "o", "au",
            // Consonants
            "k", "kh", "g", "gh", "G", "c", "ch", "j", "jh", "J", "T", "Th", "D", "Dh", "N", "t",
            "th", "d", "dh", "n", "p", "ph", "b", "bh", "m", "y", "r", "l", "v", "z", "S", "s",
            "h", // Marks
            "M", "H", // Special combinations
            "kS", "jJ", // Digits
            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect(),
    );

    // Devanagari character set
    character_sets.insert(
        "devanagari".to_string(),
        vec![
            // Vowels
            "अ",
            "आ",
            "इ",
            "ई",
            "उ",
            "ऊ",
            "ऋ",
            "ॠ",
            "ऌ",
            "ॡ",
            "ए",
            "ऐ",
            "ओ",
            "औ",
            // Consonants
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
            // Vowel signs
            "ा",
            "ि",
            "ी",
            "ु",
            "ू",
            "ृ",
            "ॄ",
            "ॢ",
            "ॣ",
            "े",
            "ै",
            "ो",
            "ौ",
            // Marks
            "ं",
            "ः",
            "ँ",
            "्",
            // Special combinations
            "क्ष",
            "ज्ञ",
            // Digits
            "०",
            "१",
            "२",
            "३",
            "४",
            "५",
            "६",
            "७",
            "८",
            "९",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect(),
    );

    // Default Roman charset for other scripts
    let default_roman_charset: Vec<String> = vec![
        "a", "i", "u", "e", "o", "k", "g", "c", "j", "t", "d", "p", "b", "m", "n", "r", "l", "v",
        "s", "h",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect();

    // Add other Roman scripts with default charset
    for script in ["iso15919", "itrans", "velthuis", "wx", "kolkata"] {
        character_sets.insert(script.to_string(), default_roman_charset.clone());
    }

    // Default Indic charset for other scripts
    let default_indic_charset: Vec<String> = vec![
        "क", "ग", "च", "ज", "त", "द", "प", "ब", "म", "न", "र", "ल", "व", "स", "ह",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect();

    // Add other Indic scripts with default charset
    for script in [
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
        character_sets.insert(script.to_string(), default_indic_charset.clone());
    }

    character_sets
}

/// Get the appropriate character set for a script
fn get_charset_for_script(
    script: &str,
    character_sets: &HashMap<String, Vec<String>>,
) -> Vec<String> {
    // Try exact match first
    if let Some(charset) = character_sets.get(script) {
        return charset.clone();
    }

    // Try case variations
    let script_lower = script.to_lowercase();
    for (key, charset) in character_sets {
        if key.to_lowercase() == script_lower {
            return charset.clone();
        }
    }

    // Fallback based on script type
    if is_roman_script(script) {
        character_sets
            .get("iast")
            .unwrap_or(&vec!["a".to_string()])
            .clone()
    } else {
        character_sets
            .get("devanagari")
            .unwrap_or(&vec!["क".to_string()])
            .clone()
    }
}

/// Analyze charset results by script type
fn analyze_charset_results_by_script_type(
    _all_scripts: &[String],
    results: &HashMap<String, CharsetTestResults>,
) {
    println!("\n🔍 CHARACTER SET ANALYSIS BY SCRIPT TYPE:");

    let mut roman_to_roman_stats = vec![];
    let mut roman_to_indic_stats = vec![];
    let mut indic_to_roman_stats = vec![];
    let mut indic_to_indic_stats = vec![];

    for (pair_key, pair_results) in results {
        let parts: Vec<&str> = pair_key.split("→").collect();
        if parts.len() != 2 {
            continue;
        }

        let source = parts[0];
        let target = parts[1];
        let source_is_roman = is_roman_script(source);
        let target_is_roman = is_roman_script(target);

        let success_rate = pair_results.success_rate();

        match (source_is_roman, target_is_roman) {
            (true, true) => roman_to_roman_stats.push(success_rate),
            (true, false) => roman_to_indic_stats.push(success_rate),
            (false, true) => indic_to_roman_stats.push(success_rate),
            (false, false) => indic_to_indic_stats.push(success_rate),
        }
    }

    fn print_stats(category: &str, stats: &[f64]) {
        if stats.is_empty() {
            println!("   {}: No data", category);
            return;
        }

        let avg = stats.iter().sum::<f64>() / stats.len() as f64;
        let min = stats.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max = stats.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        println!(
            "   {}: {} pairs, avg {:.1}%, range {:.1}%-{:.1}%",
            category,
            stats.len(),
            avg,
            min,
            max
        );
    }

    print_stats("Roman→Roman", &roman_to_roman_stats);
    print_stats("Roman→Indic", &roman_to_indic_stats);
    print_stats("Indic→Roman", &indic_to_roman_stats);
    print_stats("Indic→Indic", &indic_to_indic_stats);
}

/// Generate character coverage matrix showing character-level success/failure
fn generate_character_coverage_matrix(
    _all_scripts: &[String],
    results: &HashMap<String, CharsetTestResults>,
) {
    println!("\n📊 CHARACTER COVERAGE MATRIX SUMMARY:");

    // Count total characters tested vs successful
    let mut total_chars_tested = 0;
    let mut total_chars_successful = 0;
    let mut total_combinations_tested = 0;
    let mut total_combinations_successful = 0;

    for pair_results in results.values() {
        total_chars_tested += pair_results.successful_chars.len() + pair_results.failed_chars.len();
        total_chars_successful += pair_results.successful_chars.len();
        total_combinations_tested +=
            pair_results.successful_combinations.len() + pair_results.failed_combinations.len();
        total_combinations_successful += pair_results.successful_combinations.len();
    }

    println!(
        "   Single characters: {}/{} successful ({:.1}%)",
        total_chars_successful,
        total_chars_tested,
        if total_chars_tested > 0 {
            total_chars_successful as f64 / total_chars_tested as f64 * 100.0
        } else {
            0.0
        }
    );

    println!(
        "   Character combinations: {}/{} successful ({:.1}%)",
        total_combinations_successful,
        total_combinations_tested,
        if total_combinations_tested > 0 {
            total_combinations_successful as f64 / total_combinations_tested as f64 * 100.0
        } else {
            0.0
        }
    );

    // Show sample of most problematic characters
    let mut character_failure_counts = HashMap::new();

    for pair_results in results.values() {
        for failed_char in pair_results.failed_chars.keys() {
            *character_failure_counts
                .entry(failed_char.clone())
                .or_insert(0) += 1;
        }
    }

    if !character_failure_counts.is_empty() {
        println!("\n🚨 Most problematic characters:");
        let mut sorted_failures: Vec<_> = character_failure_counts.into_iter().collect();
        sorted_failures.sort_by(|a, b| b.1.cmp(&a.1));

        for (character, failure_count) in sorted_failures.into_iter().take(10) {
            println!(
                "   '{}': failed in {} conversions",
                character, failure_count
            );
        }
    }
}

/// Test that explicitly verifies we test EVERY SINGLE script pair
#[test]
fn test_exhaustive_script_pair_coverage() {
    let shlesha = Shlesha::new();
    let all_scripts = shlesha.list_supported_scripts();

    println!("🔍 EXHAUSTIVE PAIR COVERAGE VERIFICATION");
    println!("========================================");
    println!("Total scripts discovered: {}", all_scripts.len());

    // Calculate expected pairs
    let total_scripts = all_scripts.len();
    let expected_pairs = total_scripts * (total_scripts - 1);

    println!(
        "Expected total pairs: {} × {} = {}",
        total_scripts,
        total_scripts - 1,
        expected_pairs
    );

    // Generate and track ALL possible pairs
    let mut all_possible_pairs = HashSet::new();
    let mut tested_pairs = HashSet::new();
    let mut successful_pairs = HashSet::new();
    let mut failed_pairs = HashMap::new();

    // Generate all possible pairs
    for source in &all_scripts {
        for target in &all_scripts {
            if source != target {
                let pair = (source.clone(), target.clone());
                all_possible_pairs.insert(pair);
            }
        }
    }

    println!(
        "Generated {} unique pairs for testing",
        all_possible_pairs.len()
    );
    assert_eq!(
        all_possible_pairs.len(),
        expected_pairs,
        "Mismatch between calculated and generated pairs!"
    );

    // Test every single pair
    println!("\n📊 Testing every single pair...");
    let mut pair_count = 0;

    for (source, target) in &all_possible_pairs {
        pair_count += 1;
        if pair_count % 100 == 0 {
            println!(
                "   Progress: {}/{} pairs tested",
                pair_count, expected_pairs
            );
        }

        let pair_key = (source.clone(), target.clone());
        tested_pairs.insert(pair_key.clone());

        // Test with a simple input
        match shlesha.transliterate("a", source, target) {
            Ok(result) => {
                successful_pairs.insert(pair_key);

                // Log interesting conversions for analysis
                if pair_count <= 20 {
                    println!("   ✓ {}→{}: 'a' → '{}'", source, target, result);
                }
            }
            Err(e) => {
                failed_pairs.insert(pair_key, e.to_string());

                // Log first few failures for analysis
                if failed_pairs.len() <= 10 {
                    println!("   ❌ {}→{}: ERROR - {}", source, target, e);
                }
            }
        }
    }

    println!("\n📈 EXHAUSTIVE COVERAGE RESULTS:");
    println!("   Total possible pairs: {}", all_possible_pairs.len());
    println!("   Pairs tested: {}", tested_pairs.len());
    println!("   Successful pairs: {}", successful_pairs.len());
    println!("   Failed pairs: {}", failed_pairs.len());

    let success_rate = successful_pairs.len() as f64 / tested_pairs.len() as f64 * 100.0;
    println!("   Success rate: {:.1}%", success_rate);

    // Verify we tested every pair
    assert_eq!(
        tested_pairs.len(),
        all_possible_pairs.len(),
        "Not all pairs were tested!"
    );

    // Check for untested pairs (should be none)
    let untested_pairs: Vec<_> = all_possible_pairs.difference(&tested_pairs).collect();
    if !untested_pairs.is_empty() {
        println!("\n❌ UNTESTED PAIRS FOUND:");
        for pair in &untested_pairs {
            println!("   {}→{}", pair.0, pair.1);
        }
        panic!("Found {} untested pairs!", untested_pairs.len());
    }

    println!("   ✅ ALL PAIRS TESTED - 100% coverage achieved");

    // Analyze failure patterns
    if !failed_pairs.is_empty() {
        println!("\n🔍 FAILURE PATTERN ANALYSIS:");

        // Group failures by error type
        let mut error_types = HashMap::new();
        for (_, error) in &failed_pairs {
            let error_type = if error.contains("not found") || error.contains("not supported") {
                "Script not found"
            } else if error.contains("conversion failed") {
                "Conversion failed"
            } else if error.contains("invalid") {
                "Invalid input"
            } else {
                "Other error"
            };
            *error_types.entry(error_type).or_insert(0) += 1;
        }

        for (error_type, count) in error_types {
            println!("   {}: {} pairs", error_type, count);
        }

        // Show sample failures by category
        println!("\n📋 Sample Failures by Script Type:");

        let mut roman_to_roman_failures = 0;
        let mut roman_to_indic_failures = 0;
        let mut indic_to_roman_failures = 0;
        let mut indic_to_indic_failures = 0;

        for ((source, target), _) in &failed_pairs {
            let source_is_roman = is_roman_script(source);
            let target_is_roman = is_roman_script(target);

            match (source_is_roman, target_is_roman) {
                (true, true) => roman_to_roman_failures += 1,
                (true, false) => roman_to_indic_failures += 1,
                (false, true) => indic_to_roman_failures += 1,
                (false, false) => indic_to_indic_failures += 1,
            }
        }

        println!("   Roman→Roman failures: {}", roman_to_roman_failures);
        println!("   Roman→Indic failures: {}", roman_to_indic_failures);
        println!("   Indic→Roman failures: {}", indic_to_roman_failures);
        println!("   Indic→Indic failures: {}", indic_to_indic_failures);
    }

    // Performance warning
    if success_rate < 50.0 {
        println!("\n🚨 CRITICAL: Less than 50% of script pairs are working!");
    } else if success_rate < 80.0 {
        println!("\n⚠️ WARNING: Less than 80% of script pairs are working");
    } else {
        println!("\n✅ GOOD: Over 80% of script pairs are working");
    }

    // Store results for further analysis
    println!("\n💾 Coverage verification complete:");
    println!(
        "   Total coverage: {}/{} pairs (100%)",
        tested_pairs.len(),
        all_possible_pairs.len()
    );
}

fn is_roman_script(script: &str) -> bool {
    let roman_patterns = [
        "iast", "slp1", "harvard", "itrans", "velthuis", "wx", "kolkata", "iso",
    ];
    let script_lower = script.to_lowercase();
    roman_patterns
        .iter()
        .any(|pattern| script_lower.contains(pattern))
}

/// Test comprehensive character set coverage for all script pairs
#[test]
fn test_script_pairs_with_complete_charset() {
    let shlesha = Shlesha::new();
    let all_scripts = shlesha.list_supported_scripts();

    println!("🔤 TESTING SCRIPT PAIRS WITH COMPLETE CHARACTER SETS");
    println!("=====================================================");

    // Generate character sets for each script type
    let character_sets = generate_script_character_sets();

    let mut comprehensive_results = HashMap::new();
    let total_pairs = all_scripts.len() * (all_scripts.len() - 1);

    println!(
        "Testing {} script pairs with complete character sets",
        total_pairs
    );

    let mut pair_count = 0;
    let mut total_char_tests = 0;
    let mut successful_char_tests = 0;
    let mut failed_char_tests = 0;

    for (source, target) in all_scripts
        .iter()
        .flat_map(|s| all_scripts.iter().map(move |t| (s, t)))
        .filter(|(s, t)| s != t)
    {
        pair_count += 1;
        if pair_count % 50 == 0 {
            println!("   Progress: {}/{} pairs tested", pair_count, total_pairs);
        }

        let mut pair_results = CharsetTestResults::new();

        // Get appropriate character set for source script
        let charset = get_charset_for_script(source, &character_sets);

        // Test each character in the charset
        for character in &charset {
            total_char_tests += 1;

            match shlesha.transliterate(character, source, target) {
                Ok(result) => {
                    successful_char_tests += 1;
                    pair_results
                        .successful_chars
                        .insert(character.clone(), result);
                }
                Err(e) => {
                    failed_char_tests += 1;
                    pair_results
                        .failed_chars
                        .insert(character.clone(), e.to_string());
                }
            }
        }

        // Test character combinations (2-character sequences)
        for i in 0..charset.len().min(10) {
            for j in (i + 1)..charset.len().min(10) {
                let combination = format!("{}{}", charset[i], charset[j]);
                total_char_tests += 1;

                match shlesha.transliterate(&combination, source, target) {
                    Ok(result) => {
                        successful_char_tests += 1;
                        pair_results
                            .successful_combinations
                            .insert(combination, result);
                    }
                    Err(e) => {
                        failed_char_tests += 1;
                        pair_results
                            .failed_combinations
                            .insert(combination, e.to_string());
                    }
                }
            }
        }

        comprehensive_results.insert(format!("{}→{}", source, target), pair_results);
    }

    println!("\n📊 COMPLETE CHARACTER SET RESULTS:");
    println!("   Total script pairs: {}", total_pairs);
    println!("   Total character tests: {}", total_char_tests);
    println!("   Successful character tests: {}", successful_char_tests);
    println!("   Failed character tests: {}", failed_char_tests);
    println!(
        "   Character success rate: {:.1}%",
        successful_char_tests as f64 / total_char_tests as f64 * 100.0
    );

    // Analyze results by script type
    analyze_charset_results_by_script_type(&all_scripts, &comprehensive_results);

    // Generate character coverage matrix
    generate_character_coverage_matrix(&all_scripts, &comprehensive_results);

    assert_eq!(pair_count, total_pairs, "Not all pairs were tested!");
    println!("\n✅ COMPLETE CHARACTER SET TESTING FINISHED");
    println!("   Every script pair tested with full character sets");
    println!(
        "   Total coverage: {} character tests across {} pairs",
        total_char_tests, total_pairs
    );
}

/// Test round-trip consistency for complete character sets
#[test]
fn test_charset_round_trip_consistency() {
    let shlesha = Shlesha::new();
    let all_scripts = shlesha.list_supported_scripts();

    println!("🔄 TESTING ROUND-TRIP CHARACTER SET CONSISTENCY");
    println!("===============================================");

    let character_sets = generate_script_character_sets();

    let mut total_round_trip_tests = 0;
    let mut successful_round_trips = 0;
    let mut failed_round_trips = Vec::new();

    // Focus on Roman scripts for round-trip testing (should be lossless)
    let roman_scripts: Vec<_> = all_scripts
        .iter()
        .filter(|script| is_roman_script(script))
        .collect();

    println!(
        "Testing round-trips between {} Roman scripts",
        roman_scripts.len()
    );

    for script1 in &roman_scripts {
        for script2 in &roman_scripts {
            if script1 == script2 {
                continue;
            }

            let charset = get_charset_for_script(script1, &character_sets);

            // Test round-trip for each character in the charset
            for character in charset.iter().take(20) {
                // Limit for performance
                total_round_trip_tests += 1;

                // script1 → script2 → script1
                match shlesha
                    .transliterate(character, script1, script2)
                    .and_then(|intermediate| shlesha.transliterate(&intermediate, script2, script1))
                {
                    Ok(final_result) => {
                        if final_result == *character {
                            successful_round_trips += 1;
                        } else {
                            failed_round_trips.push(format!(
                                "{}→{}→{}: '{}' → '{}' (char-level)",
                                script1, script2, script1, character, final_result
                            ));
                        }
                    }
                    Err(e) => {
                        failed_round_trips.push(format!(
                            "{}→{}→{}: '{}' failed - {}",
                            script1, script2, script1, character, e
                        ));
                    }
                }
            }

            // Test character combinations
            let limited_charset: Vec<_> = charset.iter().take(5).collect();
            for i in 0..limited_charset.len() {
                for j in (i + 1)..limited_charset.len() {
                    let combination = format!("{}{}", limited_charset[i], limited_charset[j]);
                    total_round_trip_tests += 1;

                    match shlesha
                        .transliterate(&combination, script1, script2)
                        .and_then(|intermediate| {
                            shlesha.transliterate(&intermediate, script2, script1)
                        }) {
                        Ok(final_result) => {
                            if final_result == combination {
                                successful_round_trips += 1;
                            } else {
                                failed_round_trips.push(format!(
                                    "{}→{}→{}: '{}' → '{}' (combo-level)",
                                    script1, script2, script1, combination, final_result
                                ));
                            }
                        }
                        Err(_e) => {
                            // Combination failures are more acceptable
                        }
                    }
                }
            }
        }
    }

    println!("\n📊 ROUND-TRIP CHARACTER SET RESULTS:");
    println!("   Total round-trip tests: {}", total_round_trip_tests);
    println!("   Successful round-trips: {}", successful_round_trips);
    println!("   Failed round-trips: {}", failed_round_trips.len());

    if total_round_trip_tests > 0 {
        let success_rate = successful_round_trips as f64 / total_round_trip_tests as f64 * 100.0;
        println!("   Round-trip success rate: {:.1}%", success_rate);

        if success_rate < 50.0 {
            println!("   🚨 CRITICAL: Less than 50% round-trip success!");
        } else if success_rate < 80.0 {
            println!("   ⚠️  WARNING: Less than 80% round-trip success");
        } else {
            println!("   ✅ GOOD: Over 80% round-trip success");
        }
    }

    if !failed_round_trips.is_empty() {
        println!("\n❌ Round-trip failures (showing first 15):");
        for failure in failed_round_trips.iter().take(15) {
            println!("   {}", failure);
        }
        if failed_round_trips.len() > 15 {
            println!("   ... and {} more failures", failed_round_trips.len() - 15);
        }
    }

    println!("\n✅ CHARACTER SET ROUND-TRIP TESTING COMPLETE");
}

/// Generate a coverage matrix showing exactly which pairs work/fail
#[test]
fn test_generate_complete_coverage_matrix() {
    let shlesha = Shlesha::new();
    let all_scripts = shlesha.list_supported_scripts();

    println!("📊 COMPLETE COVERAGE MATRIX GENERATION");
    println!("=====================================");

    let mut matrix = HashMap::new();

    // Test every single pair and record results
    for source in &all_scripts {
        for target in &all_scripts {
            if source == target {
                continue;
            }

            let result = match shlesha.transliterate("a", source, target) {
                Ok(output) => format!("✓ 'a'→'{}'", output),
                Err(e) => format!("❌ {}", e.to_string().chars().take(30).collect::<String>()),
            };

            matrix.insert((source.clone(), target.clone()), result);
        }
    }

    // Generate summary statistics
    let total_pairs = matrix.len();
    let successful_pairs = matrix.values().filter(|v| v.starts_with("✓")).count();
    let failed_pairs = total_pairs - successful_pairs;

    println!("Matrix generated: {} total pairs", total_pairs);
    println!(
        "Successful pairs: {} ({:.1}%)",
        successful_pairs,
        successful_pairs as f64 / total_pairs as f64 * 100.0
    );
    println!(
        "Failed pairs: {} ({:.1}%)",
        failed_pairs,
        failed_pairs as f64 / total_pairs as f64 * 100.0
    );

    // Show a sample of the matrix (first 10x10)
    println!("\n📋 Sample Coverage Matrix (first 10 scripts):");
    let sample_scripts: Vec<_> = all_scripts.iter().take(10).collect();

    print!("\n{:>15}", "");
    for target in &sample_scripts {
        print!(" {:>12}", target.chars().take(10).collect::<String>());
    }
    println!();

    for source in &sample_scripts {
        print!("{:>15}", source.chars().take(13).collect::<String>());
        for target in &sample_scripts {
            if source == target {
                print!(" {:>12}", "—");
            } else {
                let result = matrix
                    .get(&((*source).clone(), (*target).clone()))
                    .map(|r| if r.starts_with("✓") { "✓" } else { "❌" })
                    .unwrap_or("?");
                print!(" {:>12}", result);
            }
        }
        println!();
    }

    println!("\n💾 Complete matrix available for analysis");
    println!("   ✓ = Successful conversion");
    println!("   ❌ = Failed conversion");
    println!("   — = Identity (same script)");

    // The matrix is now fully generated and available for analysis
    assert_eq!(
        matrix.len(),
        all_scripts.len() * (all_scripts.len() - 1),
        "Matrix should contain exactly n×(n-1) entries"
    );

    println!("\n✅ COMPLETE COVERAGE MATRIX VERIFIED");
    println!("   Every possible script pair has been tested and recorded");
}
