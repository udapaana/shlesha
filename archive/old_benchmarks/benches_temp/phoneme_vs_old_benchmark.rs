use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use shlesha::{
    Transliterator, TransliteratorBuilder, 
    PhonemeTransliterator, PhonemeTransliteratorBuilder,
    SchemaParser
};

fn get_test_corpus() -> Vec<(&'static str, &'static str, usize)> {
    vec![
        ("single_word", "नमस्ते", 1),
        ("short_sentence", "अहं संस्कृतं वदामि", 4),
        ("medium_text", "तत्र शूरा महेष्वासा भीमार्जुनसमा युधि । युयुधानो विराटश्च द्रुपदश्च महारथः ॥", 15),
        ("long_text", "धृतराष्ट्र उवाच । धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः । मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय ॥ सञ्जय उवाच । दृष्ट्वा तु पाण्डवानीकं व्यूढं दुर्योधनस्तदा । आचार्यमुपसङ्गम्य राजा वचनमब्रवीत् ॥", 50),
    ]
}

fn setup_old_transliterator() -> Transliterator {
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema = SchemaParser::parse_str(iast_schema).unwrap();
    
    TransliteratorBuilder::new()
        .with_schema(dev_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build()
}

fn setup_phoneme_transliterator() -> PhonemeTransliterator {
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema = SchemaParser::parse_str(iast_schema).unwrap();
    
    PhonemeTransliteratorBuilder::new()
        .with_schema(dev_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build()
}

fn bench_comparison(c: &mut Criterion) {
    let old_transliterator = setup_old_transliterator();
    let mut phoneme_transliterator = setup_phoneme_transliterator();
    let test_corpus = get_test_corpus();
    
    let mut group = c.benchmark_group("old_vs_phoneme");
    
    for (name, text, _word_count) in test_corpus.iter() {
        // Benchmark old parser
        group.bench_with_input(
            BenchmarkId::new("old_parser", name),
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
        
        // Benchmark phoneme parser
        group.bench_with_input(
            BenchmarkId::new("phoneme_parser", name),
            text,
            |b, text| {
                b.iter(|| {
                    phoneme_transliterator.transliterate(
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

fn bench_throughput(c: &mut Criterion) {
    let old_transliterator = setup_old_transliterator();
    let mut phoneme_transliterator = setup_phoneme_transliterator();
    
    // Create different sized texts
    let verse = "तत्र शूरा महेष्वासा भीमार्जुनसमा युधि । युयुधानो विराटश्च द्रुपदश्च महारथः ॥ ";
    let sizes = vec![
        ("100KB", verse.repeat(1000)),
        ("1MB", verse.repeat(10000)),
    ];
    
    let mut group = c.benchmark_group("throughput_comparison");
    
    for (size_name, text) in sizes.iter() {
        let text_size = text.len();
        group.throughput(criterion::Throughput::Bytes(text_size as u64));
        
        group.bench_with_input(
            BenchmarkId::new("old_parser", size_name),
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
        
        group.bench_with_input(
            BenchmarkId::new("phoneme_parser", size_name),
            &text,
            |b, text| {
                b.iter(|| {
                    phoneme_transliterator.transliterate(
                        black_box(text),
                        black_box("Devanagari"),
                        black_box("IAST")
                    )
                })
            },
        );
    }
    
    group.finish();
    
    // Print phoneme parser statistics
    println!("\nPhoneme Parser Statistics:");
    let stats = phoneme_transliterator.get_parse_stats();
    println!("  Total chars processed: {}", stats.total_chars_processed);
    println!("  Enum phonemes used: {} ({:.1}% efficiency)", 
        stats.enum_phonemes_used, 
        stats.allocation_efficiency());
    println!("  Extension phonemes used: {}", stats.extension_phonemes_used);
    println!("  String allocations: {}", stats.string_allocations);
    println!("  Avg parse time per char: {:.2} ns", stats.avg_parse_time_per_char_ns());
}

criterion_group!(benches, bench_comparison, bench_throughput);
criterion_main!(benches);