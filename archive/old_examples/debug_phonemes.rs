use shlesha::{PhonemeTransliteratorBuilder, SchemaParser};

fn main() {
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema = SchemaParser::parse_str(iast_schema).unwrap();
    
    let mut phoneme_transliterator = PhonemeTransliteratorBuilder::new()
        .with_schema(dev_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build();
    
    // Test texts from our benchmark
    let texts = vec![
        "नमस्ते",
        "अहं संस्कृतं वदामि",
        "तत्र शूरा महेष्वासा भीमार्जुनसमा युधि",
    ];
    
    for text in &texts {
        println!("\n=== Analyzing: {} ===", text);
        
        // Print each character
        for (i, ch) in text.chars().enumerate() {
            println!("  {}: '{}' (U+{:04X})", i, ch, ch as u32);
        }
        
        // Reset stats
        phoneme_transliterator.reset_stats();
        
        // Run transliteration
        let _result = phoneme_transliterator.transliterate(text, "Devanagari", "IAST").unwrap();
        
        // Print stats
        let stats = phoneme_transliterator.get_parse_stats();
        println!("Stats:");
        println!("  Total chars: {}", stats.total_chars_processed);
        println!("  Enum phonemes: {} ({:.1}%)", stats.enum_phonemes_used, stats.allocation_efficiency());
        println!("  Extension phonemes: {}", stats.extension_phonemes_used);
        println!("  String allocations: {}", stats.string_allocations);
    }
}