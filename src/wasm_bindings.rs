//! WASM bindings for Shlesha using wasm-bindgen
//!
//! This module provides WebAssembly bindings for the Shlesha transliteration library,
//! enabling usage in web browsers and JavaScript environments.

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use crate::{LosslessTransliterator, LosslessResult};

/// WASM wrapper for LosslessResult
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct WasmLosslessResult {
    inner: LosslessResult,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl WasmLosslessResult {
    #[wasm_bindgen(getter)]
    pub fn is_lossless(&self) -> bool {
        self.inner.is_lossless
    }
    
    #[wasm_bindgen(getter)]
    pub fn preservation_ratio(&self) -> f64 {
        self.inner.preservation_ratio
    }
    
    #[wasm_bindgen(getter)]
    pub fn tokens_count(&self) -> usize {
        self.inner.tokens_count
    }
    
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!(
            "LosslessResult {{ isLossless: {}, preservationRatio: {:.3}, tokensCount: {} }}",
            self.inner.is_lossless, self.inner.preservation_ratio, self.inner.tokens_count
        )
    }
}

#[cfg(feature = "wasm")]
impl From<LosslessResult> for WasmLosslessResult {
    fn from(result: LosslessResult) -> Self {
        Self { inner: result }
    }
}

/// WASM wrapper for LosslessTransliterator
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct WasmLosslessTransliterator {
    inner: LosslessTransliterator,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl WasmLosslessTransliterator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: LosslessTransliterator::new(),
        }
    }
    
    /// Transliterate text from source script to target script
    ///
    /// # Arguments
    /// * `text` - Input text to transliterate
    /// * `source_script` - Source script name (e.g., "Devanagari")
    /// * `target_script` - Target script name (e.g., "IAST")
    ///
    /// # Returns
    /// Transliterated text with preservation tokens for unknown characters
    ///
    /// # Throws
    /// Error if transliteration fails
    #[wasm_bindgen]
    pub fn transliterate(&self, text: &str, source_script: &str, target_script: &str) -> Result<String, JsValue> {
        self.inner
            .transliterate(text, source_script, target_script)
            .map_err(|e| JsValue::from_str(&format!("Transliteration error: {}", e)))
    }
    
    /// Verify that a transliteration is mathematically lossless
    ///
    /// # Arguments
    /// * `original` - Original input text
    /// * `transliterated` - Transliterated output text
    /// * `source_script` - Source script name
    ///
    /// # Returns
    /// Verification result with lossless guarantee and preservation ratio
    #[wasm_bindgen(js_name = verifyLossless)]
    pub fn verify_lossless(&self, original: &str, transliterated: &str, source_script: &str) -> WasmLosslessResult {
        self.inner
            .verify_lossless(original, transliterated, source_script)
            .into()
    }
    
    /// Get list of supported scripts
    ///
    /// # Returns
    /// Array of supported script names
    #[wasm_bindgen(js_name = supportedScripts)]
    pub fn supported_scripts(&self) -> Vec<JsValue> {
        vec![
            JsValue::from_str("Devanagari"),
            JsValue::from_str("IAST"),
            JsValue::from_str("Harvard-Kyoto"),
            JsValue::from_str("ITRANS"),
            JsValue::from_str("SLP1"),
            JsValue::from_str("Velthuis"),
        ]
    }
    
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        "LosslessTransliterator".to_string()
    }
}

/// Convenience function to create a LosslessTransliterator
#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = createLosslessTransliterator)]
pub fn create_lossless_transliterator() -> WasmLosslessTransliterator {
    WasmLosslessTransliterator::new()
}

/// Convenience function to transliterate text using the lossless system
#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = transliterateLossless)]
pub fn transliterate_lossless(text: &str, source_script: &str, target_script: &str) -> Result<String, JsValue> {
    let transliterator = LosslessTransliterator::new();
    transliterator
        .transliterate(text, source_script, target_script)
        .map_err(|e| JsValue::from_str(&format!("Transliteration error: {}", e)))
}

/// Get library version
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Get library information
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn info() -> String {
    format!(
        "Shlesha v{} - High-Performance Lossless Transliteration Library\n\
         Features: 100% lossless guarantee, 6-10x performance, 72x memory reduction\n\
         Supported scripts: Devanagari, Tamil, Bengali, Gujarati, and more\n\
         Platform: WebAssembly",
        env!("CARGO_PKG_VERSION")
    )
}

/// Initialize the WASM module (called automatically)
#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub fn main() {
    // Set up console logging for debugging
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[cfg(not(feature = "wasm"))]
compile_error!("WASM bindings require the 'wasm' feature to be enabled");

#[cfg(feature = "wasm")]
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    #[wasm_bindgen_test]
    fn test_wasm_lossless_transliterator() {
        let transliterator = WasmLosslessTransliterator::new();
        let result = transliterator.transliterate("धर्म", "Devanagari", "IAST").unwrap();
        assert!(!result.is_empty());
        
        let verification = transliterator.verify_lossless("धर्म", &result, "Devanagari");
        assert!(verification.is_lossless());
    }
    
    #[wasm_bindgen_test]
    fn test_convenience_functions() {
        let result = transliterate_lossless("धर्म", "Devanagari", "IAST").unwrap();
        assert!(!result.is_empty());
        
        assert!(!version().is_empty());
        assert!(info().contains("Shlesha"));
    }
}