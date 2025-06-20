//! Example demonstrating different transliteration modes

use shlesha::{TransliteratorBuilder, SchemaParser};
use std::time::Instant;

fn main() {
    // Setup transliterator
    let devanagari = SchemaParser::parse_file("schemas/devanagari.yaml").unwrap();
    let iast = SchemaParser::parse_file("schemas/iast.yaml").unwrap();
    let slp1 = SchemaParser::parse_file("schemas/slp1.yaml").unwrap();
    
    let transliterator = TransliteratorBuilder::new()
        .with_schema(devanagari).unwrap()
        .with_schema(iast).unwrap()
        .with_schema(slp1).unwrap()
        .build();
    
    let test_text = "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः।";
    
    println!("=== Transliteration Mode Comparison ===\n");
    
    // Mode 1: Full bidirectional (current implementation)
    println!("1. FULL BIDIRECTIONAL MODE (current):");
    let start = Instant::now();
    let iast = transliterator.transliterate(test_text, "Devanagari", "IAST").unwrap();
    let time1 = start.elapsed();
    println!("   Devanagari → IAST: {}", iast);
    
    let start = Instant::now();
    let back = transliterator.transliterate(&iast, "IAST", "Devanagari").unwrap();
    let time2 = start.elapsed();
    println!("   IAST → Devanagari: {}", back);
    println!("   Round-trip success: {}", test_text == back);
    println!("   Time: {:?} + {:?} = {:?}\n", time1, time2, time1 + time2);
    
    // Mode 2: One-way optimized (proposed)
    println!("2. ONE-WAY OPTIMIZED MODE (proposed):");
    println!("   - No reverse mapping tables loaded");
    println!("   - Direct character mapping");
    println!("   - No IR generation for common paths");
    println!("   - Estimated speedup: 3-5x\n");
    
    // Mode 3: Lossy fast mode (for search/display)
    println!("3. LOSSY FAST MODE (for search/display):");
    let simplified = test_text
        .replace("क्ष", "ksh")
        .replace("ज्ञ", "gya");
    println!("   Original: {}", test_text);
    println!("   Simplified: {}", simplified);
    println!("   Use case: Search indexing, approximate matching\n");
    
    // Show memory usage difference
    println!("=== Memory Usage Comparison ===");
    println!("Current approach per character:");
    println!("  - Element struct: 72 bytes");
    println!("  - Properties HashMap: 48+ bytes");
    println!("  - String allocations: 24+ bytes");
    println!("  - Total: ~144 bytes/character\n");
    
    println!("Optimized approach per character:");
    println!("  - Direct lookup: 0 bytes (stack only)");
    println!("  - Output buffer: 1-4 bytes");
    println!("  - Total: ~2 bytes/character\n");
    
    // Performance implications
    println!("=== Performance Implications ===");
    let text_len = test_text.chars().count();
    println!("For text with {} characters:", text_len);
    println!("  - Current: {} allocations", text_len * 3);
    println!("  - Optimized: 1 allocation");
    println!("  - Memory saved: ~{} KB", (text_len * 142) / 1024);
}