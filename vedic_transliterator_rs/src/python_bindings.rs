//! Python bindings for Shlesha transliteration engine
//! 
//! Provides high-performance Python API using PyO3

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::exceptions::PyRuntimeError;

#[cfg(feature = "python")]
use crate::{transliterate, TargetScheme, TransliterationError, TransliterationCompiler};

#[cfg(feature = "python")]
#[pyclass]
pub struct Transliterator {
    compiler: TransliterationCompiler,
    from_scheme: TargetScheme,
    to_scheme: TargetScheme,
}

#[cfg(feature = "python")]
#[pymethods]
impl Transliterator {
    #[new]
    fn new(from_script: &str, to_script: &str) -> PyResult<Self> {
        let from_scheme = parse_scheme_name(from_script)
            .map_err(|e| PyRuntimeError::new_err(format!("Invalid from_script: {}", e)))?;
        let to_scheme = parse_scheme_name(to_script)
            .map_err(|e| PyRuntimeError::new_err(format!("Invalid to_script: {}", e)))?;
        
        let mut compiler = TransliterationCompiler::new();
        Self::initialize_builtin_schemes(&mut compiler)
            .map_err(|e| PyRuntimeError::new_err(format!("Failed to load schemas: {}", e)))?;
        
        Ok(Transliterator {
            compiler,
            from_scheme,
            to_scheme,
        })
    }
    
    fn transliterate(&self, text: &str) -> PyResult<String> {
        // Parse input to tokens
        let tokens = self.compiler.parse(text, self.from_scheme)
            .map_err(|e| PyRuntimeError::new_err(format!("Parse error: {}", e)))?;
        
        // Render tokens to output
        let output = self.compiler.render(&tokens, self.to_scheme)
            .map_err(|e| PyRuntimeError::new_err(format!("Render error: {}", e)))?;
        
        Ok(output)
    }
    
    #[getter]
    fn from_script(&self) -> String {
        self.from_scheme.to_string()
    }
    
    #[getter] 
    fn to_script(&self) -> String {
        self.to_scheme.to_string()
    }
}

#[cfg(feature = "python")]
fn parse_scheme_name(name: &str) -> Result<TargetScheme, String> {
    match name.to_lowercase().as_str() {
        "devanagari" => Ok(TargetScheme::Devanagari),
        "iast" => Ok(TargetScheme::Iast),
        "slp1" => Ok(TargetScheme::Slp1),
        "iso15919" => Ok(TargetScheme::Iso15919),
        _ => Err(format!("Unsupported scheme: {}", name)),
    }
}

#[cfg(feature = "python")]
#[pyfunction]
fn quick_transliterate(text: &str, from_script: &str, to_script: &str) -> PyResult<String> {
    let from_scheme = parse_scheme_name(from_script)
        .map_err(|e| PyRuntimeError::new_err(e))?;
    let to_scheme = parse_scheme_name(to_script)
        .map_err(|e| PyRuntimeError::new_err(e))?;
    
    let result = transliterate(text, from_scheme, to_scheme)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    
    Ok(result.text)
}

#[cfg(feature = "python")]
#[pymodule]
fn shlesha(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Transliterator>()?;
    m.add_function(wrap_pyfunction!(quick_transliterate, m)?)?;
    
    // Add version info
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__doc__", "High-performance Sanskrit transliteration engine")?;
    
    Ok(())
}

// Python-specific helper functions
#[cfg(feature = "python")]
impl Transliterator {
    /// Initialize builtin schemes (Python-specific helper)
    fn initialize_builtin_schemes(compiler: &mut TransliterationCompiler) -> Result<(), TransliterationError> {
        // The schemas are loaded on-demand via the caching system
        // This is just a placeholder for the Python bindings
        Ok(())
    }
}