use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use shlesha::Shlesha;
use std::time::Duration;

// Test text for benchmarking
const TEST_TEXTS: &[(&str, &str)] = &[
    ("short", "dharma"),
    ("medium", "dharma yoga meditation sanskrit"),
    (
        "long",
        "dharma yoga meditation sanskrit wisdom enlightenment practice spiritual",
    ),
];

// IAST schema content for runtime loading
const RUNTIME_IAST_SCHEMA: &str = r#"
metadata:
  name: "runtime_iast"
  script_type: "roman"
  description: "Runtime loaded IAST"
  has_implicit_a: false

target: "iso15919"

mappings:
  vowels:
    "a": "a"
    "ā": "ā"
    "i": "i"
    "ī": "ī"
    "u": "u"
    "ū": "ū"
    "ṛ": "r̥"
    "ṝ": "r̥̄"
    "ḷ": "l̥"
    "ḹ": "l̥̄"
    "e": "e"
    "ai": "ai"
    "o": "o"
    "au": "au"
  
  consonants:
    "k": "k"
    "kh": "kh"
    "g": "g"
    "gh": "gh"
    "ṅ": "ṅ"
    "c": "c"
    "ch": "ch"
    "j": "j"
    "jh": "jh"
    "ñ": "ñ"
    "ṭ": "ṭ"
    "ṭh": "ṭh"
    "ḍ": "ḍ"
    "ḍh": "ḍh"
    "ṇ": "ṇ"
    "t": "t"
    "th": "th"
    "d": "d"
    "dh": "dh"
    "n": "n"
    "p": "p"
    "ph": "ph"
    "b": "b"
    "bh": "bh"
    "m": "m"
    "y": "y"
    "r": "r"
    "l": "l"
    "v": "v"
    "ś": "ś"
    "ṣ": "ṣ"
    "s": "s"
    "h": "h"
  
  marks:
    "ṃ": "ṁ"
    "ḥ": "ḥ"
    "'": "'"
  
  special:
    "kṣ": "kṣ"
    "jñ": "jñ"
"#;

fn setup_runtime_transliterator() -> Shlesha {
    let mut transliterator = Shlesha::new();
    transliterator
        .load_schema_from_string(RUNTIME_IAST_SCHEMA, "runtime_iast")
        .expect("Failed to load runtime schema");
    transliterator
}

fn bench_builtin_vs_runtime(c: &mut Criterion) {
    let mut group = c.benchmark_group("builtin_vs_runtime");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(100);

    // Setup transliterators
    let builtin_transliterator = Shlesha::new();
    let runtime_transliterator = setup_runtime_transliterator();

    for (text_name, text) in TEST_TEXTS {
        let text_len = text.len();
        group.throughput(Throughput::Bytes(text_len as u64));

        // Benchmark built-in IAST
        group.bench_with_input(
            BenchmarkId::new("builtin_iast", text_name),
            text,
            |b, text| {
                b.iter(|| {
                    builtin_transliterator
                        .transliterate(black_box(text), black_box("iast"), black_box("devanagari"))
                        .unwrap()
                })
            },
        );

        // Benchmark runtime IAST
        group.bench_with_input(
            BenchmarkId::new("runtime_iast", text_name),
            text,
            |b, text| {
                b.iter(|| {
                    runtime_transliterator
                        .transliterate(
                            black_box(text),
                            black_box("runtime_iast"),
                            black_box("devanagari"),
                        )
                        .unwrap()
                })
            },
        );

        // Benchmark built-in IAST reverse
        group.bench_with_input(
            BenchmarkId::new("builtin_iast_reverse", text_name),
            text,
            |b, text| {
                // First convert to Devanagari
                let deva_text = builtin_transliterator
                    .transliterate(text, "iast", "devanagari")
                    .unwrap();

                b.iter(|| {
                    builtin_transliterator
                        .transliterate(
                            black_box(&deva_text),
                            black_box("devanagari"),
                            black_box("iast"),
                        )
                        .unwrap()
                })
            },
        );

        // Benchmark runtime IAST reverse (through ISO since runtime schemas go through registry)
        group.bench_with_input(
            BenchmarkId::new("runtime_iast_reverse", text_name),
            text,
            |b, text| {
                // First convert to Devanagari
                let deva_text = runtime_transliterator
                    .transliterate(text, "runtime_iast", "devanagari")
                    .unwrap();

                b.iter(|| {
                    runtime_transliterator
                        .transliterate(
                            black_box(&deva_text),
                            black_box("devanagari"),
                            black_box("runtime_iast"),
                        )
                        .unwrap()
                })
            },
        );
    }

    group.finish();
}

fn bench_schema_loading_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("schema_loading");

    // Benchmark schema loading time
    group.bench_function("load_schema_from_string", |b| {
        b.iter(|| {
            let mut transliterator = Shlesha::new();
            transliterator
                .load_schema_from_string(black_box(RUNTIME_IAST_SCHEMA), black_box("temp_iast"))
                .unwrap();
        })
    });

    // Benchmark transliterator creation with runtime schema
    group.bench_function("create_with_runtime_schema", |b| {
        b.iter(setup_runtime_transliterator)
    });

    // Benchmark built-in transliterator creation (baseline)
    group.bench_function("create_builtin_only", |b| b.iter(Shlesha::new));

    group.finish();
}

fn bench_script_management(c: &mut Criterion) {
    let mut group = c.benchmark_group("script_management");

    let transliterator = setup_runtime_transliterator();

    // Benchmark listing supported scripts
    group.bench_function("list_supported_scripts", |b| {
        b.iter(|| black_box(transliterator.list_supported_scripts()))
    });

    // Benchmark checking script support
    group.bench_function("supports_script_builtin", |b| {
        b.iter(|| black_box(transliterator.supports_script(black_box("iast"))))
    });

    group.bench_function("supports_script_runtime", |b| {
        b.iter(|| black_box(transliterator.supports_script(black_box("runtime_iast"))))
    });

    // Benchmark getting schema info
    group.bench_function("get_schema_info", |b| {
        b.iter(|| black_box(transliterator.get_schema_info(black_box("runtime_iast"))))
    });

    group.finish();
}

fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");

    // Benchmark multiple runtime schema loading
    group.bench_function("load_multiple_schemas", |b| {
        b.iter(|| {
            let mut transliterator = Shlesha::new();

            // Load multiple variations of the same schema
            for i in 0..5 {
                let schema_name = format!("runtime_iast_{}", i);
                transliterator
                    .load_schema_from_string(
                        black_box(RUNTIME_IAST_SCHEMA),
                        black_box(&schema_name),
                    )
                    .unwrap();
            }
        })
    });

    // Benchmark schema clearing
    group.bench_function("clear_runtime_schemas", |b| {
        b.iter_batched(
            || {
                let mut transliterator = Shlesha::new();
                for i in 0..5 {
                    let schema_name = format!("runtime_iast_{}", i);
                    transliterator
                        .load_schema_from_string(RUNTIME_IAST_SCHEMA, &schema_name)
                        .unwrap();
                }
                transliterator
            },
            |mut transliterator| {
                transliterator.clear_runtime_schemas();
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_builtin_vs_runtime,
    bench_schema_loading_overhead,
    bench_script_management,
    bench_memory_usage
);

criterion_main!(benches);
