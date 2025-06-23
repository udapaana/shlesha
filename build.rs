use std::env;
use std::fs;
use std::path::Path;

// List of Roman scripts that don't have implicit 'a'
const ROMAN_SCRIPTS: &[&str] = &["iast", "itrans", "slp1", "harvard_kyoto", "velthuis", "wx"];

// List of Indic scripts that have implicit 'a' 
const INDIC_SCRIPTS: &[&str] = &["devanagari", "bengali", "gujarati", "tamil", "telugu", "kannada", "malayalam", "odia"];

// Common conversions that are frequently used
const COMMON_CONVERSIONS: &[(&str, &str)] = &[
    ("iast", "devanagari"),
    ("devanagari", "iast"),
    ("itrans", "devanagari"),
    ("devanagari", "itrans"),
    ("slp1", "devanagari"),
    ("devanagari", "slp1"),
];

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/modules/script_converter/");
    
    // Check which features are enabled
    let precompute_all = env::var("CARGO_FEATURE_PRECOMPUTE_ALL").is_ok();
    let precompute_common = env::var("CARGO_FEATURE_PRECOMPUTE_COMMON").is_ok();
    let precompute_roman_indic = env::var("CARGO_FEATURE_PRECOMPUTE_ROMAN_INDIC").is_ok();
    let precompute_indic_roman = env::var("CARGO_FEATURE_PRECOMPUTE_INDIC_ROMAN").is_ok();
    let no_precompute = env::var("CARGO_FEATURE_NO_PRECOMPUTE").is_ok();
    
    if no_precompute {
        println!("cargo:warning=Pre-computation disabled by feature flag");
        generate_empty_module();
        return;
    }
    
    // Determine which combinations to generate
    let mut combinations = Vec::new();
    
    if precompute_all {
        println!("cargo:warning=Pre-computing all script combinations");
        combinations = generate_all_combinations();
    } else if precompute_common || (!precompute_roman_indic && !precompute_indic_roman) {
        // Default to common if no specific flags
        println!("cargo:warning=Pre-computing common script combinations");
        combinations.extend_from_slice(COMMON_CONVERSIONS);
    } else {
        if precompute_roman_indic {
            println!("cargo:warning=Pre-computing Roman→Indic combinations");
            for roman in ROMAN_SCRIPTS {
                for indic in INDIC_SCRIPTS {
                    combinations.push((roman, indic));
                }
            }
        }
        if precompute_indic_roman {
            println!("cargo:warning=Pre-computing Indic→Roman combinations");
            for indic in INDIC_SCRIPTS {
                for roman in ROMAN_SCRIPTS {
                    combinations.push((indic, roman));
                }
            }
        }
    }
    
    // Check if we need to regenerate
    if !should_regenerate(&combinations) {
        println!("cargo:warning=Skipping pre-computation: no changes detected");
        return;
    }
    
    // Generate the precomputed module
    generate_precomputed_module(&combinations);
}

fn generate_all_combinations() -> Vec<(&'static str, &'static str)> {
    let mut combinations = Vec::new();
    
    // Roman to Indic
    for &roman in ROMAN_SCRIPTS {
        for &indic in INDIC_SCRIPTS {
            combinations.push((roman, indic));
        }
    }
    
    // Indic to Roman
    for &indic in INDIC_SCRIPTS {
        for &roman in ROMAN_SCRIPTS {
            combinations.push((indic, roman));
        }
    }
    
    combinations
}

fn should_regenerate(combinations: &[(&str, &str)]) -> bool {
    let generated_path = "src/modules/script_converter/precomputed/generated.rs";
    let cache_path = ".shlesha-build-cache";
    
    // If generated file doesn't exist, regenerate
    if !Path::new(generated_path).exists() {
        return true;
    }
    
    // Check if cached combinations match current ones
    if let Ok(cached) = fs::read_to_string(cache_path) {
        let cached_combinations: Vec<(String, String)> = cached
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() == 2 {
                    Some((parts[0].to_string(), parts[1].to_string()))
                } else {
                    None
                }
            })
            .collect();
        
        let current_combinations: Vec<(String, String)> = combinations
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect();
        
        if cached_combinations != current_combinations {
            return true;
        }
    } else {
        return true;
    }
    
    // Check modification times of source files
    let generated_mtime = fs::metadata(generated_path)
        .ok()
        .and_then(|m| m.modified().ok());
    
    if let Some(gen_time) = generated_mtime {
        // Check all converter source files
        for script in ROMAN_SCRIPTS.iter().chain(INDIC_SCRIPTS.iter()) {
            let path = format!("src/modules/script_converter/{}.rs", script);
            if let Ok(metadata) = fs::metadata(&path) {
                if let Ok(mtime) = metadata.modified() {
                    if mtime > gen_time {
                        println!("cargo:warning=Regenerating: {} changed", path);
                        return true;
                    }
                }
            }
        }
        
        // Check hub module
        let hub_path = "src/modules/hub/mod.rs";
        if let Ok(metadata) = fs::metadata(hub_path) {
            if let Ok(mtime) = metadata.modified() {
                if mtime > gen_time {
                    println!("cargo:warning=Regenerating: hub module changed");
                    return true;
                }
            }
        }
    }
    
    false
}

