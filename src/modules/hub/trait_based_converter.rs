use super::{AbugidaToken, AlphabetToken, HubError, HubToken, HubTokenSequence};

/// Trait-based implementation of hub conversions with proper implicit 'a' handling
/// This works with any set of generated tokens by using their traits rather than specific names
pub struct TraitBasedConverter;

impl TraitBasedConverter {
    /// Convert abugida tokens to alphabet tokens using trait-based approach
    pub fn abugida_to_alphabet(tokens: &HubTokenSequence) -> Result<HubTokenSequence, HubError> {
        let mut result = Vec::new();
        let mut stack = Vec::new(); // Stack for managing implicit vowels
        
        for token in tokens {
            match token {
                HubToken::Abugida(abugida_token) => {
                    if abugida_token.is_consonant() {
                        // Find corresponding alphabet consonant
                        if let Some(alphabet_token) = Self::find_alphabet_match(abugida_token) {
                            stack.push(HubToken::Alphabet(alphabet_token));
                            // Push implicit 'a' vowel
                            stack.push(HubToken::Alphabet(AlphabetToken::VowelA));
                        } else {
                            // No mapping - preserve as unknown
                            if let AbugidaToken::Unknown(s) = abugida_token {
                                stack.push(HubToken::Alphabet(AlphabetToken::Unknown(s.clone())));
                            } else {
                                return Err(HubError::MappingNotFound(
                                    format!("No alphabet mapping for {:?}", abugida_token)
                                ));
                            }
                        }
                    } else if abugida_token.is_virama() {
                        // Virama suppresses the implicit 'a' - pop it from stack
                        if let Some(HubToken::Alphabet(AlphabetToken::VowelA)) = stack.last() {
                            stack.pop(); // Remove the implicit 'a'
                        }
                        // Don't output the virama itself
                    } else if abugida_token.is_vowel_sign() {
                        // Vowel sign replaces the implicit 'a'
                        if let Some(HubToken::Alphabet(AlphabetToken::VowelA)) = stack.last() {
                            stack.pop(); // Remove the implicit 'a'
                        }
                        // Convert vowel sign to corresponding vowel
                        if let Some(vowel) = abugida_token.sign_to_vowel() {
                            if let Some(alphabet_vowel) = Self::find_alphabet_match(&vowel) {
                                stack.push(HubToken::Alphabet(alphabet_vowel));
                            }
                        }
                    } else if abugida_token.is_vowel() {
                        // Flush stack and add independent vowel
                        result.extend(stack.drain(..));
                        
                        if let Some(alphabet_vowel) = Self::find_alphabet_match(abugida_token) {
                            result.push(HubToken::Alphabet(alphabet_vowel));
                        } else if let AbugidaToken::Unknown(s) = abugida_token {
                            result.push(HubToken::Alphabet(AlphabetToken::Unknown(s.clone())));
                        }
                    } else if abugida_token.is_mark() {
                        // Flush stack first
                        result.extend(stack.drain(..));
                        
                        if let Some(alphabet_mark) = Self::find_alphabet_match(abugida_token) {
                            result.push(HubToken::Alphabet(alphabet_mark));
                        } else if let AbugidaToken::Unknown(s) = abugida_token {
                            result.push(HubToken::Alphabet(AlphabetToken::Unknown(s.clone())));
                        }
                    } else {
                        // Unknown token type - flush stack and preserve
                        result.extend(stack.drain(..));
                        if let AbugidaToken::Unknown(s) = abugida_token {
                            result.push(HubToken::Alphabet(AlphabetToken::Unknown(s.clone())));
                        }
                    }
                }
                HubToken::Alphabet(_) => {
                    // Already alphabet - flush stack and pass through
                    result.extend(stack.drain(..));
                    result.push(token.clone());
                }
            }
        }
        
        // Flush any remaining tokens
        result.extend(stack);
        
        Ok(result)
    }
    
