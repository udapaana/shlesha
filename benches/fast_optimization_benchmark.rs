use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use shlesha::Shlesha;

// Fast benchmark suite for iterative optimization
// Focuses on core conversion paths with smaller test data

fn hub_conversions(c: &mut Criterion) {
    let transliterator = Shlesha::new();
    
    // Small test cases for rapid iteration
    let test_cases = vec![
        ("short", "धर्म"),        // 4 chars - basic word
        ("medium", "धर्मशास्त्र"),  // 10 chars - compound word  
    ];
    
    let mut group = c.benchmark_group("hub_fast");
    
    for (size, text) in test_cases {
        // Devanagari -> ISO (critical path through hub processing)
        group.bench_with_input(
            BenchmarkId::new("deva_to_iso", size),
            &text,
            |b, text| {
                b.iter(|| {
                    black_box(transliterator.transliterate(
                        black_box(text),
                        black_box("devanagari"),
                        black_box("iso"),
                    ).unwrap())
                })
            },
        );
        
        // ISO -> Devanagari (reverse critical path)
        group.bench_with_input(
            BenchmarkId::new("iso_to_deva", size), 
            &"dharma",
            |b, text| {
                b.iter(|| {
                    black_box(transliterator.transliterate(
                        black_box(text),
                        black_box("iso"),
                        black_box("devanagari"),
                    ).unwrap())
                })
            },
        );
    }
    
    group.finish();
}

fn script_converter_paths(c: &mut Criterion) {
    let transliterator = Shlesha::new();
    
    let mut group = c.benchmark_group("converters_fast");
    
    // Test common conversion patterns
    let conversions = vec![
        ("indic_indic", "धर्म", "devanagari", "gujarati"),    // Indic -> Indic (1 hop)
        ("roman_roman", "dharma", "iast", "itrans"),          // Roman -> Roman (1 hop) 
        ("indic_roman", "धर्म", "devanagari", "iast"),        // Indic -> Roman (2 hop)
        ("roman_indic", "dharma", "iast", "devanagari"),      // Roman -> Indic (2 hop)
    ];
    
    for (pattern, text, from, to) in conversions {
        group.bench_with_input(
            BenchmarkId::new(pattern, text.len()),
            &(text, from, to),
            |b, (text, from, to)| {
                b.iter(|| {
                    black_box(transliterator.transliterate(
                        black_box(text),
                        black_box(from),
                        black_box(to),
                    ).unwrap())
                })
            },
        );
    }
    
    group.finish();
}

fn memory_allocation_stress(c: &mut Criterion) {
    let transliterator = Shlesha::new();
    
    let mut group = c.benchmark_group("memory_stress");
    
    // Test repeated allocations to measure string allocation improvements
    group.bench_function("repeated_small_conversions", |b| {
        b.iter(|| {
            for _ in 0..10 {
                black_box(transliterator.transliterate(
                    black_box("क"),
                    black_box("devanagari"), 
                    black_box("iso"),
                ).unwrap());
            }
        })
    });
    
    group.finish();
}

// Include basic correctness tests alongside benchmarks
#[cfg(test)]
mod benchmark_correctness_tests {
    use super::*;
    
    #[test]
    fn verify_benchmark_results() {
        let transliterator = Shlesha::new();
        
        // Verify our benchmark cases produce correct results
        assert_eq!(
            transliterator.transliterate("धर्म", "devanagari", "iso").unwrap(),
            "dharma"
        );
        
        assert_eq!(
            transliterator.transliterate("dharma", "iso", "devanagari").unwrap(),
            "धर्म"
        );
        
        assert_eq!(
            transliterator.transliterate("धर्म", "devanagari", "gujarati").unwrap(),
            "ધર્મ"
        );
        
        assert_eq!(
            transliterator.transliterate("dharma", "iast", "itrans").unwrap(),
            "dharma"
        );
    }
}

// Run both benchmarks and tests
criterion_group!(
    fast_benches,
    hub_conversions,
    script_converter_paths, 
    memory_allocation_stress
);
criterion_main!(fast_benches);