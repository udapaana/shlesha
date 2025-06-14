//! Sanskrit token system - the compiler's intermediate representation
//! 
//! Tokens represent the atomic units of Sanskrit text, providing a 
//! script-independent representation for perfect round-trip transliteration.

use fxhash::FxHashMap;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicU32, Ordering};
use once_cell::sync::Lazy;

pub mod levels;
pub use levels::*;

/// Global token registry for efficient lookups with fast hash
static TOKEN_REGISTRY: Lazy<Arc<RwLock<FxHashMap<String, u32>>>> = 
    Lazy::new(|| Arc::new(RwLock::new(FxHashMap::default())));

static TOKEN_COUNTER: AtomicU32 = AtomicU32::new(0);

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
    /// Register a new token or return existing one (optimized with atomic counter)
    pub fn register(name: String) -> Self {
        // Fast path: check if token already exists
        {
            let registry = TOKEN_REGISTRY.read().unwrap();
            if let Some(&id) = registry.get(&name) {
                return SanskritToken::Named(name, id);
            }
        }
        
        // Slow path: need to register new token
        let mut registry = TOKEN_REGISTRY.write().unwrap();
        
        // Double-check after acquiring write lock
        if let Some(&id) = registry.get(&name) {
            return SanskritToken::Named(name, id);
        }
        
        let id = TOKEN_COUNTER.fetch_add(1, Ordering::Relaxed);
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
    
    /// Create from string (optimized lookup)
    pub fn from_name(name: &str) -> Self {
        match name {
            "SPACE" => SanskritToken::Space,
            _ => {
                // Try fast path first
                {
                    let registry = TOKEN_REGISTRY.read().unwrap();
                    if let Some(&id) = registry.get(name) {
                        return SanskritToken::Named(name.to_string(), id);
                    }
                }
                // Register if not found
                Self::register(name.to_string())
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