use crate::modules::core::unknown_handler::TransliterationMetadata;
use thiserror::Error;

pub mod manual_converter;
pub mod token_converters;
pub mod token_string_impl;
pub mod tokens;
pub use token_converters::TokenToStringConverter;
pub use tokens::{AbugidaToken, AlphabetToken, HubToken, HubTokenSequence};

#[derive(Error, Debug, Clone)]
pub enum HubError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Mapping not found: {0}")]
    MappingNotFound(String),
    #[error("Conversion failed: {0}")]
    ConversionFailed(String),
}

/// Hub format representation - token-based only
#[derive(Debug, Clone, PartialEq)]
pub enum HubFormat {
    // Token-based formats only
    AbugidaTokens(HubTokenSequence),
    AlphabetTokens(HubTokenSequence),
}

impl HubFormat {
    /// Convert tokens to string representation
    pub fn to_string(&self) -> String {
        match self {
            HubFormat::AbugidaTokens(tokens) => {
                let converter = TokenToStringConverter::new();
                converter.tokens_to_devanagari(tokens)
            }
            HubFormat::AlphabetTokens(tokens) => {
                let converter = TokenToStringConverter::new();
                converter.tokens_to_iso(tokens)
            }
        }
    }

    /// Check if this is abugida format
    pub fn is_abugida(&self) -> bool {
        matches!(self, HubFormat::AbugidaTokens(_))
    }

    /// Check if this is alphabet format
    pub fn is_alphabet(&self) -> bool {
        matches!(self, HubFormat::AlphabetTokens(_))
    }
}

// Type aliases for backward compatibility
pub type HubInput = HubFormat;
pub type HubOutput = HubFormat;

#[derive(Debug, Clone)]
pub struct HubResult {
    pub output: HubOutput,
    pub metadata: Option<TransliterationMetadata>,
}

/// Core hub trait for token-based bidirectional conversion
pub trait HubTrait {
    /// Three conversion methods - simplified for clarity
    fn abugida_to_alphabet_tokens(
        &self,
        tokens: &HubTokenSequence,
    ) -> Result<HubTokenSequence, HubError>;
    fn alphabet_to_abugida_tokens(
        &self,
        tokens: &HubTokenSequence,
    ) -> Result<HubTokenSequence, HubError>;
    fn identity_transform(&self, tokens: &HubTokenSequence) -> Result<HubTokenSequence, HubError> {
        // Default implementation - just clone
        Ok(tokens.clone())
    }

    /// Generic conversion between hub formats - routes to appropriate method
    fn convert(&self, input: &HubInput, target_is_alphabet: bool) -> Result<HubOutput, HubError> {
        match (input, target_is_alphabet) {
            (HubFormat::AbugidaTokens(tokens), true) => {
                let alphabet_tokens = self.abugida_to_alphabet_tokens(tokens)?;
                Ok(HubFormat::AlphabetTokens(alphabet_tokens))
            }
            (HubFormat::AbugidaTokens(tokens), false) => {
                let abugida_tokens = self.identity_transform(tokens)?;
                Ok(HubFormat::AbugidaTokens(abugida_tokens))
            }
            (HubFormat::AlphabetTokens(tokens), false) => {
                let abugida_tokens = self.alphabet_to_abugida_tokens(tokens)?;
                Ok(HubFormat::AbugidaTokens(abugida_tokens))
            }
            (HubFormat::AlphabetTokens(tokens), true) => {
                let alphabet_tokens = self.identity_transform(tokens)?;
                Ok(HubFormat::AlphabetTokens(alphabet_tokens))
            }
        }
    }

    /// Generic conversion with metadata
    fn convert_with_metadata(&self, input: &HubInput) -> Result<HubResult, HubError> {
        match input {
            HubFormat::AbugidaTokens(tokens) => {
                let alphabet_tokens = self.abugida_to_alphabet_tokens(tokens)?;
                Ok(HubResult {
                    output: HubFormat::AlphabetTokens(alphabet_tokens),
                    metadata: None, // TODO: Implement metadata for token conversion
                })
            }
            HubFormat::AlphabetTokens(tokens) => {
                let abugida_tokens = self.alphabet_to_abugida_tokens(tokens)?;
                Ok(HubResult {
                    output: HubFormat::AbugidaTokens(abugida_tokens),
                    metadata: None, // TODO: Implement metadata for token conversion
                })
            }
        }
    }
}

/// Central hub implementing token-based conversions
pub struct Hub {
    #[allow(dead_code)]
    generated_hub: crate::generated::GeneratedHub,
}

impl Hub {
    pub fn new() -> Self {
        Self {
            generated_hub: crate::generated::GeneratedHub::new(),
        }
    }
}

impl HubTrait for Hub {
    fn abugida_to_alphabet_tokens(
        &self,
        tokens: &HubTokenSequence,
    ) -> Result<HubTokenSequence, HubError> {
        // Use manual implementation for proper implicit 'a' handling
        manual_converter::ManualHubConverter::abugida_to_alphabet(tokens)
    }

    fn alphabet_to_abugida_tokens(
        &self,
        tokens: &HubTokenSequence,
    ) -> Result<HubTokenSequence, HubError> {
        // Use manual implementation for proper implicit 'a' handling
        manual_converter::ManualHubConverter::alphabet_to_abugida(tokens)
    }
}

impl Default for Hub {
    fn default() -> Self {
        Self::new()
    }
}

// Token-based hub tests
#[cfg(test)]
mod token_tests;
