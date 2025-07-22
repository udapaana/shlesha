use super::{AbugidaToken, AlphabetToken, HubError, HubToken, HubTokenSequence};

/// Trait-based implementation of hub conversions with proper implicit 'a' handling
/// Uses an optimized state machine approach instead of stack-based processing
pub struct TraitBasedConverter;

impl TraitBasedConverter {
    /// Convert abugida tokens to alphabet tokens using state machine approach
    pub fn abugida_to_alphabet(tokens: &HubTokenSequence) -> Result<HubTokenSequence, HubError> {
        // Pre-allocate with estimated capacity
        let mut result = Vec::with_capacity(tokens.len());

        let mut i = 0;
        while i < tokens.len() {
            match &tokens[i] {
                HubToken::Abugida(abugida_token) => {
                    if abugida_token.is_consonant() {
                        // Find corresponding alphabet consonant
                        if let Some(alphabet_token) = abugida_token.to_alphabet() {
                            result.push(HubToken::Alphabet(alphabet_token));

                            // Check if next token is virama or vowel sign
                            let has_explicit_vowel = if i + 1 < tokens.len() {
                                match &tokens[i + 1] {
                                    HubToken::Abugida(next) => {
                                        next.is_virama() || next.is_vowel_sign()
                                    }
                                    _ => false,
                                }
                            } else {
                                false
                            };

                            // Add implicit 'a' if no virama or vowel sign follows
                            if !has_explicit_vowel {
                                result.push(HubToken::Alphabet(AlphabetToken::VowelA));
                            }
                        } else {
                            // No mapping - preserve as unknown
                            if let AbugidaToken::Unknown(s) = abugida_token {
                                result.push(HubToken::Alphabet(AlphabetToken::Unknown(s.clone())));
                            } else {
                                return Err(HubError::MappingNotFound(format!(
                                    "No alphabet mapping for {:?}",
                                    abugida_token
                                )));
                            }
                        }
                    } else if abugida_token.is_virama() {
                        // Virama consumed - skip it (implicit 'a' already suppressed above)
                    } else if abugida_token.is_vowel_sign() {
                        // Convert vowel sign to corresponding vowel
                        if let Some(vowel) = abugida_token.sign_to_vowel() {
                            if let Some(alphabet_vowel) = vowel.to_alphabet() {
                                result.push(HubToken::Alphabet(alphabet_vowel));
                            }
                        }
                    } else if abugida_token.is_vowel() {
                        // Independent vowel
                        if let Some(alphabet_vowel) = abugida_token.to_alphabet() {
                            result.push(HubToken::Alphabet(alphabet_vowel));
                        } else if let AbugidaToken::Unknown(s) = abugida_token {
                            result.push(HubToken::Alphabet(AlphabetToken::Unknown(s.clone())));
                        }
                    } else if abugida_token.is_mark() {
                        if let Some(alphabet_mark) = abugida_token.to_alphabet() {
                            let current_token = HubToken::Alphabet(alphabet_mark);

                            // In Roman scripts, vedic accents come before yogavaha marks
                            // If we're converting from Indic (where it's yogavaha + accent)
                            // to Roman (where it's accent + yogavaha), check if this is a vedic accent
                            // and the previous token was yogavaha
                            if current_token.is_vedic_accent() && !result.is_empty() {
                                if let Some(last_token) = result.last() {
                                    if last_token.is_yogavaha() {
                                        // Pop the yogavaha, push vedic accent, then push yogavaha back
                                        let yogavaha = result.pop().unwrap();
                                        result.push(current_token);
                                        result.push(yogavaha);
                                    } else {
                                        result.push(current_token);
                                    }
                                } else {
                                    result.push(current_token);
                                }
                            } else {
                                result.push(current_token);
                            }
                        } else if let AbugidaToken::Unknown(s) = abugida_token {
                            result.push(HubToken::Alphabet(AlphabetToken::Unknown(s.clone())));
                        }
                    } else {
                        // Unknown token type - preserve
                        if let AbugidaToken::Unknown(s) = abugida_token {
                            result.push(HubToken::Alphabet(AlphabetToken::Unknown(s.clone())));
                        }
                    }
                }
                HubToken::Alphabet(_) => {
                    // Already alphabet - pass through
                    result.push(tokens[i].clone());
                }
            }
            i += 1;
        }

        Ok(result)
    }

