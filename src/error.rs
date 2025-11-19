//! Error types for Trueno-DB
//!
//! Toyota Way: Clear error messages with actionable guidance (Respect for People)

use thiserror::Error;

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;

/// Trueno-DB error types
#[derive(Error, Debug)]
pub enum Error {
    /// GPU initialization failed
    #[error("GPU initialization failed: {0}\nFalling back to SIMD backend")]
    GpuInitFailed(String),

    /// VRAM exhaustion (should not occur with morsel paging)
    #[error("VRAM exhausted: {0}\nThis indicates a bug in morsel-based paging. Please report.")]
    VramExhausted(String),

    /// Backend equivalence test failed (critical bug)
    #[error("Backend equivalence failed: GPU result != SIMD result\nGPU: {gpu_result}\nSIMD: {simd_result}")]
    BackendMismatch {
        /// GPU computed result
        gpu_result: String,
        /// SIMD computed result
        simd_result: String,
    },

    /// Query parsing error
    #[error("SQL parse error: {0}")]
    ParseError(String),

    /// Storage error (Parquet/Arrow)
    #[error("Storage error: {0}")]
    StorageError(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Generic error
    #[error("{0}")]
    Other(String),
}
