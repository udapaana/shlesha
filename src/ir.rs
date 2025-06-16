use std::fmt;
use std::collections::HashMap;
use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementType(pub String);

impl ElementType {
    pub const CONSONANT: &'static str = "consonant";
    pub const VOWEL_INDEPENDENT: &'static str = "vowel_independent";
    pub const VOWEL_DEPENDENT: &'static str = "vowel_dependent";
    pub const VOWEL: &'static str = "vowel";
    pub const MODIFIER: &'static str = "modifier";
    pub const VIRAMA: &'static str = "virama";
    pub const NUKTA: &'static str = "nukta";
    pub const ANUSVARA: &'static str = "anusvara";
    pub const VISARGA: &'static str = "visarga";
    pub const AVAGRAHA: &'static str = "avagraha";
    pub const ACCENT: &'static str = "accent";
    pub const NUMERAL: &'static str = "numeral";
    pub const PUNCTUATION: &'static str = "punctuation";
    pub const WHITESPACE: &'static str = "whitespace";
    pub const UNKNOWN: &'static str = "unknown";
}

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    pub element_type: ElementType,
    pub grapheme: String,
    pub canonical: String,
    pub properties: HashMap<String, PropertyValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    Bool(bool),
    String(String),
    Number(f64),
    List(Vec<PropertyValue>),
    Map(HashMap<String, PropertyValue>),
}

impl Element {
    pub fn new(element_type: impl Into<String>, grapheme: impl Into<String>, canonical: impl Into<String>) -> Self {
        Self {
            element_type: ElementType(element_type.into()),
            grapheme: grapheme.into(),
            canonical: canonical.into(),
            properties: HashMap::new(),
        }
    }

    pub fn with_property(mut self, key: impl Into<String>, value: PropertyValue) -> Self {
        self.properties.insert(key.into(), value);
        self
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.properties.get(key) {
            Some(PropertyValue::Bool(b)) => Some(*b),
            _ => None,
        }
    }

