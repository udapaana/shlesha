use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use shlesha::Shlesha;
use std::fs;
use std::time::Duration;

// Test data sets
const SMALL_TEXT: &str = "धर्म";
const MEDIUM_TEXT: &str = "धर्म योग भारत संस्कृत वेद उपनिषद् गीता रामायण महाभारत";
const LARGE_TEXT: &str = "धर्म योग भारत संस्कृत वेद उपनिषद् गीता रामायण महाभारत पुराण शास्त्र दर्शन आयुर्वेद ज्योतिष व्याकरण छन्द निरुक्त कल्प शिक्षा स्मृति श्रुति आचार विचार संस्कार परम्परा सत्य अहिंसा करुणा दया प्रेम शान्ति आनन्द मोक्ष निर्वाण समाधि ध्यान प्राणायाम आसन मन्त्र यन्त्र तन्त्र";

// Focused benchmark combinations
const HUB_CONVERSIONS: &[(&str, &str)] = &[("devanagari", "iso15919"), ("iso15919", "devanagari")];

const INDIC_TO_ROMAN: &[(&str, &str)] = &[
    ("telugu", "iast"),
    ("telugu", "itrans"),
    ("telugu", "slp1"),
    ("telugu", "harvard_kyoto"),
    ("telugu", "wx"),
    ("telugu", "velthuis"),
];

const ROMAN_TO_INDIC: &[(&str, &str)] = &[
    ("iast", "devanagari"),
    ("iast", "bengali"),
    ("iast", "telugu"),
    ("iast", "gujarati"),
    ("iast", "kannada"),
    ("iast", "malayalam"),
    ("velthuis", "devanagari"),
];

const ROMAN_TO_ROMAN: &[(&str, &str)] = &[
    ("iast", "itrans"),
    ("itrans", "iast"),
    ("iast", "velthuis"),
    ("velthuis", "iast"),
    ("slp1", "wx"),
    ("wx", "slp1"),
];

const INDIC_TO_INDIC: &[(&str, &str)] = &[
    ("bengali", "telugu"),
    ("telugu", "bengali"),
    ("telugu", "malayalam"),
    ("malayalam", "telugu"),
];

// Tamil only supports one-way conversion (from Tamil to other scripts)
const TAMIL_ONE_WAY: &[(&str, &str)] = &[
    ("tamil", "devanagari"),
    ("tamil", "iast"), // Tamil → Devanagari → ISO → IAST
    ("tamil", "slp1"), // Tamil → Devanagari → ISO → SLP1
];

struct BenchmarkResult {
    script_from: String,
    script_to: String,
    category: String,
    text_size: String,
    throughput_chars_per_sec: f64,
    latency_ns: f64,
}

impl BenchmarkResult {
    fn to_csv_row(&self) -> String {
        format!(
            "{},{},{},{},{:.0},{:.0}",
            self.script_from,
            self.script_to,
            self.category,
            self.text_size,
            self.throughput_chars_per_sec,
            self.latency_ns
        )
    }
}

fn benchmark_conversion_set(c: &mut Criterion, category: &str, conversions: &[(&str, &str)]) {
    let transliterator = Shlesha::new();
    let mut results = Vec::new();

    let mut group = c.benchmark_group(category);
    group.measurement_time(Duration::from_secs(3));
    group.sample_size(50);

    for &(from_script, to_script) in conversions {
        for (size_name, text) in [
            ("small", SMALL_TEXT),
            ("medium", MEDIUM_TEXT),
            ("large", LARGE_TEXT),
        ] {
            let benchmark_name = format!("{}_{}_to_{}", size_name, from_script, to_script);

            group.throughput(Throughput::Bytes(text.len() as u64));
            group.bench_with_input(
                BenchmarkId::new(&benchmark_name, text.len()),
                &text,
                |b, text| {
                    b.iter(|| {
                        transliterator
                            .transliterate(
                                black_box(text),
                                black_box(from_script),
                                black_box(to_script),
                            )
                            .unwrap()
                    })
                },
            );

            // Measure for results collection
            let start = std::time::Instant::now();
            let _ = transliterator
                .transliterate(text, from_script, to_script)
                .unwrap();
            let duration = start.elapsed();

            results.push(BenchmarkResult {
                script_from: from_script.to_string(),
                script_to: to_script.to_string(),
                category: category.to_string(),
                text_size: size_name.to_string(),
                throughput_chars_per_sec: text.chars().count() as f64 / duration.as_secs_f64(),
                latency_ns: duration.as_nanos() as f64,
            });
        }
    }

    group.finish();
    write_benchmark_results(category, &results);
}

