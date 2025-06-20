use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::collections::HashMap;
use shlesha::{
    phoneme_parser::PhonemeParser,
    zero_alloc_generator::ZeroAllocGenerator,
    transliterator::Transliterator,
    schema_parser::{Schema, ScriptType},
};

// Test data with varying complexity
const SIMPLE_TEXT: &str = "क";
const MEDIUM_TEXT: &str = "कर्म धर्म";
const COMPLEX_TEXT: &str = "कर्म धर्म योग गुरु शांति प्रकृति संस्कृत";

const SIMPLE_SLP1: &str = "k";
const MEDIUM_SLP1: &str = "karma Darma";
const COMPLEX_SLP1: &str = "karma Darma yoga guru SAnti prakfti saMskfta";

/// Our new zero-allocation phoneme system
struct PhonemeSystem {
    parser: PhonemeParser,
    generator: ZeroAllocGenerator,
}

impl PhonemeSystem {
    fn new() -> Self {
        let mut parser = PhonemeParser::new();
        let generator = ZeroAllocGenerator::new();
        
        // Load minimal schemas
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
    
    fn transliterate(&mut self, text: &str, from_script: &str, to_script: &str) -> Result<String, Box<dyn std::error::Error>> {
        let phonemes = self.parser.parse_to_phonemes(text, from_script)?;
        let result = self.generator.generate(&phonemes, to_script)?;
        Ok(result)
    }
}

/// Simple character mapping approach (baseline)
struct SimpleMapping {
    devanagari_to_slp1: HashMap<char, &'static str>,
    slp1_to_devanagari: HashMap<&'static str, char>,
}

impl SimpleMapping {
    fn new() -> Self {
        let mut devanagari_to_slp1 = HashMap::new();
        let mut slp1_to_devanagari = HashMap::new();
        
        // Basic mappings
        devanagari_to_slp1.insert('क', "k");
        devanagari_to_slp1.insert('ख', "K");
        devanagari_to_slp1.insert('ग', "g");
        devanagari_to_slp1.insert('घ', "G");
        devanagari_to_slp1.insert('ङ', "N");
        devanagari_to_slp1.insert('च', "c");
        devanagari_to_slp1.insert('छ', "C");
        devanagari_to_slp1.insert('ज', "j");
        devanagari_to_slp1.insert('झ', "J");
        devanagari_to_slp1.insert('ञ', "Y");
        devanagari_to_slp1.insert('ट', "w");
        devanagari_to_slp1.insert('ठ', "W");
        devanagari_to_slp1.insert('ड', "q");
        devanagari_to_slp1.insert('ढ', "Q");
        devanagari_to_slp1.insert('ण', "R");
        devanagari_to_slp1.insert('त', "t");
        devanagari_to_slp1.insert('थ', "T");
        devanagari_to_slp1.insert('द', "d");
        devanagari_to_slp1.insert('ध', "D");
        devanagari_to_slp1.insert('न', "n");
        devanagari_to_slp1.insert('प', "p");
        devanagari_to_slp1.insert('फ', "P");
        devanagari_to_slp1.insert('ब', "b");
        devanagari_to_slp1.insert('भ', "B");
        devanagari_to_slp1.insert('म', "m");
        devanagari_to_slp1.insert('य', "y");
        devanagari_to_slp1.insert('र', "r");
        devanagari_to_slp1.insert('ल', "l");
        devanagari_to_slp1.insert('व', "v");
        devanagari_to_slp1.insert('श', "S");
        devanagari_to_slp1.insert('ष', "z");
        devanagari_to_slp1.insert('स', "s");
        devanagari_to_slp1.insert('ह', "h");
        devanagari_to_slp1.insert('अ', "a");
        devanagari_to_slp1.insert('आ', "A");
        devanagari_to_slp1.insert('इ', "i");
        devanagari_to_slp1.insert('ई', "I");
        devanagari_to_slp1.insert('उ', "u");
        devanagari_to_slp1.insert('ऊ', "U");
        devanagari_to_slp1.insert('ऋ', "f");
        devanagari_to_slp1.insert('ए', "e");
        devanagari_to_slp1.insert('ऐ', "Y");
        devanagari_to_slp1.insert('ओ', "o");
        devanagari_to_slp1.insert('औ', "V");
        devanagari_to_slp1.insert('ं', "M");
        devanagari_to_slp1.insert('ः', "H");
        devanagari_to_slp1.insert('्', "");
        devanagari_to_slp1.insert(' ', " ");
        devanagari_to_slp1.insert('।', ".");
        
        // Reverse mappings
        for (&dev_char, &slp1_str) in &devanagari_to_slp1 {
            if !slp1_str.is_empty() {
                slp1_to_devanagari.insert(slp1_str, dev_char);
            }
        }
        
        Self {
            devanagari_to_slp1,
            slp1_to_devanagari,
        }
    }
    
