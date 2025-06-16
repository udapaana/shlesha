use std::collections::HashMap;
use crate::ir::{Element, ElementType, AbugidaIR, AlphabetIR, IR};
use crate::schema_parser::{Schema, ScriptType, ElementMapping, SchemaParser};
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
}

pub struct Parser {
    schemas: HashMap<String, Schema>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            schemas: HashMap::new(),
        }
    }
    
    pub fn load_schema(&mut self, schema: Schema) {
        self.schemas.insert(schema.name.clone(), schema);
    }
    
    pub fn parse(&self, input: &str, script_name: &str) -> Result<IR, ParseError> {
        let schema = self.schemas.get(script_name)
            .ok_or_else(|| ParseError::UnknownScript(script_name.to_string()))?;
        
        match schema.script_type {
            ScriptType::Abugida => self.parse_abugida(input, schema),
            ScriptType::Alphabet => self.parse_alphabet(input, schema),
        }
    }
    
    fn parse_abugida(&self, input: &str, schema: &Schema) -> Result<IR, ParseError> {
        let mut ir = AbugidaIR::new(schema.name.clone());
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            let mut matched = false;
            
            // Try to match longest sequences first (up to 4 characters for complex ligatures)
            for len in (1..=4).rev() {
                if i + len > chars.len() {
                    continue;
                }
                
                let sequence: String = chars[i..i + len].iter().collect();
                
                // Check all mapping categories
                for (category, mappings) in &schema.mappings {
                    if let Some(mapping) = mappings.get(&sequence) {
                        let element = self.create_element_from_mapping(&sequence, mapping, category);
                        ir.push(element);
                        i += len;
                        matched = true;
                        break;
                    }
                }
                
                if matched {
                    break;
                }
            }
            
            if !matched {
                // Handle whitespace and unknown characters
                let ch = chars[i];
                if ch.is_whitespace() {
                    ir.push(Element::new(
                        ElementType::WHITESPACE,
                        ch.to_string(),
                        ch.to_string()
                    ));
                } else {
                    ir.push(Element::new(
                        ElementType::UNKNOWN,
                        ch.to_string(),
                        ch.to_string()
                    ));
                }
                i += 1;
            }
        }
        
        Ok(IR::Abugida(ir))
    }
    
    fn parse_alphabet(&self, input: &str, schema: &Schema) -> Result<IR, ParseError> {
        let mut ir = AlphabetIR::new(schema.name.clone());
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            let mut matched = false;
            
            // For alphabets, try longer sequences first (for digraphs like 'kh', 'ch', etc.)
            for len in (1..=4).rev() {
                if i + len > chars.len() {
                    continue;
                }
                
                let sequence: String = chars[i..i + len].iter().collect();
                
                // Check all mapping categories
                for (category, mappings) in &schema.mappings {
                    if let Some(mapping) = mappings.get(&sequence) {
                        let element = self.create_element_from_mapping(&sequence, mapping, category);
                        ir.push(element);
                        i += len;
                        matched = true;
                        break;
                    }
                }
                
                if matched {
                    break;
                }
            }
            
            if !matched {
                let ch = chars[i];
                if ch.is_whitespace() {
                    ir.push(Element::new(
                        ElementType::WHITESPACE,
                        ch.to_string(),
                        ch.to_string()
                    ));
                } else if ch.is_ascii_punctuation() {
                    ir.push(Element::new(
                        ElementType::PUNCTUATION,
                        ch.to_string(),
                        ch.to_string()
                    ));
                } else {
                    ir.push(Element::new(
                        ElementType::UNKNOWN,
                        ch.to_string(),
                        ch.to_string()
                    ));
                }
                i += 1;
            }
        }
        
        Ok(IR::Alphabet(ir))
    }
    
    fn create_element_from_mapping(
        &self,
        grapheme: &str,
        mapping: &ElementMapping,
        category: &str
    ) -> Element {
        let element_type = mapping.element_type.as_ref()
            .map(|t| t.clone())
            .unwrap_or_else(|| self.infer_element_type(category));
        
        let mut element = Element::new(
            element_type,
            grapheme,
            &mapping.canonical
        );
        
        // Add all properties from the mapping
        for (key, value) in &mapping.properties {
            if key != "canonical" && key != "type" {
                element.properties.insert(
                    key.clone(),
                    SchemaParser::yaml_value_to_property_value(value)
                );
            }
        }
        
        element
    }
    
    fn infer_element_type(&self, category: &str) -> String {
        match category {
            "consonants" => ElementType::CONSONANT.to_string(),
            "vowels" => ElementType::VOWEL.to_string(),
            "modifiers" => ElementType::MODIFIER.to_string(),
            "numerals" => ElementType::NUMERAL.to_string(),
            "punctuation" => ElementType::PUNCTUATION.to_string(),
            "accents" => ElementType::ACCENT.to_string(),
            _ => ElementType::UNKNOWN.to_string(),
        }
    }
}

pub struct ParserBuilder {
    parser: Parser,
}

impl ParserBuilder {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
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
    
    pub fn build(self) -> Parser {
        self.parser
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema_parser::SchemaParser;
    
    #[test]
    fn test_parse_devanagari() {
        let schema_yaml = r#"
name: "Devanagari"
type: abugida

mappings:
  consonants:
    "क":
      canonical: "ka"
      has_inherent_vowel: true
    "ख":
      canonical: "kha"
      has_inherent_vowel: true
  vowels:
    "अ":
      canonical: "a"
      type: vowel_independent
    "ि":
      canonical: "i"
      type: vowel_dependent
  modifiers:
    "्":
      canonical: ""
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
                assert_eq!(ir.elements[0].canonical, "ka");
                assert_eq!(ir.elements[1].canonical, "i");
            }
            _ => panic!("Expected Abugida IR"),
        }
    }
    
    #[test]
    fn test_parse_iast() {
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
    "ā":
      canonical: "ā"
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
                assert_eq!(ir.elements.len(), 2);
                assert_eq!(ir.elements[0].canonical, "kh");
                assert_eq!(ir.elements[1].canonical, "a");
            }
            _ => panic!("Expected Alphabet IR"),
        }
    }
}