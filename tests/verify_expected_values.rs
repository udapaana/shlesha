use shlesha::{TransliteratorBuilder, SchemaParser};

#[test]
fn verify_basic_consonant_mappings() {
    // Load just Devanagari and IAST to verify our expected values
    let devanagari_schema = SchemaParser::parse_str(include_str!("../schemas/devanagari.yaml")).unwrap();
    let iast_schema = SchemaParser::parse_str(include_str!("../schemas/iast.yaml")).unwrap();

    let transliterator = TransliteratorBuilder::new()
        .with_schema(devanagari_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build();

    // Test basic consonants from Devanagari to IAST to see actual results
    let test_consonants = vec![
        "क", "ख", "ग", "घ", "ङ",
        "च", "छ", "ज", "झ", "ञ", 
        "ट", "ठ", "ड", "ढ", "ण",
        "त", "थ", "द", "ध", "न",
        "प", "फ", "ब", "भ", "म",
        "य", "र", "ल", "व",
        "श", "ष", "स", "ह",
    ];

    println!("=== ACTUAL DEVANAGARI TO IAST MAPPINGS ===");
    for consonant in &test_consonants {
        match transliterator.transliterate(consonant, "Devanagari", "IAST") {
            Ok(result) => println!("(\"{}\", \"{}\"),", consonant, result),
            Err(e) => println!("\"{}\" -> ERROR: {}", consonant, e),
        }
    }

    // Test vowels
    let test_vowels = vec![
        "अ", "आ", "इ", "ई", "उ", "ऊ", "ऋ", "ॠ", "ऌ", "ॡ", "ए", "ऐ", "ओ", "औ"
    ];

    println!("\n=== ACTUAL VOWEL MAPPINGS ===");
    for vowel in &test_vowels {
        match transliterator.transliterate(vowel, "Devanagari", "IAST") {
            Ok(result) => println!("(\"{}\", \"{}\"),", vowel, result),
            Err(e) => println!("\"{}\" -> ERROR: {}", vowel, e),
        }
    }

    // Test some basic words
    let test_words = vec![
        "राम", "धर्म", "कर्म", "योग", "गुरु", "माता", "पिता", "जल", "अग्नि", "वायु"
    ];

    println!("\n=== ACTUAL WORD MAPPINGS ===");
    for word in &test_words {
        match transliterator.transliterate(word, "Devanagari", "IAST") {
            Ok(result) => println!("(\"{}\", \"{}\"),", word, result),
            Err(e) => println!("\"{}\" -> ERROR: {}", word, e),
        }
    }
}

#[test]
fn verify_romanization_scheme_mappings() {
    // Load romanization schemes and test basic mappings
    let iast_schema = SchemaParser::parse_str(include_str!("../schemas/iast.yaml")).unwrap();
    let harvard_kyoto_schema = SchemaParser::parse_str(include_str!("../schemas/harvard_kyoto.yaml")).unwrap();
    let itrans_schema = SchemaParser::parse_str(include_str!("../schemas/itrans.yaml")).unwrap();

    let transliterator = TransliteratorBuilder::new()
        .with_schema(iast_schema).unwrap()
        .with_schema(harvard_kyoto_schema).unwrap()
        .with_schema(itrans_schema).unwrap()
        .build();

    // Test IAST to Harvard-Kyoto basic mappings
    let iast_consonants = vec![
        "ka", "kha", "ga", "ta", "da", "na", "pa", "ba", "ma", "sa"
    ];

    println!("\n=== IAST TO HARVARD-KYOTO ===");
    for consonant in &iast_consonants {
        match transliterator.transliterate(consonant, "IAST", "Harvard-Kyoto") {
            Ok(result) => println!("\"{}\" -> \"{}\"", consonant, result),
            Err(e) => println!("\"{}\" -> ERROR: {}", consonant, e),
        }
    }

    println!("\n=== IAST TO ITRANS ===");
    for consonant in &iast_consonants {
        match transliterator.transliterate(consonant, "IAST", "ITRANS") {
            Ok(result) => println!("\"{}\" -> \"{}\"", consonant, result),
            Err(e) => println!("\"{}\" -> ERROR: {}", consonant, e),
        }
    }
}

#[test]
fn verify_all_script_basic_chars() {
    // Load all schemas to test basic character availability
    let scripts = vec![
        ("Devanagari", include_str!("../schemas/devanagari.yaml")),
        ("Bengali", include_str!("../schemas/bengali.yaml")),
        ("Tamil", include_str!("../schemas/tamil.yaml")),
        ("Telugu", include_str!("../schemas/telugu.yaml")),
        ("Kannada", include_str!("../schemas/kannada.yaml")),
        ("Malayalam", include_str!("../schemas/malayalam.yaml")),
        ("Gujarati", include_str!("../schemas/gujarati.yaml")),
        ("Odia", include_str!("../schemas/odia.yaml")),
        ("Gurmukhi", include_str!("../schemas/gurmukhi.yaml")),
        ("IAST", include_str!("../schemas/iast.yaml")),
    ];

    let mut builder = TransliteratorBuilder::new();
    for (_, content) in &scripts {
        let schema = SchemaParser::parse_str(content).unwrap();
        builder = builder.with_schema(schema).unwrap();
    }
    let transliterator = builder.build();

    // Test basic consonants that should exist in ALL scripts
    let basic_consonants = vec!["क", "त", "न", "प", "म", "स"];
    
    println!("\n=== TESTING BASIC CONSONANTS ACROSS ALL SCRIPTS ===");
    for consonant in &basic_consonants {
        println!("\nTesting {}:", consonant);
        
        // First convert to IAST to get canonical form
        match transliterator.transliterate(consonant, "Devanagari", "IAST") {
            Ok(iast_form) => {
                println!("  {} -> IAST: {}", consonant, iast_form);
                
                // Test conversion to all other scripts
                for (script_name, _) in &scripts {
                    if *script_name != "IAST" && *script_name != "Devanagari" {
                        match transliterator.transliterate(&iast_form, "IAST", script_name) {
                            Ok(result) => {
                                // Test round-trip
                                match transliterator.transliterate(&result, script_name, "IAST") {
                                    Ok(back_to_iast) => {
                                        if back_to_iast == iast_form {
                                            println!("  ✅ {}: {} (round-trip OK)", script_name, result);
                                        } else {
                                            println!("  ❌ {}: {} (round-trip: {} != {})", script_name, result, back_to_iast, iast_form);
                                        }
                                    }
                                    Err(e) => println!("  ❌ {}: {} (round-trip ERROR: {})", script_name, result, e),
                                }
                            }
                            Err(e) => println!("  ❌ {}: ERROR: {}", script_name, e),
                        }
                    }
                }
            }
            Err(e) => println!("  ERROR converting to IAST: {}", e),
        }
    }
}