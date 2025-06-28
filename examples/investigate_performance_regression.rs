use shlesha::Shlesha;
use std::time::Instant;

fn main() {
    let shlesha = Shlesha::new();
    
    println!("🔍 Investigating Performance Regression");
    println!("======================================");
    
    // Use the same test data as earlier successful tests
    let simple_text = "dharma yoga bhārata saṃskṛta veda upaniṣad gītā";
    let char_count = simple_text.len();
    
    println!("Test text: '{}' ({} chars)", simple_text, char_count);
    println!();
    
    // Test the exact conversions that showed regression
    let test_cases = vec![
        ("iast", "devanagari", "Earlier: ~1.07M chars/sec, Current: 231K chars/sec"),
        ("devanagari", "iast", "Earlier: ~3.28M chars/sec, Current: 513K chars/sec"),
        ("itrans", "devanagari", "Earlier: ~960K chars/sec, Current: 212K chars/sec"),
        ("iso15919", "devanagari", "Direct mapping test"),
        ("devanagari", "iso15919", "Direct mapping test"),
    ];
    
    for (from, to, note) in test_cases {
        println!("Testing {} → {} ({})", from, to, note);
        
        // Test with different iteration counts to match earlier methodology
        for &iterations in &[100, 1000, 10000] {
            let throughput = measure_throughput(&shlesha, from, to, simple_text, iterations);
            println!("  {} iterations: {:.0} chars/sec", iterations, throughput);
        }
        
        // Test latency like our earlier tests
        let avg_latency = measure_latency(&shlesha, from, to, simple_text, 1000);
        println!("  Average latency: {:.0}ns", avg_latency);
        
        println!();
    }
    
    // Test if direct mappings are actually being used
    println!("🧪 Direct Mapping Verification:");
    test_direct_mapping_usage(&shlesha);
    
    println!("\n🔧 Performance Analysis:");
    println!("- If direct mappings show similar performance to hub-based, integration may be broken");
    println!("- If latency is much higher than earlier tests, there may be build/optimization issues");
    println!("- Compare with earlier test methodology to identify regression cause");
}

fn measure_throughput(shlesha: &Shlesha, from: &str, to: &str, text: &str, iterations: usize) -> f64 {
    // Warmup
    for _ in 0..10 {
        let _ = shlesha.transliterate(text, from, to);
    }
    
    let start = Instant::now();
    for _ in 0..iterations {
        if let Err(e) = shlesha.transliterate(text, from, to) {
            eprintln!("Error: {}", e);
            return 0.0;
        }
    }
    let duration = start.elapsed();
    
    let total_chars = text.len() * iterations;
    total_chars as f64 / duration.as_secs_f64()
}

fn measure_latency(shlesha: &Shlesha, from: &str, to: &str, text: &str, iterations: usize) -> f64 {
    let mut times = Vec::with_capacity(iterations);
    
    // Warmup
    for _ in 0..10 {
        let _ = shlesha.transliterate(text, from, to);
    }
    
    for _ in 0..iterations {
        let start = Instant::now();
        if let Err(e) = shlesha.transliterate(text, from, to) {
            eprintln!("Error: {}", e);
            return 0.0;
        }
        let duration = start.elapsed();
        times.push(duration.as_nanos() as f64);
    }
    
    times.iter().sum::<f64>() / times.len() as f64
}

fn test_direct_mapping_usage(shlesha: &Shlesha) {
    let direct_tests = vec![
        ("iso15919", "devanagari", "ka ga ma"),
        ("devanagari", "iso15919", "क ग म"),
    ];
    
    let hub_tests = vec![
        ("iast", "devanagari", "ka ga ma"),
        ("devanagari", "iast", "क ग म"),
    ];
    
    println!("Direct mapping conversions (should be fast):");
    for (from, to, text) in direct_tests {
        let throughput = measure_throughput(shlesha, from, to, text, 1000);
        println!("  {} → {}: {:.0} chars/sec", from, to, throughput);
    }
    
    println!("Hub-based conversions (comparison):");
    for (from, to, text) in hub_tests {
        let throughput = measure_throughput(shlesha, from, to, text, 1000);
        println!("  {} → {}: {:.0} chars/sec", from, to, throughput);
    }
}