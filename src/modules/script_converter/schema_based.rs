//! Schema-based converter for runtime-loaded custom scripts
//!
//! This converter uses schemas loaded from the SchemaRegistry to perform
//! conversions for custom encoding schemes loaded at runtime.

use super::{ConverterError, ScriptConverter};
use crate::modules::core::unknown_handler::{
    TransliterationMetadata, TransliterationResult, UnknownToken,
};
use crate::modules::hub::HubInput;
use crate::modules::registry::{Schema, SchemaRegistry, SchemaRegistryTrait};
use rustc_hash::FxHashMap;
use std::sync::Arc;

/// A converter that uses runtime-loaded schemas for transliteration
pub struct SchemaBasedConverter {
    registry: Arc<SchemaRegistry>,
}

impl SchemaBasedConverter {
    /// Create a new schema-based converter with a reference to the schema registry
    pub fn new(registry: Arc<SchemaRegistry>) -> Self {
        Self { registry }
    }

    /// Get the hub script type for a given schema
    #[inline]
    fn get_hub_script_type(&self, schema: &Schema) -> &str {
        match schema.script_type.as_str() {
            "brahmic" | "indic" => "devanagari",
            "romanized" | "roman" => "iso15919",
            _ => "iso15919", // Default to ISO for unknown types
        }
    }

    /// Apply character mappings from schema
    #[inline]
    fn apply_mappings(
        &self,
        input: &str,
        mappings: &FxHashMap<String, String>,
        reverse: bool,
    ) -> (String, Vec<UnknownToken>) {
        let mut result = String::with_capacity(input.len() * 2); // Pre-allocate for worst case scenario
        let mut unknown_tokens = Vec::new();
        let mut chars = input.chars().peekable();
        let mut position = 0;

        // Create reverse mappings if needed
        let reverse_mappings: FxHashMap<String, String>;
        let mapping_to_use = if reverse {
            reverse_mappings = mappings
                .iter()
                .map(|(k, v)| (v.clone(), k.clone()))
                .collect();
            &reverse_mappings
        } else {
            mappings
        };

        while let Some(ch) = chars.next() {
            let mut matched = false;

            // Try multi-character matches first (for digraphs like "kh", "gh", etc.)
            if let Some(next_ch) = chars.peek() {
                // Use separate buffers to avoid borrow checker issues
                let mut ch_buf = [0u8; 4];
                let mut next_ch_buf = [0u8; 4];
                let ch_str = ch.encode_utf8(&mut ch_buf);
                let next_ch_str = next_ch.encode_utf8(&mut next_ch_buf);

                // Create a small string with pre-allocated capacity for the two-character key
                let mut two_char_key = String::with_capacity(8);
                two_char_key.push_str(ch_str);
                two_char_key.push_str(next_ch_str);

                if let Some(mapped) = mapping_to_use.get(&two_char_key) {
                    result.push_str(mapped);
                    chars.next(); // Consume the second character
                    position += 2;
                    matched = true;
                }
            }

            // Single character match
            if !matched {
                // Use a small buffer to avoid String allocation for single chars
                let mut ch_buf = [0u8; 4]; // Max UTF-8 char is 4 bytes
                let ch_str = ch.encode_utf8(&mut ch_buf);

                if let Some(mapped) = mapping_to_use.get(ch_str) {
                    result.push_str(mapped);
                } else {
                    // Unknown character - pass through
                    result.push(ch);
                    unknown_tokens.push(UnknownToken {
                        script: "custom".to_string(),
                        token: ch,
                        unicode: format!("U+{:04X}", ch as u32),
                        position,
                        is_extension: true,
                    });
                }
                position += 1;
            }
        }

        (result, unknown_tokens)
    }
}

