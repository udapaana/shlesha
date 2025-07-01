//! Command-line tool for managing Shlesha's profile-guided optimization system
//!
//! This tool provides commands to:
//! - Profile existing text files
//! - Generate optimizations from profiles
//! - Benchmark optimization effectiveness
//! - Manage profile data

use shlesha::{Shlesha, modules::profiler::{ProfilerConfig, OptimizationGenerator}};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug)]
enum Command {
    Profile {
        input_file: String,
        from_script: String,
        to_script: String,
        output_dir: Option<String>,
    },
    GenerateOptimizations {
        profile_dir: String,
        output_dir: String,
    },
    Benchmark {
        optimization_file: String,
        test_file: String,
    },
    Stats {
        profile_dir: String,
    },
    Help,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return;
    }

    let command = match parse_args(&args) {
        Ok(cmd) => cmd,
        Err(e) => {
            eprintln!("Error: {}", e);
            print_help();
            return;
        }
    };

    match execute_command(command) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn parse_args(args: &[String]) -> Result<Command, String> {
    match args[1].as_str() {
        "profile" => {
            if args.len() < 5 {
                return Err("Profile command requires: input_file from_script to_script [output_dir]".to_string());
            }
            Ok(Command::Profile {
                input_file: args[2].clone(),
                from_script: args[3].clone(),
                to_script: args[4].clone(),
                output_dir: args.get(5).cloned(),
            })
        }
        "generate" => {
            if args.len() < 4 {
                return Err("Generate command requires: profile_dir output_dir".to_string());
            }
            Ok(Command::GenerateOptimizations {
                profile_dir: args[2].clone(),
                output_dir: args[3].clone(),
            })
        }
        "benchmark" => {
            if args.len() < 4 {
                return Err("Benchmark command requires: optimization_file test_file".to_string());
            }
            Ok(Command::Benchmark {
                optimization_file: args[2].clone(),
                test_file: args[3].clone(),
            })
        }
        "stats" => {
            if args.len() < 3 {
                return Err("Stats command requires: profile_dir".to_string());
            }
            Ok(Command::Stats {
                profile_dir: args[2].clone(),
            })
        }
        "help" | "--help" | "-h" => Ok(Command::Help),
        _ => Err(format!("Unknown command: {}", args[1])),
    }
}

fn execute_command(command: Command) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Command::Profile { input_file, from_script, to_script, output_dir } => {
            profile_file(input_file, from_script, to_script, output_dir)?;
        }
        Command::GenerateOptimizations { profile_dir, output_dir } => {
            generate_optimizations(profile_dir, output_dir)?;
        }
        Command::Benchmark { optimization_file, test_file } => {
            benchmark_optimization(optimization_file, test_file)?;
        }
        Command::Stats { profile_dir } => {
            show_stats(profile_dir)?;
        }
        Command::Help => {
            print_help();
        }
    }
    Ok(())
}

fn profile_file(
    input_file: String,
    from_script: String,
    to_script: String,
    output_dir: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Profiling file: {}", input_file);
    println!("Conversion: {} -> {}", from_script, to_script);

    let text = fs::read_to_string(&input_file)?;
    
    // Configure profiler
    let mut config = ProfilerConfig::default();
    if let Some(dir) = output_dir {
        config.profile_dir = PathBuf::from(dir);
    }
    
    // Create transliterator with profiling
    let mut transliterator = Shlesha::new();
    transliterator.enable_profiling_with_config(config);

    // Process the text
    let start = Instant::now();
    let result = transliterator.transliterate(&text, &from_script, &to_script)?;
    let processing_time = start.elapsed();

    println!("Processing completed in: {:?}", processing_time);
    println!("Input length: {} characters", text.len());
    println!("Output length: {} characters", result.len());

    // Save profiles
    transliterator.save_profiles();
    
    // Show profile statistics
    if let Some(stats) = transliterator.get_profile_stats() {
        for ((from, to), profile_stats) in stats {
            println!("\nProfile for {} -> {}:", from, to);
            println!("  Total sequences profiled: {}", profile_stats.total_sequences_profiled);
            println!("  Unique sequences: {}", profile_stats.unique_sequences);
            println!("  Top 10 sequences:");
            for (i, (seq, count)) in profile_stats.top_sequences.iter().enumerate() {
                if i >= 10 { break; }
                println!("    {}: {} ({}x)", i + 1, seq, count);
            }
        }
    }

    println!("\nProfileing completed. Profile data saved to profiles directory.");
    Ok(())
}

fn generate_optimizations(
    profile_dir: String,
    output_dir: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating optimizations from profiles in: {}", profile_dir);
    
    // Create transliterator and load profiles
    let mut config = ProfilerConfig::default();
    config.profile_dir = PathBuf::from(&profile_dir);
    config.optimization_dir = PathBuf::from(&output_dir);
    
    let mut transliterator = Shlesha::new();
    transliterator.enable_profiling_with_config(config);
    
    // Generate optimizations
    let optimizations = transliterator.generate_optimizations();
    
    if optimizations.is_empty() {
        println!("No optimizations generated. Check that profile directory contains valid profiles.");
        return Ok(());
    }

    // Use the optimization generator to create actual mappings
    let generator = OptimizationGenerator::new();
    
    let mut generated_count = 0;
    for optimization in &optimizations {
        println!("Generated optimization for {} -> {}", 
                 optimization.from_script, optimization.to_script);
        println!("  Sequences: {}", optimization.sequence_mappings.len());
        println!("  Words: {}", optimization.word_mappings.len());
        println!("  Total entries: {}", optimization.metadata.sequence_count);
        
        // Save the optimization
        let filename = format!("{}_{}_opt.json", optimization.from_script, optimization.to_script);
        let output_path = PathBuf::from(&output_dir).join(filename);
        let json = serde_json::to_string_pretty(optimization)?;
        fs::write(&output_path, json)?;
        
        generated_count += 1;
    }

    println!("\nGenerated {} optimization files in: {}", generated_count, output_dir);
    Ok(())
}

