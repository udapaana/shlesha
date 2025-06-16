use clap::{Parser, ValueEnum};
use shlesha::{Transliterator, TransliteratorBuilder};
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(Debug, Clone, ValueEnum)]
enum Script {
    Devanagari,
    IAST,
    HarvardKyoto,
    SLP1,
    ISO15919,
    Telugu,
}

impl Script {
    fn as_str(&self) -> &'static str {
        match self {
            Script::Devanagari => "Devanagari",
            Script::IAST => "IAST",
            Script::HarvardKyoto => "Harvard-Kyoto",
            Script::SLP1 => "SLP1",
            Script::ISO15919 => "ISO15919",
            Script::Telugu => "Telugu",
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "shlesha")]
#[command(about = "A high-performance transliterator for Indic scripts", long_about = None)]
struct Args {
    /// Source script
    #[arg(short = 'f', long = "from", value_enum)]
    from: Script,

    /// Target script
    #[arg(short = 't', long = "to", value_enum)]
    to: Script,

    /// Text to transliterate (if not provided, reads from file or stdin)
    text: Option<String>,

    /// Input file (stdin if not specified)
    #[arg(short = 'i', long = "input")]
    input: Option<PathBuf>,

    /// Output file (stdout if not specified)
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,

    /// Schema directory
    #[arg(short = 's', long = "schemas", default_value = "schemas")]
    schema_dir: PathBuf,

    /// Extensions to enable (can be specified multiple times)
    #[arg(short = 'e', long = "extension")]
    extensions: Vec<String>,

    /// Verbose output
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Build transliterator
    let mut builder = TransliteratorBuilder::new();
    
    // Load schemas from directory
    if args.verbose {
        eprintln!("Loading schemas from: {}", args.schema_dir.display());
    }
    
    builder = builder.with_schema_directory(&args.schema_dir)?;
    
    let mut transliterator = builder.build();
    
    // Add extensions
    for ext in &args.extensions {
        if args.verbose {
            eprintln!("Enabling extension: {}", ext);
        }
        transliterator.add_extension(ext)?;
    }

    // Read input
    let input_text = if let Some(text) = args.text {
        // Text provided as command line argument
        text
    } else if let Some(input_path) = args.input {
        if args.verbose {
            eprintln!("Reading from: {}", input_path.display());
        }
        std::fs::read_to_string(input_path)?
    } else {
        if args.verbose {
            eprintln!("Reading from stdin...");
        }
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    // Transliterate
    let start = std::time::Instant::now();
    let output_text = transliterator.transliterate(
        &input_text,
        args.from.as_str(),
        args.to.as_str()
    )?;
    let elapsed = start.elapsed();

    if args.verbose {
        eprintln!("Transliteration completed in {:?}", elapsed);
        eprintln!("Input: {} chars, Output: {} chars", 
                  input_text.len(), output_text.len());
        eprintln!("Throughput: {:.2} MB/s", 
                  input_text.len() as f64 / elapsed.as_secs_f64() / 1_048_576.0);
    }

    // Write output
    if let Some(output_path) = args.output {
        if args.verbose {
            eprintln!("Writing to: {}", output_path.display());
        }
        std::fs::write(output_path, output_text)?;
    } else {
        io::stdout().write_all(output_text.as_bytes())?;
        io::stdout().flush()?;
    }

    Ok(())
}