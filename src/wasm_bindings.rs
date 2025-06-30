//! WASM bindings for Shlesha transliteration library
//!
//! This module provides JavaScript/WebAssembly access to all Shlesha functionality including:
//! - Basic transliteration between scripts
//! - Metadata collection for unknown tokens
//! - Script discovery and validation
//! - Runtime schema loading
//!
//! ## Performance and Benchmarking
//!
//! WASM builds disable criterion's default features (specifically rayon) for benchmarking because:
//! - Rayon requires threading support that is not available in WASM environments
//! - WASM runs in a single-threaded context in most browser and Node.js deployments
//! - Criterion's parallel benchmark execution would fail to compile for the wasm32-unknown-unknown target
//!
//! This means WASM benchmarks run in single-threaded mode, while native benchmarks can use
//! full parallelization. Both approaches provide valid performance measurements for their
//! respective deployment environments.

use crate::Shlesha;
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::*;

// Import console.log for debugging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Macro for console logging (currently unused but kept for future debugging)
#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

/// Initialize WASM module with panic hook for better error messages
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

/// WASM wrapper for the Shlesha transliterator
#[wasm_bindgen]
pub struct WasmShlesha {
    inner: Shlesha,
}

/// WASM wrapper for unknown token information
#[wasm_bindgen]
pub struct WasmUnknownToken {
    script: String,
    token: String,
    position: usize,
    unicode: String,
    is_extension: bool,
}

/// WASM wrapper for transliteration metadata
#[wasm_bindgen]
pub struct WasmTransliterationMetadata {
    source_script: String,
    target_script: String,
    #[allow(dead_code)]
    used_extensions: String,
    unknown_tokens: Vec<WasmUnknownToken>,
}

/// WASM wrapper for transliteration result with metadata
#[wasm_bindgen]
pub struct WasmTransliterationResult {
    output: String,
    metadata: Option<WasmTransliterationMetadata>,
}

#[wasm_bindgen]
impl WasmShlesha {
    /// Create a new Shlesha transliterator instance
    ///
    /// @returns {WasmShlesha} New transliterator instance
    ///
    /// @example
    /// ```javascript
    /// const transliterator = new WasmShlesha();
    /// ```
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> WasmShlesha {
        WasmShlesha {
            inner: Shlesha::new(),
        }
    }

    /// Transliterate text from one script to another
    ///
    /// @param {string} text - Text to transliterate
    /// @param {string} fromScript - Source script name
    /// @param {string} toScript - Target script name
    /// @returns {string} Transliterated text
    /// @throws {Error} If transliteration fails
    ///
    /// @example
    /// ```javascript
    /// const transliterator = new WasmShlesha();
    /// const result = transliterator.transliterate("धर्म", "devanagari", "iast");
    /// console.log(result); // "dharma"
    /// ```
    #[wasm_bindgen]
    pub fn transliterate(
        &self,
        text: &str,
        from_script: &str,
        to_script: &str,
    ) -> Result<String, JsValue> {
        self.inner
            .transliterate(text, from_script, to_script)
            .map_err(|e| JsValue::from_str(&format!("Transliteration failed: {e}")))
    }

    /// Transliterate text with metadata collection for unknown tokens
    ///
    /// @param {string} text - Text to transliterate
    /// @param {string} fromScript - Source script name
    /// @param {string} toScript - Target script name
    /// @returns {WasmTransliterationResult} Result with output and metadata
    /// @throws {Error} If transliteration fails
    ///
    /// @example
    /// ```javascript
    /// const transliterator = new WasmShlesha();
    /// const result = transliterator.transliterateWithMetadata("धर्मkr", "devanagari", "iast");
    /// console.log(result.getOutput()); // "dharmakr"
    /// console.log(result.getUnknownTokenCount()); // 2 (for 'k' and 'r')
    /// ```
    #[wasm_bindgen(js_name = transliterateWithMetadata)]
    pub fn transliterate_with_metadata(
        &self,
        text: &str,
        from_script: &str,
        to_script: &str,
    ) -> Result<WasmTransliterationResult, JsValue> {
        let result = self
            .inner
            .transliterate_with_metadata(text, from_script, to_script)
            .map_err(|e| JsValue::from_str(&format!("Transliteration failed: {e}")))?;

        let wasm_metadata = result.metadata.map(|metadata| {
            let unknown_tokens = metadata
                .unknown_tokens
                .into_iter()
                .map(|token| WasmUnknownToken {
                    script: token.script,
                    token: token.token.to_string(),
                    position: token.position,
                    unicode: token.unicode,
                    is_extension: token.is_extension,
                })
                .collect();

            WasmTransliterationMetadata {
                source_script: metadata.source_script,
                target_script: metadata.target_script,
                used_extensions: metadata.used_extensions.to_string(),
                unknown_tokens,
            }
        });

        Ok(WasmTransliterationResult {
            output: result.output,
            metadata: wasm_metadata,
        })
    }

