use shlesha::{TransliteratorBuilder, SchemaParser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create sample schemas inline for testing
    let devanagari_schema = r#"
name: "Devanagari"
type: abugida

mappings:
  consonants:
    "न": { canonical: "na" }
    "म": { canonical: "ma" }
    "स": { canonical: "sa" }
    "क": { canonical: "ka" }
    "त": { canonical: "ta" }
  vowels:
    "अ": { canonical: "a", type: vowel_independent }
    "आ": { canonical: "ā", type: vowel_independent }
    "ा": { canonical: "ā", type: vowel_dependent }
    "ि": { canonical: "i", type: vowel_dependent }
  modifiers:
    "्": { canonical: "", type: virama }
    "ं": { canonical: "ṃ", type: anusvara }
    "ः": { canonical: "ḥ", type: visarga }
"#;

    let iast_schema = r#"
name: "IAST"
type: alphabet

mappings:
  consonants:
    "n": { canonical: "n" }
    "m": { canonical: "m" }
    "s": { canonical: "s" }
    "k": { canonical: "k" }
    "t": { canonical: "t" }
  vowels:
    "a": { canonical: "a" }
    "ā": { canonical: "ā" }
    "i": { canonical: "i" }
  modifiers:
    "ṃ": { canonical: "ṃ", type: anusvara }
    "ḥ": { canonical: "ḥ", type: visarga }
"#;

    // Parse schemas
    let dev_schema = SchemaParser::parse_str(devanagari_schema)?;
    let iast_schema = SchemaParser::parse_str(iast_schema)?;

    // Build transliterator
    let transliterator = TransliteratorBuilder::new()
        .with_schema(dev_schema)?
        .with_schema(iast_schema)?
        .build();

    // Test cases
    println!("=== Devanagari to IAST ===");
    
    let test_cases = vec![
        ("नमस्ते", "namaste"),
        ("संस्कृतम्", "saṃskṛtam"),
        ("काम", "kāma"),
        ("किम्", "kim"),
    ];

    for (devanagari, expected) in &test_cases {
        match transliterator.transliterate(devanagari, "Devanagari", "IAST") {
            Ok(result) => {
                println!("{} → {} (expected: {})", devanagari, result, expected);
            }
            Err(e) => {
                println!("Error transliterating {}: {}", devanagari, e);
            }
        }
    }

    println!("\n=== IAST to Devanagari ===");
    
    let reverse_cases = vec![
        ("namaste", "नमस्ते"),
        ("kāma", "काम"),
    ];

    for (iast, expected) in &reverse_cases {
        match transliterator.transliterate(iast, "IAST", "Devanagari") {
            Ok(result) => {
                println!("{} → {} (expected: {})", iast, result, expected);
            }
            Err(e) => {
                println!("Error transliterating {}: {}", iast, e);
            }
        }
    }

    Ok(())
}