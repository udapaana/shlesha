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
    /// Debug available scripts
    Debug,
    /// Debug test conversion
    DebugTest,
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
        /// Show detailed metadata breakdown
        #[arg(short, long)]
        verbose: bool,
    },
    /// List supported scripts
    Scripts,
}

fn main() {
    let cli = Cli::parse();
    let transliterator = Shlesha::new();

    match cli.command {
        Commands::Transliterate {
            from,
            to,
            text,
            verbose,
        } => {
            // Get input text
            let input = match text {
                Some(t) => t,
                None => {
                    use std::io::Read;
                    let mut buffer = String::new();
                    std::io::stdin()
                        .read_to_string(&mut buffer)
                        .expect("Failed to read from stdin");
                    buffer.trim().to_string()
                }
            };

            // Perform transliteration with or without metadata
            if verbose {
                match transliterator.transliterate_with_metadata(&input, &from, &to) {
                    Ok(result) => {
                        // Detailed metadata output
                        println!("{}", result.output);
                        if let Some(metadata) = result.metadata {
                            println!("\nMetadata:");
                            println!(
                                "  Source: {} -> Target: {}",
                                metadata.source_script, metadata.target_script
                            );
                            println!("  Extensions used: {}", metadata.used_extensions);
                            if !metadata.unknown_tokens.is_empty() {
                                println!("  Unknown tokens: {}", metadata.unknown_tokens.len());
                                for (i, token) in metadata.unknown_tokens.iter().enumerate() {
                                    println!(
                                        "    {}. '{}' at position {} ({})",
                                        i + 1,
                                        token.token,
                                        token.position,
                                        token.unicode
                                    );
                                }
                            } else {
                                println!("  Unknown tokens: 0");
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {e}");
                        std::process::exit(1);
                    }
                }
            } else {
                // Regular transliteration without metadata
                match transliterator.transliterate(&input, &from, &to) {
                    Ok(result) => println!("{result}"),
                    Err(e) => {
                        eprintln!("Error: {e}");
                        std::process::exit(1);
                    }
                }
            }
        }

        Commands::DebugTest => {
            let transliterator = Shlesha::new();

            // Test the API that's failing
            let result = transliterator
                .transliterate_with_metadata("अ", "devanagari", "iast")
                .unwrap();

            println!("Output: '{}'", result.output);
            println!("Expected: 'a'");

            // Also test direct conversion
            let simple_result = transliterator
                .transliterate("अ", "devanagari", "iast")
                .unwrap();

            println!("Simple result: '{}'", simple_result);
        }

        Commands::Debug => {
            use shlesha::modules::script_converter::ScriptConverterRegistry;
            let registry = ScriptConverterRegistry::new_with_all_converters();
            let scripts = registry.list_supported_scripts();

            println!("Available scripts:");
            for script in &scripts {
                println!("  - {}", script);
            }
            println!("Total: {} scripts", scripts.len());

            // Test specific scripts that tests are looking for
            println!("\nScript availability:");
            println!("  devanagari: {}", registry.supports_script("devanagari"));
            println!("  iso: {}", registry.supports_script("iso"));
            println!("  iso15919: {}", registry.supports_script("iso15919"));
            println!("  iast: {}", registry.supports_script("iast"));
        }

        Commands::Scripts => {
            println!("Currently supported scripts:");

            let scripts = transliterator.list_supported_scripts();
            for script in scripts {
                // Provide descriptions for known scripts
                let description = match script.as_str() {
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
                println!("  {script} - {description}");
            }
        }
    }
}
