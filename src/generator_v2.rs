use std::collections::HashMap;
use crate::ir_v2::{AbugidaIR, AlphabetIR, IR, ScriptId, SchemeId};
use crate::element_id::{ElementId, ElementRegistry};
use crate::schema_parser::Schema;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GenerateError {
    #[error("Unknown target script: {0}")]
    UnknownScript(String),
    
    #[error("Missing mapping for element: {0:?}")]
    MissingMapping(ElementId),
    
    #[error("Invalid IR for target script")]
    InvalidIR,
    
    #[error("Registry error: {0}")]
    RegistryError(String),
}

pub struct GeneratorV2 {
    schemas: HashMap<String, Schema>,
    registry: ElementRegistry,
    // Efficient reverse lookup tables: canonical -> grapheme
    abugida_mappings: HashMap<(String, String), String>, // (script, canonical) -> grapheme
    alphabet_mappings: HashMap<(String, String), String>, // (scheme, canonical) -> grapheme
    // Cache for frequently used mappings
    mapping_cache: HashMap<(ElementId, String), String>,
}

impl GeneratorV2 {
    pub fn new(registry: ElementRegistry) -> Self {
        Self {
            schemas: HashMap::new(),
            registry,
            abugida_mappings: HashMap::new(),
            alphabet_mappings: HashMap::new(),
            mapping_cache: HashMap::new(),
        }
    }
    
    pub fn load_schema(&mut self, schema: Schema) {
        self.build_reverse_mappings(&schema);
        self.schemas.insert(schema.name.clone(), schema);
        // Clear cache when schemas change
        self.mapping_cache.clear();
    }
    
    pub fn generate(&self, ir: &IR, target_script: &str) -> Result<String, GenerateError> {
        let schema = self.schemas.get(target_script)
            .ok_or_else(|| GenerateError::UnknownScript(target_script.to_string()))?;
        
        match ir {
            IR::Abugida(abugida_ir) => {
                if matches!(schema.script_type, crate::schema_parser::ScriptType::Abugida) {
                    self.generate_abugida(abugida_ir, target_script)
                } else {
                    Err(GenerateError::InvalidIR)
                }
            }
            IR::Alphabet(alphabet_ir) => {
                if matches!(schema.script_type, crate::schema_parser::ScriptType::Alphabet) {
                    self.generate_alphabet(alphabet_ir, target_script)
                } else {
                    Err(GenerateError::InvalidIR)
                }
            }
        }
    }
    
    fn generate_abugida(&self, ir: &AbugidaIR, target_script: &str) -> Result<String, GenerateError> {
        let mut result = String::new();
        
        for atom in &ir.elements {
            let element_id = atom.element_id;
            
            // Try cache first
            let cache_key = (element_id, target_script.to_string());
            if let Some(cached_grapheme) = self.mapping_cache.get(&cache_key) {
                result.push_str(cached_grapheme);
                continue;
            }
            
            // Get canonical name from registry
            let canonical_name = self.registry.get_name(element_id)
                .ok_or_else(|| GenerateError::MissingMapping(element_id))?;
            
            // Look up grapheme in target script
            let lookup_key = (target_script.to_string(), canonical_name.to_string());
            
            if let Some(grapheme) = self.abugida_mappings.get(&lookup_key) {
                result.push_str(grapheme);
                
                // Apply any modifiers
                if !atom.modifiers.is_empty() {
                    let modifier_graphemes = self.generate_modifiers(&atom.modifiers, target_script)?;
                    result.push_str(&modifier_graphemes);
                }
            } else {
                // Fallback: use the original grapheme from the IR
                result.push_str(ir.get_grapheme(atom));
            }
        }
        
        Ok(result)
    }
    
    fn generate_alphabet(&self, ir: &AlphabetIR, target_scheme: &str) -> Result<String, GenerateError> {
        let mut result = String::new();
        
        for atom in &ir.elements {
            let element_id = atom.element_id;
            
            // Try cache first
            let cache_key = (element_id, target_scheme.to_string());
            if let Some(cached_grapheme) = self.mapping_cache.get(&cache_key) {
                result.push_str(cached_grapheme);
                continue;
            }
            
            // Get canonical name from registry
            let canonical_name = self.registry.get_name(element_id)
                .ok_or_else(|| GenerateError::MissingMapping(element_id))?;
            
            // Look up grapheme in target scheme
            let lookup_key = (target_scheme.to_string(), canonical_name.to_string());
            
            if let Some(grapheme) = self.alphabet_mappings.get(&lookup_key) {
                result.push_str(grapheme);
            } else {
                // Fallback: use the original grapheme from the IR
                result.push_str(ir.get_grapheme(atom));
            }
        }
        
        Ok(result)
    }
    
    fn generate_modifiers(&self, modifiers: &crate::ir_v2::ModifierSet, _target_script: &str) -> Result<String, GenerateError> {
        let result = String::new();
        
        // This is a simplified implementation
        // In practice, you'd iterate through the modifier set and generate each modifier
        if !modifiers.is_empty() {
            // For now, just indicate that modifiers are present
            // TODO: Implement proper modifier enumeration and generation
        }
        
        Ok(result)
    }
    
