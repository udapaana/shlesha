# Schema Syntax Simplification

## Current Problem: Too Verbose

### Current Syntax (5 lines per character!)
```yaml
"क":
  canonical: "ka"
  varga: "ka-varga"
  voiced: false
  aspirated: false
```

### Multiply by 100+ characters = **500+ lines** just for basic mappings!

## Proposed Simplified Syntax

### 1. **Minimal Mapping (Most Common Case)**
```yaml
# Just the essential mapping
consonants: "क=ka ख=kha ग=ga घ=gha ङ=ṅa"
vowels: "अ=a आ=ā इ=i ई=ī उ=u ऊ=ū"
modifiers: "्= ं=ṃ ः=ḥ"
```

### 2. **Compact List Format**
```yaml
consonants:
  - "क=ka ख=kha ग=ga घ=gha ङ=ṅa"  # ka-varga
  - "च=ca छ=cha ज=ja झ=jha ञ=ña"   # ca-varga
  - "ट=ṭa ठ=ṭha ड=ḍa ढ=ḍha ण=ṇa" # ṭa-varga
```

### 3. **Properties as Suffixes (When Needed)**
```yaml
consonants:
  basic: "क=ka ख=kha ग=ga घ=gha ङ=ṅa"
  aspirated: "ख kha घ gha झ jha ढ ḍha भ bha"  # Mark aspirated ones
  voiced: "ग ga घ gha ज ja झ jha ड ḍa"        # Mark voiced ones
```

### 4. **Template System**
```yaml
# Define templates for common patterns
templates:
  ka_varga: "क=ka ख=kha ग=ga घ=gha ङ=ṅa"
  ca_varga: "च=ca छ=cha ज=ja झ=jha ञ=ña"
  
consonants:
  - template: ka_varga
  - template: ca_varga
```

### 5. **Auto-Inference (Smartest)**
```yaml
# Auto-infer properties from canonical forms
consonants: |
  क=ka   ख=kha   ग=ga   घ=gha   ङ=ṅa
  च=ca   छ=cha   ज=ja   झ=jha   ञ=ña  
  ट=ṭa   ठ=ṭha   ड=ḍa   ढ=ḍha   ण=ṇa
  त=ta   थ=tha   द=da   ध=dha   न=na
  प=pa   फ=pha   ब=ba   भ=bha   म=ma

# System auto-detects:
# - Aspiration from 'h' in canonical form
# - Voice from position in varga
# - Varga from grouping
```

## Complete Simplified Schema Example

### Before (Current): ~400 lines
```yaml
name: "Devanagari"
type: abugida

metadata:
  version: "1.0.0"
  author: "Shlesha Contributors"
  description: "Devanagari script mapping for Sanskrit and modern Indic languages"

element_types:
  consonant:
    description: "Basic consonant with inherent 'a' vowel"
    properties:
      has_inherent_vowel:
        type: bool
        default: true
      varga:
        type: string
        description: "Consonant class"
      voiced:
        type: bool
        default: false
      aspirated:
        type: bool
        default: false

mappings:
  consonants:
    "क":
      canonical: "ka"
      varga: "ka-varga"
      voiced: false
      aspirated: false
    # ... 98 more like this
```

### After (Simplified): ~20 lines
```yaml
name: Devanagari
type: abugida

# Smart compact syntax - system infers properties
consonants: |
  क=ka   ख=kha   ग=ga   घ=gha   ङ=ṅa
  च=ca   छ=cha   ज=ja   झ=jha   ञ=ña  
  ट=ṭa   ठ=ṭha   ड=ḍa   ढ=ḍha   ण=ṇa
  त=ta   थ=tha   द=da   ध=dha   न=na
  प=pa   फ=pha   ब=ba   भ=bha   म=ma
  य=ya   र=ra    ल=la   व=va
  श=śa   ष=ṣa    स=sa   ह=ha

vowels: "अ=a आ=ā इ=i ई=ī उ=u ऊ=ū ऋ=ṛ ॠ=ṝ ऌ=ḷ ॡ=ḹ ए=e ऐ=ai ओ=o औ=au"

vowel_marks: "ा=ā ि=i ी=ī ु=u ू=ū ृ=ṛ ॄ=ṝ ॢ=ḷ ॣ=ḹ े=e ै=ai ो=o ौ=au"

modifiers: "्= ं=ṃ ः=ḥ ँ=m̐ ऽ='"

numerals: "०=0 १=1 २=2 ३=3 ४=4 ५=5 ६=6 ७=7 ८=8 ९=9"

punctuation: "।=. ॥=.."
```

