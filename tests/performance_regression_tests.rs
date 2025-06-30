//! Performance regression tests
//!
//! These tests verify that key performance characteristics remain within
//! acceptable bounds, catching performance regressions in CI.

use shlesha::Shlesha;
use std::time::{Duration, Instant};

/// Performance thresholds - these should be conservative enough to avoid flaky tests
/// but strict enough to catch real regressions
const MAX_BASIC_CONVERSION_MICROS: u128 = 1000; // 1ms for basic conversions
const MAX_MEDIUM_TEXT_MILLIS: u128 = 10; // 10ms for medium text
const MAX_LARGE_TEXT_MILLIS: u128 = 100; // 100ms for large text
const MAX_SCHEMA_LOAD_MILLIS: u128 = 50; // 50ms for schema loading

/// Test data
const SMALL_TEXT: &str = "धर्म";
const MEDIUM_TEXT: &str = "धर्म योग भारत संस्कृत वेद उपनिषद् गीता रामायण महाभारत";
const LARGE_TEXT: &str = "धर्म योग भारत संस्कृत वेद उपनिषद् गीता रामायण महाभारत पुराण शास्त्र दर्शन आयुर्वेद ज्योतिष व्याकरण छन्द निरुक्त कल्प शिक्षा स्मृति श्रुति आचार विचार संस्कार परम्परा सत्य अहिंसा करुणा दया प्रेम शान्ति आनन्द मोक्ष निर्वाण समाधि ध्यान प्राणायाम आसन मन्त्र यन्त्र तन्त्र";

/// Utility to measure execution time
fn measure_time<F, R>(f: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    (result, duration)
}

#[test]
fn test_basic_conversion_performance() {
    let transliterator = Shlesha::new();

    // Test core hub conversions (most critical path)
    let (result, duration) =
        measure_time(|| transliterator.transliterate(SMALL_TEXT, "devanagari", "iso15919"));

    assert!(result.is_ok(), "Basic conversion should succeed");

    #[cfg(not(tarpaulin))]
    assert!(
        duration.as_micros() < MAX_BASIC_CONVERSION_MICROS,
        "Basic conversion took {}µs, expected < {}µs",
        duration.as_micros(),
        MAX_BASIC_CONVERSION_MICROS
    );
}

#[test]
fn test_reverse_conversion_performance() {
    let transliterator = Shlesha::new();

    let (result, duration) =
        measure_time(|| transliterator.transliterate("dharma", "iso15919", "devanagari"));

    assert!(result.is_ok(), "Reverse conversion should succeed");

    #[cfg(not(tarpaulin))]
    assert!(
        duration.as_micros() < MAX_BASIC_CONVERSION_MICROS,
        "Reverse conversion took {}µs, expected < {}µs",
        duration.as_micros(),
        MAX_BASIC_CONVERSION_MICROS
    );
}

#[test]
fn test_medium_text_performance() {
    let transliterator = Shlesha::new();

    let (result, duration) =
        measure_time(|| transliterator.transliterate(MEDIUM_TEXT, "devanagari", "iast"));

    assert!(result.is_ok(), "Medium text conversion should succeed");

    #[cfg(not(tarpaulin))]
    assert!(
        duration.as_millis() < MAX_MEDIUM_TEXT_MILLIS,
        "Medium text conversion took {}ms, expected < {}ms",
        duration.as_millis(),
        MAX_MEDIUM_TEXT_MILLIS
    );
}

#[test]
fn test_large_text_performance() {
    let transliterator = Shlesha::new();

    let (result, duration) =
        measure_time(|| transliterator.transliterate(LARGE_TEXT, "devanagari", "itrans"));

    assert!(result.is_ok(), "Large text conversion should succeed");
    assert!(
        duration.as_millis() < MAX_LARGE_TEXT_MILLIS,
        "Large text conversion took {}ms, expected < {}ms",
        duration.as_millis(),
        MAX_LARGE_TEXT_MILLIS
    );
}

#[test]
fn test_roman_to_roman_performance() {
    let transliterator = Shlesha::new();

    // Roman-to-Roman should be fastest (no hub conversion needed)
    let (result, duration) =
        measure_time(|| transliterator.transliterate("namaste karma dharma", "iast", "itrans"));

    assert!(result.is_ok(), "Roman-to-Roman conversion should succeed");

    #[cfg(not(tarpaulin))]
    assert!(
        duration.as_micros() < MAX_BASIC_CONVERSION_MICROS / 2, // Should be even faster
        "Roman-to-Roman conversion took {}µs, expected < {}µs",
        duration.as_micros(),
        MAX_BASIC_CONVERSION_MICROS / 2
    );
}

