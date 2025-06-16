use shlesha::*;
use std::collections::HashMap;

#[cfg(test)]
mod ir_tests {
    use super::*;

    #[test]
    fn test_element_creation_with_properties() {
        let element = Element::new("consonant", "क", "ka")
            .with_property("has_inherent_vowel", PropertyValue::Bool(true))
            .with_property("varga", PropertyValue::String("ka-varga".to_string()))
            .with_property("position", PropertyValue::Number(1.0));

        assert_eq!(element.element_type.0, "consonant");
        assert_eq!(element.grapheme, "क");
        assert_eq!(element.canonical, "ka");
        assert_eq!(element.get_bool("has_inherent_vowel"), Some(true));
        assert_eq!(element.get_string("varga"), Some("ka-varga"));
    }

    #[test]
    fn test_custom_element_types() {
        let element = Element::new("vedic_accent_marker", "॑", "udatta");
        assert_eq!(element.element_type.0, "vedic_accent_marker");
    }

    #[test]
    fn test_ir_with_extensions() {
        let mut ir = AbugidaIR::new("Devanagari".to_string());
        ir.push(Element::new("consonant", "क", "ka"));
        
        let mut extension = Extension {
            name: "test_extension".to_string(),
            priority: 1,
            mappings: HashMap::new(),
        };
        
        extension.mappings.insert("क".to_string(), ExtensionMapping {
            from: "क".to_string(),
            to: "क़".to_string(),
            element_type: Some(ElementType("modified_consonant".to_string())),
            properties: HashMap::new(),
        });
        
        ir.add_extension(extension);
        ir.apply_extensions();
        
        assert_eq!(ir.elements[0].grapheme, "क़");
        assert_eq!(ir.elements[0].element_type.0, "modified_consonant");
    }

    #[test]
    fn test_metadata_with_custom_properties() {
        let mut metadata = Metadata::default();
        metadata.custom_properties.insert(
            "source_document".to_string(),
            PropertyValue::String("manuscript_123".to_string())
        );
        metadata.warnings.push("Non-standard character found".to_string());
        
        assert_eq!(metadata.warnings.len(), 1);
        assert!(matches!(
            metadata.custom_properties.get("source_document"),
            Some(PropertyValue::String(s)) if s == "manuscript_123"
        ));
    }
}

#[cfg(test)]
mod schema_tests {
    use super::*;

    #[test]
    fn test_extensible_schema_parsing() {
        let yaml = r#"
name: "TestScript"
type: abugida

element_types:
  custom_modifier:
    description: "A custom modifier for testing"
    properties:
      intensity:
        type: number
        default: 1.0
        description: "Intensity of the modification"

mappings:
  consonants:
    "त":
      canonical: "ta"
      type: consonant
      custom_prop: "test_value"
  custom_modifiers:
    "◌":
      canonical: "mod"
      type: custom_modifier
      intensity: 2.5

extensions:
  test_extension:
    description: "Test extension"
    priority: 10
    mappings:
      "त":
        to: "त्"
        properties:
          extended: true
"#;
        
        let schema = SchemaParser::parse_str(yaml).unwrap();
        assert_eq!(schema.name, "TestScript");
        assert_eq!(schema.element_types.len(), 1);
        assert!(schema.element_types.contains_key("custom_modifier"));
        assert_eq!(schema.extensions.len(), 1);
    }

    #[test]
    fn test_extension_file_parsing() {
        let yaml = r#"
name: "academic_notation"
description: "Academic notation extensions"
applies_to: ["Devanagari", "IAST", "Harvard-Kyoto"]

extensions:
  manuscript_marks:
    description: "Special marks found in manuscripts"
    priority: 5
    mappings:
      "॰":
        to: "..."
        properties:
          meaning: "text_omission"
      "꣼":
        to: "[?]"
        properties:
          meaning: "unclear_reading"
"#;
        
        let ext_file: ExtensionFile = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(ext_file.name, "academic_notation");
        assert_eq!(ext_file.applies_to.len(), 3);
        assert!(ext_file.extensions.contains_key("manuscript_marks"));
    }

