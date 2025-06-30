use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;
use shlesha::Shlesha;
use std::time::Duration;

// Same Telugu text samples as the Python script for direct comparison
const TELUGU_SAMPLES: &[&str] = &[
    "నమస్కారం",    // namaskaram
    "భారతదేశం",    // bharatadesha
    "సంస్కృతం",     // sanskritam
    "తెలుగు",       // telugu
    "వేదం",        // vedam
    "గీత",         // gita
    "రామాయణం",     // ramayanam
    "మహాభారతం",    // mahabharatam
    "శ్రీకృష్ణ",     // shrikrishna
    "గురుపూర్ణిమ",   // gurupurnima
    "అష్టాధ్యాయి",   // ashtadhyayi
    "పాణిని",       // panini
    "కాలిదాస",      // kalidasa
    "శాకుంతలం",     // shakuntalam
    "మేఘదూతం",      // meghadutam
];

fn benchmark_shlesha_batch(c: &mut Criterion) {
    let transliterator = Shlesha::new();
    
    let mut group = c.benchmark_group("telugu_to_slp1_batch");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(100);
    
    // Calculate total text length for throughput
    let _total_chars: usize = TELUGU_SAMPLES.iter().map(|s| s.chars().count()).sum();
    group.throughput(Throughput::Elements(TELUGU_SAMPLES.len() as u64));
    
    group.bench_function("shlesha_15_samples", |b| {
        b.iter(|| {
            for &text in TELUGU_SAMPLES {
                let _result = transliterator
                    .transliterate(black_box(text), black_box("telugu"), black_box("slp1"))
                    .unwrap();
            }
        })
    });
    
    group.finish();
}

fn benchmark_individual_samples(c: &mut Criterion) {
    let transliterator = Shlesha::new();
    
    let mut group = c.benchmark_group("individual_telugu_samples");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(100);
    
    for (i, &sample) in TELUGU_SAMPLES.iter().enumerate() {
        let char_count = sample.chars().count();
        group.throughput(Throughput::Elements(char_count as u64));
        
        group.bench_with_input(
            BenchmarkId::new("shlesha", format!("sample_{}_{}chars", i + 1, char_count)),
            sample,
            |b, text| {
                b.iter(|| {
                    transliterator
                        .transliterate(black_box(text), black_box("telugu"), black_box("slp1"))
                        .unwrap()
                })
            },
        );
    }
    
    group.finish();
}

fn benchmark_comparison_patterns(c: &mut Criterion) {
    let transliterator = Shlesha::new();
    
    let mut group = c.benchmark_group("comparison_patterns");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(100);
    
    // Test the same patterns as mentioned in the performance comparison
    let test_cases = [
        ("telugu_to_slp1", "telugu", "slp1", TELUGU_SAMPLES),
        // Add IAST samples for Roman→Indic comparison
        ("iast_to_telugu", "iast", "telugu", &[
            "namaskāraṃ", "bhāratadeśaṃ", "saṃskṛtaṃ", "telugu", "vedaṃ",
            "gītā", "rāmāyaṇaṃ", "mahābhārataṃ", "śrīkṛṣṇa", "gurupūrṇimā",
            "aṣṭādhyāyī", "pāṇini", "kālidāsa", "śākuntalaṃ", "meghadūtaṃ",
        ]),
    ];
    
    for (test_name, from_script, to_script, samples) in &test_cases {
        let _total_chars: usize = samples.iter().map(|s| s.chars().count()).sum();
        group.throughput(Throughput::Elements(samples.len() as u64));
        
        group.bench_function(*test_name, |b| {
            b.iter(|| {
                for &text in *samples {
                    let _result = transliterator
                        .transliterate(black_box(text), black_box(from_script), black_box(to_script))
                        .unwrap();
                }
            })
        });
    }
    
    group.finish();
}

fn performance_analysis() {
    let transliterator = Shlesha::new();
    
    println!("\n=== Shlesha Performance Analysis ===");
    println!("Testing {} Telugu samples to SLP1", TELUGU_SAMPLES.len());
    
    // Warm up
    for &text in TELUGU_SAMPLES {
        let _ = transliterator.transliterate(text, "telugu", "slp1").unwrap();
    }
    
    // Time multiple iterations
    let iterations = 1000;
    let start = std::time::Instant::now();
    
    for _ in 0..iterations {
        for &text in TELUGU_SAMPLES {
            let _ = transliterator.transliterate(text, "telugu", "slp1").unwrap();
        }
    }
    
    let duration = start.elapsed();
    let total_texts = iterations * TELUGU_SAMPLES.len();
    let texts_per_second = total_texts as f64 / duration.as_secs_f64();
    
    println!("Total texts processed: {}", total_texts);
    println!("Total time: {:.3} seconds", duration.as_secs_f64());
    println!("Throughput: {:.0} texts/second", texts_per_second);
    println!("Average time per text: {:.3} µs", duration.as_nanos() as f64 / total_texts as f64 / 1000.0);
    
    // Show sample results
    println!("\nSample translations:");
    for (_i, &text) in TELUGU_SAMPLES.iter().take(5).enumerate() {
        let result = transliterator.transliterate(text, "telugu", "slp1").unwrap();
        println!("  {} → {}", text, result);
    }
}

criterion_group!(
    name = comparison_benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(100);
    targets = benchmark_shlesha_batch, benchmark_individual_samples, benchmark_comparison_patterns
);

criterion_main!(comparison_benches);

// Helper function to run analysis
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_performance_analysis() {
        performance_analysis();
    }
}