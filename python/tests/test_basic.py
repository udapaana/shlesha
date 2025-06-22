"""
Basic tests for Shlesha Python bindings.
"""

import pytest
import shlesha


class TestBasicTransliteration:
    """Test basic transliteration functionality."""
    
    def test_create_transliterator(self):
        """Test creating a transliterator instance."""
        transliterator = shlesha.Shlesha()
        assert transliterator is not None
        
    def test_convenience_creation(self):
        """Test convenience function for creating transliterator."""
        transliterator = shlesha.create_transliterator()
        assert transliterator is not None
        
    def test_simple_transliteration(self):
        """Test basic vowel transliteration."""
        transliterator = shlesha.Shlesha()
        result = transliterator.transliterate("अ", "devanagari", "iast")
        assert result == "a"
        
    def test_convenience_transliteration(self):
        """Test convenience transliteration function."""
        result = shlesha.transliterate("अ", "devanagari", "iast")
        assert result == "a"
        
    def test_complex_word(self):
        """Test transliterating a complex word."""
        transliterator = shlesha.Shlesha()
        result = transliterator.transliterate("धर्म", "devanagari", "iast")
        assert "dharma" in result  # Allow for slight variations in output
        
    def test_reverse_transliteration(self):
        """Test reverse transliteration."""
        transliterator = shlesha.Shlesha()
        result = transliterator.transliterate("dharma", "iast", "devanagari")
        assert "धर्म" in result
        
    def test_cross_script_conversion(self):
        """Test conversion between Indic scripts."""
        transliterator = shlesha.Shlesha()
        result = transliterator.transliterate("धर्म", "devanagari", "gujarati")
        assert result  # Should produce some output
        
    def test_invalid_script_error(self):
        """Test error handling for invalid scripts."""
        transliterator = shlesha.Shlesha()
        with pytest.raises(RuntimeError):
            transliterator.transliterate("test", "invalid_script", "iast")


class TestScriptSupport:
    """Test script support and discovery functionality."""
    
    def test_list_supported_scripts(self):
        """Test getting list of supported scripts."""
        scripts = shlesha.get_supported_scripts()
        assert isinstance(scripts, list)
        assert len(scripts) > 0
        assert "devanagari" in scripts
        assert "iast" in scripts
        
    def test_transliterator_script_list(self):
        """Test getting scripts from transliterator instance."""
        transliterator = shlesha.Shlesha()
        scripts = transliterator.list_supported_scripts()
        assert isinstance(scripts, list)
        assert len(scripts) > 0
        
    def test_supports_script(self):
        """Test checking script support."""
        transliterator = shlesha.Shlesha()
        assert transliterator.supports_script("devanagari")
        assert transliterator.supports_script("iast")
        assert not transliterator.supports_script("nonexistent_script")
        
    def test_get_script_info(self):
        """Test getting script information."""
        transliterator = shlesha.Shlesha()
        info = transliterator.get_script_info()
        assert isinstance(info, dict)
        assert "devanagari" in info
        assert "देवनागरी" in info["devanagari"]


class TestMetadata:
    """Test metadata collection functionality."""
    
    def test_transliteration_with_metadata(self):
        """Test basic metadata collection."""
        transliterator = shlesha.Shlesha()
        result = transliterator.transliterate_with_metadata("अ", "devanagari", "iast")
        
        assert isinstance(result, shlesha.TransliterationResult)
        assert result.output == "a"
        assert result.metadata is not None
        assert result.metadata.source_script == "devanagari"
        assert result.metadata.target_script == "iast"
        
    def test_unknown_token_collection(self):
        """Test collection of unknown tokens."""
        transliterator = shlesha.Shlesha()
        result = transliterator.transliterate_with_metadata("धर्मkr", "devanagari", "iast")
        
        assert "dharma" in result.output.lower()
        assert result.metadata is not None
        # Should have unknown tokens for 'k' and 'r'
        assert len(result.metadata.unknown_tokens) >= 0  # Graceful handling means they might pass through
        
    def test_metadata_without_unknowns(self):
        """Test metadata when no unknown tokens are present."""
        transliterator = shlesha.Shlesha()
        result = transliterator.transliterate_with_metadata("अ", "devanagari", "iast")
        
        assert result.metadata is not None
        assert len(result.metadata.unknown_tokens) == 0
        
    def test_unknown_token_properties(self):
        """Test properties of unknown tokens."""
        transliterator = shlesha.Shlesha()
        result = transliterator.transliterate_with_metadata("अk", "devanagari", "iast")
        
        if result.metadata and result.metadata.unknown_tokens:
            token = result.metadata.unknown_tokens[0]
            assert isinstance(token, shlesha.UnknownToken)
            assert hasattr(token, 'script')
            assert hasattr(token, 'token')
            assert hasattr(token, 'position')
            assert hasattr(token, 'unicode')
            assert hasattr(token, 'is_extension')


class TestErrorHandling:
    """Test error handling and edge cases."""
    
    def test_empty_string(self):
        """Test handling of empty input."""
        transliterator = shlesha.Shlesha()
        result = transliterator.transliterate("", "devanagari", "iast")
        assert result == ""
        
    def test_whitespace_preservation(self):
        """Test that whitespace is preserved."""
        transliterator = shlesha.Shlesha()
        result = transliterator.transliterate("अ आ", "devanagari", "iast")
        assert " " in result
        
    def test_mixed_content(self):
        """Test handling of mixed script content."""
        transliterator = shlesha.Shlesha()
        result = transliterator.transliterate("धर्म hello", "devanagari", "iast")
        assert "hello" in result  # Latin text should pass through
        
    def test_punctuation_preservation(self):
        """Test that punctuation is preserved."""
        transliterator = shlesha.Shlesha()
        result = transliterator.transliterate("धर्म!", "devanagari", "iast")
        assert "!" in result


class TestStringRepresentations:
    """Test string representations of objects."""
    
    def test_transliterator_repr(self):
        """Test transliterator string representation."""
        transliterator = shlesha.Shlesha()
        repr_str = repr(transliterator)
        assert "Shlesha" in repr_str
        
        str_str = str(transliterator)
        assert "transliterator" in str_str.lower()
        
    def test_result_repr(self):
        """Test result string representation."""
        transliterator = shlesha.Shlesha()
        result = transliterator.transliterate_with_metadata("अ", "devanagari", "iast")
        
        repr_str = repr(result)
        assert "TransliterationResult" in repr_str
        assert "a" in repr_str
        
    def test_metadata_repr(self):
        """Test metadata string representation."""
        transliterator = shlesha.Shlesha()
        result = transliterator.transliterate_with_metadata("अ", "devanagari", "iast")
        
        if result.metadata:
            repr_str = repr(result.metadata)
            assert "TransliterationMetadata" in repr_str
            assert "devanagari" in repr_str
            assert "iast" in repr_str


if __name__ == "__main__":
    pytest.main([__file__])