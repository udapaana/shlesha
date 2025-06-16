use std::collections::HashMap;
use crate::ir_v2::{AbugidaIR, AlphabetIR, IR, AbugidaAtom, AlphabetAtom, ModifierSet, ScriptId, SchemeId};
use crate::element_id::{ElementId, ElementType, ElementRegistry};
use crate::schema_parser::{Schema, ScriptType, ElementMapping};
use crate::runtime_extension::RuntimeExtensionManager;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unknown script or scheme: {0}")]
    UnknownScript(String),
    
    #[error("Invalid character at position {position}: {character}")]
    InvalidCharacter { character: char, position: usize },
    
    #[error("Incomplete sequence at position {0}")]
    IncompleteSequence(usize),
    
    #[error("Schema error: {0}")]
    SchemaError(String),
    
    #[error("Element registry error: {0}")]
    RegistryError(String),
}

pub struct ParserV2 {
    schemas: HashMap<String, Schema>,
    registry: ElementRegistry,
    extension_manager: RuntimeExtensionManager,
    // Cache for performance
    element_cache: HashMap<String, ElementId>,
    longest_sequence_cache: HashMap<String, usize>,
}

impl ParserV2 {
    pub fn new() -> Self {
        Self {
            schemas: HashMap::new(),
            registry: ElementRegistry::default(),
            extension_manager: RuntimeExtensionManager::new(),
            element_cache: HashMap::new(),
            longest_sequence_cache: HashMap::new(),
        }
    }
    
    pub fn load_schema(&mut self, mut schema: Schema) {
        // Apply any active variants to the schema
        schema = self.extension_manager.apply_to_schema(schema);
        
        // Register all elements in the schema with the registry
        self.register_schema_elements(&schema);
        
        // Store the schema
        self.schemas.insert(schema.name.clone(), schema);
        
        // Clear caches when schema changes
        self.element_cache.clear();
        self.longest_sequence_cache.clear();
    }
    
    pub fn load_variant(&mut self, variant_yaml: &str) -> Result<(), ParseError> {
        self.extension_manager.load_variant_from_yaml(variant_yaml)
            .map_err(|e| ParseError::RegistryError(format!("Variant load error: {}", e)))?;
        
        // Recompute all schemas with new variant
        let schema_names: Vec<String> = self.schemas.keys().cloned().collect();
        for schema_name in schema_names {
            if let Some(schema) = self.schemas.remove(&schema_name) {
                self.load_schema(schema);
            }
        }
        
        Ok(())
    }
    
    pub fn registry(&self) -> &ElementRegistry {
        &self.registry
    }
    
    pub fn parse(&self, input: &str, script_name: &str) -> Result<IR, ParseError> {
        let schema = self.schemas.get(script_name)
            .ok_or_else(|| ParseError::UnknownScript(script_name.to_string()))?;
        
        match schema.script_type {
            ScriptType::Abugida => self.parse_abugida(input, schema),
            ScriptType::Alphabet => self.parse_alphabet(input, schema),
        }
    }
    
    fn register_schema_elements(&mut self, schema: &Schema) {
        for (category, mappings) in &schema.mappings {
            let element_type = self.category_to_element_type(category);
            
            for (grapheme, mapping) in mappings {
                // Determine the actual element type
                let actual_type = if let Some(ref type_name) = mapping.element_type {
                    self.type_name_to_element_type(type_name)
                } else {
                    element_type
                };
                
                // Register with canonical name
                let element_id = self.registry.register(actual_type, mapping.canonical.clone());
                
                // Cache the grapheme -> element_id mapping
                self.element_cache.insert(grapheme.clone(), element_id);
            }
        }
    }
    
