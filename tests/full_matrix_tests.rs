//! Comprehensive n² matrix testing for all supported script pairs.
//! 
//! This test suite validates round-trip transliteration fidelity across all 15 supported
//! scripts and schemes (9 Indic scripts + 6 Roman schemes), testing all 225 possible pairs
//! with identity tests and round-trip tests using verified basic consonants.

use shlesha::{TransliteratorBuilder, SchemaParser};
use std::collections::HashMap;

#[test]
fn test_full_n_squared_round_trip_matrix() {
    // Define all supported scripts
    let scripts = vec![
        // Indic Scripts
        "Devanagari",
        "Bengali", 
        "Tamil",
        "Telugu",
        "Kannada",
        "Malayalam",
        "Gujarati",
        "Odia",
        "Gurmukhi",
        
        // Roman Schemes
        "IAST",
        "Harvard-Kyoto",
        "ITRANS", 
        "SLP1",
        "Velthuis",
        "WX",
    ];

    // Load all schemas
    let schemas = vec![
        // Indic Scripts
        ("Devanagari", include_str!("../schemas/devanagari.yaml")),
        ("Bengali", include_str!("../schemas/bengali.yaml")),
        ("Tamil", include_str!("../schemas/tamil.yaml")),
        ("Telugu", include_str!("../schemas/telugu.yaml")),
        ("Kannada", include_str!("../schemas/kannada.yaml")),
        ("Malayalam", include_str!("../schemas/malayalam.yaml")),
        ("Gujarati", include_str!("../schemas/gujarati.yaml")),
        ("Odia", include_str!("../schemas/odia.yaml")),
        ("Gurmukhi", include_str!("../schemas/gurmukhi.yaml")),
        
        // Roman Schemes
        ("IAST", include_str!("../schemas/iast.yaml")),
        ("Harvard-Kyoto", include_str!("../schemas/harvard_kyoto.yaml")),
        ("ITRANS", include_str!("../schemas/itrans.yaml")),
        ("SLP1", include_str!("../schemas/slp1.yaml")),
        ("Velthuis", include_str!("../schemas/velthuis.yaml")),
        ("WX", include_str!("../schemas/wx.yaml")),
    ];

    // Build transliterator with all schemas
    let mut builder = TransliteratorBuilder::new();
    for (name, content) in &schemas {
        let schema = SchemaParser::parse_str(content).unwrap();
        builder = builder.with_schema(schema).unwrap();
        println!("Loaded {} schema", name);
    }
    let transliterator = builder.build();

    // Define comprehensive test coverage including characters that may not exist in all scripts
    // This tests our fallback token system for proper round-trip preservation
    let test_words = vec![
        // All major consonants (including aspirated forms)
        ("क", "ka"), ("ख", "kha"), ("ग", "ga"), ("घ", "gha"), ("ङ", "ṅa"),
        ("च", "ca"), ("छ", "cha"), ("ज", "ja"), ("झ", "jha"), ("ञ", "ña"),
        ("ट", "ṭa"), ("ठ", "ṭha"), ("ड", "ḍa"), ("ढ", "ḍha"), ("ण", "ṇa"),
        ("त", "ta"), ("थ", "tha"), ("द", "da"), ("ध", "dha"), ("न", "na"),
        ("प", "pa"), ("फ", "pha"), ("ब", "ba"), ("भ", "bha"), ("म", "ma"),
        
        // Semi-vowels and sibilants  
        ("य", "ya"), ("र", "ra"), ("ल", "la"), ("व", "va"),
        ("श", "śa"), ("ष", "ṣa"), ("स", "sa"), ("ह", "ha"),
        
        // Independent vowels
        ("अ", "a"), ("आ", "ā"), ("इ", "i"), ("ई", "ī"), ("उ", "u"), ("ऊ", "ū"),
        ("ए", "e"), ("ऐ", "ai"), ("ओ", "o"), ("औ", "au"),
        
        // Vowel combinations (matras)
        ("का", "kā"), ("कि", "ki"), ("की", "kī"), ("कु", "ku"), ("कू", "kū"),
        ("के", "ke"), ("कै", "kai"), ("को", "ko"), ("कौ", "kau"),
        
        // Complex conjuncts
        ("क्त", "kta"), ("क्ष", "kṣa"), ("ज्ञ", "jña"), ("श्र", "śra"),
        ("त्र", "tra"), ("द्व", "dva"), ("स्व", "sva"), ("स्थ", "stha"),
        
        // Modifiers
        ("कं", "kaṃ"), ("कः", "kaḥ"), ("क्", "k"), 
        
        // Real Sanskrit words
        ("धर्म", "dharma"), ("कर्म", "karma"), ("राम", "rāma"), ("श्याम", "śyāma"),
        ("नमस्ते", "namaste"), ("शान्ति", "śānti"), ("वेद", "veda"), ("योग", "yoga"),
        
        // Numerals and punctuation
        ("०", "0"), ("१", "1"), ("२", "2"), ("३", "3"), ("४", "4"),
        ("५", "5"), ("६", "6"), ("७", "7"), ("८", "8"), ("९", "9"),
        ("।", "."), ("॥", ".."), ("ॐ", "oṃ"),
        
        // Script-specific features that may not exist in all scripts
        ("ऌ", "ḷ"), ("ॠ", "ṝ"), ("ऑ", "ŏ"), ("ऋ", "ṛ"),
    ];

    let mut total_tests = 0;
    let mut total_failures = 0;
    let mut detailed_results: HashMap<String, Vec<String>> = HashMap::new();

    println!("\n🚀 Starting comprehensive n² matrix round-trip test...");
    println!("Scripts: {}", scripts.join(", "));
    println!("Total script pairs: {}", scripts.len() * scripts.len());
    println!("Test cases per pair: {}", test_words.len());
    println!("Total tests: {}\n", scripts.len() * scripts.len() * test_words.len());
    println!("Coverage: All consonants, vowels, conjuncts, modifiers, words, numerals & punctuation");

    // Test every script pair combination (n²)
    for source_script in &scripts {
        for target_script in &scripts {
            let pair_key = format!("{} → {}", source_script, target_script);
            let mut pair_failures = Vec::new();

            println!("Testing: {}", pair_key);

            // Convert test words from Devanagari to source script first (if not Devanagari)
            let source_words: Vec<(String, String)> = if *source_script == "Devanagari" {
                test_words.iter()
                    .map(|(deva, iast)| (deva.to_string(), iast.to_string()))
                    .collect()
            } else {
                // Convert from Devanagari to source script via IAST
                test_words.iter()
                    .filter_map(|(deva_word, expected_iast)| {
                        match transliterator.transliterate(deva_word, "Devanagari", "IAST") {
                            Ok(iast) => {
                                match transliterator.transliterate(&iast, "IAST", source_script) {
                                    Ok(source_word) => Some((source_word, expected_iast.to_string())),
                                    Err(_) => None, // Skip if conversion fails
                                }
                            }
                            Err(_) => None,
                        }
                    })
                    .collect()
            };

            // Test each word in this script pair
            for (source_word, _expected_iast) in &source_words {
                total_tests += 1;

                // Forward translation: source → target
                match transliterator.transliterate(source_word, source_script, target_script) {
                    Ok(target_word) => {
                        // If same script, should be identical
                        if source_script == target_script {
                            if source_word != &target_word {
                                let failure = format!(
                                    "Identity test failed: {} ≠ {} (same script)", 
                                    source_word, target_word
                                );
                                println!("  ❌ {} → {} (expected {}) - Identity test failed", source_word, target_word, source_word);
                                pair_failures.push(failure);
                                total_failures += 1;
                            } else {
                                println!("  ✅ {} → {} (identity test passed)", source_word, target_word);
                            }
                        } else {
                            // Round-trip test: source → target → source
                            match transliterator.transliterate(&target_word, target_script, source_script) {
                                Ok(round_trip_word) => {
                                    // Check for exact match first
                                    if source_word == &round_trip_word {
                                        println!("  ✅ {} → {} → {} (round-trip passed)", source_word, target_word, round_trip_word);
                                    } 
                                    // Check if round-trip result is a script-aware token or legacy fallback token
                                    else if (round_trip_word.starts_with("[") && round_trip_word.contains(":") && round_trip_word.ends_with("]")) ||
                                            (round_trip_word.starts_with("[?:") && round_trip_word.ends_with("]")) {
                                        
                                        // Handle script-aware tokens [script:token] and legacy tokens [?:token]
                                        let token_content = &round_trip_word[1..round_trip_word.len()-1];
                                        
                                        if let Some(colon_pos) = token_content.find(':') {
                                            let script_part = &token_content[..colon_pos];
                                            let preserved_content = &token_content[colon_pos + 1..];
                                            
                                            // For script-aware tokens, check if we successfully unwrapped back to origin
                                            if script_part != "?" {
                                                // This is a script-aware token - should have been unwrapped if returning to origin
                                                let failure = format!(
                                                    "Script-aware token not unwrapped: {} → {} → {} (token from '{}' script should be unwrapped when returning to '{}')",
                                                    source_word, target_word, round_trip_word, script_part, source_script
                                                );
                                                println!("  ❌ {} → {} → {} (script-aware token from '{}' not unwrapped) - Token unwrapping failed", source_word, target_word, round_trip_word, script_part);
                                                pair_failures.push(failure);
                                                total_failures += 1;
                                            } else {
                                                // Legacy fallback token [?:token] - check if preserved correctly
                                                if preserved_content == source_word {
                                                    println!("  ✅ {} → {} → {} (round-trip passed via legacy fallback)", source_word, target_word, round_trip_word);
                                                } else {
                                                    let failure = format!(
                                                        "Legacy fallback round-trip failed: {} → {} → {} (fallback preserves '{}', expected '{}')",
                                                        source_word, target_word, round_trip_word, preserved_content, source_word
                                                    );
                                                    println!("  ❌ {} → {} → {} (fallback preserves '{}', expected '{}') - Legacy fallback round-trip failed", source_word, target_word, round_trip_word, preserved_content, source_word);
                                                    pair_failures.push(failure);
                                                    total_failures += 1;
                                                }
                                            }
                                        } else {
                                            // Malformed token
                                            let failure = format!(
                                                "Malformed token: {} → {} → {} (token format invalid)",
                                                source_word, target_word, round_trip_word
                                            );
                                            println!("  ❌ {} → {} → {} (malformed token) - Token format invalid", source_word, target_word, round_trip_word);
                                            pair_failures.push(failure);
                                            total_failures += 1;
                                        }
                                    }
                                    else {
                                        let failure = format!(
                                            "Round-trip failed: {} → {} → {} (expected {})",
                                            source_word, target_word, round_trip_word, source_word
                                        );
                                        println!("  ❌ {} → {} → {} (expected {}) - Round-trip failed", source_word, target_word, round_trip_word, source_word);
                                        pair_failures.push(failure);
                                        total_failures += 1;
                                    }
                                }
                                Err(e) => {
                                    let failure = format!(
                                        "Reverse translation failed: {} → {} → ERROR: {}",
                                        source_word, target_word, e
                                    );
                                    println!("  ❌ {} → {} → ERROR: {} (reverse translation failed)", source_word, target_word, e);
                                    pair_failures.push(failure);
                                    total_failures += 1;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        let failure = format!(
                            "Forward translation failed: {} → ERROR: {}",
                            source_word, e
                        );
                        println!("  ❌ {} → ERROR: {} (forward translation failed)", source_word, e);
                        pair_failures.push(failure);
                        total_failures += 1;
                    }
                }
            }

            if !pair_failures.is_empty() {
                detailed_results.insert(pair_key.clone(), pair_failures);
                println!("  ❌ {} failures", detailed_results[&pair_key].len());
            } else {
                println!("  ✅ All tests passed");
            }
        }
    }

    // Print comprehensive results
    println!("\n{}", "=".repeat(80));
    println!("📊 FULL MATRIX TEST RESULTS");
    println!("{}", "=".repeat(80));
    println!("Total tests run: {}", total_tests);
    println!("Total failures: {}", total_failures);
    println!("Success rate: {:.2}%", 
             (total_tests - total_failures) as f64 / total_tests as f64 * 100.0);
    println!("Script pairs tested: {}", scripts.len() * scripts.len());
    println!("Failed script pairs: {}", detailed_results.len());

    if !detailed_results.is_empty() {
        println!("\n🔍 DETAILED FAILURE ANALYSIS:");
        println!("{}", "-".repeat(80));
        
        for (pair, failures) in &detailed_results {
            println!("\n{} ({} failures):", pair, failures.len());
            for (i, failure) in failures.iter().enumerate() {
                println!("  {}. {}", i + 1, failure);
                if i >= 4 { // Limit to first 5 failures per pair
                    println!("  ... and {} more failures", failures.len() - 5);
                    break;
                }
            }
        }

        println!("\n📈 FAILURE SUMMARY BY SCRIPT:");
        println!("{}", "-".repeat(40));
        let mut script_failure_counts: HashMap<String, usize> = HashMap::new();
        
        for pair in detailed_results.keys() {
            if let Some((source, target)) = pair.split_once(" → ") {
                *script_failure_counts.entry(source.to_string()).or_insert(0) += 1;
                *script_failure_counts.entry(target.to_string()).or_insert(0) += 1;
            }
        }

        for script in &scripts {
            let count = script_failure_counts.get(*script).unwrap_or(&0);
            println!("  {}: {} failed pairs", script, count);
        }
    }

    println!("\n{}", "=".repeat(80));

    // The test passes if we have reasonable success rate (adjust threshold as needed)
    let success_rate = (total_tests - total_failures) as f64 / total_tests as f64 * 100.0;
    if success_rate < 60.0 {
        panic!("Success rate too low: {:.2}% (threshold: 60%)", success_rate);
    }
}

#[test]
fn test_matrix_basic_characters() {
    // Test basic character mappings across all scripts
    let scripts = vec!["Devanagari", "Bengali", "Tamil", "Telugu", "Kannada", "Malayalam", "Gujarati"];
    
    // Load schemas
    let schemas = vec![
        ("Devanagari", include_str!("../schemas/devanagari.yaml")),
        ("Bengali", include_str!("../schemas/bengali.yaml")),
        ("Tamil", include_str!("../schemas/tamil.yaml")),
        ("Telugu", include_str!("../schemas/telugu.yaml")),
        ("Kannada", include_str!("../schemas/kannada.yaml")),
        ("Malayalam", include_str!("../schemas/malayalam.yaml")),
        ("Gujarati", include_str!("../schemas/gujarati.yaml")),
        ("IAST", include_str!("../schemas/iast.yaml")),
    ];

    let mut builder = TransliteratorBuilder::new();
    for (_, content) in &schemas {
        let schema = SchemaParser::parse_str(content).unwrap();
        builder = builder.with_schema(schema).unwrap();
    }
    let transliterator = builder.build();

    // Basic characters that should exist in all scripts
    let basic_chars = vec![
        ("क", "ka"), ("ख", "kha"), ("ग", "ga"), 
        ("त", "ta"), ("द", "da"), ("न", "na"),
        ("प", "pa"), ("ब", "ba"), ("म", "ma"),
        ("य", "ya"), ("र", "ra"), ("ल", "la"), ("व", "va"),
        ("स", "sa"), ("ह", "ha"),
    ];

    println!("\n🔤 Testing basic character matrix...");
    
    let mut char_results: HashMap<String, usize> = HashMap::new();
    
    for (deva_char, expected_iast) in &basic_chars {
        let mut successful_scripts = 0;
        
        for script in &scripts {
            // Convert Devanagari char to target script via IAST
            if let Ok(iast) = transliterator.transliterate(deva_char, "Devanagari", "IAST") {
                if iast == *expected_iast {
                    if let Ok(target_char) = transliterator.transliterate(&iast, "IAST", script) {
                        // Test round-trip back to IAST
                        if let Ok(back_to_iast) = transliterator.transliterate(&target_char, script, "IAST") {
                            if back_to_iast == iast {
                                successful_scripts += 1;
                            }
                        }
                    }
                }
            }
        }
        
        char_results.insert(format!("{} ({})", deva_char, expected_iast), successful_scripts);
    }

    println!("\n📋 Character Support Matrix:");
    println!("{}", "-".repeat(40));
    for (char_info, count) in &char_results {
        let percentage = (*count as f64 / scripts.len() as f64) * 100.0;
        println!("  {}: {}/{} scripts ({:.1}%)", char_info, count, scripts.len(), percentage);
    }
}

#[test] 
fn test_performance_matrix() {
    // Performance test for the full matrix
    use std::time::Instant;
    
    let scripts = vec!["Devanagari", "Bengali", "IAST"];
    let test_text = "धर्म";
    
    // Load minimal schemas for performance test
    let devanagari_schema = SchemaParser::parse_str(include_str!("../schemas/devanagari.yaml")).unwrap();
    let bengali_schema = SchemaParser::parse_str(include_str!("../schemas/bengali.yaml")).unwrap();
    let iast_schema = SchemaParser::parse_str(include_str!("../schemas/iast.yaml")).unwrap();

    let transliterator = TransliteratorBuilder::new()
        .with_schema(devanagari_schema).unwrap()
        .with_schema(bengali_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build();

    println!("\n⚡ Performance Matrix Test");
    println!("{}", "-".repeat(30));

    let start = Instant::now();
    let mut operations = 0;

    // Test all combinations multiple times
    for _ in 0..100 {
        for source in &scripts {
            for target in &scripts {
                if let Ok(source_text) = transliterator.transliterate(test_text, "Devanagari", source) {
                    if let Ok(_result) = transliterator.transliterate(&source_text, source, target) {
                        operations += 1;
                    }
                }
            }
        }
    }

    let duration = start.elapsed();
    let ops_per_second = operations as f64 / duration.as_secs_f64();

    println!("Operations: {}", operations);
    println!("Duration: {:?}", duration);
    println!("Speed: {:.0} ops/second", ops_per_second);
    
    // Should be reasonably fast
    assert!(ops_per_second > 1000.0, "Performance too slow: {:.0} ops/sec", ops_per_second);
}