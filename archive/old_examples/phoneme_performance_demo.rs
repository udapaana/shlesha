use std::fs;
use shlesha::{
    phoneme_parser::PhonemeParser,
    schema_parser::{SchemaParser, ScriptType},
    Schema,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Zero-Allocation Phoneme Parser Demo");
    
    // Create phoneme parser
    let mut phoneme_parser = PhonemeParser::new();
    
    // Load minimal schemas for testing
    let devanagari_schema = Schema {
        name: "Devanagari".to_string(),
        script_type: ScriptType::Abugida,
        element_types: std::collections::HashMap::new(),
        mappings: std::collections::HashMap::new(),
        extensions: std::collections::HashMap::new(),
        metadata: None,
    };
    
    let iast_schema = Schema {
        name: "IAST".to_string(),
        script_type: ScriptType::Alphabet,
        element_types: std::collections::HashMap::new(),
        mappings: std::collections::HashMap::new(),
        extensions: std::collections::HashMap::new(),
        metadata: None,
    };
    
    phoneme_parser.load_schema(devanagari_schema);
    phoneme_parser.load_schema(iast_schema);
    
    // Test Sanskrit words for performance
    let test_words = vec![
        ("क", "Devanagari"),           // single consonant
        ("कर्म", "Devanagari"),        // karma  
        ("धर्म", "Devanagari"),        // dharma
        ("संस्कृत", "Devanagari"),     // saṃskṛta
        ("प्रकृति", "Devanagari"),      // prakṛti
        ("उपनिषद्", "Devanagari"),     // upaniṣad
        ("ka", "IAST"),               // single syllable
        ("karma", "IAST"),            // karma
        ("dharma", "IAST"),           // dharma
        ("saṃskṛta", "IAST"),         // saṃskṛta
        ("prakṛti", "IAST"),          // prakṛti
        ("upaniṣad", "IAST"),         // upaniṣad
    ];
    
    println!("\n🔤 Testing zero-allocation phoneme parsing:");
    
    for (text, script) in &test_words {
        phoneme_parser.reset_stats(); // Reset for individual measurement
        
        let start = std::time::Instant::now();
        match phoneme_parser.parse_to_ir(text, script) {
            Ok(ir) => {
                let duration = start.elapsed();
                let stats = phoneme_parser.get_stats();
                
                println!("   {} ({}) → {:?}", text, script, duration);
                println!("     • Chars: {}, Enum phonemes: {}, Extensions: {}", 
                    stats.total_chars_processed,
                    stats.enum_phonemes_used,
                    stats.extension_phonemes_used
                );
                println!("     • Efficiency: {:.1}%, String allocations: {}", 
                    stats.allocation_efficiency(),
                    stats.string_allocations
                );
                
                // Show IR type
                match ir {
                    shlesha::ir::IR::Abugida(abugida) => {
                        println!("     • IR: Abugida ({} elements)", abugida.elements.len());
                    },
                    shlesha::ir::IR::Alphabet(alphabet) => {
                        println!("     • IR: Alphabet ({} elements)", alphabet.elements.len());
                    },
                }
            },
            Err(e) => println!("   {} ({}) → Error: {}", text, script, e),
        }
        println!();
    }
    
    // Cumulative performance test
    println!("📊 Cumulative Performance Test:");
    phoneme_parser.reset_stats();
    
    let large_text = "कर्म धर्म योग गुरु शांति प्रकृति संस्कृत वेद उपनिषद्";
    let iterations = 1000;
    
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = phoneme_parser.parse_to_ir(large_text, "Devanagari");
    }
    let total_duration = start.elapsed();
    
    let stats = phoneme_parser.get_stats();
    
    println!("   Text: \"{}\"", large_text);
    println!("   Iterations: {}", iterations);
    println!("   Total time: {:?}", total_duration);
    println!("   Average per iteration: {:?}", total_duration / iterations);
    println!("   Average per character: {:.2}ns", stats.avg_parse_time_per_char_ns());
    println!("   Total chars processed: {}", stats.total_chars_processed);
    println!("   Enum phonemes: {} ({:.1}%)", 
        stats.enum_phonemes_used,
        stats.allocation_efficiency()
    );
    println!("   Extension phonemes: {}", stats.extension_phonemes_used);
    println!("   String allocations: {}", stats.string_allocations);
    
    // Memory efficiency analysis
    println!("\n🧠 Memory Efficiency Analysis:");
    let enum_size = stats.enum_phonemes_used * 2; // 2 bytes per enum phoneme
    let extension_size = stats.extension_phonemes_used * 80; // ~80 bytes per extension
    let total_memory = enum_size + extension_size;
    
    println!("   Enum phonemes memory: {} bytes ({} × 2)", enum_size, stats.enum_phonemes_used);
    println!("   Extension phonemes memory: {} bytes ({} × 80)", extension_size, stats.extension_phonemes_used);
    println!("   Total memory: {} bytes", total_memory);
    println!("   Avg memory per char: {:.2} bytes", total_memory as f64 / stats.total_chars_processed as f64);
    
    // Compare with theoretical string-based approach
    let string_based_memory = stats.total_chars_processed * 24; // Estimate: 24 bytes per string Element
    let memory_savings = (string_based_memory as f64 - total_memory as f64) / string_based_memory as f64 * 100.0;
    
    println!("\n💡 vs String-based approach:");
    println!("   String-based estimate: {} bytes ({} × 24)", string_based_memory, stats.total_chars_processed);
    println!("   Phoneme-based actual: {} bytes", total_memory);
    println!("   Memory saved: {:.1}%", memory_savings);
    
    // Performance projections
    let chars_per_second = stats.total_chars_processed as f64 / (total_duration.as_secs_f64());
    println!("\n⚡ Performance Projections:");
    println!("   Characters per second: {:.0}", chars_per_second);
    println!("   Words per second (~5 chars): {:.0}", chars_per_second / 5.0);
    println!("   Lines per second (~50 chars): {:.0}", chars_per_second / 50.0);
    println!("   Pages per second (~2500 chars): {:.0}", chars_per_second / 2500.0);
    
    println!("\n🎯 Dual IR System Benefits:");
    println!("   ✅ Abugida IR for Indic scripts (inherent vowel handling)");
    println!("   ✅ Alphabet IR for Roman scripts (character-by-character)");
    println!("   ✅ Zero-allocation parsing for known phonemes ({}% efficiency)", stats.allocation_efficiency());
    println!("   ✅ Extensible fallback for unknown characters");
    println!("   ✅ 2-byte enum phonemes vs ~24-byte string elements");
    
    println!("\n🚀 Next Steps:");
    println!("   • Implement full IndicPhoneme enum (~600 variants)");
    println!("   • Add comprehensive script lookup tables");
    println!("   • Optimize conjunct recognition");
    println!("   • Add phoneme-to-phoneme transformations");
    println!("   • Build zero-allocation generator");
    
    Ok(())
}