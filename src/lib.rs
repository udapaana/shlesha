//! # Shlesha: High-Performance Extensible Transliteration Library
//!
//! Shlesha is a next-generation transliteration library using a hub-and-spoke architecture
//! with Devanagari ↔ ISO-15919 as the central hub for maximum accuracy and runtime extensibility.
//!
//! ## Key Features
//!
//! - **Hub-and-Spoke Architecture**: All transliteration flows through proven Devanagari ↔ ISO-15919 mapping
//! - **Runtime Extensibility**: Add new scripts without recompilation via schema loading
//! - **Modular Design**: Clean separation of concerns with interface-based communication
//! - **High Performance**: Optimized string processing with caching
//!
//! ## Quick Start
//!
//! ```rust
//! use shlesha::Shlesha;
//!
//! let transliterator = Shlesha::new();
//! let result = transliterator
//!     .transliterate("धर्म", "devanagari", "iso")
//!     .unwrap();
//! println!("{}", result); // "dharma"
//! ```

pub mod modules;

// Optional binding modules
#[cfg(feature = "python")]
pub mod python_bindings;

#[cfg(feature = "wasm")]
pub mod wasm_bindings;

use modules::hub::{Hub, HubTrait, HubInput, HubOutput};
use modules::script_converter::ScriptConverterRegistry;
use modules::registry::{SchemaRegistry, SchemaRegistryTrait};

// Re-export unknown handler types for public API
pub use modules::core::unknown_handler::{
    TransliterationResult,
    TransliterationMetadata,
    UnknownToken,
};

/// Main transliterator struct implementing hub-and-spoke architecture
pub struct Shlesha {
    hub: Hub,
    script_converter_registry: ScriptConverterRegistry,
    registry: SchemaRegistry,
}

impl Shlesha {
    /// Create a new Shlesha transliterator instance
    pub fn new() -> Self {
        // Use the complete registry with all available converters
        let script_converter_registry = ScriptConverterRegistry::default();
        
        Self {
            hub: Hub::new(),
            script_converter_registry,
            registry: SchemaRegistry::new(),
        }
    }

    /// Transliterate text from one script to another via the central hub
    pub fn transliterate(&self, text: &str, from: &str, to: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Convert source script to hub format (Devanagari or ISO)
        let hub_input = self.script_converter_registry.to_hub(from, text)?;
        
        // Smart hub processing based on input and desired output
        let result = match (&hub_input, to.to_lowercase().as_str()) {
            // Direct passthrough cases - no hub processing needed
            (HubInput::Devanagari(deva), "devanagari" | "deva") => deva.clone(),
            (HubInput::Iso(iso), "iso" | "iso15919" | "iso-15919") => iso.clone(),
            
            // Hub processing needed - convert between formats
            (HubInput::Devanagari(deva), _) => {
                // Try direct Devanagari → target conversion first (for Indic scripts)
                let deva_hub_input = HubInput::Devanagari(deva.clone());
                match self.script_converter_registry.from_hub(to, &deva_hub_input) {
                    Ok(result) => result,
                    Err(_) => {
                        // If direct conversion fails, convert through ISO: Devanagari → ISO → target
                        let hub_output = self.hub.deva_to_iso(&deva)?;
                        if let HubOutput::Iso(ref iso_result) = hub_output {
                            let iso_hub_input = HubInput::Iso(iso_result.clone());
                            self.script_converter_registry.from_hub(to, &iso_hub_input)?
                        } else {
                            return Err("Expected ISO output from hub".into());
                        }
                    }
                }
            },
            (HubInput::Iso(iso), _) => {
                // Try direct ISO → target conversion first
                let iso_hub_input = HubInput::Iso(iso.clone());
                match self.script_converter_registry.from_hub(to, &iso_hub_input) {
                    Ok(result) => result,
                    Err(_) => {
                        // If direct conversion fails, convert through Devanagari: ISO → Devanagari → target
                        let hub_output = self.hub.iso_to_deva(&iso)?;
                        if let HubOutput::Devanagari(ref deva_result) = hub_output {
                            let deva_hub_input = HubInput::Devanagari(deva_result.clone());
                            self.script_converter_registry.from_hub(to, &deva_hub_input)?
                        } else {
                            return Err("Expected Devanagari output from hub".into());
                        }
                    }
                }
            },
        };
        
        Ok(result)
    }

    /// Load a new script schema at runtime
    pub fn load_schema(&mut self, schema_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(self.registry.load_schema(schema_path)?)
    }
    
    /// Get list of supported scripts
    pub fn list_supported_scripts(&self) -> Vec<&str> {
        self.script_converter_registry.list_supported_scripts()
    }
    
    /// Check if a script is supported
    pub fn supports_script(&self, script: &str) -> bool {
        self.script_converter_registry.supports_script(script)
    }
    
