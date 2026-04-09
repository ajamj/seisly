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
    Rainbow,
    BlueWhiteRed,
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
            ColormapPreset::Rainbow,
            ColormapPreset::BlueWhiteRed,
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
                ColormapPreset::Rainbow => {
                    // Red → Orange → Yellow → Green → Cyan → Blue → Violet
                    if t < 1.0 / 6.0 {
                        Self::lerp_color([255, 0, 0], [255, 127, 0], t * 6.0)
                    } else if t < 2.0 / 6.0 {
                        Self::lerp_color([255, 127, 0], [255, 255, 0], (t - 1.0 / 6.0) * 6.0)
                    } else if t < 3.0 / 6.0 {
                        Self::lerp_color([255, 255, 0], [0, 255, 0], (t - 2.0 / 6.0) * 6.0)
                    } else if t < 4.0 / 6.0 {
                        Self::lerp_color([0, 255, 0], [0, 255, 255], (t - 3.0 / 6.0) * 6.0)
                    } else if t < 5.0 / 6.0 {
                        Self::lerp_color([0, 255, 255], [0, 0, 255], (t - 4.0 / 6.0) * 6.0)
                    } else {
                        Self::lerp_color([0, 0, 255], [127, 0, 255], (t - 5.0 / 6.0) * 6.0)
                    }
                }
                ColormapPreset::BlueWhiteRed => {
                    // Same as Seismic — explicit alias
                    if t < 0.5 {
                        Self::lerp_color([0, 0, 255], [255, 255, 255], t * 2.0)
                    } else {
                        Self::lerp_color([255, 255, 255], [255, 0, 0], (t - 0.5) * 2.0)
                    }
                }
            };
            data.extend_from_slice(&color);
        }
        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rainbow_colormap_has_256_entries() {
        let data = ColormapManager::generate_preset_data(&ColormapPreset::Rainbow);
        assert_eq!(data.len(), 1024); // 256 * 4 bytes (RGBA)
    }

    #[test]
    fn test_rainbow_first_entry_is_red() {
        let data = ColormapManager::generate_preset_data(&ColormapPreset::Rainbow);
        assert_eq!(data[0], 255); // R
        assert_eq!(data[1], 0); // G
        assert_eq!(data[2], 0); // B
        assert_eq!(data[3], 255); // A
    }

    #[test]
    fn test_bluewhitered_midpoint_is_white() {
        let data = ColormapManager::generate_preset_data(&ColormapPreset::BlueWhiteRed);
        // The exact white point is at the transition between the two lerp segments.
        // Index 127 is the last entry in the blue-to-white segment (t=127/255, t*2=0.996)
        // Index 128 is the first in the white-to-red segment (t=128/255, (t-0.5)*2=0.004)
        // Check that both are very close to white (within 3 of 255)
        for idx in [127, 128] {
            let offset = idx * 4;
            assert!(
                data[offset] >= 252,
                "R at index {} should be ~255, got {}",
                idx,
                data[offset]
            );
            assert!(
                data[offset + 1] >= 252,
                "G at index {} should be ~255, got {}",
                idx,
                data[offset + 1]
            );
            assert!(
                data[offset + 2] >= 252,
                "B at index {} should be ~255, got {}",
                idx,
                data[offset + 2]
            );
        }
    }

    #[test]
    fn test_bluewhitered_has_256_entries() {
        let data = ColormapManager::generate_preset_data(&ColormapPreset::BlueWhiteRed);
        assert_eq!(data.len(), 1024);
    }

    #[test]
    fn test_bluewhitered_first_entry_is_blue() {
        let data = ColormapManager::generate_preset_data(&ColormapPreset::BlueWhiteRed);
        assert_eq!(data[0], 0); // R
        assert_eq!(data[1], 0); // G
        assert_eq!(data[2], 255); // B
        assert_eq!(data[3], 255); // A
    }

    #[test]
    fn test_bluewhitered_last_entry_is_red() {
        let data = ColormapManager::generate_preset_data(&ColormapPreset::BlueWhiteRed);
        let offset = 255 * 4;
        assert_eq!(data[offset], 255); // R
        assert_eq!(data[offset + 1], 0); // G
        assert_eq!(data[offset + 2], 0); // B
        assert_eq!(data[offset + 3], 255); // A
    }

    #[test]
    fn test_rainbow_last_entry_is_violet() {
        let data = ColormapManager::generate_preset_data(&ColormapPreset::Rainbow);
        let offset = 255 * 4;
        // Last entry should be close to violet (127, 0, 255)
        assert_eq!(data[offset + 1], 0); // G should be 0
        assert_eq!(data[offset + 2], 255); // B should be 255
        assert_eq!(data[offset + 3], 255); // A should be 255
    }
}
