use handlebars::Handlebars;
use rustc_hash::FxHashMap;
use serde_json::json;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(serde::Deserialize, Debug, Clone)]
struct ScriptMetadata {
    name: String,
    #[allow(dead_code)]
    script_type: String,
    #[allow(dead_code)]
    has_implicit_a: bool,
    aliases: Option<Vec<String>>,
}

#[derive(serde::Deserialize, Debug, Clone)]
struct TokenMappings {
    vowels: Option<FxHashMap<String, TokenMapping>>, // "VowelA" -> ["a", "A"] or "VowelA" -> "a"
    consonants: Option<FxHashMap<String, TokenMapping>>, // "ConsonantK" -> ["k", "K"]
    vowel_signs: Option<FxHashMap<String, TokenMapping>>, // For abugida scripts
    marks: Option<FxHashMap<String, TokenMapping>>,  // "MarkAnusvara" -> ["M", "ṁ"]
    digits: Option<FxHashMap<String, TokenMapping>>, // "Digit0" -> "0"
    special: Option<FxHashMap<String, TokenMapping>>, // "SpecialKs" -> ["kS", "kṣ"]
    extended: Option<FxHashMap<String, TokenMapping>>, // "ExtendedQ" -> "q"
    vedic: Option<FxHashMap<String, TokenMapping>>,  // "MarkUdatta" -> "॑"
}

// Support both single string and array of strings for flexibility
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(untagged)]
enum TokenMapping {
    Single(String),        // "a"
    Multiple(Vec<String>), // ["a", "A"]
}

#[derive(serde::Deserialize, Debug, Clone)]
struct CodegenConfig {
    #[allow(dead_code)]
    processor_type: String,
}

