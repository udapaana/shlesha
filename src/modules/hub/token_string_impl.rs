// Manual implementation of Display and FromStr for token enums
// This provides bijective string representation for all tokens

use super::tokens::{AbugidaToken, AlphabetToken};
use std::fmt;
use std::str::FromStr;

// Macro to implement Display and FromStr for token enums
macro_rules! impl_token_string {
    ($enum_name:ident, $($variant:ident),* $(,)?) => {
        impl fmt::Display for $enum_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $($enum_name::$variant => write!(f, stringify!($variant)),)*
                    $enum_name::Unknown(c) => write!(f, "Unknown({})", c),
                }
            }
        }

        impl FromStr for $enum_name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(stringify!($variant) => Ok($enum_name::$variant),)*
                    s if s.starts_with("Unknown(") && s.ends_with(')') => {
                        let inner = &s[8..s.len()-1];
                        if let Some(c) = inner.chars().next() {
                            if inner.chars().count() == 1 {
                                return Ok($enum_name::Unknown(c));
                            }
                        }
                        Err(format!("Invalid Unknown token: {}", s))
                    }
                    _ => Err(format!("Unknown {}: {}", stringify!($enum_name), s)),
                }
            }
        }
    };
}

// Implement for AbugidaToken
impl_token_string!(
    AbugidaToken,
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
    ConsonantK,
    ConsonantKh,
    ConsonantG,
    ConsonantGh,
    ConsonantNg,
    ConsonantC,
    ConsonantCh,
    ConsonantJ,
    ConsonantJh,
    ConsonantNy,
    ConsonantT,
    ConsonantTh,
    ConsonantD,
    ConsonantDh,
    ConsonantN,
    ConsonantTt,
    ConsonantTth,
    ConsonantDd,
    ConsonantDdh,
    ConsonantNn,
    ConsonantP,
    ConsonantPh,
    ConsonantB,
    ConsonantBh,
    ConsonantM,
    ConsonantY,
    ConsonantR,
    ConsonantL,
    ConsonantV,
    ConsonantLl,
    ConsonantSh,
    ConsonantSs,
    ConsonantS,
    ConsonantH,
    MarkAnusvara,
    MarkVisarga,
    MarkCandrabindu,
    MarkNukta,
    MarkVirama,
    MarkAvagraha,
    MarkUdatta,
    MarkAnudatta,
    MarkDoubleSvarita,
    MarkTripleSvarita,
    ConsonantQa,
    ConsonantZa,
    ConsonantFa,
    ConsonantGha,
    ConsonantKha,
    ConsonantRra,
    ConsonantRrha,
    ConsonantYa,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9
);

// Implement for AlphabetToken
impl_token_string!(
    AlphabetToken,
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
    ConsonantK,
    ConsonantKh,
    ConsonantG,
    ConsonantGh,
    ConsonantNg,
    ConsonantC,
    ConsonantCh,
    ConsonantJ,
    ConsonantJh,
    ConsonantNy,
    ConsonantT,
    ConsonantTh,
    ConsonantD,
    ConsonantDh,
    ConsonantN,
    ConsonantTt,
    ConsonantTth,
    ConsonantDd,
    ConsonantDdh,
    ConsonantNn,
    ConsonantP,
    ConsonantPh,
    ConsonantB,
    ConsonantBh,
    ConsonantM,
    ConsonantY,
    ConsonantR,
    ConsonantL,
    ConsonantV,
    ConsonantLl,
    ConsonantSh,
    ConsonantSs,
    ConsonantS,
    ConsonantH,
    MarkAnusvara,
    MarkVisarga,
    MarkCandrabindu,
    MarkAvagraha,
    MarkUdatta,
    MarkAnudatta,
    MarkDoubleSvarita,
    MarkTripleSvarita,
    SpecialKs,
    SpecialJn,
    ConsonantQa,
    ConsonantZa,
    ConsonantFa,
    ConsonantGha,
    ConsonantKha,
    ConsonantRra,
    ConsonantRrha,
    ConsonantYa,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abugida_token_roundtrip() {
        let token = AbugidaToken::ConsonantLl;
        let string = token.to_string();
        assert_eq!(string, "ConsonantLl");

        let parsed = string.parse::<AbugidaToken>().unwrap();
        assert_eq!(parsed, token);
    }

    #[test]
    fn test_alphabet_token_roundtrip() {
        let token = AlphabetToken::SpecialKs;
        let string = token.to_string();
        assert_eq!(string, "SpecialKs");

        let parsed = string.parse::<AlphabetToken>().unwrap();
        assert_eq!(parsed, token);
    }

    #[test]
    fn test_unknown_token_roundtrip() {
        let token = AbugidaToken::Unknown('x');
        let string = token.to_string();
        assert_eq!(string, "Unknown(x)");

        let parsed = string.parse::<AbugidaToken>().unwrap();
        assert_eq!(parsed, token);
    }
}
