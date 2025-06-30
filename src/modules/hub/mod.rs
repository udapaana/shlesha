use crate::modules::core::unknown_handler::{TransliterationMetadata, UnknownToken};
use rustc_hash::FxHashMap;
use thiserror::Error;
use unicode_normalization::UnicodeNormalization;

#[derive(Error, Debug, Clone)]
pub enum HubError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Mapping not found: {0}")]
    MappingNotFound(String),
    #[error("Conversion failed: {0}")]
    ConversionFailed(String),
}

/// Hub format representation - used for both input and output
#[derive(Debug, Clone, PartialEq)]
pub enum HubFormat {
    Devanagari(String),
    Iso(String),
}

impl HubFormat {
    /// Extract the string content regardless of format
    pub fn as_str(&self) -> &str {
        match self {
            HubFormat::Devanagari(s) => s,
            HubFormat::Iso(s) => s,
        }
    }

    /// Check if this is Devanagari format
    pub fn is_devanagari(&self) -> bool {
        matches!(self, HubFormat::Devanagari(_))
    }

    /// Check if this is ISO format
    pub fn is_iso(&self) -> bool {
        matches!(self, HubFormat::Iso(_))
    }
}

// Type aliases for backward compatibility
pub type HubInput = HubFormat;
pub type HubOutput = HubFormat;

#[derive(Debug, Clone)]
pub struct HubResult {
    pub output: HubOutput,
    pub metadata: Option<TransliterationMetadata>,
}

/// Core hub trait for Devanagari ↔ ISO-15919 bidirectional conversion
pub trait HubTrait {
    /// Convert Devanagari to ISO-15919
    fn deva_to_iso(&self, input: &str) -> Result<HubOutput, HubError>;

    /// Convert ISO-15919 to Devanagari
    fn iso_to_deva(&self, input: &str) -> Result<HubOutput, HubError>;

    /// Convert with metadata collection
    fn deva_to_iso_with_metadata(&self, input: &str) -> Result<HubResult, HubError>;
    fn iso_to_deva_with_metadata(&self, input: &str) -> Result<HubResult, HubError>;

    /// Generic conversion between hub formats
    fn convert(&self, input: &HubInput) -> Result<HubOutput, HubError> {
        match input {
            HubFormat::Devanagari(text) => self.deva_to_iso(text),
            HubFormat::Iso(text) => self.iso_to_deva(text),
        }
    }

    /// Generic conversion with metadata
    fn convert_with_metadata(&self, input: &HubInput) -> Result<HubResult, HubError> {
        match input {
            HubFormat::Devanagari(text) => self.deva_to_iso_with_metadata(text),
            HubFormat::Iso(text) => self.iso_to_deva_with_metadata(text),
        }
    }
}

/// Central hub implementing Devanagari ↔ ISO-15919 conversion
pub struct Hub {
    deva_to_iso_map: FxHashMap<char, &'static str>,
    iso_to_deva_map: FxHashMap<&'static str, char>,
}

