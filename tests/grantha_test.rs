#[cfg(test)]
mod grantha_tests {
    use shlesha::Shlesha;

    /// Regression test for retroflex/dental stop mapping in Grantha.
    ///
    /// The hub convention is that the single-letter consonant tokens
    /// (ConsonantT/Th/D/Dh/N) are RETROFLEX (ṭ ṭh ḍ ḍh ṇ) and the doubled
    /// tokens (ConsonantTt/Tth/Dd/Ddh/Nn) are DENTAL (t th d dh n), matching
    /// Devanagari and every other Indic schema. The Grantha schema previously
    /// had these two series swapped, so ṭa rendered as 𑌤 (GRANTHA LETTER TA,
    /// dental) and ta rendered as 𑌟 (GRANTHA LETTER TTA, retroflex).
    ///
    /// A pure round-trip cannot catch this because the swap is internally
    /// consistent, so this test pins the exact expected glyphs.
    #[test]
    fn test_grantha_retroflex_dental_stops() {
        let t = Shlesha::new();

        // Retroflex series -> Grantha retroflex letters (U+1131F..U+11323)
        assert_eq!(t.transliterate("ṭa", "iso15919", "grantha").unwrap(), "𑌟"); // TTA
        assert_eq!(t.transliterate("ṭha", "iso15919", "grantha").unwrap(), "𑌠"); // TTHA
        assert_eq!(t.transliterate("ḍa", "iso15919", "grantha").unwrap(), "𑌡"); // DDA
        assert_eq!(t.transliterate("ḍha", "iso15919", "grantha").unwrap(), "𑌢"); // DDHA
        assert_eq!(t.transliterate("ṇa", "iso15919", "grantha").unwrap(), "𑌣"); // NNA

        // Dental series -> Grantha dental letters (U+11324..U+11328)
        assert_eq!(t.transliterate("ta", "iso15919", "grantha").unwrap(), "𑌤"); // TA
        assert_eq!(t.transliterate("tha", "iso15919", "grantha").unwrap(), "𑌥"); // THA
        assert_eq!(t.transliterate("da", "iso15919", "grantha").unwrap(), "𑌦"); // DA
        assert_eq!(t.transliterate("dha", "iso15919", "grantha").unwrap(), "𑌧"); // DHA
        assert_eq!(t.transliterate("na", "iso15919", "grantha").unwrap(), "𑌨"); // NA
    }

    #[test]
    fn test_grantha_stops_roundtrip() {
        let t = Shlesha::new();
        let iso = "ṭaṭhaḍaḍhaṇatathadadhana";
        let grantha = t.transliterate(iso, "iso15919", "grantha").unwrap();
        let back = t.transliterate(&grantha, "grantha", "iso15919").unwrap();
        assert_eq!(back, iso);
    }
}
