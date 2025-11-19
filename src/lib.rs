//! # Trueno-DB: GPU-First Embedded Analytics Database
//!
//! **Version**: 0.1.0 (Phase 1 MVP)
//!
//! Trueno-DB is a GPU-aware, compute-intensity-based embedded analytics database
//! designed for high-performance aggregations with graceful degradation from
//! GPU → SIMD → Scalar.
//!
//! ## Design Principles (Toyota Way Aligned)
//!
//! - **Muda elimination**: Kernel fusion minimizes `PCIe` transfers
//! - **Poka-Yoke safety**: Out-of-core execution prevents VRAM OOM
//! - **Genchi Genbutsu**: Physics-based cost model (5x rule for GPU dispatch)
//! - **Jidoka**: Backend equivalence tests (GPU == SIMD == Scalar)
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use trueno_db::Database;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let db = Database::builder()
//!         .backend(Backend::CostBased)
//!         .morsel_size_mb(128)
//!         .build()?;
//!
//!     db.load_table("events", "data/events.parquet").await?;
//!
//!     let result = db.query(
//!         "SELECT category, sum(value) FROM events GROUP BY category"
//!     ).execute().await?;
//!
//!     println!("Backend: {:?}", result.backend_info());
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

pub mod backend;
pub mod error;
pub mod query;
pub mod storage;

pub use error::{Error, Result};

/// Database instance
pub struct Database {
    _private: (),
}

/// Backend selection strategy
#[derive(Debug, Clone, Copy)]
pub enum Backend {
    /// Cost-based dispatch (arithmetic intensity)
    CostBased,
    /// Force GPU execution
    Gpu,
    /// Force SIMD execution
    Simd,
}

impl Database {
    /// Create a new database builder
    #[must_use]
    pub fn builder() -> DatabaseBuilder {
        DatabaseBuilder::default()
    }
}

/// Database builder
#[derive(Default)]
pub struct DatabaseBuilder {
    _private: (),
}

impl DatabaseBuilder {
    /// Set backend selection strategy
    #[must_use]
    pub const fn backend(self, _backend: Backend) -> Self {
        self
    }

    /// Set morsel size for out-of-core execution (Poka-Yoke)
    #[must_use]
    pub const fn morsel_size_mb(self, _size: usize) -> Self {
        self
    }

    /// Build the database
    ///
    /// # Errors
    ///
    /// Returns error if GPU initialization fails
    pub const fn build(self) -> Result<Database> {
        Ok(Database { _private: () })
    }
}
