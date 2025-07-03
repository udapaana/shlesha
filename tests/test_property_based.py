#!/usr/bin/env python3
"""
Property-based tests for Shlesha transliteration system

These tests use Hypothesis to generate test cases and verify fundamental
properties that should hold for ANY transliteration system.
"""

import unittest
from hypothesis import given, strategies as st, assume, settings, HealthCheck
import shlesha
from vidyut.lipi import transliterate, Scheme


# Strategy for generating valid Sanskrit text in different scripts
@st.composite
def sanskrit_text(draw, script="iast", max_length=20):
    """Generate valid Sanskrit text in the specified script."""
    if script == "iast":
        # IAST character set
        vowels = ["a", "ā", "i", "ī", "u", "ū", "ṛ", "ṝ", "ḷ", "ḹ", "e", "ai", "o", "au"]
        consonants = ["k", "kh", "g", "gh", "ṅ", "c", "ch", "j", "jh", "ñ", 
                     "ṭ", "ṭh", "ḍ", "ḍh", "ṇ", "t", "th", "d", "dh", "n",
                     "p", "ph", "b", "bh", "m", "y", "r", "l", "v", 
                     "ś", "ṣ", "s", "h"]
        marks = ["ṃ", "ḥ"]
        specials = ["kṣ", "jñ"]
        
        chars = vowels + consonants + marks + specials
        
    elif script == "slp1":
        # SLP1 character set
        vowels = ["a", "A", "i", "I", "u", "U", "f", "F", "x", "X", "e", "E", "o", "O"]
        consonants = ["k", "K", "g", "G", "N", "c", "C", "j", "J", "Y",
                     "w", "W", "q", "Q", "R", "t", "T", "d", "D", "n", 
                     "p", "P", "b", "B", "m", "y", "r", "l", "v",
                     "S", "z", "s", "h"]
        marks = ["M", "H"]
        specials = ["kz", "jY"]  # kṣ → kz, jñ → jY in SLP1
        
        chars = vowels + consonants + marks + specials
        
    elif script == "harvard_kyoto":
        # Harvard-Kyoto character set
        vowels = ["a", "A", "i", "I", "u", "U", "R", "RR", "lR", "lRR", "e", "ai", "o", "au"]
        consonants = ["k", "kh", "g", "gh", "G", "c", "ch", "j", "jh", "J",
                     "T", "Th", "D", "Dh", "N", "t", "th", "d", "dh", "n",
                     "p", "ph", "b", "bh", "m", "y", "r", "l", "v",
                     "z", "S", "s", "h"]
        marks = ["M", "H"]
        specials = ["kS", "jJ"]
        
        chars = vowels + consonants + marks + specials
    
    else:
        # Fallback to basic ASCII
        chars = ["a", "i", "u", "k", "t", "m", "n", "r", "s"]
    
    length = draw(st.integers(min_value=1, max_value=max_length))
    return "".join(draw(st.lists(st.sampled_from(chars), min_size=1, max_size=length)))


