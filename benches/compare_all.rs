use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use shlesha::{Transliterator, TransliteratorBuilder, SchemaParser};

// To run this benchmark with all comparisons:
// 1. Add to Cargo.toml:
// [dev-dependencies]
// vidyut-lipi = "0.5"
// aksharamukha = "0.2"  # if available
// dharmamitra = "0.1"   # if available
//
// 2. Run: cargo bench --bench compare_all --features compare-all

// Test corpus with increasing complexity
fn get_test_corpus() -> Vec<(&'static str, &'static str, usize)> {
    vec![
        // Simple words
        ("नमस्ते", "namaste", 1),
        ("संस्कृतम्", "saṃskṛtam", 1),
        
        // Words with conjuncts
        ("कृष्ण", "kṛṣṇa", 1),
        ("ज्ञानम्", "jñānam", 1),
        
        // Short sentences (10 words)
        ("अहं संस्कृतं पठामि", "ahaṃ saṃskṛtaṃ paṭhāmi", 3),
        ("तत्र शूरा महेष्वासा भीमार्जुनसमा युधि", "tatra śūrā maheṣvāsā bhīmārjunasamā yudhi", 7),
        
        // Medium text (50 words)
        ("धृतराष्ट्र उवाच धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय", 
         "dhṛtarāṣṭra uvāca dharmakṣetre kurukṣetre samavetā yuyutsavaḥ māmakāḥ pāṇḍavāścaiva kimakurvata sañjaya", 13),
         
        // Complex with special characters
        ("श्रीभगवानुवाच । कुतस्त्वा कश्मलमिदं विषमे समुपस्थितम् । अनार्यजुष्टमस्वर्ग्यमकीर्तिकरमर्जुन ॥",
         "śrībhagavānuvāca . kutastvā kaśmalamidaṃ viṣame samupasthitam . anāryajuṣṭamasvargyamakīrtikaramarjuna ..", 11),
    ]
}

// Large corpus for throughput testing
fn get_large_corpus() -> Vec<(&'static str, usize)> {
    let verse = "तत्र शूरा महेष्वासा भीमार्जुनसमा युधि । युयुधानो विराटश्च द्रुपदश्च महारथः ॥ ";
    vec![
        (verse, 1), // Single verse
        (&verse.repeat(10), 10), // 10 verses
        (&verse.repeat(100), 100), // 100 verses
        (&verse.repeat(1000), 1000), // 1000 verses (~100KB)
        (&verse.repeat(10000), 10000), // 10000 verses (~1MB)
    ]
}

fn setup_shlesha() -> Transliterator {
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema = SchemaParser::parse_str(iast_schema).unwrap();
    
    TransliteratorBuilder::new()
        .with_schema(dev_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build()
}

// Benchmark Shlesha
fn bench_shlesha_accuracy(c: &mut Criterion) {
    let transliterator = setup_shlesha();
    let test_corpus = get_test_corpus();
    
    let mut group = c.benchmark_group("accuracy/shlesha");
    
    for (devanagari, expected_iast, word_count) in test_corpus.iter() {
        group.bench_with_input(
            BenchmarkId::new("dev_to_iast", word_count),
            devanagari,
            |b, text| {
                b.iter(|| {
                    let result = transliterator.transliterate(
                        black_box(text),
                        black_box("Devanagari"),
                        black_box("IAST")
                    ).unwrap();
                    black_box(result);
                })
            },
        );
    }
    
    group.finish();
}

#[cfg(feature = "compare-vidyut")]
fn bench_vidyut_accuracy(c: &mut Criterion) {
    use vidyut_lipi::{Scheme, transliterate};
    
    let test_corpus = get_test_corpus();
    let mut group = c.benchmark_group("accuracy/vidyut");
    
    for (devanagari, expected_iast, word_count) in test_corpus.iter() {
        group.bench_with_input(
            BenchmarkId::new("dev_to_iast", word_count),
            devanagari,
            |b, text| {
                b.iter(|| {
                    let result = transliterate(
                        black_box(text),
                        black_box(Scheme::Devanagari),
                        black_box(Scheme::Iast)
                    );
                    black_box(result);
                })
            },
        );
    }
    
    group.finish();
}

// Throughput benchmarks
fn bench_shlesha_throughput(c: &mut Criterion) {
    let transliterator = setup_shlesha();
    let large_corpus = get_large_corpus();
    
    let mut group = c.benchmark_group("throughput/shlesha");
    
    for (text, verse_count) in large_corpus.iter() {
        let text_size = text.len();
        group.throughput(criterion::Throughput::Bytes(text_size as u64));
        
        group.bench_with_input(
            BenchmarkId::new("verses", verse_count),
            text,
            |b, text| {
                b.iter(|| {
                    let result = transliterator.transliterate(
                        black_box(text),
                        black_box("Devanagari"),
                        black_box("IAST")
                    ).unwrap();
                    black_box(result);
                })
            },
        );
    }
    
    group.finish();
}

#[cfg(feature = "compare-vidyut")]
fn bench_vidyut_throughput(c: &mut Criterion) {
    use vidyut_lipi::{Scheme, transliterate};
    
    let large_corpus = get_large_corpus();
    let mut group = c.benchmark_group("throughput/vidyut");
    
    for (text, verse_count) in large_corpus.iter() {
        let text_size = text.len();
        group.throughput(criterion::Throughput::Bytes(text_size as u64));
        
        group.bench_with_input(
            BenchmarkId::new("verses", verse_count),
            text,
            |b, text| {
                b.iter(|| {
                    let result = transliterate(
                        black_box(text),
                        black_box(Scheme::Devanagari),
                        black_box(Scheme::Iast)
                    );
                    black_box(result);
                })
            },
        );
    }
    
    group.finish();
}

// Memory usage benchmark
fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    
    // Measure initialization cost
    group.bench_function("shlesha_init", |b| {
        b.iter(|| {
            let transliterator = setup_shlesha();
            black_box(transliterator);
        })
    });
    
    #[cfg(feature = "compare-vidyut")]
    group.bench_function("vidyut_init", |b| {
        use vidyut_lipi::Scheme;
        b.iter(|| {
            // Vidyut is stateless, so just create the enum
            let _from = Scheme::Devanagari;
            let _to = Scheme::Iast;
        })
    });
    
    group.finish();
}

