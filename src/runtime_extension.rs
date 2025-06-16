use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::element_id::{ElementId, ElementType, ElementRegistry};
use crate::ir_v2::{PropertyKey, PropertyValue, SmallPropertySet};
use crate::schema_parser::{Schema, ElementMapping};

/// User-friendly interface for extending the transliterator at runtime
#[derive(Debug, Clone)]
pub struct RuntimeExtensionManager {
    registry: ElementRegistry,
    variant_definitions: HashMap<String, VariantDefinition>,
    active_variants: Vec<String>,
}

/// Definition of a morphological variant discovered in texts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantDefinition {
    pub name: String,
    pub description: String,
    pub base_element: String,          // e.g., "ka" - what this is a variant of
    pub variant_type: VariantType,
    pub graphemes: VariantGraphemes,   // How it appears in different scripts
    pub properties: HashMap<String, serde_yaml::Value>,
    pub manuscript_sources: Vec<String>,
    pub created_by: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariantType {
    ConsonantVariant {
        aspiration_change: bool,
        voicing_change: bool,
        place_change: Option<String>,
    },
    VowelVariant {
        length_change: bool,
        quality_change: Option<String>,
    },
    LigatureVariant {
        components: Vec<String>,
        binding_type: String,
    },
    AccentVariant {
        base_accent: String,
        modification: String,
    },
    ScribalVariant {
        error_type: String,
        frequency: f32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantGraphemes {
    pub primary_script: String,      // e.g., "Devanagari"
    pub primary_form: String,        // e.g., "क़"
    pub romanizations: HashMap<String, String>, // scheme -> form
    pub other_scripts: HashMap<String, String>, // script -> form
}

/// Simple API for non-technical users
impl RuntimeExtensionManager {
    pub fn new() -> Self {
        Self {
            registry: ElementRegistry::default(),
            variant_definitions: HashMap::new(),
            active_variants: Vec::new(),
        }
    }

    /// Simple method: "I found a new character that looks like X but sounds like Y"
    pub fn add_simple_variant(
        &mut self,
        name: &str,
        description: &str,
        similar_to: &str,           // "ka", "ga", etc.
        devanagari_form: &str,      // "क़"
        iast_form: &str,            // "qa"
        manuscript_source: &str,
    ) -> Result<ElementId, ExtensionError> {
        let variant = VariantDefinition {
            name: name.to_string(),
            description: description.to_string(),
            base_element: similar_to.to_string(),
            variant_type: VariantType::ConsonantVariant {
                aspiration_change: false,
                voicing_change: false,
                place_change: Some("foreign".to_string()),
            },
            graphemes: VariantGraphemes {
                primary_script: "Devanagari".to_string(),
                primary_form: devanagari_form.to_string(),
                romanizations: {
                    let mut map = HashMap::new();
                    map.insert("IAST".to_string(), iast_form.to_string());
                    map
                },
                other_scripts: HashMap::new(),
            },
            properties: HashMap::new(),
            manuscript_sources: vec![manuscript_source.to_string()],
            created_by: "user".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        self.add_variant(variant)
    }

    /// Advanced method: Full variant definition
    pub fn add_variant(&mut self, variant: VariantDefinition) -> Result<ElementId, ExtensionError> {
        // Register the new element
        let element_id = self.registry.register_extension(variant.name.clone());
        
        // Store the definition
        self.variant_definitions.insert(variant.name.clone(), variant);
        
        Ok(element_id)
    }

    /// Load variant from a simple YAML file
    pub fn load_variant_from_yaml(&mut self, yaml_content: &str) -> Result<ElementId, ExtensionError> {
        let variant: VariantDefinition = serde_yaml::from_str(yaml_content)
            .map_err(|e| ExtensionError::ParseError(format!("YAML parse error: {}", e)))?;
        
        self.add_variant(variant)
    }

    /// Generate schema updates to include the new variants
    pub fn generate_schema_updates(&self) -> HashMap<String, SchemaUpdate> {
        let mut updates = HashMap::new();
        
        // For each script/scheme, generate the necessary mappings
        for variant in self.variant_definitions.values() {
            // Update Devanagari schema
            let devanagari_update = SchemaUpdate {
                script_name: variant.graphemes.primary_script.clone(),
                new_mappings: {
                    let mut mappings = HashMap::new();
                    mappings.insert(
                        variant.graphemes.primary_form.clone(),
                        ElementMapping {
                            canonical: variant.name.clone(),
                            element_type: Some("extension".to_string()),
                            properties: variant.properties.clone(),
                        }
                    );
                    mappings
                },
            };
            updates.insert(variant.graphemes.primary_script.clone(), devanagari_update);

            // Update romanization schemes
            for (scheme, form) in &variant.graphemes.romanizations {
                let scheme_update = SchemaUpdate {
                    script_name: scheme.clone(),
                    new_mappings: {
                        let mut mappings = HashMap::new();
                        mappings.insert(
                            form.clone(),
                            ElementMapping {
                                canonical: variant.name.clone(),
                                element_type: Some("extension".to_string()),
                                properties: variant.properties.clone(),
                            }
                        );
                        mappings
                    },
                };
                updates.insert(scheme.clone(), scheme_update);
            }
        }
        
        updates
    }

    /// Apply all active variants to modify schemas
    pub fn apply_to_schema(&self, mut schema: Schema) -> Schema {
        for variant_name in &self.active_variants {
            if let Some(variant) = self.variant_definitions.get(variant_name) {
                // Add new element type definition
                schema.element_types.insert(
                    variant.name.clone(),
                    crate::schema_parser::ElementTypeDefinition {
                        description: variant.description.clone(),
                        properties: HashMap::new(), // Could be expanded
                        inherits_from: Some(variant.base_element.clone()),
                    }
                );

                // Add mappings to appropriate categories
                let category = match variant.variant_type {
                    VariantType::ConsonantVariant { .. } => "consonants",
                    VariantType::VowelVariant { .. } => "vowels",
                    VariantType::AccentVariant { .. } => "accents",
                    _ => "extensions",
                };

                let mappings = schema.mappings.entry(category.to_string()).or_insert_with(HashMap::new);
                
                // Add the primary form
                if schema.name == variant.graphemes.primary_script {
                    mappings.insert(
                        variant.graphemes.primary_form.clone(),
                        ElementMapping {
                            canonical: variant.name.clone(),
                            element_type: Some("extension".to_string()),
                            properties: variant.properties.clone(),
                        }
                    );
                }
                
                // Add romanizations if this is a romanization scheme
                if let Some(form) = variant.graphemes.romanizations.get(&schema.name) {
                    mappings.insert(
                        form.clone(),
                        ElementMapping {
                            canonical: variant.name.clone(),
                            element_type: Some("extension".to_string()),
                            properties: variant.properties.clone(),
                        }
                    );
                }
            }
        }
        
        schema
    }

    /// Activate a variant for use
    pub fn activate_variant(&mut self, variant_name: &str) -> Result<(), ExtensionError> {
        if !self.variant_definitions.contains_key(variant_name) {
            return Err(ExtensionError::VariantNotFound(variant_name.to_string()));
        }
        
        if !self.active_variants.contains(&variant_name.to_string()) {
            self.active_variants.push(variant_name.to_string());
        }
        
        Ok(())
    }

    /// Save variant definition to file for sharing
    pub fn save_variant_to_file(&self, variant_name: &str, path: &str) -> Result<(), ExtensionError> {
        let variant = self.variant_definitions.get(variant_name)
            .ok_or_else(|| ExtensionError::VariantNotFound(variant_name.to_string()))?;
        
        let yaml = serde_yaml::to_string(variant)
            .map_err(|e| ExtensionError::SerializationError(format!("YAML error: {}", e)))?;
        
        std::fs::write(path, yaml)
            .map_err(|e| ExtensionError::IoError(format!("File write error: {}", e)))?;
        
        Ok(())
    }

    /// List all available variants
    pub fn list_variants(&self) -> Vec<VariantSummary> {
        self.variant_definitions.values()
            .map(|v| VariantSummary {
                name: v.name.clone(),
                description: v.description.clone(),
                base_element: v.base_element.clone(),
                active: self.active_variants.contains(&v.name),
                manuscript_sources: v.manuscript_sources.clone(),
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct VariantSummary {
    pub name: String,
    pub description: String,
    pub base_element: String,
    pub active: bool,
    pub manuscript_sources: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SchemaUpdate {
    pub script_name: String,
    pub new_mappings: HashMap<String, ElementMapping>,
}

#[derive(Debug, thiserror::Error)]
pub enum ExtensionError {
    #[error("Variant not found: {0}")]
    VariantNotFound(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("Invalid variant definition: {0}")]
    InvalidDefinition(String),
}

/// User-friendly CLI/GUI interface
impl RuntimeExtensionManager {
    /// Interactive method for non-technical users
    pub fn interactive_add_variant(&mut self) -> Result<ElementId, ExtensionError> {
        println!("=== Add New Character Variant ===");
        println!("Help: You found a character that looks different from the standard forms.");
        println!("We'll help you add it to the transliterator.\n");

        print!("What would you like to call this variant? (e.g., 'qa_variant'): ");
        let name = self.read_input()?;

        print!("Describe what makes this variant special: ");
        let description = self.read_input()?;

        print!("What standard character is this most similar to? (e.g., 'ka', 'ga'): ");
        let base = self.read_input()?;

        print!("How does this character look in Devanagari? (e.g., 'क़'): ");
        let devanagari = self.read_input()?;

        print!("How should this be romanized in IAST? (e.g., 'qa'): ");
        let iast = self.read_input()?;

        print!("Which manuscript or text did you find this in?: ");
        let source = self.read_input()?;

        self.add_simple_variant(&name, &description, &base, &devanagari, &iast, &source)
    }

    fn read_input(&self) -> Result<String, ExtensionError> {
        use std::io::{self, Write};
        
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .map_err(|e| ExtensionError::IoError(format!("Input error: {}", e)))?;
        
        Ok(input.trim().to_string())
    }
}

/// Example YAML file for defining variants
pub const EXAMPLE_VARIANT_YAML: &str = r#"
name: "qa_variant"
description: "Arabic/Persian qa sound in Sanskrit texts"
base_element: "ka"
variant_type: !ConsonantVariant
  aspiration_change: false
  voicing_change: false
  place_change: "uvular"
graphemes:
  primary_script: "Devanagari"
  primary_form: "क़"
  romanizations:
    IAST: "qa"
    Harvard-Kyoto: "qa"
    SLP1: "qa"
  other_scripts:
    Telugu: "క఼"
    Bengali: "ক়"
properties:
  foreign_origin: true
  arabic_persian: true
  nukta_based: true
manuscript_sources:
  - "MS Bharat Kala Bhavan 1234"
  - "Delhi Arabic Mahabharata"
created_by: "Dr. Smith"
created_at: "2024-01-15T10:30:00Z"
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_variant_addition() {
        let mut manager = RuntimeExtensionManager::new();
        
        let element_id = manager.add_simple_variant(
            "qa_variant",
            "Arabic qa sound",
            "ka",
            "क़",
            "qa",
            "MS Test 123"
        ).unwrap();
        
        assert_eq!(manager.variant_definitions.len(), 1);
        
        let variants = manager.list_variants();
        assert_eq!(variants.len(), 1);
        assert_eq!(variants[0].name, "qa_variant");
        assert_eq!(variants[0].base_element, "ka");
        assert!(!variants[0].active);
    }

    #[test]
    fn test_variant_activation() {
        let mut manager = RuntimeExtensionManager::new();
        
        manager.add_simple_variant(
            "qa_variant",
            "Arabic qa sound",
            "ka",
            "क़",
            "qa",
            "MS Test 123"
        ).unwrap();

        manager.activate_variant("qa_variant").unwrap();
        
        let variants = manager.list_variants();
        assert!(variants[0].active);
    }

    #[test]
    fn test_schema_update_generation() {
        let mut manager = RuntimeExtensionManager::new();
        
        manager.add_simple_variant(
            "qa_variant",
            "Arabic qa sound",
            "ka",
            "क़",
            "qa",
            "MS Test 123"
        ).unwrap();

        let updates = manager.generate_schema_updates();
        
        assert!(updates.contains_key("Devanagari"));
        assert!(updates.contains_key("IAST"));
        
        let devanagari_update = &updates["Devanagari"];
        assert!(devanagari_update.new_mappings.contains_key("क़"));
    }

    #[test]
    fn test_yaml_round_trip() {
        let mut manager = RuntimeExtensionManager::new();
        
        // Load from YAML
        manager.load_variant_from_yaml(EXAMPLE_VARIANT_YAML).unwrap();
        
        // Check it was loaded correctly
        let variants = manager.list_variants();
        assert_eq!(variants.len(), 1);
        assert_eq!(variants[0].name, "qa_variant");
        
        // Save to file and reload
        manager.save_variant_to_file("qa_variant", "/tmp/test_variant.yaml").unwrap();
        
        let mut manager2 = RuntimeExtensionManager::new();
        let yaml_content = std::fs::read_to_string("/tmp/test_variant.yaml").unwrap();
        manager2.load_variant_from_yaml(&yaml_content).unwrap();
        
        let variants2 = manager2.list_variants();
        assert_eq!(variants2.len(), 1);
        assert_eq!(variants2[0].name, "qa_variant");
    }
}