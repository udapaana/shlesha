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
        let dummy_input = HubFormat::AlphabetTokens(vec![]);
        self.from_hub(script, &dummy_input).is_ok()
    }
}

/// Trait for token-based converters (generated from schemas)
pub trait TokenConverter: Send + Sync {
    /// Convert string to tokens
    fn string_to_tokens(&self, input: &str) -> HubTokenSequence;

    /// Convert tokens to string
    fn tokens_to_string(&self, tokens: &HubTokenSequence) -> String;

    /// Get the script name this converter handles
    fn script_name(&self) -> &'static str;

    /// Get whether this converter handles alphabet tokens (Roman) or abugida tokens (Indic)
    fn is_alphabet(&self) -> bool;
}

/// Registry for token-based converters
#[derive(Default)]
pub struct TokenConverterRegistry {
    converters: Vec<Box<dyn TokenConverter>>,
    /// Cache mapping script names to converter indices for O(1) lookup
    script_to_converter: FxHashMap<String, usize>,
}

impl TokenConverterRegistry {
    pub fn new() -> Self {
        Self {
            converters: Vec::new(),
            script_to_converter: FxHashMap::default(),
        }
    }

    pub fn register_converter(&mut self, converter: Box<dyn TokenConverter>) {
        let converter_index = self.converters.len();
        let script_name = converter.script_name().to_string();

        self.script_to_converter
            .insert(script_name, converter_index);
        self.converters.push(converter);
    }

    pub fn register_converter_with_aliases(
        &mut self,
        converter: Box<dyn TokenConverter>,
        aliases: &[&str],
    ) {
        let converter_index = self.converters.len();
        let script_name = converter.script_name().to_string();

        // Register primary script name
        self.script_to_converter
            .insert(script_name, converter_index);

        // Register all aliases
        for alias in aliases {
            self.script_to_converter
                .insert(alias.to_string(), converter_index);
        }

        self.converters.push(converter);
    }

    pub fn convert_to_tokens(
        &self,
        script: &str,
        input: &str,
    ) -> Result<HubTokenSequence, ConverterError> {
        // Try direct script name first
        if let Some(&converter_index) = self.script_to_converter.get(script) {
            let tokens = self.converters[converter_index].string_to_tokens(input);
            return Ok(tokens);
        }

        Err(ConverterError::ConversionFailed {
            script: script.to_string(),
            reason: format!("No token converter found for script: {}", script),
        })
    }

    pub fn convert_from_tokens(
        &self,
        script: &str,
        tokens: &HubTokenSequence,
    ) -> Result<String, ConverterError> {
        // Try direct script name first
        if let Some(&converter_index) = self.script_to_converter.get(script) {
            let output = self.converters[converter_index].tokens_to_string(tokens);
            return Ok(output);
        }

        Err(ConverterError::ConversionFailed {
            script: script.to_string(),
            reason: format!("No token converter found for script: {}", script),
        })
    }

    pub fn supports_script(&self, script: &str) -> bool {
        self.script_to_converter.contains_key(script)
    }

    pub fn list_supported_scripts(&self) -> Vec<String> {
        self.script_to_converter.keys().cloned().collect()
    }

    pub fn is_alphabet_script(&self, script: &str) -> bool {
        self.script_to_converter
            .get(script)
            .map(|&idx| self.converters[idx].is_alphabet())
            .unwrap_or(false)
    }
}

/// Registry for script converters
pub struct ScriptConverterRegistry {
    converters: Vec<Box<dyn ScriptConverter>>,
    /// Cache mapping script names to converter indices for O(1) lookup
    script_to_converter: FxHashMap<String, usize>,
    /// Token-based converter registry
    token_converters: TokenConverterRegistry,
}

