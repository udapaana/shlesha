//! Build-time support for mapping data
//! 
//! This module is designed to be used from build.rs to generate high-performance
//! static mappings at compile time.

use std::path::Path;
use std::fs;

/// Generate all static mapping modules from TOML files
pub fn generate_all_static_mappings() -> Result<(), Box<dyn std::error::Error>> {
    let mappings_dir = Path::new("mappings/base");
    let output_dir = Path::new("src/modules/mapping_data/generated");
    
    // Create output directory
    fs::create_dir_all(output_dir)?;
    
    // Generate mod.rs for the generated module
    let mut mod_content = String::from(
        "//! Auto-generated mapping modules\n\
         //! DO NOT EDIT MANUALLY\n\n"
    );
    
    // Process each TOML file
    for entry in fs::read_dir(mappings_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let stem = path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            
            // Skip metadata file
            if stem == "metadata" {
                continue;
            }
            
            let module_name = stem.replace('-', "_");
            let output_file = output_dir.join(format!("{}.rs", module_name));
            
            // Generate the static module
            generate_optimized_module(&path, &output_file)?;
            
            // Add to mod.rs
            mod_content.push_str(&format!("pub mod {};\n", module_name));
        }
    }
    
    // Write mod.rs
    fs::write(output_dir.join("mod.rs"), mod_content)?;
    
    Ok(())
}

/// Generate an optimized module with static mappings
fn generate_optimized_module(
    toml_path: &Path,
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    use super::loader::{load_mapping_file, flatten_mappings};
    
    let mapping_file = load_mapping_file(toml_path)?;
    let mut code = String::new();
    
    // Header
    code.push_str(&format!(
        "//! Auto-generated from {}\n\
         //! DO NOT EDIT MANUALLY\n\
         //!\n\
         //! This module provides zero-cost static mappings for runtime performance.\n\n",
        toml_path.display()
    ));
    
    // For the Hub (ISO <-> Devanagari), generate char-based mappings
    if toml_path.file_name().unwrap() == "iso_devanagari.toml" {
        code.push_str(&generate_hub_optimized_code(&mapping_file)?);
    } else {
        code.push_str(&generate_standard_mappings(&mapping_file)?);
    }
    
    fs::write(output_path, code)?;
    Ok(())
}

/// Generate highly optimized code for the hub mappings
fn generate_hub_optimized_code(
    mapping_file: &super::loader::MappingFile,
) -> Result<String, Box<dyn std::error::Error>> {
    use super::loader::flatten_mappings;
    
    let mut code = String::new();
    code.push_str("use once_cell::sync::Lazy;\n");
    code.push_str("use rustc_hash::FxHashMap;\n\n");
    
    // Generate static arrays for maximum performance
    code.push_str("/// ISO to Devanagari mappings (static slices for zero allocation)\n");
    code.push_str("pub static ISO_TO_DEVA_PAIRS: &[(&str, char)] = &[\n");
    
    if let Some(mappings) = &mapping_file.mappings {
        let flat = flatten_mappings(mappings);
        let mut sorted: Vec<_> = flat.iter().collect();
        sorted.sort_by_key(|(k, _)| k.as_str());
        
        for (key, value) in sorted {
            if value.chars().count() == 1 {
                let ch = value.chars().next().unwrap();
                code.push_str(&format!("    ({:?}, '{}'),\n", key, ch));
            }
        }
    }
    code.push_str("];\n\n");
    
    // Generate lazy static HashMap for convenience
    code.push_str("/// Lazy-initialized HashMap for ISO to Devanagari\n");
    code.push_str("pub static ISO_TO_DEVA: Lazy<FxHashMap<&'static str, char>> = Lazy::new(|| {\n");
    code.push_str("    ISO_TO_DEVA_PAIRS.iter().copied().collect()\n");
    code.push_str("});\n\n");
    
    // Generate reverse mappings
    code.push_str("/// Devanagari to ISO mappings\n");
    code.push_str("pub static DEVA_TO_ISO_PAIRS: &[(char, &str)] = &[\n");
    
    if let Some(mappings) = &mapping_file.mappings {
        let flat = flatten_mappings(mappings);
        let mut sorted: Vec<_> = flat.iter().collect();
        sorted.sort_by_key(|(_, v)| v.as_str());
        
        for (key, value) in sorted {
            if value.chars().count() == 1 {
                let ch = value.chars().next().unwrap();
                code.push_str(&format!("    ('{}', {:?}),\n", ch, key));
            }
        }
    }
    code.push_str("];\n\n");
    
    code.push_str("pub static DEVA_TO_ISO: Lazy<FxHashMap<char, &'static str>> = Lazy::new(|| {\n");
    code.push_str("    DEVA_TO_ISO_PAIRS.iter().copied().collect()\n");
    code.push_str("});\n");
    
    Ok(code)
}

/// Generate standard string-based mappings
fn generate_standard_mappings(
    mapping_file: &super::loader::MappingFile,
) -> Result<String, Box<dyn std::error::Error>> {
    use super::loader::flatten_mappings;
    
    let mut code = String::new();
    code.push_str("use once_cell::sync::Lazy;\n");
    code.push_str("use rustc_hash::FxHashMap;\n\n");
    
    // Generate mappings based on what's in the file
    if let Some(to_iso) = &mapping_file.to_iso {
        let flat = flatten_mappings(to_iso);
        code.push_str(&generate_static_array("TO_ISO_PAIRS", &flat));
        code.push_str(&generate_lazy_hashmap("TO_ISO", "TO_ISO_PAIRS"));
    }
    
    if let Some(from_iso) = &mapping_file.from_iso {
        code.push_str(&generate_static_array("FROM_ISO_PAIRS", from_iso));
        code.push_str(&generate_lazy_hashmap("FROM_ISO", "FROM_ISO_PAIRS"));
    }
    
    if let Some(mappings) = &mapping_file.mappings {
        let flat = flatten_mappings(mappings);
        code.push_str(&generate_static_array("MAPPINGS_PAIRS", &flat));
        code.push_str(&generate_lazy_hashmap("MAPPINGS", "MAPPINGS_PAIRS"));
    }
    
    Ok(code)
}

fn generate_static_array(name: &str, mappings: &HashMap<String, String>) -> String {
    let mut code = format!("pub static {}: &[(&str, &str)] = &[\n", name);
    
    let mut sorted: Vec<_> = mappings.iter().collect();
    sorted.sort_by_key(|(k, _)| k.as_str());
    
    for (key, value) in sorted {
        code.push_str(&format!("    ({:?}, {:?}),\n", key, value));
    }
    
    code.push_str("];\n\n");
    code
}

fn generate_lazy_hashmap(name: &str, array_name: &str) -> String {
    format!(
        "pub static {}: Lazy<FxHashMap<&'static str, &'static str>> = Lazy::new(|| {{\n\
         \    {}.iter().copied().collect()\n\
         }});\n\n",
        name, array_name
    )
}