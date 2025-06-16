use shlesha::element_id::*;
use shlesha::ir_v2::*;
use std::collections::HashMap;

#[cfg(test)]
mod element_id_tests {
    use super::*;

    #[test]
    fn test_element_id_bit_packing() {
        let consonant_id = ElementId::new(ElementType::Consonant, 12345);
        assert_eq!(consonant_id.element_type(), ElementType::Consonant);
        assert_eq!(consonant_id.id(), 12345);
        
        let vowel_id = ElementId::new(ElementType::VowelDependent, 67890);
        assert_eq!(vowel_id.element_type(), ElementType::VowelDependent);
        assert_eq!(vowel_id.id(), 67890);
        
        // Test maximum ID
        let max_id = ElementId::new(ElementType::Extension, 0x00FF_FFFF);
        assert_eq!(max_id.id(), 0x00FF_FFFF);
    }
    
    #[test]
    fn test_element_type_queries() {
        let consonant = ElementId::new(ElementType::Consonant, 1);
        let vowel_dep = ElementId::new(ElementType::VowelDependent, 1);
        let vowel_indep = ElementId::new(ElementType::VowelIndependent, 1);
        let virama = ElementId::new(ElementType::Virama, 1);
        let nukta = ElementId::new(ElementType::Nukta, 1);
        let accent = ElementId::new(ElementType::AccentUdatta, 1);
        
        assert!(consonant.is_consonant());
        assert!(!consonant.is_vowel());
        assert!(!consonant.is_modifier());
        
        assert!(vowel_dep.is_vowel());
        assert!(vowel_indep.is_vowel());
        assert!(!vowel_dep.is_consonant());
        
        assert!(virama.is_modifier());
        assert!(nukta.is_modifier());
        assert!(!virama.is_consonant());
        
        assert!(!accent.is_modifier()); // AccentUdatta is not in the modifier category
    }
    
    #[test]
    fn test_element_registry_basic() {
        let mut registry = ElementRegistry::new();
        
        let ka_id = registry.register(ElementType::Consonant, "ka".to_string());
        let kha_id = registry.register(ElementType::Consonant, "kha".to_string());
        let a_id = registry.register(ElementType::VowelIndependent, "a".to_string());
        
        assert_ne!(ka_id, kha_id);
        assert_ne!(ka_id, a_id);
        
        assert_eq!(registry.get_by_name("ka"), Some(ka_id));
        assert_eq!(registry.get_by_name("kha"), Some(kha_id));
        assert_eq!(registry.get_by_name("a"), Some(a_id));
        assert_eq!(registry.get_by_name("nonexistent"), None);
        
        assert_eq!(registry.get_name(ka_id), Some("ka"));
        assert_eq!(registry.get_name(kha_id), Some("kha"));
        assert_eq!(registry.get_name(a_id), Some("a"));
    }
    
    #[test]
    fn test_element_registry_deduplication() {
        let mut registry = ElementRegistry::new();
        
        let ka_id1 = registry.register(ElementType::Consonant, "ka".to_string());
        let ka_id2 = registry.register(ElementType::Consonant, "ka".to_string());
        
        assert_eq!(ka_id1, ka_id2);
        
        // Different types should get different IDs even with same name
        let ka_vowel_id = registry.register(ElementType::VowelIndependent, "ka".to_string());
        assert_ne!(ka_id1, ka_vowel_id);
    }
    
    #[test]
    fn test_element_registry_extensions() {
        let mut registry = ElementRegistry::new();
        
        let ext1_id = registry.register_extension("vedic_tone_marker".to_string());
        let ext2_id = registry.register_extension("manuscript_variant".to_string());
        
        assert_eq!(ext1_id.element_type(), ElementType::Extension);
        assert_eq!(ext2_id.element_type(), ElementType::Extension);
        assert_ne!(ext1_id, ext2_id);
        
        assert_eq!(registry.get_name(ext1_id), Some("vedic_tone_marker"));
        assert_eq!(registry.get_name(ext2_id), Some("manuscript_variant"));
    }
    
