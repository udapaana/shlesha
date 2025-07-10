use shlesha::Shlesha;

fn main() {
    let shlesha = Shlesha::new();
    
    println!("Testing token-based conversion from SLP1 to Telugu...");
    
    match shlesha.transliterate("dharma", "slp1", "telugu") {
        Ok(result) => {
            println!("Success: '{}' -> '{}'", "dharma", result);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    
    // Test another conversion
    match shlesha.transliterate("dharma", "slp1", "devanagari") {
        Ok(result) => {
            println!("Success: '{}' -> '{}'", "dharma", result);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    
    // Test telugu to itrans
    match shlesha.transliterate("ధర్మ", "telugu", "itrans") {
        Ok(result) => {
            println!("Success: '{}' -> '{}'", "ధర్మ", result);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}