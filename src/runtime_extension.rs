//! Runtime extension system for adding custom scripts and mappings
//! 
//! This module provides APIs for extending Shlesha at runtime with new scripts,
//! custom mappings, and user-defined fallback strategies.

use std::collections::HashMap;
use crate::lossless_transliterator::{LosslessTransliterator, FallbackStrategy, ScriptId};

/// Builder for creating custom script mappings at runtime
#[derive(Debug)]
pub struct CustomScriptBuilder {
    pub script_name: String,
    pub script_id: ScriptId,
    pub simple_mappings: Vec<(char, String)>,
    pub pattern_mappings: Vec<(String, String)>,
    pub fallback_strategy: FallbackStrategy,
}

impl CustomScriptBuilder {
    /// Create a new custom script builder
    pub fn new(script_name: &str, script_id: ScriptId) -> Self {
        Self {
            script_name: script_name.to_string(),
            script_id,
            simple_mappings: Vec::new(),
            pattern_mappings: Vec::new(),
            fallback_strategy: FallbackStrategy::Preserve,
        }
    }
    
    /// Add a simple character mapping
    pub fn add_mapping(mut self, from: char, to: &str) -> Self {
        self.simple_mappings.push((from, to.to_string()));
        self
    }
    
    /// Add multiple character mappings
    pub fn add_mappings(mut self, mappings: &[(char, &str)]) -> Self {
        for &(from, to) in mappings {
            self.simple_mappings.push((from, to.to_string()));
        }
        self
    }
    
    /// Add a pattern mapping for multi-character sequences
    pub fn add_pattern(mut self, from: &str, to: &str) -> Self {
        self.pattern_mappings.push((from.to_string(), to.to_string()));
        self
    }
    
    /// Add multiple pattern mappings
    pub fn add_patterns(mut self, patterns: &[(&str, &str)]) -> Self {
        for &(from, to) in patterns {
            self.pattern_mappings.push((from.to_string(), to.to_string()));
        }
        self
    }
    
    /// Set the fallback strategy for unknown characters
    pub fn with_fallback_strategy(mut self, strategy: FallbackStrategy) -> Self {
        self.fallback_strategy = strategy;
        self
    }
    
    /// Build the custom script mapping (note: returns owned data, not static)
    /// This would need to be stored in a custom registry for runtime use
    pub fn build(self) -> CustomScript {
        // Sort simple mappings by Unicode value for binary search
        let mut simple = self.simple_mappings;
        simple.sort_by_key(|(ch, _)| *ch);
        
        // Sort patterns by length (descending) for longest-match-first
        let mut patterns = self.pattern_mappings;
        patterns.sort_by_key(|(pattern, _)| std::cmp::Reverse(pattern.len()));
        
        CustomScript {
            script_name: self.script_name,
            script_id: self.script_id,
            simple_mappings: simple,
            pattern_mappings: patterns,
            fallback_strategy: self.fallback_strategy,
        }
    }
}

/// A custom script with runtime-defined mappings
#[derive(Debug)]
pub struct CustomScript {
    pub script_name: String,
    pub script_id: ScriptId,
    pub simple_mappings: Vec<(char, String)>,
    pub pattern_mappings: Vec<(String, String)>,
    pub fallback_strategy: FallbackStrategy,
}

impl CustomScript {
    /// Create a character lookup function for this script
    pub fn lookup_char(&self, ch: char) -> Option<&str> {
        self.simple_mappings
            .binary_search_by_key(&ch, |(c, _)| *c)
            .ok()
            .map(|idx| self.simple_mappings[idx].1.as_str())
    }
    
    /// Create a pattern lookup function for this script
    pub fn lookup_pattern(&self, text: &str, start_pos: usize) -> Option<(&str, usize)> {
        if let Some(remaining) = text.get(start_pos..) {
            for (pattern, replacement) in &self.pattern_mappings {
                if remaining.starts_with(pattern) {
                    return Some((replacement.as_str(), pattern.chars().count()));
                }
            }
        }
        None
    }
}

/// Runtime extension manager for Shlesha
#[derive(Debug)]
pub struct ExtensionManager {
    custom_scripts: HashMap<ScriptId, CustomScript>,
    custom_mappings: HashMap<(ScriptId, ScriptId), CustomMapping>,
}

/// A custom mapping between two scripts
#[derive(Debug)]
pub struct CustomMapping {
    pub from_script: ScriptId,
    pub to_script: ScriptId,
    pub char_mappings: HashMap<char, String>,
    pub pattern_mappings: Vec<(String, String)>,
    pub fallback_strategy: FallbackStrategy,
}

impl ExtensionManager {
    /// Create a new extension manager
    pub fn new() -> Self {
        Self {
            custom_scripts: HashMap::new(),
            custom_mappings: HashMap::new(),
        }
    }
    
