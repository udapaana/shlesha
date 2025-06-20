use criterion::{black_box, criterion_group, criterion_main, Criterion};
use shlesha::{Transliterator, TransliteratorBuilder, SchemaParser};

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

fn bench_shlesha_performance(c: &mut Criterion) {
    let transliterator = setup_shlesha();
    
    let test_cases = vec![
        ("नमस्ते", "Single word"),
        ("धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः", "Bhagavad Gita verse"),
        ("तत्र शूरा महेष्वासा भीमार्जुनसमा युधि । युयुधानो विराटश्च द्रुपदश्च महारथः ॥", "Long verse"),
    ];
    
    let mut group = c.benchmark_group("shlesha_performance");
    
    for (text, name) in test_cases {
        group.bench_function(name, |b| {
            b.iter(|| {
                transliterator.transliterate(
                    black_box(text),
                    black_box("Devanagari"),
                    black_box("IAST")
                ).unwrap()
            })
        });
    }
    
    group.finish();
}

#[cfg(feature = "compare-vidyut")]
fn bench_vidyut_performance(c: &mut Criterion) {
    use vidyut_lipi::{Mapping, Scheme, transliterate};
    
    let mapping = Mapping::new(Scheme::Devanagari, Scheme::Iast);
    
    let test_cases = vec![
        ("नमस्ते", "Single word"),
        ("धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः", "Bhagavad Gita verse"),
        ("तत्र शूरा महेष्वासा भीमार्जुनसमा युधि । युयुधानो विराटश्च द्रुपदश्च महारथः ॥", "Long verse"),
    ];
    
    let mut group = c.benchmark_group("vidyut_performance");
    
    for (text, name) in test_cases {
        group.bench_function(name, |b| {
            b.iter(|| {
                transliterate(
                    black_box(text),
                    black_box(&mapping)
                )
            })
        });
    }
    
    group.finish();
}

fn bench_throughput(c: &mut Criterion) {
    let transliterator = setup_shlesha();
    
    // Create different sizes of text for throughput testing
    let base_verse = "तत्र शूरा महेष्वासा भीमार्जुनसमा युधि । ";
    let text_500 = base_verse.repeat(10);
    let text_5kb = base_verse.repeat(100);
    let text_50kb = base_verse.repeat(1000);
    let sizes = vec![
        (base_verse, "50 bytes"),
        (text_500.as_str(), "500 bytes"),
        (text_5kb.as_str(), "5KB"),
        (text_50kb.as_str(), "50KB"),
    ];
    
    let mut group = c.benchmark_group("throughput_comparison");
    
    for (text, size) in sizes {
        let text_size = text.len();
        group.throughput(criterion::Throughput::Bytes(text_size as u64));
        
        group.bench_function(format!("shlesha_{}", size), |b| {
            b.iter(|| {
                transliterator.transliterate(
                    black_box(text),
                    black_box("Devanagari"),
                    black_box("IAST")
                ).unwrap()
            })
        });
        
        #[cfg(feature = "compare-vidyut")]
        group.bench_function(format!("vidyut_{}", size), |b| {
            use vidyut_lipi::{Mapping, Scheme, transliterate};
            let mapping = Mapping::new(Scheme::Devanagari, Scheme::Iast);
            b.iter(|| {
                transliterate(
                    black_box(text),
                    black_box(&mapping)
                )
            })
        });
    }
    
    group.finish();
}

#[cfg(not(feature = "compare-vidyut"))]
criterion_group!(benches, bench_shlesha_performance, bench_throughput);

#[cfg(feature = "compare-vidyut")]
criterion_group!(benches, bench_shlesha_performance, bench_vidyut_performance, bench_throughput);

criterion_main!(benches);