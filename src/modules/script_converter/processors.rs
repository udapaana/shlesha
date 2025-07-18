use super::ConverterError;
use aho_corasick::AhoCorasick;
use rustc_hash::FxHashMap;
use std::collections::HashMap;

/// Helper to build optimized mapping structures for fast lookup
pub struct FastMappingBuilder;

impl FastMappingBuilder {
    /// Build optimized FxHashMap with first-character indexing for schema-generated converters
    #[inline]
    pub fn build_optimized_mapping<'a>(
        mappings: &'a [(&'a str, &'a str)],
    ) -> (FxHashMap<&'a str, &'a str>, FxHashMap<char, Vec<&'a str>>) {
        let mut mapping = FxHashMap::default();
        let mut by_first_char: FxHashMap<char, Vec<&'a str>> = FxHashMap::default();

        // Build both mappings simultaneously
        for &(from, to) in mappings {
            mapping.insert(from, to);

            // Index by first character for fast prefix lookup
            if let Some(first_char) = from.chars().next() {
                by_first_char.entry(first_char).or_default().push(from);
            }
        }

        // Sort candidates by length descending for greedy longest match
        for candidates in by_first_char.values_mut() {
            candidates.sort_by_key(|b| std::cmp::Reverse(b.len()));
        }

        (mapping, by_first_char)
    }

    /// Build Aho-Corasick automaton for ultra-fast pattern matching
    #[inline]
    pub fn build_aho_corasick_mapping<'a>(
        mappings: &'a [(&'a str, &'a str)],
    ) -> Result<(AhoCorasick, Vec<&'a str>), aho_corasick::BuildError> {
        let mut patterns = Vec::new();
        let mut replacements = Vec::new();

        // Sort by length descending for greedy longest match
        let mut sorted_mappings = mappings.to_vec();
        sorted_mappings.sort_by_key(|(from, _)| std::cmp::Reverse(from.len()));

        for &(from, to) in &sorted_mappings {
            patterns.push(from);
            replacements.push(to);
        }

        let ac = AhoCorasick::builder()
            .match_kind(aho_corasick::MatchKind::LeftmostLongest)
            .build(patterns)?;

        Ok((ac, replacements))
    }

    /// Convert std::HashMap to FxHashMap for improved performance
    #[inline]
    pub fn to_fx_hashmap<'a>(
        std_map: &'a HashMap<&'a str, &'a str>,
    ) -> FxHashMap<&'a str, &'a str> {
        let mut fx_map = FxHashMap::default();
        for (&key, &value) in std_map {
            fx_map.insert(key, value);
        }
        fx_map
    }
}

/// Shared processor for Roman script conversions (IAST, ITRANS, SLP1, etc.)
/// Handles the common logic for all Roman transliteration schemes
pub struct RomanScriptProcessor;

impl RomanScriptProcessor {
    /// Process Roman script text using the provided mapping table
    /// Uses optimized algorithm with FxHashMap internally for better performance
    #[inline]
    pub fn process(input: &str, mapping: &HashMap<&str, &str>) -> Result<String, ConverterError> {
        // Convert to FxHashMap for better performance, then use optimized algorithm
        let fx_mapping = FastMappingBuilder::to_fx_hashmap(mapping);
        Self::process_with_fx_hashmap(input, &fx_mapping)
    }

    /// Optimized version using FxHashMap for better performance
    /// This is used internally by schema-generated converters
    #[inline]
    pub fn process_with_fx_hashmap(
        input: &str,
        mapping: &FxHashMap<&str, &str>,
    ) -> Result<String, ConverterError> {
        let mut result = String::with_capacity(input.len() * 2); // Pre-allocate for worst case
        let mut chars = input.char_indices();

        while let Some((i, ch)) = chars.next() {
            // Fast path for whitespace
            if ch.is_whitespace() {
                result.push(ch);
                continue;
            }

            let mut matched = false;
            let remaining = &input[i..];

            // Try to match sequences of decreasing length (4, 3, 2, 1)
            // OPTIMIZED: Direct string slicing instead of Vec/String allocation
            for len in (1..=4).rev() {
                // Find the end position for a sequence of 'len' characters
                let mut end_pos = 0;
                let mut char_count = 0;
                for (pos, _) in remaining.char_indices() {
                    char_count += 1;
                    if char_count == len {
                        end_pos = pos + remaining[pos..].chars().next().map_or(0, |c| c.len_utf8());
                        break;
                    }
                }

                if char_count == len || (len == 1 && !remaining.is_empty()) {
                    let seq = if char_count == len {
                        &remaining[..end_pos]
                    } else {
                        // Single character case - take first character
                        &remaining[..remaining.chars().next().unwrap().len_utf8()]
                    };

                    if let Some(&mapped_str) = mapping.get(seq) {
                        result.push_str(mapped_str);
                        // Skip the matched characters (len - 1 because we already have the first one)
                        for _ in 1..len {
                            chars.next();
                        }
                        matched = true;
                        break;
                    }
                }
            }

            if !matched {
                // Character not found in mapping - preserve as-is
                result.push(ch);
            }
        }

        Ok(result)
    }

