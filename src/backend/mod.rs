//! Compute backend dispatcher
//!
//! Toyota Way Principles:
//! - Genchi Genbutsu: Physics-based cost model (`PCIe` Gen4 x16 = 32 GB/s)
//! - Muda elimination: GPU only if compute > 5x transfer time

/// Cost-based backend selection
///
/// References:
/// - Gregg & Hazelwood (2011): `PCIe` bus bottleneck analysis
/// - Breß et al. (2014): Operator variant selection on heterogeneous hardware
pub struct BackendDispatcher {
    _private: (),
}

/// `PCIe` Gen4 x16 bandwidth: 32 GB/s
const PCIE_BANDWIDTH_GBPS: f64 = 32.0;

/// Minimum data size for GPU consideration (10 MB)
const MIN_GPU_DATA_SIZE_BYTES: usize = 10_000_000;

/// GPU compute throughput (conservative estimate: 100 GFLOP/s)
/// Modern GPUs can achieve 1-10 TFLOP/s, but we use conservative estimate
const GPU_THROUGHPUT_GFLOPS: f64 = 100.0;

/// Transfer overhead multiplier (5x rule from spec)
/// GPU compute must be > 5x transfer time to be worthwhile
const TRANSFER_OVERHEAD_MULTIPLIER: f64 = 5.0;

impl BackendDispatcher {
    /// Select backend based on arithmetic intensity (FLOPs/Byte)
    ///
    /// # Arguments
    /// * `total_bytes` - Total data size in bytes
    /// * `estimated_flops` - Estimated floating point operations
    ///
    /// # Returns
    /// Backend selection (GPU, SIMD, or Scalar)
    ///
    /// # Algorithm
    /// 1. Check minimum data size threshold (10 MB)
    /// 2. Calculate `PCIe` transfer time: bytes / 32 GB/s
    /// 3. Estimate GPU compute time: FLOPs / 100 GFLOP/s
    /// 4. Apply 5x rule: GPU only if compute > 5x transfer
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn select(total_bytes: usize, estimated_flops: f64) -> super::Backend {
        // Rule 1: Minimum data size threshold (10 MB)
        if total_bytes < MIN_GPU_DATA_SIZE_BYTES {
            return super::Backend::Simd;
        }

        // Rule 2: Calculate transfer time (PCIe Gen4 x16 = 32 GB/s)
        let pcie_transfer_time_ms =
            (total_bytes as f64 / (PCIE_BANDWIDTH_GBPS * 1_000_000_000.0)) * 1000.0;

        // Rule 3: Estimate GPU compute time
        let estimated_gpu_compute_ms =
            (estimated_flops / (GPU_THROUGHPUT_GFLOPS * 1_000_000_000.0)) * 1000.0;

        // Rule 4: Apply 5x rule (Toyota Way: Genchi Genbutsu - physics-based decision)
        if estimated_gpu_compute_ms > pcie_transfer_time_ms * TRANSFER_OVERHEAD_MULTIPLIER {
            super::Backend::Gpu
        } else {
            super::Backend::Simd
        }
    }
}
