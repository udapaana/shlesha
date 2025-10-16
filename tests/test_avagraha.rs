use shlesha::Shlesha;

#[test]
fn test_slp1_avagraha() {
    let shlesha = Shlesha::new();

    // Test SLP1 backtick to Devanagari avagraha
    let result = shlesha.transliterate("`", "slp1", "devanagari").unwrap();
    assert_eq!(
        result, "ऽ",
        "SLP1 backtick should convert to Devanagari avagraha"
    );

    // Test Devanagari avagraha to SLP1 backtick
    let result = shlesha.transliterate("ऽ", "devanagari", "slp1").unwrap();
    assert_eq!(
        result, "`",
        "Devanagari avagraha should convert to SLP1 backtick"
    );

    // Test in context
    let result = shlesha
        .transliterate("namo`stu", "slp1", "devanagari")
        .unwrap();
    assert_eq!(result, "नमोऽस्तु", "SLP1 avagraha in context");

    let result = shlesha
        .transliterate("नमोऽस्तु", "devanagari", "slp1")
        .unwrap();
    assert_eq!(result, "namo`stu", "Devanagari avagraha in context to SLP1");
}

#[test]
fn test_avagraha_across_scripts() {
    let shlesha = Shlesha::new();

    // Test SLP1 to IAST (now converts to apostrophe since IAST has avagraha)
    let result = shlesha.transliterate("`", "slp1", "iast").unwrap();
    assert_eq!(
        result, "'",
        "SLP1 avagraha to IAST should convert to apostrophe"
    );

    // Test IAST to SLP1 with apostrophe
    let result = shlesha.transliterate("'", "iast", "slp1").unwrap();
    assert_eq!(
        result, "`",
        "IAST apostrophe should convert to SLP1 backtick"
    );
}
