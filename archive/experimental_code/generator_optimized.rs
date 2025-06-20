use std::collections::HashMap;
use crate::ir::{ElementType, AbugidaIR, AlphabetIR, IR};
use crate::schema_parser::{Schema, ScriptType};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GenerateError {
    #[error("Unknown target script: {0}")]
    UnknownScript(String),
    
    #[error("Missing mapping for element: {0}")]
    MissingMapping(String),
    
    #[error("Invalid IR for target script")]
    InvalidIR,
}

pub struct GeneratorOptimized {
    schemas: HashMap<String, Schema>,
    // Cache reverse mappings for each schema
    reverse_mappings_cache: HashMap<String, HashMap<String, String>>,
}

impl GeneratorOptimized {
    pub fn new() -> Self {
        Self {
            schemas: HashMap::new(),
            reverse_mappings_cache: HashMap::new(),
        }
    }
    
    pub fn load_schema(&mut self, schema: Schema) {
        // Build and cache reverse mappings
        let mut reverse = HashMap::new();
        for (_, category_mappings) in &schema.mappings {
            for (grapheme, mapping) in category_mappings {
                reverse.insert(mapping.canonical.clone(), grapheme.clone());
            }
        }
        
        self.reverse_mappings_cache.insert(schema.name.clone(), reverse);
        self.schemas.insert(schema.name.clone(), schema);
    }
    
    pub fn generate(&self, ir: &IR, target_script: &str) -> Result<String, GenerateError> {
        let schema = self.schemas.get(target_script)
            .ok_or_else(|| GenerateError::UnknownScript(target_script.to_string()))?;
        
        match (&ir, &schema.script_type) {
            (IR::Abugida(abugida), ScriptType::Abugida) => {
                self.generate_abugida(abugida, target_script)
            }
            (IR::Alphabet(alphabet), ScriptType::Alphabet) => {
                self.generate_alphabet(alphabet, target_script)
            }
            _ => Err(GenerateError::InvalidIR),
        }
    }
    
    fn generate_abugida(&self, ir: &AbugidaIR, target_script: &str) -> Result<String, GenerateError> {
        // Pre-allocate with estimated capacity
        let mut result = String::with_capacity(ir.elements.len() * 3);
        
        // Get cached reverse mappings
        let reverse_mappings = self.reverse_mappings_cache.get(target_script)
            .ok_or_else(|| GenerateError::UnknownScript(target_script.to_string()))?;
        
        for element in &ir.elements {
            match element.element_type.0.as_str() {
                ElementType::WHITESPACE => {
                    result.push_str(&element.grapheme);
                }
                ElementType::UNKNOWN => {
                    // Mark unknown elements
                    result.push_str("[?:");
                    result.push_str(&element.grapheme);
                    result.push(']');
                }
                _ => {
                    // Look up in reverse mappings
                    if let Some(grapheme) = reverse_mappings.get(&element.canonical) {
                        result.push_str(grapheme);
                    } else {
                        // If no mapping found, use the grapheme directly
                        result.push_str(&element.grapheme);
                    }
                }
            }
        }
        
        Ok(result)
    }
    
    fn generate_alphabet(&self, ir: &AlphabetIR, target_script: &str) -> Result<String, GenerateError> {
        // Pre-allocate with estimated capacity
        let mut result = String::with_capacity(ir.elements.len() * 2);
        
        // Get cached reverse mappings
        let reverse_mappings = self.reverse_mappings_cache.get(target_script)
            .ok_or_else(|| GenerateError::UnknownScript(target_script.to_string()))?;
        
        for element in &ir.elements {
            match element.element_type.0.as_str() {
                ElementType::WHITESPACE => {
                    result.push_str(&element.grapheme);
                }
                ElementType::PUNCTUATION => {
                    result.push_str(&element.grapheme);
                }
                ElementType::UNKNOWN => {
                    // Mark unknown elements
                    result.push_str("[?:");
                    result.push_str(&element.grapheme);
                    result.push(']');
                }
                _ => {
                    // Look up in reverse mappings
                    if let Some(grapheme) = reverse_mappings.get(&element.canonical) {
                        result.push_str(grapheme);
                    } else {
                        // If no mapping found, use the grapheme directly
                        result.push_str(&element.grapheme);
                    }
                }
            }
        }
        
        Ok(result)
    }
}