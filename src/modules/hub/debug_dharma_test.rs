#[cfg(test)]
mod debug_dharma_test {

    use crate::modules::hub::{Hub, HubOutput, HubTrait};

    #[test]
    fn debug_dharma_conversion() {
        let hub = Hub::new();

        println!("Testing 'dharma' conversion:");

        // First check what mappings we have
        println!("Checking individual mappings:");
        for seq in [
            "d", "h", "a", "r", "m", "dh", "ha", "ar", "ma", "dha", "har", "arm", "rma", "dharm",
            "harma", "dharma",
        ] {
            if hub.iso_to_deva_map.contains_key(seq) {
                println!("  '{}' → '{}'", seq, hub.iso_to_deva_map[seq]);
            } else {
                println!("  '{}' → NOT FOUND", seq);
            }
        }

        // Try the conversion
        match hub.iso_to_deva("dharma") {
            Ok(HubOutput::Devanagari(result)) => {
                println!("\nConversion succeeded: 'dharma' → '{}'", result);
            }
            Err(e) => {
                println!("\nConversion failed: {:?}", e);
            }
            _ => panic!("Unexpected output type"),
        }
    }
}
