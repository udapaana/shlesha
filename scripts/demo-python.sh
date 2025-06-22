#!/bin/bash
# Interactive Python demo for Shlesha

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[DEMO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_status "Starting Shlesha Python demo..."

# Check if uv environment exists
if ! command -v uv &> /dev/null || [[ ! -f "pyproject.toml" ]]; then
    print_error "uv environment not found. Run ./scripts/setup-dev.sh first"
    exit 1
fi

# Check if shlesha is installed
if ! uv run python -c "import shlesha" 2>/dev/null; then
    print_status "Building Python bindings..."
    ./scripts/build-all.sh
fi

# Create interactive demo script
cat > python_demo.py << 'EOF'
#!/usr/bin/env python3
"""
Shlesha Python API Demo
Interactive demonstration of transliteration capabilities
"""

import shlesha

def print_header(title):
    print(f"\n{'='*50}")
    print(f" {title}")
    print(f"{'='*50}")

def demo_basic_transliteration():
    print_header("Basic Transliteration")
    
    transliterator = shlesha.Shlesha()
    
    examples = [
        ("धर्म", "devanagari", "iast", "Sanskrit word for 'dharma'"),
        ("भारत", "devanagari", "iast", "Sanskrit/Hindi for 'India'"),
        ("नमस्ते", "devanagari", "itrans", "Sanskrit greeting"),
        ("योग", "devanagari", "slp1", "Sanskrit for 'yoga'"),
        ("dharma", "iast", "devanagari", "IAST to Devanagari"),
    ]
    
    for text, from_script, to_script, description in examples:
        result = transliterator.transliterate(text, from_script, to_script)
        print(f"{description}:")
        print(f"  {text} ({from_script}) → {result} ({to_script})")

def demo_metadata_collection():
    print_header("Metadata Collection")
    
    transliterator = shlesha.Shlesha()
    
    # Example with unknown characters
    text = "धर्मkr123"
    result = transliterator.transliterate_with_metadata(text, "devanagari", "iast")
    
    print(f"Input: {text}")
    print(f"Output: {result.output}")
    print(f"Unknown tokens found: {len(result.metadata.unknown_tokens)}")
    
    for i, token in enumerate(result.metadata.unknown_tokens, 1):
        print(f"  {i}. '{token.token}' at position {token.position} (Unicode: {token.unicode})")

def demo_script_discovery():
    print_header("Script Discovery")
    
    transliterator = shlesha.Shlesha()
    
    scripts = transliterator.list_supported_scripts()
    print(f"Supported scripts ({len(scripts)} total):")
    
    # Group scripts by type
    roman_scripts = [s for s in scripts if s in ['iast', 'itrans', 'slp1', 'harvard_kyoto', 'velthuis', 'wx', 'iso15919']]
    indic_scripts = [s for s in scripts if s not in roman_scripts]
    
    print("\nRoman/ASCII scripts:")
    for script in sorted(roman_scripts):
        print(f"  • {script}")
    
    print("\nIndic scripts:")
    for script in sorted(indic_scripts):
        print(f"  • {script}")

def demo_cross_script_conversion():
    print_header("Cross-Script Conversion")
    
    transliterator = shlesha.Shlesha()
    
    # Convert Sanskrit word across multiple scripts
    original = "धर्म"
    target_scripts = ["bengali", "tamil", "telugu", "gujarati", "kannada"]
    
    print(f"Converting '{original}' (Devanagari) to various scripts:")
    
    for script in target_scripts:
        try:
            result = transliterator.transliterate(original, "devanagari", script)
            print(f"  {script.capitalize()}: {result}")
        except Exception as e:
            print(f"  {script.capitalize()}: Error - {e}")

def demo_convenience_functions():
    print_header("Convenience Functions")
    
    # Direct function call
    result = shlesha.transliterate("अ", "devanagari", "iast")
    print(f"Direct function: shlesha.transliterate('अ', 'devanagari', 'iast') → '{result}'")
    
    # Get supported scripts
    scripts = shlesha.get_supported_scripts()
    print(f"Get scripts: shlesha.get_supported_scripts() → {len(scripts)} scripts")
    
    # Create transliterator
    t = shlesha.create_transliterator()
    result2 = t.transliterate("आ", "devanagari", "iast")
    print(f"Create transliterator: {result2}")

def interactive_demo():
    print_header("Interactive Demo")
    
    transliterator = shlesha.Shlesha()
    
    print("Enter text to transliterate (or 'quit' to exit):")
    print("Format: <text> <from_script> <to_script>")
    print("Example: धर्म devanagari iast")
    print()
    
    while True:
        try:
            user_input = input("> ").strip()
            if user_input.lower() in ['quit', 'exit', 'q']:
                break
                
            parts = user_input.split()
            if len(parts) != 3:
                print("Please provide: <text> <from_script> <to_script>")
                continue
                
            text, from_script, to_script = parts
            result = transliterator.transliterate(text, from_script, to_script)
            print(f"Result: {result}")
            
            # Also show metadata
            meta_result = transliterator.transliterate_with_metadata(text, from_script, to_script)
            if meta_result.metadata and meta_result.metadata.unknown_tokens:
                print(f"Unknown tokens: {len(meta_result.metadata.unknown_tokens)}")
                
        except KeyboardInterrupt:
            break
        except Exception as e:
            print(f"Error: {e}")

def main():
    print_header("Shlesha Python API Demo")
    print("High-performance Sanskrit transliteration library")
    print()
    
    try:
        demo_basic_transliteration()
        demo_metadata_collection() 
        demo_script_discovery()
        demo_cross_script_conversion()
        demo_convenience_functions()
        interactive_demo()
        
    except KeyboardInterrupt:
        print("\n\nDemo interrupted by user")
    except Exception as e:
        print(f"\nError during demo: {e}")
    
    print_header("Demo Complete")
    print("Thank you for trying Shlesha!")

if __name__ == "__main__":
    main()
EOF

# Run the demo
print_success "Python bindings ready! Starting interactive demo..."
uv run python python_demo.py

# Cleanup
rm python_demo.py