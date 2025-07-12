use shlesha::Shlesha;

fn main() {
    let shlesha = Shlesha::new();

    println!("=== Testing MarkVirama handling ===");

    // Test Telugu → SLP1 (the case that was failing)
    let input = "సంస్కృతం";
    match shlesha.transliterate(input, "telugu", "slp1") {
        Ok(result) => println!("Telugu → SLP1: {} → {}", input, result),
        Err(e) => println!("Error: {}", e),
    }

    // Test other Indic → Roman conversions that should now work
    let inputs = [
        ("సంస్కృతం", "telugu", "iast"),
        ("নমস্তে", "bengali", "iast"),
        ("નમસ્તે", "gujarati", "iast"),
    ];

    for (text, from, to) in inputs {
        match shlesha.transliterate(text, from, to) {
            Ok(result) => println!("{} → {}: {} → {}", from, to, text, result),
            Err(e) => println!("Error: {}", e),
        }
    }
}
