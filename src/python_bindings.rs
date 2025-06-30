//! Python bindings for Shlesha transliteration library
//!
//! This module provides Python access to all Shlesha functionality including:
//! - Basic transliteration between scripts
//! - Metadata collection for unknown tokens
//! - Script discovery and validation
//! - Runtime schema loading

use pyo3::prelude::*;
use std::collections::HashMap;
use once_cell::sync::Lazy;

use crate::Shlesha;

// Global transliterator instance for convenience function
static GLOBAL_TRANSLITERATOR: Lazy<Shlesha> = Lazy::new(|| Shlesha::new());

/// Python wrapper for the Shlesha transliterator
#[pyclass(unsendable)]
pub struct PyShlesha {
    inner: Shlesha,
}

/// Python wrapper for transliteration metadata
#[pyclass]
#[derive(Clone)]
pub struct PyTransliterationMetadata {
    #[pyo3(get)]
    source_script: String,
    #[pyo3(get)]
    target_script: String,
    #[pyo3(get)]
    used_extensions: String,
    #[pyo3(get)]
    unknown_tokens: Vec<PyUnknownToken>,
}

/// Python wrapper for unknown token information
#[pyclass]
#[derive(Clone)]
pub struct PyUnknownToken {
    #[pyo3(get)]
    script: String,
    #[pyo3(get)]
    token: String,
    #[pyo3(get)]
    position: usize,
    #[pyo3(get)]
    unicode: String,
    #[pyo3(get)]
    is_extension: bool,
}

/// Python wrapper for transliteration result with metadata
#[pyclass]
pub struct PyTransliterationResult {
    #[pyo3(get)]
    output: String,
    #[pyo3(get)]
    metadata: Option<PyTransliterationMetadata>,
}

#[pymethods]
impl PyShlesha {
    /// Create a new Shlesha transliterator instance
    #[new]
    fn new() -> Self {
        Self {
            inner: Shlesha::new(),
        }
    }

