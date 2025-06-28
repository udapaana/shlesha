use std::time::Instant;
use shlesha::modules::script_converter::{ScriptConverter, SLP1Converter, OptimizedSLP1Converter};

fn main() {
    println!("Roman Script Processor Optimization Benchmark");
    println!("============================================");
    println!("");
    
    // Create test data with complex sequences that trigger the allocation hotspots
    let slp1_text = "dharmakSetrekukSAstramulamasUktamukhAdharmakSetrajYAnamukhAnamastubhyam";
    let long_text = slp1_text.repeat(100);  // 7,000 chars
    
    println!("Test data: {} characters", long_text.len());
    println!("Sample text: {}", slp1_text);
    println!("");
    
    // Create converters
    let original_converter = SLP1Converter::new();
    let optimized_converter = OptimizedSLP1Converter::new();
    
    // Warm up both converters
    println!("Warming up...");
    for _ in 0..100 {
        let _ = original_converter.to_hub("slp1", &long_text);
        let _ = optimized_converter.to_hub("slp1", &long_text);
    }
    
    let iterations = 100u32;
    
    // Benchmark original implementation
    println!("Benchmarking Original RomanScriptProcessor...");
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = original_converter.to_hub("slp1", &long_text).unwrap();
    }
    let original_duration = start.elapsed();
    let original_avg_time = original_duration / iterations;
    let original_throughput = (long_text.len() as f64 * iterations as f64) / original_duration.as_secs_f64() / 1_000_000.0;
    
    // Benchmark optimized implementation
    println!("Benchmarking Optimized RomanScriptProcessor...");
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = optimized_converter.to_hub("slp1", &long_text).unwrap();
    }
    let optimized_duration = start.elapsed();
    let optimized_avg_time = optimized_duration / iterations;
    let optimized_throughput = (long_text.len() as f64 * iterations as f64) / optimized_duration.as_secs_f64() / 1_000_000.0;
    
    // Calculate improvement
    let speed_improvement = original_duration.as_secs_f64() / optimized_duration.as_secs_f64();
    let throughput_improvement = optimized_throughput / original_throughput;
    
    println!("");
    println!("=== RESULTS ===");
    println!("Original RomanScriptProcessor:");
    println!("  Average time per conversion: {:?}", original_avg_time);
    println!("  Throughput: {:.2} MB per second", original_throughput);
    println!("  Total time: {:?}", original_duration);
    
    println!("");
    println!("Optimized RomanScriptProcessor:");
    println!("  Average time per conversion: {:?}", optimized_avg_time);
    println!("  Throughput: {:.2} MB per second", optimized_throughput);
    println!("  Total time: {:?}", optimized_duration);
    
    println!("");
    println!("=== IMPROVEMENT ===");
    println!("  Speed improvement: {:.2}x", speed_improvement);
    println!("  Throughput improvement: {:.2}x", throughput_improvement);
    if speed_improvement > 1.0 {
        println!("  Performance gain: {:.1}%", (speed_improvement - 1.0) * 100.0);
    } else {
        println!("  Performance loss: {:.1}%", (1.0 - speed_improvement) * 100.0);
    }
    
    // Verify correctness
    println!("");
    println!("=== CORRECTNESS CHECK ===");
    let test_cases = vec![
        "dharma",
        "kSetra", 
        "jYAna",
        "dharmakSetrekukSAstramulamasUktamukhA",
        "A", // Long vowel
        "f", // Vocalic r
        "KG", // Aspirated consonants
        "M", // Anusvara
        "H", // Visarga
    ];
    
    let mut all_correct = true;
    for test_case in test_cases {
        let original_result = original_converter.to_hub("slp1", test_case).unwrap();
        let optimized_result = optimized_converter.to_hub("slp1", test_case).unwrap();
        
        let original_text = match &original_result {
            shlesha::modules::hub::HubInput::Iso(text) => text,
            shlesha::modules::hub::HubInput::Devanagari(text) => text,
        };
        
        let optimized_text = match &optimized_result {
            shlesha::modules::hub::HubInput::Iso(text) => text,
            shlesha::modules::hub::HubInput::Devanagari(text) => text,
        };
        
        if original_text != optimized_text {
            println!("  MISMATCH for '{}': '{}' vs '{}'", test_case, original_text, optimized_text);
            all_correct = false;
        }
    }
    
    if all_correct {
        println!("  All test cases match between original and optimized versions");
    }
    
    println!("");
    println!("=== OPTIMIZATION SUMMARY ===");
    println!("  Eliminated allocation hotspots in RomanScriptProcessor:");
    println!("  - Removed Vec<char> allocation in line 35 of processors");
    println!("  - Removed String allocation in line 37 of processors");
    println!("  - Eliminated String::from_iter for sequence matching");
    println!("  - Direct string slicing instead of character collection");
    println!("  - Added ASCII-only fast path for pure ASCII text");
    println!("  - Auto-detection between Unicode and ASCII processing");
    
    if speed_improvement > 1.2 {
        println!("");
        println!("SIGNIFICANT IMPROVEMENT: {:.1}% faster!", (speed_improvement - 1.0) * 100.0);
        println!("   This optimization should be kept and applied to other converters.");
    } else if speed_improvement > 1.0 {
        println!("");
        println!("MINOR IMPROVEMENT: {:.1}% faster", (speed_improvement - 1.0) * 100.0);
        println!("   This optimization provides modest gains.");
    } else {
        println!("");
        println!("REGRESSION: {:.1}% slower", (1.0 - speed_improvement) * 100.0);
        println!("   This optimization should be reverted.");
    }
}