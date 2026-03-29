//! Fault surface renderer with transparency support

use sf_core::domain::surface::Mesh;
use wgpu::util::DeviceExt;
use wgpu::{
    BindGroup, BindGroupLayout, Buffer, BufferUsages, ColorTargetState, ColorWrites, Device,
    FragmentState, MultisampleState, PipelineLayoutDescriptor, PrimitiveState, PrimitiveTopology,
    RenderPass, RenderPipeline, ShaderModuleDescriptor, ShaderSource, VertexState,
};

/// GPU fault mesh with vertex and index buffers
pub struct FaultMesh {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_count: u32,
    pub center: [f32; 3],
}

impl FaultMesh {
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

        // Create vertex buffer with position and normal
        // Format: [x, y, z, nx, ny, nz] per vertex
        let mut vertex_data = Vec::with_capacity(mesh.vertices.len() * 6);
        let normals = mesh.normals.as_ref();

        for (i, v) in mesh.vertices.iter().enumerate() {
            vertex_data.push(v[0]);
            vertex_data.push(v[1]);
            vertex_data.push(v[2]);

            if let Some(normals) = normals {
                if i < normals.len() {
                    vertex_data.push(normals[i][0]);
                    vertex_data.push(normals[i][1]);
                    vertex_data.push(normals[i][2]);
                } else {
                    // Default normal (0, 0, 1)
                    vertex_data.push(0.0);
                    vertex_data.push(0.0);
                    vertex_data.push(1.0);
                }
            } else {
                // Default normal (0, 0, 1)
                vertex_data.push(0.0);
                vertex_data.push(0.0);
                vertex_data.push(1.0);
            }
        }

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Fault Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        // Create index buffer
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Fault Index Buffer"),
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

/// Uniform buffer for fault rendering with MVP matrices
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct FaultUniforms {
    model: [[f32; 4]; 4],
    view: [[f32; 4]; 4],
    projection: [[f32; 4]; 4],
    color: [f32; 4],
}

impl FaultUniforms {
    fn new(
        model: [[f32; 4]; 4],
        view: [[f32; 4]; 4],
        projection: [[f32; 4]; 4],
        color: [f32; 4],
    ) -> Self {
        Self {
            model,
            view,
            projection,
            color,
        }
    }
}

/// Prepared fault render data for a specific fault
pub struct FaultRenderData {
    bind_group: BindGroup,
    _uniform_buffer: Buffer,
}

/// Renderer for fault surfaces with transparency
pub struct FaultRenderer {
    pipeline: RenderPipeline,
    bind_group_layout: BindGroupLayout,
    uniform_bind_group_layout: BindGroupLayout,
}

impl FaultRenderer {
    pub fn new(device: &Device, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Fault Shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/fault.wgsl").into()),
        });

        // Uniform bind group layout for MVP + color
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("fault_uniform_bind_group_layout"),
            });

        // Vertex bind group layout (empty for now, but kept for future use)
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[],
            label: Some("fault_bind_group_layout"),
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Fault Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout, &uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Fault Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 24, // 6 * 4 bytes (position + normal)
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x3,
                        },
                        wgpu::VertexAttribute {
                            offset: 12,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x3,
                        },
                    ],
                }],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            pipeline,
            bind_group_layout,
            uniform_bind_group_layout,
        }
    }

    pub fn prepare_fault(
        &self,
        device: &Device,
        color: [f32; 4],
        model: [[f32; 4]; 4],
        view: [[f32; 4]; 4],
        projection: [[f32; 4]; 4],
    ) -> FaultRenderData {
        let uniform = FaultUniforms::new(model, view, projection, color);
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Fault Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("fault_bind_group"),
        });

        FaultRenderData {
            bind_group,
            _uniform_buffer: uniform_buffer,
        }
    }

    pub fn render<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        fault_mesh: &'a FaultMesh,
        render_data: &'a FaultRenderData,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &render_data.bind_group, &[]);
        render_pass.set_vertex_buffer(0, fault_mesh.vertex_buffer().slice(..));
        render_pass.set_index_buffer(
            fault_mesh.index_buffer().slice(..),
            wgpu::IndexFormat::Uint32,
        );
        render_pass.draw_indexed(0..fault_mesh.index_count(), 0, 0..1);
    }

    pub fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sf_core::domain::surface::Mesh;

    #[test]
    fn test_fault_uniforms() {
        let identity = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let uniform = FaultUniforms::new(identity, identity, identity, [1.0, 0.0, 0.0, 0.5]);
        assert_eq!(uniform.color, [1.0, 0.0, 0.0, 0.5]);
    }

    #[test]
    fn test_fault_mesh_center() {
        let mesh = Mesh::new(
            vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [1.0, 3.0, 0.0]],
            vec![0, 1, 2],
        );

        // Verify center calculation logic
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
