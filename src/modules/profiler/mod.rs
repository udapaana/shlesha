//! Profile-Guided Optimization Module
//!
//! This module implements runtime profiling and optimization for Shlesha:
//! - Collects usage statistics on character sequences during transliteration
//! - Generates optimized lookup tables based on actual usage patterns
//! - Supports hot-reloading of optimizations without recompilation
//! - Focuses on frequently used Sanskrit/Hindi words and phrases

pub mod hot_reload;
pub mod optimizer;

pub use hot_reload::{HotReloadManager, OptimizationCache};
pub use optimizer::{OptimizationBenchmark, OptimizationGenerator};

use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime};

/// Usage statistics for a character sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceStats {
    /// The character sequence
    pub sequence: String,
    /// Number of times this sequence was encountered
    pub count: u64,
    /// Last time this sequence was used
    pub last_used: SystemTime,
    /// Average processing time in nanoseconds
    pub avg_processing_ns: f64,
}

/// Profile data for a specific conversion path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionProfile {
    /// Source script (e.g., "devanagari")
    pub from_script: String,
    /// Target script (e.g., "iso15919")
    pub to_script: String,
    /// Map of character sequences to their usage statistics
    pub sequences: FxHashMap<String, SequenceStats>,
    /// Total number of conversions profiled
    pub total_conversions: u64,
    /// Profile creation time
    pub created_at: SystemTime,
    /// Last update time
    pub updated_at: SystemTime,
}

/// Optimized lookup table for fast conversion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedLookupTable {
    /// Conversion path this table optimizes
    pub from_script: String,
    pub to_script: String,
    /// Direct sequence mappings (multi-char sequences)
    pub sequence_mappings: FxHashMap<String, String>,
    /// Common word mappings
    pub word_mappings: FxHashMap<String, String>,
    /// Metadata about the optimization
    pub metadata: OptimizationMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationMetadata {
    /// When this optimization was generated
    pub generated_at: SystemTime,
    /// Number of sequences in the optimization
    pub sequence_count: usize,
    /// Minimum sequence frequency to be included
    pub min_frequency: u64,
    /// Profile data used to generate this optimization
    pub profile_stats: ProfileStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileStats {
    pub total_sequences_profiled: u64,
    pub unique_sequences: usize,
    pub top_sequences: Vec<(String, u64)>,
}

/// Configuration for the profiler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilerConfig {
    /// Enable/disable profiling
    pub enabled: bool,
    /// Directory to store profile data
    pub profile_dir: PathBuf,
    /// Directory to store optimized tables
    pub optimization_dir: PathBuf,
    /// Minimum frequency for a sequence to be optimized
    pub min_sequence_frequency: u64,
    /// Maximum number of sequences to optimize per table
    pub max_sequences_per_table: usize,
    /// Auto-save interval for profiles
    pub auto_save_interval: Duration,
    /// Enable hot-reloading of optimizations
    pub hot_reload_enabled: bool,
}

impl Default for ProfilerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            profile_dir: PathBuf::from("profiles"),
            optimization_dir: PathBuf::from("optimizations"),
            min_sequence_frequency: 10,
            max_sequences_per_table: 1000,
            auto_save_interval: Duration::from_secs(300), // 5 minutes
            hot_reload_enabled: true,
        }
    }
}

/// Main profiler struct that manages runtime profiling
pub struct Profiler {
    config: ProfilerConfig,
    /// Active profiles being collected
    profiles: Arc<RwLock<FxHashMap<(String, String), ConversionProfile>>>,
    /// Currently loaded optimizations
    optimizations: Arc<RwLock<FxHashMap<(String, String), OptimizedLookupTable>>>,
    /// Last save time
    last_save_time: Arc<Mutex<Instant>>,
}

impl Profiler {
    /// Create a new profiler with default configuration
    pub fn new() -> Self {
        Self::with_config(ProfilerConfig::default())
    }

    /// Create a new profiler with custom configuration
    pub fn with_config(config: ProfilerConfig) -> Self {
        // Create directories if they don't exist
        if config.enabled {
            let _ = fs::create_dir_all(&config.profile_dir);
            let _ = fs::create_dir_all(&config.optimization_dir);
        }

        let profiler = Self {
            config,
            profiles: Arc::new(RwLock::new(FxHashMap::default())),
            optimizations: Arc::new(RwLock::new(FxHashMap::default())),
            last_save_time: Arc::new(Mutex::new(Instant::now())),
        };

        // Load existing profiles and optimizations
        profiler.load_profiles();
        profiler.load_optimizations();

        profiler
    }

