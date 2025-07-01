//! TOML mapping file loader

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Structure matching the TOML file format
#[derive(Debug, Deserialize, Serialize)]
pub struct MappingFile {
    pub metadata: MappingFileMetadata,
    pub mappings: Option<MappingCategories>,
    pub to_iso: Option<MappingCategories>,
    pub from_iso: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MappingFileMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_script: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_script: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_type: Option<String>,
    pub has_implicit_a: Option<bool>,
    pub from_has_implicit_a: Option<bool>,
    pub to_has_implicit_a: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MappingCategories {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vowels: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consonants: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vowel_marks: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numerals: Option<HashMap<String, String>>,
}

/// Load a mapping file from disk
pub fn load_mapping_file(path: &Path) -> Result<MappingFile, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let mapping: MappingFile = toml::from_str(&content)?;
    Ok(mapping)
}

/// Flatten categorized mappings into a single HashMap
pub fn flatten_mappings(categories: &MappingCategories) -> HashMap<String, String> {
    let mut result = HashMap::new();

    if let Some(vowels) = &categories.vowels {
        result.extend(vowels.clone());
    }
    if let Some(consonants) = &categories.consonants {
        result.extend(consonants.clone());
    }
    if let Some(vowel_marks) = &categories.vowel_marks {
        result.extend(vowel_marks.clone());
    }
    if let Some(special) = &categories.special {
        result.extend(special.clone());
    }
    if let Some(numerals) = &categories.numerals {
        result.extend(numerals.clone());
    }

    result
}

/// Load all mapping files from a directory
pub fn load_all_mappings(
    dir: &Path,
) -> Result<HashMap<String, MappingFile>, Box<dyn std::error::Error>> {
    let mut mappings = HashMap::new();

    if dir.exists() && dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                let file_name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();

                match load_mapping_file(&path) {
                    Ok(mapping) => {
                        mappings.insert(file_name, mapping);
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to load {}: {}", path.display(), e);
                    }
                }
            }
        }
    }

    Ok(mappings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_mappings() {
        let categories = MappingCategories {
            vowels: Some(HashMap::from([
                ("a".to_string(), "अ".to_string()),
                ("ā".to_string(), "आ".to_string()),
            ])),
            consonants: Some(HashMap::from([("ka".to_string(), "क".to_string())])),
            vowel_marks: None,
            special: None,
            numerals: None,
        };

        let flat = flatten_mappings(&categories);
        assert_eq!(flat.len(), 3);
        assert_eq!(flat.get("a"), Some(&"अ".to_string()));
        assert_eq!(flat.get("ka"), Some(&"क".to_string()));
    }
}
