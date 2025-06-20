//! Fast, asymmetric transliterator optimized for common use cases

use std::collections::HashMap;

/// Fast transliterator that trades perfect bidirectionality for performance
pub struct FastTransliterator {
    /// Direct character mappings for fast lookup
    direct_maps: HashMap<String, HashMap<char, &'static str>>,
    /// Multi-character mappings (digraphs, etc.)
    multi_maps: HashMap<String, HashMap<&'static str, &'static str>>,
}

impl FastTransliterator {
    pub fn new() -> Self {
        let mut ft = Self {
            direct_maps: HashMap::new(),
            multi_maps: HashMap::new(),
        };
        
        // Initialize common mappings
        ft.init_devanagari_to_iast();
        ft.init_devanagari_to_slp1();
        
        ft
    }
    
    fn init_devanagari_to_iast(&mut self) {
        let mut direct = HashMap::new();
        let mut multi = HashMap::new();
        
        // Direct consonants
        direct.insert('क', "ka");
        direct.insert('ख', "kha");
        direct.insert('ग', "ga");
        direct.insert('घ', "gha");
        direct.insert('ङ', "ṅa");
        direct.insert('च', "ca");
        direct.insert('छ', "cha");
        direct.insert('ज', "ja");
        direct.insert('झ', "jha");
        direct.insert('ञ', "ña");
        direct.insert('ट', "ṭa");
        direct.insert('ठ', "ṭha");
        direct.insert('ड', "ḍa");
        direct.insert('ढ', "ḍha");
        direct.insert('ण', "ṇa");
        direct.insert('त', "ta");
        direct.insert('थ', "tha");
        direct.insert('द', "da");
        direct.insert('ध', "dha");
        direct.insert('न', "na");
        direct.insert('प', "pa");
        direct.insert('फ', "pha");
        direct.insert('ब', "ba");
        direct.insert('भ', "bha");
        direct.insert('म', "ma");
        direct.insert('य', "ya");
        direct.insert('र', "ra");
        direct.insert('ल', "la");
        direct.insert('व', "va");
        direct.insert('श', "śa");
        direct.insert('ष', "ṣa");
        direct.insert('स', "sa");
        direct.insert('ह', "ha");
        
        // Vowels
        direct.insert('अ', "a");
        direct.insert('आ', "ā");
        direct.insert('इ', "i");
        direct.insert('ई', "ī");
        direct.insert('उ', "u");
        direct.insert('ऊ', "ū");
        direct.insert('ऋ', "ṛ");
        direct.insert('ॠ', "ṝ");
        direct.insert('ऌ', "ḷ");
        direct.insert('ए', "e");
        direct.insert('ऐ', "ai");
        direct.insert('ओ', "o");
        direct.insert('औ', "au");
        
        // Matras
        direct.insert('ा', "ā");
        direct.insert('ि', "i");
        direct.insert('ी', "ī");
        direct.insert('ु', "u");
        direct.insert('ू', "ū");
        direct.insert('ृ', "ṛ");
        direct.insert('े', "e");
        direct.insert('ै', "ai");
        direct.insert('ो', "o");
        direct.insert('ौ', "au");
        
        // Special characters
        direct.insert('्', "");  // Virama
        direct.insert('ं', "ṃ");
        direct.insert('ः', "ḥ");
        direct.insert('।', ".");
        direct.insert('॥', "..");
        
        // Multi-character mappings (compounds)
        multi.insert("क्ष", "kṣa");
        multi.insert("ज्ञ", "jña");
        multi.insert("श्र", "śra");
        
        self.direct_maps.insert("Devanagari_to_IAST".to_string(), direct);
        self.multi_maps.insert("Devanagari_to_IAST".to_string(), multi);
    }
    
    fn init_devanagari_to_slp1(&mut self) {
        let mut direct = HashMap::new();
        
        // SLP1 mappings (more compact)
        direct.insert('क', "ka");
        direct.insert('ख', "Ka");
        direct.insert('ग', "ga");
        direct.insert('घ', "Ga");
        direct.insert('ङ', "Na");
        // ... etc
        
        direct.insert('्', "");
        direct.insert('ं', "M");
        direct.insert('ः', "H");
        
        self.direct_maps.insert("Devanagari_to_SLP1".to_string(), direct);
    }
    
    /// Fast transliteration without IR or complex transformations
    pub fn transliterate(&self, input: &str, from: &str, to: &str) -> Result<String, String> {
        let key = format!("{}_{}", from, to);
        
        let direct = self.direct_maps.get(&key)
            .ok_or_else(|| format!("Unsupported path: {} to {}", from, to))?;
        let multi = self.multi_maps.get(&key);
        
        let mut output = String::with_capacity(input.len() * 2);
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            let mut matched = false;
            
            // Try multi-char patterns first (if available)
            if let Some(multi_map) = multi {
                for len in (2..=3).rev() {
                    if i + len <= chars.len() {
                        let substr: String = chars[i..i + len].iter().collect();
                        if let Some(replacement) = multi_map.get(substr.as_str()) {
                            output.push_str(replacement);
                            i += len;
                            matched = true;
                            break;
                        }
                    }
                }
            }
            
            // Single character lookup
            if !matched {
                if let Some(replacement) = direct.get(&chars[i]) {
                    output.push_str(replacement);
                } else if chars[i].is_whitespace() {
                    output.push(chars[i]);
                } else {
                    // Unknown character - preserve with token
                    output.push_str(&format!("[{}:{}]", from.to_lowercase(), chars[i]));
                }
                i += 1;
            }
        }
        
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fast_devanagari_to_iast() {
        let ft = FastTransliterator::new();
        
        assert_eq!(
            ft.transliterate("धर्म", "Devanagari", "to_IAST").unwrap(),
            "dharma"
        );
        
        assert_eq!(
            ft.transliterate("कर्म", "Devanagari", "to_IAST").unwrap(),
            "karma"
        );
        
        assert_eq!(
            ft.transliterate("क्ष", "Devanagari", "to_IAST").unwrap(),
            "kṣa"
        );
    }
}