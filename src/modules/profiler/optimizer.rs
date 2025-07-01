//! Optimization Generator Module
//!
//! This module generates optimized lookup tables from profile data by:
//! - Running actual conversions to build mappings
//! - Creating multi-character sequence optimizations
//! - Building common word dictionaries
//! - Generating efficient data structures for runtime lookup

use super::{ConversionProfile, OptimizedLookupTable};
use crate::Shlesha;
use rustc_hash::FxHashMap;
use std::time::Instant;

/// Optimization generator that creates lookup tables from profiles
pub struct OptimizationGenerator {
    transliterator: Shlesha,
}

impl Default for OptimizationGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizationGenerator {
    /// Create a new optimization generator
    pub fn new() -> Self {
        Self {
            transliterator: Shlesha::new(),
        }
    }

    /// Generate optimized lookup table from a conversion profile
    pub fn generate_from_profile(
        &self,
        profile: &ConversionProfile,
        top_sequences: &[(String, u64)],
    ) -> Result<OptimizedLookupTable, Box<dyn std::error::Error>> {
        let mut sequence_mappings = FxHashMap::default();
        let mut word_mappings = FxHashMap::default();

        // Process each sequence and generate its mapping
        for (sequence, _count) in top_sequences {
            // Skip if we've already processed this as part of a longer sequence
            if sequence_mappings.contains_key(sequence) {
                continue;
            }

            // Convert the sequence
            match self.transliterator.transliterate(
                sequence,
                &profile.from_script,
                &profile.to_script,
            ) {
                Ok(converted) => {
                    // Determine if this is a word or sequence
                    if sequence.contains(char::is_whitespace) || sequence.chars().count() > 4 {
                        word_mappings.insert(sequence.clone(), converted);
                    } else {
                        sequence_mappings.insert(sequence.clone(), converted);
                    }
                }
                Err(_) => {
                    // Skip sequences that can't be converted
                    continue;
                }
            }
        }

        // Generate common Sanskrit/Hindi word optimizations
        self.add_common_words(&mut word_mappings, &profile.from_script, &profile.to_script);

        let total_count = sequence_mappings.len() + word_mappings.len();

        Ok(OptimizedLookupTable {
            from_script: profile.from_script.clone(),
            to_script: profile.to_script.clone(),
            sequence_mappings,
            word_mappings,
            metadata: super::OptimizationMetadata {
                generated_at: std::time::SystemTime::now(),
                sequence_count: total_count,
                min_frequency: 0, // Will be set by profiler
                profile_stats: super::ProfileStats {
                    total_sequences_profiled: profile.total_conversions,
                    unique_sequences: profile.sequences.len(),
                    top_sequences: top_sequences.to_vec(),
                },
            },
        })
    }

    /// Add common Sanskrit/Hindi words to the optimization table
    fn add_common_words(
        &self,
        word_mappings: &mut FxHashMap<String, String>,
        from_script: &str,
        to_script: &str,
    ) {
        // Common Sanskrit/Hindi words that appear frequently
        let common_words = match from_script {
            "devanagari" => vec![
                // Sanskrit religious/philosophical terms
                "धर्म",
                "कर्म",
                "योग",
                "वेद",
                "मन्त्र",
                "तन्त्र",
                "यन्त्र",
                "आत्मा",
                "ब्रह्म",
                "मोक्ष",
                "निर्वाण",
                "समाधि",
                "ध्यान",
                "प्राण",
                "चक्र",
                "गुरु",
                "शिष्य",
                "साधना",
                "सिद्धि",
                // Common words
                "नमस्ते",
                "नमस्कार",
                "श्री",
                "ॐ",
                "स्वामी",
                "महा",
                "राज",
                "देव",
                "देवी",
                "मन्दिर",
                "पूजा",
                "आरती",
                "प्रसाद",
                // Texts and scriptures
                "रामायण",
                "महाभारत",
                "गीता",
                "उपनिषद्",
                "पुराण",
                "सूत्र",
                "शास्त्र",
                "वेदान्त",
                "संस्कृत",
                "हिन्दी",
                "भारत",
                "भाषा",
                // Common verbs/actions
                "करना",
                "होना",
                "जाना",
                "आना",
                "देना",
                "लेना",
                "कहना",
                "सुनना",
                "देखना",
                "समझना",
                "पढ़ना",
                "लिखना",
            ],
            "iast" | "iso15919" => vec![
                // Sanskrit terms in Roman
                "dharma",
                "karma",
                "yoga",
                "veda",
                "mantra",
                "tantra",
                "yantra",
                "ātmā",
                "ātman",
                "brahma",
                "brahman",
                "mokṣa",
                "nirvāṇa",
                "samādhi",
                "dhyāna",
                "prāṇa",
                "cakra",
                "guru",
                "śiṣya",
                "sādhanā",
                "siddhi",
                // Common words
                "namaste",
                "namaskāra",
                "śrī",
                "oṁ",
                "om",
                "svāmī",
                "mahā",
                "deva",
                "devī",
                "mandira",
                "pūjā",
                "āratī",
                "prasāda",
                // Texts
                "rāmāyaṇa",
                "mahābhārata",
                "gītā",
                "upaniṣad",
                "purāṇa",
                "sūtra",
                "śāstra",
                "vedānta",
                "saṃskṛta",
                "hindī",
                "bhārata",
                "bhāṣā",
            ],
            _ => vec![],
        };

        // Convert and add common words
        for word in common_words {
            if let Ok(converted) = self
                .transliterator
                .transliterate(word, from_script, to_script)
            {
                word_mappings.insert(word.to_string(), converted);
            }
        }

        // Add common bigrams and trigrams for Sanskrit
        if from_script == "devanagari" {
            let common_sequences = vec![
                // Common consonant clusters
                "क्ष",
                "ज्ञ",
                "त्र",
                "श्र",
                "स्व",
                "द्व",
                "त्व",
                "स्थ",
                "प्र",
                "ब्र",
                "क्र",
                "ग्र",
                "द्र",
                "फ्र",
                "श्व",
                "ह्व",
                // Common syllables
                "नि",
                "अनु",
                "प्रति",
                "सम्",
                "उप",
                "अधि",
                "अभि",
                "वि",
                // Common endings
                "ता",
                "त्व",
                "आनि",
                "एषु",
                "स्य",
                "तः",
                "भिः",
                "भ्यः",
            ];

            for seq in common_sequences {
                if let Ok(converted) =
                    self.transliterator
                        .transliterate(seq, from_script, to_script)
                {
                    word_mappings.insert(seq.to_string(), converted);
                }
            }
        }
    }

