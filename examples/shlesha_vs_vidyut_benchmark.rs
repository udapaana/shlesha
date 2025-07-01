use shlesha::Shlesha;
use std::time::Instant;
use vidyut_lipi::{Lipika, Scheme};

fn main() {
    println!("🏆 Shlesha vs Vidyut Cross-Hub Performance Benchmark");
    println!("===================================================");
    println!();
    println!("This benchmark tests the most challenging conversions in Shlesha's");
    println!("hub-and-spoke architecture: those requiring BOTH hub formats.");
    println!();
    println!("Cross-hub conversions require 3 steps:");
    println!("  • Indic → Roman: Source → Devanagari → ISO-15919 → Target");
    println!("  • Roman → Indic: Source → ISO-15919 → Devanagari → Target");
    println!();
    println!("These conversions cross the Devanagari ↔ ISO-15919 bridge,");
    println!("which is the runtime hub mapping connecting Indic and Roman scripts.");
    println!();

    // Initialize transliterators
    let shlesha = Shlesha::new();
    let mut vidyut = Lipika::new();

    println!("Testing libraries:");
    println!("  📚 Shlesha: 3-step conversion through both hubs");
    println!("  🚀 Vidyut: Direct single-step conversion");
    println!();

    // Test data sets
    let test_cases = vec![
        ("Short Text", "dharma", 1),
        (
            "Medium Text",
            "dharmakSetrekuruSetrasamavetAyuyutsavaHmAmakAHpANDavAzcaivakimakurvatasaMjaya",
            1,
        ),
        (
            "Long Text",
            "dharmakSetrekuruSetrasamavetAyuyutsavaHmAmakAHpANDavAzcaivakimakurvatasaMjaya",
            100,
        ),
        (
            "Very Long Text",
            "dharmakSetrekuruSetrasamavetAyuyutsavaHmAmakAHpANDavAzcaivakimakurvatasaMjaya",
            1000,
        ),
    ];

    let mut all_results = Vec::new();

    for (test_name, base_text, repeat_count) in test_cases {
        println!("\n=== {} ===", test_name);
        println!("Text length: {} characters", base_text.len() * repeat_count);

        // Test Indic → Roman (cross-hub) conversions
        println!("\n📖 Indic → Roman (Cross-Hub: 3 steps):");

        // Bengali text
        let bengali_text = "ধর্মক্ষেত্রে কুরুক্ষেত্রে সমবেতা যুযুৎসবঃ মামকাঃ পাণ্ডবাশ্চৈব".repeat(repeat_count);
        all_results.push(benchmark_conversion(
            &shlesha,
            &mut vidyut,
            &bengali_text,
            "bengali",
            "iast",
            Scheme::Bengali,
            Scheme::Iast,
        ));
        all_results.push(benchmark_conversion(
            &shlesha,
            &mut vidyut,
            &bengali_text,
            "bengali",
            "slp1",
            Scheme::Bengali,
            Scheme::Slp1,
        ));

        // Telugu text
        let telugu_text = "ధర్మక్షేత్రే కురుక్షేత్రే సమవేతా యుయుత్సవః మామకాః పాండవాశ్చైవ".repeat(repeat_count);
        all_results.push(benchmark_conversion(
            &shlesha,
            &mut vidyut,
            &telugu_text,
            "telugu",
            "itrans",
            Scheme::Telugu,
            Scheme::Itrans,
        ));
        all_results.push(benchmark_conversion(
            &shlesha,
            &mut vidyut,
            &telugu_text,
            "telugu",
            "iast",
            Scheme::Telugu,
            Scheme::Iast,
        ));

        // Test Roman → Indic (cross-hub) conversions
        println!("\n📖 Roman → Indic (Cross-Hub: 3 steps):");

        // Roman text samples
        let slp1_text = base_text.repeat(repeat_count);
        all_results.push(benchmark_conversion(
            &shlesha,
            &mut vidyut,
            &slp1_text,
            "slp1",
            "bengali",
            Scheme::Slp1,
            Scheme::Bengali,
        ));
        all_results.push(benchmark_conversion(
            &shlesha,
            &mut vidyut,
            &slp1_text,
            "slp1",
            "telugu",
            Scheme::Slp1,
            Scheme::Telugu,
        ));

        let iast_text = "dharmakṣetre kurukṣetre samavetā yuyutsavaḥ".repeat(repeat_count);
        all_results.push(benchmark_conversion(
            &shlesha,
            &mut vidyut,
            &iast_text,
            "iast",
            "bengali",
            Scheme::Iast,
            Scheme::Bengali,
        ));
        all_results.push(benchmark_conversion(
            &shlesha,
            &mut vidyut,
            &iast_text,
            "iast",
            "telugu",
            Scheme::Iast,
            Scheme::Telugu,
        ));

        println!("\n{}", "─".repeat(60));
    }

    // Performance summary
    println!("\n📊 PERFORMANCE SUMMARY");
    println!("=====================");

    // Calculate averages
    let indic_to_roman: Vec<_> = all_results
        .iter()
        .filter(|r| is_indic(&r.from) && is_roman(&r.to))
        .collect();
    let roman_to_indic: Vec<_> = all_results
        .iter()
        .filter(|r| is_roman(&r.from) && is_indic(&r.to))
        .collect();

    println!("\nAverage Performance (Cross-Hub Only):");

    if !indic_to_roman.is_empty() {
        let avg_ratio: f64 =
            indic_to_roman.iter().map(|r| r.speed_ratio).sum::<f64>() / indic_to_roman.len() as f64;
        println!(
            "Indic → Roman: Shlesha is {:.1}x {} than Vidyut",
            if avg_ratio > 1.0 {
                avg_ratio
            } else {
                1.0 / avg_ratio
            },
            if avg_ratio > 1.0 { "faster" } else { "slower" }
        );
    }

    if !roman_to_indic.is_empty() {
        let avg_ratio: f64 =
            roman_to_indic.iter().map(|r| r.speed_ratio).sum::<f64>() / roman_to_indic.len() as f64;
        println!(
            "Roman → Indic: Shlesha is {:.1}x {} than Vidyut",
            if avg_ratio > 1.0 {
                avg_ratio
            } else {
                1.0 / avg_ratio
            },
            if avg_ratio > 1.0 { "faster" } else { "slower" }
        );
    }

    println!("\n🔍 Why Cross-Hub Matters:");
    println!("Cross-hub conversions are the most performance-critical paths because:");
    println!("1. They require 3 conversion steps vs direct conversion");
    println!("2. They must traverse the Devanagari ↔ ISO-15919 bridge");
    println!("3. They represent the worst-case scenario for hub architectures");
    println!("4. Optimizing these paths benefits all cross-script conversions");

    println!("\n📈 Optimization Opportunities:");
    println!("The Devanagari ↔ ISO-15919 mapping is the bottleneck.");
    println!("Future optimizations should focus on this critical path.");
}

