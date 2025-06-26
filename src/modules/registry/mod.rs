use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Error, Debug, Clone)]
pub enum RegistryError {
    #[error("Schema not found: {0}")]
    SchemaNotFound(String),
    #[error("Load failed: {0}")]
    LoadFailed(String),
    #[error("Invalid schema: {0}")]
    InvalidSchema(String),
    #[error("Registration failed: {0}")]
    RegistrationFailed(String),
    #[error("IO error: {0}")]
    IoError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
}

/// Represents metadata about a schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaMetadata {
    pub has_implicit_a: bool,
    pub direction: String,
    pub case_sensitive: bool,
}

impl Default for SchemaMetadata {
    fn default() -> Self {
        Self {
            has_implicit_a: false,
            direction: "ltr".to_string(),
            case_sensitive: true,
        }
    }
}

/// Validation rules for a schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRules {
    pub patterns: HashMap<String, String>,
}

/// Represents a complete schema loaded from YAML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaFile {
    pub name: String,
    pub version: String,
    pub script_type: String,
    pub description: String,
    pub author: String,
    pub metadata: SchemaMetadata,
    pub mappings: HashMap<String, HashMap<String, String>>,
    pub validation: ValidationRules,
}

/// Represents a schema in the registry
#[derive(Debug, Clone)]
pub struct Schema {
    pub name: String,
    pub script_type: String,
    pub version: String,
    pub mappings: HashMap<String, String>,
    pub metadata: SchemaMetadata,
}

impl Schema {
    pub fn new(name: String, script_type: String) -> Self {
        Self {
            name,
            script_type,
            version: "1.0.0".to_string(),
            mappings: HashMap::new(),
            metadata: SchemaMetadata::default(),
        }
    }
    
    /// Create a Schema from a loaded SchemaFile
    pub fn from_schema_file(schema_file: SchemaFile) -> Result<Self, RegistryError> {
        // Flatten the nested mappings structure
        let mut flattened_mappings = HashMap::new();
        
        for (_category, mappings) in schema_file.mappings {
            for (key, value) in mappings {
                flattened_mappings.insert(key, value);
            }
        }
        
        Ok(Self {
            name: schema_file.name,
            script_type: schema_file.script_type,
            version: schema_file.version,
            mappings: flattened_mappings,
            metadata: schema_file.metadata,
        })
    }
}

pub trait SchemaRegistryTrait {
    fn get_schema(&self, script_name: &str) -> Option<&Schema>;
    fn register_schema(&mut self, name: String, schema: Schema) -> Result<(), RegistryError>;
    fn load_schema(&mut self, schema_path: &str) -> Result<(), RegistryError>;
    fn list_schemas(&self) -> Vec<&str>;
    fn validate_schema(&self, schema: &Schema) -> Result<(), RegistryError>;
}

#[derive(Clone)]
pub struct SchemaRegistry {
    schemas: HashMap<String, Schema>,
    schema_cache: HashMap<String, SchemaFile>,
}

