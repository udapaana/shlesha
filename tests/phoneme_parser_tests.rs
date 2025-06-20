use shlesha::{PhonemeTransliterator, PhonemeTransliteratorBuilder, SchemaParser};

#[test]
fn test_phoneme_parser_basic_devanagari() {
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema = SchemaParser::parse_str(iast_schema).unwrap();
    
    let mut transliterator = PhonemeTransliteratorBuilder::new()
        .with_schema(dev_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build();
    
    // Test single word
    let result = transliterator.transliterate("नमस्ते", "Devanagari", "IAST").unwrap();
    assert_eq!(result, "namaste");
    
    // Check that we achieved high enum efficiency
    let stats = transliterator.get_parse_stats();
    assert!(stats.allocation_efficiency() >= 90.0, "Expected >90% efficiency, got {:.1}%", stats.allocation_efficiency());
}

#[test]
fn test_phoneme_parser_vowels_and_matras() {
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema = SchemaParser::parse_str(iast_schema).unwrap();
    
    let mut transliterator = PhonemeTransliteratorBuilder::new()
        .with_schema(dev_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build();
    
    // Test independent vowels
    let result = transliterator.transliterate("अ", "Devanagari", "IAST").unwrap();
    assert_eq!(result, "a");
    
    let result = transliterator.transliterate("आ", "Devanagari", "IAST").unwrap();
    assert_eq!(result, "ā");
    
    let result = transliterator.transliterate("इ", "Devanagari", "IAST").unwrap();
    assert_eq!(result, "i");
    
    // Test dependent vowels (matras)
    let result = transliterator.transliterate("का", "Devanagari", "IAST").unwrap();
    assert_eq!(result, "kā");
    
    let result = transliterator.transliterate("कि", "Devanagari", "IAST").unwrap();
    assert_eq!(result, "ki");
    
    let result = transliterator.transliterate("कु", "Devanagari", "IAST").unwrap();
    assert_eq!(result, "ku");
    
    let result = transliterator.transliterate("के", "Devanagari", "IAST").unwrap();
    assert_eq!(result, "ke");
    
    let result = transliterator.transliterate("को", "Devanagari", "IAST").unwrap();
    assert_eq!(result, "ko");
}

#[test]
fn test_phoneme_parser_conjuncts() {
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema = SchemaParser::parse_str(iast_schema).unwrap();
    
    let mut transliterator = PhonemeTransliteratorBuilder::new()
        .with_schema(dev_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build();
    
    // Test virama
    let result = transliterator.transliterate("क्", "Devanagari", "IAST").unwrap();
    assert_eq!(result, "k");
    
    // Test conjuncts
    let result = transliterator.transliterate("क्त", "Devanagari", "IAST").unwrap();
    assert_eq!(result, "kta");
    
    let result = transliterator.transliterate("स्त", "Devanagari", "IAST").unwrap();
    assert_eq!(result, "sta");
}

#[test]
fn test_phoneme_parser_modifiers() {
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema = SchemaParser::parse_str(iast_schema).unwrap();
    
    let mut transliterator = PhonemeTransliteratorBuilder::new()
        .with_schema(dev_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build();
    
    // Test anusvara
    let result = transliterator.transliterate("अं", "Devanagari", "IAST").unwrap();
    assert_eq!(result, "aṃ");
    
    // Test visarga
    let result = transliterator.transliterate("अः", "Devanagari", "IAST").unwrap();
    assert_eq!(result, "aḥ");
    
    // Test candrabindu
    let result = transliterator.transliterate("अँ", "Devanagari", "IAST").unwrap();
    assert_eq!(result, "am̐");
}

#[test]
fn test_phoneme_parser_complete_words() {
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema = SchemaParser::parse_str(iast_schema).unwrap();
    
    let mut transliterator = PhonemeTransliteratorBuilder::new()
        .with_schema(dev_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build();
    
    // Test complex Sanskrit words
    let test_cases = vec![
        ("संस्कृत", "saṃskṛta"),
        ("वदामि", "vadāmi"),
        ("भीमार्जुन", "bhīmārjuna"),
        ("युधि", "yudhi"),
        ("महेष्वास", "maheṣvāsa"),
    ];
    
    for (devanagari, expected_iast) in test_cases {
        let result = transliterator.transliterate(devanagari, "Devanagari", "IAST").unwrap();
        assert_eq!(result, expected_iast, "Failed for input: {}", devanagari);
    }
}

#[test]
fn test_phoneme_parser_performance_stats() {
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema = SchemaParser::parse_str(iast_schema).unwrap();
    
    let mut transliterator = PhonemeTransliteratorBuilder::new()
        .with_schema(dev_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build();
    
    // Reset stats
    transliterator.reset_stats();
    
    // Process text
    let text = "तत्र शूरा महेष्वासा भीमार्जुनसमा युधि";
    let _result = transliterator.transliterate(text, "Devanagari", "IAST").unwrap();
    
    // Check stats
    let stats = transliterator.get_parse_stats();
    assert!(stats.total_chars_processed > 0);
    assert!(stats.enum_phonemes_used > 0);
    assert!(stats.allocation_efficiency() > 75.0, "Expected >75% efficiency, got {:.1}%", stats.allocation_efficiency());
    assert!(stats.avg_parse_time_per_char_ns() > 0.0);
}

#[test]
fn test_phoneme_parser_all_consonants() {
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema = SchemaParser::parse_str(iast_schema).unwrap();
    
    let mut transliterator = PhonemeTransliteratorBuilder::new()
        .with_schema(dev_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build();
    
    // Test all basic consonants
    let consonant_tests = vec![
        // Velars
        ("क", "ka"), ("ख", "kha"), ("ग", "ga"), ("घ", "gha"), ("ङ", "ṅa"),
        // Palatals
        ("च", "ca"), ("छ", "cha"), ("ज", "ja"), ("झ", "jha"), ("ञ", "ña"),
        // Retroflexes
        ("ट", "ṭa"), ("ठ", "ṭha"), ("ड", "ḍa"), ("ढ", "ḍha"), ("ण", "ṇa"),
        // Dentals
        ("त", "ta"), ("थ", "tha"), ("द", "da"), ("ध", "dha"), ("न", "na"),
        // Labials
        ("प", "pa"), ("फ", "pha"), ("ब", "ba"), ("भ", "bha"), ("म", "ma"),
        // Others
        ("य", "ya"), ("र", "ra"), ("ल", "la"), ("व", "va"),
        ("श", "śa"), ("ष", "ṣa"), ("स", "sa"), ("ह", "ha"),
    ];
    
    for (devanagari, expected_iast) in consonant_tests {
        let result = transliterator.transliterate(devanagari, "Devanagari", "IAST").unwrap();
        assert_eq!(result, expected_iast, "Failed for consonant: {}", devanagari);
    }
}

#[test]
fn test_phoneme_parser_bidirectional() {
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema = SchemaParser::parse_str(iast_schema).unwrap();
    
    let mut transliterator = PhonemeTransliteratorBuilder::new()
        .with_schema(dev_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build();
    
    // Test Devanagari -> IAST
    let result1 = transliterator.transliterate("नमस्ते", "Devanagari", "IAST").unwrap();
    assert_eq!(result1, "namaste");
    
    // Test IAST -> Devanagari (this tests bidirectionality)
    let result2 = transliterator.transliterate("namaste", "IAST", "Devanagari").unwrap();
    assert_eq!(result2, "नमस्ते");
}

#[test]
fn test_phoneme_parser_error_handling() {
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema = SchemaParser::parse_str(iast_schema).unwrap();
    
    let mut transliterator = PhonemeTransliteratorBuilder::new()
        .with_schema(dev_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build();
    
    // Test unknown script
    let result = transliterator.transliterate("test", "UnknownScript", "IAST");
    assert!(result.is_err());
    
    // Test empty input
    let result = transliterator.transliterate("", "Devanagari", "IAST").unwrap();
    assert_eq!(result, "");
}

#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn test_phoneme_parser_vs_old_performance() {
        let devanagari_schema = include_str!("../schemas/devanagari.yaml");
        let iast_schema = include_str!("../schemas/iast.yaml");
        
        let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
        let iast_schema = SchemaParser::parse_str(iast_schema).unwrap();
        
        // Setup old transliterator
        let old_transliterator = shlesha::TransliteratorBuilder::new()
            .with_schema(dev_schema.clone()).unwrap()
            .with_schema(iast_schema.clone()).unwrap()
            .build();
        
        // Setup phoneme transliterator
        let mut phoneme_transliterator = PhonemeTransliteratorBuilder::new()
            .with_schema(dev_schema).unwrap()
            .with_schema(iast_schema).unwrap()
            .build();
        
        let text = "तत्र शूरा महेष्वासा भीमार्जुनसमा युधि";
        let iterations = 1000;
        
        // Benchmark old parser
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = old_transliterator.transliterate(text, "Devanagari", "IAST").unwrap();
        }
        let old_time = start.elapsed();
        
        // Benchmark phoneme parser
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = phoneme_transliterator.transliterate(text, "Devanagari", "IAST").unwrap();
        }
        let phoneme_time = start.elapsed();
        
        let speedup = old_time.as_nanos() as f64 / phoneme_time.as_nanos() as f64;
        println!("Old parser: {:?}", old_time);
        println!("Phoneme parser: {:?}", phoneme_time);
        println!("Speedup: {:.2}x", speedup);
        
        // Assert that phoneme parser is faster
        assert!(speedup > 1.5, "Expected at least 1.5x speedup, got {:.2}x", speedup);
    }
}