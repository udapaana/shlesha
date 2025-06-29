use std::collections::HashSet;

/// Represents an unknown token found during transliteration
#[derive(Debug, Clone, PartialEq)]
pub struct UnknownToken {
    /// The script where the unknown token was found
    pub script: String,
    /// The unknown character
    pub token: char,
    /// Unicode codepoint representation
    pub unicode: String,
    /// Position in the original input (byte offset)
    pub position: usize,
    /// Whether this came from a runtime extension
    pub is_extension: bool,
}

impl UnknownToken {
    /// Create a new unknown token
    pub fn new(script: &str, token: char, position: usize, is_extension: bool) -> Self {
        Self {
            script: script.to_string(),
            token,
            unicode: format!("U+{:04X}", token as u32),
            position,
            is_extension,
        }
    }

    /// Format as annotation string if needed
    pub fn format(&self) -> String {
        if self.is_extension {
            format!("[ext:{}:{}:{}]", self.script, self.token, self.unicode)
        } else {
            format!("[{}:{}:{}]", self.script, self.token, self.unicode)
        }
    }
}

/// Metadata collected during transliteration
#[derive(Debug, Clone, Default)]
pub struct TransliterationMetadata {
    /// Unknown tokens found during conversion
    pub unknown_tokens: Vec<UnknownToken>,
    /// Source script
    pub source_script: String,
    /// Target script  
    pub target_script: String,
    /// Whether any runtime extensions were used
    pub used_extensions: bool,
}

impl TransliterationMetadata {
    pub fn new(source_script: &str, target_script: &str) -> Self {
        Self {
            unknown_tokens: Vec::new(),
            source_script: source_script.to_string(),
            target_script: target_script.to_string(),
            used_extensions: false,
        }
    }

    /// Add an unknown token to the metadata
    pub fn add_unknown(&mut self, token: UnknownToken) {
        if token.is_extension {
            self.used_extensions = true;
        }
        self.unknown_tokens.push(token);
    }

    /// Get unique unknown characters (for creating custom mappings)
    pub fn unique_unknowns(&self) -> Vec<char> {
        let mut unique: HashSet<char> = HashSet::new();
        for token in &self.unknown_tokens {
            unique.insert(token.token);
        }
        let mut result: Vec<char> = unique.into_iter().collect();
        result.sort();
        result
    }

    /// Generate a report of unknown tokens
    pub fn report(&self) -> String {
        if self.unknown_tokens.is_empty() {
            return format!(
                "No unknown tokens found in {} → {} conversion",
                self.source_script, self.target_script
            );
        }

        let mut report = format!(
            "Unknown tokens in {} → {} conversion:\n",
            self.source_script, self.target_script
        );

        // Group by unique character
        let unique = self.unique_unknowns();
        for ch in unique {
            let occurrences: Vec<&UnknownToken> = self
                .unknown_tokens
                .iter()
                .filter(|t| t.token == ch)
                .collect();

            report.push_str(&format!(
                "  '{}' ({}) - {} occurrence(s)\n",
                ch,
                occurrences[0].unicode,
                occurrences.len()
            ));
        }

        if self.used_extensions {
            report.push_str("\nNote: Some unknown tokens came from runtime extensions\n");
        }

        report
    }
}

/// Result of transliteration with optional metadata
#[derive(Debug, Clone)]
pub struct TransliterationResult {
    /// The transliterated output (clean, no annotations)
    pub output: String,
    /// Optional metadata about the conversion
    pub metadata: Option<TransliterationMetadata>,
}

impl TransliterationResult {
    /// Create a simple result without metadata
    pub fn simple(output: String) -> Self {
        Self {
            output,
            metadata: None,
        }
    }

    /// Create a result with metadata
    pub fn with_metadata(output: String, metadata: TransliterationMetadata) -> Self {
        Self {
            output,
            metadata: Some(metadata),
        }
    }

    /// Get the output with unknown tokens annotated
    pub fn annotated_output(&self) -> String {
        match &self.metadata {
            None => self.output.clone(),
            Some(metadata) => {
                if metadata.unknown_tokens.is_empty() {
                    return self.output.clone();
                }

                // Sort tokens by position (reverse order to not affect positions)
                let mut tokens = metadata.unknown_tokens.clone();
                tokens.sort_by(|a, b| b.position.cmp(&a.position));

                let mut result = self.output.clone();
                for token in tokens {
                    // This is simplified - in practice we'd need char boundary handling
                    let annotation = token.format();
                    // Insert annotation after the unknown character
                    if let Some(char_boundary) = result
                        .char_indices()
                        .find(|(i, _)| *i == token.position)
                        .and_then(|(i, _ch)| {
                            result
                                .char_indices()
                                .find(|(j, _)| *j > i)
                                .map(|(j, _)| j)
                                .or(Some(result.len()))
                        })
                    {
                        result.insert_str(char_boundary, &annotation);
                    }
                }

                result
            }
        }
    }
}

/// Trait for converters that support unknown token tracking
pub trait UnknownHandler {
    /// Check if a character is known for a given script
    fn is_known_char(&self, script: &str, ch: char) -> bool;

    /// Process a string and track unknown characters
    fn process_with_unknowns(
        &self,
        script: &str,
        input: &str,
        is_extension: bool,
    ) -> (String, Vec<UnknownToken>) {
        let mut output = String::new();
        let mut unknowns = Vec::new();

        for (pos, ch) in input.char_indices() {
            if self.is_known_char(script, ch) {
                output.push(ch);
            } else {
                output.push(ch); // Pass through
                unknowns.push(UnknownToken::new(script, ch, pos, is_extension));
            }
        }

        (output, unknowns)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknown_token_creation() {
        let token = UnknownToken::new("devanagari", '☺', 5, false);
        assert_eq!(token.script, "devanagari");
        assert_eq!(token.token, '☺');
        assert_eq!(token.unicode, "U+263A");
        assert_eq!(token.format(), "[devanagari:☺:U+263A]");
    }

    #[test]
    fn test_extension_token_format() {
        let token = UnknownToken::new("vedavms", '†', 10, true);
        assert_eq!(token.format(), "[ext:vedavms:†:U+2020]");
    }

    #[test]
    fn test_metadata_unique_unknowns() {
        let mut metadata = TransliterationMetadata::new("source", "target");
        metadata.add_unknown(UnknownToken::new("source", 'a', 0, false));
        metadata.add_unknown(UnknownToken::new("source", 'b', 1, false));
        metadata.add_unknown(UnknownToken::new("source", 'a', 2, false));

        let unique = metadata.unique_unknowns();
        assert_eq!(unique, vec!['a', 'b']);
    }

    #[test]
    fn test_transliteration_result() {
        let result = TransliterationResult::simple("dharma".to_string());
        assert_eq!(result.output, "dharma");
        assert!(result.metadata.is_none());
    }
}