impl SchemaRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            schemas: HashMap::new(),
            schema_cache: HashMap::new(),
        };
        
        // Register built-in schemas
        registry.register_builtin_schemas();
        registry
    }

    fn register_builtin_schemas(&mut self) {
        // Register core schemas that are always available
        let devanagari_schema = Schema::new("devanagari".to_string(), "brahmic".to_string());
        let iso_schema = Schema::new("iso15919".to_string(), "romanized".to_string());
        
        // For now, register empty schemas as placeholders
        let _ = self.register_schema("devanagari".to_string(), devanagari_schema);
        let _ = self.register_schema("iso15919".to_string(), iso_schema);
    }
    
    /// Load a schema from a YAML file
    fn load_schema_from_file(&mut self, path: &Path) -> Result<Schema, RegistryError> {
        // Read the file
        let contents = fs::read_to_string(path)
            .map_err(|e| RegistryError::IoError(format!("Failed to read file: {}", e)))?;
        
        // Parse YAML
        let schema_file: SchemaFile = serde_yaml::from_str(&contents)
            .map_err(|e| RegistryError::ParseError(format!("Failed to parse YAML: {}", e)))?;
        
        // Cache the schema file
        self.schema_cache.insert(schema_file.name.clone(), schema_file.clone());
        
        // Convert to Schema
        Schema::from_schema_file(schema_file)
    }
    
    /// Load all schemas from a directory
    pub fn load_schemas_from_directory(&mut self, dir_path: &str) -> Result<usize, RegistryError> {
        let dir = Path::new(dir_path);
        
        if !dir.is_dir() {
            return Err(RegistryError::LoadFailed(format!("Not a directory: {}", dir_path)));
        }
        
        let mut loaded_count = 0;
        
        // Walk through directory recursively
        for entry in fs::read_dir(dir)
            .map_err(|e| RegistryError::IoError(format!("Failed to read directory: {}", e)))? 
        {
            let entry = entry
                .map_err(|e| RegistryError::IoError(format!("Failed to read directory entry: {}", e)))?;
            let path = entry.path();
            
            if path.is_file() {
                // Check if it's a YAML file
                if let Some(ext) = path.extension() {
                    if ext == "yaml" || ext == "yml" {
                        // Try to load the schema
                        match self.load_schema(path.to_str().unwrap_or("")) {
                            Ok(_) => loaded_count += 1,
                            Err(e) => {
                                // Log error but continue loading other schemas
                                eprintln!("Warning: Failed to load schema from {:?}: {}", path, e);
                            }
                        }
                    }
                }
            } else if path.is_dir() {
                // Recursively load from subdirectories
                if let Ok(count) = self.load_schemas_from_directory(path.to_str().unwrap_or("")) {
                    loaded_count += count;
                }
            }
        }
        
        Ok(loaded_count)
    }
}

impl SchemaRegistryTrait for SchemaRegistry {
    fn get_schema(&self, script_name: &str) -> Option<&Schema> {
        self.schemas.get(script_name)
    }

    fn register_schema(&mut self, name: String, schema: Schema) -> Result<(), RegistryError> {
        // Validate the schema before registration
        self.validate_schema(&schema)?;
        
        self.schemas.insert(name, schema);
        Ok(())
    }

    fn load_schema(&mut self, schema_path: &str) -> Result<(), RegistryError> {
        let path = Path::new(schema_path);
        
        if !path.exists() {
            return Err(RegistryError::LoadFailed(format!("Schema file not found: {}", schema_path)));
        }
        
        let schema = self.load_schema_from_file(path)?;
        let name = schema.name.clone();
        
        self.register_schema(name, schema)
    }

    fn list_schemas(&self) -> Vec<&str> {
        let mut schemas: Vec<&str> = self.schemas.keys().map(|s| s.as_str()).collect();
        schemas.sort();
        schemas
    }
    