impl Hub {
    pub fn new() -> Self {
        let mut deva_to_iso = FxHashMap::default();
        let mut iso_to_deva = FxHashMap::default();

        // Core vowels
        deva_to_iso.insert('अ', "a");
        deva_to_iso.insert('आ', "ā");
        deva_to_iso.insert('इ', "i");
        deva_to_iso.insert('ई', "ī");
        deva_to_iso.insert('उ', "u");
        deva_to_iso.insert('ऊ', "ū");
        deva_to_iso.insert('ऋ', "r̥");
        deva_to_iso.insert('ऌ', "l̥"); // vocalic L
        deva_to_iso.insert('ए', "e");
        deva_to_iso.insert('ऐ', "ai"); // AI
        deva_to_iso.insert('ओ', "o");
        deva_to_iso.insert('औ', "au");
        // Additional vowels
        deva_to_iso.insert('ॠ', "r̥̄"); // vocalic RR
        deva_to_iso.insert('ॡ', "l̥̄"); // vocalic LL

        // Core consonants
        deva_to_iso.insert('क', "ka");
        deva_to_iso.insert('ख', "kha");
        deva_to_iso.insert('ग', "ga");
        deva_to_iso.insert('घ', "gha");
        deva_to_iso.insert('ङ', "ṅa");
        deva_to_iso.insert('च', "ca");
        deva_to_iso.insert('छ', "cha");
        deva_to_iso.insert('ज', "ja");
        deva_to_iso.insert('झ', "jha");
        deva_to_iso.insert('ञ', "ña");
        deva_to_iso.insert('ट', "ṭa");
        deva_to_iso.insert('ठ', "ṭha");
        deva_to_iso.insert('ड', "ḍa");
        deva_to_iso.insert('ढ', "ḍha");
        deva_to_iso.insert('ण', "ṇa");
        deva_to_iso.insert('त', "ta");
        deva_to_iso.insert('थ', "tha");
        deva_to_iso.insert('द', "da");
        deva_to_iso.insert('ध', "dha");
        deva_to_iso.insert('न', "na");
        deva_to_iso.insert('प', "pa");
        deva_to_iso.insert('फ', "pha");
        deva_to_iso.insert('ब', "ba");
        deva_to_iso.insert('भ', "bha");
        deva_to_iso.insert('म', "ma");
        deva_to_iso.insert('य', "ya");
        deva_to_iso.insert('र', "ra");
        deva_to_iso.insert('ल', "la");
        deva_to_iso.insert('व', "va");
        deva_to_iso.insert('श', "śa");
        deva_to_iso.insert('ष', "ṣa");
        deva_to_iso.insert('स', "sa");
        deva_to_iso.insert('ह', "ha");

        // Vowel signs (mātrās)
        deva_to_iso.insert('ा', "ā");
        deva_to_iso.insert('ि', "i");
        deva_to_iso.insert('ी', "ī");
        deva_to_iso.insert('ु', "u");
        deva_to_iso.insert('ू', "ū");
        deva_to_iso.insert('ृ', "r̥");
        deva_to_iso.insert('ॄ', "r̥̄"); // vocalic RR sign
        deva_to_iso.insert('ॢ', "l̥"); // vocalic L sign
        deva_to_iso.insert('ॣ', "l̥̄"); // vocalic LL sign
        deva_to_iso.insert('े', "e");
        deva_to_iso.insert('ै', "ai"); // AI sign
        deva_to_iso.insert('ो', "o");
        deva_to_iso.insert('ौ', "au");

        // Special marks
        deva_to_iso.insert('्', ""); // virama (halant)
        deva_to_iso.insert('ँ', "m̐"); // candrabindu
        deva_to_iso.insert('ं', "ṁ"); // anusvara
        deva_to_iso.insert('ः', "ḥ"); // visarga
        deva_to_iso.insert('ऽ', "'"); // avagraha
        deva_to_iso.insert('़', ""); // nukta (modifies preceding consonant)
        deva_to_iso.insert('ॐ', "oṁ"); // om symbol

        // Build reverse mapping with priority handling for ambiguous cases
        for (&deva_char, &iso_str) in &deva_to_iso {
            if !iso_str.is_empty() {
                // Handle ambiguous mappings: prefer independent vowels over vowel signs
                if let Some(&_existing_char) = iso_to_deva.get(iso_str) {
                    // If we already have a mapping, keep the independent vowel
                    // Independent vowels: अ-औ (U+0905-U+0914) and ॠ-ॡ (U+0960-U+0961)
                    // Vowel signs: ा-ौ (U+093E-U+094C) and ॄ-ॣ (U+0944, U+0962-U+0963)
                    let is_independent_vowel = ((deva_char as u32) >= 0x0905
                        && (deva_char as u32) <= 0x0914)
                        || ((deva_char as u32) >= 0x0960 && (deva_char as u32) <= 0x0961);
                    if is_independent_vowel {
                        // This is an independent vowel, prefer it
                        iso_to_deva.insert(iso_str, deva_char);
                    }
                    // If existing is already an independent vowel, keep it
                } else {
                    iso_to_deva.insert(iso_str, deva_char);
                }
            }
        }

        // Add additional punctuation and digits
        deva_to_iso.insert('।', "।"); // Devanagari danda
        deva_to_iso.insert('॥', "॥"); // double danda
        iso_to_deva.insert("।", '।');
        iso_to_deva.insert("॥", '॥');

        // Devanagari digits
        deva_to_iso.insert('०', "0");
        deva_to_iso.insert('१', "1");
        deva_to_iso.insert('२', "2");
        deva_to_iso.insert('३', "3");
        deva_to_iso.insert('४', "4");
        deva_to_iso.insert('५', "5");
        deva_to_iso.insert('६', "6");
        deva_to_iso.insert('७', "7");
        deva_to_iso.insert('८', "8");
        deva_to_iso.insert('९', "9");

        // Add reverse mappings for digits (optional - design choice)
        iso_to_deva.insert("0", '०');
        iso_to_deva.insert("1", '१');
        iso_to_deva.insert("2", '२');
        iso_to_deva.insert("3", '३');
        iso_to_deva.insert("4", '४');
        iso_to_deva.insert("5", '५');
        iso_to_deva.insert("6", '६');
        iso_to_deva.insert("7", '७');
        iso_to_deva.insert("8", '८');
        iso_to_deva.insert("9", '९');

        // Additional Sanskrit/Vedic consonants
        deva_to_iso.insert('ळ', "ḷa"); // Marathi/Dravidian retroflex L

        // Nukta consonants (precomposed characters U+0958-U+095F)
        deva_to_iso.insert('\u{0958}', "qa"); // क़ QA
        deva_to_iso.insert('\u{0959}', "ḵẖa"); // ख़ KHHA
        deva_to_iso.insert('\u{095A}', "ġa"); // ग़ GHHA
        deva_to_iso.insert('\u{095B}', "za"); // ज़ ZA
        deva_to_iso.insert('\u{095C}', "ṛa"); // ड़ DDDHA
        deva_to_iso.insert('\u{095D}', "ṛha"); // ढ़ RHA
        deva_to_iso.insert('\u{095E}', "fa"); // फ़ FA
        deva_to_iso.insert('\u{095F}', "ẏa"); // य़ YYA

        // Add reverse mappings for nukta consonants
        iso_to_deva.insert("qa", '\u{0958}');
        iso_to_deva.insert("ḵẖa", '\u{0959}');
        iso_to_deva.insert("ġa", '\u{095A}');
        iso_to_deva.insert("za", '\u{095B}');
        iso_to_deva.insert("ṛa", '\u{095C}');
        iso_to_deva.insert("ṛha", '\u{095D}');
        iso_to_deva.insert("fa", '\u{095E}');
        iso_to_deva.insert("ẏa", '\u{095F}');

        // Add reverse mapping for additional consonants
        iso_to_deva.insert("ḷa", 'ळ');

        // Add reverse mapping for special characters that might be missed
        iso_to_deva.insert("'", 'ऽ');
        iso_to_deva.insert("oṁ", 'ॐ');

        Self {
            deva_to_iso_map: deva_to_iso,
            iso_to_deva_map: iso_to_deva,
        }
    }
}

