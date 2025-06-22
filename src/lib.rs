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
        // For now, just return simple result without metadata
        // TODO: Implement full metadata collection through hub and converters
        let output = self.transliterate(text, from, to)?;
        Ok(crate::modules::core::unknown_handler::TransliterationResult::simple(output))
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
}