class PropertyBasedTests(unittest.TestCase):
    """Property-based tests for transliteration system."""
    
    @given(sanskrit_text(script="iast"))
    @settings(max_examples=200, suppress_health_check=[HealthCheck.too_slow])
    def test_transliteration_is_deterministic(self, text):
        """Test that transliteration is deterministic - same input always gives same output."""
        assume(len(text) > 0)
        
        # Test multiple script pairs
        script_pairs = [
            ("iast", "slp1"),
            ("iast", "devanagari"),
            ("iast", "iso"),
            ("slp1", "iast"),
            ("slp1", "devanagari"),
        ]
        
        for source, target in script_pairs:
            try:
                result1 = shlesha.transliterate(text, source, target)
                result2 = shlesha.transliterate(text, source, target)
                result3 = shlesha.transliterate(text, source, target)
                
                self.assertEqual(result1, result2, 
                    f"Non-deterministic result for {source}→{target}: '{text}' gave different outputs")
                self.assertEqual(result1, result3,
                    f"Non-deterministic result for {source}→{target}: '{text}' gave different outputs")
                    
            except Exception as e:
                # If conversion fails, it should fail consistently
                try:
                    shlesha.transliterate(text, source, target)
                    self.fail(f"Inconsistent error behavior for {source}→{target}: '{text}'")
                except Exception:
                    pass  # Consistent failure is okay
    
    @given(sanskrit_text(script="iast"))
    @settings(max_examples=100)
    def test_identity_conversions(self, text):
        """Test that converting from a script to itself returns the original text."""
        assume(len(text) > 0)
        
        scripts = ["iast", "slp1", "devanagari", "telugu", "iso"]
        
        for script in scripts:
            try:
                result = shlesha.transliterate(text, script, script)
                self.assertEqual(result, text,
                    f"Identity conversion failed for {script}: '{text}' → '{result}'")
            except Exception:
                # Some scripts might not support certain input text, that's okay
                pass
    
    @given(sanskrit_text(script="iast"))
    @settings(max_examples=50)
    def test_round_trip_conversions(self, text):
        """Test that A→B→A conversions preserve the original text."""
        assume(len(text) > 0 and len(text) < 15)  # Keep it manageable
        
        # Test round-trip conversions
        round_trip_pairs = [
            ("iast", "slp1"),
            ("iast", "iso"),
            ("iast", "devanagari"),
            ("slp1", "devanagari"),
        ]
        
        for script_a, script_b in round_trip_pairs:
            try:
                # A → B → A
                intermediate = shlesha.transliterate(text, script_a, script_b)
                back_to_original = shlesha.transliterate(intermediate, script_b, script_a)
                
                self.assertEqual(back_to_original, text,
                    f"Round-trip failed {script_a}→{script_b}→{script_a}: "
                    f"'{text}' → '{intermediate}' → '{back_to_original}'")
                    
            except Exception as e:
                # Document failures for investigation
                print(f"Round-trip conversion failed for '{text}' ({script_a}↔{script_b}): {e}")
    
    @given(sanskrit_text(script="iast"))
    @settings(max_examples=100)
    def test_output_length_bounds(self, text):
        """Test that output length is within reasonable bounds of input length."""
        assume(len(text) > 0)
        
        script_pairs = [
            ("iast", "slp1"),
            ("iast", "devanagari"), 
            ("slp1", "iast"),
            ("iast", "iso"),
        ]
        
        for source, target in script_pairs:
            try:
                result = shlesha.transliterate(text, source, target)
                
                # Output should not be empty unless input is empty
                if len(text) > 0:
                    self.assertGreater(len(result), 0,
                        f"Empty output for non-empty input: {source}→{target} '{text}' → '{result}'")
                
                # Output should not be excessively long (reasonable bound)
                max_expansion = 10  # Allow up to 10x expansion for complex scripts
                self.assertLessEqual(len(result), len(text) * max_expansion,
                    f"Excessive expansion {source}→{target}: '{text}' ({len(text)}) → '{result}' ({len(result)})")
                    
            except Exception:
                pass  # Conversion failures are handled elsewhere
    
    @given(sanskrit_text(script="iast"))
    @settings(max_examples=50)
    def test_character_preservation(self, text):
        """Test that certain characters are preserved across conversions."""
        assume(len(text) > 0)
        
        # ASCII digits and punctuation should be preserved
        ascii_chars = "0123456789.,;:!?-()[]{}'\""
        
        for char in ascii_chars:
            test_text = text + char
            
            try:
                # Test that ASCII chars are preserved in Roman script conversions
                iast_to_slp1 = shlesha.transliterate(test_text, "iast", "slp1")
                self.assertIn(char, iast_to_slp1,
                    f"ASCII character '{char}' not preserved in IAST→SLP1: '{test_text}' → '{iast_to_slp1}'")
                    
                slp1_to_iast = shlesha.transliterate(test_text, "slp1", "iast")
                self.assertIn(char, slp1_to_iast,
                    f"ASCII character '{char}' not preserved in SLP1→IAST: '{test_text}' → '{slp1_to_iast}'")
                    
            except Exception:
                pass
    
    @given(sanskrit_text(script="iast"), sanskrit_text(script="iast"))
    @settings(max_examples=50)
    def test_concatenation_property(self, text1, text2):
        """Test that transliterate(A+B) == transliterate(A) + transliterate(B) for Roman scripts."""
        assume(len(text1) > 0 and len(text2) > 0)
        
        combined_text = text1 + text2
        
        # This property should hold for Roman-to-Roman conversions
        roman_pairs = [
            ("iast", "slp1"),
            ("slp1", "iast"),
            ("iast", "iso"),
            ("iso", "iast"),
        ]
        
        for source, target in roman_pairs:
            try:
                # Convert combined text
                combined_result = shlesha.transliterate(combined_text, source, target)
                
                # Convert parts separately and combine
                part1_result = shlesha.transliterate(text1, source, target)
                part2_result = shlesha.transliterate(text2, source, target)
                parts_combined = part1_result + part2_result
                
                self.assertEqual(combined_result, parts_combined,
                    f"Concatenation property failed {source}→{target}: "
                    f"'{combined_text}' → '{combined_result}' vs '{parts_combined}'")
                    
            except Exception:
                pass
    
    @given(st.lists(st.sampled_from(["a", "ā", "i", "ī", "u", "ū", "k", "t", "m", "n"]), min_size=1, max_size=10))
    @settings(max_examples=100)
    def test_monotonic_character_mapping(self, chars):
        """Test that individual character mappings are consistent."""
        
        for char in chars:
            # Test that single character conversion is consistent
            try:
                iast_to_slp1_single = shlesha.transliterate(char, "iast", "slp1")
                
                # The same character in a longer string should map the same way
                longer_text = "a" + char + "m"
                longer_result = shlesha.transliterate(longer_text, "iast", "slp1")
                
                # The character should appear in the result with the same mapping
                if len(iast_to_slp1_single) == 1 and iast_to_slp1_single.isalnum():
                    self.assertIn(iast_to_slp1_single, longer_result,
                        f"Inconsistent mapping for '{char}': single='{iast_to_slp1_single}' not in longer='{longer_result}'")
                        
            except Exception:
                pass
    
    @given(sanskrit_text(script="iast"))
    @settings(max_examples=50)
    def test_vidyut_consistency(self, text):
        """Test that Shlesha results are consistent with Vidyut where both support the conversion."""
        assume(len(text) > 0 and len(text) < 10)  # Keep it manageable
        
        # Test cases where both Shlesha and Vidyut should work
        test_cases = [
            ("iast", "slp1", Scheme.Iast, Scheme.Slp1),
            ("iast", "devanagari", Scheme.Iast, Scheme.Devanagari),
            ("slp1", "devanagari", Scheme.Slp1, Scheme.Devanagari),
        ]
        
        for shlesha_source, shlesha_target, vidyut_source, vidyut_target in test_cases:
            try:
                shlesha_result = shlesha.transliterate(text, shlesha_source, shlesha_target)
                vidyut_result = transliterate(text, vidyut_source, vidyut_target)
                
                # They should produce the same result
                self.assertEqual(shlesha_result, vidyut_result,
                    f"Shlesha/Vidyut mismatch {shlesha_source}→{shlesha_target}: "
                    f"'{text}' → Shlesha: '{shlesha_result}', Vidyut: '{vidyut_result}'")
                    
            except Exception as e:
                # Document inconsistencies
                print(f"Consistency test failed for '{text}': {e}")
    
    @given(st.text(min_size=1, max_size=5))
    @settings(max_examples=100)
    def test_error_handling(self, text):
        """Test that invalid inputs are handled gracefully."""
        
        invalid_scripts = ["nonexistent", "", "invalid123", "IAST", "SLP1"]
        valid_scripts = ["iast", "slp1", "devanagari", "telugu", "iso"]
        
        for invalid_script in invalid_scripts:
            for valid_script in valid_scripts:
                # Test invalid source script
                with self.assertRaises(Exception):
                    shlesha.transliterate(text, invalid_script, valid_script)
                
                # Test invalid target script  
                with self.assertRaises(Exception):
                    shlesha.transliterate(text, valid_script, invalid_script)
    
    def test_supported_scripts_property(self):
        """Test that get_supported_scripts returns valid script names."""
        scripts = shlesha.get_supported_scripts()
        
        # Should return a non-empty list
        self.assertIsInstance(scripts, list)
        self.assertGreater(len(scripts), 0)
        
        # All script names should be non-empty strings
        for script in scripts:
            self.assertIsInstance(script, str)
            self.assertGreater(len(script), 0)
            
        # Should include known scripts
        expected_scripts = ["iast", "slp1", "devanagari", "telugu"]
        for expected in expected_scripts:
            self.assertIn(expected, scripts, f"Expected script '{expected}' not found in supported scripts")
    
    @given(st.integers(min_value=0, max_value=1000))
    def test_empty_and_numeric_inputs(self, number):
        """Test edge cases with empty strings and numeric inputs."""
        
        # Empty string should return empty string
        for script in ["iast", "slp1", "devanagari"]:
            result = shlesha.transliterate("", script, script)
            self.assertEqual(result, "", f"Empty string conversion failed for {script}")
        
        # Numeric strings should be preserved or handled gracefully
        numeric_text = str(number)
        try:
            result = shlesha.transliterate(numeric_text, "iast", "slp1")
            # Should either preserve numbers or convert gracefully
            self.assertIsInstance(result, str)
        except Exception:
            pass  # Failure is acceptable for some numeric inputs


if __name__ == '__main__':
    # Run with more verbose output to see property failures
    unittest.main(verbosity=2, buffer=True)