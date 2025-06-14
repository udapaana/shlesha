//! Bidirectional transliteration compiler
//! 
//! Token-based compiler that provides perfect round-trip transliteration
//! by using tokens as an intermediate representation.

use crate::tokens::{SanskritToken, TokenWithMetadata};
use crate::{TargetScheme, TransliterationError};
use std::collections::HashMap;
use std::path::Path;
use unicode_normalization::UnicodeNormalization;

/// Scheme definition for parsing and rendering
#[derive(Debug, Clone)]
pub struct SchemeDefinition {
    pub name: String,
    /// For parsing: input string → token
    input_to_token: HashMap<String, SanskritToken>,
    /// For rendering: token → output string  
    token_to_output: HashMap<String, String>,
    /// Ordered patterns for longest-match parsing
    parsing_patterns: Vec<String>,
}

impl SchemeDefinition {
    /// Create from TOML mappings
    pub fn from_toml_mappings(name: String, mappings: HashMap<String, String>) -> Self {
        let mut input_to_token = HashMap::new();
        let mut token_to_output = HashMap::new();
        let mut parsing_patterns = Vec::new();
        
        for (token_name, output_string) in mappings {
            let token = SanskritToken::register(token_name.clone());
            
            // Build both directions
            input_to_token.insert(output_string.clone(), token);
            token_to_output.insert(token_name, output_string.clone());
            parsing_patterns.push(output_string);
        }
        
        // Sort patterns by length descending for longest-match parsing
        parsing_patterns.sort_by(|a, b| b.len().cmp(&a.len()).then(a.cmp(b)));
        
        Self {
            name,
            input_to_token,
            token_to_output,
            parsing_patterns,
        }
    }
    
    /// Parse input text to tokens with intelligent script-aware parsing
    pub fn parse(&self, text: &str) -> Vec<TokenWithMetadata> {
        let normalized = text.nfc().collect::<String>();
        
        // Detect script and use appropriate parser
        if self.is_devanagari_text(&normalized) {
            self.parse_devanagari(&normalized)
        } else {
            self.parse_romanization(&normalized)
        }
    }
    
    /// Check if text is primarily Devanagari
    fn is_devanagari_text(&self, text: &str) -> bool {
        let devanagari_chars = text.chars()
            .filter(|c| *c as u32 >= 0x0900 && *c as u32 <= 0x097F)
            .count();
        let total_chars = text.chars().filter(|c| !c.is_whitespace()).count();
        
        total_chars > 0 && devanagari_chars as f64 / total_chars as f64 > 0.5
    }
    
    /// Parse Devanagari text with conjunct awareness using multi-level approach
    fn parse_devanagari(&self, text: &str) -> Vec<TokenWithMetadata> {
        // Force syllable-aware approach for debugging
        match self.parse_devanagari_with_syllables(text) {
            Ok(tokens) => tokens,
            Err(_e) => {
                // For debugging: if syllable parsing fails, still try it but report the error
                // In production, this would fallback to longest-match parsing
                self.parse_devanagari_longest_match(text)
            }
        }
    }
    
    /// Advanced Devanagari parsing with syllable awareness
    fn parse_devanagari_with_syllables(&self, text: &str) -> Result<Vec<TokenWithMetadata>, String> {
        let mut tokens = Vec::new();
        let mut i = 0;
        let chars: Vec<char> = text.chars().collect();
        
        while i < chars.len() {
            let position = i;
            
            // Handle whitespace
            if chars[i].is_whitespace() {
                let mut whitespace = String::new();
                while i < chars.len() && chars[i].is_whitespace() {
                    whitespace.push(chars[i]);
                    i += 1;
                }
                tokens.push(TokenWithMetadata::new(
                    SanskritToken::Space,
                    whitespace,
                    position,
                ));
                continue;
            }
            
            
            // Try to parse a syllable
            if let Some((syllable_tokens, chars_consumed)) = self.parse_devanagari_syllable_smart(&chars, i) {
                tokens.extend(syllable_tokens);
                i += chars_consumed;
            } else {
                // Single character fallback
                let ch = chars[i].to_string();
                if let Some(token) = self.input_to_token.get(&ch) {
                    tokens.push(TokenWithMetadata::new(
                        token.clone(),
                        ch,
                        position,
                    ));
                } else {
                    tokens.push(TokenWithMetadata::new(
                        SanskritToken::Unknown(ch.clone()),
                        ch,
                        position,
                    ));
                }
                i += 1;
            }
        }
        
        Ok(tokens)
    }
    
