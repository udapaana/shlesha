use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::collections::HashMap;
use shlesha::{
    phoneme_parser::PhonemeParser,
    zero_alloc_generator::ZeroAllocGenerator,
    schema_parser::{Schema, ScriptType},
};

// Real-world test cases with expected results
const TEST_CASES: &[(&str, &str)] = &[
    // Simple cases
    ("क", "k"),
    ("कर", "kr"),
    ("धर्म", "Drm"),
    
    // Medium complexity
    ("कर्म", "krm"), 
    ("योग", "yog"),
    ("गुरु", "guru"),
    
    // Complex cases
    ("संस्कृत", "sAskfta"),
    ("प्रकृति", "prakfti"),
    ("भगवद्गीता", "BagavdgItA"),
    
    // Very complex
    ("अध्यात्मविद्या", "aDyAtmavidyA"),
    ("सर्वधर्मान्परित्यज्य", "sarvaDarmAnparityajya"),
];

/// Our zero-allocation phoneme system
struct ShleshaPhonemeBased {
    parser: PhonemeParser,
    generator: ZeroAllocGenerator,
}

impl ShleshaPhonemeBased {
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
    
    fn transliterate(&mut self, text: &str) -> String {
        match self.parser.parse_to_phonemes(text, "Devanagari") {
            Ok(phonemes) => {
                match self.generator.generate(&phonemes, "SLP1") {
                    Ok(result) => result,
                    Err(_) => text.to_string()
                }
            },
            Err(_) => text.to_string()
        }
    }
}

/// Simple character mapping baseline 
struct SimpleCharMapping {
    mapping: HashMap<char, &'static str>,
}

impl SimpleCharMapping {
    fn new() -> Self {
        let mut mapping = HashMap::new();
        
        // Basic consonants
        mapping.insert('क', "k");
        mapping.insert('ख', "K");
        mapping.insert('ग', "g");
        mapping.insert('घ', "G");
        mapping.insert('ङ', "N");
        mapping.insert('च', "c");
        mapping.insert('छ', "C");
        mapping.insert('ज', "j");
        mapping.insert('झ', "J");
        mapping.insert('ञ', "Y");
        mapping.insert('ट', "w");
        mapping.insert('ठ', "W");
        mapping.insert('ड', "q");
        mapping.insert('ढ', "Q");
        mapping.insert('ण', "R");
        mapping.insert('त', "t");
        mapping.insert('थ', "T");
        mapping.insert('द', "d");
        mapping.insert('ध', "D");
        mapping.insert('न', "n");
        mapping.insert('प', "p");
        mapping.insert('फ', "P");
        mapping.insert('ब', "b");
        mapping.insert('भ', "B");
        mapping.insert('म', "m");
        mapping.insert('य', "y");
        mapping.insert('र', "r");
        mapping.insert('ल', "l");
        mapping.insert('व', "v");
        mapping.insert('श', "S");
        mapping.insert('ष', "z");
        mapping.insert('स', "s");
        mapping.insert('ह', "h");
        
        // Vowels
        mapping.insert('अ', "a");
        mapping.insert('आ', "A");
        mapping.insert('इ', "i");
        mapping.insert('ई', "I");
        mapping.insert('उ', "u");
        mapping.insert('ऊ', "U");
        mapping.insert('ऋ', "f");
        mapping.insert('ए', "e");
        mapping.insert('ऐ', "Y");
        mapping.insert('ओ', "o");
        mapping.insert('औ', "V");
        
        // Modifiers
        mapping.insert('ं', "M");
        mapping.insert('ः', "H");
        mapping.insert('्', "");
        mapping.insert(' ', " ");
        
        // Vowel marks
        mapping.insert('ा', "A");
        mapping.insert('ि', "i");
        mapping.insert('ी', "I");
        mapping.insert('ु', "u");
        mapping.insert('ू', "U");
        mapping.insert('ृ', "f");
        mapping.insert('े', "e");
        mapping.insert('ै', "Y");
        mapping.insert('ो', "o");
        mapping.insert('ौ', "V");
        
        Self { mapping }
    }
    
