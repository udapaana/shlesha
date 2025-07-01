//! Profile-Guided Optimization Demo
//!
//! This example demonstrates the complete workflow of Shlesha's
//! profile-guided optimization system:
//! 1. Collecting usage profiles during transliteration
//! 2. Generating optimized lookup tables from profiles
//! 3. Using optimizations to speed up subsequent transliterations
//! 4. Hot-reloading optimizations without restart

use shlesha::{Shlesha, modules::profiler::{ProfilerConfig, OptimizationGenerator}};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Shlesha Profile-Guided Optimization Demo ===\n");

    // Create test data
    let test_texts = vec![
        // Common Sanskrit/Hindi words and phrases
        "धर्म कर्म योग वेद मन्त्र तन्त्र यन्त्र आत्मा ब्रह्म मोक्ष निर्वाण समाधि ध्यान",
        "नमस्ते नमस्कार श्री ॐ स्वामी महा राज देव देवी मन्दिर पूजा आरती प्रसाद",
        "रामायण महाभारत गीता उपनिषद् पुराण सूत्र शास्त्र वेदान्त संस्कृत हिन्दी भारत",
        "धर्म पुत्र युधिष्ठिर भीम अर्जुन नकुल सहदेव द्रौपदी कृष्ण राधा गोपी गोकुल वृन्दावन",
        "सत्य अहिंसा करुणा दया प्रेम शान्ति आनन्द प्राण चक्र गुरु शिष्य साधना सिद्धि",
    ];

    // Step 1: Demonstrate basic profiling
    println!("Step 1: Collecting usage profiles during transliteration");
    println!("======================================================");
    
    let (transliterator, profile_stats) = demonstrate_profiling(&test_texts)?;
    
    // Step 2: Generate optimizations
    println!("\nStep 2: Generating optimized lookup tables");
    println!("==========================================");
    
    let optimizations = demonstrate_optimization_generation(&transliterator, &profile_stats)?;
    
    // Step 3: Benchmark optimizations
    println!("\nStep 3: Benchmarking optimization effectiveness");
    println!("==============================================");
    
    demonstrate_optimization_benchmarking(&optimizations, &test_texts)?;
    
    // Step 4: Hot-reload demonstration
    println!("\nStep 4: Hot-reload demonstration");
    println!("================================");
    
    demonstrate_hot_reload(&transliterator)?;
    
    // Step 5: Show real-world usage example
    println!("\nStep 5: Real-world usage example");
    println!("================================");
    
    demonstrate_real_world_usage()?;

    println!("\n=== Demo completed successfully! ===");
    println!("\nKey takeaways:");
    println!("• Profiling automatically identifies frequently used sequences");
    println!("• Optimizations provide significant speedup for common patterns");
    println!("• Hot-reloading allows updating optimizations without restart");
    println!("• The system is especially effective for Sanskrit/Hindi text");
    
    Ok(())
}

fn demonstrate_profiling(
    test_texts: &[&str],
) -> Result<(Shlesha, rustc_hash::FxHashMap<(String, String), shlesha::modules::profiler::ProfileStats>), Box<dyn std::error::Error>> {
    // Configure profiler for demonstration
    let mut config = ProfilerConfig::default();
    config.min_sequence_frequency = 3; // Lower threshold for demo
    config.profile_dir = PathBuf::from("demo_profiles");
    config.optimization_dir = PathBuf::from("demo_optimizations");
    
    // Create directories
    fs::create_dir_all(&config.profile_dir)?;
    fs::create_dir_all(&config.optimization_dir)?;
    
    // Create transliterator with profiling enabled
    let mut transliterator = Shlesha::new();
    transliterator.enable_profiling_with_config(config);

    println!("Processing {} test texts to collect usage patterns...", test_texts.len());
    
    // Process each text multiple times to simulate real usage
    let conversion_pairs = [
        ("devanagari", "iast"),
        ("devanagari", "iso15919"),
        ("devanagari", "itrans"),
    ];
    
    for (from, to) in &conversion_pairs {
        println!("Profiling conversion: {} -> {}", from, to);
        
        for (i, text) in test_texts.iter().enumerate() {
            // Process each text multiple times to build up frequency data
            for iteration in 0..5 {
                let start = Instant::now();
                let result = transliterator.transliterate(text, from, to)?;
                let duration = start.elapsed();
                
                if iteration == 0 {
                    let display_input = if text.chars().count() > 20 {
                        format!("{}...", text.chars().take(20).collect::<String>())
                    } else {
                        text.to_string()
                    };
                    let display_output = if result.chars().count() > 20 {
                        format!("{}...", result.chars().take(20).collect::<String>())
                    } else {
                        result.clone()
                    };
                    println!("  Text {}: '{}' -> '{}'", i + 1, display_input, display_output);
                    println!("    Processing time: {:?}", duration);
                }
            }
        }
    }

    // Save profiles
    transliterator.save_profiles();
    
    // Get and display profile statistics
    let profile_stats = transliterator.get_profile_stats().unwrap_or_default();
    
    println!("\nProfile Statistics:");
    for ((from, to), stats) in &profile_stats {
        println!("  {} -> {}:", from, to);
        println!("    Total sequences profiled: {}", stats.total_sequences_profiled);
        println!("    Unique sequences: {}", stats.unique_sequences);
        println!("    Top 5 frequent sequences:");
        for (i, (seq, count)) in stats.top_sequences.iter().take(5).enumerate() {
            println!("      {}: '{}' ({}x)", i + 1, seq, count);
        }
    }
    
    Ok((transliterator, profile_stats))
}