    /// Smart syllable parsing that handles conjuncts properly
    fn parse_devanagari_syllable_smart(&self, chars: &[char], start: usize) -> Option<(Vec<TokenWithMetadata>, usize)> {
        let mut i = start;
        let mut syllable_chars = Vec::new();
        
        // Check for independent vowel first
        if i < chars.len() && self.is_independent_vowel(chars[i]) {
            let ch = chars[i].to_string();
            if let Some(token) = self.input_to_token.get(&ch) {
                return Some((vec![TokenWithMetadata::new(token.clone(), ch, start)], 1));
            }
        }
        
        // Collect consonant cluster
        let mut consonant_count = 0;
        while i < chars.len() && self.is_consonant_char(chars[i]) {
            syllable_chars.push(chars[i]);
            consonant_count += 1;
            i += 1;
            
            // Check for halant (virama)
            if i < chars.len() && chars[i] == '्' {
                syllable_chars.push(chars[i]);
                i += 1;
                // Continue to next consonant
            } else {
                break; // No halant, end of consonant cluster
            }
        }
        
        // Must have at least one consonant to proceed
        if consonant_count == 0 {
            return None;
        }
        
        // Collect vowel mark (if any)
        if i < chars.len() && self.is_vowel_mark(chars[i]) {
            syllable_chars.push(chars[i]);
            i += 1;
        }
        
        // Now we have a complete syllable - create appropriate token
        let syllable_text: String = syllable_chars.iter().collect();
        // eprintln!("DEBUG: Parsed syllable '{}' from chars: {:?}", syllable_text, syllable_chars);
        
        // Try to find exact match first
        if let Some(token) = self.input_to_token.get(&syllable_text) {
            return Some((vec![TokenWithMetadata::new(token.clone(), syllable_text, start)], i - start));
        }
        
        // If no exact match, create a compound token name for the syllable
        let token_name = self.create_syllable_token_name(&syllable_chars);
        let token = SanskritToken::register(token_name);
        
        Some((vec![TokenWithMetadata::new(token, syllable_text, start)], i - start))
    }
    
    /// Create a logical token name for a syllable
    fn create_syllable_token_name(&self, chars: &[char]) -> String {
        // For "ग्नि", we want consonants without vowel + final consonant with vowel
        // Result should be: GA (remove 'a') + NI = "GA_NI" which renders as "g" + "ni" = "gni"
        let mut parts = Vec::new();
        let mut i = 0;
        
        // Parse consonant cluster
        let mut consonants = Vec::new();
        while i < chars.len() && self.is_consonant_char(chars[i]) {
            consonants.push(chars[i]);
            i += 1;
            
            // Skip halant
            if i < chars.len() && chars[i] == '्' {
                i += 1;
            } else {
                break; // No halant, end of cluster
            }
        }
        
        // Find vowel mark (if any)
        let vowel_mark = if i < chars.len() && self.is_vowel_mark(chars[i]) {
            Some(chars[i])
        } else {
            None
        };
        
        // Create tokens: all consonants except last get "A", last gets the vowel
        for (idx, consonant) in consonants.iter().enumerate() {
            if let Some(base_name) = self.consonant_to_syllable_token_name(*consonant) {
                if idx == consonants.len() - 1 {
                    // Last consonant: add the vowel mark or inherent 'a'
                    if let Some(vowel_ch) = vowel_mark {
                        if let Some(vowel_name) = self.vowel_mark_to_token_name(vowel_ch) {
                            // Combine consonant with vowel matra for last consonant
                            parts.push(format!("{}_{}", base_name, vowel_name));
                        } else {
                            parts.push(base_name); // Fallback
                        }
                    } else {
                        parts.push(base_name); // Inherent 'a'
                    }
                } else {
                    // Non-final consonant: just the consonant part (will be stripped during rendering)
                    parts.push(base_name);
                }
            }
        }
        
        parts.join("_")
    }
    