    /// Transliterate text from one script to another
    ///
    /// Args:
    ///     text (str): Text to transliterate
    ///     from_script (str): Source script name
    ///     to_script (str): Target script name
    ///
    /// Returns:
    ///     str: Transliterated text
    ///
    /// Raises:
    ///     RuntimeError: If transliteration fails
    ///
    /// Example:
    ///     >>> transliterator = Shlesha()
    ///     >>> result = transliterator.transliterate("धर्म", "devanagari", "iast")
    ///     >>> print(result)  # "dharma"
    fn transliterate(&self, text: &str, from_script: &str, to_script: &str) -> PyResult<String> {
        self.inner
            .transliterate(text, from_script, to_script)
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Transliteration failed: {e}"
                ))
            })
    }

    /// Transliterate text with metadata collection for unknown tokens
    ///
    /// Args:
    ///     text (str): Text to transliterate
    ///     from_script (str): Source script name
    ///     to_script (str): Target script name
    ///
    /// Returns:
    ///     PyTransliterationResult: Result with output and metadata
    ///
    /// Raises:
    ///     RuntimeError: If transliteration fails
    ///
    /// Example:
    ///     >>> transliterator = Shlesha()
    ///     >>> result = transliterator.transliterate_with_metadata("धर्मkr", "devanagari", "iast")
    ///     >>> print(result.output)  # "dharmakr"
    ///     >>> print(len(result.metadata.unknown_tokens))  # 2 (for 'k' and 'r')
    fn transliterate_with_metadata(
        &self,
        text: &str,
        from_script: &str,
        to_script: &str,
    ) -> PyResult<PyTransliterationResult> {
        let result = self
            .inner
            .transliterate_with_metadata(text, from_script, to_script)
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Transliteration failed: {e}"
                ))
            })?;

        let py_metadata = result.metadata.map(|metadata| {
            let unknown_tokens = metadata
                .unknown_tokens
                .into_iter()
                .map(|token| PyUnknownToken {
                    script: token.script,
                    token: token.token.to_string(),
                    position: token.position,
                    unicode: token.unicode,
                    is_extension: token.is_extension,
                })
                .collect();

            PyTransliterationMetadata {
                source_script: metadata.source_script,
                target_script: metadata.target_script,
                used_extensions: metadata.used_extensions.to_string(),
                unknown_tokens,
            }
        });

        Ok(PyTransliterationResult {
            output: result.output,
            metadata: py_metadata,
        })
    }

    /// Get list of supported scripts
    ///
    /// Returns:
    ///     List[str]: List of supported script names
    ///
    /// Example:
    ///     >>> transliterator = Shlesha()
    ///     >>> scripts = transliterator.list_supported_scripts()
    ///     >>> print("devanagari" in scripts)  # True
    fn list_supported_scripts(&self) -> Vec<String> {
        self.inner
            .list_supported_scripts()
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    /// Check if a script is supported
    ///
    /// Args:
    ///     script (str): Script name to check
    ///
    /// Returns:
    ///     bool: True if script is supported
    ///
    /// Example:
    ///     >>> transliterator = Shlesha()
    ///     >>> print(transliterator.supports_script("devanagari"))  # True
    ///     >>> print(transliterator.supports_script("unknown"))     # False
    fn supports_script(&self, script: &str) -> bool {
        self.inner.supports_script(script)
    }

    /// Load a schema from a file path for runtime script support
    ///
    /// Args:
    ///     file_path (str): Path to YAML schema file
    ///
    /// Raises:
    ///     RuntimeError: If schema loading fails
    ///
    /// Example:
    ///     >>> transliterator = Shlesha()
    ///     >>> transliterator.load_schema_from_file("custom_script.yaml")
    fn load_schema_from_file(&mut self, file_path: &str) -> PyResult<()> {
        self.inner.load_schema_from_file(file_path).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Schema loading failed: {e}"))
        })
    }

    /// Load a schema from YAML content string
    ///
    /// Args:
    ///     yaml_content (str): YAML schema content
    ///     schema_name (str): Name for the schema
    ///
    /// Raises:
    ///     RuntimeError: If schema loading fails
    ///
    /// Example:
    ///     >>> yaml_content = '''
    ///     ... metadata:
    ///     ...   name: "custom"
    ///     ...   script_type: "roman"
    ///     ... mappings:
    ///     ...   vowels:
    ///     ...     "a": "a"
    ///     ... '''
    ///     >>> transliterator = Shlesha()
    ///     >>> transliterator.load_schema_from_string(yaml_content, "custom")
    fn load_schema_from_string(&mut self, yaml_content: &str, schema_name: &str) -> PyResult<()> {
        self.inner
            .load_schema_from_string(yaml_content, schema_name)
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Schema loading failed: {e}"
                ))
            })
    }

    /// Get information about a loaded runtime schema
    ///
    /// Args:
    ///     script_name (str): Name of the script
    ///
    /// Returns:
    ///     Dict[str, Any] | None: Schema information or None if not found
    ///
    /// Example:
    ///     >>> info = transliterator.get_schema_info("custom")
    ///     >>> print(info["description"])
    fn get_schema_info(&self, py: Python<'_>, script_name: &str) -> PyResult<Option<PyObject>> {
        Ok(self.inner.get_schema_info(script_name).map(|info| {
            let dict = pyo3::types::PyDict::new(py);
            dict.set_item("name", info.name).unwrap();
            dict.set_item("description", info.description).unwrap();
            dict.set_item("script_type", info.script_type).unwrap();
            dict.set_item("is_runtime_loaded", info.is_runtime_loaded)
                .unwrap();
            dict.set_item("mapping_count", info.mapping_count).unwrap();
            dict.into()
        }))
    }

    /// Remove a runtime loaded schema
    ///
    /// Args:
    ///     script_name (str): Name of the script to remove
    ///
    /// Returns:
    ///     bool: True if schema was removed, False if not found
    ///
    /// Example:
    ///     >>> success = transliterator.remove_schema("custom")
    ///     >>> print(success)  # True if removed
    fn remove_schema(&mut self, script_name: &str) -> bool {
        self.inner.remove_schema(script_name)
    }

    /// Clear all runtime loaded schemas
    ///
    /// Example:
    ///     >>> transliterator.clear_runtime_schemas()
    fn clear_runtime_schemas(&mut self) {
        self.inner.clear_runtime_schemas()
    }

    /// Get script information as a dictionary
    ///
    /// Returns:
    ///     Dict[str, str]: Mapping of script names to descriptions
    fn get_script_info(&self) -> HashMap<String, String> {
        let mut info = HashMap::new();

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
            info.insert(script.to_string(), description.to_string());
        }

        info
    }

    /// Python representation
    fn __repr__(&self) -> String {
        let scripts = self.inner.list_supported_scripts();
        format!("Shlesha(supported_scripts={})", scripts.len())
    }

    /// Python string representation
    fn __str__(&self) -> String {
        format!(
            "Shlesha transliterator with {} supported scripts",
            self.inner.list_supported_scripts().len()
        )
    }
}

