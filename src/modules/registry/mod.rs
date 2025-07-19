use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

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

/// Statistics about the schema registry
#[derive(Debug, Clone)]
pub struct RegistryStats {
    /// Total number of registered schemas
    pub total_schemas: usize,
    /// Number of Roman script schemas
    pub roman_scripts: usize,
    /// Number of Brahmic script schemas  
    pub brahmic_scripts: usize,
    /// Number of schemas with implicit 'a' vowels
    pub implicit_a_scripts: usize,
    /// Number of schemas currently cached
    pub cached_schemas: usize,
    /// Total number of mappings across all schemas
    pub total_mappings: usize,
}

/// Represents metadata about a schema (unified format matching build system)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaMetadata {
    pub name: String,
    pub script_type: String,
    pub has_implicit_a: bool,
    pub description: Option<String>,
    pub aliases: Option<Vec<String>>,
}

impl Default for SchemaMetadata {
    fn default() -> Self {
        Self {
            name: String::new(),
            script_type: "roman".to_string(),
            has_implicit_a: false,
            description: None,
            aliases: None,
        }
    }
}

/// Script mappings structure (matches build system)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaMapping {
    pub vowels: Option<FxHashMap<String, String>>,
    pub consonants: Option<FxHashMap<String, String>>,
    pub vowel_signs: Option<FxHashMap<String, String>>,
    pub marks: Option<FxHashMap<String, String>>,
    pub digits: Option<FxHashMap<String, String>>,
    pub sanskrit_extensions: Option<FxHashMap<String, String>>,
    pub special: Option<FxHashMap<String, String>>,
}

/// Code generation configuration (optional)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodegenConfig {
    pub mapping_type: Option<String>,
    pub processor_type: Option<String>,
}

/// Represents a complete schema loaded from YAML (unified format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaFile {
    pub metadata: SchemaMetadata,
    pub target: Option<String>, // "iso15919" for Roman, "devanagari" for Indic (default)
    pub mappings: SchemaMapping,
    pub codegen: Option<CodegenConfig>,
}

/// Represents a schema in the registry
#[derive(Debug, Clone)]
pub struct Schema {
    pub name: String,
    pub script_type: String,
    pub target: String,
    pub mappings: FxHashMap<String, String>,
    pub metadata: SchemaMetadata,
}

impl Schema {
    pub fn new(name: String, script_type: String) -> Self {
        Self {
            name: name.clone(),
            script_type: script_type.clone(),
            target: if script_type == "roman" {
                "iso15919".to_string()
            } else {
                "devanagari".to_string()
            },
            mappings: FxHashMap::default(),
            metadata: SchemaMetadata {
                name,
                script_type,
                has_implicit_a: false,
                description: None,
                aliases: None,
            },
        }
    }

    /// Create a Schema from a loaded SchemaFile
    pub fn from_schema_file(schema_file: SchemaFile) -> Result<Self, RegistryError> {
        // Flatten the nested mappings structure
        let mut flattened_mappings = FxHashMap::default();

        // Flatten vowels
        if let Some(vowels) = &schema_file.mappings.vowels {
            flattened_mappings.extend(vowels.clone());
        }

        // Flatten consonants
        if let Some(consonants) = &schema_file.mappings.consonants {
            flattened_mappings.extend(consonants.clone());
        }

        // Flatten vowel signs
        if let Some(vowel_signs) = &schema_file.mappings.vowel_signs {
            flattened_mappings.extend(vowel_signs.clone());
        }

        // Flatten marks
        if let Some(marks) = &schema_file.mappings.marks {
            flattened_mappings.extend(marks.clone());
        }

        // Flatten digits
        if let Some(digits) = &schema_file.mappings.digits {
            flattened_mappings.extend(digits.clone());
        }

        // Flatten sanskrit extensions
        if let Some(sanskrit_extensions) = &schema_file.mappings.sanskrit_extensions {
            flattened_mappings.extend(sanskrit_extensions.clone());
        }

        // Flatten special characters
        if let Some(special) = &schema_file.mappings.special {
            flattened_mappings.extend(special.clone());
        }

        let target = schema_file.target.unwrap_or_else(|| {
            if schema_file.metadata.script_type == "roman" {
                "iso15919".to_string()
            } else {
                "devanagari".to_string()
            }
        });

        Ok(Self {
            name: schema_file.metadata.name.clone(),
            script_type: schema_file.metadata.script_type.clone(),
            target,
            mappings: flattened_mappings,
            metadata: schema_file.metadata,
        })
    }
}

