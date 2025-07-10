use shlesha::modules::hub::tokens::{AbugidaToken, AlphabetToken, HubToken};
use shlesha::modules::hub::{Hub, HubTrait};

fn main() {
    println!("Testing token-based hub conversion...");
    
    let hub = Hub::new();
    
    // Test AlphabetToken → AbugidaToken
    let alphabet_tokens = vec![
        HubToken::Alphabet(AlphabetToken::ConsonantK),
        HubToken::Alphabet(AlphabetToken::VowelA),
    ];
    
    println!("Input alphabet tokens: {:?}", alphabet_tokens);
    
    match hub.alphabet_to_abugida_tokens(&alphabet_tokens) {
        Ok(abugida_tokens) => {
            println!("Converted to abugida tokens: {:?}", abugida_tokens);
            
            // Test AbugidaToken → AlphabetToken (reverse)
            match hub.abugida_to_alphabet_tokens(&abugida_tokens) {
                Ok(back_to_alphabet) => {
                    println!("Converted back to alphabet tokens: {:?}", back_to_alphabet);
                }
                Err(e) => {
                    println!("Error converting back: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}