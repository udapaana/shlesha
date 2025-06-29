//! Centralized mapping data management module
//!
//! This module provides a data-driven approach to managing script conversion mappings,
//! separating data from logic and enabling both runtime and compile-time access.

pub mod generated;
pub mod loader;
// pub mod codegen;
// pub mod build_support;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

use self::loader::{flatten_mappings, load_mapping_file};
use crate::modules::{ModuleTodoQueue, TodoItem, TodoResponse};

/// Mapping data types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingData {
    pub mappings: HashMap<String, String>,
    pub metadata: MappingMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingMetadata {
    pub script: String,
    pub script_type: String,
    pub has_implicit_a: bool,
}

/// Public interface for the mapping data module
pub struct MappingDataManager {
    todo_queue: ModuleTodoQueue,
    /// Cache of loaded mappings
    mapping_cache: Arc<Mutex<HashMap<String, MappingData>>>,
}

impl MappingDataManager {
    pub fn new(todo_queue: ModuleTodoQueue) -> Self {
        Self {
            todo_queue,
            mapping_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Initialize the module and load base mappings
    pub fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // For now, we'll use hardcoded mappings until we implement TOML loading
        // This follows our incremental development approach
        self.load_base_mappings()?;
        Ok(())
    }

    /// Process todos for this module
    pub fn process_todos(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(todo) = self.todo_queue.get_todo("mapping_data") {
            let response = match todo.action.as_str() {
                "get_mappings" => self.handle_get_mappings(&todo),
                "validate_mappings" => self.handle_validate_mappings(&todo),
                "compose_mappings" => self.handle_compose_mappings(&todo),
                _ => Err(format!("Unknown action: {}", todo.action).into()),
            };

            // Send response if channel provided
            if let Some(response_channel) = &todo.response_channel {
                let todo_response = match response {
                    Ok(data) => TodoResponse {
                        success: true,
                        data: Some(data),
                        error: None,
                    },
                    Err(e) => TodoResponse {
                        success: false,
                        data: None,
                        error: Some(e.to_string()),
                    },
                };
                let _ = response_channel.send(todo_response);
            }
        }
        Ok(())
    }

    /// Handle get_mappings request
    fn handle_get_mappings(&self, todo: &TodoItem) -> Result<Value, Box<dyn std::error::Error>> {
        let from_script = todo
            .data
            .get("from_script")
            .and_then(|v| v.as_str())
            .ok_or("Missing from_script")?;
        let to_script = todo
            .data
            .get("to_script")
            .and_then(|v| v.as_str())
            .ok_or("Missing to_script")?;

        // For now, return empty mappings
        // TODO: Implement actual mapping lookup
        Ok(json!({
            "mappings": {},
            "metadata": {
                "from_script": from_script,
                "to_script": to_script,
            }
        }))
    }

    /// Handle validate_mappings request
    fn handle_validate_mappings(
        &self,
        _todo: &TodoItem,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        // TODO: Implement mapping validation
        Ok(json!({
            "valid": true,
            "errors": []
        }))
    }

    /// Handle compose_mappings request
    fn handle_compose_mappings(
        &self,
        todo: &TodoItem,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let _first_step = todo.data.get("first_step").ok_or("Missing first_step")?;
        let _second_step = todo.data.get("second_step").ok_or("Missing second_step")?;

        // TODO: Implement mapping composition
        Ok(json!({
            "composed": true,
            "mappings": {}
        }))
    }

    /// Load base mappings from TOML files
    fn load_base_mappings(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let base_dir = Path::new("mappings/base");

        if !base_dir.exists() {
            // If mappings directory doesn't exist, use hardcoded fallback
            return self.load_fallback_mappings();
        }

        let mut cache = self.mapping_cache.lock().unwrap();

        // Load ISO to Devanagari mappings
        if let Ok(mapping_file) = load_mapping_file(&base_dir.join("iso_devanagari.toml")) {
            if let Some(mappings) = &mapping_file.mappings {
                let flat_mappings = flatten_mappings(mappings);
                let mapping_data = MappingData {
                    metadata: MappingMetadata {
                        script: "iso_devanagari".to_string(),
                        script_type: "roman_to_indic".to_string(),
                        has_implicit_a: false,
                    },
                    mappings: flat_mappings,
                };
                cache.insert("iso_to_devanagari".to_string(), mapping_data);
            }
        }

        // Load IAST mappings
        if let Ok(mapping_file) = load_mapping_file(&base_dir.join("iast.toml")) {
            if let Some(to_iso) = &mapping_file.to_iso {
                let flat_mappings = flatten_mappings(to_iso);
                let mapping_data = MappingData {
                    metadata: MappingMetadata {
                        script: "iast".to_string(),
                        script_type: "roman".to_string(),
                        has_implicit_a: false,
                    },
                    mappings: flat_mappings,
                };
                cache.insert("iast_to_iso".to_string(), mapping_data);
            }
        }

        Ok(())
    }

    /// Fallback hardcoded mappings if TOML files not found
    fn load_fallback_mappings(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut cache = self.mapping_cache.lock().unwrap();

        // Add basic ISO to Devanagari mappings
        let iso_to_deva = MappingData {
            metadata: MappingMetadata {
                script: "iso".to_string(),
                script_type: "roman".to_string(),
                has_implicit_a: false,
            },
            mappings: vec![
                ("a", "अ"),
                ("ā", "आ"),
                ("i", "इ"),
                ("ī", "ई"),
                ("u", "उ"),
                ("ū", "ऊ"),
                ("r̥", "ऋ"),
                ("r̥̄", "ॠ"),
                ("l̥", "ऌ"),
                ("l̥̄", "ॡ"),
                ("e", "ए"),
                ("ai", "ऐ"),
                ("o", "ओ"),
                ("au", "औ"),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        };

        cache.insert("iso_to_devanagari".to_string(), iso_to_deva);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapping_data_manager_creation() {
        let todo_queue = ModuleTodoQueue::new();
        let manager = MappingDataManager::new(todo_queue);
        assert!(manager.mapping_cache.lock().unwrap().is_empty());
    }
}
