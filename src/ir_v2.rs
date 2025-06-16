use std::collections::HashMap;
use crate::element_id::{ElementId, ElementType, ElementRegistry};

/// Compact element representation optimized for large texts
#[derive(Clone, Debug)]
pub struct Element {
    pub id: ElementId,                    // 4 bytes - typed identifier
    pub grapheme_offset: u32,             // 4 bytes - offset into string pool
    pub properties: SmallPropertySet,     // 8 bytes - most elements have 0-2 properties
}

/// Space-optimized property storage
/// Most elements have 0-2 properties, so we optimize for that case
#[derive(Clone, Debug)]
pub enum SmallPropertySet {
    Empty,                                           // 0 properties
    One(PropertyKey, PropertyValue),                 // 1 property  
    Two(PropertyKey, PropertyValue, PropertyKey, PropertyValue), // 2 properties
    Many(HashMap<PropertyKey, PropertyValue>),       // 3+ properties (rare)
}

/// Property keys are interned for memory efficiency
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PropertyKey(u32);

#[derive(Clone, Debug, PartialEq)]
pub enum PropertyValue {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    F32(f32),
    StringRef(u32),  // Offset into string pool
}

/// String pool for memory-efficient string storage
#[derive(Clone, Debug)]
pub struct StringPool {
    strings: Vec<String>,
    lookup: HashMap<String, u32>,
}

impl StringPool {
    pub fn new() -> Self {
        Self {
            strings: Vec::new(),
            lookup: HashMap::new(),
        }
    }
    
    pub fn intern(&mut self, s: String) -> u32 {
        if let Some(&offset) = self.lookup.get(&s) {
            return offset;
        }
        
        let offset = self.strings.len() as u32;
        self.lookup.insert(s.clone(), offset);
        self.strings.push(s);
        offset
    }
    
    pub fn get(&self, offset: u32) -> Option<&str> {
        self.strings.get(offset as usize).map(|s| s.as_str())
    }
    
    pub fn get_or_empty(&self, offset: u32) -> &str {
        self.get(offset).unwrap_or("")
    }
}

/// Abugida IR - atomic units include inherent vowels
#[derive(Clone, Debug)]
pub struct AbugidaIR {
    pub elements: Vec<AbugidaAtom>,
    pub string_pool: StringPool,
    pub script_id: ScriptId,
    pub metadata: IRMetadata,
}

/// Alphabet IR - atomic units are pure consonants/vowels
#[derive(Clone, Debug)]
pub struct AlphabetIR {
    pub elements: Vec<AlphabetAtom>,
    pub string_pool: StringPool,
    pub scheme_id: SchemeId,
    pub metadata: IRMetadata,
}

/// Abugida atomic unit - may represent consonant+vowel combination
#[derive(Copy, Clone, Debug)]
pub struct AbugidaAtom {
    pub element_id: ElementId,        // What this represents (ka, kha, etc.)
    pub grapheme_offset: u32,         // Surface form in string pool
    pub modifiers: ModifierSet,       // Applied modifiers (nukta, accents, etc.)
}

/// Alphabet atomic unit - simpler, no inherent vowel semantics
#[derive(Copy, Clone, Debug)]
pub struct AlphabetAtom {
    pub element_id: ElementId,        // What this represents (k, a, etc.)
    pub grapheme_offset: u32,         // Surface form in string pool
}

/// Compact representation of modifiers using bitsets
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ModifierSet(u32);

impl ModifierSet {
    pub fn new() -> Self {
        Self(0)
    }
    
    pub fn with_modifier(mut self, modifier_id: ElementId) -> Self {
        if modifier_id.is_modifier() {
            let bit = modifier_id.id() as u32;
            if bit < 32 {
                self.0 |= 1 << bit;
            }
        }
        self
    }
    
