use std::path::Path;
use crate::ir::{IR, Extension};
use crate::parser::{Parser, ParseError};
use crate::transformer::{Transformer, TransformError};
use crate::generator::{Generator, GenerateError};
use crate::schema_parser::{Schema, SchemaRegistry, SchemaError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransliteratorError {
    #[error("Parse error: {0}")]
    ParseError(#[from] ParseError),
    
    #[error("Transform error: {0}")]
    TransformError(#[from] TransformError),
    
    #[error("Generate error: {0}")]
    GenerateError(#[from] GenerateError),
    
    #[error("Schema error: {0}")]
    SchemaError(#[from] SchemaError),
    
    #[error("Unknown script: {0}")]
    UnknownScript(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
}

pub struct Transliterator {
    parser: Parser,
    transformer: Transformer,
    generator: Generator,
    schema_registry: SchemaRegistry,
    active_extensions: Vec<String>,
}

impl Transliterator {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            transformer: Transformer::new(),
            generator: Generator::new(),
            schema_registry: SchemaRegistry::new(),
            active_extensions: Vec::new(),
        }
    }
    
    pub fn load_schema(&mut self, schema: Schema) -> Result<(), TransliteratorError> {
        self.parser.load_schema(schema.clone());
        self.generator.load_schema(schema.clone());
        self.schema_registry.register(schema);
        Ok(())
    }
    
    pub fn load_schema_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), TransliteratorError> {
        let schema = crate::schema_parser::SchemaParser::parse_file(path)?;
        self.load_schema(schema)
    }
    
    pub fn load_schema_directory<P: AsRef<Path>>(&mut self, dir: P) -> Result<usize, TransliteratorError> {
        let count = self.schema_registry.load_directory(&dir)?;
        
        // Reload schemas into parser and generator
        for (_, schema) in self.schema_registry.schemas.iter() {
            self.parser.load_schema(schema.clone());
            self.generator.load_schema(schema.clone());
        }
        
        Ok(count)
    }
    
    pub fn add_extension(&mut self, extension_name: &str) -> Result<(), TransliteratorError> {
        self.active_extensions.push(extension_name.to_string());
        Ok(())
    }
    
    pub fn clear_extensions(&mut self) {
        self.active_extensions.clear();
    }
    
    pub fn transliterate(
        &self,
        input: &str,
        from_script: &str,
        to_script: &str
    ) -> Result<String, TransliteratorError> {
        // Parse input to IR
        let mut source_ir = self.parser.parse(input, from_script)?;
        
        // Apply active extensions to source IR
        self.apply_extensions(&mut source_ir, from_script)?;
        
        // Determine target type
        let target_type = self.get_script_type(to_script)?;
        
        // Transform IR if needed
        let target_ir = if self.get_script_type(from_script)? != target_type {
            self.transformer.transform(source_ir, &target_type)?
        } else {
            source_ir
        };
        
        // Generate output
        let output = self.generator.generate(&target_ir, to_script)?;
        
        Ok(output)
    }
    
    pub fn transliterate_with_extensions(
        &self,
        input: &str,
        from_script: &str,
        to_script: &str,
        extensions: &[&str]
    ) -> Result<String, TransliteratorError> {
        let mut temp_transliterator = self.clone_with_extensions(extensions)?;
        temp_transliterator.transliterate(input, from_script, to_script)
    }
    
    fn apply_extensions(&self, ir: &mut IR, script_name: &str) -> Result<(), TransliteratorError> {
        if self.active_extensions.is_empty() {
            return Ok(());
        }
        
        let schema = self.schema_registry.get(script_name)
            .ok_or_else(|| TransliteratorError::UnknownScript(script_name.to_string()))?;
        
        for ext_name in &self.active_extensions {
            if let Some(ext_def) = schema.extensions.get(ext_name) {
                let extension = Extension {
                    name: ext_name.clone(),
                    priority: ext_def.priority,
                    mappings: ext_def.mappings.iter()
                        .map(|(k, v)| {
                            let mapping = crate::ir::ExtensionMapping {
                                from: k.clone(),
                                to: v.to.clone(),
                                element_type: v.element_type.as_ref()
                                    .map(|t| crate::ir::ElementType(t.clone())),
                                properties: v.properties.iter()
                                    .map(|(pk, pv)| {
                                        (pk.clone(), crate::schema_parser::SchemaParser::yaml_value_to_property_value(pv))
                                    })
                                    .collect(),
                            };
                            (k.clone(), mapping)
                        })
                        .collect(),
                };
                
                ir.add_extension(extension);
            }
        }
        
        // Apply all extensions
        match ir {
            IR::Abugida(abugida) => abugida.apply_extensions(),
            IR::Alphabet(alphabet) => alphabet.apply_extensions(),
        }
        
        Ok(())
    }
    
    fn get_script_type(&self, script_name: &str) -> Result<&'static str, TransliteratorError> {
        let schema = self.schema_registry.get(script_name)
            .ok_or_else(|| TransliteratorError::UnknownScript(script_name.to_string()))?;
        
        match schema.script_type {
            crate::schema_parser::ScriptType::Abugida => Ok("abugida"),
            crate::schema_parser::ScriptType::Alphabet => Ok("alphabet"),
        }
    }
    
    fn clone_with_extensions(&self, extensions: &[&str]) -> Result<Transliterator, TransliteratorError> {
        let mut new_transliterator = Transliterator::new();
        
        // Copy all schemas
        for (_, schema) in self.schema_registry.schemas.iter() {
            new_transliterator.load_schema(schema.clone())?;
        }
        
        // Add specified extensions
        for ext in extensions {
            new_transliterator.add_extension(ext)?;
        }
        
        Ok(new_transliterator)
    }
}

pub struct TransliteratorBuilder {
    transliterator: Transliterator,
}

impl TransliteratorBuilder {
    pub fn new() -> Self {
        Self {
            transliterator: Transliterator::new(),
        }
    }
    
    pub fn with_schema(mut self, schema: Schema) -> Result<Self, TransliteratorError> {
        self.transliterator.load_schema(schema)?;
        Ok(self)
    }
    
    pub fn with_schema_file<P: AsRef<Path>>(mut self, path: P) -> Result<Self, TransliteratorError> {
        self.transliterator.load_schema_file(path)?;
        Ok(self)
    }
    
    pub fn with_schema_directory<P: AsRef<Path>>(mut self, dir: P) -> Result<Self, TransliteratorError> {
        self.transliterator.load_schema_directory(dir)?;
        Ok(self)
    }
    
    pub fn with_extension(mut self, extension_name: &str) -> Result<Self, TransliteratorError> {
        self.transliterator.add_extension(extension_name)?;
        Ok(self)
    }
    
    pub fn build(self) -> Transliterator {
        self.transliterator
    }
}

impl Default for Transliterator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema_parser::SchemaParser;
    
    #[test]
    fn test_basic_transliteration() {
        let devanagari_schema = r#"
name: "Devanagari"
type: abugida

mappings:
  consonants:
    "क":
      canonical: "ka"
    "न":
      canonical: "na"
    "म":
      canonical: "ma"
    "स":
      canonical: "sa"
  vowels:
    "अ":
      canonical: "a"
    "आ":
      canonical: "ā"
      dependent_form: "ा"
  modifiers:
    "्":
      canonical: ""
      type: virama
"#;
        
        let iast_schema = r#"
name: "IAST"
type: alphabet

mappings:
  consonants:
    "k":
      canonical: "k"
    "n":
      canonical: "n"
    "m":
      canonical: "m"
    "s":
      canonical: "s"
  vowels:
    "a":
      canonical: "a"
    "ā":
      canonical: "ā"
"#;
        
        let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
        let iast_schema = SchemaParser::parse_str(iast_schema).unwrap();
        
        let transliterator = TransliteratorBuilder::new()
            .with_schema(dev_schema).unwrap()
            .with_schema(iast_schema).unwrap()
            .build();
        
        // Test Devanagari to IAST
        let result = transliterator.transliterate("नमस्", "Devanagari", "IAST").unwrap();
        assert_eq!(result, "namas");
        
        // Test IAST to Devanagari
        let result = transliterator.transliterate("nama", "IAST", "Devanagari").unwrap();
        assert_eq!(result, "नम");
    }
}