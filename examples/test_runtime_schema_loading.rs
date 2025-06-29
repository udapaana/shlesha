use shlesha::modules::registry::{SchemaRegistry, SchemaRegistryTrait};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing runtime schema loading with unified format...");

    let mut registry = SchemaRegistry::new();

    // List built-in schemas
    println!("\nBuilt-in schemas:");
    for schema_name in registry.list_schemas() {
        println!("  - {}", schema_name);
    }

    // Test loading an existing schema file
    let test_schema_path = "schemas/iast.yaml";
    if std::path::Path::new(test_schema_path).exists() {
        println!("\nLoading schema from: {}", test_schema_path);
        match registry.load_schema(test_schema_path) {
            Ok(()) => {
                println!("✅ Successfully loaded IAST schema");

                // Check if it's available
                if let Some(schema) = registry.get_schema("iast") {
                    println!("✅ IAST schema is available in registry");
                    println!("   Name: {}", schema.name);
                    println!("   Script type: {}", schema.script_type);
                    println!("   Target: {}", schema.target);
                    println!("   Mappings count: {}", schema.mappings.len());

                    // Show some sample mappings
                    println!("   Sample mappings:");
                    for (key, value) in schema.mappings.iter().take(5) {
                        println!("     {} → {}", key, value);
                    }
                } else {
                    println!("❌ IAST schema not found in registry");
                }
            }
            Err(e) => {
                println!("❌ Failed to load IAST schema: {}", e);
            }
        }
    } else {
        println!("⚠️  Test schema file not found: {}", test_schema_path);
    }

    // Test loading all schemas from directory
    println!("\nLoading all schemas from schemas/ directory...");
    match registry.load_schemas_from_directory("schemas") {
        Ok(count) => {
            println!("✅ Successfully loaded {} schemas", count);

            println!("\nAll available schemas:");
            for schema_name in registry.list_schemas() {
                if let Some(schema) = registry.get_schema(schema_name) {
                    println!(
                        "  - {} ({}, target: {})",
                        schema_name, schema.script_type, schema.target
                    );
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to load schemas from directory: {}", e);
        }
    }

    Ok(())
}