    pub fn has_modifier(&self, modifier_id: ElementId) -> bool {
        if modifier_id.is_modifier() {
            let bit = modifier_id.id() as u32;
            bit < 32 && (self.0 & (1 << bit)) != 0
        } else {
            false
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

/// Script/Scheme identifiers
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ScriptId(u16);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SchemeId(u16);

impl ScriptId {
    pub const DEVANAGARI: Self = Self(0);
    pub const TELUGU: Self = Self(1);
    pub const TAMIL: Self = Self(2);
    pub const KANNADA: Self = Self(3);
    pub const MALAYALAM: Self = Self(4);
    pub const BENGALI: Self = Self(5);
    pub const GUJARATI: Self = Self(6);
    
    pub fn new(id: u16) -> Self {
        Self(id)
    }
    
    pub fn id(self) -> u16 {
        self.0
    }
}

impl SchemeId {
    pub const IAST: Self = Self(0);
    pub const HARVARD_KYOTO: Self = Self(1);
    pub const SLP1: Self = Self(2);
    pub const ISO15919: Self = Self(3);
    pub const ITRANS: Self = Self(4);
    
    pub fn new(id: u16) -> Self {
        Self(id)
    }
    
    pub fn id(self) -> u16 {
        self.0
    }
}

#[derive(Clone, Debug, Default)]
pub struct IRMetadata {
    pub source_info: Option<String>,
    pub warnings: Vec<String>,
    pub statistics: IRStatistics,
    pub custom_properties: SmallPropertySet,
}

#[derive(Clone, Debug, Default)]
pub struct IRStatistics {
    pub total_elements: usize,
    pub unique_graphemes: usize,
    pub modifier_usage: HashMap<ElementId, u32>,
    pub processing_time_ns: u64,
}

/// Unified IR type for the public API
#[derive(Clone, Debug)]
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
    
    pub fn element_count(&self) -> usize {
        match self {
            IR::Abugida(ir) => ir.elements.len(),
            IR::Alphabet(ir) => ir.elements.len(),
        }
    }
    
    pub fn memory_usage(&self) -> MemoryUsage {
        match self {
            IR::Abugida(ir) => {
                MemoryUsage {
                    element_data: ir.elements.len() * std::mem::size_of::<AbugidaAtom>(),
                    string_pool: ir.string_pool.strings.iter().map(|s| s.len()).sum::<usize>(),
                    metadata: std::mem::size_of_val(&ir.metadata),
                }
            }
            IR::Alphabet(ir) => {
                MemoryUsage {
                    element_data: ir.elements.len() * std::mem::size_of::<AlphabetAtom>(),
                    string_pool: ir.string_pool.strings.iter().map(|s| s.len()).sum::<usize>(),
                    metadata: std::mem::size_of_val(&ir.metadata),
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryUsage {
    pub element_data: usize,
    pub string_pool: usize,
    pub metadata: usize,
}

impl MemoryUsage {
    pub fn total(&self) -> usize {
        self.element_data + self.string_pool + self.metadata
    }
}

impl AbugidaIR {
    pub fn new(script_id: ScriptId) -> Self {
        Self {
            elements: Vec::new(),
            string_pool: StringPool::new(),
            script_id,
            metadata: IRMetadata::default(),
        }
    }
    
    pub fn push(&mut self, element_id: ElementId, grapheme: String) -> AbugidaAtom {
        let grapheme_offset = self.string_pool.intern(grapheme);
        let atom = AbugidaAtom {
            element_id,
            grapheme_offset,
            modifiers: ModifierSet::new(),
        };
        self.elements.push(atom);
        atom
    }
    
    pub fn push_with_modifiers(&mut self, element_id: ElementId, grapheme: String, modifiers: ModifierSet) -> AbugidaAtom {
        let grapheme_offset = self.string_pool.intern(grapheme);
        let atom = AbugidaAtom {
            element_id,
            grapheme_offset,
            modifiers,
        };
        self.elements.push(atom);
        atom
    }
    
    pub fn get_grapheme(&self, atom: &AbugidaAtom) -> &str {
        self.string_pool.get_or_empty(atom.grapheme_offset)
    }
}

impl AlphabetIR {
    pub fn new(scheme_id: SchemeId) -> Self {
        Self {
            elements: Vec::new(),
            string_pool: StringPool::new(),
            scheme_id,
            metadata: IRMetadata::default(),
        }
    }
    
    pub fn push(&mut self, element_id: ElementId, grapheme: String) -> AlphabetAtom {
        let grapheme_offset = self.string_pool.intern(grapheme);
        let atom = AlphabetAtom {
            element_id,
            grapheme_offset,
        };
        self.elements.push(atom);
        atom
    }
    
    pub fn get_grapheme(&self, atom: &AlphabetAtom) -> &str {
        self.string_pool.get_or_empty(atom.grapheme_offset)
    }
}

impl SmallPropertySet {
    pub fn new() -> Self {
        Self::Empty
    }
    
    pub fn with_property(self, key: PropertyKey, value: PropertyValue) -> Self {
        match self {
            Self::Empty => Self::One(key, value),
            Self::One(k1, v1) => Self::Two(k1, v1, key, value),
            Self::Two(k1, v1, k2, v2) => {
                let mut map = HashMap::new();
                map.insert(k1, v1);
                map.insert(k2, v2);
                map.insert(key, value);
                Self::Many(map)
            }
            Self::Many(mut map) => {
                map.insert(key, value);
                Self::Many(map)
            }
        }
    }
    
    pub fn get(&self, key: PropertyKey) -> Option<&PropertyValue> {
        match self {
            Self::Empty => None,
            Self::One(k, v) if *k == key => Some(v),
            Self::One(_, _) => None,
            Self::Two(k1, v1, k2, v2) => {
                if *k1 == key { Some(v1) }
                else if *k2 == key { Some(v2) }
                else { None }
            }
            Self::Many(map) => map.get(&key),
        }
    }
}

impl Default for SmallPropertySet {
    fn default() -> Self {
        Self::Empty
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_efficiency() {
        // Test that our structures are memory efficient
        assert_eq!(std::mem::size_of::<AbugidaAtom>(), 12); // 4 + 4 + 4 bytes
        assert_eq!(std::mem::size_of::<AlphabetAtom>(), 8);  // 4 + 4 bytes
        assert_eq!(std::mem::size_of::<ElementId>(), 4);
        assert_eq!(std::mem::size_of::<ModifierSet>(), 4);
    }
    
    #[test]
    fn test_string_pool() {
        let mut pool = StringPool::new();
        
        let offset1 = pool.intern("ka".to_string());
        let offset2 = pool.intern("kha".to_string());
        let offset3 = pool.intern("ka".to_string()); // Should reuse
        
        assert_eq!(offset1, offset3); // Deduplication
        assert_ne!(offset1, offset2);
        
        assert_eq!(pool.get(offset1), Some("ka"));
        assert_eq!(pool.get(offset2), Some("kha"));
    }
    
    #[test]
    fn test_modifier_set() {
        use crate::element_id::ElementRegistry;
        
        let mut registry = ElementRegistry::default();
        let nukta_id = registry.register(ElementType::Nukta, "nukta".to_string());
        let virama_id = registry.register(ElementType::Virama, "virama".to_string());
        
        let modifiers = ModifierSet::new()
            .with_modifier(nukta_id)
            .with_modifier(virama_id);
        
        assert!(modifiers.has_modifier(nukta_id));
        assert!(modifiers.has_modifier(virama_id));
        assert!(!modifiers.is_empty());
    }
    
    #[test]
    fn test_abugida_ir_construction() {
        use crate::element_id::ElementRegistry;
        
        let mut registry = ElementRegistry::default();
        let ka_id = registry.register(ElementType::Consonant, "ka".to_string());
        
        let mut ir = AbugidaIR::new(ScriptId::DEVANAGARI);
        let atom = ir.push(ka_id, "क".to_string());
        
        assert_eq!(ir.elements.len(), 1);
        assert_eq!(ir.get_grapheme(&atom), "क");
        assert_eq!(atom.element_id, ka_id);
    }
}