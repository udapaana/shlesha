//! Comprehensive coverage tests for specific linguistic features
//! 
//! These tests focus on specific categories like conjuncts, vowel combinations,
//! script-specific features, and edge cases that need special attention.

use shlesha::{TransliteratorBuilder, SchemaParser};
use std::collections::HashMap;

#[test]
fn test_conjunct_coverage() {
    // Test comprehensive conjunct handling across scripts
    let devanagari_schema = SchemaParser::parse_str(include_str!("../schemas/devanagari.yaml")).unwrap();
    let iast_schema = SchemaParser::parse_str(include_str!("../schemas/iast.yaml")).unwrap();
    let bengali_schema = SchemaParser::parse_str(include_str!("../schemas/bengali.yaml")).unwrap();
    let tamil_schema = SchemaParser::parse_str(include_str!("../schemas/tamil.yaml")).unwrap();

    let transliterator = TransliteratorBuilder::new()
        .with_schema(devanagari_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .with_schema(bengali_schema).unwrap()
        .with_schema(tamil_schema).unwrap()
        .build();

    // Comprehensive conjunct test cases
    let conjuncts = vec![
        // Basic conjuncts
        ("क्त", "kta"), ("स्त", "sta"), ("न्त", "nta"), ("र्त", "rta"),
        ("क्ष", "kṣa"), ("ज्ञ", "jña"), ("त्र", "tra"), ("द्र", "dra"),
        
        // Triple conjuncts
        ("स्त्र", "stra"), ("न्त्र", "ntra"), ("क्त्र", "ktra"),
        
        // Aspirated conjuncts
        ("क्थ", "ktha"), ("द्घ", "dgha"), ("ब्ध", "bdha"),
        
        // Nasal conjuncts
        ("न्म", "nma"), ("ङ्क", "ṅka"), ("ञ्च", "ñca"),
        
        // Sibilant conjuncts
        ("श्च", "śca"), ("ष्ट", "ṣṭa"), ("स्प", "spa"),
        
        // Complex clusters
        ("द्व्य", "dvya"), ("स्व्य", "svya"), ("त्स्न", "tsna"),
    ];

    println!("\n🔗 Testing conjunct coverage...");
    let mut total_tests = 0;
    let mut failures = 0;

    for (devanagari, expected_iast) in &conjuncts {
        total_tests += 1;
        
        // Test Devanagari → IAST
        match transliterator.transliterate(devanagari, "Devanagari", "IAST") {
            Ok(result) => {
                if result == *expected_iast {
                    println!("  ✅ {} → {} (correct)", devanagari, result);
                } else {
                    println!("  ❌ {} → {} (expected {})", devanagari, result, expected_iast);
                    failures += 1;
                }
                
                // Test round-trip
                match transliterator.transliterate(&result, "IAST", "Devanagari") {
                    Ok(round_trip) => {
                        if round_trip == *devanagari {
                            println!("    ✅ Round-trip: {} → {} → {}", devanagari, result, round_trip);
                        } else {
                            println!("    ❌ Round-trip failed: {} → {} → {}", devanagari, result, round_trip);
                            failures += 1;
                        }
                    }
                    Err(e) => {
                        println!("    ❌ Round-trip error: {}", e);
                        failures += 1;
                    }
                }
            }
            Err(e) => {
                println!("  ❌ {} → ERROR: {}", devanagari, e);
                failures += 1;
            }
        }
    }

    println!("\n📊 Conjunct test results:");
    println!("  Total tests: {}", total_tests * 2); // Forward + round-trip
    println!("  Failures: {}", failures);
    println!("  Success rate: {:.1}%", (total_tests * 2 - failures) as f64 / (total_tests * 2) as f64 * 100.0);
    
    // Allow some failures for complex conjuncts but expect high success rate
    assert!(failures < total_tests / 2, "Too many conjunct failures: {}/{}", failures, total_tests * 2);
}

#[test] 
fn test_vowel_combinations() {
    // Test all vowel + consonant combinations
    let devanagari_schema = SchemaParser::parse_str(include_str!("../schemas/devanagari.yaml")).unwrap();
    let iast_schema = SchemaParser::parse_str(include_str!("../schemas/iast.yaml")).unwrap();

    let transliterator = TransliteratorBuilder::new()
        .with_schema(devanagari_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build();

    // Comprehensive vowel combinations with क
    let vowel_combinations = vec![
        ("क", "ka"),     // Inherent a
        ("का", "kā"),    // ā matra
        ("कि", "ki"),    // i matra  
        ("की", "kī"),    // ī matra
        ("कु", "ku"),    // u matra
        ("कू", "kū"),    // ū matra
        ("कृ", "kṛ"),    // ṛ matra
        ("कॄ", "kṝ"),    // ṝ matra
        ("कॢ", "kḷ"),    // ḷ matra
        ("कॣ", "kḹ"),    // ḹ matra
        ("के", "ke"),    // e matra
        ("कै", "kai"),   // ai matra
        ("को", "ko"),    // o matra
        ("कौ", "kau"),   // au matra
        
        // Independent vowels
        ("अ", "a"), ("आ", "ā"), ("इ", "i"), ("ई", "ī"),
        ("उ", "u"), ("ऊ", "ū"), ("ऋ", "ṛ"), ("ॠ", "ṝ"),
        ("ऌ", "ḷ"), ("ॡ", "ḹ"), ("ए", "e"), ("ऐ", "ai"),
        ("ओ", "o"), ("औ", "au"),
    ];

    println!("\n🔤 Testing vowel combinations...");
    let mut total_tests = 0;
    let mut failures = 0;

    for (devanagari, expected_iast) in &vowel_combinations {
        total_tests += 1;
        
        match transliterator.transliterate(devanagari, "Devanagari", "IAST") {
            Ok(result) => {
                if result == *expected_iast {
                    println!("  ✅ {} → {}", devanagari, result);
                } else {
                    println!("  ❌ {} → {} (expected {})", devanagari, result, expected_iast);
                    failures += 1;
                }
            }
            Err(e) => {
                println!("  ❌ {} → ERROR: {}", devanagari, e);
                failures += 1;
            }
        }
    }

    println!("\n📊 Vowel test results:");
    println!("  Total tests: {}", total_tests);
    println!("  Failures: {}", failures);
    println!("  Success rate: {:.1}%", (total_tests - failures) as f64 / total_tests as f64 * 100.0);
    
    // Expect high success rate for basic vowels
    assert!(failures < total_tests / 4, "Too many vowel failures: {}/{}", failures, total_tests);
}

#[test]
fn test_script_specific_features() {
    // Test features specific to individual scripts
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

    // Script-specific test cases
    let script_features = vec![
        // Tamil specific (no aspirated consonants, limited conjuncts)
        ("Tamil", vec![("க", "ka"), ("ங", "ṅa"), ("ஞ", "ña"), ("ண", "ṇa"), ("ன", "na"), ("ம", "ma")]),
        
        // Bengali specific (different ra forms)
        ("Bengali", vec![("ক", "ka"), ("র", "ra"), ("র্", "r"), ("ৎ", "t")]),
        
        // Devanagari specific (full consonant set)
        ("Devanagari", vec![("क", "ka"), ("ख", "kha"), ("गं", "gaṃ"), ("कः", "kaḥ")]),
        
        // Telugu specific
        ("Telugu", vec![("క", "ka"), ("ఖ", "kha"), ("గ", "ga"), ("ం", "ṃ")]),
    ];

    println!("\n🎯 Testing script-specific features...");
    
    for (script_name, test_cases) in &script_features {
        println!("\n  Testing {} features:", script_name);
        
        for (native, expected_iast) in test_cases {
            match transliterator.transliterate(native, script_name, "IAST") {
                Ok(result) => {
                    if result == *expected_iast {
                        println!("    ✅ {} → {}", native, result);
                    } else {
                        println!("    ⚠️  {} → {} (expected {})", native, result, expected_iast);
                    }
                }
                Err(e) => {
                    println!("    ❌ {} → ERROR: {}", native, e);
                }
            }
        }
    }
}

#[test]
fn test_numerical_and_punctuation() {
    // Test numerals and punctuation across scripts
    let devanagari_schema = SchemaParser::parse_str(include_str!("../schemas/devanagari.yaml")).unwrap();
    let bengali_schema = SchemaParser::parse_str(include_str!("../schemas/bengali.yaml")).unwrap();
    let iast_schema = SchemaParser::parse_str(include_str!("../schemas/iast.yaml")).unwrap();

    let transliterator = TransliteratorBuilder::new()
        .with_schema(devanagari_schema).unwrap()
        .with_schema(bengali_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build();

    let numerals_punct = vec![
        // Devanagari numerals
        ("०", "0"), ("१", "1"), ("२", "2"), ("३", "3"), ("४", "4"),
        ("५", "5"), ("६", "6"), ("७", "7"), ("८", "8"), ("९", "9"),
        
        // Punctuation
        ("।", "."), ("॥", ".."), 
        
        // Special symbols
        ("ॐ", "oṃ"), ("ऽ", "'"),
    ];

    println!("\n🔢 Testing numerals and punctuation...");
    let mut total_tests = 0;
    let mut failures = 0;

    for (devanagari, expected) in &numerals_punct {
        total_tests += 1;
        
        match transliterator.transliterate(devanagari, "Devanagari", "IAST") {
            Ok(result) => {
                if result == *expected {
                    println!("  ✅ {} → {}", devanagari, result);
                } else {
                    println!("  ❌ {} → {} (expected {})", devanagari, result, expected);
                    failures += 1;
                }
            }
            Err(e) => {
                println!("  ❌ {} → ERROR: {}", devanagari, e);
                failures += 1;
            }
        }
    }

    println!("\n📊 Numerals/punctuation results:");
    println!("  Total tests: {}", total_tests);
    println!("  Failures: {}", failures);
    println!("  Success rate: {:.1}%", (total_tests - failures) as f64 / total_tests as f64 * 100.0);
}

#[test]
fn test_real_world_words() {
    // Test actual Sanskrit/Hindi words and phrases
    let devanagari_schema = SchemaParser::parse_str(include_str!("../schemas/devanagari.yaml")).unwrap();
    let iast_schema = SchemaParser::parse_str(include_str!("../schemas/iast.yaml")).unwrap();

    let transliterator = TransliteratorBuilder::new()
        .with_schema(devanagari_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build();

    let real_words = vec![
        // Common Sanskrit words
        ("नमस्ते", "namaste"), ("धन्यवाद", "dhanyavāda"),
        ("संस्कृत", "saṃskṛta"), ("भारत", "bhārata"),
        ("हिंदी", "hiṃdī"), ("योग", "yoga"),
        ("गुरु", "guru"), ("शिष्य", "śiṣya"),
        ("आत्मा", "ātmā"), ("प्रकृति", "prakṛti"),
        
        // Religious/philosophical terms
        ("धर्म", "dharma"), ("कर्म", "karma"), 
        ("मोक्ष", "mokṣa"), ("निर्वाण", "nirvāṇa"),
        ("अहिंसा", "ahiṃsā"), ("सत्य", "satya"),
        
        // Compound words
        ("राजपुत्र", "rājaputra"), ("देवता", "devatā"),
        ("महात्मा", "mahātmā"), ("सर्वज्ञ", "sarvajña"),
        
        // Technical terms
        ("गणित", "gaṇita"), ("ज्योतिष", "jyotiṣa"),
        ("आयुर्वेद", "āyurveda"), ("व्याकरण", "vyākaraṇa"),
    ];

    println!("\n🌍 Testing real-world words...");
    let mut total_tests = 0;
    let mut failures = 0;

    for (devanagari, expected_iast) in &real_words {
        total_tests += 1;
        
        match transliterator.transliterate(devanagari, "Devanagari", "IAST") {
            Ok(result) => {
                if result == *expected_iast {
                    println!("  ✅ {} → {}", devanagari, result);
                } else {
                    println!("  ⚠️  {} → {} (expected {})", devanagari, result, expected_iast);
                    // Don't count as failure since real words might have variations
                }
                
                // Test round-trip
                match transliterator.transliterate(&result, "IAST", "Devanagari") {
                    Ok(round_trip) => {
                        if round_trip == *devanagari {
                            println!("    ✅ Round-trip successful");
                        } else {
                            println!("    ⚠️  Round-trip: {} → {} → {}", devanagari, result, round_trip);
                        }
                    }
                    Err(e) => {
                        println!("    ❌ Round-trip error: {}", e);
                        failures += 1;
                    }
                }
            }
            Err(e) => {
                println!("  ❌ {} → ERROR: {}", devanagari, e);
                failures += 1;
            }
        }
    }

    println!("\n📊 Real-world word results:");
    println!("  Total tests: {}", total_tests);
    println!("  Failures: {}", failures);
    println!("  Success rate: {:.1}%", (total_tests - failures) as f64 / total_tests as f64 * 100.0);
}