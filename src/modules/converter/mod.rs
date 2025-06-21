use crate::modules::hub::HubInput;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ConverterError {
    #[error("Unsupported script: {0}")]
    UnsupportedScript(String),
    #[error("Conversion failed: {0}")]
    ConversionFailed(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub trait ScriptConverterTrait {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError>;
}

pub struct ScriptConverter;

impl ScriptConverter {
    pub fn new() -> Self {
        Self
    }

    fn detect_script(&self, script: &str) -> Result<ScriptType, ConverterError> {
        match script.to_lowercase().as_str() {
            "devanagari" | "deva" => Ok(ScriptType::Devanagari),
            "iso" | "iso15919" | "iso-15919" => Ok(ScriptType::Iso),
            _ => Err(ConverterError::UnsupportedScript(script.to_string())),
        }
    }
}

impl ScriptConverterTrait for ScriptConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {
        let script_type = self.detect_script(script)?;
        
        match script_type {
            ScriptType::Devanagari => Ok(HubInput::Devanagari(input.to_string())),
            ScriptType::Iso => Ok(HubInput::Iso(input.to_string())),
        }
    }
}

impl Default for ScriptConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
enum ScriptType {
    Devanagari,
    Iso,
}

// TODO List for Converter Module:
// - [ ] Implement preservation tokens for unknown characters: [<script>:<token>:<unicode_point>]
// - [ ] Add script-specific conversion optimizations
// - [ ] Integrate with schema registry for dynamic script support
// - [ ] Add validation for input text encoding
// - [ ] Implement batch conversion for performance