    /// High-performance version with first-character indexing for maximum speed
    #[inline]
    pub fn process_with_fast_lookup(
        input: &str,
        mapping: &FxHashMap<&str, &str>,
        by_first_char: &FxHashMap<char, Vec<&str>>,
    ) -> Result<String, ConverterError> {
        let mut result = String::with_capacity(input.len() * 2); // Pre-allocate for worst case
        let mut i = 0;
        let input_bytes = input.as_bytes();

        while i < input_bytes.len() {
            let ch = input_bytes[i] as char;

            // Fast path for whitespace
            if ch.is_whitespace() {
                result.push(ch);
                i += 1;
                continue;
            }

            let mut matched = false;

            // Use first-character indexing for O(1) prefix lookup (Vidyut technique)
            if let Some(candidates) = by_first_char.get(&ch) {
                // Candidates are pre-sorted by length descending for greedy longest match
                for &candidate in candidates.iter() {
                    let candidate_len = candidate.len();

                    // Check if we have enough characters remaining
                    if i + candidate_len <= input.len() {
                        // Direct byte slice comparison - no allocations!
                        if let Ok(slice) = std::str::from_utf8(&input_bytes[i..i + candidate_len]) {
                            if slice == candidate {
                                if let Some(&mapped_str) = mapping.get(candidate) {
                                    result.push_str(mapped_str);
                                    i += candidate_len;
                                    matched = true;
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            if !matched {
                // Single character fallback - check if it's in mapping
                let ch_str = std::str::from_utf8(&input_bytes[i..i + 1]).unwrap_or("");
                if let Some(&mapped_str) = mapping.get(ch_str) {
                    result.push_str(mapped_str);
                } else {
                    // Character not found in mapping - preserve as-is
                    result.push(ch);
                }
                i += 1;
            }
        }

        Ok(result)
    }

    /// Ultra-high-performance version using Aho-Corasick automaton for pattern matching
    /// This provides the fastest possible longest-match transliteration
    #[inline]
    pub fn process_with_aho_corasick(
        input: &str,
        ac: &AhoCorasick,
        replacements: &[&str],
    ) -> Result<String, ConverterError> {
        let mut result = String::with_capacity(input.len() * 2);
        let mut last_end = 0;

        // Use Aho-Corasick to find all matches with leftmost-longest strategy
        for mat in ac.find_iter(input) {
            let match_start = mat.start();
            let match_end = mat.end();
            let pattern_id = mat.pattern().as_usize();

            // Add any text before this match
            if last_end < match_start {
                result.push_str(&input[last_end..match_start]);
            }

            // Add the replacement text
            result.push_str(replacements[pattern_id]);
            last_end = match_end;
        }

        // Add any remaining text after the last match
        if last_end < input.len() {
            result.push_str(&input[last_end..]);
        }

        Ok(result)
    }
}

/// Shared processor for Indic script conversions (Devanagari, Bengali, Tamil, etc.)
/// Handles the common logic for all Indic scripts with implicit 'a' vowel
pub struct IndicScriptProcessor;

impl IndicScriptProcessor {
    /// Process Indic script text to hub format (ISO or Devanagari)
    /// Handles implicit 'a' vowel and virama logic
    #[inline]
    pub fn to_hub(
        input: &str,
        consonant_map: &HashMap<&str, &str>,
        vowel_map: &HashMap<&str, &str>,
        vowel_sign_map: &HashMap<&str, &str>,
        misc_map: &HashMap<&str, &str>,
        virama: char,
    ) -> Result<String, ConverterError> {
        let mut result = String::with_capacity(input.len() * 2);
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            // Check for whitespace
            if ch.is_whitespace() {
                result.push(ch);
                continue;
            }

            // Check for consonants
            if let Some(&cons) = consonant_map.get(&ch.to_string().as_str()) {
                result.push_str(cons);

                // Check if followed by virama or vowel sign
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == virama {
                        // Consonant without vowel - skip the virama and don't add 'a'
                        chars.next();
                    } else if vowel_sign_map.contains_key(&next_ch.to_string().as_str()) {
                        // Consonant with explicit vowel sign - don't add 'a'
                    } else {
                        // Consonant with implicit 'a'
                        result.push('a');
                    }
                } else {
                    // End of string - add implicit 'a'
                    result.push('a');
                }
            }
            // Check for independent vowels
            else if let Some(&vowel) = vowel_map.get(&ch.to_string().as_str()) {
                result.push_str(vowel);
            }
            // Check for vowel signs (should not appear independently, but handle gracefully)
            else if let Some(&vowel_sign) = vowel_sign_map.get(&ch.to_string().as_str()) {
                result.push_str(vowel_sign);
            }
            // Check for misc characters (anusvara, visarga, etc.)
            else if let Some(&misc) = misc_map.get(&ch.to_string().as_str()) {
                result.push_str(misc);
            }
            // Preserve unknown characters
            else {
                result.push(ch);
            }
        }

        Ok(result)
    }

    /// Process hub format (ISO or Devanagari) to Indic script
    /// Handles vowel mark generation and consonant cluster formation
    #[inline]
    pub fn from_hub(
        input: &str,
        mapping: &HashMap<&str, &str>,
        _has_implicit_a: bool,
    ) -> Result<String, ConverterError> {
        // Use the Roman script processor for the basic mapping logic
        // The Indic-specific handling (vowel marks, consonant clusters) is
        // managed by the token-based conversion system in the hub module
        RomanScriptProcessor::process(input, mapping)
    }
}
