# Vedic Accent Marks: Unicode Ambiguities and Design Decisions

## The Problem

Unicode's naming of Vedic accent marks creates significant confusion because the names don't match actual usage across different Vedic traditions:

### Unicode Names vs. Actual Usage

1. **U+0951 ॑** - Named "DEVANAGARI STRESS SIGN UDATTA"
   - In Rigveda Samhita: marks **svarita** (not udatta)
   - In Maitrāyaṇīya/Kaṭhaka Saṃhitās: marks **udatta**
   - Most modern publications use it for **svarita**

2. **U+0952 ॒** - Named "DEVANAGARI STRESS SIGN ANUDATTA"
   - Generally consistent across traditions as anudatta
   - Sometimes combined with numerals for other accent categories

3. **Udatta** in practice:
   - Often left **unmarked** (the default/neutral tone)
   - Different traditions handle this differently

## Complexity Across Traditions

Different Vedic textual traditions employ varying marking systems:

- **Rigveda**: udatta is unmarked, ॑ marks svarita
- **Yajurveda** (various schools): Different conventions
- **Samaveda**: Uses numerical notation system
- **Atharvaveda**: Has its own conventions

The same visual mark can represent different linguistic functions depending on the tradition, manuscript, or editorial convention.

## Our Solution: Visual Token Names

To avoid imposing one tradition's interpretation over others, Shlesha uses **visual descriptions** for tokens in built-in schemas:

### Token Naming Convention

Instead of linguistic function names (MarkUdatta, MarkSvarita), we use visual descriptions:

- `MarkVerticalLineAbove` - ॑ (U+0951)
- `MarkLineBelow` - ॒ (U+0952)  
- `MarkDoubleVerticalAbove` - ᳚ (U+1CDA)
- `MarkTripleVerticalAbove` - ᳛ (U+1CDB)

### Benefits

1. **No ambiguity**: The token name describes what you see, not what it might mean
2. **Tradition-neutral**: Each tradition can interpret the marks according to their conventions
3. **Self-documenting**: The visual nature is immediately clear
4. **Future-proof**: New traditions or interpretations don't require token changes

## For Runtime Schemas

Runtime schemas have full flexibility to use either approach:

1. **Visual tokens** (recommended for consistency)
2. **Linguistic tokens** (if targeting a specific tradition)
   - Can use `MarkUdatta`, `MarkSvarita` etc. if desired
   - Useful for tradition-specific implementations

## Implementation Notes

### Built-in Schemas
All built-in schemas use visual token names to maintain neutrality and avoid confusion.

### Conversion Logic
When converting between scripts, the visual marks are preserved. The linguistic interpretation is left to the user or application layer.

### Font Considerations
Some fonts may not properly position these combining marks, especially on consonants with subscripts. This is a font limitation, not a Unicode or Shlesha issue.

## References

1. Unicode Standard, Chapter 12: South and Central Asian Scripts
2. "Vedic Extensions" Unicode block (U+1CD0-1CFF)
3. Various Vedic editorial traditions and their documentation
4. ISCII-91 standard (historical reference)

## See Also

- [ADDING_SCHEMAS.md](ADDING_SCHEMAS.md) - For implementing Vedic marks in new schemas
- Unicode Vedic Extensions block documentation