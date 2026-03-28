//! StrataForge Render Module
//! 
//! Provides wgpu-based rendering primitives for 3D visualization.
//! 
//! Note: This is a stub implementation for v0.1.
//! Full rendering implementation will be added in subsequent versions.

pub mod mesh;
pub mod lines;
pub mod scene;

pub use mesh::MeshRenderer;
pub use lines::LineRenderer;
pub use scene::Scene;
