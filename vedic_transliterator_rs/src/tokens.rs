//! Sanskrit token system - the compiler's intermediate representation
//! 
//! Tokens represent the atomic units of Sanskrit text, providing a 
//! script-independent representation for perfect round-trip transliteration.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;

pub mod levels;
pub use levels::*;

/// Global token registry for efficient lookups
static TOKEN_REGISTRY: Lazy<Arc<RwLock<HashMap<String, u32>>>> = 
    Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

static TOKEN_COUNTER: Lazy<Arc<RwLock<u32>>> = 
    Lazy::new(|| Arc::new(RwLock::new(0)));

/// Sanskrit token - the intermediate representation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SanskritToken {
    /// Named token with unique ID for fast comparison
    Named(String, u32),
    /// Unknown/unrecognized input
    Unknown(String),
    /// Whitespace/separator
    Space,
}

impl SanskritToken {
    /// Register a new token or return existing one
    pub fn register(name: String) -> Self {
        let mut registry = TOKEN_REGISTRY.write().unwrap();
        
        if let Some(&id) = registry.get(&name) {
            return SanskritToken::Named(name, id);
        }
        
        let mut counter = TOKEN_COUNTER.write().unwrap();
        let id = *counter;
        *counter += 1;
        
        registry.insert(name.clone(), id);
        SanskritToken::Named(name, id)
    }
    
    /// Get token name for display/debugging
    pub fn name(&self) -> &str {
        match self {
            SanskritToken::Named(name, _) => name,
            SanskritToken::Unknown(name) => name,
            SanskritToken::Space => "SPACE",
        }
    }
    
    /// Create from string
    pub fn from_name(name: &str) -> Self {
        match name {
            "SPACE" => SanskritToken::Space,
            _ => {
                let registry = TOKEN_REGISTRY.read().unwrap();
                if let Some(&id) = registry.get(name) {
                    SanskritToken::Named(name.to_string(), id)
                } else {
                    drop(registry);
                    Self::register(name.to_string())
                }
            }
        }
    }
    
    /// Check if token is unknown
    pub fn is_unknown(&self) -> bool {
        matches!(self, SanskritToken::Unknown(_))
    }
}

/// Token with parsing metadata
#[derive(Debug, Clone)]
pub struct TokenWithMetadata {
    pub token: SanskritToken,
    pub original_text: String,
    pub position: usize,
}

impl TokenWithMetadata {
    pub fn new(token: SanskritToken, original_text: String, position: usize) -> Self {
        Self {
            token,
            original_text,
            position,
        }
    }
}