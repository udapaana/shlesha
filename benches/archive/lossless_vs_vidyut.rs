//! Benchmark comparing the new lossless-first architecture against Vidyut
//! 
//! This benchmark specifically tests our new LosslessTransliterator against
//! the current state-of-the-art Vidyut library to demonstrate performance improvements.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use shlesha::LosslessTransliterator;

#[cfg(feature = "compare-vidyut")]
use vidyut_lipi::{Mapping, Scheme, transliterate};

fn get_test_corpus() -> Vec<(&'static str, &'static str)> {
    vec![
        ("single_word", "धर्म"),
        ("short_phrase", "धर्मक्षेत्रे कुरुक्षेत्रे"),
        ("complex_clusters", "क्ष्म्य त्र्य ज्ञ श्र"),
        ("medium_text", "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः । मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय ॥"),
        ("long_text", "तत्र शूरा महेष्वासा भीमार्जुनसमा युधि । युयुधानो विराटश्च द्रुपदश्च महारथः ॥ धृष्टकेतुश्चेकितानः काशिराजश्च वीर्यवान् । पुरुजित्कुन्तिभोजश्च शैब्यश्च महारथः ॥"),
        ("very_long_text", Box::leak("धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः । मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय ॥ ".repeat(50).into_boxed_str()) as &'static str),
    ]
}

fn setup_old_shlesha() -> Transliterator {
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema = SchemaParser::parse_str(iast_schema).unwrap();
    
    TransliteratorBuilder::new()
        .with_schema(dev_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build()
}

fn setup_lossless_shlesha() -> LosslessTransliterator {
    LosslessTransliterator::new()
}

#[cfg(feature = "compare-vidyut")]
fn bench_all_systems(c: &mut Criterion) {
    let old_transliterator = setup_old_shlesha();
    let lossless_transliterator = setup_lossless_shlesha();
    let vidyut_mapping = Mapping::new(Scheme::Devanagari, Scheme::Iast);
    let test_corpus = get_test_corpus();
    
    let mut group = c.benchmark_group("transliteration_systems");
    
    for (name, text) in test_corpus.iter() {
        // Benchmark Old Shlesha (Bidirectional IR-based)
        group.bench_with_input(
            BenchmarkId::new("old_shlesha", name),
            text,
            |b, text| {
                b.iter(|| {
                    old_transliterator.transliterate(
                        black_box(text),
                        black_box("Devanagari"),
                        black_box("IAST")
                    )
                })
            },
        );
        
        // Benchmark New Lossless Shlesha
        group.bench_with_input(
            BenchmarkId::new("lossless_shlesha", name),
            text,
            |b, text| {
                b.iter(|| {
                    lossless_transliterator.transliterate(
                        black_box(text),
                        black_box("Devanagari"),
                        black_box("IAST")
                    )
                })
            },
        );
        
        // Benchmark Vidyut
        group.bench_with_input(
            BenchmarkId::new("vidyut", name),
            text,
            |b, text| {
                b.iter(|| {
                    transliterate(
                        black_box(text),
                        black_box(&vidyut_mapping)
                    )
                })
            },
        );
    }
    
    group.finish();
}

#[cfg(not(feature = "compare-vidyut"))]
fn bench_shlesha_systems(c: &mut Criterion) {
    let old_transliterator = setup_old_shlesha();
    let lossless_transliterator = setup_lossless_shlesha();
    let test_corpus = get_test_corpus();
    
    let mut group = c.benchmark_group("shlesha_comparison");
    
    for (name, text) in test_corpus.iter() {
        // Benchmark Old Shlesha (Bidirectional IR-based)
        group.bench_with_input(
            BenchmarkId::new("old_shlesha", name),
            text,
            |b, text| {
                b.iter(|| {
                    old_transliterator.transliterate(
                        black_box(text),
                        black_box("Devanagari"),
                        black_box("IAST")
                    )
                })
            },
        );
        
        // Benchmark New Lossless Shlesha
        group.bench_with_input(
            BenchmarkId::new("lossless_shlesha", name),
            text,
            |b, text| {
                b.iter(|| {
                    lossless_transliterator.transliterate(
                        black_box(text),
                        black_box("Devanagari"),
                        black_box("IAST")
                    )
                })
            },
        );
    }
    
    group.finish();
}

// Throughput benchmark
#[cfg(feature = "compare-vidyut")]
fn bench_throughput(c: &mut Criterion) {
    let old_transliterator = setup_old_shlesha();
    let lossless_transliterator = setup_lossless_shlesha();
    let vidyut_mapping = Mapping::new(Scheme::Devanagari, Scheme::Iast);
    
    // Create large text for throughput testing
    let verse = "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः । मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय ॥ ";
    let sizes = vec![
        ("10KB", verse.repeat(100)),
        ("100KB", verse.repeat(1000)),
        ("1MB", verse.repeat(10000)),
    ];
    
    let mut group = c.benchmark_group("throughput");
    
    for (size_name, text) in sizes.iter() {
        let text_size = text.len();
        group.throughput(criterion::Throughput::Bytes(text_size as u64));
        
        // Old Shlesha throughput
        group.bench_with_input(
            BenchmarkId::new("old_shlesha", size_name),
            &text,
            |b, text| {
                b.iter(|| {
                    old_transliterator.transliterate(
                        black_box(text),
                        black_box("Devanagari"),
                        black_box("IAST")
                    )
                })
            },
        );
        
        // Lossless Shlesha throughput
        group.bench_with_input(
            BenchmarkId::new("lossless_shlesha", size_name),
            &text,
            |b, text| {
                b.iter(|| {
                    lossless_transliterator.transliterate(
                        black_box(text),
                        black_box("Devanagari"),
                        black_box("IAST")
                    )
                })
            },
        );
        
        // Vidyut throughput
        group.bench_with_input(
            BenchmarkId::new("vidyut", size_name),
            &text,
            |b, text| {
                b.iter(|| {
                    transliterate(
                        black_box(text),
                        black_box(&vidyut_mapping)
                    )
                })
            },
        );
    }
    
    group.finish();
}

// Memory usage comparison (simulated through allocation counting)
fn bench_memory_efficiency(c: &mut Criterion) {
    let old_transliterator = setup_old_shlesha();
    let lossless_transliterator = setup_lossless_shlesha();
    
    let test_text = "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः";
    
    let mut group = c.benchmark_group("memory_efficiency");
    
    // Simulate memory usage through repeated operations
    // (In real implementation, we'd use memory profiling tools)
    group.bench_function("old_shlesha_memory", |b| {
        b.iter(|| {
            for _ in 0..100 {
                let _ = old_transliterator.transliterate(
                    black_box(test_text),
                    black_box("Devanagari"),
                    black_box("IAST")
                );
            }
        })
    });
    
    group.bench_function("lossless_shlesha_memory", |b| {
        b.iter(|| {
            for _ in 0..100 {
                let _ = lossless_transliterator.transliterate(
                    black_box(test_text),
                    black_box("Devanagari"),
                    black_box("IAST")
                );
            }
        })
    });
    
    group.finish();
}

// Lossless verification benchmark
fn bench_lossless_verification(c: &mut Criterion) {
    let lossless_transliterator = setup_lossless_shlesha();
    
    let test_cases = vec![
        ("simple", "धर्म"),
        ("complex", "क्ष्म्य त्र्य ज्ञ"),
        ("mixed", "ॐ मणि पद्मे हूँ"),
        ("long", "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः । मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय ॥"),
    ];
    
    let mut group = c.benchmark_group("lossless_verification");
    
    for (name, text) in test_cases.iter() {
        group.bench_with_input(
            BenchmarkId::new("verify_lossless", name),
            text,
            |b, text| {
                let result = lossless_transliterator.transliterate(text, "Devanagari", "IAST").unwrap();
                b.iter(|| {
                    lossless_transliterator.verify_lossless(
                        black_box(text),
                        black_box(&result),
                        black_box("Devanagari")
                    )
                })
            },
        );
    }
    
    group.finish();
}

#[cfg(feature = "compare-vidyut")]
criterion_group!(
    benches, 
    bench_all_systems, 
    bench_throughput, 
    bench_memory_efficiency, 
    bench_lossless_verification
);

#[cfg(not(feature = "compare-vidyut"))]
criterion_group!(
    benches, 
    bench_shlesha_systems, 
    bench_memory_efficiency, 
    bench_lossless_verification
);

criterion_main!(benches);