    fn build_reverse_mappings(&mut self, schema: &Schema) {
        let script_name = &schema.name;
        
        for (_category, mappings) in &schema.mappings {
            for (grapheme, element_mapping) in mappings {
                let canonical = &element_mapping.canonical;
                let lookup_key = (script_name.clone(), canonical.clone());
                
                match schema.script_type {
                    crate::schema_parser::ScriptType::Abugida => {
                        self.abugida_mappings.insert(lookup_key, grapheme.clone());
                    }
                    crate::schema_parser::ScriptType::Alphabet => {
                        self.alphabet_mappings.insert(lookup_key, grapheme.clone());
                    }
                }
            }
        }
    }
    
    /// Generate with quality metrics for debugging
    pub fn generate_with_metrics(&self, ir: &IR, target_script: &str) -> Result<GenerationResult, GenerateError> {
        let start_time = std::time::Instant::now();
        let output = self.generate(ir, target_script)?;
        let generation_time = start_time.elapsed();
        
        let metrics = GenerationMetrics {
            generation_time_ns: generation_time.as_nanos() as u64,
            output_length: output.len(),
            element_count: match ir {
                IR::Abugida(abugida) => abugida.elements.len(),
                IR::Alphabet(alphabet) => alphabet.elements.len(),
            },
            cache_hits: 0, // TODO: Implement cache hit tracking
            fallback_count: 0, // TODO: Implement fallback tracking
        };
        
        Ok(GenerationResult {
            output,
            metrics,
        })
    }
    
    /// Batch generation for multiple targets
    pub fn generate_batch(&self, ir: &IR, target_scripts: &[&str]) -> Result<HashMap<String, String>, GenerateError> {
        let mut results = HashMap::new();
        
        for &target in target_scripts {
            let output = self.generate(ir, target)?;
            results.insert(target.to_string(), output);
        }
        
        Ok(results)
    }
    
    /// Streaming generation for very large texts
    pub fn generate_streaming<W: std::io::Write>(&self, ir: &IR, target_script: &str, writer: &mut W) -> Result<usize, GenerateError> {
        let mut bytes_written = 0;
        
        match ir {
            IR::Abugida(abugida_ir) => {
                for atom in &abugida_ir.elements {
                    let element_id = atom.element_id;
                    
                    let canonical_name = self.registry.get_name(element_id)
                        .ok_or_else(|| GenerateError::MissingMapping(element_id))?;
                    
                    let lookup_key = (target_script.to_string(), canonical_name.to_string());
                    
                    let grapheme = if let Some(grapheme) = self.abugida_mappings.get(&lookup_key) {
                        grapheme
                    } else {
                        abugida_ir.get_grapheme(atom)
                    };
                    
                    let written = writer.write(grapheme.as_bytes())
                        .map_err(|e| GenerateError::UnknownScript(format!("Write error: {}", e)))?;
                    bytes_written += written;
                }
            }
            IR::Alphabet(alphabet_ir) => {
                for atom in &alphabet_ir.elements {
                    let element_id = atom.element_id;
                    
                    let canonical_name = self.registry.get_name(element_id)
                        .ok_or_else(|| GenerateError::MissingMapping(element_id))?;
                    
                    let lookup_key = (target_script.to_string(), canonical_name.to_string());
                    
                    let grapheme = if let Some(grapheme) = self.alphabet_mappings.get(&lookup_key) {
                        grapheme
                    } else {
                        alphabet_ir.get_grapheme(atom)
                    };
                    
                    let written = writer.write(grapheme.as_bytes())
                        .map_err(|e| GenerateError::UnknownScript(format!("Write error: {}", e)))?;
                    bytes_written += written;
                }
            }
        }
        
        Ok(bytes_written)
    }
}

#[derive(Debug, Clone)]
pub struct GenerationResult {
    pub output: String,
    pub metrics: GenerationMetrics,
}

#[derive(Debug, Clone)]
pub struct GenerationMetrics {
    pub generation_time_ns: u64,
    pub output_length: usize,
    pub element_count: usize,
    pub cache_hits: usize,
    pub fallback_count: usize,
}

impl GenerationMetrics {
    pub fn throughput_chars_per_second(&self) -> f64 {
        if self.generation_time_ns == 0 {
            return 0.0;
        }
        
        let time_seconds = self.generation_time_ns as f64 / 1_000_000_000.0;
        self.output_length as f64 / time_seconds
    }
    
    pub fn throughput_elements_per_second(&self) -> f64 {
        if self.generation_time_ns == 0 {
            return 0.0;
        }
        
        let time_seconds = self.generation_time_ns as f64 / 1_000_000_000.0;
        self.element_count as f64 / time_seconds
    }
}

pub struct GeneratorBuilder {
    registry: ElementRegistry,
}