pub trait SchemaRegistryTrait {
    fn get_schema(&self, script_name: &str) -> Option<&Schema>;
    fn register_schema(&mut self, name: String, schema: Schema) -> Result<(), RegistryError>;
    fn add_schema(&mut self, name: String, schema: Schema) -> Result<(), RegistryError>;
    fn load_schema(&mut self, schema_path: &str) -> Result<(), RegistryError>;
    fn load_schema_from_string(
        &mut self,
        yaml_content: &str,
        schema_name: &str,
    ) -> Result<(), RegistryError>;
    fn list_schemas(&self) -> Vec<&str>;
    fn list_schemas_owned(&self) -> Vec<String>;
    fn validate_schema(&self, schema: &Schema) -> Result<(), RegistryError>;
    fn remove_schema(&mut self, script_name: &str) -> bool;
    fn clear(&mut self);

    /// Get the count of registered schemas
    fn schema_count(&self) -> usize;

    /// Check if a schema with given name exists
    fn has_schema(&self, script_name: &str) -> bool;

    /// Get schema metadata without returning the full schema
    fn get_schema_metadata(&self, script_name: &str) -> Option<&SchemaMetadata>;

    /// Get statistics about the registry
    fn get_registry_stats(&self) -> RegistryStats;
}

#[derive(Clone)]
pub struct SchemaRegistry {
    schemas: FxHashMap<String, Schema>,
    schema_cache: FxHashMap<String, SchemaFile>,
}

