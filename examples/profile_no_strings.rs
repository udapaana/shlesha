use shlesha::{TransliteratorBuilder, SchemaParser};
use std::time::Instant;
use std::collections::HashMap;

// Simulate optimized parsing without string allocations
fn optimized_parse_simulation(text: &str) -> Vec<&str> {
    // Pre-built lookup table using &str keys (no allocations)
    let mut lookup = HashMap::new();
    lookup.insert(['क'].as_slice(), "ka");
    lookup.insert(['ृ'].as_slice(), "ṛ");
    lookup.insert(['ष'].as_slice(), "ṣa");
    lookup.insert(['्'].as_slice(), "");
    lookup.insert(['ण'].as_slice(), "ṇa");
    lookup.insert(['ा'].as_slice(), "ā");
    lookup.insert(['र'].as_slice(), "ra");
    lookup.insert(['ज'].as_slice(), "ja");
    lookup.insert(['ु'].as_slice(), "u");
    lookup.insert(['न'].as_slice(), "na");
    lookup.insert(['स'].as_slice(), "sa");
    lookup.insert(['ं'].as_slice(), "ṃ");
    lookup.insert(['व'].as_slice(), "va");
    lookup.insert(['द'].as_slice(), "da");
    lookup.insert(['ः'].as_slice(), "ḥ");
    
    // Multi-character sequences
    lookup.insert(['क', 'ृ'].as_slice(), "kṛ");
    lookup.insert(['स', 'ं'].as_slice(), "saṃ");
    
    let chars: Vec<char> = text.chars().collect();
    let mut result = Vec::new();
    let mut i = 0;
    
    while i < chars.len() {
        let mut matched = false;
        
        // Try longer matches first (no string allocation!)
        for len in (1..=4).rev() {
            if i + len > chars.len() {
                continue;
            }
            
            let slice = &chars[i..i + len];
            if let Some(&canonical) = lookup.get(slice) {
                result.push(canonical);
                i += len;
                matched = true;
                break;
            }
        }
        
        if !matched {
            result.push("[?]"); // Unknown character
            i += 1;
        }
    }
    
    result
}

fn main() {
    // Setup standard Shlesha
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema_parsed = SchemaParser::parse_str(iast_schema).unwrap();
    
    let transliterator = TransliteratorBuilder::new()
        .with_schema(dev_schema).unwrap()
        .with_schema(iast_schema_parsed).unwrap()
        .build();
    
    let text = "कृष्णार्जुनसंवादः";
    let iterations = 10000;
    
    println!("Profiling: {} ({} chars)\n", text, text.chars().count());
    
    // Benchmark current Shlesha
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = transliterator.transliterate(text, "Devanagari", "IAST").unwrap();
    }
    let shlesha_time = start.elapsed() / iterations as u32;
    
    // Benchmark optimized approach (no string allocations)
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = optimized_parse_simulation(text);
    }
    let optimized_time = start.elapsed() / iterations as u32;
    
    println!("Current Shlesha: {:.2}µs", shlesha_time.as_micros() as f64);
    println!("No-allocation simulation: {:.2}µs", optimized_time.as_micros() as f64);
    println!("Potential speedup: {:.1}x", shlesha_time.as_micros() as f64 / optimized_time.as_micros() as f64);
    
    // Verify correctness
    let shlesha_result = transliterator.transliterate(text, "Devanagari", "IAST").unwrap();
    let optimized_result: String = optimized_parse_simulation(text).join("");
    
    println!("\nResults:");
    println!("Shlesha:   {}", shlesha_result);
    println!("Optimized: {}", optimized_result);
    println!("Match: {}", shlesha_result == optimized_result);
    
    println!("\n=== KEY INSIGHT ===");
    println!("The problem is fundamental architecture:");
    println!("1. YAML schema forces String keys in HashMap<String, ElementMapping>");
    println!("2. Parser must allocate String to look up in HashMap");
    println!("3. We immediately throw away String after converting to IR");
    println!();
    println!("SOLUTION: Pre-build character-slice lookup tables during schema load");
    println!("- HashMap<&[char], ElementId> instead of HashMap<String, ElementMapping>");
    println!("- No runtime string allocations");
    println!("- Direct char slice -> IR conversion");
    println!();
    println!("This would require:");
    println!("1. Schema loading phase builds optimized lookup structures");
    println!("2. Parser uses char slices as keys (zero-copy)");
    println!("3. Maintain current extensibility but with pre-compiled efficiency");
}