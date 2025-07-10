use super::{HubError, HubToken, HubTokenSequence, AbugidaToken, AlphabetToken};

/// Manual implementation of hub conversions with proper implicit 'a' handling
pub struct ManualHubConverter;

impl ManualHubConverter {
    /// Convert abugida tokens to alphabet tokens using stack-based approach for implicit 'a' handling
    pub fn abugida_to_alphabet(tokens: &HubTokenSequence) -> Result<HubTokenSequence, HubError> {
        let mut result = Vec::new();
        let mut stack = Vec::new(); // Stack for managing implicit vowels
        
        for token in tokens {
            match token {
                HubToken::Abugida(abugida_token) => {
                    if token.is_consonant() {
                        // Push consonant to stack
                        let alphabet_consonant = Self::map_consonant(abugida_token)?;
                        stack.push(HubToken::Alphabet(alphabet_consonant));
                        // Push implicit 'a' vowel
                        stack.push(HubToken::Alphabet(AlphabetToken::VowelA));
                    } else if token.is_virama() {
                        // Virama suppresses the implicit 'a' - pop it from stack
                        if let Some(HubToken::Alphabet(AlphabetToken::VowelA)) = stack.last() {
                            stack.pop(); // Remove the implicit 'a'
                        }
                        // Don't output the virama itself - it's just a suppression marker
                    } else if token.is_vowel_sign() {
                        // Vowel sign replaces the implicit 'a' - pop it and add the vowel sign
                        if let Some(HubToken::Alphabet(AlphabetToken::VowelA)) = stack.last() {
                            stack.pop(); // Remove the implicit 'a'
                        }
                        // Map vowel sign to corresponding vowel
                        let vowel = Self::map_vowel_sign(abugida_token)?;
                        stack.push(HubToken::Alphabet(vowel));
                    } else if token.is_vowel() {
                        // Flush stack and add independent vowel
                        result.extend(stack.drain(..));
                        let vowel = Self::map_vowel(abugida_token)?;
                        result.push(HubToken::Alphabet(vowel));
                    } else {
                        // Other tokens (marks, digits, unknown) - flush stack and add directly
                        result.extend(stack.drain(..));
                        let mapped_token = Self::map_other_token(abugida_token)?;
                        result.push(HubToken::Alphabet(mapped_token));
                    }
                }
                HubToken::Alphabet(alphabet_token) => {
                    // Already alphabet, flush stack and pass through
                    result.extend(stack.drain(..));
                    result.push(HubToken::Alphabet(alphabet_token.clone()));
                }
            }
        }
        
        // Flush any remaining tokens in stack
        result.extend(stack.drain(..));
        
        Ok(result)
    }
    
    /// Map consonant tokens 1:1
    fn map_consonant(token: &AbugidaToken) -> Result<AlphabetToken, HubError> {
        match token {
            AbugidaToken::ConsonantK => Ok(AlphabetToken::ConsonantK),
            AbugidaToken::ConsonantKh => Ok(AlphabetToken::ConsonantKh),
            AbugidaToken::ConsonantG => Ok(AlphabetToken::ConsonantG),
            AbugidaToken::ConsonantGh => Ok(AlphabetToken::ConsonantGh),
            AbugidaToken::ConsonantNg => Ok(AlphabetToken::ConsonantNg),
            AbugidaToken::ConsonantC => Ok(AlphabetToken::ConsonantC),
            AbugidaToken::ConsonantCh => Ok(AlphabetToken::ConsonantCh),
            AbugidaToken::ConsonantJ => Ok(AlphabetToken::ConsonantJ),
            AbugidaToken::ConsonantJh => Ok(AlphabetToken::ConsonantJh),
            AbugidaToken::ConsonantNy => Ok(AlphabetToken::ConsonantNy),
            AbugidaToken::ConsonantT => Ok(AlphabetToken::ConsonantT),
            AbugidaToken::ConsonantTh => Ok(AlphabetToken::ConsonantTh),
            AbugidaToken::ConsonantD => Ok(AlphabetToken::ConsonantD),
            AbugidaToken::ConsonantDh => Ok(AlphabetToken::ConsonantDh),
            AbugidaToken::ConsonantN => Ok(AlphabetToken::ConsonantN),
            AbugidaToken::ConsonantTt => Ok(AlphabetToken::ConsonantTt),
            AbugidaToken::ConsonantTth => Ok(AlphabetToken::ConsonantTth),
            AbugidaToken::ConsonantDd => Ok(AlphabetToken::ConsonantDd),
            AbugidaToken::ConsonantDdh => Ok(AlphabetToken::ConsonantDdh),
            AbugidaToken::ConsonantNn => Ok(AlphabetToken::ConsonantNn),
            AbugidaToken::ConsonantP => Ok(AlphabetToken::ConsonantP),
            AbugidaToken::ConsonantPh => Ok(AlphabetToken::ConsonantPh),
            AbugidaToken::ConsonantB => Ok(AlphabetToken::ConsonantB),
            AbugidaToken::ConsonantBh => Ok(AlphabetToken::ConsonantBh),
            AbugidaToken::ConsonantM => Ok(AlphabetToken::ConsonantM),
            AbugidaToken::ConsonantY => Ok(AlphabetToken::ConsonantY),
            AbugidaToken::ConsonantR => Ok(AlphabetToken::ConsonantR),
            AbugidaToken::ConsonantL => Ok(AlphabetToken::ConsonantL),
            AbugidaToken::ConsonantV => Ok(AlphabetToken::ConsonantV),
            AbugidaToken::ConsonantLl => Ok(AlphabetToken::ConsonantLl),
            AbugidaToken::ConsonantSh => Ok(AlphabetToken::ConsonantSh),
            AbugidaToken::ConsonantSs => Ok(AlphabetToken::ConsonantSs),
            AbugidaToken::ConsonantS => Ok(AlphabetToken::ConsonantS),
            AbugidaToken::ConsonantH => Ok(AlphabetToken::ConsonantH),
            _ => Err(HubError::MappingNotFound(format!("Not a consonant: {:?}", token))),
        }
    }
    
