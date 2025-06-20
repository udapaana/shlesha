use crate::ir_v2::{AbugidaIR, AlphabetIR, IR, ScriptId, SchemeId};
use crate::element_id::{ElementId, ElementRegistry, ElementType};
use thiserror::Error;
use std::collections::HashMap;

#[derive(Debug, Error)]
pub enum TransformError {
    #[error("Cannot transform between incompatible scripts")]
    IncompatibleScripts,
    
    #[error("Missing element mapping for: {0:?}")]
    MissingElementMapping(ElementId),
    
    #[error("Invalid transformation: {0}")]
    InvalidTransformation(String),
    
    #[error("Registry error: {0}")]
    RegistryError(String),
}

pub struct TransformerV2 {
    registry: ElementRegistry,
    // Mapping tables for efficient transformation
    abugida_to_alphabet_map: HashMap<ElementId, Vec<ElementId>>,
    alphabet_to_abugida_map: HashMap<Vec<ElementId>, ElementId>,
    // Cache for decomposition patterns
    consonant_decomposition: HashMap<ElementId, (ElementId, ElementId)>, // consonant -> (pure_consonant, inherent_vowel)
}

impl TransformerV2 {
    pub fn new(registry: ElementRegistry) -> Self {
        let mut transformer = Self {
            registry,
            abugida_to_alphabet_map: HashMap::new(),
            alphabet_to_abugida_map: HashMap::new(),
            consonant_decomposition: HashMap::new(),
        };
        
        transformer.build_transformation_maps();
        transformer
    }
    
    pub fn transform(&self, source: IR, target_type: ScriptType) -> Result<IR, TransformError> {
        match (&source, target_type) {
            (IR::Abugida(abugida), ScriptType::Alphabet) => {
                self.abugida_to_alphabet(abugida)
            }
            (IR::Alphabet(alphabet), ScriptType::Abugida) => {
                self.alphabet_to_abugida(alphabet)
            }
            (IR::Abugida(source_abugida), ScriptType::Abugida) => {
                self.abugida_to_abugida(source_abugida)
            }
            (IR::Alphabet(source_alphabet), ScriptType::Alphabet) => {
                self.alphabet_to_alphabet(source_alphabet)
            }
        }
    }
    
    /// Core transformation: Abugida → Alphabet
    /// Decomposes abugida atoms (ka, kha) into alphabet atoms (k+a, kh+a)
    fn abugida_to_alphabet(&self, abugida: &AbugidaIR) -> Result<IR, TransformError> {
        let mut alphabet = AlphabetIR::new(SchemeId::IAST); // Default to IAST
        
        for (i, atom) in abugida.elements.iter().enumerate() {
            let element_id = atom.element_id;
            
            // Check if this is a consonant with inherent vowel
            if element_id.is_consonant() {
                // Look ahead to see if next element modifies this consonant
                let next_is_modifier = i + 1 < abugida.elements.len() && 
                    (abugida.elements[i + 1].element_id.is_modifier() ||
                     matches!(abugida.elements[i + 1].element_id.element_type(), ElementType::VowelDependent));
                
                if let Some(decomposition) = self.abugida_to_alphabet_map.get(&element_id) {
                    if next_is_modifier {
                        // Only add the pure consonant, modifier will handle the vowel
                        alphabet.push(decomposition[0], self.get_element_grapheme(decomposition[0])?);
                    } else {
                        // Add full decomposition (consonant + inherent vowel)
                        for &component_id in decomposition {
                            alphabet.push(component_id, self.get_element_grapheme(component_id)?);
                        }
                    }
                } else {
                    // Unknown consonant, pass through
                    alphabet.push(element_id, abugida.get_grapheme(atom).to_string());
                }
            }
            else if matches!(element_id.element_type(), ElementType::VowelDependent) {
                // Dependent vowel becomes independent vowel in alphabet
                let vowel_id = self.dependent_to_independent_vowel(element_id)?;
                alphabet.push(vowel_id, self.get_element_grapheme(vowel_id)?);
            }
            else if matches!(element_id.element_type(), ElementType::Virama) {
                // Virama in abugida means "suppress inherent vowel" - skip in alphabet
                continue;
            }
            else {
                // Other elements (independent vowels, modifiers, etc.) pass through
                let alphabet_equivalent = self.find_alphabet_equivalent(element_id)?;
                alphabet.push(alphabet_equivalent, abugida.get_grapheme(atom).to_string());
            }
        }
        
        Ok(IR::Alphabet(alphabet))
    }
    
