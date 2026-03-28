//! StrataForge Render Module
//!
//! Provides wgpu-based rendering primitives for 3D visualization.
//!
//! Note: This is a stub implementation for v0.1.
//! Full rendering implementation will be added in subsequent versions.

pub mod colormaps;
pub mod fault_renderer;
pub mod lines;
pub mod logs;
pub mod mesh;
pub mod points;
pub mod renderer;
pub mod scene;

pub use colormaps::{ColormapManager, ColormapPreset};
pub use fault_renderer::{FaultMesh, FaultRenderData, FaultRenderer};
pub use lines::LineRenderer;
pub use logs::LogRenderer;
pub use mesh::MeshRenderer;
pub use points::PointRenderer;
pub use renderer::Renderer;
pub use scene::Scene;
