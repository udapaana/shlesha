// Debug the token flow to see where virama tokens are lost
use shlesha::Shlesha;

fn main() {
    let transliterator = Shlesha::new();
    
    // Test a simple word with virama
    let test_cases = vec![
        ("धर्म", "devanagari", "iast"),    // Should work (abugida -> alphabet)
        ("धर्म", "devanagari", "gujarati"), // Broken (abugida -> abugida)
    ];
    
    for (input, from, to) in test_cases {
        println!("\n=== Testing: {} ({} -> {}) ===", input, from, to);
        
        match transliterator.transliterate(input, from, to) {
            Ok(result) => {
                println!("✅ Result: {}", result);
                println!("   Characters: {:?}", result.chars().collect::<Vec<_>>());
                
                // Print Unicode codepoints
                for (i, ch) in result.chars().enumerate() {
                    println!("   [{}] {} (U+{:04X})", i, ch, ch as u32);
                }
            }
            Err(e) => {
                println!("❌ Error: {}", e);
            }
        }
    }
    
    println!("\n=== Analysis ===");
    println!("If IAST works but Gujarati doesn't, the issue is:");
    println!("1. Hub converter abugida->alphabet discards viramas (correct for IAST)");
    println!("2. Hub converter should preserve AbugidaTokens for abugida->abugida");
    println!("3. Template should output virama tokens when present");
}