//! Core benchmarks for Shlesha performance testing
//! 
//! This file contains the essential benchmarks for measuring:
//! - Lossless transliterator performance
//! - Cross-system comparison
//! - Multi-script support

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use shlesha::LosslessTransliterator;

fn get_test_corpus() -> Vec<(&'static str, String, usize)> {
    let very_long_text = "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः । मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय ॥ ".repeat(100);
    
    vec![
        // (name, text, approximate_word_count)
        ("single_word", "धर्म".to_string(), 1),
        ("short_phrase", "धर्मक्षेत्रे कुरुक्षेत्रे".to_string(), 2),
        ("complex_clusters", "क्ष्म्य त्र्य ज्ञ श्र".to_string(), 4),
        ("medium_text", "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः । मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय ॥".to_string(), 15),
        ("long_text", "तत्र शूरा महेष्वासा भीमार्जुनसमा युधि । युयुधानो विराटश्च द्रुपदश्च महारथः ॥ धृष्टकेतुश्चेकितानः काशिराजश्च वीर्यवान् । पुरुजित्कुन्तिभोजश्च शैब्यश्च महारथः ॥".to_string(), 30),
        ("very_long_text", very_long_text, 1500),
    ]
}

fn setup_lossless_transliterator() -> LosslessTransliterator {
    LosslessTransliterator::new()
}

fn bench_lossless_system(c: &mut Criterion) {
    let transliterator = setup_lossless_transliterator();
    let test_corpus = get_test_corpus();
    
    let mut group = c.benchmark_group("lossless_transliterator");
    
    for (name, text, _word_count) in test_corpus.iter() {
        group.bench_with_input(
            BenchmarkId::new("devanagari_to_iast", name),
            text,
            |b, text| {
                b.iter(|| {
                    transliterator.transliterate(
                        black_box(text),
                        black_box("Devanagari"),
                        black_box("IAST")
                    ).unwrap()
                })
            },
        );
    }
    
    group.finish();
}

fn bench_lossless_verification(c: &mut Criterion) {
    let transliterator = setup_lossless_transliterator();
    let test_cases = vec![
        ("simple", "धर्म"),
        ("complex", "क्ष्म्य त्र्य ज्ञ"),
        ("mixed", "ॐ मणि पद्मे हूँ"),
        ("long", "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः । मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय ॥"),
    ];
    
    let mut group = c.benchmark_group("lossless_verification");
    
    for (name, text) in test_cases.iter() {
        let result = transliterator.transliterate(text, "Devanagari", "IAST").unwrap();
        group.bench_with_input(
            BenchmarkId::new("verify_lossless", name),
            text,
            |b, text| {
                b.iter(|| {
                    transliterator.verify_lossless(
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

fn bench_throughput(c: &mut Criterion) {
    let lossless_transliterator = setup_lossless_transliterator();
    
    // Create different sized texts for throughput testing
    let verse = "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः । मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय ॥ ";
    let sizes = vec![
        ("1KB", verse.repeat(10)),
        ("10KB", verse.repeat(100)),
        ("100KB", verse.repeat(1000)),
    ];
    
    let mut group = c.benchmark_group("throughput");
    
    for (size_name, text) in sizes.iter() {
        let text_size = text.len();
        group.throughput(criterion::Throughput::Bytes(text_size as u64));
        
        // Lossless system throughput
        group.bench_with_input(
            BenchmarkId::new("lossless", size_name),
            &text,
            |b, text| {
                b.iter(|| {
                    lossless_transliterator.transliterate(
                        black_box(text),
                        black_box("Devanagari"),
                        black_box("IAST")
                    ).unwrap()
                })
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches, 
    bench_lossless_system, 
    bench_lossless_verification,
    bench_throughput
);
criterion_main!(benches);