    /// Convert alphabet tokens to abugida tokens using stack-based approach
    pub fn alphabet_to_abugida(tokens: &HubTokenSequence) -> Result<HubTokenSequence, HubError> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        
        for token in tokens {
            match token {
                HubToken::Alphabet(alphabet_token) => {
                    if alphabet_token.is_consonant() {
                        // First, check if we need to process the previous consonant
                        if !stack.is_empty() {
                            // Previous consonant followed by another consonant - needs virama
                            // Pop the implicit 'a' if it exists
                            if let Some(HubToken::Alphabet(AlphabetToken::VowelA)) = stack.last() {
                                stack.pop();
                            }
                            // Add virama after the previous consonant
                            stack.push(HubToken::Abugida(AbugidaToken::MarkVirama));
                        }
                        
                        // Flush stack to result
                        result.append(&mut stack);
                        
                        // Convert and push consonant
                        if let Some(abugida_consonant) = Self::find_abugida_match(alphabet_token) {
                            stack.push(HubToken::Abugida(abugida_consonant));
                            // Push implicit 'a' (will be removed if followed by consonant or vowel sign)
                            stack.push(HubToken::Alphabet(AlphabetToken::VowelA));
                        } else if let AlphabetToken::Unknown(s) = alphabet_token {
                            stack.push(HubToken::Abugida(AbugidaToken::Unknown(s.clone())));
                        }
                    } else if alphabet_token.is_vowel() {
                        if *alphabet_token == AlphabetToken::VowelA {
                            // Explicit 'a' - if stack has implicit 'a', it's already correct
                            if stack.is_empty() || !matches!(stack.last(), Some(HubToken::Alphabet(AlphabetToken::VowelA))) {
                                // Independent 'a' at start or after non-consonant
                                result.append(&mut stack);
                                if let Some(abugida_vowel) = Self::find_abugida_match(alphabet_token) {
                                    result.push(HubToken::Abugida(abugida_vowel));
                                }
                            } else {
                                // We have implicit 'a' in stack, explicit 'a' confirms it
                                // Flush the stack with the consonant + implicit 'a'
                                // But first remove the implicit 'a' token, as it's already inherent in the consonant
                                if let Some(HubToken::Alphabet(AlphabetToken::VowelA)) = stack.last() {
                                    stack.pop();
                                }
                                result.append(&mut stack);
                            }
                        } else {
                            // Non-'a' vowel
                            if !stack.is_empty() {
                                // Pop implicit 'a' if present
                                if let Some(HubToken::Alphabet(AlphabetToken::VowelA)) = stack.last() {
                                    stack.pop();
                                }
                                // Add vowel sign
                                if let Some(abugida_vowel) = Self::find_abugida_match(alphabet_token) {
                                    if let Some(sign) = abugida_vowel.vowel_to_sign() {
                                        stack.push(HubToken::Abugida(sign));
                                    }
                                }
                                // Flush stack
                                result.append(&mut stack);
                            } else {
                                // Independent vowel
                                if let Some(abugida_vowel) = Self::find_abugida_match(alphabet_token) {
                                    result.push(HubToken::Abugida(abugida_vowel));
                                }
                            }
                        }
                    } else if alphabet_token.is_mark() {
                        // Marks - flush stack and add
                        result.append(&mut stack);
                        if let Some(abugida_mark) = Self::find_abugida_match(alphabet_token) {
                            result.push(HubToken::Abugida(abugida_mark));
                        } else if let AlphabetToken::Unknown(s) = alphabet_token {
                            result.push(HubToken::Abugida(AbugidaToken::Unknown(s.clone())));
                        }
                    } else if let AlphabetToken::Unknown(s) = alphabet_token {
                        // Unknown token - check if previous was consonant
                        if !stack.is_empty() {
                            // Pop implicit 'a' and add virama
                            if let Some(HubToken::Alphabet(AlphabetToken::VowelA)) = stack.last() {
                                stack.pop();
                                stack.push(HubToken::Abugida(AbugidaToken::MarkVirama));
                            }
                        }
                        result.append(&mut stack);
                        result.push(HubToken::Abugida(AbugidaToken::Unknown(s.clone())));
                    }
                }
                HubToken::Abugida(_) => {
                    // Already abugida - flush stack and pass through
                    result.append(&mut stack);
                    result.push(token.clone());
                }
            }
        }
        
        // Process final stack
        if !stack.is_empty() {
            // If last token in stack is implicit 'a', remove it and add virama
            let mut final_stack = stack;
            
            // Check if the last token is implicit 'a' after a consonant
            if final_stack.len() >= 2 {
                if let Some(HubToken::Alphabet(AlphabetToken::VowelA)) = final_stack.last() {
                    // Check if the token before the 'a' is a consonant
                    if let Some(HubToken::Abugida(abugida_token)) = final_stack.get(final_stack.len() - 2) {
                        if abugida_token.is_consonant() {
                            // Remove the implicit 'a' and add virama
                            final_stack.pop();
                            final_stack.push(HubToken::Abugida(AbugidaToken::MarkVirama));
                        }
                    }
                }
            } else if final_stack.len() == 1 {
                // Single consonant at end needs virama
                if let Some(HubToken::Abugida(abugida_token)) = final_stack.last() {
                    if abugida_token.is_consonant() {
                        final_stack.push(HubToken::Abugida(AbugidaToken::MarkVirama));
                    }
                }
            }
            
            result.append(&mut final_stack);
        }
        
        Ok(result)
    }
    
    // Helper to find matching alphabet token
    fn find_alphabet_match(abugida: &AbugidaToken) -> Option<AlphabetToken> {
        // Use the generated mapping method
        abugida.to_alphabet()
    }
    
    // Helper to find matching abugida token  
    fn find_abugida_match(alphabet: &AlphabetToken) -> Option<AbugidaToken> {
        // Use the generated mapping method
        alphabet.to_abugida()
    }
}