impl GeneratorBuilder {
    pub fn new() -> Self {
        Self {
            registry: ElementRegistry::default(),
        }
    }
    
    pub fn with_registry(mut self, registry: ElementRegistry) -> Self {
        self.registry = registry;
        self
    }
    
    pub fn with_schema(mut self, schema: Schema) -> Self {
        let mut generator = GeneratorV2::new(self.registry.clone());
        generator.load_schema(schema);
        self.registry = generator.registry;
        self
    }
    
    pub fn with_schemas(mut self, schemas: Vec<Schema>) -> Self {
        let mut generator = GeneratorV2::new(self.registry.clone());
        for schema in schemas {
            generator.load_schema(schema);
        }
        self.registry = generator.registry;
        self
    }
    
    pub fn build(self) -> GeneratorV2 {
        GeneratorV2::new(self.registry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema_parser::SchemaParser;
    use crate::element_id::ElementRegistry;
    
    #[test]
    fn test_generator_v2_basic() {
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
"#;
        
        let schema = SchemaParser::parse_str(schema_yaml).unwrap();
        let mut registry = ElementRegistry::default();
        
        // Register elements
        let ka_id = registry.register(crate::element_id::ElementType::Consonant, "ka".to_string());
        let kha_id = registry.register(crate::element_id::ElementType::Consonant, "kha".to_string());
        
        let mut generator = GeneratorV2::new(registry);
        generator.load_schema(schema);
        
        // Create IR
        let mut ir = AbugidaIR::new(ScriptId::DEVANAGARI);
        ir.push(ka_id, "क".to_string());
        ir.push(kha_id, "ख".to_string());
        
        let result = generator.generate(&IR::Abugida(ir), "Devanagari").unwrap();
        assert_eq!(result, "कख");
    }
    
    #[test]
    fn test_generator_v2_with_metrics() {
        let schema_yaml = r#"
name: "IAST"
type: alphabet

mappings:
  consonants:
    "k":
      canonical: "k"
  vowels:
    "a":
      canonical: "a"
"#;
        
        let schema = SchemaParser::parse_str(schema_yaml).unwrap();
        let mut registry = ElementRegistry::default();
        
        let k_id = registry.register(crate::element_id::ElementType::Consonant, "k".to_string());
        let a_id = registry.register(crate::element_id::ElementType::Vowel, "a".to_string());
        
        let mut generator = GeneratorV2::new(registry);
        generator.load_schema(schema);
        
        let mut ir = AlphabetIR::new(SchemeId::IAST);
        ir.push(k_id, "k".to_string());
        ir.push(a_id, "a".to_string());
        
        let result = generator.generate_with_metrics(&IR::Alphabet(ir), "IAST").unwrap();
        
        assert_eq!(result.output, "ka");
        assert!(result.metrics.generation_time_ns > 0);
        assert_eq!(result.metrics.output_length, 2);
        assert_eq!(result.metrics.element_count, 2);
    }
    
    #[test]
    fn test_generator_v2_batch() {
        let devanagari_schema = r#"
name: "Devanagari"
type: abugida
mappings:
  consonants:
    "क": { canonical: "ka" }
"#;
        
        let iast_schema = r#"
name: "IAST"
type: alphabet
mappings:
  consonants:
    "ka": { canonical: "ka" }
"#;
        
        let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
        let iast_schema = SchemaParser::parse_str(iast_schema).unwrap();
        
        let mut registry = ElementRegistry::default();
        let ka_id = registry.register(crate::element_id::ElementType::Consonant, "ka".to_string());
        
        let mut generator = GeneratorV2::new(registry);
        generator.load_schema(dev_schema);
        generator.load_schema(iast_schema);
        
        let mut ir = AbugidaIR::new(ScriptId::DEVANAGARI);
        ir.push(ka_id, "क".to_string());
        
        let results = generator.generate_batch(&IR::Abugida(ir), &["Devanagari"]).unwrap();
        
        assert!(results.contains_key("Devanagari"));
        assert_eq!(results["Devanagari"], "क");
    }
    
    #[test]
    fn test_generator_v2_streaming() {
        let schema_yaml = r#"
name: "IAST"
type: alphabet
mappings:
  consonants:
    "k": { canonical: "k" }
"#;
        
        let schema = SchemaParser::parse_str(schema_yaml).unwrap();
        let mut registry = ElementRegistry::default();
        let k_id = registry.register(crate::element_id::ElementType::Consonant, "k".to_string());
        
        let mut generator = GeneratorV2::new(registry);
        generator.load_schema(schema);
        
        let mut ir = AlphabetIR::new(SchemeId::IAST);
        ir.push(k_id, "k".to_string());
        
        let mut output = Vec::new();
        let bytes_written = generator.generate_streaming(&IR::Alphabet(ir), "IAST", &mut output).unwrap();
        
        assert_eq!(bytes_written, 1);
        assert_eq!(String::from_utf8(output).unwrap(), "k");
    }
}