use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use shlesha::{
    phoneme_parser::PhonemeParser,
    zero_alloc_generator::ZeroAllocGenerator,
    schema_parser::{Schema, ScriptType},
};

const TEST_TEXT: &str = "कर्म धर्म योग गुरु शांति";

/// Our new zero-allocation phoneme system
struct PhonemeSystem {
    parser: PhonemeParser,
    generator: ZeroAllocGenerator,
}

impl PhonemeSystem {
    fn new() -> Self {
        let mut parser = PhonemeParser::new();
        let generator = ZeroAllocGenerator::new();
        
        let devanagari_schema = Schema {
            name: "Devanagari".to_string(),
            script_type: ScriptType::Abugida,
            element_types: HashMap::new(),
            mappings: HashMap::new(),
            extensions: HashMap::new(),
            metadata: None,
        };
        
        let slp1_schema = Schema {
            name: "SLP1".to_string(),
            script_type: ScriptType::Alphabet,
            element_types: HashMap::new(),
            mappings: HashMap::new(),
            extensions: HashMap::new(),
            metadata: None,
        };
        
        parser.load_schema(devanagari_schema);
        parser.load_schema(slp1_schema);
        
        Self { parser, generator }
    }
    
    fn transliterate(&mut self, text: &str, from_script: &str, to_script: &str) -> String {
        let phonemes = self.parser.parse_to_phonemes(text, from_script).unwrap();
        self.generator.generate(&phonemes, to_script).unwrap()
    }
}

/// Simple mapping (baseline)
struct SimpleMapping {
    mapping: HashMap<char, &'static str>,
}

impl SimpleMapping {
    fn new() -> Self {
        let mut mapping = HashMap::new();
        mapping.insert('क', "k");
        mapping.insert('र', "r");
        mapping.insert('्', "");
        mapping.insert('म', "m");
        mapping.insert(' ', " ");
        mapping.insert('ध', "D");
        mapping.insert('य', "y");
        mapping.insert('ो', "o");
        mapping.insert('ग', "g");
        mapping.insert('ु', "u");
        mapping.insert('श', "S");
        mapping.insert('ा', "A");
        mapping.insert('ं', "M");
        mapping.insert('त', "t");
        mapping.insert('ि', "i");
        
        Self { mapping }
    }
    
    fn transliterate(&self, text: &str) -> String {
        text.chars().map(|c| *self.mapping.get(&c).unwrap_or(&"?")).collect()
    }
}

fn bench_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("transliteration_comparison");
    
    // Our phoneme system
    group.bench_function("phoneme_system", |b| {
        let mut system = PhonemeSystem::new();
        b.iter(|| {
            black_box(system.transliterate(black_box(TEST_TEXT), "Devanagari", "SLP1"))
        });
    });
    
    // Simple mapping baseline
    group.bench_function("simple_mapping", |b| {
        let system = SimpleMapping::new();
        b.iter(|| {
            black_box(system.transliterate(black_box(TEST_TEXT)))
        });
    });
    
    group.finish();
}

criterion_group!(benches, bench_comparison);
criterion_main!(benches);