impl ScriptConverterRegistry {
    pub fn new() -> Self {
        Self {
            converters: Vec::new(),
            script_to_converter: FxHashMap::default(),
            token_converters: TokenConverterRegistry::new(),
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
        // Resolve script aliases using schema registry
        let resolved_script = if let Some(registry) = schema_registry {
            if let Some(schema) = registry.find_schema_by_alias(script) {
                &schema.name
            } else {
                script
            }
        } else {
            script
        };

        // Try token-based converters first
        if self.token_converters.supports_script(resolved_script) {
            let tokens = self
                .token_converters
                .convert_to_tokens(resolved_script, input)?;

            // Convert tokens to appropriate hub format
            let hub_format = if self.token_converters.is_alphabet_script(resolved_script) {
                HubFormat::AlphabetTokens(tokens)
            } else {
                HubFormat::AbugidaTokens(tokens)
            };

            return Ok(hub_format);
        }

        // Resolve aliases first (including schema registry aliases)
        let canonical_script = self.resolve_script_alias_with_registry(script, schema_registry);

        // Fast lookup using HashMap cache instead of linear search
        if let Some(&converter_index) = self.script_to_converter.get(&canonical_script) {
            return self.converters[converter_index].to_hub(&canonical_script, input);
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
        // Resolve script aliases using schema registry
        let resolved_script = if let Some(registry) = schema_registry {
            if let Some(schema) = registry.find_schema_by_alias(script) {
                &schema.name
            } else {
                script
            }
        } else {
            script
        };

        // Try token-based converters first
        if self.token_converters.supports_script(resolved_script) {
            // Extract tokens from hub format
            let tokens = match hub_input {
                HubFormat::AlphabetTokens(tokens) => tokens,
                HubFormat::AbugidaTokens(tokens) => tokens,
            };

            // Convert tokens to string
            let result = self
                .token_converters
                .convert_from_tokens(resolved_script, tokens)?;
            return Ok(result);
        }

        // Resolve aliases first (including schema registry aliases)
        let canonical_script = self.resolve_script_alias_with_registry(script, schema_registry);

        // Fast lookup using HashMap cache instead of linear search
        if let Some(&converter_index) = self.script_to_converter.get(&canonical_script) {
            return self.converters[converter_index].from_hub(&canonical_script, hub_input);
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
        // Try token-based converters first
        if self.token_converters.supports_script(script) {
            let tokens = self.token_converters.convert_to_tokens(script, input)?;

            // Convert tokens to appropriate hub format
            let hub_format = if self.token_converters.is_alphabet_script(script) {
                HubFormat::AlphabetTokens(tokens)
            } else {
                HubFormat::AbugidaTokens(tokens)
            };

            // Create basic metadata for script → hub conversion
            let metadata = TransliterationMetadata::new(script, script);

            return Ok((hub_format, metadata));
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
        // Try token-based converters first
        if self.token_converters.supports_script(script) {
            // Extract tokens from hub format
            let tokens = match hub_input {
                HubFormat::AlphabetTokens(tokens) => tokens,
                HubFormat::AbugidaTokens(tokens) => tokens,
            };

            // Convert tokens to string
            let result = self.token_converters.convert_from_tokens(script, tokens)?;

            // Create basic metadata for hub → script conversion
            let metadata = TransliterationMetadata::new(script, script);

            return Ok(TransliterationResult {
                output: result,
                metadata: Some(metadata),
            });
        }

        // Resolve aliases first (hardcoded only, no schema registry available here)
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
        self.supports_script_with_registry(script, None)
    }

    /// Check if a script is supported by any converter (with schema registry for alias resolution)
    pub fn supports_script_with_registry(
        &self,
        script: &str,
        schema_registry: Option<&crate::modules::registry::SchemaRegistry>,
    ) -> bool {
        // Special case: Devanagari is always supported (hub format)
        if script.to_lowercase() == "devanagari" || script.to_lowercase() == "deva" {
            return true;
        }

        // Check direct script name first
        if self.script_to_converter.contains_key(script)
            || self.token_converters.supports_script(script)
        {
            return true;
        }

        // Resolve script aliases using schema registry
        let resolved_script = if let Some(registry) = schema_registry {
            if let Some(schema) = registry.find_schema_by_alias(script) {
                &schema.name
            } else {
                // Fallback to hardcoded aliases for built-in converters
                self.resolve_script_alias(script)
            }
        } else {
            // No schema registry available, use hardcoded aliases only
            self.resolve_script_alias(script)
        };

        // Check resolved script name
        if resolved_script != script {
            self.script_to_converter.contains_key(resolved_script)
                || self.token_converters.supports_script(resolved_script)
        } else {
            false
        }
    }

    /// Resolve script aliases to canonical script names
    fn resolve_script_alias<'a>(&self, script: &'a str) -> &'a str {
        // Check hardcoded aliases first for built-in converters
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
            "iso" => "iso15919",
            _ => script,
        }
    }

    /// Resolve script aliases using schema registry
    fn resolve_script_alias_with_registry(
        &self,
        script: &str,
        schema_registry: Option<&crate::modules::registry::SchemaRegistry>,
    ) -> String {
        // First try hardcoded aliases
        let resolved = self.resolve_script_alias(script);
        if resolved != script {
            return resolved.to_string();
        }

        // If no hardcoded alias found and we have a schema registry, check for schema aliases
        if let Some(registry) = schema_registry {
            if let Some(schema) = registry.find_schema_by_alias(script) {
                return schema.name.clone();
            }
        }

        script.to_string()
    }

    /// Get all supported scripts across all converters
    pub fn list_supported_scripts(&self) -> Vec<String> {
        let mut scripts: Vec<String> = self.script_to_converter.keys().cloned().collect();

        // Add token-based converter scripts
        let token_scripts = self.token_converters.list_supported_scripts();
        scripts.extend(token_scripts);

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

        // Resolve aliases first (hardcoded only, no schema registry available here)
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

        // Resolve aliases first (hardcoded only, no schema registry available here)
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

        // Register token-based converters with their aliases from schemas
        for (converter, aliases) in register_token_converters_with_aliases() {
            if aliases.is_empty() {
                registry.token_converters.register_converter(converter);
            } else {
                let alias_refs: Vec<&str> = aliases.iter().map(|s| s.as_str()).collect();
                registry
                    .token_converters
                    .register_converter_with_aliases(converter, &alias_refs);
            }
        }

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

        // Resolve aliases first (hardcoded only, no schema registry available here)
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

// Include generated schema-based converters
include!(concat!(env!("OUT_DIR"), "/schema_generated.rs"));

// All script converters are now schema-generated from YAML schemas in the schemas/ directory

// Re-export commonly used types (primary interface)
pub use ScriptConverterRegistry as ConverterRegistry; // Main interface for callers
                                                      // Note: ScriptConverter, ConverterError already public in this module

// Re-export individual converters (for advanced usage)
// Schema-generated converters are automatically available (no re-export needed)

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
        let registry = Arc::new(ScriptConverterRegistry::default());

        let registry_clone = Arc::clone(&registry);
        let handle = thread::spawn(move || {
            let scripts = registry_clone.list_supported_scripts();
            assert!(scripts.contains(&"devanagari".to_string()));
        });

        handle.join().unwrap();
    }
}