#[pymethods]
impl PyTransliterationResult {
    /// Python representation
    fn __repr__(&self) -> String {
        match &self.metadata {
            Some(metadata) => format!(
                "TransliterationResult(output='{}', unknown_tokens={})",
                self.output,
                metadata.unknown_tokens.len()
            ),
            None => format!("TransliterationResult(output='{}')", self.output),
        }
    }
}

#[pymethods]
impl PyTransliterationMetadata {
    /// Python representation
    fn __repr__(&self) -> String {
        format!(
            "TransliterationMetadata(source='{}', target='{}', unknown_tokens={})",
            self.source_script,
            self.target_script,
            self.unknown_tokens.len()
        )
    }
}

#[pymethods]
impl PyUnknownToken {
    /// Python representation
    fn __repr__(&self) -> String {
        format!(
            "UnknownToken(script='{}', token='{}', position={})",
            self.script, self.token, self.position
        )
    }
}

/// Convenience function to create a new Shlesha instance
///
/// Returns:
///     PyShlesha: New transliterator instance
///
/// Example:
///     >>> from shlesha import Shlesha
///     >>> transliterator = Shlesha()
#[pyfunction]
fn create_transliterator() -> PyShlesha {
    PyShlesha::new()
}

/// Convenience function for direct transliteration
///
/// Args:
///     text (str): Text to transliterate
///     from_script (str): Source script name
///     to_script (str): Target script name
///
/// Returns:
///     str: Transliterated text
///
/// Example:
///     >>> from shlesha import transliterate
///     >>> result = transliterate("धर्म", "devanagari", "iast")
///     >>> print(result)  # "dharma"
#[pyfunction]
fn transliterate(text: &str, from_script: &str, to_script: &str) -> PyResult<String> {
    GLOBAL_TRANSLITERATOR
        .transliterate(text, from_script, to_script)
        .map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Transliteration failed: {e}"
            ))
        })
}

/// Get list of all supported scripts
///
/// Returns:
///     List[str]: List of supported script names
///
/// Example:
///     >>> from shlesha import get_supported_scripts
///     >>> scripts = get_supported_scripts()
///     >>> print("devanagari" in scripts)  # True
#[pyfunction]
fn get_supported_scripts() -> Vec<String> {
    let transliterator = Shlesha::new();
    transliterator
        .list_supported_scripts()
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

/// Python module definition
#[pymodule]
fn shlesha(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add classes
    m.add_class::<PyShlesha>()?;
    m.add_class::<PyTransliterationResult>()?;
    m.add_class::<PyTransliterationMetadata>()?;
    m.add_class::<PyUnknownToken>()?;

    // Add convenience functions
    m.add_function(wrap_pyfunction!(create_transliterator, m)?)?;
    m.add_function(wrap_pyfunction!(transliterate, m)?)?;
    m.add_function(wrap_pyfunction!(get_supported_scripts, m)?)?;

    // Add module metadata
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__author__", "Shlesha Contributors")?;
    m.add(
        "__description__",
        "High-performance extensible transliteration library",
    )?;

    Ok(())
}

// Note: PyShlesha is exported as the main Shlesha class for Python through the module

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_basic_transliteration() {
        let transliterator = PyShlesha::new();
        let result = transliterator
            .transliterate("अ", "devanagari", "iast")
            .unwrap();
        assert_eq!(result, "a");
    }

    #[test]
    fn test_python_metadata_collection() {
        let transliterator = PyShlesha::new();
        let result = transliterator
            .transliterate_with_metadata("धर्मkr", "devanagari", "iast")
            .unwrap();
        assert!(result.output.contains("dharma"));
        assert!(result.metadata.is_some());

        let metadata = result.metadata.unwrap();
        assert_eq!(metadata.source_script, "devanagari");
        assert_eq!(metadata.target_script, "iast");
        // Should have unknown tokens for 'k' and 'r'
        assert!(metadata.unknown_tokens.len() > 0);
    }

    #[test]
    fn test_python_script_support() {
        let transliterator = PyShlesha::new();
        assert!(transliterator.supports_script("devanagari"));
        assert!(transliterator.supports_script("iast"));
        assert!(!transliterator.supports_script("nonexistent"));

        let scripts = transliterator.list_supported_scripts();
        assert!(!scripts.is_empty());
        assert!(scripts.iter().any(|s| s == "devanagari"));
    }

    #[test]
    fn test_convenience_functions() {
        let result = transliterate("अ", "devanagari", "iast").unwrap();
        assert_eq!(result, "a");

        let scripts = get_supported_scripts();
        assert!(!scripts.is_empty());
        assert!(scripts.iter().any(|s| s == "devanagari"));
    }
}