    /// Benchmark the effectiveness of an optimization table
    pub fn benchmark_optimization(
        &self,
        optimization: &OptimizedLookupTable,
        test_text: &str,
    ) -> OptimizationBenchmark {
        let start = Instant::now();
        let _baseline_result = self
            .transliterator
            .transliterate(
                test_text,
                &optimization.from_script,
                &optimization.to_script,
            )
            .unwrap_or_default();
        let baseline_time = start.elapsed();

        // Simulate optimized conversion
        let start = Instant::now();
        let _optimized_result = self.simulate_optimized_conversion(test_text, optimization);
        let optimized_time = start.elapsed();

        OptimizationBenchmark {
            baseline_time_ns: baseline_time.as_nanos() as u64,
            optimized_time_ns: optimized_time.as_nanos() as u64,
            speedup_factor: baseline_time.as_secs_f64() / optimized_time.as_secs_f64(),
            cache_hits: self.count_cache_hits(test_text, optimization),
            total_sequences: test_text.chars().count(),
        }
    }

    /// Simulate conversion using optimization table
    fn simulate_optimized_conversion(
        &self,
        text: &str,
        optimization: &OptimizedLookupTable,
    ) -> String {
        let mut result = String::new();
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let mut matched = false;

            // Try to match longest sequences first
            for len in (1..=5).rev() {
                if i + len > chars.len() {
                    continue;
                }

                let sequence: String = chars[i..i + len].iter().collect();

                // Check word mappings first (for multi-word sequences)
                if let Some(mapped) = optimization.word_mappings.get(&sequence) {
                    result.push_str(mapped);
                    i += len;
                    matched = true;
                    break;
                }

                // Then check sequence mappings
                if let Some(mapped) = optimization.sequence_mappings.get(&sequence) {
                    result.push_str(mapped);
                    i += len;
                    matched = true;
                    break;
                }
            }

            if !matched {
                // Fallback to regular conversion for this character
                let ch_str = chars[i].to_string();
                if let Ok(converted) = self.transliterator.transliterate(
                    &ch_str,
                    &optimization.from_script,
                    &optimization.to_script,
                ) {
                    result.push_str(&converted);
                } else {
                    result.push(chars[i]);
                }
                i += 1;
            }
        }

        result
    }

    /// Count how many sequences would hit the optimization cache
    fn count_cache_hits(&self, text: &str, optimization: &OptimizedLookupTable) -> usize {
        let mut hits = 0;
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let mut matched = false;

            for len in (1..=5).rev() {
                if i + len > chars.len() {
                    continue;
                }

                let sequence: String = chars[i..i + len].iter().collect();

                if optimization.word_mappings.contains_key(&sequence)
                    || optimization.sequence_mappings.contains_key(&sequence)
                {
                    hits += 1;
                    i += len;
                    matched = true;
                    break;
                }
            }

            if !matched {
                i += 1;
            }
        }

        hits
    }
}

/// Benchmark results for an optimization
#[derive(Debug, Clone)]
pub struct OptimizationBenchmark {
    pub baseline_time_ns: u64,
    pub optimized_time_ns: u64,
    pub speedup_factor: f64,
    pub cache_hits: usize,
    pub total_sequences: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::profiler::ConversionProfile;
    use rustc_hash::FxHashMap;
    use std::time::SystemTime;

    #[test]
    fn test_optimization_generation() {
        let generator = OptimizationGenerator::new();

        let profile = ConversionProfile {
            from_script: "devanagari".to_string(),
            to_script: "iast".to_string(),
            sequences: FxHashMap::default(),
            total_conversions: 100,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        };

        // Add some test sequences
        let sequences = vec![
            ("धर्म".to_string(), 50),
            ("योग".to_string(), 30),
            ("कर्म".to_string(), 25),
        ];

        let optimization = generator
            .generate_from_profile(&profile, &sequences)
            .unwrap();

        assert_eq!(optimization.from_script, "devanagari");
        assert_eq!(optimization.to_script, "iast");
        assert!(optimization.sequence_mappings.contains_key("धर्म"));
        assert!(optimization.sequence_mappings.contains_key("योग"));
    }

    #[test]
    fn test_common_words_addition() {
        let generator = OptimizationGenerator::new();
        let mut word_mappings = FxHashMap::default();

        generator.add_common_words(&mut word_mappings, "devanagari", "iast");

        // Should have added common Sanskrit words
        assert!(word_mappings.contains_key("धर्म"));
        assert!(word_mappings.contains_key("नमस्ते"));
        assert!(word_mappings.contains_key("योग"));
    }
}
