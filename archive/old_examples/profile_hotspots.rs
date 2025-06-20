use shlesha::{Transliterator, TransliteratorBuilder, SchemaParser, Parser, Generator, Transformer};
use std::time::Instant;

fn main() {
    // Setup
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema_parsed = SchemaParser::parse_str(iast_schema).unwrap();
    
    let transliterator = TransliteratorBuilder::new()
        .with_schema(dev_schema.clone()).unwrap()
        .with_schema(iast_schema_parsed.clone()).unwrap()
        .build();
    
    // Setup individual components
    let mut parser = Parser::new();
    parser.load_schema(dev_schema.clone());
    parser.load_schema(iast_schema_parsed.clone());
    
    let mut generator = Generator::new();
    generator.load_schema(dev_schema.clone());
    generator.load_schema(iast_schema_parsed.clone());
    
    let transformer = Transformer::new();
    
    let text = "कृष्णार्जुनसंवादः";
    println!("Profiling text: {} ({} chars, {} bytes)\n", text, text.chars().count(), text.len());
    
    let iterations = 10000;
    
    // Profile each step in isolation
    println!("=== COMPONENT-LEVEL PROFILING ===");
    
    // 1. Parse step
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = parser.parse(text, "Devanagari").unwrap();
    }
    let parse_time = start.elapsed() / iterations as u32;
    println!("1. Parsing: {:.2}µs", parse_time.as_micros() as f64);
    
    // Get IR for next steps
    let ir = parser.parse(text, "Devanagari").unwrap();
    
    // 2. Transform step
    let start = Instant::now();
    for _ in 0..iterations {
        let ir_copy = parser.parse(text, "Devanagari").unwrap(); // Re-parse since IR doesn't clone
        let _ = transformer.transform(ir_copy, "alphabet").unwrap();
    }
    let transform_time = start.elapsed() / iterations as u32;
    println!("2. Transformation: {:.2}µs", transform_time.as_micros() as f64);
    
    // Get transformed IR
    let ir_for_transform = parser.parse(text, "Devanagari").unwrap();
    let transformed_ir = transformer.transform(ir_for_transform, "alphabet").unwrap();
    
    // 3. Generate step
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = generator.generate(&transformed_ir, "IAST").unwrap();
    }
    let gen_time = start.elapsed() / iterations as u32;
    println!("3. Generation: {:.2}µs", gen_time.as_micros() as f64);
    
    // 4. Full pipeline
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = transliterator.transliterate(text, "Devanagari", "IAST").unwrap();
    }
    let full_time = start.elapsed() / iterations as u32;
    println!("4. Full pipeline: {:.2}µs", full_time.as_micros() as f64);
    
    let component_sum = parse_time + transform_time + gen_time;
    let overhead = full_time.saturating_sub(component_sum);
    
    println!("\n=== OVERHEAD ANALYSIS ===");
    println!("Component sum: {:.2}µs", component_sum.as_micros() as f64);
    println!("Pipeline overhead: {:.2}µs ({:.1}%)", 
        overhead.as_micros() as f64,
        (overhead.as_micros() as f64 / full_time.as_micros() as f64) * 100.0
    );
    
    // Profile parsing in detail
    println!("\n=== PARSING DETAIL ===");
    
    // Schema lookup time (simulate with transliterator since parser.schemas is private)
    let start = Instant::now();
    for _ in 0..iterations {
        // This includes schema lookup overhead
        let _ = text.chars().count();
    }
    let schema_lookup_time = start.elapsed() / iterations as u32;
    println!("Basic operation overhead: {:.2}µs", schema_lookup_time.as_micros() as f64);
    
    // Character collection time
    let start = Instant::now();
    for _ in 0..iterations {
        let _: Vec<char> = text.chars().collect();
    }
    let char_collect_time = start.elapsed() / iterations as u32;
    println!("Char collection: {:.2}µs", char_collect_time.as_micros() as f64);
    
    // String allocation simulation (most expensive part)
    let chars: Vec<char> = text.chars().collect();
    let start = Instant::now();
    for _ in 0..iterations {
        for len in (1..=4).rev() {
            for i in 0..(chars.len().saturating_sub(len - 1)) {
                let _: String = chars[i..i + len].iter().collect();
            }
        }
    }
    let string_alloc_time = start.elapsed() / iterations as u32;
    println!("String allocations: {:.2}µs", string_alloc_time.as_micros() as f64);
    
    // Simulate HashMap lookups with a simple map
    use std::collections::HashMap;
    let mut test_map = HashMap::new();
    test_map.insert("क".to_string(), "ka");
    test_map.insert("कृ".to_string(), "kṛ");
    
    let start = Instant::now();
    for _ in 0..iterations {
        for len in (1..=4).rev() {
            for i in 0..(chars.len().saturating_sub(len - 1)) {
                let sequence: String = chars[i..i + len].iter().collect();
                let _ = test_map.get(&sequence);
            }
        }
    }
    let lookup_time = start.elapsed() / iterations as u32;
    println!("HashMap lookups: {:.2}µs", lookup_time.as_micros() as f64);
    
    println!("\n=== MEMORY ALLOCATION ANALYSIS ===");
    
    // Count total allocations per parse (estimated)
    let char_count = text.chars().count();
    let estimated_categories = 6; // consonants, vowels, modifiers, etc.
    let max_attempts = char_count * 4 * estimated_categories; // 4 lengths * categories
    println!("Estimated max string allocations per parse: {}", max_attempts);
    println!("Actual successful matches: {}", char_count);
    println!("Allocation efficiency: {:.1}%", (char_count as f64 / max_attempts as f64) * 100.0);
    
    println!("\n=== COMPARISON WITH SIMPLE APPROACH ===");
    
    // Simulate a simpler, non-extensible approach
    let mut simple_map = HashMap::new();
    simple_map.insert("क", "ka");
    simple_map.insert("ृ", "ṛ");
    simple_map.insert("ष", "ṣa");
    simple_map.insert("्", "");
    simple_map.insert("ण", "ṇa");
    simple_map.insert("ा", "ā");
    simple_map.insert("र", "ra");
    simple_map.insert("ज", "ja");
    simple_map.insert("ु", "u");
    simple_map.insert("न", "na");
    simple_map.insert("स", "sa");
    simple_map.insert("ं", "ṃ");
    simple_map.insert("व", "va");
    simple_map.insert("द", "da");
    simple_map.insert("ः", "ḥ");
    
    let start = Instant::now();
    for _ in 0..iterations {
        let mut result = String::new();
        for ch in text.chars() {
            if let Some(mapped) = simple_map.get(&ch.to_string().as_str()) {
                result.push_str(mapped);
            } else {
                result.push(ch);
            }
        }
    }
    let simple_time = start.elapsed() / iterations as u32;
    
    println!("Simple char-by-char mapping: {:.2}µs", simple_time.as_micros() as f64);
    println!("Speedup if simplified: {:.1}x", full_time.as_micros() as f64 / simple_time.as_micros() as f64);
}