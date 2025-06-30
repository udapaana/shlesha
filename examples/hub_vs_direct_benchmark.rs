use shlesha::Shlesha;
use std::time::Instant;
use vidyut_lipi::{Lipika, Scheme};

fn main() {
    println!("🌟 Hub-and-Spoke vs Direct Conversion Performance");
    println!("=================================================");
    println!();
    println!("Comparing identical conversions:");
    println!("  📚 Shlesha: Hub-and-spoke (2-3 hops through Devanagari/ISO-15919)");
    println!("  🚀 Vidyut: Direct conversion (optimized single-step)");
    println!();

    // Initialize transliterators
    let shlesha = Shlesha::new();
    let mut vidyut = Lipika::new();

    // Test cases: cross-script conversions that both support
    let test_cases = vec![
        // Indic → Roman conversions
        (
            "Telugu → SLP1",
            "telugu",
            "slp1",
            Scheme::Telugu,
            Scheme::Slp1,
            "ధర్మక్షేత్ర",
        ),
        (
            "Telugu → IAST",
            "telugu",
            "iast",
            Scheme::Telugu,
            Scheme::Iast,
            "ధర్మక్షేత్ర",
        ),
        (
            "Bengali → SLP1",
            "bengali",
            "slp1",
            Scheme::Bengali,
            Scheme::Slp1,
            "ধর্মক্ষেত্র",
        ),
        (
            "Bengali → IAST",
            "bengali",
            "iast",
            Scheme::Bengali,
            Scheme::Iast,
            "ধর্মক্ষেত্র",
        ),
        (
            "Tamil → SLP1",
            "tamil",
            "slp1",
            Scheme::Tamil,
            Scheme::Slp1,
            "தர்மக்ஷேத்ர",
        ),
        // Roman → Indic conversions
        (
            "SLP1 → Telugu",
            "slp1",
            "telugu",
            Scheme::Slp1,
            Scheme::Telugu,
            "dharmakSetra",
        ),
        (
            "IAST → Telugu",
            "iast",
            "telugu",
            Scheme::Iast,
            Scheme::Telugu,
            "dharmakṣetra",
        ),
        (
            "SLP1 → Bengali",
            "slp1",
            "bengali",
            Scheme::Slp1,
            Scheme::Bengali,
            "dharmakSetra",
        ),
        (
            "IAST → Bengali",
            "iast",
            "bengali",
            Scheme::Iast,
            Scheme::Bengali,
            "dharmakṣetra",
        ),
        (
            "SLP1 → Tamil",
            "slp1",
            "tamil",
            Scheme::Slp1,
            Scheme::Tamil,
            "dharmakSetra",
        ),
        // Cross-Indic conversions (3-hop for Shlesha vs direct for Vidyut)
        (
            "Telugu → Bengali",
            "telugu",
            "bengali",
            Scheme::Telugu,
            Scheme::Bengali,
            "ధర్మక్షేత్ర",
        ),
        (
            "Bengali → Telugu",
            "bengali",
            "telugu",
            Scheme::Bengali,
            Scheme::Telugu,
            "ধর্মক্ষেত্র",
        ),
        (
            "Tamil → Malayalam",
            "tamil",
            "malayalam",
            Scheme::Tamil,
            Scheme::Malayalam,
            "தர்மக்ஷேத்ர",
        ),
        (
            "Telugu → Tamil",
            "telugu",
            "tamil",
            Scheme::Telugu,
            Scheme::Tamil,
            "ధర్మక్షేత్ర",
        ),
    ];

    // Text length variations
    let text_scales = vec![
        ("Short", 1, 10000),
        ("Medium", 10, 1000),
        ("Long", 100, 100),
    ];

    for (scale_name, repeat_count, iterations) in text_scales {
        println!("=== {} Text ===", scale_name);

        for (conversion_name, shlesha_from, shlesha_to, vidyut_from, vidyut_to, base_text) in
            &test_cases
        {
            let test_text = base_text.repeat(repeat_count);
            let text_len = test_text.chars().count();

            println!("📖 {} ({} chars):", conversion_name, text_len);

            // Benchmark Shlesha (hub-and-spoke)
            let start_time = Instant::now();
            let mut shlesha_result = String::new();
            let mut shlesha_success = true;
            for _ in 0..iterations {
                match shlesha.transliterate(&test_text, shlesha_from, shlesha_to) {
                    Ok(result) => shlesha_result = result,
                    Err(e) => {
                        println!("  ❌ Shlesha error: {}", e);
                        shlesha_success = false;
                        break;
                    }
                }
            }

            if shlesha_success {
                let shlesha_duration = start_time.elapsed();
                let shlesha_throughput = (test_text.len() as f64 * iterations as f64)
                    / (shlesha_duration.as_secs_f64() * 1024.0 * 1024.0);
                let shlesha_avg = shlesha_duration / iterations;

                // Benchmark Vidyut (direct conversion)
                let start_time = Instant::now();
                let mut vidyut_result = String::new();
                for _ in 0..iterations {
                    vidyut_result = vidyut.transliterate(&test_text, *vidyut_from, *vidyut_to);
                }
                let vidyut_duration = start_time.elapsed();
                let vidyut_throughput = (test_text.len() as f64 * iterations as f64)
                    / (vidyut_duration.as_secs_f64() * 1024.0 * 1024.0);
                let vidyut_avg = vidyut_duration / iterations;

                // Compare results
                let ratio = shlesha_throughput / vidyut_throughput;
                let comparison = if ratio > 1.0 {
                    format!("Hub is {:.2}x faster than Direct", ratio)
                } else {
                    format!("Hub is {:.2}x slower than Direct", 1.0 / ratio)
                };

                println!(
                    "  📚 Shlesha (Hub):  {:.2} MB/s ({:?} avg)",
                    shlesha_throughput, shlesha_avg
                );
                println!(
                    "  🚀 Vidyut (Direct): {:.2} MB/s ({:?} avg)",
                    vidyut_throughput, vidyut_avg
                );
                println!("  📊 Performance:     {}", comparison);

                // Correctness check for short text
                if text_len < 20 {
                    // Normalize results for comparison (remove extra spaces, etc.)
                    let shlesha_norm = shlesha_result.trim().replace(" ", "");
                    let vidyut_norm = vidyut_result.trim().replace(" ", "");

                    if shlesha_norm == vidyut_norm {
                        println!("  ✅ Results match");
                    } else {
                        println!("  ⚠️  Results differ:");
                        println!(
                            "      Shlesha: {}",
                            shlesha_result.chars().take(30).collect::<String>()
                        );
                        println!(
                            "      Vidyut:  {}",
                            vidyut_result.chars().take(30).collect::<String>()
                        );
                    }
                }
            }
            println!();
        }

        println!("────────────────────────────────────────────────────────────");
        println!();
    }

    // Architecture analysis
    println!("🔍 ARCHITECTURE ANALYSIS");
    println!("========================");
    println!("Shlesha Hub-and-Spoke Conversions:");
    println!("  📚 Indic → Roman: Script → Devanagari → ISO-15919 → Roman (2 hops)");
    println!("  📚 Roman → Indic: Roman → ISO-15919 → Devanagari → Script (2 hops)");
    println!("  📚 Cross-Indic:   Source → Devanagari → Target (1 hop)");
    println!();
    println!("Vidyut Direct Conversions:");
    println!("  🚀 Any → Any: Optimized direct mapping (0 hops)");
    println!("  🚀 Compile-time optimization for each path");
    println!("  🚀 No intermediate representations");
    println!();

    println!("🏆 PERFORMANCE TRADE-OFFS");
    println!("=========================");
    println!("Shlesha Advantages:");
    println!("  ✅ Runtime extensibility (load new schemas)");
    println!("  ✅ Consistent conversion paths (via proven hubs)");
    println!("  ✅ Easy to add scripts (just map to/from hub)");
    println!("  ✅ Unified architecture for all conversions");
    println!();
    println!("Vidyut Advantages:");
    println!("  ✅ Maximum performance (direct conversions)");
    println!("  ✅ No intermediate conversion overhead");
    println!("  ✅ Highly optimized for each conversion path");
    println!("  ✅ Minimal memory allocation");
    println!();
    println!("Trade-off Summary:");
    println!("  📊 Shlesha: Slight performance cost for major extensibility gain");
    println!("  📊 Vidyut: Maximum performance, limited extensibility");
    println!("  📊 Both: Excellent choices for different use cases");
}
