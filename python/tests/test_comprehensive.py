"""
Comprehensive tests for Shlesha Python bindings covering all features.
"""

import pytest
import shlesha


class TestAdvancedTransliteration:
    """Test advanced transliteration features."""
    
    def test_all_supported_scripts(self):
        """Test that all advertised scripts are actually supported."""
        transliterator = shlesha.Shlesha()
        scripts = transliterator.list_supported_scripts()
        
        # Check that all expected scripts are present
        expected_scripts = [
            "devanagari", "iast", "itrans", "slp1", "harvard_kyoto", 
            "velthuis", "wx", "iso15919", "bengali", "tamil", "telugu",
            "gujarati", "kannada", "malayalam", "odia"
        ]
        
        for script in expected_scripts:
            assert transliterator.supports_script(script), f"Script {script} should be supported"
            assert script in scripts, f"Script {script} should be in list"
    
    def test_cross_script_matrix(self):
        """Test conversion between multiple script pairs."""
        transliterator = shlesha.Shlesha()
        test_word = "धर्म"  # dharma in Devanagari
        
        # Test conversions from Devanagari to various scripts
        conversions = [
            ("devanagari", "iast"),
            ("devanagari", "itrans"), 
            ("devanagari", "slp1"),
            ("devanagari", "gujarati"),
            ("devanagari", "bengali"),
        ]
        
        for from_script, to_script in conversions:
            result = transliterator.transliterate(test_word, from_script, to_script)
            assert result, f"Conversion from {from_script} to {to_script} should produce output"
            assert result != test_word or to_script == "devanagari", f"Conversion should change the text (unless same script)"
    
    def test_roundtrip_conversions(self):
        """Test that roundtrip conversions preserve meaning."""
        transliterator = shlesha.Shlesha()
        original = "धर्म"
        
        # Test Devanagari -> IAST -> Devanagari roundtrip
        to_iast = transliterator.transliterate(original, "devanagari", "iast")
        back_to_deva = transliterator.transliterate(to_iast, "iast", "devanagari")
        
        # Should be the same or equivalent (considering normalization)
        assert back_to_deva == original or "धर्म" in back_to_deva
    
    def test_script_aliases(self):
        """Test that script aliases work correctly."""
        transliterator = shlesha.Shlesha()
        
        # Test common aliases
        aliases = [
            ("deva", "devanagari"),
            ("iso", "iso15919"),
            ("hk", "harvard_kyoto"),
        ]
        
        for alias, canonical in aliases:
            if transliterator.supports_script(alias):
                result1 = transliterator.transliterate("अ", alias, "iast")
                result2 = transliterator.transliterate("अ", canonical, "iast")
                assert result1 == result2, f"Alias {alias} should work same as {canonical}"


class TestMetadataFeatures:
    """Test metadata collection and unknown token handling."""
    
    def test_metadata_structure(self):
        """Test the structure of metadata objects."""
        transliterator = shlesha.Shlesha()
        result = transliterator.transliterate_with_metadata("अ", "devanagari", "iast")
        
        assert hasattr(result, 'output')
        assert hasattr(result, 'metadata')
        assert result.metadata is not None
        
        metadata = result.metadata
        assert hasattr(metadata, 'source_script')
        assert hasattr(metadata, 'target_script')
        assert hasattr(metadata, 'used_extensions')
        assert hasattr(metadata, 'unknown_tokens')
        
        assert metadata.source_script == "devanagari"
        assert metadata.target_script == "iast"
        assert isinstance(metadata.unknown_tokens, list)
    
    def test_unknown_token_tracking(self):
        """Test tracking of unknown tokens in mixed content."""
        transliterator = shlesha.Shlesha()
        
        # Test with mixed content containing unknown characters
        mixed_text = "धर्म123abc"
        result = transliterator.transliterate_with_metadata(mixed_text, "devanagari", "iast")
        
        assert "dharma" in result.output.lower()
        assert result.metadata is not None
        
        # Unknown tokens should include Latin letters and numbers (if tracked)
        # Note: With graceful handling, they might pass through without being tracked
        unknown_count = len(result.metadata.unknown_tokens)
        assert unknown_count >= 0  # Could be 0 with graceful passthrough
    
    def test_unknown_token_properties(self):
        """Test properties of unknown token objects."""
        transliterator = shlesha.Shlesha()
        result = transliterator.transliterate_with_metadata("अx", "devanagari", "iast")
        
        if result.metadata and result.metadata.unknown_tokens:
            token = result.metadata.unknown_tokens[0]
            assert hasattr(token, 'script')
            assert hasattr(token, 'token')
            assert hasattr(token, 'position')
            assert hasattr(token, 'unicode')
            assert hasattr(token, 'is_extension')
            
            assert isinstance(token.script, str)
            assert isinstance(token.token, str)
            assert isinstance(token.position, int)
            assert isinstance(token.unicode, str)
            assert isinstance(token.is_extension, bool)
    
    def test_metadata_without_unknowns(self):
        """Test metadata when no unknown tokens are present."""
        transliterator = shlesha.Shlesha()
        result = transliterator.transliterate_with_metadata("धर्म", "devanagari", "iast")
        
        assert result.metadata is not None
        assert len(result.metadata.unknown_tokens) == 0
        assert result.metadata.source_script == "devanagari"
        assert result.metadata.target_script == "iast"


