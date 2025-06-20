use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use std::time::Duration;
use std::collections::HashMap;

// Import Shlesha
use shlesha::{Transliterator, TransliteratorBuilder, Schema};

// Import vidyut-lipi (already in dev-dependencies)
use vidyut_lipi::{Scheme, transliterate as vidyut_transliterate};

// Import any other available Rust transliteration crates
// Note: IndicScriptSwap is mentioned in search results but may not be production ready
// #[cfg(feature = "indicscriptswap")]
// use indicscriptswap::transliterate as indicscriptswap_transliterate;

/// Standard test corpus with varying complexity
const TEST_CORPUS: &[(&str, &str, &str)] = &[
    // (description, devanagari_text, expected_iast)
    ("single_char", "क", "ka"),
    ("single_word", "नमस्ते", "namaste"),
    ("short_sentence", "अहं संस्कृतं वदामि", "ahaṃ saṃskṛtaṃ vadāmi"),
    ("with_conjuncts", "कृष्णार्जुनसंवादः", "kṛṣṇārjunasaṃvādaḥ"),
    ("with_vedic", "अग्निमीळे पुरोहितं", "agnimīḷe purohitaṃ"),
    ("medium_verse", "तत्र शूरा महेष्वासा भीमार्जुनसमा युधि", "tatra śūrā maheṣvāsā bhīmārjunasamā yudhi"),
    ("long_sentence", "धृतराष्ट्र उवाच धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः", "dhṛtarāṣṭra uvāca dharmakṣetre kurukṣetre samavetā yuyutsavaḥ"),
];

/// Accuracy test cases
const ACCURACY_TESTS: &[(&str, &str)] = &[
    ("नमस्ते", "namaste"),
    ("संस्कृतम्", "saṃskṛtam"),
    ("कृष्ण", "kṛṣṇa"),
    ("ज्ञान", "jñāna"),
    ("अग्निमीळे", "agnimīḷe"),
    ("यज्ञस्य", "yajñasya"),
    ("ऋत्विजम्", "ṛtvijam"),
    ("द्वौ", "dvau"),
    ("त्र्यम्बकम्", "tryambakam"),
    ("पुष्टिवर्धनम्", "puṣṭivardhanam"),
    ("श्रीः", "śrīḥ"),
    ("कार्त्स्न्येन", "kārtsnyena"),
    ("वाङ्मयम्", "vāṅmayam"),
    ("षट्कर्म", "ṣaṭkarma"),
    ("अष्टाध्यायी", "aṣṭādhyāyī"),
];

/// Large corpus for throughput testing
fn generate_large_corpus() -> Vec<(String, usize)> {
    vec![
        // 1KB of text
        (
            "तत्र शूरा महेष्वासा भीमार्जुनसमा युधि । युयुधानो विराटश्च द्रुपदश्च महारथः ॥ ".repeat(10),
            1_000
        ),
        // 10KB of text
        (
            "धृतराष्ट्र उवाच धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः । मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय ॥ ".repeat(50),
            10_000
        ),
        // 100KB of text
        (
            "कर्मण्येवाधिकारस्ते मा फलेषु कदाचन । मा कर्मफलहेतुर्भूर्मा ते सङ्गोऽस्त्वकर्मणि ॥ ".repeat(500),
            100_000
        ),
        // 1MB of text
        (
            "तत्र शूरा महेष्वासा भीमार्जुनसमा युधि । युयुधानो विराटश्च द्रुपदश्च महारथः ॥ ".repeat(5000),
            1_000_000
        ),
    ]
}

/// Trait for benchmarking different transliterators
trait TransliteratorBenchmark {
    fn name(&self) -> &'static str;
    fn transliterate(&self, text: &str, from: &str, to: &str) -> Result<String, Box<dyn std::error::Error>>;
    fn is_available(&self) -> bool { true }
}

/// Shlesha transliterator wrapper
struct ShleshaTransliterator {
    transliterator: Transliterator,
}

