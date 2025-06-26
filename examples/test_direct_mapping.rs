use shlesha::Shlesha;

fn main() {
    let shlesha = Shlesha::new();
    
    println!("🧪 Testing Direct Mapping Integration");
    println!("====================================");
    
    // Test text
    let test_text = "namaste yoga dharma";
    
    // Test if direct mapping exists
    println!("\n📊 Direct Mapping Tests:");
    
    // ISO to Devanagari (should use direct mapping)
    match shlesha.transliterate(test_text, "iso15919", "devanagari") {
        Ok(result) => println!("  ✅ iso15919 → devanagari: {}", result),
        Err(e) => println!("  ❌ iso15919 → devanagari: Error - {}", e),
    }
    
    // Devanagari to ISO (should use direct mapping)
    let deva_text = "नमस्ते योग धर्म";
    match shlesha.transliterate(deva_text, "devanagari", "iso15919") {
        Ok(result) => println!("  ✅ devanagari → iso15919: {}", result),
        Err(e) => println!("  ❌ devanagari → iso15919: Error - {}", e),
    }
    
    // IAST to Devanagari (should go through hub)
    match shlesha.transliterate(test_text, "iast", "devanagari") {
        Ok(result) => println!("  ✅ iast → devanagari: {}", result),
        Err(e) => println!("  ❌ iast → devanagari: Error - {}", e),
    }
    
    println!("\n💡 Interpretation:");
    println!("- Direct mappings for iso15919 ↔ devanagari should be faster than hub-based conversions");
    println!("- Other conversions still use the hub system");
}