    /// Fallback to longest-match parsing (original approach)
    fn parse_devanagari_longest_match(&self, text: &str) -> Vec<TokenWithMetadata> {
        
        let mut tokens = Vec::new();
        let mut i = 0;
        let chars: Vec<char> = text.chars().collect();
        
        while i < chars.len() {
            let position = i;
            let mut matched = false;
            
            // Handle whitespace
            if chars[i].is_whitespace() {
                let mut whitespace = String::new();
                while i < chars.len() && chars[i].is_whitespace() {
                    whitespace.push(chars[i]);
                    i += 1;
                }
                tokens.push(TokenWithMetadata::new(
                    SanskritToken::Space,
                    whitespace,
                    position,
                ));
                continue;
            }
            
            // Try longest patterns first
            for pattern in &self.parsing_patterns {
                let pattern_chars: Vec<char> = pattern.chars().collect();
                
                if i + pattern_chars.len() <= chars.len() {
                    let substr: String = chars[i..i+pattern_chars.len()].iter().collect();
                    
                    if substr == *pattern {
                        if let Some(token) = self.input_to_token.get(pattern) {
                            tokens.push(TokenWithMetadata::new(
                                token.clone(),
                                pattern.clone(),
                                position,
                            ));
                            i += pattern_chars.len();
                            matched = true;
                            break;
                        }
                    }
                }
            }
            
            if !matched {
                // Fallback: single character
                let ch = chars[i].to_string();
                if let Some(token) = self.input_to_token.get(&ch) {
                    tokens.push(TokenWithMetadata::new(
                        token.clone(),
                        ch,
                        position,
                    ));
                } else {
                    tokens.push(TokenWithMetadata::new(
                        SanskritToken::Unknown(ch.clone()),
                        ch,
                        position,
                    ));
                }
                i += 1;
            }
        }
        
        tokens
    }
    
    /// Helper methods for character classification
    fn is_independent_vowel(&self, ch: char) -> bool {
        let code = ch as u32;
        (0x0905..=0x0914).contains(&code) || (0x0960..=0x0961).contains(&code)
    }
    
    fn is_consonant_char(&self, ch: char) -> bool {
        let code = ch as u32;
        (0x0915..=0x0939).contains(&code) || ch == 'ळ'
    }
    
    fn is_vowel_mark(&self, ch: char) -> bool {
        let code = ch as u32;
        (0x093E..=0x094C).contains(&code) || (0x0962..=0x0963).contains(&code)
    }
    
    fn consonant_to_syllable_token_name(&self, ch: char) -> Option<String> {
        // Map Devanagari consonants to syllable token names (with inherent 'a')
        match ch {
            'क' => Some("KA".to_string()),
            'ख' => Some("KHA".to_string()),
            'ग' => Some("GA".to_string()),
            'घ' => Some("GHA".to_string()),
            'ङ' => Some("NGA".to_string()),
            'च' => Some("CA".to_string()),
            'छ' => Some("CHA".to_string()),
            'ज' => Some("JA".to_string()),
            'झ' => Some("JHA".to_string()),
            'ञ' => Some("NYA".to_string()),
            'ट' => Some("TTA".to_string()),
            'ठ' => Some("TTHA".to_string()),
            'ड' => Some("DDA".to_string()),
            'ढ' => Some("DDHA".to_string()),
            'ण' => Some("NNA".to_string()),
            'त' => Some("TA".to_string()),
            'थ' => Some("THA".to_string()),
            'द' => Some("DA".to_string()),
            'ध' => Some("DHA".to_string()),
            'न' => Some("NA".to_string()),
            'प' => Some("PA".to_string()),
            'फ' => Some("PHA".to_string()),
            'ब' => Some("BA".to_string()),
            'भ' => Some("BHA".to_string()),
            'म' => Some("MA".to_string()),
            'य' => Some("YA".to_string()),
            'र' => Some("RA".to_string()),
            'ल' => Some("LA".to_string()),
            'ळ' => Some("LLA".to_string()),
            'व' => Some("VA".to_string()),
            'श' => Some("SHA".to_string()),
            'ष' => Some("SSA".to_string()),
            'स' => Some("SA".to_string()),
            'ह' => Some("HA".to_string()),
            _ => None,
        }
    }
    