    #[test]
    fn test_schema_registry_with_extensions() {
        let mut registry = SchemaRegistry::new();
        
        let schema = Schema {
            name: "TestScript".to_string(),
            script_type: ScriptType::Alphabet,
            element_types: HashMap::new(),
            mappings: HashMap::new(),
            extensions: HashMap::new(),
            metadata: None,
        };
        
        registry.register(schema);
        
        let ext_file = ExtensionFile {
            name: "test_extensions".to_string(),
            description: "Test".to_string(),
            applies_to: vec!["TestScript".to_string()],
            extensions: {
                let mut exts = HashMap::new();
                exts.insert("ext1".to_string(), ExtensionDefinition {
                    description: "Extension 1".to_string(),
                    priority: 1,
                    mappings: HashMap::new(),
                    conditions: HashMap::new(),
                });
                exts
            },
        };
        
        registry.register_extension_file(ext_file);
        
        let schema_with_ext = registry.get_with_extensions("TestScript", &["ext1"]).unwrap();
        assert!(schema_with_ext.extensions.contains_key("ext1"));
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn test_longest_match_parsing() {
        let schema_yaml = r#"
name: "TestAbugida"
type: abugida

mappings:
  consonants:
    "क": { canonical: "ka" }
    "क्ष": { canonical: "kṣa" }  # Should match this, not क + ष
    "ष": { canonical: "ṣa" }
"#;
        
        let schema = SchemaParser::parse_str(schema_yaml).unwrap();
        let parser = ParserBuilder::new()
            .with_schema(schema)
            .build();
        
        let result = parser.parse("क्ष", "TestAbugida").unwrap();
        
        match result {
            IR::Abugida(ir) => {
                assert_eq!(ir.elements.len(), 1);
                assert_eq!(ir.elements[0].canonical, "kṣa");
            }
            _ => panic!("Expected Abugida IR"),
        }
    }

    #[test]
    fn test_unknown_character_handling() {
        let schema_yaml = r#"
name: "Limited"
type: alphabet

mappings:
  consonants:
    "k": { canonical: "k" }
"#;
        
        let schema = SchemaParser::parse_str(schema_yaml).unwrap();
        let parser = ParserBuilder::new()
            .with_schema(schema)
            .build();
        
        let result = parser.parse("k@z", "Limited").unwrap();
        
        match result {
            IR::Alphabet(ir) => {
                assert_eq!(ir.elements.len(), 3);
                assert_eq!(ir.elements[0].element_type.0, ElementType::CONSONANT);
                assert_eq!(ir.elements[1].element_type.0, ElementType::UNKNOWN);
                assert_eq!(ir.elements[2].element_type.0, ElementType::UNKNOWN);
            }
            _ => panic!("Expected Alphabet IR"),
        }
    }
}

#[cfg(test)]
mod transformer_tests {
    use super::*;

    #[test]
    fn test_property_preservation_during_transform() {
        let transformer = Transformer::new();
        
        let mut abugida = AbugidaIR::new("Devanagari".to_string());
        let consonant = Element::new(ElementType::CONSONANT, "क", "ka")
            .with_property("varga", PropertyValue::String("ka-varga".to_string()))
            .with_property("manuscript_note", PropertyValue::String("damaged".to_string()));
        abugida.push(consonant);
        
        let result = transformer.transform(IR::Abugida(abugida), "alphabet").unwrap();
        
        match result {
            IR::Alphabet(alphabet) => {
                // Properties should be preserved through transformation
                let first_element = &alphabet.elements[0];
                assert_eq!(first_element.get_string("varga"), Some("ka-varga"));
                assert_eq!(first_element.get_string("manuscript_note"), Some("damaged"));
            }
            _ => panic!("Expected Alphabet IR"),
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_runtime_extension_application() {
        let base_schema = r#"
name: "ExtensibleScript"
type: alphabet

mappings:
  consonants:
    "k": { canonical: "k" }
    "t": { canonical: "t" }
  vowels:
    "a": { canonical: "a" }
"#;

        let schema = SchemaParser::parse_str(base_schema).unwrap();
        let mut transliterator = TransliteratorBuilder::new()
            .with_schema(schema).unwrap()
            .build();

        // Add a runtime extension
        let extension_schema = r#"
name: "ExtensibleScript"
type: alphabet

mappings:
  consonants:
    "k": { canonical: "k" }
    "t": { canonical: "t" }
  vowels:
    "a": { canonical: "a" }

extensions:
  phonetic_marks:
    description: "IPA-style phonetic marks"
    priority: 1
    mappings:
      "k":
        to: "kʰ"
        properties:
          aspirated: true
"#;

        let schema_with_ext = SchemaParser::parse_str(extension_schema).unwrap();
        transliterator.load_schema(schema_with_ext).unwrap();
        transliterator.add_extension("phonetic_marks").unwrap();
        
        // Extension should not apply without activation
        let result1 = transliterator.transliterate("ka", "ExtensibleScript", "ExtensibleScript").unwrap();
        assert_eq!(result1, "ka");
        
        // With extension active, 'k' should become 'kʰ'
        let result2 = transliterator.transliterate_with_extensions(
            "ka", 
            "ExtensibleScript", 
            "ExtensibleScript", 
            &["phonetic_marks"]
        ).unwrap();
        assert_eq!(result2, "kʰa");
    }
}