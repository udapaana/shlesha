use shlesha::Shlesha;

fn main() {
    let shlesha = Shlesha::new();

    println!("=== Testing IAST conversions ===");

    // Test IAST → ITRANS
    let input1 = "namaste";
    match shlesha.transliterate(input1, "iast", "itrans") {
        Ok(result) => println!("IAST → ITRANS: {} → {}", input1, result),
        Err(e) => println!("Error: {}", e),
    }

    // Test IAST → Telugu
    let input2 = "dharma";
    match shlesha.transliterate(input2, "iast", "telugu") {
        Ok(result) => println!("IAST → Telugu: {} → {}", input2, result),
        Err(e) => println!("Error: {}", e),
    }
}