fn write_benchmark_results(category: &str, results: &[BenchmarkResult]) {
    let _ = fs::create_dir_all("target");
    let filename = format!("target/benchmark_results_{}.csv", category);
    let mut csv_content = String::from(
        "script_from,script_to,category,text_size,throughput_chars_per_sec,latency_ns\n",
    );

    for result in results {
        csv_content.push_str(&result.to_csv_row());
        csv_content.push('\n');
    }

    let _ = fs::write(&filename, csv_content);
}

fn generate_markdown_report() {
    let categories = [
        "hub",
        "indic_to_roman",
        "roman_to_indic",
        "roman_to_roman",
        "indic_to_indic",
        "tamil_one_way",
    ];
    let mut all_results = Vec::new();

    // Read all CSV files
    for category in &categories {
        let filename = format!("target/benchmark_results_{}.csv", category);
        if let Ok(content) = fs::read_to_string(&filename) {
            for line in content.lines().skip(1) {
                // Skip header
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() == 6 {
                    all_results.push(BenchmarkResult {
                        script_from: parts[0].to_string(),
                        script_to: parts[1].to_string(),
                        category: parts[2].to_string(),
                        text_size: parts[3].to_string(),
                        throughput_chars_per_sec: parts[4].parse().unwrap_or(0.0),
                        latency_ns: parts[5].parse().unwrap_or(0.0),
                    });
                }
            }
        }
    }

    // Generate markdown
    let mut md_content = String::new();
    md_content.push_str("# Shlesha Performance Benchmark Results\n\n");
    md_content.push_str("## Overview\n\n");
    md_content.push_str("This benchmark covers representative performance patterns:\n");
    md_content.push_str("- **Hub**: Direct hub conversions (fastest path)\n");
    md_content
        .push_str("- **Indic → Roman**: 2-hop conversions via hub (Telugu → Roman scripts)\n");
    md_content.push_str("- **Roman → Indic**: 2-hop conversions via hub (IAST → Indic scripts)\n");
    md_content.push_str("- **Roman → Roman**: 1-hop conversions via ISO hub\n");
    md_content.push_str("- **Indic → Indic**: 3-hop conversions via hub\n\n");

    // Hub Scripts Performance
    md_content.push_str("## Hub Scripts (Direct Devanagari ↔ ISO-15919)\n\n");
    md_content.push_str("| From | To | Text Size | Throughput (chars/sec) | Latency (ns) |\n");
    md_content.push_str("|------|----|-----------|-----------------------|-------------|\n");

    for result in &all_results {
        if result.category == "hub" {
            md_content.push_str(&format!(
                "| {} | {} | {} | {:.0} | {:.0} |\n",
                result.script_from,
                result.script_to,
                result.text_size,
                result.throughput_chars_per_sec,
                result.latency_ns
            ));
        }
    }

    // Indic to Roman Performance
    md_content.push_str("\n## Indic → Roman Scripts (Telugu → Roman)\n\n");
    md_content.push_str("| From | To | Text Size | Throughput (chars/sec) | Latency (ns) |\n");
    md_content.push_str("|------|----|-----------|-----------------------|-------------|\n");

    for result in &all_results {
        if result.category == "indic_to_roman" {
            md_content.push_str(&format!(
                "| {} | {} | {} | {:.0} | {:.0} |\n",
                result.script_from,
                result.script_to,
                result.text_size,
                result.throughput_chars_per_sec,
                result.latency_ns
            ));
        }
    }

    // Roman to Indic Performance
    md_content.push_str("\n## Roman → Indic Scripts (IAST → Indic)\n\n");
    md_content.push_str("| From | To | Text Size | Throughput (chars/sec) | Latency (ns) |\n");
    md_content.push_str("|------|----|-----------|-----------------------|-------------|\n");

    for result in &all_results {
        if result.category == "roman_to_indic" {
            md_content.push_str(&format!(
                "| {} | {} | {} | {:.0} | {:.0} |\n",
                result.script_from,
                result.script_to,
                result.text_size,
                result.throughput_chars_per_sec,
                result.latency_ns
            ));
        }
    }

    // Roman to Roman Performance
    md_content.push_str("\n## Roman → Roman Scripts\n\n");
    md_content.push_str("| From | To | Text Size | Throughput (chars/sec) | Latency (ns) |\n");
    md_content.push_str("|------|----|-----------|-----------------------|-------------|\n");

    for result in &all_results {
        if result.category == "roman_to_roman" {
            md_content.push_str(&format!(
                "| {} | {} | {} | {:.0} | {:.0} |\n",
                result.script_from,
                result.script_to,
                result.text_size,
                result.throughput_chars_per_sec,
                result.latency_ns
            ));
        }
    }

    // Indic to Indic Performance
    md_content.push_str("\n## Indic → Indic Scripts\n\n");
    md_content.push_str("| From | To | Text Size | Throughput (chars/sec) | Latency (ns) |\n");
    md_content.push_str("|------|----|-----------|-----------------------|-------------|\n");

    for result in &all_results {
        if result.category == "indic_to_indic" {
            md_content.push_str(&format!(
                "| {} | {} | {} | {:.0} | {:.0} |\n",
                result.script_from,
                result.script_to,
                result.text_size,
                result.throughput_chars_per_sec,
                result.latency_ns
            ));
        }
    }

    // Tamil One-way Performance
    md_content.push_str("\n## Tamil One-way Conversions\n\n");
    md_content.push_str("| From | To | Text Size | Throughput (chars/sec) | Latency (ns) |\n");
    md_content.push_str("|------|----|-----------|-----------------------|-------------|\n");

    for result in &all_results {
        if result.category == "tamil_one_way" {
            md_content.push_str(&format!(
                "| {} | {} | {} | {:.0} | {:.0} |\n",
                result.script_from,
                result.script_to,
                result.text_size,
                result.throughput_chars_per_sec,
                result.latency_ns
            ));
        }
    }

    // Performance Analysis
    md_content.push_str("\n## Performance Analysis\n\n");

    let mut category_stats = std::collections::HashMap::new();
    for result in &all_results {
        let stats = category_stats.entry(&result.category).or_insert(Vec::new());
        stats.push(result.throughput_chars_per_sec);
    }

    md_content
        .push_str("| Conversion Type | Avg Throughput (chars/sec) | Relative Performance |\n");
    md_content.push_str("|-----------------|----------------------------|---------------------|\n");

    let mut category_avgs = Vec::new();
    for (category, throughputs) in &category_stats {
        if !throughputs.is_empty() {
            let avg = throughputs.iter().sum::<f64>() / throughputs.len() as f64;
            category_avgs.push((category.as_str(), avg));
        }
    }

    category_avgs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let baseline = category_avgs.first().map(|(_, avg)| *avg).unwrap_or(1.0);

    for (category, avg) in category_avgs {
        let relative = avg / baseline;
        let category_name = match category {
            "hub" => "Hub (Direct)",
            "indic_to_roman" => "Indic → Roman (2-hop)",
            "roman_to_indic" => "Roman → Indic (2-hop)",
            "roman_to_roman" => "Roman → Roman (1-hop)",
            "indic_to_indic" => "Indic → Indic (3-hop)",
            "tamil_one_way" => "Tamil (One-way)",
            _ => category,
        };
        md_content.push_str(&format!(
            "| {} | {:.0} | {:.2}x |\n",
            category_name, avg, relative
        ));
    }

    let _ = fs::write("target/BENCHMARK_RESULTS.md", md_content);
}

fn bench_hub_conversions(c: &mut Criterion) {
    benchmark_conversion_set(c, "hub", HUB_CONVERSIONS);
}

fn bench_indic_to_roman(c: &mut Criterion) {
    benchmark_conversion_set(c, "indic_to_roman", INDIC_TO_ROMAN);
}

fn bench_roman_to_indic(c: &mut Criterion) {
    benchmark_conversion_set(c, "roman_to_indic", ROMAN_TO_INDIC);
}

fn bench_roman_to_roman(c: &mut Criterion) {
    benchmark_conversion_set(c, "roman_to_roman", ROMAN_TO_ROMAN);
}

fn bench_indic_to_indic(c: &mut Criterion) {
    benchmark_conversion_set(c, "indic_to_indic", INDIC_TO_INDIC);
}

fn bench_tamil_one_way(c: &mut Criterion) {
    benchmark_conversion_set(c, "tamil_one_way", TAMIL_ONE_WAY);
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(5))
        .sample_size(50);
    targets = bench_hub_conversions, bench_indic_to_roman, bench_roman_to_indic, bench_roman_to_roman, bench_indic_to_indic, bench_tamil_one_way
);

criterion_main!(benches);
