use thiserror::Error;
use std::collections::HashMap;
use crate::modules::hub::{HubInput, HubError};
use crate::modules::core::unknown_handler::{TransliterationResult, TransliterationMetadata};

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
// - Sinhala (සිංහල)
// - Grantha (𑌗𑍍𑌰𑌨𑍍𑌥)
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
// - Kolkata/Calcutta scheme
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
    
    /// Convert text from hub format to a specific script (reverse conversion)
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        // Default implementation for converters that don't support reverse conversion
        Err(ConverterError::ConversionFailed {
            script: script.to_string(),
            reason: "Reverse conversion not supported by this converter".to_string(),
        })
    }
    
    /// Convert text with metadata collection for unknown tokens
    fn to_hub_with_metadata(&self, script: &str, input: &str) -> Result<(HubInput, TransliterationMetadata), ConverterError> {
        // Default implementation - just call regular to_hub and return empty metadata
        let hub_input = self.to_hub(script, input)?;
        let metadata = TransliterationMetadata::new(script, "hub");
        Ok((hub_input, metadata))
    }
    
    /// Convert from hub with metadata collection for unknown tokens
    fn from_hub_with_metadata(&self, script: &str, hub_input: &HubInput) -> Result<TransliterationResult, ConverterError> {
        // Default implementation - just call regular from_hub and return simple result
        let output = self.from_hub(script, hub_input)?;
        Ok(TransliterationResult::simple(output))
    }
    
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
    /// Cache mapping script names to converter indices for O(1) lookup
    script_to_converter: HashMap<String, usize>,
}

impl ScriptConverterRegistry {
    pub fn new() -> Self {
        Self {
            converters: Vec::new(),
            script_to_converter: HashMap::new(),
        }
    }
    
    /// Register a script converter
    pub fn register_converter(&mut self, converter: Box<dyn ScriptConverter>) {
        let converter_index = self.converters.len();
        
        // Cache script mappings for fast lookup
        for script in converter.supported_scripts() {
            self.script_to_converter.insert(script.to_string(), converter_index);
        }
        
        self.converters.push(converter);
    }
    
