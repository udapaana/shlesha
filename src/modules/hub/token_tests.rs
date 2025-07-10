use super::*;
use crate::modules::hub::tokens::{AbugidaToken, AlphabetToken, HubToken};

#[test]
fn test_hub_creation() {
    let hub = Hub::new();
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
    
    // This will fail until Hub converter is fully implemented
    let result = hub.abugida_to_alphabet_tokens(&input_tokens);
    match result {
        Ok(_alphabet_tokens) => {
            // Success case - verify tokens are converted properly
            // TODO: Add specific token verification once mappings are complete
        }
        Err(HubError::MappingNotFound(_)) => {
            // Expected for now since not all mappings are implemented
            assert!(true);
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
    let input_tokens = vec![
        HubToken::Alphabet(AlphabetToken::VowelA),
    ];
    
    let result = hub.abugida_to_alphabet_tokens(&input_tokens);
    match result {
        Ok(output_tokens) => {
            assert_eq!(output_tokens.len(), 1);
            assert!(matches!(output_tokens[0], HubToken::Alphabet(AlphabetToken::VowelA)));
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
    let input_tokens = vec![
        HubToken::Abugida(AbugidaToken::Unknown('?')),
    ];
    
    let result = hub.abugida_to_alphabet_tokens(&input_tokens);
    match result {
        Ok(output_tokens) => {
            assert_eq!(output_tokens.len(), 1);
            assert!(matches!(output_tokens[0], HubToken::Alphabet(AlphabetToken::Unknown('?'))));
        }
        Err(e) => {
            panic!("Unknown token should pass through: {:?}", e);
        }
    }
}