use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use shlesha::{TransliteratorBuilder, SchemaParser};
use std::time::Duration;

// Test data of varying sizes
const SMALL_TEXT: &str = "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः";
const MEDIUM_TEXT: &str = "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः । मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय ॥ दृष्ट्वा तु पाण्डवानीकं व्यूढं दुर्योधनस्तदा । आचार्यमुपसङ्गम्य राजा वचनमब्रवीत् ॥";
const LARGE_TEXT: &str = include_str!("../bench_data/large.txt");

fn setup_shlesha() -> shlesha::Transliterator {
    let devanagari = SchemaParser::parse_file("schemas/devanagari.yaml").unwrap();
    let iast = SchemaParser::parse_file("schemas/iast.yaml").unwrap();
    let slp1 = SchemaParser::parse_file("schemas/slp1.yaml").unwrap();
    let harvard_kyoto = SchemaParser::parse_file("schemas/harvard_kyoto.yaml").unwrap();
    
    TransliteratorBuilder::new()
        .with_schema(devanagari).unwrap()
        .with_schema(iast).unwrap()
        .with_schema(slp1).unwrap()
        .with_schema(harvard_kyoto).unwrap()
        .build()
}

fn benchmark_shlesha(c: &mut Criterion) {
    let transliterator = setup_shlesha();
    
    let mut group = c.benchmark_group("Shlesha");
    group.measurement_time(Duration::from_secs(10));
    
    for (name, text) in &[("small", SMALL_TEXT), ("medium", MEDIUM_TEXT), ("large", LARGE_TEXT)] {
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
        
        group.bench_with_input(
            BenchmarkId::new("devanagari_to_slp1", name),
            text,
            |b, text| {
                b.iter(|| {
                    transliterator.transliterate(
                        black_box(text),
                        black_box("Devanagari"),
                        black_box("SLP1")
                    )
                })
            },
        );
    }
    
    // Round-trip benchmark
    group.bench_function("round_trip_deva_iast_deva", |b| {
        b.iter(|| {
            let iast = transliterator.transliterate(
                black_box(MEDIUM_TEXT),
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

// Note: Vidyut benchmarks would go here if we had the library available
// For now, we'll just benchmark Shlesha and compare with previously recorded results

criterion_group!(benches, benchmark_shlesha);
criterion_main!(benches);