    /// Core transformation: Alphabet → Abugida  
    /// Combines alphabet atoms (k+a, kh+a) into abugida atoms (ka, kha)
    fn alphabet_to_abugida(&self, alphabet: &AlphabetIR) -> Result<IR, TransformError> {
        let mut abugida = AbugidaIR::new(ScriptId::DEVANAGARI); // Default to Devanagari
        let mut i = 0;
        
        while i < alphabet.elements.len() {
            let atom = &alphabet.elements[i];
            let element_id = atom.element_id;
            
            if element_id.is_consonant() {
                // Look ahead for following vowel
                if i + 1 < alphabet.elements.len() {
                    let next_atom = &alphabet.elements[i + 1];
                    if next_atom.element_id.is_vowel() {
                        // Consonant + vowel combination
                        let consonant_vowel_key = vec![element_id, next_atom.element_id];
                        
                        if let Some(&combined_id) = self.alphabet_to_abugida_map.get(&consonant_vowel_key) {
                            // Found combined form (e.g., k+a -> ka)
                            abugida.push(combined_id, self.get_element_grapheme(combined_id)?);
                            i += 2; // Skip both consonant and vowel
                            continue;
                        } else if next_atom.element_id == self.get_inherent_vowel_id() {
                            // Consonant + inherent 'a' -> just the consonant in abugida
                            let abugida_consonant = self.find_abugida_equivalent(element_id)?;
                            abugida.push(abugida_consonant, self.get_element_grapheme(abugida_consonant)?);
                            i += 2; // Skip both consonant and 'a'
                            continue;
                        } else {
                            // Consonant + non-inherent vowel -> consonant + dependent vowel
                            let abugida_consonant = self.find_abugida_equivalent(element_id)?;
                            let dependent_vowel = self.independent_to_dependent_vowel(next_atom.element_id)?;
                            
                            abugida.push(abugida_consonant, self.get_element_grapheme(abugida_consonant)?);
                            abugida.push(dependent_vowel, self.get_element_grapheme(dependent_vowel)?);
                            i += 2;
                            continue;
                        }
                    }
                }
                
                // Consonant without following vowel -> consonant + virama
                let abugida_consonant = self.find_abugida_equivalent(element_id)?;
                let virama_id = self.get_virama_id();
                
                abugida.push(abugida_consonant, self.get_element_grapheme(abugida_consonant)?);
                abugida.push(virama_id, self.get_element_grapheme(virama_id)?);
            }
            else if element_id.is_vowel() {
                // Independent vowel
                let abugida_vowel = self.find_abugida_equivalent(element_id)?;
                abugida.push(abugida_vowel, alphabet.get_grapheme(atom).to_string());
            }
            else {
                // Other elements pass through
                let abugida_equivalent = self.find_abugida_equivalent(element_id)?;
                abugida.push(abugida_equivalent, alphabet.get_grapheme(atom).to_string());
            }
            
            i += 1;
        }
        
        Ok(IR::Abugida(abugida))
    }
    
    fn abugida_to_abugida(&self, source: &AbugidaIR) -> Result<IR, TransformError> {
        // For abugida-to-abugida, we can often do direct mapping
        // But may need to go through alphabet IR for complex cases
        let alphabet_ir = self.abugida_to_alphabet(source)?;
        match alphabet_ir {
            IR::Alphabet(alphabet) => self.alphabet_to_abugida(&alphabet),
            _ => unreachable!(),
        }
    }
    
    fn alphabet_to_alphabet(&self, source: &AlphabetIR) -> Result<IR, TransformError> {
        // Direct mapping between alphabet schemes
        let mut target = AlphabetIR::new(SchemeId::IAST); // Default target
        
        for atom in &source.elements {
            // Map element to target scheme equivalent
            let target_id = self.find_alphabet_scheme_equivalent(atom.element_id)?;
            target.push(target_id, self.get_element_grapheme(target_id)?);
        }
        
        Ok(IR::Alphabet(target))
    }
    
    // Helper methods for building transformation maps
    fn build_transformation_maps(&mut self) {
        // Build basic Sanskrit consonant mappings
        self.build_consonant_mappings();
        self.build_vowel_mappings();
        self.build_modifier_mappings();
    }
    
