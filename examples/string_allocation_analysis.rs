use shlesha::modules::script_converter::{ScriptConverter, TeluguConverter};
use std::time::Instant;

fn main() {
    println!("String Allocation Analysis");
    println!("=========================\n");
    
    // Create test data
    let telugu_text = "తెలుగు లిపిలో వ్రాయబడిన పాఠ్యం";
    let long_text = telugu_text.repeat(1000);
    
    // Create converter
    let converter = TeluguConverter::new();
    
    // Warm up
    for _ in 0..100 {
        let _ = converter.to_hub("telugu", &long_text);
    }
    
    // Measure current implementation
    let iterations = 1000u32;
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _ = converter.to_hub("telugu", &long_text).unwrap();
    }
    
    let duration = start.elapsed();
    let avg_time = duration / iterations;
    let throughput = (long_text.len() as f64 * iterations as f64) / duration.as_secs_f64() / 1_000_000.0;
    
    println!("Current Implementation:");
    println!("  Average time per conversion: {:?}", avg_time);
    println!("  Throughput: {:.2} MB/s", throughput);
    println!("  Input size: {} bytes", long_text.len());
    
    // Analyze allocations using a simple profiling approach
    println!("\nPotential allocation hotspots:");
    println!("  - to_string() calls in character lookup");
    println!("  - Intermediate Vec<char> allocation");
    println!("  - String building in result");
}