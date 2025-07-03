#!/usr/bin/env python3
"""
Verify Transliteration Correctness
Ensures both libraries are producing correct outputs
"""

import shlesha
from vidyut.lipi import transliterate, Scheme

def verify_conversions():
    """Verify that conversions are actually happening correctly."""
    print("🔍 Transliteration Output Verification")
    print("=" * 80)
    print("Checking if both libraries are producing correct outputs")
    print()
    
    test_cases = [
        {
            'name': 'IAST → SLP1',
            'input': 'dharmakṣetraṃ kurukṣetraṃ',
            'shlesha_args': ('iast', 'slp1'),
            'vidyut_args': (Scheme.Iast, Scheme.Slp1),
            'expected_chars': {
                'ṣ': 'z',  # ṣ should become z
                'ṃ': 'M',  # ṃ should become M
                'dh': 'D', # dh should become D
                'kṣ': 'kz' # kṣ should become kz
            }
        },
        {
            'name': 'IAST → Telugu',
            'input': 'saṃskṛtam',
            'shlesha_args': ('iast', 'telugu'),
            'vidyut_args': (Scheme.Iast, Scheme.Telugu),
            'expected_telugu': True
        },
        {
            'name': 'Telugu → Devanagari',
            'input': 'సంస్కృతం',
            'shlesha_args': ('telugu', 'devanagari'),
            'vidyut_args': (Scheme.Telugu, Scheme.Devanagari),
            'expected_devanagari': True
        },
        {
            'name': 'IAST → Devanagari',
            'input': 'namaskāram',
            'shlesha_args': ('iast', 'devanagari'),
            'vidyut_args': (Scheme.Iast, Scheme.Devanagari),
            'expected_chars': {
                'ā': 'ा',  # ā should become ा (vowel sign)
                'nam': 'नम', # na + ma
                'ram': 'रम्' # ra + m
            }
        }
    ]
    
    for test in test_cases:
        print(f"Test: {test['name']}")
        print(f"Input: '{test['input']}'")
        
        # Run transliterations
        shlesha_output = shlesha.transliterate(
            test['input'], 
            test['shlesha_args'][0], 
            test['shlesha_args'][1]
        )
        
        vidyut_output = transliterate(
            test['input'],
            test['vidyut_args'][0],
            test['vidyut_args'][1]
        )
        
        print(f"Shlesha: '{shlesha_output}'")
        print(f"Vidyut:  '{vidyut_output}'")
        
        # Check if conversion actually happened
        if shlesha_output == test['input']:
            print("❌ WARNING: Shlesha output is identical to input - no conversion!")
        
        # For SLP1, check specific character conversions
        if test['name'] == 'IAST → SLP1':
            print("\nCharacter mapping check:")
            for orig, expected in test['expected_chars'].items():
                if orig in test['input']:
                    shlesha_has = expected in shlesha_output or orig in shlesha_output
                    vidyut_has = expected in vidyut_output
                    
                    print(f"  {orig} → {expected}:")
                    print(f"    Shlesha: {'✓' if expected in shlesha_output else '✗'} ({'kept original' if orig in shlesha_output else 'converted'})")
                    print(f"    Vidyut:  {'✓' if vidyut_has else '✗'}")
        
        # Check script detection
        if 'expected_telugu' in test:
            # Check for Telugu Unicode range
            telugu_chars = any('\u0C00' <= c <= '\u0C7F' for c in shlesha_output)
            print(f"  Contains Telugu characters: Shlesha {'✓' if telugu_chars else '✗'}")
            telugu_chars = any('\u0C00' <= c <= '\u0C7F' for c in vidyut_output)
            print(f"  Contains Telugu characters: Vidyut {'✓' if telugu_chars else '✗'}")
        
        if 'expected_devanagari' in test:
            # Check for Devanagari Unicode range
            deva_chars = any('\u0900' <= c <= '\u097F' for c in shlesha_output)
            print(f"  Contains Devanagari characters: Shlesha {'✓' if deva_chars else '✗'}")
            deva_chars = any('\u0900' <= c <= '\u097F' for c in vidyut_output)
            print(f"  Contains Devanagari characters: Vidyut {'✓' if deva_chars else '✗'}")
        
        print("-" * 80)
        print()