fn benchmark_optimization(
    optimization_file: String,
    test_file: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Benchmarking optimization: {}", optimization_file);
    println!("Test file: {}", test_file);

    // Load optimization
    let opt_content = fs::read_to_string(&optimization_file)?;
    let optimization: shlesha::modules::profiler::OptimizedLookupTable = 
        serde_json::from_str(&opt_content)?;

    // Load test text
    let test_text = fs::read_to_string(&test_file)?;

    // Create generator and benchmark
    let generator = OptimizationGenerator::new();
    let benchmark = generator.benchmark_optimization(&optimization, &test_text);

    println!("\nBenchmark Results:");
    println!("  Baseline time: {} ns", benchmark.baseline_time_ns);
    println!("  Optimized time: {} ns", benchmark.optimized_time_ns);
    println!("  Speedup factor: {:.2}x", benchmark.speedup_factor);
    println!("  Cache hits: {}/{} sequences", benchmark.cache_hits, benchmark.total_sequences);
    println!("  Cache hit rate: {:.1}%", 
             (benchmark.cache_hits as f64 / benchmark.total_sequences as f64) * 100.0);

    if benchmark.speedup_factor > 1.0 {
        println!("  ✓ Optimization provides {:.1}% speedup", 
                 (benchmark.speedup_factor - 1.0) * 100.0);
    } else if benchmark.speedup_factor < 1.0 {
        println!("  ⚠ Optimization is {:.1}% slower", 
                 (1.0 - benchmark.speedup_factor) * 100.0);
    } else {
        println!("  → No significant performance difference");
    }

    Ok(())
}

fn show_stats(profile_dir: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Profile statistics for: {}", profile_dir);

    let profile_path = PathBuf::from(&profile_dir);
    if !profile_path.exists() {
        println!("Profile directory does not exist: {}", profile_dir);
        return Ok(());
    }

    let mut total_profiles = 0;
    let mut total_sequences = 0;
    let mut total_conversions = 0;

    // Read all profile files
    for entry in fs::read_dir(&profile_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(profile) = serde_json::from_str::<shlesha::modules::profiler::ConversionProfile>(&content) {
                    total_profiles += 1;
                    total_sequences += profile.sequences.len();
                    total_conversions += profile.total_conversions;

                    println!("\nProfile: {} -> {}", profile.from_script, profile.to_script);
                    println!("  Conversions: {}", profile.total_conversions);
                    println!("  Unique sequences: {}", profile.sequences.len());
                    println!("  Created: {:?}", profile.created_at);
                    println!("  Updated: {:?}", profile.updated_at);

                    // Show top sequences
                    let mut sequences: Vec<_> = profile.sequences.iter().collect();
                    sequences.sort_by_key(|(_, stats)| std::cmp::Reverse(stats.count));
                    
                    println!("  Top sequences:");
                    for (i, (seq, stats)) in sequences.iter().take(5).enumerate() {
                        println!("    {}: '{}' ({}x, avg: {:.0}ns)", 
                                 i + 1, seq, stats.count, stats.avg_processing_ns);
                    }
                }
            }
        }
    }

    println!("\n=== Summary ===");
    println!("Total profiles: {}", total_profiles);
    println!("Total unique sequences: {}", total_sequences);
    println!("Total conversions recorded: {}", total_conversions);

    if total_profiles > 0 {
        println!("Average sequences per profile: {:.1}", 
                 total_sequences as f64 / total_profiles as f64);
        println!("Average conversions per profile: {:.1}", 
                 total_conversions as f64 / total_profiles as f64);
    }

    Ok(())
}

fn print_help() {
    println!("Shlesha Profile-Guided Optimization Tool");
    println!();
    println!("USAGE:");
    println!("  profiler_cli <command> [options]");
    println!();
    println!("COMMANDS:");
    println!("  profile <input_file> <from_script> <to_script> [output_dir]");
    println!("      Profile a text file and collect usage statistics");
    println!("      Example: profiler_cli profile bhagavad_gita.txt devanagari iast");
    println!();
    println!("  generate <profile_dir> <output_dir>");
    println!("      Generate optimization tables from collected profiles");
    println!("      Example: profiler_cli generate profiles optimizations");
    println!();
    println!("  benchmark <optimization_file> <test_file>");
    println!("      Benchmark the effectiveness of an optimization");
    println!("      Example: profiler_cli benchmark devanagari_iast_opt.json test.txt");
    println!();
    println!("  stats <profile_dir>");
    println!("      Show statistics about collected profiles");
    println!("      Example: profiler_cli stats profiles");
    println!();
    println!("  help");
    println!("      Show this help message");
    println!();
    println!("SUPPORTED SCRIPTS:");
    println!("  devanagari, iast, iso15919, itrans, slp1, harvard_kyoto, velthuis, wx");
    println!("  bengali, telugu, tamil, kannada, malayalam, gujarati, gurmukhi, odia");
}