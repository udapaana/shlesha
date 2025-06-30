use shlesha::Shlesha;

fn main() {
    let shlesha = Shlesha::new();

    println!("🔍 Debugging Direct Mapping");
    println!("===========================");

    // Test exact matches from the generated mappings
    let test_cases = vec![
        ("namaste", "ISO words with exact matches"),
        ("ka", "Single consonant"),
        ("ga", "Another consonant"),
        ("dharma", "Word with 'dha'"),
        ("yoga", "Word with 'ya'"),
    ];

    for (input, desc) in test_cases {
        println!("\n🧪 Testing: {} ({})", input, desc);

        // Test ISO -> Devanagari direct mapping
        match shlesha.transliterate(input, "iso15919", "devanagari") {
            Ok(result) => println!("  ✅ iso15919 → devanagari: {} → {}", input, result),
            Err(e) => println!("  ❌ iso15919 → devanagari: {} → Error: {}", input, e),
        }

        // Test the reverse to see if it works
        if let Ok(deva_result) = shlesha.transliterate(input, "iso15919", "devanagari") {
            match shlesha.transliterate(&deva_result, "devanagari", "iso15919") {
                Ok(back_to_iso) => println!(
                    "  ✅ Round-trip: {} → {} → {}",
                    input, deva_result, back_to_iso
                ),
                Err(e) => println!("  ❌ Round-trip failed: {}", e),
            }
        }

        // Compare with hub-based conversion (IAST)
        match shlesha.transliterate(input, "iast", "devanagari") {
            Ok(hub_result) => println!("  📊 Hub-based (IAST): {} → {}", input, hub_result),
            Err(e) => println!("  ❌ Hub-based error: {}", e),
        }
    }

    println!("\n💡 Analysis:");
    println!("- Direct mappings should handle exact sequences like 'ka', 'ga', 'dha'");
    println!("- Differences between direct and hub-based show pre-computation impact");
}
