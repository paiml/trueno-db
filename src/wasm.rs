//! WebAssembly bindings for trueno-db browser deployment.
//!
//! Provides JavaScript-accessible API with tiered compute:
//! - Tier 1: WebGPU compute shaders (if available)
//! - Tier 2: WASM SIMD128 via trueno backend
//! - Tier 3: Scalar fallback
//!
//! # Architecture
//!
//! ```text
//! Browser JS → wasm-bindgen → Database → trueno (SIMD128) → Results
//!                              ↓
//!                         QueryEngine
//!                              ↓
//!                         Arrow Tables
//! ```

#![cfg(target_arch = "wasm32")]

use js_sys::{Object, Reflect, Uint8Array};
use wasm_bindgen::prelude::*;
use web_sys::{console, window};

#[cfg(feature = "gpu")]
use wasm_bindgen_futures::JsFuture;

pub mod http_range;

/// Initialize WASM module with panic hook for better error messages
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
    console::log_1(&"trueno-db WASM initialized".into());
}

/// Get library version
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Database configuration for browser deployment
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct DatabaseConfig {
    backend: String,
    cache_size_mb: usize,
}

#[wasm_bindgen]
impl DatabaseConfig {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            backend: "auto".to_string(),
            cache_size_mb: 256,
        }
    }

    /// Set compute backend: "auto", "webgpu", "simd128", "scalar"
    #[wasm_bindgen]
    pub fn backend(mut self, backend: &str) -> Self {
        self.backend = backend.to_string();
        self
    }

    #[wasm_bindgen]
    pub fn cache_size_mb(mut self, size: usize) -> Self {
        self.cache_size_mb = size;
        self
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// In-browser analytics database with GPU/SIMD acceleration
#[wasm_bindgen]
pub struct Database {
    config: DatabaseConfig,
    // Will add: inner: Arc<crate::Database>
}

#[wasm_bindgen]
impl Database {
    /// Create new database instance
    #[wasm_bindgen(constructor)]
    pub fn new(config: Option<DatabaseConfig>) -> Self {
        let config = config.unwrap_or_default();
        console::log_1(&format!("Database created with backend: {}", config.backend).into());

        Self { config }
    }

    /// Load table from URL (supports HTTP range requests for streaming)
    #[wasm_bindgen]
    pub async fn load_table(&mut self, name: String, url: String) -> Result<(), JsValue> {
        console::log_1(&format!("Loading table '{}' from {}", name, url).into());

        // TODO: Implement HTTP range request Parquet reader
        Err(JsValue::from_str("Not yet implemented"))
    }

    /// Execute SQL query and return Arrow IPC format
    #[wasm_bindgen]
    pub async fn query(&self, sql: String) -> Result<Uint8Array, JsValue> {
        console::log_1(&format!("Executing query: {}", sql).into());

        // TODO: Implement query execution
        Err(JsValue::from_str("Not yet implemented"))
    }

    /// Get query execution plan (for debugging)
    #[wasm_bindgen]
    pub fn explain(&self, sql: String) -> Result<String, JsValue> {
        console::log_1(&format!("EXPLAIN: {}", sql).into());

        // TODO: Implement EXPLAIN
        Err(JsValue::from_str("Not yet implemented"))
    }
}

/// Detect available compute capabilities
#[wasm_bindgen]
pub async fn detect_capabilities() -> Result<JsValue, JsValue> {
    let window = window().ok_or_else(|| JsValue::from_str("No window"))?;
    let navigator = window.navigator();

    let caps = Object::new();

    // Check WebGPU
    let webgpu_available = check_webgpu(&navigator).await;
    Reflect::set(&caps, &"webgpu".into(), &JsValue::from(webgpu_available))?;

    // Check SIMD128 (compile-time feature)
    #[cfg(target_feature = "simd128")]
    let simd128 = true;
    #[cfg(not(target_feature = "simd128"))]
    let simd128 = false;
    Reflect::set(&caps, &"simd128".into(), &JsValue::from(simd128))?;

    // Determine best tier
    let tier = if webgpu_available {
        "webgpu"
    } else if simd128 {
        "simd128"
    } else {
        "scalar"
    };
    Reflect::set(&caps, &"tier".into(), &JsValue::from(tier))?;

    Ok(caps.into())
}

async fn check_webgpu(_navigator: &web_sys::Navigator) -> bool {
    #[cfg(feature = "gpu")]
    {
        if let Some(gpu) = _navigator.gpu() {
            let adapter_promise = gpu.request_adapter();
            if let Ok(adapter_val) = JsFuture::from(adapter_promise).await {
                return !adapter_val.is_null() && !adapter_val.is_undefined();
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_config_builder() {
        let config = DatabaseConfig::new().backend("simd128").cache_size_mb(512);

        assert_eq!(config.backend, "simd128");
        assert_eq!(config.cache_size_mb, 512);
    }
}
