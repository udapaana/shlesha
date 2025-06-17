use shlesha::{Transliterator, TransliteratorBuilder, SchemaParser, Parser, Generator, Transformer};
use std::time::Instant;

fn main() {
    // Load schemas
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema_parsed = SchemaParser::parse_str(iast_schema).unwrap();
    
    // Setup transliterator
    let transliterator = TransliteratorBuilder::new()
        .with_schema(dev_schema.clone()).unwrap()
        .with_schema(iast_schema_parsed.clone()).unwrap()
        .build();
    
    // Setup individual components for profiling
    let mut parser = Parser::new();
    parser.load_schema(dev_schema.clone());
    parser.load_schema(iast_schema_parsed.clone());
    
    let mut generator = Generator::new();
    generator.load_schema(dev_schema.clone());
    generator.load_schema(iast_schema_parsed.clone());
    
    let transformer = Transformer::new();
    
    // Test text
    let text = "कृष्णार्जुनसंवादः";
    println!("Profiling text: {} ({} chars)\n", text, text.chars().count());
    
    // Profile full transliteration
    let iterations = 1000;
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = transliterator.transliterate(text, "Devanagari", "IAST").unwrap();
    }
    let full_time = start.elapsed() / iterations as u32;
    println!("Full transliteration: {:.2}µs", full_time.as_micros() as f64);
    
    // Profile parsing
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = parser.parse(text, "Devanagari").unwrap();
    }
    let parse_time = start.elapsed() / iterations as u32;
    println!("  - Parsing: {:.2}µs ({:.1}%)", 
        parse_time.as_micros() as f64,
        (parse_time.as_micros() as f64 / full_time.as_micros() as f64) * 100.0
    );
    
    // Get IR for other steps
    let ir = parser.parse(text, "Devanagari").unwrap();
    
    // Profile transformation
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = transformer.transform(ir.clone(), "alphabet").unwrap();
    }
    let transform_time = start.elapsed() / iterations as u32;
    println!("  - Transformation: {:.2}µs ({:.1}%)", 
        transform_time.as_micros() as f64,
        (transform_time.as_micros() as f64 / full_time.as_micros() as f64) * 100.0
    );
    
    // Get transformed IR
    let transformed_ir = transformer.transform(ir.clone(), "alphabet").unwrap();
    
    // Profile generation
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = generator.generate(&transformed_ir, "IAST").unwrap();
    }
    let gen_time = start.elapsed() / iterations as u32;
    println!("  - Generation: {:.2}µs ({:.1}%)", 
        gen_time.as_micros() as f64,
        (gen_time.as_micros() as f64 / full_time.as_micros() as f64) * 100.0
    );
    
    // Other overhead
    let component_total = parse_time + transform_time + gen_time;
    let overhead = full_time.saturating_sub(component_total);
    println!("  - Other overhead: {:.2}µs ({:.1}%)", 
        overhead.as_micros() as f64,
        (overhead.as_micros() as f64 / full_time.as_micros() as f64) * 100.0
    );
    
    // Memory allocation analysis
    println!("\nMemory Allocation Analysis:");
    
    // Parse and count allocations
    let ir = parser.parse(text, "Devanagari").unwrap();
    match &ir {
        shlesha::IR::Abugida(abugida) => {
            println!("  - Elements in IR: {}", abugida.elements.len());
            let total_string_bytes: usize = abugida.elements.iter()
                .map(|e| e.grapheme.len() + e.canonical.len())
                .sum();
            println!("  - Total string bytes in IR: {}", total_string_bytes);
        }
        _ => {}
    }
    
    // Test different text sizes
    println!("\nScaling Analysis:");
    for size in [1, 10, 50, 100, 500] {
        let test_text = "न".repeat(size);
        
        let start = Instant::now();
        for _ in 0..100 {
            let _ = transliterator.transliterate(&test_text, "Devanagari", "IAST").unwrap();
        }
        let time = start.elapsed() / 100;
        let per_char = time.as_micros() as f64 / size as f64;
        
        println!("  {} chars: {:.2}µs total, {:.2}µs/char", size, time.as_micros() as f64, per_char);
    }
}