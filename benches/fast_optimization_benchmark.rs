use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use shlesha::Shlesha;

// Fast benchmark suite for iterative optimization
// Focuses on Telugu ↔ SLP1 roundtrip (Indic ↔ Roman non-hub script)
// This tests the critical path: Telugu → Devanagari → ISO → SLP1 → ISO → Devanagari → Telugu

fn telugu_slp1_roundtrip(c: &mut Criterion) {
    let transliterator = Shlesha::new();

    // Telugu test cases for rapid iteration
    let test_cases = vec![
        ("short", "ధర్మ"),      // 4 chars - basic word "dharma" in Telugu
        ("medium", "ధర్మశాస్త్ర"), // 10 chars - compound word "dharmashaastra"
    ];

    let mut group = c.benchmark_group("telugu_slp1_roundtrip");

    for (size, telugu_text) in test_cases {
        // Telugu -> SLP1 (Indic -> Roman via hub)
        group.bench_with_input(
            BenchmarkId::new("telugu_to_slp1", size),
            &telugu_text,
            |b, text| {
                b.iter(|| {
                    black_box(
                        transliterator
                            .transliterate(black_box(text), black_box("telugu"), black_box("slp1"))
                            .unwrap(),
                    )
                })
            },
        );

        // Get SLP1 result for reverse conversion benchmark
        let slp1_result = transliterator
            .transliterate(telugu_text, "telugu", "slp1")
            .unwrap();

        // SLP1 -> Telugu (Roman -> Indic via hub)
        group.bench_with_input(
            BenchmarkId::new("slp1_to_telugu", size),
            &slp1_result,
            |b, text| {
                b.iter(|| {
                    black_box(
                        transliterator
                            .transliterate(black_box(text), black_box("slp1"), black_box("telugu"))
                            .unwrap(),
                    )
                })
            },
        );

        // Full roundtrip (Telugu -> SLP1 -> Telugu)
        group.bench_with_input(
            BenchmarkId::new("full_roundtrip", size),
            &telugu_text,
            |b, text| {
                b.iter(|| {
                    let slp1 = black_box(
                        transliterator
                            .transliterate(black_box(text), black_box("telugu"), black_box("slp1"))
                            .unwrap(),
                    );
                    black_box(
                        transliterator
                            .transliterate(black_box(&slp1), black_box("slp1"), black_box("telugu"))
                            .unwrap(),
                    )
                })
            },
        );
    }

    group.finish();
}

fn character_mapping_stress(c: &mut Criterion) {
    let transliterator = Shlesha::new();

    let mut group = c.benchmark_group("character_mapping");

    // Test character-level mapping performance (target for Perfect Hash optimization)
    let single_chars = vec![
        ("telugu_consonant", "ధ", "telugu", "slp1"),
        ("telugu_vowel", "అ", "telugu", "slp1"),
        ("slp1_consonant", "D", "slp1", "telugu"),
        ("slp1_vowel", "a", "slp1", "telugu"),
    ];

    for (char_type, text, from, to) in single_chars {
        group.bench_with_input(
            BenchmarkId::new(char_type, 1),
            &(text, from, to),
            |b, (text, from, to)| {
                b.iter(|| {
                    black_box(
                        transliterator
                            .transliterate(black_box(text), black_box(from), black_box(to))
                            .unwrap(),
                    )
                })
            },
        );
    }

    group.finish();
}

fn string_allocation_stress(c: &mut Criterion) {
    let transliterator = Shlesha::new();

    let mut group = c.benchmark_group("string_allocation");

    // Test repeated allocations to measure string allocation improvements
    group.bench_function("repeated_telugu_slp1", |b| {
        b.iter(|| {
            for _ in 0..10 {
                black_box(
                    transliterator
                        .transliterate(
                            black_box("క"), // Telugu "ka"
                            black_box("telugu"),
                            black_box("slp1"),
                        )
                        .unwrap(),
                );
            }
        })
    });

    // Test longer text to stress string capacity pre-calculation
    group.bench_function("long_telugu_text", |b| {
        let long_text = "ధర్మక్షేత్రే కురుక్షేత్రే సమవేతా యుయుత్సవః"; // Bhagavad Gita opening
        b.iter(|| {
            black_box(
                transliterator
                    .transliterate(black_box(long_text), black_box("telugu"), black_box("slp1"))
                    .unwrap(),
            )
        })
    });

    group.finish();
}

// Include basic correctness tests alongside benchmarks
#[cfg(test)]
mod benchmark_correctness_tests {
    use super::*;

    #[test]
    fn verify_telugu_slp1_benchmark_results() {
        let transliterator = Shlesha::new();

        // Verify our Telugu ↔ SLP1 benchmark cases produce correct results
        assert_eq!(
            transliterator
                .transliterate("ధర్మ", "telugu", "slp1")
                .unwrap(),
            "Darma"
        );

        assert_eq!(
            transliterator
                .transliterate("Darma", "slp1", "telugu")
                .unwrap(),
            "ధర్మ"
        );

        // Test roundtrip correctness
        let original = "ధర్మశాస్త్ర";
        let slp1 = transliterator
            .transliterate(original, "telugu", "slp1")
            .unwrap();
        let roundtrip = transliterator
            .transliterate(&slp1, "slp1", "telugu")
            .unwrap();
        assert_eq!(original, roundtrip);

        // Test single character mappings
        assert_eq!(
            transliterator.transliterate("క", "telugu", "slp1").unwrap(),
            "ka"
        );

        assert_eq!(
            transliterator.transliterate("అ", "telugu", "slp1").unwrap(),
            "a"
        );
    }
}

// Run optimization-focused benchmarks
criterion_group!(
    fast_optimization_benches,
    telugu_slp1_roundtrip,
    character_mapping_stress,
    string_allocation_stress
);
criterion_main!(fast_optimization_benches);
