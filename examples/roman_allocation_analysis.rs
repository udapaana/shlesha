use std::time::Instant;
use shlesha::modules::script_converter::{ScriptConverter, Slp1Converter};

fn main() {
    println!("Roman Script Allocation Analysis");
    println!("================================\n");
    
    // Create test data with complex sequences that trigger allocations
    let slp1_text = "dharmakSetrekukSAstramulamasUktamukhAdharmakSetrajYAnamukhAnamastubhyam";
    let long_text = slp1_text.repeat(1000);  // 70,000 chars
    
    println!("Test data: {} characters, {} bytes", 
             long_text.chars().count(), long_text.len());
    
    // Create converter
    let converter = Slp1Converter::new();
    
    // Warm up
    for _ in 0..100 {
        let _ = converter.to_hub("slp1", &long_text);
    }
    
    let iterations = 1000u32;
    
    // Measure performance
    println!("Testing SLP1 converter...");
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = converter.to_hub("slp1", &long_text).unwrap();
    }
    let duration = start.elapsed();
    let avg_time = duration / iterations;
    let throughput = (long_text.len() as f64 * iterations as f64) / duration.as_secs_f64() / 1_000_000.0;
    
    println!("\n=== ROMAN SCRIPT PERFORMANCE ===");
    println!("  Average time: {:?}", avg_time);
    println!("  Throughput: {:.2} MB/s", throughput);
    
    println!("\nPotential optimizations in RomanScriptProcessor:");
    println!("  1. Vec<char> allocation in line 35 of processors.rs");
    println!("  2. String allocation in line 37 of processors.rs");
    println!("  3. String reallocation due to capacity growth");
    println!("  4. Character iteration inefficiency");
}