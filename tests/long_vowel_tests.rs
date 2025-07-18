#[cfg(test)]
mod long_vowel_tests {
    use shlesha::Shlesha;

    #[test]
    fn test_telugu_short_long_e_distinction() {
        let transliterator = Shlesha::new();

        // Test short e
        let result = transliterator
            .transliterate("ఎ", "telugu", "iso15919")
            .unwrap();
        assert_eq!(result, "e");

        // Test long e
        let result = transliterator
            .transliterate("ఏ", "telugu", "iso15919")
            .unwrap();
        assert_eq!(result, "ē");

        // Test short o
        let result = transliterator
            .transliterate("ఒ", "telugu", "iso15919")
            .unwrap();
        assert_eq!(result, "o");

        // Test long o
        let result = transliterator
            .transliterate("ఓ", "telugu", "iso15919")
            .unwrap();
        assert_eq!(result, "ō");
    }

    #[test]
    fn test_iso_to_telugu_long_vowels() {
        let transliterator = Shlesha::new();

        // Test the original failing case
        let result = transliterator
            .transliterate("ukō", "iso15919", "telugu")
            .unwrap();
        assert_eq!(result, "ఉకో"); // Should use long o vowel sign

        // Test more cases
        let result = transliterator
            .transliterate("ēkam", "iso15919", "telugu")
            .unwrap();
        assert_eq!(result, "ఏకమ్"); // Should use long e

        let result = transliterator
            .transliterate("kōṭi", "iso15919", "telugu")
            .unwrap();
        assert_eq!(result, "కోటి"); // Should use long o vowel sign
    }

    #[test]
    fn test_iso_to_deva_long_vowels() {
        let transliterator = Shlesha::new();

        // Devanagari has been updated to distinguish short/long e/o
        let result = transliterator
            .transliterate("e", "iso15919", "devanagari")
            .unwrap();
        assert_eq!(result, "ऎ"); // Short e

        let result = transliterator
            .transliterate("ē", "iso15919", "devanagari")
            .unwrap();
        assert_eq!(result, "ए"); // Long e

        let result = transliterator
            .transliterate("o", "iso15919", "devanagari")
            .unwrap();
        assert_eq!(result, "ऒ"); // Short o

        let result = transliterator
            .transliterate("ō", "iso15919", "devanagari")
            .unwrap();
        assert_eq!(result, "ओ"); // Long o
    }

    #[test]
    fn test_round_trip_with_long_vowels() {
        let transliterator = Shlesha::new();

        // Telugu text with long vowels
        let telugu_text = "ఏకమేవ అద్వితీయమ్";

        // Telugu -> ISO -> Telugu should preserve distinctions
        let iso = transliterator
            .transliterate(telugu_text, "telugu", "iso15919")
            .unwrap();
        let back_to_telugu = transliterator
            .transliterate(&iso, "iso15919", "telugu")
            .unwrap();
        assert_eq!(back_to_telugu, telugu_text);

        // ISO text with long vowels
        let iso_text = "ēkaṁ kōṭi";

        // ISO -> Telugu -> ISO should preserve distinctions
        let telugu = transliterator
            .transliterate(iso_text, "iso15919", "telugu")
            .unwrap();
        let back_to_iso = transliterator
            .transliterate(&telugu, "telugu", "iso15919")
            .unwrap();
        assert_eq!(back_to_iso, iso_text);
    }

    #[test]
    fn test_dependent_vowel_signs() {
        let transliterator = Shlesha::new();

        // Test short e vowel sign
        let result = transliterator
            .transliterate("కె", "telugu", "iso15919")
            .unwrap();
        assert_eq!(result, "ke");

        // Test long e vowel sign
        let result = transliterator
            .transliterate("కే", "telugu", "iso15919")
            .unwrap();
        assert_eq!(result, "kē");

        // Test short o vowel sign
        let result = transliterator
            .transliterate("కొ", "telugu", "iso15919")
            .unwrap();
        assert_eq!(result, "ko");

        // Test long o vowel sign
        let result = transliterator
            .transliterate("కో", "telugu", "iso15919")
            .unwrap();
        assert_eq!(result, "kō");
    }
}