impl HubTrait for Hub {
    fn deva_to_iso(&self, input: &str) -> Result<HubOutput, HubError> {
        // Pre-calculate capacity: Devanagari -> ISO typically expands due to romanization
        let estimated_capacity = input.len() * 2;
        let mut result = String::with_capacity(estimated_capacity);

        // Use char_indices for efficient iteration without Vec allocation
        let mut chars = input.char_indices().peekable();

        while let Some((_byte_pos, current_char)) = chars.next() {
            if current_char.is_whitespace() {
                result.push(current_char);
                continue;
            }

            // Handle ASCII punctuation except for characters that might be in our mapping
            if current_char.is_ascii_punctuation() && current_char != '\'' {
                result.push(current_char);
                continue;
            }

            // Check for consonant + nukta combinations (decomposed form)
            let mut effective_char = current_char;
            let mut skip_next = false;

            if let Some((_, next_char)) = chars.peek() {
                if *next_char == '़' {
                    // Try to find the precomposed nukta character
                    let nukta_mapping = match current_char {
                        'क' => Some('\u{0958}'), // क़
                        'ख' => Some('\u{0959}'), // ख़
                        'ग' => Some('\u{095A}'), // ग़
                        'ज' => Some('\u{095B}'), // ज़
                        'ड' => Some('\u{095C}'), // ड़
                        'ढ' => Some('\u{095D}'), // ढ़
                        'फ' => Some('\u{095E}'), // फ़
                        'य' => Some('\u{095F}'), // य़
                        _ => None,
                    };

                    if let Some(nukta_char) = nukta_mapping {
                        effective_char = nukta_char;
                        skip_next = true; // consume nukta on next iteration
                    }
                }
            }

            // Skip nukta if we just processed it
            if skip_next {
                chars.next(); // consume the nukta character
            }

            // Check if this is a consonant followed by virama or vowel sign
            if let Some(&iso_str) = self.deva_to_iso_map.get(&effective_char) {
                // Peek at next character to determine processing
                if let Some((_, next_char)) = chars.peek() {
                    if *next_char == '्' {
                        // Consonant + virama: remove inherent 'a'
                        if iso_str.ends_with('a') && iso_str.len() > 1 {
                            result.push_str(&iso_str[..iso_str.len() - 1]);
                        } else {
                            result.push_str(iso_str);
                        }
                        chars.next(); // consume the virama
                        continue;
                    } else if let Some(&vowel_sign) = self.deva_to_iso_map.get(next_char) {
                        // Check if next character is a vowel sign (mātrā)
                        let is_vowel_sign = matches!(
                            *next_char,
                            'ा' | 'ि'
                                | 'ी'
                                | 'ु'
                                | 'ू'
                                | 'ृ'
                                | 'ॄ'
                                | 'ॢ'
                                | 'ॣ'
                                | 'े'
                                | 'ै'
                                | 'ो'
                                | 'ौ'
                        );

                        if is_vowel_sign && iso_str.ends_with('a') && iso_str.len() > 1 {
                            // Consonant + vowel sign: replace inherent 'a' with the vowel
                            result.push_str(&iso_str[..iso_str.len() - 1]);
                            result.push_str(vowel_sign);
                            chars.next(); // consume the vowel sign
                            continue;
                        }
                    }
                }

                // Regular character or no special following character
                result.push_str(iso_str);
            } else {
                // Unknown character - pass through gracefully
                result.push(current_char);
            }
        }

        Ok(HubOutput::Iso(result))
    }

