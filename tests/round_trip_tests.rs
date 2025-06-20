use shlesha::{PhonemeTransliteratorBuilder, TransliteratorBuilder, SchemaParser};

#[test]
fn test_round_trip_accuracy() {
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema = SchemaParser::parse_str(iast_schema).unwrap();
    
    // Test with regular transliterator
    let transliterator = TransliteratorBuilder::new()
        .with_schema(dev_schema.clone()).unwrap()
        .with_schema(iast_schema.clone()).unwrap()
        .build();
    
    // Test cases for round-trip
    let test_cases = vec![
        // Basic consonants
        "क", "ख", "ग", "घ", "ङ",
        "च", "छ", "ज", "झ", "ञ",
        "ट", "ठ", "ड", "ढ", "ण",
        "त", "थ", "द", "ध", "न",
        "प", "फ", "ब", "भ", "म",
        "य", "र", "ल", "व",
        "श", "ष", "स", "ह",
        
        // Vowels
        "अ", "आ", "इ", "ई", "उ", "ऊ",
        "ऋ", "ॠ", "ऌ", "ॡ",
        "ए", "ऐ", "ओ", "औ",
        
        // Consonants with vowels
        "का", "कि", "की", "कु", "कू",
        "कृ", "कॄ", "कॢ", "कॣ",
        "के", "कै", "को", "कौ",
        
        // Modifiers
        "कं", "कः", "कँ",
        
        // Virama (halant)
        "क्", "त्", "न्",
        
        // Common conjuncts
        "क्ष", "ज्ञ", "त्र", "श्र",
        "क्त", "क्व", "स्त", "स्व",
        
        // Complex words
        "नमस्ते", "संस्कृत", "धर्म", "कर्म",
        "महाभारत", "रामायण", "कृष्ण", "अर्जुन",
        
        // Sentences
        "अहं संस्कृतं वदामि",
        "तत्त्वमसि",
        "सत्यमेव जयते",
    ];
    
    let mut failures = Vec::new();
    
    for original in test_cases {
        // Forward: Devanagari → IAST
        let iast = transliterator.transliterate(original, "Devanagari", "IAST").unwrap();
        
        // Backward: IAST → Devanagari
        let back = transliterator.transliterate(&iast, "IAST", "Devanagari").unwrap();
        
        if original != back {
            failures.push((original, iast.clone(), back.clone()));
            println!("Round-trip failure: {} → {} → {} (expected {})", 
                     original, iast, back, original);
        }
    }
    
    assert!(failures.is_empty(), 
            "Round-trip failures: {:?}", failures);
}

#[test]
fn test_round_trip_with_phoneme_parser() {
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema = SchemaParser::parse_str(iast_schema).unwrap();
    
    // Test with phoneme transliterator
    let mut transliterator = PhonemeTransliteratorBuilder::new()
        .with_schema(dev_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build();
    
    // Same test cases
    let test_cases = vec![
        "नमस्ते", "संस्कृत", "धर्म", "कर्म",
        "कृष्ण", "अर्जुन", "महाभारत",
    ];
    
    let mut failures = Vec::new();
    
    for original in test_cases {
        let iast = transliterator.transliterate(original, "Devanagari", "IAST").unwrap();
        let back = transliterator.transliterate(&iast, "IAST", "Devanagari").unwrap();
        
        if original != back {
            failures.push((original, iast.clone(), back.clone()));
        }
    }
    
    assert!(failures.is_empty(), 
            "Phoneme parser round-trip failures: {:?}", failures);
}

#[test]
fn test_expected_iast_values() {
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema = SchemaParser::parse_str(iast_schema).unwrap();
    
    let transliterator = TransliteratorBuilder::new()
        .with_schema(dev_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build();
    
    // Verify all expected IAST values
    let test_cases = vec![
        // Basic consonants
        ("क", "ka"), ("ख", "kha"), ("ग", "ga"), ("घ", "gha"), ("ङ", "ṅa"),
        ("च", "ca"), ("छ", "cha"), ("ज", "ja"), ("झ", "jha"), ("ञ", "ña"),
        ("ट", "ṭa"), ("ठ", "ṭha"), ("ड", "ḍa"), ("ढ", "ḍha"), ("ण", "ṇa"),
        ("त", "ta"), ("थ", "tha"), ("द", "da"), ("ध", "dha"), ("न", "na"),
        ("प", "pa"), ("फ", "pha"), ("ब", "ba"), ("भ", "bha"), ("म", "ma"),
        ("य", "ya"), ("र", "ra"), ("ल", "la"), ("व", "va"),
        ("श", "śa"), ("ष", "ṣa"), ("स", "sa"), ("ह", "ha"),
        
        // Vowels
        ("अ", "a"), ("आ", "ā"), ("इ", "i"), ("ई", "ī"), 
        ("उ", "u"), ("ऊ", "ū"), ("ऋ", "ṛ"), ("ॠ", "ṝ"),
        ("ऌ", "ḷ"), ("ॡ", "ḹ"), ("ए", "e"), ("ऐ", "ai"),
        ("ओ", "o"), ("औ", "au"),
        
        // Consonants with vowels
        ("का", "kā"), ("कि", "ki"), ("की", "kī"), ("कु", "ku"), ("कू", "kū"),
        ("कृ", "kṛ"), ("कॄ", "kṝ"), ("के", "ke"), ("कै", "kai"),
        ("को", "ko"), ("कौ", "kau"),
        
        // Modifiers
        ("अं", "aṃ"), ("अः", "aḥ"), ("अँ", "am̐"),
        
        // Virama
        ("क्", "k"), ("त्", "t"), ("न्", "n"),
        
        // Conjuncts
        ("क्ष", "kṣa"), ("ज्ञ", "jña"), ("त्र", "tra"), ("श्र", "śra"),
        ("क्त", "kta"), ("स्त", "sta"), ("स्व", "sva"),
        
        // Words
        ("नमस्ते", "namaste"), ("संस्कृत", "saṃskṛta"),
        ("धर्म", "dharma"), ("कर्म", "karma"),
        ("कृष्ण", "kṛṣṇa"), ("अर्जुन", "arjuna"),
        ("महाभारत", "mahābhārata"), ("रामायण", "rāmāyaṇa"),
        
        // Special cases
        ("पुत्र", "putra"), ("मित्र", "mitra"),
        ("अग्नि", "agni"), ("इन्द्र", "indra"),
        ("गङ्गा", "gaṅgā"), ("चन्द्र", "candra"),
    ];
    
    for (devanagari, expected_iast) in test_cases {
        let result = transliterator.transliterate(devanagari, "Devanagari", "IAST").unwrap();
        assert_eq!(result, expected_iast, 
                   "Failed for {}: got '{}', expected '{}'", 
                   devanagari, result, expected_iast);
    }
}