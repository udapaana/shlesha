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
//! ## Design Philosophy
//!
//! - **Factual documentation**: Technical decisions are explained with their reasons and trade-offs,
//!   without promotional language or unsupported claims
//! - **Transparency**: Design constraints and limitations are documented alongside capabilities
//! - **Evidence-based**: Performance claims and comparisons are backed by measurable benchmarks
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

// ToString/FromStr implementations are now in modules/hub/token_string_impl.rs

// Import hub trait to use the hub
use modules::hub::HubTrait;

// Optional binding modules
#[cfg(feature = "python")]
pub mod python_bindings;

#[cfg(feature = "wasm")]
pub mod wasm_bindings;

use modules::hub::Hub;
#[cfg(not(target_arch = "wasm32"))]
use modules::profiler::{OptimizationCache, Profiler, ProfilerConfig};
use modules::registry::{SchemaRegistry, SchemaRegistryTrait};
#[cfg(not(target_arch = "wasm32"))]
use modules::runtime::RuntimeCompiler;
use modules::schema::{Schema as RuntimeSchema, SchemaBuilder};
use modules::script_converter::ScriptConverterRegistry;

// Re-export unknown handler types for public API
pub use modules::core::unknown_handler::{
    TransliterationMetadata, TransliterationResult, UnknownToken,
};

/// Information about a schema (built-in or runtime loaded)
#[derive(Debug, Clone)]
pub struct SchemaInfo {
    pub name: String,
    pub description: String,
    pub script_type: String,
    pub is_runtime_loaded: bool,
    pub mapping_count: usize,
}

/// Processor source for handling both static and runtime compiled processors
#[derive(Debug)]
pub enum ProcessorSource {
    /// Compile-time generated (built-in scripts)
    Static,
    /// Runtime compiled (same performance!)
    RuntimeCompiled(Box<dyn std::any::Any + Send + Sync>),
    /// Fallback only (development/testing)
    Dynamic,
}

/// Main transliterator struct implementing hub-and-spoke architecture
pub struct Shlesha {
    hub: Hub,
    script_converter_registry: ScriptConverterRegistry,
    registry: SchemaRegistry,
    #[cfg(not(target_arch = "wasm32"))]
    runtime_compiler: Option<RuntimeCompiler>,
    processors: std::collections::HashMap<String, ProcessorSource>,
    #[cfg(not(target_arch = "wasm32"))]
    profiler: Option<Profiler>,
    #[cfg(not(target_arch = "wasm32"))]
    optimization_cache: OptimizationCache,
}

impl Shlesha {
    /// Create a new Shlesha transliterator instance
    pub fn new() -> Self {
        // Use the complete registry with all available converters
        let script_converter_registry = ScriptConverterRegistry::default();

        // Create schema registry and try to load built-in schemas
        let mut registry = SchemaRegistry::new();

        // Try to load the devanagari schema from the schemas directory
        // This enables proper schema-based processing for devanagari
        if registry.load_schema("schemas/devanagari.yaml").is_err() {
            // If loading fails (e.g., in tests or different working directory), continue with placeholder
        }

        Self {
            hub: Hub::new(),
            script_converter_registry,
            registry,
            #[cfg(not(target_arch = "wasm32"))]
            runtime_compiler: RuntimeCompiler::new().ok(),
            processors: std::collections::HashMap::new(),
            #[cfg(not(target_arch = "wasm32"))]
            profiler: None,
            #[cfg(not(target_arch = "wasm32"))]
            optimization_cache: OptimizationCache::new(),
        }
    }

    /// Transliterate text from one script to another via the central hub
    pub fn transliterate(
        &self,
        text: &str,
        from: &str,
        to: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::time::Instant;
            let start_time = Instant::now();

            // Try optimized conversion first if available
            let result = self
                .optimization_cache
                .apply_optimization(text, from, to, |text| {
                    self.transliterate_internal(text, from, to)
                });

            // Record profiling data if enabled
            if let Some(ref profiler) = self.profiler {
                let processing_time = start_time.elapsed();
                profiler.record_conversion(from, to, text, processing_time);
            }

            result
        }