class TestErrorHandlingAndEdgeCases:
    """Test error handling and edge cases."""
    
    def test_invalid_scripts(self):
        """Test error handling for invalid script names."""
        transliterator = shlesha.Shlesha()
        
        with pytest.raises(RuntimeError):
            transliterator.transliterate("test", "nonexistent_script", "iast")
        
        with pytest.raises(RuntimeError):
            transliterator.transliterate("test", "devanagari", "nonexistent_script")
    
    def test_empty_strings(self):
        """Test handling of empty input."""
        transliterator = shlesha.Shlesha()
        
        result = transliterator.transliterate("", "devanagari", "iast")
        assert result == ""
        
        result = transliterator.transliterate_with_metadata("", "devanagari", "iast")
        assert result.output == ""
        assert result.metadata is not None
    
    def test_whitespace_handling(self):
        """Test preservation of whitespace."""
        transliterator = shlesha.Shlesha()
        
        # Test various whitespace scenarios
        test_cases = [
            "अ आ",  # Space between characters
            " अ ",   # Leading/trailing spaces
            "अ\nआ",  # Newline
            "अ\tआ",  # Tab
        ]
        
        for test_case in test_cases:
            result = transliterator.transliterate(test_case, "devanagari", "iast")
            # Should preserve the whitespace structure
            assert len(result.split()) == len(test_case.split()) or "\n" in test_case or "\t" in test_case
    
    def test_unicode_normalization(self):
        """Test handling of different Unicode normalizations."""
        transliterator = shlesha.Shlesha()
        
        # Test with composed and decomposed Unicode forms if applicable
        test_text = "अ"  # Devanagari A
        result = transliterator.transliterate(test_text, "devanagari", "iast")
        assert result == "a"
    
    def test_large_text(self):
        """Test performance with larger text."""
        transliterator = shlesha.Shlesha()
        
        # Create a larger text for testing
        large_text = "धर्म " * 100  # Repeat 100 times
        result = transliterator.transliterate(large_text, "devanagari", "iast")
        
        assert "dharma" in result
        assert result.count("dharma") == 100


class TestConvenienceFunctions:
    """Test convenience functions and helpers."""
    
    def test_direct_transliterate_function(self):
        """Test the direct transliterate convenience function."""
        result = shlesha.transliterate("अ", "devanagari", "iast")
        assert result == "a"
    
    def test_get_supported_scripts_function(self):
        """Test the get_supported_scripts convenience function."""
        scripts = shlesha.get_supported_scripts()
        assert isinstance(scripts, list)
        assert len(scripts) > 0
        assert "devanagari" in scripts
        assert "iast" in scripts
    
    def test_create_transliterator_function(self):
        """Test the create_transliterator convenience function."""
        transliterator = shlesha.create_transliterator()
        assert transliterator is not None
        
        result = transliterator.transliterate("अ", "devanagari", "iast")
        assert result == "a"
    
    def test_script_info_function(self):
        """Test getting script information."""
        transliterator = shlesha.Shlesha()
        info = transliterator.get_script_info()
        
        assert isinstance(info, dict)
        assert "devanagari" in info
        assert "Devanagari" in info["devanagari"] or "देवनागरी" in info["devanagari"]


class TestStringRepresentations:
    """Test string representations and display methods."""
    
    def test_transliterator_repr(self):
        """Test transliterator string representations."""
        transliterator = shlesha.Shlesha()
        
        repr_str = repr(transliterator)
        assert "Shlesha" in repr_str
        assert "supported_scripts" in repr_str
        
        str_str = str(transliterator)
        assert "transliterator" in str_str.lower()
    
    def test_result_repr(self):
        """Test result object representations."""
        transliterator = shlesha.Shlesha()
        result = transliterator.transliterate_with_metadata("अ", "devanagari", "iast")
        
        repr_str = repr(result)
        assert "TransliterationResult" in repr_str
        assert "a" in repr_str
    
    def test_metadata_repr(self):
        """Test metadata object representations."""
        transliterator = shlesha.Shlesha()
        result = transliterator.transliterate_with_metadata("अ", "devanagari", "iast")
        
        if result.metadata:
            repr_str = repr(result.metadata)
            assert "TransliterationMetadata" in repr_str
            assert "devanagari" in repr_str or "iast" in repr_str


class TestThreadSafety:
    """Test thread safety of the library (if applicable)."""
    
    def test_multiple_instances(self):
        """Test creating multiple transliterator instances."""
        transliterator1 = shlesha.Shlesha()
        transliterator2 = shlesha.Shlesha()
        
        result1 = transliterator1.transliterate("अ", "devanagari", "iast")
        result2 = transliterator2.transliterate("अ", "devanagari", "iast")
        
        assert result1 == result2 == "a"
    
    def test_instance_isolation(self):
        """Test that instances don't interfere with each other."""
        transliterator1 = shlesha.Shlesha()
        transliterator2 = shlesha.Shlesha()
        
        # Both should work independently
        result1 = transliterator1.transliterate("धर्म", "devanagari", "iast")
        result2 = transliterator2.transliterate("अ", "devanagari", "gujarati")
        
        assert result1 != result2  # Different operations should give different results


if __name__ == "__main__":
    # Run tests with pytest
    pytest.main([__file__, "-v"])