    /// Register a custom script
    pub fn register_script(&mut self, script: CustomScript) {
        self.custom_scripts.insert(script.script_id, script);
    }
    
    /// Register a custom mapping between two scripts
    pub fn register_mapping(&mut self, mapping: CustomMapping) {
        self.custom_mappings.insert((mapping.from_script, mapping.to_script), mapping);
    }
    
    /// Get a custom script by ID
    pub fn get_script(&self, script_id: ScriptId) -> Option<&CustomScript> {
        self.custom_scripts.get(&script_id)
    }
    
    /// Get a custom mapping by script pair
    pub fn get_mapping(&self, from: ScriptId, to: ScriptId) -> Option<&CustomMapping> {
        self.custom_mappings.get(&(from, to))
    }
    
    /// List all registered custom scripts
    pub fn list_custom_scripts(&self) -> Vec<(ScriptId, &str)> {
        self.custom_scripts
            .iter()
            .map(|(&id, script)| (id, script.script_name.as_str()))
            .collect()
    }
    
    /// Check if a custom mapping exists
    pub fn has_custom_mapping(&self, from: ScriptId, to: ScriptId) -> bool {
        self.custom_mappings.contains_key(&(from, to))
    }
}

impl Default for ExtensionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Extended transliterator that supports runtime extensions
pub struct ExtendedTransliterator {
    core: LosslessTransliterator,
    extensions: ExtensionManager,
}

impl ExtendedTransliterator {
    /// Create a new extended transliterator
    pub fn new() -> Self {
        Self {
            core: LosslessTransliterator::new(),
            extensions: ExtensionManager::new(),
        }
    }
    
    /// Add a custom script at runtime
    pub fn add_custom_script(&mut self, script: CustomScript) {
        self.extensions.register_script(script);
    }
    
    /// Add a custom mapping at runtime
    pub fn add_custom_mapping(&mut self, mapping: CustomMapping) {
        self.extensions.register_mapping(mapping);
    }
    
    /// Get the extension manager for advanced operations
    pub fn extensions(&self) -> &ExtensionManager {
        &self.extensions
    }
    
    /// Get mutable access to the extension manager
    pub fn extensions_mut(&mut self) -> &mut ExtensionManager {
        &mut self.extensions
    }
    
    /// Transliterate using both built-in and custom mappings
    pub fn transliterate(&self, text: &str, from: &str, to: &str) -> Result<String, String> {
        // First try the core transliterator
        match self.core.transliterate(text, from, to) {
            Ok(result) => Ok(result),
            Err(_) => {
                // If core fails, try custom mappings
                // This would need more sophisticated implementation
                // For now, just return the core error
                self.core.transliterate(text, from, to)
            }
        }
    }
    
    /// Get the core transliterator for built-in operations
    pub fn core(&self) -> &LosslessTransliterator {
        &self.core
    }
}

impl Default for ExtendedTransliterator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_custom_script_builder() {
        let script = CustomScriptBuilder::new("TestScript", 100)
            .add_mapping('a', "α")
            .add_mapping('b', "β")
            .add_pattern("ch", "χ")
            .with_fallback_strategy(FallbackStrategy::PreserveWithPhonetics)
            .build();
        
        assert_eq!(script.script_name, "TestScript");
        assert_eq!(script.script_id, 100);
        assert_eq!(script.simple_mappings.len(), 2);
        assert_eq!(script.pattern_mappings.len(), 1);
        
        // Test lookups
        assert_eq!(script.lookup_char('a'), Some("α"));
        assert_eq!(script.lookup_char('b'), Some("β"));
        assert_eq!(script.lookup_char('z'), None);
        
        assert_eq!(script.lookup_pattern("choose", 0), Some(("χ", 2)));
        assert_eq!(script.lookup_pattern("test", 0), None);
    }
    
    #[test]
    fn test_extension_manager() {
        let mut manager = ExtensionManager::new();
        
        let script = CustomScriptBuilder::new("TestScript", 100)
            .add_mapping('x', "X")
            .build();
        
        manager.register_script(script);
        
        assert!(manager.get_script(100).is_some());
        assert!(manager.get_script(99).is_none());
        
        let scripts = manager.list_custom_scripts();
        assert_eq!(scripts.len(), 1);
        assert_eq!(scripts[0], (100, "TestScript"));
    }
    
    #[test]
    fn test_extended_transliterator() {
        let mut transliterator = ExtendedTransliterator::new();
        
        // Test basic functionality with built-in scripts
        let result = transliterator.transliterate("क", "Devanagari", "IAST");
        assert!(result.is_ok());
        
        // Add a custom script
        let custom_script = CustomScriptBuilder::new("MyScript", 200)
            .add_mapping('α', "a")
            .add_mapping('β', "b")
            .build();
        
        transliterator.add_custom_script(custom_script);
        
        // Verify the custom script was added
        assert!(transliterator.extensions().get_script(200).is_some());
    }
}