    /// Map vowel tokens 1:1
    fn map_vowel(token: &AbugidaToken) -> Result<AlphabetToken, HubError> {
        match token {
            AbugidaToken::VowelA => Ok(AlphabetToken::VowelA),
            AbugidaToken::VowelAa => Ok(AlphabetToken::VowelAa),
            AbugidaToken::VowelI => Ok(AlphabetToken::VowelI),
            AbugidaToken::VowelIi => Ok(AlphabetToken::VowelIi),
            AbugidaToken::VowelU => Ok(AlphabetToken::VowelU),
            AbugidaToken::VowelUu => Ok(AlphabetToken::VowelUu),
            AbugidaToken::VowelVocalicR => Ok(AlphabetToken::VowelVocalicR),
            AbugidaToken::VowelVocalicRr => Ok(AlphabetToken::VowelVocalicRr),
            AbugidaToken::VowelVocalicL => Ok(AlphabetToken::VowelVocalicL),
            AbugidaToken::VowelVocalicLl => Ok(AlphabetToken::VowelVocalicLl),
            AbugidaToken::VowelE => Ok(AlphabetToken::VowelE),
            AbugidaToken::VowelAi => Ok(AlphabetToken::VowelAi),
            AbugidaToken::VowelO => Ok(AlphabetToken::VowelO),
            AbugidaToken::VowelAu => Ok(AlphabetToken::VowelAu),
            _ => Err(HubError::MappingNotFound(format!("Not a vowel: {:?}", token))),
        }
    }
    
    /// Map vowel sign tokens to corresponding vowels
    fn map_vowel_sign(token: &AbugidaToken) -> Result<AlphabetToken, HubError> {
        match token {
            AbugidaToken::VowelSignAa => Ok(AlphabetToken::VowelAa),
            AbugidaToken::VowelSignI => Ok(AlphabetToken::VowelI),
            AbugidaToken::VowelSignIi => Ok(AlphabetToken::VowelIi),
            AbugidaToken::VowelSignU => Ok(AlphabetToken::VowelU),
            AbugidaToken::VowelSignUu => Ok(AlphabetToken::VowelUu),
            AbugidaToken::VowelSignVocalicR => Ok(AlphabetToken::VowelVocalicR),
            AbugidaToken::VowelSignVocalicRr => Ok(AlphabetToken::VowelVocalicRr),
            AbugidaToken::VowelSignVocalicL => Ok(AlphabetToken::VowelVocalicL),
            AbugidaToken::VowelSignVocalicLl => Ok(AlphabetToken::VowelVocalicLl),
            AbugidaToken::VowelSignE => Ok(AlphabetToken::VowelE),
            AbugidaToken::VowelSignAi => Ok(AlphabetToken::VowelAi),
            AbugidaToken::VowelSignO => Ok(AlphabetToken::VowelO),
            AbugidaToken::VowelSignAu => Ok(AlphabetToken::VowelAu),
            _ => Err(HubError::MappingNotFound(format!("Not a vowel sign: {:?}", token))),
        }
    }
    
    /// Map other tokens (marks, digits, unknown)
    fn map_other_token(token: &AbugidaToken) -> Result<AlphabetToken, HubError> {
        match token {
            // Marks
            AbugidaToken::MarkAnusvara => Ok(AlphabetToken::MarkAnusvara),
            AbugidaToken::MarkVisarga => Ok(AlphabetToken::MarkVisarga),
            AbugidaToken::MarkCandrabindu => Ok(AlphabetToken::MarkCandrabindu),
            AbugidaToken::MarkAvagraha => Ok(AlphabetToken::MarkAvagraha),
            
            // Unknown
            AbugidaToken::Unknown(c) => Ok(AlphabetToken::Unknown(*c)),
            
            _ => Err(HubError::MappingNotFound(format!("No mapping for abugida token: {:?}", token))),
        }
    }
}