impl ShleshaTransliterator {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let devanagari_schema = include_str!("../schemas/devanagari.yaml");
        let iast_schema = include_str!("../schemas/iast.yaml");
        
        let dev_schema = Schema::from_yaml_str(devanagari_schema)?;
        let iast_schema = Schema::from_yaml_str(iast_schema)?;
        
        let transliterator = TransliteratorBuilder::new()
            .with_schema(dev_schema)?
            .with_schema(iast_schema)?
            .build();
            
        Ok(Self { transliterator })
    }
}

impl TransliteratorBenchmark for ShleshaTransliterator {
    fn name(&self) -> &'static str {
        "Shlesha"
    }
    
    fn transliterate(&self, text: &str, from: &str, to: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self.transliterator.transliterate(text, from, to)?)
    }
}

/// Vidyut-lipi transliterator wrapper
struct VidyutTransliterator;

impl TransliteratorBenchmark for VidyutTransliterator {
    fn name(&self) -> &'static str {
        "Vidyut-lipi"
    }
    
    fn transliterate(&self, text: &str, from: &str, to: &str) -> Result<String, Box<dyn std::error::Error>> {
        let from_scheme = match from {
            "Devanagari" | "devanagari" => Scheme::Devanagari,
            "IAST" | "iast" => Scheme::Iast,
            _ => return Err("Unsupported source scheme".into()),
        };
        
        let to_scheme = match to {
            "Devanagari" | "devanagari" => Scheme::Devanagari,
            "IAST" | "iast" => Scheme::Iast,
            _ => return Err("Unsupported target scheme".into()),
        };
        
        Ok(vidyut_transliterate(text, from_scheme, to_scheme))
    }
}

/// Simple character mapping baseline (for comparison)
struct SimpleMapping {
    devanagari_to_iast: HashMap<char, &'static str>,
}

impl SimpleMapping {
    fn new() -> Self {
        let mut devanagari_to_iast = HashMap::new();
        
        // Basic consonants
        devanagari_to_iast.insert('क', "ka");
        devanagari_to_iast.insert('ख', "kha");
        devanagari_to_iast.insert('ग', "ga");
        devanagari_to_iast.insert('घ', "gha");
        devanagari_to_iast.insert('ङ', "ṅa");
        devanagari_to_iast.insert('च', "ca");
        devanagari_to_iast.insert('छ', "cha");
        devanagari_to_iast.insert('ज', "ja");
        devanagari_to_iast.insert('झ', "jha");
        devanagari_to_iast.insert('ञ', "ña");
        devanagari_to_iast.insert('ट', "ṭa");
        devanagari_to_iast.insert('ठ', "ṭha");
        devanagari_to_iast.insert('ड', "ḍa");
        devanagari_to_iast.insert('ढ', "ḍha");
        devanagari_to_iast.insert('ण', "ṇa");
        devanagari_to_iast.insert('त', "ta");
        devanagari_to_iast.insert('थ', "tha");
        devanagari_to_iast.insert('द', "da");
        devanagari_to_iast.insert('ध', "dha");
        devanagari_to_iast.insert('न', "na");
        devanagari_to_iast.insert('प', "pa");
        devanagari_to_iast.insert('फ', "pha");
        devanagari_to_iast.insert('ब', "ba");
        devanagari_to_iast.insert('भ', "bha");
        devanagari_to_iast.insert('म', "ma");
        devanagari_to_iast.insert('य', "ya");
        devanagari_to_iast.insert('र', "ra");
        devanagari_to_iast.insert('ल', "la");
        devanagari_to_iast.insert('व', "va");
        devanagari_to_iast.insert('श', "śa");
        devanagari_to_iast.insert('ष', "ṣa");
        devanagari_to_iast.insert('स', "sa");
        devanagari_to_iast.insert('ह', "ha");
        
        // Vowels
        devanagari_to_iast.insert('अ', "a");
        devanagari_to_iast.insert('आ', "ā");
        devanagari_to_iast.insert('इ', "i");
        devanagari_to_iast.insert('ई', "ī");
        devanagari_to_iast.insert('उ', "u");
        devanagari_to_iast.insert('ऊ', "ū");
        devanagari_to_iast.insert('ऋ', "ṛ");
        devanagari_to_iast.insert('ॠ', "ṝ");
        devanagari_to_iast.insert('ऌ', "ḷ");
        devanagari_to_iast.insert('ए', "e");
        devanagari_to_iast.insert('ऐ', "ai");
        devanagari_to_iast.insert('ओ', "o");
        devanagari_to_iast.insert('औ', "au");
        
        // Matras
        devanagari_to_iast.insert('ा', "ā");
        devanagari_to_iast.insert('ि', "i");
        devanagari_to_iast.insert('ी', "ī");
        devanagari_to_iast.insert('ु', "u");
        devanagari_to_iast.insert('ू', "ū");
        devanagari_to_iast.insert('ृ', "ṛ");
        devanagari_to_iast.insert('े', "e");
        devanagari_to_iast.insert('ै', "ai");
        devanagari_to_iast.insert('ो', "o");
        devanagari_to_iast.insert('ौ', "au");
        
        // Other marks
        devanagari_to_iast.insert('ं', "ṃ");
        devanagari_to_iast.insert('ः', "ḥ");
        devanagari_to_iast.insert('्', "");
        devanagari_to_iast.insert(' ', " ");
        
        Self { devanagari_to_iast }
    }
}

