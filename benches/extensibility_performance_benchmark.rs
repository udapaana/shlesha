//! Runtime Extensibility Performance Benchmark
//! 
//! Measures the performance impact of runtime extensibility features:
//! - Core LosslessTransliterator (baseline)
//! - ExtendedTransliterator with no custom scripts (overhead measurement)
//! - ExtendedTransliterator with custom scripts (full system)
//! - Custom script creation and lookup operations
//!
//! To run: cargo bench --bench extensibility_performance_benchmark

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use std::time::Duration;
use shlesha::{LosslessTransliterator, ExtendedTransliterator, CustomScriptBuilder, FallbackStrategy};

/// Test corpus matching other benchmarks for consistency
fn get_test_corpus() -> Vec<(&'static str, String, usize)> {
    let very_long = "धृतराष्ट्र उवाच । धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः । मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय ॥ ".repeat(10);
    let extreme = "कर्मण्येवाधिकारस्ते मा फलेषु कदाचन । मा कर्मफलहेतुर्भूर्मा ते सङ्गोऽस्त्वकर्मणि ॥ ".repeat(100);
    
    vec![
        // (name, text, approx_char_count)  
        ("single_char", "क".to_string(), 1),
        ("single_word", "नमस्ते".to_string(), 6),
        ("short_sentence", "अहं संस्कृतं वदामि".to_string(), 18),
        ("medium_text", "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः".to_string(), 42),
        ("long_text", 
         "तत्र शूरा महेष्वासा भीमार्जुनसमा युधि । युयुधानो विराटश्च द्रुपदश्च महारथः ॥ \
          धृतराष्ट्र उवाच । धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः । मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय ॥".to_string(), 
         200),
        ("very_long_text", very_long, 1000),
        ("extreme_text", extreme, 10000),
    ]
}

/// Benchmark core LosslessTransliterator (baseline performance)
fn bench_core_transliterator(c: &mut Criterion) {
    let transliterator = LosslessTransliterator::new();
    let test_corpus = get_test_corpus();
    
    let mut group = c.benchmark_group("core_lossless_transliterator");
    group.measurement_time(Duration::from_secs(10));
    
    for (name, text, size) in test_corpus.iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            &text,
            |b, text| {
                b.iter(|| {
                    black_box(transliterator.transliterate(
                        black_box(text), 
                        "Devanagari", 
                        "IAST"
                    ).unwrap())
                })
            },
        );
    }
    
    group.finish();
}

/// Benchmark ExtendedTransliterator with no custom scripts (measures overhead)
fn bench_extended_transliterator_baseline(c: &mut Criterion) {
    let transliterator = ExtendedTransliterator::new();
    let test_corpus = get_test_corpus();
    
    let mut group = c.benchmark_group("extended_transliterator_baseline");
    group.measurement_time(Duration::from_secs(10));
    
    for (name, text, size) in test_corpus.iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            &text,
            |b, text| {
                b.iter(|| {
                    black_box(transliterator.transliterate(
                        black_box(text), 
                        "Devanagari", 
                        "IAST"
                    ).unwrap())
                })
            },
        );
    }
    
    group.finish();
}

