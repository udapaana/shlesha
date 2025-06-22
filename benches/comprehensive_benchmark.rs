use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use shlesha::Shlesha;
use std::fs;
use std::time::Duration;

// Test data sets
const SMALL_TEXT: &str = "धर्म";
const MEDIUM_TEXT: &str = "धर्म योग भारत संस्कृत वेद उपनिषद् गीता रामायण महाभारत";
const LARGE_TEXT: &str = "धर्म योग भारत संस्कृत वेद उपनिषद् गीता रामायण महाभारत पुराण शास्त्र दर्शन आयुर्वेद ज्योतिष व्याकरण छन्द निरुक्त कल्प शिक्षा स्मृति श्रुति आचार विचार संस्कार परम्परा सत्य अहिंसा करुणा दया प्रेम शान्ति आनन्द मोक्ष निर्वाण समाधि ध्यान प्राणायाम आसन मन्त्र यन्त्र तन्त्र";

// Hub scripts (direct devanagari <-> iso)
const HUB_SCRIPTS: &[&str] = &["devanagari", "iso15919"];

// Standard Indic scripts
const STANDARD_SCRIPTS: &[&str] = &["bengali", "tamil", "telugu", "gujarati", "kannada", "malayalam", "odia"];

// Roman/ASCII schemes (extensions)
const EXTENSION_SCRIPTS: &[&str] = &["iast", "itrans", "slp1", "harvard_kyoto", "velthuis", "wx"];

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
        format!("{},{},{},{},{:.0},{:.0}", 
            self.script_from, self.script_to, self.category, 
            self.text_size, self.throughput_chars_per_sec, self.latency_ns)
    }
}

