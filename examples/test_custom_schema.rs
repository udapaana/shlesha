use shlesha::Shlesha;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing custom schema functionality...");

    // Create a new Shlesha instance
    let mut transliterator = Shlesha::new();

    // Test without custom schema first
    println!("\n1. Testing supported scripts before loading custom schema:");
    let scripts = transliterator.list_supported_scripts();
    println!("Built-in scripts: {:?}", scripts);

    // Try to use custom encoding before loading (should fail)
    println!("\n2. Testing custom encoding before loading schema:");
    match transliterator.transliterate("kam", "my_custom_encoding", "devanagari") {
        Ok(result) => println!("Unexpected success: {}", result),
        Err(e) => println!("Expected error: {}", e),
    }

    // Load the custom schema
    println!("\n3. Loading custom schema...");
    transliterator.load_schema_from_file("examples/custom_encoding.yaml")?;
    println!("Custom schema loaded successfully!");

    // Test if the custom script is now supported
    println!("\n4. Testing if custom script is now supported:");
    let supports_custom = transliterator.supports_script("my_custom_encoding");
    println!("Supports 'my_custom_encoding': {}", supports_custom);

    // Test conversion with custom encoding
    println!("\n5. Testing conversion with custom encoding:");
    let test_cases = vec![("k", "क"), ("kam", "कम"), ("gam", "गम"), ("pani", "पनि")];

    for (input, expected) in test_cases {
        match transliterator.transliterate(input, "my_custom_encoding", "devanagari") {
            Ok(result) => {
                println!("  '{}' -> '{}' (expected: '{}')", input, result, expected);
                if result == expected {
                    println!("    ✓ Correct!");
                } else {
                    println!("    ✗ Mismatch!");
                }
            }
            Err(e) => println!("  '{}' -> Error: {}", input, e),
        }
    }

    // Test reverse conversion (custom -> devanagari -> custom)
    println!("\n6. Testing round-trip conversion:");
    match transliterator.transliterate("kam", "my_custom_encoding", "devanagari") {
        Ok(devanagari_result) => {
            println!(
                "  my_custom_encoding -> devanagari: 'kam' -> '{}'",
                devanagari_result
            );

            // Try to convert back (this might not work perfectly yet)
            match transliterator.transliterate(
                &devanagari_result,
                "devanagari",
                "my_custom_encoding",
            ) {
                Ok(custom_result) => {
                    println!(
                        "  devanagari -> my_custom_encoding: '{}' -> '{}'",
                        devanagari_result, custom_result
                    );
                }
                Err(e) => println!("  devanagari -> my_custom_encoding: Error: {}", e),
            }
        }
        Err(e) => println!("  Error in first conversion: {}", e),
    }

    println!("\n7. Testing conversion to other scripts:");
    match transliterator.transliterate("kam", "my_custom_encoding", "iast") {
        Ok(result) => println!("  my_custom_encoding -> iast: 'kam' -> '{}'", result),
        Err(e) => println!("  my_custom_encoding -> iast: Error: {}", e),
    }

    println!("\nCustom schema test completed!");
    Ok(())
}