    fn devanagari_to_slp1(&self, text: &str) -> String {
        let mut result = String::new();
        for ch in text.chars() {
            if let Some(&slp1) = self.devanagari_to_slp1.get(&ch) {
                result.push_str(slp1);
            } else {
                result.push(ch); // Unknown character, keep as-is
            }
        }
        result
    }
    
    fn slp1_to_devanagari(&self, text: &str) -> String {
        let mut result = String::new();
        let mut chars = text.chars().peekable();
        
        while let Some(ch) = chars.next() {
            let ch_str = ch.to_string();
            if let Some(&dev_char) = self.slp1_to_devanagari.get(ch_str.as_str()) {
                result.push(dev_char);
            } else {
                result.push(ch); // Unknown character, keep as-is
            }
        }
        result
    }
}

fn bench_phoneme_system(c: &mut Criterion) {
    let mut group = c.benchmark_group("phoneme_system");
    
    for (name, devanagari_text, slp1_text) in [
        ("simple", SIMPLE_TEXT, SIMPLE_SLP1),
        ("medium", MEDIUM_TEXT, MEDIUM_SLP1),
        ("complex", COMPLEX_TEXT, COMPLEX_SLP1),
    ] {
        group.bench_with_input(
            BenchmarkId::new("dev_to_slp1", name),
            &devanagari_text,
            |b, &text| {
                let mut system = PhonemeSystem::new();
                b.iter(|| {
                    black_box(system.transliterate(black_box(text), "Devanagari", "SLP1").unwrap())
                });
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("slp1_to_dev", name),
            &slp1_text,
            |b, &text| {
                let mut system = PhonemeSystem::new();
                b.iter(|| {
                    black_box(system.transliterate(black_box(text), "SLP1", "Devanagari").unwrap())
                });
            },
        );
    }
    
    group.finish();
}

fn bench_simple_mapping(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_mapping");
    let system = SimpleMapping::new();
    
    for (name, devanagari_text, slp1_text) in [
        ("simple", SIMPLE_TEXT, SIMPLE_SLP1),
        ("medium", MEDIUM_TEXT, MEDIUM_SLP1),
        ("complex", COMPLEX_TEXT, COMPLEX_SLP1),
    ] {
        group.bench_with_input(
            BenchmarkId::new("dev_to_slp1", name),
            &devanagari_text,
            |b, &text| {
                b.iter(|| {
                    black_box(system.devanagari_to_slp1(black_box(text)))
                });
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("slp1_to_dev", name),
            &slp1_text,
            |b, &text| {
                b.iter(|| {
                    black_box(system.slp1_to_devanagari(black_box(text)))
                });
            },
        );
    }
    
    group.finish();
}

fn bench_round_trip(c: &mut Criterion) {
    let mut group = c.benchmark_group("round_trip");
    
    for (name, text) in [
        ("simple", SIMPLE_TEXT),
        ("medium", MEDIUM_TEXT), 
        ("complex", COMPLEX_TEXT),
    ] {
        // Phoneme system round trip
        group.bench_with_input(
            BenchmarkId::new("phoneme_system", name),
            &text,
            |b, &text| {
                let mut system = PhonemeSystem::new();
                b.iter(|| {
                    let slp1 = system.transliterate(black_box(text), "Devanagari", "SLP1").unwrap();
                    let back = system.transliterate(&slp1, "SLP1", "Devanagari").unwrap();
                    black_box(back)
                });
            },
        );
        
        // Simple mapping round trip
        group.bench_with_input(
            BenchmarkId::new("simple_mapping", name),
            &text,
            |b, &text| {
                let system = SimpleMapping::new();
                b.iter(|| {
                    let slp1 = system.devanagari_to_slp1(black_box(text));
                    let back = system.slp1_to_devanagari(&slp1);
                    black_box(back)
                });
            },
        );
    }
    
    group.finish();
}

fn bench_compare_approaches(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_approaches");
    group.sample_size(50); // Fewer samples for faster comparison
    
    let text = COMPLEX_TEXT; // Use complex text for comparison
    
    // Our zero-allocation phoneme system
    group.bench_function("phoneme_system", |b| {
        let mut system = PhonemeSystem::new();
        b.iter(|| {
            black_box(system.transliterate(black_box(text), "Devanagari", "SLP1").unwrap())
        });
    });
    
    // Simple mapping (fastest baseline)
    group.bench_function("simple_mapping", |b| {
        let system = SimpleMapping::new();
        b.iter(|| {
            black_box(system.devanagari_to_slp1(black_box(text)))
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_phoneme_system,
    bench_simple_mapping,
    bench_round_trip,
    bench_compare_approaches
);

criterion_main!(benches);