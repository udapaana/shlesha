use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::schema_parser::{Schema, ScriptType, ElementMapping};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimplifiedSchema {
    pub name: String,
    
    #[serde(rename = "type")]
    pub script_type: ScriptType,
    
    // Compact syntax fields
    #[serde(default)]
    pub consonants: Option<String>,
    
    #[serde(default)]
    pub vowels: Option<String>,
    
    #[serde(default)]
    pub vowel_marks: Option<String>,
    
    #[serde(default)]
    pub modifiers: Option<String>,
    
    #[serde(default)]
    pub numerals: Option<String>,
    
    #[serde(default)]
    pub punctuation: Option<String>,
    
    // Extensions with simplified syntax
    #[serde(default)]
    pub extensions: HashMap<String, String>,
    
    // Optional metadata (simplified)
    #[serde(default)]
    pub description: Option<String>,
    
    #[serde(default)]
    pub version: Option<String>,
}

pub struct SimplifiedSchemaParser;

impl SimplifiedSchemaParser {
    pub fn parse_str(input: &str) -> Result<Schema, crate::schema_parser::SchemaError> {
        // Try to parse as simplified format
        let simplified: SimplifiedSchema = serde_yaml::from_str(input)
            .map_err(|e| crate::schema_parser::SchemaError::YamlParseError(e))?;
        
        // Convert to full schema
        Self::expand_to_full_schema(simplified)
    }
    
    fn expand_to_full_schema(simplified: SimplifiedSchema) -> Result<Schema, crate::schema_parser::SchemaError> {
        let mut mappings = HashMap::new();
        
        // Parse each category
        if let Some(consonants) = simplified.consonants {
            let consonant_mappings = Self::parse_compact_mappings(&consonants, "consonant")?;
            mappings.insert("consonants".to_string(), consonant_mappings);
        }
        
        if let Some(vowels) = simplified.vowels {
            let vowel_mappings = Self::parse_compact_mappings(&vowels, "vowel_independent")?;
            mappings.insert("vowels".to_string(), vowel_mappings);
        }
        
        if let Some(vowel_marks) = simplified.vowel_marks {
            let mark_mappings = Self::parse_compact_mappings(&vowel_marks, "vowel_dependent")?;
            mappings.insert("vowel_marks".to_string(), mark_mappings);
        }
        
        if let Some(modifiers) = simplified.modifiers {
            let modifier_mappings = Self::parse_compact_mappings(&modifiers, "modifier")?;
            mappings.insert("modifiers".to_string(), modifier_mappings);
        }
        
        if let Some(numerals) = simplified.numerals {
            let numeral_mappings = Self::parse_compact_mappings(&numerals, "numeral")?;
            mappings.insert("numerals".to_string(), numeral_mappings);
        }
        
        if let Some(punctuation) = simplified.punctuation {
            let punct_mappings = Self::parse_compact_mappings(&punctuation, "punctuation")?;
            mappings.insert("punctuation".to_string(), punct_mappings);
        }
        
        // Handle extensions
        let mut extensions = HashMap::new();
        for (ext_name, ext_mappings) in simplified.extensions {
            let extension_def = crate::schema_parser::ExtensionDefinition {
                description: format!("Extension: {}", ext_name),
                priority: 0,
                mappings: Self::parse_extension_mappings(&ext_mappings)?,
                conditions: HashMap::new(),
            };
            extensions.insert(ext_name, extension_def);
        }
        
        // Build metadata
        let metadata = if simplified.description.is_some() || simplified.version.is_some() {
            Some(crate::schema_parser::SchemaMetadata {
                version: simplified.version.unwrap_or_else(|| "1.0.0".to_string()),
                author: "Generated from simplified schema".to_string(),
                description: simplified.description.unwrap_or_else(|| format!("{} script", simplified.name)),
                references: vec![],
            })
        } else {
            None
        };
        
        Ok(Schema {
            name: simplified.name,
            script_type: simplified.script_type,
            element_types: HashMap::new(), // Auto-generated
            mappings,
            extensions,
            metadata,
        })
    }
    
    /// Parse compact mapping syntax like "क=ka ख=kha ग=ga"
    fn parse_compact_mappings(
        input: &str, 
        element_type: &str
    ) -> Result<HashMap<String, ElementMapping>, crate::schema_parser::SchemaError> {
        let mut mappings = HashMap::new();
        
        // Handle multi-line input
        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // Parse space-separated mappings: "क=ka ख=kha ग=ga"
            for pair in line.split_whitespace() {
                if let Some((grapheme, canonical)) = pair.split_once('=') {
                    let mapping = ElementMapping {
                        canonical: canonical.to_string(),
                        element_type: Some(element_type.to_string()),
                        properties: Self::infer_properties(canonical, element_type),
                    };
                    mappings.insert(grapheme.to_string(), mapping);
                }
            }
        }
        
        Ok(mappings)
    }
    
    /// Parse extension mappings (same format as compact mappings)
    fn parse_extension_mappings(
        input: &str
    ) -> Result<HashMap<String, crate::schema_parser::ExtensionMapping>, crate::schema_parser::SchemaError> {
        let mut mappings = HashMap::new();
        
        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            for pair in line.split_whitespace() {
                if let Some((grapheme, canonical)) = pair.split_once('=') {
                    let ext_mapping = crate::schema_parser::ExtensionMapping {
                        to: canonical.to_string(),
                        element_type: None, // Auto-inferred
                        properties: HashMap::new(),
                    };
                    mappings.insert(grapheme.to_string(), ext_mapping);
                }
            }
        }
        
        Ok(mappings)
    }
    
    /// Auto-infer properties from canonical form and element type
    fn infer_properties(canonical: &str, element_type: &str) -> HashMap<String, serde_yaml::Value> {
        let mut properties = HashMap::new();
        
        match element_type {
            "consonant" => {
                // Infer aspiration from 'h' in canonical form
                if canonical.contains('h') && canonical.len() > 1 {
                    properties.insert("aspirated".to_string(), serde_yaml::Value::Bool(true));
                }
                
                // Infer voice from common patterns (simplified heuristic)
                let voiced_consonants = ["g", "gh", "j", "jh", "ḍ", "ḍh", "d", "dh", "b", "bh", "ṅ", "ñ", "ṇ", "n", "m", "y", "r", "l", "v", "h"];
                if voiced_consonants.iter().any(|&v| canonical.starts_with(v)) {
                    properties.insert("voiced".to_string(), serde_yaml::Value::Bool(true));
                }
                
                // Default: has inherent vowel
                properties.insert("has_inherent_vowel".to_string(), serde_yaml::Value::Bool(true));
            }
            
            "vowel_independent" | "vowel_dependent" => {
                // Infer vowel length from canonical form
                if canonical.chars().count() > 1 || canonical.contains('ā') || canonical.contains('ī') || canonical.contains('ū') {
                    properties.insert("is_long".to_string(), serde_yaml::Value::Bool(true));
                }
            }
            
            _ => {} // No special inference for other types
        }
        
        properties
    }
}