    fn vowel_mark_to_token_name(&self, ch: char) -> Option<String> {
        match ch {
            'ा' => Some("AA_MATRA".to_string()),
            'ि' => Some("I_MATRA".to_string()),
            'ी' => Some("II_MATRA".to_string()),
            'ु' => Some("U_MATRA".to_string()),
            'ू' => Some("UU_MATRA".to_string()),
            'ृ' => Some("RI_MATRA".to_string()),
            'ॄ' => Some("RII_MATRA".to_string()),
            'ॢ' => Some("LI_MATRA".to_string()),
            'ॣ' => Some("LII_MATRA".to_string()),
            'े' => Some("E_MATRA".to_string()),
            'ै' => Some("AI_MATRA".to_string()),
            'ो' => Some("O_MATRA".to_string()),
            'ौ' => Some("AU_MATRA".to_string()),
            _ => None,
        }
    }
    
    fn is_vowel_token_name(&self, name: &str) -> bool {
        matches!(name, "A" | "AA" | "I" | "II" | "U" | "UU" | "RI" | "RII" | "LI" | "LII" | "E" | "AI" | "O" | "AU")
    }
    
    /// Parse romanization schemes (IAST, SLP1) with longest-match
    fn parse_romanization(&self, text: &str) -> Vec<TokenWithMetadata> {
        let mut tokens = Vec::new();
        let mut i = 0;
        let chars: Vec<char> = text.chars().collect();
        
        while i < chars.len() {
            let position = i;
            let mut matched = false;
            
            // Handle whitespace
            if chars[i].is_whitespace() {
                let mut whitespace = String::new();
                while i < chars.len() && chars[i].is_whitespace() {
                    whitespace.push(chars[i]);
                    i += 1;
                }
                tokens.push(TokenWithMetadata::new(
                    SanskritToken::Space,
                    whitespace,
                    position,
                ));
                continue;
            }
            
            // Try longest patterns first
            for pattern in &self.parsing_patterns {
                let pattern_chars: Vec<char> = pattern.chars().collect();
                
                if i + pattern_chars.len() <= chars.len() {
                    let substr: String = chars[i..i+pattern_chars.len()].iter().collect();
                    
                    if substr == *pattern {
                        if let Some(token) = self.input_to_token.get(pattern) {
                            tokens.push(TokenWithMetadata::new(
                                token.clone(),
                                pattern.clone(),
                                position,
                            ));
                            i += pattern_chars.len();
                            matched = true;
                            break;
                        }
                    }
                }
            }
            
            if !matched {
                let ch = chars[i].to_string();
                tokens.push(TokenWithMetadata::new(
                    SanskritToken::Unknown(ch.clone()),
                    ch,
                    position,
                ));
                i += 1;
            }
        }
        
        tokens
    }
    
