use shlesha::{Transliterator, TransliteratorBuilder, SchemaParser};
use std::time::Instant;

fn main() {
    // Setup transliterator
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema_parsed = SchemaParser::parse_str(iast_schema).unwrap();
    
    let transliterator = TransliteratorBuilder::new()
        .with_schema(dev_schema).unwrap()
        .with_schema(iast_schema_parsed).unwrap()
        .build();
    
    // Test cases
    let test_cases = vec![
        ("क", "Single consonant"),
        ("नम", "Two consonants"),
        ("नमस्ते", "Simple word"),
        ("संस्कृतम्", "With complex conjuncts"),
        ("कृष्णार्जुनसंवादः", "Complex word"),
        ("धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः", "Verse fragment"),
    ];
    
    println!("Performance Analysis by Text Complexity:\n");
    println!("{:<30} {:<10} {:<15} {:<15} {:<15}", "Text", "Chars", "Time (µs)", "µs/char", "MB/s");
    println!("{}", "-".repeat(85));
    
    for (text, description) in test_cases {
        // Warm up
        for _ in 0..100 {
            let _ = transliterator.transliterate(text, "Devanagari", "IAST");
        }
        
        // Measure
        let iterations = 1000;
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = transliterator.transliterate(text, "Devanagari", "IAST").unwrap();
        }
        let total_time = start.elapsed();
        let avg_time = total_time / iterations as u32;
        let avg_micros = avg_time.as_micros() as f64;
        
        let char_count = text.chars().count();
        let bytes = text.len();
        let micros_per_char = avg_micros / char_count as f64;
        let mb_per_sec = (bytes as f64 / avg_micros) * 1_000_000.0 / 1_048_576.0;
        
        println!("{:<30} {:<10} {:<15.2} {:<15.2} {:<15.2}", 
            format!("{} ({})", text, description), 
            char_count,
            avg_micros,
            micros_per_char,
            mb_per_sec
        );
    }
    
    // Profile character-by-character parsing
    println!("\n\nDetailed Character Processing Analysis:");
    
    let text = "कृष्णः";
    println!("\nAnalyzing: {} ({} chars, {} bytes)", text, text.chars().count(), text.len());
    
    // Process each character separately to understand per-character overhead
    let chars: Vec<char> = text.chars().collect();
    for (i, ch) in chars.iter().enumerate() {
        let char_str = ch.to_string();
        
        // Warm up
        for _ in 0..100 {
            let _ = transliterator.transliterate(&char_str, "Devanagari", "IAST");
        }
        
        let iterations = 1000;
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = transliterator.transliterate(&char_str, "Devanagari", "IAST").unwrap();
        }
        let avg_time = start.elapsed() / iterations as u32;
        
        println!("  Char {}: '{}' - {:.2}µs", i, ch, avg_time.as_micros() as f64);
    }
    
    // Test string allocation impact
    println!("\n\nString Size Impact Analysis:");
    let base = "न";
    for size in [1, 10, 50, 100, 500, 1000] {
        let text = base.repeat(size);
        
        // Warm up
        for _ in 0..100 {
            let _ = transliterator.transliterate(&text, "Devanagari", "IAST");
        }
        
        let iterations = 100;
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = transliterator.transliterate(&text, "Devanagari", "IAST").unwrap();
        }
        let avg_time = start.elapsed() / iterations as u32;
        let avg_micros = avg_time.as_micros() as f64;
        let micros_per_char = avg_micros / size as f64;
        
        println!("  {} chars: {:.2}µs total, {:.2}µs/char", size, avg_micros, micros_per_char);
    }
}