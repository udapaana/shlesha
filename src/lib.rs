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
use modules::converter::{ScriptConverter, ScriptConverterTrait};
use modules::generator::{TargetGenerator, TargetGeneratorTrait};
use modules::registry::{SchemaRegistry, SchemaRegistryTrait};

/// Main transliterator struct implementing hub-and-spoke architecture
pub struct Shlesha {
    hub: Hub,
    converter: ScriptConverter,
    generator: TargetGenerator,
    registry: SchemaRegistry,
}

impl Shlesha {
    /// Create a new Shlesha transliterator instance
    pub fn new() -> Self {
        Self {
            hub: Hub::new(),
            converter: ScriptConverter::new(),
            generator: TargetGenerator::new(),
            registry: SchemaRegistry::new(),
        }
    }

    /// Transliterate text from one script to another via the central hub
    pub fn transliterate(&self, text: &str, from: &str, to: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Convert source script to hub format (Devanagari or ISO)
        let hub_input = self.converter.to_hub(from, text)?;
        
        // Process through the hub (Devanagari ↔ ISO-15919)
        let hub_output = match hub_input {
            HubInput::Devanagari(deva) => self.hub.deva_to_iso(&deva)?,
            HubInput::Iso(iso) => self.hub.iso_to_deva(&iso)?,
        };
        
        // Generate target script from hub output
        Ok(self.generator.from_hub(&hub_output, to)?)
    }

    /// Load a new script schema at runtime
    pub fn load_schema(&mut self, schema_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(self.registry.load_schema(schema_path)?)
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