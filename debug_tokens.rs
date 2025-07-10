use shlesha::Shlesha;

fn main() {
    let shlesha = Shlesha::new();
    let input = "धर्मkr";
    
    // Test conversion
    println\!("Input: {}", input);
    match shlesha.transliterate(input, "devanagari", "gujarati") {
        Ok(result) => println\!("Output: {}", result),
        Err(e) => println\!("Error: {}", e),
    }
}
EOF < /dev/null