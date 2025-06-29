use shlesha::Shlesha;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing Runtime Schema Loading API");
    println!("====================================");

    let mut transliterator = Shlesha::new();

    // Test 1: Basic runtime schema loading from string
    println!("\n✅ Test 1: Load schema from YAML string");
    let custom_schema = r#"
metadata:
  name: "test_script"
  script_type: "roman" 
  has_implicit_a: false
  description: "Test script for API demonstration"

target: "iso15919"

mappings:
  vowels:
    "a": "a"
    "e": "e"
    "i": "i"
    "o": "o"
    "u": "u"
  consonants:
    "k": "k"
    "g": "g"
    "t": "t"
    "d": "d"
    "p": "p"
    "b": "b"
    "r": "r"
    "l": "l"
    "s": "s"
    "h": "h"
"#;

    transliterator.load_schema_from_string(custom_schema, "test_script")?;
    println!("  ✓ Schema loaded successfully");

    // Test 2: Check that the schema is available
    println!("\n✅ Test 2: Verify schema availability");
    assert!(transliterator.supports_script("test_script"));
    println!("  ✓ Custom script is now supported");

    // Test 3: Get schema information
    println!("\n✅ Test 3: Get schema information");
    if let Some(info) = transliterator.get_schema_info("test_script") {
        println!("  ✓ Schema info retrieved:");
        println!("    - Name: {}", info.name);
        println!("    - Description: {}", info.description);
        println!("    - Type: {}", info.script_type);
        println!("    - Runtime loaded: {}", info.is_runtime_loaded);
        println!("    - Mapping count: {}", info.mapping_count);
    } else {
        return Err("Schema info not found".into());
    }

    // Test 4: Use the runtime schema for transliteration
    println!("\n✅ Test 4: Test transliteration with runtime schema");
    let result = transliterator.transliterate("hello", "test_script", "devanagari")?;
    println!("  ✓ 'hello' (test_script) → '{}' (devanagari)", result);

    // Test 5: List all supported scripts (should include our custom one)
    println!("\n✅ Test 5: List all supported scripts");
    let scripts = transliterator.list_supported_scripts();
    let has_custom = scripts.iter().any(|s| s == "test_script");
    assert!(has_custom);
    println!("  ✓ Custom script found in supported scripts list");
    println!("  ✓ Total scripts supported: {}", scripts.len());

    // Test 6: Load another schema from string with different content
    println!("\n✅ Test 6: Load second schema");
    let second_schema = r#"
metadata:
  name: "simple_roman"
  script_type: "roman"
  has_implicit_a: false
  description: "Simplified Roman script"

target: "iso15919"

mappings:
  vowels:
    "a": "a"
    "i": "i"
    "u": "u"
  consonants:
    "m": "m"
    "n": "n"
    "t": "t"
    "r": "r"
"#;

    transliterator.load_schema_from_string(second_schema, "simple_roman")?;
    println!("  ✓ Second schema loaded successfully");

    // Test the second schema
    let result2 = transliterator.transliterate("manta", "simple_roman", "devanagari")?;
    println!("  ✓ 'manta' (simple_roman) → '{}' (devanagari)", result2);

    // Test 7: Schema removal
    println!("\n✅ Test 7: Schema removal");
    let removed = transliterator.remove_schema("simple_roman");
    assert!(removed);
    println!("  ✓ 'simple_roman' schema removed");

    // Verify it's no longer supported
    assert!(!transliterator.supports_script("simple_roman"));
    println!("  ✓ 'simple_roman' no longer supported");

    // Test 8: Clear all runtime schemas
    println!("\n✅ Test 8: Clear all runtime schemas");
    let initial_count = transliterator.list_supported_scripts().len();
    transliterator.clear_runtime_schemas();
    let final_count = transliterator.list_supported_scripts().len();

    // Should still have built-in scripts but no runtime ones
    assert!(final_count <= initial_count);
    assert!(!transliterator.supports_script("test_script"));
    println!("  ✓ All runtime schemas cleared");
    println!("  ✓ Script count: {} → {}", initial_count, final_count);

    // Test 9: Error handling for invalid schema
    println!("\n✅ Test 9: Error handling for invalid schema");
    let invalid_schema = "invalid: yaml: content";
    match transliterator.load_schema_from_string(invalid_schema, "invalid") {
        Ok(_) => return Err("Should have failed to load invalid schema".into()),
        Err(_) => println!("  ✓ Invalid schema correctly rejected"),
    }

    println!("\n🎉 All runtime schema API tests passed!");
    println!("   Runtime schema loading is working correctly across:");
    println!("   - Schema loading from strings");
    println!("   - Schema information retrieval");
    println!("   - Transliteration with runtime schemas");
    println!("   - Schema management (add/remove/clear)");
    println!("   - Error handling");

    Ok(())
}