    fn parse_abugida(&self, input: &str, schema: &Schema) -> Result<IR, ParseError> {
        let script_id = self.script_name_to_id(&schema.name);
        let mut ir = AbugidaIR::new(script_id);
        
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            let (matched_length, element_id, grapheme) = self.find_longest_match(&chars, i, schema)?;
            
            if matched_length > 0 {
                // Check if this is a modifier that should be applied to the previous element
                if self.is_modifier_element(element_id) && !ir.elements.is_empty() {
                    // Apply modifier to the last element
                    let last_idx = ir.elements.len() - 1;
                    ir.elements[last_idx].modifiers = ir.elements[last_idx].modifiers.with_modifier(element_id);
                } else {
                    // Add as new element
                    ir.push(element_id, grapheme);
                }
                i += matched_length;
            } else {
                // Handle unknown character
                let ch = chars[i];
                if ch.is_whitespace() {
                    let whitespace_id = self.get_or_create_whitespace_element();
                    ir.push(whitespace_id, ch.to_string());
                } else {
                    let unknown_id = self.get_or_create_unknown_element();
                    ir.push(unknown_id, ch.to_string());
                }
                i += 1;
            }
        }
        
        Ok(IR::Abugida(ir))
    }
    
    fn parse_alphabet(&self, input: &str, schema: &Schema) -> Result<IR, ParseError> {
        let scheme_id = self.scheme_name_to_id(&schema.name);
        let mut ir = AlphabetIR::new(scheme_id);
        
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            let (matched_length, element_id, grapheme) = self.find_longest_match(&chars, i, schema)?;
            
            if matched_length > 0 {
                ir.push(element_id, grapheme);
                i += matched_length;
            } else {
                // Handle unknown character
                let ch = chars[i];
                if ch.is_whitespace() {
                    let whitespace_id = self.get_or_create_whitespace_element();
                    ir.push(whitespace_id, ch.to_string());
                } else if ch.is_ascii_punctuation() {
                    let punct_id = self.get_or_create_punctuation_element();
                    ir.push(punct_id, ch.to_string());
                } else {
                    let unknown_id = self.get_or_create_unknown_element();
                    ir.push(unknown_id, ch.to_string());
                }
                i += 1;
            }
        }
        
        Ok(IR::Alphabet(ir))
    }
    
    fn find_longest_match(&self, chars: &[char], start: usize, schema: &Schema) -> Result<(usize, ElementId, String), ParseError> {
        let mut best_match = (0, ElementId::new(ElementType::Unknown, 0), String::new());
        
        // Try sequences of length 1 to 6 (covers most complex conjuncts)
        for len in 1..=6.min(chars.len() - start) {
            let sequence: String = chars[start..start + len].iter().collect();
            
            // Check all mapping categories for this sequence
            for (_, mappings) in &schema.mappings {
                if let Some(mapping) = mappings.get(&sequence) {
                    if let Some(&element_id) = self.element_cache.get(&sequence) {
                        best_match = (len, element_id, sequence.clone());
                        // Continue to find longest match
                    }
                }
            }
        }
        
        Ok(best_match)
    }
    
    fn is_modifier_element(&self, element_id: ElementId) -> bool {
        element_id.is_modifier()
    }
    
    fn get_or_create_whitespace_element(&self) -> ElementId {
        // Use a well-known whitespace element ID
        ElementId::new(ElementType::Whitespace, 0)
    }
    
    fn get_or_create_punctuation_element(&self) -> ElementId {
        ElementId::new(ElementType::Punctuation, 0)
    }
    
    fn get_or_create_unknown_element(&self) -> ElementId {
        ElementId::new(ElementType::Unknown, 0)
    }
    
    fn category_to_element_type(&self, category: &str) -> ElementType {
        match category {
            "consonants" => ElementType::Consonant,
            "vowels" => ElementType::Vowel,
            "vowels_independent" => ElementType::VowelIndependent,
            "vowels_dependent" => ElementType::VowelDependent,
            "modifiers" => ElementType::Modifier,
            "accents" => ElementType::Accent,
            "numerals" => ElementType::Numeral,
            "punctuation" => ElementType::Punctuation,
            _ => ElementType::Extension,
        }
    }
    
    fn type_name_to_element_type(&self, type_name: &str) -> ElementType {
        match type_name {
            "consonant" => ElementType::Consonant,
            "vowel" => ElementType::Vowel,
            "vowel_independent" => ElementType::VowelIndependent,
            "vowel_dependent" => ElementType::VowelDependent,
            "virama" => ElementType::Virama,
            "nukta" => ElementType::Nukta,
            "anusvara" => ElementType::Anusvara,
            "visarga" => ElementType::Visarga,
            "avagraha" => ElementType::Avagraha,
            "accent" => ElementType::Accent,
            "accent_udatta" => ElementType::AccentUdatta,
            "accent_anudatta" => ElementType::AccentAnudatta,
            "accent_svarita" => ElementType::AccentSvarita,
            "numeral" => ElementType::Numeral,
            "punctuation" => ElementType::Punctuation,
            "whitespace" => ElementType::Whitespace,
            "extension" => ElementType::Extension,
            _ => ElementType::Extension,
        }
    }
    
    fn script_name_to_id(&self, name: &str) -> ScriptId {
        match name {
            "Devanagari" => ScriptId::DEVANAGARI,
            "Telugu" => ScriptId::TELUGU,
            "Tamil" => ScriptId::TAMIL,
            "Kannada" => ScriptId::KANNADA,
            "Malayalam" => ScriptId::MALAYALAM,
            "Bengali" => ScriptId::BENGALI,
            "Gujarati" => ScriptId::GUJARATI,
            _ => ScriptId::new(999), // Unknown script
        }
    }
    
    fn scheme_name_to_id(&self, name: &str) -> SchemeId {
        match name {
            "IAST" => SchemeId::IAST,
            "Harvard-Kyoto" => SchemeId::HARVARD_KYOTO,
            "SLP1" => SchemeId::SLP1,
            "ISO15919" => SchemeId::ISO15919,
            "ITRANS" => SchemeId::ITRANS,
            _ => SchemeId::new(999), // Unknown scheme
        }
    }
}