    /// Convert alphabet tokens to abugida tokens using state machine approach
    pub fn alphabet_to_abugida(tokens: &HubTokenSequence) -> Result<HubTokenSequence, HubError> {
        // Pre-allocate with estimated capacity (worst case: each consonant needs a virama)
        let mut result = Vec::with_capacity(tokens.len() * 2);

        let mut i = 0;
        while i < tokens.len() {
            match &tokens[i] {
                HubToken::Alphabet(alphabet_token) => {
                    if alphabet_token.is_consonant() {
                        // Convert consonant
                        if let Some(abugida_consonant) = alphabet_token.to_abugida() {
                            result.push(HubToken::Abugida(abugida_consonant));

                            // Look ahead to determine if we need a virama
                            let needs_virama = if i + 1 < tokens.len() {
                                match &tokens[i + 1] {
                                    HubToken::Alphabet(next) => {
                                        if *next == AlphabetToken::VowelA {
                                            // Explicit 'a' after consonant - skip it
                                            i += 1;
                                            false
                                        } else if next.is_vowel() {
                                            // Other vowel - will be converted to vowel sign
                                            false
                                        } else if next.is_consonant() || next.is_mark() {
                                            // Consonant cluster or mark - needs virama
                                            true
                                        } else {
                                            // Unknown or other - needs virama
                                            true
                                        }
                                    }
                                    _ => true, // Non-alphabet token - needs virama
                                }
                            } else {
                                // End of input - final consonant needs virama
                                true
                            };

                            if needs_virama {
                                result.push(HubToken::Abugida(AbugidaToken::MarkVirama));
                            }
                        } else if let AlphabetToken::Unknown(s) = alphabet_token {
                            result.push(HubToken::Abugida(AbugidaToken::Unknown(s.clone())));
                        }
                    } else if alphabet_token.is_vowel() {
                        // Check if this vowel follows a consonant (for vowel sign conversion)
                        let prev_was_consonant = if !result.is_empty() {
                            match result.last() {
                                Some(HubToken::Abugida(prev)) => prev.is_consonant(),
                                _ => false,
                            }
                        } else {
                            false
                        };

                        if prev_was_consonant && *alphabet_token != AlphabetToken::VowelA {
                            // Convert to vowel sign after consonant
                            if let Some(abugida_vowel) = alphabet_token.to_abugida() {
                                if let Some(sign) = abugida_vowel.vowel_to_sign() {
                                    // Remove virama if it was added
                                    if let Some(HubToken::Abugida(AbugidaToken::MarkVirama)) =
                                        result.last()
                                    {
                                        result.pop();
                                    }
                                    result.push(HubToken::Abugida(sign));
                                }
                            }
                        } else if *alphabet_token != AlphabetToken::VowelA || !prev_was_consonant {
                            // Independent vowel (not implicit 'a')
                            if let Some(abugida_vowel) = alphabet_token.to_abugida() {
                                result.push(HubToken::Abugida(abugida_vowel));
                            }
                        }
                        // If it's VowelA after consonant, it's implicit - already handled
                    } else if alphabet_token.is_mark() {
                        if let Some(abugida_mark) = alphabet_token.to_abugida() {
                            let current_token = HubToken::Abugida(abugida_mark);

                            // In Indic scripts, yogavaha marks come before vedic accents
                            // If we're converting from Roman (where it's accent + yogavaha)
                            // to Indic (where it's yogavaha + accent), we need to check ahead
                            if current_token.is_vedic_accent() && i + 1 < tokens.len() {
                                if let HubToken::Alphabet(next_token) = &tokens[i + 1] {
                                    if next_token.is_yogavaha() {
                                        // Convert and push yogavaha first
                                        if let Some(abugida_yogavaha) = next_token.to_abugida() {
                                            result.push(HubToken::Abugida(abugida_yogavaha));
                                        }
                                        // Then push the vedic accent
                                        result.push(current_token);
                                        // Skip the next token since we already processed it
                                        i += 2;
                                        continue;
                                    }
                                }
                            }

                            result.push(current_token);
                        } else if let AlphabetToken::Unknown(s) = alphabet_token {
                            result.push(HubToken::Abugida(AbugidaToken::Unknown(s.clone())));
                        }
                    } else if let AlphabetToken::Unknown(s) = alphabet_token {
                        result.push(HubToken::Abugida(AbugidaToken::Unknown(s.clone())));
                    } else {
                        // Other tokens - try direct mapping
                        if let Some(abugida_token) = alphabet_token.to_abugida() {
                            result.push(HubToken::Abugida(abugida_token));
                        }
                    }
                }
                HubToken::Abugida(_) => {
                    // Already abugida - pass through
                    result.push(tokens[i].clone());
                }
            }
            i += 1;
        }

        Ok(result)
    }
}