def test_specific_conversions():
    """Test specific problematic conversions."""
    print("\n🔧 Specific Conversion Tests")
    print("=" * 80)
    
    # Test IAST to SLP1 mappings
    mappings = [
        ('a', 'a'), ('ā', 'A'), ('i', 'i'), ('ī', 'I'), ('u', 'u'), ('ū', 'U'),
        ('ṛ', 'f'), ('ṝ', 'F'), ('ḷ', 'x'), ('ḹ', 'X'),
        ('e', 'e'), ('ai', 'E'), ('o', 'o'), ('au', 'O'),
        ('k', 'k'), ('kh', 'K'), ('g', 'g'), ('gh', 'G'), ('ṅ', 'N'),
        ('c', 'c'), ('ch', 'C'), ('j', 'j'), ('jh', 'J'), ('ñ', 'Y'),
        ('ṭ', 'w'), ('ṭh', 'W'), ('ḍ', 'q'), ('ḍh', 'Q'), ('ṇ', 'R'),
        ('t', 't'), ('th', 'T'), ('d', 'd'), ('dh', 'D'), ('n', 'n'),
        ('p', 'p'), ('ph', 'P'), ('b', 'b'), ('bh', 'B'), ('m', 'm'),
        ('y', 'y'), ('r', 'r'), ('l', 'l'), ('v', 'v'),
        ('ś', 'S'), ('ṣ', 'z'), ('s', 's'), ('h', 'h'),
        ('ṃ', 'M'), ('ḥ', 'H'),
        ('kṣ', 'kz'), ('jñ', 'jY')
    ]
    
    print("Testing individual IAST → SLP1 character mappings:")
    errors = 0
    
    for iast_char, expected_slp1 in mappings:
        shlesha_result = shlesha.transliterate(iast_char, 'iast', 'slp1')
        vidyut_result = transliterate(iast_char, Scheme.Iast, Scheme.Slp1)
        
        shlesha_correct = shlesha_result == expected_slp1
        vidyut_correct = vidyut_result == expected_slp1
        
        if not shlesha_correct or not vidyut_correct:
            print(f"  '{iast_char}' → '{expected_slp1}':")
            print(f"    Shlesha: '{shlesha_result}' {'✓' if shlesha_correct else '✗'}")
            print(f"    Vidyut:  '{vidyut_result}' {'✓' if vidyut_correct else '✗'}")
            errors += 1
    
    if errors == 0:
        print("  ✓ All individual character mappings correct!")
    else:
        print(f"  ✗ Found {errors} incorrect mappings")
    
    # Test a full word
    print("\nTesting full word conversion:")
    test_word = "saṃskṛtam"
    expected = "saMskftam"
    
    shlesha_result = shlesha.transliterate(test_word, 'iast', 'slp1')
    vidyut_result = transliterate(test_word, Scheme.Iast, Scheme.Slp1)
    
    print(f"  Input: '{test_word}'")
    print(f"  Expected: '{expected}'")
    print(f"  Shlesha:  '{shlesha_result}' {'✓' if shlesha_result == expected else '✗'}")
    print(f"  Vidyut:   '{vidyut_result}' {'✓' if vidyut_result == expected else '✗'}")

def check_supported_scripts():
    """Check what scripts are supported."""
    print("\n📋 Supported Scripts Check")
    print("=" * 80)
    
    scripts = shlesha.get_supported_scripts()
    print(f"Shlesha supports {len(scripts)} scripts:")
    
    # Group by type
    roman_scripts = [s for s in scripts if s in ['iast', 'slp1', 'itrans', 'harvard_kyoto', 'hk', 'velthuis', 'wx', 'iso15919', 'iso', 'kolkata']]
    indic_scripts = [s for s in scripts if s not in roman_scripts]
    
    print(f"\nRoman scripts ({len(roman_scripts)}):")
    for script in sorted(roman_scripts):
        print(f"  • {script}")
    
    print(f"\nIndic scripts ({len(indic_scripts)}):")
    for script in sorted(indic_scripts):
        print(f"  • {script}")

def main():
    """Run all verification tests."""
    print("🔍 Shlesha Transliteration Verification Suite")
    print("=" * 80)
    print("Ensuring conversions are happening correctly\n")
    
    # Check supported scripts
    check_supported_scripts()
    
    # Verify conversions
    verify_conversions()
    
    # Test specific mappings
    test_specific_conversions()
    
    print("\n💡 Summary:")
    print("If Shlesha is returning unchanged text, it might be:")
    print("  1. The script names are incorrect (use lowercase)")
    print("  2. The conversion path is not implemented")
    print("  3. The internal routing is failing")

if __name__ == "__main__":
    main()