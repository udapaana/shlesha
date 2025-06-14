//! Bidirectional Sanskrit Transliteration Engine
//! 
//! Token-based compiler architecture for perfect round-trip transliteration
//! between any supported scripts (Devanagari, IAST, SLP1, etc.)

mod tokens;
mod compiler;

pub use tokens::*;
pub use compiler::*;

/// Supported target schemes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetScheme {
    Devanagari,
    Iast,
    Slp1,
    Iso15919,
}

impl std::fmt::Display for TargetScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetScheme::Devanagari => write!(f, "devanagari"),
            TargetScheme::Iast => write!(f, "iast"),
            TargetScheme::Slp1 => write!(f, "slp1"),
            TargetScheme::Iso15919 => write!(f, "iso15919"),
        }
    }
}

/// Main transliteration result
#[derive(Debug, Clone)]
pub struct TransliterationResult {
    pub text: String,
    pub confidence: f64,
}

/// Transliteration errors
#[derive(Debug, thiserror::Error)]
pub enum TransliterationError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Unsupported scheme: {0}")]
    UnsupportedScheme(String),
    #[error("Processing error: {0}")]
    ProcessingError(String),
}

/// High-level transliteration function
pub fn transliterate(
    text: &str,
    from_scheme: TargetScheme,
    to_scheme: TargetScheme,
) -> Result<TransliterationResult, TransliterationError> {
    let mut compiler = TransliterationCompiler::new();
    compiler.load_builtin_schemes()?;
    
    // Parse input to tokens (IR)
    let tokens = compiler.parse(text, from_scheme)?;
    
    // Render tokens to output
    let output = compiler.render(&tokens, to_scheme)?;
    
    Ok(TransliterationResult {
        text: output,
        confidence: 1.0, // TODO: Calculate based on unknown tokens
    })
}