use shlesha::Shlesha;
use std::time::Instant;

fn main() {
    let shlesha = Shlesha::new();
    
    println!("🚀 Direct Mapping Performance Analysis");
    println!("=====================================");
    
    // Test different conversion patterns to understand where pre-computation helps
    let test_cases = vec![
        // Direct mapping candidates (currently iso15919 ↔ devanagari) 
        ("iso15919", "devanagari", "ka ga na ma ra la sa ha", "Direct mapping (implemented)"),
        ("devanagari", "iso15919", "क ग न म र ल स ह", "Direct mapping (implemented)"),
        
        // Hub-based conversions that could benefit from pre-computation
        ("iast", "devanagari", "ka ga na ma ra la sa ha", "Hub-based (could benefit)"),
        ("devanagari", "iast", "क ग न म र ल स ह", "Hub-based (could benefit)"),
        ("itrans", "devanagari", "ka ga na ma ra la sa ha", "Hub-based (could benefit)"),
        ("devanagari", "itrans", "क ग न म र ल स ह", "Hub-based (could benefit)"),
        
        // Hub-to-hub conversions (no pre-computation benefit expected)
        ("devanagari", "telugu", "क ग न म र ल स ह", "Hub-to-hub (no benefit)"),
        ("devanagari", "tamil", "क ग न म र ल स ह", "Hub-to-hub (no benefit)"),
        
        // Roman-to-roman conversions (no pre-computation benefit expected) 
        ("iast", "itrans", "ka ga na ma ra la sa ha", "Roman-to-roman (no benefit)"),
        ("iast", "slp1", "ka ga na ma ra la sa ha", "Roman-to-roman (no benefit)"),
    ];
    
    println!("\n📊 Performance Analysis by Conversion Pattern:\n");
    
    let mut results_by_category = std::collections::HashMap::new();
    
    for (from, to, text, category) in test_cases {
        let throughput = measure_throughput(&shlesha, from, to, text, 1000);
        
        results_by_category.entry(category).or_insert_with(Vec::new).push((from, to, throughput));
        
        println!("  ✅ {} → {}: {:.0} chars/sec ({})", from, to, throughput, category);
    }
    
    println!("\n📈 Analysis by Category:\n");
    
    for (category, results) in results_by_category {
        println!("🔹 **{}**", category);
        let avg_throughput: f64 = results.iter().map(|(_, _, t)| t).sum::<f64>() / results.len() as f64;
        println!("   Average: {:.0} chars/sec", avg_throughput);
        
        for (from, to, throughput) in results {
            println!("   - {} → {}: {:.0} chars/sec", from, to, throughput);
        }
        println!();
    }
    
    println!("💡 Key Insights:");
    println!("- Direct mappings (iso15919 ↔ devanagari) show the current pre-computation impact");
    println!("- Hub-based conversions (iast, itrans) could benefit from expanded pre-computation");
    println!("- Hub-to-hub and roman-to-roman show baseline performance without pre-computation");
    
    // Test complex vs simple text to see algorithmic differences
    println!("\n🧪 Complex vs Simple Text Analysis:\n");
    
    let simple_text = "ka ga ma";  // Simple characters only
    let complex_text = "dharma yoga bhārata"; // Complex sequences with conjuncts
    
    let test_conversions = vec![
        ("iast", "devanagari"),
        ("devanagari", "iast"), 
        ("iso15919", "devanagari"),
        ("devanagari", "iso15919"),
    ];
    
    for (from, to) in test_conversions {
        let simple_perf = measure_throughput(&shlesha, from, to, simple_text, 1000);
        let complex_perf = measure_throughput(&shlesha, from, to, complex_text, 1000);
        let complexity_ratio = simple_perf / complex_perf;
        
        println!("  {} → {}: Simple={:.0} chars/sec, Complex={:.0} chars/sec, Ratio={:.2}x", 
                from, to, simple_perf, complex_perf, complexity_ratio);
    }
    
    println!("\n🎯 Pre-computation Optimization Roadmap:");
    println!("1. **Immediate**: Expand direct mappings to iast ↔ devanagari, itrans ↔ devanagari");
    println!("2. **Medium-term**: Add multi-character pattern optimization for conjuncts");
    println!("3. **Long-term**: Implement streaming/batch processing optimizations");
}

fn measure_throughput(shlesha: &Shlesha, from: &str, to: &str, text: &str, iterations: usize) -> f64 {
    // Warmup
    for _ in 0..10 {
        let _ = shlesha.transliterate(text, from, to);
    }
    
    // Measure
    let start = Instant::now();
    for _ in 0..iterations {
        if let Err(e) = shlesha.transliterate(text, from, to) {
            eprintln!("Error in {} → {}: {}", from, to, e);
            return 0.0;
        }
    }
    let duration = start.elapsed();
    
    let total_chars = text.len() * iterations;
    total_chars as f64 / duration.as_secs_f64()
}