impl ScriptConverter for SchemaBasedConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        // Get the schema from registry
        let schema =
            self.registry
                .get_schema(script)
                .ok_or_else(|| ConverterError::InvalidInput {
                    script: script.to_string(),
                    message: format!("Schema not found for script: {script}"),
                })?;

        // Apply mappings
        let (converted, _) = self.apply_mappings(input, &schema.mappings, false);

        // Return appropriate hub format based on script type
        match self.get_hub_script_type(schema) {
            "devanagari" => Ok(HubInput::Devanagari(converted)),
            "iso15919" => Ok(HubInput::Iso(converted)),
            _ => Err(ConverterError::ConversionFailed {
                script: script.to_string(),
                reason: "Unknown script type".to_string(),
            }),
        }
    }

    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {
        // Get the schema from registry
        let schema =
            self.registry
                .get_schema(script)
                .ok_or_else(|| ConverterError::InvalidInput {
                    script: script.to_string(),
                    message: format!("Schema not found for script: {script}"),
                })?;

        // Validate that the input format matches what the schema expects
        let expected_hub_type = self.get_hub_script_type(schema);
        let input = match (hub_input, expected_hub_type) {
            (HubInput::Devanagari(text), "devanagari") => text,
            (HubInput::Iso(text), "iso15919") => text,
            _ => {
                return Err(ConverterError::ConversionFailed {
                    script: script.to_string(),
                    reason: format!(
                        "Input format mismatch: schema expects {expected_hub_type} but got {}",
                        match hub_input {
                            HubInput::Devanagari(_) => "devanagari",
                            HubInput::Iso(_) => "iso15919",
                        }
                    ),
                });
            }
        };

        // Apply reverse mappings
        let (converted, _) = self.apply_mappings(input, &schema.mappings, true);
        Ok(converted)
    }

    fn to_hub_with_metadata(
        &self,
        script: &str,
        input: &str,
    ) -> Result<(HubInput, TransliterationMetadata), ConverterError> {
        // Get the schema from registry
        let schema =
            self.registry
                .get_schema(script)
                .ok_or_else(|| ConverterError::InvalidInput {
                    script: script.to_string(),
                    message: format!("Schema not found for script: {script}"),
                })?;

        // Apply mappings and collect unknown tokens
        let (converted, unknown_tokens) = self.apply_mappings(input, &schema.mappings, false);

        // Create metadata
        let mut metadata = TransliterationMetadata::new(script, "hub");
        metadata.unknown_tokens = unknown_tokens;

        // Return appropriate hub format based on script type
        let hub_input = match self.get_hub_script_type(schema) {
            "devanagari" => HubInput::Devanagari(converted),
            "iso15919" => HubInput::Iso(converted),
            _ => {
                return Err(ConverterError::ConversionFailed {
                    script: script.to_string(),
                    reason: "Unknown script type".to_string(),
                })
            }
        };

        Ok((hub_input, metadata))
    }

    fn from_hub_with_metadata(
        &self,
        script: &str,
        hub_input: &HubInput,
    ) -> Result<TransliterationResult, ConverterError> {
        // Get the schema from registry
        let schema =
            self.registry
                .get_schema(script)
                .ok_or_else(|| ConverterError::InvalidInput {
                    script: script.to_string(),
                    message: format!("Schema not found for script: {script}"),
                })?;

        // Validate that the input format matches what the schema expects
        let expected_hub_type = self.get_hub_script_type(schema);
        let input = match (hub_input, expected_hub_type) {
            (HubInput::Devanagari(text), "devanagari") => text,
            (HubInput::Iso(text), "iso15919") => text,
            _ => {
                return Err(ConverterError::ConversionFailed {
                    script: script.to_string(),
                    reason: format!(
                        "Input format mismatch: schema expects {expected_hub_type} but got {}",
                        match hub_input {
                            HubInput::Devanagari(_) => "devanagari",
                            HubInput::Iso(_) => "iso15919",
                        }
                    ),
                });
            }
        };

        // Apply reverse mappings
        let (converted, unknown_tokens) = self.apply_mappings(input, &schema.mappings, true);

        // Create metadata
        let mut metadata = TransliterationMetadata::new("hub", script);
        metadata.unknown_tokens = unknown_tokens;

        Ok(TransliterationResult {
            output: converted,
            metadata: Some(metadata),
        })
    }

    fn supported_scripts(&self) -> Vec<&'static str> {
        // This converter supports all scripts in the registry dynamically
        // We can't return static strings for dynamic content, so return empty
        vec![]
    }

    fn supports_script(&self, script: &str) -> bool {
        // Check if the schema exists in the registry
        self.registry.get_schema(script).is_some()
    }

    fn script_has_implicit_a(&self, script: &str) -> bool {
        // Check the schema metadata
        if let Some(schema) = self.registry.get_schema(script) {
            schema.metadata.has_implicit_a
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_based_converter_creation() {
        let registry = Arc::new(SchemaRegistry::new());
        let converter = SchemaBasedConverter::new(registry);

        // Should not support any scripts by default (only built-in placeholders)
        assert!(!converter.supports_script("my_custom_script"));
    }
}
