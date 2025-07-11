#[test]
fn test_virama_across_scripts() {
    use shlesha::Shlesha;
    
    let transliterator = Shlesha::new();
    let input = "धर्म"; // dha + ra + virama + ma
    
    println!("Testing virama handling across scripts:");
    println!("Input: {} (Devanagari)", input);
    
    // Test various Indic scripts
    let scripts = vec![
        ("bengali", "ধর্ম"),
        ("gujarati", "ધર્મ"), 
        ("telugu", "ధర్మ"),
        ("kannada", "ಧರ್ಮ"),
        ("malayalam", "ധര്മ"),
    ];
    
    for (script, expected) in scripts {
        match transliterator.transliterate(input, "devanagari", script) {
            Ok(result) => {
                let chars_result: Vec<char> = result.chars().collect();
                let chars_expected: Vec<char> = expected.chars().collect();
                
                println!("\n{} script:", script);
                println!("  Result:   {} ({} chars)", result, chars_result.len());
                println!("  Expected: {} ({} chars)", expected, chars_expected.len());
                
                if chars_result.len() != chars_expected.len() {
                    println!("  ❌ Character count mismatch!");
                    for (i, ch) in chars_result.iter().enumerate() {
                        println!("    [{}] {} (U+{:04X})", i, ch, *ch as u32);
                    }
                    println!("  Expected breakdown:");
                    for (i, ch) in chars_expected.iter().enumerate() {
                        println!("    [{}] {} (U+{:04X})", i, ch, *ch as u32);
                    }
                } else {
                    println!("  ✅ Character count matches");
                }
            },
            Err(e) => println!("{}: Error - {}", script, e),
        }
    }
}