impl TransliteratorBenchmark for SimpleMapping {
    fn name(&self) -> &'static str {
        "Simple Mapping (baseline)"
    }
    
    fn transliterate(&self, text: &str, _from: &str, _to: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut result = String::new();
        for ch in text.chars() {
            if let Some(&iast) = self.devanagari_to_iast.get(&ch) {
                result.push_str(iast);
            } else {
                result.push(ch);
            }
        }
        Ok(result)
    }
}

/// Run performance benchmarks
fn bench_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("rust_transliterators");
    
    // Initialize transliterators
    let transliterators: Vec<Box<dyn TransliteratorBenchmark>> = vec![
        Box::new(ShleshaTransliterator::new().expect("Failed to create Shlesha transliterator")),
        Box::new(VidyutTransliterator),
        Box::new(SimpleMapping::new()),
    ];
    
    // Benchmark each test case
    for (desc, devanagari, _expected) in TEST_CORPUS {
        for transliterator in &transliterators {
            if !transliterator.is_available() {
                continue;
            }
            
            group.bench_with_input(
                BenchmarkId::new(transliterator.name(), desc),
                devanagari,
                |b, text| {
                    b.iter(|| {
                        black_box(transliterator.transliterate(
                            black_box(text),
                            black_box("Devanagari"),
                            black_box("IAST")
                        ).unwrap())
                    });
                },
            );
        }
    }
    
    group.finish();
}

/// Run throughput benchmarks
fn bench_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("rust_throughput");
    
    // Initialize transliterators
    let transliterators: Vec<Box<dyn TransliteratorBenchmark>> = vec![
        Box::new(ShleshaTransliterator::new().expect("Failed to create Shlesha transliterator")),
        Box::new(VidyutTransliterator),
        Box::new(SimpleMapping::new()),
    ];
    
    let large_corpus = generate_large_corpus();
    
    for (text, size) in &large_corpus {
        group.throughput(Throughput::Bytes(*size as u64));
        
        for transliterator in &transliterators {
            if !transliterator.is_available() {
                continue;
            }
            
            group.bench_with_input(
                BenchmarkId::new(transliterator.name(), format!("{}KB", size / 1000)),
                text,
                |b, text| {
                    b.iter(|| {
                        black_box(transliterator.transliterate(
                            black_box(text),
                            black_box("Devanagari"),
                            black_box("IAST")
                        ).unwrap())
                    });
                },
            );
        }
    }
    
    group.finish();
}

