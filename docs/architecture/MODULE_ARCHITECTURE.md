# Shlesha Transliterator - Module Architecture

> **Note**: All module-level todos have been moved to the centralized `TODO.md` file in the project root. This provides better visibility and tracking of all development tasks.

## Hub-and-Spoke Design

### Core Philosophy
- **Central Hub**: Devanagari ↔ ISO-15919 bidirectional mapping
- **Spokes**: All other scripts connect through the hub
- **Runtime Extensibility**: Dynamic schema loading without recompilation
- **Centralized Task Management**: All todos maintained in `TODO.md` file

## Module Definitions

### 1. Hub Module
**Status**: IMMUTABLE (except when actively developing)
**Interface**: 
```rust
pub trait Hub {
    fn deva_to_iso(&self, input: &str) -> Result<String, HubError>;
    fn iso_to_deva(&self, input: &str) -> Result<String, HubError>;
}
```
**Todos**: See `TODO.md`

### 2. Script Converter Module  
**Status**: IMMUTABLE
**Interface**:
```rust
pub trait ScriptConverter {
    fn to_hub(&self, script: &str, input: &str) -> Result<HubInput, ConverterError>;
}
```
**Todos**: See `TODO.md`

### 3. Target Generator Module
**Status**: IMMUTABLE  
**Interface**:
```rust
pub trait TargetGenerator {
    fn from_hub(&self, hub_output: &HubOutput, target_script: &str) -> Result<String, GeneratorError>;
}
```
**Todos**: See `TODO.md`

### 4. Runtime Extension Module
**Status**: IMMUTABLE
**Interface**:
```rust
pub trait RuntimeExtension {
    fn load_schema(&mut self, schema_path: &str) -> Result<(), ExtensionError>;
    fn register_mapping(&mut self, mapping: CustomMapping) -> Result<(), ExtensionError>;
}
```
**Todos**: See `TODO.md`

### 5. Schema Registry Module
**Status**: IMMUTABLE
**Interface**:
```rust
pub trait SchemaRegistry {
    fn get_schema(&self, script_name: &str) -> Option<&Schema>;
    fn register_schema(&mut self, name: String, schema: Schema) -> Result<(), RegistryError>;
}
```
**Todos**: See `TODO.md`

### 6. Test Module
**Status**: IMMUTABLE
**Interface**:
```rust
pub trait TestFramework {
    fn run_correctness_tests(&self) -> TestResults;
    fn run_roundtrip_tests(&self) -> TestResults;
}
```
**Todos**: See `TODO.md`

### 7. Benchmark Module
**Status**: IMMUTABLE
**Interface**:
```rust
pub trait BenchmarkFramework {
    fn run_performance_tests(&self) -> BenchmarkResults;
    fn compare_implementations(&self) -> ComparisonResults;
}
```
**Todos**: See `TODO.md`

### 8. CLI Module
**Status**: IMMUTABLE
**Interface**:
```rust
pub trait CLIInterface {
    fn execute_command(&self, args: &[String]) -> Result<(), CLIError>;
}
```
**Todos**: See `TODO.md`

### 9. API Bindings Module
**Status**: IMMUTABLE
**Interface**:
```rust
pub trait APIBindings {
    fn python_bindings(&self) -> PyResult<()>;
    fn wasm_bindings(&self) -> Result<(), WasmError>;
}
```
**Todos**: See `TODO.md`

### 10. Error Handling Module
**Status**: IMMUTABLE
**Interface**:
```rust
pub trait ErrorHandler {
    fn handle_error(&self, error: &dyn Error) -> ErrorResponse;
}
```
**Todos**: See `TODO.md`

### 11. Validation Module
**Status**: IMMUTABLE
**Interface**:
```rust
pub trait Validator {
    fn validate_input(&self, script: &str, text: &str) -> ValidationResult;
    fn validate_output(&self, result: &str) -> ValidationResult;
}
```
**Todos**: See `TODO.md`

### 12. Cache Module
**Status**: IMMUTABLE
**Interface**:
```rust
pub trait Cache {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: String, value: String);
}
```
**Todos**: See `TODO.md`

### 13. Configuration Module
**Status**: IMMUTABLE
**Interface**:
```rust
pub trait Configuration {
    fn load_config(&mut self, path: &str) -> Result<(), ConfigError>;
    fn get_setting(&self, key: &str) -> Option<&str>;
}
```
**Todos**: See `TODO.md`

## Module Interaction Rules

1. **Single Module Mutability**: Only one module can be mutable at a time
2. **Interface-Only Access**: Modules communicate only through defined interfaces
3. **Todo-Based Changes**: Cross-module changes require adding to target module's todo list
4. **Hub-Centric Flow**: All transliteration flows through Devanagari ↔ ISO-15919 hub