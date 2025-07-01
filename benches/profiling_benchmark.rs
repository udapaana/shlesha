//! Benchmark for the profile-guided optimization system
//!
//! This benchmark measures the effectiveness of the profiling system
//! by comparing baseline performance with optimized performance.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use shlesha::{Shlesha, modules::profiler::ProfilerConfig};
use std::path::PathBuf;
use std::time::Duration;

// Test data with repeated patterns (realistic for Sanskrit/Hindi)
const REPEATED_SANSKRIT_TEXT: &str = "धर्म कर्म योग वेद मन्त्र धर्म योग कर्म वेद धर्म मन्त्र योग कर्म वेद धर्म योग वेद कर्म मन्त्र धर्म योग कर्म वेद मन्त्र धर्म योग";

const COMMON_WORDS: &[&str] = &[
    "धर्म", "कर्म", "योग", "वेद", "मन्त्र",
    "भगवान्", "देव", "देवी", "गुरु", "शिष्य",
    "नमस्ते", "श्री", "महा", "राज", "पुत्र",
];

fn benchmark_baseline_vs_optimized(c: &mut Criterion) {
    let mut group = c.benchmark_group("profiling_effectiveness");

    // Create baseline transliterator (no profiling)
    let baseline_transliterator = Shlesha::new();

    // Create transliterator with profiling
    let mut config = ProfilerConfig::default();
    config.profile_dir = PathBuf::from("bench_profiles");
    config.optimization_dir = PathBuf::from("bench_optimizations");
    config.min_sequence_frequency = 2; // Low threshold for benchmark
    
    let mut profiled_transliterator = Shlesha::new();
    profiled_transliterator.enable_profiling_with_config(config);

    // Build up profile data by processing text multiple times
    for _ in 0..20 {
        for word in COMMON_WORDS {
            let _ = profiled_transliterator.transliterate(word, "devanagari", "iast");
        }
        let _ = profiled_transliterator.transliterate(REPEATED_SANSKRIT_TEXT, "devanagari", "iast");
    }

    // Generate and load optimizations
    let optimizations = profiled_transliterator.generate_optimizations();
    for optimization in optimizations {
        profiled_transliterator.load_optimization(optimization);
    }

    // Benchmark baseline performance
    group.bench_with_input(
        BenchmarkId::new("baseline", "repeated_text"),
        &REPEATED_SANSKRIT_TEXT,
        |b, text| {
            b.iter(|| {
                black_box(baseline_transliterator.transliterate(
                    black_box(text),
                    black_box("devanagari"),
                    black_box("iast"),
                ).unwrap())
            })
        },
    );

    // Benchmark optimized performance
    group.bench_with_input(
        BenchmarkId::new("optimized", "repeated_text"),
        &REPEATED_SANSKRIT_TEXT,
        |b, text| {
            b.iter(|| {
                black_box(profiled_transliterator.transliterate(
                    black_box(text),
                    black_box("devanagari"),
                    black_box("iast"),
                ).unwrap())
            })
        },
    );

    // Benchmark individual common words
    for word in COMMON_WORDS.iter().take(5) {
        group.bench_with_input(
            BenchmarkId::new("baseline_word", word),
            word,
            |b, word| {
                b.iter(|| {
                    black_box(baseline_transliterator.transliterate(
                        black_box(word),
                        black_box("devanagari"),
                        black_box("iast"),
                    ).unwrap())
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("optimized_word", word),
            word,
            |b, word| {
                b.iter(|| {
                    black_box(profiled_transliterator.transliterate(
                        black_box(word),
                        black_box("devanagari"),
                        black_box("iast"),
                    ).unwrap())
                })
            },
        );
    }

    group.finish();
}

fn benchmark_profiling_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("profiling_overhead");

    let baseline_transliterator = Shlesha::new();
    let mut profiled_transliterator = Shlesha::with_profiling();

    // Benchmark the overhead of profiling itself
    group.bench_function("baseline_no_profiling", |b| {
        b.iter(|| {
            black_box(baseline_transliterator.transliterate(
                black_box("धर्म"),
                black_box("devanagari"),
                black_box("iast"),
            ).unwrap())
        })
    });

    group.bench_function("with_profiling", |b| {
        b.iter(|| {
            black_box(profiled_transliterator.transliterate(
                black_box("धर्म"),
                black_box("devanagari"),
                black_box("iast"),
            ).unwrap())
        })
    });

    group.finish();
}

fn benchmark_optimization_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimization_generation");

    // Create transliterator with substantial profile data
    let mut transliterator = Shlesha::with_profiling();
    
    // Build up a realistic profile
    for i in 0..100 {
        for word in COMMON_WORDS {
            let _ = transliterator.transliterate(word, "devanagari", "iast");
        }
        if i % 10 == 0 {
            let _ = transliterator.transliterate(REPEATED_SANSKRIT_TEXT, "devanagari", "iast");
        }
    }

    group.bench_function("generate_optimizations", |b| {
        b.iter(|| {
            black_box(transliterator.generate_optimizations())
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_baseline_vs_optimized,
    benchmark_profiling_overhead,
    benchmark_optimization_generation
);
criterion_main!(benches);