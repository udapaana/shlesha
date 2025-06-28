use shlesha::modules::script_converter::{ScriptConverter, TeluguConverter};
use shlesha::modules::script_converter::optimized_telugu::OptimizedTeluguConverter;
use std::time::Instant;

fn main() {
    println!("String Allocation Optimization Benchmark");
    println!("========================================\n");
    
    // Create test data
    let telugu_text = "తెలుగు లిపిలో వ్రాయబడిన పాఠ్యం";
    let long_text = telugu_text.repeat(1000);
    
    // Create converters
    let original_converter = TeluguConverter::new();
    let optimized_converter = OptimizedTeluguConverter::new();
    
    println!("Test input: {} characters", long_text.len());
    println!("Sample text: {}\n", telugu_text);
    
    // Warm up both converters
    for _ in 0..100 {
        let _ = original_converter.to_hub("telugu", &long_text);
        let _ = optimized_converter.to_hub("telugu", &long_text);
    }
    
    let iterations = 1000u32;
    
    // Benchmark original implementation
    println!("Benchmarking Original Implementation...");
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = original_converter.to_hub("telugu", &long_text).unwrap();
    }
    let original_duration = start.elapsed();
    let original_avg_time = original_duration / iterations;
    let original_throughput = (long_text.len() as f64 * iterations as f64) / original_duration.as_secs_f64() / 1_000_000.0;
    
    // Benchmark optimized implementation
    println!("Benchmarking Optimized Implementation...");
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = optimized_converter.to_hub("telugu", &long_text).unwrap();
    }
    let optimized_duration = start.elapsed();
    let optimized_avg_time = optimized_duration / iterations;
    let optimized_throughput = (long_text.len() as f64 * iterations as f64) / optimized_duration.as_secs_f64() / 1_000_000.0;
    
    // Calculate improvement
    let speed_improvement = original_duration.as_secs_f64() / optimized_duration.as_secs_f64();
    let throughput_improvement = optimized_throughput / original_throughput;
    
    println!("\n=== RESULTS ===");
    println!("Original Implementation:");
    println!("  Average time per conversion: {:?}", original_avg_time);
    println!("  Throughput: {:.2} MB/s", original_throughput);
    println!("  Total time: {:?}", original_duration);
    
    println!("\nOptimized Implementation:");
    println!("  Average time per conversion: {:?}", optimized_avg_time);
    println!("  Throughput: {:.2} MB/s", optimized_throughput);
    println!("  Total time: {:?}", optimized_duration);
    
    println!("\n=== IMPROVEMENT ===");
    println!("  Speed improvement: {:.2}x faster", speed_improvement);
    println!("  Throughput improvement: {:.2}x higher", throughput_improvement);
    if speed_improvement > 1.0 {
        println!("  Performance gain: {:.1}%", (speed_improvement - 1.0) * 100.0);
    } else {
        println!("  Performance loss: {:.1}%", (1.0 - speed_improvement) * 100.0);
    }
    
    // Verify correctness
    println!("\n=== CORRECTNESS CHECK ===");
    let sample_input = "తెలుగు";
    let original_result = original_converter.to_hub("telugu", sample_input).unwrap();
    let optimized_result = optimized_converter.to_hub("telugu", sample_input).unwrap();
    
    match (&original_result, &optimized_result) {
        (shlesha::modules::hub::HubInput::Devanagari(orig), 
         shlesha::modules::hub::HubInput::Devanagari(opt)) => {
            if orig == opt {
                println!("  ✅ Outputs match: '{}'", orig);
            } else {
                println!("  ❌ Outputs differ:");
                println!("    Original: '{}'", orig);
                println!("    Optimized: '{}'", opt);
            }
        }
        _ => println!("  ❌ Different output types"),
    }
    
    println!("\n=== OPTIMIZATION SUMMARY ===");
    println!("  - Eliminated to_string() allocations in character lookup");
    println!("  - Used char keys instead of String keys in HashMaps");
    println!("  - Reduced intermediate string allocations");
    println!("  - Optimized string capacity pre-allocation");
}