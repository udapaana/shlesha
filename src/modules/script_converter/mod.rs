use crate::modules::core::unknown_handler::{TransliterationMetadata, TransliterationResult};
use crate::modules::hub::{HubError, HubInput};
use rustc_hash::FxHashMap;
use thiserror::Error;

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

/// Statistics about converter capabilities
#[derive(Debug, Clone)]
pub struct ConverterStats {
    /// Total number of registered converters
    pub total_converters: usize,
    /// Total number of supported scripts (including aliases)
    pub total_scripts: usize,
    /// Number of scripts that support bidirectional conversion
    pub bidirectional_scripts: usize,
    /// Number of scripts with implicit 'a' vowels (Indic scripts)
    pub implicit_a_scripts: usize,
}

/// Core trait for converting from various scripts to hub format
pub trait ScriptConverter: Send + Sync {
    /// Convert text from a specific script to hub input format
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError>;

    /// Convert text from hub format to a specific script (reverse conversion)
    #[allow(clippy::wrong_self_convention)]
    fn from_hub(&self, script: &str, _hub_input: &HubInput) -> Result<String, ConverterError> {
        // Default implementation for converters that don't support reverse conversion
        Err(ConverterError::ConversionFailed {
            script: script.to_string(),
            reason: "Reverse conversion not supported by this converter".to_string(),
        })
    }

    /// Convert text with metadata collection for unknown tokens
    fn to_hub_with_metadata(
        &self,
        script: &str,
        input: &str,
    ) -> Result<(HubInput, TransliterationMetadata), ConverterError> {
        // Default implementation - just call regular to_hub and return empty metadata
        let hub_input = self.to_hub(script, input)?;
        let metadata = TransliterationMetadata::new(script, "hub");
        Ok((hub_input, metadata))
    }

    /// Convert from hub with metadata collection for unknown tokens
    #[allow(clippy::wrong_self_convention)]
    fn from_hub_with_metadata(
        &self,
        script: &str,
        hub_input: &HubInput,
    ) -> Result<TransliterationResult, ConverterError> {
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

    /// Check if this converter supports bidirectional conversion
    fn supports_reverse_conversion(&self, script: &str) -> bool {
        // Default implementation - try a dummy conversion to see if it errors
        use crate::modules::hub::HubFormat;
        let dummy_input = HubFormat::Iso("test".to_string());
        self.from_hub(script, &dummy_input).is_ok()
    }
}

/// Registry for script converters
pub struct ScriptConverterRegistry {
    converters: Vec<Box<dyn ScriptConverter>>,
    /// Cache mapping script names to converter indices for O(1) lookup
    script_to_converter: FxHashMap<String, usize>,
}

impl ScriptConverterRegistry {
    pub fn new() -> Self {
        Self {
            converters: Vec::new(),
            script_to_converter: FxHashMap::default(),
        }
    }

    /// Register a script converter
    pub fn register_converter(&mut self, converter: Box<dyn ScriptConverter>) {
        let converter_index = self.converters.len();

        // Cache script mappings for fast lookup
        for script in converter.supported_scripts() {
            self.script_to_converter
                .insert(script.to_string(), converter_index);
        }

        self.converters.push(converter);
    }

    /// Convert text from any supported script to hub format
    pub fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        self.to_hub_with_schema_registry(script, input, None)
    }

