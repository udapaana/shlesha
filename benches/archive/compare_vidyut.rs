use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

// Note: To run this benchmark, you need to add vidyut-lipi to dev-dependencies:
// [dev-dependencies]
// vidyut-lipi = "0.5"

#[cfg(feature = "compare-vidyut")]
use vidyut_lipi::{Scheme, transliterate};

use shlesha::LosslessTransliterator;

fn get_test_corpus() -> Vec<(&'static str, usize)> {
    vec![
        // Single word
        ("नमस्ते", 1),
        
        // Short sentence
        ("अहं संस्कृतं वदामि", 10),
        
        // Medium text (50 words)
        ("तत्र शूरा महेष्वासा भीमार्जुनसमा युधि । युयुधानो विराटश्च द्रुपदश्च महारथः ॥", 50),
        
        // Long text (100 words)
        ("धृतराष्ट्र उवाच । धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः । मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय ॥ 
          सञ्जय उवाच । दृष्ट्वा तु पाण्डवानीकं व्यूढं दुर्योधनस्तदा । आचार्यमुपसङ्गम्य राजा वचनमब्रवीत् ॥", 100),
          
        // Very long text (500 words) - repeat a verse multiple times
        (&"तत्र शूरा महेष्वासा भीमार्जुनसमा युधि । युयुधानो विराटश्च द्रुपदश्च महारथः ॥ ".repeat(50), 500),
    ]
}

fn setup_shlesha() -> LosslessTransliterator {
    LosslessTransliterator::new()
}

fn bench_shlesha(c: &mut Criterion) {
    let transliterator = setup_shlesha();
    let test_corpus = get_test_corpus();
    
    let mut group = c.benchmark_group("transliteration_comparison");
    
    for (text, word_count) in test_corpus.iter() {
        group.bench_with_input(
            BenchmarkId::new("shlesha", word_count),
            text,
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

#[cfg(feature = "compare-vidyut")]
fn bench_vidyut(c: &mut Criterion) {
    let test_corpus = get_test_corpus();
    
    let mut group = c.benchmark_group("transliteration_comparison");
    
    for (text, word_count) in test_corpus.iter() {
        group.bench_with_input(
            BenchmarkId::new("vidyut", word_count),
            text,
            |b, text| {
                b.iter(|| {
                    transliterate(
                        black_box(text),
                        black_box(Scheme::Devanagari),
                        black_box(Scheme::Iast)
                    )
                })
            },
        );
    }
    
    group.finish();
}

// Throughput benchmark
fn bench_throughput(c: &mut Criterion) {
    let transliterator = setup_shlesha();
    
    // Create a large corpus (approximately 1MB of text)
    let verse = "तत्र शूरा महेष्वासा भीमार्जुनसमा युधि । युयुधानो विराटश्च द्रुपदश्च महारथः ॥ ";
    let large_text = verse.repeat(10000); // ~1MB
    let text_size = large_text.len();
    
    let mut group = c.benchmark_group("throughput");
    group.throughput(criterion::Throughput::Bytes(text_size as u64));
    
    group.bench_function("shlesha_1mb", |b| {
        b.iter(|| {
            transliterator.transliterate(
                black_box(&large_text),
                black_box("Devanagari"),
                black_box("IAST")
            )
        })
    });
    
    #[cfg(feature = "compare-vidyut")]
    group.bench_function("vidyut_1mb", |b| {
        b.iter(|| {
            transliterate(
                black_box(&large_text),
                black_box(Scheme::Devanagari),
                black_box(Scheme::Iast)
            )
        })
    });
    
    group.finish();
}

#[cfg(not(feature = "compare-vidyut"))]
criterion_group!(benches, bench_shlesha, bench_throughput);

#[cfg(feature = "compare-vidyut")]
criterion_group!(benches, bench_shlesha, bench_vidyut, bench_throughput);

criterion_main!(benches);