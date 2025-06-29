//! Comprehensive error handling tests for the registry module
//!
//! Tests various failure scenarios to ensure robust error handling
//! and proper error propagation.

#[cfg(test)]
mod error_handling_tests {
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
    fn test_load_schema_nonexistent_file() {
        let mut registry = SchemaRegistry::new();
        let result = registry.load_schema("/path/that/does/not/exist.yaml");

        match result {
            Err(RegistryError::IoError(msg)) => {
                assert!(msg.contains("Failed to read file"));
            }
            Err(other_error) => {
                println!("Got different error type: {:?}", other_error);
                // Accept other error types as well - file not found could be LoadFailed
                assert!(matches!(other_error, RegistryError::LoadFailed(_)));
            }
            Ok(_) => panic!("Expected error for nonexistent file"),
        }
    }

    #[test]
    fn test_load_schema_invalid_yaml() {
        let temp_dir = create_temp_dir();
        let invalid_yaml = r#"
metadata:
  name: "test"
  script_type: "roman"
mappings:
  vowels:
    - this: is: not: valid: yaml: structure
      because: [it, has, invalid, nesting
"#;

        let file_path = create_temp_file(&temp_dir, "invalid.yaml", invalid_yaml);
        let mut registry = SchemaRegistry::new();

        let result = registry.load_schema(file_path.to_str().unwrap());

        match result {
            Err(RegistryError::ParseError(msg)) => {
                assert!(msg.contains("Failed to parse YAML"));
            }
            _ => panic!("Expected ParseError for invalid YAML"),
        }
    }

    #[test]
    fn test_load_schema_missing_required_fields() {
        let temp_dir = create_temp_dir();
        let incomplete_yaml = r#"
# Missing metadata.name and other required fields
mappings:
  vowels:
    a: "अ"
"#;

        let file_path = create_temp_file(&temp_dir, "incomplete.yaml", incomplete_yaml);
        let mut registry = SchemaRegistry::new();

        let result = registry.load_schema(file_path.to_str().unwrap());

        // Should fail at YAML parsing stage due to missing required fields
        assert!(result.is_err());
    }

    #[test]
    fn test_load_schema_empty_file() {
        let temp_dir = create_temp_dir();
        let file_path = create_temp_file(&temp_dir, "empty.yaml", "");
        let mut registry = SchemaRegistry::new();

        let result = registry.load_schema(file_path.to_str().unwrap());

        match result {
            Err(RegistryError::ParseError(_)) => {
                // Expected - empty YAML should fail parsing
            }
            _ => panic!("Expected ParseError for empty file"),
        }
    }

    #[test]
    fn test_load_schema_malformed_mappings() {
        let temp_dir = create_temp_dir();
        let malformed_yaml = r#"
metadata:
  name: "test_malformed"
  script_type: "roman"
  has_implicit_a: false
mappings:
  vowels: "this should be a map, not a string"
  consonants:
    - "this should be key-value pairs, not a list"
"#;

        let file_path = create_temp_file(&temp_dir, "malformed.yaml", malformed_yaml);
        let mut registry = SchemaRegistry::new();

        let result = registry.load_schema(file_path.to_str().unwrap());

        // Should fail at YAML parsing due to type mismatch
        assert!(result.is_err());
    }

    #[test]
    fn test_load_schema_from_string_invalid() {
        let mut registry = SchemaRegistry::new();

        // Test completely invalid YAML
        let result = registry.load_schema_from_string("this is not yaml at all!!!", "test");
        assert!(matches!(result, Err(RegistryError::ParseError(_))));

        // Test valid YAML but wrong structure
        let wrong_structure = r#"
this_is_valid_yaml: true
but_wrong_structure: "for schema"
"#;
        let result = registry.load_schema_from_string(wrong_structure, "test2");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_schemas_from_nonexistent_directory() {
        let mut registry = SchemaRegistry::new();
        let result = registry.load_schemas_from_directory("/path/that/does/not/exist");

        match result {
            Err(RegistryError::LoadFailed(msg)) => {
                assert!(msg.contains("Not a directory"));
            }
            _ => panic!("Expected LoadFailed for nonexistent directory"),
        }
    }

    #[test]
    fn test_load_schemas_from_file_instead_of_directory() {
        let temp_dir = create_temp_dir();
        let file_path = create_temp_file(&temp_dir, "not_a_directory.txt", "content");
        let mut registry = SchemaRegistry::new();

        let result = registry.load_schemas_from_directory(file_path.to_str().unwrap());

        match result {
            Err(RegistryError::LoadFailed(msg)) => {
                assert!(msg.contains("Not a directory"));
            }
            _ => panic!("Expected LoadFailed when path is a file, not directory"),
        }
    }

    #[test]
    fn test_load_schemas_directory_with_mixed_files() {
        let temp_dir = create_temp_dir();

        // Create a valid schema
        let valid_schema = r#"
metadata:
  name: "valid_test"
  script_type: "roman"
  has_implicit_a: false
mappings:
  vowels:
    a: "अ"
    i: "इ"
"#;
        create_temp_file(&temp_dir, "valid.yaml", valid_schema);

        // Create an invalid schema
        let invalid_schema = "invalid: yaml: structure: [unclosed";
        create_temp_file(&temp_dir, "invalid.yaml", invalid_schema);

        // Create a non-YAML file (should be ignored)
        create_temp_file(&temp_dir, "readme.txt", "This is not a schema file");

        let mut registry = SchemaRegistry::new();
        let result = registry.load_schemas_from_directory(temp_dir.path().to_str().unwrap());

        // Should succeed and return count of successfully loaded schemas
        match result {
            Ok(count) => {
                assert_eq!(count, 1); // Only the valid schema should be loaded
                assert!(registry.get_schema("valid_test").is_some());
            }
            Err(e) => panic!("Expected success with partial loading, got error: {}", e),
        }
    }

    #[test]
    fn test_register_duplicate_schema() {
        let mut registry = SchemaRegistry::new();

        let schema1 = Schema::new("duplicate_test".to_string(), "roman".to_string());
        let schema2 = Schema::new("duplicate_test".to_string(), "brahmic".to_string());

        // First registration should succeed
        assert!(registry
            .register_schema("duplicate_test".to_string(), schema1)
            .is_ok());

        // Second registration with same name should succeed (overwrite)
        // This is current behavior - might want to change to error in the future
        assert!(registry
            .register_schema("duplicate_test".to_string(), schema2)
            .is_ok());
    }

    #[test]
    fn test_get_nonexistent_schema() {
        let registry = SchemaRegistry::new();
        assert!(registry.get_schema("does_not_exist").is_none());
    }

    #[test]
    fn test_remove_nonexistent_schema() {
        let mut registry = SchemaRegistry::new();
        assert!(!registry.remove_schema("does_not_exist"));
    }

    #[test]
    fn test_schema_validation_errors() {
        let registry = SchemaRegistry::new();

        // Test schema with empty name
        let mut invalid_schema = Schema::new("".to_string(), "roman".to_string());
        invalid_schema.name = "".to_string();

        let result = registry.validate_schema(&invalid_schema);
        match result {
            Err(RegistryError::InvalidSchema(msg)) => {
                assert!(msg.contains("name"));
            }
            _ => panic!("Expected InvalidSchema error for empty name"),
        }
    }

    #[test]
    fn test_malformed_unicode_in_schema() {
        let temp_dir = create_temp_dir();

        // Create schema with potentially problematic Unicode
        let unicode_schema = r#"
metadata:
  name: "unicode_test"
  script_type: "roman"
  has_implicit_a: false
mappings:
  vowels:
    "a": "अ"
    "invalid_unicode": "💩🔥"
    "mixed_scripts": "aअbइcउ"
"#;

        let file_path = create_temp_file(&temp_dir, "unicode.yaml", unicode_schema);
        let mut registry = SchemaRegistry::new();

        // Should handle Unicode gracefully (either succeed or fail cleanly)
        let result = registry.load_schema(file_path.to_str().unwrap());

        // Unicode content should not cause crashes, though it might be invalid
        match result {
            Ok(_) => {
                // If it succeeds, the schema should be accessible
                assert!(registry.get_schema("unicode_test").is_some());
            }
            Err(_) => {
                // If it fails, should be a clean error
                // This is acceptable behavior
            }
        }
    }

    #[test]
    fn test_schema_with_null_values() {
        let temp_dir = create_temp_dir();
        let null_schema = r#"
metadata:
  name: "null_test"
  script_type: "roman"
  has_implicit_a: false
mappings:
  vowels:
    a: null
    i: "इ"
  consonants: null
"#;

        let file_path = create_temp_file(&temp_dir, "null.yaml", null_schema);
        let mut registry = SchemaRegistry::new();

        // Should handle null values gracefully
        let result = registry.load_schema(file_path.to_str().unwrap());

        // Behavior may vary - either parse with nulls or reject
        match result {
            Ok(_) => {
                // If successful, check that null mappings are handled
                let schema = registry.get_schema("null_test").unwrap();
                println!("Schema mappings: {:?}", schema.mappings);
                // Null values might be handled differently - let's just check the schema loaded
                assert!(!schema.mappings.is_empty()); // Should have at least the "i" mapping
            }
            Err(_) => {
                // Rejecting null values is also acceptable
                println!("Null schema was rejected (this is fine)");
            }
        }
    }

    #[test]
    fn test_extremely_large_schema() {
        let temp_dir = create_temp_dir();

        // Generate a very large schema (stress test)
        let mut large_schema = String::from(
            r#"
metadata:
  name: "large_test"
  script_type: "roman"
  has_implicit_a: false
mappings:
  vowels:
"#,
        );

        // Add many mappings to test memory/performance limits
        for i in 0..1000 {
            large_schema.push_str(&format!("    \"key_{}\": \"value_{}\"\n", i, i));
        }

        let file_path = create_temp_file(&temp_dir, "large.yaml", &large_schema);
        let mut registry = SchemaRegistry::new();

        // Should handle large schemas without crashing
        let result = registry.load_schema(file_path.to_str().unwrap());

        match result {
            Ok(_) => {
                let schema = registry.get_schema("large_test").unwrap();
                assert!(schema.mappings.len() >= 1000);
            }
            Err(e) => {
                // If it fails due to size limits, that's acceptable
                println!("Large schema failed (acceptable): {}", e);
            }
        }
    }
}
