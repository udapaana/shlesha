use super::*;
use crate::modules::hub::tokens::{AbugidaToken, AlphabetToken, HubToken};

#[test]
fn test_hub_creation() {
    let _hub = Hub::new();
    assert!(true); // Basic creation test
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
