use shlesha::Shlesha;
use shlesha::modules::hub::tokens::{HubToken, AbugidaToken};
use shlesha::modules::hub::HubFormat;

fn main() {
    let transliterator = Shlesha::new();
    
    println!("=== Debugging Gujarati Virama Issue ===\n");
    
    // Step 1: Convert Devanagari to hub tokens
    println!("Step 1: Devanagari → Hub Tokens");
    let input = "धर्म";
    println!("Input: {}", input);
    
    // We need to trace the actual token conversion
    // Let's check what tokens we get for धर्म
    let expected_tokens = vec![
        "ध (dha)",
        "र (ra)", 
        "् (virama)",
        "म (ma)"
    ];
    
    println!("\nExpected token sequence:");
    for token in &expected_tokens {
        println!("  {}", token);
    }
    
    // Step 2: Convert to Gujarati
    println!("\nStep 2: Hub Tokens → Gujarati");
    let result = transliterator.transliterate(input, "devanagari", "gujarati").unwrap();
    println!("Result: {} (got {} chars)", result, result.chars().count());
    
    // Character breakdown
    println!("\nCharacter breakdown:");
    for (i, ch) in result.chars().enumerate() {
        println!("  [{}] {} (U+{:04X})", i, ch, ch as u32);
    }
    
    println!("\nExpected: ધર્મ");
    let expected = "ધર્મ";
    for (i, ch) in expected.chars().enumerate() {
        println!("  [{}] {} (U+{:04X})", i, ch, ch as u32);
    }
    
    // The issue is that we're getting ધરમ (no virama) instead of ધર્મ
    // This suggests the virama is being skipped entirely
    
    println!("\n=== Analysis ===");
    println!("The template is skipping virama tokens (line 269-271)");
    println!("But the consonant logic (line 190-252) should add viramas");
    println!("The pattern is: ધ (dha) + ર (ra) + ् (virama) + મ (ma)");
    println!("We need virama after ર because મ follows");
}