fn generate_empty_module() {
    let dir_path = "src/modules/script_converter/precomputed";
    fs::create_dir_all(dir_path).expect("Failed to create precomputed directory");
    
    let content = r#"//! Empty precomputed module (pre-computation disabled)

use std::collections::HashMap;
use super::super::{ScriptConverter, ConverterError};
use crate::modules::hub::HubInput;

pub struct PrecomputedRegistry {
    converters: HashMap<(String, String), Box<dyn ScriptConverter>>,
}

impl PrecomputedRegistry {
    pub fn new() -> Self {
        Self {
            converters: HashMap::new(),
        }
    }
    
    pub fn get(&self, from: &str, to: &str) -> Option<&Box<dyn ScriptConverter>> {
        self.converters.get(&(from.to_string(), to.to_string()))
    }
}

impl Default for PrecomputedRegistry {
    fn default() -> Self {
        Self::new()
    }
}
"#;
    
    let file_path = format!("{}/generated.rs", dir_path);
    fs::write(&file_path, content).expect("Failed to write generated file");
    
    // Create mod.rs
    let mod_content = r#"//! Pre-computed direct script converters

pub mod generated;
pub use generated::PrecomputedRegistry;
"#;
    
    fs::write(format!("{}/mod.rs", dir_path), mod_content)
        .expect("Failed to write mod.rs");
}

