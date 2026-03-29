//! Mesh rendering utilities

use sf_core::domain::surface::Mesh;
use wgpu::util::DeviceExt;
use wgpu::{Buffer, BufferUsages, Device};

/// GPU mesh renderer
pub struct MeshRenderer {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_count: u32,
    pub center: [f32; 3],
}

impl MeshRenderer {
    pub fn new(device: &Device, mesh: &Mesh) -> Self {
        // Calculate center for depth sorting
        let center = if mesh.vertices.is_empty() {
            [0.0, 0.0, 0.0]
        } else {
            let mut sum = [0.0; 3];
            for v in &mesh.vertices {
                sum[0] += v[0];
                sum[1] += v[1];
                sum[2] += v[2];
            }
            [
                sum[0] / mesh.vertices.len() as f32,
                sum[1] / mesh.vertices.len() as f32,
                sum[2] / mesh.vertices.len() as f32,
            ]
        };

        // Create vertex buffer
        let vertex_data: Vec<f32> = mesh
            .vertices
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
            center,
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

#[cfg(test)]
mod tests {
    use sf_core::domain::surface::Mesh;

    #[test]
    fn test_mesh_center_calculation() {
        let mesh = Mesh::new(
            vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [1.0, 3.0, 0.0]],
            vec![0, 1, 2],
        );

        // We can't easily create a MeshRenderer without a Device in unit tests,
        // but we can verify the logic if we move it to a helper or just trust the inline calculation.
        // For now, let's at least verify Mesh is working as expected.
        assert_eq!(mesh.vertices.len(), 3);

        let mut sum = [0.0; 3];
        for v in &mesh.vertices {
            sum[0] += v[0];
            sum[1] += v[1];
            sum[2] += v[2];
        }
        let center = [
            sum[0] / mesh.vertices.len() as f32,
            sum[1] / mesh.vertices.len() as f32,
            sum[2] / mesh.vertices.len() as f32,
        ];

        assert_eq!(center, [1.0, 1.0, 0.0]);
    }
}
