//! Mesh rendering utilities

use sf_core::domain::surface::Mesh;
use wgpu::{Device, Buffer, BufferUsages};
use wgpu::util::DeviceExt;

/// GPU mesh renderer
pub struct MeshRenderer {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_count: u32,
}

impl MeshRenderer {
    pub fn new(device: &Device, mesh: &Mesh) -> Self {
        // Create vertex buffer
        let vertex_data: Vec<f32> = mesh.vertices
            .iter()
            .flat_map(|v| v.iter().copied())
            .collect();
        
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        
        // Create index buffer
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh Index Buffer"),
            contents: bytemuck::cast_slice(&mesh.indices),
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
        });
        
        Self {
            vertex_buffer,
            index_buffer,
            index_count: mesh.indices.len() as u32,
        }
    }
    
    pub fn index_count(&self) -> u32 {
        self.index_count
    }
    
    pub fn vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }
    
    pub fn index_buffer(&self) -> &Buffer {
        &self.index_buffer
    }
}
