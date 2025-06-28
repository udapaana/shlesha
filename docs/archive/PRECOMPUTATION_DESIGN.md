# Pre-computation Design Document

## Overview

Pre-computation optimizes the hub-and-spoke architecture by pre-computing the first conversion step at compile time, storing intermediate results to reduce runtime lookups.

## What Gets Pre-computed

### 1. Roman → Devanagari (for Roman → Indic paths)
- **Input**: Roman script text (e.g., IAST "ka", "dharma")
- **Process at build time**:
  1. Create converter instance (e.g., `IASTConverter`)
  2. Create Hub instance
  3. For each test input:
     - `converter.to_hub(input)` → ISO result
     - `hub.iso_to_deva(iso_result)` → Devanagari result
     - Store: `input → devanagari_result`
- **Runtime**: Direct lookup of Roman → Devanagari

### 2. Indic → ISO (for Indic → Roman paths)
- **Input**: Indic script text (e.g., Telugu "క", "ధర్మ")
- **Process at build time**:
  1. Create converter instance (e.g., `TeluguConverter`)
  2. Create Hub instance
  3. For each test input:
     - `converter.to_hub(input)` → Devanagari result
     - `hub.deva_to_iso(deva_result)` → ISO result
     - Store: `input → iso_result`
- **Runtime**: Direct lookup of Indic → ISO

## Build.rs Implementation Strategy

### 1. Test String Generation

We need comprehensive test strings that cover all character combinations:

```rust
fn generate_test_strings_for_script(script: &str) -> Vec<String> {
    match script {
        "iast" => vec![
            // Individual characters
            "a", "ā", "i", "ī", "u", "ū", "ṛ", "ṝ", "ḷ", "ḹ",
            "e", "ai", "o", "au",
            
            // Consonants without vowels (for composition)
            "k", "kh", "g", "gh", "ṅ",
            "c", "ch", "j", "jh", "ñ",
            // ... all consonants
            
            // Consonant + vowel combinations
            "ka", "kā", "ki", "kī", "ku", "kū", "kṛ", "kṝ",
            "kha", "khā", "khi", "khī", // ... etc
            
            // Special combinations
            "kṣa", "jña", "tra", "śra",
            
            // With viramas
            "k", "kt", "kta",
            
            // Common words to ensure correctness
            "dharma", "karma", "yoga", "bhārata"
        ],
        // ... similar for other scripts
    }
}
```

### 2. Actual Conversion Using Hub

```rust
fn pre_compute_roman_to_devanagari(roman_script: &str) -> HashMap<String, String> {
    // This would be in build.rs, so we'd need to compile and load the actual modules
    // For now, showing the logic:
    
    let converter = create_converter(roman_script); // e.g., IASTConverter
    let hub = Hub::new();
    let mut mappings = HashMap::new();
    
    for test_string in generate_test_strings_for_script(roman_script) {
        // Step 1: Roman → ISO
        match converter.to_hub(roman_script, &test_string) {
            Ok(HubInput::Iso(iso_text)) => {
                // Step 2: ISO → Devanagari
                match hub.iso_to_deva(&iso_text) {
                    Ok(HubOutput::Devanagari(deva_text)) => {
                        mappings.insert(test_string, deva_text);
                    }
                    _ => {} // Skip if conversion fails
                }
            }
            _ => {} // Skip if not ISO output
        }
    }
    
    mappings
}
```

### 3. Code Generation

The generated converter should:
1. Store the pre-computed mappings
2. Use longest-match-first lookup (like RomanScriptProcessor)
3. Handle unknown characters gracefully

```rust
fn generate_direct_converter(
    from_script: &str, 
    to_script: &str,
    mappings: HashMap<String, String>
) -> String {
    format!(r#"
/// Direct converter from {from} to {to}
pub struct {struct_name} {{
    mappings: HashMap<&'static str, &'static str>,
}}

impl {struct_name} {{
    pub fn new() -> Self {{
        let mut mappings = HashMap::new();
        {mapping_inserts}
        Self {{ mappings }}
    }}
}}

impl ScriptConverter for {struct_name} {{
    fn to_hub(&self, _script: &str, input: &str) -> Result<HubInput, ConverterError> {{
        // For Roman → Devanagari, output is Devanagari
        // For Indic → ISO, output is ISO
        let output = self.convert(input)?;
        Ok({hub_output_variant}(output))
    }}
    
    fn from_hub(&self, _script: &str, _hub_input: &HubInput) -> Result<String, ConverterError> {{
        Err(ConverterError::ConversionFailed {{
            script: "{from}".to_string(),
            reason: "Direct converter bypasses hub".to_string(),
        }})
    }}
    
    // ... other trait methods
}}
"#, 
    from = from_script,
    to = to_script,
    struct_name = generate_struct_name(from_script, to_script),
    mapping_inserts = generate_mapping_inserts(&mappings),
    hub_output_variant = if to_script == "devanagari" { "HubInput::Devanagari" } else { "HubInput::Iso" }
    )
}
```

## Key Design Principles

1. **Use the Hub**: All conversions go through the actual Hub at build time
2. **Comprehensive Coverage**: Test strings must cover all possible character combinations
3. **Graceful Degradation**: Unknown characters fall back to runtime conversion
4. **Maintainability**: Generated code is readable and follows project patterns
5. **Correctness**: Pre-computed results must match runtime results exactly

## Testing Strategy

1. **Correctness Tests**: Verify pre-computed results match runtime hub conversion
2. **Coverage Tests**: Ensure all character combinations are pre-computed
3. **Performance Tests**: Measure actual performance improvement
4. **Fallback Tests**: Verify graceful handling of unknown inputs