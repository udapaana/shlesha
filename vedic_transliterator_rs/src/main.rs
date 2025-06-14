//! Bidirectional Sanskrit Transliteration CLI
//! 
//! High-performance command-line tool for bidirectional transliteration
//! between Sanskrit scripts using token-based compiler architecture.

use std::time::Instant;
use clap::{Parser, Subcommand};
use colored::*;
use vedic_transliterator::{transliterate, TargetScheme, TransliterationError};

#[derive(Parser)]
#[command(name = "vedic_transliterator")]
#[command(about = "Bidirectional Sanskrit transliteration tool")]
#[command(version = "2.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Transliterate text between schemes
    Text {
        /// Input text to transliterate
        #[arg(short, long)]
        input: String,
        
        /// Source scheme (devanagari, iast, slp1, iso15919)
        #[arg(short, long, default_value = "devanagari")]
        from: String,
        
        /// Target scheme (devanagari, iast, slp1, iso15919)
        #[arg(short, long, default_value = "iast")]
        to: String,
        
        /// Test round-trip accuracy
        #[arg(long)]
        round_trip: bool,
    },
    
    /// List available schemes
    List,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Text { input, from, to, round_trip } => {
            handle_text_command(&input, &from, &to, round_trip)?;
        }
        Commands::List => {
            handle_list_command();
        }
    }
    
    Ok(())
}

fn handle_text_command(
    input: &str, 
    from_str: &str, 
    to_str: &str, 
    round_trip: bool
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "Bidirectional Sanskrit Transliteration".bright_cyan().bold());
    println!("{}", "======================================".bright_cyan());
    
    let from_scheme = parse_scheme(from_str)?;
    let to_scheme = parse_scheme(to_str)?;
    
    println!("Input:  {}", input.bright_white());
    println!("From:   {}", from_str.bright_yellow());
    println!("To:     {}", to_str.bright_green());
    println!();
    
    // Forward transliteration
    let start = Instant::now();
    let result = transliterate(input, from_scheme, to_scheme)?;
    let elapsed = start.elapsed();
    
    println!("{}", "Result:".bright_cyan().bold());
    println!("{}", result.text.bright_white().bold());
    println!();
    println!("Confidence: {:.1}%", (result.confidence * 100.0).to_string().bright_green());
    println!("Time: {:.2}ms", elapsed.as_secs_f64() * 1000.0);
    
    if round_trip {
        println!();
        println!("{}", "Round-trip Test:".bright_cyan().bold());
        
        // Reverse transliteration
        let reverse_start = Instant::now();
        match transliterate(&result.text, to_scheme, from_scheme) {
            Ok(reverse_result) => {
                let reverse_elapsed = reverse_start.elapsed();
                
                println!("Reverse:    {}", reverse_result.text.bright_white());
                
                let accuracy = calculate_accuracy(input, &reverse_result.text);
                let status = if accuracy > 0.99 {
                    "Perfect".bright_green()
                } else if accuracy > 0.95 {
                    "Excellent".bright_yellow()
                } else if accuracy > 0.85 {
                    "Good".bright_yellow()
                } else {
                    "Poor".bright_red()
                };
                
                println!("Accuracy:   {:.1}% ({})", (accuracy * 100.0), status);
                println!("Round-trip time: {:.2}ms", reverse_elapsed.as_secs_f64() * 1000.0);
                
                if accuracy < 1.0 {
                    println!();
                    println!("{}", "Character differences:".bright_yellow());
                    show_differences(input, &reverse_result.text);
                }
            }
            Err(e) => {
                println!("{}", format!("✗ Round-trip failed: {}", e).bright_red());
            }
        }
    }
    
    Ok(())
}

fn handle_list_command() {
    println!("{}", "Available Schemes".bright_cyan().bold());
    println!("{}", "=================".bright_cyan());
    
    println!();
    println!("{}", "Supported Scripts:".bright_yellow().bold());
    println!("  • {}  - Devanagari script (क, ख, ग, ...)", "devanagari".bright_white());
    println!("  • {}       - International Alphabet of Sanskrit Transliteration", "iast".bright_white());
    println!("  • {}       - Sanskrit Library Phonetic Basic", "slp1".bright_white());
    println!("  • {}   - ISO 15919 transliteration", "iso15919".bright_white());
    
    println!();
    println!("{}", "Examples:".bright_yellow().bold());
    println!("  # Convert Devanagari to IAST");
    println!("  {} text --input \"अग्निमीळे\" --from devanagari --to iast", "vedic_transliterator".bright_cyan());
    println!();
    println!("  # Test round-trip accuracy");
    println!("  {} text --input \"agnimīḷe\" --from iast --to devanagari --round-trip", "vedic_transliterator".bright_cyan());
}

fn parse_scheme(scheme_str: &str) -> Result<TargetScheme, TransliterationError> {
    match scheme_str.to_lowercase().as_str() {
        "devanagari" => Ok(TargetScheme::Devanagari),
        "iast" => Ok(TargetScheme::Iast),
        "slp1" => Ok(TargetScheme::Slp1),
        "iso15919" => Ok(TargetScheme::Iso15919),
        _ => Err(TransliterationError::UnsupportedScheme(scheme_str.to_string())),
    }
}

fn calculate_accuracy(original: &str, result: &str) -> f64 {
    if original == result {
        return 1.0;
    }
    
    let orig_chars: Vec<char> = original.chars().collect();
    let result_chars: Vec<char> = result.chars().collect();
    
    let max_len = orig_chars.len().max(result_chars.len());
    if max_len == 0 {
        return 1.0;
    }
    
    let matches = orig_chars.iter()
        .zip(result_chars.iter())
        .filter(|(a, b)| a == b)
        .count();
    
    matches as f64 / max_len as f64
}

fn show_differences(original: &str, result: &str) {
    let orig_chars: Vec<char> = original.chars().collect();
    let result_chars: Vec<char> = result.chars().collect();
    
    let max_len = orig_chars.len().max(result_chars.len());
    
    for i in 0..max_len {
        let orig_char = orig_chars.get(i).map(|c| c.to_string()).unwrap_or("∅".to_string());
        let result_char = result_chars.get(i).map(|c| c.to_string()).unwrap_or("∅".to_string());
        
        if orig_char != result_char {
            println!("  Position {}: {} → {}", 
                i.to_string().bright_white(),
                orig_char.bright_red(),
                result_char.bright_green()
            );
        }
    }
}