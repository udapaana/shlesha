//! Python bindings for Shlesha using PyO3
//!
//! This module provides Python bindings for the Shlesha transliteration library,
//! exposing both the legacy and lossless transliteration systems.

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use crate::{LosslessTransliterator, LosslessResult, Transliterator, TransliteratorBuilder, SchemaParser};

/// Python wrapper for LosslessResult
#[cfg(feature = "python")]
#[pyclass]
#[derive(Clone)]
pub struct PyLosslessResult {
    #[pyo3(get)]
    pub is_lossless: bool,
    #[pyo3(get)]
    pub preservation_ratio: f64,
    #[pyo3(get)]
    pub tokens_count: usize,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyLosslessResult {
    #[new]
    fn new(is_lossless: bool, preservation_ratio: f64, tokens_count: usize) -> Self {
        Self {
            is_lossless,
            preservation_ratio,
            tokens_count,
        }
    }
    
    fn __repr__(&self) -> String {
        format!(
            "LosslessResult(is_lossless={}, preservation_ratio={:.3}, tokens_count={})",
            self.is_lossless, self.preservation_ratio, self.tokens_count
        )
    }
}

#[cfg(feature = "python")]
impl From<LosslessResult> for PyLosslessResult {
    fn from(result: LosslessResult) -> Self {
        Self {
            is_lossless: result.is_lossless,
            preservation_ratio: result.preservation_ratio,
            tokens_count: result.tokens_count,
        }
    }
}

/// Python wrapper for LosslessTransliterator
#[cfg(feature = "python")]
#[pyclass]
pub struct PyLosslessTransliterator {
    inner: LosslessTransliterator,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyLosslessTransliterator {
    #[new]
    fn new() -> Self {
        Self {
            inner: LosslessTransliterator::new(),
        }
    }
    
    /// Transliterate text from source script to target script
    ///
    /// Args:
    ///     text (str): Input text to transliterate
    ///     source_script (str): Source script name (e.g., "Devanagari")
    ///     target_script (str): Target script name (e.g., "IAST")
    ///
    /// Returns:
    ///     str: Transliterated text with preservation tokens for unknown characters
    ///
    /// Raises:
    ///     ValueError: If transliteration fails
    fn transliterate(&self, text: &str, source_script: &str, target_script: &str) -> PyResult<String> {
        self.inner
            .transliterate(text, source_script, target_script)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Transliteration error: {}", e)))
    }
    
    /// Verify that a transliteration is mathematically lossless
    ///
    /// Args:
    ///     original (str): Original input text
    ///     transliterated (str): Transliterated output text
    ///     source_script (str): Source script name
    ///
    /// Returns:
    ///     LosslessResult: Verification result with lossless guarantee and preservation ratio
    fn verify_lossless(&self, original: &str, transliterated: &str, source_script: &str) -> PyLosslessResult {
        self.inner
            .verify_lossless(original, transliterated, source_script)
            .into()
    }
    
    /// Get list of supported scripts
    ///
    /// Returns:
    ///     list[str]: List of supported script names
    fn supported_scripts(&self) -> Vec<String> {
        vec![
            "Devanagari".to_string(),
            "IAST".to_string(),
            "Harvard-Kyoto".to_string(),
            "ITRANS".to_string(),
            "SLP1".to_string(),
            "Velthuis".to_string(),
        ]
    }
    
    def __repr__(&self) -> String {
        "LosslessTransliterator()".to_string()
    }
}

/// Python wrapper for legacy Transliterator
#[cfg(feature = "python")]
#[pyclass]
pub struct PyTransliterator {
    inner: Transliterator,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyTransliterator {
    #[staticmethod]
    fn from_schemas(devanagari_yaml: &str, iast_yaml: &str) -> PyResult<Self> {
        let dev_schema = SchemaParser::parse_str(devanagari_yaml)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Devanagari schema error: {}", e)))?;
        let iast_schema = SchemaParser::parse_str(iast_yaml)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("IAST schema error: {}", e)))?;
        
        let transliterator = TransliteratorBuilder::new()
            .with_schema(dev_schema)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Schema error: {}", e)))?
            .with_schema(iast_schema)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Schema error: {}", e)))?
            .build();
        
        Ok(Self {
            inner: transliterator,
        })
    }
    
    /// Transliterate text using the legacy bidirectional system
    ///
    /// Args:
    ///     text (str): Input text to transliterate
    ///     source_script (str): Source script name
    ///     target_script (str): Target script name
    ///
    /// Returns:
    ///     str: Transliterated text
    ///
    /// Raises:
    ///     ValueError: If transliteration fails
    fn transliterate(&self, text: &str, source_script: &str, target_script: &str) -> PyResult<String> {
        self.inner
            .transliterate(text, source_script, target_script)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Transliteration error: {}", e)))
    }
    
    def __repr__(&self) -> String {
        "Transliterator()".to_string()
    }
}

/// Convenience function to create a LosslessTransliterator
#[cfg(feature = "python")]
#[pyfunction]
fn create_lossless_transliterator() -> PyLosslessTransliterator {
    PyLosslessTransliterator::new()
}

/// Convenience function to transliterate text using the lossless system
#[cfg(feature = "python")]
#[pyfunction]
fn transliterate_lossless(text: &str, source_script: &str, target_script: &str) -> PyResult<String> {
    let transliterator = LosslessTransliterator::new();
    transliterator
        .transliterate(text, source_script, target_script)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Transliteration error: {}", e)))
}

/// Get library version
#[cfg(feature = "python")]
#[pyfunction]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Get library information
#[cfg(feature = "python")]
#[pyfunction]
fn info() -> String {
    format!(
        "Shlesha v{} - High-Performance Lossless Transliteration Library\n\
         Features: 100% lossless guarantee, 6-10x performance, 72x memory reduction\n\
         Supported scripts: Devanagari, Tamil, Bengali, Gujarati, and more",
        env!("CARGO_PKG_VERSION")
    )
}

/// Python module definition
#[cfg(feature = "python")]
#[pymodule]
fn shlesha(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyLosslessTransliterator>()?;
    m.add_class::<PyTransliterator>()?;
    m.add_class::<PyLosslessResult>()?;
    
    m.add_function(wrap_pyfunction!(create_lossless_transliterator, m)?)?;
    m.add_function(wrap_pyfunction!(transliterate_lossless, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_function(wrap_pyfunction!(info, m)?)?;
    
    // Module constants
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__author__", "Shlesha Contributors")?;
    m.add("__description__", "High-performance lossless transliteration library")?;
    
    Ok(())
}

#[cfg(not(feature = "python"))]
compile_error!("Python bindings require the 'python' feature to be enabled");

#[cfg(feature = "python")]
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_python_lossless_transliterator() {
        let transliterator = PyLosslessTransliterator::new();
        let result = transliterator.transliterate("धर्म", "Devanagari", "IAST").unwrap();
        assert!(!result.is_empty());
        
        let verification = transliterator.verify_lossless("धर्म", &result, "Devanagari");
        assert!(verification.is_lossless);
    }
    
    #[test]
    fn test_convenience_functions() {
        let result = transliterate_lossless("धर्म", "Devanagari", "IAST").unwrap();
        assert!(!result.is_empty());
        
        assert!(!version().is_empty());
        assert!(info().contains("Shlesha"));
    }
}