    pub fn get_string(&self, key: &str) -> Option<&str> {
        match self.properties.get(key) {
            Some(PropertyValue::String(s)) => Some(s.as_str()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AbugidaIR {
    pub elements: Vec<Element>,
    pub script: String,
    pub metadata: Metadata,
    pub extensions: IndexMap<String, Extension>,
}

#[derive(Debug, Clone)]
pub struct AlphabetIR {
    pub elements: Vec<Element>,
    pub scheme: String,
    pub metadata: Metadata,
    pub extensions: IndexMap<String, Extension>,
}

#[derive(Debug, Clone)]
pub struct Extension {
    pub name: String,
    pub priority: i32,
    pub mappings: HashMap<String, ExtensionMapping>,
}

#[derive(Debug, Clone)]
pub struct ExtensionMapping {
    pub from: String,
    pub to: String,
    pub element_type: Option<ElementType>,
    pub properties: HashMap<String, PropertyValue>,
}

#[derive(Debug, Clone, Default)]
pub struct Metadata {
    pub source_position: Vec<usize>,
    pub warnings: Vec<String>,
    pub is_normalized: bool,
    pub custom_properties: HashMap<String, PropertyValue>,
}

pub enum IR {
    Abugida(AbugidaIR),
    Alphabet(AlphabetIR),
}

impl IR {
    pub fn is_abugida(&self) -> bool {
        matches!(self, IR::Abugida(_))
    }

    pub fn is_alphabet(&self) -> bool {
        matches!(self, IR::Alphabet(_))
    }

    pub fn add_extension(&mut self, extension: Extension) {
        match self {
            IR::Abugida(ir) => ir.add_extension(extension),
            IR::Alphabet(ir) => ir.add_extension(extension),
        }
    }
}

impl AbugidaIR {
    pub fn new(script: String) -> Self {
        Self {
            elements: Vec::new(),
            script,
            metadata: Metadata::default(),
            extensions: IndexMap::new(),
        }
    }

    pub fn push(&mut self, element: Element) {
        self.elements.push(element);
    }

    pub fn with_metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn add_extension(&mut self, extension: Extension) {
        self.extensions.insert(extension.name.clone(), extension);
    }

    pub fn apply_extensions(&mut self) {
        let extensions: Vec<_> = self.extensions.values().cloned().collect();
        
        for ext in extensions.iter() {
            let mut new_elements = Vec::new();
            
            for element in &self.elements {
                if let Some(mapping) = ext.mappings.get(&element.grapheme) {
                    let mut new_element = Element::new(
                        mapping.element_type.as_ref()
                            .map(|t| t.0.clone())
                            .unwrap_or_else(|| element.element_type.0.clone()),
                        &mapping.to,
                        &element.canonical,
                    );
                    
                    for (k, v) in &element.properties {
                        new_element.properties.insert(k.clone(), v.clone());
                    }
                    
                    for (k, v) in &mapping.properties {
                        new_element.properties.insert(k.clone(), v.clone());
                    }
                    
                    new_elements.push(new_element);
                } else {
                    new_elements.push(element.clone());
                }
            }
            
            self.elements = new_elements;
        }
    }
}

impl AlphabetIR {
    pub fn new(scheme: String) -> Self {
        Self {
            elements: Vec::new(),
            scheme,
            metadata: Metadata::default(),
            extensions: IndexMap::new(),
        }
    }

    pub fn push(&mut self, element: Element) {
        self.elements.push(element);
    }

    pub fn with_metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn add_extension(&mut self, extension: Extension) {
        self.extensions.insert(extension.name.clone(), extension);
    }

    pub fn apply_extensions(&mut self) {
        let extensions: Vec<_> = self.extensions.values().cloned().collect();
        
        for ext in extensions.iter() {
            let mut new_elements = Vec::new();
            
            for element in &self.elements {
                if let Some(mapping) = ext.mappings.get(&element.grapheme) {
                    let mut new_element = Element::new(
                        mapping.element_type.as_ref()
                            .map(|t| t.0.clone())
                            .unwrap_or_else(|| element.element_type.0.clone()),
                        &mapping.to,
                        &element.canonical,
                    );
                    
                    for (k, v) in &element.properties {
                        new_element.properties.insert(k.clone(), v.clone());
                    }
                    
                    for (k, v) in &mapping.properties {
                        new_element.properties.insert(k.clone(), v.clone());
                    }
                    
                    new_elements.push(new_element);
                } else {
                    new_elements.push(element.clone());
                }
            }
            
            self.elements = new_elements;
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grapheme)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extensible_abugida_ir() {
        let mut ir = AbugidaIR::new("Devanagari".to_string());
        
        let consonant = Element::new(ElementType::CONSONANT, "क", "ka")
            .with_property("has_inherent_vowel", PropertyValue::Bool(true))
            .with_property("aspirated", PropertyValue::Bool(false));
        
        ir.push(consonant);
        
        let vowel = Element::new(ElementType::VOWEL_DEPENDENT, "ि", "i");
        ir.push(vowel);

        assert_eq!(ir.elements.len(), 2);
        assert_eq!(ir.script, "Devanagari");
        
        let first = &ir.elements[0];
        assert_eq!(first.get_bool("has_inherent_vowel"), Some(true));
        assert_eq!(first.get_bool("aspirated"), Some(false));
    }

    #[test]
    fn test_custom_element_types() {
        let mut ir = AlphabetIR::new("CustomScheme".to_string());
        
        let custom_element = Element::new("custom_modifier", "~", "tilde")
            .with_property("custom_property", PropertyValue::String("custom_value".to_string()));
        
        ir.push(custom_element);
        
        assert_eq!(ir.elements[0].element_type.0, "custom_modifier");
        assert_eq!(ir.elements[0].get_string("custom_property"), Some("custom_value"));
    }

    #[test]
    fn test_extensions() {
        let mut ir = AbugidaIR::new("Devanagari".to_string());
        ir.push(Element::new(ElementType::CONSONANT, "क", "ka"));
        
        let mut extension = Extension {
            name: "vedic_accents".to_string(),
            priority: 1,
            mappings: HashMap::new(),
        };
        
        extension.mappings.insert("क".to_string(), ExtensionMapping {
            from: "क".to_string(),
            to: "क॑".to_string(),
            element_type: None,
            properties: {
                let mut props = HashMap::new();
                props.insert("accent".to_string(), PropertyValue::String("udatta".to_string()));
                props
            },
        });
        
        ir.add_extension(extension);
        ir.apply_extensions();
        
        assert_eq!(ir.elements[0].grapheme, "क॑");
        assert_eq!(ir.elements[0].get_string("accent"), Some("udatta"));
    }
}