//! Trueno-DB Browser Demo
//!
//! WebAssembly package for in-browser analytics with tiered compute:
//! - Tier 1: WebGPU (if available)
//! - Tier 2: WASM SIMD128
//! - Tier 3: Scalar fallback

use wasm_bindgen::prelude::*;
use web_sys::console;

// Re-export main types from parent crate
pub use trueno_db::wasm::*;

/// Get demo version
#[wasm_bindgen]
pub fn demo_version() -> String {
    format!("trueno-db-wasm v{}", env!("CARGO_PKG_VERSION"))
}