/// Run accuracy tests (not a benchmark, but useful for comparison)
fn bench_accuracy(c: &mut Criterion) {
    let mut group = c.benchmark_group("rust_accuracy");
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_secs(1));
    
    // Initialize transliterators
    let transliterators: Vec<Box<dyn TransliteratorBenchmark>> = vec![
        Box::new(ShleshaTransliterator::new().expect("Failed to create Shlesha transliterator")),
        Box::new(VidyutTransliterator),
        Box::new(SimpleMapping::new()),
    ];
    
    // Test accuracy for each transliterator
    for transliterator in &transliterators {
        if !transliterator.is_available() {
            continue;
        }
        
        let mut correct = 0;
        let mut total = 0;
        
        for (devanagari, expected) in ACCURACY_TESTS {
            total += 1;
            if let Ok(result) = transliterator.transliterate(devanagari, "Devanagari", "IAST") {
                if result == *expected {
                    correct += 1;
                }
            }
        }
        
        let accuracy = (correct as f64 / total as f64) * 100.0;
        println!("{} accuracy: {:.1}% ({}/{})", transliterator.name(), accuracy, correct, total);
    }
    
    group.finish();
}

/// Memory allocation benchmark
fn bench_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("rust_memory");
    
    let transliterators: Vec<Box<dyn TransliteratorBenchmark>> = vec![
        Box::new(ShleshaTransliterator::new().expect("Failed to create Shlesha transliterator")),
        Box::new(VidyutTransliterator),
        Box::new(SimpleMapping::new()),
    ];
    
    let test_text = "तत्र शूरा महेष्वासा भीमार्जुनसमा युधि";
    
    for transliterator in &transliterators {
        if !transliterator.is_available() {
            continue;
        }
        
        group.bench_function(
            BenchmarkId::new("repeated_transliteration", transliterator.name()),
            |b| {
                b.iter(|| {
                    // Perform 100 transliterations to test allocation patterns
                    for _ in 0..100 {
                        let _result = transliterator.transliterate(
                            black_box(test_text),
                            black_box("Devanagari"),
                            black_box("IAST")
                        ).unwrap();
                    }
                });
            },
        );
    }
    
    group.finish();
}

/// Round-trip benchmark (Devanagari -> IAST -> Devanagari)
fn bench_round_trip(c: &mut Criterion) {
    let mut group = c.benchmark_group("rust_round_trip");
    
    // Only test transliterators that support bidirectional conversion
    let shlesha = ShleshaTransliterator::new().expect("Failed to create Shlesha transliterator");
    let vidyut = VidyutTransliterator;
    
    for (desc, devanagari, _expected) in TEST_CORPUS {
        // Shlesha round-trip
        group.bench_with_input(
            BenchmarkId::new("Shlesha", desc),
            devanagari,
            |b, text| {
                b.iter(|| {
                    let iast = shlesha.transliterate(
                        black_box(text),
                        black_box("Devanagari"),
                        black_box("IAST")
                    ).unwrap();
                    let _back = shlesha.transliterate(
                        black_box(&iast),
                        black_box("IAST"),
                        black_box("Devanagari")
                    ).unwrap();
                });
            },
        );
        
        // Vidyut round-trip
        group.bench_with_input(
            BenchmarkId::new("Vidyut-lipi", desc),
            devanagari,
            |b, text| {
                b.iter(|| {
                    let iast = vidyut.transliterate(
                        black_box(text),
                        black_box("Devanagari"),
                        black_box("IAST")
                    ).unwrap();
                    let _back = vidyut.transliterate(
                        black_box(&iast),
                        black_box("IAST"),
                        black_box("Devanagari")
                    ).unwrap();
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_performance,
    bench_throughput,
    bench_accuracy,
    bench_memory,
    bench_round_trip
);

criterion_main!(benches);