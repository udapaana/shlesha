#[cfg(test)]
mod dharma_tests {

    use crate::modules::hub::{Hub, HubOutput, HubTrait};

    #[test]
    fn test_dharma_transliteration() {
        let hub = Hub::new();

        // Test "धर्म" (dharma) - this should produce "dharma" not "dharama"
        let result = hub.deva_to_iso("धर्म").unwrap();
        if let HubOutput::Iso(iso) = result {
            assert_eq!(iso, "dharma", "धर्म should transliterate to 'dharma'");
        } else {
            panic!("Expected ISO output");
        }
    }

    #[test]
    fn test_virama_handling() {
        let hub = Hub::new();

        // Test consonant + virama removes inherent 'a'
        let result = hub.deva_to_iso("क्").unwrap();
        if let HubOutput::Iso(iso) = result {
            assert_eq!(iso, "k", "क् should transliterate to 'k' (no inherent a)");
        } else {
            panic!("Expected ISO output");
        }

        // Test consonant without virama keeps inherent 'a'
        let result = hub.deva_to_iso("क").unwrap();
        if let HubOutput::Iso(iso) = result {
            assert_eq!(
                iso, "ka",
                "क should transliterate to 'ka' (with inherent a)"
            );
        } else {
            panic!("Expected ISO output");
        }
    }

    #[test]
    fn test_consonant_clusters() {
        let hub = Hub::new();

        // Test "कर्म" (karma)
        let result = hub.deva_to_iso("कर्म").unwrap();
        if let HubOutput::Iso(iso) = result {
            assert_eq!(iso, "karma");
        } else {
            panic!("Expected ISO output");
        }
    }
}
