//! Hot-Reload Manager
//!
//! This module manages dynamic loading and updating of optimizations at runtime:
//! - Watches optimization directory for changes
//! - Validates and loads new optimizations without restart
//! - Provides thread-safe access to current optimizations
//! - Supports rollback on failed loads

use super::{OptimizedLookupTable, Profiler};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, SystemTime};

/// Hot-reload manager for optimization tables
pub struct HotReloadManager {
    /// Directory to watch for optimization files
    watch_dir: PathBuf,
    /// Reference to the profiler
    profiler: Arc<Profiler>,
    /// Track last modification times
    last_check: Arc<RwLock<SystemTime>>,
    /// Enable/disable hot reloading
    enabled: Arc<RwLock<bool>>,
}

impl HotReloadManager {
    /// Create a new hot-reload manager
    pub fn new(watch_dir: PathBuf, profiler: Arc<Profiler>) -> Self {
        Self {
            watch_dir,
            profiler,
            last_check: Arc::new(RwLock::new(SystemTime::now())),
            enabled: Arc::new(RwLock::new(true)),
        }
    }

    /// Start watching for optimization updates in a background thread
    pub fn start_watching(self: Arc<Self>) {
        let manager = self.clone();

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(10)); // Check every 10 seconds

                if !*manager.enabled.read().unwrap() {
                    continue;
                }