fn demonstrate_optimization_generation(
    transliterator: &Shlesha,
    _profile_stats: &rustc_hash::FxHashMap<(String, String), shlesha::modules::profiler::ProfileStats>,
) -> Result<Vec<shlesha::modules::profiler::OptimizedLookupTable>, Box<dyn std::error::Error>> {
    println!("Generating optimized lookup tables from collected profiles...");
    
    let optimizations = transliterator.generate_optimizations();
    
    println!("Generated {} optimization tables:", optimizations.len());
    
    for optimization in &optimizations {
        println!("  {} -> {}:", optimization.from_script, optimization.to_script);
        println!("    Sequence mappings: {}", optimization.sequence_mappings.len());
        println!("    Word mappings: {}", optimization.word_mappings.len());
        println!("    Total optimized entries: {}", optimization.metadata.sequence_count);
        println!("    Generated at: {:?}", optimization.metadata.generated_at);
        
        // Show some sample mappings
        println!("    Sample sequence mappings:");
        for (from, to) in optimization.sequence_mappings.iter().take(3) {
            println!("      '{}' -> '{}'", from, to);
        }
        
        if !optimization.word_mappings.is_empty() {
            println!("    Sample word mappings:");
            for (from, to) in optimization.word_mappings.iter().take(3) {
                println!("      '{}' -> '{}'", from, to);
            }
        }
    }
    
    // Save optimizations for hot-reload demo
    for optimization in &optimizations {
        let filename = format!("{}_{}_opt.json", optimization.from_script, optimization.to_script);
        let path = PathBuf::from("demo_optimizations").join(filename);
        let json = serde_json::to_string_pretty(optimization)?;
        fs::write(&path, json)?;
        println!("  Saved optimization to: {:?}", path);
    }
    
    Ok(optimizations)
}

fn demonstrate_optimization_benchmarking(
    optimizations: &[shlesha::modules::profiler::OptimizedLookupTable],
    test_texts: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Benchmarking optimization effectiveness...");
    
    let generator = OptimizationGenerator::new();
    
    for optimization in optimizations {
        println!("\nBenchmarking {} -> {} optimization:", 
                 optimization.from_script, optimization.to_script);
        
        let combined_text = test_texts.join(" ");
        let benchmark = generator.benchmark_optimization(optimization, &combined_text);
        
        println!("  Baseline processing time: {} ns", benchmark.baseline_time_ns);
        println!("  Optimized processing time: {} ns", benchmark.optimized_time_ns);
        println!("  Speedup factor: {:.2}x", benchmark.speedup_factor);
        println!("  Cache hits: {}/{} sequences", benchmark.cache_hits, benchmark.total_sequences);
        println!("  Cache hit rate: {:.1}%", 
                 (benchmark.cache_hits as f64 / benchmark.total_sequences as f64) * 100.0);
        
        if benchmark.speedup_factor > 1.0 {
            println!("  ✓ Optimization provides {:.1}% performance improvement", 
                     (benchmark.speedup_factor - 1.0) * 100.0);
        } else {
            println!("  → Optimization overhead detected (common for small texts)");
        }
    }
    
    Ok(())
}

