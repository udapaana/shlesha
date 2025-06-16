use shlesha::ir_v2::IR;
use shlesha::parser_v2::{ParserV2, ParserBuilder};
use shlesha::transformer_v2::{TransformerV2, TransformerBuilder, ScriptType};
use shlesha::generator_v2::GeneratorV2;
use shlesha::schema_parser::{Schema, SchemaParser, ElementMapping};
use shlesha::runtime_extension::RuntimeExtensionManager;
use std::collections::HashMap;

#[cfg(test)]
mod pipeline_integration_tests {
    use super::*;

    #[test]
    fn test_complete_pipeline_devanagari_to_iast() {
        // Create schemas
        let devanagari_schema = create_test_devanagari_schema();
        let iast_schema = create_test_iast_schema();
        
        // Set up parser
        let parser = ParserBuilder::new()
            .with_schema(devanagari_schema)
            .with_schema(iast_schema.clone())
            .build();
        
        // Set up transformer
        let transformer = TransformerBuilder::new()
            .with_registry(parser.registry().clone())
            .build();
        
        // Set up generator
        let mut generator = GeneratorV2::new(parser.registry().clone());
        generator.load_schema(iast_schema);
        
        // Test input: "कर्म" (karma)
        let input = "कर्म";
        
        // Step 1: Parse Devanagari
        let devanagari_ir = parser.parse(input, "Devanagari").unwrap();
        
        // Step 2: Transform to alphabet
        let alphabet_ir = transformer.transform(devanagari_ir, ScriptType::Alphabet).unwrap();
        
        // Step 3: Generate IAST
        let output = generator.generate(&alphabet_ir, "IAST").unwrap();
        
        assert_eq!(output, "karma");
    }
    
    #[test]
    fn test_complete_pipeline_iast_to_devanagari() {
        // Create schemas
        let devanagari_schema = create_test_devanagari_schema();
        let iast_schema = create_test_iast_schema();
        
        // Set up parser
        let parser = ParserBuilder::new()
            .with_schema(devanagari_schema.clone())
            .with_schema(iast_schema)
            .build();
        
        // Set up transformer
        let transformer = TransformerBuilder::new()
            .with_registry(parser.registry().clone())
            .build();
        
        // Set up generator
        let mut generator = GeneratorV2::new(parser.registry().clone());
        generator.load_schema(devanagari_schema);
        
        // Test input: "karma"
        let input = "karma";
        
        // Step 1: Parse IAST
        let iast_ir = parser.parse(input, "IAST").unwrap();
        
        // Step 2: Transform to abugida
        let abugida_ir = transformer.transform(iast_ir, ScriptType::Abugida).unwrap();
        
        // Step 3: Generate Devanagari
        let output = generator.generate(&abugida_ir, "Devanagari").unwrap();
        
        assert_eq!(output, "कर्म");
    }
    
    #[test]
    fn test_round_trip_transliteration() {
        // Create schemas
        let devanagari_schema = create_test_devanagari_schema();
        let iast_schema = create_test_iast_schema();
        
        // Set up parser
        let parser = ParserBuilder::new()
            .with_schema(devanagari_schema.clone())
            .with_schema(iast_schema.clone())
            .build();
        
        // Set up transformer
        let transformer = TransformerBuilder::new()
            .with_registry(parser.registry().clone())
            .build();
        
        // Set up generator
        let mut generator = GeneratorV2::new(parser.registry().clone());
        generator.load_schema(devanagari_schema);
        generator.load_schema(iast_schema);
        
        let original_text = "धर्म";
        
        // Round trip: Devanagari → IAST → Devanagari
        let step1_ir = parser.parse(original_text, "Devanagari").unwrap();
        let step2_ir = transformer.transform(step1_ir, ScriptType::Alphabet).unwrap();
        let iast_text = generator.generate(&step2_ir, "IAST").unwrap();
        
        let step3_ir = parser.parse(&iast_text, "IAST").unwrap();
        let step4_ir = transformer.transform(step3_ir, ScriptType::Abugida).unwrap();
        let final_text = generator.generate(&step4_ir, "Devanagari").unwrap();
        
        assert_eq!(final_text, original_text);
    }
    
