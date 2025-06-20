use shlesha::{Transliterator, TransliteratorBuilder, SchemaParser};
use std::time::Instant;

fn main() {
    // Setup transliterator
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let start = Instant::now();
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema_parsed = SchemaParser::parse_str(iast_schema).unwrap();
    let parse_time = start.elapsed();
    println!("Schema parsing time: {:?}", parse_time);
    
    let start = Instant::now();
    let transliterator = TransliteratorBuilder::new()
        .with_schema(dev_schema).unwrap()
        .with_schema(iast_schema_parsed).unwrap()
        .build();
    let build_time = start.elapsed();
    println!("Transliterator build time: {:?}", build_time);
    
    // Test simple word
    let text = "नमस्ते";
    println!("\nProfiling: {}", text);
    
    // Warm up
    for _ in 0..100 {
        let _ = transliterator.transliterate(text, "Devanagari", "IAST");
    }
    
    // Profile individual components
    let iterations = 1000;
    
    // Full transliteration
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = transliterator.transliterate(text, "Devanagari", "IAST").unwrap();
    }
    let total_time = start.elapsed();
    println!("Average transliteration time: {:?}", total_time / iterations as u32);
    
    // Test longer text
    let long_text = "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय";
    println!("\nProfiling longer text ({} chars)", long_text.chars().count());
    
    // Warm up
    for _ in 0..100 {
        let _ = transliterator.transliterate(long_text, "Devanagari", "IAST");
    }
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = transliterator.transliterate(long_text, "Devanagari", "IAST").unwrap();
    }
    let total_time = start.elapsed();
    println!("Average transliteration time: {:?}", total_time / iterations as u32);
}