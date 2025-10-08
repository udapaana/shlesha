// Runtime compilation and caching modules are only available for non-WASM targets
// WASM environments cannot spawn processes or access the filesystem in the same way
#[cfg(not(target_arch = "wasm32"))]
pub mod cache;
#[cfg(not(target_arch = "wasm32"))]
pub mod compiler;

#[cfg(not(target_arch = "wasm32"))]
pub use cache::{CacheManager, CompilationCache};
#[cfg(not(target_arch = "wasm32"))]
pub use compiler::RuntimeCompiler;