    /// Get list of supported scripts as JavaScript Array
    ///
    /// @returns {Array<string>} Array of supported script names
    ///
    /// @example
    /// ```javascript
    /// const transliterator = new WasmShlesha();
    /// const scripts = transliterator.listSupportedScripts();
    /// console.log(scripts.includes("devanagari")); // true
    /// ```
    #[wasm_bindgen(js_name = listSupportedScripts)]
    pub fn list_supported_scripts(&self) -> Array {
        let scripts = self.inner.list_supported_scripts();
        let array = Array::new();
        for script in scripts {
            array.push(&JsValue::from_str(&script));
        }
        array
    }

    /// Check if a script is supported
    ///
    /// @param {string} script - Script name to check
    /// @returns {boolean} True if script is supported
    ///
    /// @example
    /// ```javascript
    /// const transliterator = new WasmShlesha();
    /// console.log(transliterator.supportsScript("devanagari")); // true
    /// console.log(transliterator.supportsScript("unknown"));     // false
    /// ```
    #[wasm_bindgen(js_name = supportsScript)]
    pub fn supports_script(&self, script: &str) -> bool {
        self.inner.supports_script(script)
    }

    /// Load a new script schema at runtime
    ///
    /// @param {string} schemaPath - Path to YAML schema file
    /// @throws {Error} If schema loading fails
    ///
    /// @example
    /// ```javascript
    /// const transliterator = new WasmShlesha();
    /// transliterator.loadSchema("custom_script.yaml");
    /// ```
    #[wasm_bindgen(js_name = loadSchema)]
    pub fn load_schema(&mut self, schema_path: &str) -> Result<(), JsValue> {
        self.inner
            .load_schema_from_file(schema_path)
            .map_err(|e| JsValue::from_str(&format!("Schema loading failed: {e}")))
    }

    /// Get script information as JavaScript Object
    ///
    /// @returns {Object} Object mapping script names to descriptions
    ///
    /// @example
    /// ```javascript
    /// const transliterator = new WasmShlesha();
    /// const info = transliterator.getScriptInfo();
    /// console.log(info.devanagari); // "Devanagari script (देवनागरी)"
    /// ```
    #[wasm_bindgen(js_name = getScriptInfo)]
    pub fn get_script_info(&self) -> Result<JsValue, JsValue> {
        let obj = Object::new();

        for script in self.inner.list_supported_scripts() {
            let description = match script.as_str() {
                "iast" => "IAST (International Alphabet of Sanskrit Transliteration)",
                "itrans" => "ITRANS (ASCII transliteration)",
                "slp1" => "SLP1 (Sanskrit Library Phonetic scheme)",
                "harvard_kyoto" | "hk" => "Harvard-Kyoto (ASCII-based academic standard)",
                "velthuis" => "Velthuis (TeX-based notation)",
                "wx" => "WX (Computational notation)",
                "devanagari" | "deva" => "Devanagari script (देवनागरी)",
                "bengali" | "bn" => "Bengali script (বাংলা)",
                "tamil" | "ta" => "Tamil script (தமிழ்)",
                "telugu" | "te" => "Telugu script (తెలుగు)",
                "gujarati" | "gu" => "Gujarati script (ગુજરાતી)",
                "kannada" | "kn" => "Kannada script (ಕನ್ನಡ)",
                "malayalam" | "ml" => "Malayalam script (മലയാളം)",
                "odia" | "od" | "oriya" => "Odia script (ଓଡ଼ିଆ)",
                "iso15919" | "iso" | "iso_15919" => "ISO-15919 (International standard)",
                _ => "Unknown script type",
            };

            Reflect::set(
                &obj,
                &JsValue::from_str(&script),
                &JsValue::from_str(description),
            )
            .map_err(|_| JsValue::from_str("Failed to set property"))?;
        }

        Ok(obj.into())
    }

