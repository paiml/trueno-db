<div align="center">

<img src=".github/trueno-db-hero.svg" alt="trueno-db" width="600">

**GPU-First Embedded Analytics with SIMD Fallback**

[![CI](https://github.com/paiml/trueno-db/actions/workflows/ci.yml/badge.svg)](https://github.com/paiml/trueno-db/actions)

</div>

---

GPU-first embedded analytics database with graceful degradation: **GPU → SIMD → Scalar**

## Features

- **Cost-based dispatch**: GPU only when compute > 5x transfer time
- **Morsel-based paging**: Out-of-core execution (128MB chunks)
- **JIT WGSL compiler**: Kernel fusion for single-pass execution
- **GPU kernels**: SUM, MIN, MAX, COUNT, AVG, fused filter+sum
- **SIMD fallback**: Trueno integration (AVX-512/AVX2/SSE2)
- **SQL interface**: SELECT, WHERE, aggregations, ORDER BY, LIMIT

## Installation

```toml
[dependencies]
trueno-db = "0.3"

# Optional: GPU acceleration
trueno-db = { version = "0.3", features = ["gpu"] }
```

## Quick Start

```rust
use trueno_db::query::{QueryEngine, QueryExecutor};
use trueno_db::storage::StorageEngine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let storage = StorageEngine::load_parquet("data/events.parquet")?;
    let engine = QueryEngine::new();
    let executor = QueryExecutor::new();

    let plan = engine.parse(
        "SELECT COUNT(*), SUM(value), AVG(value) FROM events WHERE value > 100"
    )?;
    let result = executor.execute(&plan, &storage)?;
    Ok(())
}
```

## Performance

**SIMD Aggregation (1M rows, AMD Threadripper 7960X)**:

| Operation | SIMD | Scalar | Speedup |
|-----------|------|--------|---------|
| SUM | 228µs | 634µs | 2.78x |
| MIN | 228µs | 1,048µs | 4.60x |
| MAX | 228µs | 257µs | 1.13x |
| AVG | 228µs | 634µs | 2.78x |

## Examples

```bash
cargo run --example sql_query_interface --release
cargo run --example benchmark_shootout --release
cargo run --example gaming_leaderboards --release
cargo run --example gpu_aggregations --features gpu --release
```

## Development

```bash
make build           # Build
make test            # Run tests
make quality-gate    # lint + test + coverage
make bench-comparison
```

## License

MIT