    #[test]
    fn test_element_registry_id_allocation() {
        let mut registry = ElementRegistry::new();
        
        let c1 = registry.register(ElementType::Consonant, "k".to_string());
        let c2 = registry.register(ElementType::Consonant, "kh".to_string());
        let c3 = registry.register(ElementType::Consonant, "g".to_string());
        
        // IDs should be allocated sequentially within type
        assert_eq!(c1.id(), 0);
        assert_eq!(c2.id(), 1);
        assert_eq!(c3.id(), 2);
        
        let v1 = registry.register(ElementType::VowelIndependent, "a".to_string());
        let v2 = registry.register(ElementType::VowelIndependent, "i".to_string());
        
        // Vowel IDs should start from 0 independently
        assert_eq!(v1.id(), 0);
        assert_eq!(v2.id(), 1);
    }
}

#[cfg(test)]
mod string_pool_tests {
    use super::*;
    
    #[test]
    fn test_string_pool_basic() {
        let mut pool = StringPool::new();
        
        let offset1 = pool.intern("hello".to_string());
        let offset2 = pool.intern("world".to_string());
        let offset3 = pool.intern("hello".to_string()); // Duplicate
        
        assert_eq!(offset1, offset3); // Deduplication
        assert_ne!(offset1, offset2);
        
        assert_eq!(pool.get(offset1), Some("hello"));
        assert_eq!(pool.get(offset2), Some("world"));
        assert_eq!(pool.get(999), None);
        
        assert_eq!(pool.get_or_empty(offset1), "hello");
        assert_eq!(pool.get_or_empty(999), "");
    }
    
    #[test]
    fn test_string_pool_unicode() {
        let mut pool = StringPool::new();
        
        let devanagari = pool.intern("क".to_string());
        let telugu = pool.intern("క".to_string());
        let tamil = pool.intern("க".to_string());
        
        assert_eq!(pool.get(devanagari), Some("क"));
        assert_eq!(pool.get(telugu), Some("క"));
        assert_eq!(pool.get(tamil), Some("க"));
    }
    
    #[test]
    fn test_string_pool_memory_efficiency() {
        let mut pool = StringPool::new();
        
        // Intern the same string many times
        let text = "repeated_string".to_string();
        let mut offsets = Vec::new();
        
        for _ in 0..1000 {
            offsets.push(pool.intern(text.clone()));
        }
        
        // All offsets should be the same (deduplication)
        assert!(offsets.iter().all(|&offset| offset == offsets[0]));
        
        // Pool should contain only one copy
        assert_eq!(pool.strings.len(), 1);
    }
}

#[cfg(test)]
mod modifier_set_tests {
    use super::*;
    
    #[test]
    fn test_modifier_set_basic() {
        let mut registry = ElementRegistry::default();
        let nukta_id = registry.register(ElementType::Nukta, "nukta".to_string());
        let virama_id = registry.register(ElementType::Virama, "virama".to_string());
        let anusvara_id = registry.register(ElementType::Anusvara, "anusvara".to_string());
        
        let modifiers = ModifierSet::new()
            .with_modifier(nukta_id)
            .with_modifier(virama_id);
        
        assert!(modifiers.has_modifier(nukta_id));
        assert!(modifiers.has_modifier(virama_id));
        assert!(!modifiers.has_modifier(anusvara_id));
        assert!(!modifiers.is_empty());
        
        let empty_modifiers = ModifierSet::new();
        assert!(empty_modifiers.is_empty());
        assert!(!empty_modifiers.has_modifier(nukta_id));
    }
    
