//! Auto-generated from iso_devanagari.toml
//! DO NOT EDIT MANUALLY

#![allow(dead_code)]

use std::collections::HashMap;
use once_cell::sync::Lazy;

/// ISO to Devanagari character mappings
pub static ISO_TO_DEVA: Lazy<HashMap<&'static str, char>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("0", '०');
    m.insert("1", '१');
    m.insert("2", '२');
    m.insert("3", '३');
    m.insert("4", '४');
    m.insert("5", '५');
    m.insert("6", '६');
    m.insert("7", '७');
    m.insert("8", '८');
    m.insert("9", '९');
    m.insert("a", 'अ');
    m.insert("ai", 'ऐ');
    m.insert("au", 'औ');
    m.insert("ba", 'ब');
    m.insert("bha", 'भ');
    m.insert("ca", 'च');
    m.insert("cha", 'छ');
    m.insert("da", 'द');
    m.insert("dha", 'ध');
    m.insert("e", 'ए');
    m.insert("ga", 'ग');
    m.insert("gha", 'घ');
    m.insert("ha", 'ह');
    m.insert("i", 'इ');
    m.insert("ja", 'ज');
    m.insert("jha", 'झ');
    m.insert("ka", 'क');
    m.insert("kha", 'ख');
    m.insert("la", 'ल');
    m.insert("l\u{325}", 'ऌ');
    m.insert("l\u{325}\u{304}", 'ॡ');
    m.insert("ma", 'म');
    m.insert("na", 'न');
    m.insert("o", 'ओ');
    m.insert("pa", 'प');
    m.insert("pha", 'फ');
    m.insert("ra", 'र');
    m.insert("r\u{325}", 'ऋ');
    m.insert("r\u{325}\u{304}", 'ॠ');
    m.insert("sa", 'स');
    m.insert("ta", 'त');
    m.insert("tha", 'थ');
    m.insert("u", 'उ');
    m.insert("va", 'व');
    m.insert("ya", 'य');
    m.insert("ña", 'ञ');
    m.insert("ā", 'आ');
    m.insert("ī", 'ई');
    m.insert("śa", 'श');
    m.insert("ū", 'ऊ');
    m.insert("ḍa", 'ड');
    m.insert("ḍha", 'ढ');
    m.insert("ṅa", 'ङ');
    m.insert("ṇa", 'ण');
    m.insert("ṣa", 'ष');
    m.insert("ṭa", 'ट');
    m.insert("ṭha", 'ठ');
    m
});

/// Devanagari to ISO character mappings
pub static DEVA_TO_ISO: Lazy<HashMap<char, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert('७', "7");
    m.insert('ॠ', "r\u{325}\u{304}");
    m.insert('ऐ', "ai");
    m.insert('द', "da");
    m.insert('ह', "ha");
    m.insert('त', "ta");
    m.insert('न', "na");
    m.insert('ठ', "ṭha");
    m.insert('ग', "ga");
    m.insert('ण', "ṇa");
    m.insert('८', "8");
    m.insert('र', "ra");
    m.insert('२', "2");
    m.insert('१', "1");
    m.insert('घ', "gha");
    m.insert('ट', "ṭa");
    m.insert('आ', "ā");
    m.insert('ञ', "ña");
    m.insert('६', "6");
    m.insert('ऊ', "ū");
    m.insert('५', "5");
    m.insert('इ', "i");
    m.insert('ष', "ṣa");
    m.insert('ख', "kha");
    m.insert('झ', "jha");
    m.insert('ऋ', "r\u{325}");
    m.insert('उ', "u");
    m.insert('ढ', "ḍha");
    m.insert('प', "pa");
    m.insert('९', "9");
    m.insert('ओ', "o");
    m.insert('ई', "ī");
    m.insert('म', "ma");
    m.insert('छ', "cha");
    m.insert('अ', "a");
    m.insert('भ', "bha");
    m.insert('ड', "ḍa");
    m.insert('ए', "e");
    m.insert('ल', "la");
    m.insert('ध', "dha");
    m.insert('य', "ya");
    m.insert('व', "va");
    m.insert('ङ', "ṅa");
    m.insert('ऌ', "l\u{325}");
    m.insert('थ', "tha");
    m.insert('ॡ', "l\u{325}\u{304}");
    m.insert('श', "śa");
    m.insert('ज', "ja");
    m.insert('फ', "pha");
    m.insert('औ', "au");
    m.insert('च', "ca");
    m.insert('४', "4");
    m.insert('क', "ka");
    m.insert('ब', "ba");
    m.insert('३', "3");
    m.insert('०', "0");
    m.insert('स', "sa");
    m
});
