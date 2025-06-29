use shlesha::Shlesha;
use std::time::Instant;

fn main() {
    let transliterator = Shlesha::new();

    // Test data
    let iast_text = "dharma yoga bhārata saṃskṛta veda upaniṣad gītā rāmāyaṇa mahābhārata";
    let deva_text = "धर्म योग भारत संस्कृत वेद उपनिषद् गीता रामायण महाभारत";

    println!("=== Profiling Shlesha Performance ===\n");

    // Test Roman → Devanagari (where Shlesha is slower)
    println!("Roman → Devanagari conversions (Shlesha weakness):");

    let start = Instant::now();
    for _ in 0..1000 {
        let _ = transliterator
            .transliterate(iast_text, "iast", "devanagari")
            .unwrap();
    }
    let elapsed = start.elapsed();
    println!(
        "IAST → Devanagari: {} chars/sec",
        (iast_text.len() * 1000) as f64 / elapsed.as_secs_f64()
    );

    let start = Instant::now();
    for _ in 0..1000 {
        let _ = transliterator
            .transliterate(iast_text, "iast", "tamil")
            .unwrap();
    }
    let elapsed = start.elapsed();
    println!(
        "IAST → Tamil: {} chars/sec",
        (iast_text.len() * 1000) as f64 / elapsed.as_secs_f64()
    );

    // Test Roman → Roman (where Shlesha is also slower)
    println!("\nRoman → Roman conversions (Shlesha weakness):");

    let start = Instant::now();
    for _ in 0..1000 {
        let _ = transliterator
            .transliterate(iast_text, "iast", "itrans")
            .unwrap();
    }
    let elapsed = start.elapsed();
    println!(
        "IAST → ITRANS: {} chars/sec",
        (iast_text.len() * 1000) as f64 / elapsed.as_secs_f64()
    );

    let start = Instant::now();
    for _ in 0..1000 {
        let _ = transliterator
            .transliterate(iast_text, "iast", "slp1")
            .unwrap();
    }
    let elapsed = start.elapsed();
    println!(
        "IAST → SLP1: {} chars/sec",
        (iast_text.len() * 1000) as f64 / elapsed.as_secs_f64()
    );

    // Test Devanagari → Indic (where Shlesha excels)
    println!("\nDevanagari → Indic conversions (Shlesha strength):");

    let start = Instant::now();
    for _ in 0..1000 {
        let _ = transliterator
            .transliterate(deva_text, "devanagari", "tamil")
            .unwrap();
    }
    let elapsed = start.elapsed();
    println!(
        "Devanagari → Tamil: {} chars/sec",
        (deva_text.len() * 1000) as f64 / elapsed.as_secs_f64()
    );

    let start = Instant::now();
    for _ in 0..1000 {
        let _ = transliterator
            .transliterate(deva_text, "devanagari", "telugu")
            .unwrap();
    }
    let elapsed = start.elapsed();
    println!(
        "Devanagari → Telugu: {} chars/sec",
        (deva_text.len() * 1000) as f64 / elapsed.as_secs_f64()
    );

    // Test Devanagari → Roman (where Vidyut is faster)
    println!("\nDevanagari → Roman conversions:");

    let start = Instant::now();
    for _ in 0..1000 {
        let _ = transliterator
            .transliterate(deva_text, "devanagari", "iast")
            .unwrap();
    }
    let elapsed = start.elapsed();
    println!(
        "Devanagari → IAST: {} chars/sec",
        (deva_text.len() * 1000) as f64 / elapsed.as_secs_f64()
    );

    let start = Instant::now();
    for _ in 0..1000 {
        let _ = transliterator
            .transliterate(deva_text, "devanagari", "itrans")
            .unwrap();
    }
    let elapsed = start.elapsed();
    println!(
        "Devanagari → ITRANS: {} chars/sec",
        (deva_text.len() * 1000) as f64 / elapsed.as_secs_f64()
    );
}
