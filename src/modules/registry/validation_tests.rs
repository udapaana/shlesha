//! Schema registry validation and edge case tests
//!
//! Tests the schema registry's validation logic and edge cases
//! beyond basic error handling.

#[cfg(test)]
mod validation_tests {
    use super::super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    /// Create a temporary directory for testing
    fn create_temp_dir() -> TempDir {
        tempfile::tempdir().expect("Failed to create temp directory")
    }

    /// Create a temporary file with given content
    fn create_temp_file(dir: &TempDir, name: &str, content: &str) -> std::path::PathBuf {
        let file_path = dir.path().join(name);
        let mut file = fs::File::create(&file_path).expect("Failed to create temp file");
        file.write_all(content.as_bytes())
            .expect("Failed to write to temp file");
        file_path
    }

    #[test]
    fn test_schema_validation_empty_mappings() {
        let registry = SchemaRegistry::new();

        let mut schema = Schema::new("test_empty".to_string(), "roman".to_string());
        schema.mappings.clear(); // Remove all mappings

        let result = registry.validate_schema(&schema);

        // Current validation doesn't check mappings, so this should pass
        assert!(
            result.is_ok(),
            "Empty mappings should be allowed by current validation"
        );
    }

    #[test]
    fn test_schema_validation_empty_name() {
        let registry = SchemaRegistry::new();

        let mut schema = Schema::new("".to_string(), "roman".to_string());
        schema.name = "".to_string();

        let result = registry.validate_schema(&schema);

        // Empty name should be invalid
        match result {
            Err(RegistryError::InvalidSchema(msg)) => {
                assert!(msg.contains("name") || msg.contains("empty"));
            }
            _ => panic!("Expected InvalidSchema error for empty name"),
        }
    }

    #[test]
    fn test_schema_validation_invalid_script_type() {
        let registry = SchemaRegistry::new();

        let mut schema = Schema::new("test_invalid_type".to_string(), "invalid_type".to_string());
        schema.script_type = "invalid_type".to_string(); // Set the actual field that's validated
        schema.metadata.script_type = "invalid_type".to_string();

        let result = registry.validate_schema(&schema);

        // Invalid script type should trigger validation error
        match result {
            Err(RegistryError::InvalidSchema(msg)) => {
                assert!(msg.contains("Invalid script type"));
            }
            _ => panic!("Expected InvalidSchema error for invalid script type"),
        }
    }

    #[test]
    fn test_schema_with_conflicting_mappings() {
        let temp_dir = create_temp_dir();
        let conflicting_schema = r#"
metadata:
  name: "conflicting_test"
  script_type: "roman"
  has_implicit_a: false
mappings:
  vowels:
    a: "अ"
    a: "आ"  # Duplicate key - should cause YAML parse error or overwrite
  consonants:
    k: "क"
    k: "ख"  # Another duplicate
"#;

        let file_path = create_temp_file(&temp_dir, "conflicting.yaml", conflicting_schema);
        let mut registry = SchemaRegistry::new();

        let result = registry.load_schema(file_path.to_str().unwrap());

        // Should either parse successfully (with overwrites) or fail cleanly
        match result {
            Ok(_) => {
                // If it succeeds, check that one of the values was used
                let schema = registry.get_schema("conflicting_test").unwrap();
                assert!(schema.mappings.contains_key("a"));
                assert!(schema.mappings.contains_key("k"));
            }
            Err(_) => {
                // Failing on conflicting mappings is also acceptable
            }
        }
    }

    #[test]
    fn test_schema_with_circular_references() {
        let temp_dir = create_temp_dir();

        // Create schema that might have circular mapping references
        let circular_schema = r#"
metadata:
  name: "circular_test"
  script_type: "roman"
  has_implicit_a: false
mappings:
  vowels:
    a: "b"
    b: "c"
    c: "a"  # Circular reference in mappings
  consonants:
    k: "g"
    g: "k"  # Another circular reference
"#;

        let file_path = create_temp_file(&temp_dir, "circular.yaml", circular_schema);
        let mut registry = SchemaRegistry::new();

        let result = registry.load_schema(file_path.to_str().unwrap());

        // Should load successfully - circular mappings aren't necessarily invalid
        assert!(result.is_ok(), "Circular mappings should be allowed");

        let schema = registry.get_schema("circular_test").unwrap();
        assert_eq!(schema.mappings.get("a"), Some(&"b".to_string()));
        assert_eq!(schema.mappings.get("k"), Some(&"g".to_string()));
    }