    /// Convert text from any supported script to hub format
    pub fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        self.to_hub_with_schema_registry(script, input, None)
    }
    
    /// Convert text from any supported script to hub format with optional schema registry
    pub fn to_hub_with_schema_registry(&self, script: &str, input: &str, schema_registry: Option<&crate::modules::registry::SchemaRegistry>) -> Result<HubInput, ConverterError> {
        // Special case: if source is already Devanagari (hub format), return directly
        if script.to_lowercase() == "devanagari" || script.to_lowercase() == "deva" {
            return Ok(HubInput::Devanagari(input.to_string()));
        }
        
        // Resolve aliases first
        let canonical_script = self.resolve_script_alias(script);
        
        // Fast lookup using HashMap cache instead of linear search
        if let Some(&converter_index) = self.script_to_converter.get(canonical_script) {
            return self.converters[converter_index].to_hub(canonical_script, input);
        }
        
        // Fallback to schema-based converter for runtime-loaded scripts
        if let Some(registry) = schema_registry {
            let schema_converter = schema_based::SchemaBasedConverter::new(std::sync::Arc::new(registry.clone()));
            if schema_converter.supports_script(script) {
                return schema_converter.to_hub(script, input);
            }
        }
        
        Err(ConverterError::ConversionFailed {
            script: script.to_string(),
            reason: "No converter found for script".to_string(),
        })
    }
    
    /// Convert text from hub format to any supported script (reverse conversion)
    pub fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        self.from_hub_with_schema_registry(script, hub_input, None)
    }
    
    /// Convert text from hub format to any supported script with optional schema registry
    pub fn from_hub_with_schema_registry(&self, script: &str, hub_input: &HubInput, schema_registry: Option<&crate::modules::registry::SchemaRegistry>) -> Result<String, ConverterError> {
        // Special case: if target is Devanagari (hub format), return directly
        if script.to_lowercase() == "devanagari" || script.to_lowercase() == "deva" {
            match hub_input {
                HubInput::Devanagari(deva_text) => return Ok(deva_text.clone()),
                HubInput::Iso(_) => {
                    // Need to convert ISO to Devanagari via hub
                    // This will be handled by the calling code
                }
            }
        }
        
        // Fast lookup using HashMap cache instead of linear search
        if let Some(&converter_index) = self.script_to_converter.get(script) {
            return self.converters[converter_index].from_hub(script, hub_input);
        }
        
        // Fallback to schema-based converter for runtime-loaded scripts
        if let Some(registry) = schema_registry {
            let schema_converter = schema_based::SchemaBasedConverter::new(std::sync::Arc::new(registry.clone()));
            if schema_converter.supports_script(script) {
                return schema_converter.from_hub(script, hub_input);
            }
        }
        
        Err(ConverterError::ConversionFailed {
            script: script.to_string(),
            reason: "No converter found for script".to_string(),
        })
    }
    
    /// Convert text from any supported script to hub format with metadata collection
    pub fn to_hub_with_metadata(&self, script: &str, input: &str) -> Result<(HubInput, TransliterationMetadata), ConverterError> {
        // Special case: if source is already Devanagari (hub format), return directly
        if script.to_lowercase() == "devanagari" || script.to_lowercase() == "deva" {
            let metadata = TransliterationMetadata::new(script, "hub");
            return Ok((HubInput::Devanagari(input.to_string()), metadata));
        }
        
        // Fast lookup using HashMap cache instead of linear search
        if let Some(&converter_index) = self.script_to_converter.get(script) {
            return self.converters[converter_index].to_hub_with_metadata(script, input);
        }
        
        // The metadata methods would also need schema registry support
        // For now, keeping original error until we add schema support here too
        
        Err(ConverterError::ConversionFailed {
            script: script.to_string(),
            reason: "No converter found for script".to_string(),
        })
    }
    
    /// Convert text from hub format to any supported script with metadata collection
    pub fn from_hub_with_metadata(&self, script: &str, hub_input: &HubInput) -> Result<TransliterationResult, ConverterError> {
        // Special case: if target is Devanagari (hub format), return directly
        if script.to_lowercase() == "devanagari" || script.to_lowercase() == "deva" {
            match hub_input {
                HubInput::Devanagari(deva_text) => {
                    return Ok(TransliterationResult::simple(deva_text.clone()));
                },
                HubInput::Iso(_) => {
                    // Need to convert ISO to Devanagari via hub - this will be handled by calling code
                }
            }
        }
        
        // Fast lookup using HashMap cache instead of linear search
        if let Some(&converter_index) = self.script_to_converter.get(script) {
            return self.converters[converter_index].from_hub_with_metadata(script, hub_input);
        }
        
        // The metadata methods would also need schema registry support
        // For now, keeping original error until we add schema support here too
        
        Err(ConverterError::ConversionFailed {
            script: script.to_string(),
            reason: "No converter found for script".to_string(),
        })
    }
    
    /// Check if a script is supported by any converter
    pub fn supports_script(&self, script: &str) -> bool {
        // Special case: Devanagari is always supported (hub format)
        if script.to_lowercase() == "devanagari" || script.to_lowercase() == "deva" {
            return true;
        }
        
        // Check direct script name
        if self.script_to_converter.contains_key(script) {
            return true;
        }
        
        // Check common aliases
        let canonical_script = self.resolve_script_alias(script);
        self.script_to_converter.contains_key(canonical_script)
    }
    
    /// Resolve script aliases to canonical script names
    fn resolve_script_alias<'a>(&self, script: &'a str) -> &'a str {
        match script {
            "hk" => "harvard_kyoto",
            "bn" => "bengali", 
            "ta" => "tamil",
            "te" => "telugu",
            "gu" => "gujarati",
            "kn" => "kannada",
            "ml" => "malayalam",
            "or" => "odia",
            "pa" => "gurmukhi",
            "si" => "sinhala",
            "deva" => "devanagari",
            _ => script,
        }
    }
    
    /// Get all supported scripts across all converters
    pub fn list_supported_scripts(&self) -> Vec<&str> {
        let mut scripts: Vec<&str> = self.script_to_converter.keys().map(|s| s.as_str()).collect();
        
        // Add Devanagari as it's always supported (hub format)
        scripts.push("devanagari");
        
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
        
        // Register all schema-generated converters (TOML/YAML based)
        register_schema_generated_converters(&mut registry);
        
        // Register remaining hand-coded converters that don't have schemas yet
        registry.register_converter(Box::new(IASTConverter::new()));
        registry.register_converter(Box::new(KolkataConverter::new()));
        registry.register_converter(Box::new(GranthaConverter::new()));
        
        // Register ISO-15919 hub format converter
        registry.register_converter(Box::new(ISO15919Converter::new()));
        
        registry
    }
    
    /// Convert text from any supported script to hub format
    /// Returns an error if the script is not supported
    pub fn convert_to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        self.to_hub(script, input)
    }
    
    /// Get information about whether a script has implicit vowels
    pub fn script_has_implicit_vowels(&self, script: &str) -> Result<bool, ConverterError> {
        // Special case: Devanagari (hub format) always has implicit 'a' vowels
        if script.to_lowercase() == "devanagari" || script.to_lowercase() == "deva" {
            return Ok(true);
        }
        
        // Resolve aliases first
        let canonical_script = self.resolve_script_alias(script);
        
        // Fast lookup using HashMap cache instead of linear search
        if let Some(&converter_index) = self.script_to_converter.get(canonical_script) {
            return Ok(self.converters[converter_index].script_has_implicit_a(canonical_script));
        }
        
        Err(ConverterError::ConversionFailed {
            script: script.to_string(),
            reason: "Script not supported".to_string(),
        })
    }
}