    /// Render tokens to output text
    pub fn render(&self, tokens: &[TokenWithMetadata]) -> String {
        tokens.iter()
            .map(|token_meta| {
                match &token_meta.token {
                    SanskritToken::Space => token_meta.original_text.clone(),
                    SanskritToken::Unknown(text) => format!("[?{}]", text),
                    SanskritToken::Named(name, _) => {
                        // First try direct mapping
                        if let Some(output) = self.token_to_output.get(name) {
                            return output.clone();
                        }
                        
                        
                        // Try intelligent rendering for compound tokens
                        if let Some(rendered) = self.render_compound_token(name) {
                            return rendered;
                        }
                        
                        // Fallback
                        format!("[?{}]", name)
                    }
                }
            })
            .collect()
    }
    
    /// Intelligent rendering for compound tokens like "G_N_I"
    fn render_compound_token(&self, compound_name: &str) -> Option<String> {
        
        // Check if this looks like a compound token (contains underscores)
        if !compound_name.contains('_') {
            return None;
        }
        
        let parts: Vec<&str> = compound_name.split('_').collect();
        
        // Re-combine parts that are actually single tokens like "I_MATRA"
        let mut fixed_parts = Vec::new();
        let mut i = 0;
        while i < parts.len() {
            if i + 1 < parts.len() && parts[i + 1] == "MATRA" {
                // Combine this part with "_MATRA"
                fixed_parts.push(format!("{}_{}", parts[i], parts[i + 1]));
                i += 2;
            } else {
                fixed_parts.push(parts[i].to_string());
                i += 1;
            }
        }
        let mut result = String::new();
        
        // For romanization schemes, render conjunct consonants without inherent 'a'
        if self.is_romanization_scheme() {
            for (i, part) in fixed_parts.iter().enumerate() {
                if let Some(output) = self.token_to_output.get(&part.to_string()) {
                    if i < fixed_parts.len() - 1 && self.is_syllable_consonant_token(part) {
                        // Consonants before the last part in conjunct - remove inherent 'a'
                        let stripped = self.remove_inherent_a(output);
                        result.push_str(&stripped);
                    } else {
                        // Last part or vowels - keep as is
                        result.push_str(output);
                    }
                } else {
                    return None; // Can't render unknown part
                }
            }
        } else {
            // For Devanagari and other Indic scripts, just concatenate
            for part in fixed_parts {
                if let Some(output) = self.token_to_output.get(&part.to_string()) {
                    result.push_str(output);
                } else {
                    return None;
                }
            }
        }
        
        Some(result)
    }
    
    /// Check if this scheme is a romanization scheme
    fn is_romanization_scheme(&self) -> bool {
        // Simple heuristic: if 'A' maps to 'a', it's probably romanization
        self.token_to_output.get("A").map_or(false, |s| s == "a")
    }
    
    /// Check if a token represents a syllable consonant (with inherent 'a')
    fn is_syllable_consonant_token(&self, token: &str) -> bool {
        matches!(token, 
            "KA" | "KHA" | "GA" | "GHA" | "NGA" |
            "CA" | "CHA" | "JA" | "JHA" | "NYA" |
            "TTA" | "TTHA" | "DDA" | "DDHA" | "NNA" |
            "TA" | "THA" | "DA" | "DHA" | "NA" |
            "PA" | "PHA" | "BA" | "BHA" | "MA" |
            "YA" | "RA" | "LA" | "LLA" | "VA" |
            "SHA" | "SSA" | "SA" | "HA"
        )
    }
    
    /// Check if a token represents a vowel
    fn is_vowel_token(&self, token: &str) -> bool {
        matches!(token, "A" | "AA" | "I" | "II" | "U" | "UU" | "RI" | "RII" | "LI" | "LII" | "E" | "AI" | "O" | "AU")
    }
    
    /// Remove inherent 'a' from a consonant rendering
    fn remove_inherent_a(&self, consonant_with_a: &str) -> String {
        // For most romanization schemes, just remove the trailing 'a'
        if consonant_with_a.ends_with('a') && consonant_with_a.len() > 1 {
            consonant_with_a[..consonant_with_a.len()-1].to_string()
        } else {
            consonant_with_a.to_string()
        }
    }
}

