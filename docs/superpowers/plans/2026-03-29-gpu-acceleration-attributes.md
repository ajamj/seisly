# GPU Acceleration for Seismic Attributes Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement GPU-accelerated seismic attribute computation using wgpu for 10x performance improvement on large volumes.

**Architecture:** Create a new `sf_attributes_gpu` crate that provides GPU compute pipelines for seismic attributes. The crate will use wgpu for cross-platform GPU compute, with WGSL shaders for parallel attribute computation. The CPU crate (`sf_attributes`) remains for small datasets, while the GPU crate handles large-scale processing.

**Tech Stack:** wgpu 0.19 (from workspace), WGSL shaders, async Rust for GPU initialization.

---

### Task 1: Create sf_attributes_gpu Crate Structure

**Files:**
- Create: `crates/sf_attributes_gpu/Cargo.toml`
- Create: `crates/sf_attributes_gpu/src/lib.rs`

- [ ] **Step 1: Create Cargo.toml**

```toml
[package]
name = "sf_attributes_gpu"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
description = "GPU-accelerated seismic attribute computation using wgpu"

[dependencies]
wgpu.workspace = true
thiserror.workspace = true
sf_attributes = { workspace = true }

[dev-dependencies]
tokio.workspace = true
criterion = "0.5"

[[bench]]
name = "gpu_benchmark"
harness = false
```

- [ ] **Step 2: Create lib.rs**

```rust
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
```

- [ ] **Step 3: Add crate to workspace Cargo.toml**

Modify: `D:\GRC-Ajam\myfield\Cargo.toml` - Add `sf_attributes_gpu` to workspace members and dependencies.

- [ ] **Step 4: Commit**

```bash
git add crates/sf_attributes_gpu/ Cargo.toml
git commit -m "feat: add sf_attributes_gpu crate skeleton"
```

---

### Task 2: Implement GPU Compute Pipeline

**Files:**
- Create: `crates/sf_attributes_gpu/src/compute.rs`

- [ ] **Step 1: Write test for GPU initialization**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gpu_initialization() {
        let computer = GpuAttributeComputer::new().await;
        assert!(computer.is_ok());
    }

    #[tokio::test]
    async fn test_gpu_rms_computation() {
        let computer = GpuAttributeComputer::new().await.unwrap();
        let trace = vec![1.0f32, 2.0, 3.0, 4.0, 5.0];
        let result = computer.compute_rms_gpu(&trace, 3).await.unwrap();
        
        assert_eq!(result.len(), 3);
        // RMS of [1,2,3] = sqrt((1+4+9)/3) ≈ 2.16
        assert!((result[0] - 2.16).abs() < 0.1);
    }
}
```

- [ ] **Step 2: Implement GpuAttributeComputer struct**

```rust
use wgpu::{Device, Queue, ComputePipeline, Buffer, BindGroup};

