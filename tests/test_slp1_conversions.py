#!/usr/bin/env python3
"""
Unit tests for SLP1 conversions

These tests verify that all Roman script conversions work correctly,
particularly the hub-based conversion system.
"""

import unittest
import shlesha
from vidyut.lipi import transliterate, Scheme


class TestSLP1Conversions(unittest.TestCase):
    """Test SLP1 conversion correctness."""
    
    def test_iast_to_slp1_individual_characters(self):
        """Test individual character mappings from IAST to SLP1."""
        mappings = [
            # Vowels
            ("a", "a"), ("ā", "A"), ("i", "i"), ("ī", "I"), 
            ("u", "u"), ("ū", "U"), ("ṛ", "f"), ("ṝ", "F"),
            ("ḷ", "x"), ("ḹ", "X"), ("e", "e"), ("o", "o"),
            ("ai", "E"), ("au", "O"),
            
            # Consonants
            ("k", "k"), ("kh", "K"), ("g", "g"), ("gh", "G"), ("ṅ", "N"),
            ("c", "c"), ("ch", "C"), ("j", "j"), ("jh", "J"), ("ñ", "Y"),
            ("ṭ", "w"), ("ṭh", "W"), ("ḍ", "q"), ("ḍh", "Q"), ("ṇ", "R"),
            ("t", "t"), ("th", "T"), ("d", "d"), ("dh", "D"), ("n", "n"),
            ("p", "p"), ("ph", "P"), ("b", "b"), ("bh", "B"), ("m", "m"),
            ("y", "y"), ("r", "r"), ("l", "l"), ("v", "v"),
            ("ś", "S"), ("ṣ", "z"), ("s", "s"), ("h", "h"),
            
            # Marks
            ("ṃ", "M"), ("ḥ", "H"),
            
            # Special combinations
            ("kṣ", "kz"), ("jñ", "jY"),
        ]
        
        failed_mappings = []
        
        for iast_char, expected_slp1 in mappings:
            with self.subTest(iast=iast_char, expected=expected_slp1):
                result = shlesha.transliterate(iast_char, 'iast', 'slp1')
                if result != expected_slp1:
                    failed_mappings.append((iast_char, expected_slp1, result))
                self.assertEqual(result, expected_slp1, 
                    f"IAST '{iast_char}' should convert to SLP1 '{expected_slp1}', got '{result}'")
        
        # Report all failures for easier debugging
        if failed_mappings:
            failure_report = "\n".join([
                f"  '{iast}' → expected '{expected}', got '{actual}'"
                for iast, expected, actual in failed_mappings
            ])
            self.fail(f"Failed character mappings:\n{failure_report}")
    
    def test_slp1_to_iast_individual_characters(self):
        """Test individual character mappings from SLP1 to IAST."""
        mappings = [
            # Vowels  
            ("a", "a"), ("A", "ā"), ("i", "i"), ("I", "ī"),
            ("u", "u"), ("U", "ū"), ("f", "ṛ"), ("F", "ṝ"),
            ("x", "ḷ"), ("X", "ḹ"), ("e", "e"), ("o", "o"),
            ("E", "ai"), ("O", "au"),
            
            # Key consonants
            ("K", "kh"), ("G", "gh"), ("N", "ṅ"), ("C", "ch"), 
            ("J", "jh"), ("Y", "ñ"), ("w", "ṭ"), ("W", "ṭh"),
            ("q", "ḍ"), ("Q", "ḍh"), ("R", "ṇ"), ("T", "th"),
            ("D", "dh"), ("P", "ph"), ("B", "bh"), ("S", "ś"),
            ("z", "ṣ"),
            
            # Marks
            ("M", "ṃ"), ("H", "ḥ"),
        ]
        
        failed_mappings = []
        
        for slp1_char, expected_iast in mappings:
            with self.subTest(slp1=slp1_char, expected=expected_iast):
                result = shlesha.transliterate(slp1_char, 'slp1', 'iast')
                if result != expected_iast:
                    failed_mappings.append((slp1_char, expected_iast, result))
                self.assertEqual(result, expected_iast,
                    f"SLP1 '{slp1_char}' should convert to IAST '{expected_iast}', got '{result}'")
        
        if failed_mappings:
            failure_report = "\n".join([
                f"  '{slp1}' → expected '{expected}', got '{actual}'"
                for slp1, expected, actual in failed_mappings
            ])
            self.fail(f"Failed character mappings:\n{failure_report}")
    
    def test_iast_to_slp1_words(self):
        """Test word-level conversions from IAST to SLP1."""
        test_words = [
            ("saṃskṛtam", "saMskftam"),
            ("dharmakṣetre", "Darmakzetre"), 
            ("kurukṣetre", "kurukzetre"),
            ("namaskāram", "namaskAram"),
            ("bhagavadgītā", "BagavadgItA"),
            ("aṣṭāṅgayoga", "azwANgayoga"),
        ]
        
        for iast_word, expected_slp1 in test_words:
            with self.subTest(word=iast_word):
                result = shlesha.transliterate(iast_word, 'iast', 'slp1')
                self.assertEqual(result, expected_slp1,
                    f"IAST '{iast_word}' should convert to SLP1 '{expected_slp1}', got '{result}'")
    
    def test_slp1_to_iast_words(self):
        """Test word-level conversions from SLP1 to IAST."""
        test_words = [
            ("saMskftam", "saṃskṛtam"),
            ("Darmakzetre", "dharmakṣetre"),
            ("kurukzetre", "kurukṣetre"), 
            ("namaskAram", "namaskāram"),
        ]
        
        for slp1_word, expected_iast in test_words:
            with self.subTest(word=slp1_word):
                result = shlesha.transliterate(slp1_word, 'slp1', 'iast')
                self.assertEqual(result, expected_iast,
                    f"SLP1 '{slp1_word}' should convert to IAST '{expected_iast}', got '{result}'")
    
    def test_bidirectional_consistency(self):
        """Test that IAST ↔ SLP1 conversions are consistent."""
        test_cases = [
            "saṃskṛtam",
            "dharmakṣetre", 
            "namaskāram",
            "bhagavadgītā",
        ]
        
        for original in test_cases:
            with self.subTest(original=original):
                # IAST → SLP1 → IAST should return to original
                slp1_result = shlesha.transliterate(original, 'iast', 'slp1')
                back_to_iast = shlesha.transliterate(slp1_result, 'slp1', 'iast')
                
                self.assertEqual(back_to_iast, original,
                    f"Round-trip conversion failed: '{original}' → '{slp1_result}' → '{back_to_iast}'")
    
    def test_compare_with_vidyut(self):
        """Compare Shlesha results with Vidyut reference implementation."""
        test_cases = [
            "saṃskṛtam",
            "dharmakṣetre",
            "namaskāram", 
            "ā", "ī", "ū", "ṛ", "ṃ", "ḥ", "ś", "ṣ", "kṣ"
        ]
        
        for test_input in test_cases:
            with self.subTest(input=test_input):
                # IAST → SLP1
                shlesha_result = shlesha.transliterate(test_input, 'iast', 'slp1')
                vidyut_result = transliterate(test_input, Scheme.Iast, Scheme.Slp1)
                
                self.assertEqual(shlesha_result, vidyut_result,
                    f"Shlesha and Vidyut disagree on '{test_input}': "
                    f"Shlesha='{shlesha_result}', Vidyut='{vidyut_result}'")