fn benchmark_transliteration_category(c: &mut Criterion, category: &str, scripts: &[&str]) {
    let transliterator = Shlesha::new();
    let mut results = Vec::new();
    
    let mut group = c.benchmark_group(category);
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(100);
    
    for &from_script in scripts {
        for &to_script in scripts {
            if from_script == to_script {
                continue;
            }
            
            // Test with different text sizes
            for (size_name, text) in [
                ("small", SMALL_TEXT),
                ("medium", MEDIUM_TEXT), 
                ("large", LARGE_TEXT)
            ] {
                let benchmark_name = format!("{}_{}_to_{}", size_name, from_script, to_script);
                
                group.throughput(Throughput::Bytes(text.len() as u64));
                group.bench_with_input(
                    BenchmarkId::new(&benchmark_name, text.len()),
                    &text,
                    |b, text| {
                        b.iter(|| {
                            transliterator.transliterate(
                                black_box(text),
                                black_box(from_script),
                                black_box(to_script)
                            ).unwrap()
                        })
                    }
                );
                
                // Measure for results collection
                let start = std::time::Instant::now();
                let _ = transliterator.transliterate(text, from_script, to_script).unwrap();
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
    }
    
    group.finish();
    
    // Write results to file
    write_benchmark_results(category, &results);
}

fn benchmark_cross_category(c: &mut Criterion) {
    let transliterator = Shlesha::new();
    let mut results = Vec::new();
    
    let mut group = c.benchmark_group("cross_category");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(100);
    
    // Hub to Standard scripts
    for &hub_script in HUB_SCRIPTS {
        for &standard_script in STANDARD_SCRIPTS {
            for (size_name, text) in [
                ("small", SMALL_TEXT),
                ("medium", MEDIUM_TEXT),
                ("large", LARGE_TEXT)
            ] {
                let benchmark_name = format!("{}_{}_to_{}", size_name, hub_script, standard_script);
                
                group.throughput(Throughput::Bytes(text.len() as u64));
                group.bench_with_input(
                    BenchmarkId::new(&benchmark_name, text.len()),
                    &text,
                    |b, text| {
                        b.iter(|| {
                            transliterator.transliterate(
                                black_box(text),
                                black_box(hub_script),
                                black_box(standard_script)
                            ).unwrap()
                        })
                    }
                );
                
                let start = std::time::Instant::now();
                let _ = transliterator.transliterate(text, hub_script, standard_script).unwrap();
                let duration = start.elapsed();
                
                results.push(BenchmarkResult {
                    script_from: hub_script.to_string(),
                    script_to: standard_script.to_string(),
                    category: "cross_hub_to_standard".to_string(),
                    text_size: size_name.to_string(),
                    throughput_chars_per_sec: text.chars().count() as f64 / duration.as_secs_f64(),
                    latency_ns: duration.as_nanos() as f64,
                });
            }
        }
    }
    
    // Hub to Extension scripts
    for &hub_script in HUB_SCRIPTS {
        for &ext_script in EXTENSION_SCRIPTS {
            for (size_name, text) in [
                ("small", SMALL_TEXT),
                ("medium", MEDIUM_TEXT),
                ("large", LARGE_TEXT)
            ] {
                let benchmark_name = format!("{}_{}_to_{}", size_name, hub_script, ext_script);
                
                group.throughput(Throughput::Bytes(text.len() as u64));
                group.bench_with_input(
                    BenchmarkId::new(&benchmark_name, text.len()),
                    &text,
                    |b, text| {
                        b.iter(|| {
                            transliterator.transliterate(
                                black_box(text),
                                black_box(hub_script),
                                black_box(ext_script)
                            ).unwrap()
                        })
                    }
                );
                
                let start = std::time::Instant::now();
                let _ = transliterator.transliterate(text, hub_script, ext_script).unwrap();
                let duration = start.elapsed();
                
                results.push(BenchmarkResult {
                    script_from: hub_script.to_string(),
                    script_to: ext_script.to_string(),
                    category: "cross_hub_to_extension".to_string(),
                    text_size: size_name.to_string(),
                    throughput_chars_per_sec: text.chars().count() as f64 / duration.as_secs_f64(),
                    latency_ns: duration.as_nanos() as f64,
                });
            }
        }
    }
    
    group.finish();
    write_benchmark_results("cross_category", &results);
}

fn write_benchmark_results(category: &str, results: &[BenchmarkResult]) {
    let _ = fs::create_dir_all("target");
    let filename = format!("target/benchmark_results_{}.csv", category);
    let mut csv_content = String::from("script_from,script_to,category,text_size,throughput_chars_per_sec,latency_ns\n");
    
    for result in results {
        csv_content.push_str(&result.to_csv_row());
        csv_content.push('\n');
    }
    
    let _ = fs::write(&filename, csv_content);
}

fn generate_markdown_report() {
    let categories = ["hub", "standard", "extension", "cross_category"];
    let mut all_results = Vec::new();
    
    // Read all CSV files
    for category in &categories {
        let filename = format!("target/benchmark_results_{}.csv", category);
        if let Ok(content) = fs::read_to_string(&filename) {
            for line in content.lines().skip(1) { // Skip header
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
    
    // Hub Scripts Performance
    md_content.push_str("## Hub Scripts (Devanagari ↔ ISO-15919)\n\n");
    md_content.push_str("| From | To | Text Size | Throughput (chars/sec) | Latency (ns) |\n");
    md_content.push_str("|------|----|-----------|-----------------------|-------------|\n");
    
    for result in &all_results {
        if result.category == "hub" {
            md_content.push_str(&format!(
                "| {} | {} | {} | {:.0} | {:.0} |\n",
                result.script_from, result.script_to, result.text_size,
                result.throughput_chars_per_sec, result.latency_ns
            ));
        }
    }
    
    // Standard Scripts Performance
    md_content.push_str("\n## Standard Indic Scripts\n\n");
    md_content.push_str("| From | To | Text Size | Throughput (chars/sec) | Latency (ns) |\n");
    md_content.push_str("|------|----|-----------|-----------------------|-------------|\n");
    
    for result in &all_results {
        if result.category == "standard" {
            md_content.push_str(&format!(
                "| {} | {} | {} | {:.0} | {:.0} |\n",
                result.script_from, result.script_to, result.text_size,
                result.throughput_chars_per_sec, result.latency_ns
            ));
        }
    }
    
    // Extension Scripts Performance
    md_content.push_str("\n## Extension Scripts (Roman/ASCII)\n\n");
    md_content.push_str("| From | To | Text Size | Throughput (chars/sec) | Latency (ns) |\n");
    md_content.push_str("|------|----|-----------|-----------------------|-------------|\n");
    
    for result in &all_results {
        if result.category == "extension" {
            md_content.push_str(&format!(
                "| {} | {} | {} | {:.0} | {:.0} |\n",
                result.script_from, result.script_to, result.text_size,
                result.throughput_chars_per_sec, result.latency_ns
            ));
        }
    }
    
    // Cross-Category Performance
    md_content.push_str("\n## Cross-Category Performance\n\n");
    md_content.push_str("| From | To | Category | Text Size | Throughput (chars/sec) | Latency (ns) |\n");
    md_content.push_str("|------|----|-----------|-----------|-----------------------|-------------|\n");
    
    for result in &all_results {
        if result.category.starts_with("cross_") {
            md_content.push_str(&format!(
                "| {} | {} | {} | {} | {:.0} | {:.0} |\n",
                result.script_from, result.script_to, result.category, result.text_size,
                result.throughput_chars_per_sec, result.latency_ns
            ));
        }
    }
    
    // Summary Statistics
    md_content.push_str("\n## Summary Statistics\n\n");
    
    let hub_avg = calculate_avg_throughput(&all_results, "hub");
    let standard_avg = calculate_avg_throughput(&all_results, "standard");
    let extension_avg = calculate_avg_throughput(&all_results, "extension");
    
    md_content.push_str("| Category | Average Throughput (chars/sec) |\n");
    md_content.push_str("|----------|--------------------------------|\n");
    md_content.push_str(&format!("| Hub Scripts | {:.0} |\n", hub_avg));
    md_content.push_str(&format!("| Standard Scripts | {:.0} |\n", standard_avg));
    md_content.push_str(&format!("| Extension Scripts | {:.0} |\n", extension_avg));
    
    let _ = fs::write("target/BENCHMARK_RESULTS.md", md_content);
}

fn calculate_avg_throughput(results: &[BenchmarkResult], category: &str) -> f64 {
    let filtered: Vec<&BenchmarkResult> = results.iter()
        .filter(|r| r.category == category)
        .collect();
    
    if filtered.is_empty() {
        return 0.0;
    }
    
    let sum: f64 = filtered.iter().map(|r| r.throughput_chars_per_sec).sum();
    sum / filtered.len() as f64
}

fn bench_hub_scripts(c: &mut Criterion) {
    benchmark_transliteration_category(c, "hub", HUB_SCRIPTS);
}

fn bench_standard_scripts(c: &mut Criterion) {
    benchmark_transliteration_category(c, "standard", STANDARD_SCRIPTS);
}

fn bench_extension_scripts(c: &mut Criterion) {
    benchmark_transliteration_category(c, "extension", EXTENSION_SCRIPTS);
}

fn bench_cross_category(c: &mut Criterion) {
    benchmark_cross_category(c);
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(100);
    targets = bench_hub_scripts, bench_standard_scripts, bench_extension_scripts, bench_cross_category
);

criterion_main!(benches);