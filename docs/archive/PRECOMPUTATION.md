# Pre-computation Performance Optimization

Shlesha provides compile-time performance optimization through pre-computed direct converters that bypass the hub-and-spoke architecture for maximum speed.

## Overview

By default, Shlesha uses a hub-and-spoke architecture where conversions flow through central hub formats (Devanagari ↔ ISO-15919). While this ensures consistency and extensibility, it requires two steps for Roman↔Indic conversions:

- **Standard**: `IAST → ISO-15919 → Devanagari` (2 lookups)
- **Optimized**: `IAST → Devanagari` (1 lookup, ~2x faster)

## Feature Flags

### `precompute-common` (Default)

Pre-computes the most frequently used conversions:

```bash
cargo build  # Uses precompute-common by default
```

**Generated converters (6):**
- IAST ↔ Devanagari
- ITRANS ↔ Devanagari  
- SLP1 ↔ Devanagari

**Performance:** ~2x improvement for these common cases  
**Binary size:** +5-10MB

### `precompute-all`

Pre-computes all possible Roman↔Indic combinations:

```bash
cargo build --features "precompute-all"
```

**Generated converters (96):**
- 6 Roman scripts × 8 Indic scripts × 2 directions = 96 converters

**Performance:** ~2x improvement for all Roman↔Indic conversions  
**Binary size:** +35MB

### `precompute-roman-indic`

Pre-computes only Roman → Indic conversions:

```bash
cargo build --features "precompute-roman-indic"
```

**Generated converters (48):**
- All Roman scripts to all Indic scripts

**Performance:** ~2x improvement for Roman → Indic only  
**Binary size:** +18MB

### `precompute-indic-roman`

Pre-computes only Indic → Roman conversions:

```bash
cargo build --features "precompute-indic-roman"
```

**Generated converters (48):**
- All Indic scripts to all Roman scripts

**Performance:** ~2x improvement for Indic → Roman only  
**Binary size:** +18MB

### `no-precompute`

Disables all pre-computation for minimal binary size:

```bash
cargo build --no-default-features --features "no-precompute"
```

**Generated converters:** 0  
**Performance:** Standard hub-and-spoke performance  
**Binary size:** Minimal (~1.4MB)

## How It Works

### Build-Time Generation

The `build.rs` script automatically:

1. **Detects feature flags** to determine which converters to generate
2. **Composes mappings** by combining Roman→ISO and ISO→Devanagari mappings
3. **Generates Rust code** with direct conversion lookup tables
4. **Caches results** to avoid regeneration when nothing changes

### Runtime Selection

The `ScriptConverterRegistry` automatically:

1. **Checks for direct converters** first
2. **Falls back to hub-and-spoke** if no direct converter exists
3. **Maintains API compatibility** - no changes to user code required

### Caching System

The build system includes intelligent caching:

```bash
# First build - generates converters
warning: Pre-computing common script combinations
warning: Generating 6 pre-computed converters

# Subsequent builds - uses cache
warning: Skipping pre-computation: no changes detected
```

Cache invalidation triggers:
- Source converter files modified
- Hub module modified  
- Feature flags changed
- Generated files missing

## Performance Impact

### Benchmark Results

| Conversion Type | Standard | Pre-computed | Improvement |
|----------------|----------|--------------|-------------|
| IAST → Devanagari | 1.46M chars/sec | ~2.9M chars/sec | **~2x** |
| ITRANS → Devanagari | Similar | Similar | **~2x** |
| Devanagari → IAST | Similar | Similar | **~2x** |
| Roman → Roman | 77M chars/sec | 77M chars/sec | No change |
| Indic → Indic | 83M chars/sec | 83M chars/sec | No change |

### Binary Size Impact

| Configuration | Binary Size | Memory Usage | Compile Time |
|---------------|-------------|--------------|--------------|
| `no-precompute` | ~1.4MB | Minimal | Fastest |
| `precompute-common` | ~6-11MB | Low | Fast |
| `precompute-roman-indic` | ~19MB | Medium | Medium |
| `precompute-indic-roman` | ~19MB | Medium | Medium |
| `precompute-all` | ~36MB | High | Slower |

## Usage Recommendations

### For Applications

**General purpose applications:**
```bash
cargo build  # Uses precompute-common (default)
```

**High-performance Roman↔Indic conversion:**
```bash
cargo build --features "precompute-all"
```

**Memory-constrained environments:**
```bash
cargo build --no-default-features --features "no-precompute"
```

### For Libraries

**When distributing as a library:**
```bash
# Let users choose optimization level
cargo build --no-default-features
```

**When bundling with applications:**
```bash
cargo build --features "precompute-common"
```

### For WASM

**For web deployment (size-critical):**
```bash
wasm-pack build --target web --no-default-features --features "wasm,no-precompute"
```

**For Node.js (performance-critical):**
```bash
wasm-pack build --target nodejs --features "wasm,precompute-common"
```

## Implementation Details

### Generated Code Structure

Each pre-computed converter generates:

```rust
pub struct IastToDevanagariDirect {
    mappings: HashMap<&'static str, &'static str>,
}

impl IastToDevanagariDirect {
    pub fn new() -> Self {
        let mut mappings = HashMap::new();
        mappings.insert("ka", "क");
        mappings.insert("kha", "ख");
        // ... all mappings
        Self { mappings }
    }
}
```

### Registry Integration

```rust
pub struct PrecomputedRegistry {
    converters: HashMap<(String, String), Box<dyn ScriptConverter>>,
}

impl PrecomputedRegistry {
    pub fn get(&self, from: &str, to: &str) -> Option<&Box<dyn ScriptConverter>> {
        self.converters.get(&(from.to_string(), to.to_string()))
    }
}
```

## Troubleshooting

### Build Issues

**Error: "failed to resolve: use of undeclared type"**
- Cause: Trying to generate converters for unsupported combinations
- Solution: Use supported feature flags or implement missing converters

**Warning: "Regenerating: X changed"**
- Cause: Source files modified since last build
- Solution: Normal behavior - caches are being updated

### Performance Issues

**Not seeing performance improvements:**
- Check that you're using the right feature flags
- Verify the conversion path is pre-computed
- Use `cargo build --features "precompute-all"` to ensure coverage

**Binary too large:**
- Use `precompute-common` instead of `precompute-all`
- Use `no-precompute` for minimal size
- Consider lazy loading for large applications

### Memory Issues

**High memory usage:**
- Reduce pre-computation scope
- Use `precompute-common` instead of `precompute-all`
- Monitor RSS memory usage in production

## Future Enhancements

### Planned Features

1. **Lazy Loading**: Load pre-computed converters on-demand
2. **Compression**: Compress mapping tables for smaller binaries
3. **Partial Generation**: Generate only needed converters based on usage analysis
4. **Runtime Profiling**: Automatic optimization based on usage patterns

### Contributing

To add support for new script combinations:

1. Implement the base converters (Roman→ISO, Indic→Devanagari)
2. Add composition logic in `build.rs`
3. Update feature flag configurations
4. Add comprehensive tests

See [CONTRIBUTING.md](../CONTRIBUTING.md) for detailed guidelines.