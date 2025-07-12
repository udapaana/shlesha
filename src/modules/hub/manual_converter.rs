use super::{AbugidaToken, AlphabetToken, HubError, HubToken, HubTokenSequence};

/// Manual implementation of hub conversions with proper implicit 'a' handling
pub struct ManualHubConverter;

impl ManualHubConverter {
    /// Check if a token is an extended nukta consonant
    fn is_extended_token(token: &AlphabetToken) -> bool {
        matches!(
            token,
            AlphabetToken::ConsonantQa
                | AlphabetToken::ConsonantZa
                | AlphabetToken::ConsonantFa
                | AlphabetToken::ConsonantGha
                | AlphabetToken::ConsonantKha
                | AlphabetToken::ConsonantRra
                | AlphabetToken::ConsonantRrha
                | AlphabetToken::ConsonantYa
        )
    }
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
                        result.append(&mut stack);
                        let vowel = Self::map_vowel(abugida_token)?;
                        result.push(HubToken::Alphabet(vowel));
                    } else {
                        // Other tokens (marks, digits, unknown) - flush stack and add directly
                        result.append(&mut stack);
                        let mapped_token = Self::map_other_token(abugida_token)?;
                        result.push(HubToken::Alphabet(mapped_token));
                    }
                }
                HubToken::Alphabet(alphabet_token) => {
                    // Already alphabet, flush stack and pass through
                    result.append(&mut stack);
                    result.push(HubToken::Alphabet(alphabet_token.clone()));
                }
            }
        }

        // Flush any remaining tokens in stack
        result.append(&mut stack);

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

            // Nukta consonants
            AbugidaToken::ConsonantQa => Ok(AlphabetToken::ConsonantQa),
            AbugidaToken::ConsonantZa => Ok(AlphabetToken::ConsonantZa),
            AbugidaToken::ConsonantFa => Ok(AlphabetToken::ConsonantFa),
            AbugidaToken::ConsonantGha => Ok(AlphabetToken::ConsonantGha),
            AbugidaToken::ConsonantKha => Ok(AlphabetToken::ConsonantKha),
            AbugidaToken::ConsonantRra => Ok(AlphabetToken::ConsonantRra),
            AbugidaToken::ConsonantRrha => Ok(AlphabetToken::ConsonantRrha),
            AbugidaToken::ConsonantYa => Ok(AlphabetToken::ConsonantYa),

            _ => Err(HubError::MappingNotFound(format!(
                "Not a consonant: {:?}",
                token
            ))),
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
            _ => Err(HubError::MappingNotFound(format!(
                "Not a vowel: {:?}",
                token
            ))),
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
            _ => Err(HubError::MappingNotFound(format!(
                "Not a vowel sign: {:?}",
                token
            ))),
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

            // Vedic marks
            AbugidaToken::MarkUdatta => Ok(AlphabetToken::MarkUdatta),
            AbugidaToken::MarkAnudatta => Ok(AlphabetToken::MarkAnudatta),
            AbugidaToken::MarkDoubleSvarita => Ok(AlphabetToken::MarkDoubleSvarita),
            AbugidaToken::MarkTripleSvarita => Ok(AlphabetToken::MarkTripleSvarita),

            // Unknown - pass through as character
            AbugidaToken::Unknown(c) => Ok(AlphabetToken::Unknown(*c)),

            // For any other unmapped token, pass it through as an unknown character
            // This handles cases where tokens exist but aren't mapped yet
            _ => {
                // Try to extract a representative character if possible
                // For now, just pass through as a placeholder character
                Ok(AlphabetToken::Unknown('?'))
            }
        }
    }

    /// Convert alphabet tokens to abugida tokens with proper implicit 'a' handling
    pub fn alphabet_to_abugida(tokens: &HubTokenSequence) -> Result<HubTokenSequence, HubError> {
        let mut result = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            match &tokens[i] {
                HubToken::Alphabet(alphabet_token) => {
                    if alphabet_token.is_consonant() {
                        // Check if this consonant is followed by 'a'
                        let has_explicit_a = i + 1 < tokens.len()
                            && matches!(tokens[i + 1], HubToken::Alphabet(AlphabetToken::VowelA));

                        if has_explicit_a {
                            // Consonant + explicit 'a' -> just the consonant (implicit 'a' in abugida)
                            let abugida_consonant = Self::reverse_map_consonant(alphabet_token)?;
                            result.push(HubToken::Abugida(abugida_consonant));
                            i += 2; // Skip both consonant and 'a'
                        } else if i + 1 < tokens.len() && tokens[i + 1].is_vowel() {
                            // Consonant + other vowel -> consonant + vowel sign
                            let abugida_consonant = Self::reverse_map_consonant(alphabet_token)?;
                            result.push(HubToken::Abugida(abugida_consonant));

                            if let HubToken::Alphabet(vowel) = &tokens[i + 1] {
                                let vowel_sign = Self::reverse_map_vowel_to_sign(vowel)?;
                                result.push(HubToken::Abugida(vowel_sign));
                            }
                            i += 2; // Skip both consonant and vowel
                        } else {
                            // Consonant alone -> consonant + virama
                            let abugida_consonant = Self::reverse_map_consonant(alphabet_token)?;
                            result.push(HubToken::Abugida(abugida_consonant));
                            result.push(HubToken::Abugida(AbugidaToken::MarkVirama));
                            i += 1;
                        }
                    } else if alphabet_token.is_vowel() {
                        // Independent vowel
                        let abugida_vowel = Self::reverse_map_vowel(alphabet_token)?;
                        result.push(HubToken::Abugida(abugida_vowel));
                        i += 1;
                    } else if Self::is_extended_token(alphabet_token) {
                        // Nukta consonants - treat as regular consonants
                        // Check if this consonant is followed by 'a'
                        let has_explicit_a = i + 1 < tokens.len()
                            && matches!(tokens[i + 1], HubToken::Alphabet(AlphabetToken::VowelA));

                        if has_explicit_a {
                            // Nukta consonant + explicit 'a' -> just the extended consonant (implicit 'a' in abugida)
                            let abugida_consonant =
                                Self::reverse_map_extended_consonant(alphabet_token)?;
                            result.push(HubToken::Abugida(abugida_consonant));
                            i += 2; // Skip both consonant and 'a'
                        } else if i + 1 < tokens.len() && tokens[i + 1].is_vowel() {
                            // Nukta consonant + other vowel -> extended consonant + vowel sign
                            let abugida_consonant =
                                Self::reverse_map_extended_consonant(alphabet_token)?;
                            result.push(HubToken::Abugida(abugida_consonant));

                            if let HubToken::Alphabet(vowel) = &tokens[i + 1] {
                                let vowel_sign = Self::reverse_map_vowel_to_sign(vowel)?;
                                result.push(HubToken::Abugida(vowel_sign));
                            }
                            i += 2; // Skip both consonant and vowel
                        } else {
                            // Nukta consonant alone -> extended consonant + virama
                            let abugida_consonant =
                                Self::reverse_map_extended_consonant(alphabet_token)?;
                            result.push(HubToken::Abugida(abugida_consonant));
                            result.push(HubToken::Abugida(AbugidaToken::MarkVirama));
                            i += 1;
                        }
                    } else {
                        // Other tokens (marks, digits, unknown)
                        let abugida_token = Self::reverse_map_other_token(alphabet_token)?;
                        result.push(HubToken::Abugida(abugida_token));
                        i += 1;
                    }
                }
                HubToken::Abugida(abugida_token) => {
                    // Already abugida, pass through
                    result.push(HubToken::Abugida(*abugida_token));
                    i += 1;
                }
            }
        }

        Ok(result)
    }

    /// Reverse map extended consonant tokens to their AbugidaToken equivalents
    fn reverse_map_extended_consonant(token: &AlphabetToken) -> Result<AbugidaToken, HubError> {
        match token {
            AlphabetToken::ConsonantQa => Ok(AbugidaToken::ConsonantQa),
            AlphabetToken::ConsonantZa => Ok(AbugidaToken::ConsonantZa),
            AlphabetToken::ConsonantFa => Ok(AbugidaToken::ConsonantFa),
            AlphabetToken::ConsonantGha => Ok(AbugidaToken::ConsonantGha),
            AlphabetToken::ConsonantKha => Ok(AbugidaToken::ConsonantKha),
            AlphabetToken::ConsonantRra => Ok(AbugidaToken::ConsonantRra),
            AlphabetToken::ConsonantRrha => Ok(AbugidaToken::ConsonantRrha),
            AlphabetToken::ConsonantYa => Ok(AbugidaToken::ConsonantYa),
            _ => Err(HubError::MappingNotFound(format!(
                "Not an extended consonant: {:?}",
                token
            ))),
        }
    }

    /// Reverse map consonant tokens
    fn reverse_map_consonant(token: &AlphabetToken) -> Result<AbugidaToken, HubError> {
        match token {
            AlphabetToken::ConsonantK => Ok(AbugidaToken::ConsonantK),
            AlphabetToken::ConsonantKh => Ok(AbugidaToken::ConsonantKh),
            AlphabetToken::ConsonantG => Ok(AbugidaToken::ConsonantG),
            AlphabetToken::ConsonantGh => Ok(AbugidaToken::ConsonantGh),
            AlphabetToken::ConsonantNg => Ok(AbugidaToken::ConsonantNg),
            AlphabetToken::ConsonantC => Ok(AbugidaToken::ConsonantC),
            AlphabetToken::ConsonantCh => Ok(AbugidaToken::ConsonantCh),
            AlphabetToken::ConsonantJ => Ok(AbugidaToken::ConsonantJ),
            AlphabetToken::ConsonantJh => Ok(AbugidaToken::ConsonantJh),
            AlphabetToken::ConsonantNy => Ok(AbugidaToken::ConsonantNy),
            AlphabetToken::ConsonantT => Ok(AbugidaToken::ConsonantT),
            AlphabetToken::ConsonantTh => Ok(AbugidaToken::ConsonantTh),
            AlphabetToken::ConsonantD => Ok(AbugidaToken::ConsonantD),
            AlphabetToken::ConsonantDh => Ok(AbugidaToken::ConsonantDh),
            AlphabetToken::ConsonantN => Ok(AbugidaToken::ConsonantN),
            AlphabetToken::ConsonantTt => Ok(AbugidaToken::ConsonantTt),
            AlphabetToken::ConsonantTth => Ok(AbugidaToken::ConsonantTth),
            AlphabetToken::ConsonantDd => Ok(AbugidaToken::ConsonantDd),
            AlphabetToken::ConsonantDdh => Ok(AbugidaToken::ConsonantDdh),
            AlphabetToken::ConsonantNn => Ok(AbugidaToken::ConsonantNn),
            AlphabetToken::ConsonantP => Ok(AbugidaToken::ConsonantP),
            AlphabetToken::ConsonantPh => Ok(AbugidaToken::ConsonantPh),
            AlphabetToken::ConsonantB => Ok(AbugidaToken::ConsonantB),
            AlphabetToken::ConsonantBh => Ok(AbugidaToken::ConsonantBh),
            AlphabetToken::ConsonantM => Ok(AbugidaToken::ConsonantM),
            AlphabetToken::ConsonantY => Ok(AbugidaToken::ConsonantY),
            AlphabetToken::ConsonantR => Ok(AbugidaToken::ConsonantR),
            AlphabetToken::ConsonantL => Ok(AbugidaToken::ConsonantL),
            AlphabetToken::ConsonantV => Ok(AbugidaToken::ConsonantV),
            AlphabetToken::ConsonantLl => Ok(AbugidaToken::ConsonantLl),
            AlphabetToken::ConsonantSh => Ok(AbugidaToken::ConsonantSh),
            AlphabetToken::ConsonantSs => Ok(AbugidaToken::ConsonantSs),
            AlphabetToken::ConsonantS => Ok(AbugidaToken::ConsonantS),
            AlphabetToken::ConsonantH => Ok(AbugidaToken::ConsonantH),

            // Nukta consonants
            AlphabetToken::ConsonantQa => Ok(AbugidaToken::ConsonantQa),
            AlphabetToken::ConsonantZa => Ok(AbugidaToken::ConsonantZa),
            AlphabetToken::ConsonantFa => Ok(AbugidaToken::ConsonantFa),
            AlphabetToken::ConsonantGha => Ok(AbugidaToken::ConsonantGha),
            AlphabetToken::ConsonantKha => Ok(AbugidaToken::ConsonantKha),
            AlphabetToken::ConsonantRra => Ok(AbugidaToken::ConsonantRra),
            AlphabetToken::ConsonantRrha => Ok(AbugidaToken::ConsonantRrha),
            AlphabetToken::ConsonantYa => Ok(AbugidaToken::ConsonantYa),

            _ => Err(HubError::MappingNotFound(format!(
                "Not a consonant: {:?}",
                token
            ))),
        }
    }

    /// Reverse map vowel tokens
    fn reverse_map_vowel(token: &AlphabetToken) -> Result<AbugidaToken, HubError> {
        match token {
            AlphabetToken::VowelA => Ok(AbugidaToken::VowelA),
            AlphabetToken::VowelAa => Ok(AbugidaToken::VowelAa),
            AlphabetToken::VowelI => Ok(AbugidaToken::VowelI),
            AlphabetToken::VowelIi => Ok(AbugidaToken::VowelIi),
            AlphabetToken::VowelU => Ok(AbugidaToken::VowelU),
            AlphabetToken::VowelUu => Ok(AbugidaToken::VowelUu),
            AlphabetToken::VowelVocalicR => Ok(AbugidaToken::VowelVocalicR),
            AlphabetToken::VowelVocalicRr => Ok(AbugidaToken::VowelVocalicRr),
            AlphabetToken::VowelVocalicL => Ok(AbugidaToken::VowelVocalicL),
            AlphabetToken::VowelVocalicLl => Ok(AbugidaToken::VowelVocalicLl),
            AlphabetToken::VowelE => Ok(AbugidaToken::VowelE),
            AlphabetToken::VowelAi => Ok(AbugidaToken::VowelAi),
            AlphabetToken::VowelO => Ok(AbugidaToken::VowelO),
            AlphabetToken::VowelAu => Ok(AbugidaToken::VowelAu),
            _ => Err(HubError::MappingNotFound(format!(
                "Not a vowel: {:?}",
                token
            ))),
        }
    }

    /// Map vowel tokens to vowel sign tokens
    fn reverse_map_vowel_to_sign(token: &AlphabetToken) -> Result<AbugidaToken, HubError> {
        match token {
            AlphabetToken::VowelAa => Ok(AbugidaToken::VowelSignAa),
            AlphabetToken::VowelI => Ok(AbugidaToken::VowelSignI),
            AlphabetToken::VowelIi => Ok(AbugidaToken::VowelSignIi),
            AlphabetToken::VowelU => Ok(AbugidaToken::VowelSignU),
            AlphabetToken::VowelUu => Ok(AbugidaToken::VowelSignUu),
            AlphabetToken::VowelVocalicR => Ok(AbugidaToken::VowelSignVocalicR),
            AlphabetToken::VowelVocalicRr => Ok(AbugidaToken::VowelSignVocalicRr),
            AlphabetToken::VowelVocalicL => Ok(AbugidaToken::VowelSignVocalicL),
            AlphabetToken::VowelVocalicLl => Ok(AbugidaToken::VowelSignVocalicLl),
            AlphabetToken::VowelE => Ok(AbugidaToken::VowelSignE),
            AlphabetToken::VowelAi => Ok(AbugidaToken::VowelSignAi),
            AlphabetToken::VowelO => Ok(AbugidaToken::VowelSignO),
            AlphabetToken::VowelAu => Ok(AbugidaToken::VowelSignAu),
            _ => Err(HubError::MappingNotFound(format!(
                "No vowel sign for: {:?}",
                token
            ))),
        }
    }

    /// Reverse map other tokens (marks, digits, unknown)
    fn reverse_map_other_token(token: &AlphabetToken) -> Result<AbugidaToken, HubError> {
        match token {
            // Marks
            AlphabetToken::MarkAnusvara => Ok(AbugidaToken::MarkAnusvara),
            AlphabetToken::MarkVisarga => Ok(AbugidaToken::MarkVisarga),
            AlphabetToken::MarkCandrabindu => Ok(AbugidaToken::MarkCandrabindu),
            AlphabetToken::MarkAvagraha => Ok(AbugidaToken::MarkAvagraha),

            // Vedic marks
            AlphabetToken::MarkUdatta => Ok(AbugidaToken::MarkUdatta),
            AlphabetToken::MarkAnudatta => Ok(AbugidaToken::MarkAnudatta),
            AlphabetToken::MarkDoubleSvarita => Ok(AbugidaToken::MarkDoubleSvarita),
            AlphabetToken::MarkTripleSvarita => Ok(AbugidaToken::MarkTripleSvarita),

            // Nukta tokens are handled separately in alphabet_to_abugida conversion

            // Unknown - pass through as character
            AlphabetToken::Unknown(c) => Ok(AbugidaToken::Unknown(*c)),

            // For any other unmapped token, pass it through as an unknown character
            _ => Ok(AbugidaToken::Unknown('?')),
        }
    }
}