impl TokenMapping {
    #[allow(dead_code)]
    fn get_preferred(&self) -> String {
        match self {
            TokenMapping::Single(s) => s.clone(),
            TokenMapping::Multiple(vec) => vec.first().unwrap_or(&"".to_string()).clone(),
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
struct ScriptSchema {
    metadata: ScriptMetadata,
    target: Option<String>, // "alphabet_tokens" or "abugida_tokens" (optional for legacy schemas)
    mappings: TokenMappings,
    #[allow(dead_code)]
    codegen: Option<CodegenConfig>,
}

// Convert TokenMapping mappings to legacy String mappings for compatibility
#[allow(dead_code)]
fn flatten_token_mappings(mappings: &FxHashMap<String, TokenMapping>) -> FxHashMap<String, String> {
    mappings
        .iter()
        .map(|(k, v)| (k.clone(), v.get_preferred()))
        .collect()
}

fn main() {
    println!("cargo:rerun-if-changed=schemas/");
    println!("cargo:rerun-if-changed=templates/");

    if let Err(e) = generate_tokens_from_schemas() {
        println!("cargo:warning=Failed to generate tokens: {e}");
    }

    if let Err(e) = generate_schema_based_converters() {
        println!("cargo:warning=Failed to generate schema-based converters: {e}");
    }
}

/// Collect all unique tokens from schemas and generate tokens.rs
fn generate_tokens_from_schemas() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let schemas_dir = Path::new("schemas");

    // Collections for unique tokens
    let mut abugida_vowels = BTreeSet::new();
    let mut abugida_vowel_signs = BTreeSet::new();
    let mut abugida_consonants = BTreeSet::new();
    let mut abugida_marks = BTreeSet::new();
    let mut abugida_special = BTreeSet::new();
    let mut abugida_digits = BTreeSet::new();
    let mut abugida_vedic = BTreeSet::new();

    let mut alphabet_vowels = BTreeSet::new();
    let mut alphabet_consonants = BTreeSet::new();
    let mut alphabet_marks = BTreeSet::new();
    let mut alphabet_special = BTreeSet::new();
    let mut alphabet_digits = BTreeSet::new();
    let mut alphabet_vedic = BTreeSet::new();

    // Process all YAML schemas
    if schemas_dir.exists() {
        for entry in fs::read_dir(schemas_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                let content = fs::read_to_string(&path)?;
                let schema: ScriptSchema = serde_yaml::from_str(&content)
                    .map_err(|e| format!("Failed to parse YAML schema {}: {e}", path.display()))?;

                // Skip debug schemas
                if schema.metadata.name == "abugida_tokens"
                    || schema.metadata.name == "alphabet_tokens"
                {
                    continue;
                }

                // Skip non-token schemas
                let target = match &schema.target {
                    Some(t) => t,
                    None => continue,
                };

                let is_abugida = target == "abugida_tokens";
                let is_alphabet = target == "alphabet_tokens";

                if !is_abugida && !is_alphabet {
                    continue;
                }

                // Collect tokens from each category
                if let Some(vowels) = &schema.mappings.vowels {
                    for token in vowels.keys() {
                        if is_abugida {
                            abugida_vowels.insert(token.clone());
                        } else {
                            alphabet_vowels.insert(token.clone());
                        }
                    }
                }

                if let Some(vowel_signs) = &schema.mappings.vowel_signs {
                    for token in vowel_signs.keys() {
                        if is_abugida {
                            abugida_vowel_signs.insert(token.clone());
                        }
                    }
                }

                if let Some(consonants) = &schema.mappings.consonants {
                    for token in consonants.keys() {
                        if is_abugida {
                            abugida_consonants.insert(token.clone());
                        } else {
                            alphabet_consonants.insert(token.clone());
                        }
                    }
                }

                if let Some(marks) = &schema.mappings.marks {
                    for token in marks.keys() {
                        if is_abugida {
                            abugida_marks.insert(token.clone());
                        } else {
                            alphabet_marks.insert(token.clone());
                        }
                    }
                }

                if let Some(special) = &schema.mappings.special {
                    for token in special.keys() {
                        if is_abugida {
                            abugida_special.insert(token.clone());
                        } else {
                            alphabet_special.insert(token.clone());
                        }
                    }
                }

                if let Some(digits) = &schema.mappings.digits {
                    for token in digits.keys() {
                        if is_abugida {
                            abugida_digits.insert(token.clone());
                        } else {
                            alphabet_digits.insert(token.clone());
                        }
                    }
                }

                if let Some(vedic) = &schema.mappings.vedic {
                    for token in vedic.keys() {
                        if is_abugida {
                            abugida_vedic.insert(token.clone());
                        } else {
                            alphabet_vedic.insert(token.clone());
                        }
                    }
                }
            }
        }
    }

    // Generate tokens.rs using template
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(false);
    handlebars.register_escape_fn(handlebars::no_escape);
    handlebars.register_template_file("tokens", "templates/tokens.hbs")?;

    // Generate vowel to sign mappings
    let mut vowel_to_sign_mappings = Vec::new();
    for vowel in &abugida_vowels {
        if vowel == "VowelA" {
            continue; // No sign for short 'a'
        }
        // Extract the vowel suffix (e.g., "Aa" from "VowelAa")
        if let Some(suffix) = vowel.strip_prefix("Vowel") {
            let sign_name = format!("VowelSign{}", suffix);
            if abugida_vowel_signs.contains(&sign_name) {
                vowel_to_sign_mappings.push(json!({
                    "vowel": vowel,
                    "sign": sign_name,
                }));
            }
        }
    }

    // Generate same sound mappings (tokens with same name exist in both systems)
    let mut same_sound_mappings = Vec::new();
    let mut abugida_to_alphabet_mappings = Vec::new();
    let mut alphabet_to_abugida_mappings = Vec::new();

    // Collect all abugida tokens
    let all_abugida_tokens: Vec<_> = abugida_vowels
        .iter()
        .chain(abugida_consonants.iter())
        .chain(abugida_marks.iter())
        .chain(abugida_special.iter())
        .chain(abugida_vedic.iter())
        .chain(abugida_digits.iter())
        .collect();

    // Collect all alphabet tokens
    let all_alphabet_tokens: Vec<_> = alphabet_vowels
        .iter()
        .chain(alphabet_consonants.iter())
        .chain(alphabet_marks.iter())
        .chain(alphabet_special.iter())
        .chain(alphabet_vedic.iter())
        .chain(alphabet_digits.iter())
        .collect();

    // Generate direct mappings where token names match
    for abugida_token in &all_abugida_tokens {
        if all_alphabet_tokens.iter().any(|a| a == abugida_token) {
            same_sound_mappings.push(json!({
                "abugida": abugida_token,
                "alphabet": abugida_token,
            }));
            abugida_to_alphabet_mappings.push(json!({
                "from": abugida_token,
                "to": abugida_token,
            }));
        }
    }

    // Generate reverse mappings
    for alphabet_token in &all_alphabet_tokens {
        if all_abugida_tokens.iter().any(|a| a == alphabet_token) {
            alphabet_to_abugida_mappings.push(json!({
                "from": alphabet_token,
                "to": alphabet_token,
            }));
        }
    }

    // Add vowel sign to vowel mappings
    for sign in &abugida_vowel_signs {
        // Extract the vowel part from VowelSignXxx
        if let Some(vowel_suffix) = sign.strip_prefix("VowelSign") {
            let vowel_name = format!("Vowel{}", vowel_suffix);
            if alphabet_vowels.contains(&vowel_name) {
                abugida_to_alphabet_mappings.push(json!({
                    "from": sign,
                    "to": vowel_name,
                }));
            }
        }
    }

    // Handle special cases where tokens don't exist in one system
    // These could be read from schema files in the future
    let special_mappings = vec![
        // If alphabet doesn't have long e/o, they still map to themselves for preservation
        ("VowelEe", "VowelEe"),
        ("VowelOo", "VowelOo"),
        // Vocalic L often doesn't exist in many scripts
        ("VowelL", "VowelL"),
        ("VowelLl", "VowelLl"),
    ];

    for (abugida, alphabet) in special_mappings {
        if (abugida_vowels.contains(abugida) || abugida_marks.contains(abugida))
            && !alphabet_vowels.contains(alphabet)
            && !alphabet_marks.contains(alphabet)
        {
            // This token exists in abugida but not alphabet - it will be preserved as-is
            abugida_to_alphabet_mappings.push(json!({
                "from": abugida,
                "to": abugida,  // Map to itself for preservation
            }));
        }
    }

    let template_data = json!({
        "abugida_vowels": abugida_vowels.into_iter().collect::<Vec<_>>(),
        "abugida_vowel_signs": abugida_vowel_signs.into_iter().collect::<Vec<_>>(),
        "abugida_consonants": abugida_consonants.into_iter().collect::<Vec<_>>(),
        "abugida_marks": abugida_marks.into_iter().collect::<Vec<_>>(),
        "abugida_special": abugida_special.into_iter().collect::<Vec<_>>(),
        "abugida_vedic": abugida_vedic.into_iter().collect::<Vec<_>>(),
        "abugida_digits": abugida_digits.into_iter().collect::<Vec<_>>(),
        "alphabet_vowels": alphabet_vowels.into_iter().collect::<Vec<_>>(),
        "alphabet_consonants": alphabet_consonants.into_iter().collect::<Vec<_>>(),
        "alphabet_marks": alphabet_marks.into_iter().collect::<Vec<_>>(),
        "alphabet_special": alphabet_special.into_iter().collect::<Vec<_>>(),
        "alphabet_vedic": alphabet_vedic.into_iter().collect::<Vec<_>>(),
        "alphabet_digits": alphabet_digits.into_iter().collect::<Vec<_>>(),
        "vowel_to_sign_mappings": vowel_to_sign_mappings,
        "same_sound_mappings": same_sound_mappings,
        "abugida_to_alphabet_mappings": abugida_to_alphabet_mappings,
        "alphabet_to_abugida_mappings": alphabet_to_abugida_mappings,
    });

    let tokens_code = handlebars.render("tokens", &template_data)?;
    fs::write(out_dir.join("tokens_generated.rs"), tokens_code)?;

    Ok(())
}

fn generate_schema_based_converters() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let schemas_dir = Path::new("schemas");

    // Initialize Handlebars template engine - token-based only!
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(false);
    handlebars.register_escape_fn(handlebars::no_escape);
    handlebars.register_template_file(
        "token_based_converter",
        "templates/token_based_converter.hbs",
    )?;
    handlebars.register_template_file("direct_converter", "templates/direct_converter.hbs")?;

    // Register helper functions for templates
    handlebars.register_helper("uppercase", Box::new(uppercase_helper));
    handlebars.register_helper("lowercase", Box::new(lowercase_helper));
    handlebars.register_helper("capitalize", Box::new(capitalize_helper));
    handlebars.register_helper("escape", Box::new(escape_helper));
    handlebars.register_helper("eq", Box::new(eq_helper));