/// Benchmark ExtendedTransliterator with custom scripts loaded
fn bench_extended_transliterator_with_custom(c: &mut Criterion) {
    let mut transliterator = ExtendedTransliterator::new();
    
    // Add several custom scripts to test realistic overhead
    let greek_script = CustomScriptBuilder::new("AncientGreek", 100)
        .add_mappings(&[
            ('α', "a"), ('β', "b"), ('γ', "g"), ('δ', "d"), ('ε', "e"),
            ('ζ', "z"), ('η', "ē"), ('θ', "th"), ('ι', "i"), ('κ', "k"),
            ('λ', "l"), ('μ', "m"), ('ν', "n"), ('ξ', "x"), ('ο', "o"),
            ('π', "p"), ('ρ', "r"), ('σ', "s"), ('τ', "t"), ('υ', "y"),
            ('φ', "ph"), ('χ', "ch"), ('ψ', "ps"), ('ω', "ō"),
        ])
        .add_patterns(&[
            ("αι", "ai"), ("ει", "ei"), ("οι", "oi"), 
            ("αυ", "au"), ("ευ", "eu"), ("ου", "ou"),
        ])
        .with_fallback_strategy(FallbackStrategy::PreserveWithPhonetics)
        .build();
    
    let math_script = CustomScriptBuilder::new("MathNotation", 101)
        .add_mappings(&[
            ('α', "alpha"), ('β', "beta"), ('γ', "gamma"), ('δ', "delta"),
            ('ε', "epsilon"), ('λ', "lambda"), ('μ', "mu"), ('π', "pi"),
            ('σ', "sigma"), ('τ', "tau"), ('φ', "phi"), ('ω', "omega"),
        ])
        .add_patterns(&[
            ("∞", "infinity"), ("∑", "sum"), ("∏", "product"), 
            ("∫", "integral"), ("∂", "partial"), ("∇", "nabla"),
        ])
        .build();
    
    let ascii_script = CustomScriptBuilder::new("ASCII_Roman", 102)
        .add_mappings(&[
            ('क', "ka"), ('ख', "kha"), ('ग', "ga"), ('घ', "gha"),
            ('च', "cha"), ('छ', "chha"), ('ज', "ja"), ('झ', "jha"),
            ('त', "ta"), ('थ', "tha"), ('द', "da"), ('ध', "dha"),
            ('प', "pa"), ('फ', "pha"), ('ब', "ba"), ('भ', "bha"),
        ])
        .add_patterns(&[
            ("क्ष", "ksha"), ("ज्ञ", "gnja"), ("त्र", "tra"), ("श्र", "shra"),
        ])
        .build();
    
    transliterator.add_custom_script(greek_script);
    transliterator.add_custom_script(math_script);
    transliterator.add_custom_script(ascii_script);
    
    let test_corpus = get_test_corpus();
    
    let mut group = c.benchmark_group("extended_transliterator_with_custom");
    group.measurement_time(Duration::from_secs(10));
    
    for (name, text, size) in test_corpus.iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            &text,
            |b, text| {
                b.iter(|| {
                    black_box(transliterator.transliterate(
                        black_box(text), 
                        "Devanagari", 
                        "IAST"
                    ).unwrap())
                })
            },
        );
    }
    
    group.finish();
}

/// Benchmark custom script creation and lookup operations
fn bench_custom_script_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("custom_script_operations");
    group.measurement_time(Duration::from_secs(5));
    
    // Benchmark script creation
    group.bench_function("script_creation_small", |b| {
        b.iter(|| {
            black_box(
                CustomScriptBuilder::new("TestScript", 200)
                    .add_mapping('α', "a")
                    .add_mapping('β', "b")
                    .add_mapping('γ', "g")
                    .build()
            )
        })
    });
    
    group.bench_function("script_creation_large", |b| {
        b.iter(|| {
            black_box(
                CustomScriptBuilder::new("TestScript", 200)
                    .add_mappings(&[
                        ('α', "a"), ('β', "b"), ('γ', "g"), ('δ', "d"), ('ε', "e"),
                        ('ζ', "z"), ('η', "ē"), ('θ', "th"), ('ι', "i"), ('κ', "k"),
                        ('λ', "l"), ('μ', "m"), ('ν', "n"), ('ξ', "x"), ('ο', "o"),
                        ('π', "p"), ('ρ', "r"), ('σ', "s"), ('τ', "t"), ('υ', "y"),
                        ('φ', "ph"), ('χ', "ch"), ('ψ', "ps"), ('ω', "ō"),
                    ])
                    .add_patterns(&[
                        ("αι", "ai"), ("ει", "ei"), ("οι", "oi"), 
                        ("αυ", "au"), ("ευ", "eu"), ("ου", "ou"),
                        ("αβ", "ab"), ("γδ", "gd"), ("εζ", "ez"),
                    ])
                    .with_fallback_strategy(FallbackStrategy::PreserveWithPhonetics)
                    .build()
            )
        })
    });
    
    // Benchmark lookups
    let script = CustomScriptBuilder::new("TestScript", 200)
        .add_mappings(&[
            ('α', "a"), ('β', "b"), ('γ', "g"), ('δ', "d"), ('ε', "e"),
            ('ζ', "z"), ('η', "ē"), ('θ', "th"), ('ι', "i"), ('κ', "k"),
        ])
        .add_patterns(&[
            ("αι", "ai"), ("ει", "ei"), ("οι", "oi"), ("αυ", "au"),
        ])
        .build();
    
    group.bench_function("char_lookup_hit", |b| {
        b.iter(|| {
            black_box(script.lookup_char(black_box('α')))
        })
    });
    
    group.bench_function("char_lookup_miss", |b| {
        b.iter(|| {
            black_box(script.lookup_char(black_box('ℵ')))
        })
    });
    
    group.bench_function("pattern_lookup_hit", |b| {
        b.iter(|| {
            black_box(script.lookup_pattern(black_box("αιδιος"), black_box(0)))
        })
    });
    
    group.bench_function("pattern_lookup_miss", |b| {
        b.iter(|| {
            black_box(script.lookup_pattern(black_box("xyz"), black_box(0)))
        })
    });
    
    // Benchmark transliterator creation
    group.bench_function("extended_transliterator_creation", |b| {
        b.iter(|| {
            black_box(ExtendedTransliterator::new())
        })
    });
    
    group.finish();
}