    /// Record usage of a character sequence during conversion
    pub fn record_sequence(
        &self,
        from_script: &str,
        to_script: &str,
        sequence: &str,
        processing_time: Duration,
    ) {
        if !self.config.enabled {
            return;
        }

        let key = (from_script.to_string(), to_script.to_string());
        let mut profiles = self.profiles.write().unwrap();

        let profile = profiles
            .entry(key.clone())
            .or_insert_with(|| ConversionProfile {
                from_script: from_script.to_string(),
                to_script: to_script.to_string(),
                sequences: FxHashMap::default(),
                total_conversions: 0,
                created_at: SystemTime::now(),
                updated_at: SystemTime::now(),
            });

        profile.total_conversions += 1;
        profile.updated_at = SystemTime::now();

        let stats = profile
            .sequences
            .entry(sequence.to_string())
            .or_insert_with(|| SequenceStats {
                sequence: sequence.to_string(),
                count: 0,
                last_used: SystemTime::now(),
                avg_processing_ns: 0.0,
            });

        stats.count += 1;
        stats.last_used = SystemTime::now();

        // Update average processing time
        let new_time_ns = processing_time.as_nanos() as f64;
        if stats.count == 1 {
            stats.avg_processing_ns = new_time_ns;
        } else {
            // Weighted average
            stats.avg_processing_ns = (stats.avg_processing_ns * (stats.count - 1) as f64
                + new_time_ns)
                / stats.count as f64;
        }

        // Check if we should auto-save
        drop(profiles); // Release write lock
        self.maybe_auto_save();
    }

    /// Record usage of an entire text during conversion
    pub fn record_conversion(
        &self,
        from_script: &str,
        to_script: &str,
        text: &str,
        processing_time: Duration,
    ) {
        if !self.config.enabled {
            return;
        }

        // Extract sequences from the text
        let sequences = self.extract_sequences(text);
        let time_per_sequence = processing_time / sequences.len() as u32;

        for sequence in sequences {
            self.record_sequence(from_script, to_script, &sequence, time_per_sequence);
        }
    }

    /// Extract meaningful sequences from text
    fn extract_sequences(&self, text: &str) -> Vec<String> {
        let mut sequences = Vec::new();
        let chars: Vec<char> = text.chars().collect();

        // Extract individual characters
        for ch in &chars {
            if !ch.is_whitespace() && !ch.is_ascii_punctuation() {
                sequences.push(ch.to_string());
            }
        }

        // Extract bigrams
        for window in chars.windows(2) {
            if !window[0].is_whitespace() && !window[1].is_whitespace() {
                sequences.push(window.iter().collect());
            }
        }

        // Extract trigrams
        for window in chars.windows(3) {
            if !window[0].is_whitespace() && !window[2].is_whitespace() {
                sequences.push(window.iter().collect());
            }
        }

        // Extract words (space-separated)
        for word in text.split_whitespace() {
            if word.len() > 1 && word.len() <= 20 {
                // Reasonable word length
                sequences.push(word.to_string());
            }
        }

        sequences
    }

    /// Generate optimized lookup tables from current profiles
    pub fn generate_optimizations(&self) -> Vec<OptimizedLookupTable> {
        let profiles = self.profiles.read().unwrap();
        let mut optimizations = Vec::new();

        for ((from_script, to_script), profile) in profiles.iter() {
            // Get top sequences by frequency
            let mut sequences: Vec<_> = profile
                .sequences
                .iter()
                .filter(|(_, stats)| stats.count >= self.config.min_sequence_frequency)
                .map(|(seq, stats)| (seq.clone(), stats.count))
                .collect();

            sequences.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
            sequences.truncate(self.config.max_sequences_per_table);

            if sequences.is_empty() {
                continue;
            }

            // Create optimization table
            let optimization = OptimizedLookupTable {
                from_script: from_script.clone(),
                to_script: to_script.clone(),
                sequence_mappings: FxHashMap::default(), // Will be populated by converter
                word_mappings: FxHashMap::default(),     // Will be populated by converter
                metadata: OptimizationMetadata {
                    generated_at: SystemTime::now(),
                    sequence_count: sequences.len(),
                    min_frequency: self.config.min_sequence_frequency,
                    profile_stats: ProfileStats {
                        total_sequences_profiled: profile.total_conversions,
                        unique_sequences: profile.sequences.len(),
                        top_sequences: sequences.clone(),
                    },
                },
            };

            optimizations.push(optimization);
        }

        optimizations
    }

    /// Get optimization table for a specific conversion path
    pub fn get_optimization(
        &self,
        from_script: &str,
        to_script: &str,
    ) -> Option<OptimizedLookupTable> {
        let optimizations = self.optimizations.read().unwrap();
        optimizations
            .get(&(from_script.to_string(), to_script.to_string()))
            .cloned()
    }

    /// Load optimization table (for hot-reloading)
    pub fn load_optimization(&self, table: OptimizedLookupTable) {
        if !self.config.hot_reload_enabled {
            return;
        }

        let mut optimizations = self.optimizations.write().unwrap();
        let key = (table.from_script.clone(), table.to_script.clone());
        optimizations.insert(key, table);
    }

    /// Save current profiles to disk
    pub fn save_profiles(&self) {
        let profiles = self.profiles.read().unwrap();

        for ((from_script, to_script), profile) in profiles.iter() {
            let filename = format!("{}_{}_profile.json", from_script, to_script);
            let path = self.config.profile_dir.join(filename);

            if let Ok(json) = serde_json::to_string_pretty(profile) {
                let _ = fs::write(path, json);
            }
        }

        *self.last_save_time.lock().unwrap() = Instant::now();
    }

