"""
Shlesha - High-Performance Extensible Transliteration Library

A comprehensive transliteration library for Sanskrit and Indic scripts with bidirectional 
conversion support using a hub-and-spoke architecture.

Classes:
    Shlesha: Main transliterator class
    TransliterationResult: Result object with output and metadata
    TransliterationMetadata: Metadata about the transliteration process
    UnknownToken: Information about unknown/untranslatable tokens

Functions:
    transliterate: Direct transliteration function
    get_supported_scripts: Get list of all supported scripts
    create_transliterator: Create a new Shlesha instance

Example:
    >>> from shlesha import Shlesha, transliterate
    >>> 
    >>> # Using the class
    >>> transliterator = Shlesha()
    >>> result = transliterator.transliterate("धर्म", "devanagari", "iast")
    >>> print(result)  # "dharma"
    >>> 
    >>> # Using the convenience function
    >>> result = transliterate("धर्म", "devanagari", "iast")
    >>> print(result)  # "dharma"
    >>> 
    >>> # With metadata collection
    >>> result = transliterator.transliterate_with_metadata("धर्मkr", "devanagari", "iast")
    >>> print(result.output)  # "dharmakr"
    >>> print(len(result.metadata.unknown_tokens))  # 2 (for 'k' and 'r')
"""

from .shlesha import (
    PyShlesha as Shlesha,
    PyTransliterationResult as TransliterationResult,
    PyTransliterationMetadata as TransliterationMetadata,
    PyUnknownToken as UnknownToken,
    create_transliterator,
    transliterate,
    get_supported_scripts,
    __version__,
    __author__,
    __description__,
)

__all__ = [
    "Shlesha",
    "TransliterationResult", 
    "TransliterationMetadata",
    "UnknownToken",
    "create_transliterator",
    "transliterate",
    "get_supported_scripts",
    "__version__",
    "__author__",
    "__description__",
]

# Supported scripts documentation
SUPPORTED_SCRIPTS = {
    # Romanization schemes
    "iast": "IAST (International Alphabet of Sanskrit Transliteration)",
    "itrans": "ITRANS (ASCII transliteration)", 
    "slp1": "SLP1 (Sanskrit Library Phonetic scheme)",
    "harvard_kyoto": "Harvard-Kyoto (ASCII-based academic standard)",
    "velthuis": "Velthuis (TeX-based notation)",
    "wx": "WX (Computational notation)",
    "iso15919": "ISO-15919 (International standard)",
    
    # Indic scripts
    "devanagari": "Devanagari script (देवनागरी)",
    "bengali": "Bengali script (বাংলা)", 
    "tamil": "Tamil script (தமிழ்)",
    "telugu": "Telugu script (తెలుగు)",
    "gujarati": "Gujarati script (ગુજરાતી)",
    "kannada": "Kannada script (ಕನ್ನಡ)",
    "malayalam": "Malayalam script (മലയാളം)",
    "odia": "Odia script (ଓଡ଼ିଆ)",
}

# Script aliases
SCRIPT_ALIASES = {
    "deva": "devanagari",
    "bn": "bengali", 
    "ta": "tamil",
    "te": "telugu",
    "gu": "gujarati",
    "kn": "kannada",
    "ml": "malayalam",
    "od": "odia",
    "oriya": "odia",
    "hk": "harvard_kyoto",
    "iso": "iso15919",
    "iso_15919": "iso15919",
    "bangla": "bengali",
}

def get_script_description(script_name: str) -> str:
    """
    Get description for a script name.
    
    Args:
        script_name: Name of the script
        
    Returns:
        Description string for the script
        
    Example:
        >>> get_script_description("devanagari")
        'Devanagari script (देवनागरी)'
    """
    # Resolve alias if needed
    resolved_name = SCRIPT_ALIASES.get(script_name, script_name)
    return SUPPORTED_SCRIPTS.get(resolved_name, "Unknown script")

def list_script_families() -> dict:
    """
    Get scripts organized by family/type.
    
    Returns:
        Dictionary with script families
        
    Example:
        >>> families = list_script_families()
        >>> print(families['indic'])  # ['devanagari', 'bengali', ...]
    """
    return {
        "romanization": [
            "iast", "itrans", "slp1", "harvard_kyoto", 
            "velthuis", "wx", "iso15919"
        ],
        "indic": [
            "devanagari", "bengali", "tamil", "telugu",
            "gujarati", "kannada", "malayalam", "odia"
        ]
    }

def validate_conversion_pair(from_script: str, to_script: str) -> bool:
    """
    Check if a conversion pair is supported.
    
    Args:
        from_script: Source script name
        to_script: Target script name
        
    Returns:
        True if conversion is supported
        
    Example:
        >>> validate_conversion_pair("devanagari", "iast")
        True
    """
    transliterator = create_transliterator()
    return (transliterator.supports_script(from_script) and 
            transliterator.supports_script(to_script))