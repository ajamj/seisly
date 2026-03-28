//! StrataForge Render Module
//! 
//! Provides wgpu-based rendering primitives for 3D visualization.
//! 
//! Note: This is a stub implementation for v0.1.
//! Full rendering implementation will be added in subsequent versions.

pub mod mesh;
pub mod lines;
pub mod points;
pub mod scene;
pub mod renderer;
pub mod colormaps;

pub use mesh::MeshRenderer;
pub use lines::LineRenderer;
pub use points::PointRenderer;
pub use scene::Scene;
pub use renderer::Renderer;
pub use colormaps::{ColormapManager, ColormapPreset};
