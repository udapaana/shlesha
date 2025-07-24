use super::*;
use crate::modules::hub::tokens::{AbugidaToken, AlphabetToken, HubToken};

#[test]
fn test_hub_creation() {
    let hub = Hub::new();
    // Verify hub is created and functional
    assert!(hub.abugida_to_alphabet_tokens(&vec![]).is_ok());
}

#[test]
fn test_basic_token_conversion() {
    let hub = Hub::new();

    // Test basic abugida to alphabet conversion
    let input_tokens = vec![
        HubToken::Abugida(AbugidaToken::VowelA),
        HubToken::Abugida(AbugidaToken::ConsonantK),
    ];

    let result = hub.abugida_to_alphabet_tokens(&input_tokens);
    match result {
        Ok(alphabet_tokens) => {
            // Verify the conversion worked correctly
            // ConsonantK in abugida becomes ConsonantK + implicit 'a' in alphabet
            assert_eq!(alphabet_tokens.len(), 3);
            assert!(matches!(
                alphabet_tokens[0],
                HubToken::Alphabet(AlphabetToken::VowelA)
            ));
            assert!(matches!(
                alphabet_tokens[1],
                HubToken::Alphabet(AlphabetToken::ConsonantK)
            ));
            assert!(matches!(
                alphabet_tokens[2],
                HubToken::Alphabet(AlphabetToken::VowelA)
            )); // implicit 'a'
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[test]
fn test_passthrough_tokens() {
    let hub = Hub::new();

    // Test that alphabet tokens pass through abugida conversion
    let input_tokens = vec![HubToken::Alphabet(AlphabetToken::VowelA)];

    let result = hub.abugida_to_alphabet_tokens(&input_tokens);
    match result {
        Ok(output_tokens) => {
            assert_eq!(output_tokens.len(), 1);
            assert!(matches!(
                output_tokens[0],
                HubToken::Alphabet(AlphabetToken::VowelA)
            ));
        }
        Err(e) => {
            panic!("Passthrough should not fail: {:?}", e);
        }
    }
}

#[test]
fn test_unknown_token_handling() {
    let hub = Hub::new();

    // Test unknown character handling
    let input_tokens = vec![HubToken::Abugida(AbugidaToken::Unknown("?".to_string()))];

    let result = hub.abugida_to_alphabet_tokens(&input_tokens);
    match result {
        Ok(output_tokens) => {
            assert_eq!(output_tokens.len(), 1);
            match &output_tokens[0] {
                HubToken::Alphabet(AlphabetToken::Unknown(s)) => {
                    assert_eq!(s, "?");
                }
                _ => panic!("Expected Unknown alphabet token"),
            }
        }
        Err(e) => {
            panic!("Unknown token should pass through: {:?}", e);
        }
    }
}

#[test]
fn test_mark_reordering_abugida_to_alphabet() {
    let hub = Hub::new();

    // Test case: Abugida tokens [ConsonantM, MarkVisarga, MarkVerticalLineAbove]
    // Should become [ConsonantM, VowelA (implicit), MarkVerticalLineAbove, MarkVisarga] in alphabet
    let input_tokens = vec![
        HubToken::Abugida(AbugidaToken::ConsonantM),
        HubToken::Abugida(AbugidaToken::MarkVisarga),
        HubToken::Abugida(AbugidaToken::MarkVerticalLineAbove),
    ];

    let result = hub.abugida_to_alphabet_tokens(&input_tokens);
    match result {
        Ok(output_tokens) => {
            println!("Abugida to Alphabet tokens: {:?}", output_tokens);

            // Should have: ConsonantM, implicit VowelA, MarkVerticalLineAbove, MarkVisarga
            // When converting from Indic to Roman, vedic accents come before yogavaha marks
            assert_eq!(output_tokens.len(), 4);
            assert!(matches!(
                output_tokens[0],
                HubToken::Alphabet(AlphabetToken::ConsonantM)
            ));
            assert!(matches!(
                output_tokens[1],
                HubToken::Alphabet(AlphabetToken::VowelA)
            ));
            assert!(matches!(
                output_tokens[2],
                HubToken::Alphabet(AlphabetToken::MarkVerticalLineAbove)
            ));
            assert!(matches!(
                output_tokens[3],
                HubToken::Alphabet(AlphabetToken::MarkVisarga)
            ));
        }
        Err(e) => panic!("Conversion failed: {:?}", e),
    }
}

#[test]
fn test_mark_reordering_alphabet_to_abugida() {
    let hub = Hub::new();

    // Test case: Alphabet tokens [ConsonantM, VowelA, MarkVerticalLineAbove, MarkVisarga]
    // Should become [ConsonantM, MarkVisarga, MarkVerticalLineAbove] in abugida (yogavaha before vedic)
    let input_tokens = vec![
        HubToken::Alphabet(AlphabetToken::ConsonantM),
        HubToken::Alphabet(AlphabetToken::VowelA),
        HubToken::Alphabet(AlphabetToken::MarkVerticalLineAbove),
        HubToken::Alphabet(AlphabetToken::MarkVisarga),
    ];

    let result = hub.alphabet_to_abugida_tokens(&input_tokens);
    match result {
        Ok(output_tokens) => {
            println!("Alphabet to Abugida tokens: {:?}", output_tokens);

            // Should have: ConsonantM (no virama as followed by VowelA), MarkVisarga, MarkVerticalLineAbove
            assert_eq!(output_tokens.len(), 3);
            assert!(matches!(
                output_tokens[0],
                HubToken::Abugida(AbugidaToken::ConsonantM)
            ));
            assert!(matches!(
                output_tokens[1],
                HubToken::Abugida(AbugidaToken::MarkVisarga)
            ));
            assert!(matches!(
                output_tokens[2],
                HubToken::Abugida(AbugidaToken::MarkVerticalLineAbove)
            ));
        }
        Err(e) => panic!("Conversion failed: {:?}", e),
    }
}