fn demonstrate_hot_reload(
    _transliterator: &Shlesha,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting up hot-reload manager...");
    
    // Create a new transliterator to simulate running application
    let app_transliterator = Shlesha::new();
    
    // Load existing optimizations
    let opt_dir = PathBuf::from("demo_optimizations");
    if opt_dir.exists() {
        for entry in fs::read_dir(&opt_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(optimization) = serde_json::from_str::<shlesha::modules::profiler::OptimizedLookupTable>(&content) {
                        app_transliterator.load_optimization(optimization);
                        println!("  Loaded optimization from: {:?}", path.file_name().unwrap());
                    }
                }
            }
        }
    }
    
    // Test conversion with loaded optimizations
    let test_text = "धर्म योग कर्म वेद मन्त्र";
    println!("\nTesting conversion with loaded optimizations:");
    println!("  Input: '{}'", test_text);
    
    let start = Instant::now();
    let result = app_transliterator.transliterate(test_text, "devanagari", "iast")?;
    let duration = start.elapsed();
    
    println!("  Output: '{}'", result);
    println!("  Processing time: {:?}", duration);
    
    // Simulate creating a new optimization file for hot-reload
    println!("\nSimulating hot-reload by creating new optimization...");
    
    // Create a simple test optimization
    let mut test_optimization = shlesha::modules::profiler::OptimizedLookupTable {
        from_script: "devanagari".to_string(),
        to_script: "test_hot_reload".to_string(),
        sequence_mappings: rustc_hash::FxHashMap::default(),
        word_mappings: rustc_hash::FxHashMap::default(),
        metadata: shlesha::modules::profiler::OptimizationMetadata {
            generated_at: std::time::SystemTime::now(),
            sequence_count: 1,
            min_frequency: 1,
            profile_stats: shlesha::modules::profiler::ProfileStats {
                total_sequences_profiled: 100,
                unique_sequences: 10,
                top_sequences: vec![("धर्म".to_string(), 50)],
            },
        },
    };
    test_optimization.sequence_mappings.insert("धर्म".to_string(), "DHARMA_HOT_RELOAD".to_string());
    
    let hot_reload_file = opt_dir.join("devanagari_test_hot_reload_opt.json");
    let json = serde_json::to_string_pretty(&test_optimization)?;
    fs::write(&hot_reload_file, json)?;
    
    println!("  Created new optimization file: {:?}", hot_reload_file.file_name().unwrap());
    
    // Load the new optimization
    app_transliterator.load_optimization(test_optimization);
    println!("  Hot-reloaded new optimization successfully!");
    
    // Clean up
    let _ = fs::remove_file(&hot_reload_file);
    
    Ok(())
}

fn demonstrate_real_world_usage() -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating real-world usage scenario...");
    
    // Simulate a web application processing Sanskrit texts
    let web_app_transliterator = Shlesha::with_profiling();
    
    // Simulate multiple user requests
    let user_requests = vec![
        ("भगवद्गीता", "devanagari", "iast", "User request: transliterate book title"),
        ("श्रीमद्भागवत", "devanagari", "itrans", "User request: transliterate scripture name"),
        ("रामायण महाभारत", "devanagari", "iso15919", "User request: epic names"),
        ("ॐ नमो भगवते वासुदेवाय", "devanagari", "iast", "User request: mantra"),
        ("धर्म अर्थ काम मोक्ष", "devanagari", "iast", "User request: four goals of life"),
    ];
    
    println!("\nProcessing user requests (building profiles):");
    for (i, (text, from, to, description)) in user_requests.iter().enumerate() {
        println!("  {}: {}", i + 1, description);
        
        let start = Instant::now();
        let result = web_app_transliterator.transliterate(text, from, to)?;
        let duration = start.elapsed();
        
        println!("     '{}' -> '{}'", text, result);
        println!("     Processing time: {:?}", duration);
    }
    
    // After some time, generate optimizations
    println!("\nAfter collecting usage data, generating optimizations...");
    let optimizations = web_app_transliterator.generate_optimizations();
    
    if !optimizations.is_empty() {
        println!("  Generated {} optimization tables", optimizations.len());
        
        // Load optimizations
        for optimization in optimizations {
            web_app_transliterator.load_optimization(optimization);
        }
        
        println!("  Loaded optimizations into application");
        
        // Process same requests again with optimizations
        println!("\nProcessing same requests with optimizations:");
        for (i, (text, from, to, description)) in user_requests.iter().enumerate() {
            let start = Instant::now();
            let _result = web_app_transliterator.transliterate(text, from, to)?;
            let duration = start.elapsed();
            
            println!("  {}: {} -> {:?}", i + 1, description, duration);
        }
    } else {
        println!("  Not enough data to generate optimizations yet");
        println!("  (In real usage, collect more data over time)");
    }
    
    // Show final statistics
    if let Some(stats) = web_app_transliterator.get_profile_stats() {
        println!("\nFinal application statistics:");
        for ((from, to), profile_stats) in stats {
            println!("  {} -> {}: {} conversions, {} unique sequences", 
                     from, to, profile_stats.total_sequences_profiled, profile_stats.unique_sequences);
        }
    }
    
    Ok(())
}