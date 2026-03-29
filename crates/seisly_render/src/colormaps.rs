use std::collections::HashMap;
use wgpu::{
    Device, Extent3d, Queue, Texture, TextureDescriptor, TextureDimension, TextureFormat,
    TextureUsages, TextureView,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColormapPreset {
    Seismic,
    Viridis,
    Magma,
    Gray,
}

pub struct ColormapManager {
    textures: HashMap<ColormapPreset, (Texture, TextureView)>,
}

impl ColormapManager {
    pub fn new(device: &Device, queue: &Queue) -> Self {
        let mut textures = HashMap::new();

        for preset in [
            ColormapPreset::Seismic,
            ColormapPreset::Viridis,
            ColormapPreset::Magma,
            ColormapPreset::Gray,
        ] {
            let data = Self::generate_preset_data(&preset);
            let texture = device.create_texture(&TextureDescriptor {
                label: Some(&format!("Colormap Texture {:?}", preset)),
                size: Extent3d {
                    width: 256,
                    height: 1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D1,
                format: TextureFormat::Rgba8Unorm,
                usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
                view_formats: &[],
            });

            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &data,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: None, // bytes_per_row is optional for 1D/2D with height 1
                    rows_per_image: None,
                },
                Extent3d {
                    width: 256,
                    height: 1,
                    depth_or_array_layers: 1,
                },
            );

            let view = texture.create_view(&Default::default());
            textures.insert(preset, (texture, view));
        }

        Self { textures }
    }

    pub fn get_view(&self, preset: ColormapPreset) -> Option<&TextureView> {
        self.textures.get(&preset).map(|(_, view)| view)
    }

    fn lerp_color(c1: [u8; 3], c2: [u8; 3], t: f32) -> [u8; 4] {
        [
            (c1[0] as f32 + (c2[0] as f32 - c1[0] as f32) * t) as u8,
            (c1[1] as f32 + (c2[1] as f32 - c1[1] as f32) * t) as u8,
            (c1[2] as f32 + (c2[2] as f32 - c1[2] as f32) * t) as u8,
            255,
        ]
    }

    fn generate_preset_data(preset: &ColormapPreset) -> Vec<u8> {
        let mut data = Vec::with_capacity(256 * 4);
        for i in 0..256 {
            let t = i as f32 / 255.0;
            let color = match preset {
                ColormapPreset::Seismic => {
                    // Blue-White-Red
                    if t < 0.5 {
                        Self::lerp_color([0, 0, 255], [255, 255, 255], t * 2.0)
                    } else {
                        Self::lerp_color([255, 255, 255], [255, 0, 0], (t - 0.5) * 2.0)
                    }
                }
                ColormapPreset::Viridis => {
                    if t < 0.25 {
                        Self::lerp_color([68, 1, 84], [59, 82, 139], t * 4.0)
                    } else if t < 0.5 {
                        Self::lerp_color([59, 82, 139], [33, 145, 140], (t - 0.25) * 4.0)
                    } else if t < 0.75 {
                        Self::lerp_color([33, 145, 140], [94, 201, 98], (t - 0.5) * 4.0)
                    } else {
                        Self::lerp_color([94, 201, 98], [253, 231, 37], (t - 0.75) * 4.0)
                    }
                }
                ColormapPreset::Magma => {
                    if t < 0.25 {
                        Self::lerp_color([0, 0, 4], [80, 18, 123], t * 4.0)
                    } else if t < 0.5 {
                        Self::lerp_color([80, 18, 123], [182, 54, 121], (t - 0.25) * 4.0)
                    } else if t < 0.75 {
                        Self::lerp_color([182, 54, 121], [251, 135, 97], (t - 0.5) * 4.0)
                    } else {
                        Self::lerp_color([251, 135, 97], [251, 252, 191], (t - 0.75) * 4.0)
                    }
                }
                ColormapPreset::Gray => {
                    let val = (t * 255.0) as u8;
                    [val, val, val, 255]
                }
            };
            data.extend_from_slice(&color);
        }
        data
    }
}