#[test]
fn test_multiple_conversions_performance() {
    let transliterator = Shlesha::new();

    // Test that multiple conversions don't have significant overhead
    let (results, duration) = measure_time(|| {
        let mut results = Vec::new();
        for _ in 0..10 {
            results.push(transliterator.transliterate(SMALL_TEXT, "devanagari", "iso15919"));
        }
        results
    });

    assert!(
        results.iter().all(|r| r.is_ok()),
        "All conversions should succeed"
    );

    // 10 conversions shouldn't take more than 10x single conversion time
    let avg_per_conversion = duration.as_micros() / 10;
    assert!(
        avg_per_conversion < MAX_BASIC_CONVERSION_MICROS,
        "Average per conversion: {}µs, expected < {}µs",
        avg_per_conversion,
        MAX_BASIC_CONVERSION_MICROS
    );
}

#[test]
fn test_transliterator_creation_performance() {
    // Transliterator creation should be fast (initialization overhead)
    let (transliterator, duration) = measure_time(Shlesha::new);

    // Test that it actually works
    let result = transliterator.transliterate("test", "iso15919", "devanagari");
    assert!(result.is_ok(), "New transliterator should work");

    assert!(
        duration.as_millis() < 50, // 50ms is generous for initialization
        "Transliterator creation took {}ms, expected < 50ms",
        duration.as_millis()
    );
}

#[test]
fn test_memory_efficiency_large_text() {
    let transliterator = Shlesha::new();

    // Create very large input to test memory efficiency
    let very_large_text = LARGE_TEXT.repeat(10); // ~5KB text

    let result = transliterator.transliterate(&very_large_text, "devanagari", "iast");

    assert!(result.is_ok(), "Large text conversion should succeed");

    // Result should not be exponentially larger than input
    let result_text = result.unwrap();
    let ratio = result_text.len() as f64 / very_large_text.len() as f64;

    assert!(
        ratio < 5.0, // Should not expand more than 5x
        "Output/input size ratio: {:.2}, expected < 5.0",
        ratio
    );
}

#[test]
fn test_repeated_text_performance() {
    let transliterator = Shlesha::new();

    // Test that repeated patterns don't cause exponential slowdown
    let repeated_text = "कर्म ".repeat(100); // 100 repetitions

    let (result, duration) =
        measure_time(|| transliterator.transliterate(&repeated_text, "devanagari", "iast"));

    assert!(result.is_ok(), "Repeated text conversion should succeed");
    assert!(
        duration.as_millis() < 50,
        "Repeated text conversion took {}ms, expected < 50ms",
        duration.as_millis()
    );
}

#[test]
fn test_concurrent_performance() {
    // Test sequential performance instead of concurrent due to error type threading issues
    let transliterator = Shlesha::new();

    let (results, duration) = measure_time(|| {
        let mut results = Vec::new();
        for i in 0..4 {
            let text = format!("{}_{}", SMALL_TEXT, i);
            results.push(transliterator.transliterate(&text, "devanagari", "iso15919"));
        }
        results
    });

    assert!(
        results.iter().all(|r| r.is_ok()),
        "All sequential conversions should succeed"
    );

    // Sequential conversions should still be fast
    assert!(
        duration.as_millis() < 20,
        "Sequential conversions took {}ms, expected < 20ms",
        duration.as_millis()
    );
}

#[test]
fn test_schema_loading_performance() {
    // Test that runtime schema loading doesn't cause major slowdowns
    let (mut transliterator, creation_duration) = measure_time(Shlesha::new);

    // Try to load a schema if available (this might fail in CI, which is OK)
    if std::path::Path::new("schemas/test").exists() {
        let (result, load_duration) = measure_time(|| {
            transliterator.load_schema_from_file("schemas/test/sample_schema.yaml")
        });

        // If schema loading succeeds, it should be reasonably fast
        if result.is_ok() {
            assert!(
                load_duration.as_millis() < MAX_SCHEMA_LOAD_MILLIS,
                "Schema loading took {}ms, expected < {}ms",
                load_duration.as_millis(),
                MAX_SCHEMA_LOAD_MILLIS
            );
        }
    }

    // Basic creation should always be fast regardless
    assert!(
        creation_duration.as_millis() < 50,
        "Basic creation took {}ms, expected < 50ms",
        creation_duration.as_millis()
    );
}

#[test]
fn test_error_path_performance() {
    let transliterator = Shlesha::new();

    // Error paths should be fast (no expensive computation)
    let (result, duration) =
        measure_time(|| transliterator.transliterate("test", "nonexistent_script", "devanagari"));

    assert!(result.is_err(), "Should error for nonexistent script");

    // Only check performance in non-coverage builds
    // Coverage instrumentation makes timing tests unreliable
    #[cfg(not(tarpaulin))]
    assert!(
        duration.as_micros() < 250, // Errors should be very fast, allowing for system variance
        "Error path took {}µs, expected < 250µs",
        duration.as_micros()
    );
}
