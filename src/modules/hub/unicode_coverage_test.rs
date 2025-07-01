#[cfg(test)]
mod unicode_coverage_test {

    use crate::modules::hub::Hub;

    #[test]
    fn verify_devanagari_unicode_coverage() {
        let hub = Hub::new();

        println!("\n=== Devanagari Unicode Coverage Analysis ===\n");

        // Unicode Devanagari block: U+0900 to U+097F

        // Various signs (U+0900–U+0903)
        let various_signs = vec![
            ('\u{0900}', "invocational sign"), // ऀ
            ('\u{0901}', "candrabindu"),       // ँ
            ('\u{0902}', "anusvara"),          // ं
            ('\u{0903}', "visarga"),           // ः
        ];

        // Independent vowels (U+0904–U+0914)
        let independent_vowels = vec![
            ('\u{0904}', "short A"),   // ऄ
            ('\u{0905}', "A"),         // अ
            ('\u{0906}', "AA"),        // आ
            ('\u{0907}', "I"),         // इ
            ('\u{0908}', "II"),        // ई
            ('\u{0909}', "U"),         // उ
            ('\u{090A}', "UU"),        // ऊ
            ('\u{090B}', "vocalic R"), // ऋ
            ('\u{090C}', "vocalic L"), // ऌ
            ('\u{090D}', "candra E"),  // ऍ
            ('\u{090E}', "short E"),   // ऎ
            ('\u{090F}', "E"),         // ए
            ('\u{0910}', "AI"),        // ऐ
            ('\u{0911}', "candra O"),  // ऑ
            ('\u{0912}', "short O"),   // ऒ
            ('\u{0913}', "O"),         // ओ
            ('\u{0914}', "AU"),        // औ
        ];

        // Consonants (U+0915–U+0939)
        let consonants = vec![
            ('\u{0915}', "KA"),   // क
            ('\u{0916}', "KHA"),  // ख
            ('\u{0917}', "GA"),   // ग
            ('\u{0918}', "GHA"),  // घ
            ('\u{0919}', "NGA"),  // ङ
            ('\u{091A}', "CA"),   // च
            ('\u{091B}', "CHA"),  // छ
            ('\u{091C}', "JA"),   // ज
            ('\u{091D}', "JHA"),  // झ
            ('\u{091E}', "NYA"),  // ञ
            ('\u{091F}', "TTA"),  // ट
            ('\u{0920}', "TTHA"), // ठ
            ('\u{0921}', "DDA"),  // ड
            ('\u{0922}', "DDHA"), // ढ
            ('\u{0923}', "NNA"),  // ण
            ('\u{0924}', "TA"),   // त
            ('\u{0925}', "THA"),  // थ
            ('\u{0926}', "DA"),   // द
            ('\u{0927}', "DHA"),  // ध
            ('\u{0928}', "NA"),   // न
            ('\u{0929}', "NNNA"), // ऩ (Nukta form)
            ('\u{092A}', "PA"),   // प
            ('\u{092B}', "PHA"),  // फ
            ('\u{092C}', "BA"),   // ब
            ('\u{092D}', "BHA"),  // भ
            ('\u{092E}', "MA"),   // म
            ('\u{092F}', "YA"),   // य
            ('\u{0930}', "RA"),   // र
            ('\u{0931}', "RRA"),  // ऱ (Nukta form)
            ('\u{0932}', "LA"),   // ल
            ('\u{0933}', "LLA"),  // ळ
            ('\u{0934}', "LLLA"), // ऴ (Nukta form)
            ('\u{0935}', "VA"),   // व
            ('\u{0936}', "SHA"),  // श
            ('\u{0937}', "SSA"),  // ष
            ('\u{0938}', "SA"),   // स
            ('\u{0939}', "HA"),   // ह
        ];

        // Various signs (U+093A–U+093C)
        let various_signs_2 = vec![
            ('\u{093A}', "oe sign"),  // ऺ
            ('\u{093B}', "ooe sign"), // ऻ
            ('\u{093C}', "nukta"),    // ़
        ];

        // Dependent vowel signs (U+093D–U+094D)
        let dependent_vowel_signs = vec![
            ('\u{093D}', "avagraha"),        // ऽ
            ('\u{093E}', "AA sign"),         // ा
            ('\u{093F}', "I sign"),          // ि
            ('\u{0940}', "II sign"),         // ी
            ('\u{0941}', "U sign"),          // ु
            ('\u{0942}', "UU sign"),         // ू
            ('\u{0943}', "vocalic R sign"),  // ृ
            ('\u{0944}', "vocalic RR sign"), // ॄ
            ('\u{0945}', "candra E sign"),   // ॅ
            ('\u{0946}', "short E sign"),    // ॆ
            ('\u{0947}', "E sign"),          // े
            ('\u{0948}', "AI sign"),         // ै
            ('\u{0949}', "candra O sign"),   // ॉ
            ('\u{094A}', "short O sign"),    // ॊ
            ('\u{094B}', "O sign"),          // ो
            ('\u{094C}', "AU sign"),         // ौ
            ('\u{094D}', "virama"),          // ्
        ];

        // Additional signs (U+094E–U+0957)
        let additional_signs = vec![
            ('\u{094E}', "prishthamatra E"),      // ॎ
            ('\u{094F}', "AW sign"),              // ॏ
            ('\u{0950}', "om"),                   // ॐ
            ('\u{0951}', "stress mark udatta"),   // ॑
            ('\u{0952}', "stress mark anudatta"), // ॒
            ('\u{0953}', "grave accent"),         // ॓
            ('\u{0954}', "acute accent"),         // ॔
            ('\u{0955}', "vowel length mark"),    // ॕ
            ('\u{0956}', "AI length mark"),       // ॖ
            ('\u{0957}', "AU length mark"),       // ॗ
        ];

        // Additional consonants (U+0958–U+095F)
        let additional_consonants = vec![
            ('\u{0958}', "QA"),    // क़
            ('\u{0959}', "KHHA"),  // ख़
            ('\u{095A}', "GHHA"),  // ग़
            ('\u{095B}', "ZA"),    // ज़
            ('\u{095C}', "DDDHA"), // ड़
            ('\u{095D}', "RHA"),   // ढ़
            ('\u{095E}', "FA"),    // फ़
            ('\u{095F}', "YYA"),   // य़
        ];

        // Additional vowels and signs (U+0960–U+0977)
        let additional_vowels = vec![
            ('\u{0960}', "vocalic RR"),        // ॠ
            ('\u{0961}', "vocalic LL"),        // ॡ
            ('\u{0962}', "vocalic L sign"),    // ॢ
            ('\u{0963}', "vocalic LL sign"),   // ॣ
            ('\u{0964}', "danda"),             // ।
            ('\u{0965}', "double danda"),      // ॥
            ('\u{0966}', "digit zero"),        // ०
            ('\u{0967}', "digit one"),         // १
            ('\u{0968}', "digit two"),         // २
            ('\u{0969}', "digit three"),       // ३
            ('\u{096A}', "digit four"),        // ४
            ('\u{096B}', "digit five"),        // ५
            ('\u{096C}', "digit six"),         // ६
            ('\u{096D}', "digit seven"),       // ७
            ('\u{096E}', "digit eight"),       // ८
            ('\u{096F}', "digit nine"),        // ९
            ('\u{0970}', "abbreviation sign"), // ॰
            ('\u{0971}', "high spacing dot"),  // ॱ
            ('\u{0972}', "candra A"),          // ॲ
            ('\u{0973}', "OE"),                // ॳ
            ('\u{0974}', "OOE"),               // ॴ
            ('\u{0975}', "AW"),                // ॵ
            ('\u{0976}', "UE"),                // ॶ
            ('\u{0977}', "UUE"),               // ॷ
        ];

        // Check coverage for each category
        println!("Various Signs (U+0900–U+0903):");
        check_coverage(&hub, &various_signs);

        println!("\nIndependent Vowels (U+0904–U+0914):");
        check_coverage(&hub, &independent_vowels);

        println!("\nConsonants (U+0915–U+0939):");
        check_coverage(&hub, &consonants);

        println!("\nVarious Signs 2 (U+093A–U+093C):");
        check_coverage(&hub, &various_signs_2);

        println!("\nDependent Vowel Signs (U+093D–U+094D):");
        check_coverage(&hub, &dependent_vowel_signs);

        println!("\nAdditional Signs (U+094E–U+0957):");
        check_coverage(&hub, &additional_signs);

        println!("\nAdditional Consonants (U+0958–U+095F):");
        check_coverage(&hub, &additional_consonants);

        println!("\nAdditional Vowels and Signs (U+0960–U+0977):");
        check_coverage(&hub, &additional_vowels);

        // Summary
        let mut total_chars = 0;
        let mut mapped_chars = 0;

        for chars in [
            &various_signs,
            &independent_vowels,
            &consonants,
            &various_signs_2,
            &dependent_vowel_signs,
            &additional_signs,
            &additional_consonants,
            &additional_vowels,
        ] {
            for (ch, _) in chars {
                total_chars += 1;
                if hub.deva_to_iso_map.contains_key(ch) {
                    mapped_chars += 1;
                }
            }
        }

        println!("\n=== SUMMARY ===");
        println!("Total Devanagari characters in Unicode: {}", total_chars);
        println!("Characters mapped in hub: {}", mapped_chars);
        println!(
            "Coverage: {:.1}%",
            (mapped_chars as f64 / total_chars as f64) * 100.0
        );

        // List essential missing characters
        println!("\n=== Essential Missing Characters ===");
        let essential_missing = vec![
            ('\u{0901}', "candrabindu (ँ)", "m̐"),
            ('\u{090C}', "vocalic L (ऌ)", "l̥"),
            ('\u{0910}', "AI (ऐ)", "ai"),
            ('\u{093C}', "nukta (़)", "़"),
            ('\u{093D}', "avagraha (ऽ)", "'"),
            ('\u{0944}', "vocalic RR sign (ॄ)", "r̥̄"),
            ('\u{0948}', "AI sign (ै)", "ai"),
            ('\u{0950}', "om (ॐ)", "oṁ"),
            ('\u{0960}', "vocalic RR (ॠ)", "r̥̄"),
            ('\u{0961}', "vocalic LL (ॡ)", "l̥̄"),
            ('\u{0962}', "vocalic L sign (ॢ)", "l̥"),
            ('\u{0963}', "vocalic LL sign (ॣ)", "l̥̄"),
            ('\u{0965}', "double danda (॥)", "||"),
        ];

        for (ch, name, iso) in essential_missing {
            if !hub.deva_to_iso_map.contains_key(&ch) {
                println!("  {} {} → {}", ch, name, iso);
            }
        }
    }

    fn check_coverage(hub: &Hub, chars: &[(char, &str)]) {
        let mut mapped = 0;
        let mut unmapped = Vec::new();

        for (ch, name) in chars {
            if hub.deva_to_iso_map.contains_key(ch) {
                mapped += 1;
            } else {
                unmapped.push((*ch, *name));
            }
        }

        println!("  Mapped: {}/{}", mapped, chars.len());
        if !unmapped.is_empty() {
            println!("  Missing:");
            for (ch, name) in unmapped {
                println!("    {} U+{:04X} {}", ch, ch as u32, name);
            }
        }
    }
}
