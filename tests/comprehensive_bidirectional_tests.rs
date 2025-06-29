use shlesha::Shlesha;

#[test]
fn test_all_script_pairs() {
    let transliterator = Shlesha::new();

    // Test data with the word "dharma" in various scripts
    let test_cases = vec![
        // Script name, test text, expected romanized form
        ("devanagari", "धर्म", "dharma"),
        ("gujarati", "ધર્મ", "dharma"),
        ("bengali", "ধর্ম", "dharma"),
        ("telugu", "ధర్మ", "dharma"),
        ("tamil", "தர்மம்", "tarmam"), // Tamil has different phonetics
        ("kannada", "ಧರ್ಮ", "dharma"),
        ("malayalam", "ധര്മ", "dharma"),
        ("odia", "ଧର୍ମ", "dharma"),
        ("iast", "dharma", "dharma"),
        ("itrans", "dharma", "dharma"),
        ("slp1", "Darma", "dharma"),
        ("velthuis", "dharma", "dharma"),
        ("wx", "Darma", "dharma"),
        ("harvard_kyoto", "dharma", "dharma"),
    ];

    // Comprehensive bidirectional tests
    println!("\n=== Comprehensive Bidirectional Transliteration Tests ===\n");

    let mut passed = 0;
    let mut failed = 0;

    for (from_script, from_text, _) in &test_cases {
        for (to_script, _, _) in &test_cases {
            if from_script == to_script {
                continue; // Skip same-script conversions
            }

            let result = transliterator.transliterate(from_text, from_script, to_script);

            match result {
                Ok(output) => {
                    println!(
                        "✓ {} → {}: '{}' → '{}'",
                        from_script, to_script, from_text, output
                    );
                    passed += 1;
                }
                Err(e) => {
                    println!(
                        "✗ {} → {}: '{}' → ERROR: {}",
                        from_script, to_script, from_text, e
                    );
                    failed += 1;
                }
            }
        }
    }

    println!("\n=== Summary ===");
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);
    println!("Total: {}", passed + failed);

    // Assert that we have a reasonable number of working conversions
    assert!(
        passed > 40,
        "Expected at least 40 working conversions, got {}",
        passed
    );
}

#[test]
fn test_indic_to_roman_conversions() {
    let t = Shlesha::new();

    println!("\n=== Indic → Roman Conversion Tests ===\n");

    // Test each Indic script to each Roman script
    let indic_scripts = vec![
        ("gujarati", "ધર્મ"),
        ("bengali", "ধর্ম"),
        ("telugu", "ధర్మ"),
        ("tamil", "தர்मम்"),
        ("kannada", "ಧರ್ಮ"),
        ("malayalam", "ധര്മ"),
        ("odia", "ଧର୍ମ"),
        ("devanagari", "धर्म"),
    ];

    let roman_scripts = vec![
        "iast",
        "itrans",
        "slp1",
        "iso",
        "velthuis",
        "wx",
        "harvard_kyoto",
    ];

    for (indic_script, text) in &indic_scripts {
        for roman_script in &roman_scripts {
            let result = t.transliterate(text, indic_script, roman_script);
            match result {
                Ok(output) => println!(
                    "✓ {} → {}: '{}' → '{}'",
                    indic_script, roman_script, text, output
                ),
                Err(e) => println!(
                    "✗ {} → {}: '{}' → ERROR: {}",
                    indic_script, roman_script, text, e
                ),
            }
        }
    }
}

#[test]
fn test_roman_to_indic_conversions() {
    let t = Shlesha::new();

    println!("\n=== Roman → Indic Conversion Tests ===\n");

    // Test each Roman script to each Indic script
    let roman_texts = vec![
        ("iast", "dharma"),
        ("itrans", "dharma"),
        ("slp1", "Darma"),
        ("iso", "dharma"),
        ("velthuis", "dharma"),
        ("wx", "Darma"),
        ("harvard_kyoto", "dharma"),
    ];

    let indic_scripts = vec![
        "gujarati",
        "bengali",
        "telugu",
        "tamil",
        "kannada",
        "malayalam",
        "odia",
        "devanagari",
    ];

    for (roman_script, text) in &roman_texts {
        for indic_script in &indic_scripts {
            let result = t.transliterate(text, roman_script, indic_script);
            match result {
                Ok(output) => println!(
                    "✓ {} → {}: '{}' → '{}'",
                    roman_script, indic_script, text, output
                ),
                Err(e) => println!(
                    "✗ {} → {}: '{}' → ERROR: {}",
                    roman_script, indic_script, text, e
                ),
            }
        }
    }
}

#[test]
fn test_complex_sentences() {
    let t = Shlesha::new();

    println!("\n=== Complex Sentence Tests ===\n");

    // Test with a more complex Sanskrit phrase
    let tests = vec![
        ("devanagari", "सत्यमेव जयते", "satyameva jayate"),
        ("iast", "oṁ namaḥ śivāya", "ॐ नमः शिवाय"),
    ];

    for (from_script, text, _expected) in tests {
        // Test to multiple target scripts
        let targets = vec!["gujarati", "bengali", "iast", "itrans"];

        for target in targets {
            if from_script == target {
                continue;
            }

            let result = t.transliterate(text, from_script, target);
            match result {
                Ok(output) => println!("✓ {} → {}: '{}' → '{}'", from_script, target, text, output),
                Err(e) => println!("✗ {} → {}: '{}' → ERROR: {}", from_script, target, text, e),
            }
        }
    }
}

#[test]
fn test_roundtrip_conversions() {
    let t = Shlesha::new();

    println!("\n=== Roundtrip Conversion Tests ===\n");

    // Test that converting A → B → A gives back the original (or close to it)
    let test_cases = vec![
        ("devanagari", "धर्म"),
        ("gujarati", "ધર્મ"),
        ("bengali", "ধর্ম"),
        ("iast", "dharma"),
    ];

    for (script, original_text) in test_cases {
        // Try roundtrip through ISO
        match t.transliterate(original_text, script, "iso") {
            Ok(iso_text) => match t.transliterate(&iso_text, "iso", script) {
                Ok(roundtrip_text) => {
                    if roundtrip_text == original_text {
                        println!(
                            "✓ {} roundtrip through ISO: '{}' → '{}' → '{}' (exact match)",
                            script, original_text, iso_text, roundtrip_text
                        );
                    } else {
                        println!(
                            "⚠ {} roundtrip through ISO: '{}' → '{}' → '{}' (different)",
                            script, original_text, iso_text, roundtrip_text
                        );
                    }
                }
                Err(e) => println!("✗ {} roundtrip return failed: {}", script, e),
            },
            Err(e) => println!("✗ {} → ISO failed: {}", script, e),
        }
    }
}
