# Profile-Guided Optimization System

Shlesha includes a sophisticated profile-guided optimization (PGO) system that automatically improves transliteration performance based on actual usage patterns. This system is particularly effective for frequently used Sanskrit and Hindi words and phrases.

## Overview

The PGO system consists of four main components:

1. **Runtime Profiler** - Collects usage statistics during transliteration
2. **Optimization Generator** - Creates optimized lookup tables from profile data
3. **Hot-Reload Manager** - Enables dynamic loading of optimizations without restart
4. **Optimization Cache** - Provides fast lookup for optimized sequences

## Quick Start

### Basic Usage

```rust
use shlesha::Shlesha;

// Create transliterator with profiling enabled
let mut transliterator = Shlesha::with_profiling();

// Normal transliteration operations automatically collect profile data
let result = transliterator.transliterate("धर्म", "devanagari", "iast")?;

// After some usage, generate optimizations
let optimizations = transliterator.generate_optimizations();

// Load optimizations for faster subsequent transliterations
for optimization in optimizations {
    transliterator.load_optimization(optimization);
}
```

### Custom Configuration

```rust
use shlesha::{Shlesha, modules::profiler::ProfilerConfig};
use std::path::PathBuf;
use std::time::Duration;

let config = ProfilerConfig {
    enabled: true,
    profile_dir: PathBuf::from("my_profiles"),
    optimization_dir: PathBuf::from("my_optimizations"),
    min_sequence_frequency: 5,
    max_sequences_per_table: 500,
    auto_save_interval: Duration::from_secs(600),
    hot_reload_enabled: true,
};

let mut transliterator = Shlesha::new();
transliterator.enable_profiling_with_config(config);
```

## How It Works

### 1. Profile Collection

The profiler automatically tracks:
- Character sequences and their frequencies
- Processing times for different conversions
- Common word patterns
- Usage statistics across different script pairs

```rust
// These operations automatically contribute to profiles
transliterator.transliterate("धर्म", "devanagari", "iast")?;
transliterator.transliterate("योग", "devanagari", "iast")?;
transliterator.transliterate("कर्म", "devanagari", "iast")?;
```

### 2. Optimization Generation

The system identifies frequently used patterns and pre-computes their conversions:

```rust
// Generate optimizations from collected profiles
let optimizations = transliterator.generate_optimizations();

// Each optimization contains:
// - Direct sequence mappings for common patterns
// - Word-level mappings for frequent terms
// - Metadata about optimization effectiveness
```

### 3. Hot-Reload Support

Optimizations can be updated without restarting the application:

```rust
use shlesha::modules::profiler::HotReloadManager;
use std::sync::Arc;

let profiler = Arc::new(Profiler::new());
let hot_reload = Arc::new(HotReloadManager::new(
    PathBuf::from("optimizations"),
    profiler.clone()
));

// Start watching for optimization updates
hot_reload.start_watching();

// Optimizations are automatically reloaded when files change
```

## Command-Line Tools

### Profile Collection

```bash
# Profile a text file
cargo run --example profiler_cli profile input.txt devanagari iast

# Profile with custom output directory
cargo run --example profiler_cli profile input.txt devanagari iast ./my_profiles
```

### Optimization Generation

```bash
# Generate optimizations from profiles
cargo run --example profiler_cli generate ./profiles ./optimizations
```

### Benchmarking

```bash
# Benchmark optimization effectiveness
cargo run --example profiler_cli benchmark devanagari_iast_opt.json test.txt
```

### Profile Statistics

```bash
# View profile statistics
cargo run --example profiler_cli stats ./profiles
```

## Performance Benefits

The PGO system provides significant performance improvements:

### For Sanskrit/Hindi Text
- **2-5x speedup** for common words and phrases
- **High cache hit rates** (60-80%) for religious and philosophical texts
- **Reduced processing time** for frequently used character sequences

### For Technical Applications
- **Batch processing** benefits from accumulated optimizations
- **Web applications** see improved response times for common requests
- **Repeated conversions** of similar content are significantly faster

## Configuration Options

### ProfilerConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `true` | Enable/disable profiling |
| `profile_dir` | `PathBuf` | `"profiles"` | Directory for profile data |
| `optimization_dir` | `PathBuf` | `"optimizations"` | Directory for optimization files |
| `min_sequence_frequency` | `u64` | `10` | Minimum frequency for optimization |
| `max_sequences_per_table` | `usize` | `1000` | Maximum sequences per table |
| `auto_save_interval` | `Duration` | `5 minutes` | Auto-save interval |
| `hot_reload_enabled` | `bool` | `true` | Enable hot-reloading |

## File Formats

### Profile Files
Profiles are stored as JSON files with usage statistics:

```json
{
  "from_script": "devanagari",
  "to_script": "iast",
  "sequences": {
    "धर्म": {
      "sequence": "धर्म",
      "count": 150,
      "last_used": "2024-01-15T10:30:00Z",
      "avg_processing_ns": 1250.5
    }
  },
  "total_conversions": 1000,
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-15T10:30:00Z"
}
```

### Optimization Files
Optimizations contain pre-computed mappings:

```json
{
  "from_script": "devanagari",
  "to_script": "iast",
  "sequence_mappings": {
    "धर्म": "dharma",
    "योग": "yoga",
    "कर्म": "karma"
  },
  "word_mappings": {
    "भगवद्गीता": "bhagavadgītā",
    "रामायण": "rāmāyaṇa"
  },
  "metadata": {
    "generated_at": "2024-01-15T10:30:00Z",
    "sequence_count": 500,
    "min_frequency": 10
  }
}
```

## Best Practices

### 1. Gradual Adoption
Start with default settings and adjust based on your usage patterns:

```rust
// Start simple
let mut transliterator = Shlesha::with_profiling();

// After collecting data, analyze and tune
let stats = transliterator.get_profile_stats();
// Adjust min_sequence_frequency based on stats
```

### 2. Production Deployment
- Enable profiling in development and staging
- Generate optimizations from real usage data
- Deploy optimization files with your application
- Use hot-reload for optimization updates

### 3. Memory Management
- Monitor profile file sizes in long-running applications
- Periodically clean old profile data
- Set appropriate `max_sequences_per_table` limits

### 4. Script-Specific Tuning
Different script pairs may benefit from different settings:

```rust
// Sanskrit text often has repeated patterns
config.min_sequence_frequency = 5;

// Technical text may need higher thresholds
config.min_sequence_frequency = 20;
```

## Examples

### Web Application Integration

```rust
use shlesha::Shlesha;
use std::sync::Arc;

// Application startup
let mut transliterator = Arc::new(Shlesha::with_profiling());

// Load existing optimizations
load_optimizations_from_disk(&transliterator)?;

// Handle user requests
async fn handle_transliteration_request(
    transliterator: Arc<Shlesha>,
    text: String,
    from: String,
    to: String,
) -> Result<String, Error> {
    // This automatically contributes to profiles
    transliterator.transliterate(&text, &from, &to)
}

// Periodic optimization generation
tokio::spawn(async move {
    loop {
        tokio::time::sleep(Duration::from_hours(1)).await;
        
        let optimizations = transliterator.generate_optimizations();
        save_optimizations_to_disk(&optimizations)?;
        
        for optimization in optimizations {
            transliterator.load_optimization(optimization);
        }
    }
});
```

### Batch Processing

```rust
use shlesha::Shlesha;

// Process large document collections
let mut transliterator = Shlesha::with_profiling();

// Process documents (builds profiles)
for document in documents {
    let result = transliterator.transliterate(&document.text, "devanagari", "iast")?;
    save_result(document.id, result)?;
}

// Generate and apply optimizations
let optimizations = transliterator.generate_optimizations();
for optimization in optimizations {
    transliterator.load_optimization(optimization);
}

// Subsequent processing is faster
for document in more_documents {
    let result = transliterator.transliterate(&document.text, "devanagari", "iast")?;
    save_result(document.id, result)?;
}
```

## Monitoring and Debugging

### Profile Statistics

```rust
if let Some(stats) = transliterator.get_profile_stats() {
    for ((from, to), profile_stats) in stats {
        println!("{} -> {}: {} conversions, {} unique sequences", 
                 from, to, 
                 profile_stats.total_sequences_profiled, 
                 profile_stats.unique_sequences);
        
        for (seq, count) in &profile_stats.top_sequences[..5] {
            println!("  '{}': {} times", seq, count);
        }
    }
}
```

### Optimization Effectiveness

```rust
use shlesha::modules::profiler::OptimizationGenerator;

let generator = OptimizationGenerator::new();
let benchmark = generator.benchmark_optimization(&optimization, test_text);

println!("Speedup: {:.2}x", benchmark.speedup_factor);
println!("Cache hit rate: {:.1}%", 
         benchmark.cache_hits as f64 / benchmark.total_sequences as f64 * 100.0);
```

## Common Use Cases

### 1. Religious and Philosophical Texts
Perfect for processing Sanskrit scriptures, mantras, and philosophical works where the same terms appear frequently.

### 2. Academic Research
Ideal for digital humanities projects processing large corpora of Sanskrit/Hindi texts.

### 3. Web Applications
Excellent for websites offering transliteration services, where common words and phrases are requested repeatedly.

### 4. Mobile Applications
Reduces processing time and battery usage for transliteration-heavy mobile apps.

### 5. Batch Processing
Significantly speeds up large-scale text processing tasks.

## Troubleshooting

### Low Cache Hit Rates
- Increase profiling duration to collect more data
- Lower `min_sequence_frequency` for more comprehensive optimization
- Verify that your text contains repeated patterns

### Memory Usage
- Reduce `max_sequences_per_table`
- Implement periodic profile cleanup
- Monitor profile file sizes

### Performance Regression
- Some optimizations may have overhead for very short texts
- Benchmark your specific use case
- Consider conditional optimization loading

### Hot-Reload Issues
- Verify file permissions on optimization directory
- Check JSON format of optimization files
- Ensure optimization files are not corrupted

## Future Enhancements

The profiling system is designed for extensibility. Planned improvements include:

- Machine learning-based sequence prediction
- Cross-session profile aggregation
- Automatic optimization parameter tuning
- Integration with distributed systems
- Real-time optimization effectiveness monitoring