// Runtime compilation and caching are not supported in WASM environments
// This module requires filesystem access, dynamic library loading, and process spawning
#![cfg(not(target_arch = "wasm32"))]

use blake3::Hasher;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::modules::schema::{Schema, SchemaMetadata};

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Cache corruption: {0}")]
    CorruptionError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationCache {
    pub schema_hash: String,
    pub dylib_path: PathBuf,
    pub generated_code: String,
    pub metadata: SchemaMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
struct CacheIndex {
    entries: HashMap<String, CacheEntry>,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CacheEntry {
    schema_hash: String,
    dylib_path: PathBuf,
    source_path: PathBuf,
    metadata_path: PathBuf,
    created_at: u64,
    last_accessed: u64,
}

pub struct CacheManager {
    cache_dir: PathBuf,
    index: CacheIndex,
}

impl CacheManager {
    pub fn new() -> Result<Self, CacheError> {
        let cache_dir = Self::get_cache_directory()?;
        fs::create_dir_all(&cache_dir)?;

        // Create subdirectories
        fs::create_dir_all(cache_dir.join("compiled"))?;
        fs::create_dir_all(cache_dir.join("source"))?;

        let index = Self::load_or_create_index(&cache_dir)?;

        Ok(Self { cache_dir, index })
    }

    fn get_cache_directory() -> Result<PathBuf, CacheError> {
        let cache_base = if let Ok(xdg_cache) = std::env::var("XDG_CACHE_HOME") {
            PathBuf::from(xdg_cache)
        } else if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".cache")
        } else if let Ok(appdata) = std::env::var("APPDATA") {
            PathBuf::from(appdata)
        } else {
            return Err(CacheError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not determine cache directory",
            )));
        };

        Ok(cache_base.join("shlesha"))
    }

    fn load_or_create_index(cache_dir: &Path) -> Result<CacheIndex, CacheError> {
        let index_path = cache_dir.join("index.json");

        if index_path.exists() {
            let content = fs::read_to_string(&index_path)?;
            let index: CacheIndex = serde_json::from_str(&content)?;

            // Validate cache version compatibility
            if index.version != env!("CARGO_PKG_VERSION") {
                // Clear incompatible cache
                return Ok(CacheIndex {
                    entries: HashMap::new(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                });
            }

            Ok(index)
        } else {
            Ok(CacheIndex {
                entries: HashMap::new(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            })
        }
    }

    pub fn generate_cache_key(&self, schema: &Schema) -> String {
        let mut hasher = Hasher::new();

        // Hash schema content for deterministic cache key
        let schema_json = serde_json::to_string(schema).unwrap_or_default();
        hasher.update(schema_json.as_bytes());

        // Include Shlesha version to invalidate cache on updates
        hasher.update(env!("CARGO_PKG_VERSION").as_bytes());

        // Include template file hash if it exists
        if let Ok(template_content) = fs::read_to_string("templates/token_based_converter.hbs") {
            hasher.update(template_content.as_bytes());
        }

        hex::encode(hasher.finalize().as_bytes())
    }

    pub fn get_cached(&mut self, cache_key: &str) -> Result<Option<CompilationCache>, CacheError> {
        if let Some(entry) = self.index.entries.get_mut(cache_key) {
            // Check if files still exist
            if entry.dylib_path.exists() && entry.source_path.exists() {
                // Update last accessed time
                entry.last_accessed = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                // Load compilation cache
                let metadata_content = fs::read_to_string(&entry.metadata_path)?;
                let metadata: SchemaMetadata = serde_json::from_str(&metadata_content)?;

                let source_content = fs::read_to_string(&entry.source_path)?;

                return Ok(Some(CompilationCache {
                    schema_hash: cache_key.to_string(),
                    dylib_path: entry.dylib_path.clone(),
                    generated_code: source_content,
                    metadata,
                }));
            } else {
                // Remove invalid entry
                self.index.entries.remove(cache_key);
            }
        }

        Ok(None)
    }

    pub fn store_cache(
        &mut self,
        cache_key: &str,
        cache: &CompilationCache,
    ) -> Result<(), CacheError> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Generate file paths
        let dylib_dest = self
            .cache_dir
            .join("compiled")
            .join(format!("{}.dylib", cache_key));
        let source_dest = self
            .cache_dir
            .join("source")
            .join(format!("{}.rs", cache_key));
        let metadata_dest = self
            .cache_dir
            .join("compiled")
            .join(format!("{}.meta", cache_key));

        // Copy dylib to cache
        fs::copy(&cache.dylib_path, &dylib_dest)?;

        // Store generated source
        fs::write(&source_dest, &cache.generated_code)?;

        // Store metadata
        let metadata_json = serde_json::to_string_pretty(&cache.metadata)?;
        fs::write(&metadata_dest, metadata_json)?;

        // Update index
        let entry = CacheEntry {
            schema_hash: cache_key.to_string(),
            dylib_path: dylib_dest,
            source_path: source_dest,
            metadata_path: metadata_dest,
            created_at: timestamp,
            last_accessed: timestamp,
        };

        self.index.entries.insert(cache_key.to_string(), entry);
        self.save_index()?;

        Ok(())
    }

    fn save_index(&self) -> Result<(), CacheError> {
        let index_path = self.cache_dir.join("index.json");
        let index_json = serde_json::to_string_pretty(&self.index)?;
        fs::write(index_path, index_json)?;
        Ok(())
    }

    pub fn cleanup_old_entries(&mut self, max_age_days: u64) -> Result<(), CacheError> {
        let cutoff_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - (max_age_days * 24 * 60 * 60);

        let mut to_remove = Vec::new();

        for (key, entry) in &self.index.entries {
            if entry.last_accessed < cutoff_time {
                to_remove.push(key.clone());

                // Remove files
                let _ = fs::remove_file(&entry.dylib_path);
                let _ = fs::remove_file(&entry.source_path);
                let _ = fs::remove_file(&entry.metadata_path);
            }
        }

        for key in to_remove {
            self.index.entries.remove(&key);
        }

        self.save_index()?;
        Ok(())
    }

    pub fn clear_cache(&mut self) -> Result<(), CacheError> {
        // Remove all cache files
        for entry in self.index.entries.values() {
            let _ = fs::remove_file(&entry.dylib_path);
            let _ = fs::remove_file(&entry.source_path);
            let _ = fs::remove_file(&entry.metadata_path);
        }

        // Clear index
        self.index.entries.clear();
        self.save_index()?;

        Ok(())
    }
}

impl Drop for CacheManager {
    fn drop(&mut self) {
        // Save index on drop
        let _ = self.save_index();
    }
}
