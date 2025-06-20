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

// Optimized trie structure for fast longest-match parsing
struct TrieNode {
    mapping: Option<(String, ElementMapping)>, // category, mapping
    children: HashMap<char, Box<TrieNode>>,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            mapping: None,
            children: HashMap::new(),
        }
    }
}

pub struct ParserOptimized {
    // Schema name -> Trie for that schema
    schema_tries: HashMap<String, TrieNode>,
    schemas: HashMap<String, Schema>,
}

impl ParserOptimized {
    pub fn new() -> Self {
        Self {
            schema_tries: HashMap::new(),
            schemas: HashMap::new(),
        }
    }
    
    pub fn load_schema(&mut self, schema: Schema) {
        // Build trie for this schema
        let mut root = TrieNode::new();
        
        for (category, mappings) in &schema.mappings {
            for (grapheme, mapping) in mappings {
                let mut node = &mut root;
                for ch in grapheme.chars() {
                    node = node.children.entry(ch)
                        .or_insert_with(|| Box::new(TrieNode::new()));
                }
                node.mapping = Some((category.clone(), mapping.clone()));
            }
        }
        
        self.schema_tries.insert(schema.name.clone(), root);
        self.schemas.insert(schema.name.clone(), schema);
    }
    
    pub fn parse(&self, input: &str, script_name: &str) -> Result<IR, ParseError> {
        let schema = self.schemas.get(script_name)
            .ok_or_else(|| ParseError::UnknownScript(script_name.to_string()))?;
        
        let trie = self.schema_tries.get(script_name)
            .ok_or_else(|| ParseError::UnknownScript(script_name.to_string()))?;
        
        match schema.script_type {
            ScriptType::Abugida => self.parse_abugida_optimized(input, schema, trie),
            ScriptType::Alphabet => self.parse_alphabet_optimized(input, schema, trie),
        }
    }
    
    fn parse_abugida_optimized(&self, input: &str, schema: &Schema, trie: &TrieNode) -> Result<IR, ParseError> {
        let mut ir = AbugidaIR::new(schema.name.clone());
        let input_chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        
        while i < input_chars.len() {
            // Use trie for longest match
            let (match_len, mapping_info) = self.find_longest_match(&input_chars[i..], trie);
            
            if let Some((category, mapping)) = mapping_info {
                // Create grapheme without allocating string
                let grapheme_end = i + match_len;
                let grapheme: String = input_chars[i..grapheme_end].iter().collect();
                
                let element = self.create_element_from_mapping(&grapheme, &mapping, &category);
                ir.push(element);
                i += match_len;
            } else {
                // Handle single character
                let ch = input_chars[i];
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
    
    fn parse_alphabet_optimized(&self, input: &str, schema: &Schema, trie: &TrieNode) -> Result<IR, ParseError> {
        let mut ir = AlphabetIR::new(schema.name.clone());
        let input_chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        
        while i < input_chars.len() {
            // Use trie for longest match
            let (match_len, mapping_info) = self.find_longest_match(&input_chars[i..], trie);
            
            if let Some((category, mapping)) = mapping_info {
                let grapheme_end = i + match_len;
                let grapheme: String = input_chars[i..grapheme_end].iter().collect();
                
                let element = self.create_element_from_mapping(&grapheme, &mapping, &category);
                ir.push(element);
                i += match_len;
            } else {
                // Handle single character
                let ch = input_chars[i];
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
    
    // Find longest match in trie
    fn find_longest_match(&self, chars: &[char], root: &TrieNode) -> (usize, Option<(String, ElementMapping)>) {
        let mut node = root;
        let mut last_match = None;
        let mut last_match_len = 0;
        
        for (i, &ch) in chars.iter().enumerate() {
            if let Some(child) = node.children.get(&ch) {
                node = child;
                if let Some(ref mapping) = node.mapping {
                    last_match = Some(mapping.clone());
                    last_match_len = i + 1;
                }
            } else {
                break;
            }
        }
        
        (last_match_len, last_match)
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