// Round-trip fidelity benchmark
fn bench_round_trip(c: &mut Criterion) {
    let transliterator = setup_shlesha();
    let test_corpus = get_test_corpus();
    
    let mut group = c.benchmark_group("round_trip");
    
    group.bench_function("shlesha", |b| {
        b.iter(|| {
            for (devanagari, _, _) in test_corpus.iter() {
                let iast = transliterator.transliterate(
                    black_box(devanagari),
                    black_box("Devanagari"),
                    black_box("IAST")
                ).unwrap();
                
                let back_to_dev = transliterator.transliterate(
                    black_box(&iast),
                    black_box("IAST"),
                    black_box("Devanagari")
                ).unwrap();
                
                black_box(back_to_dev);
            }
        })
    });
    
    #[cfg(feature = "compare-vidyut")]
    group.bench_function("vidyut", |b| {
        use vidyut_lipi::{Scheme, transliterate};
        
        b.iter(|| {
            for (devanagari, _, _) in test_corpus.iter() {
                let iast = transliterate(
                    black_box(devanagari),
                    black_box(Scheme::Devanagari),
                    black_box(Scheme::Iast)
                );
                
                let back_to_dev = transliterate(
                    black_box(&iast),
                    black_box(Scheme::Iast),
                    black_box(Scheme::Devanagari)
                );
                
                black_box(back_to_dev);
            }
        })
    });
    
    group.finish();
}

// Comparative summary
fn bench_summary(c: &mut Criterion) {
    let transliterator = setup_shlesha();
    let verse = "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय";
    
    let mut group = c.benchmark_group("summary");
    
    // Shlesha
    group.bench_function("shlesha", |b| {
        b.iter(|| {
            transliterator.transliterate(
                black_box(verse),
                black_box("Devanagari"),
                black_box("IAST")
            )
        })
    });
    
    #[cfg(feature = "compare-vidyut")]
    group.bench_function("vidyut", |b| {
        use vidyut_lipi::{Scheme, transliterate};
        b.iter(|| {
            transliterate(
                black_box(verse),
                black_box(Scheme::Devanagari),
                black_box(Scheme::Iast)
            )
        })
    });
    
    group.finish();
}

#[cfg(not(feature = "compare-vidyut"))]
criterion_group!(
    benches,
    bench_shlesha_accuracy,
    bench_shlesha_throughput,
    bench_memory_usage,
    bench_round_trip,
    bench_summary
);

#[cfg(feature = "compare-vidyut")]
criterion_group!(
    benches,
    bench_shlesha_accuracy,
    bench_vidyut_accuracy,
    bench_shlesha_throughput,
    bench_vidyut_throughput,
    bench_memory_usage,
    bench_round_trip,
    bench_summary
);

criterion_main!(benches);