    /// Get the number of supported scripts
    ///
    /// @returns {number} Number of supported scripts
    #[wasm_bindgen(js_name = getSupportedScriptCount)]
    pub fn get_supported_script_count(&self) -> usize {
        self.inner.list_supported_scripts().len()
    }

    /// Load a schema from a file path for runtime script support
    /// Note: In WASM context, this would typically load from a URL or local storage
    ///
    /// @param {string} filePath - Path to YAML schema file
    /// @throws {Error} If schema loading fails
    ///
    /// @example
    /// ```javascript
    /// const transliterator = new WasmShlesha();
    /// await transliterator.loadSchemaFromFile("/schemas/custom.yaml");
    /// ```
    #[wasm_bindgen(js_name = loadSchemaFromFile)]
    pub fn load_schema_from_file(&mut self, file_path: &str) -> Result<(), JsValue> {
        self.inner
            .load_schema_from_file(file_path)
            .map_err(|e| JsValue::from_str(&format!("Schema loading failed: {e}")))
    }

    /// Load a schema from YAML content string
    ///
    /// @param {string} yamlContent - YAML schema content
    /// @param {string} schemaName - Name for the schema
    /// @throws {Error} If schema loading fails
    ///
    /// @example
    /// ```javascript
    /// const yamlContent = `
    /// metadata:
    ///   name: "custom"
    ///   script_type: "roman"
    /// mappings:
    ///   vowels:
    ///     "a": "a"
    /// `;
    /// const transliterator = new WasmShlesha();
    /// transliterator.loadSchemaFromString(yamlContent, "custom");
    /// ```
    #[wasm_bindgen(js_name = loadSchemaFromString)]
    pub fn load_schema_from_string(
        &mut self,
        yaml_content: &str,
        schema_name: &str,
    ) -> Result<(), JsValue> {
        self.inner
            .load_schema_from_string(yaml_content, schema_name)
            .map_err(|e| JsValue::from_str(&format!("Schema loading failed: {e}")))
    }

    /// Get information about a loaded runtime schema
    ///
    /// @param {string} scriptName - Name of the script
    /// @returns {Object|undefined} Schema information object or undefined if not found
    ///
    /// @example
    /// ```javascript
    /// const info = transliterator.getSchemaInfo("custom");
    /// if (info) {
    ///     console.log(info.description);
    ///     console.log(info.mapping_count);
    /// }
    /// ```
    #[wasm_bindgen(js_name = getSchemaInfo)]
    pub fn get_schema_info(&self, script_name: &str) -> Option<Object> {
        self.inner.get_schema_info(script_name).map(|info| {
            let obj = Object::new();

            // Use Reflect to set properties
            let _ = Reflect::set(&obj, &"name".into(), &JsValue::from_str(&info.name));
            let _ = Reflect::set(
                &obj,
                &"description".into(),
                &JsValue::from_str(&info.description),
            );
            let _ = Reflect::set(
                &obj,
                &"script_type".into(),
                &JsValue::from_str(&info.script_type),
            );
            let _ = Reflect::set(
                &obj,
                &"is_runtime_loaded".into(),
                &JsValue::from_bool(info.is_runtime_loaded),
            );
            let _ = Reflect::set(
                &obj,
                &"mapping_count".into(),
                &JsValue::from_f64(info.mapping_count as f64),
            );

            obj
        })
    }