/// Main transliteration compiler
pub struct TransliterationCompiler {
    schemes: HashMap<String, SchemeDefinition>,
}

impl TransliterationCompiler {
    pub fn new() -> Self {
        Self {
            schemes: HashMap::new(),
        }
    }
    
    /// Add a scheme from TOML content
    pub fn add_scheme_from_toml(&mut self, scheme_name: &str, toml_content: &str) -> Result<(), TransliterationError> {
        let mappings = self.parse_toml_mappings(toml_content)?;
        let scheme = SchemeDefinition::from_toml_mappings(scheme_name.to_string(), mappings);
        self.schemes.insert(scheme_name.to_string(), scheme);
        Ok(())
    }
    
    /// Parse TOML content to extract mappings
    fn parse_toml_mappings(&self, toml_content: &str) -> Result<HashMap<String, String>, TransliterationError> {
        #[derive(serde::Deserialize)]
        struct TomlSchema {
            vowels: Option<HashMap<String, String>>,
            vowel_marks: Option<HashMap<String, String>>,
            consonants: Option<HashMap<String, String>>,
            special_marks: Option<HashMap<String, String>>,
            digits: Option<HashMap<String, String>>,
            punctuation: Option<HashMap<String, String>>,
            additional_consonants: Option<HashMap<String, String>>,
            ligatures: Option<HashMap<String, String>>,
            mappings: Option<HashMap<String, String>>,
        }
        
        let schema: TomlSchema = toml::from_str(toml_content)
            .map_err(|e| TransliterationError::ProcessingError(format!("TOML parse error: {}", e)))?;
        
        let mut all_mappings = HashMap::new();
        
        // Collect from all sections
        if let Some(vowels) = schema.vowels { all_mappings.extend(vowels); }
        if let Some(vowel_marks) = schema.vowel_marks { all_mappings.extend(vowel_marks); }
        if let Some(consonants) = schema.consonants { all_mappings.extend(consonants); }
        if let Some(special_marks) = schema.special_marks { all_mappings.extend(special_marks); }
        if let Some(digits) = schema.digits { all_mappings.extend(digits); }
        if let Some(punctuation) = schema.punctuation { all_mappings.extend(punctuation); }
        if let Some(additional_consonants) = schema.additional_consonants { all_mappings.extend(additional_consonants); }
        if let Some(ligatures) = schema.ligatures { all_mappings.extend(ligatures); }
        if let Some(flat_mappings) = schema.mappings { all_mappings.extend(flat_mappings); }
        
        Ok(all_mappings)
    }
    
    /// Load all schemes from schemas directory
    pub fn load_builtin_schemes(&mut self) -> Result<(), TransliterationError> {
        // Find schemas directory and load all schemas recursively
        let possible_schema_roots = [
            "../shlesha/schemas",   // From data directory
            "../../schemas",       // From build directory  
            "../schemas",          // Alternative
            "./schemas",           // Current directory
            "schemas",             // Direct subdirectory
        ];
        
        // eprintln!("DEBUG: Looking for schemas in possible roots: {:?}", possible_schema_roots);
        
        for schema_root in &possible_schema_roots {
            if Path::new(schema_root).exists() {
                // eprintln!("DEBUG: Found schemas directory: {}", schema_root);
                return self.load_schemas_from_directory(schema_root);
            }
        }
        
        Err(TransliterationError::ProcessingError(
            "Schemas directory not found".to_string()
        ))
    }
    
