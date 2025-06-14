# Round-Trip Transliteration Accuracy Report

**Generated**: Sat Jun 14 16:17:19 2025

## Executive Summary

- **Total Tests**: 504
- **Successful Tests**: 432
- **Overall Success Rate**: 85.7%

## Engine Rankings

### By Average Accuracy

1. **Shlesha**: 100.0% (Success Rate: 100.0%)
2. **Vidyut_Lipi**: 99.1% (Success Rate: 42.9%)
3. **Dharmamitra**: 97.3% (Success Rate: 100.0%)
4. **Aksharamukha**: 96.3% (Success Rate: 100.0%)

## Detailed Results

### Shlesha

- **Success Rate**: 100.0%
- **Average Accuracy**: 100.0%
- **Tests Passed**: 126/126

#### Lowest Accuracy Cases

- **devanagari_iast** (basic_sanskrit): 100.0%
  - Original: `अग्निमीळे पुरोहितं`
  - Round-trip: `अग्निमीळे पुरोहितं`

- **devanagari_iast** (basic_sanskrit): 100.0%
  - Original: `तत्त्वमसि`
  - Round-trip: `तत्त्वमसि`

- **devanagari_iast** (basic_sanskrit): 100.0%
  - Original: `सर्वं खल्विदं ब्रह्म`
  - Round-trip: `सर्वं खल्विदं ब्रह्म`

### Aksharamukha

- **Success Rate**: 100.0%
- **Average Accuracy**: 96.3%
- **Tests Passed**: 126/126

#### Lowest Accuracy Cases

- **devanagari_iast** (rare_characters): 0.0%
  - Original: `क़ख़ग़ज़फ़`
  - Round-trip: `क़ख़ग़ज़फ़`
  - Issues: replace: 'क़ख़ग़ज़फ़' -> 'क़ख़ग़ज़फ़'

- **devanagari_harvard_kyoto** (rare_characters): 0.0%
  - Original: `क़ख़ग़ज़फ़`
  - Round-trip: `क़ख़ग़ज़फ़`
  - Issues: replace: 'क़ख़ग़ज़फ़' -> 'क़ख़ग़ज़फ़'

- **devanagari_slp1** (rare_characters): 0.0%
  - Original: `क़ख़ग़ज़फ़`
  - Round-trip: `क़ख़ग़ज़फ़`
  - Issues: replace: 'क़ख़ग़ज़फ़' -> 'क़ख़ग़ज़फ़'

### Dharmamitra

- **Success Rate**: 100.0%
- **Average Accuracy**: 97.3%
- **Tests Passed**: 126/126

#### Lowest Accuracy Cases

- **devanagari_iast** (rare_characters): 0.0%
  - Original: `क़ख़ग़ज़फ़`
  - Round-trip: `क़ख़ग़ज़फ़`
  - Issues: replace: 'क़ख़ग़ज़फ़' -> 'क़ख़ग़ज़फ़'

- **devanagari_harvard_kyoto** (rare_characters): 0.0%
  - Original: `क़ख़ग़ज़फ़`
  - Round-trip: `क़ख़ग़ज़फ़`
  - Issues: replace: 'क़ख़ग़ज़फ़' -> 'क़ख़ग़ज़फ़'

- **devanagari_slp1** (rare_characters): 0.0%
  - Original: `क़ख़ग़ज़फ़`
  - Round-trip: `क़ख़ग़ज़फ़`
  - Issues: replace: 'क़ख़ग़ज़फ़' -> 'क़ख़ग़ज़फ़'

### Vidyut_Lipi

- **Success Rate**: 42.9%
- **Average Accuracy**: 99.1%
- **Tests Passed**: 54/126

#### Lowest Accuracy Cases

- **devanagari_harvard_kyoto** (rare_characters): 80.0%
  - Original: `ॠकारलृकारौ`
  - Round-trip: `ॠकारऌकारौ`
  - Issues: replace: 'लृ' -> 'ऌ'

- **devanagari_iast** (vedic_accents): 87.5%
  - Original: `व॒रुणे॑न पश्यता॒`
  - Round-trip: `व॒रुण्ḙन पश्यता॒`
  - Issues: replace: 'े॑' -> '्ḙ'

- **devanagari_iast** (vedic_accents): 94.7%
  - Original: `इन्द्र॑ म॒रुत्व॑ान्`
  - Round-trip: `इन्द्र॑ म॒रुत्व॑आन्`
  - Issues: replace: 'ा' -> 'आ'