## Smart Inference Rules

### 1. **Aspiration Detection**
```yaml
# 'h' in canonical = aspirated
"ख=kha" → aspirated: true
"क=ka"  → aspirated: false
```

### 2. **Voice Detection**
```yaml
# Position in varga determines voice
"क ख | ग घ ङ"  → "क ख" unvoiced, "ग घ ङ" voiced
```

### 3. **Varga Classification**
```yaml
# Grouping by lines
Line 1: "क=ka ख=kha ग=ga घ=gha ङ=ṅa" → ka-varga
Line 2: "च=ca छ=cha ज=ja झ=jha ञ=ña"  → ca-varga
```

### 4. **Element Type Inference**
```yaml
consonants: → type: consonant
vowels: → type: vowel_independent  
vowel_marks: → type: vowel_dependent
modifiers: → type: modifier
```

## Extension Syntax Simplification

### Current Extension (Verbose)
```yaml
extensions:
  vedic_accents:
    description: "Vedic accent marks for tonal Sanskrit"
    priority: 10
    mappings:
      "॑":
        to: ""
        element_type: "accent"
        properties:
          accent_type: "udatta"
          tone: "high"
      "॒":
        to: ""
        element_type: "accent"
        properties:
          accent_type: "anudatta"
          tone: "low"
```

### Simplified Extension
```yaml
extensions:
  vedic_accents: "॑=udatta ॒=anudatta ᳚=svarita"
  
  # Or with minimal context
  vedic_accents:
    description: "Vedic accent marks"
    mappings: "॑=udatta ॒=anudatta ᳚=svarita"
```

## Implementation Strategy

### 1. **Parser Enhancement**
```rust
struct SimplifiedSchemaParser {
    // Parse compact syntax
    fn parse_compact_mappings(&self, input: &str) -> Vec<(String, String)>
    
    // Auto-infer properties  
    fn infer_properties(&self, mappings: &[(String, String)]) -> PropertyInference
    
    // Convert to full internal representation
    fn expand_to_full_schema(&self, simplified: SimplifiedSchema) -> Schema
}
```

### 2. **Backward Compatibility**
```rust
impl SchemaParser {
    fn parse_str(input: &str) -> Result<Schema, SchemaError> {
        // Try simplified format first
        if let Ok(simplified) = SimplifiedSchemaParser::parse(input) {
            return Ok(simplified.expand_to_full_schema());
        }
        
        // Fall back to verbose format
        self.parse_verbose_format(input)
    }
}
```

### 3. **Smart Defaults**
```rust
impl PropertyInference {
    fn infer_consonant_properties(&self, canonical: &str) -> Properties {
        Properties {
            aspirated: canonical.contains('h'),
            voiced: self.detect_voice_from_position(),
            varga: self.detect_varga_from_grouping(),
            has_inherent_vowel: true, // Default for abugidas
        }
    }
}
```

## User Experience Improvement

### Before: 😫
- 400+ lines for basic Devanagari
- 5 lines per character
- Complex YAML nesting
- Hard to see patterns
- Error-prone property definitions

### After: 😍  
- 20 lines for complete Devanagari
- Visual script layout
- Auto-inferred properties
- Easy to spot mistakes
- Copy-paste friendly

## Next Steps

1. **Implement simplified parser** 
2. **Add auto-inference rules**
3. **Create migration tool** (verbose → simplified)
4. **Update documentation** with simplified examples
5. **Test with real users** to validate UX

The goal: **Anyone should be able to create a new script mapping in 10 minutes**, not 2 hours.