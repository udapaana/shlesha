use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AbugidaToken {
    // Vowels (independent)
    VowelA,
    VowelAa,
    VowelI,
    VowelIi,
    VowelU,
    VowelUu,
    VowelVocalicR,
    VowelVocalicRr,
    VowelVocalicL,
    VowelVocalicLl,
    VowelE,
    VowelAi,
    VowelO,
    VowelAu,
    
    // Vowel signs (dependent)
    VowelSignAa,
    VowelSignI,
    VowelSignIi,
    VowelSignU,
    VowelSignUu,
    VowelSignVocalicR,
    VowelSignVocalicRr,
    VowelSignVocalicL,
    VowelSignVocalicLl,
    VowelSignE,
    VowelSignAi,
    VowelSignO,
    VowelSignAu,
    
    // Consonants - Velar
    ConsonantK,
    ConsonantKh,
    ConsonantG,
    ConsonantGh,
    ConsonantNg,
    
    // Consonants - Palatal
    ConsonantC,
    ConsonantCh,
    ConsonantJ,
    ConsonantJh,
    ConsonantNy,
    
    // Consonants - Retroflex
    ConsonantT,
    ConsonantTh,
    ConsonantD,
    ConsonantDh,
    ConsonantN,
    
    // Consonants - Dental
    ConsonantTt,
    ConsonantTth,
    ConsonantDd,
    ConsonantDdh,
    ConsonantNn,
    
    // Consonants - Labial
    ConsonantP,
    ConsonantPh,
    ConsonantB,
    ConsonantBh,
    ConsonantM,
    
    // Consonants - Semivowels and liquids
    ConsonantY,
    ConsonantR,
    ConsonantL,
    ConsonantV,
    ConsonantLl, // ḷ
    
    // Consonants - Sibilants and aspirate
    ConsonantSh,
    ConsonantSs,
    ConsonantS,
    ConsonantH,
    
    // Marks
    MarkAnusvara,
    MarkVisarga,
    MarkCandrabindu,
    MarkNukta,
    MarkVirama,
    MarkAvagraha,
    
    // Special/Vedic marks
    MarkUdatta,
    MarkAnudatta,
    MarkDoubleSvarita,
    MarkTripleSvarita,
    
    // Nukta consonants (treated as regular consonants)
    ConsonantQa,  // क़ (qa)
    ConsonantZa,  // ज़ (za)  
    ConsonantFa,  // फ़ (fa)
    ConsonantGha, // ग़ (ġa)
    ConsonantKha, // ख़ (ḵha) 
    ConsonantRra, // ड़ (ṛa)
    ConsonantRrha, // ढ़ (ṛha)
    ConsonantYa,  // य़ (ẏa)
    
    // Digits
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    
    // Unknown character passthrough
    Unknown(char),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlphabetToken {
    // Vowels
    VowelA,
    VowelAa,
    VowelI,
    VowelIi,
    VowelU,
    VowelUu,
    VowelVocalicR,
    VowelVocalicRr,
    VowelVocalicL,
    VowelVocalicLl,
    VowelE,
    VowelAi,
    VowelO,
    VowelAu,
    
    // Consonants - Velar
    ConsonantK,
    ConsonantKh,
    ConsonantG,
    ConsonantGh,
    ConsonantNg,
    
    // Consonants - Palatal
    ConsonantC,
    ConsonantCh,
    ConsonantJ,
    ConsonantJh,
    ConsonantNy,
    
    // Consonants - Retroflex
    ConsonantT,
    ConsonantTh,
    ConsonantD,
    ConsonantDh,
    ConsonantN,
    
    // Consonants - Dental
    ConsonantTt,
    ConsonantTth,
    ConsonantDd,
    ConsonantDdh,
    ConsonantNn,
    
    // Consonants - Labial
    ConsonantP,
    ConsonantPh,
    ConsonantB,
    ConsonantBh,
    ConsonantM,
    
    // Consonants - Semivowels and liquids
    ConsonantY,
    ConsonantR,
    ConsonantL,
    ConsonantV,
    ConsonantLl, // ḷ
    
    // Consonants - Sibilants and aspirate
    ConsonantSh,
    ConsonantSs,
    ConsonantS,
    ConsonantH,
    
    // Marks
    MarkAnusvara,
    MarkVisarga,
    MarkCandrabindu,
    MarkAvagraha,
    
    // Special/Vedic marks
    MarkUdatta,
    MarkAnudatta,
    MarkDoubleSvarita,
    MarkTripleSvarita,
    
    // Special combinations
    SpecialKs,  // kṣ
    SpecialJn,  // jñ
    
    // Nukta consonants (treated as regular consonants)
    ConsonantQa,  // qa
    ConsonantZa,  // za
    ConsonantFa,  // fa
    ConsonantGha, // ġa
    ConsonantKha, // ḵha
    ConsonantRra, // ṛa
    ConsonantRrha, // ṛha
    ConsonantYa,  // ẏa
    
    // Digits
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    
    // Unknown character passthrough
    Unknown(char),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HubToken {
    Abugida(AbugidaToken),
    Alphabet(AlphabetToken),
}

impl HubToken {
    pub fn is_vowel(&self) -> bool {
        match self {
            HubToken::Abugida(token) => matches!(token, 
                AbugidaToken::VowelA | AbugidaToken::VowelAa | AbugidaToken::VowelI | 
                AbugidaToken::VowelIi | AbugidaToken::VowelU | AbugidaToken::VowelUu |
                AbugidaToken::VowelVocalicR | AbugidaToken::VowelVocalicRr | 
                AbugidaToken::VowelVocalicL | AbugidaToken::VowelVocalicLl |
                AbugidaToken::VowelE | AbugidaToken::VowelAi | AbugidaToken::VowelO | AbugidaToken::VowelAu
            ),
            HubToken::Alphabet(token) => matches!(token,
                AlphabetToken::VowelA | AlphabetToken::VowelAa | AlphabetToken::VowelI |
                AlphabetToken::VowelIi | AlphabetToken::VowelU | AlphabetToken::VowelUu |
                AlphabetToken::VowelVocalicR | AlphabetToken::VowelVocalicRr |
                AlphabetToken::VowelVocalicL | AlphabetToken::VowelVocalicLl |
                AlphabetToken::VowelE | AlphabetToken::VowelAi | AlphabetToken::VowelO | AlphabetToken::VowelAu
            ),
        }
    }
    
    pub fn is_consonant(&self) -> bool {
        match self {
            HubToken::Abugida(token) => matches!(token,
                AbugidaToken::ConsonantK | AbugidaToken::ConsonantKh | AbugidaToken::ConsonantG |
                AbugidaToken::ConsonantGh | AbugidaToken::ConsonantNg | AbugidaToken::ConsonantC |
                AbugidaToken::ConsonantCh | AbugidaToken::ConsonantJ | AbugidaToken::ConsonantJh |
                AbugidaToken::ConsonantNy | AbugidaToken::ConsonantT | AbugidaToken::ConsonantTh |
                AbugidaToken::ConsonantD | AbugidaToken::ConsonantDh | AbugidaToken::ConsonantN |
                AbugidaToken::ConsonantTt | AbugidaToken::ConsonantTth | AbugidaToken::ConsonantDd |
                AbugidaToken::ConsonantDdh | AbugidaToken::ConsonantNn | AbugidaToken::ConsonantP |
                AbugidaToken::ConsonantPh | AbugidaToken::ConsonantB | AbugidaToken::ConsonantBh |
                AbugidaToken::ConsonantM | AbugidaToken::ConsonantY | AbugidaToken::ConsonantR |
                AbugidaToken::ConsonantL | AbugidaToken::ConsonantV | AbugidaToken::ConsonantLl |
                AbugidaToken::ConsonantSh | AbugidaToken::ConsonantSs | AbugidaToken::ConsonantS |
                AbugidaToken::ConsonantH | AbugidaToken::ConsonantQa | AbugidaToken::ConsonantZa |
                AbugidaToken::ConsonantFa | AbugidaToken::ConsonantGha | AbugidaToken::ConsonantKha |
                AbugidaToken::ConsonantRra | AbugidaToken::ConsonantRrha | AbugidaToken::ConsonantYa
            ),
            HubToken::Alphabet(token) => matches!(token,
                AlphabetToken::ConsonantK | AlphabetToken::ConsonantKh | AlphabetToken::ConsonantG |
                AlphabetToken::ConsonantGh | AlphabetToken::ConsonantNg | AlphabetToken::ConsonantC |
                AlphabetToken::ConsonantCh | AlphabetToken::ConsonantJ | AlphabetToken::ConsonantJh |
                AlphabetToken::ConsonantNy | AlphabetToken::ConsonantT | AlphabetToken::ConsonantTh |
                AlphabetToken::ConsonantD | AlphabetToken::ConsonantDh | AlphabetToken::ConsonantN |
                AlphabetToken::ConsonantTt | AlphabetToken::ConsonantTth | AlphabetToken::ConsonantDd |
                AlphabetToken::ConsonantDdh | AlphabetToken::ConsonantNn | AlphabetToken::ConsonantP |
                AlphabetToken::ConsonantPh | AlphabetToken::ConsonantB | AlphabetToken::ConsonantBh |
                AlphabetToken::ConsonantM | AlphabetToken::ConsonantY | AlphabetToken::ConsonantR |
                AlphabetToken::ConsonantL | AlphabetToken::ConsonantV | AlphabetToken::ConsonantLl |
                AlphabetToken::ConsonantSh | AlphabetToken::ConsonantSs | AlphabetToken::ConsonantS |
                AlphabetToken::ConsonantH | AlphabetToken::ConsonantQa | AlphabetToken::ConsonantZa |
                AlphabetToken::ConsonantFa | AlphabetToken::ConsonantGha | AlphabetToken::ConsonantKha |
                AlphabetToken::ConsonantRra | AlphabetToken::ConsonantRrha | AlphabetToken::ConsonantYa
            ),
        }
    }
    
    pub fn is_mark(&self) -> bool {
        match self {
            HubToken::Abugida(token) => matches!(token,
                AbugidaToken::MarkAnusvara | AbugidaToken::MarkVisarga | AbugidaToken::MarkCandrabindu |
                AbugidaToken::MarkNukta | AbugidaToken::MarkVirama | AbugidaToken::MarkAvagraha |
                AbugidaToken::MarkUdatta | AbugidaToken::MarkAnudatta | AbugidaToken::MarkDoubleSvarita |
                AbugidaToken::MarkTripleSvarita
            ),
            HubToken::Alphabet(token) => matches!(token,
                AlphabetToken::MarkAnusvara | AlphabetToken::MarkVisarga | AlphabetToken::MarkCandrabindu |
                AlphabetToken::MarkAvagraha
            ),
        }
    }
    
    pub fn is_vowel_sign(&self) -> bool {
        match self {
            HubToken::Abugida(token) => matches!(token,
                AbugidaToken::VowelSignAa | AbugidaToken::VowelSignI | AbugidaToken::VowelSignIi |
                AbugidaToken::VowelSignU | AbugidaToken::VowelSignUu | AbugidaToken::VowelSignVocalicR |
                AbugidaToken::VowelSignVocalicRr | AbugidaToken::VowelSignVocalicL | AbugidaToken::VowelSignVocalicLl |
                AbugidaToken::VowelSignE | AbugidaToken::VowelSignAi | AbugidaToken::VowelSignO | AbugidaToken::VowelSignAu
            ),
            HubToken::Alphabet(_) => false, // Alphabet tokens don't have vowel signs
        }
    }
    
    pub fn is_virama(&self) -> bool {
        match self {
            HubToken::Abugida(AbugidaToken::MarkVirama) => true,
            _ => false,
        }
    }
}

impl AlphabetToken {
    pub fn is_vowel(&self) -> bool {
        matches!(self,
            AlphabetToken::VowelA | AlphabetToken::VowelAa | AlphabetToken::VowelI |
            AlphabetToken::VowelIi | AlphabetToken::VowelU | AlphabetToken::VowelUu |
            AlphabetToken::VowelVocalicR | AlphabetToken::VowelVocalicRr |
            AlphabetToken::VowelVocalicL | AlphabetToken::VowelVocalicLl |
            AlphabetToken::VowelE | AlphabetToken::VowelAi | AlphabetToken::VowelO | AlphabetToken::VowelAu
        )
    }
    
    pub fn is_consonant(&self) -> bool {
        matches!(self,
            AlphabetToken::ConsonantK | AlphabetToken::ConsonantKh | AlphabetToken::ConsonantG |
            AlphabetToken::ConsonantGh | AlphabetToken::ConsonantNg | AlphabetToken::ConsonantC |
            AlphabetToken::ConsonantCh | AlphabetToken::ConsonantJ | AlphabetToken::ConsonantJh |
            AlphabetToken::ConsonantNy | AlphabetToken::ConsonantT | AlphabetToken::ConsonantTh |
            AlphabetToken::ConsonantD | AlphabetToken::ConsonantDh | AlphabetToken::ConsonantN |
            AlphabetToken::ConsonantTt | AlphabetToken::ConsonantTth | AlphabetToken::ConsonantDd |
            AlphabetToken::ConsonantDdh | AlphabetToken::ConsonantNn | AlphabetToken::ConsonantP |
            AlphabetToken::ConsonantPh | AlphabetToken::ConsonantB | AlphabetToken::ConsonantBh |
            AlphabetToken::ConsonantM | AlphabetToken::ConsonantY | AlphabetToken::ConsonantR |
            AlphabetToken::ConsonantL | AlphabetToken::ConsonantV | AlphabetToken::ConsonantLl |
            AlphabetToken::ConsonantSh | AlphabetToken::ConsonantSs | AlphabetToken::ConsonantS |
            AlphabetToken::ConsonantH
        )
    }
}

pub type HubTokenSequence = Vec<HubToken>;