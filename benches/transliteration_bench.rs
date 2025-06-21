use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use shlesha::LosslessTransliterator;

fn get_test_data() -> Vec<(&'static str, &'static str)> {
    vec![
        // Simple words
        ("नमस्ते", "namaste"),
        ("संस्कृतम्", "saṃskṛtam"),
        
        // Medium complexity
        ("भगवद्गीता", "bhagavadgītā"),
        ("कृष्णार्जुनसंवादः", "kṛṣṇārjunasaṃvādaḥ"),
        
        // Complex with conjuncts
        ("अग्निमीळे पुरोहितं यज्ञस्य देवमृत्विजम्", "agnimīḷe purohitaṃ yajñasya devamṛtvijam"),
        
        // Long text (first verse of Bhagavad Gita)
        ("धृतराष्ट्र उवाच धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय",
         "dhṛtarāṣṭra uvāca dharmakṣetre kurukṣetre samavetā yuyutsavaḥ māmakāḥ pāṇḍavāścaiva kimakurvata sañjaya"),
    ]
}

fn setup_transliterator() -> LosslessTransliterator {
    LosslessTransliterator::new()
}

fn benchmark_shlesha_devanagari_to_iast(c: &mut Criterion) {
    let transliterator = setup_transliterator();
    let test_data = get_test_data();
    
    let mut group = c.benchmark_group("shlesha_dev_to_iast");
    
    for (idx, (input, _expected)) in test_data.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("transliterate", format!("{}_{}", idx, input.len())),
            input,
            |b, input| {
                b.iter(|| {
                    transliterator.transliterate(
                        black_box(input),
                        black_box("Devanagari"),
                        black_box("IAST")
                    ).unwrap()
                })
            },
        );
    }
    
    group.finish();
}

fn benchmark_shlesha_iast_to_devanagari(c: &mut Criterion) {
    let transliterator = setup_transliterator();
    let test_data = get_test_data();
    
    let mut group = c.benchmark_group("shlesha_iast_to_dev");
    
    for (idx, (_input, romanized)) in test_data.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("transliterate", format!("{}_{}", idx, romanized.len())),
            romanized,
            |b, input| {
                b.iter(|| {
                    transliterator.transliterate(
                        black_box(input),
                        black_box("IAST"),
                        black_box("Devanagari")
                    ).unwrap()
                })
            },
        );
    }
    
    group.finish();
}

fn benchmark_shlesha_round_trip(c: &mut Criterion) {
    let transliterator = setup_transliterator();
    let test_data = get_test_data();
    
    c.bench_function("shlesha_round_trip", |b| {
        b.iter(|| {
            for (devanagari, _) in test_data.iter() {
                let iast = transliterator.transliterate(
                    black_box(devanagari),
                    black_box("Devanagari"),
                    black_box("IAST")
                ).unwrap();
                
                let _ = transliterator.transliterate(
                    black_box(&iast),
                    black_box("IAST"),
                    black_box("Devanagari")
                ).unwrap();
            }
        })
    });
}

// Benchmark with extensions (commented out - method not implemented)
// fn benchmark_shlesha_with_extensions(c: &mut Criterion) {
//     let transliterator = setup_transliterator();
//     let test_data = get_test_data();
//     
//     c.bench_function("shlesha_with_extensions", |b| {
//         b.iter(|| {
//             for (input, _) in test_data.iter() {
//                 let _ = transliterator.transliterate_with_extensions(
//                     black_box(input),
//                     black_box("Devanagari"),
//                     black_box("IAST"),
//                     black_box(&["vedic_accents"])
//                 );
//             }
//         })
//     });
// }

criterion_group!(
    benches,
    benchmark_shlesha_devanagari_to_iast,
    benchmark_shlesha_iast_to_devanagari,
    benchmark_shlesha_round_trip
);

criterion_main!(benches);