    #[test]
    fn test_modifier_set_non_modifier_elements() {
        let mut registry = ElementRegistry::default();
        let consonant_id = registry.register(ElementType::Consonant, "ka".to_string());
        let vowel_id = registry.register(ElementType::VowelIndependent, "a".to_string());
        let nukta_id = registry.register(ElementType::Nukta, "nukta".to_string());
        
        let modifiers = ModifierSet::new()
            .with_modifier(consonant_id)  // Should be ignored
            .with_modifier(vowel_id)      // Should be ignored
            .with_modifier(nukta_id);     // Should be added
        
        assert!(!modifiers.has_modifier(consonant_id));
        assert!(!modifiers.has_modifier(vowel_id));
        assert!(modifiers.has_modifier(nukta_id));
    }
    
    #[test]
    fn test_modifier_set_many_modifiers() {
        let mut registry = ElementRegistry::default();
        let mut modifier_ids = Vec::new();
        
        // Create many modifiers
        for i in 0..10 {
            let id = registry.register(ElementType::Modifier, format!("mod_{}", i));
            modifier_ids.push(id);
        }
        
        let mut modifiers = ModifierSet::new();
        for &id in &modifier_ids {
            modifiers = modifiers.with_modifier(id);
        }
        
        // All should be present
        for &id in &modifier_ids {
            assert!(modifiers.has_modifier(id));
        }
        assert!(!modifiers.is_empty());
    }
}

#[cfg(test)]
mod property_set_tests {
    use super::*;
    
    #[test]
    fn test_small_property_set_empty() {
        let props = SmallPropertySet::new();
        
        match props {
            SmallPropertySet::Empty => {}
            _ => panic!("Expected Empty variant"),
        }
        
        assert!(props.get(PropertyKey(0)).is_none());
    }
    
    #[test]
    fn test_small_property_set_one() {
        let key = PropertyKey(1);
        let value = PropertyValue::Bool(true);
        
        let props = SmallPropertySet::new()
            .with_property(key, value.clone());
        
        match props {
            SmallPropertySet::One(k, _) => assert_eq!(k, key),
            _ => panic!("Expected One variant"),
        }
        
        assert_eq!(props.get(key), Some(&PropertyValue::Bool(true)));
        assert!(props.get(PropertyKey(2)).is_none());
    }
    
    #[test]
    fn test_small_property_set_two() {
        let key1 = PropertyKey(1);
        let key2 = PropertyKey(2);
        let value1 = PropertyValue::Bool(true);
        let value2 = PropertyValue::U32(42);
        
        let props = SmallPropertySet::new()
            .with_property(key1, value1.clone())
            .with_property(key2, value2.clone());
        
        match props {
            SmallPropertySet::Two(k1, _, k2, _) => {
                assert_eq!(k1, key1);
                assert_eq!(k2, key2);
            }
            _ => panic!("Expected Two variant"),
        }
        
        assert_eq!(props.get(key1), Some(&PropertyValue::Bool(true)));
        assert_eq!(props.get(key2), Some(&PropertyValue::U32(42)));
        assert!(props.get(PropertyKey(3)).is_none());
    }
    
    #[test]
    fn test_small_property_set_many() {
        let mut props = SmallPropertySet::new()
            .with_property(PropertyKey(1), PropertyValue::Bool(true))
            .with_property(PropertyKey(2), PropertyValue::U32(42))
            .with_property(PropertyKey(3), PropertyValue::F32(3.14));
        
        match props {
            SmallPropertySet::Many(ref map) => {
                assert_eq!(map.len(), 3);
            }
            _ => panic!("Expected Many variant"),
        }
        
        assert_eq!(props.get(PropertyKey(1)), Some(&PropertyValue::Bool(true)));
        assert_eq!(props.get(PropertyKey(2)), Some(&PropertyValue::U32(42)));
        assert_eq!(props.get(PropertyKey(3)), Some(&PropertyValue::F32(3.14)));
        assert!(props.get(PropertyKey(4)).is_none());
    }
    