    #[test]
    fn test_schema_with_unicode_edge_cases() {
        let temp_dir = create_temp_dir();

        let unicode_schema = r#"
metadata:
  name: "unicode_test"
  script_type: "roman"
  has_implicit_a: false
mappings:
  vowels:
    "a": "अ"
    "ā": "आ"
    "ä": "Invalid"
  consonants:
    "k": "क"
    "क": "k"  # Reverse mapping with Devanagari key
  special:
    "": "empty_key_test"  # Empty key
    " ": "space_key"      # Space key
    "\t": "tab_key"       # Tab key
    "\n": "newline_key"   # Newline key
"#;

        let file_path = create_temp_file(&temp_dir, "unicode.yaml", unicode_schema);
        let mut registry = SchemaRegistry::new();

        let result = registry.load_schema(file_path.to_str().unwrap());

        // Should handle Unicode edge cases gracefully
        match result {
            Ok(_) => {
                let schema = registry.get_schema("unicode_test").unwrap();

                // Check that valid Unicode mappings work
                assert!(schema.mappings.contains_key("a"));
                assert!(schema.mappings.contains_key("ā"));
                assert!(schema.mappings.contains_key("k"));

                // Devanagari keys should be preserved
                assert!(schema.mappings.contains_key("क"));
            }
            Err(_) => {
                // Rejecting problematic Unicode is also acceptable
            }
        }
    }

    #[test]
    fn test_schema_with_very_long_keys_and_values() {
        let temp_dir = create_temp_dir();

        // Generate very long keys and values
        let long_key = "a".repeat(1000);
        let long_value = "अ".repeat(500);

        let long_schema = format!(
            r#"
metadata:
  name: "long_test"
  script_type: "roman"
  has_implicit_a: false
mappings:
  vowels:
    "{}": "{}"
    "normal": "अ"
"#,
            long_key, long_value
        );

        let file_path = create_temp_file(&temp_dir, "long.yaml", &long_schema);
        let mut registry = SchemaRegistry::new();

        let result = registry.load_schema(file_path.to_str().unwrap());

        // Should handle long strings without crashing
        match result {
            Ok(_) => {
                let schema = registry.get_schema("long_test").unwrap();
                assert!(schema.mappings.contains_key(&long_key));
                assert_eq!(schema.mappings.get(&long_key), Some(&long_value));
            }
            Err(_) => {
                // Rejecting overly long mappings is acceptable
            }
        }
    }

    #[test]
    fn test_schema_with_numeric_keys() {
        let temp_dir = create_temp_dir();

        let numeric_schema = r#"
metadata:
  name: "numeric_test"
  script_type: "roman"
  has_implicit_a: false
mappings:
  digits:
    "0": "०"
    "1": "१"
    "2": "२"
  vowels:
    "123": "invalid_vowel"  # Numeric key in wrong section
"#;

        let file_path = create_temp_file(&temp_dir, "numeric.yaml", numeric_schema);
        let mut registry = SchemaRegistry::new();

        let result = registry.load_schema(file_path.to_str().unwrap());

        // Should handle numeric keys appropriately
        assert!(result.is_ok(), "Numeric keys should be handled");

        let schema = registry.get_schema("numeric_test").unwrap();
        assert!(schema.mappings.contains_key("0"));
        assert!(schema.mappings.contains_key("1"));
        assert!(schema.mappings.contains_key("123"));
    }

    #[test]
    fn test_multiple_schema_registration_order() {
        let mut registry = SchemaRegistry::new();

        // Register multiple schemas and test that order is preserved
        let schemas = vec![
            ("first", "roman"),
            ("second", "brahmic"),
            ("third", "roman"),
        ];

        for (name, script_type) in &schemas {
            let schema = Schema::new(name.to_string(), script_type.to_string());
            registry.register_schema(name.to_string(), schema).unwrap();
        }

        let schema_list = registry.list_schemas_owned();

        // Should contain all registered schemas
        assert!(schema_list.contains(&"first".to_string()));
        assert!(schema_list.contains(&"second".to_string()));
        assert!(schema_list.contains(&"third".to_string()));

        // List should be sorted (according to implementation)
        let mut expected = schema_list.clone();
        expected.sort();
        assert_eq!(schema_list, expected);
    }