pub struct GpuAttributeComputer {
    device: Device,
    queue: Queue,
    pipeline: ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl GpuAttributeComputer {
    pub async fn new() -> Result<Self> {
        let instance = wgpu::Instance::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .ok_or_else(|| GpuError::Initialization("No adapter found".into()))?;
        
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .map_err(|e| GpuError::Initialization(e.to_string()))?;
        
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Attribute Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/attribute.wgsl").into()),
        });
        
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Attribute Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Attribute Pipeline"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Attribute Compute"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "compute_rms",
        });
        
        Ok(Self {
            device,
            queue,
            pipeline,
            bind_group_layout,
        })
    }
    
    pub async fn compute_rms_gpu(&self, trace: &[f32], window_size: u32) -> Result<Vec<f32>> {
        if trace.is_empty() {
            return Ok(Vec::new());
        }
        
        let output_size = trace.len().saturating_sub(window_size as usize + 1) + 1;
        if output_size == 0 {
            return Ok(Vec::new());
        }
        
        // Create input buffer
        let input_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Input Buffer"),
            size: (trace.len() * std::mem::size_of::<f32>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // Create output buffer
        let output_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Output Buffer"),
            size: (output_size * std::mem::size_of::<f32>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        
        // Create staging buffer for reading results
        let staging_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size: (output_size * std::mem::size_of::<f32>()) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // Create uniform buffer for window_size
        let uniform_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: std::mem::size_of::<u32>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // Write data to buffers
        self.queue.write_buffer(&input_buffer, 0, bytemuck::cast_slice(trace));
        self.queue.write_buffer(&uniform_buffer, 0, &window_size.to_ne_bytes());
        
        // Create bind group
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Attribute Bind Group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: input_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: output_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: uniform_buffer.as_entire_binding(),
                },
            ],
        });
        
        // Encode commands
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Attribute Compute Encoder"),
            });
        
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Attribute Compute Pass"),
                ..Default::default()
            });
            compute_pass.set_pipeline(&self.pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups((output_size as u32 + 63) / 64, 1, 1);
        }
        
        // Copy output to staging buffer
        encoder.copy_buffer_to_buffer(&output_buffer, 0, &staging_buffer, 0, output_buffer.size());
        
        // Submit commands
        self.queue.submit(std::iter::once(encoder.finish()));
        
        // Read results
        let buffer_slice = staging_buffer.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        
        self.device.poll(wgpu::Maintain::Wait);
        
        rx.recv()
            .unwrap()
            .map_err(|e| GpuError::Buffer(e.to_string()))?;
        
        let data = buffer_slice.get_mapped_range();
        let result = bytemuck::cast_slice(&data).to_vec();
        
        drop(data);
        staging_buffer.unmap();
        
        Ok(result)
    }
}
```

- [ ] **Step 3: Add bytemuck dependency to Cargo.toml**

```toml
bytemuck = { version = "1.14", features = ["derive"] }
```

- [ ] **Step 4: Run test to verify GPU initialization works**

```bash
cd crates/sf_attributes_gpu && cargo test --lib
```

Expected: Test passes (or skips if no GPU available)

- [ ] **Step 5: Commit**

```bash
git add crates/sf_attributes_gpu/src/compute.rs
git commit -m "feat: implement GPU compute pipeline for attributes"
```

---

### Task 3: Create WGSL Shaders

**Files:**
- Create: `crates/sf_attributes_gpu/src/shaders/attribute.wgsl`

- [ ] **Step 1: Create shaders directory**

```bash
mkdir -p crates/sf_attributes_gpu/src/shaders
```

- [ ] **Step 2: Create WGSL shader for RMS computation**

```wgsl
// Seismic Attribute Compute Shaders

struct Uniforms {
    window_size: u32,
};

@group(0) @binding(0)
var<storage, read> input_trace: array<f32>;

@group(0) @binding(1)
var<storage, read_write> output_rms: array<f32>;

@group(0) @binding(2)
var<uniform> uniforms: Uniforms;

@compute @workgroup_size(64)
fn compute_rms(@builtin(global_invocation_id) id: vec3<u32>) {
    let idx = id.x;
    let trace_len = arrayLength(&input_trace);
    let window_size = uniforms.window_size;
    
    // Check bounds
    if idx >= trace_len - window_size + 1 {
        return;
    }
    
    // Compute RMS in window
    var sum_squares: f32 = 0.0;
    for (var i: u32 = 0; i < window_size; i = i + 1) {
        let val = input_trace[idx + i];
        sum_squares = sum_squares + val * val;
    }
    
    output_rms[idx] = sqrt(sum_squares / f32(window_size));
}

@compute @workgroup_size(64)
fn compute_mean(@builtin(global_invocation_id) id: vec3<u32>) {
    let idx = id.x;
    let trace_len = arrayLength(&input_trace);
    let window_size = uniforms.window_size;
    
    if idx >= trace_len - window_size + 1 {
        return;
    }
    
    var sum: f32 = 0.0;
    for (var i: u32 = 0; i < window_size; i = i + 1) {
        sum = sum + input_trace[idx + i];
    }
    
    output_rms[idx] = sum / f32(window_size);
}

