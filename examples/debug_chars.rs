use shlesha::LosslessTransliterator;

fn main() {
    let trans = LosslessTransliterator::new();
    let test_texts = ["फबपलक", "कखवलगप", "व", "प", "ल"];
    
    for test_text in &test_texts {
        println!("\n=== Testing: {} ===", test_text);
        
        match trans.transliterate(test_text, "Devanagari", "IAST") {
            Ok(result) => {
                println!("Result: {}", result);
                let verification = trans.verify_lossless(test_text, &result, "Devanagari");
                println!("Is lossless: {}", verification.is_lossless);
                println!("Preservation ratio: {:.3}", verification.preservation_ratio);
                println!("Tokens count: {}", verification.tokens_count);
                
                if !verification.is_lossless {
                    println!("DETAILS:");
                    for (i, ch) in test_text.chars().enumerate() {
                        let single_result = trans.transliterate(&ch.to_string(), "Devanagari", "IAST").unwrap();
                        println!("  {}: {} -> {}", i, ch, single_result);
                        if single_result.contains("[") {
                            println!("    ^ This character created a token!");
                        }
                    }
                }
            }
            Err(e) => println!("Error: {}", e)
        }
    }
}