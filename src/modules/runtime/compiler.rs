// Runtime compilation is not supported in WASM environments
// This module requires filesystem access, process spawning (cargo), and dynamic library loading
#![cfg(not(target_arch = "wasm32"))]

use handlebars::Handlebars;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;
use thiserror::Error;

use super::cache::{CacheManager, CompilationCache};
use crate::modules::schema::Schema;

#[derive(Debug, Error)]
pub enum RuntimeCompilerError {
    #[error("Template error: {0}")]
    TemplateError(#[from] handlebars::RenderError),
    #[error("Template file error: {0}")]
    TemplateFileError(#[from] handlebars::TemplateError),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Cache error: {0}")]
    CacheError(#[from] super::cache::CacheError),
    #[error("Compilation error: {0}")]
    CompilationError(String),
    #[error("Library loading error: {0}")]
    LibraryLoadingError(String),
}

pub struct RuntimeCompiler {
    template_engine: Handlebars<'static>,
    cache_manager: CacheManager,
    temp_dir: Option<TempDir>,
}

impl RuntimeCompiler {
    pub fn new() -> Result<Self, RuntimeCompilerError> {
        let mut template_engine = Handlebars::new();

        // Load the same templates used by build.rs
        let template_path = "templates/token_based_converter.hbs";
        if Path::new(template_path).exists() {
            template_engine.register_template_file("token_based_converter", template_path)?;
        }

        let cache_manager = CacheManager::new()?;

        Ok(Self {
            template_engine,
            cache_manager,
            temp_dir: None,
        })
    }

    pub fn compile_schema(
        &mut self,
        schema: &Schema,
    ) -> Result<CompiledProcessor, RuntimeCompilerError> {
        // Check cache first
        let cache_key = self.cache_manager.generate_cache_key(schema);

        if let Some(cached) = self.cache_manager.get_cached(&cache_key)? {
            return Ok(CompiledProcessor::from_cache(cached));
        }

        // Generate code using the same template as build.rs
        let template_data = self.prepare_template_data(schema)?;
        let generated_code = self
            .template_engine
            .render("token_based_converter", &template_data)?;

        // Create temporary crate
        let temp_crate_dir = self.create_temp_crate(schema, &generated_code)?;

        // Compile to dylib
        let dylib_path = self.compile_to_dylib(&temp_crate_dir)?;

        // Cache the result
        let compilation_cache = CompilationCache {
            schema_hash: cache_key.clone(),
            dylib_path: dylib_path.clone(),
            generated_code: generated_code.clone(),
            metadata: schema.metadata.clone(),
        };

        self.cache_manager
            .store_cache(&cache_key, &compilation_cache)?;

        Ok(CompiledProcessor::new(dylib_path, schema.clone()))
    }

    fn prepare_template_data(&self, schema: &Schema) -> Result<Value, RuntimeCompilerError> {
        // Convert schema to the same format expected by the Handlebars template
        let mut template_data = serde_json::Map::new();

        // Basic metadata
        template_data.insert(
            "script_name".to_string(),
            Value::String(schema.metadata.name.clone()),
        );
        template_data.insert(
            "struct_name".to_string(),
            Value::String(format!("{}Converter", schema.metadata.name)),
        );

        // Determine if this is alphabet or abugida based on target
        let is_alphabet = schema.target == "alphabet_tokens";
        template_data.insert("is_alphabet".to_string(), Value::Bool(is_alphabet));

        // Convert mappings to template format
        let mut mappings = Vec::new();
        for (category, entries) in &schema.mappings {
            let mut category_mappings = serde_json::Map::new();
            category_mappings.insert("category".to_string(), Value::String(category.clone()));

            let mut entries_list = Vec::new();
            for (token, mapping) in entries {
                let mut entry = serde_json::Map::new();
                entry.insert("token".to_string(), Value::String(token.clone()));

                // Handle both single and multiple input mappings
                match mapping {
                    Value::String(single_input) => {
                        entry.insert("preferred".to_string(), Value::String(single_input.clone()));
                        entry.insert(
                            "all_inputs".to_string(),
                            Value::Array(vec![Value::String(single_input.clone())]),
                        );
                    }
                    Value::Array(multiple_inputs) => {
                        if let Some(first) = multiple_inputs.first() {
                            entry.insert("preferred".to_string(), first.clone());
                        }
                        entry.insert("all_inputs".to_string(), mapping.clone());
                    }
                    _ => {
                        return Err(RuntimeCompilerError::CompilationError(
                            "Invalid mapping format".to_string(),
                        ))
                    }
                }

                entries_list.push(Value::Object(entry));
            }

            category_mappings.insert("entries".to_string(), Value::Array(entries_list));
            mappings.push(Value::Object(category_mappings));
        }

        template_data.insert("mappings".to_string(), Value::Array(mappings));

        // Check if there are multi-character mappings
        let has_multi_char = schema
            .mappings
            .values()
            .flat_map(|entries| entries.values())
            .any(|mapping| match mapping {
                Value::String(s) => s.len() > 1,
                Value::Array(arr) => arr.iter().any(|v| {
                    if let Value::String(s) = v {
                        s.len() > 1
                    } else {
                        false
                    }
                }),
                _ => false,
            });

        template_data.insert(
            "has_multi_char_mappings".to_string(),
            Value::Bool(has_multi_char),
        );

        Ok(Value::Object(template_data))
    }

    fn create_temp_crate(
        &mut self,
        schema: &Schema,
        generated_code: &str,
    ) -> Result<PathBuf, RuntimeCompilerError> {
        let temp_dir = TempDir::new()?;
        let crate_dir = temp_dir.path().to_path_buf();

        // Create Cargo.toml
        let cargo_toml = format!(
            r#"
[package]
name = "{}_runtime"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
serde = {{ version = "1.0", features = ["derive"] }}
"#,
            schema.metadata.name
        );

        fs::write(crate_dir.join("Cargo.toml"), cargo_toml)?;

        // Create src directory
        let src_dir = crate_dir.join("src");
        fs::create_dir_all(&src_dir)?;

        // Write lib.rs with generated code
        let lib_rs = format!(
            r#"
// Runtime generated token processor for {}
use serde::{{Deserialize, Serialize}};

{}

// Export functions for FFI
#[no_mangle]
pub extern "C" fn convert_string_to_tokens(input: *const std::os::raw::c_char) -> *mut std::os::raw::c_char {{
    // Implementation for FFI interface
    std::ptr::null_mut()
}}

#[no_mangle]
pub extern "C" fn convert_tokens_to_string(tokens: *const std::os::raw::c_char) -> *mut std::os::raw::c_char {{
    // Implementation for FFI interface
    std::ptr::null_mut()
}}
"#,
            schema.metadata.name, generated_code
        );

        fs::write(src_dir.join("lib.rs"), lib_rs)?;

        // Store temp_dir to keep it alive
        self.temp_dir = Some(temp_dir);

        Ok(crate_dir)
    }

    fn compile_to_dylib(&self, crate_dir: &Path) -> Result<PathBuf, RuntimeCompilerError> {
        // Run cargo build --release
        let output = Command::new("cargo")
            .args(["build", "--release"])
            .current_dir(crate_dir)
            .output()?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(RuntimeCompilerError::CompilationError(format!(
                "Cargo build failed: {}",
                error_msg
            )));
        }

        // Find the compiled dylib
        let target_dir = crate_dir.join("target").join("release");

        // Platform-specific dylib extensions
        let dylib_extension = if cfg!(target_os = "macos") {
            "dylib"
        } else if cfg!(target_os = "windows") {
            "dll"
        } else {
            "so"
        };

        let dylib_files: Vec<_> = fs::read_dir(&target_dir)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext == dylib_extension)
                    .unwrap_or(false)
            })
            .collect();

        if dylib_files.is_empty() {
            return Err(RuntimeCompilerError::CompilationError(
                "No dylib found after compilation".to_string(),
            ));
        }

        Ok(dylib_files[0].path())
    }
}

/// Represents a compiled processor that can be loaded dynamically
pub struct CompiledProcessor {
    dylib_path: PathBuf,
    schema: Schema,
    // Future: loaded dylib handle for actual function calls
}

impl CompiledProcessor {
    fn new(dylib_path: PathBuf, schema: Schema) -> Self {
        Self { dylib_path, schema }
    }

    fn from_cache(cache: CompilationCache) -> Self {
        Self {
            dylib_path: cache.dylib_path,
            schema: Schema {
                metadata: cache.metadata,
                target: "unknown".to_string(), // Will be populated from cache metadata
                mappings: HashMap::new(),      // Will be populated from dylib
            },
        }
    }

    pub fn get_dylib_path(&self) -> &Path {
        &self.dylib_path
    }

    pub fn get_schema(&self) -> &Schema {
        &self.schema
    }
}

impl Default for RuntimeCompiler {
    fn default() -> Self {
        Self::new().expect("Failed to create RuntimeCompiler")
    }
}