impl SchemaRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            schemas: FxHashMap::default(),
            schema_cache: FxHashMap::default(),
        };

        // Register built-in schemas
        registry.register_builtin_schemas();
        registry
    }

    fn register_builtin_schemas(&mut self) {
        // Register core schemas that are always available
        let devanagari_schema = Schema::new("devanagari".to_string(), "brahmic".to_string());
        let iso_schema = Schema::new("iso15919".to_string(), "roman".to_string());

        // For now, register empty schemas as placeholders
        let _ = self.register_schema("devanagari".to_string(), devanagari_schema);
        let _ = self.register_schema("iso15919".to_string(), iso_schema);
    }

    /// Load a schema from a YAML file
    fn load_schema_from_file(&mut self, path: &Path) -> Result<Schema, RegistryError> {
        // Read the file
        let contents = fs::read_to_string(path)
            .map_err(|e| RegistryError::IoError(format!("Failed to read file: {e}")))?;

        // Parse YAML
        let schema_file: SchemaFile = serde_yaml::from_str(&contents)
            .map_err(|e| RegistryError::ParseError(format!("Failed to parse YAML: {e}")))?;

        // Cache the schema file
        self.schema_cache
            .insert(schema_file.metadata.name.clone(), schema_file.clone());

        // Convert to Schema
        Schema::from_schema_file(schema_file)
    }

    /// Load all schemas from a directory
    pub fn load_schemas_from_directory(&mut self, dir_path: &str) -> Result<usize, RegistryError> {
        let dir = Path::new(dir_path);

        if !dir.is_dir() {
            return Err(RegistryError::LoadFailed(format!(
                "Not a directory: {dir_path}"
            )));
        }

        let mut loaded_count = 0;

        // Walk through directory recursively
        for entry in fs::read_dir(dir)
            .map_err(|e| RegistryError::IoError(format!("Failed to read directory: {e}")))?
        {
            let entry = entry.map_err(|e| {
                RegistryError::IoError(format!("Failed to read directory entry: {e}"))
            })?;
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
                                eprintln!("Warning: Failed to load schema from {path:?}: {e}");
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

    /// Get schemas by script type
    pub fn get_schemas_by_type(&self, script_type: &str) -> Vec<&Schema> {
        self.schemas
            .values()
            .filter(|schema| schema.script_type == script_type)
            .collect()
    }

    /// Get all schemas with implicit 'a' vowels
    pub fn get_implicit_a_schemas(&self) -> Vec<&Schema> {
        self.schemas
            .values()
            .filter(|schema| schema.metadata.has_implicit_a)
            .collect()
    }

    /// Find schemas by alias
    pub fn find_schema_by_alias(&self, alias: &str) -> Option<&Schema> {
        self.schemas.values().find(|schema| {
            schema
                .metadata
                .aliases
                .as_ref()
                .map(|aliases| aliases.contains(&alias.to_string()))
                .unwrap_or(false)
        })
    }

    /// Check if registry is empty (only built-in schemas)
    pub fn is_empty(&self) -> bool {
        // Consider empty if only built-in schemas remain
        self.schemas.len() <= 2 // devanagari and iso15919
    }

    /// Export registry configuration as YAML (useful for debugging)
    pub fn export_summary(&self) -> String {
        let stats = self.get_registry_stats();
        format!(
            "Registry Summary:\n\
            - Total schemas: {}\n\
            - Roman scripts: {}\n\
            - Brahmic scripts: {}\n\
            - Schemas with implicit 'a': {}\n\
            - Cached schemas: {}\n\
            - Total mappings: {}\n\
            - Schema names: [{}]",
            stats.total_schemas,
            stats.roman_scripts,
            stats.brahmic_scripts,
            stats.implicit_a_scripts,
            stats.cached_schemas,
            stats.total_mappings,
            self.list_schemas().join(", ")
        )
    }
}

impl SchemaRegistryTrait for SchemaRegistry {
    fn get_schema(&self, script_name: &str) -> Option<&Schema> {
        // First try exact name match
        if let Some(schema) = self.schemas.get(script_name) {
            return Some(schema);
        }

        // If not found, try alias lookup
        self.find_schema_by_alias(script_name)
    }

    fn register_schema(&mut self, name: String, schema: Schema) -> Result<(), RegistryError> {
        // Validate the schema before registration
        self.validate_schema(&schema)?;

        self.schemas.insert(name, schema);
        Ok(())
    }

    fn add_schema(&mut self, name: String, schema: Schema) -> Result<(), RegistryError> {
        // Same as register_schema for now
        self.register_schema(name, schema)
    }

    fn load_schema(&mut self, schema_path: &str) -> Result<(), RegistryError> {
        let path = Path::new(schema_path);

        if !path.exists() {
            return Err(RegistryError::LoadFailed(format!(
                "Schema file not found: {schema_path}"
            )));
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
            return Err(RegistryError::InvalidSchema(
                "Schema name cannot be empty".to_string(),
            ));
        }

        if schema.script_type.is_empty() {
            return Err(RegistryError::InvalidSchema(
                "Script type cannot be empty".to_string(),
            ));
        }

        // Validate script type
        if !["roman", "brahmic"].contains(&schema.script_type.as_str()) {
            return Err(RegistryError::InvalidSchema(
                "Invalid script type".to_string(),
            ));
        }

        Ok(())
    }

    fn load_schema_from_string(
        &mut self,
        yaml_content: &str,
        schema_name: &str,
    ) -> Result<(), RegistryError> {
        // Parse YAML content
        let schema_file: SchemaFile = serde_yaml::from_str(yaml_content)
            .map_err(|e| RegistryError::ParseError(format!("Failed to parse YAML: {e}")))?;

        // Create schema from parsed content
        let mut schema = Schema::from_schema_file(schema_file)?;

        // Override name if provided
        if !schema_name.is_empty() {
            schema.name = schema_name.to_string();
        }

        // Register the schema
        let name = schema.name.clone();
        self.register_schema(name, schema)
    }

    fn list_schemas_owned(&self) -> Vec<String> {
        let mut schemas: Vec<String> = self.schemas.keys().cloned().collect();
        schemas.sort();
        schemas
    }

    fn remove_schema(&mut self, script_name: &str) -> bool {
        self.schemas.remove(script_name).is_some()
    }

    fn clear(&mut self) {
        self.schemas.clear();
        self.schema_cache.clear();
    }

    fn schema_count(&self) -> usize {
        self.schemas.len()
    }

    fn has_schema(&self, script_name: &str) -> bool {
        // Check both exact name and alias
        self.schemas.contains_key(script_name) || self.find_schema_by_alias(script_name).is_some()
    }

    fn get_schema_metadata(&self, script_name: &str) -> Option<&SchemaMetadata> {
        // Use get_schema to support both exact names and aliases
        self.get_schema(script_name).map(|schema| &schema.metadata)
    }

    fn get_registry_stats(&self) -> RegistryStats {
        let total_schemas = self.schemas.len();
        let roman_scripts = self
            .schemas
            .values()
            .filter(|schema| schema.script_type == "roman")
            .count();
        let brahmic_scripts = self
            .schemas
            .values()
            .filter(|schema| schema.script_type == "brahmic")
            .count();
        let implicit_a_scripts = self
            .schemas
            .values()
            .filter(|schema| schema.metadata.has_implicit_a)
            .count();
        let cached_schemas = self.schema_cache.len();
        let total_mappings = self
            .schemas
            .values()
            .map(|schema| schema.mappings.len())
            .sum();

        RegistryStats {
            total_schemas,
            roman_scripts,
            brahmic_scripts,
            implicit_a_scripts,
            cached_schemas,
            total_mappings,
        }
    }
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

mod error_tests;

#[cfg(test)]
mod validation_tests;

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
            script_type: "roman".to_string(),
            target: "iso15919".to_string(),
            mappings: FxHashMap::default(),
            metadata: SchemaMetadata {
                name: "test".to_string(),
                script_type: "roman".to_string(),
                has_implicit_a: false,
                description: None,
                aliases: None,
            },
        };

        assert!(registry
            .register_schema("test".to_string(), test_schema)
            .is_ok());
        assert!(registry.get_schema("test").is_some());
    }

    #[test]
    fn test_schema_validation() {
        let registry = SchemaRegistry::new();

        // Test empty name
        let invalid_schema = Schema {
            name: "".to_string(),
            script_type: "roman".to_string(),
            target: "iso15919".to_string(),
            mappings: FxHashMap::default(),
            metadata: SchemaMetadata::default(),
        };

        assert!(registry.validate_schema(&invalid_schema).is_err());

        // Test invalid script type
        let invalid_script_type_schema = Schema {
            name: "test".to_string(),
            script_type: "invalid".to_string(),
            target: "iso15919".to_string(),
            mappings: FxHashMap::default(),
            metadata: SchemaMetadata::default(),
        };

        assert!(registry
            .validate_schema(&invalid_script_type_schema)
            .is_err());
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
            assert_eq!(schema.script_type, "roman");
            assert_eq!(schema.target, "devanagari");
            assert!(!schema.metadata.has_implicit_a);
        }
    }

    #[test]
    fn test_list_schemas_sorted() {
        let mut registry = SchemaRegistry::new();

        // Add schemas in non-alphabetical order
        let schema1 = Schema::new("zulu".to_string(), "brahmic".to_string());
        let schema2 = Schema::new("arabic".to_string(), "roman".to_string());

        registry
            .register_schema("zulu".to_string(), schema1)
            .unwrap();
        registry
            .register_schema("arabic".to_string(), schema2)
            .unwrap();

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
        schema.metadata.description = Some("Test description".to_string());
        schema.metadata.aliases = Some(vec!["test_alias".to_string()]);

        registry
            .register_schema("test_meta".to_string(), schema)
            .unwrap();

        // Retrieve and verify metadata
        let retrieved = registry.get_schema("test_meta").unwrap();
        assert!(retrieved.metadata.has_implicit_a);
        assert_eq!(
            retrieved.metadata.description,
            Some("Test description".to_string())
        );
        assert_eq!(
            retrieved.metadata.aliases,
            Some(vec!["test_alias".to_string()])
        );
    }

    #[test]
    fn test_new_interface_methods() {
        let mut registry = SchemaRegistry::new();

        // Test schema count
        let initial_count = registry.schema_count();
        assert_eq!(initial_count, 2); // Built-in schemas

        // Add a test schema
        let test_schema = Schema::new("test_interface".to_string(), "roman".to_string());
        registry
            .register_schema("test_interface".to_string(), test_schema)
            .unwrap();

        // Test has_schema
        assert!(registry.has_schema("test_interface"));
        assert!(!registry.has_schema("nonexistent"));

        // Test schema count after addition
        assert_eq!(registry.schema_count(), 3);

        // Test get_schema_metadata
        let metadata = registry.get_schema_metadata("test_interface");
        assert!(metadata.is_some());
        assert_eq!(metadata.unwrap().name, "test_interface");

        // Test get_registry_stats
        let stats = registry.get_registry_stats();
        assert_eq!(stats.total_schemas, 3);
        assert!(stats.roman_scripts >= 1);

        // Test get_schemas_by_type
        let roman_schemas = registry.get_schemas_by_type("roman");
        assert!(!roman_schemas.is_empty());

        // Test export_summary
        let summary = registry.export_summary();
        assert!(summary.contains("Registry Summary"));
        assert!(summary.contains("test_interface"));

        // Test is_empty (should be false after adding schemas)
        assert!(!registry.is_empty());
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
            assert_eq!(cached.metadata.name, "sample");
            assert_eq!(cached.metadata.script_type, "roman");
        }
    }
}
