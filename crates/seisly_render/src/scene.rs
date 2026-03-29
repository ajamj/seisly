//! 3D scene management

use crate::{LineRenderer, MeshRenderer, PointRenderer};

/// 3D scene containing renderable objects
pub struct Scene {
    pub meshes: Vec<MeshRenderer>,
    pub lines: Vec<LineRenderer>,
    pub points: Vec<PointRenderer>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            meshes: vec![],
            lines: vec![],
            points: vec![],
        }
    }

    pub fn add_mesh(&mut self, renderer: MeshRenderer) {
        self.meshes.push(renderer);
    }

    pub fn add_lines(&mut self, renderer: LineRenderer) {
        self.lines.push(renderer);
    }

    pub fn add_points(&mut self, renderer: PointRenderer) {
        self.points.push(renderer);
    }

    pub fn mesh_count(&self) -> usize {
        self.meshes.len()
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn point_count(&self) -> usize {
        self.points.len()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
