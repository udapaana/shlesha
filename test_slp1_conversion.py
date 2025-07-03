#!/usr/bin/env python3
"""
Test SLP1 conversion issue
"""

import shlesha
from vidyut.lipi import transliterate, Scheme

def test_slp1_conversions():
    """Test that SLP1 conversions work correctly."""
    print("🔬 Testing SLP1 Conversion Issues")
    print("=" * 60)
    
    # Test basic character mappings
    test_cases = [
        # (iast_input, expected_slp1_output)
        ("ā", "A"),
        ("ī", "I"), 
        ("ū", "U"),
        ("ṛ", "f"),
        ("ṝ", "F"),
        ("ṃ", "M"),
        ("ḥ", "H"),
        ("ś", "S"),
        ("ṣ", "z"),
        ("ṅ", "N"),
        ("ñ", "Y"),
        ("ṇ", "R"),
        ("ṭ", "w"),
        ("ḍ", "q"),
        ("kṣ", "kz"),
        ("ai", "E"),
        ("au", "O"),
    ]
    
    print("Testing individual character mappings:")
    errors = 0
    for iast_char, expected_slp1 in test_cases:
        result = shlesha.transliterate(iast_char, 'iast', 'slp1')
        vidyut_result = transliterate(iast_char, Scheme.Iast, Scheme.Slp1)
        
        correct = (result == expected_slp1)
        vidyut_correct = (vidyut_result == expected_slp1)
        
        if not correct:
            print(f"  ❌ '{iast_char}' → expected '{expected_slp1}', got '{result}'")
            errors += 1
        else:
            print(f"  ✅ '{iast_char}' → '{result}'")
        
        if not vidyut_correct:
            print(f"     Vidyut: '{vidyut_result}' (reference)")
    
    print(f"\nErrors found: {errors}/{len(test_cases)}")
    
    # Test full words
    print("\nTesting full words:")
    word_tests = [
        ("saṃskṛtam", "saMskftam"),
        ("dharmakṣetre", "Darmakzetre"),
        ("kurukṣetre", "kurukzetre"),
        ("namaskāram", "namaskAram"),
    ]
    
    for word, expected in word_tests:
        result = shlesha.transliterate(word, 'iast', 'slp1')
        vidyut_result = transliterate(word, Scheme.Iast, Scheme.Slp1)
        
        print(f"  Input: '{word}'")
        print(f"  Expected: '{expected}'")
        print(f"  Shlesha:  '{result}' {'✅' if result == expected else '❌'}")
        print(f"  Vidyut:   '{vidyut_result}' {'✅' if vidyut_result == expected else '❌'}")
        print()

def test_reverse_conversion():
    """Test SLP1 → IAST conversion."""
    print("🔄 Testing SLP1 → IAST (reverse conversion)")
    print("=" * 60)
    
    test_cases = [
        ("A", "ā"),
        ("I", "ī"),
        ("U", "ū"),
        ("f", "ṛ"),
        ("F", "ṝ"),
        ("M", "ṃ"),
        ("H", "ḥ"),
        ("S", "ś"),
        ("z", "ṣ"),
        ("saMskftam", "saṃskṛtam"),
    ]
    
    for slp1_input, expected_iast in test_cases:
        result = shlesha.transliterate(slp1_input, 'slp1', 'iast')
        vidyut_result = transliterate(slp1_input, Scheme.Slp1, Scheme.Iast)
        
        correct = (result == expected_iast)
        vidyut_correct = (vidyut_result == expected_iast)
        
        print(f"  '{slp1_input}' → '{result}' {'✅' if correct else '❌'}")
        if not correct:
            print(f"    Expected: '{expected_iast}'")
        if not vidyut_correct:
            print(f"    Vidyut: '{vidyut_result}'")

if __name__ == "__main__":
    test_slp1_conversions()
    print()
    test_reverse_conversion()