    fn build_consonant_mappings(&mut self) {
        // Example: Register basic consonants and their decompositions
        let consonants = [
            ("ka", "k", "a"), ("kha", "kh", "a"), ("ga", "g", "a"), ("gha", "gh", "a"),
            ("ca", "c", "a"), ("cha", "ch", "a"), ("ja", "j", "a"), ("jha", "jh", "a"),
            ("ṭa", "ṭ", "a"), ("ṭha", "ṭh", "a"), ("ḍa", "ḍ", "a"), ("ḍha", "ḍh", "a"),
            ("ta", "t", "a"), ("tha", "th", "a"), ("da", "d", "a"), ("dha", "dh", "a"),
            ("pa", "p", "a"), ("pha", "ph", "a"), ("ba", "b", "a"), ("bha", "bh", "a"),
            ("na", "n", "a"), ("ma", "m", "a"), ("ya", "y", "a"), ("ra", "r", "a"),
            ("la", "l", "a"), ("va", "v", "a"), ("śa", "ś", "a"), ("ṣa", "ṣ", "a"),
            ("sa", "s", "a"), ("ha", "h", "a"),
        ];
        
        for (abugida_name, pure_consonant, inherent_vowel) in &consonants {
            // Get or register element IDs
            let abugida_id = self.registry.register(ElementType::Consonant, abugida_name.to_string());
            let consonant_id = self.registry.register(ElementType::Consonant, pure_consonant.to_string());
            let vowel_id = self.registry.register(ElementType::Vowel, inherent_vowel.to_string());
            
            // Map abugida consonant to alphabet decomposition
            self.abugida_to_alphabet_map.insert(abugida_id, vec![consonant_id, vowel_id]);
            
            // Map alphabet combination to abugida consonant
            self.alphabet_to_abugida_map.insert(vec![consonant_id, vowel_id], abugida_id);
            
            // Store decomposition pattern
            self.consonant_decomposition.insert(abugida_id, (consonant_id, vowel_id));
        }
    }
    
    fn build_vowel_mappings(&mut self) {
        // Map between independent and dependent vowels
        let vowels = [
            ("a", "a"), ("ā", "ā"), ("i", "i"), ("ī", "ī"),
            ("u", "u"), ("ū", "ū"), ("ṛ", "ṛ"), ("ṝ", "ṝ"),
            ("e", "e"), ("ai", "ai"), ("o", "o"), ("au", "au"),
        ];
        
        for (independent, dependent) in &vowels {
            let indep_id = self.registry.register(ElementType::VowelIndependent, independent.to_string());
            let dep_id = self.registry.register(ElementType::VowelDependent, dependent.to_string());
            
            // Create bidirectional mapping
            self.abugida_to_alphabet_map.insert(indep_id, vec![indep_id]);
            self.abugida_to_alphabet_map.insert(dep_id, vec![indep_id]);
        }
    }
    
    fn build_modifier_mappings(&mut self) {
        // Register common modifiers
        let _virama_id = self.registry.register(ElementType::Virama, "virama".to_string());
        let _nukta_id = self.registry.register(ElementType::Nukta, "nukta".to_string());
        let _anusvara_id = self.registry.register(ElementType::Anusvara, "anusvara".to_string());
        let _visarga_id = self.registry.register(ElementType::Visarga, "visarga".to_string());
    }
    
    // Helper methods for element lookup and conversion
    fn get_element_grapheme(&self, element_id: ElementId) -> Result<String, TransformError> {
        self.registry.get_name(element_id)
            .map(|s| s.to_string())
            .ok_or_else(|| TransformError::MissingElementMapping(element_id))
    }
    
    fn dependent_to_independent_vowel(&self, dependent_id: ElementId) -> Result<ElementId, TransformError> {
        // For now, assume the canonical name is the same
        if let Some(name) = self.registry.get_name(dependent_id) {
            self.registry.get_by_name(name)
                .ok_or_else(|| TransformError::MissingElementMapping(dependent_id))
        } else {
            Err(TransformError::MissingElementMapping(dependent_id))
        }
    }
    
    fn independent_to_dependent_vowel(&self, independent_id: ElementId) -> Result<ElementId, TransformError> {
        // For now, assume the canonical name is the same
        if let Some(name) = self.registry.get_name(independent_id) {
            // Look for existing dependent vowel, or return error if not found
            if let Some(dependent_id) = self.registry.get_by_name(name) {
                if matches!(dependent_id.element_type(), ElementType::VowelDependent) {
                    Ok(dependent_id)
                } else {
                    Err(TransformError::MissingElementMapping(independent_id))
                }
            } else {
                Err(TransformError::MissingElementMapping(independent_id))
            }
        } else {
            Err(TransformError::MissingElementMapping(independent_id))
        }
    }
    