    #[test]
    fn test_runtime_variant_integration() {
        // Create base schemas
        let devanagari_schema = create_test_devanagari_schema();
        let iast_schema = create_test_iast_schema();
        
        // Set up parser
        let mut parser = ParserBuilder::new()
            .with_schema(devanagari_schema.clone())
            .with_schema(iast_schema.clone())
            .build();
        
        // Add runtime variant for "qa" sound
        let variant_yaml = r#"
name: "qa_variant"
description: "Arabic qa sound"
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
  other_scripts: {}
properties: {}
manuscript_sources: ["Test MS"]
created_by: "test"
created_at: "2024-01-01T00:00:00Z"
"#;
        
        // Load the variant
        parser.load_variant(variant_yaml).unwrap();
        
        // Set up transformer and generator
        let transformer = TransformerBuilder::new()
            .with_registry(parser.registry().clone())
            .build();
        
        let mut generator = GeneratorV2::new(parser.registry().clone());
        generator.load_schema(devanagari_schema);
        generator.load_schema(iast_schema);
        
        // Test the variant: "क़" should transliterate to "qa"
        let devanagari_ir = parser.parse("क़", "Devanagari").unwrap();
        let alphabet_ir = transformer.transform(devanagari_ir, ScriptType::Alphabet).unwrap();
        let output = generator.generate(&alphabet_ir, "IAST").unwrap();
        
        assert_eq!(output, "qa");
    }
    
    #[test]
    fn test_complex_conjunct_processing() {
        // Create schemas
        let devanagari_schema = create_extended_devanagari_schema();
        let iast_schema = create_test_iast_schema();
        
        // Set up pipeline
        let parser = ParserBuilder::new()
            .with_schema(devanagari_schema.clone())
            .with_schema(iast_schema.clone())
            .build();
        
        let transformer = TransformerBuilder::new()
            .with_registry(parser.registry().clone())
            .build();
        
        let mut generator = GeneratorV2::new(parser.registry().clone());
        generator.load_schema(iast_schema);
        
        // Test complex conjunct: "क्ष" (kṣa)
        let input = "क्ष";
        
        let devanagari_ir = parser.parse(input, "Devanagari").unwrap();
        let alphabet_ir = transformer.transform(devanagari_ir, ScriptType::Alphabet).unwrap();
        let output = generator.generate(&alphabet_ir, "IAST").unwrap();
        
        assert_eq!(output, "kṣa");
    }
    
    #[test]
    fn test_virama_handling_in_pipeline() {
        // Create schemas
        let devanagari_schema = create_test_devanagari_schema();
        let iast_schema = create_test_iast_schema();
        
        // Set up pipeline
        let parser = ParserBuilder::new()
            .with_schema(devanagari_schema.clone())
            .with_schema(iast_schema.clone())
            .build();
        
        let transformer = TransformerBuilder::new()
            .with_registry(parser.registry().clone())
            .build();
        
        let mut generator = GeneratorV2::new(parser.registry().clone());
        generator.load_schema(iast_schema);
        
        // Test virama: "क्" (k with virama)
        let input = "क्";
        
        let devanagari_ir = parser.parse(input, "Devanagari").unwrap();
        let alphabet_ir = transformer.transform(devanagari_ir, ScriptType::Alphabet).unwrap();
        let output = generator.generate(&alphabet_ir, "IAST").unwrap();
        
        // Should output just "k" (no inherent vowel due to virama)
        assert_eq!(output, "k");
    }
    
    #[test]
    fn test_dependent_vowel_processing() {
        // Create schemas
        let devanagari_schema = create_test_devanagari_schema();
        let iast_schema = create_test_iast_schema();
        
        // Set up pipeline
        let parser = ParserBuilder::new()
            .with_schema(devanagari_schema.clone())
            .with_schema(iast_schema.clone())
            .build();
        
        let transformer = TransformerBuilder::new()
            .with_registry(parser.registry().clone())
            .build();
        
        let mut generator = GeneratorV2::new(parser.registry().clone());
        generator.load_schema(iast_schema);
        
        // Test dependent vowel: "कि" (ki)
        let input = "कि";
        
        let devanagari_ir = parser.parse(input, "Devanagari").unwrap();
        let alphabet_ir = transformer.transform(devanagari_ir, ScriptType::Alphabet).unwrap();
        let output = generator.generate(&alphabet_ir, "IAST").unwrap();
        
        assert_eq!(output, "ki");
    }
    
    #[test]
    fn test_performance_with_large_text() {
        // Create schemas
        let devanagari_schema = create_test_devanagari_schema();
        let iast_schema = create_test_iast_schema();
        
        // Set up pipeline
        let parser = ParserBuilder::new()
            .with_schema(devanagari_schema.clone())
            .with_schema(iast_schema.clone())
            .build();
        
        let transformer = TransformerBuilder::new()
            .with_registry(parser.registry().clone())
            .build();
        
        let mut generator = GeneratorV2::new(parser.registry().clone());
        generator.load_schema(iast_schema);
        
        // Create large text (repeated "कर्म")
        let input = "कर्म ".repeat(1000);
        
        let start_time = std::time::Instant::now();
        
        let devanagari_ir = parser.parse(&input, "Devanagari").unwrap();
        let alphabet_ir = transformer.transform(devanagari_ir, ScriptType::Alphabet).unwrap();
        let output = generator.generate(&alphabet_ir, "IAST").unwrap();
        
        let duration = start_time.elapsed();
        
        // Should complete in reasonable time
        assert!(duration.as_millis() < 1000); // Less than 1 second
        
        // Output should be correct
        assert!(output.starts_with("karma "));
        assert!(output.len() > 5000); // Should be a long string
    }
    
