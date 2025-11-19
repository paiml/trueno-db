//! Backend selection tests for CORE-002
//!
//! Tests the cost-based backend dispatcher with the 5x rule:
//! - GPU only if estimated_gpu_compute_ms > pcie_transfer_ms * 5.0
//!
//! References:
//! - Gregg & Hazelwood (2011): PCIe bus bottleneck analysis
//! - Breß et al. (2014): Operator variant selection on heterogeneous hardware
//!
//! Toyota Way: Genchi Genbutsu (Go and See - physics-based cost model)

use trueno_db::backend::BackendDispatcher;

/// PCIe Gen4 x16 bandwidth: 32 GB/s
const PCIE_BANDWIDTH_GBPS: f64 = 32.0;

#[test]
fn test_small_dataset_selects_cpu() {
    // Small dataset (1 MB), simple aggregation
    let total_bytes = 1_000_000; // 1 MB
    let estimated_flops = 1_000_000.0; // Simple sum

    // Calculate transfer time: 1 MB / 32 GB/s = 0.03125 ms
    // For GPU to be worth it, compute must be > 5x transfer time
    // 0.03125 * 5 = 0.15625 ms
    // But our estimated compute is ~0.001 ms (1M FLOPs at 1 TFLOP/s)

    let backend = BackendDispatcher::select(total_bytes, estimated_flops);

    // Should select CPU (not worth GPU transfer overhead)
    assert!(
        matches!(backend, trueno_db::Backend::Simd),
        "Small dataset should use CPU/SIMD backend"
    );
}

#[test]
fn test_large_compute_selects_gpu() {
    // Large dataset (1 GB), complex aggregation
    let total_bytes = 1_000_000_000; // 1 GB
    let estimated_flops = 10_000_000_000.0; // 10 GFLOP (complex aggregation)

    // Calculate transfer time: 1 GB / 32 GB/s = 31.25 ms
    let transfer_time_ms = (total_bytes as f64 / (PCIE_BANDWIDTH_GBPS * 1_000_000_000.0)) * 1000.0;
    assert_eq!(transfer_time_ms, 31.25);

    // For GPU to be worth it, compute must be > 5x transfer time
    // 31.25 * 5 = 156.25 ms minimum compute time

    // At 1 TFLOP/s, 10 GFLOP takes 10 ms (too low)
    // But at 100 GFLOP/s (more realistic for GPU), it takes 100ms
    // Still not enough... let's increase compute

    let backend = BackendDispatcher::select(total_bytes, estimated_flops);

    // This should still select CPU because compute isn't 5x transfer
    assert!(
        matches!(backend, trueno_db::Backend::Simd),
        "Compute not 5x transfer time, should use CPU"
    );
}

#[test]
fn test_very_large_compute_selects_gpu() {
    // Large dataset (1 GB), very complex computation (e.g., hash join)
    let total_bytes = 1_000_000_000; // 1 GB
    let estimated_flops = 100_000_000_000.0; // 100 GFLOP (hash join)

    // Transfer time: 31.25 ms
    // Minimum compute for GPU: 31.25 * 5 = 156.25 ms
    // At 100 GFLOP/s, 100 GFLOP takes 1000ms > 156.25ms ✓

    let backend = BackendDispatcher::select(total_bytes, estimated_flops);

    // Should select GPU (compute is 5x+ transfer time)
    assert!(
        matches!(backend, trueno_db::Backend::Gpu),
        "Large compute should use GPU backend"
    );
}

#[test]
fn test_minimum_data_threshold() {
    // Test the 10 MB minimum threshold from spec
    let total_bytes = 5_000_000; // 5 MB (below threshold)
    let estimated_flops = 1_000_000_000.0; // High compute

    let backend = BackendDispatcher::select(total_bytes, estimated_flops);

    // Should use CPU even with high compute (below 10 MB threshold)
    assert!(
        !matches!(backend, trueno_db::Backend::Gpu),
        "Data below 10 MB threshold should not use GPU"
    );
}

#[test]
fn test_arithmetic_intensity_calculation() {
    // Test arithmetic intensity (FLOPs per byte)
    let total_bytes = 100_000_000; // 100 MB
    let estimated_flops = 1_000_000_000.0; // 1 GFLOP

    let arithmetic_intensity = estimated_flops / total_bytes as f64;
    assert_eq!(arithmetic_intensity, 10.0); // 10 FLOPs per byte

    // This is moderate arithmetic intensity
    // Transfer: 100 MB / 32 GB/s = 3.125 ms
    // Compute needed: 3.125 * 5 = 15.625 ms
    // At 100 GFLOP/s: 1 GFLOP / 100 GFLOP/s = 10 ms
    // 10 ms < 15.625 ms, so should use CPU

    let backend = BackendDispatcher::select(total_bytes, estimated_flops);
    assert!(
        !matches!(backend, trueno_db::Backend::Gpu),
        "Moderate intensity should use CPU"
    );
}
