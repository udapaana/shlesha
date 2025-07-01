#!/bin/bash
set -e

echo "🧪 Running comprehensive functionality tests..."

# Use existing venv from PyPI test
source test_pypi_venv/bin/activate

python3 -c "
import shlesha
import sys

def test_basic_transliteration():
    '''Test basic transliteration functionality'''
    test_cases = [
        ('धर्म', 'devanagari', 'iast', 'dharma'),
        ('योग', 'devanagari', 'iast', 'yoga'),
        ('अ', 'devanagari', 'iast', 'a'),
        ('dharma', 'iast', 'devanagari', 'धर्म'),
    ]
    
    for input_text, from_script, to_script, expected in test_cases:
        result = shlesha.transliterate(input_text, from_script, to_script)
        if result == expected:
            print(f'✅ {input_text} ({from_script} → {to_script}) = {result}')
        else:
            print(f'❌ {input_text} ({from_script} → {to_script}) = {result}, expected {expected}')
            return False
    return True

def test_class_methods():
    '''Test class-based API'''
    translator = shlesha.Shlesha()
    
    # Test basic transliteration
    result = translator.transliterate('नमस्ते', 'devanagari', 'iast')
    print(f'✅ Class method: नमस्ते → {result}')
    
    # Test script support
    scripts = translator.list_supported_scripts()
    if 'devanagari' in scripts and 'iast' in scripts:
        print(f'✅ Script support: {len(scripts)} scripts available')
    else:
        print('❌ Missing core scripts')
        return False
        
    # Test script validation
    if translator.supports_script('devanagari') and translator.supports_script('iast'):
        print('✅ Script validation works')
    else:
        print('❌ Script validation failed')
        return False
        
    return True

def test_metadata_functionality():
    '''Test metadata collection'''
    translator = shlesha.Shlesha()
    
    # Test with mixed content that should generate metadata
    result = translator.transliterate_with_metadata('धर्मkr', 'devanagari', 'iast')
    
    if hasattr(result, 'output') and hasattr(result, 'metadata'):
        print(f'✅ Metadata collection: output={result.output}')
        if result.metadata and hasattr(result.metadata, 'unknown_tokens'):
            print(f'✅ Unknown tokens tracked: {len(result.metadata.unknown_tokens)} tokens')
        else:
            print('⚠️ No metadata or unknown tokens')
    else:
        print('❌ Metadata structure invalid')
        return False
        
    return True

def test_error_handling():
    '''Test error handling'''
    try:
        shlesha.transliterate('test', 'invalid_script', 'iast')
        print('❌ Should have raised error for invalid script')
        return False
    except:
        print('✅ Error handling for invalid scripts works')
        
    return True

def test_convenience_functions():
    '''Test convenience functions'''
    scripts = shlesha.get_supported_scripts()
    if len(scripts) > 0 and 'devanagari' in scripts:
        print(f'✅ get_supported_scripts: {len(scripts)} scripts')
    else:
        print('❌ get_supported_scripts failed')
        return False
        
    translator = shlesha.create_transliterator()
    if translator and hasattr(translator, 'transliterate'):
        print('✅ create_transliterator works')
    else:
        print('❌ create_transliterator failed')
        return False
        
    return True

def test_version_info():
    '''Test version and metadata'''
    if hasattr(shlesha, '__version__') and shlesha.__version__:
        print(f'✅ Version info: {shlesha.__version__}')
    else:
        print('❌ No version info')
        return False
        
    return True

# Run all tests
print('Running comprehensive functionality tests...')
tests = [
    test_basic_transliteration,
    test_class_methods,
    test_metadata_functionality,
    test_error_handling,
    test_convenience_functions,
    test_version_info,
]

all_passed = True
for test in tests:
    try:
        if not test():
            all_passed = False
    except Exception as e:
        print(f'❌ Test {test.__name__} failed with exception: {e}')
        import traceback
        traceback.print_exc()
        all_passed = False

if all_passed:
    print('🎉 All functionality tests passed!')
else:
    print('💥 Some functionality tests failed!')
    sys.exit(1)
"

echo "✅ Functionality tests completed"