    fn validate_schema(&self, schema: &Schema) -> Result<(), RegistryError> {
        // Basic validation rules
        if schema.name.is_empty() {
            return Err(RegistryError::InvalidSchema("Schema name cannot be empty".to_string()));
        }
        
        if schema.script_type.is_empty() {
            return Err(RegistryError::InvalidSchema("Script type cannot be empty".to_string()));
        }
        
        // Validate version format (basic check)
        if !schema.version.contains('.') {
            return Err(RegistryError::InvalidSchema("Invalid version format".to_string()));
        }
        
        Ok(())
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
    fn test_schema_registry_creation() {
        let registry = SchemaRegistry::new();
        
        // Should have built-in schemas
        let schemas = registry.list_schemas();
        assert!(schemas.contains(&"devanagari"));
        assert!(schemas.contains(&"iso15919"));
    }
    
    #[test]
    fn test_schema_registration() {
        let mut registry = SchemaRegistry::new();
        
        let test_schema = Schema {
            name: "test".to_string(),
            script_type: "romanized".to_string(),
            version: "1.0.0".to_string(),
            mappings: HashMap::new(),
            metadata: SchemaMetadata::default(),
        };
        
        assert!(registry.register_schema("test".to_string(), test_schema).is_ok());
        assert!(registry.get_schema("test").is_some());
    }
    
    #[test]
    fn test_schema_validation() {
        let registry = SchemaRegistry::new();
        
        // Test empty name
        let invalid_schema = Schema {
            name: "".to_string(),
            script_type: "romanized".to_string(),
            version: "1.0.0".to_string(),
            mappings: HashMap::new(),
            metadata: SchemaMetadata::default(),
        };
        
        assert!(registry.validate_schema(&invalid_schema).is_err());
        
        // Test invalid version
        let invalid_version_schema = Schema {
            name: "test".to_string(),
            script_type: "romanized".to_string(),
            version: "100".to_string(),
            mappings: HashMap::new(),
            metadata: SchemaMetadata::default(),
        };
        
        assert!(registry.validate_schema(&invalid_version_schema).is_err());
    }
    
    #[test]
    fn test_load_schema_from_yaml() {
        let mut registry = SchemaRegistry::new();
        
        // This test will only work if the test schema file exists
        let test_path = "schemas/test/sample_schema.yaml";
        if std::path::Path::new(test_path).exists() {
            let result = registry.load_schema(test_path);
            assert!(result.is_ok());
            
            // Verify the schema was loaded
            let schema = registry.get_schema("sample");
            assert!(schema.is_some());
            
            let schema = schema.unwrap();
            assert_eq!(schema.name, "sample");
            assert_eq!(schema.version, "1.0.0");
            assert_eq!(schema.script_type, "romanized");
            assert!(!schema.metadata.has_implicit_a);
        }
    }
    
    #[test] 
    fn test_list_schemas_sorted() {
        let mut registry = SchemaRegistry::new();
        
        // Add schemas in non-alphabetical order
        let schema1 = Schema::new("zulu".to_string(), "brahmic".to_string());
        let schema2 = Schema::new("arabic".to_string(), "abjad".to_string());
        
        registry.register_schema("zulu".to_string(), schema1).unwrap();
        registry.register_schema("arabic".to_string(), schema2).unwrap();
        
        let schemas = registry.list_schemas();
        
        // Should be sorted alphabetically
        assert_eq!(schemas[0], "arabic");
        assert_eq!(schemas[1], "devanagari");
        assert_eq!(schemas[2], "iso15919");
        assert_eq!(schemas[3], "zulu");
    }
    
    #[test]
    fn test_load_schemas_from_directory() {
        let mut registry = SchemaRegistry::new();
        
        // This test will only work if the schemas directory exists
        let test_dir = "schemas";
        if std::path::Path::new(test_dir).exists() {
            let result = registry.load_schemas_from_directory(test_dir);
            assert!(result.is_ok());
            
            let count = result.unwrap();
            assert!(count > 0); // Should have loaded at least one schema
            
            // Verify the sample schema was loaded
            assert!(registry.get_schema("sample").is_some());
        }
    }
    
    #[test]
    fn test_schema_metadata() {
        let mut registry = SchemaRegistry::new();
        
        // Create a schema with specific metadata
        let mut schema = Schema::new("test_meta".to_string(), "brahmic".to_string());
        schema.metadata.has_implicit_a = true;
        schema.metadata.direction = "rtl".to_string();
        schema.metadata.case_sensitive = false;
        
        registry.register_schema("test_meta".to_string(), schema).unwrap();
        
        // Retrieve and verify metadata
        let retrieved = registry.get_schema("test_meta").unwrap();
        assert!(retrieved.metadata.has_implicit_a);
        assert_eq!(retrieved.metadata.direction, "rtl");
        assert!(!retrieved.metadata.case_sensitive);
    }
    
    #[test]
    fn test_schema_caching() {
        let mut registry = SchemaRegistry::new();
        
        // Load a schema file
        let test_path = "schemas/test/sample_schema.yaml";
        if std::path::Path::new(test_path).exists() {
            registry.load_schema(test_path).unwrap();
            
            // Verify it's in the cache
            assert!(registry.schema_cache.contains_key("sample"));
            
            let cached = registry.schema_cache.get("sample").unwrap();
            assert_eq!(cached.version, "1.0.0");
            assert_eq!(cached.author, "Shlesha Test Suite");
        }
    }
}

// TODO List for Registry Module:
// - [x] Implement YAML schema file loading
// - [x] Add schema validation
// - [x] Implement dynamic schema registration (load from directory)
// - [ ] Add schema versioning support (handle multiple versions)
// - [x] Implement basic schema caching (HashMap cache)