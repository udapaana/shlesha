use shlesha::Shlesha;

fn main() {
    let transliterator = Shlesha::new();
    
    // Test the API that's failing
    let result = transliterator
        .transliterate_with_metadata("अ", "devanagari", "iast")
        .unwrap();
    
    println!("Output: '{}'", result.output);
    println!("Expected: 'a'");
    
    // Also test direct conversion
    let simple_result = transliterator
        .transliterate("अ", "devanagari", "iast")
        .unwrap();
    
    println!("Simple result: '{}'", simple_result);
}