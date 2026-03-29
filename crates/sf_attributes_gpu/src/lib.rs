//! GPU-Accelerated Seismic Attributes for StrataForge
//!
//! Provides GPU compute pipelines for seismic attribute computation
//! using wgpu for cross-platform GPU acceleration.

mod compute;

pub use compute::GpuAttributeComputer;

/// GPU compute error types
#[derive(Debug, thiserror::Error)]
pub enum GpuError {
    #[error("GPU initialization failed: {0}")]
    Initialization(String),
    #[error("Buffer operation failed: {0}")]
    Buffer(String),
    #[error("Compute dispatch failed: {0}")]
    Dispatch(String),
}

pub type Result<T> = std::result::Result<T, GpuError>;
