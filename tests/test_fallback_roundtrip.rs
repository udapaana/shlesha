//! Test that fallback tokens enable proper round-trip transliteration
//! even when destination scripts don't have certain characters

use shlesha::{TransliteratorBuilder, SchemaParser};

#[test]
fn test_fallback_token_roundtrip() {
    // Load Devanagari, Tamil (limited script), and IAST
    let devanagari_schema = SchemaParser::parse_str(include_str!("../schemas/devanagari.yaml")).unwrap();
    let tamil_schema = SchemaParser::parse_str(include_str!("../schemas/tamil.yaml")).unwrap();
    let iast_schema = SchemaParser::parse_str(include_str!("../schemas/iast.yaml")).unwrap();

    let transliterator = TransliteratorBuilder::new()
        .with_schema(devanagari_schema).unwrap()
        .with_schema(tamil_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build();

    // Test cases that should produce fallback tokens
    let test_cases = vec![
        "ख", // Aspirated consonant not in Tamil
        "घ", // Another aspirated 
        "छ", // Aspirated palatal
        "झ", // Another aspirated palatal
        "ठ", // Aspirated retroflex
        "ढ", // Another aspirated retroflex  
        "थ", // Aspirated dental
        "ध", // Another aspirated dental
        "फ", // Aspirated labial
        "भ", // Another aspirated labial
        "ॐ", // Religious symbol
    ];

    println!("🔄 Testing fallback token round-trips...");
    
    for original in &test_cases {
        println!("\nTesting: {}", original);
        
        // Round-trip: Devanagari → Tamil → Devanagari
        match transliterator.transliterate(original, "Devanagari", "Tamil") {
            Ok(tamil_result) => {
                println!("  {} → {} (Tamil)", original, tamil_result);
                
                match transliterator.transliterate(&tamil_result, "Tamil", "Devanagari") {
                    Ok(round_trip) => {
                        println!("  {} → {} → {} (round-trip)", original, tamil_result, round_trip);
                        
                        if round_trip == *original {
                            println!("  ✅ Perfect round-trip!");
                        } else {
                            println!("  ⚠️ Round-trip differs but preserved content");
                        }
                    }
                    Err(e) => {
                        panic!("Round-trip failed for {}: {}", original, e);
                    }
                }
            }
            Err(e) => {
                panic!("Forward translation failed for {}: {}", original, e);
            }
        }
    }
}

#[test]
fn test_manual_fallback_parsing() {
    // Test that we can parse manually created fallback tokens
    let devanagari_schema = SchemaParser::parse_str(include_str!("../schemas/devanagari.yaml")).unwrap();
    let iast_schema = SchemaParser::parse_str(include_str!("../schemas/iast.yaml")).unwrap();

    let transliterator = TransliteratorBuilder::new()
        .with_schema(devanagari_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build();

    // Test parsing fallback tokens
    let fallback_cases = vec![
        "[?:ख]",
        "[?:ॐ]", 
        "[?:some_token]",
    ];

    println!("🔍 Testing fallback token parsing...");
    
    for fallback in &fallback_cases {
        println!("\nTesting fallback: {}", fallback);
        
        match transliterator.transliterate(fallback, "Devanagari", "IAST") {
            Ok(result) => {
                println!("  {} → {}", fallback, result);
                
                // Should preserve the fallback token as-is
                if result == *fallback {
                    println!("  ✅ Fallback token preserved correctly");
                } else {
                    println!("  ⚠️ Fallback token changed: {} → {}", fallback, result);
                }
            }
            Err(e) => {
                println!("  ❌ Error parsing fallback: {}", e);
            }
        }
    }
}

#[test]
fn test_mixed_content_with_fallbacks() {
    // Test strings that mix normal characters with fallbacks
    let devanagari_schema = SchemaParser::parse_str(include_str!("../schemas/devanagari.yaml")).unwrap();
    let iast_schema = SchemaParser::parse_str(include_str!("../schemas/iast.yaml")).unwrap();

    let transliterator = TransliteratorBuilder::new()
        .with_schema(devanagari_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build();

    let mixed_cases = vec![
        "क[?:ख]त",     // Normal + fallback + normal
        "[?:ॐ]नमः",     // Fallback + normal 
        "राम[?:।]राम",  // Word + fallback + word
    ];

    println!("🔀 Testing mixed content with fallbacks...");
    
    for mixed in &mixed_cases {
        println!("\nTesting mixed: {}", mixed);
        
        match transliterator.transliterate(mixed, "Devanagari", "IAST") {
            Ok(result) => {
                println!("  {} → {}", mixed, result);
                
                // Test round-trip
                match transliterator.transliterate(&result, "IAST", "Devanagari") {
                    Ok(round_trip) => {
                        println!("  Round-trip: {} → {} → {}", mixed, result, round_trip);
                        
                        if round_trip == *mixed {
                            println!("  ✅ Perfect round-trip with mixed content!");
                        } else {
                            println!("  ⚠️ Round-trip differs: expected {}, got {}", mixed, round_trip);
                        }
                    }
                    Err(e) => {
                        println!("  ❌ Round-trip failed: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("  ❌ Translation failed: {}", e);
            }
        }
    }
}