    #[test]
    fn test_property_value_types() {
        let bool_val = PropertyValue::Bool(true);
        let u8_val = PropertyValue::U8(255);
        let u16_val = PropertyValue::U16(65535);
        let u32_val = PropertyValue::U32(4294967295);
        let f32_val = PropertyValue::F32(3.14159);
        let string_ref_val = PropertyValue::StringRef(42);
        
        match bool_val {
            PropertyValue::Bool(true) => {}
            _ => panic!("Expected Bool(true)"),
        }
        
        match u32_val {
            PropertyValue::U32(4294967295) => {}
            _ => panic!("Expected U32(4294967295)"),
        }
    }
}

#[cfg(test)]
mod abugida_ir_tests {
    use super::*;
    
    #[test]
    fn test_abugida_ir_basic_construction() {
        let mut registry = ElementRegistry::default();
        let ka_id = registry.register(ElementType::Consonant, "ka".to_string());
        let kha_id = registry.register(ElementType::Consonant, "kha".to_string());
        
        let mut ir = AbugidaIR::new(ScriptId::DEVANAGARI);
        
        let atom1 = ir.push(ka_id, "क".to_string());
        let atom2 = ir.push(kha_id, "ख".to_string());
        
        assert_eq!(ir.elements.len(), 2);
        assert_eq!(ir.script_id, ScriptId::DEVANAGARI);
        
        assert_eq!(atom1.element_id, ka_id);
        assert_eq!(atom2.element_id, kha_id);
        
        assert_eq!(ir.get_grapheme(&atom1), "क");
        assert_eq!(ir.get_grapheme(&atom2), "ख");
    }
    
    #[test]
    fn test_abugida_ir_with_modifiers() {
        let mut registry = ElementRegistry::default();
        let ka_id = registry.register(ElementType::Consonant, "ka".to_string());
        let nukta_id = registry.register(ElementType::Nukta, "nukta".to_string());
        let virama_id = registry.register(ElementType::Virama, "virama".to_string());
        
        let modifiers = ModifierSet::new()
            .with_modifier(nukta_id)
            .with_modifier(virama_id);
        
        let mut ir = AbugidaIR::new(ScriptId::DEVANAGARI);
        let atom = ir.push_with_modifiers(ka_id, "क़्".to_string(), modifiers);
        
        assert_eq!(ir.elements.len(), 1);
        assert_eq!(atom.element_id, ka_id);
        assert!(atom.modifiers.has_modifier(nukta_id));
        assert!(atom.modifiers.has_modifier(virama_id));
        assert_eq!(ir.get_grapheme(&atom), "क़्");
    }
    
    #[test]
    fn test_abugida_ir_string_deduplication() {
        let mut registry = ElementRegistry::default();
        let ka_id = registry.register(ElementType::Consonant, "ka".to_string());
        
        let mut ir = AbugidaIR::new(ScriptId::DEVANAGARI);
        
        // Add the same grapheme multiple times
        let atom1 = ir.push(ka_id, "क".to_string());
        let atom2 = ir.push(ka_id, "क".to_string());
        let atom3 = ir.push(ka_id, "क".to_string());
        
        // Should all share the same offset (string deduplication)
        assert_eq!(atom1.grapheme_offset, atom2.grapheme_offset);
        assert_eq!(atom2.grapheme_offset, atom3.grapheme_offset);
        
        // But pool should contain only one copy
        assert_eq!(ir.string_pool.strings.len(), 1);
    }
    
    #[test]
    fn test_abugida_ir_complex_text() {
        let mut registry = ElementRegistry::default();
        
        // Register Sanskrit syllables
        let ka_id = registry.register(ElementType::Consonant, "ka".to_string());
        let r_id = registry.register(ElementType::Consonant, "ra".to_string());
        let ma_id = registry.register(ElementType::Consonant, "ma".to_string());
        let na_id = registry.register(ElementType::Consonant, "na".to_string());
        let virama_id = registry.register(ElementType::Virama, "virama".to_string());
        
        let mut ir = AbugidaIR::new(ScriptId::DEVANAGARI);
        
        // Build "karma" = क + र् + म
        ir.push(ka_id, "क".to_string());
        ir.push_with_modifiers(r_id, "र्".to_string(), 
                               ModifierSet::new().with_modifier(virama_id));
        ir.push(ma_id, "म".to_string());
        
        assert_eq!(ir.elements.len(), 3);
        
        // Check the virama modifier
        assert!(ir.elements[1].modifiers.has_modifier(virama_id));
        assert!(!ir.elements[0].modifiers.has_modifier(virama_id));
        assert!(!ir.elements[2].modifiers.has_modifier(virama_id));
    }
}

