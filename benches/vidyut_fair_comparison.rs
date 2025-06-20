use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use shlesha::{Transliterator, TransliteratorBuilder, SchemaParser};

#[cfg(feature = "compare-vidyut")]
use vidyut_lipi::{Mapping, Scheme, transliterate};

fn get_test_corpus() -> Vec<(&'static str, &'static str, usize)> {
    vec![
        // (name, text, word_count)
        ("single_word", "नमस्ते", 1),
        ("short_sentence", "अहं संस्कृतं वदामि", 4),
        ("medium_text", "तत्र शूरा महेष्वासा भीमार्जुनसमा युधि । युयुधानो विराटश्च द्रुपदश्च महारथः ॥", 15),
        ("long_text", "धृतराष्ट्र उवाच । धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः । मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय ॥ सञ्जय उवाच । दृष्ट्वा तु पाण्डवानीकं व्यूढं दुर्योधनस्तदा । आचार्यमुपसङ्गम्य राजा वचनमब्रवीत् ॥", 50),
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

#[cfg(feature = "compare-vidyut")]
fn bench_comparison(c: &mut Criterion) {
    let transliterator = setup_shlesha();
    let mapping = Mapping::new(Scheme::Devanagari, Scheme::Iast);
    let test_corpus = get_test_corpus();
    
    let mut group = c.benchmark_group("devanagari_to_iast");
    
    for (name, text, _word_count) in test_corpus.iter() {
        // Benchmark Shlesha
        group.bench_with_input(
            BenchmarkId::new("shlesha", name),
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
        
        // Benchmark Vidyut
        group.bench_with_input(
            BenchmarkId::new("vidyut", name),
            text,
            |b, text| {
                b.iter(|| {
                    transliterate(
                        black_box(text),
                        black_box(&mapping)
                    )
                })
            },
        );
    }
    
    group.finish();
}

#[cfg(feature = "compare-vidyut")]
fn bench_throughput(c: &mut Criterion) {
    let transliterator = setup_shlesha();
    let mapping = Mapping::new(Scheme::Devanagari, Scheme::Iast);
    
    // Create different sized texts
    let verse = "तत्र शूरा महेष्वासा भीमार्जुनसमा युधि । युयुधानो विराटश्च द्रुपदश्च महारथः ॥ ";
    let sizes = vec![
        ("100KB", verse.repeat(1000)),
        ("1MB", verse.repeat(10000)),
    ];
    
    let mut group = c.benchmark_group("throughput");
    
    for (size_name, text) in sizes.iter() {
        let text_size = text.len();
        group.throughput(criterion::Throughput::Bytes(text_size as u64));
        
        group.bench_with_input(
            BenchmarkId::new("shlesha", size_name),
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
        
        group.bench_with_input(
            BenchmarkId::new("vidyut", size_name),
            &text,
            |b, text| {
                b.iter(|| {
                    transliterate(
                        black_box(text),
                        black_box(&mapping)
                    )
                })
            },
        );
    }
    
    group.finish();
}

#[cfg(not(feature = "compare-vidyut"))]
fn bench_shlesha_only(c: &mut Criterion) {
    let transliterator = setup_shlesha();
    let test_corpus = get_test_corpus();
    
    let mut group = c.benchmark_group("shlesha_only");
    
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
                    )
                })
            },
        );
    }
    
    group.finish();
}

#[cfg(feature = "compare-vidyut")]
criterion_group!(benches, bench_comparison, bench_throughput);

#[cfg(not(feature = "compare-vidyut"))]
criterion_group!(benches, bench_shlesha_only);

criterion_main!(benches);