//! Auto-generated from iast.toml
//! DO NOT EDIT MANUALLY

#![allow(dead_code)]

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// TO_ISO mappings
pub static TO_ISO: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("'", "'\"      # avagraha (same)");
    m.insert("a", "a");
    m.insert("ai", "ai");
    m.insert("au", "au");
    m.insert("b", "b");
    m.insert("bh", "bh");
    m.insert("c", "c");
    m.insert("ch", "ch");
    m.insert("d", "d");
    m.insert("dh", "dh");
    m.insert("e", "e");
    m.insert("g", "g");
    m.insert("gh", "gh");
    m.insert("h", "h");
    m.insert("i", "i");
    m.insert("j", "j");
    m.insert("jh", "jh");
    m.insert("k", "k");
    m.insert("kh", "kh");
    m.insert("l", "l");
    m.insert("m", "m");
    m.insert("m\u{310}", "m\u{310}\"      # candrabindu (same)");
    m.insert("n", "n");
    m.insert("o", "o");
    m.insert("p", "p");
    m.insert("ph", "ph");
    m.insert("r", "r");
    m.insert("s", "s");
    m.insert("t", "t");
    m.insert("th", "th");
    m.insert("u", "u");
    m.insert("v", "v");
    m.insert("y", "y");
    m.insert("ñ", "ñ");
    m.insert("ā", "ā");
    m.insert("ī", "ī");
    m.insert("ś", "ś");
    m.insert("ū", "ū");
    m.insert("ḍ", "ḍ");
    m.insert("ḍh", "ḍh");
    m.insert("ḥ", "ḥ\"      # visarga (same)");
    m.insert("ḷ", "l\u{325}\"     # IAST ḷ → ISO l\u{325}");
    m.insert("ḹ", "l\u{325}\u{304}\"    # IAST ḹ → ISO l\u{325}\u{304}");
    m.insert("ṁ", "ṁ\"      # alternate anusvara");
    m.insert("ṃ", "ṁ\"      # IAST anusvara → ISO anusvara");
    m.insert("ṅ", "ṅ");
    m.insert("ṇ", "ṇ");
    m.insert("ṛ", "r\u{325}\"     # IAST ṛ → ISO r\u{325}");
    m.insert("ṝ", "r\u{325}\u{304}\"    # IAST ṝ → ISO r\u{325}\u{304}");
    m.insert("ṣ", "ṣ");
    m.insert("ṭ", "ṭ");
    m.insert("ṭh", "ṭh");
    m
});

/// FROM_ISO mappings
pub static FROM_ISO: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("l\u{325}", "ḷ\"      # ISO l\u{325} → IAST ḷ");
    m.insert("l\u{325}\u{304}", "ḹ\"     # ISO l\u{325}\u{304} → IAST ḹ");
    m.insert("r\u{325}", "ṛ\"      # ISO r\u{325} → IAST ṛ");
    m.insert("r\u{325}\u{304}", "ṝ\"     # ISO r\u{325}\u{304} → IAST ṝ");
    m.insert("ṁ", "ṃ\"      # ISO anusvara → IAST anusvara");
    m
});
