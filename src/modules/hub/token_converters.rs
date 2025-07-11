use super::tokens::{AbugidaToken, AlphabetToken, HubToken, HubTokenSequence};
use rustc_hash::FxHashMap;

pub struct TokenToStringConverter {
    abugida_to_devanagari: FxHashMap<AbugidaToken, &'static str>,
    alphabet_to_iso: FxHashMap<AlphabetToken, &'static str>,
    devanagari_to_abugida: FxHashMap<char, AbugidaToken>,
    iso_to_alphabet: FxHashMap<String, AlphabetToken>,
}

impl TokenToStringConverter {
    pub fn new() -> Self {
        let mut abugida_to_devanagari = FxHashMap::default();
        let mut alphabet_to_iso = FxHashMap::default();
        let mut devanagari_to_abugida = FxHashMap::default();
        let mut iso_to_alphabet = FxHashMap::default();

        // Abugida (Devanagari) token mappings
        abugida_to_devanagari.insert(AbugidaToken::VowelA, "अ");
        abugida_to_devanagari.insert(AbugidaToken::VowelAa, "आ");
        abugida_to_devanagari.insert(AbugidaToken::VowelI, "इ");
        abugida_to_devanagari.insert(AbugidaToken::VowelIi, "ई");
        abugida_to_devanagari.insert(AbugidaToken::VowelU, "उ");
        abugida_to_devanagari.insert(AbugidaToken::VowelUu, "ऊ");
        abugida_to_devanagari.insert(AbugidaToken::VowelVocalicR, "ऋ");
        abugida_to_devanagari.insert(AbugidaToken::VowelVocalicRr, "ॠ");
        abugida_to_devanagari.insert(AbugidaToken::VowelVocalicL, "ऌ");
        abugida_to_devanagari.insert(AbugidaToken::VowelVocalicLl, "ॡ");
        abugida_to_devanagari.insert(AbugidaToken::VowelE, "ए");
        abugida_to_devanagari.insert(AbugidaToken::VowelAi, "ऐ");
        abugida_to_devanagari.insert(AbugidaToken::VowelO, "ओ");
        abugida_to_devanagari.insert(AbugidaToken::VowelAu, "औ");

        // Vowel signs
        abugida_to_devanagari.insert(AbugidaToken::VowelSignAa, "ा");
        abugida_to_devanagari.insert(AbugidaToken::VowelSignI, "ि");
        abugida_to_devanagari.insert(AbugidaToken::VowelSignIi, "ी");
        abugida_to_devanagari.insert(AbugidaToken::VowelSignU, "ु");
        abugida_to_devanagari.insert(AbugidaToken::VowelSignUu, "ू");
        abugida_to_devanagari.insert(AbugidaToken::VowelSignVocalicR, "ृ");
        abugida_to_devanagari.insert(AbugidaToken::VowelSignVocalicRr, "ॄ");
        abugida_to_devanagari.insert(AbugidaToken::VowelSignVocalicL, "ॢ");
        abugida_to_devanagari.insert(AbugidaToken::VowelSignVocalicLl, "ॣ");
        abugida_to_devanagari.insert(AbugidaToken::VowelSignE, "े");
        abugida_to_devanagari.insert(AbugidaToken::VowelSignAi, "ै");
        abugida_to_devanagari.insert(AbugidaToken::VowelSignO, "ो");
        abugida_to_devanagari.insert(AbugidaToken::VowelSignAu, "ौ");

        // Consonants
        abugida_to_devanagari.insert(AbugidaToken::ConsonantK, "क");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantKh, "ख");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantG, "ग");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantGh, "घ");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantNg, "ङ");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantC, "च");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantCh, "छ");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantJ, "ज");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantJh, "झ");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantNy, "ञ");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantT, "ट");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantTh, "ठ");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantD, "ड");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantDh, "ढ");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantN, "ण");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantTt, "त");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantTth, "थ");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantDd, "द");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantDdh, "ध");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantNn, "न");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantP, "प");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantPh, "फ");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantB, "ब");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantBh, "भ");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantM, "म");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantY, "य");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantR, "र");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantL, "ल");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantV, "व");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantLl, "ळ");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantSh, "श");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantSs, "ष");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantS, "स");
        abugida_to_devanagari.insert(AbugidaToken::ConsonantH, "ह");

        // Marks
        abugida_to_devanagari.insert(AbugidaToken::MarkAnusvara, "ं");
        abugida_to_devanagari.insert(AbugidaToken::MarkVisarga, "ः");
        abugida_to_devanagari.insert(AbugidaToken::MarkCandrabindu, "ँ");
        abugida_to_devanagari.insert(AbugidaToken::MarkNukta, "़");
        abugida_to_devanagari.insert(AbugidaToken::MarkVirama, "्");
        abugida_to_devanagari.insert(AbugidaToken::MarkAvagraha, "ऽ");

        // Special/Vedic marks
        abugida_to_devanagari.insert(AbugidaToken::MarkUdatta, "॑");
        abugida_to_devanagari.insert(AbugidaToken::MarkAnudatta, "॒");
        abugida_to_devanagari.insert(AbugidaToken::MarkDoubleSvarita, "᳚");
        abugida_to_devanagari.insert(AbugidaToken::MarkTripleSvarita, "᳛");

        // Digits
        abugida_to_devanagari.insert(AbugidaToken::Digit0, "०");
        abugida_to_devanagari.insert(AbugidaToken::Digit1, "१");
        abugida_to_devanagari.insert(AbugidaToken::Digit2, "२");
        abugida_to_devanagari.insert(AbugidaToken::Digit3, "३");
        abugida_to_devanagari.insert(AbugidaToken::Digit4, "४");
        abugida_to_devanagari.insert(AbugidaToken::Digit5, "५");
        abugida_to_devanagari.insert(AbugidaToken::Digit6, "६");
        abugida_to_devanagari.insert(AbugidaToken::Digit7, "७");
        abugida_to_devanagari.insert(AbugidaToken::Digit8, "८");
        abugida_to_devanagari.insert(AbugidaToken::Digit9, "९");

        // Build reverse mapping for Devanagari -> Abugida
        for (&token, &char_str) in &abugida_to_devanagari {
            if let Some(c) = char_str.chars().next() {
                devanagari_to_abugida.insert(c, token);
            }
        }

        // Alphabet (ISO) token mappings
        alphabet_to_iso.insert(AlphabetToken::VowelA, "a");
        alphabet_to_iso.insert(AlphabetToken::VowelAa, "ā");
        alphabet_to_iso.insert(AlphabetToken::VowelI, "i");
        alphabet_to_iso.insert(AlphabetToken::VowelIi, "ī");
        alphabet_to_iso.insert(AlphabetToken::VowelU, "u");
        alphabet_to_iso.insert(AlphabetToken::VowelUu, "ū");
        alphabet_to_iso.insert(AlphabetToken::VowelVocalicR, "r̥");
        alphabet_to_iso.insert(AlphabetToken::VowelVocalicRr, "r̥̄");
        alphabet_to_iso.insert(AlphabetToken::VowelVocalicL, "l̥");
        alphabet_to_iso.insert(AlphabetToken::VowelVocalicLl, "l̥̄");  // This fixes the ambiguity!
        alphabet_to_iso.insert(AlphabetToken::VowelE, "e");
        alphabet_to_iso.insert(AlphabetToken::VowelAi, "ai");
        alphabet_to_iso.insert(AlphabetToken::VowelO, "o");
        alphabet_to_iso.insert(AlphabetToken::VowelAu, "au");

        // Consonants
        alphabet_to_iso.insert(AlphabetToken::ConsonantK, "k");
        alphabet_to_iso.insert(AlphabetToken::ConsonantKh, "kh");
        alphabet_to_iso.insert(AlphabetToken::ConsonantG, "g");
        alphabet_to_iso.insert(AlphabetToken::ConsonantGh, "gh");
        alphabet_to_iso.insert(AlphabetToken::ConsonantNg, "ṅ");
        alphabet_to_iso.insert(AlphabetToken::ConsonantC, "c");
        alphabet_to_iso.insert(AlphabetToken::ConsonantCh, "ch");
        alphabet_to_iso.insert(AlphabetToken::ConsonantJ, "j");
        alphabet_to_iso.insert(AlphabetToken::ConsonantJh, "jh");
        alphabet_to_iso.insert(AlphabetToken::ConsonantNy, "ñ");
        alphabet_to_iso.insert(AlphabetToken::ConsonantT, "ṭ");
        alphabet_to_iso.insert(AlphabetToken::ConsonantTh, "ṭh");
        alphabet_to_iso.insert(AlphabetToken::ConsonantD, "ḍ");
        alphabet_to_iso.insert(AlphabetToken::ConsonantDh, "ḍh");
        alphabet_to_iso.insert(AlphabetToken::ConsonantN, "ṇ");
        alphabet_to_iso.insert(AlphabetToken::ConsonantTt, "t");
        alphabet_to_iso.insert(AlphabetToken::ConsonantTth, "th");
        alphabet_to_iso.insert(AlphabetToken::ConsonantDd, "d");
        alphabet_to_iso.insert(AlphabetToken::ConsonantDdh, "dh");
        alphabet_to_iso.insert(AlphabetToken::ConsonantNn, "n");
        alphabet_to_iso.insert(AlphabetToken::ConsonantP, "p");
        alphabet_to_iso.insert(AlphabetToken::ConsonantPh, "ph");
        alphabet_to_iso.insert(AlphabetToken::ConsonantB, "b");
        alphabet_to_iso.insert(AlphabetToken::ConsonantBh, "bh");
        alphabet_to_iso.insert(AlphabetToken::ConsonantM, "m");
        alphabet_to_iso.insert(AlphabetToken::ConsonantY, "y");
        alphabet_to_iso.insert(AlphabetToken::ConsonantR, "r");
        alphabet_to_iso.insert(AlphabetToken::ConsonantL, "l");
        alphabet_to_iso.insert(AlphabetToken::ConsonantV, "v");
        alphabet_to_iso.insert(AlphabetToken::ConsonantLl, "ḷ");
        alphabet_to_iso.insert(AlphabetToken::ConsonantSh, "ś");
        alphabet_to_iso.insert(AlphabetToken::ConsonantSs, "ṣ");
        alphabet_to_iso.insert(AlphabetToken::ConsonantS, "s");
        alphabet_to_iso.insert(AlphabetToken::ConsonantH, "h");

        // Marks
        alphabet_to_iso.insert(AlphabetToken::MarkAnusvara, "ṁ");
        alphabet_to_iso.insert(AlphabetToken::MarkVisarga, "ḥ");
        alphabet_to_iso.insert(AlphabetToken::MarkCandrabindu, "m̐");
        alphabet_to_iso.insert(AlphabetToken::MarkAvagraha, "'");

        // Special combinations
        alphabet_to_iso.insert(AlphabetToken::SpecialKs, "kṣ");
        alphabet_to_iso.insert(AlphabetToken::SpecialJn, "jñ");

        // Nukta characters
        alphabet_to_iso.insert(AlphabetToken::ConsonantQa, "qa");
        alphabet_to_iso.insert(AlphabetToken::ConsonantZa, "za");
        alphabet_to_iso.insert(AlphabetToken::ConsonantFa, "fa");
        alphabet_to_iso.insert(AlphabetToken::ConsonantGha, "ġa");
        alphabet_to_iso.insert(AlphabetToken::ConsonantKha, "ḵẖa");
        alphabet_to_iso.insert(AlphabetToken::ConsonantRra, "ṛa");
        alphabet_to_iso.insert(AlphabetToken::ConsonantRrha, "ṛha");
        alphabet_to_iso.insert(AlphabetToken::ConsonantYa, "ẏa");

        // Digits
        alphabet_to_iso.insert(AlphabetToken::Digit0, "0");
        alphabet_to_iso.insert(AlphabetToken::Digit1, "1");
        alphabet_to_iso.insert(AlphabetToken::Digit2, "2");
        alphabet_to_iso.insert(AlphabetToken::Digit3, "3");
        alphabet_to_iso.insert(AlphabetToken::Digit4, "4");
        alphabet_to_iso.insert(AlphabetToken::Digit5, "5");
        alphabet_to_iso.insert(AlphabetToken::Digit6, "6");
        alphabet_to_iso.insert(AlphabetToken::Digit7, "7");
        alphabet_to_iso.insert(AlphabetToken::Digit8, "8");
        alphabet_to_iso.insert(AlphabetToken::Digit9, "9");

        // Build reverse mapping for ISO -> Alphabet
        for (token, &string) in &alphabet_to_iso {
            iso_to_alphabet.insert(string.to_string(), token.clone());
        }

        // Special cases for ambiguous mappings
        // Multiple inputs can map to the same token, but token always outputs preferred form
        iso_to_alphabet.insert("RR".to_string(), AlphabetToken::VowelVocalicRr);  // Harvard-Kyoto
        iso_to_alphabet.insert("lR".to_string(), AlphabetToken::VowelVocalicL);   // Harvard-Kyoto  
        iso_to_alphabet.insert("lRR".to_string(), AlphabetToken::VowelVocalicLl); // Harvard-Kyoto - FIXES THE AMBIGUITY!

        Self {
            abugida_to_devanagari,
            alphabet_to_iso,
            devanagari_to_abugida,
            iso_to_alphabet,
        }
    }

    pub fn tokens_to_devanagari(&self, tokens: &HubTokenSequence) -> String {
        let mut result = String::new();
        for token in tokens {
            match token {
                HubToken::Abugida(abugida_token) => {
                    if let Some(&text) = self.abugida_to_devanagari.get(abugida_token) {
                        result.push_str(text);
                    } else if let AbugidaToken::Unknown(c) = abugida_token {
                        result.push(*c);
                    }
                }
                HubToken::Alphabet(_) => {
                    // This shouldn't happen - Alphabet tokens should be converted to Abugida first
                    panic!("Cannot convert Alphabet token directly to Devanagari");
                }
            }
        }
        result
    }

    pub fn tokens_to_iso(&self, tokens: &HubTokenSequence) -> String {
        let mut result = String::new();
        for token in tokens {
            match token {
                HubToken::Alphabet(alphabet_token) => {
                    if let Some(&text) = self.alphabet_to_iso.get(alphabet_token) {
                        result.push_str(text);
                    // No more Literal variants - unknown text handled separately
                    }
                }
                HubToken::Abugida(_) => {
                    // This shouldn't happen - Abugida tokens should be converted to Alphabet first
                    panic!("Cannot convert Abugida token directly to ISO");
                }
            }
        }
        result
    }

    pub fn devanagari_to_tokens(&self, text: &str) -> HubTokenSequence {
        let mut tokens = Vec::new();
        for c in text.chars() {
            if let Some(&token) = self.devanagari_to_abugida.get(&c) {
                tokens.push(HubToken::Abugida(token));
            } else {
                tokens.push(HubToken::Abugida(AbugidaToken::Unknown(c)));
            }
        }
        tokens
    }

    pub fn iso_to_tokens(&self, text: &str) -> HubTokenSequence {
        let mut tokens = Vec::new();
        let mut chars = text.chars().peekable();
        
        while let Some(c) = chars.next() {
            // Try to match multi-character sequences first
            let mut matched = false;
            
            // Try 3-character sequences
            if let Some(c2) = chars.peek().copied() {
                let mut remaining_chars = chars.clone();
                remaining_chars.next(); // consume c2
                if let Some(c3) = remaining_chars.peek().copied() {
                    let three_char: String = [c, c2, c3].iter().collect();
                    if let Some(token) = self.iso_to_alphabet.get(&three_char) {
                        tokens.push(HubToken::Alphabet(token.clone()));
                        chars.next(); // consume c2
                        chars.next(); // consume c3
                        matched = true;
                    }
                }
            }
            
            // Try 2-character sequences if 3-char didn't match
            if !matched {
                if let Some(c2) = chars.peek().copied() {
                    let two_char: String = [c, c2].iter().collect();
                    if let Some(token) = self.iso_to_alphabet.get(&two_char) {
                        tokens.push(HubToken::Alphabet(token.clone()));
                        chars.next(); // consume c2
                        matched = true;
                    }
                }
            }
            
            // Try single character
            if !matched {
                let single_char = c.to_string();
                if let Some(token) = self.iso_to_alphabet.get(&single_char) {
                    tokens.push(HubToken::Alphabet(token.clone()));
                } else {
                    // Unknown character - for now skip, or we could have a dedicated Unknown token
                    // TODO: Handle unknown characters properly
                }
            }
        }
        
        tokens
    }
}

impl Default for TokenToStringConverter {
    fn default() -> Self {
        Self::new()
    }
}