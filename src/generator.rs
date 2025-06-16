use std::collections::HashMap;
use crate::ir::{ElementType, AbugidaIR, AlphabetIR, IR};
use crate::schema_parser::{Schema, ScriptType};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GenerateError {
    #[error("Unknown target script: {0}")]
    UnknownScript(String),
    
    #[error("Missing mapping for element: {0}")]
    MissingMapping(String),
    
    #[error("Invalid IR for target script")]
    InvalidIR,
}

pub struct Generator {
    schemas: HashMap<String, Schema>,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            schemas: HashMap::new(),
        }
    }
    
    pub fn load_schema(&mut self, schema: Schema) {
        self.schemas.insert(schema.name.clone(), schema);
    }
    
    pub fn generate(&self, ir: &IR, target_script: &str) -> Result<String, GenerateError> {
        let schema = self.schemas.get(target_script)
            .ok_or_else(|| GenerateError::UnknownScript(target_script.to_string()))?;
        
        match (&ir, &schema.script_type) {
            (IR::Abugida(abugida), ScriptType::Abugida) => {
                self.generate_abugida(abugida, schema)
            }
            (IR::Alphabet(alphabet), ScriptType::Alphabet) => {
                self.generate_alphabet(alphabet, schema)
            }
            _ => Err(GenerateError::InvalidIR),
        }
    }
    
    fn generate_abugida(&self, ir: &AbugidaIR, schema: &Schema) -> Result<String, GenerateError> {
        let mut result = String::new();
        let reverse_mappings = self.build_reverse_mappings(schema);
        
        for element in &ir.elements {
            match element.element_type.0.as_str() {
                ElementType::WHITESPACE => {
                    result.push_str(&element.grapheme);
                }
                ElementType::UNKNOWN => {
                    // Mark unknown elements
                    result.push_str(&format!("[?:{}]", element.grapheme));
                }
                _ => {
                    // Look up in reverse mappings
                    if let Some(grapheme) = reverse_mappings.get(&element.canonical) {
                        result.push_str(grapheme);
                    } else {
                        // If no mapping found, use the grapheme directly
                        result.push_str(&element.grapheme);
                    }
                }
            }
        }
        
        Ok(result)
    }
    
    fn generate_alphabet(&self, ir: &AlphabetIR, schema: &Schema) -> Result<String, GenerateError> {
        let mut result = String::new();
        let reverse_mappings = self.build_reverse_mappings(schema);
        
        for element in &ir.elements {
            match element.element_type.0.as_str() {
                ElementType::WHITESPACE => {
                    result.push_str(&element.grapheme);
                }
                ElementType::UNKNOWN => {
                    result.push_str(&format!("[?:{}]", element.grapheme));
                }
                _ => {
                    if let Some(grapheme) = reverse_mappings.get(&element.canonical) {
                        result.push_str(grapheme);
                    } else {
                        result.push_str(&element.grapheme);
                    }
                }
            }
        }
        
        Ok(result)
    }
    
    fn build_reverse_mappings(&self, schema: &Schema) -> HashMap<String, String> {
        let mut reverse = HashMap::new();
        
        for (_, category_mappings) in &schema.mappings {
            for (grapheme, mapping) in category_mappings {
                reverse.insert(mapping.canonical.clone(), grapheme.clone());
            }
        }
        
        reverse
    }
}

pub struct GeneratorBuilder {
    generator: Generator,
}

impl GeneratorBuilder {
    pub fn new() -> Self {
        Self {
            generator: Generator::new(),
        }
    }
    
    pub fn with_schema(mut self, schema: Schema) -> Self {
        self.generator.load_schema(schema);
        self
    }
    
    pub fn with_schemas(mut self, schemas: Vec<Schema>) -> Self {
        for schema in schemas {
            self.generator.load_schema(schema);
        }
        self
    }
    
    pub fn build(self) -> Generator {
        self.generator
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema_parser::SchemaParser;
    use crate::ir::PropertyValue;
    
    #[test]
    fn test_generate_devanagari() {
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
    "ि":
      canonical: "i"
      type: vowel_dependent
"#;
        
        let schema = SchemaParser::parse_str(schema_yaml).unwrap();
        let generator = GeneratorBuilder::new()
            .with_schema(schema)
            .build();
        
        let mut ir = AbugidaIR::new("Devanagari".to_string());
        ir.push(Element::new(ElementType::CONSONANT, "क", "ka"));
        ir.push(Element::new(ElementType::VOWEL_DEPENDENT, "ि", "i"));
        
        let result = generator.generate(&IR::Abugida(ir), "Devanagari").unwrap();
        assert_eq!(result, "कि");
    }
    
    #[test]
    fn test_generate_iast() {
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
        let generator = GeneratorBuilder::new()
            .with_schema(schema)
            .build();
        
        let mut ir = AlphabetIR::new("IAST".to_string());
        ir.push(Element::new(ElementType::CONSONANT, "k", "k"));
        ir.push(Element::new(ElementType::VOWEL, "i", "i"));
        
        let result = generator.generate(&IR::Alphabet(ir), "IAST").unwrap();
        assert_eq!(result, "ki");
    }
}