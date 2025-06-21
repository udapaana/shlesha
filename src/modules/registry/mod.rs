use std::collections::HashMap;
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
}

#[derive(Debug, Clone)]
pub struct Schema {
    pub name: String,
    pub script_type: String,
    pub mappings: HashMap<String, String>,
}

impl Schema {
    pub fn new(name: String, script_type: String) -> Self {
        Self {
            name,
            script_type,
            mappings: HashMap::new(),
        }
    }
}

pub trait SchemaRegistryTrait {
    fn get_schema(&self, script_name: &str) -> Option<&Schema>;
    fn register_schema(&mut self, name: String, schema: Schema) -> Result<(), RegistryError>;
    fn load_schema(&mut self, schema_path: &str) -> Result<(), RegistryError>;
    fn list_schemas(&self) -> Vec<&str>;
}

pub struct SchemaRegistry {
    schemas: HashMap<String, Schema>,
}

impl SchemaRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            schemas: HashMap::new(),
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
}

impl SchemaRegistryTrait for SchemaRegistry {
    fn get_schema(&self, script_name: &str) -> Option<&Schema> {
        self.schemas.get(script_name)
    }

    fn register_schema(&mut self, name: String, schema: Schema) -> Result<(), RegistryError> {
        self.schemas.insert(name, schema);
        Ok(())
    }

    fn load_schema(&mut self, _schema_path: &str) -> Result<(), RegistryError> {
        // TODO: Implement YAML schema loading
        Err(RegistryError::LoadFailed("Schema loading not yet implemented".to_string()))
    }

    fn list_schemas(&self) -> Vec<&str> {
        self.schemas.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// TODO List for Registry Module:
// - [ ] Implement YAML schema file loading
// - [ ] Add schema validation
// - [ ] Implement dynamic schema registration
// - [ ] Add schema versioning support
// - [ ] Implement schema caching for performance