    /// Load profiles from disk
    fn load_profiles(&self) {
        if !self.config.profile_dir.exists() {
            return;
        }

        let mut profiles = self.profiles.write().unwrap();

        if let Ok(entries) = fs::read_dir(&self.config.profile_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(profile) = serde_json::from_str::<ConversionProfile>(&content) {
                            let key = (profile.from_script.clone(), profile.to_script.clone());
                            profiles.insert(key, profile);
                        }
                    }
                }
            }
        }
    }

    /// Save optimizations to disk
    pub fn save_optimizations(&self, optimizations: &[OptimizedLookupTable]) {
        for optimization in optimizations {
            let filename = format!(
                "{}_{}_opt.json",
                optimization.from_script, optimization.to_script
            );
            let path = self.config.optimization_dir.join(filename);

            if let Ok(json) = serde_json::to_string_pretty(optimization) {
                let _ = fs::write(path, json);
            }
        }
    }

    /// Load optimizations from disk
    fn load_optimizations(&self) {
        if !self.config.optimization_dir.exists() {
            return;
        }

        let mut optimizations = self.optimizations.write().unwrap();

        if let Ok(entries) = fs::read_dir(&self.config.optimization_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(opt) = serde_json::from_str::<OptimizedLookupTable>(&content) {
                            let key = (opt.from_script.clone(), opt.to_script.clone());
                            optimizations.insert(key, opt);
                        }
                    }
                }
            }
        }
    }

    /// Check if we should auto-save profiles
    fn maybe_auto_save(&self) {
        let last_save = *self.last_save_time.lock().unwrap();
        if last_save.elapsed() >= self.config.auto_save_interval {
            self.save_profiles();
        }
    }

    /// Get profile statistics for monitoring
    pub fn get_profile_stats(&self) -> FxHashMap<(String, String), ProfileStats> {
        let profiles = self.profiles.read().unwrap();
        let mut stats = FxHashMap::default();

        for (key, profile) in profiles.iter() {
            let mut top_sequences: Vec<_> = profile
                .sequences
                .iter()
                .map(|(seq, stats)| (seq.clone(), stats.count))
                .collect();

            top_sequences.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
            top_sequences.truncate(10);

            stats.insert(
                key.clone(),
                ProfileStats {
                    total_sequences_profiled: profile.total_conversions,
                    unique_sequences: profile.sequences.len(),
                    top_sequences,
                },
            );
        }

        stats
    }

    /// Clear all profile data
    pub fn clear_profiles(&self) {
        let mut profiles = self.profiles.write().unwrap();
        profiles.clear();
    }

    /// Enable or disable profiling
    pub fn set_enabled(&mut self, enabled: bool) {
        self.config.enabled = enabled;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Duration;

    #[test]
    fn test_profiler_creation() {
        let profiler = Profiler::new();
        assert!(profiler.config.enabled);
    }

    #[test]
    fn test_sequence_recording() {
        let profiler = Profiler::new();

        profiler.record_sequence("devanagari", "iso15919", "धर्म", Duration::from_nanos(1000));
        profiler.record_sequence("devanagari", "iso15919", "धर्म", Duration::from_nanos(1200));
        profiler.record_sequence("devanagari", "iso15919", "योग", Duration::from_nanos(800));

        let profiles = profiler.profiles.read().unwrap();
        let key = ("devanagari".to_string(), "iso15919".to_string());

        assert!(profiles.contains_key(&key));
        let profile = &profiles[&key];
        assert_eq!(profile.sequences.len(), 2);
        assert_eq!(profile.sequences["धर्म"].count, 2);
        assert_eq!(profile.sequences["योग"].count, 1);
    }

    #[test]
    fn test_sequence_extraction() {
        let profiler = Profiler::new();
        let sequences = profiler.extract_sequences("धर्म योग");

        // Should extract individual chars, bigrams, trigrams, and words
        assert!(sequences.contains(&"ध".to_string()));
        assert!(sequences.contains(&"धर".to_string()));
        assert!(sequences.contains(&"धर्म".to_string()));
        assert!(sequences.contains(&"योग".to_string()));
    }

    #[test]
    fn test_optimization_generation() {
        let mut config = ProfilerConfig::default();
        config.min_sequence_frequency = 1; // Lower threshold for testing
        let profiler = Profiler::with_config(config);

        // Record some sequences
        for _ in 0..5 {
            profiler.record_sequence("devanagari", "iso15919", "धर्म", Duration::from_nanos(1000));
        }
        for _ in 0..3 {
            profiler.record_sequence("devanagari", "iso15919", "योग", Duration::from_nanos(800));
        }

        let optimizations = profiler.generate_optimizations();
        assert_eq!(optimizations.len(), 1);

        let opt = &optimizations[0];
        assert_eq!(opt.from_script, "devanagari");
        assert_eq!(opt.to_script, "iso15919");
        assert_eq!(opt.metadata.sequence_count, 2);
    }
}