    /// Transliterate text with metadata collection for unknown tokens
    pub fn transliterate_with_metadata(&self, text: &str, from: &str, to: &str) 
        -> Result<crate::modules::core::unknown_handler::TransliterationResult, Box<dyn std::error::Error>> {
        use crate::modules::hub::HubTrait;
        
        // Convert source script to hub format with metadata collection
        let (hub_input, from_metadata) = self.script_converter_registry.to_hub_with_metadata(from, text)?;
        
        // Smart hub processing based on input and desired output - with metadata
        let (result, to_metadata) = match (&hub_input, to.to_lowercase().as_str()) {
            // Direct passthrough cases - no hub processing needed
            (modules::hub::HubInput::Devanagari(deva), "devanagari" | "deva") => {
                let result = modules::core::unknown_handler::TransliterationResult::simple(deva.clone());
                (result, None)
            },
            (modules::hub::HubInput::Iso(iso), "iso" | "iso15919" | "iso-15919") => {
                let result = modules::core::unknown_handler::TransliterationResult::simple(iso.clone());
                (result, None)
            },
            
            // Hub processing needed - convert between formats with metadata
            (modules::hub::HubInput::Devanagari(deva), _) => {
                // Try direct Devanagari → target conversion first (for Indic scripts)
                match self.script_converter_registry.from_hub_with_metadata(to, &hub_input) {
                    Ok(result) => (result, None),
                    Err(_) => {
                        // If direct conversion fails, convert through ISO: Devanagari → ISO → target
                        let hub_result = self.hub.deva_to_iso_with_metadata(&deva)?;
                        let iso_hub_input = match &hub_result.output {
                            modules::hub::HubOutput::Iso(iso_result) => modules::hub::HubInput::Iso(iso_result.clone()),
                            _ => return Err("Expected ISO output from hub".into()),
                        };
                        let final_result = self.script_converter_registry.from_hub_with_metadata(to, &iso_hub_input)?;
                        (final_result, hub_result.metadata)
                    }
                }
            },
            (modules::hub::HubInput::Iso(iso), _) => {
                // Try direct ISO → target conversion first
                match self.script_converter_registry.from_hub_with_metadata(to, &hub_input) {
                    Ok(result) => (result, None),
                    Err(_) => {
                        // If direct conversion fails, convert through Devanagari: ISO → Devanagari → target
                        let hub_result = self.hub.iso_to_deva_with_metadata(&iso)?;
                        let deva_hub_input = match &hub_result.output {
                            modules::hub::HubOutput::Devanagari(deva_result) => modules::hub::HubInput::Devanagari(deva_result.clone()),
                            _ => return Err("Expected Devanagari output from hub".into()),
                        };
                        let final_result = self.script_converter_registry.from_hub_with_metadata(to, &deva_hub_input)?;
                        (final_result, hub_result.metadata)
                    }
                }
            },
        };
        
        // Combine metadata from different stages
        let mut final_metadata = result.metadata.unwrap_or_else(|| modules::core::unknown_handler::TransliterationMetadata::new(from, to));
        
        // Add from_stage metadata (script → hub)
        if !from_metadata.unknown_tokens.is_empty() {
            final_metadata.unknown_tokens.extend(from_metadata.unknown_tokens);
        }
        
        // Add hub_stage metadata if available
        if let Some(hub_metadata) = to_metadata {
            final_metadata.unknown_tokens.extend(hub_metadata.unknown_tokens);
        }
        
        Ok(modules::core::unknown_handler::TransliterationResult {
            output: result.output,
            metadata: Some(final_metadata),
        })
    }
}

impl Default for Shlesha {
    fn default() -> Self {
        Self::new()
    }
}

/// Library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version_info() {
        assert!(!VERSION.is_empty());
        println!("Shlesha version: {}", VERSION);
    }
    
    #[test]
    fn test_transliterator_creation() {
        let _transliterator = Shlesha::new();
    }
    
    #[test]
    fn test_basic_metadata_collection() {
        let transliterator = Shlesha::new();
        
        // Test basic conversion with metadata using a simple vowel
        let result = transliterator.transliterate_with_metadata("अ", "devanagari", "iast").unwrap();
        assert_eq!(result.output, "a");
        assert!(result.metadata.is_some());
        
        let metadata = result.metadata.unwrap();
        assert_eq!(metadata.source_script, "devanagari");
        assert_eq!(metadata.target_script, "iast");
        // For a normal conversion, there should be no unknown tokens
        assert!(metadata.unknown_tokens.is_empty());
    }
    
    #[test]
    fn test_unknown_character_handling_at_all_levels() {
        let transliterator = Shlesha::new();
        
        // Level 1: Main Shlesha transliterate - should pass through unknown characters
        let result = transliterator.transliterate("धर्मkr", "devanagari", "iso").unwrap();
        assert_eq!(result, "dharmakr"); // Unknown 'k' and 'r' should pass through
        
        // Level 2: Cross-script with unknown characters (Devanagari → Gujarati)
        let result = transliterator.transliterate("धर्मkr", "devanagari", "gujarati").unwrap();
        assert_eq!(result, "ધર્મkr"); // Latin chars should pass through
        
        // Level 3: Roman script with unknown characters (IAST → Devanagari)
        let result = transliterator.transliterate("dharmakr", "iast", "devanagari").unwrap();
        assert_eq!(result, "धर्मक्र्"); // Should handle known parts, pass through rest
        
        // Test metadata collection with unknown characters  
        let result = transliterator.transliterate_with_metadata("धर्मkr", "devanagari", "iso").unwrap();
        assert_eq!(result.output, "dharmakr");
        // Should have metadata tracking the unknown characters
        assert!(result.metadata.is_some());
    }
    
    #[test] 
    fn test_mixed_content_graceful_handling() {
        let transliterator = Shlesha::new();
        
        // Test mixed Devanagari and Latin
        let result = transliterator.transliterate("धर्म hello world", "devanagari", "iso").unwrap();
        assert_eq!(result, "dharma hello world");
        
        // Test mixed with punctuation
        let result = transliterator.transliterate("धर्म! 123", "devanagari", "iso").unwrap();
        assert_eq!(result, "dharma! 123");
        
        // Test completely unknown string
        let result = transliterator.transliterate("xyz123", "devanagari", "iso").unwrap();
        assert_eq!(result, "xyz123"); // Should pass through unchanged
    }
}