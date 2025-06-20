use shlesha::{PhonemeTransliteratorBuilder, SchemaParser};

fn main() {
    let devanagari_schema = include_str!("../schemas/devanagari.yaml");
    let iast_schema = include_str!("../schemas/iast.yaml");
    
    let dev_schema = SchemaParser::parse_str(devanagari_schema).unwrap();
    let iast_schema = SchemaParser::parse_str(iast_schema).unwrap();
    
    let mut transliterator = PhonemeTransliteratorBuilder::new()
        .with_schema(dev_schema).unwrap()
        .with_schema(iast_schema).unwrap()
        .build();
    
    // Test simple transliterations
    let tests = vec![
        "न",
        "क",
        "ख", 
        "अ",
        "आ",
        "नमस्ते",
        "का",
        "कि",
        "अं",
    ];
    
    for input in tests {
        let result = transliterator.transliterate(input, "Devanagari", "IAST").unwrap();
        println!("'{}' -> '{}'", input, result);
    }
}