    let mut generated_code = String::new();
    let mut converter_registrations = Vec::new();
    let mut schemas = Vec::new();

    // Add header with all necessary imports once
    generated_code.push_str(
        r#"
// Auto-generated converters from TOML/YAML schemas
// DO NOT EDIT - Generated by build.rs at compile time

#[allow(unused_imports)]
#[allow(unreachable_patterns)]
#[allow(dead_code)]
#[allow(clippy::new_without_default)]
#[allow(clippy::clone_on_copy)]
#[allow(clippy::match_like_matches_macro)]
#[allow(clippy::duplicated_attributes)]

use once_cell::sync::Lazy;
use crate::modules::hub::HubFormat;
use crate::modules::hub::tokens::{AbugidaToken, AlphabetToken, HubToken, HubTokenSequence};
use aho_corasick::AhoCorasick;

"#,
    );

    // Process YAML schemas
    if schemas_dir.exists() {
        for entry in fs::read_dir(schemas_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                println!("cargo:rerun-if-changed={}", path.display());

                let content = fs::read_to_string(&path)?;
                let schema: ScriptSchema = serde_yaml::from_str(&content)
                    .map_err(|e| format!("Failed to parse YAML schema {}: {e}", path.display()))?;

                // Add schema to collection for Hub generation
                schemas.push(schema.clone());

                // Only process token-based schemas
                if let Some(ref target) = schema.target {
                    if target != "alphabet_tokens" && target != "abugida_tokens" {
                        continue; // Skip non-token schemas
                    }
                } else {
                    continue; // Skip schemas without target
                }

                let converter_code =
                    generate_converter_from_schema(&handlebars, &schema).map_err(|e| {
                        format!(
                            "Failed to generate converter for {}: {e}",
                            schema.metadata.name
                        )
                    })?;
                generated_code.push_str(&converter_code);

                // Only register token-based converters!
                if let Some(ref target) = schema.target {
                    if target == "alphabet_tokens" || target == "abugida_tokens" {
                        converter_registrations.push(format!(
                            "{}Converter",
                            capitalize_first(&schema.metadata.name)
                        ));
                    }
                }

                // No more Roman → Devanagari converters - everything goes through tokens!
            }
        }
    }

    // Hub converter is no longer needed - using trait_based_converter instead

    // Generate direct converters for common script pairs to bypass hub overhead
    if let Ok(direct_code) = generate_direct_converters(&handlebars, &schemas) {
        fs::write(out_dir.join("direct_converters_generated.rs"), direct_code)?;
    }

    // Generate token-based converter registry with aliases
    let token_registrations = converter_registrations
        .iter()
        .map(|name| format!("        Box::new({name}::new()),"))
        .collect::<Vec<_>>()
        .join("\n");

    // Generate registration with aliases
    let token_registrations_with_aliases = schemas
        .iter()
        .filter_map(|schema| {
            let converter_name = format!(
                "{}Converter",
                schema
                    .metadata
                    .name
                    .chars()
                    .next()
                    .unwrap()
                    .to_uppercase()
                    .to_string()
                    + &schema.metadata.name[1..]
            );

            if converter_registrations.contains(&converter_name) {
                let aliases = schema
                    .metadata
                    .aliases
                    .as_ref()
                    .map(|aliases| {
                        aliases
                            .iter()
                            .map(|alias| format!("\"{alias}\".to_string()"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    })
                    .unwrap_or_default();

                Some(format!(
                    "        (Box::new({converter_name}::new()), vec![{aliases}]),"
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    generated_code.push_str(&format!(r#"
/// Register token-based converters
pub fn register_schema_generated_converters(_registry: &mut crate::modules::script_converter::ScriptConverterRegistry) {{
    // Old string-based registration system has been removed
    // Token-based converters are managed separately
}}

/// Get all token-based converters (legacy)
pub fn register_token_converters() -> Vec<Box<dyn crate::modules::script_converter::TokenConverter>> {{
    vec![
{token_registrations}
    ]
}}

/// Get all token-based converters with their aliases
pub fn register_token_converters_with_aliases() -> Vec<(Box<dyn crate::modules::script_converter::TokenConverter>, Vec<String>)> {{
    vec![
{token_registrations_with_aliases}
    ]
}}
"#));

    // Generate script type helper functions
    let mut brahmic_scripts = Vec::new();
    let mut roman_scripts = Vec::new();

    for schema in &schemas {
        match schema.metadata.script_type.as_str() {
            "brahmic" => {
                brahmic_scripts.push(format!("\"{}\"", schema.metadata.name.to_lowercase()));
                if let Some(aliases) = &schema.metadata.aliases {
                    for alias in aliases {
                        brahmic_scripts.push(format!("\"{}\"", alias.to_lowercase()));
                    }
                }
            }
            "roman" => {
                roman_scripts.push(format!("\"{}\"", schema.metadata.name.to_lowercase()));
                if let Some(aliases) = &schema.metadata.aliases {
                    for alias in aliases {
                        roman_scripts.push(format!("\"{}\"", alias.to_lowercase()));
                    }
                }
            }
            _ => {}
        }
    }

    let script_helpers = format!(
        r#"
/// Check if a script is an Indic/Brahmic script
pub fn is_indic_script(script: &str) -> bool {{
    matches!(
        script.to_lowercase().as_str(),
        {}
    )
}}

/// Check if a script is a Roman script
pub fn is_roman_script(script: &str) -> bool {{
    matches!(
        script.to_lowercase().as_str(),
        {}
    )
}}
"#,
        brahmic_scripts.join("\n            | "),
        roman_scripts.join("\n            | ")
    );

    generated_code.push_str(&script_helpers);

    // Write generated code
    fs::write(out_dir.join("schema_generated.rs"), generated_code)?;
    Ok(())
}

fn generate_converter_from_schema(
    handlebars: &Handlebars,
    schema: &ScriptSchema,
) -> Result<String, Box<dyn std::error::Error>> {
    // Token-based only! All converters must use tokens
    generate_token_based_converter(handlebars, schema)
}

fn capitalize_first(s: &str) -> String {
    // Convert kebab-case and snake_case to PascalCase
    s.split(&['-', '_'][..])
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

// Template-based converter generators

#[allow(dead_code)]
fn generate_roman_converter_with_template(
    handlebars: &Handlebars,
    struct_name: &str,
    script_name: &str,
    mappings: &FxHashMap<String, String>,
    metadata: &ScriptMetadata,
    canonical_forms: &Option<FxHashMap<String, String>>,
    use_aho_corasick: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    // Sort by length (longest first) for proper matching
    let mut sorted_mappings: Vec<_> = mappings.iter().collect();
    sorted_mappings.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    // Prepare ALL reverse mappings for template (not just multi-character ones)
    // Note: reverse mapping means ISO → source_script, so ISO should be the key
    let mut reverse_mappings: Vec<(&str, &str)> = sorted_mappings
        .iter()
        .map(|(from, to)| (to.as_str(), from.as_str())) // (ISO, source_script)
        .collect();

    // Sort by length (longest first) for proper matching priority
    reverse_mappings.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    // Create reverse mappings with preference for canonical forms
    let mut reverse_priority_mappings: FxHashMap<&str, &str> = FxHashMap::default();

    // If canonical forms are specified in schema, use them; otherwise use first occurrence
    if let Some(canonical) = canonical_forms {
        // Pre-build sets for faster lookup (avoids repeated iteration)
        let iso_values: std::collections::HashSet<&str> =
            mappings.values().map(|s| s.as_str()).collect();
        let source_keys: std::collections::HashSet<&str> =
            mappings.keys().map(|s| s.as_str()).collect();

        // First, add all mappings
        for (iso_char, source_char) in reverse_mappings {
            reverse_priority_mappings.insert(iso_char, source_char);
        }

        // Then override with canonical forms where specified
        for (iso_char, canonical_source) in canonical {
            // Quick verification using pre-built sets
            if iso_values.contains(iso_char.as_str())
                && source_keys.contains(canonical_source.as_str())
            {
                reverse_priority_mappings.insert(iso_char, canonical_source);
            }
        }
    } else {
        // No canonical forms specified, use first occurrence (original behavior)
        for (iso_char, source_char) in reverse_mappings {
            reverse_priority_mappings
                .entry(iso_char)
                .or_insert(source_char);
        }
    }

    // Convert to JSON for template
    let mappings_json: FxHashMap<&str, &str> = mappings
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();

    // Add script-specific aliases
    let aliases = match script_name {
        "harvard_kyoto" => vec![
            ("hk_to_iso", "harvard_kyoto_to_iso"),
            ("iso_to_hk", "iso_to_harvard_kyoto"),
        ],
        _ => vec![],
    };

    let template_data = json!({
        "struct_name": struct_name,
        "script_name": script_name,
        "has_implicit_a": metadata.has_implicit_a,
        "mappings": mappings_json,
        "reverse_priority_mappings": reverse_priority_mappings,
        "aliases": aliases
    });

    let template_name = if use_aho_corasick {
        "roman_converter_aho_corasick"
    } else {
        "roman_converter"
    };

    let code = handlebars.render(template_name, &template_data)?;
    Ok(code)
}

// Handlebars helper functions
fn uppercase_helper(
    h: &handlebars::Helper,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _rc: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0).unwrap().value().as_str().unwrap();
    out.write(&param.to_uppercase())?;
    Ok(())
}

fn lowercase_helper(
    h: &handlebars::Helper,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _rc: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0).unwrap().value().as_str().unwrap();
    out.write(&param.to_lowercase())?;
    Ok(())
}

fn capitalize_helper(
    h: &handlebars::Helper,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _rc: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0).unwrap().value().as_str().unwrap();
    out.write(&capitalize_first(param))?;
    Ok(())
}

fn escape_helper(
    h: &handlebars::Helper,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _rc: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0).unwrap().value().as_str().unwrap();
    out.write(&escape_string(param))?;
    Ok(())
}

fn eq_helper(
    h: &handlebars::Helper,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _rc: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param1 = h.param(0).unwrap().value().as_str().unwrap();
    let param2 = h.param(1).unwrap().value().as_str().unwrap();
    if param1 == param2 {
        out.write("true")?;
    } else {
        out.write("false")?;
    }
    Ok(())
}

#[allow(dead_code)]
fn generate_indic_converter_with_template(
    handlebars: &Handlebars,
    struct_name: &str,
    script_name: &str,
    mappings: &FxHashMap<String, String>,
    metadata: &ScriptMetadata,
) -> Result<String, Box<dyn std::error::Error>> {
    // Filter to only single-character mappings for optimized HashMap<char, char>
    let char_mappings: FxHashMap<&str, &str> = mappings
        .iter()
        .filter(|(from, to)| from.chars().count() == 1 && to.chars().count() == 1)
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();

    let template_data = json!({
        "struct_name": struct_name,
        "script_name": script_name,
        "has_implicit_a": metadata.has_implicit_a,
        "char_mappings": char_mappings
    });

    let code = handlebars.render("indic_standard_converter", &template_data)?;
    Ok(code)
}

#[allow(dead_code)]
fn generate_extended_indic_converter_with_template(
    handlebars: &Handlebars,
    struct_name: &str,
    script_name: &str,
    mappings: &FxHashMap<String, String>,
    metadata: &ScriptMetadata,
) -> Result<String, Box<dyn std::error::Error>> {
    // Use all mappings as strings for complex script support
    let string_mappings: FxHashMap<&str, &str> = mappings
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();

    let template_data = json!({
        "struct_name": struct_name,
        "script_name": script_name,
        "has_implicit_a": metadata.has_implicit_a,
        "string_mappings": string_mappings
    });

    let code = handlebars.render("indic_extended_converter", &template_data)?;
    Ok(code)
}

#[allow(dead_code)]
fn generate_roman_to_devanagari_converter(
    handlebars: &Handlebars,
    schema: &ScriptSchema,
) -> Result<String, Box<dyn std::error::Error>> {
    let script_name = &schema.metadata.name;
    let struct_name = format!("{}DevanagariConverter", capitalize_first(script_name));

    // Get ISO-15919 to Devanagari mappings from hub (at compile time)
    let iso_to_deva_mappings = get_iso_to_devanagari_mappings();

    // Compose Roman → ISO-15919 → Devanagari mappings at COMPILE TIME
    let mut roman_to_deva_mappings = FxHashMap::default();

    // Process all mapping categories from the schema
    let all_roman_mappings = [
        &schema.mappings.vowels,
        &schema.mappings.consonants,
        &schema.mappings.marks,
        &schema.mappings.digits,
        &schema.mappings.special,
    ];

    for mappings in all_roman_mappings.iter().copied().flatten() {
        for (roman_key, iso_token_mapping) in mappings.iter() {
            let iso_value = iso_token_mapping.get_preferred(); // Get the preferred ISO mapping

            // Direct mapping for all keys
            if let Some(deva_value) = iso_to_deva_mappings.get(&iso_value) {
                roman_to_deva_mappings.insert(roman_key.clone(), deva_value.clone());
            }

            // For consonants, also try with 'a' suffix (for inherent vowel)
            let iso_with_a = format!("{iso_value}a");
            if let Some(deva_value) = iso_to_deva_mappings.get(&iso_with_a) {
                let roman_with_a = format!("{roman_key}a");
                roman_to_deva_mappings.insert(roman_with_a, deva_value.clone());
            }
        }
    }

    // Add vowel sign mappings by deriving them from hub data
    let vowels_as_strings = schema.mappings.vowels.as_ref().map(flatten_token_mappings);
    add_vowel_sign_mappings(
        &mut roman_to_deva_mappings,
        &iso_to_deva_mappings,
        &vowels_as_strings,
    );

    // Sort by length (longest first) for proper matching
    let mut sorted_mappings: Vec<_> = roman_to_deva_mappings.iter().collect();
    sorted_mappings.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    // Convert to template format - use the original String keys
    let mappings_for_template = &roman_to_deva_mappings;

    let template_data = json!({
        "struct_name": struct_name,
        "script_name": format!("{}_devanagari", script_name),
        "original_script": script_name,
        "has_implicit_a": schema.metadata.has_implicit_a,
        "mappings": mappings_for_template,
    });

    let code = handlebars.render("roman_to_devanagari_direct", &template_data)?;
    Ok(code)
}

#[allow(dead_code)]
fn get_iso_to_devanagari_mappings() -> FxHashMap<String, String> {
    // Hardcode the hub mappings since we can't access the crate from build.rs
    // This is still better than before since we're deriving vowel signs dynamically
    let mut iso_to_deva_mappings = FxHashMap::default();

    // Core vowels
    iso_to_deva_mappings.insert("a".to_string(), "अ".to_string());
    iso_to_deva_mappings.insert("ā".to_string(), "आ".to_string());
    iso_to_deva_mappings.insert("i".to_string(), "इ".to_string());
    iso_to_deva_mappings.insert("ī".to_string(), "ई".to_string());
    iso_to_deva_mappings.insert("u".to_string(), "उ".to_string());
    iso_to_deva_mappings.insert("ū".to_string(), "ऊ".to_string());
    iso_to_deva_mappings.insert("r̥".to_string(), "ऋ".to_string());
    iso_to_deva_mappings.insert("r̥̄".to_string(), "ॠ".to_string());
    iso_to_deva_mappings.insert("l̥".to_string(), "ऌ".to_string());
    iso_to_deva_mappings.insert("l̥̄".to_string(), "ॡ".to_string());
    iso_to_deva_mappings.insert("e".to_string(), "ए".to_string());
    iso_to_deva_mappings.insert("ai".to_string(), "ऐ".to_string());
    iso_to_deva_mappings.insert("o".to_string(), "ओ".to_string());
    iso_to_deva_mappings.insert("au".to_string(), "औ".to_string());

    // Consonants with inherent 'a' vowel
    iso_to_deva_mappings.insert("ka".to_string(), "क".to_string());
    iso_to_deva_mappings.insert("kha".to_string(), "ख".to_string());
    iso_to_deva_mappings.insert("ga".to_string(), "ग".to_string());
    iso_to_deva_mappings.insert("gha".to_string(), "घ".to_string());
    iso_to_deva_mappings.insert("ṅa".to_string(), "ङ".to_string());
    iso_to_deva_mappings.insert("ca".to_string(), "च".to_string());
    iso_to_deva_mappings.insert("cha".to_string(), "छ".to_string());
    iso_to_deva_mappings.insert("ja".to_string(), "ज".to_string());
    iso_to_deva_mappings.insert("jha".to_string(), "झ".to_string());
    iso_to_deva_mappings.insert("ña".to_string(), "ञ".to_string());
    iso_to_deva_mappings.insert("ṭa".to_string(), "ट".to_string());
    iso_to_deva_mappings.insert("ṭha".to_string(), "ठ".to_string());
    iso_to_deva_mappings.insert("ḍa".to_string(), "ड".to_string());
    iso_to_deva_mappings.insert("ḍha".to_string(), "ढ".to_string());
    iso_to_deva_mappings.insert("ṇa".to_string(), "ण".to_string());
    iso_to_deva_mappings.insert("ta".to_string(), "त".to_string());
    iso_to_deva_mappings.insert("tha".to_string(), "थ".to_string());
    iso_to_deva_mappings.insert("da".to_string(), "द".to_string());
    iso_to_deva_mappings.insert("dha".to_string(), "ध".to_string());
    iso_to_deva_mappings.insert("na".to_string(), "न".to_string());
    iso_to_deva_mappings.insert("pa".to_string(), "प".to_string());
    iso_to_deva_mappings.insert("pha".to_string(), "फ".to_string());
    iso_to_deva_mappings.insert("ba".to_string(), "ब".to_string());
    iso_to_deva_mappings.insert("bha".to_string(), "भ".to_string());
    iso_to_deva_mappings.insert("ma".to_string(), "म".to_string());
    iso_to_deva_mappings.insert("ya".to_string(), "य".to_string());
    iso_to_deva_mappings.insert("ra".to_string(), "र".to_string());
    iso_to_deva_mappings.insert("la".to_string(), "ल".to_string());
    iso_to_deva_mappings.insert("va".to_string(), "व".to_string());
    iso_to_deva_mappings.insert("śa".to_string(), "श".to_string());
    iso_to_deva_mappings.insert("ṣa".to_string(), "ष".to_string());
    iso_to_deva_mappings.insert("sa".to_string(), "स".to_string());
    iso_to_deva_mappings.insert("ha".to_string(), "ह".to_string());

    // Consonants with other vowels for vowel sign extraction
    iso_to_deva_mappings.insert("kā".to_string(), "का".to_string());
    iso_to_deva_mappings.insert("ki".to_string(), "कि".to_string());
    iso_to_deva_mappings.insert("kī".to_string(), "की".to_string());
    iso_to_deva_mappings.insert("ku".to_string(), "कु".to_string());
    iso_to_deva_mappings.insert("kū".to_string(), "कू".to_string());
    iso_to_deva_mappings.insert("kr̥".to_string(), "कृ".to_string());
    iso_to_deva_mappings.insert("kr̥̄".to_string(), "कॄ".to_string());
    iso_to_deva_mappings.insert("kl̥".to_string(), "कॢ".to_string());
    iso_to_deva_mappings.insert("kl̥̄".to_string(), "कॣ".to_string());
    iso_to_deva_mappings.insert("ke".to_string(), "के".to_string());
    iso_to_deva_mappings.insert("kai".to_string(), "कै".to_string());
    iso_to_deva_mappings.insert("ko".to_string(), "को".to_string());
    iso_to_deva_mappings.insert("kau".to_string(), "कौ".to_string());

    // Marks
    iso_to_deva_mappings.insert("ṁ".to_string(), "ं".to_string());
    iso_to_deva_mappings.insert("ḥ".to_string(), "ः".to_string());
    iso_to_deva_mappings.insert("m̐".to_string(), "ँ".to_string());
    iso_to_deva_mappings.insert("'".to_string(), "ऽ".to_string());

    // Digits
    iso_to_deva_mappings.insert("0".to_string(), "०".to_string());
    iso_to_deva_mappings.insert("1".to_string(), "१".to_string());
    iso_to_deva_mappings.insert("2".to_string(), "२".to_string());
    iso_to_deva_mappings.insert("3".to_string(), "३".to_string());
    iso_to_deva_mappings.insert("4".to_string(), "४".to_string());
    iso_to_deva_mappings.insert("5".to_string(), "५".to_string());
    iso_to_deva_mappings.insert("6".to_string(), "६".to_string());
    iso_to_deva_mappings.insert("7".to_string(), "७".to_string());
    iso_to_deva_mappings.insert("8".to_string(), "८".to_string());
    iso_to_deva_mappings.insert("9".to_string(), "९".to_string());

    // Special
    iso_to_deva_mappings.insert("ḷa".to_string(), "ळ".to_string());
    iso_to_deva_mappings.insert("।".to_string(), "।".to_string());
    iso_to_deva_mappings.insert("॥".to_string(), "॥".to_string());
    iso_to_deva_mappings.insert("kṣa".to_string(), "क्ष".to_string());
    iso_to_deva_mappings.insert("jña".to_string(), "ज्ञ".to_string());

    // Nukta consonants (for extended characters) - using precomposed forms
    iso_to_deva_mappings.insert("qa".to_string(), "\u{0958}".to_string()); // क़ precomposed
    iso_to_deva_mappings.insert("ḵẖa".to_string(), "\u{0959}".to_string()); // ख़ precomposed
    iso_to_deva_mappings.insert("ġa".to_string(), "\u{095A}".to_string()); // ग़ precomposed
    iso_to_deva_mappings.insert("za".to_string(), "\u{095B}".to_string()); // ज़ precomposed
    iso_to_deva_mappings.insert("ṛa".to_string(), "\u{095C}".to_string()); // ड़ precomposed
    iso_to_deva_mappings.insert("ṛha".to_string(), "\u{095D}".to_string()); // ढ़ precomposed
    iso_to_deva_mappings.insert("fa".to_string(), "\u{095E}".to_string()); // फ़ precomposed
    iso_to_deva_mappings.insert("ẏa".to_string(), "\u{095F}".to_string()); // य़ precomposed

    iso_to_deva_mappings
}

#[allow(dead_code)]
fn add_vowel_sign_mappings(
    roman_to_deva_mappings: &mut FxHashMap<String, String>,
    iso_to_deva_mappings: &FxHashMap<String, String>,
    vowel_mappings: &Option<FxHashMap<String, String>>,
) {
    if let Some(vowels) = vowel_mappings {
        for (roman_vowel, iso_vowel) in vowels.iter() {
            // Get the vowel sign by testing with a sample consonant
            if let (Some(ka_vowel), Some(ka_base)) = (
                iso_to_deva_mappings.get(&format!("k{iso_vowel}")),
                iso_to_deva_mappings.get("ka"),
            ) {
                // Extract vowel sign by comparing ka + vowel vs ka
                if ka_vowel != ka_base && ka_vowel.starts_with(ka_base) {
                    let vowel_sign = &ka_vowel[ka_base.len()..];
                    if !vowel_sign.is_empty() {
                        roman_to_deva_mappings.insert(
                            format!("__vowel_sign_{roman_vowel}"),
                            vowel_sign.to_string(),
                        );
                    }
                }
            }
        }
    }
}

fn generate_token_based_converter(
    handlebars: &Handlebars,
    schema: &ScriptSchema,
) -> Result<String, Box<dyn std::error::Error>> {
    let script_name = &schema.metadata.name;
    let struct_name = format!("{}Converter", capitalize_first(script_name));
    let is_alphabet = schema.target.as_deref() == Some("alphabet_tokens");

    // Collect all mappings with their categories
    let mut mappings = Vec::new();

    if let Some(ref vowels) = schema.mappings.vowels {
        let entries: Vec<_> = vowels
            .iter()
            .map(|(token, mapping)| {
                let (preferred, all_inputs) = match mapping {
                    TokenMapping::Single(s) => (s.clone(), vec![s.clone()]),
                    TokenMapping::Multiple(v) => (v[0].clone(), v.clone()),
                };
                // Debug output to understand the issue
                json!({
                    "token": token,
                    "preferred": preferred,
                    "all_inputs": all_inputs
                })
            })
            .collect();
        mappings.push(json!({
            "category": "Vowels",
            "entries": entries
        }));
    }

    if let Some(ref consonants) = schema.mappings.consonants {
        let entries: Vec<_> = consonants
            .iter()
            .map(|(token, mapping)| {
                let (preferred, all_inputs) = match mapping {
                    TokenMapping::Single(s) => (s.clone(), vec![s.clone()]),
                    TokenMapping::Multiple(v) => (v[0].clone(), v.clone()),
                };
                json!({
                    "token": token,
                    "preferred": preferred,
                    "all_inputs": all_inputs
                })
            })
            .collect();
        mappings.push(json!({
            "category": "Consonants",
            "entries": entries
        }));
    }

    if let Some(ref vowel_signs) = schema.mappings.vowel_signs {
        let entries: Vec<_> = vowel_signs
            .iter()
            .map(|(token, mapping)| {
                let (preferred, all_inputs) = match mapping {
                    TokenMapping::Single(s) => (s.clone(), vec![s.clone()]),
                    TokenMapping::Multiple(v) => (v[0].clone(), v.clone()),
                };
                json!({
                    "token": token,
                    "preferred": preferred,
                    "all_inputs": all_inputs
                })
            })
            .collect();
        mappings.push(json!({
            "category": "Vowel Signs",
            "entries": entries
        }));
    }

    if let Some(ref marks) = schema.mappings.marks {
        let entries: Vec<_> = marks
            .iter()
            .map(|(token, mapping)| {
                let (preferred, all_inputs) = match mapping {
                    TokenMapping::Single(s) => (s.clone(), vec![s.clone()]),
                    TokenMapping::Multiple(v) => (v[0].clone(), v.clone()),
                };
                json!({
                    "token": token,
                    "preferred": preferred,
                    "all_inputs": all_inputs
                })
            })
            .collect();
        mappings.push(json!({
            "category": "Marks",
            "entries": entries
        }));
    }

    if let Some(ref special) = schema.mappings.special {
        let entries: Vec<_> = special
            .iter()
            .map(|(token, mapping)| {
                let (preferred, all_inputs) = match mapping {
                    TokenMapping::Single(s) => (s.clone(), vec![s.clone()]),
                    TokenMapping::Multiple(v) => (v[0].clone(), v.clone()),
                };
                json!({
                    "token": token,
                    "preferred": preferred,
                    "all_inputs": all_inputs
                })
            })
            .collect();
        mappings.push(json!({
            "category": "Special",
            "entries": entries
        }));
    }

    if let Some(ref extended) = schema.mappings.extended {
        let entries: Vec<_> = extended
            .iter()
            .map(|(token, mapping)| {
                let (preferred, all_inputs) = match mapping {
                    TokenMapping::Single(s) => (s.clone(), vec![s.clone()]),
                    TokenMapping::Multiple(v) => (v[0].clone(), v.clone()),
                };
                json!({
                    "token": token,
                    "preferred": preferred,
                    "all_inputs": all_inputs
                })
            })
            .collect();
        mappings.push(json!({
            "category": "Extended",
            "entries": entries
        }));
    }

    if let Some(ref vedic) = schema.mappings.vedic {
        let entries: Vec<_> = vedic
            .iter()
            .map(|(token, mapping)| {
                let (preferred, all_inputs) = match mapping {
                    TokenMapping::Single(s) => (s.clone(), vec![s.clone()]),
                    TokenMapping::Multiple(v) => (v[0].clone(), v.clone()),
                };
                json!({
                    "token": token,
                    "preferred": preferred,
                    "all_inputs": all_inputs
                })
            })
            .collect();
        mappings.push(json!({
            "category": "Vedic",
            "entries": entries
        }));
    }

    if let Some(ref digits) = schema.mappings.digits {
        let entries: Vec<_> = digits
            .iter()
            .map(|(token, mapping)| {
                let (preferred, all_inputs) = match mapping {
                    TokenMapping::Single(s) => (s.clone(), vec![s.clone()]),
                    TokenMapping::Multiple(v) => (v[0].clone(), v.clone()),
                };
                json!({
                    "token": token,
                    "preferred": preferred,
                    "all_inputs": all_inputs
                })
            })
            .collect();
        mappings.push(json!({
            "category": "Digits",
            "entries": entries
        }));
    }

    // Check if there are multi-character mappings
    let has_multi_char_mappings = schema
        .mappings
        .vowels
        .as_ref()
        .map(|m| m.keys().any(|k| k.len() > 1))
        .unwrap_or(false)
        || schema
            .mappings
            .consonants
            .as_ref()
            .map(|m| m.keys().any(|k| k.len() > 1))
            .unwrap_or(false)
        || schema
            .mappings
            .special
            .as_ref()
            .map(|m| m.keys().any(|k| k.len() > 1))
            .unwrap_or(false);

    let template_data = json!({
        "struct_name": struct_name,
        "script_name": script_name,
        "is_alphabet": is_alphabet,
        "target_type": schema.target.as_ref().unwrap_or(&"unknown".to_string()),
        "mappings": mappings,
        "has_multi_char_mappings": has_multi_char_mappings,
    });

    handlebars
        .render("token_based_converter", &template_data)
        .map_err(|e| format!("Template rendering failed: {e}").into())
}

/// Generate direct converters for common script pairs to bypass hub overhead
fn generate_direct_converters(
    handlebars: &Handlebars,
    schemas: &[ScriptSchema],
) -> Result<String, Box<dyn std::error::Error>> {
    let mut generated_code = String::new();

    // Add imports and header
    generated_code.push_str(
        r#"
// Auto-generated direct converters - bypass hub for maximum performance
// DO NOT EDIT - Generated by build.rs at compile time

#[allow(unreachable_patterns)]
#[allow(dead_code)]

use crate::modules::script_converter::ConverterError;
use aho_corasick::AhoCorasick;
use once_cell::sync::Lazy;

"#,
    );

    // Common high-performance conversion pairs
    let conversion_pairs = vec![
        // Roman ↔ Roman (very common)
        ("iast", "slp1"),
        ("slp1", "iast"),
        ("iast", "itrans"),
        ("itrans", "iast"),
        ("iast", "harvard_kyoto"),
        ("harvard_kyoto", "iast"),
        // Indic → Roman (performance critical)
        ("devanagari", "iast"),
        ("devanagari", "slp1"),
        ("telugu", "iast"),
        ("telugu", "slp1"),
        ("bengali", "iast"),
        ("tamil", "iast"),
        ("gujarati", "iast"),
        // Roman → Indic (also important)
        ("iast", "devanagari"),
        ("slp1", "devanagari"),
        ("iast", "telugu"),
        ("slp1", "telugu"),
        ("iast", "bengali"),
        ("iast", "tamil"),
        ("iast", "kannada"),
        ("iast", "gujarati"),
        // New Vedic scripts - high priority ones
        ("iast", "grantha"),
        ("iast", "sharada"),
        ("iast", "nandinagari"),
        ("iast", "newa"),
        ("iast", "siddham"),
        ("iast", "modi"),
        ("iast", "bhaiksuki"),
        ("iast", "kaithi"),
        ("iast", "takri"),
        ("iast", "dogra"),
    ];

    // Find schemas by name
    let schema_map: FxHashMap<_, _> = schemas
        .iter()
        .map(|s| (s.metadata.name.as_str(), s))
        .collect();

    for (from_script, to_script) in &conversion_pairs {
        if let (Some(from_schema), Some(to_schema)) =
            (schema_map.get(from_script), schema_map.get(to_script))
        {
            if let Ok(converter_code) =
                generate_single_direct_converter(handlebars, from_schema, to_schema)
            {
                generated_code.push_str(&converter_code);
                generated_code.push('\n');
            }
        }
    }

    // Add registry function to access all direct converters
    generated_code.push_str(r#"
/// Registry of all direct converters for fast lookup
pub struct DirectConverterRegistry {
    converters: std::collections::HashMap<(String, String), Box<dyn DirectConverter>>,
}

impl DirectConverterRegistry {
    pub fn new() -> Self {
        let mut converters: std::collections::HashMap<(String, String), Box<dyn DirectConverter>> = std::collections::HashMap::new();
        
        // Register all generated direct converters
"#);

    // Register each converter
    for (from_script, to_script) in &conversion_pairs {
        if schema_map.contains_key(from_script) && schema_map.contains_key(to_script) {
            let struct_name = format!(
                "{}To{}Converter",
                capitalize_first(from_script),
                capitalize_first(to_script)
            );
            generated_code.push_str(&format!(
                "        converters.insert((\"{}\".to_string(), \"{}\".to_string()), Box::new({}::new()));\n",
                from_script, to_script, struct_name
            ));
        }
    }

    generated_code.push_str(
        r#"        
        Self { converters }
    }
    
    pub fn get_converter(&self, from: &str, to: &str) -> Option<&dyn DirectConverter> {
        self.converters.get(&(from.to_string(), to.to_string()))
            .map(|c| c.as_ref())
    }
}

pub trait DirectConverter: Send + Sync {
    fn convert(&self, input: &str) -> Result<String, ConverterError>;
    fn from_script(&self) -> &'static str;
    fn to_script(&self) -> &'static str;
}
"#,
    );

    Ok(generated_code)
}

/// Generate a single direct converter between two schemas
fn generate_single_direct_converter(
    handlebars: &Handlebars,
    from_schema: &ScriptSchema,
    to_schema: &ScriptSchema,
) -> Result<String, Box<dyn std::error::Error>> {
    let from_script = &from_schema.metadata.name;
    let to_script = &to_schema.metadata.name;

    // Build direct mapping by computing token-to-token conversion at build time
    let mut direct_mappings = Vec::new();

    // Get all mappings from both schemas
    let from_mappings = collect_all_mappings(from_schema);
    let to_mappings = collect_all_mappings(to_schema);

    // Create reverse mapping for target schema (string -> token)
    let mut to_reverse_map = FxHashMap::default();
    for (token, strings) in &to_mappings {
        for string in strings {
            to_reverse_map.insert(token.clone(), string.clone());
        }
    }

    // For each source pattern, find the corresponding target pattern
    for (token, from_strings) in &from_mappings {
        if let Some(to_string) = to_reverse_map.get(token) {
            for from_string in from_strings {
                direct_mappings.push(json!({
                    "from_pattern": from_string,
                    "to_pattern": to_string
                }));
            }
        }
    }

    // Sort by length (longest first) for proper matching
    direct_mappings.sort_by(|a, b| {
        let a_len = a["from_pattern"].as_str().unwrap().len();
        let b_len = b["from_pattern"].as_str().unwrap().len();
        b_len.cmp(&a_len)
    });

    let struct_name = format!(
        "{}To{}Converter",
        capitalize_first(from_script),
        capitalize_first(to_script)
    );

    let template_data = json!({
        "struct_name": struct_name,
        "from_script": from_script,
        "to_script": to_script,
        "direct_mappings": direct_mappings,
    });

    let mut converter_code = handlebars.render("direct_converter", &template_data)?;

    // Add trait implementation
    converter_code.push_str(&format!(
        r#"

impl DirectConverter for {} {{
    fn convert(&self, input: &str) -> Result<String, ConverterError> {{
        self.convert(input)
    }}
    
    fn from_script(&self) -> &'static str {{
        "{}"
    }}
    
    fn to_script(&self) -> &'static str {{
        "{}"
    }}
}}
"#,
        struct_name, from_script, to_script
    ));

    Ok(converter_code)
}

/// Collect all mappings from a schema (token -> [strings])
fn collect_all_mappings(schema: &ScriptSchema) -> FxHashMap<String, Vec<String>> {
    let mut mappings = FxHashMap::default();

    // Process each mapping category
    if let Some(ref vowels) = schema.mappings.vowels {
        for (token, mapping) in vowels {
            let strings = match mapping {
                TokenMapping::Single(s) => vec![s.clone()],
                TokenMapping::Multiple(v) => v.clone(),
            };
            mappings.insert(token.clone(), strings);
        }
    }

    if let Some(ref consonants) = schema.mappings.consonants {
        for (token, mapping) in consonants {
            let strings = match mapping {
                TokenMapping::Single(s) => vec![s.clone()],
                TokenMapping::Multiple(v) => v.clone(),
            };
            mappings.insert(token.clone(), strings);
        }
    }

    if let Some(ref marks) = schema.mappings.marks {
        for (token, mapping) in marks {
            let strings = match mapping {
                TokenMapping::Single(s) => vec![s.clone()],
                TokenMapping::Multiple(v) => v.clone(),
            };
            mappings.insert(token.clone(), strings);
        }
    }

    if let Some(ref digits) = schema.mappings.digits {
        for (token, mapping) in digits {
            let strings = match mapping {
                TokenMapping::Single(s) => vec![s.clone()],
                TokenMapping::Multiple(v) => v.clone(),
            };
            mappings.insert(token.clone(), strings);
        }
    }

    if let Some(ref special) = schema.mappings.special {
        for (token, mapping) in special {
            let strings = match mapping {
                TokenMapping::Single(s) => vec![s.clone()],
                TokenMapping::Multiple(v) => v.clone(),
            };
            mappings.insert(token.clone(), strings);
        }
    }

    if let Some(ref extended) = schema.mappings.extended {
        for (token, mapping) in extended {
            let strings = match mapping {
                TokenMapping::Single(s) => vec![s.clone()],
                TokenMapping::Multiple(v) => v.clone(),
            };
            mappings.insert(token.clone(), strings);
        }
    }

    mappings
}