    #[test]
    fn test_batch_generation() {
        // Create schemas
        let devanagari_schema = create_test_devanagari_schema();
        let iast_schema = create_test_iast_schema();
        let harvard_kyoto_schema = create_test_harvard_kyoto_schema();
        
        // Set up pipeline
        let parser = ParserBuilder::new()
            .with_schema(devanagari_schema.clone())
            .with_schema(iast_schema.clone())
            .with_schema(harvard_kyoto_schema.clone())
            .build();
        
        let transformer = TransformerBuilder::new()
            .with_registry(parser.registry().clone())
            .build();
        
        let mut generator = GeneratorV2::new(parser.registry().clone());
        generator.load_schema(iast_schema);
        generator.load_schema(harvard_kyoto_schema);
        
        // Parse Devanagari
        let input = "धर्म";
        let devanagari_ir = parser.parse(input, "Devanagari").unwrap();
        let alphabet_ir = transformer.transform(devanagari_ir, ScriptType::Alphabet).unwrap();
        
        // Generate to multiple targets
        let results = generator.generate_batch(&alphabet_ir, &["IAST", "Harvard-Kyoto"]).unwrap();
        
        assert_eq!(results.len(), 2);
        assert_eq!(results["IAST"], "dharma");
        assert_eq!(results["Harvard-Kyoto"], "dharma"); // Simplified for test
    }
}

// Helper functions to create test schemas
fn create_test_devanagari_schema() -> Schema {
    let schema_yaml = r#"
name: "Devanagari"
type: abugida

mappings:
  consonants:
    "क":
      canonical: "ka"
    "ख":
      canonical: "kha"
    "ग":
      canonical: "ga"
    "घ":
      canonical: "gha"
    "च":
      canonical: "ca"
    "छ":
      canonical: "cha"
    "ज":
      canonical: "ja"
    "झ":
      canonical: "jha"
    "ट":
      canonical: "ṭa"
    "ठ":
      canonical: "ṭha"
    "ड":
      canonical: "ḍa"
    "ढ":
      canonical: "ḍha"
    "त":
      canonical: "ta"
    "थ":
      canonical: "tha"
    "द":
      canonical: "da"
    "ध":
      canonical: "dha"
    "न":
      canonical: "na"
    "प":
      canonical: "pa"
    "फ":
      canonical: "pha"
    "ब":
      canonical: "ba"
    "भ":
      canonical: "bha"
    "म":
      canonical: "ma"
    "य":
      canonical: "ya"
    "र":
      canonical: "ra"
    "ल":
      canonical: "la"
    "व":
      canonical: "va"
    "श":
      canonical: "śa"
    "ष":
      canonical: "ṣa"
    "स":
      canonical: "sa"
    "ह":
      canonical: "ha"
  vowels_independent:
    "अ":
      canonical: "a"
    "आ":
      canonical: "ā"
    "इ":
      canonical: "i"
    "ई":
      canonical: "ī"
    "उ":
      canonical: "u"
    "ऊ":
      canonical: "ū"
    "ऋ":
      canonical: "ṛ"
    "ॠ":
      canonical: "ṝ"
    "ए":
      canonical: "e"
    "ऐ":
      canonical: "ai"
    "ओ":
      canonical: "o"
    "औ":
      canonical: "au"
  vowels_dependent:
    "ा":
      canonical: "ā"
      type: vowel_dependent
    "ि":
      canonical: "i"
      type: vowel_dependent
    "ी":
      canonical: "ī"
      type: vowel_dependent
    "ु":
      canonical: "u"
      type: vowel_dependent
    "ू":
      canonical: "ū"
      type: vowel_dependent
    "ृ":
      canonical: "ṛ"
      type: vowel_dependent
    "ॄ":
      canonical: "ṝ"
      type: vowel_dependent
    "े":
      canonical: "e"
      type: vowel_dependent
    "ै":
      canonical: "ai"
      type: vowel_dependent
    "ो":
      canonical: "o"
      type: vowel_dependent
    "ौ":
      canonical: "au"
      type: vowel_dependent
  modifiers:
    "्":
      canonical: "virama"
      type: virama
    "़":
      canonical: "nukta"
      type: nukta
    "ं":
      canonical: "anusvara"
      type: anusvara
    "ः":
      canonical: "visarga"
      type: visarga
"#;
    
    SchemaParser::parse_str(schema_yaml).unwrap()
}

