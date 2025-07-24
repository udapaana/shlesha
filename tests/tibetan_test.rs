#[cfg(test)]
mod tibetan_tests {
    use shlesha::Shlesha;

    #[test]
    fn test_tibetan_basic() {
        let transliterator = Shlesha::new();

        // Test basic conversion from Devanagari to Tibetan
        let result = transliterator
            .transliterate("नमः", "devanagari", "tibetan")
            .unwrap();
        assert_eq!(result, "ནམཿ");

        // Test with Vedic accent
        let result = transliterator
            .transliterate("नमः॑", "devanagari", "tibetan")
            .unwrap();
        assert_eq!(result, "ནམཿ॑");
    }

    #[test]
    fn test_tibetan_vowels() {
        let transliterator = Shlesha::new();

        // Test independent vowels
        let result = transliterator
            .transliterate("अ आ इ ई उ ऊ", "devanagari", "tibetan")
            .unwrap();
        assert_eq!(result, "ཨ ཨཱ ཨི ཨཱི ཨུ ཨཱུ");
    }

    #[test]
    fn test_tibetan_aspirated() {
        let transliterator = Shlesha::new();

        // Test aspirated consonants
        let result = transliterator
            .transliterate("घ झ ढ ध भ", "devanagari", "tibetan")
            .unwrap();
        assert_eq!(result, "གྷ ཛྷ ཌྷ དྷ བྷ");
    }

    #[test]
    fn test_tibetan_roundtrip() {
        let transliterator = Shlesha::new();
        let test_text = "धर्मः";

        // Devanagari -> Tibetan -> Devanagari
        let tibetan = transliterator
            .transliterate(test_text, "devanagari", "tibetan")
            .unwrap();
        let back = transliterator
            .transliterate(&tibetan, "tibetan", "devanagari")
            .unwrap();
        assert_eq!(back, test_text);
    }

    #[test]
    fn test_tibetan_mantra() {
        let transliterator = Shlesha::new();

        // Test Om Mani Padme Hum
        let result = transliterator
            .transliterate("ॐ मणि पद्मे हूं", "devanagari", "tibetan")
            .unwrap();
        // Note: ॐ might not convert directly, but the rest should
        assert!(result.contains("མཎི"));
        assert!(result.contains("པད྄མེ")); // Virama between द and म
        assert!(result.contains("ཧཱུཾ"));
    }
}