class TestOtherRomanScripts(unittest.TestCase):
    """Test other Roman script conversions to ensure they work correctly."""
    
    def test_iast_to_iso(self):
        """Test IAST to ISO-15919 conversion."""
        test_cases = [
            ("ṛ", "r̥"),
            ("ṝ", "r̥̄"), 
            ("ṃ", "ṁ"),
            ("saṃskṛtam", "saṁskr̥tam"),
        ]
        
        for iast_input, expected_iso in test_cases:
            with self.subTest(input=iast_input):
                result = shlesha.transliterate(iast_input, 'iast', 'iso')
                self.assertEqual(result, expected_iso,
                    f"IAST '{iast_input}' should convert to ISO '{expected_iso}', got '{result}'")
    
    def test_harvard_kyoto_to_slp1(self):
        """Test Harvard-Kyoto to SLP1 conversion.""" 
        test_cases = [
            ("R", "f"),  # HK R → SLP1 f
            ("RR", "F"), # HK RR → SLP1 F  
            ("M", "M"),  # HK M → SLP1 M
            ("H", "H"),  # HK H → SLP1 H
        ]
        
        for hk_input, expected_slp1 in test_cases:
            with self.subTest(input=hk_input):
                result = shlesha.transliterate(hk_input, 'harvard_kyoto', 'slp1')
                self.assertEqual(result, expected_slp1,
                    f"Harvard-Kyoto '{hk_input}' should convert to SLP1 '{expected_slp1}', got '{result}'")


if __name__ == '__main__':
    unittest.main(verbosity=2)