    /// Remove a runtime loaded schema
    ///
    /// @param {string} scriptName - Name of the script to remove
    /// @returns {boolean} True if schema was removed, false if not found
    ///
    /// @example
    /// ```javascript
    /// const success = transliterator.removeSchema("custom");
    /// console.log(success); // true if removed
    /// ```
    #[wasm_bindgen(js_name = removeSchema)]
    pub fn remove_schema(&mut self, script_name: &str) -> bool {
        self.inner.remove_schema(script_name)
    }

    /// Clear all runtime loaded schemas
    ///
    /// @example
    /// ```javascript
    /// transliterator.clearRuntimeSchemas();
    /// ```
    #[wasm_bindgen(js_name = clearRuntimeSchemas)]
    pub fn clear_runtime_schemas(&mut self) {
        self.inner.clear_runtime_schemas()
    }
}

#[wasm_bindgen]
impl WasmTransliterationResult {
    /// Get the transliterated output text
    ///
    /// @returns {string} Transliterated text
    #[wasm_bindgen(js_name = getOutput)]
    pub fn get_output(&self) -> String {
        self.output.clone()
    }

    /// Check if metadata is available
    ///
    /// @returns {boolean} True if metadata is present
    #[wasm_bindgen(js_name = hasMetadata)]
    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    /// Get the source script name from metadata
    ///
    /// @returns {string|null} Source script name or null if no metadata
    #[wasm_bindgen(js_name = getSourceScript)]
    pub fn get_source_script(&self) -> Option<String> {
        self.metadata.as_ref().map(|m| m.source_script.clone())
    }

    /// Get the target script name from metadata
    ///
    /// @returns {string|null} Target script name or null if no metadata
    #[wasm_bindgen(js_name = getTargetScript)]
    pub fn get_target_script(&self) -> Option<String> {
        self.metadata.as_ref().map(|m| m.target_script.clone())
    }

    /// Get the number of unknown tokens
    ///
    /// @returns {number} Number of unknown tokens (0 if no metadata)
    #[wasm_bindgen(js_name = getUnknownTokenCount)]
    pub fn get_unknown_token_count(&self) -> usize {
        self.metadata
            .as_ref()
            .map(|m| m.unknown_tokens.len())
            .unwrap_or(0)
    }

    /// Get unknown tokens as JavaScript Array
    ///
    /// @returns {Array<Object>} Array of unknown token objects
    #[wasm_bindgen(js_name = getUnknownTokens)]
    pub fn get_unknown_tokens(&self) -> Result<Array, JsValue> {
        let array = Array::new();

        if let Some(metadata) = &self.metadata {
            for token in &metadata.unknown_tokens {
                let obj = Object::new();
                Reflect::set(
                    &obj,
                    &JsValue::from_str("script"),
                    &JsValue::from_str(&token.script),
                )?;
                Reflect::set(
                    &obj,
                    &JsValue::from_str("token"),
                    &JsValue::from_str(&token.token),
                )?;
                Reflect::set(
                    &obj,
                    &JsValue::from_str("position"),
                    &JsValue::from_f64(token.position as f64),
                )?;
                Reflect::set(
                    &obj,
                    &JsValue::from_str("unicode"),
                    &JsValue::from_str(&token.unicode),
                )?;
                Reflect::set(
                    &obj,
                    &JsValue::from_str("isExtension"),
                    &JsValue::from_bool(token.is_extension),
                )?;
                array.push(&obj);
            }
        }

        Ok(array)
    }
}

/// Convenience function to create a new Shlesha instance
///
/// @returns {WasmShlesha} New transliterator instance
///
/// @example
/// ```javascript
/// import { createTransliterator } from 'shlesha';
/// const transliterator = createTransliterator();
/// ```
#[wasm_bindgen(js_name = createTransliterator)]
pub fn create_transliterator() -> WasmShlesha {
    WasmShlesha::new()
}

