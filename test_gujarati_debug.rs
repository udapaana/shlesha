use shlesha::Shlesha;

fn main() {
    let transliterator = Shlesha::new();
    
    // Test the specific case
    let input = "धर्म";
    println!("Input: {}", input);
    
    // Convert to Gujarati
    let result = transliterator.transliterate(input, "devanagari", "gujarati").unwrap();
    println!("Result: {}", result);
    
    // Show character breakdown
    for (i, ch) in result.chars().enumerate() {
        println!("  {}: {} (U+{:04X})", i, ch, ch as u32);
    }
    
    // Expected: ધર્મ
    println!("\nExpected breakdown:");
    let expected = "ધર્મ";
    for (i, ch) in expected.chars().enumerate() {
        println!("  {}: {} (U+{:04X})", i, ch, ch as u32);
    }
}