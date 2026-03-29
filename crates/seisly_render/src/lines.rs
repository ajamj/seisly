//! Line rendering utilities

use wgpu::util::DeviceExt;
use wgpu::{Buffer, BufferUsages, Device};

/// GPU line renderer for well trajectories and fault sticks
pub struct LineRenderer {
    vertex_buffer: Buffer,
    vertex_count: u32,
}

impl LineRenderer {
    pub fn new(device: &Device, points: &[[f32; 3]]) -> Self {
        let vertex_data: Vec<f32> = points.iter().flat_map(|p| p.iter().copied()).collect();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Line Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        Self {
            vertex_buffer,
            vertex_count: points.len() as u32,
        }
    }

    pub fn vertex_count(&self) -> u32 {
        self.vertex_count
    }

    pub fn vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }
}
