//! GPU compute backend using wgpu (WebGPU)
//!
//! Toyota Way Principles:
//! - Muda elimination: GPU only when compute > 5x transfer time
//! - Genchi Genbutsu: Empirical benchmarks prove 50-100x speedups
//!
//! Architecture:
//! - WGSL compute shaders for parallel reduction
//! - Workgroup size: 256 threads (GPU warp size optimization)
//! - Two-stage reduction: workgroup-local + global
//!
//! References:
//! - `HeavyDB` (2017): GPU aggregation patterns
//! - Harris (2007): Optimizing parallel reduction in CUDA
//! - Leis et al. (2014): Morsel-driven parallelism

use crate::{Error, Result};
use arrow::array::{Array, Float32Array, Int32Array};
use wgpu;

pub mod kernels;

/// GPU compute engine for aggregations
pub struct GpuEngine {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl GpuEngine {
    /// Initialize GPU engine
    ///
    /// # Errors
    /// Returns error if GPU initialization fails (no GPU available, driver issues, etc.)
    pub async fn new() -> Result<Self> {
        // Request GPU adapter
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| Error::GpuInitFailed("No GPU adapter found".to_string()))?;

        // Request device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Trueno-DB GPU Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None,
            )
            .await
            .map_err(|e| Error::GpuInitFailed(format!("Failed to create device: {e}")))?;

        Ok(Self { device, queue })
    }

    /// Execute SUM aggregation on GPU
    ///
    /// # Arguments
    /// * `data` - Input array (Int32 or Float32)
    ///
    /// # Returns
    /// Sum of all elements
    ///
    /// # Errors
    /// Returns error if GPU execution fails
    pub async fn sum_i32(&self, data: &Int32Array) -> Result<i32> {
        kernels::sum_i32(&self.device, &self.queue, data).await
    }

    /// Execute SUM aggregation on GPU (f32)
    ///
    /// # Errors
    /// Returns error if GPU execution fails
    pub async fn sum_f32(&self, data: &Float32Array) -> Result<f32> {
        kernels::sum_f32(&self.device, &self.queue, data).await
    }

    /// Execute COUNT aggregation on GPU
    ///
    /// # Errors
    /// Returns error if GPU execution fails
    pub async fn count(&self, data: &dyn Array) -> Result<usize> {
        kernels::count(&self.device, &self.queue, data).await
    }

    /// Execute MIN aggregation on GPU
    ///
    /// # Errors
    /// Returns error if GPU execution fails
    pub async fn min_i32(&self, data: &Int32Array) -> Result<i32> {
        kernels::min_i32(&self.device, &self.queue, data).await
    }

    /// Execute MAX aggregation on GPU
    ///
    /// # Errors
    /// Returns error if GPU execution fails
    pub async fn max_i32(&self, data: &Int32Array) -> Result<i32> {
        kernels::max_i32(&self.device, &self.queue, data).await
    }

    /// Execute AVG aggregation on GPU (reuses sum + count)
    ///
    /// # Errors
    /// Returns error if GPU execution fails
    #[allow(clippy::cast_precision_loss)]
    pub async fn avg_f32(&self, data: &Float32Array) -> Result<f32> {
        let sum = self.sum_f32(data).await?;
        let count = self.count(data).await?;
        if count == 0 {
            Ok(0.0)
        } else {
            Ok(sum / count as f32)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow::array::Int32Array;

    #[tokio::test]
    async fn test_gpu_init() {
        // This test may fail on machines without GPU
        match GpuEngine::new().await {
            Ok(_engine) => {
                // GPU initialization succeeded
            }
            Err(e) => {
                // Expected on machines without GPU
                eprintln!("GPU initialization failed (expected on CI): {e}");
            }
        }
    }

    #[tokio::test]
    async fn test_gpu_sum_basic() {
        let Ok(engine) = GpuEngine::new().await else {
            eprintln!("Skipping GPU test (no GPU available)");
            return;
        };

        let data = Int32Array::from(vec![1, 2, 3, 4, 5]);
        let result = engine.sum_i32(&data).await.unwrap();
        assert_eq!(result, 15);
    }
}
