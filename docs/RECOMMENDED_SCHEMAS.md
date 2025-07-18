# Recommended Additional Schemas for Shlesha

This document lists schemas that could be added to enhance Shlesha's transliteration capabilities.

## Priority 1: Essential Additions

### 1. **Kyoto-Harvard (KH)**
- **Use case**: Academic publications using reverse HK notation
- **Key feature**: Uppercase for retroflexes (T, D, N) vs lowercase in HK
- **Example**: `schemas/examples/kyoto_harvard.yaml`

### 2. **URL-Safe Sanskrit**
- **Use case**: Web applications, REST APIs, permalinks
- **Key feature**: Only uses alphanumeric + dash/underscore
- **Example**: `schemas/examples/url_safe.yaml`

### 3. **ISCII (IS 13194:1991)**
- **Use case**: Legacy government data, pre-Unicode systems
- **Key feature**: Indian government standard before Unicode
- **Example**: `schemas/examples/iscii.yaml`

## Priority 2: Historical Scripts

### 4. **Sharada**
- **Use case**: Kashmir manuscripts, tantric texts
- **Unicode**: U+11180–U+111DF
- **Status**: Living script for religious texts

### 5. **Modi**
- **Use case**: Marathi historical documents (pre-1950)
- **Unicode**: U+11600–U+1165F
- **Status**: Recently revived for cultural preservation

### 6. **Brahmi**
- **Use case**: Ancient inscriptions, Buddhist texts
- **Unicode**: U+11000–U+1107F
- **Status**: Historical only

## Priority 3: Regional Extensions

### 7. **Tibetan Sanskrit**
- **Use case**: Buddhist canonical texts
- **Special needs**: Stacking rules, special marks
- **Unicode**: U+0F00–U+0FFF

### 8. **Newa (Prachalit)**
- **Use case**: Nepal Sanskrit manuscripts
- **Unicode**: U+11400–U+1147F
- **Status**: Active in Nepal

### 9. **Dravidian ITRANS**
- **Use case**: Tamil/Telugu/Kannada with special sounds
- **Features**: zha (ழ), RRa (ற), special nasals

## Priority 4: Specialized Systems

### 10. **Baraha**
- **Use case**: Popular in Karnataka/Andhra
- **Features**: Intuitive for native speakers
- **Variations**: Baraha-Kannada, Baraha-Telugu

### 11. **Hunterian**
- **Use case**: Indian government geographical names
- **Features**: Simplified, no diacritics
- **Official**: Survey of India standard

### 12. **ISO 15919 Vedic**
- **Use case**: Vedic texts with accent marks
- **Features**: Udatta, anudatta, svarita marks
- **Extensions**: Rare Vedic characters

## Implementation Notes

1. **Schema Structure**: All schemas should follow the existing YAML format with:
   - `metadata`: name, script_type, description, aliases
   - `mappings`: vowels, consonants, marks, digits, special

2. **Testing**: Each new schema should include:
   - Basic round-trip tests
   - Sample text conversions
   - Edge case handling

3. **Documentation**: For each schema, provide:
   - Historical context
   - Common use cases
   - Comparison with similar schemes
   - Example conversions

## Community Contributions

We welcome community contributions for additional schemas. Please:
1. Follow the schema format in `schemas/examples/`
2. Include comprehensive mappings
3. Add tests and documentation
4. Submit via pull request

## Future Considerations

- **Machine Learning**: Auto-detect transliteration schemes
- **Fuzzy Matching**: Handle common typos/variations
- **Composite Schemes**: Mix multiple schemes in one text
- **Script Variants**: Regional variations of standard scripts