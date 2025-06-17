use shlesha::{Transliterator, TransliteratorBuilder, SchemaParser};
use shlesha::transliterator_optimized::TransliteratorOptimized;
use shlesha::parser_optimized::ParserOptimized;
use shlesha::generator_optimized::GeneratorOptimized;
use std::time::Instant;

fn main() {
    // Load schemas
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema_parsed = SchemaParser::parse_str(iast_schema).unwrap();
    
    // Setup original transliterator
    let original = TransliteratorBuilder::new()
        .with_schema(dev_schema.clone()).unwrap()
        .with_schema(iast_schema_parsed.clone()).unwrap()
        .build();
    
    // Setup optimized transliterator
    let mut optimized = TransliteratorOptimized::new();
    optimized.load_schema(dev_schema.clone()).unwrap();
    optimized.load_schema(iast_schema_parsed.clone()).unwrap();
    
    // Test cases
    let test_cases = vec![
        ("नमस्ते", "Simple word"),
        ("संस्कृतम्", "With anusvara"),
        ("कृष्णार्जुनसंवादः", "Complex word"),
        ("धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय", "Full verse"),
    ];
    
    println!("Performance Comparison: Original vs Optimized\n");
    println!("{:<40} {:<15} {:<15} {:<15}", "Text", "Original (µs)", "Optimized (µs)", "Speedup");
    println!("{}", "-".repeat(85));
    
    for (text, description) in test_cases {
        // Warm up both
        for _ in 0..100 {
            let _ = original.transliterate(text, "Devanagari", "IAST");
            let _ = optimized.transliterate(text, "Devanagari", "IAST");
        }
        
        // Measure original
        let iterations = 1000;
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = original.transliterate(text, "Devanagari", "IAST").unwrap();
        }
        let original_time = start.elapsed() / iterations as u32;
        
        // Measure optimized
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = optimized.transliterate(text, "Devanagari", "IAST").unwrap();
        }
        let optimized_time = start.elapsed() / iterations as u32;
        
        let speedup = original_time.as_micros() as f64 / optimized_time.as_micros() as f64;
        
        println!("{:<40} {:<15.2} {:<15.2} {:<15.2}x", 
            format!("{} ({})", description, text.chars().count()),
            original_time.as_micros() as f64,
            optimized_time.as_micros() as f64,
            speedup
        );
    }
    
    // Component-level profiling
    println!("\n\nComponent-level Analysis:");
    
    // Setup components
    let mut parser_opt = ParserOptimized::new();
    parser_opt.load_schema(dev_schema.clone());
    
    let mut generator_opt = GeneratorOptimized::new();
    generator_opt.load_schema(iast_schema_parsed.clone());
    
    let text = "कृष्णार्जुनसंवादः";
    let iterations = 10000;
    
    // Measure parsing
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = parser_opt.parse(text, "Devanagari").unwrap();
    }
    let parse_time = start.elapsed() / iterations as u32;
    println!("  Optimized Parser: {:.2}µs", parse_time.as_micros() as f64);
    
    // Parse once for generation test
    let ir = parser_opt.parse(text, "Devanagari").unwrap();
    
    // Measure generation
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = generator_opt.generate(&ir, "IAST").unwrap();
    }
    let gen_time = start.elapsed() / iterations as u32;
    println!("  Optimized Generator: {:.2}µs", gen_time.as_micros() as f64);
    
    println!("  Total component time: {:.2}µs", (parse_time + gen_time).as_micros() as f64);
}