/// Benchmark extension management operations
fn bench_extension_management(c: &mut Criterion) {
    let mut group = c.benchmark_group("extension_management");
    group.measurement_time(Duration::from_secs(5));
    
    // Setup transliterator with multiple scripts
    let mut transliterator = ExtendedTransliterator::new();
    
    for i in 0..10 {
        let script = CustomScriptBuilder::new(&format!("Script{}", i), 100 + i)
            .add_mapping('a', "alpha")
            .add_mapping('b', "beta")
            .build();
        transliterator.add_custom_script(script);
    }
    
    group.bench_function("list_custom_scripts", |b| {
        b.iter(|| {
            black_box(transliterator.extensions().list_custom_scripts())
        })
    });
    
    group.bench_function("check_custom_mapping", |b| {
        b.iter(|| {
            black_box(transliterator.extensions().has_custom_mapping(black_box(100), black_box(101)))
        })
    });
    
    group.bench_function("get_custom_script", |b| {
        b.iter(|| {
            black_box(transliterator.extensions().get_script(black_box(105)))
        })
    });
    
    group.finish();
}

/// Optional: Compare with Vidyut if feature enabled
#[cfg(feature = "compare-vidyut")]
fn bench_vidyut_vs_extensible(c: &mut Criterion) {
    use vidyut_lipi::{Mapping, Scheme, transliterate};
    
    let test_corpus = get_test_corpus();
    let mut group = c.benchmark_group("vidyut_vs_extensible_comparison");
    group.measurement_time(Duration::from_secs(10));
    
    // Setup systems
    let core_transliterator = LosslessTransliterator::new();
    let extended_transliterator = ExtendedTransliterator::new();
    let vidyut_mapping = Mapping::new(Scheme::Devanagari, Scheme::Iast).unwrap();
    
    for (name, text, size) in test_corpus.iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        
        // Shlesha Core
        group.bench_with_input(
            BenchmarkId::new("shlesha_core", name),
            &text,
            |b, text| {
                b.iter(|| {
                    black_box(core_transliterator.transliterate(
                        black_box(text), 
                        "Devanagari", 
                        "IAST"
                    ).unwrap())
                })
            },
        );
        
        // Shlesha Extended
        group.bench_with_input(
            BenchmarkId::new("shlesha_extended", name),
            &text,
            |b, text| {
                b.iter(|| {
                    black_box(extended_transliterator.transliterate(
                        black_box(text), 
                        "Devanagari", 
                        "IAST"
                    ).unwrap())
                })
            },
        );
        
        // Vidyut
        group.bench_with_input(
            BenchmarkId::new("vidyut", name),
            &text,
            |b, text| {
                b.iter(|| {
                    black_box(transliterate(black_box(text), &vidyut_mapping))
                })
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    extensibility_benches,
    bench_core_transliterator,
    bench_extended_transliterator_baseline,
    bench_extended_transliterator_with_custom,
    bench_custom_script_operations,
    bench_extension_management
);

#[cfg(feature = "compare-vidyut")]
criterion_group!(
    vidyut_comparison,
    bench_vidyut_vs_extensible
);

#[cfg(not(feature = "compare-vidyut"))]
criterion_main!(extensibility_benches);

#[cfg(feature = "compare-vidyut")]
criterion_main!(extensibility_benches, vidyut_comparison);