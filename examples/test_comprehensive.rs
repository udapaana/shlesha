use shlesha::Shlesha;

fn main() {
    let shlesha = Shlesha::new();

    println!("=== Comprehensive Script Matrix Test ===");

    let test_word = "namaste";
    let scripts = vec![
        "iast",
        "itrans",
        "slp1",
        "harvard_kyoto",
        "kolkata",
        "velthuis",
        "wx",
        "iso15919_tokens",
        "devanagari_tokens",
        "bengali",
        "tamil",
        "gujarati",
        "telugu",
    ];

    // Test each script conversion
    #[allow(clippy::never_loop)]
    for from_script in &scripts {
        for to_script in &scripts {
            if from_script != to_script {
                match shlesha.transliterate(test_word, from_script, to_script) {
                    Ok(result) => {
                        println!(
                            "{} → {}: {} → {}",
                            from_script, to_script, test_word, result
                        );
                    }
                    Err(e) => {
                        println!("{} → {}: ERROR - {}", from_script, to_script, e);
                    }
                }
            }
        }
        break; // Just test from IAST for now to see what's working
    }
}