    #[test]
    fn test_schema_overwrite_behavior() {
        let mut registry = SchemaRegistry::new();

        // Register a schema
        let mut original_schema = Schema::new("overwrite_test".to_string(), "roman".to_string());
        original_schema
            .mappings
            .insert("a".to_string(), "original".to_string());
        registry
            .register_schema("overwrite_test".to_string(), original_schema)
            .unwrap();

        // Verify original is there
        let retrieved = registry.get_schema("overwrite_test").unwrap();
        assert_eq!(retrieved.mappings.get("a"), Some(&"original".to_string()));

        // Overwrite with new schema
        let mut new_schema = Schema::new("overwrite_test".to_string(), "brahmic".to_string());
        new_schema
            .mappings
            .insert("a".to_string(), "new".to_string());
        registry
            .register_schema("overwrite_test".to_string(), new_schema)
            .unwrap();

        // Verify overwrite worked
        let retrieved = registry.get_schema("overwrite_test").unwrap();
        assert_eq!(retrieved.mappings.get("a"), Some(&"new".to_string()));
        assert_eq!(retrieved.script_type, "brahmic");
    }

    #[test]
    fn test_schema_cache_consistency() {
        let temp_dir = create_temp_dir();
        let test_schema = r#"
metadata:
  name: "cache_test"
  script_type: "roman"
  has_implicit_a: false
mappings:
  vowels:
    a: "अ"
"#;

        let file_path = create_temp_file(&temp_dir, "cache.yaml", test_schema);
        let mut registry = SchemaRegistry::new();

        // Load schema first time
        registry.load_schema(file_path.to_str().unwrap()).unwrap();
        let first_name = registry.get_schema("cache_test").unwrap().name.clone();
        let first_script_type = registry
            .get_schema("cache_test")
            .unwrap()
            .script_type
            .clone();
        let first_mappings = registry.get_schema("cache_test").unwrap().mappings.clone();

        // Load same schema again
        registry.load_schema(file_path.to_str().unwrap()).unwrap();
        let second_retrieval = registry.get_schema("cache_test").unwrap();

        // Should be consistent
        assert_eq!(first_name, second_retrieval.name);
        assert_eq!(first_script_type, second_retrieval.script_type);
        assert_eq!(first_mappings, second_retrieval.mappings);
    }

    #[test]
    fn test_clear_registry_functionality() {
        let mut registry = SchemaRegistry::new();

        // Add some schemas
        let schema1 = Schema::new("clear_test1".to_string(), "roman".to_string());
        let schema2 = Schema::new("clear_test2".to_string(), "brahmic".to_string());

        registry
            .register_schema("clear_test1".to_string(), schema1)
            .unwrap();
        registry
            .register_schema("clear_test2".to_string(), schema2)
            .unwrap();

        // Verify they're there
        assert!(registry.get_schema("clear_test1").is_some());
        assert!(registry.get_schema("clear_test2").is_some());
        assert_eq!(registry.list_schemas().len(), 4); // 2 built-ins + 2 custom

        // Clear registry
        registry.clear();

        // Should be empty now
        assert!(registry.get_schema("clear_test1").is_none());
        assert!(registry.get_schema("clear_test2").is_none());
        assert_eq!(registry.list_schemas().len(), 0);
    }

    #[test]
    fn test_schema_metadata_preservation() {
        let temp_dir = create_temp_dir();
        let metadata_schema = r#"
metadata:
  name: "metadata_test"
  script_type: "roman"
  has_implicit_a: false
  description: "Test schema with full metadata"
  aliases: ["alias1", "alias2", "alias3"]
mappings:
  vowels:
    a: "अ"
"#;

        let file_path = create_temp_file(&temp_dir, "metadata.yaml", metadata_schema);
        let mut registry = SchemaRegistry::new();

        registry.load_schema(file_path.to_str().unwrap()).unwrap();
        let schema = registry.get_schema("metadata_test").unwrap();

        // Check that all metadata is preserved
        assert_eq!(schema.metadata.name, "metadata_test");
        assert_eq!(schema.metadata.script_type, "roman");
        assert_eq!(schema.metadata.has_implicit_a, false);
        assert_eq!(
            schema.metadata.description,
            Some("Test schema with full metadata".to_string())
        );

        if let Some(aliases) = &schema.metadata.aliases {
            assert_eq!(aliases.len(), 3);
            assert!(aliases.contains(&"alias1".to_string()));
            assert!(aliases.contains(&"alias2".to_string()));
            assert!(aliases.contains(&"alias3".to_string()));
        } else {
            panic!("Aliases should be preserved");
        }
    }
}
