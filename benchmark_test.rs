use shlesha::Shlesha;
use std::time::Instant;

fn main() {
    let transliterator = Shlesha::new();
    
    // Test data of varying sizes
    let small_text = "धर्म";
    let medium_text = "धर्म योग भारत संस्कृत वेद उपनिषद् गीता रामायण महाभारत";
    let large_text = "धर्म योग भारत संस्कृत वेद उपनिषद् गीता रामायण महाभारत पुराण शास्त्र दर्शन आयुर्वेद ज्योतिष व्याकरण छन्द निरुक्त कल्प शिक्षा स्मृति श्रुति आचार विचार संस्कार परम्परा सत्य अहिंसा करुणा दया प्रेम शान्ति आनन्द मोक्ष निर्वाण समाधि ध्यान प्राणायाम आसन मन्त्र यन्त्र तन्त्र";
    
    println!("Shlesha Performance Benchmark");
    println!("============================");
    
    // Small text benchmark
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = transliterator.transliterate(small_text, "devanagari", "iso").unwrap();
    }
    let small_duration = start.elapsed();
    println!("Small text (1000 iterations): {:?} ({:.2}μs per operation)", 
             small_duration, small_duration.as_nanos() as f64 / 1000.0 / 1000.0);
    
    // Medium text benchmark
    let start = Instant::now();
    for _ in 0..100 {
        let _ = transliterator.transliterate(medium_text, "devanagari", "iast").unwrap();
    }
    let medium_duration = start.elapsed();
    println!("Medium text (100 iterations): {:?} ({:.2}μs per operation)", 
             medium_duration, medium_duration.as_nanos() as f64 / 100.0 / 1000.0);
    
    // Large text benchmark
    let start = Instant::now();
    for _ in 0..10 {
        let _ = transliterator.transliterate(large_text, "devanagari", "itrans").unwrap();
    }
    let large_duration = start.elapsed();
    println!("Large text (10 iterations): {:?} ({:.2}ms per operation)", 
             large_duration, large_duration.as_millis() as f64 / 10.0);
    
    // Cross-script conversion benchmark
    let start = Instant::now();
    for _ in 0..100 {
        let _ = transliterator.transliterate(medium_text, "devanagari", "gujarati").unwrap();
    }
    let cross_duration = start.elapsed();
    println!("Cross-script (100 iterations): {:?} ({:.2}μs per operation)", 
             cross_duration, cross_duration.as_nanos() as f64 / 100.0 / 1000.0);
    
    // Character count analysis
    println!("\nText Analysis:");
    println!("Small text: {} chars, {} bytes", small_text.chars().count(), small_text.len());
    println!("Medium text: {} chars, {} bytes", medium_text.chars().count(), medium_text.len());
    println!("Large text: {} chars, {} bytes", large_text.chars().count(), large_text.len());
    
    // Throughput analysis
    let chars_per_sec = (medium_text.chars().count() * 100) as f64 / medium_duration.as_secs_f64();
    println!("\nThroughput: {:.0} characters/second", chars_per_sec);
}