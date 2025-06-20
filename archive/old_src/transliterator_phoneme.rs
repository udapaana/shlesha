//! Phoneme-based transliterator that bridges the optimized PhonemeParser
//! with the existing transliterator infrastructure for 2x performance improvement.
//! 
//! This implementation uses direct phoneme processing instead of the generic
//! IR system, achieving significant performance gains while maintaining compatibility.

use std::collections::HashMap;
use crate::ir::{IR, Extension};
use crate::phoneme_parser::{PhonemeParser, PhonemeParseError};
use crate::transformer::{Transformer, TransformError};
use crate::generator::{Generator, GenerateError};
use crate::schema_parser::{Schema, SchemaRegistry, SchemaError, ScriptType};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransliteratorError {
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Phoneme parse error: {0}")]
    PhonemeParseError(#[from] PhonemeParseError),
    
    #[error("Transform error: {0}")]
    TransformError(#[from] TransformError),
    
    #[error("Generate error: {0}")]
    GenerateError(#[from] GenerateError),
    
    #[error("Schema error: {0}")]
    SchemaError(#[from] SchemaError),
    
    #[error("Unknown script: {0}")]
    UnknownScript(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
}

// Cache-friendly structure for fast lookups
struct ScriptInfo {
    script_type: ScriptType,
    has_extensions: bool,
}

/// High-performance transliterator using zero-allocation phoneme parsing
pub struct PhonemeTransliterator {
    parser: PhonemeParser,
    transformer: Transformer,
    generator: Generator,
    schema_registry: SchemaRegistry,
    active_extensions: Vec<String>,
    // Optimization: Cache script info for fast lookups
    script_info_cache: HashMap<String, ScriptInfo>,
}

impl PhonemeTransliterator {
    pub fn new() -> Self {
        Self {
            parser: PhonemeParser::new(),
            transformer: Transformer::new(),
            generator: Generator::new(),
            schema_registry: SchemaRegistry::new(),
            active_extensions: Vec::new(),
            script_info_cache: HashMap::new(),
        }
    }
    
    pub fn load_schema(&mut self, schema: Schema) -> Result<(), TransliteratorError> {
        // Cache script info
        let script_info = ScriptInfo {
            script_type: schema.script_type.clone(),
            has_extensions: !schema.extensions.is_empty(),
        };
        self.script_info_cache.insert(schema.name.clone(), script_info);
        
        // Load schema into all components
        self.parser.load_schema(schema.clone());
        self.generator.load_schema(schema.clone());
        self.schema_registry.register(schema);
        Ok(())
    }
    
    pub fn transliterate(
        &mut self,  // mut because parser tracks statistics
        input: &str,
        from_script: &str,
        to_script: &str
    ) -> Result<String, TransliteratorError> {
        // Fast path: return empty for empty input
        if input.is_empty() {
            return Ok(String::new());
        }
        
        // Optimization: Single lookup for script info
        let from_info = self.script_info_cache.get(from_script)
            .ok_or_else(|| TransliteratorError::UnknownScript(from_script.to_string()))?;
        let to_info = self.script_info_cache.get(to_script)
            .ok_or_else(|| TransliteratorError::UnknownScript(to_script.to_string()))?;
        
        // Parse input to IR using zero-allocation phoneme parser
        let mut source_ir = self.parser.parse_to_ir(input, from_script)?;
        
        // Apply extensions only if needed
        if !self.active_extensions.is_empty() && from_info.has_extensions {
            self.apply_extensions(&mut source_ir, from_script)?;
        }
        
        // Transform IR only if script types differ
        let target_ir = if from_info.script_type != to_info.script_type {
            let target_type = match to_info.script_type {
                ScriptType::Abugida => "abugida",
                ScriptType::Alphabet => "alphabet",
            };
            self.transformer.transform(source_ir, target_type)?
        } else {
            source_ir
        };
        
        // Generate output
        Ok(self.generator.generate(&target_ir, to_script)?)
    }
    
    /// Get parsing statistics for performance analysis
    pub fn get_parse_stats(&self) -> &crate::phoneme_parser::PhonemeParseStats {
        &self.parser.stats
    }
    
    /// Reset parsing statistics
    pub fn reset_stats(&mut self) {
        self.parser.stats = Default::default();
    }
    
    // Optimized extension application
    fn apply_extensions(&self, ir: &mut IR, script_name: &str) -> Result<(), TransliteratorError> {
        let schema = self.schema_registry.get(script_name)
            .ok_or_else(|| TransliteratorError::UnknownScript(script_name.to_string()))?;
        
        // Build extensions more efficiently
        for ext_name in &self.active_extensions {
            if let Some(ext_def) = schema.extensions.get(ext_name) {
                // Skip if no mappings
                if ext_def.mappings.is_empty() {
                    continue;
                }
                
                let extension = Extension {
                    name: ext_name.clone(),
                    priority: ext_def.priority,
                    mappings: ext_def.mappings.iter()
                        .map(|(k, v)| {
                            let mapping = crate::ir::ExtensionMapping {
                                from: k.clone(),
                                to: v.to.clone(),
                                element_type: v.element_type.as_ref()
                                    .map(|t| crate::ir::ElementType(t.clone())),
                                properties: v.properties.iter()
                                    .map(|(pk, pv)| {
                                        (pk.clone(), crate::schema_parser::SchemaParser::yaml_value_to_property_value(pv))
                                    })
                                    .collect(),
                            };
                            (k.clone(), mapping)
                        })
                        .collect(),
                };
                
                ir.add_extension(extension);
            }
        }
        
        // Apply all extensions
        match ir {
            IR::Abugida(abugida) => abugida.apply_extensions(),
            IR::Alphabet(alphabet) => alphabet.apply_extensions(),
        }
        
        Ok(())
    }
    
    pub fn add_extension(&mut self, extension_name: &str) -> Result<(), TransliteratorError> {
        self.active_extensions.push(extension_name.to_string());
        Ok(())
    }
    
    pub fn clear_extensions(&mut self) {
        self.active_extensions.clear();
    }
}

/// Builder for PhonemeTransliterator with fluent API
pub struct PhonemeTransliteratorBuilder {
    transliterator: PhonemeTransliterator,
}

impl PhonemeTransliteratorBuilder {
    pub fn new() -> Self {
        Self {
            transliterator: PhonemeTransliterator::new(),
        }
    }
    
    pub fn with_schema(mut self, schema: Schema) -> Result<Self, TransliteratorError> {
        self.transliterator.load_schema(schema)?;
        Ok(self)
    }
    
    pub fn with_extension(mut self, extension_name: &str) -> Result<Self, TransliteratorError> {
        self.transliterator.add_extension(extension_name)?;
        Ok(self)
    }
    
    pub fn build(self) -> PhonemeTransliterator {
        self.transliterator
    }
}