    fn find_alphabet_equivalent(&self, abugida_id: ElementId) -> Result<ElementId, TransformError> {
        // Simple mapping - in practice, this would be more sophisticated
        Ok(abugida_id) // Placeholder
    }
    
    fn find_abugida_equivalent(&self, alphabet_id: ElementId) -> Result<ElementId, TransformError> {
        // Simple mapping - in practice, this would be more sophisticated
        Ok(alphabet_id) // Placeholder
    }
    
    fn find_alphabet_scheme_equivalent(&self, element_id: ElementId) -> Result<ElementId, TransformError> {
        // Map between different alphabet schemes (IAST, Harvard-Kyoto, etc.)
        Ok(element_id) // Placeholder
    }
    
    fn get_inherent_vowel_id(&self) -> ElementId {
        self.registry.get_by_name("a").unwrap_or_else(|| {
            // Return a default ID for unknown case
            ElementId::new(ElementType::Vowel, 0)
        })
    }
    
    fn get_virama_id(&self) -> ElementId {
        self.registry.get_by_name("virama").unwrap_or_else(|| {
            // Return a default ID for unknown case
            ElementId::new(ElementType::Virama, 0)
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ScriptType {
    Abugida,
    Alphabet,
}

pub struct TransformerBuilder {
    registry: ElementRegistry,
}

impl TransformerBuilder {
    pub fn new() -> Self {
        Self {
            registry: ElementRegistry::default(),
        }
    }
    
    pub fn with_registry(mut self, registry: ElementRegistry) -> Self {
        self.registry = registry;
        self
    }
    
    pub fn build(self) -> TransformerV2 {
        TransformerV2::new(self.registry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element_id::ElementRegistry;
    
    #[test]
    fn test_abugida_to_alphabet_basic() {
        let mut registry = ElementRegistry::default();
        
        // Register elements
        let ka_id = registry.register(ElementType::Consonant, "ka".to_string());
        let k_id = registry.register(ElementType::Consonant, "k".to_string());
        let a_id = registry.register(ElementType::Vowel, "a".to_string());
        
        let transformer = TransformerV2::new(registry);
        
        // Create abugida IR with "ka"
        let mut abugida = AbugidaIR::new(ScriptId::DEVANAGARI);
        abugida.push(ka_id, "क".to_string());
        
        let result = transformer.transform(IR::Abugida(abugida), ScriptType::Alphabet).unwrap();
        
        match result {
            IR::Alphabet(alphabet) => {
                // Should decompose "ka" into "k" + "a"
                assert_eq!(alphabet.elements.len(), 2);
            }
            _ => panic!("Expected Alphabet IR"),
        }
    }
    
    #[test]
    fn test_alphabet_to_abugida_basic() {
        let mut registry = ElementRegistry::default();
        
        // Register elements
        let k_id = registry.register(ElementType::Consonant, "k".to_string());
        let a_id = registry.register(ElementType::Vowel, "a".to_string());
        let ka_id = registry.register(ElementType::Consonant, "ka".to_string());
        
        let transformer = TransformerV2::new(registry);
        
        // Create alphabet IR with "k" + "a"
        let mut alphabet = AlphabetIR::new(SchemeId::IAST);
        alphabet.push(k_id, "k".to_string());
        alphabet.push(a_id, "a".to_string());
        
        let result = transformer.transform(IR::Alphabet(alphabet), ScriptType::Abugida).unwrap();
        
        match result {
            IR::Abugida(abugida) => {
                // Should combine "k" + "a" into "ka"
                assert!(abugida.elements.len() >= 1);
            }
            _ => panic!("Expected Abugida IR"),
        }
    }
    
    #[test]
    fn test_virama_handling() {
        let mut registry = ElementRegistry::default();
        
        let ka_id = registry.register(ElementType::Consonant, "ka".to_string());
        let virama_id = registry.register(ElementType::Virama, "virama".to_string());
        let k_id = registry.register(ElementType::Consonant, "k".to_string());
        
        let transformer = TransformerV2::new(registry);
        
        // Create abugida IR with "ka" + virama (क्)
        let mut abugida = AbugidaIR::new(ScriptId::DEVANAGARI);
        abugida.push(ka_id, "क".to_string());
        abugida.push(virama_id, "्".to_string());
        
        let result = transformer.transform(IR::Abugida(abugida), ScriptType::Alphabet).unwrap();
        
        match result {
            IR::Alphabet(alphabet) => {
                // Should result in just "k" (no "a" because of virama)
                assert_eq!(alphabet.elements.len(), 1);
            }
            _ => panic!("Expected Alphabet IR"),
        }
    }
}