    fn iso_to_deva(&self, input: &str) -> Result<HubOutput, HubError> {
        // Pre-calculate capacity: ISO -> Devanagari typically contracts due to combining
        let estimated_capacity = input.len().max(32); // Minimum reasonable size
        let mut result = String::with_capacity(estimated_capacity);

        // Normalize ISO input to composed form (NFC) to handle combining characters
        let normalized: String = input.nfc().collect();
        let chars: Vec<char> = normalized.chars().collect(); // Keep Vec for complex lookahead logic
        let mut i = 0;

        while i < chars.len() {
            let ch = chars[i];

            if ch.is_whitespace() {
                result.push(ch);
                i += 1;
                continue;
            }

            // Handle ASCII punctuation except for characters that might be in our mapping
            if ch.is_ascii_punctuation() && ch != '\'' {
                result.push(ch);
                i += 1;
                continue;
            }

            // First check for combining characters and create extended sequences
            let mut parsed = false;
            let mut extended_seq_opt: Option<(String, usize)> = None;

            // Check if current char has combining characters following
            if i + 1 < chars.len() {
                let mut extend_len = 1;
                while i + extend_len < chars.len() {
                    let next_char = chars[i + extend_len];
                    if (next_char as u32) >= 0x0300 && (next_char as u32) <= 0x036F {
                        extend_len += 1;
                    } else {
                        break;
                    }
                }

                if extend_len > 1 {
                    let extended_seq = chars[i..i + extend_len].iter().collect::<String>();
                    if self.iso_to_deva_map.contains_key(extended_seq.as_str()) {
                        extended_seq_opt = Some((extended_seq, extend_len));
                    }
                }
            }

            // If we found a direct extended sequence match, use it
            if let Some((seq, len)) = extended_seq_opt {
                result.push(self.iso_to_deva_map[seq.as_str()]);
                i += len;
                parsed = true;
            }

            // If not parsed yet, try to find consonant + vowel combinations
            if !parsed {
                // We need to try different splits: 1+3, 1+2, 1+1, 2+2, 2+1, 3+1
                for total_len in (2..=4).rev() {
                    if i + total_len > chars.len() {
                        continue;
                    }

                    // Try different consonant lengths within this total
                    for cons_len in 1..total_len {
                        let vowel_len = total_len - cons_len;
                        if vowel_len > 3 {
                            continue;
                        } // Maximum vowel length

                        let cons_seq: String = chars[i..i + cons_len].iter().collect();
                        let cons_with_a = format!("{cons_seq}a");

                        if let Some(&cons_char) = self.iso_to_deva_map.get(cons_with_a.as_str()) {
                            // Found a consonant, check the vowel part
                            let vowel_seq: String =
                                chars[i + cons_len..i + total_len].iter().collect();

                            // Check if this vowel exists and has a sign form
                            if self.iso_to_deva_map.contains_key(vowel_seq.as_str()) {
                                if vowel_seq == "a" {
                                    // Inherent vowel - just output the consonant
                                    result.push(cons_char);
                                    i += total_len;
                                    parsed = true;
                                    break;
                                } else {
                                    let vowel_sign = match vowel_seq.as_str() {
                                        "ā" => Some('ा'),
                                        "i" => Some('ि'),
                                        "ī" => Some('ी'),
                                        "u" => Some('ु'),
                                        "ū" => Some('ू'),
                                        "r̥" => Some('ृ'),
                                        "r̥̄" => Some('ॄ'),
                                        "l̥" => Some('ॢ'),
                                        "l̥̄" => Some('ॣ'),
                                        "e" => Some('े'),
                                        "ai" => Some('ै'),
                                        "o" => Some('ो'),
                                        "au" => Some('ौ'),
                                        _ => None,
                                    };

                                    if let Some(sign) = vowel_sign {
                                        // Consonant + vowel sign
                                        result.push(cons_char);
                                        result.push(sign);
                                        i += total_len;
                                        parsed = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    if parsed {
                        break;
                    }
                }

                // If no consonant+vowel found, try bare consonant
                if !parsed {
                    for cons_len in (1..=3).rev() {
                        if i + cons_len > chars.len() {
                            continue;
                        }

                        let cons_seq: String = chars[i..i + cons_len].iter().collect();
                        let cons_with_a = format!("{cons_seq}a");

                        if let Some(&cons_char) = self.iso_to_deva_map.get(cons_with_a.as_str()) {
                            // No vowel found, treat as bare consonant
                            result.push(cons_char);
                            result.push('्'); // virama
                            i += cons_len;
                            parsed = true;
                            break;
                        }
                    }
                }

                // If consonant+vowel parsing failed, try direct matches
                if !parsed {
                    let mut best_match: Option<(String, usize)> = None;

                    // Try sequences of decreasing length, checking for combining characters
                    for len in (1..=4).rev() {
                        if i + len > chars.len() {
                            continue;
                        }

                        let seq: String = chars[i..i + len].iter().collect();
                        if self.iso_to_deva_map.contains_key(seq.as_str()) {
                            best_match = Some((seq, len));
                            break;
                        }
                    }

                    if let Some((matched_seq, len)) = best_match {
                        result.push(self.iso_to_deva_map[matched_seq.as_str()]);
                        i += len;
                        parsed = true;
                    }
                }
            }

            if !parsed {
                // Unknown character - pass through gracefully
                result.push(ch);
                i += 1;
            }
        }

        Ok(HubOutput::Devanagari(result))
    }

    fn deva_to_iso_with_metadata(&self, input: &str) -> Result<HubResult, HubError> {
        let mut result = String::with_capacity(input.len() * 2); // Pre-allocate for expansion
        let mut metadata = TransliterationMetadata::new("devanagari", "iso15919");

        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let current_char = chars[i];

            if current_char.is_whitespace() {
                result.push(current_char);
                i += 1;
                continue;
            }

            if current_char.is_ascii_punctuation() && current_char != '\'' {
                result.push(current_char);
                i += 1;
                continue;
            }

            // Check for consonant + nukta combinations (decomposed form)
            let mut effective_char = current_char;
            let mut char_consumed = 1;

            if i + 1 < chars.len() && chars[i + 1] == '़' {
                // Try to find the precomposed nukta character
                let nukta_mapping = match current_char {
                    'क' => Some('\u{0958}'), // क़
                    'ख' => Some('\u{0959}'), // ख़
                    'ग' => Some('\u{095A}'), // ग़
                    'ज' => Some('\u{095B}'), // ज़
                    'ड' => Some('\u{095C}'), // ड़
                    'ढ' => Some('\u{095D}'), // ढ़
                    'फ' => Some('\u{095E}'), // फ़
                    'य' => Some('\u{095F}'), // य़
                    _ => None,
                };

                if let Some(nukta_char) = nukta_mapping {
                    effective_char = nukta_char;
                    char_consumed = 2; // consume both base char and nukta
                }
            }

            // Check if this is a consonant followed by virama or vowel sign
            if let Some(&iso_str) = self.deva_to_iso_map.get(&effective_char) {
                // Check next character (considering nukta consumption)
                if i + char_consumed < chars.len() {
                    let next_char = chars[i + char_consumed];

                    if next_char == '्' {
                        // Consonant + virama: remove inherent 'a'
                        if iso_str.ends_with('a') && iso_str.len() > 1 {
                            result.push_str(&iso_str[..iso_str.len() - 1]);
                        } else {
                            result.push_str(iso_str);
                        }
                        i += char_consumed + 1; // Skip consonant(+nukta) and virama
                        continue;
                    } else if let Some(&vowel_sign) = self.deva_to_iso_map.get(&next_char) {
                        // Check if next character is a vowel sign (mātrā)
                        let is_vowel_sign = matches!(
                            next_char,
                            'ा' | 'ि'
                                | 'ी'
                                | 'ु'
                                | 'ू'
                                | 'ृ'
                                | 'ॄ'
                                | 'ॢ'
                                | 'ॣ'
                                | 'े'
                                | 'ै'
                                | 'ो'
                                | 'ौ'
                        );

                        if is_vowel_sign && iso_str.ends_with('a') && iso_str.len() > 1 {
                            // Consonant + vowel sign: replace inherent 'a' with the vowel
                            result.push_str(&iso_str[..iso_str.len() - 1]);
                            result.push_str(vowel_sign);
                            i += char_consumed + 1; // Skip consonant(+nukta) and vowel sign
                            continue;
                        }
                    }
                }

                // Regular character or no special following character
                result.push_str(iso_str);
                i += char_consumed;
            } else {
                // Unknown character - add to metadata and pass through
                let unknown_token =
                    UnknownToken::new("devanagari", current_char, result.len(), false);
                metadata.add_unknown(unknown_token);
                result.push(current_char);
                i += char_consumed;
            }
        }

        Ok(HubResult {
            output: HubOutput::Iso(result),
            metadata: Some(metadata),
        })
    }

    fn iso_to_deva_with_metadata(&self, input: &str) -> Result<HubResult, HubError> {
        let mut result = String::with_capacity(input.len() * 2); // Pre-allocate for expansion
        let mut metadata = TransliterationMetadata::new("iso15919", "devanagari");

        let normalized: String = input.nfc().collect();
        let chars: Vec<char> = normalized.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let ch = chars[i];

            if ch.is_whitespace() {
                result.push(ch);
                i += 1;
                continue;
            }

            if ch.is_ascii_punctuation() && ch != '\'' {
                result.push(ch);
                i += 1;
                continue;
            }

            // Try to match sequences with combining characters
            let mut found = false;
            let mut extend_len = 1;

            // Look for longer sequences first
            while i + extend_len <= chars.len() && extend_len <= 4 {
                let test_seq = chars[i..i + extend_len].iter().collect::<String>();
                if self.iso_to_deva_map.contains_key(test_seq.as_str()) {
                    result.push(self.iso_to_deva_map[test_seq.as_str()]);
                    i += extend_len;
                    found = true;
                    break;
                }
                extend_len += 1;
            }

            if !found {
                // Unknown character - add to metadata and pass through
                let unknown_token = UnknownToken::new("iso15919", ch, result.len(), false);
                metadata.add_unknown(unknown_token);
                result.push(ch);
                i += 1;
            }
        }

        Ok(HubResult {
            output: HubOutput::Devanagari(result),
            metadata: Some(metadata),
        })
    }
}

impl Default for Hub {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod comprehensive_tests;

#[cfg(test)]
mod debug_test;

#[cfg(test)]
mod debug_dharma_test;

#[cfg(test)]
mod unicode_coverage_test;

#[cfg(test)]
mod extended_coverage_tests;

#[cfg(test)]
mod malformed_input_tests;

#[cfg(test)]
mod original_tests {
    use super::*;

    #[test]
    fn test_hub_creation() {
        let hub = Hub::new();
        assert!(!hub.deva_to_iso_map.is_empty());
        assert!(!hub.iso_to_deva_map.is_empty());
    }

    #[test]
    fn test_basic_deva_to_iso() {
        let hub = Hub::new();

        let result = hub.deva_to_iso("अ").unwrap();
        if let HubOutput::Iso(iso) = result {
            assert_eq!(iso, "a");
        } else {
            panic!("Expected ISO output");
        }

        let result = hub.deva_to_iso("आ").unwrap();
        if let HubOutput::Iso(iso) = result {
            assert_eq!(iso, "ā");
        } else {
            panic!("Expected ISO output");
        }
    }

    #[test]
    fn test_basic_iso_to_deva() {
        let hub = Hub::new();

        let result = hub.iso_to_deva("a").unwrap();
        if let HubOutput::Devanagari(deva) = result {
            assert_eq!(deva, "अ");
        } else {
            panic!("Expected Devanagari output");
        }

        let result = hub.iso_to_deva("ā").unwrap();
        if let HubOutput::Devanagari(deva) = result {
            assert_eq!(deva, "आ");
        } else {
            panic!("Expected Devanagari output");
        }
    }

    #[test]
    fn test_consonant_conversion() {
        let hub = Hub::new();

        let result = hub.deva_to_iso("क").unwrap();
        if let HubOutput::Iso(iso) = result {
            assert_eq!(iso, "ka");
        } else {
            panic!("Expected ISO output");
        }

        let result = hub.iso_to_deva("ka").unwrap();
        if let HubOutput::Devanagari(deva) = result {
            assert_eq!(deva, "क");
        } else {
            panic!("Expected Devanagari output");
        }
    }

    #[test]
    fn test_roundtrip_conversion() {
        let hub = Hub::new();

        // Test roundtrip: Deva -> ISO -> Deva
        let original = "क";
        let to_iso = hub.deva_to_iso(original).unwrap();
        if let HubOutput::Iso(iso_text) = to_iso {
            let back_to_deva = hub.iso_to_deva(&iso_text).unwrap();
            if let HubOutput::Devanagari(deva_text) = back_to_deva {
                assert_eq!(deva_text, original);
            } else {
                panic!("Expected Devanagari output");
            }
        } else {
            panic!("Expected ISO output");
        }
    }

    #[test]
    fn test_whitespace_preservation() {
        let hub = Hub::new();

        let result = hub.deva_to_iso("क म").unwrap();
        if let HubOutput::Iso(iso) = result {
            assert_eq!(iso, "ka ma");
        } else {
            panic!("Expected ISO output");
        }

        let result = hub.iso_to_deva("ka ma").unwrap();
        if let HubOutput::Devanagari(deva) = result {
            assert_eq!(deva, "क म");
        } else {
            panic!("Expected Devanagari output");
        }
    }

    #[test]
    fn test_unknown_devanagari_character_passthrough() {
        let hub = Hub::new();

        // Unknown characters should pass through gracefully
        let result = hub.deva_to_iso("xyz").unwrap();
        if let HubOutput::Iso(iso) = result {
            assert_eq!(iso, "xyz"); // Should pass through unchanged
        } else {
            panic!("Expected ISO output");
        }

        // Test mixed known and unknown
        let result = hub.deva_to_iso("अxyzआ").unwrap();
        if let HubOutput::Iso(iso) = result {
            assert_eq!(iso, "axyzā"); // Should convert known, pass through unknown
        } else {
            panic!("Expected ISO output");
        }
    }

    #[test]
    fn test_empty_string() {
        let hub = Hub::new();

        let result = hub.deva_to_iso("").unwrap();
        if let HubOutput::Iso(iso) = result {
            assert_eq!(iso, "");
        } else {
            panic!("Expected ISO output");
        }

        let result = hub.iso_to_deva("").unwrap();
        if let HubOutput::Devanagari(deva) = result {
            assert_eq!(deva, "");
        } else {
            panic!("Expected Devanagari output");
        }
    }
}

// TODO List for Hub Module:
// - [ ] Add support for conjunct consonants
// - [ ] Implement proper handling of complex Devanagari sequences
// - [ ] Add support for Vedic accents
// - [ ] Optimize mapping lookup performance
// - [ ] Add comprehensive Unicode normalization
// - [ ] Implement preservation tokens for unknown mappings: [<script>:<token>:<unicode_point>]