    /// Dynamically load all .toml files from schemas directory
    pub fn load_schemas_from_directory(&mut self, schemas_dir: &str) -> Result<(), TransliterationError> {
        use std::fs;
        use std::path::Path;
        
        let schema_path = Path::new(schemas_dir);
        if !schema_path.exists() {
            return Err(TransliterationError::ProcessingError(
                format!("Schemas directory not found: {}", schemas_dir)
            ));
        }
        
        // Recursively find all .toml files
        let toml_files = self.find_toml_files(schema_path)?;
        
        for toml_file in toml_files {
            let content = fs::read_to_string(&toml_file)
                .map_err(|e| TransliterationError::ProcessingError(
                    format!("Failed to read {}: {}", toml_file.display(), e)
                ))?;
            
            // Extract scheme name from metadata or filename
            let scheme_name = self.extract_scheme_name(&content, &toml_file)?;
            // eprintln!("DEBUG: Loading scheme '{}' from {}", scheme_name, toml_file.display());
            
            self.add_scheme_from_toml(&scheme_name, &content)?;
        }
        
        Ok(())
    }
    
    /// Recursively find all .toml files in directory
    fn find_toml_files(&self, dir: &Path) -> Result<Vec<std::path::PathBuf>, TransliterationError> {
        use std::fs;
        
        let mut toml_files = Vec::new();
        
        if dir.is_dir() {
            let entries = fs::read_dir(dir)
                .map_err(|e| TransliterationError::ProcessingError(
                    format!("Failed to read directory {}: {}", dir.display(), e)
                ))?;
            
            for entry in entries {
                let entry = entry.map_err(|e| TransliterationError::ProcessingError(
                    format!("Failed to read directory entry: {}", e)
                ))?;
                let path = entry.path();
                
                if path.is_dir() {
                    // Recursively search subdirectories
                    toml_files.extend(self.find_toml_files(&path)?);
                } else if path.extension().map_or(false, |ext| ext == "toml") {
                    toml_files.push(path);
                }
            }
        }
        
        Ok(toml_files)
    }
    
    /// Extract scheme name from TOML metadata or filename
    fn extract_scheme_name(&self, content: &str, file_path: &Path) -> Result<String, TransliterationError> {
        // First try to get name from TOML metadata
        #[derive(serde::Deserialize)]
        struct MetadataOnly {
            metadata: Option<MetadataSection>,
        }
        
        #[derive(serde::Deserialize)]
        struct MetadataSection {
            name: Option<String>,
            scheme_id: Option<String>,
        }
        
        if let Ok(parsed) = toml::from_str::<MetadataOnly>(content) {
            if let Some(metadata) = parsed.metadata {
                if let Some(scheme_id) = metadata.scheme_id {
                    return Ok(scheme_id);
                }
                if let Some(name) = metadata.name {
                    // Convert display name to scheme ID
                    return Ok(name.to_lowercase().replace(" ", "_"));
                }
            }
        }
        
        // Fallback to filename without extension
        let filename = file_path.file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| TransliterationError::ProcessingError(
                format!("Invalid filename: {}", file_path.display())
            ))?;
        
        Ok(filename.to_string())
    }
    
    /// Parse input text to tokens using specified scheme
    pub fn parse(&self, text: &str, scheme: TargetScheme) -> Result<Vec<TokenWithMetadata>, TransliterationError> {
        let scheme_name = scheme.to_string();
        let scheme_def = self.schemes.get(&scheme_name)
            .ok_or_else(|| TransliterationError::UnsupportedScheme(scheme_name))?;
        
        Ok(scheme_def.parse(text))
    }
    
    /// Render tokens to output using specified scheme
    pub fn render(&self, tokens: &[TokenWithMetadata], scheme: TargetScheme) -> Result<String, TransliterationError> {
        let scheme_name = scheme.to_string();
        let scheme_def = self.schemes.get(&scheme_name)
            .ok_or_else(|| TransliterationError::UnsupportedScheme(scheme_name))?;
        
        Ok(scheme_def.render(tokens))
    }
    
    /// Get available schemes
    pub fn available_schemes(&self) -> Vec<&str> {
        self.schemes.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for TransliterationCompiler {
    fn default() -> Self {
        let mut compiler = Self::new();
        if let Err(e) = compiler.load_builtin_schemes() {
            eprintln!("Warning: Failed to load builtin schemes: {}", e);
        }
        compiler
    }
}