struct BenchmarkResult {
    from: String,
    to: String,
    speed_ratio: f64,
}

fn benchmark_conversion(
    shlesha: &Shlesha,
    vidyut: &mut Lipika,
    text: &str,
    from: &str,
    to: &str,
    vidyut_from: Scheme,
    vidyut_to: Scheme,
) -> BenchmarkResult {
    let iterations = if text.len() < 100 {
        10000
    } else if text.len() < 10000 {
        1000
    } else {
        100
    };

    print!("  {} → {}: ", from, to);

    // Warm up
    for _ in 0..10 {
        let _ = shlesha.transliterate(text, from, to);
        let _ = vidyut.transliterate(text, vidyut_from, vidyut_to);
    }

    // Benchmark Shlesha
    let start = Instant::now();
    let mut success = true;
    for _ in 0..iterations {
        if shlesha.transliterate(text, from, to).is_err() {
            success = false;
            break;
        }
    }
    let shlesha_duration = start.elapsed();
    let shlesha_throughput = if success {
        (text.len() as f64 * iterations as f64) / shlesha_duration.as_secs_f64() / 1_000_000.0
    } else {
        0.0
    };

    // Benchmark Vidyut
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = vidyut.transliterate(text, vidyut_from, vidyut_to);
    }
    let vidyut_duration = start.elapsed();
    let vidyut_throughput =
        (text.len() as f64 * iterations as f64) / vidyut_duration.as_secs_f64() / 1_000_000.0;

    // Calculate relative performance
    let speed_ratio = if success {
        vidyut_duration.as_secs_f64() / shlesha_duration.as_secs_f64()
    } else {
        0.0
    };

    if success {
        println!(
            "Shlesha {:.2}x {} ({:.2} MB/s vs {:.2} MB/s)",
            if speed_ratio > 1.0 {
                speed_ratio
            } else {
                1.0 / speed_ratio
            },
            if speed_ratio > 1.0 {
                "faster"
            } else {
                "slower"
            },
            shlesha_throughput,
            vidyut_throughput
        );
    } else {
        println!("Shlesha: ERROR - conversion not supported");
    }

    BenchmarkResult {
        from: from.to_string(),
        to: to.to_string(),
        speed_ratio,
    }
}

fn is_roman(script: &str) -> bool {
    matches!(
        script,
        "slp1" | "iast" | "itrans" | "hk" | "velthuis" | "wx" | "iso15919"
    )
}

fn is_indic(script: &str) -> bool {
    matches!(
        script,
        "devanagari"
            | "bengali"
            | "telugu"
            | "tamil"
            | "kannada"
            | "malayalam"
            | "gujarati"
            | "gurmukhi"
            | "odia"
    )
}
