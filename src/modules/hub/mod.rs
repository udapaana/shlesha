use crate::modules::core::unknown_handler::TransliterationMetadata;
use thiserror::Error;

pub mod tokens;
pub mod trait_based_converter;
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
    pub fn to_debug_string(&self) -> String {
        match self {
            HubFormat::AbugidaTokens(tokens) => {
                // Simple debug representation showing token names
                tokens
                    .iter()
                    .map(|t| format!("{:?}", t))
                    .collect::<Vec<_>>()
                    .join(" ")
            }
            HubFormat::AlphabetTokens(tokens) => {
                // Simple debug representation showing token names
                tokens
                    .iter()
                    .map(|t| format!("{:?}", t))
                    .collect::<Vec<_>>()
                    .join(" ")
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
                    metadata: None,
                })
            }
            HubFormat::AlphabetTokens(tokens) => {
                let abugida_tokens = self.alphabet_to_abugida_tokens(tokens)?;
                Ok(HubResult {
                    output: HubFormat::AbugidaTokens(abugida_tokens),
                    metadata: None,
                })
            }
        }
    }
}

/// Central hub implementing token-based conversions
pub struct Hub {}

impl Hub {
    pub fn new() -> Self {
        Self {}
    }
}

impl HubTrait for Hub {
    fn abugida_to_alphabet_tokens(
        &self,
        tokens: &HubTokenSequence,
    ) -> Result<HubTokenSequence, HubError> {
        // Use trait-based implementation with generated mappings
        trait_based_converter::TraitBasedConverter::abugida_to_alphabet(tokens)
    }

    fn alphabet_to_abugida_tokens(
        &self,
        tokens: &HubTokenSequence,
    ) -> Result<HubTokenSequence, HubError> {
        // Use trait-based implementation with generated mappings
        trait_based_converter::TraitBasedConverter::alphabet_to_abugida(tokens)
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