#[cfg(test)]
mod alphabet_ir_tests {
    use super::*;
    
    #[test]
    fn test_alphabet_ir_basic_construction() {
        let mut registry = ElementRegistry::default();
        let k_id = registry.register(ElementType::Consonant, "k".to_string());
        let a_id = registry.register(ElementType::Vowel, "a".to_string());
        let r_id = registry.register(ElementType::Consonant, "r".to_string());
        
        let mut ir = AlphabetIR::new(SchemeId::IAST);
        
        let atom1 = ir.push(k_id, "k".to_string());
        let atom2 = ir.push(a_id, "a".to_string());
        let atom3 = ir.push(r_id, "r".to_string());
        
        assert_eq!(ir.elements.len(), 3);
        assert_eq!(ir.scheme_id, SchemeId::IAST);
        
        assert_eq!(ir.get_grapheme(&atom1), "k");
        assert_eq!(ir.get_grapheme(&atom2), "a");
        assert_eq!(ir.get_grapheme(&atom3), "r");
    }
    
    #[test]
    fn test_alphabet_ir_diacritics() {
        let mut registry = ElementRegistry::default();
        let k_id = registry.register(ElementType::Consonant, "k".to_string());
        let a_long_id = registry.register(ElementType::Vowel, "ā".to_string());
        let m_id = registry.register(ElementType::Consonant, "m".to_string());
        
        let mut ir = AlphabetIR::new(SchemeId::IAST);
        
        // Build "kām" in IAST
        ir.push(k_id, "k".to_string());
        ir.push(a_long_id, "ā".to_string());
        ir.push(m_id, "m".to_string());
        
        assert_eq!(ir.elements.len(), 3);
        assert_eq!(ir.get_grapheme(&ir.elements[1]), "ā");
    }
    
    #[test]
    fn test_alphabet_ir_conjuncts_decomposed() {
        let mut registry = ElementRegistry::default();
        let k_id = registry.register(ElementType::Consonant, "k".to_string());
        let r_id = registry.register(ElementType::Consonant, "r".to_string());
        let s_id = registry.register(ElementType::Consonant, "s".to_string());
        let n_id = registry.register(ElementType::Consonant, "n".to_string());
        let a_id = registry.register(ElementType::Vowel, "a".to_string());
        
        let mut ir = AlphabetIR::new(SchemeId::IAST);
        
        // Build "kṛṣṇa" as decomposed atoms: k + ṛ + ṣ + ṇ + a
        ir.push(k_id, "k".to_string());
        ir.push(r_id, "ṛ".to_string());  // Note: this should probably be a vowel
        ir.push(s_id, "ṣ".to_string());
        ir.push(n_id, "ṇ".to_string());
        ir.push(a_id, "a".to_string());
        
        assert_eq!(ir.elements.len(), 5);
    }
}

#[cfg(test)]
mod unified_ir_tests {
    use super::*;
    
    #[test]
    fn test_ir_type_checking() {
        let abugida_ir = AbugidaIR::new(ScriptId::DEVANAGARI);
        let alphabet_ir = AlphabetIR::new(SchemeId::IAST);
        
        let unified_abugida = IR::Abugida(abugida_ir);
        let unified_alphabet = IR::Alphabet(alphabet_ir);
        
        assert!(unified_abugida.is_abugida());
        assert!(!unified_abugida.is_alphabet());
        
        assert!(unified_alphabet.is_alphabet());
        assert!(!unified_alphabet.is_abugida());
    }
    
