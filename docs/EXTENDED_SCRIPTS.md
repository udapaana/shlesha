# Extended Scripts in Shlesha

## Overview

Some scripts in Shlesha are classified as "extended scripts" which have special handling. Currently, Tamil is the only extended script.

## Why Tamil is an Extended Script

Tamil uses a **superscript notation system** to represent Sanskrit sounds that don't exist in traditional Tamil:

```
க  → क (ka)        - Basic Tamil sound
க² → ख (kha)       - Aspirated version (marked with superscript ²)
க³ → ग (ga)        - Voiced version (marked with superscript ³)  
க⁴ → घ (gha)       - Voiced aspirated (marked with superscript ⁴)
```

This notation allows Tamil to represent the full Sanskrit phoneme inventory while maintaining its traditional character set.

## Design Decision: One-way Conversion Only

Extended scripts like Tamil **only support forward conversion** (Tamil → Devanagari) but not reverse conversion (Devanagari → Tamil). This is a deliberate design choice for two reasons:

### 1. Performance Optimization

Supporting bidirectional conversion with superscripts would require:
- Using `FxHashMap<String, String>` instead of `FxHashMap<char, char>`
- Checking multiple character sequences on every character
- Additional string allocations and comparisons

This would significantly impact performance for all Tamil text processing, even when superscripts aren't used.

### 2. Practical Usage Patterns

In practice:
- Tamil text with Sanskrit extensions typically flows in one direction (source → hub)
- The superscript notation is primarily used for Sanskrit texts in Tamil script
- Native Tamil text doesn't use these extensions and works fine with standard converters

## Technical Implementation

Extended scripts use the `indic_extended_converter.hbs` template which:
- Uses `FxHashMap<String, String>` for complex multi-character mappings
- Implements sophisticated forward conversion with superscript support
- Explicitly blocks reverse conversion with a clear error message

## Future Considerations

If bidirectional conversion becomes necessary for extended scripts, potential solutions include:
1. Hybrid approach: Use char mappings for common cases, string mappings for superscripts
2. Separate converters: One optimized for native text, one for Sanskrit extensions
3. Runtime detection: Choose converter based on presence of superscripts

For now, the one-way conversion limitation is an acceptable trade-off that maintains performance for the common use case while still supporting the full Sanskrit phoneme inventory when needed.