// Submodules for specific script converters
// Shared processing logic
pub mod processors;
// Schema-based converter for runtime-loaded scripts
pub mod schema_based;

// Include generated schema-based converters
include!(concat!(env!("OUT_DIR"), "/schema_generated.rs"));

// Script converters
pub mod iast;
// Roman script converters replaced by schema-generated ones
// pub mod itrans;
// pub mod slp1;
// pub mod harvard_kyoto;
// pub mod velthuis;
// pub mod wx;
// Hand-coded converters replaced by schema-generated ones
// pub mod devanagari;
// pub mod bengali;
// pub mod tamil;
// pub mod telugu;
// pub mod optimized_telugu;
// pub mod gujarati;
// pub mod kannada;
// pub mod malayalam;
// pub mod odia;
// pub mod gurmukhi;
// pub mod sinhala;

pub mod iso15919;
pub mod kolkata;
pub mod grantha;

// Legacy optimized converters (replaced by schema-generated ones)
// pub mod slp1_optimized;

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
// Schema-generated converters are automatically available (no re-export needed)
// pub use itrans::ITRANSConverter;
// pub use slp1::SLP1Converter;  
// pub use harvard_kyoto::HarvardKyotoConverter;
// pub use velthuis::VelthuisConverter;
// pub use wx::WXConverter;
// pub use devanagari::DevanagariConverter;
// pub use bengali::BengaliConverter;
// pub use tamil::TamilConverter;
// pub use telugu::TeluguConverter;
// pub use optimized_telugu::OptimizedTeluguConverter;
// pub use slp1_optimized::OptimizedSLP1Converter;
// pub use gujarati::GujaratiConverter;
// pub use kannada::KannadaConverter;
// pub use malayalam::MalayalamConverter;
// pub use odia::OdiaConverter;
// pub use gurmukhi::GurmukhiConverter;
// pub use sinhala::SinhalaConverter;
pub use iso15919::ISO15919Converter;
pub use kolkata::KolkataConverter;
pub use grantha::GranthaConverter;

// TODO List for Script Converter Module:
// - [ ] Handle ambiguous mappings with superscripted numerals when:
//     - One character in source script maps to multiple characters in destination script
//     - Multiple characters in source script map to one character in destination script
//     - Example: Tamil ப could map to ப² (pha), ப³ (ba), or ப⁴ (bha) to disambiguate
//     - This would help preserve information in bidirectional conversions
// - [ ] Add support for Grantha script used for Sanskrit in Tamil Nadu
// - [ ] Add support for Sinhala script
// - [ ] Add support for Tibetan script
// - [ ] Add support for Thai/Lao scripts (for Sanskrit/Pali texts)
// - [ ] Implement contextual conversion rules for better accuracy
// - [ ] Add script-specific validation rules
// - [ ] Implement script detection for automatic source script identification