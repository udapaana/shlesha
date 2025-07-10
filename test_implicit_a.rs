use shlesha::Shlesha;

fn main() {
    let shlesha = Shlesha::new();
    
    // Test ITRANS → Telugu conversion
    let input = "dharma";
    println!("Input: {}", input);
    
    match shlesha.transliterate("itrans", "telugu", input) {
        Ok(result) => println!("ITRANS → Telugu: {} → {}", input, result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Test SLP1 → Telugu conversion  
    let input2 = "Darma";
    println!("\nInput: {}", input2);
    
    match shlesha.transliterate("slp1", "telugu", input2) {
        Ok(result) => println!("SLP1 → Telugu: {} → {}", input2, result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Test Telugu → SLP1 conversion
    let input3 = "ధర్మ";
    println!("\nInput: {}", input3);
    
    match shlesha.transliterate("telugu", "slp1", input3) {
        Ok(result) => println!("Telugu → SLP1: {} → {}", input3, result),
        Err(e) => println!("Error: {}", e),
    }
}