    fn transliterate(&self, text: &str) -> String {
        text.chars()
            .map(|c| *self.mapping.get(&c).unwrap_or(&""))
            .collect()
    }
}

fn bench_performance_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("performance_comparison");
    
    for (complexity, cases) in [
        ("simple", &TEST_CASES[0..3]),
        ("medium", &TEST_CASES[3..6]), 
        ("complex", &TEST_CASES[6..9]),
        ("very_complex", &TEST_CASES[9..]),
    ] {
        let combined_text: String = cases.iter().map(|(text, _)| *text).collect::<Vec<_>>().join(" ");
        
        group.bench_with_input(
            BenchmarkId::new("shlesha_phoneme", complexity),
            &combined_text,
            |b, text| {
                let mut system = ShleshaPhonemeBased::new();
                b.iter(|| {
                    black_box(system.transliterate(black_box(text)))
                });
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("simple_mapping", complexity),
            &combined_text,
            |b, text| {
                let system = SimpleCharMapping::new();
                b.iter(|| {
                    black_box(system.transliterate(black_box(text)))
                });
            },
        );
    }
    
    group.finish();
}

fn bench_accuracy_test(c: &mut Criterion) {
    let mut group = c.benchmark_group("accuracy_test");
    group.sample_size(10); // Fewer samples since this is about correctness
    
    group.bench_function("shlesha_accuracy", |b| {
        let mut system = ShleshaPhonemeBased::new();
        b.iter(|| {
            let mut correct = 0;
            let mut total = 0;
            
            for (input, expected) in TEST_CASES {
                let result = system.transliterate(input);
                if result.contains(&expected.chars().collect::<String>()) {
                    correct += 1;
                }
                total += 1;
            }
            
            black_box((correct, total))
        });
    });
    
    group.bench_function("simple_mapping_accuracy", |b| {
        let system = SimpleCharMapping::new();
        b.iter(|| {
            let mut correct = 0;
            let mut total = 0;
            
            for (input, expected) in TEST_CASES {
                let result = system.transliterate(input);
                if result.contains(&expected.chars().collect::<String>()) {
                    correct += 1;
                }
                total += 1;
            }
            
            black_box((correct, total))
        });
    });
    
    group.finish();
}

fn bench_throughput_test(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput_test");
    
    // Create a large text by repeating test cases
    let large_text = TEST_CASES.iter()
        .map(|(text, _)| *text)
        .cycle()
        .take(1000)
        .collect::<Vec<_>>()
        .join(" ");
    
    group.bench_function("shlesha_large_text", |b| {
        let mut system = ShleshaPhonemeBased::new();
        b.iter(|| {
            black_box(system.transliterate(black_box(&large_text)))
        });
    });
    
    group.bench_function("simple_mapping_large_text", |b| {
        let system = SimpleCharMapping::new();
        b.iter(|| {
            black_box(system.transliterate(black_box(&large_text)))
        });
    });
    
    group.finish();
}

fn bench_memory_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");
    
    // Test memory allocation patterns
    group.bench_function("shlesha_allocations", |b| {
        b.iter(|| {
            let mut system = ShleshaPhonemeBased::new();
            for (text, _) in TEST_CASES.iter().cycle().take(100) {
                let _result = system.transliterate(text);
            }
        });
    });
    
    group.bench_function("simple_mapping_allocations", |b| {
        b.iter(|| {
            let system = SimpleCharMapping::new();
            for (text, _) in TEST_CASES.iter().cycle().take(100) {
                let _result = system.transliterate(text);
            }
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_performance_comparison,
    bench_accuracy_test,
    bench_throughput_test,
    bench_memory_patterns
);

criterion_main!(benches);