    #[test]
    fn test_ir_element_count() {
        let mut registry = ElementRegistry::default();
        let ka_id = registry.register(ElementType::Consonant, "ka".to_string());
        let kha_id = registry.register(ElementType::Consonant, "kha".to_string());
        
        let mut abugida_ir = AbugidaIR::new(ScriptId::DEVANAGARI);
        abugida_ir.push(ka_id, "क".to_string());
        abugida_ir.push(kha_id, "ख".to_string());
        
        let unified_ir = IR::Abugida(abugida_ir);
        assert_eq!(unified_ir.element_count(), 2);
    }
    
    #[test]
    fn test_ir_memory_usage() {
        let mut registry = ElementRegistry::default();
        let ka_id = registry.register(ElementType::Consonant, "ka".to_string());
        
        let mut abugida_ir = AbugidaIR::new(ScriptId::DEVANAGARI);
        abugida_ir.push(ka_id, "क".to_string());
        
        let unified_ir = IR::Abugida(abugida_ir);
        let memory_usage = unified_ir.memory_usage();
        
        assert!(memory_usage.element_data > 0);
        assert!(memory_usage.string_pool > 0);
        assert!(memory_usage.total() > 0);
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    
    #[test]
    fn test_large_text_construction() {
        let mut registry = ElementRegistry::default();
        let ka_id = registry.register(ElementType::Consonant, "ka".to_string());
        let kha_id = registry.register(ElementType::Consonant, "kha".to_string());
        let ga_id = registry.register(ElementType::Consonant, "ga".to_string());
        
        let element_ids = [ka_id, kha_id, ga_id];
        let graphemes = ["क", "ख", "ग"];
        
        let mut ir = AbugidaIR::new(ScriptId::DEVANAGARI);
        
        // Add 10,000 elements
        for i in 0..10_000 {
            let idx = i % 3;
            ir.push(element_ids[idx], graphemes[idx].to_string());
        }
        
        assert_eq!(ir.elements.len(), 10_000);
        
        // String deduplication should mean only 3 unique strings
        assert_eq!(ir.string_pool.strings.len(), 3);
        
        let memory_usage = IR::Abugida(ir).memory_usage();
        
        // Memory usage should be reasonable
        println!("Memory usage for 10k elements: {} bytes", memory_usage.total());
        assert!(memory_usage.total() < 1_000_000); // Less than 1MB for 10k elements
    }
    
    #[test]
    fn test_element_id_lookup_performance() {
        let mut registry = ElementRegistry::default();
        
        // Register many elements
        let mut element_ids = Vec::new();
        for i in 0..1000 {
            let id = registry.register(ElementType::Consonant, format!("consonant_{}", i));
            element_ids.push(id);
        }
        
        // Lookup should be fast
        for (i, &id) in element_ids.iter().enumerate() {
            let name = registry.get_name(id).unwrap();
            assert_eq!(name, format!("consonant_{}", i));
        }
        
        // Reverse lookup should also be fast
        for i in 0..1000 {
            let name = format!("consonant_{}", i);
            let id = registry.get_by_name(&name).unwrap();
            assert_eq!(id, element_ids[i]);
        }
    }
    
    #[test]
    fn test_modifier_set_performance() {
        let mut registry = ElementRegistry::default();
        let mut modifier_ids = Vec::new();
        
        // Create many modifiers
        for i in 0..20 {
            let id = registry.register(ElementType::Modifier, format!("modifier_{}", i));
            modifier_ids.push(id);
        }
        
        // Build modifier set with all modifiers
        let mut modifiers = ModifierSet::new();
        for &id in &modifier_ids {
            modifiers = modifiers.with_modifier(id);
        }
        
        // Test presence checks are fast
        for &id in &modifier_ids {
            assert!(modifiers.has_modifier(id));
        }
        
        // Test that modifier set is still compact
        assert_eq!(std::mem::size_of_val(&modifiers), 4);
    }
}