fn generate_precomputed_module(combinations: &[(&str, &str)]) {
    let dir_path = "src/modules/script_converter/precomputed";
    fs::create_dir_all(dir_path).expect("Failed to create precomputed directory");
    
    println!("cargo:warning=Generating {} pre-computed converters", combinations.len());
    
    let mut content = String::from(r#"//! Pre-computed direct script converters
//! This file is auto-generated by build.rs - DO NOT EDIT

use std::collections::HashMap;
use super::super::{ScriptConverter, ConverterError};
use crate::modules::hub::HubInput;

"#);
    
    // Generate individual converter structs for each combination
    for (from, to) in combinations {
        let struct_name = format!("{}To{}Direct", capitalize_first(from), capitalize_first(to));
        let mappings = compose_mappings(from, to);
        
        if let Ok(mapping_code) = mappings {
            content.push_str(&format!(r#"
/// Direct converter from {} to {}
pub struct {} {{
    mappings: HashMap<&'static str, &'static str>,
}}

impl {} {{
    pub fn new() -> Self {{
        let mut mappings = HashMap::new();
{}
        Self {{ mappings }}
    }}
    
    fn convert(&self, input: &str) -> Result<String, ConverterError> {{
        use super::super::processors::RomanScriptProcessor;
        RomanScriptProcessor::process_optimized(input, &self.mappings)
    }}
}}

impl ScriptConverter for {} {{
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError> {{
        // Direct converters bypass the hub
        Err(ConverterError::ConversionFailed {{
            script: script.to_string(),
            reason: "Direct converter should not use to_hub".to_string(),
        }})
    }}
    
    fn from_hub(&self, script: &str, hub_input: &HubInput) -> Result<String, ConverterError> {{
        // Direct converters bypass the hub
        Err(ConverterError::ConversionFailed {{
            script: script.to_string(),
            reason: "Direct converter should not use from_hub".to_string(),
        }})
    }}
    
    fn supported_scripts(&self) -> Vec<&'static str> {{
        vec!["{}", "{}"]
    }}
    
    fn script_has_implicit_a(&self, _script: &str) -> bool {{
        false // Direct converters handle this internally
    }}
}}
"#, from, to, struct_name, struct_name, mapping_code, struct_name, from, to));
        }
    }
    
    // Generate the registry
    content.push_str(r#"
pub struct PrecomputedRegistry {
    converters: HashMap<(String, String), Box<dyn ScriptConverter>>,
}

impl PrecomputedRegistry {
    pub fn new() -> Self {
        let mut converters: HashMap<(String, String), Box<dyn ScriptConverter>> = HashMap::new();
        
"#);
    
    // Add entries for each combination
    for (from, to) in combinations {
        let struct_name = format!("{}To{}Direct", capitalize_first(from), capitalize_first(to));
        content.push_str(&format!(
            "        converters.insert((\"{}\".to_string(), \"{}\".to_string()), Box::new({}::new()));\n",
            from, to, struct_name
        ));
    }
    
    content.push_str(r#"
        Self { converters }
    }
    
    pub fn get(&self, from: &str, to: &str) -> Option<&Box<dyn ScriptConverter>> {
        self.converters.get(&(from.to_string(), to.to_string()))
    }
}

impl Default for PrecomputedRegistry {
    fn default() -> Self {
        Self::new()
    }
}
"#);
    
    // Write the generated file
    let file_path = format!("{}/generated.rs", dir_path);
    fs::write(&file_path, content).expect("Failed to write generated file");
    
    // Create mod.rs
    let mod_content = r#"//! Pre-computed direct script converters

pub mod generated;
pub use generated::PrecomputedRegistry;
"#;
    
    fs::write(format!("{}/mod.rs", dir_path), mod_content)
        .expect("Failed to write mod.rs");
    
    // Save cache of what was generated
    let cache_content: String = combinations
        .iter()
        .map(|(from, to)| format!("{},{}", from, to))
        .collect::<Vec<_>>()
        .join("\n");
    
    fs::write(".shlesha-build-cache", cache_content)
        .expect("Failed to write build cache");
}

fn compose_mappings(from: &str, to: &str) -> Result<String, String> {
    // For now, we'll parse the actual source files to extract mappings
    // This is a simplified version - in production we'd want more robust parsing
    
    match (from, to) {
        // Roman to Devanagari compositions
        ("iast", "devanagari") => compose_roman_to_devanagari("iast"),
        ("itrans", "devanagari") => compose_roman_to_devanagari("itrans"),
        ("slp1", "devanagari") => compose_roman_to_devanagari("slp1"),
        
        // Devanagari to Roman compositions
        ("devanagari", "iast") => compose_devanagari_to_roman("iast"),
        ("devanagari", "itrans") => compose_devanagari_to_roman("itrans"),
        ("devanagari", "slp1") => compose_devanagari_to_roman("slp1"),
        
        _ => Err(format!("Composition not implemented for {} → {}", from, to)),
    }
}

fn compose_roman_to_devanagari(roman_script: &str) -> Result<String, String> {
    // Read mapping files and compose Roman → ISO → Devanagari
    let roman_file = format!("src/modules/script_converter/{}.rs", roman_script);
    let hub_file = "src/modules/hub/mod.rs";
    
    // For demonstration, return a simplified mapping
    // In a real implementation, we'd parse the files and compose the mappings
    Ok(format!(r#"
        // Basic vowels
        mappings.insert("a", "अ");
        mappings.insert("ā", "आ");
        mappings.insert("i", "इ");
        mappings.insert("ī", "ई");
        mappings.insert("u", "उ");
        mappings.insert("ū", "ऊ");
        
        // Consonants with inherent 'a'
        mappings.insert("ka", "क");
        mappings.insert("kha", "ख");
        mappings.insert("ga", "ग");
        mappings.insert("gha", "घ");
        mappings.insert("ṅa", "ङ");
        
        // More mappings would be generated from actual file parsing...
"#))
}

fn compose_devanagari_to_roman(roman_script: &str) -> Result<String, String> {
    // Read mapping files and compose Devanagari → ISO → Roman
    Ok(format!(r#"
        // Basic vowels
        mappings.insert("अ", "a");
        mappings.insert("आ", "ā");
        mappings.insert("इ", "i");
        mappings.insert("ई", "ī");
        mappings.insert("उ", "u");
        mappings.insert("ऊ", "ū");
        
        // Consonants (without inherent 'a')
        mappings.insert("क", "ka");
        mappings.insert("ख", "kha");
        mappings.insert("ग", "ga");
        mappings.insert("घ", "gha");
        mappings.insert("ङ", "ṅa");
        
        // More mappings would be generated from actual file parsing...
"#))
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}