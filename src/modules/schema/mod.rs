use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaMetadata {
    pub name: String,
    pub script_type: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub author: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub metadata: SchemaMetadata,
    pub target: String,
    pub mappings: HashMap<String, HashMap<String, Value>>,
}

impl Schema {
    pub fn from_yaml_str(yaml_str: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml_str)
    }

    pub fn from_json_str(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    pub fn to_yaml_string(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }

    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn is_alphabet_target(&self) -> bool {
        self.target == "alphabet_tokens"
    }

    pub fn is_abugida_target(&self) -> bool {
        self.target == "abugida_tokens"
    }

    pub fn get_all_tokens(&self) -> Vec<String> {
        self.mappings
            .values()
            .flat_map(|category| category.keys())
            .cloned()
            .collect()
    }

    pub fn get_all_inputs(&self) -> Vec<String> {
        self.mappings
            .values()
            .flat_map(|category| category.values())
            .flat_map(|mapping| match mapping {
                Value::String(s) => vec![s.clone()],
                Value::Array(arr) => arr
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect(),
                _ => vec![],
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct SchemaBuilder {
    metadata: SchemaMetadata,
    target: String,
    mappings: HashMap<String, HashMap<String, Value>>,
}

impl SchemaBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            metadata: SchemaMetadata {
                name: name.to_string(),
                script_type: "unknown".to_string(),
                description: None,
                version: None,
                author: None,
            },
            target: "alphabet_tokens".to_string(),
            mappings: HashMap::new(),
        }
    }

    pub fn script_type(mut self, script_type: &str) -> Self {
        self.metadata.script_type = script_type.to_string();
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.metadata.description = Some(description.to_string());
        self
    }

    pub fn version(mut self, version: &str) -> Self {
        self.metadata.version = Some(version.to_string());
        self
    }

    pub fn author(mut self, author: &str) -> Self {
        self.metadata.author = Some(author.to_string());
        self
    }

    pub fn target(mut self, target: &str) -> Self {
        self.target = target.to_string();
        self
    }

    pub fn add_vowel_mapping(self, token: &str, inputs: &[&str]) -> Self {
        self.add_mapping("vowels", token, inputs)
    }

    pub fn add_consonant_mapping(self, token: &str, inputs: &[&str]) -> Self {
        self.add_mapping("consonants", token, inputs)
    }

    pub fn add_mark_mapping(self, token: &str, inputs: &[&str]) -> Self {
        self.add_mapping("marks", token, inputs)
    }

    pub fn add_digit_mapping(self, token: &str, inputs: &[&str]) -> Self {
        self.add_mapping("digits", token, inputs)
    }

    pub fn add_mapping(mut self, category: &str, token: &str, inputs: &[&str]) -> Self {
        let category_map = self.mappings.entry(category.to_string()).or_default();

        let value = if inputs.len() == 1 {
            Value::String(inputs[0].to_string())
        } else {
            Value::Array(
                inputs
                    .iter()
                    .map(|s| Value::String(s.to_string()))
                    .collect(),
            )
        };

        category_map.insert(token.to_string(), value);
        self
    }

    pub fn build(self) -> Schema {
        Schema {
            metadata: self.metadata,
            target: self.target,
            mappings: self.mappings,
        }
    }
}
