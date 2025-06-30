//! Code generation from TOML mappings
//! 
//! This module generates static Rust code from TOML mappings for zero-overhead runtime performance.

use rustc_hash::FxHashMap;
use std::fs;
use std::path::Path;
use crate::modules::mapping_data::loader::{load_mapping_file, flatten_mappings};

/// Generate static Rust code from TOML mappings
pub fn generate_static_mappings(
    toml_path: &Path,
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mapping_file = load_mapping_file(toml_path)?;
    let mut code = String::new();
    
    // Generate file header
    code.push_str(&format!(
        "//! Auto-generated from {}\n//! DO NOT EDIT MANUALLY\n\n",
        toml_path.display()
    ));
    
    code.push_str("use rustc_hash::FxHashMap;\n\n");
    
    // Generate the mapping function
    if let Some(mappings) = &mapping_file.mappings {
        let flat = flatten_mappings(mappings);
        code.push_str(&generate_mapping_function("get_mappings", &flat));
    }
    
    if let Some(to_iso) = &mapping_file.to_iso {
        let flat = flatten_mappings(to_iso);
        code.push_str(&generate_mapping_function("get_to_iso_mappings", &flat));
    }
    
    if let Some(from_iso) = &mapping_file.from_iso {
        code.push_str(&generate_simple_mapping_function("get_from_iso_mappings", from_iso));
    }
    
    fs::write(output_path, code)?;
    Ok(())
}

/// Generate a static mapping function with &'static str
fn generate_mapping_function(fn_name: &str, mappings: &FxHashMap<String, String>) -> String {
    let mut code = format!(
        "/// Get {} as a static HashMap\n\
         pub fn {}() -> FxHashMap<&'static str, &'static str> {{\n\
         \tlet mut map = FxHashMap::default();\n",
        fn_name, fn_name, mappings.len()
    );
    
    // Sort for consistent output
    let mut sorted: Vec<_> = mappings.iter().collect();
    sorted.sort_by_key(|(k, _)| k.as_str());
    
    for (key, value) in sorted {
        code.push_str(&format!("\tmap.insert({:?}, {:?});\n", key, value));
    }
    
    code.push_str("\tmap\n}\n\n");
    code
}

/// Generate a simple mapping function
fn generate_simple_mapping_function(fn_name: &str, mappings: &FxHashMap<String, String>) -> String {
    let mut code = format!(
        "/// Get {} as a static HashMap\n\
         pub fn {}() -> FxHashMap<&'static str, &'static str> {{\n\
         \tlet mut map = FxHashMap::default();\n",
        fn_name, fn_name, mappings.len()
    );
    
    let mut sorted: Vec<_> = mappings.iter().collect();
    sorted.sort_by_key(|(k, _)| k.as_str());
    
    for (key, value) in sorted {
        code.push_str(&format!("\tmap.insert({:?}, {:?});\n", key, value));
    }
    
    code.push_str("\tmap\n}\n\n");
    code
}

/// Generate optimized char-based mappings for Devanagari
pub fn generate_char_mappings(
    toml_path: &Path,
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mapping_file = load_mapping_file(toml_path)?;
    let mut code = String::new();
    
    code.push_str(&format!(
        "//! Auto-generated from {}\n//! DO NOT EDIT MANUALLY\n\n",
        toml_path.display()
    ));
    
    code.push_str("use rustc_hash::FxHashMap;\n\n");
    
    // For ISO -> Devanagari, we can optimize to char mappings
    if toml_path.file_name().unwrap() == "iso_devanagari.toml" {
        code.push_str(
            "/// Get ISO to Devanagari mappings optimized for single chars\n\
             pub fn get_iso_to_deva_char_mappings() -> FxHashMap<&'static str, char> {\n\
             \tlet mut map = FxHashMap::default();\n"
        );
        
        if let Some(mappings) = &mapping_file.mappings {
            let flat = flatten_mappings(mappings);
            let mut sorted: Vec<_> = flat.iter().collect();
            sorted.sort_by_key(|(k, _)| k.as_str());
            
            for (key, value) in sorted {
                if value.chars().count() == 1 {
                    let ch = value.chars().next().unwrap();
                    code.push_str(&format!("\tmap.insert({:?}, '{}');\n", key, ch));
                }
            }
        }
        
        code.push_str("\tmap\n}\n\n");
        
        // Also generate reverse mapping
        code.push_str(
            "/// Get Devanagari to ISO mappings\n\
             pub fn get_deva_to_iso_mappings() -> FxHashMap<char, &'static str> {\n\
             \tlet mut map = FxHashMap::default();\n"
        );
        
        if let Some(mappings) = &mapping_file.mappings {
            let flat = flatten_mappings(mappings);
            let mut sorted: Vec<_> = flat.iter().collect();
            sorted.sort_by_key(|(k, _)| k.as_str());
            
            for (key, value) in sorted {
                if value.chars().count() == 1 {
                    let ch = value.chars().next().unwrap();
                    code.push_str(&format!("\tmap.insert('{}', {:?});\n", ch, key));
                }
            }
        }
        
        code.push_str("\tmap\n}\n");
    }
    
    fs::write(output_path, code)?;
    Ok(())
}