fn create_test_iast_schema() -> Schema {
    let schema_yaml = r#"
name: "IAST"
type: alphabet

mappings:
  consonants:
    "k":
      canonical: "k"
    "kh":
      canonical: "kh"
    "g":
      canonical: "g"
    "gh":
      canonical: "gh"
    "c":
      canonical: "c"
    "ch":
      canonical: "ch"
    "j":
      canonical: "j"
    "jh":
      canonical: "jh"
    "ṭ":
      canonical: "ṭ"
    "ṭh":
      canonical: "ṭh"
    "ḍ":
      canonical: "ḍ"
    "ḍh":
      canonical: "ḍh"
    "t":
      canonical: "t"
    "th":
      canonical: "th"
    "d":
      canonical: "d"
    "dh":
      canonical: "dh"
    "n":
      canonical: "n"
    "p":
      canonical: "p"
    "ph":
      canonical: "ph"
    "b":
      canonical: "b"
    "bh":
      canonical: "bh"
    "m":
      canonical: "m"
    "y":
      canonical: "y"
    "r":
      canonical: "r"
    "l":
      canonical: "l"
    "v":
      canonical: "v"
    "ś":
      canonical: "ś"
    "ṣ":
      canonical: "ṣ"
    "s":
      canonical: "s"
    "h":
      canonical: "h"
  vowels:
    "a":
      canonical: "a"
    "ā":
      canonical: "ā"
    "i":
      canonical: "i"
    "ī":
      canonical: "ī"
    "u":
      canonical: "u"
    "ū":
      canonical: "ū"
    "ṛ":
      canonical: "ṛ"
    "ṝ":
      canonical: "ṝ"
    "e":
      canonical: "e"
    "ai":
      canonical: "ai"
    "o":
      canonical: "o"
    "au":
      canonical: "au"
  modifiers:
    "ṃ":
      canonical: "anusvara"
    "ḥ":
      canonical: "visarga"
"#;
    
    SchemaParser::parse_str(schema_yaml).unwrap()
}

fn create_extended_devanagari_schema() -> Schema {
    let mut schema = create_test_devanagari_schema();
    
    // Add complex conjuncts
    schema.mappings.get_mut("consonants").unwrap().insert(
        "क्ष".to_string(),
        ElementMapping {
            canonical: "kṣa".to_string(),
            element_type: Some("consonant".to_string()),
            properties: HashMap::new(),
        }
    );
    
    schema.mappings.get_mut("consonants").unwrap().insert(
        "ज्ञ".to_string(),
        ElementMapping {
            canonical: "jña".to_string(),
            element_type: Some("consonant".to_string()),
            properties: HashMap::new(),
        }
    );
    
    schema
}

fn create_test_harvard_kyoto_schema() -> Schema {
    let schema_yaml = r#"
name: "Harvard-Kyoto"
type: alphabet

mappings:
  consonants:
    "k":
      canonical: "k"
    "kh":
      canonical: "kh"
    "g":
      canonical: "g"
    "gh":
      canonical: "gh"
    "c":
      canonical: "c"
    "ch":
      canonical: "ch"
    "j":
      canonical: "j"
    "jh":
      canonical: "jh"
    "T":
      canonical: "ṭ"
    "Th":
      canonical: "ṭh"
    "D":
      canonical: "ḍ"
    "Dh":
      canonical: "ḍh"
    "t":
      canonical: "t"
    "th":
      canonical: "th"
    "d":
      canonical: "d"
    "dh":
      canonical: "dh"
    "n":
      canonical: "n"
    "p":
      canonical: "p"
    "ph":
      canonical: "ph"
    "b":
      canonical: "b"
    "bh":
      canonical: "bh"
    "m":
      canonical: "m"
    "y":
      canonical: "y"
    "r":
      canonical: "r"
    "l":
      canonical: "l"
    "v":
      canonical: "v"
    "z":
      canonical: "ś"
    "S":
      canonical: "ṣ"
    "s":
      canonical: "s"
    "h":
      canonical: "h"
  vowels:
    "a":
      canonical: "a"
    "A":
      canonical: "ā"
    "i":
      canonical: "i"
    "I":
      canonical: "ī"
    "u":
      canonical: "u"
    "U":
      canonical: "ū"
    "R":
      canonical: "ṛ"
    "RR":
      canonical: "ṝ"
    "e":
      canonical: "e"
    "ai":
      canonical: "ai"
    "o":
      canonical: "o"
    "au":
      canonical: "au"
  modifiers:
    "M":
      canonical: "anusvara"
    "H":
      canonical: "visarga"
"#;
    
    SchemaParser::parse_str(schema_yaml).unwrap()
}