        #[cfg(target_arch = "wasm32")]
        {
            self.transliterate_internal(text, from, to)
        }
    }

    /// Internal transliteration method (the original implementation)
    fn transliterate_internal(
        &self,
        text: &str,
        from: &str,
        to: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Identity conversion - if source and target are the same, return input unchanged
        if from == to {
            return Ok(text.to_string());
        }

        // Convert source script to hub format (Devanagari or ISO)
        let hub_input = self.script_converter_registry.to_hub_with_schema_registry(
            from,
            text,
            Some(&self.registry),
        )?;

        // Apply hub conversion if needed (cross-token-type conversion)
        let final_hub_input = match (&hub_input, from, to) {
            // Cross-token-type conversion needed
            (modules::hub::HubFormat::AlphabetTokens(_), _, _)
                if self.script_converter_registry.supports_script(to) =>
            {
                let tokens = match &hub_input {
                    modules::hub::HubFormat::AlphabetTokens(tokens) => tokens,
                    _ => return Err("Expected AlphabetTokens".into()),
                };

                // Check if target script needs AbugidaTokens
                if self.is_indic_script(to) {
                    // Convert AlphabetTokens to AbugidaTokens via hub
                    let abugida_tokens = self.hub.alphabet_to_abugida_tokens(tokens)?;
                    modules::hub::HubFormat::AbugidaTokens(abugida_tokens)
                } else {
                    hub_input
                }
            }
            (modules::hub::HubFormat::AbugidaTokens(_), _, _)
                if self.script_converter_registry.supports_script(to) =>
            {
                let tokens = match &hub_input {
                    modules::hub::HubFormat::AbugidaTokens(tokens) => tokens,
                    _ => return Err("Expected AbugidaTokens".into()),
                };

                // Check if target script needs AlphabetTokens
                if self.is_roman_script(to) {
                    // Convert AbugidaTokens to AlphabetTokens via hub
                    let alphabet_tokens = self.hub.abugida_to_alphabet_tokens(tokens)?;
                    modules::hub::HubFormat::AlphabetTokens(alphabet_tokens)
                } else {
                    hub_input
                }
            }
            _ => hub_input,
        };

        // Convert from hub format to target script
        let result = self
            .script_converter_registry
            .from_hub_with_schema_registry(to, &final_hub_input, Some(&self.registry))?;

        Ok(result)
    }

    /// Check if a script is a Roman transliteration scheme
    fn is_roman_script(&self, script: &str) -> bool {
        modules::script_converter::is_roman_script(script)
    }

    /// Check if a script is an Indic script
    fn is_indic_script(&self, script: &str) -> bool {
        modules::script_converter::is_indic_script(script)
    }

    /// Transliterate text with metadata collection for unknown tokens
    pub fn transliterate_with_metadata(
        &self,
        text: &str,
        from: &str,
        to: &str,
    ) -> Result<
        crate::modules::core::unknown_handler::TransliterationResult,
        Box<dyn std::error::Error>,
    > {
        // Convert source script to hub format with metadata collection
        let (hub_input, from_metadata) = self
            .script_converter_registry
            .to_hub_with_metadata(from, text)?;

        // Smart hub processing based on input and desired output - with metadata
        // Apply the same hub conversion logic as the simple transliteration path
        let final_hub_input = match (&hub_input, from, to) {
            (modules::hub::HubFormat::AlphabetTokens(_), _, _)
                if self.script_converter_registry.supports_script(to) =>
            {
                let tokens = match &hub_input {
                    modules::hub::HubFormat::AlphabetTokens(tokens) => tokens,
                    _ => return Err("Expected AlphabetTokens".into()),
                };

                // Check if target script needs AbugidaTokens
                if self.is_indic_script(to) {
                    // Convert AlphabetTokens to AbugidaTokens via hub
                    let abugida_tokens = self.hub.alphabet_to_abugida_tokens(tokens)?;
                    modules::hub::HubFormat::AbugidaTokens(abugida_tokens)
                } else {
                    hub_input
                }
            }
            (modules::hub::HubFormat::AbugidaTokens(_), _, _)
                if self.script_converter_registry.supports_script(to) =>
            {
                let tokens = match &hub_input {
                    modules::hub::HubFormat::AbugidaTokens(tokens) => tokens,
                    _ => return Err("Expected AbugidaTokens".into()),
                };

                // Check if target script needs AlphabetTokens
                if self.is_roman_script(to) {
                    // Convert AbugidaTokens to AlphabetTokens via hub
                    let alphabet_tokens = self.hub.abugida_to_alphabet_tokens(tokens)?;
                    modules::hub::HubFormat::AlphabetTokens(alphabet_tokens)
                } else {
                    hub_input
                }
            }
            _ => hub_input,
        };

        let (result, to_metadata) = match self
            .script_converter_registry
            .from_hub_with_metadata(to, &final_hub_input)
        {
            Ok(result) => (
                result,
                None::<modules::core::unknown_handler::TransliterationMetadata>,
            ),
            Err(e) => {
                return Err(format!("Conversion failed: {}", e).into());
            }
        };

        // Combine metadata from different stages
        let mut final_metadata =
            modules::core::unknown_handler::TransliterationMetadata::new(from, to);

        // If result has metadata, copy over any unknown tokens but keep correct source/target
        if let Some(result_metadata) = result.metadata {
            final_metadata
                .unknown_tokens
                .extend(result_metadata.unknown_tokens);
        }

        // Add from_stage metadata (script → hub)
        if !from_metadata.unknown_tokens.is_empty() {
            final_metadata
                .unknown_tokens
                .extend(from_metadata.unknown_tokens);
        }

        // Add hub_stage metadata if available
        if let Some(hub_metadata) = to_metadata {
            final_metadata
                .unknown_tokens
                .extend(hub_metadata.unknown_tokens);
        }

        Ok(modules::core::unknown_handler::TransliterationResult {
            output: result.output,
            metadata: Some(final_metadata),
        })
    }

    /// Load a schema from a file path for runtime script support
    pub fn load_schema_from_file(
        &mut self,
        file_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.registry.load_schema(file_path)?;
        Ok(())
    }

    /// Load a schema from YAML content string
    pub fn load_schema_from_string(
        &mut self,
        yaml_content: &str,
        schema_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.registry
            .load_schema_from_string(yaml_content, schema_name)?;
        Ok(())
    }

    /// Add a runtime schema with compilation (if available)
    pub fn add_runtime_schema(
        &mut self,
        schema: RuntimeSchema,
    ) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            match &mut self.runtime_compiler {
                Some(compiler) => {
                    match compiler.compile_schema(&schema) {
                        Ok(compiled) => {
                            // Same performance as static processors!
                            self.processors.insert(
                                schema.metadata.name.clone(),
                                ProcessorSource::RuntimeCompiled(Box::new(compiled)),
                            );
                            return Ok(());
                        }
                        Err(_) => {
                            // Graceful fallback to registry-based processing
                        }
                    }
                }
                None => {
                    // No runtime compiler available, fall back to registry
                }
            }
        }

        // WASM or fallback: Use registry-based processing
        let registry_schema = self.convert_runtime_schema_to_registry(&schema);
        let _ = self
            .registry
            .add_schema(schema.metadata.name.clone(), registry_schema);
        self.processors
            .insert(schema.metadata.name.clone(), ProcessorSource::Dynamic);

        Ok(())
    }

    /// Create schema using builder pattern
    pub fn create_schema(&mut self, name: &str) -> SchemaBuilder {
        SchemaBuilder::new(name)
    }

    /// Convert RuntimeSchema to registry Schema format
    fn convert_runtime_schema_to_registry(
        &self,
        runtime_schema: &RuntimeSchema,
    ) -> modules::registry::Schema {
        use modules::registry::{Schema as RegistrySchema, SchemaMetadata as RegistryMetadata};
        use rustc_hash::FxHashMap;

        // Flatten the nested mappings into a single hashmap
        let mut flattened_mappings = FxHashMap::default();

        for entries in runtime_schema.mappings.values() {
            for (token, mapping) in entries {
                // For registry schema, we use the first (preferred) mapping
                let preferred_mapping = match mapping {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Array(arr) => arr
                        .first()
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    _ => continue,
                };
                flattened_mappings.insert(token.clone(), preferred_mapping);
            }
        }

        RegistrySchema {
            name: runtime_schema.metadata.name.clone(),
            script_type: runtime_schema.metadata.script_type.clone(),
            target: runtime_schema.target.clone(),
            mappings: flattened_mappings,
            metadata: RegistryMetadata {
                name: runtime_schema.metadata.name.clone(),
                script_type: runtime_schema.metadata.script_type.clone(),
                has_implicit_a: false, // Default for now
                description: runtime_schema.metadata.description.clone(),
                aliases: None, // Not available in RuntimeSchema
            },
        }
    }

    /// Get list of all available scripts (built-in + runtime loaded)
    pub fn list_supported_scripts(&self) -> Vec<String> {
        let mut scripts = self
            .script_converter_registry
            .list_supported_scripts()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        // Add runtime loaded schemas
        let runtime_scripts = self.registry.list_schemas_owned();
        scripts.extend(runtime_scripts);

        scripts.sort();
        scripts.dedup();
        scripts
    }

    /// Check if a specific script is supported (built-in or runtime)
    pub fn supports_script(&self, script_name: &str) -> bool {
        self.script_converter_registry
            .supports_script_with_registry(script_name, Some(&self.registry))
            || self.registry.get_schema(script_name).is_some()
    }

    /// Get information about a loaded runtime schema
    pub fn get_schema_info(&self, script_name: &str) -> Option<SchemaInfo> {
        self.registry
            .get_schema(script_name)
            .map(|schema| SchemaInfo {
                name: schema.metadata.name.clone(),
                description: schema.metadata.description.clone().unwrap_or_default(),
                script_type: schema.metadata.script_type.clone(),
                is_runtime_loaded: true,
                mapping_count: schema.mappings.values().map(|m| m.len()).sum(),
            })
    }

    /// Remove a runtime loaded schema
    pub fn remove_schema(&mut self, script_name: &str) -> bool {
        self.registry.remove_schema(script_name)
    }

    /// Clear all runtime loaded schemas
    pub fn clear_runtime_schemas(&mut self) {
        self.registry.clear();
    }

    /// Create a new Shlesha instance with a custom registry
    pub fn with_registry(registry: SchemaRegistry) -> Self {
        let script_converter_registry = ScriptConverterRegistry::default();

        Self {
            hub: Hub::new(),
            script_converter_registry,
            registry,
            #[cfg(not(target_arch = "wasm32"))]
            runtime_compiler: None, // Initialize later if needed
            processors: std::collections::HashMap::new(),
            #[cfg(not(target_arch = "wasm32"))]
            profiler: None,
            #[cfg(not(target_arch = "wasm32"))]
            optimization_cache: OptimizationCache::new(),
        }
    }

    /// Enable profiling with default configuration
    #[cfg(not(target_arch = "wasm32"))]
    pub fn enable_profiling(&mut self) {
        self.profiler = Some(Profiler::new());
    }

    /// Enable profiling with custom configuration
    #[cfg(not(target_arch = "wasm32"))]
    pub fn enable_profiling_with_config(&mut self, config: ProfilerConfig) {
        self.profiler = Some(Profiler::with_config(config));
    }

    /// Disable profiling
    #[cfg(not(target_arch = "wasm32"))]
    pub fn disable_profiling(&mut self) {
        self.profiler = None;
    }

    /// Get profiling statistics
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_profile_stats(
        &self,
    ) -> Option<rustc_hash::FxHashMap<(String, String), modules::profiler::ProfileStats>> {
        self.profiler.as_ref().map(|p| p.get_profile_stats())
    }

    /// Generate optimized lookup tables from current profiles
    #[cfg(not(target_arch = "wasm32"))]
    pub fn generate_optimizations(&self) -> Vec<modules::profiler::OptimizedLookupTable> {
        self.profiler
            .as_ref()
            .map(|p| p.generate_optimizations())
            .unwrap_or_default()
    }

    /// Load an optimization table for hot-reloading
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_optimization(&self, optimization: modules::profiler::OptimizedLookupTable) {
        self.optimization_cache.load(optimization);
    }

    /// Save current profiles to disk
    #[cfg(not(target_arch = "wasm32"))]
    pub fn save_profiles(&self) {
        if let Some(ref profiler) = self.profiler {
            profiler.save_profiles();
        }
    }

    /// Create Shlesha instance with profiling enabled
    #[cfg(not(target_arch = "wasm32"))]
    pub fn with_profiling() -> Self {
        let mut instance = Self::new();
        instance.enable_profiling();
        instance
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
        // VERSION is a const, so we just print it
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
        let result = transliterator
            .transliterate_with_metadata("अ", "devanagari", "iast")
            .unwrap();
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
        let result = transliterator
            .transliterate("धर्मkr", "devanagari", "iso15919")
            .unwrap();
        assert_eq!(result, "dharmakr"); // Unknown 'k' and 'r' should pass through

        // Level 2: Cross-script with unknown characters (Devanagari → Gujarati)
        let result = transliterator
            .transliterate("धर्मkr", "devanagari", "gujarati")
            .unwrap();

        // Debug: Print the actual result
        println!("Test input: धर्मkr");
        println!("Expected: ધર્મkr");
        println!("Actual:   {}", result);

        // Test simpler case to debug virama issue
        let simple_result = transliterator
            .transliterate("धर्म", "devanagari", "gujarati")
            .unwrap();
        println!("\nSimple test: धर्म -> {}", simple_result);
        println!("Expected:    ધર્મ");

        // Compare with working case (to roman)
        let roman_result = transliterator
            .transliterate("धर्म", "devanagari", "iast")
            .unwrap();
        println!("To Roman:    {}", roman_result);

        // DEBUG: Test individual characters to trace virama
        println!("\nDEBUG: Character by character analysis");
        for (i, ch) in "धर्म".chars().enumerate() {
            println!("  [{}] {} (U+{:04X})", i, ch, ch as u32);
            if ch == '्' {
                println!("      ^ This is the virama character!");
            }
        }

        assert_eq!(result, "ધર્મkr"); // Latin chars should pass through

        // Level 3: Roman script with unknown characters (IAST → Devanagari)
        let result = transliterator
            .transliterate("dharmaqx", "iast", "devanagari")
            .unwrap();
        // q and x are not part of IAST, so they pass through unchanged
        // Note: "dharma" has implicit 'a' after both 'r' and 'm', so no virama
        assert_eq!(result, "धर्मqx");

        // Test metadata collection with unknown characters
        let result = transliterator
            .transliterate_with_metadata("धर्मkr", "devanagari", "iso15919")
            .unwrap();
        assert_eq!(result.output, "dharmakr");
        // Should have metadata tracking the unknown characters
        assert!(result.metadata.is_some());
    }

    #[test]
    fn test_mixed_content_graceful_handling() {
        let transliterator = Shlesha::new();

        // Test mixed Devanagari and Latin
        let result = transliterator
            .transliterate("धर्म hello world", "devanagari", "iso15919")
            .unwrap();
        assert_eq!(result, "dharma hello world");

        // Test mixed with punctuation
        let result = transliterator
            .transliterate("धर्म! 123", "devanagari", "iso15919")
            .unwrap();
        assert_eq!(result, "dharma! 123");

        // Test completely unknown string
        let result = transliterator
            .transliterate("xyz123", "devanagari", "iso15919")
            .unwrap();
        assert_eq!(result, "xyz123"); // Should pass through unchanged
    }

    #[test]
    fn test_virama_across_scripts() {
        let transliterator = Shlesha::new();
        let input = "धर्म"; // dha + ra + virama + ma

        println!("Testing virama handling across scripts:");
        println!("Input: {} (Devanagari)", input);

        // Test various Indic scripts
        let scripts = vec![
            ("bengali", "ধর্ম"),
            ("gujarati", "ધર્મ"),
            ("telugu", "ధర్మ"),
            ("kannada", "ಧರ್ಮ"),
            ("malayalam", "ധര്മ"),
        ];

        for (script, expected) in scripts {
            match transliterator.transliterate(input, "devanagari", script) {
                Ok(result) => {
                    let chars_result: Vec<char> = result.chars().collect();
                    let chars_expected: Vec<char> = expected.chars().collect();

                    println!("\n{} script:", script);
                    println!("  Result:   {} ({} chars)", result, chars_result.len());
                    println!("  Expected: {} ({} chars)", expected, chars_expected.len());

                    if chars_result.len() != chars_expected.len() {
                        println!("  ❌ Character count mismatch!");
                        for (i, ch) in chars_result.iter().enumerate() {
                            println!("    [{}] {} (U+{:04X})", i, ch, *ch as u32);
                        }
                        println!("  Expected breakdown:");
                        for (i, ch) in chars_expected.iter().enumerate() {
                            println!("    [{}] {} (U+{:04X})", i, ch, *ch as u32);
                        }
                    } else {
                        println!("  ✅ Character count matches");
                    }
                }
                Err(e) => println!("{}: Error - {}", script, e),
            }
        }
    }
}

// Python module definition - must be in lib.rs for PyO3 to find it
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn shlesha(m: &Bound<'_, PyModule>) -> PyResult<()> {
    python_bindings::configure_module(m)
}
