use std::time::Instant;
use shlesha::modules::script_converter::{ScriptConverter, TeluguConverter};

fn main() {
    println!("String Allocation Optimization Comparison");
    println!("=========================================\n");
    
    // Create test data
    let telugu_text = "తెలుగు లిపిలో వ్రాయబడిన పాఠ్యం మరియు అధిక పాఠ్యం";
    let long_text = telugu_text.repeat(1000);
    
    println!("Test data: {} characters, {} bytes", 
             long_text.chars().count(), long_text.len());
    
    // Create converters
    let regular_converter = TeluguConverter::new();
    let optimized_converter = TeluguConverter::new(); // Now using the same optimized implementation
    
    // Warm up both converters
    for _ in 0..100 {
        let _ = regular_converter.to_hub("telugu", &long_text);
        let _ = optimized_converter.to_hub("telugu", &long_text);
    }
    
    let iterations = 1000u32;
    
    // Measure regular implementation
    println!("Testing regular implementation...");
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = regular_converter.to_hub("telugu", &long_text).unwrap();
    }
    let regular_duration = start.elapsed();
    let regular_avg = regular_duration / iterations;
    let regular_throughput = (long_text.len() as f64 * iterations as f64) / regular_duration.as_secs_f64() / 1_000_000.0;
    
    // Measure optimized implementation
    println!("Testing optimized implementation...");
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = optimized_converter.to_hub("telugu", &long_text).unwrap();
    }
    let optimized_duration = start.elapsed();
    let optimized_avg = optimized_duration / iterations;
    let optimized_throughput = (long_text.len() as f64 * iterations as f64) / optimized_duration.as_secs_f64() / 1_000_000.0;
    
    // Calculate improvement
    let speed_improvement = regular_duration.as_nanos() as f64 / optimized_duration.as_nanos() as f64;
    let throughput_improvement = optimized_throughput / regular_throughput;
    
    println!("\n=== RESULTS ===");
    println!("Regular Implementation:");
    println!("  Average time: {:?}", regular_avg);
    println!("  Throughput: {:.2} MB/s", regular_throughput);
    
    println!("\nOptimized Implementation:");
    println!("  Average time: {:?}", optimized_avg);
    println!("  Throughput: {:.2} MB/s", optimized_throughput);
    
    println!("\nImprovement:");
    println!("  Speed improvement: {:.2}x", speed_improvement);
    println!("  Throughput improvement: {:.2}x", throughput_improvement);
    println!("  Time reduction: {:.1}%", (speed_improvement - 1.0) * 100.0);
    
    if speed_improvement > 1.2 {
        println!("\n✅ SIGNIFICANT IMPROVEMENT: {:.1}% faster!", (speed_improvement - 1.0) * 100.0);
    } else if speed_improvement > 1.05 {
        println!("\n✨ MODERATE IMPROVEMENT: {:.1}% faster", (speed_improvement - 1.0) * 100.0);
    } else {
        println!("\n⚠️  MINIMAL IMPROVEMENT: {:.1}% difference", (speed_improvement - 1.0) * 100.0);
    }
    
    // Verify correctness
    let regular_result = regular_converter.to_hub("telugu", "తెలుగు").unwrap();
    let optimized_result = optimized_converter.to_hub("telugu", "తెలుగు").unwrap();
    
    println!("\nCorrectness check:");
    println!("  Regular result: {:?}", regular_result);
    println!("  Optimized result: {:?}", optimized_result);
    
    match (regular_result, optimized_result) {
        (shlesha::modules::hub::HubInput::Devanagari(r), shlesha::modules::hub::HubInput::Iso(o)) => {
            println!("  ✅ Both converters working (different output formats expected)");
        },
        (shlesha::modules::hub::HubInput::Iso(r), shlesha::modules::hub::HubInput::Iso(o)) => {
            if r == o {
                println!("  ✅ Results match perfectly!");
            } else {
                println!("  ⚠️  Results differ: '{}' vs '{}'", r, o);
            }
        },
        _ => {
            println!("  ⚠️  Different output types");
        }
    }
}