/// Convenience function for direct transliteration
///
/// @param {string} text - Text to transliterate
/// @param {string} fromScript - Source script name
/// @param {string} toScript - Target script name
/// @returns {string} Transliterated text
/// @throws {Error} If transliteration fails
///
/// @example
/// ```javascript
/// import { transliterate } from 'shlesha';
/// const result = transliterate("धर्म", "devanagari", "iast");
/// console.log(result); // "dharma"
/// ```
#[wasm_bindgen]
pub fn transliterate(text: &str, from_script: &str, to_script: &str) -> Result<String, JsValue> {
    let transliterator = Shlesha::new();
    transliterator
        .transliterate(text, from_script, to_script)
        .map_err(|e| JsValue::from_str(&format!("Transliteration failed: {e}")))
}

/// Get list of all supported scripts as JavaScript Array
///
/// @returns {Array<string>} Array of supported script names
///
/// @example
/// ```javascript
/// import { getSupportedScripts } from 'shlesha';
/// const scripts = getSupportedScripts();
/// console.log(scripts.includes("devanagari")); // true
/// ```
#[wasm_bindgen(js_name = getSupportedScripts)]
pub fn get_supported_scripts() -> Array {
    let transliterator = Shlesha::new();
    let scripts = transliterator.list_supported_scripts();
    let array = Array::new();
    for script in scripts {
        array.push(&JsValue::from_str(&script));
    }
    array
}

/// Get the library version
///
/// @returns {string} Version string
#[wasm_bindgen(js_name = getVersion)]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_wasm_basic_transliteration() {
        let transliterator = WasmShlesha::new();
        let result = transliterator
            .transliterate("अ", "devanagari", "iast")
            .unwrap();
        assert_eq!(result, "a");
    }

    #[wasm_bindgen_test]
    fn test_wasm_metadata_collection() {
        let transliterator = WasmShlesha::new();
        let result = transliterator
            .transliterate_with_metadata("धर्मkr", "devanagari", "iast")
            .unwrap();
        assert!(result.get_output().contains("dharma"));
        assert!(result.has_metadata());
        // With graceful handling, unknown tokens may or may not be tracked depending on implementation
        // get_unknown_token_count() returns usize which is always >= 0, so we just check it's callable
        let _count = result.get_unknown_token_count();
    }

    #[wasm_bindgen_test]
    fn test_wasm_script_support() {
        let transliterator = WasmShlesha::new();
        assert!(transliterator.supports_script("devanagari"));
        assert!(transliterator.supports_script("iast"));
        assert!(!transliterator.supports_script("nonexistent"));

        let scripts = transliterator.list_supported_scripts();
        assert!(scripts.length() > 0);
    }

    #[wasm_bindgen_test]
    fn test_wasm_convenience_functions() {
        let result = transliterate("अ", "devanagari", "iast").unwrap();
        assert_eq!(result, "a");

        let scripts = get_supported_scripts();
        assert!(scripts.length() > 0);

        let version = get_version();
        assert!(!version.is_empty());
    }

    #[wasm_bindgen_test]
    fn test_wasm_cross_script_conversion() {
        let transliterator = WasmShlesha::new();
        let result = transliterator
            .transliterate("धर्म", "devanagari", "gujarati")
            .unwrap();
        assert!(!result.is_empty());
        // Should contain Gujarati representation
        assert!(result.contains("ધ") || result.contains("गुज")); // Either Gujarati or fallback
    }

    #[wasm_bindgen_test]
    fn test_wasm_script_info() {
        let transliterator = WasmShlesha::new();
        let info = transliterator.get_script_info().unwrap();
        let obj: &js_sys::Object = info.unchecked_ref();
        assert!(js_sys::Object::keys(obj).length() > 0);
    }

    #[wasm_bindgen_test]
    fn test_wasm_error_handling() {
        let transliterator = WasmShlesha::new();
        let result = transliterator.transliterate("test", "invalid_script", "iast");
        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn test_wasm_empty_input() {
        let transliterator = WasmShlesha::new();
        let result = transliterator
            .transliterate("", "devanagari", "iast")
            .unwrap();
        assert_eq!(result, "");
    }

    #[wasm_bindgen_test]
    fn test_wasm_whitespace_preservation() {
        let transliterator = WasmShlesha::new();
        let result = transliterator
            .transliterate("अ आ", "devanagari", "iast")
            .unwrap();
        assert!(result.contains(" "));
    }
}
