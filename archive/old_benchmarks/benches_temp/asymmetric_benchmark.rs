use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use shlesha::{TransliteratorBuilder, SchemaParser};

const TEST_TEXTS: &[(&str, &str)] = &[
    ("small", "धर्मक्षेत्रे कुरुक्षेत्रे"),
    ("medium", "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः। मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय॥"),
    ("large", "कर्मण्येवाधिकारस्ते मा फलेषु कदाचन। मा कर्मफलहेतुर्भूर्मा ते सङ्गोऽस्त्वकर्मणि॥ योगस्थः कुरु कर्माणि सङ्गं त्यक्त्वा धनञ्जय। सिद्ध्यसिद्ध्योः समो भूत्वा समत्वं योग उच्यते॥"),
];

fn setup_bidirectional() -> shlesha::Transliterator {
    let devanagari = SchemaParser::parse_file("schemas/devanagari.yaml").unwrap();
    let iast = SchemaParser::parse_file("schemas/iast.yaml").unwrap();
    
    TransliteratorBuilder::new()
        .with_schema(devanagari).unwrap()
        .with_schema(iast).unwrap()
        .build()
}

fn benchmark_transliteration_modes(c: &mut Criterion) {
    let bidirectional = setup_bidirectional();
    
    let mut group = c.benchmark_group("Transliteration Modes");
    
    // Current bidirectional approach
    for (name, text) in TEST_TEXTS {
        group.bench_with_input(
            BenchmarkId::new("bidirectional_with_ir", name),
            text,
            |b, text| {
                b.iter(|| {
                    bidirectional.transliterate(
                        black_box(text),
                        black_box("Devanagari"),
                        black_box("IAST")
                    )
                })
            },
        );
    }
    
    // Theoretical fast path (simulated)
    for (name, text) in TEST_TEXTS {
        group.bench_with_input(
            BenchmarkId::new("direct_mapping_simulated", name),
            text,
            |b, text| {
                b.iter(|| {
                    // Simulate direct character mapping without IR
                    let mut output = String::with_capacity(text.len() * 2);
                    for ch in text.chars() {
                        match ch {
                            'क' => output.push_str("ka"),
                            'र' => output.push_str("ra"),
                            'म' => output.push_str("ma"),
                            'ध' => output.push_str("dha"),
                            '्' => {},
                            _ => output.push(ch),
                        }
                    }
                    black_box(output)
                })
            },
        );
    }
    
    // Memory allocation impact
    group.bench_function("memory_allocation_overhead", |b| {
        b.iter(|| {
            // Current approach allocates:
            // 1. IR elements vector
            // 2. Properties HashMap for each element
            // 3. Intermediate strings
            let mut allocations = Vec::new();
            for _ in 0..100 {
                allocations.push(vec![0u8; 64]); // Simulate element allocation
                allocations.push(std::collections::HashMap::<String, String>::new());
            }
            black_box(allocations)
        })
    });
    
    group.finish();
}

fn benchmark_round_trip_vs_one_way(c: &mut Criterion) {
    let transliterator = setup_bidirectional();
    let test_text = TEST_TEXTS[1].1; // medium text
    
    let mut group = c.benchmark_group("Round-trip vs One-way");
    
    // One-way transliteration
    group.bench_function("one_way_deva_to_iast", |b| {
        b.iter(|| {
            transliterator.transliterate(
                black_box(test_text),
                black_box("Devanagari"),
                black_box("IAST")
            )
        })
    });
    
    // Round-trip transliteration
    group.bench_function("round_trip_deva_iast_deva", |b| {
        b.iter(|| {
            let iast = transliterator.transliterate(
                black_box(test_text),
                black_box("Devanagari"),
                black_box("IAST")
            ).unwrap();
            transliterator.transliterate(
                black_box(&iast),
                black_box("IAST"),
                black_box("Devanagari")
            )
        })
    });
    
    group.finish();
}

criterion_group!(benches, benchmark_transliteration_modes, benchmark_round_trip_vs_one_way);
criterion_main!(benches);