                manager.check_for_updates();
            }
        });
    }

    /// Check for new or updated optimization files
    pub fn check_for_updates(&self) {
        let last_check = *self.last_check.read().unwrap();

        if let Ok(entries) = fs::read_dir(&self.watch_dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                // Only process JSON optimization files
                if path.extension().and_then(|s| s.to_str()) != Some("json") {
                    continue;
                }

                // Check if file is newer than last check
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        let modified_time = SystemTime::from(modified);

                        if modified_time > last_check {
                            self.try_load_optimization(&path);
                        }
                    }
                }
            }
        }

        *self.last_check.write().unwrap() = SystemTime::now();
    }

    /// Try to load an optimization file
    fn try_load_optimization(&self, path: &Path) {
        match fs::read_to_string(path) {
            Ok(content) => {
                match serde_json::from_str::<OptimizedLookupTable>(&content) {
                    Ok(optimization) => {
                        // Validate the optimization before loading
                        if self.validate_optimization(&optimization) {
                            self.profiler.load_optimization(optimization);
                            eprintln!("Hot-reloaded optimization from: {:?}", path);
                        } else {
                            eprintln!("Invalid optimization file: {:?}", path);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to parse optimization file {:?}: {}", path, e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read optimization file {:?}: {}", path, e);
            }
        }
    }

    /// Validate an optimization table before loading
    fn validate_optimization(&self, optimization: &OptimizedLookupTable) -> bool {
        // Basic validation checks
        if optimization.from_script.is_empty() || optimization.to_script.is_empty() {
            return false;
        }

        // Check that mappings are not empty
        if optimization.sequence_mappings.is_empty() && optimization.word_mappings.is_empty() {
            return false;
        }

        // Validate mapping entries
        for (key, value) in &optimization.sequence_mappings {
            if key.is_empty() || value.is_empty() {
                return false;
            }
        }

        for (key, value) in &optimization.word_mappings {
            if key.is_empty() || value.is_empty() {
                return false;
            }
        }

        true
    }

    /// Enable or disable hot reloading
    pub fn set_enabled(&self, enabled: bool) {
        *self.enabled.write().unwrap() = enabled;
    }

    /// Manually trigger a reload check
    pub fn reload_now(&self) {
        self.check_for_updates();
    }

    /// Get the watch directory
    pub fn watch_dir(&self) -> &Path {
        &self.watch_dir
    }
}

/// Optimization cache that integrates with the transliterator
pub struct OptimizationCache {
    /// Cached optimizations by conversion path
    cache: Arc<RwLock<FxHashMap<(String, String), OptimizedLookupTable>>>,
}

use rustc_hash::FxHashMap;

impl OptimizationCache {
    /// Create a new optimization cache
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(FxHashMap::default())),
        }
    }

    /// Get an optimization for a specific conversion path
    pub fn get(&self, from_script: &str, to_script: &str) -> Option<OptimizedLookupTable> {
        let cache = self.cache.read().unwrap();
        cache
            .get(&(from_script.to_string(), to_script.to_string()))
            .cloned()
    }

    /// Load an optimization into the cache
    pub fn load(&self, optimization: OptimizedLookupTable) {
        let mut cache = self.cache.write().unwrap();
        let key = (
            optimization.from_script.clone(),
            optimization.to_script.clone(),
        );
        cache.insert(key, optimization);
    }

    /// Clear all cached optimizations
    pub fn clear(&self) {
        let mut cache = self.cache.write().unwrap();
        cache.clear();
    }

    /// Get the number of cached optimizations
    pub fn size(&self) -> usize {
        let cache = self.cache.read().unwrap();
        cache.len()
    }

    /// Apply optimization to convert text
    pub fn apply_optimization<F>(
        &self,
        text: &str,
        from_script: &str,
        to_script: &str,
        fallback: F,
    ) -> Result<String, Box<dyn std::error::Error>>
    where
        F: Fn(&str) -> Result<String, Box<dyn std::error::Error>>,
    {
        if let Some(optimization) = self.get(from_script, to_script) {
            // Try to use optimized conversion
            let mut result = String::new();
            let mut chars = text.chars().peekable();
            let mut buffer = String::new();

            while let Some(ch) = chars.next() {
                buffer.push(ch);

                // Try to match against optimizations
                let mut matched = false;

                // Check word mappings for longer sequences
                if let Some(mapped) = optimization.word_mappings.get(&buffer) {
                    result.push_str(mapped);
                    buffer.clear();
                    matched = true;
                } else {
                    // Try sequence mappings
                    let chars: Vec<char> = buffer.chars().collect();
                    for len in (1..=chars.len()).rev() {
                        let seq = &chars[chars.len() - len..];
                        let seq_str: String = seq.iter().collect();
                        if let Some(mapped) = optimization.sequence_mappings.get(&seq_str) {
                            // Add any unmatched prefix
                            if chars.len() > len {
                                let prefix_chars = &chars[..chars.len() - len];
                                let prefix: String = prefix_chars.iter().collect();
                                result.push_str(&fallback(&prefix)?);
                            }
                            result.push_str(mapped);
                            buffer.clear();
                            matched = true;
                            break;
                        }
                    }
                }

                // If buffer is getting too long without matches, flush it
                if !matched && buffer.len() > 10 {
                    result.push_str(&fallback(&buffer)?);
                    buffer.clear();
                }
            }

            // Handle any remaining buffer
            if !buffer.is_empty() {
                result.push_str(&fallback(&buffer)?);
            }

            Ok(result)
        } else {
            // No optimization available, use fallback
            fallback(text)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::profiler::{OptimizationMetadata, ProfileStats};
    use tempfile::tempdir;

    #[test]
    fn test_hot_reload_manager_creation() {
        let temp_dir = tempdir().unwrap();
        let profiler = Arc::new(Profiler::new());
        let manager = HotReloadManager::new(temp_dir.path().to_path_buf(), profiler);

        assert_eq!(manager.watch_dir, temp_dir.path());
    }

    #[test]
    fn test_optimization_validation() {
        let temp_dir = tempdir().unwrap();
        let profiler = Arc::new(Profiler::new());
        let manager = HotReloadManager::new(temp_dir.path().to_path_buf(), profiler);

        // Valid optimization
        let mut valid_opt = OptimizedLookupTable {
            from_script: "devanagari".to_string(),
            to_script: "iast".to_string(),
            sequence_mappings: FxHashMap::default(),
            word_mappings: FxHashMap::default(),
            metadata: OptimizationMetadata {
                generated_at: SystemTime::now(),
                sequence_count: 1,
                min_frequency: 10,
                profile_stats: ProfileStats {
                    total_sequences_profiled: 100,
                    unique_sequences: 10,
                    top_sequences: vec![],
                },
            },
        };
        valid_opt
            .sequence_mappings
            .insert("धर्म".to_string(), "dharma".to_string());

        assert!(manager.validate_optimization(&valid_opt));

        // Invalid optimization (empty scripts)
        let mut invalid_opt = valid_opt.clone();
        invalid_opt.from_script = String::new();
        assert!(!manager.validate_optimization(&invalid_opt));

        // Invalid optimization (empty mappings)
        let mut invalid_opt = valid_opt.clone();
        invalid_opt.sequence_mappings.clear();
        invalid_opt.word_mappings.clear();
        assert!(!manager.validate_optimization(&invalid_opt));
    }

    #[test]
    fn test_optimization_cache() {
        let cache = OptimizationCache::new();

        let mut optimization = OptimizedLookupTable {
            from_script: "devanagari".to_string(),
            to_script: "iast".to_string(),
            sequence_mappings: FxHashMap::default(),
            word_mappings: FxHashMap::default(),
            metadata: OptimizationMetadata {
                generated_at: SystemTime::now(),
                sequence_count: 1,
                min_frequency: 10,
                profile_stats: ProfileStats {
                    total_sequences_profiled: 100,
                    unique_sequences: 10,
                    top_sequences: vec![],
                },
            },
        };
        optimization
            .sequence_mappings
            .insert("धर्म".to_string(), "dharma".to_string());

        cache.load(optimization.clone());
        assert_eq!(cache.size(), 1);

        let retrieved = cache.get("devanagari", "iast").unwrap();
        assert_eq!(retrieved.from_script, "devanagari");
        assert_eq!(retrieved.sequence_mappings["धर्म"], "dharma");
    }
}
