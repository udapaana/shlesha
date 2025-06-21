use crate::modules::hub::HubOutput;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum GeneratorError {
    #[error("Unsupported script: {0}")]
    UnsupportedScript(String),
    #[error("Generation failed: {0}")]
    GenerationFailed(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub trait TargetGeneratorTrait {
    fn from_hub(&self, hub_output: &HubOutput, target_script: &str) -> Result<String, GeneratorError>;
}

pub struct TargetGenerator;

impl TargetGenerator {
    pub fn new() -> Self {
        Self
    }

    fn detect_target_script(&self, script: &str) -> Result<TargetScriptType, GeneratorError> {
        match script.to_lowercase().as_str() {
            "devanagari" | "deva" => Ok(TargetScriptType::Devanagari),
            "iso" | "iso15919" | "iso-15919" => Ok(TargetScriptType::Iso),
            _ => Err(GeneratorError::UnsupportedScript(script.to_string())),
        }
    }
}

impl TargetGeneratorTrait for TargetGenerator {
    fn from_hub(&self, hub_output: &HubOutput, target_script: &str) -> Result<String, GeneratorError> {
        let target_type = self.detect_target_script(target_script)?;
        
        match (hub_output, target_type) {
            (HubOutput::Devanagari(deva_text), TargetScriptType::Devanagari) => {
                Ok(deva_text.clone())
            },
            (HubOutput::Iso(iso_text), TargetScriptType::Iso) => {
                Ok(iso_text.clone())
            },
            (HubOutput::Devanagari(_), TargetScriptType::Iso) => {
                Err(GeneratorError::GenerationFailed("Direct Deva to ISO should be handled by hub".to_string()))
            },
            (HubOutput::Iso(_), TargetScriptType::Devanagari) => {
                Err(GeneratorError::GenerationFailed("Direct ISO to Deva should be handled by hub".to_string()))
            },
        }
    }
}

impl Default for TargetGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
enum TargetScriptType {
    Devanagari,
    Iso,
}

// TODO List for Generator Module:
// - [ ] Implement preservation token reconstruction: [<script>:<token>:<unicode_point>] → original character
// - [ ] Add script-specific generation optimizations
// - [ ] Integrate with schema registry for dynamic target script support
// - [ ] Add support for script-specific rendering rules
// - [ ] Implement fallback strategies for unsupported characters