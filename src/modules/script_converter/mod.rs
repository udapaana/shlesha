use thiserror::Error;
use crate::modules::hub::{HubInput, HubError};

// Script Converter Module
//
// This module handles conversion from various scripts to the hub format (ISO-15919).
// 
// # Script Classification
// 
// Scripts are classified into two major categories based on how they handle consonants:
// 
// ## Scripts with Implicit 'a' (Indic Scripts)
// These scripts have consonants that inherently contain the 'a' vowel:
// - Devanagari (देवनागरी)
// - Bengali (বাংলা)  
// - Gujarati (ગુજરાતী)
// - Tamil (தமிழ்)
// - Telugu (తెలుగు)
// - Kannada (ಕನ್ನಡ)
// - Malayalam (മലയാളം)
// - Odia (ଓଡ଼ିଆ)
// - Gurmukhi (ਗੁਰਮੁਖੀ)
// 
// In these scripts, the consonant क inherently represents "ka", and requires 
// a virama (्) to suppress the vowel: क्.
// 
// ## Scripts without Implicit 'a' (Romanization Schemes)
// These scripts explicitly represent consonants without vowels:
// - ITRANS (ASCII transliteration)
// - SLP1 (Sanskrit Library Phonetic scheme)
// - IAST (International Alphabet of Sanskrit Transliteration)
// - ISO-15919 (International standard)
// - Harvard-Kyoto
// - Velthuis
// - WX notation
// 
// In these schemes, "k" represents just the consonant sound, and vowels 
// must be explicitly written: "ka", "ki", "ku", etc.

#[derive(Error, Debug, Clone)]
pub enum ConverterError {
    #[error("Invalid input for script {script}: {message}")]
    InvalidInput { script: String, message: String },
    #[error("Mapping not found for script {script}: {token}")]
    MappingNotFound { script: String, token: String },
    #[error("Conversion failed for script {script}: {reason}")]
    ConversionFailed { script: String, reason: String },
    #[error("Hub error: {0}")]
    HubError(#[from] HubError),
}

/// Core trait for converting from various scripts to hub format (ISO-15919)
pub trait ScriptConverter {
    /// Convert text from a specific script to hub input format
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError>;
    
    /// Get the list of supported scripts for this converter
    fn supported_scripts(&self) -> Vec<&'static str>;
    
    /// Check if this converter supports a specific script
    fn supports_script(&self, script: &str) -> bool {
        self.supported_scripts().contains(&script)
    }
    
    /// Check if the script has implicit 'a' vowel in consonants
    /// 
    /// Returns true for Indic scripts (Devanagari, Bengali, etc.) where consonants
    /// inherently contain the 'a' vowel and need explicit marks to suppress it.
    /// 
    /// Returns false for romanization schemes (ITRANS, SLP1, IAST, ISO-15919)
    /// where consonants are explicitly written without vowels.
    fn script_has_implicit_a(&self, script: &str) -> bool;
}

/// Registry for script converters
pub struct ScriptConverterRegistry {
    converters: Vec<Box<dyn ScriptConverter>>,
}

impl ScriptConverterRegistry {
    pub fn new() -> Self {
        Self {
            converters: Vec::new(),
        }
    }
    
    /// Register a script converter
    pub fn register_converter(&mut self, converter: Box<dyn ScriptConverter>) {
        self.converters.push(converter);
    }
    
    /// Convert text from any supported script to hub format
    pub fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        for converter in &self.converters {
            if converter.supports_script(script) {
                return converter.to_hub(script, input);
            }
        }
        
        Err(ConverterError::ConversionFailed {
            script: script.to_string(),
            reason: "No converter found for script".to_string(),
        })
    }
    
    /// Get all supported scripts across all converters
    pub fn supported_scripts(&self) -> Vec<&str> {
        let mut scripts = Vec::new();
        for converter in &self.converters {
            scripts.extend(converter.supported_scripts());
        }
        scripts.sort();
        scripts.dedup();
        scripts
    }
}

impl Default for ScriptConverterRegistry {
    fn default() -> Self {
        Self::new_with_all_converters()
    }
}

impl ScriptConverterRegistry {
    /// Create a new registry with all available converters pre-registered
    /// This is the recommended way for most users
    pub fn new_with_all_converters() -> Self {
        let mut registry = Self::new();
        
        // Register all available romanization scheme converters
        registry.register_converter(Box::new(IASTConverter::new()));
        registry.register_converter(Box::new(ITRANSConverter::new()));
        registry.register_converter(Box::new(SLP1Converter::new()));
        registry.register_converter(Box::new(HarvardKyotoConverter::new()));
        registry.register_converter(Box::new(VelthuisConverter::new()));
        registry.register_converter(Box::new(WXConverter::new()));
        
        // Register Indic script converters
        registry.register_converter(Box::new(DevanagariConverter::new()));
        registry.register_converter(Box::new(BengaliConverter::new()));
        
        // Register ISO-15919 hub format converter
        registry.register_converter(Box::new(ISO15919Converter::new()));
        
        // Register additional Indic script converters
        registry.register_converter(Box::new(TamilConverter::new()));
        registry.register_converter(Box::new(TeluguConverter::new()));
        
        registry
    }
    
    /// Get all supported script names across all registered converters
    pub fn list_supported_scripts(&self) -> Vec<&str> {
        self.supported_scripts()
    }
    
    /// Check if a script is supported by any registered converter
    pub fn supports_script(&self, script: &str) -> bool {
        self.supported_scripts().contains(&script)
    }
    
    /// Convert text from any supported script to hub format
    /// Returns an error if the script is not supported
    pub fn convert_to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        self.to_hub(script, input)
    }
    
    /// Get information about whether a script has implicit vowels
    pub fn script_has_implicit_vowels(&self, script: &str) -> Result<bool, ConverterError> {
        for converter in &self.converters {
            if converter.supports_script(script) {
                return Ok(converter.script_has_implicit_a(script));
            }
        }
        
        Err(ConverterError::ConversionFailed {
            script: script.to_string(),
            reason: "Script not supported".to_string(),
        })
    }
}

// Submodules for specific script converters
pub mod iast;
pub mod itrans;
pub mod slp1;
pub mod harvard_kyoto;
pub mod velthuis;
pub mod wx;
pub mod devanagari;
pub mod bengali;
pub mod iso15919;
pub mod tamil;
pub mod telugu;

// Integration tests
#[cfg(test)]
mod integration_tests;

// Correctness tests
#[cfg(test)]
mod correctness_tests;

// Re-export commonly used types (primary interface)  
pub use ScriptConverterRegistry as ConverterRegistry;  // Main interface for callers
// Note: ScriptConverter, ConverterError already public in this module

// Re-export individual converters (for advanced usage)
pub use iast::IASTConverter;
pub use itrans::ITRANSConverter;
pub use slp1::SLP1Converter;
pub use harvard_kyoto::HarvardKyotoConverter;
pub use velthuis::VelthuisConverter;
pub use wx::WXConverter;
pub use devanagari::DevanagariConverter;
pub use bengali::BengaliConverter;
pub use iso15919::ISO15919Converter;
pub use tamil::TamilConverter;
pub use telugu::TeluguConverter;