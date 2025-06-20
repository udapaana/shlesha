//! Simple CLI for Shlesha transliterator

use clap::{Parser, Subcommand};
use shlesha::LosslessTransliterator;
use std::io::{self, Read};

#[derive(Parser)]
#[command(name = "shlesha")]
#[command(about = "High-performance lossless transliteration", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Transliterate text from one script to another
    Transliterate {
        /// Source script (e.g., Devanagari, IAST, SLP1)
        #[arg(short, long)]
        from: String,
        
        /// Target script (e.g., Devanagari, IAST, SLP1)
        #[arg(short, long)]
        to: String,
        
        /// Text to transliterate (if not provided, reads from stdin)
        #[arg(value_name = "TEXT")]
        text: Option<String>,
    },
    
    /// Verify losslessness of a transliteration
    Verify {
        /// Original text
        #[arg(short, long)]
        original: String,
        
        /// Encoded text
        #[arg(short, long)]
        encoded: String,
        
        /// Source script
        #[arg(short, long)]
        from: String,
    },
    
    /// List supported scripts
    Scripts,
}

fn main() {
    let cli = Cli::parse();
    let transliterator = LosslessTransliterator::new();
    
    match cli.command {
        Commands::Transliterate { from, to, text } => {
            // Get input text
            let input = match text {
                Some(t) => t,
                None => {
                    let mut buffer = String::new();
                    io::stdin().read_to_string(&mut buffer)
                        .expect("Failed to read from stdin");
                    buffer.trim().to_string()
                }
            };
            
            // Perform transliteration
            match transliterator.transliterate(&input, &from, &to) {
                Ok(result) => println!("{}", result),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        
        Commands::Verify { original, encoded, from } => {
            let result = transliterator.verify_lossless(&original, &encoded, &from);
            
            println!("Lossless: {}", result.is_lossless);
            println!("Preservation ratio: {:.3}", result.preservation_ratio);
            println!("Tokens found: {}", result.tokens_count);
            println!("Entropy analysis:");
            println!("  Original: {:.3}", result.entropy_analysis.original);
            println!("  Encoded: {:.3}", result.entropy_analysis.encoded);
            println!("  Token preservation: {:.3}", result.entropy_analysis.token_preservation);
            println!("  Total preserved: {:.3}", result.entropy_analysis.total_preserved);
            
            if !result.is_lossless {
                std::process::exit(1);
            }
        }
        
        Commands::Scripts => {
            use shlesha::script_mappings::get_supported_scripts;
            
            println!("Supported scripts:");
            for (name, id) in get_supported_scripts() {
                println!("  {} (ID: {})", name, id);
            }
        }
    }
}