    /// Convert text from any supported script to hub format with optional schema registry
    pub fn to_hub_with_schema_registry(
        &self,
        script: &str,
        input: &str,
        schema_registry: Option<&crate::modules::registry::SchemaRegistry>,
    ) -> Result<HubInput, ConverterError> {
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
            let schema_converter =
                schema_based::SchemaBasedConverter::new(std::sync::Arc::new(registry.clone()));
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
    pub fn from_hub_with_schema_registry(
        &self,
        script: &str,
        hub_input: &HubInput,
        schema_registry: Option<&crate::modules::registry::SchemaRegistry>,
    ) -> Result<String, ConverterError> {
        // Special case: if target is Devanagari (hub format) and input is already Devanagari, return directly
        if script.to_lowercase() == "devanagari" || script.to_lowercase() == "deva" {
            if let HubInput::Devanagari(deva_text) = hub_input {
                return Ok(deva_text.clone());
            }
        }

        // Resolve aliases first
        let canonical_script = self.resolve_script_alias(script);

        // Fast lookup using HashMap cache instead of linear search
        if let Some(&converter_index) = self.script_to_converter.get(canonical_script) {
            return self.converters[converter_index].from_hub(canonical_script, hub_input);
        }

        // Fallback to schema-based converter for runtime-loaded scripts
        if let Some(registry) = schema_registry {
            let schema_converter =
                schema_based::SchemaBasedConverter::new(std::sync::Arc::new(registry.clone()));
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
    pub fn to_hub_with_metadata(
        &self,
        script: &str,
        input: &str,
    ) -> Result<(HubInput, TransliterationMetadata), ConverterError> {
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
    pub fn from_hub_with_metadata(
        &self,
        script: &str,
        hub_input: &HubInput,
    ) -> Result<TransliterationResult, ConverterError> {
        // Resolve aliases first
        let canonical_script = self.resolve_script_alias(script);

        // Fast lookup using HashMap cache instead of linear search
        if let Some(&converter_index) = self.script_to_converter.get(canonical_script) {
            return self.converters[converter_index]
                .from_hub_with_metadata(canonical_script, hub_input);
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
        let mut scripts: Vec<&str> = self
            .script_to_converter
            .keys()
            .map(|s| s.as_str())
            .collect();

        // Add Devanagari as it's always supported (hub format)
        scripts.push("devanagari");

        scripts.sort();
        scripts.dedup();
        scripts
    }

    /// Check if a converter supports bidirectional conversion for a specific script
    pub fn supports_reverse_conversion(&self, script: &str) -> bool {
        // Special case: Devanagari always supports reverse conversion (hub format)
        if script.to_lowercase() == "devanagari" || script.to_lowercase() == "deva" {
            return true;
        }

        // Resolve aliases first
        let canonical_script = self.resolve_script_alias(script);

        // Fast lookup using HashMap cache
        if let Some(&converter_index) = self.script_to_converter.get(canonical_script) {
            return self.converters[converter_index].supports_reverse_conversion(canonical_script);
        }

        false
    }

    /// Check if a script has implicit 'a' vowel in consonants
    pub fn script_has_implicit_a(&self, script: &str) -> bool {
        // Special case: Devanagari always has implicit 'a' vowels
        if script.to_lowercase() == "devanagari" || script.to_lowercase() == "deva" {
            return true;
        }

        // Resolve aliases first
        let canonical_script = self.resolve_script_alias(script);

        // Fast lookup using HashMap cache
        if let Some(&converter_index) = self.script_to_converter.get(canonical_script) {
            return self.converters[converter_index].script_has_implicit_a(canonical_script);
        }

        // Default to false for unknown scripts
        false
    }

    /// Get converter statistics and capabilities
    pub fn get_stats(&self) -> ConverterStats {
        let total_converters = self.converters.len();
        let total_scripts = self.list_supported_scripts().len();
        let bidirectional_scripts = self
            .list_supported_scripts()
            .iter()
            .filter(|script| self.supports_reverse_conversion(script))
            .count();
        let implicit_a_scripts = self
            .list_supported_scripts()
            .iter()
            .filter(|script| self.script_has_implicit_a(script))
            .count();

        ConverterStats {
            total_converters,
            total_scripts,
            bidirectional_scripts,
            implicit_a_scripts,
        }
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

// All script converters are now schema-generated
// Hand-coded converters (iast.rs, kolkata.rs, grantha.rs) have been migrated to schemas/
// The ISO-15919 converter is special as it's a passthrough converter (no schema needed)
pub mod iso15919;

// Legacy optimized converters (replaced by schema-generated ones)
// pub mod slp1_optimized;

// Integration tests
#[cfg(test)]
mod integration_tests;

// Correctness tests
#[cfg(test)]
mod correctness_tests;

// Re-export commonly used types (primary interface)
pub use ScriptConverterRegistry as ConverterRegistry; // Main interface for callers
                                                      // Note: ScriptConverter, ConverterError already public in this module

// Re-export individual converters (for advanced usage)
// Schema-generated converters are automatically available (no re-export needed)
pub use iso15919::ISO15919Converter; // Special passthrough converter

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

#[cfg(test)]
mod send_sync_tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_script_converter_send_sync() {
        // Test that ScriptConverter trait objects are Send + Sync
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        fn assert_send_sync<T: Send + Sync>() {}
        
        // Test Box<dyn ScriptConverter>
        assert_send::<Box<dyn ScriptConverter>>();
        assert_sync::<Box<dyn ScriptConverter>>();
        assert_send_sync::<Box<dyn ScriptConverter>>();
        
        // Test Arc<dyn ScriptConverter>
        assert_send::<Arc<dyn ScriptConverter>>();
        assert_sync::<Arc<dyn ScriptConverter>>();
        assert_send_sync::<Arc<dyn ScriptConverter>>();
    }

    #[test]
    fn test_script_converter_thread_safety() {
        // Test that we can actually use ScriptConverter in threads
        let converter: Arc<dyn ScriptConverter> = Arc::new(ISO15919Converter::new());
        
        // Clone for thread
        let converter_clone = Arc::clone(&converter);
        let handle = thread::spawn(move || {
            let scripts = converter_clone.supported_scripts();
            assert!(scripts.contains(&"iso15919"));
        });
        
        handle.join().unwrap();
    }

    #[test]
    fn test_script_converter_registry_send_sync() {
        // Test that ScriptConverterRegistry is Send + Sync
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        fn assert_send_sync<T: Send + Sync>() {}
        
        assert_send::<ScriptConverterRegistry>();
        assert_sync::<ScriptConverterRegistry>();
        assert_send_sync::<ScriptConverterRegistry>();
        
        // Test Arc<ScriptConverterRegistry>
        assert_send::<Arc<ScriptConverterRegistry>>();
        assert_sync::<Arc<ScriptConverterRegistry>>();
        assert_send_sync::<Arc<ScriptConverterRegistry>>();
    }

    #[test]
    fn test_registry_thread_safety() {
        // Test that we can actually use ScriptConverterRegistry in threads
        let registry = Arc::new(ScriptConverterRegistry::new());
        
        let registry_clone = Arc::clone(&registry);
        let handle = thread::spawn(move || {
            let scripts = registry_clone.list_supported_scripts();
            assert!(scripts.contains(&"devanagari"));
        });
        
        handle.join().unwrap();
    }
}
