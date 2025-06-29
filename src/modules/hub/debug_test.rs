#[cfg(test)]
mod debug_test {
    use super::*;
    use crate::modules::hub::Hub;

    #[test]
    fn debug_mappings() {
        let hub = Hub::new();

        println!("Deva to ISO mappings: {}", hub.deva_to_iso_map.len());
        for (deva, iso) in &hub.deva_to_iso_map {
            println!("  {} → {}", deva, if iso.is_empty() { "∅" } else { iso });
        }

        println!("\nISO to Deva mappings: {}", hub.iso_to_deva_map.len());
        for (iso, deva) in &hub.iso_to_deva_map {
            println!("  {} → {}", iso, deva);
        }

        // Find unmapped entries
        println!("\nDeva→ISO without reverse:");
        for (&deva, &iso) in &hub.deva_to_iso_map {
            if !iso.is_empty() && !hub.iso_to_deva_map.contains_key(iso) {
                println!("  {} → {} (no reverse)", deva, iso);
            }
        }

        println!("\nISO→Deva without forward:");
        for (&iso, &deva) in &hub.iso_to_deva_map {
            if !hub.deva_to_iso_map.contains_key(&deva) {
                println!("  {} → {} (no forward)", iso, deva);
            }
        }
    }
}
