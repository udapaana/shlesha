use shlesha::Shlesha;

fn main() {
    let shlesha = Shlesha::new();

    println!("=== Testing newly converted scripts ===");

    // Test Bengali
    let input1 = "namaste";
    match shlesha.transliterate(input1, "iast", "bengali") {
        Ok(result) => println!("IAST → Bengali: {} → {}", input1, result),
        Err(e) => println!("Error: {}", e),
    }

    // Test Tamil
    let input2 = "namaste";
    match shlesha.transliterate(input2, "iast", "tamil") {
        Ok(result) => println!("IAST → Tamil: {} → {}", input2, result),
        Err(e) => println!("Error: {}", e),
    }

    // Test Roman to Roman conversions
    let input3 = "namaste";
    match shlesha.transliterate(input3, "iast", "kolkata") {
        Ok(result) => println!("IAST → Kolkata: {} → {}", input3, result),
        Err(e) => println!("Error: {}", e),
    }
}
