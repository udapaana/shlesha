use shlesha::Shlesha;

fn main() {
    let shlesha = Shlesha::new();

    println!("=== Testing Implicit 'a' Vowel Handling ===");

    // Test simple vowel
    let input1 = "a";
    match shlesha.transliterate(input1, "itrans", "telugu") {
        Ok(result) => println!("Vowel: {} → {}", input1, result),
        Err(e) => println!("Error: {}", e),
    }

    // Test simple consonant (should have implicit 'a')
    let input2 = "k";
    match shlesha.transliterate(input2, "itrans", "telugu") {
        Ok(result) => println!("Consonant: {} → {}", input2, result),
        Err(e) => println!("Error: {}", e),
    }

    // Test consonant + long vowel sign
    let input3 = "kaa";
    match shlesha.transliterate(input3, "itrans", "telugu") {
        Ok(result) => println!("Consonant+long vowel: {} → {}", input3, result),
        Err(e) => println!("Error: {}", e),
    }

    // Test proper ITRANS word
    let input4 = "dharm";
    match shlesha.transliterate(input4, "itrans", "telugu") {
        Ok(result) => println!("Word: {} → {}", input4, result),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== Testing Reverse Conversion ===");

    // Test Telugu → ITRANS
    let input5 = "ధర్మ";
    match shlesha.transliterate(input5, "telugu", "itrans") {
        Ok(result) => println!("Telugu → ITRANS: {} → {}", input5, result),
        Err(e) => println!("Error: {}", e),
    }
}
