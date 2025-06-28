# Mapping Data Module Design

## Overview

The `mapping_data` module provides a centralized, data-driven approach to managing script conversion mappings. It separates mapping data from conversion logic, enabling both runtime and compile-time access to mappings.

## Module Interface

```rust
// src/modules/mapping_data/mod.rs

use std::collections::HashMap;
use crate::modules::{ModuleTodoQueue, TodoItem, TodoPriority};

/// Public interface for the mapping data module
pub struct MappingDataManager {
    todo_queue: ModuleTodoQueue,
}

impl MappingDataManager {
    pub fn new(todo_queue: ModuleTodoQueue) -> Self {
        Self { todo_queue }
    }
    
    /// Process todos for this module
    pub fn process_todos(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(todo) = self.todo_queue.get_todo("mapping_data") {
            match todo.action.as_str() {
                "get_mappings" => self.handle_get_mappings(todo)?,
                "validate_mappings" => self.handle_validate_mappings(todo)?,
                "compose_mappings" => self.handle_compose_mappings(todo)?,
                _ => return Err(format!("Unknown action: {}", todo.action).into()),
            }
        }
        Ok(())
    }
}
```

## Todo Queue Protocol

### Request: Get Mappings
```rust
TodoItem {
    module: "mapping_data",
    action: "get_mappings",
    data: json!({
        "from_script": "iast",
        "to_script": "iso",
        "mapping_type": "base"  // "base" or "composed"
    }),
    priority: TodoPriority::Normal,
    response_channel: Some(response_tx),
}
```

### Response: Mappings Data
```rust
TodoResponse {
    success: true,
    data: json!({
        "mappings": {
            "a": "a",
            "ā": "ā",
            "ṛ": "r̥",
            // ...
        }
    }),
}
```

### Request: Compose Mappings
```rust
TodoItem {
    module: "mapping_data",
    action: "compose_mappings",
    data: json!({
        "first_step": {"from": "iast", "to": "iso"},
        "second_step": {"from": "iso", "to": "devanagari"}
    }),
    priority: TodoPriority::High,
    response_channel: Some(response_tx),
}
```

## Data Storage Structure

```
mappings/
├── base/
│   ├── iast.toml
│   ├── devanagari.toml
│   ├── iso.toml
│   └── ...
├── composed/
│   └── .gitignore  # Generated at build time
└── metadata.toml
```

### Base Mapping Format (TOML)

```toml
# mappings/base/iast.toml
[metadata]
script = "iast"
type = "roman"
has_implicit_a = false

[to_iso.vowels]
"a" = "a"
"ā" = "ā"
"i" = "i"
"ī" = "ī"
"u" = "u"
"ū" = "ū"
"ṛ" = "r̥"
"ṝ" = "r̥̄"

[to_iso.consonants]
"k" = "k"
"kh" = "kh"
"g" = "g"
# ...

[from_iso]
# Reverse mappings if different from inverse
```

## Usage by Other Modules

### Hub Module
```rust
// In hub module
fn iso_to_deva(&self, iso_text: &str) -> Result<String, HubError> {
    // Request mappings via todo queue
    let todo = TodoItem {
        module: "mapping_data",
        action: "get_mappings",
        data: json!({"from_script": "iso", "to_script": "devanagari"}),
        // ...
    };
    
    self.todo_queue.add_todo(todo);
    let response = // ... wait for response
    
    // Use mappings for conversion
}
```

### Script Converter Module
```rust
// In IAST converter
impl IASTConverter {
    pub fn new(todo_queue: ModuleTodoQueue) -> Self {
        // Request mappings at initialization
        let mappings = Self::load_mappings(&todo_queue);
        Self { mappings, todo_queue }
    }
}
```

### Build Script
```rust
// In build.rs
use shlesha_mapping_data::{MappingLoader, MappingComposer};

fn main() {
    // Load base mappings
    let iast_to_iso = MappingLoader::load("mappings/base/iast.toml")?;
    let iso_to_deva = MappingLoader::load("mappings/base/iso_to_deva.toml")?;
    
    // Compose mappings
    let iast_to_deva = MappingComposer::compose(iast_to_iso, iso_to_deva)?;
    
    // Generate pre-computed converters
    generate_converter_code(iast_to_deva);
}
```

## Implementation Phases

1. **Phase 1**: Create basic module structure with file loading
2. **Phase 2**: Implement todo queue handlers
3. **Phase 3**: Migrate existing hardcoded mappings to TOML files
4. **Phase 4**: Update hub and converters to use mapping_data module
5. **Phase 5**: Update build.rs to use mapping data for pre-computation

## Benefits

1. **Single Source of Truth**: All mappings in one place
2. **Modularity**: Clean separation of data and logic
3. **Extensibility**: Easy to add new scripts/mappings
4. **Testability**: Mappings can be tested independently
5. **Build-time Optimization**: Same data used for pre-computation
6. **Maintainability**: Non-programmers can edit TOML files