use std::path::Path;
use std::collections::HashMap;
use crate::ir::{IR, Extension};
use crate::parser::{Parser, ParseError};
use crate::transformer::{Transformer, TransformError};
use crate::generator::{Generator, GenerateError};
use crate::schema_parser::{Schema, SchemaRegistry, SchemaError, ScriptType};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransliteratorError {
    #[error("Parse error: {0}")]
    ParseError(#[from] ParseError),
    
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

pub struct TransliteratorOptimized {
    parser: Parser,
    transformer: Transformer,
    generator: Generator,
    schema_registry: SchemaRegistry,
    active_extensions: Vec<String>,
    // Optimization: Cache script info for fast lookups
    script_info_cache: std::collections::HashMap<String, ScriptInfo>,
}

impl TransliteratorOptimized {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            transformer: Transformer::new(),
            generator: Generator::new(),
            schema_registry: SchemaRegistry::new(),
            active_extensions: Vec::new(),
            script_info_cache: std::collections::HashMap::new(),
        }
    }
    
    pub fn load_schema(&mut self, schema: Schema) -> Result<(), TransliteratorError> {
        // Cache script info
        let script_info = ScriptInfo {
            script_type: schema.script_type.clone(),
            has_extensions: !schema.extensions.is_empty(),
        };
        self.script_info_cache.insert(schema.name.clone(), script_info);
        
        // Load schema
        self.parser.load_schema(schema.clone());
        self.generator.load_schema(schema.clone());
        self.schema_registry.register(schema)?;
        Ok(())
    }
    
    pub fn transliterate(
        &self,
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
        
        // Parse input to IR
        let mut source_ir = self.parser.parse(input, from_script)?;
        
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
        self.generator.generate(&target_ir, to_script)
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