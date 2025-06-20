// Comprehensive benchmark comparing Shlesha with Vidyut, Dharmamitra, and Aksharamukha
// 
// To run: cargo bench --bench comprehensive_comparison --features compare-vidyut

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use std::time::Duration;

// Shlesha import
use shlesha::LosslessTransliterator;

// Vidyut import (when feature enabled)
#[cfg(feature = "compare-vidyut")]
use vidyut_lipi::{Mapping, Scheme, transliterate};

/// Test corpus with varying complexity levels
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

/// Benchmark Shlesha (lossless architecture)
fn bench_shlesha(c: &mut Criterion) {
    let transliterator = LosslessTransliterator::new();
    let test_corpus = get_test_corpus();
    
    let mut group = c.benchmark_group("shlesha_devanagari_to_iast");
    group.measurement_time(Duration::from_secs(10));
    
    for (name, text, size) in test_corpus.iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            &text,
            |b, text| {
                b.iter(|| {
                    transliterator.transliterate(
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

/// Benchmark Vidyut
#[cfg(feature = "compare-vidyut")]
fn bench_vidyut(c: &mut Criterion) {
    let mapping = Mapping::new(Scheme::Devanagari, Scheme::Iast);
    let test_corpus = get_test_corpus();
    
    let mut group = c.benchmark_group("vidyut_devanagari_to_iast");
    group.measurement_time(Duration::from_secs(10));
    
    for (name, text, size) in test_corpus.iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            &text,
            |b, text| {
                b.iter(|| {
                    transliterate(black_box(text), &mapping)
                })
            },
        );
    }
    group.finish();
}

/// Comparative benchmark: Shlesha vs Vidyut side-by-side
#[cfg(feature = "compare-vidyut")]
fn bench_comparison(c: &mut Criterion) {
    let shlesha_trans = LosslessTransliterator::new();
    let vidyut_mapping = Mapping::new(Scheme::Devanagari, Scheme::Iast);
    let test_corpus = get_test_corpus();
    
    let mut group = c.benchmark_group("comparison");
    group.measurement_time(Duration::from_secs(10));
    
    for (name, text, size) in test_corpus.iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        
        // Benchmark Shlesha
        group.bench_with_input(
            BenchmarkId::new("shlesha", name),
            &text,
            |b, text| {
                b.iter(|| {
                    shlesha_trans.transliterate(
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
            &text,
            |b, text| {
                b.iter(|| {
                    transliterate(black_box(text), &vidyut_mapping)
                })
            },
        );
    }
    group.finish();
}

/// Feature analysis benchmark - measuring specific capabilities
fn bench_features(c: &mut Criterion) {
    let transliterator = LosslessTransliterator::new();
    
    let mut group = c.benchmark_group("feature_analysis");
    
    // Test 1: Lossless verification overhead
    let test_text = "धर्मक्षेत्रे कुरुक्षेत्रे";
    let encoded = transliterator.transliterate(test_text, "Devanagari", "IAST").unwrap();
    
    group.bench_function("lossless_verification", |b| {
        b.iter(|| {
            transliterator.verify_lossless(
                black_box(test_text),
                black_box(&encoded),
                black_box("Devanagari")
            )
        })
    });
    
    // Test 2: Token extraction performance
    let text_with_tokens = "कर्म [1:ॐ] धर्म [1:ॐ:om] योग";
    group.bench_function("token_extraction", |b| {
        b.iter(|| {
            transliterator.extract_tokens(black_box(text_with_tokens))
        })
    });
    
    // Test 3: Pattern matching impact
    let pattern_heavy = "क्षत्रिय ज्ञान श्रीमान्";
    group.bench_function("pattern_heavy_text", |b| {
        b.iter(|| {
            transliterator.transliterate(
                black_box(pattern_heavy),
                black_box("Devanagari"),
                black_box("IAST")
            )
        })
    });
    
    // Test 4: Edge case handling
    let edge_cases = "ॐ मणिपद्मे हूं । ॥ १२३४५ test";
    group.bench_function("edge_cases", |b| {
        b.iter(|| {
            transliterator.transliterate(
                black_box(edge_cases),
                black_box("Devanagari"),
                black_box("IAST")
            )
        })
    });
    
    group.finish();
}

// Configure benchmarks based on available features
#[cfg(feature = "compare-vidyut")]
criterion_group!(
    benches, 
    bench_shlesha, 
    bench_vidyut, 
    bench_comparison,
    bench_features
);

#[cfg(not(feature = "compare-vidyut"))]
criterion_group!(
    benches, 
    bench_shlesha,
    bench_features
);

criterion_main!(benches);