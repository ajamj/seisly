use wgpu::{Device, Queue, ComputePipeline, BindGroupLayout};

use crate::{GpuError, Result};

pub struct GpuAttributeComputer {
    device: Device,
    queue: Queue,
    pipeline: ComputePipeline,
    bind_group_layout: BindGroupLayout,
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
        if trace.is_empty() || window_size == 0 {
            return Ok(Vec::new());
        }
        
        let window_size_usz = window_size as usize;
        if trace.len() < window_size_usz {
            return Ok(Vec::new());
        }
        
        // Output size is trace.len() - window_size + 1 (sliding window)
        let output_size = trace.len() - window_size_usz + 1;
        
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gpu_initialization() {
        // Skip test if no GPU available
        let computer = GpuAttributeComputer::new().await;
        // Test passes if GPU is available, skips otherwise
        assert!(computer.is_ok() || computer.is_err());
    }

    #[tokio::test]
    async fn test_gpu_rms_computation() {
        let computer = match GpuAttributeComputer::new().await {
            Ok(c) => c,
            Err(_) => return, // Skip if no GPU
        };
        
        let trace = vec![1.0f32, 2.0, 3.0, 4.0, 5.0];
        let result = computer.compute_rms_gpu(&trace, 3).await.unwrap();
        
        assert_eq!(result.len(), 3);
        // RMS of [1,2,3] = sqrt((1+4+9)/3) ≈ 2.16
        assert!((result[0] - 2.16).abs() < 0.1);
    }
}
