# Transliteration Engine Benchmarks

This directory contains performance and accuracy comparisons between Shlesha and other Sanskrit transliteration tools.

## Compared Tools

### Shlesha
- **Repository**: This project
- **Language**: Rust
- **Features**: Extensible schema system, Vedic support, multi-script

### Aksharamukha
- **Repository**: https://github.com/virtualvinodh/aksharamukha
- **Language**: Python
- **Features**: 100+ scripts, web interface, extensive coverage

### Dharmamitra
- **Repository**: https://github.com/sanskrit-coders/indic-transliteration  
- **Language**: Python
- **Features**: Multi-language support, good Sanskrit coverage

### Vidyut-lipi
- **Repository**: https://github.com/ambuda-org/vidyut-lipi
- **Language**: Rust
- **Features**: High performance, Sanskrit-focused

## Test Categories

### 1. Accuracy Tests (`accuracy/`)
- **Sanskrit Classical**: RV 1.1.1, BG verses, Upanishad passages
- **Vedic Texts**: Accented verses from various Vedas
- **Edge Cases**: Rare characters, complex conjuncts, special marks

### 2. Performance Tests (`performance/`)
- **Speed**: Throughput (characters/second)
- **Memory**: RAM usage during transliteration
- **Scalability**: Performance with large texts

### 3. Feature Coverage (`coverage/`)
- **Script Support**: Number of supported scripts
- **Character Coverage**: Unicode coverage per script
- **Vedic Features**: Accent marks, special symbols

## Running Benchmarks

```bash
# Install dependencies
pip install aksharamukha indic-transliteration
cargo install vidyut-lipi

# Run all benchmarks
./run_benchmarks.sh

# Run specific category
./run_accuracy_tests.sh
./run_performance_tests.sh
./run_coverage_tests.sh
```

## Results Summary

Results are updated automatically and stored in `results/` with timestamps.

### Latest Results (TODO: Auto-generate)

| Tool | Speed (chars/sec) | Memory (MB) | Sanskrit Accuracy | Vedic Support |
|------|------------------|-------------|------------------|---------------|
| Shlesha | TBD | TBD | TBD | ✅ Excellent |
| Aksharamukha | TBD | TBD | TBD | ⚠️ Limited |
| Dharmamitra | TBD | TBD | TBD | ⚠️ Limited |
| Vidyut-lipi | TBD | TBD | TBD | ❌ None |

## Test Data

Test data is sourced from:
- **Digital Corpus of Sanskrit (DCS)**
- **GRETIL** (Göttingen Register of Electronic Texts in Indian Languages)
- **Sacred-texts.com** Vedic collection
- **Unicode test suites** for edge cases

## Contributing

To add new benchmark tests:
1. Add test cases to appropriate category folder
2. Update the benchmark runner scripts
3. Ensure test data is properly licensed/attributed
4. Run tests and update results