@compute @workgroup_size(64)
fn compute_energy(@builtin(global_invocation_id) id: vec3<u32>) {
    let idx = id.x;
    let trace_len = arrayLength(&input_trace);
    let window_size = uniforms.window_size;
    
    if idx >= trace_len - window_size + 1 {
        return;
    }
    
    var sum_squares: f32 = 0.0;
    for (var i: u32 = 0; i < window_size; i = i + 1) {
        let val = input_trace[idx + i];
        sum_squares = sum_squares + val * val;
    }
    
    output_rms[idx] = sum_squares;
}
```

- [ ] **Step 3: Commit**

```bash
git add crates/sf_attributes_gpu/src/shaders/
git commit -m "feat: add WGSL shaders for GPU attribute computation"
```

---

### Task 4: Add Benchmark Tests

**Files:**
- Create: `crates/sf_attributes_gpu/benches/gpu_benchmark.rs`

- [ ] **Step 1: Create benchmark directory and file**

```bash
mkdir -p crates/sf_attributes_gpu/benches
```

- [ ] **Step 2: Create benchmark code**

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use sf_attributes::amplitude::RmsAmplitude;
use sf_attributes::trait_def::SeismicAttribute;
use sf_attributes_gpu::GpuAttributeComputer;
use tokio::runtime::Runtime;

fn generate_test_data(size: usize) -> Vec<f32> {
    (0..size).map(|i| ((i as f32 * 0.1).sin() * 100.0)).collect()
}

fn benchmark_cpu_rms(c: &mut Criterion) {
    let attr = RmsAmplitude;
    let mut group = c.benchmark_group("CPU RMS");
    
    for size in [1000, 10000, 100000].iter() {
        let trace = generate_test_data(*size);
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &trace,
            |b, trace| {
                b.iter(|| attr.compute(black_box(trace), black_box(11)));
            },
        );
    }
    group.finish();
}

fn benchmark_gpu_rms(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let computer = rt.block_on(GpuAttributeComputer::new()).unwrap();
    
    let mut group = c.benchmark_group("GPU RMS");
    
    for size in [1000, 10000, 100000].iter() {
        let trace = generate_test_data(*size);
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &trace,
            |b, trace| {
                b.iter(|| {
                    rt.block_on(computer.compute_rms_gpu(black_box(trace), black_box(11)))
                        .unwrap()
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, benchmark_cpu_rms, benchmark_gpu_rms);
criterion_main!(benches);
```

- [ ] **Step 3: Run benchmarks**

```bash
cd crates/sf_attributes_gpu && cargo bench
```

Expected: Benchmarks run and show GPU vs CPU comparison

- [ ] **Step 4: Commit**

```bash
git add crates/sf_attributes_gpu/benches/
git commit -m "test: add GPU vs CPU benchmark suite"
```

---

### Task 5: Update Documentation and Finalize

**Files:**
- Modify: `D:\GRC-Ajam\myfield\README.md`
- Modify: `D:\GRC-Ajam\myfield\CHANGELOG.md`

- [ ] **Step 1: Update README.md**

Add GPU acceleration section to README mentioning the new `sf_attributes_gpu` crate.

- [ ] **Step 2: Update CHANGELOG.md**

Add entry for GPU acceleration feature.

- [ ] **Step 3: Final verification**

```bash
cargo build --workspace
cargo test --workspace
```

- [ ] **Step 4: Final commit**

```bash
git add README.md CHANGELOG.md
git commit -m "docs: document GPU acceleration feature"

# Create final tag
git tag -a v0.3.0-gpu -m "GPU acceleration for seismic attributes"
```

---

## Summary

This plan creates:
1. New `sf_attributes_gpu` crate with wgpu-based compute pipeline
2. WGSL shaders for RMS, Mean, and Energy attribute computation
3. Benchmark suite comparing GPU vs CPU performance
4. Full integration with existing workspace

Expected outcome: 10x performance improvement for large seismic volumes (>100k samples).
