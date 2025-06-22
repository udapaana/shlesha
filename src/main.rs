//! Simple CLI for Shlesha transliterator

use clap::{Parser, Subcommand};
use shlesha::Shlesha;

#[derive(Parser)]
#[command(name = "shlesha")]
#[command(about = "High-performance extensible transliteration", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Transliterate text from one script to another
    Transliterate {
        /// Source script (e.g., devanagari, iso)
        #[arg(short, long)]
        from: String,
        /// Target script (e.g., devanagari, iso)
        #[arg(short, long)]
        to: String,
        /// Text to transliterate (or read from stdin if not provided)
        text: Option<String>,
    },
    /// List supported scripts
    Scripts,
}

fn main() {
    let cli = Cli::parse();
    let transliterator = Shlesha::new();
    
    match cli.command {
        Commands::Transliterate { from, to, text } => {
            // Get input text
            let input = match text {
                Some(t) => t,
                None => {
                    use std::io::Read;
                    let mut buffer = String::new();
                    std::io::stdin().read_to_string(&mut buffer)
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
        
        Commands::Scripts => {
            println!("Currently supported scripts:");
            
            let scripts = transliterator.list_supported_scripts();
            for script in scripts {
                // Provide descriptions for known scripts
                let description = match script {
                    "iast" => "IAST (International Alphabet of Sanskrit Transliteration)",
                    "itrans" => "ITRANS (ASCII transliteration)",
                    "slp1" => "SLP1 (Sanskrit Library Phonetic scheme)",
                    "harvard_kyoto" | "hk" => "Harvard-Kyoto (ASCII-based academic standard)",
                    "velthuis" => "Velthuis (TeX-based notation)",
                    "wx" => "WX (Computational notation)",
                    "devanagari" | "deva" => "Devanagari script (देवनागरी)",
                    "bengali" | "bn" => "Bengali script (বাংলা)",
                    "tamil" | "ta" => "Tamil script (தமிழ்)",
                    "telugu" | "te" => "Telugu script (తెలుగు)",
                    "gujarati" | "gu" => "Gujarati script (ગુજરાતી)",
                    "kannada" | "kn" => "Kannada script (ಕನ್ನಡ)",
                    "malayalam" | "ml" => "Malayalam script (മലയാളം)",
                    "odia" | "od" | "oriya" => "Odia script (ଓଡ଼ିଆ)",
                    "iso15919" | "iso" | "iso_15919" => "ISO-15919 (International standard)",
                    "bangla" => "Bengali script (বাংলা)",
                    "wx_notation" => "WX (Computational notation)",
                    _ => "Unknown script type",
                };
                println!("  {} - {}", script, description);
            }
        }
    }
}