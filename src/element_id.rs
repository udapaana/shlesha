use std::fmt;

/// Compact element identifier optimized for performance
/// Layout: [type_tag: 8 bits][id: 24 bits] = 32 bits total
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ElementId(u32);

impl ElementId {
    const TYPE_MASK: u32 = 0xFF00_0000;
    const ID_MASK: u32 = 0x00FF_FFFF;
    const TYPE_SHIFT: u32 = 24;
    
    pub fn new(element_type: ElementType, id: u32) -> Self {
        debug_assert!(id <= Self::ID_MASK, "Element ID too large: {}", id);
        Self((element_type as u32) << Self::TYPE_SHIFT | (id & Self::ID_MASK))
    }
    
    pub fn element_type(self) -> ElementType {
        ElementType::from_u8(((self.0 & Self::TYPE_MASK) >> Self::TYPE_SHIFT) as u8)
    }
    
    pub fn id(self) -> u32 {
        self.0 & Self::ID_MASK
    }
    
    pub fn is_consonant(self) -> bool {
        matches!(self.element_type(), ElementType::Consonant)
    }
    
    pub fn is_vowel(self) -> bool {
        matches!(self.element_type(), ElementType::Vowel | ElementType::VowelDependent | ElementType::VowelIndependent)
    }
    
    pub fn is_modifier(self) -> bool {
        matches!(self.element_type(), ElementType::Modifier | ElementType::Virama | ElementType::Nukta | ElementType::Anusvara | ElementType::Visarga)
    }
}

/// Element type enumeration - fits in 8 bits
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ElementType {
    // Core types (0-15)
    Consonant = 0,
    Vowel = 1,
    VowelIndependent = 2,
    VowelDependent = 3,
    
    // Modifiers (16-31)
    Modifier = 16,
    Virama = 17,
    Nukta = 18,
    Anusvara = 19,
    Visarga = 20,
    Avagraha = 21,
    
    // Accents (32-47)
    Accent = 32,
    AccentUdatta = 33,
    AccentAnudatta = 34,
    AccentSvarita = 35,
    
    // Other (48-63)
    Numeral = 48,
    Punctuation = 49,
    Whitespace = 50,
    
    // Extensible range (64-255)
    Extension = 64,
    Unknown = 255,
}

impl ElementType {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Consonant,
            1 => Self::Vowel,
            2 => Self::VowelIndependent,
            3 => Self::VowelDependent,
            16 => Self::Modifier,
            17 => Self::Virama,
            18 => Self::Nukta,
            19 => Self::Anusvara,
            20 => Self::Visarga,
            21 => Self::Avagraha,
            32 => Self::Accent,
            33 => Self::AccentUdatta,
            34 => Self::AccentAnudatta,
            35 => Self::AccentSvarita,
            48 => Self::Numeral,
            49 => Self::Punctuation,
            50 => Self::Whitespace,
            64 => Self::Extension,
            255 => Self::Unknown,
            _ => Self::Extension, // Unknown types become extensions
        }
    }
    
    pub fn is_core_type(self) -> bool {
        (self as u8) < 64
    }
}

impl fmt::Debug for ElementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ElementId({:?}, {})", self.element_type(), self.id())
    }
}

impl fmt::Display for ElementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}:{}", self.element_type(), self.id())
    }
}

/// Registry for managing element IDs and their associated data
#[derive(Clone, Debug)]
pub struct ElementRegistry {
    next_ids: [u32; 256], // Next available ID for each element type
    names: Vec<String>,   // String pool for element names
    name_to_id: std::collections::HashMap<String, ElementId>,
    id_to_name_index: std::collections::HashMap<ElementId, usize>,
}

impl ElementRegistry {
    pub fn new() -> Self {
        Self {
            next_ids: [0; 256],
            names: Vec::new(),
            name_to_id: std::collections::HashMap::new(),
            id_to_name_index: std::collections::HashMap::new(),
        }
    }
    
    pub fn register(&mut self, element_type: ElementType, name: String) -> ElementId {
        if let Some(&existing_id) = self.name_to_id.get(&name) {
            return existing_id;
        }
        
        let type_index = element_type as u8 as usize;
        let id = self.next_ids[type_index];
        self.next_ids[type_index] += 1;
        
        let element_id = ElementId::new(element_type, id);
        
        let name_index = self.names.len();
        self.names.push(name.clone());
        self.name_to_id.insert(name, element_id);
        self.id_to_name_index.insert(element_id, name_index);
        
        element_id
    }
    
    pub fn get_by_name(&self, name: &str) -> Option<ElementId> {
        self.name_to_id.get(name).copied()
    }
    
    pub fn get_name(&self, id: ElementId) -> Option<&str> {
        self.id_to_name_index.get(&id)
            .and_then(|&index| self.names.get(index))
            .map(|s| s.as_str())
    }
    
    pub fn register_extension(&mut self, name: String) -> ElementId {
        self.register(ElementType::Extension, name)
    }
}

impl Default for ElementRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        
        // Pre-register common elements for performance
        registry.register(ElementType::Virama, "virama".to_string());
        registry.register(ElementType::Nukta, "nukta".to_string());
        registry.register(ElementType::Anusvara, "anusvara".to_string());
        registry.register(ElementType::Visarga, "visarga".to_string());
        registry.register(ElementType::Avagraha, "avagraha".to_string());
        
        registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_element_id_packing() {
        let id = ElementId::new(ElementType::Consonant, 12345);
        assert_eq!(id.element_type(), ElementType::Consonant);
        assert_eq!(id.id(), 12345);
    }
    
    #[test]
    fn test_element_id_size() {
        assert_eq!(std::mem::size_of::<ElementId>(), 4);
        assert_eq!(std::mem::size_of::<ElementType>(), 1);
    }
    
    #[test]
    fn test_registry() {
        let mut registry = ElementRegistry::new();
        
        let ka_id = registry.register(ElementType::Consonant, "ka".to_string());
        let kha_id = registry.register(ElementType::Consonant, "kha".to_string());
        
        assert_ne!(ka_id, kha_id);
        assert_eq!(registry.get_by_name("ka"), Some(ka_id));
        assert_eq!(registry.get_name(ka_id), Some("ka"));
        
        // Test deduplication
        let ka_id2 = registry.register(ElementType::Consonant, "ka".to_string());
        assert_eq!(ka_id, ka_id2);
    }
    
    #[test]
    fn test_element_type_queries() {
        let consonant = ElementId::new(ElementType::Consonant, 1);
        let vowel = ElementId::new(ElementType::Vowel, 1);
        let virama = ElementId::new(ElementType::Virama, 1);
        
        assert!(consonant.is_consonant());
        assert!(!consonant.is_vowel());
        assert!(!consonant.is_modifier());
        
        assert!(vowel.is_vowel());
        assert!(!vowel.is_consonant());
        
        assert!(virama.is_modifier());
        assert!(!virama.is_consonant());
    }
}