pub struct ParserBuilder {
    parser: ParserV2,
}

impl ParserBuilder {
    pub fn new() -> Self {
        Self {
            parser: ParserV2::new(),
        }
    }
    
    pub fn with_schema(mut self, schema: Schema) -> Self {
        self.parser.load_schema(schema);
        self
    }
    
    pub fn with_schemas(mut self, schemas: Vec<Schema>) -> Self {
        for schema in schemas {
            self.parser.load_schema(schema);
        }
        self
    }
    
    pub fn with_variant_file(mut self, path: &str) -> Result<Self, ParseError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| ParseError::SchemaError(format!("Failed to read variant file: {}", e)))?;
        self.parser.load_variant(&content)?;
        Ok(self)
    }
    
    pub fn build(self) -> ParserV2 {
        self.parser
    }
}

impl Default for ParserV2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema_parser::SchemaParser;
    
    #[test]
    fn test_parser_v2_basic() {
        let schema_yaml = r#"
name: "Devanagari"
type: abugida

mappings:
  consonants:
    "क":
      canonical: "ka"
    "ख":
      canonical: "kha"
  vowels:
    "अ":
      canonical: "a"
      type: vowel_independent
    "ि":
      canonical: "i"
      type: vowel_dependent
  modifiers:
    "्":
      canonical: "virama"
      type: virama
"#;
        
        let schema = SchemaParser::parse_str(schema_yaml).unwrap();
        let parser = ParserBuilder::new()
            .with_schema(schema)
            .build();
        
        let result = parser.parse("कि", "Devanagari").unwrap();
        
        match result {
            IR::Abugida(ir) => {
                assert_eq!(ir.elements.len(), 2);
                // First element should be "क" (ka)
                assert_eq!(ir.get_grapheme(&ir.elements[0]), "क");
                // Second element should be "ि" (i)
                assert_eq!(ir.get_grapheme(&ir.elements[1]), "ि");
            }
            _ => panic!("Expected Abugida IR"),
        }
    }
    
    #[test]
    fn test_parser_v2_with_modifiers() {
        let schema_yaml = r#"
name: "Devanagari"
type: abugida

mappings:
  consonants:
    "क":
      canonical: "ka"
  modifiers:
    "्":
      canonical: "virama"
      type: virama
    "़":
      canonical: "nukta"
      type: nukta
"#;
        
        let schema = SchemaParser::parse_str(schema_yaml).unwrap();
        let parser = ParserBuilder::new()
            .with_schema(schema)
            .build();
        
        let result = parser.parse("क़्", "Devanagari").unwrap();
        
        match result {
            IR::Abugida(ir) => {
                assert_eq!(ir.elements.len(), 1); // क with modifiers applied
                let element = &ir.elements[0];
                assert_eq!(ir.get_grapheme(element), "क");
                // Check that modifiers were applied (this would need the actual ElementIds)
                assert!(!element.modifiers.is_empty());
            }
            _ => panic!("Expected Abugida IR"),
        }
    }
    
    #[test]
    fn test_parser_v2_longest_match() {
        let schema_yaml = r#"
name: "Devanagari"
type: abugida

mappings:
  consonants:
    "क":
      canonical: "ka"
    "क्ष":
      canonical: "kṣa"
    "ष":
      canonical: "ṣa"
"#;
        
        let schema = SchemaParser::parse_str(schema_yaml).unwrap();
        let parser = ParserBuilder::new()
            .with_schema(schema)
            .build();
        
        let result = parser.parse("क्ष", "Devanagari").unwrap();
        
        match result {
            IR::Abugida(ir) => {
                assert_eq!(ir.elements.len(), 1); // Should match "क्ष" as single unit
                assert_eq!(ir.get_grapheme(&ir.elements[0]), "क्ष");
            }
            _ => panic!("Expected Abugida IR"),
        }
    }
    
    #[test]
    fn test_parser_v2_alphabet() {
        let schema_yaml = r#"
name: "IAST"
type: alphabet

mappings:
  consonants:
    "k":
      canonical: "k"
    "kh":
      canonical: "kh"
  vowels:
    "a":
      canonical: "a"
    "i":
      canonical: "i"
"#;
        
        let schema = SchemaParser::parse_str(schema_yaml).unwrap();
        let parser = ParserBuilder::new()
            .with_schema(schema)
            .build();
        
        let result = parser.parse("kha", "IAST").unwrap();
        
        match result {
            IR::Alphabet(ir) => {
                assert_eq!(ir.elements.len(), 2); // "kh" + "a"
                assert_eq!(ir.get_grapheme(&ir.elements[0]), "kh");
                assert_eq!(ir.get_grapheme(&ir.elements[1]), "a");
            }
            _ => panic!("Expected Alphabet IR"),
        }
    }
    
    #[test]
    fn test_parser_v2_with_runtime_variant() {
        let schema_yaml = r#"
name: "Devanagari"
type: abugida

mappings:
  consonants:
    "क":
      canonical: "ka"
"#;
        
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
properties: {}
manuscript_sources: ["Test MS"]
created_by: "test"
created_at: "2024-01-01T00:00:00Z"
"#;
        
        let schema = SchemaParser::parse_str(schema_yaml).unwrap();
        let mut parser = ParserBuilder::new()
            .with_schema(schema)
            .build();
        
        // Load the variant
        parser.load_variant(variant_yaml).unwrap();
        
        // Now the parser should recognize क़
        let result = parser.parse("क़", "Devanagari").unwrap();
        
        match result {
            IR::Abugida(ir) => {
                assert_eq!(ir.elements.len(), 1);
                assert_eq!(ir.get_grapheme(&ir.elements[0]), "क़");
            }
            _ => panic!("Expected Abugida IR"),
        }
    }
}