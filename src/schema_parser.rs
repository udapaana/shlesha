use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::ir::PropertyValue;

#[derive(Debug, Error)]
pub enum SchemaError {
    #[error("Failed to read schema file: {0}")]
    FileReadError(#[from] std::io::Error),
    
    #[error("Failed to parse YAML: {0}")]
    YamlParseError(#[from] serde_yaml::Error),
    
    #[error("Invalid schema: {0}")]
    ValidationError(String),
    
    #[error("Missing required field: {0}")]
    MissingField(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScriptType {
    Abugida,
    Alphabet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementMapping {
    pub canonical: String,
    
    #[serde(default, rename = "type")]
    pub element_type: Option<String>,
    
    #[serde(default, flatten)]
    pub properties: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub name: String,
    
    #[serde(rename = "type")]
    pub script_type: ScriptType,
    
    #[serde(default)]
    pub element_types: HashMap<String, ElementTypeDefinition>,
    
    pub mappings: HashMap<String, HashMap<String, ElementMapping>>,
    
    #[serde(default)]
    pub extensions: HashMap<String, ExtensionDefinition>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SchemaMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementTypeDefinition {
    pub description: String,
    
    #[serde(default)]
    pub properties: HashMap<String, PropertyDefinition>,
    
    #[serde(default)]
    pub inherits_from: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDefinition {
    #[serde(rename = "type")]
    pub property_type: String,
    
    pub description: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_yaml::Value>,
    
    #[serde(default)]
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionDefinition {
    pub description: String,
    
    #[serde(default)]
    pub priority: i32,
    
    pub mappings: HashMap<String, ExtensionMapping>,
    
    #[serde(default)]
    pub conditions: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionMapping {
    pub to: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_type: Option<String>,
    
    #[serde(default)]
    pub properties: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaMetadata {
    pub version: String,
    pub author: String,
    pub description: String,
    
    #[serde(default)]
    pub references: Vec<String>,
}

pub struct SchemaParser;

impl SchemaParser {
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Schema, SchemaError> {
        let content = std::fs::read_to_string(path)?;
        Self::parse_str(&content)
    }
    
    pub fn parse_str(content: &str) -> Result<Schema, SchemaError> {
        let schema: Schema = serde_yaml::from_str(content)?;
        Self::validate_schema(&schema)?;
        Ok(schema)
    }
    
    fn validate_schema(schema: &Schema) -> Result<(), SchemaError> {
        if schema.name.is_empty() {
            return Err(SchemaError::MissingField("name".to_string()));
        }
        
        if schema.mappings.is_empty() {
            return Err(SchemaError::ValidationError(
                "Schema must have at least one mapping category".to_string()
            ));
        }
        
        Ok(())
    }
    
    pub fn yaml_value_to_property_value(value: &serde_yaml::Value) -> PropertyValue {
        match value {
            serde_yaml::Value::Bool(b) => PropertyValue::Bool(*b),
            serde_yaml::Value::Number(n) => {
                PropertyValue::Number(n.as_f64().unwrap_or(0.0))
            }
            serde_yaml::Value::String(s) => PropertyValue::String(s.clone()),
            serde_yaml::Value::Sequence(seq) => {
                PropertyValue::List(seq.iter().map(Self::yaml_value_to_property_value).collect())
            }
            serde_yaml::Value::Mapping(map) => {
                let mut result = HashMap::new();
                for (k, v) in map {
                    if let Some(key) = k.as_str() {
                        result.insert(key.to_string(), Self::yaml_value_to_property_value(v));
                    }
                }
                PropertyValue::Map(result)
            }
            _ => PropertyValue::String(String::new()),
        }
    }
}

pub struct SchemaRegistry {
    pub(crate) schemas: HashMap<String, Schema>,
    extension_files: HashMap<String, ExtensionFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionFile {
    pub name: String,
    pub description: String,
    pub applies_to: Vec<String>,
    pub extensions: HashMap<String, ExtensionDefinition>,
}

impl SchemaRegistry {
    pub fn new() -> Self {
        Self {
            schemas: HashMap::new(),
            extension_files: HashMap::new(),
        }
    }
    
    pub fn register(&mut self, schema: Schema) {
        self.schemas.insert(schema.name.clone(), schema);
    }
    
    pub fn register_extension_file(&mut self, extension_file: ExtensionFile) {
        self.extension_files.insert(extension_file.name.clone(), extension_file);
    }
    
    pub fn get(&self, name: &str) -> Option<&Schema> {
        self.schemas.get(name)
    }
    
    pub fn get_with_extensions(&self, name: &str, extension_names: &[&str]) -> Option<Schema> {
        let mut schema = self.schemas.get(name)?.clone();
        
        for ext_name in extension_names {
            for (_, ext_file) in &self.extension_files {
                if ext_file.applies_to.contains(&schema.name) {
                    if let Some(ext_def) = ext_file.extensions.get(*ext_name) {
                        schema.extensions.insert(ext_name.to_string(), ext_def.clone());
                    }
                }
            }
        }
        
        Some(schema)
    }
    
    pub fn load_directory<P: AsRef<Path>>(&mut self, dir: P) -> Result<usize, SchemaError> {
        let mut count = 0;
        
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") ||
               path.extension().and_then(|s| s.to_str()) == Some("yml") {
                let content = std::fs::read_to_string(&path)?;
                
                if path.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.contains("extension"))
                    .unwrap_or(false) {
                    let ext_file: ExtensionFile = serde_yaml::from_str(&content)?;
                    self.register_extension_file(ext_file);
                } else {
                    // Try to parse as a regular schema, skip if it fails
                    match SchemaParser::parse_str(&content) {
                        Ok(schema) => self.register(schema),
                        Err(e) => {
                            eprintln!("Warning: Skipping schema file {} - {}", path.display(), e);
                            continue;
                        }
                    }
                }
                count += 1;
            }
        }
        
        Ok(count)
    }
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_extensible_schema() {
        let yaml = r#"
name: "Devanagari"
type: abugida

element_types:
  consonant:
    description: "Basic consonant"
    properties:
      has_inherent_vowel:
        type: bool
        default: true
        description: "Whether the consonant has inherent 'a' vowel"
      aspirated:
        type: bool
        default: false
        description: "Whether the consonant is aspirated"
  
  vedic_accent:
    description: "Vedic accent marks"
    inherits_from: "modifier"

mappings:
  consonants:
    "क":
      canonical: "ka"
      type: consonant
      aspirated: false
    "ख":
      canonical: "kha"
      type: consonant
      aspirated: true
  
  vowels:
    "अ":
      canonical: "a"
      type: vowel_independent
      inherent: true
    "आ":
      canonical: "ā"
      type: vowel_independent
      dependent_form: "ा"

extensions:
  vedic_accents:
    description: "Vedic accent support"
    priority: 10
    mappings:
      "॑":
        to: "॑"
        element_type: "vedic_accent"
        properties:
          accent_type: "udatta"
"#;
        
        let schema = SchemaParser::parse_str(yaml).unwrap();
        assert_eq!(schema.name, "Devanagari");
        assert!(matches!(schema.script_type, ScriptType::Abugida));
        assert_eq!(schema.element_types.len(), 2);
        assert_eq!(schema.mappings["consonants"].len(), 2);
        assert_eq!(schema.extensions.len(), 1);
    }
    
    #[test]
    fn test_extension_file() {
        let yaml = r#"
name: "vedic_extensions"
description: "Extensions for Vedic Sanskrit"
applies_to: ["Devanagari", "IAST", "Harvard-Kyoto"]

extensions:
  vedic_accents:
    description: "Vedic accent marks"
    priority: 10
    mappings:
      "॑":
        to: "॑"
        properties:
          accent: "udatta"
      "॒":
        to: "॒"
        properties:
          accent: "anudatta"
"#;
        
        let ext_file: ExtensionFile = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(ext_file.name, "vedic_extensions");
        assert_eq!(ext_file.applies_to.len(), 3);
        assert_eq!(ext_file.extensions.len(), 1);
    }
}